//! Captures cinematic specials and Brazilian stage life through the game renderer.
//!
//! System: Development examples at the Raylib boundary. Drives the actual
//! showcase at 60 Hz and records GPU frames, local contacts and optional MP4.

use std::{
    error::Error,
    fs,
    io::Write,
    path::Path,
    process::{Child, ChildStdin, Command, Stdio},
};

use borrow_fighters::{
    characters::{CHARACTER_BODY_METRICS_PATH, CharacterBodyMetricsCatalog, CharacterId},
    config::{WINDOW_HEIGHT, WINDOW_WIDTH},
    engine::{
        assets::{GameAssets, SpriteAtlasAsset},
        render::{GamepadStatus, draw_fight, draw_move_showcase},
    },
    game::{
        arena::ArenaId,
        feature_flags::{FeatureFlag, FeatureFlags},
        world::{World, WorldSpriteCombatManifests},
    },
    scenes::{
        combat_lab::{CombatLabInput, CombatLabMove},
        move_showcase::{MoveShowcase, MoveShowcaseOptions, ShowcaseResult},
    },
};
use raylib::prelude::*;
use serde_json::json;

type ReviewResult<T> = Result<T, Box<dyn Error>>;
const VIDEO_TICKS: u32 = 180;
const STAGE_VIDEO_TICKS: u32 = 480;
const ROSTER: [CharacterId; 6] = [
    CharacterId::Rust,
    CharacterId::Duke,
    CharacterId::Go,
    CharacterId::C,
    CharacterId::Python,
    CharacterId::Cpp,
];

