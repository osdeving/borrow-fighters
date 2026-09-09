//! Captures real contacts against Rust, Duke, C and Go with the match renderer.
//!
//! Evidence uses actual body metrics, combat manifests and contact clocks. The
//! hidden window exports unchanged frames and a report; it does not send input.

use std::{
    collections::BTreeSet,
    error::Error,
    fs,
    io::Write,
    path::{Path, PathBuf},
    process::{Child, ChildStdin, Command, Stdio},
};

use borrow_fighters::{
    characters::{CHARACTER_BODY_METRICS_PATH, CharacterBodyMetricsCatalog, CharacterId},
    combat::{
        fighter::FighterInput,
        super_sequence::{CPP_BARRAGE_CADENCE, CPP_BARRAGE_HITS, CPP_BARRAGE_START},
    },
    config::{FIXED_TIMESTEP as DT, FLOOR_Y, WINDOW_HEIGHT, WINDOW_WIDTH, world_px},
    engine::{
        assets::{GameAssets, SpriteAtlasAsset},
        render::{GamepadStatus, draw_fight},
        sprites::{fighter_sprite_clip, frame_for_fighter_state},
    },
    game::{
        arena::ArenaId,
        feature_flags::{FeatureFlag, FeatureFlags},
        world::{World, WorldSpriteCombatManifests},
    },
};
use raylib::prelude::*;
use serde_json::json;

type ReviewResult<T> = Result<T, Box<dyn Error>>;
const DEFENDERS: [CharacterId; 4] = [
    CharacterId::Rust,
    CharacterId::Duke,
    CharacterId::C,
    CharacterId::Go,
];

#[derive(Clone, Copy, Debug)]
enum ContactCase {
    Body,
    Head,
    Low,
    GuardHigh,
    GuardLow,
    Throw,
    Cinematic,
    Knockout,
}

impl ContactCase {
    fn name(self) -> &'static str {
        match self {
            Self::Body => "body",
            Self::Head => "head",
            Self::Low => "low",
            Self::GuardHigh => "guard-high",
            Self::GuardLow => "guard-low",
            Self::Throw => "throw",
            Self::Cinematic => "cinematic",
            Self::Knockout => "knockout",
        }
    }

    fn inputs(self, reverse: bool) -> (FighterInput, FighterInput) {
        let mut attack = FighterInput::default();
        let mut defense = FighterInput::default();
        match self {
            Self::Body => attack.light_punch = true,
            Self::Head => {
                attack.heavy_punch = true;
                attack.left = reverse;
                attack.right = !reverse;
            }
            Self::Low => {
                attack.crouch = true;
                attack.kick = true;
            }
            Self::GuardHigh => {
                attack.light_punch = true;
                defense.block = true;
            }
            Self::GuardLow => {
                attack.crouch = true;
                attack.kick = true;
                defense.block = true;
                defense.crouch = true;
            }
            Self::Throw => {
                attack.block = true;
                attack.light_punch = true;
            }
            Self::Cinematic | Self::Knockout => attack.cinematic_special = true,
        }
        (attack, defense)
    }
}

fn atlas(assets: &GameAssets, character: CharacterId) -> ReviewResult<&SpriteAtlasAsset> {
    match character {
        CharacterId::Rust => assets.rust_fighter.as_ref(),
        CharacterId::Duke => assets.duke_fighter.as_ref(),
        CharacterId::C => assets.c_fighter.as_ref(),
        CharacterId::Go => assets.go_fighter.as_ref(),
        CharacterId::Python => assets.python_fighter.as_ref(),
        CharacterId::Cpp => assets.cpp_fighter.as_ref(),
    }
    .ok_or_else(|| format!("{} atlas did not load", character.audio_key()).into())
}

