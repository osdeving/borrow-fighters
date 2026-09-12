//! Runs an independent adventure session, fixed simulation and review capture.
//!
//! System: Adventure application boundary. This loop owns Raylib, translates
//! physical input and pauses both simulation and audio without touching fighting.
//! A host may lend its window and receive an explicit completion/exit result.

use crate::{
    adventure::{
        combat::{Action, CombatInput, Outcome},
        engine::{
            assets::Assets,
            audio::AdventureAudio,
            capture::{Recorder, export},
            opening, render,
        },
        story::{Stage, Story},
        text::TextCatalog,
    },
    runtime_paths,
};
use raylib::prelude::*;
use std::{error::Error, fs, io::Write, path::PathBuf};

pub use super::campaign_app::run_in_window as run_campaign_in_window;
pub use super::chapter_app::CampaignExit;

/// Launch configuration local to the adventure executable.
#[derive(Clone, Default)]
pub struct Options {
    pub(super) review: Option<PathBuf>,
    pub(super) capture: Option<PathBuf>,
    pub(super) start: Option<String>,
    pub(super) texts: Option<PathBuf>,
    pub(super) max_frames: Option<u32>,
    pub(super) muted: bool,
    pub(super) hidden: bool,
}

impl Options {
    /// Parses adventure arguments before either standalone or hosted window creation.
    pub fn parse(args: impl IntoIterator<Item = String>) -> Result<Option<Self>, Box<dyn Error>> {
        let mut options = Self::default();
        let mut args = args.into_iter().skip(1);
        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--review" => {
                    options.review = Some(
                        args.next()
                            .ok_or("--review requires an output directory")?
                            .into(),
                    )
                }
                "--capture" => {
                    options.capture = Some(
                        args.next()
                            .ok_or("--capture requires an output directory")?
                            .into(),
                    )
                }
                "--texts" => {
                    options.texts = Some(args.next().ok_or("--texts requires a JSON file")?.into())
                }
                "--start" => {
                    let stage = args.next().ok_or(
                        "--start requires ada, morning, encounter, opening, chapter or augusta",
                    )?;
                    if ![
                        "ada",
                        "morning",
                        "encounter",
                        "opening",
                        "chapter",
                        "augusta",
                    ]
                    .contains(&stage.as_str())
                    {
                        return Err(format!("unknown start scene: {stage}").into());
                    }
                    options.start = Some(stage);
                }
                "--frames" => {
                    options.max_frames = Some(
                        args.next()
                            .ok_or("--frames requires a positive integer")?
                            .parse()?,
                    )
                }
                "--mute" => options.muted = true,
                "--hidden" => options.hidden = true,
                "--help" | "-h" => {
                    println!(
                        "Borrow — primeiras linhas\n\ncargo run --no-default-features --features adventure --bin borrow-adventure\n\n--start ada|morning|encounter|opening|chapter|augusta  Developer scene entry\n--review DIR                 Deterministic renderer review + MP4\n--capture DIR                Record actual play + frame snapshots\n--texts PATH                 Editable UTF-8 JSON catalog\n--frames N                   Exit after N rendered frames\n--mute                       Disable audio device\n--hidden                     Hidden window for isolated review\n\nA/D/arrows move; Space/W jump; J/F attack; K/H strong; V/RT kick; Q/L guard.\nChapter: E/A interacts; Space/B jumps; Enter/RB advances dialogue.\nAugusta: J light; V kick; K spin; L Linker; Q/LB guard.\nOpening: Enter/RB advances one segment; Backspace/View skips to the menu.\nEsc/Start pauses; R retries a lost encounter; F3 shows collision; F12 saves a screenshot."
                    );
                    return Ok(None);
                }
                _ => return Err(format!("unknown adventure argument: {arg}").into()),
            }
        }
        if options.max_frames == Some(0) {
            return Err("--frames must be positive".into());
        }
        if options.review.is_some() && options.capture.is_some() {
            return Err("choose --review or --capture".into());
        }
        Ok(Some(options))
    }

    /// Whether capture review or the explicit flag requests a hidden window.
    pub fn hidden_window(&self) -> bool {
        self.hidden || self.review.is_some()
    }

    /// Explicit developer entry that resumes a chapter checkpoint or starts its intro.
    pub fn chapter() -> Self {
        Self {
            start: Some("chapter".into()),
            ..Self::default()
        }
    }

    /// Explicitly replays Ada and Rust even when the profile has already seen them.
    pub fn prologue() -> Self {
        Self {
            start: Some("ada".into()),
            ..Self::default()
        }
    }

    /// Distinguishes a direct chapter CLI from the hosted campaign submenu.
    pub fn is_chapter(&self) -> bool {
        matches!(self.start.as_deref(), Some("chapter" | "augusta"))
    }
}

/// Reads only the startup decision, keeping save details behind the adventure API.
pub fn prologue_seen() -> Result<bool, Box<dyn Error>> {
    Ok(super::chapter_store::load(&super::chapter_store::path())?.prologue_seen)
}