fn main() -> ReviewResult<()> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if !(1..=2).contains(&args.len()) {
        return Err("usage: cargo run --example capture_presentation_review -- <output-directory> [video.mp4]".into());
    }
    let output = Path::new(&args[0]);
    fs::create_dir_all(output)?;
    let (mut raylib, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Borrow Fighters - presentation review")
        .hidden()
        .msaa_4x()
        .build();
    let assets = GameAssets::load(&mut raylib, &thread);
    if assets.caramelo_run.is_none() || assets.jessica_gesture.is_none() {
        return Err("stage-life reference assets must load before presentation review".into());
    }
    let mut movie = args
        .get(1)
        .map(|path| Movie::new(Path::new(path)))
        .transpose()?;
    let metrics = CharacterBodyMetricsCatalog::load(CHARACTER_BODY_METRICS_PATH)?;
    let mut target =
        raylib.load_render_texture(&thread, WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32)?;
    let mut scenarios = Vec::new();
    for (index, character) in ROSTER.into_iter().enumerate() {
        for reverse in [false, true] {
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
            scene.select_move(CombatLabMove::CinematicSpecial);
            if reverse {
                scene.switch_sides();
            }
            let mut captures = Vec::new();
            let mut contact_tick = None;
            for tick in 1..=200 {
                scene.update(CombatLabInput::default());
                if contact_tick.is_none() && matches!(scene.result(), ShowcaseResult::Hit { .. }) {
                    contact_tick = Some(tick);
                }
                let state = scene.world().player_one.cinematic_special();
                let phase = state.and_then(|state| {
                    if state.elapsed_frames == state.active_start.saturating_sub(10) {
                        Some("startup")
                    } else if state.elapsed_frames == state.active_start + 2 {
                        Some("active")
                    } else if state.elapsed_frames == state.active_end + 22 {
                        Some("recovery")
                    } else {
                        None
                    }
                });
                if phase.is_some() || movie.is_some() && !reverse && tick <= VIDEO_TICKS {
                    {
                        let mut draw = raylib.begin_texture_mode(&thread, &mut target);
                        draw_move_showcase(
                            &mut draw,
                            &scene,
                            ArenaId::JavaStreet,
                            scene.world().elapsed_seconds,
                            FeatureFlags::default(),
                            &assets,
                        );
                    }
                    if !reverse
                        && tick <= VIDEO_TICKS
                        && let Some(movie) = &mut movie
                    {
                        movie.frame(&target)?;
                    }
                    if let Some(phase) = phase {
                        let filename = format!(
                            "{}-{}-{phase}.png",
                            character.audio_key(),
                            if reverse { "left" } else { "right" }
                        );
                        export(&target, &output.join(&filename))?;
                        let state = state.expect("scheduled capture has cinematic state");
                        let hitbox = scene
                            .world()
                            .player_one
                            .attack_box()
                            .map(|b| [b.x, b.y, b.width, b.height]);
                        captures.push(json!({"image":filename, "phase":phase, "showcase_tick":tick,
                            "attack_tick":state.elapsed_frames, "active_frames":[state.active_start,state.active_end],
                            "local_hitbox":hitbox, "opponent_health":scene.world().player_two.health, "result":format!("{:?}",scene.result())}));
                    }
                }
            }
            if captures.len() != 3
                || !matches!(scene.result(), ShowcaseResult::Hit { damage } if damage>0)
            {
                return Err(format!(
                    "{character:?}/{reverse}: expected three phases and real contact, got {} {:?}",
                    captures.len(),
                    scene.result()
                )
                .into());
            }
            scenarios.push(json!({"character":character.audio_key(),"reverse":reverse,"contact_tick":contact_tick,
                "result":format!("{:?}",scene.result()),"captures":captures,
                "video_frames": if reverse { None } else { Some([index as u32*VIDEO_TICKS,(index as u32+1)*VIDEO_TICKS-1]) }}));
            println!(
                "Captured {} facing {}",
                character.audio_key(),
                if reverse { "left" } else { "right" }
            );
        }
    }
    let world = World::new_greybox();
    let mut stages = Vec::new();
    for (arena, key, time) in [
        (ArenaId::JavaStreet, "java-street", 9.5_f32),
        (ArenaId::JavaStreet, "java-street", 35.0),
        (ArenaId::Sirius, "sirius", 8.4),
    ] {
        for enabled in [true, false] {
            let mut flags = FeatureFlags::default();
            flags.set(FeatureFlag::ShowStageLife, enabled);
            {
                let mut draw = raylib.begin_texture_mode(&thread, &mut target);
                draw_fight(
                    &mut draw,
                    &world,
                    arena,
                    time,
                    flags,
                    GamepadStatus::default(),
                    &assets,
                );
            }
            let filename = format!(
                "{key}-{:03}-{}.png",
                (time * 10.0) as u32,
                if enabled { "on" } else { "off" }
            );
            export(&target, &output.join(&filename))?;
            stages.push(json!({"image":filename,"arena":format!("{arena:?}"),"visual_time_seconds":time,"stage_life":enabled}));
        }
    }
    if let Some(movie) = &mut movie {
        for tick in 0..STAGE_VIDEO_TICKS {
            let time = 6.0 + tick as f32 / 60.0;
            {
                let mut draw = raylib.begin_texture_mode(&thread, &mut target);
                draw_fight(
                    &mut draw,
                    &world,
                    ArenaId::JavaStreet,
                    time,
                    FeatureFlags::default(),
                    GamepadStatus::default(),
                    &assets,
                );
            }
            movie.frame(&target)?;
        }
        println!("Captured eight seconds of Java Street stage life");
    }
    if let Some(movie) = movie {
        movie.finish()?;
    }
    fs::write(
        output.join("presentation-review.json"),
        serde_json::to_string_pretty(&json!({
            "render_size":[WINDOW_WIDTH,WINDOW_HEIGHT],"fps":60,"audio":false,
            "simulation":"actual MoveShowcase fixed updates; no synthetic contacts or interpolation",
            "video":args.get(1),"video_frame_count":if args.len()==2 { VIDEO_TICKS*6+STAGE_VIDEO_TICKS } else { 0 },
        "stage_video": { "arena":"JavaStreet", "visual_time_seconds":[6.0,14.0], "video_frames":[VIDEO_TICKS*6,VIDEO_TICKS*6+STAGE_VIDEO_TICKS-1], "combat":"static idle world" },
            "scenarios":scenarios,"stage_life":stages
        }))? + "\n",
    )?;
    println!("Presentation evidence written to {}", output.display());
    Ok(())
}

fn atlas(assets: &GameAssets, character: CharacterId) -> ReviewResult<&SpriteAtlasAsset> {
    match character {
        CharacterId::Rust => assets.rust_fighter.as_ref(),
        CharacterId::Duke => assets.duke_fighter.as_ref(),
        CharacterId::Go => assets.go_fighter.as_ref(),
        CharacterId::C => assets.c_fighter.as_ref(),
        CharacterId::Python => assets.python_fighter.as_ref(),
        CharacterId::Cpp => assets.cpp_fighter.as_ref(),
    }
    .ok_or_else(|| format!("{} actor atlas did not load", character.audio_key()).into())
}

fn export(target: &RenderTexture2D, path: &Path) -> ReviewResult<()> {
    let mut image = target.texture().load_image()?;
    image.flip_vertical();
    fs::write(path, image.export_image_to_memory(".png")?.as_ref())?;
    Ok(())
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
                "-",
                "-an",
                "-c:v",
                "libx264",
                "-preset",
                "fast",
                "-crf",
                "19",
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