fn main() -> ReviewResult<()> {
    let mut output = None;
    let mut video = None;
    let mut reverse = false;
    let mut protected = false;
    let mut barrage_only = false;
    let mut requested_defender = None;
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--output" => {
                output = Some(PathBuf::from(
                    args.next().ok_or("--output requires a path")?,
                ))
            }
            "--reverse" => reverse = true,
            "--video" => video = Some(PathBuf::from(args.next().ok_or("--video requires a path")?)),
            "--no-damage" => protected = true,
            "--barrage-only" => barrage_only = true,
            "--defender" => {
                let value = args
                    .next()
                    .ok_or("--defender requires rust, duke, c or go")?;
                let id = CharacterId::from_audio_key(&value).ok_or("unknown defender")?;
                if !DEFENDERS.contains(&id) {
                    return Err("defender must be rust, duke, c or go".into());
                }
                requested_defender = Some(id);
            }
            _ => return Err(format!("unknown argument: {arg}").into()),
        }
    }
    let output = output.ok_or("usage: capture_roster_contacts --output PATH [--video PATH] [--reverse] [--no-damage] [--barrage-only] [--defender ID]")?;
    if output.exists() && fs::read_dir(&output)?.next().is_some() {
        return Err("output directory is not empty; use a new evidence directory".into());
    }
    fs::create_dir_all(&output)?;
    let (mut raylib, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Borrow Fighters - roster contact review")
        .hidden()
        .msaa_4x()
        .build();
    let assets = GameAssets::load(&mut raylib, &thread);
    let metrics = CharacterBodyMetricsCatalog::load(CHARACTER_BODY_METRICS_PATH)?;
    let mut target =
        raylib.load_render_texture(&thread, WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32)?;
    let mut movie = video.as_deref().map(Movie::new).transpose()?;
    let mut video_frame_count = 0u32;
    let mut cases = vec![
        ContactCase::Body,
        ContactCase::Head,
        ContactCase::Low,
        ContactCase::GuardHigh,
        ContactCase::GuardLow,
        ContactCase::Throw,
        ContactCase::Cinematic,
    ];
    if !protected {
        cases.push(ContactCase::Knockout);
    }
    if barrage_only {
        cases = vec![ContactCase::Cinematic];
    }
    let mut report = Vec::new();
    for defender in DEFENDERS {
        if requested_defender.is_some_and(|selected| selected != defender) {
            continue;
        }
        let sheet = atlas(&assets, defender)?;
        for case in &cases {
            let video_first_frame = video_frame_count;
            let mut world =
                World::new_with_character_body_metrics(CharacterId::Cpp, defender, &metrics);
            world.set_sprite_combat_manifests(WorldSpriteCombatManifests {
                player_one: Some(atlas(&assets, CharacterId::Cpp)?.combat_manifest.clone()),
                player_two: Some(sheet.combat_manifest.clone()),
            });
            world.player_one.position.x = if reverse { 840.0 } else { 420.0 };
            world.player_two.position.x = if reverse {
                world.player_one.position.x - world.player_two.body_rect().width - world_px(4.0)
            } else {
                world.player_one.body_rect().right() + world_px(4.0)
            };
            let mut flags = FeatureFlags::default();
            flags.set(FeatureFlag::PlayerTwoTakesDamage, !protected);
            flags.set(FeatureFlag::ShowCombatDebug, false);
            flags.set(FeatureFlag::ShowControlsHelp, false);
            if protected || matches!(case, ContactCase::Knockout) {
                world.player_two.health = 1;
            }
            let initial_health = world.player_two.health;
            let (attack, defense) = case.inputs(reverse);
            let mut seen = BTreeSet::new();
            let mut observations = Vec::new();
            let mut screenshots = Vec::new();
            let mut first_contact = None;
            let mut previous_reaction = false;
            let mut recovered_at = None;
            let mut frames_run = 0;
            for tick in 0..760u32 {
                let before = world.player_two.health;
                world.update_with_flags(
                    DT,
                    if tick == 0 {
                        attack
                    } else {
                        FighterInput::default()
                    },
                    defense,
                    flags,
                );
                frames_run += 1;
                let victim = &world.player_two;
                let contact = victim.contact_reaction_state();
                let frame = frame_for_fighter_state(
                    &sheet.manifest,
                    victim,
                    fighter_sprite_clip(victim),
                    world.elapsed_seconds,
                )
                .ok_or("fighter frame unavailable")?;
                let sequence_tick = world.super_sequence().map(|sequence| sequence.tick);
                let barrage_frame = sequence_tick.is_some_and(|tick| {
                    (CPP_BARRAGE_START..CPP_BARRAGE_START + CPP_BARRAGE_HITS * CPP_BARRAGE_CADENCE)
                        .contains(&tick)
                });
                let mut capture = false;
                if let Some(contact) = contact {
                    first_contact.get_or_insert(tick);
                    if !frame.clip.starts_with("reaction_") {
                        return Err(format!(
                            "{} still uses fallback {}",
                            defender.audio_key(),
                            frame.clip
                        )
                        .into());
                    }
                    capture = seen.insert(frame.name.clone());
                    observations.push(json!({"tick":tick,"sequence_tick":sequence_tick,"profile":format!("{:?}",contact.profile),
                        "age_frames":contact.elapsed_seconds*60.0,"duration_frames":contact.duration_seconds*60.0,
                        "frame":frame.name,"clip":frame.clip,"pivot":frame.pivot,"frame_rect":frame.frame,
                        "scale":sheet.manifest.scale,"health":victim.health,"damage":before-victim.health,
                        "physical_x":victim.position.x,"physical_y":victim.position.y,"feet_y":victim.body_rect().bottom(),
                        "grounded":victim.grounded,"captured":victim.in_capture()}));
                } else if previous_reaction {
                    capture = true;
                    recovered_at = Some(tick);
                }
                previous_reaction = contact.is_some();
                // Retain each impact and its complete nine-frame response even
                // when this is a full profile capture, not a barrage-only run.
                capture |= barrage_frame;
                if barrage_only {
                    capture = barrage_frame;
                }
                if capture || movie.is_some() {
                    {
                        let mut draw = raylib.begin_texture_mode(&thread, &mut target);
                        draw_fight(
                            &mut draw,
                            &world,
                            ArenaId::JavaStreet,
                            world.elapsed_seconds,
                            flags,
                            GamepadStatus::default(),
                            &assets,
                        );
                    }
                    if let Some(movie) = &mut movie {
                        movie.frame(&target)?;
                        video_frame_count += 1;
                    }
                }
                if capture {
                    let name = format!(
                        "{}-{}-{}-{tick:04}-{}.png",
                        defender.audio_key(),
                        case.name(),
                        if reverse { "left" } else { "right" },
                        frame.name
                    );
                    export(&target, &output.join(&name))?;
                    screenshots.push(json!({"image":name,"tick":tick,"sequence_tick":sequence_tick,"frame":frame.name,"clip":frame.clip}));
                }
                if protected && victim.health != initial_health {
                    return Err("protected health changed".into());
                }
                if !world.super_sequence_active() && first_contact.is_some() {
                    if victim.is_defeated() && tick > 690 {
                        break;
                    }
                    if contact.is_none()
                        && recovered_at.is_some_and(|recovered| tick >= recovered + 20)
                    {
                        break;
                    }
                }
            }
            if first_contact.is_none() {
                return Err(
                    format!("no real contact: {defender:?}/{case:?}/reverse={reverse}").into(),
                );
            }
            if barrage_only
                && screenshots.len() != (CPP_BARRAGE_HITS * CPP_BARRAGE_CADENCE) as usize
            {
                return Err("barrage capture lost a consecutive frame".into());
            }
            println!(
                "ROSTER_CONTACT {} {} reverse={reverse} protected={protected} frames={frames_run} screenshots={}",
                defender.audio_key(),
                case.name(),
                screenshots.len()
            );
            report.push(json!({"defender":defender.audio_key(),"attacker":"cpp","case":case.name(),"reverse":reverse,"protected_health":protected,
                "initial_health":initial_health,"final_health":world.player_two.health,"first_contact_tick":first_contact,"recovered_at":recovered_at,
                "frames_run":frames_run,"video_frames":video.as_ref().map(|_| [video_first_frame,video_frame_count-1]),
                "observations":observations,"screenshots":screenshots}));
        }
    }
    if let Some(movie) = movie {
        movie.finish()?;
    }
    fs::write(
        output.join("roster-contact-review.json"),
        serde_json::to_string_pretty(&json!({
            "fps":60,"render_size":[WINDOW_WIDTH,WINDOW_HEIGHT],"source":"World::update_with_flags + render::draw_fight",
            "audio":"not captured","floor_y":FLOOR_Y,"barrage_only":barrage_only,
            "video":video,"video_frame_count":video_frame_count,"scenarios":report
        }))? + "\n",
    )?;
    Ok(())
}

