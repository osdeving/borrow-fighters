//! Captures authored super phases and exports deterministic silent review video.
//!
//! System: Development example at the Raylib boundary. The real World drives
//! capture, positions, damage and cues; readback only exports its presentation.
//! JSON cues support an offline mix; --audio separately auditions live playback.

use std::{
    collections::BTreeMap,
    error::Error,
    fs,
    io::Write,
    path::{Path, PathBuf},
    process::{Child, ChildStdin, Command, Stdio},
    time::{Duration, Instant},
};

use borrow_fighters::{
    audio::MusicTrack,
    characters::{CHARACTER_BODY_METRICS_PATH, CharacterBodyMetricsCatalog, CharacterId},
    combat::{
        fighter::FighterInput,
        super_sequence::{
            CPP_FOOTSHOT_TICK, PYTHON_PEACE_START, PYTHON_TARGET_RETURN_TICK,
            RUST_ARENA_COMMIT_TICK, super_spec,
        },
    },
    config::{ARENA_LEFT, ARENA_RIGHT, FIXED_TIMESTEP, WINDOW_HEIGHT, WINDOW_WIDTH, world_px},
    engine::{
        assets::{GameAssets, SpriteAtlasAsset},
        audio::{AUDIO_MANIFEST_PATH, AudioPlayer},
        render::{GamepadStatus, draw_fight},
    },
    game::{
        arena::ArenaId,
        feature_flags::FeatureFlags,
        world::{World, WorldSpriteCombatManifests},
    },
};
use raylib::prelude::*;
use serde_json::{Value, json};

type ReviewResult<T> = Result<T, Box<dyn Error>>;
const PREPARATION: u32 = 60;
const RESTORED_OBSERVATION: u32 = 90;
const ROSTER: [CharacterId; 5] = [
    CharacterId::Rust,
    CharacterId::Duke,
    CharacterId::C,
    CharacterId::Cpp,
    CharacterId::Python,
];

struct Options {
    output: PathBuf,
    character: Option<CharacterId>,
    arena: Option<ArenaId>,
    guard: bool,
    reverse: bool,
    both_sides: bool,
    audio: bool,
    realtime: bool,
    snapshots: bool,
    video: Option<PathBuf>,
}

impl Options {
    fn parse() -> ReviewResult<Self> {
        let mut result = Self {
            output: PathBuf::from("docs/evidence/authored-supers"),
            character: None,
            arena: None,
            guard: false,
            reverse: false,
            both_sides: false,
            audio: false,
            realtime: false,
            snapshots: true,
            video: None,
        };
        let mut args = std::env::args().skip(1);
        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--output" => result.output = args.next().ok_or("--output needs a directory")?.into(),
                "--video" => result.video = Some(args.next().ok_or("--video needs an MP4 path")?.into()),
                "--character" => result.character = Some(match args.next().ok_or("--character needs rust/duke/c/cpp/python")?.as_str() {
                    "rust" => CharacterId::Rust, "duke" | "java" => CharacterId::Duke, "c" => CharacterId::C, "cpp" | "c++" => CharacterId::Cpp, "python" | "py" => CharacterId::Python,
                    _ => return Err("only rust/duke/c/cpp/python have authored supers".into()),
                }),
                "--arena" => result.arena = Some(match args.next().ok_or("--arena needs an arena key")?.as_str() {
                    "sirius" => ArenaId::Sirius, "fortaleza" => ArenaId::Fortaleza,
                    "java-street" | "java" => ArenaId::JavaStreet, "biotic" => ArenaId::BioTic,
                    "porto-digital" | "recife" => ArenaId::PortoDigital,
                    "vale-do-pinhao" | "curitiba" => ArenaId::ValeDoPinhao,
                    _ => return Err("unknown arena; use sirius/fortaleza/java-street/biotic/porto-digital/vale-do-pinhao".into()),
                }),
                "--guard" => result.guard = true,
                "--reverse" => result.reverse = true,
                "--both-sides" => result.both_sides = true,
                "--audio" => { result.audio = true; result.realtime = true; },
                "--realtime" => result.realtime = true,
                "--no-snapshots" => result.snapshots = false,
                _ => return Err(format!("unknown argument {arg}; use --output DIR [--character rust|duke|c|cpp|python] [--arena ARENA] [--guard] [--reverse|--both-sides] [--video PATH] [--audio] [--realtime] [--no-snapshots]").into()),
            }
        }
        if result.audio && result.video.is_some() {
            return Err("--audio is live playback only; --video records deterministic silent frames. Mix the recorded JSON cues offline, or run --audio without --video to inspect live sound.".into());
        }
        Ok(result)
    }
}