/// Why a hosted session returned control, without exposing gameplay state.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SessionExit {
    /// The presentation finished and the player confirmed its final prompt.
    Completed,
    /// The player explicitly requested skipping the entire opening.
    Skipped,
    /// The player closed the window or chose to leave the adventure.
    Closed,
    /// A developer frame limit ended this run, before any continuation.
    FrameLimit,
}

/// Initializes the optional adventure and runs until window close or an explicit exit.
pub fn run(args: impl IntoIterator<Item = String>) -> Result<(), Box<dyn Error>> {
    let Some(options) = Options::parse(args)? else {
        return Ok(());
    };
    if options.is_chapter() {
        let mut builder = raylib::init();
        builder
            .size(render::WIDTH, render::HEIGHT)
            .title("Borrow Fighters — Depois do silêncio")
            .msaa_4x()
            .resizable();
        if options.hidden_window() {
            builder.hidden();
        }
        let (mut window, thread) = builder.build();
        window.set_exit_key(None);
        return match run_campaign_in_window(&mut window, &thread, options)? {
            CampaignExit::ReplayPrologue => {
                run_in_window(&mut window, &thread, Options::prologue()).map(|_| ())
            }
            _ => Ok(()),
        };
    }
    let text = TextCatalog::load(
        options
            .texts
            .as_ref()
            .cloned()
            .unwrap_or_else(|| runtime_paths::asset_path("assets/adventure/texts/pt-BR.json")),
    )?;
    let mut builder = raylib::init();
    builder
        .size(render::WIDTH, render::HEIGHT)
        .title(text.get("window.title"))
        .msaa_4x()
        .resizable();
    if options.hidden || options.review.is_some() {
        builder.hidden();
    }
    let (mut rl, thread) = builder.build();
    rl.set_exit_key(None);
    run_session(&mut rl, &thread, options, text, false).map(|_| ())
}

/// Runs in a host-owned window, waits for a fresh final confirmation and fades out.
///
/// All adventure textures, audio and capture resources are released before return.
/// Skipping the entire opening returns immediately with `SessionExit::Skipped`.
/// Closing or truncating a session never reports a completed presentation.
pub fn run_in_window(
    rl: &mut RaylibHandle,
    thread: &RaylibThread,
    options: Options,
) -> Result<SessionExit, Box<dyn Error>> {
    if options.is_chapter() {
        return match run_campaign_in_window(rl, thread, options)? {
            CampaignExit::Menu => Ok(SessionExit::Completed),
            CampaignExit::Closed => Ok(SessionExit::Closed),
            CampaignExit::FrameLimit => Ok(SessionExit::FrameLimit),
            CampaignExit::ReplayPrologue => run_in_window(rl, thread, Options::prologue()),
        };
    }
    let text = TextCatalog::load(
        options
            .texts
            .clone()
            .unwrap_or_else(|| runtime_paths::asset_path("assets/adventure/texts/pt-BR.json")),
    )?;
    rl.set_window_title(thread, text.get("window.title"));
    run_session(rl, thread, options, text, true)
}