fn export(target: &RenderTexture2D, path: &Path) -> ReviewResult<()> {
    let mut pixels = target.texture().load_image()?;
    pixels.flip_vertical();
    fs::write(path, pixels.export_image_to_memory(".png")?.as_ref())?;
    Ok(())
}

/// Streams each unchanged framebuffer at its simulation tick, without interpolation.
struct Movie {
    child: Child,
    stdin: ChildStdin,
}

impl Movie {
    fn new(path: &Path) -> ReviewResult<Self> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let mut child = Command::new("ffmpeg")
            .args([
                "-hide_banner",
                "-loglevel",
                "error",
                "-n",
                "-thread_queue_size",
                "8",
                "-f",
                "rawvideo",
                "-pixel_format",
                "rgba",
                "-video_size",
            ])
            .arg(format!("{WINDOW_WIDTH}x{WINDOW_HEIGHT}"))
            .args([
                "-framerate",
                "60",
                "-i",
                "pipe:0",
                "-an",
                "-c:v",
                "libx264",
                "-preset",
                "ultrafast",
                "-threads",
                "1",
                "-crf",
                "20",
                "-pix_fmt",
                "yuv420p",
                "-movflags",
                "+faststart",
            ])
            .arg(path)
            .stdin(Stdio::piped())
            .spawn()?;
        let stdin = child.stdin.take().ok_or("ffmpeg stdin unavailable")?;
        Ok(Self { child, stdin })
    }

    fn frame(&mut self, target: &RenderTexture2D) -> ReviewResult<()> {
        let pixels = borrow_fighters::engine::video_capture::read_render_texture_rgba(target)?;
        self.stdin.write_all(&pixels)?;
        Ok(())
    }

    fn finish(self) -> ReviewResult<()> {
        let Self { mut child, stdin } = self;
        drop(stdin);
        if !child.wait()?.success() {
            return Err("ffmpeg failed".into());
        }
        Ok(())
    }
}
