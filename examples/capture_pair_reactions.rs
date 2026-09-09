//! Captures Python/C++ contact windows and complete paired move demonstrations.
//!
//! System: Development evidence at the Raylib boundary. MoveShowcase drives the
//! real World; every selected reaction frame is logged beside its contact clock.

use borrow_fighters::{
    audio::MusicTrack,
    characters::{CHARACTER_BODY_METRICS_PATH, CharacterBodyMetricsCatalog, CharacterId},
    config::{WINDOW_HEIGHT, WINDOW_WIDTH},
    engine::{
        assets::{GameAssets, SpriteAtlasAsset},
        render::draw_move_showcase,
        sprites::{fighter_sprite_clip, frame_for_fighter_state},
    },
    game::{arena::ArenaId, feature_flags::FeatureFlags, world::WorldSpriteCombatManifests},
    scenes::{
        combat_lab::{CombatLabInput, CombatLabMove},
        move_showcase::{MoveShowcase, MoveShowcaseOptions, ShowcaseResult, ShowcaseScenario},
    },
};
use raylib::prelude::*;
use serde_json::{Value, json};
use std::{
    error::Error,
    fs,
    io::Write,
    path::{Path, PathBuf},
    process::{Child, ChildStdin, Command, Stdio},
};

type ReviewResult<T> = Result<T, Box<dyn Error>>;