fn run_session(
    rl: &mut RaylibHandle,
    thread: &RaylibThread,
    options: Options,
    text: TextCatalog,
    return_on_complete: bool,
) -> Result<SessionExit, Box<dyn Error>> {
    if options.review.is_none() {
        rl.set_target_fps(60);
    }
    let audio_device = if options.muted || options.review.is_some() {
        None
    } else {
        RaylibAudio::init_audio_device().ok()
    };
    let mut audio = AdventureAudio::new(audio_device.as_ref());
    let mut assets = Assets::load(rl, thread, text)?;
    let mut target = rl.load_render_texture(thread, render::WIDTH as u32, render::HEIGHT as u32)?;
    let mut story = Story::new();
    match options.start.as_deref() {
        Some("opening") => story = Story::presentation(),
        Some("morning") => story.advance_scene(),
        Some("encounter") => {
            story.advance_scene();
            story.advance_scene();
        }
        _ => {}
    }
    story.hub_origin = assets.landscape.map.scene("street").hub_origin;
    story.configure_map(assets.landscape.map.prologue.clone());
    let output = options.review.as_ref().or(options.capture.as_ref());
    let mut recorder = output.map(|path| Recorder::new(path)).transpose()?;
    let mut trace = output
        .map(|path| fs::File::create(path.join("telemetry.jsonl")))
        .transpose()?;
    let reviewing = options.review.is_some();
    let mut review = Review::default();
    let mut paused = false;
    let mut debug = false;
    let mut reveal_text = false;
    let mut reload_notice: Option<(bool, f32)> = None;
    let mut text_revision = 0u32;
    let mut accumulator = 0.0f32;
    let mut pending = CombatInput::default();
    let mut frame = 0u32;
    let mut previous_stage = story.stage;
    let mut events = Vec::new();
    let mut capture_accumulator = 0.0f32;
    let mut capture_seconds = 0.0f32;
    let mut previous_frame_time = std::time::Instant::now();
    let mut exit = SessionExit::Closed;
    let mut completion = ContinuePrompt::default();
    while !rl.window_should_close() {
        let now = std::time::Instant::now();
        let frame_time = if frame == 0 {
            1.0 / 60.0
        } else {
            now.duration_since(previous_frame_time).as_secs_f32()
        };
        previous_frame_time = now;
        // A host menu confirmation still belongs to the preceding frame.
        let mut suppress = frame == 0;
        let mut audio_synced_after_skip = false;
        let complete_at_start = story.stage == Stage::Complete;
        // Drain every frame: a press queued during Ada must not dismiss the
        // final prompt minutes later. Held keys are not fresh press edges.
        // The desktop backend queues GLFW_PRESS only, never GLFW_REPEAT. Keep
        // the queue so a quick press/release between frames still counts.
        let mut any_key_pressed = false;
        while rl.get_key_pressed_number().is_some() {
            any_key_pressed = true;
        }
        let any_pad_pressed = rl.get_gamepad_button_pressed().is_some_and(|button| {
            (0..4).any(|pad| {
                rl.is_gamepad_available(pad) && rl.is_gamepad_button_pressed(pad, button)
            })
        });
        let any_mouse_pressed = [
            MouseButton::MOUSE_BUTTON_LEFT,
            MouseButton::MOUSE_BUTTON_RIGHT,
            MouseButton::MOUSE_BUTTON_MIDDLE,
            MouseButton::MOUSE_BUTTON_SIDE,
            MouseButton::MOUSE_BUTTON_EXTRA,
            MouseButton::MOUSE_BUTTON_FORWARD,
            MouseButton::MOUSE_BUTTON_BACK,
        ]
        .into_iter()
        .any(|button| rl.is_mouse_button_pressed(button));
        let continue_pressed =
            complete_at_start && (any_key_pressed || any_pad_pressed || any_mouse_pressed);
        if !reviewing && frame > 0 {
            if rl.is_key_pressed(KeyboardKey::KEY_F5) {
                let result = assets.text.reload();
                let ep_result = story.ep_arrival.reload();
                let biography_result =
                    assets
                        .opening
                        .biographies
                        .reload(rl, thread, &assets.common.text);
                let motion_result = assets.locomotion.reload(rl, thread);
                let ok = result.is_ok()
                    && ep_result.is_ok()
                    && biography_result.is_ok()
                    && motion_result.is_ok();
                if let Err(error) = result {
                    eprintln!("{error}");
                }
                if let Err(error) = ep_result {
                    eprintln!("EP arrival reload: {error}");
                }
                if let Err(error) = biography_result {
                    eprintln!("Biography reload: {error}");
                }
                if let Err(error) = motion_result {
                    eprintln!("Rust animation reload: {error}");
                }
                if ok {
                    text_revision += 1;
                    rl.set_window_title(thread, assets.text.get("window.title"));
                }
                reload_notice = Some((ok, 4.0));
            }
            let has_pad = rl.is_gamepad_available(0);
            let pad_pressed = |button| has_pad && rl.is_gamepad_button_pressed(0, button);
            let skip_all = rl.is_key_pressed(KeyboardKey::KEY_BACKSPACE)
                || pad_pressed(GamepadButton::GAMEPAD_BUTTON_MIDDLE_LEFT);
            if skip_all && !completion.accepted {
                if return_on_complete {
                    exit = SessionExit::Skipped;
                    break;
                }
                story.stage = Stage::Complete;
                story.stage_ticks = 0;
                paused = false;
                suppress = true;
            }
            let confirm = rl.is_key_pressed(KeyboardKey::KEY_ENTER);
            let pause_pressed = rl.is_key_pressed(KeyboardKey::KEY_ESCAPE)
                || pad_pressed(GamepadButton::GAMEPAD_BUTTON_MIDDLE_RIGHT);
            let hosted_end = return_on_complete && complete_at_start;
            if hosted_end {
                // Every fresh button, including Escape, confirms this screen.
                // Pause/replay commands belong to the standalone completion UI.
                paused = false;
                suppress = true;
            } else if pause_pressed {
                if story.stage == Stage::Complete {
                    break;
                }
                paused = !paused;
                suppress = true;
            } else if paused
                && (confirm || pad_pressed(GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_DOWN))
            {
                paused = false;
                suppress = true;
            }
            if paused && pad_pressed(GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_RIGHT) {
                break;
            }
            if !paused && !suppress {
                if story.stage == Stage::Complete
                    && (rl.is_key_pressed(KeyboardKey::KEY_T)
                        || pad_pressed(GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_LEFT))
                {
                    story.replay_presentation();
                    suppress = true;
                }
                let retry = rl.is_key_pressed(KeyboardKey::KEY_R)
                    || pad_pressed(GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_DOWN);
                if (story.stage == Stage::Encounter && story.combat.outcome == Outcome::Defeat
                    || story.stage == Stage::Complete)
                    && retry
                {
                    story.retry();
                    suppress = true;
                } else if story.stage == Stage::Complete
                    && (confirm || pad_pressed(GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_UP))
                {
                    story.restart();
                    suppress = true;
                } else if confirm
                    || pad_pressed(GamepadButton::GAMEPAD_BUTTON_RIGHT_TRIGGER_1)
                    || (story.stage != Stage::Encounter
                        && pad_pressed(GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_DOWN))
                {
                    story.skip_segment();
                    audio.sync_after_skip(&story);
                    audio_synced_after_skip = true;
                    reveal_text = false;
                    suppress = true;
                }
            }
            if rl.is_key_pressed(KeyboardKey::KEY_F3) {
                debug = !debug;
            }
            if !paused
                && story.stage == Stage::AdaPrologue
                && (rl.is_key_pressed(KeyboardKey::KEY_TAB)
                    || pad_pressed(GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_LEFT))
            {
                reveal_text = true;
            }
            if suppress || paused {
                pending = CombatInput::default();
                accumulator = 0.0;
            }
        }
        if reviewing {
            paused = review.should_pause(&story);
            if !paused {
                story.tick(review.input(&story));
            }
            review.frames += 1;
        } else if !paused && !suppress {
            let input = input(rl);
            pending.movement = input.movement;
            pending.blocking = input.blocking;
            pending.jump_pressed |= input.jump_pressed;
            pending.light_pressed |= input.light_pressed;
            pending.heavy_pressed |= input.heavy_pressed;
            pending.kick_pressed |= input.kick_pressed;
            accumulator += frame_time.clamp(0.0, 0.1);
            let mut steps = 0;
            while accumulator >= 1.0 / 60.0 && steps < 5 {
                story.tick(pending);
                pending.jump_pressed = false;
                pending.light_pressed = false;
                pending.heavy_pressed = false;
                pending.kick_pressed = false;
                accumulator -= 1.0 / 60.0;
                steps += 1;
            }
            if steps == 5 {
                accumulator = accumulator.min(1.0 / 60.0);
            }
        }
        if return_on_complete {
            completion.tick(story.stage == Stage::Complete, continue_pressed, reviewing);
        }
        audio.update(&story, paused);
        capture_seconds += if reviewing { 1.0 / 60.0 } else { frame_time };
        if frame == 0 || story.stage != previous_stage {
            reveal_text = false;
            events.push(serde_json::json!({"frame":frame,"seconds":capture_seconds,"wall_seconds":std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).ok().map(|time| time.as_secs_f64()),"stage":format!("{:?}",story.stage),"player_hp":story.combat.player.hp,"enemy_hp":story.combat.enemy.hp}));
            previous_stage = story.stage;
        }
        if let Some(trace) = trace.as_mut() {
            let c = &story.combat;
            let ep_sample = story
                .ep_arrival
                .sample(c.enemy.position.x - story.camera_left());
            let arrival_camera = if story.arrival_active() {
                story.initial_shot()
            } else if story.ep_arrival_active() {
                ep_sample.map_or_else(crate::adventure::arrival::ArrivalShot::settled, |sample| {
                    sample.shot
                })
            } else {
                crate::adventure::arrival::ArrivalShot::settled()
            };
            let ep_arrival = serde_json::json!({
                "active":story.ep_arrival_active(),"ticks":story.ep_arrival.ticks(),
                "impact_tick":story.ep_arrival.spec.impact_tick(),
                "gameplay_tick":story.ep_arrival.spec.gameplay_tick,
                "feet_y":ep_sample.map(|sample|sample.feet_y),
                "pose":ep_sample.map(|sample|sample.pose),
                "impact_age":ep_sample.and_then(|sample|sample.impact_age)
            });
            let neighborhood = crate::adventure::neighborhood::Neighborhood::sample(&story.ambient);
            let neighbors = serde_json::json!({
                "residents": neighborhood.residents.map(|actor|serde_json::json!({
                    "id":actor.id,"x":actor.position.x,"y":actor.position.y,
                    "phase":format!("{:?}",actor.phase),"phase_ticks":actor.phase_ticks,
                    "visible":actor.visible,"facing":format!("{:?}",actor.facing)
                })),
                "dog":{"x":neighborhood.dog.position.x,"y":neighborhood.dog.position.y,
                    "phase":format!("{:?}",neighborhood.dog.phase),"phase_ticks":neighborhood.dog.phase_ticks,
                    "visible":neighborhood.dog.visible,"facing":format!("{:?}",neighborhood.dog.facing)},
                "shutter":{"phase":format!("{:?}",neighborhood.shutter.phase),
                    "phase_ticks":neighborhood.shutter.phase_ticks,"progress":neighborhood.shutter.progress}
            });
            let ambience = serde_json::json!({
                "ticks": story.ambient.ticks(),
                "kid_phase": format!("{:?}", story.ambient.kid_phase()),
                "kid_phase_ticks": story.ambient.kid_phase_ticks(),
                "kid_position": {"x": story.ambient.kid_position().x, "y": story.ambient.kid_position().y},
                "kite_release_ticks": story.ambient.kite_release_ticks(),
                "cyclists": story.ambient.cyclists().map(|actor| serde_json::json!({
                    "x": actor.position.x, "y": actor.position.y,
                    "facing": format!("{:?}", actor.facing), "animation_ticks": actor.animation_ticks,
                    "id": actor.id, "phase": format!("{:?}", actor.phase),
                    "phase_ticks": actor.phase_ticks, "visible": actor.visible,
                    "bicycle_position": {"x":actor.bicycle_position.x,"y":actor.bicycle_position.y}
                })),
                "abandoned_bicycles": story.ambient.abandoned_bicycles().map(|bicycle| bicycle.map(|bike| serde_json::json!({
                    "id": bike.id, "x": bike.position.x, "y": bike.position.y,
                    "facing": format!("{:?}",bike.facing), "drop_ticks": bike.drop_ticks
                }))),
                "accident_ticks": story.ambient.accident_ticks(),
                "traffic_cars": story.ambient.traffic_cars().map(|car| serde_json::json!({
                    "x": car.position.x, "y": car.position.y, "style": car.style,
                    "facing": format!("{:?}", car.facing), "animation_ticks": car.animation_ticks,
                    "visible": car.visible, "fleeing": car.fleeing
                })),
                "incident_car": story.ambient.incident_car().map(|car| serde_json::json!({
                    "x": car.position.x, "y": car.position.y,
                    "phase": format!("{:?}", car.phase), "phase_ticks": car.phase_ticks
                }))
            });
            writeln!(
                trace,
                "{}",
                serde_json::json!({"frame":frame,"seconds":capture_seconds,"wall_seconds":std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).ok().map(|time| time.as_secs_f64()),"stage":format!("{:?}",story.stage),"stage_ticks":story.stage_ticks,"paused":paused,"waiting_for_continue":return_on_complete && complete_at_start && !completion.accepted,"continue_accepted":completion.accepted,"text_revision":text_revision,"text_reload_ok":reload_notice.map(|v|v.0),"ticks":c.ticks,"enemy_awake":c.enemy_awake,"ambience":ambience,"neighborhood":neighbors,"ep_arrival":ep_arrival,"world":{"width":story.map.width,"camera_left":story.camera_left(),"arrival_x":story.map.arrival_x,"hub_origin":assets.landscape.map.scene("street").hub_origin},"arrival_active":story.arrival_active(),"arrival_camera":{"x":arrival_camera.target.x,"y":arrival_camera.target.y,"zoom":arrival_camera.zoom},"audio_synced_after_skip":audio_synced_after_skip,"outcome":format!("{:?}",c.outcome),"player":{"x":c.player.position.x,"y":c.player.position.y,"hp":c.player.hp,"action":format!("{:?}",c.player.action),"facing":format!("{:?}",c.player.facing)},"enemy":{"x":c.enemy.position.x,"y":c.enemy.position.y,"hp":c.enemy.hp,"action":format!("{:?}",c.enemy.action)},"hit":c.last_hit.map(|h| serde_json::json!({"target":format!("{:?}",h.target),"age":h.age_ticks,"blocked":h.blocked}))})
            )?;
        }
        {
            let mut draw = rl.begin_texture_mode(thread, &mut target);
            if return_on_complete && story.stage == Stage::Complete {
                opening::draw(&mut draw, &story, &assets);
                draw.draw_rectangle(
                    0,
                    0,
                    render::WIDTH,
                    render::HEIGHT,
                    Color::new(
                        14,
                        19,
                        26,
                        (255 * completion.fade_frames.min(24) / 24) as u8,
                    ),
                );
            } else {
                render::draw(&mut draw, &story, &assets, paused, debug, reveal_text);
            }
            if !completion.accepted {
                render::navigation(
                    &mut draw,
                    &assets,
                    &story,
                    return_on_complete,
                    paused,
                    capture_seconds,
                );
            }
            if let Some((ok, _)) = reload_notice {
                render::reload_notice(&mut draw, &assets, ok);
            }
        }
        let screenshot = !reviewing && rl.is_key_pressed(KeyboardKey::KEY_F12);
        if let Some(recorder) = recorder.as_mut() {
            if reviewing {
                if frame.is_multiple_of(2) {
                    recorder.frame(&target)?;
                }
            } else {
                capture_accumulator += frame_time;
                let copies = (capture_accumulator * 30.0) as usize;
                if copies > 0 {
                    recorder.frames(&target, copies)?;
                    capture_accumulator -= copies as f32 / 30.0;
                }
            }
            if reviewing && let Some(name) = review.snapshot(&story, paused) {
                export(&target, &recorder.directory.join(name))?;
            }
        }
        if screenshot {
            let dir = runtime_paths::capture_dir().join("adventure");
            fs::create_dir_all(&dir)?;
            export(&target, &dir.join(format!("adventure-{frame:06}.png")))?;
        }
        {
            let width = rl.get_screen_width() as f32;
            let height = rl.get_screen_height() as f32;
            let scale = (width / render::WIDTH as f32).min(height / render::HEIGHT as f32);
            let mut draw = rl.begin_drawing(thread);
            draw.clear_background(Color::BLACK);
            draw.draw_texture_pro(
                target.texture(),
                Rectangle::new(0.0, 0.0, render::WIDTH as f32, -render::HEIGHT as f32),
                Rectangle::new(
                    (width - render::WIDTH as f32 * scale) * 0.5,
                    (height - render::HEIGHT as f32 * scale) * 0.5,
                    render::WIDTH as f32 * scale,
                    render::HEIGHT as f32 * scale,
                ),
                Vector2::zero(),
                0.0,
                Color::WHITE,
            );
        }
        if let Some((_, remaining)) = reload_notice.as_mut() {
            *remaining -= frame_time;
            if *remaining <= 0.0 {
                reload_notice = None;
            }
        }
        frame += 1;
        if options.max_frames.is_some_and(|limit| frame >= limit) {
            exit = SessionExit::FrameLimit;
            break;
        }
        if return_on_complete && completion.fade_frames >= 24 {
            exit = SessionExit::Completed;
            break;
        }
        if reviewing && story.stage == Stage::Complete {
            review.complete_frames += 1;
        }
        if reviewing && review.capture_finished(return_on_complete) {
            break;
        }
    }
    if let Some(recorder) = recorder {
        let directory = recorder.directory.clone();
        recorder.finish()?;
        fs::write(
            directory.join("result.json"),
            serde_json::to_string_pretty(
                &serde_json::json!({"frames":frame,"mode":if reviewing {"deterministic_review"}else{"window_input_capture"},"exit_reason":format!("{:?}",exit),"final_stage":format!("{:?}",story.stage),"outcome":format!("{:?}",story.combat.outcome),"player_hp":story.combat.player.hp,"enemy_hp":story.combat.enemy.hp,"events":events,"paused_frames":review.paused_frames,"limitations":"Review uses simulated commands; physical gamepad and subjective animation/audio approval require human playtest."}),
            )?,
        )?;
        if reviewing && story.stage != Stage::Complete {
            return Err(
                "review did not finish through combat and remorse; inspect result.json".into(),
            );
        }
    }
    if !reviewing && records_prologue(exit) {
        super::chapter_store::remember_prologue(
            story.combat.outcome == Outcome::Victory,
            story.ambient.reaction_origin(),
        )?;
    }
    Ok(exit)
}