fn main() -> ReviewResult<()> {
    let options = Options::parse()?;
    fs::create_dir_all(&options.output)?;
    let (mut raylib, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Borrow Fighters - authored super review")
        .msaa_4x()
        .build();
    // Frame deadlines below are absolute: a per-frame FPS sleep accumulates
    // overshoot and drifts against PulseAudio's continuous wall-clock stream.
    raylib.set_target_fps(0);
    let assets = GameAssets::load(&mut raylib, &thread);
    let metrics = CharacterBodyMetricsCatalog::load(CHARACTER_BODY_METRICS_PATH)?;
    let audio_device = if options.audio {
        Some(
            RaylibAudio::init_audio_device()
                .map_err(|error| format!("audio device unavailable: {error}"))?,
        )
    } else {
        None
    };
    let mut audio = audio_device
        .as_ref()
        .map_or_else(AudioPlayer::disabled, |device| {
            AudioPlayer::load(device, AUDIO_MANIFEST_PATH)
        });
    let mut movie = options
        .video
        .as_ref()
        .map(|path| Movie::new(path))
        .transpose()?;
    let mut target =
        raylib.load_render_texture(&thread, WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32)?;
    let mut global_frame = 0;
    let mut max_frame_lateness_seconds = 0.0_f64;
    let mut scenarios = Vec::new();
    let characters = options
        .character
        .map_or_else(|| ROSTER.to_vec(), |character| vec![character]);
    let sides = if options.both_sides {
        vec![false, true]
    } else {
        vec![options.reverse]
    };
    let render_started = Instant::now();
    for character in characters {
        for &reverse in &sides {
            let spec = super_spec(character).expect("authored character only");
            let opponent = if character == CharacterId::Rust {
                CharacterId::Duke
            } else {
                CharacterId::Rust
            };
            let mut world = World::new_with_character_body_metrics(character, opponent, &metrics);
            world.player_one.position.x = if reverse {
                ARENA_RIGHT - world.player_one.body_rect().width - world_px(60.0)
            } else {
                ARENA_LEFT + world_px(60.0)
            };
            world.player_two.position.x = if reverse {
                ARENA_LEFT + world_px(60.0)
            } else {
                ARENA_RIGHT - world.player_two.body_rect().width - world_px(60.0)
            };
            world.set_sprite_combat_manifests(WorldSpriteCombatManifests {
                player_one: Some(atlas(&assets, character)?.combat_manifest.clone()),
                player_two: Some(atlas(&assets, opponent)?.combat_manifest.clone()),
            });
            let arena = options
                .arena
                .unwrap_or_else(|| ArenaId::home_for_character(character));
            let mut effective_arena = world.effective_arena(arena);
            let mut arena_transitions = Vec::new();
            let mut schedule = BTreeMap::new();
            for phase in spec.phases {
                schedule.insert(
                    phase.start + (phase.end - phase.start) / 2,
                    format!("{:?}", phase.phase),
                );
            }
            for contact in spec.contacts {
                schedule.insert(contact.tick, format!("contact-{}", contact.tick));
            }
            if character == CharacterId::Cpp {
                schedule.insert(CPP_FOOTSHOT_TICK, "footshot".into());
            }
            if character == CharacterId::Python {
                schedule.insert(PYTHON_TARGET_RETURN_TICK, "target-return".into());
                schedule.insert(420, "celebration-hop".into());
                schedule.insert(PYTHON_PEACE_START + 16, "peace".into());
            }
            if character == CharacterId::Rust {
                schedule.insert(35, "build-early".into());
                schedule.insert(RUST_ARENA_COMMIT_TICK - 1, "build-last-block".into());
                schedule.insert(RUST_ARENA_COMMIT_TICK, "arena-commit".into());
            }
            let mut screenshots = Vec::new();
            let mut cues = Vec::new();
            let initial_frame = global_frame;
            let scenario_started = Instant::now();
            audio.set_cinematic_paused(false);
            audio.play_music(music_for_arena(arena));
            println!(
                "REVIEW_START {} reverse={reverse} guard={} frame={global_frame}",
                character.audio_key(),
                options.guard
            );
            for frame in 0..PREPARATION + spec.duration_frames + RESTORED_OBSERVATION {
                if options.realtime {
                    wait_for_frame_deadline(render_started, global_frame);
                    max_frame_lateness_seconds = max_frame_lateness_seconds.max(
                        (render_started.elapsed().as_secs_f64() - global_frame as f64 / 60.0)
                            .max(0.0),
                    );
                }
                if raylib.window_should_close() {
                    return Err("capture interrupted".into());
                }
                world.update(
                    FIXED_TIMESTEP,
                    FighterInput {
                        cinematic_special: frame == PREPARATION,
                        ..FighterInput::default()
                    },
                    FighterInput {
                        block: options.guard && frame == PREPARATION,
                        ..FighterInput::default()
                    },
                );
                if world.effective_arena(arena) != effective_arena {
                    effective_arena = world.effective_arena(arena);
                    arena_transitions.push(json!({"video_frame":global_frame,"arena":format!("{effective_arena:?}"),"music_track":music_for_arena(effective_arena).key()}));
                }
                audio.set_cinematic_paused(world.super_sequence_active());
                audio.play_music(music_for_arena(effective_arena));
                let frame_wall_seconds = render_started.elapsed().as_secs_f64();
                let output_frame = global_frame;
                let events = world.take_audio_events();
                for event in &events {
                    cues.push(json!({"video_frame":output_frame,"simulation_frame":global_frame,"wall_time_seconds":frame_wall_seconds,"sequence_tick":world.super_sequence().map(|sequence|sequence.tick),"cue":format!("{:?}",event.cue),"cue_key":event.cue.key(),"move_id":event.move_id.map(|move_id|move_id.audio_key()),"slot":event.slot.map(|slot|format!("{slot:?}")),"environment":event.environment,"character":event.character.map(|character|character.audio_key())}));
                }
                audio.play_events(events);
                audio.update_streams();
                let snapshot = world.super_sequence();
                let scheduled = snapshot
                    .filter(|_| options.snapshots)
                    .and_then(|sequence| schedule.get(&sequence.tick));
                let ordinary_snapshot = match (options.snapshots, frame) {
                    (true, 30) => Some("before"),
                    (true, frame) if frame == PREPARATION + spec.duration_frames + 40 => {
                        Some("restored")
                    }
                    _ => None,
                };
                if scheduled.is_some()
                    || ordinary_snapshot.is_some()
                    || movie.is_some()
                    || options.realtime
                {
                    {
                        let mut draw = raylib.begin_texture_mode(&thread, &mut target);
                        draw_fight(
                            &mut draw,
                            &world,
                            effective_arena,
                            world.elapsed_seconds,
                            FeatureFlags::default(),
                            GamepadStatus::default(),
                            &assets,
                        );
                    }
                    if let Some(label) = scheduled.map(String::as_str).or(ordinary_snapshot) {
                        let name = format!(
                            "{}-{}-{}-{frame:03}-{label}.png",
                            character.audio_key(),
                            if reverse { "left" } else { "right" },
                            if options.guard { "guard" } else { "hit" }
                        );
                        export(&target, &options.output.join(&name))?;
                        screenshots.push(json!({"image":name,"effective_arena":format!("{effective_arena:?}"),"video_frame":output_frame,"simulation_frame":global_frame,"phase":snapshot.map(|sequence|format!("{:?}",sequence.phase())),"sequence_tick":snapshot.map(|sequence|sequence.tick),"health":[world.player_one.health,world.player_two.health],"actor_x":world.player_one.position.x,"target_x":world.player_two.position.x}));
                    }
                    if let Some(movie) = &mut movie {
                        movie.frame(&target)?;
                    }
                    {
                        let mut draw = raylib.begin_drawing(&thread);
                        draw.draw_texture_pro(
                            target.texture(),
                            Rectangle::new(0.0, 0.0, WINDOW_WIDTH as f32, -(WINDOW_HEIGHT as f32)),
                            Rectangle::new(0.0, 0.0, WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32),
                            Vector2::zero(),
                            0.0,
                            Color::WHITE,
                        );
                    }
                }
                global_frame += 1;
            }
            let expected: i32 = spec
                .contacts
                .iter()
                .map(|contact| {
                    if options.guard {
                        (contact.damage / 4).max(1)
                    } else {
                        contact.damage
                    }
                })
                .sum();
            if world.super_sequence_active()
                || world.player_two.max_health - world.player_two.health != expected
            {
                return Err(format!("incorrect final result for {character:?}/{reverse}").into());
            }
            scenarios.push(json!({"character":character.audio_key(),"move":spec.label,"arena":format!("{arena:?}"),"final_effective_arena":format!("{effective_arena:?}"),"arena_transitions":arena_transitions,"music_track":music_for_arena(arena).key(),"reverse":reverse,"guarded":options.guard,"duration_frames":spec.duration_frames,"simulation_frames":[initial_frame,global_frame-1],"video_frames":[initial_frame,global_frame-1],"wall_time_seconds":scenario_started.elapsed().as_secs_f64(),"damage":expected,"screenshots":screenshots,"audio_events":cues}));
            println!(
                "REVIEW_END {} damage={expected} frame={global_frame}",
                character.audio_key()
            );
        }
    }
    if options.realtime {
        wait_for_frame_deadline(render_started, global_frame);
    }
    let render_wall_seconds = render_started.elapsed().as_secs_f64();
    audio.set_cinematic_paused(false);
    if let Some(movie) = movie {
        movie.finish()?;
    }
    let manifest: Value = json!({"fps":60,"render_size":[WINDOW_WIDTH,WINDOW_HEIGHT],"audio":false,"live_audio_playback":options.audio,"realtime":options.realtime,"video":options.video,"video_frame_count":global_frame,"simulation_frame_count":global_frame,"snapshots":options.snapshots,"render_wall_time_seconds":render_wall_seconds,"max_frame_lateness_seconds":max_frame_lateness_seconds,"video_duration_seconds":global_frame as f64/60.0,"audio_delivery":"silent deterministic video; optional offline mix from the real World cues recorded below","simulation":"World fixed-step; real contacts, physical C++ rush, snapshots at authored phase clocks","scenarios":scenarios});
    fs::write(
        options.output.join("authored-super-review.json"),
        serde_json::to_string_pretty(&manifest)? + "\n",
    )?;
    println!("Authored evidence written to {}", options.output.display());
    Ok(())
}

fn wait_for_frame_deadline(start: Instant, frame: u32) {
    let deadline = start + Duration::from_secs_f64(frame as f64 / 60.0);
    if let Some(remaining) = deadline.checked_duration_since(Instant::now()) {
        std::thread::sleep(remaining);
    }
}

fn music_for_arena(arena: ArenaId) -> MusicTrack {
    match arena {
        ArenaId::Sirius => MusicTrack::Combat,
        ArenaId::JavaStreet => MusicTrack::CombatConsoleFloor,
        ArenaId::PortoDigital => MusicTrack::CombatChiptuneBattle,
        ArenaId::ValeDoPinhao => MusicTrack::CombatEightBitBattle,
        ArenaId::Fortaleza => MusicTrack::CombatRandomEncounter,
        ArenaId::BioTic => MusicTrack::CombatRinsTheme,
    }
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
    .ok_or_else(|| format!("{} atlas unavailable", character.audio_key()).into())
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