fn main() -> ReviewResult<()> {
    let mut output = PathBuf::from("target/pair-reaction-review");
    let mut video = None;
    let mut reverse = false;
    let mut supers_only = false;
    let mut snapshots = true;
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--output" => output = args.next().ok_or("--output requires a path")?.into(),
            "--video" => video = Some(PathBuf::from(args.next().ok_or("--video requires a path")?)),
            "--reverse" => reverse = true,
            "--supers-only" => supers_only = true,
            "--no-snapshots" => snapshots = false,
            _ => return Err(format!("unknown argument {arg}").into()),
        }
    }
    fs::create_dir_all(&output)?;
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Borrow Fighters - Python / C++ reaction review")
        .msaa_4x()
        .build();
    rl.set_target_fps(0);
    let assets = GameAssets::load(&mut rl, &thread);
    let metrics = CharacterBodyMetricsCatalog::load(CHARACTER_BODY_METRICS_PATH)?;
    let mut texture = rl.load_render_texture(&thread, WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32)?;
    let mut movie = video.as_ref().map(|path| Movie::new(path)).transpose()?;
    let mut global_frame = 0u32;
    let mut records = Vec::new();
    let mut scenarios: Vec<_> = if supers_only {
        vec![ShowcaseScenario::Attack(CombatLabMove::CinematicSpecial)]
    } else {
        CombatLabMove::ALL
            .into_iter()
            .map(ShowcaseScenario::Attack)
            .collect()
    };
    if !supers_only {
        scenarios.extend([
            ShowcaseScenario::StandingBlock,
            ShowcaseScenario::OverheadBlock,
            ShowcaseScenario::CrouchingBlock,
            ShowcaseScenario::ProjectileBlock,
        ]);
    }
    for character in [CharacterId::Cpp, CharacterId::Python] {
        for &scenario in &scenarios {
            let mut scene = MoveShowcase::new(MoveShowcaseOptions { character });
            scene.set_body_metrics(metrics.clone());
            scene.set_sprite_combat_manifests(WorldSpriteCombatManifests {
                player_one: Some(atlas(&assets, character)?.combat_manifest.clone()),
                player_two: Some(
                    atlas(&assets, scene.opponent_character())?
                        .combat_manifest
                        .clone(),
                ),
            });
            if reverse {
                scene.switch_sides();
            }
            scene.select_scenario(scenario);
            let start = global_frame;
            let mut observations = Vec::new();
            let mut audio_events = Vec::new();
            let mut images = Vec::new();
            let mut first_contact = None;
            let mut settled = 0;
            let mut was_reacting = false;
            println!(
                "PAIR_START {} {:?} reverse={reverse}",
                character.audio_key(),
                scenario
            );
            for local in 0..scene.scenario_frames() {
                scene.update(CombatLabInput::default());
                for event in scene.take_audio_events() {
                    audio_events.push(json!({"video_frame":global_frame,"cue_key":event.cue.key(),
                        "character":event.character.map(|id|id.audio_key()),
                        "move_id":event.move_id.map(|id|id.audio_key()),"environment":event.environment,
                        "slot":event.slot.map(|slot|format!("{slot:?}"))}));
                }
                let world = scene.world();
                let victim = if scenario.is_defense() {
                    &world.player_one
                } else {
                    &world.player_two
                };
                let defender = if scenario.is_defense() {
                    character
                } else {
                    scene.opponent_character()
                };
                let contact = victim.contact_reaction_state();
                let mut capture = None;
                if let Some(contact) = contact {
                    first_contact.get_or_insert(local);
                    let age = (contact.elapsed_seconds * 60.0).round() as u32;
                    let frame = frame_for_fighter_state(
                        &atlas(&assets, defender)?.manifest,
                        victim,
                        fighter_sprite_clip(victim),
                        world.elapsed_seconds,
                    )
                    .ok_or("reaction has no frame")?;
                    observations.push(json!({"video_frame":global_frame,"local_frame":local,
                        "sequence_tick":world.super_sequence().map(|s|s.tick),
                        "defender":defender.audio_key(),"profile":format!("{:?}",contact.profile),
                        "age_frames":age,"duration_frames":contact.duration_seconds*60.0,
                        "progress":contact.progress(),"sprite_frame":frame.name,"sprite_clip":frame.clip,
                        "health":victim.health,"grounded":victim.grounded,
                        "hidden":world.super_sequence().is_some_and(|s|s.target_hidden()),
                        "physical_x":victim.position.x,"physical_y":victim.position.y}));
                    if snapshots && [0, 3, 7, 14].contains(&age) {
                        capture = Some(format!(
                            "{}-{:?}-{}-{local:03}-{:?}-{age}.png",
                            character.audio_key(),
                            scenario,
                            if reverse { "left" } else { "right" },
                            contact.profile
                        ));
                    }
                }
                if snapshots && was_reacting && contact.is_none() {
                    capture = Some(format!(
                        "{}-{:?}-{}-{local:03}-recovered.png",
                        character.audio_key(),
                        scenario,
                        if reverse { "left" } else { "right" }
                    ));
                }
                was_reacting = contact.is_some();
                if movie.is_some() || capture.is_some() {
                    {
                        let mut draw = rl.begin_texture_mode(&thread, &mut texture);
                        draw_move_showcase(
                            &mut draw,
                            &scene,
                            ArenaId::JavaStreet,
                            world.elapsed_seconds,
                            FeatureFlags::default(),
                            &assets,
                        );
                    }
                    if let Some(name) = capture {
                        let mut pixels = texture.texture().load_image()?;
                        pixels.flip_vertical();
                        fs::write(
                            output.join(&name),
                            pixels.export_image_to_memory(".png")?.as_ref(),
                        )?;
                        images.push(json!({"image":name,"video_frame":global_frame}));
                    }
                    if let Some(movie) = &mut movie {
                        movie.frame(&texture)?;
                    }
                    let mut draw = rl.begin_drawing(&thread);
                    draw.draw_texture_pro(
                        texture.texture(),
                        Rectangle::new(0., 0., WINDOW_WIDTH as f32, -(WINDOW_HEIGHT as f32)),
                        Rectangle::new(0., 0., WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32),
                        Vector2::zero(),
                        0.,
                        Color::WHITE,
                    );
                }
                global_frame += 1;
                if rl.window_should_close() {
                    return Err("capture interrupted".into());
                }
                if first_contact.is_some()
                    && !world.super_sequence_active()
                    && world.player_one.contact_reaction_state().is_none()
                    && world.player_two.contact_reaction_state().is_none()
                {
                    settled += 1;
                } else {
                    settled = 0;
                }
                if settled >= 36 && local >= 110 {
                    break;
                }
            }
            if matches!(
                scene.result(),
                ShowcaseResult::Pending | ShowcaseResult::Whiff
            ) {
                return Err(format!("no contact in {:?}/{scenario:?}", character).into());
            }
            let damage = match scene.result() {
                ShowcaseResult::Hit { damage } | ShowcaseResult::Blocked { damage } => damage,
                _ => 0,
            };
            records.push(json!({"character":character.audio_key(),"opponent":scene.opponent_character().audio_key(),
                "scenario":format!("{scenario:?}"),"move":scene.move_label(),"reverse":reverse,
                "guarded":scenario.is_defense(),"result":format!("{:?}",scene.result()),"damage":damage,
                "video_frames":[start,global_frame-1],"music_track":MusicTrack::CombatConsoleFloor.key(),
                "audio_events":audio_events,"reactions":observations,"screenshots":images}));
            println!(
                "PAIR_END {} {:?} {:?} frame={global_frame}",
                character.audio_key(),
                scenario,
                scene.result()
            );
        }
    }
    if let Some(movie) = movie {
        movie.finish()?;
    }
    let report: Value = json!({"fps":60,"render_size":[WINDOW_WIDTH,WINDOW_HEIGHT],"video":video,
        "video_frame_count":global_frame,"video_duration_seconds":global_frame as f64/60.0,
        "audio_delivery":"silent deterministic video; optional offline mix from exported World cues",
        "scenarios":records});
    fs::write(
        output.join("pair-reaction-review.json"),
        serde_json::to_string_pretty(&report)? + "\n",
    )?;
    Ok(())
}

fn atlas(assets: &GameAssets, character: CharacterId) -> ReviewResult<&SpriteAtlasAsset> {
    match character {
        CharacterId::Python => assets.python_fighter.as_ref(),
        CharacterId::Cpp => assets.cpp_fighter.as_ref(),
        _ => None,
    }
    .ok_or_else(|| format!("missing {} atlas", character.audio_key()).into())
}

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
                "veryfast",
                "-threads",
                "2",
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