fn records_prologue(exit: SessionExit) -> bool {
    matches!(exit, SessionExit::Completed | SessionExit::Skipped)
}

#[derive(Default)]
struct ContinuePrompt {
    shown: bool,
    accepted: bool,
    held_frames: u32,
    fade_frames: u32,
}

impl ContinuePrompt {
    fn tick(&mut self, complete: bool, fresh_press: bool, automated_review: bool) {
        if !complete {
            *self = Self::default();
            return;
        }
        if self.shown && (fresh_press || automated_review && self.held_frames >= 180) {
            self.accepted = true;
        }
        self.shown = true;
        self.held_frames = self.held_frames.saturating_add(1);
        if self.accepted {
            self.fade_frames += 1;
        }
    }
}

fn input(rl: &RaylibHandle) -> CombatInput {
    let pad = rl.is_gamepad_available(0);
    let down = |button| pad && rl.is_gamepad_button_down(0, button);
    let pressed = |button| pad && rl.is_gamepad_button_pressed(0, button);
    let left = rl.is_key_down(KeyboardKey::KEY_A)
        || rl.is_key_down(KeyboardKey::KEY_LEFT)
        || down(GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_LEFT);
    let right = rl.is_key_down(KeyboardKey::KEY_D)
        || rl.is_key_down(KeyboardKey::KEY_RIGHT)
        || down(GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_RIGHT);
    let axis = if pad {
        rl.get_gamepad_axis_movement(0, GamepadAxis::GAMEPAD_AXIS_LEFT_X)
    } else {
        0.0
    };
    CombatInput {
        movement: if left || right {
            right as u8 as f32 - left as u8 as f32
        } else if axis.abs() > 0.2 {
            axis
        } else {
            0.0
        },
        jump_pressed: rl.is_key_pressed(KeyboardKey::KEY_SPACE)
            || rl.is_key_pressed(KeyboardKey::KEY_W)
            || pressed(GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_DOWN),
        light_pressed: rl.is_key_pressed(KeyboardKey::KEY_J)
            || rl.is_key_pressed(KeyboardKey::KEY_F)
            || pressed(GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_LEFT),
        kick_pressed: rl.is_key_pressed(KeyboardKey::KEY_V)
            || pressed(GamepadButton::GAMEPAD_BUTTON_RIGHT_TRIGGER_2),
        heavy_pressed: rl.is_key_pressed(KeyboardKey::KEY_K)
            || rl.is_key_pressed(KeyboardKey::KEY_H)
            || pressed(GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_UP),
        blocking: rl.is_key_down(KeyboardKey::KEY_Q)
            || rl.is_key_down(KeyboardKey::KEY_L)
            || down(GamepadButton::GAMEPAD_BUTTON_LEFT_TRIGGER_1),
    }
}

#[derive(Default)]
struct Review {
    frames: u32,
    paused_frames: u32,
    complete_frames: u32,
    snapshots: std::collections::BTreeSet<String>,
}

// Leave room for real combat/recovery independently from authored scene lengths.
const REVIEW_FRAME_LIMIT: u32 = super::story::PROLOGUE_TICKS
    + super::story::MORNING_TICKS
    + super::story::OPENING_TICKS
    + 60 * 90;

impl Review {
    fn capture_finished(&self, hosted: bool) -> bool {
        // The host waits for ContinuePrompt's simulated press and full fade.
        // Only the standalone capture ends as soon as its final hold is over.
        (!hosted && self.complete_frames >= 180) || self.frames >= REVIEW_FRAME_LIMIT
    }

    fn should_pause(&mut self, story: &Story) -> bool {
        if story.stage == Stage::Encounter && story.combat.enemy_awake && self.paused_frames < 30 {
            self.paused_frames += 1;
            true
        } else {
            false
        }
    }

    fn input(&self, story: &Story) -> CombatInput {
        if story.stage != Stage::Encounter {
            return CombatInput::default();
        }
        let c = &story.combat;
        let dx = c.enemy.position.x - c.player.position.x;
        let distance = dx.abs();
        let imminent = matches!(c.enemy.action, Action::Telegraph | Action::Lunge);
        CombatInput {
            movement: if distance > 95.0 || c.player.facing.sign() != dx.signum() {
                dx.signum()
            } else {
                0.0
            },
            blocking: imminent && distance < 170.0,
            heavy_pressed: !imminent && distance <= 112.0 && c.ticks.is_multiple_of(45),
            light_pressed: !imminent && distance < 95.0 && c.ticks.is_multiple_of(27),
            ..CombatInput::default()
        }
    }

    fn snapshot(&mut self, story: &Story, paused: bool) -> Option<String> {
        let candidate = match story.stage {
            Stage::AdaPrologue if story.beat_ticks() == 180 => {
                format!("ada-{:?}.png", story.prologue_beat()?)
            }
            Stage::RustMorning
                if [90, 180, 250, 320, 390, 470, 545, 650, 700].contains(&story.stage_ticks) =>
            {
                format!("morning-{}.png", story.stage_ticks)
            }
            Stage::Encounter if paused => "encounter-pause.png".into(),
            Stage::Encounter if story.combat.last_hit.is_some_and(|hit| hit.age_ticks <= 1) => {
                let hit = story.combat.last_hit?;
                if hit.blocked {
                    "encounter-block.png".into()
                } else {
                    format!("encounter-hit-{:?}.png", hit.target)
                }
            }
            Stage::Encounter if story.combat.enemy.action == Action::Telegraph => {
                "encounter-warning.png".into()
            }
            Stage::Encounter if story.arrival_active() && story.stage_ticks == 1 => {
                "encounter-arrival-kite.png".into()
            }
            Stage::Encounter
                if story.ep_arrival_active()
                    && story.ep_arrival.ticks().is_some_and(|ticks| {
                        [10, 20, 30, 40, 57, 61, 78, 110].contains(&ticks)
                    }) =>
            {
                format!("encounter-ep-arrival-{}.png", story.ep_arrival.ticks()?)
            }
            Stage::Encounter if story.arrival_active() && story.stage_ticks == 570 => {
                "encounter-arrival-descent.png".into()
            }
            Stage::Encounter if story.stage_ticks == crate::adventure::arrival::ARRIVAL_TICKS => {
                "encounter-explore.png".into()
            }
            Stage::Aftermath if story.combat.player.action == Action::Remorse => {
                format!("aftermath-{}.png", story.combat.player.action_ticks / 60)
            }
            Stage::Opening if story.stage_ticks % 60 == 30 => {
                format!("opening-{:02}.png", story.stage_ticks / 60)
            }
            Stage::Complete => "complete.png".into(),
            _ => return None,
        };
        self.snapshots
            .insert(candidate.clone())
            .then_some(candidate)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn final_prompt_ignores_earlier_input_and_waits_for_a_new_press_before_fading() {
        let mut prompt = ContinuePrompt::default();
        prompt.tick(false, true, false);
        prompt.tick(true, true, false);
        assert!(!prompt.accepted);
        assert_eq!(prompt.fade_frames, 0);

        // This also models continuing to hold the final skip key: no new edge.
        for _ in 0..600 {
            prompt.tick(true, false, false);
        }
        assert!(!prompt.accepted);
        assert_eq!(prompt.fade_frames, 0);

        prompt.tick(true, true, false);
        assert!(prompt.accepted);
        assert_eq!(prompt.fade_frames, 1);
        for _ in 1..24 {
            prompt.tick(true, false, false);
        }
        assert!(prompt.accepted);
        assert_eq!(prompt.fade_frames, 24);
    }

    #[test]
    fn leaving_completion_discards_its_confirmation_and_partial_fade() {
        let mut prompt = ContinuePrompt::default();
        prompt.tick(true, false, false);
        prompt.tick(true, true, false);
        assert!(prompt.accepted);

        prompt.tick(false, true, false);
        assert!(!prompt.shown);
        assert!(!prompt.accepted);
        assert_eq!(prompt.held_frames, 0);
        assert_eq!(prompt.fade_frames, 0);

        prompt.tick(true, true, false);
        assert!(!prompt.accepted);
        assert_eq!(prompt.fade_frames, 0);
    }

    #[test]
    fn hosted_review_survives_the_final_hold_to_confirm_and_finish_its_fade() {
        let mut prompt = ContinuePrompt::default();
        let mut review = Review::default();
        for _ in 0..180 {
            prompt.tick(true, false, true);
            review.frames += 1;
            review.complete_frames += 1;
            assert!(!prompt.accepted);
            assert!(!review.capture_finished(true));
        }
        assert!(review.capture_finished(false));
        for _ in 0..24 {
            prompt.tick(true, false, true);
            review.frames += 1;
            review.complete_frames += 1;
            assert!(prompt.accepted);
            assert!(!review.capture_finished(true));
        }
        assert_eq!(prompt.fade_frames, 24);
    }

    #[test]
    fn review_keeps_a_time_limit_for_incomplete_runs_in_both_entry_points() {
        let mut review = Review {
            frames: REVIEW_FRAME_LIMIT - 1,
            ..Review::default()
        };
        assert!(!review.capture_finished(false));
        assert!(!review.capture_finished(true));
        review.frames += 1;
        assert!(review.capture_finished(false));
        assert!(review.capture_finished(true));
    }

    #[test]
    fn cli_rejects_invalid_or_ambiguous_capture_requests() {
        for args in [
            vec!["game", "--start", "fight"],
            vec!["game", "--frames", "0"],
            vec!["game", "--review"],
            vec!["game", "--capture", "a", "--review", "b"],
        ] {
            assert!(Options::parse(args.into_iter().map(str::to_owned)).is_err());
        }
    }

    #[test]
    fn chapter_entry_preserves_capture_flags_and_does_not_replace_the_campaign_menu() {
        let options = Options::parse(
            [
                "game",
                "--start",
                "chapter",
                "--capture",
                "chapter-capture",
                "--frames",
                "120",
                "--mute",
                "--hidden",
            ]
            .map(str::to_owned),
        )
        .unwrap()
        .unwrap();
        assert!(options.is_chapter());
        assert_eq!(options.capture, Some(PathBuf::from("chapter-capture")));
        assert_eq!(options.max_frames, Some(120));
        assert!(options.muted && options.hidden_window());
        assert!(
            !Options::default().is_chapter(),
            "a normal campaign request must open its submenu"
        );
        assert!(Options::chapter().is_chapter());
        let augusta = Options::parse(["game", "--start", "augusta", "--hidden"].map(str::to_owned))
            .unwrap()
            .unwrap();
        assert!(augusta.is_chapter() && augusta.hidden_window());
        assert_eq!(augusta.start.as_deref(), Some("augusta"));
        assert_eq!(Options::prologue().start.as_deref(), Some("ada"));
    }

    #[test]
    fn only_finished_or_explicitly_skipped_prologues_update_the_profile() {
        assert!(records_prologue(SessionExit::Completed));
        assert!(records_prologue(SessionExit::Skipped));
        assert!(!records_prologue(SessionExit::Closed));
        assert!(!records_prologue(SessionExit::FrameLimit));
        let mut story = Story::new();
        story.advance_scene();
        story.advance_scene();
        story.skip_segment(); // Settle the arrival.
        story.skip_segment(); // Skip the encounter, preserving its real outcome.
        assert_eq!(story.stage, Stage::Opening);
        assert_eq!(story.combat.outcome, Outcome::Ongoing);
        assert_eq!(story.ambient.reaction_origin(), None);
    }

    #[test]
    fn renderer_review_policy_wins_through_real_contacts_and_waits_for_remorse() {
        let mut story = Story::new();
        story.advance_scene();
        story.advance_scene();
        let review = Review::default();
        for _ in 0..super::super::story::OPENING_TICKS + 60 * 70 {
            story.tick(review.input(&story));
            if story.stage == Stage::Complete {
                break;
            }
        }
        assert_eq!(story.stage, Stage::Complete);
        assert_eq!(story.combat.outcome, Outcome::Victory);
        assert_eq!(story.combat.enemy.hp, 0);
        assert!(story.combat.player.hp > 0);
    }
}
