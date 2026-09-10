//! Runs the independent adventure window, fixed simulation and review capture.
//!
//! System: Adventure application boundary. This loop owns Raylib, translates
//! physical input and pauses both simulation and audio without touching fighting.

use crate::{
    adventure::{
        combat::{Action, CombatInput, Outcome},
        engine::{assets::Assets, audio::AdventureAudio, render},
        story::{Stage, Story},
    },
    runtime_paths,
};
use raylib::prelude::*;
use std::{
    error::Error,
    fs,
    io::Write,
    path::{Path, PathBuf},
    process::{Child, ChildStdin, Command, Stdio},
};

/// Launch configuration local to the adventure executable.
#[derive(Default)]
struct Options {
    review: Option<PathBuf>,
    capture: Option<PathBuf>,
    start: Option<String>,
    max_frames: Option<u32>,
    muted: bool,
    hidden: bool,
}

impl Options {
    fn parse(args: impl IntoIterator<Item = String>) -> Result<Option<Self>, Box<dyn Error>> {
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
                "--start" => {
                    let stage = args
                        .next()
                        .ok_or("--start requires ada, morning or encounter")?;
                    if !["ada", "morning", "encounter"].contains(&stage.as_str()) {
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
                        "Borrow — primeiras linhas\n\ncargo run --no-default-features --features adventure --bin borrow-adventure\n\n--start ada|morning|encounter  Developer scene entry\n--review DIR                 Deterministic renderer review + MP4\n--capture DIR                Record actual play + frame snapshots\n--frames N                   Exit after N rendered frames\n--mute                       Disable audio device\n--hidden                     Hidden window for isolated review\n\nA/D/arrows move; Space/W jump; J/F attack; K/H strong; Q/L guard.\nEnter skips cinematic scenes; Esc pauses; R retries a lost encounter.\nF3 shows collision; F12 saves a screenshot."
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
}

/// Initializes the optional adventure and runs until window close or an explicit exit.
pub fn run(args: impl IntoIterator<Item = String>) -> Result<(), Box<dyn Error>> {
    let Some(options) = Options::parse(args)? else {
        return Ok(());
    };
    let mut builder = raylib::init();
    builder
        .size(render::WIDTH, render::HEIGHT)
        .title("Borrow — primeiras linhas")
        .msaa_4x()
        .resizable();
    if options.hidden || options.review.is_some() {
        builder.hidden();
    }
    let (mut rl, thread) = builder.build();
    rl.set_exit_key(None);
    if options.review.is_none() {
        rl.set_target_fps(60);
    }
    let audio_device = if options.muted || options.review.is_some() {
        None
    } else {
        RaylibAudio::init_audio_device().ok()
    };
    let mut audio = AdventureAudio::new(audio_device.as_ref());
    let assets = Assets::load(&mut rl, &thread)?;
    let mut target =
        rl.load_render_texture(&thread, render::WIDTH as u32, render::HEIGHT as u32)?;
    let mut story = Story::new();
    match options.start.as_deref() {
        Some("morning") => story.advance_scene(),
        Some("encounter") => {
            story.advance_scene();
            story.advance_scene();
        }
        _ => {}
    }
    let output = options.review.as_ref().or(options.capture.as_ref());
    let mut recorder = output.map(|path| Recorder::new(path)).transpose()?;
    let reviewing = options.review.is_some();
    let mut review = Review::default();
    let mut paused = false;
    let mut debug = false;
    let mut accumulator = 0.0f32;
    let mut pending = CombatInput::default();
    let mut frame = 0u32;
    let mut previous_stage = story.stage;
    let mut events = Vec::new();
    while !rl.window_should_close() {
        let mut suppress = false;
        if !reviewing {
            let has_pad = rl.is_gamepad_available(0);
            let pad_pressed = |button| has_pad && rl.is_gamepad_button_pressed(0, button);
            let confirm = rl.is_key_pressed(KeyboardKey::KEY_ENTER);
            let pause_pressed = rl.is_key_pressed(KeyboardKey::KEY_ESCAPE)
                || pad_pressed(GamepadButton::GAMEPAD_BUTTON_MIDDLE_RIGHT);
            if pause_pressed {
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
            if paused
                && (rl.is_key_pressed(KeyboardKey::KEY_BACKSPACE)
                    || pad_pressed(GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_RIGHT))
            {
                break;
            }
            if !paused && !suppress {
                let retry = rl.is_key_pressed(KeyboardKey::KEY_R)
                    || pad_pressed(GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_DOWN);
                if (story.combat.outcome == Outcome::Defeat || story.stage == Stage::Complete)
                    && retry
                {
                    story.retry();
                    suppress = true;
                } else if story.stage == Stage::Complete
                    && (confirm || pad_pressed(GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_UP))
                {
                    story.restart();
                    suppress = true;
                } else if story.stage != Stage::Encounter
                    && (confirm || pad_pressed(GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_DOWN))
                {
                    story.advance_scene();
                    suppress = true;
                }
            }
            if rl.is_key_pressed(KeyboardKey::KEY_F3) {
                debug = !debug;
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
            let input = input(&rl);
            pending.movement = input.movement;
            pending.blocking = input.blocking;
            pending.jump_pressed |= input.jump_pressed;
            pending.light_pressed |= input.light_pressed;
            pending.heavy_pressed |= input.heavy_pressed;
            accumulator += rl.get_frame_time().clamp(0.0, 0.1);
            let mut steps = 0;
            while accumulator >= 1.0 / 60.0 && steps < 5 {
                story.tick(pending);
                pending.jump_pressed = false;
                pending.light_pressed = false;
                pending.heavy_pressed = false;
                accumulator -= 1.0 / 60.0;
                steps += 1;
            }
            if steps == 5 {
                accumulator = accumulator.min(1.0 / 60.0);
            }
        }
        audio.update(&story, paused);
        if frame == 0 || story.stage != previous_stage {
            events.push(serde_json::json!({"frame":frame,"stage":format!("{:?}",story.stage),"player_hp":story.combat.player.hp,"enemy_hp":story.combat.enemy.hp}));
            previous_stage = story.stage;
        }
        {
            let mut draw = rl.begin_texture_mode(&thread, &mut target);
            render::draw(&mut draw, &story, &assets, paused, debug);
        }
        let screenshot = !reviewing && rl.is_key_pressed(KeyboardKey::KEY_F12);
        if let Some(recorder) = recorder.as_mut() {
            if frame.is_multiple_of(2) {
                recorder.frame(&target)?;
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
            let mut draw = rl.begin_drawing(&thread);
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
        frame += 1;
        if options.max_frames.is_some_and(|limit| frame >= limit) {
            break;
        }
        if reviewing && story.stage == Stage::Complete {
            review.complete_frames += 1;
        }
        if reviewing && (review.complete_frames >= 180 || frame >= 60 * 150) {
            break;
        }
    }
    if let Some(recorder) = recorder {
        let directory = recorder.directory.clone();
        recorder.finish()?;
        fs::write(
            directory.join("result.json"),
            serde_json::to_string_pretty(
                &serde_json::json!({"frames":frame,"mode":if reviewing {"deterministic_review"}else{"physical_input_capture"},"final_stage":format!("{:?}",story.stage),"outcome":format!("{:?}",story.combat.outcome),"player_hp":story.combat.player.hp,"enemy_hp":story.combat.enemy.hp,"events":events,"paused_frames":review.paused_frames,"limitations":"Review uses simulated commands; physical gamepad and subjective animation/audio approval require human playtest."}),
            )?,
        )?;
        if reviewing && story.stage != Stage::Complete {
            return Err(
                "review did not finish through combat and remorse; inspect result.json".into(),
            );
        }
    }
    Ok(())
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

impl Review {
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
            Stage::RustMorning if [90, 320, 470, 700].contains(&story.stage_ticks) => {
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
            Stage::Encounter if story.stage_ticks == 1 => "encounter-explore.png".into(),
            Stage::Aftermath if story.combat.player.action == Action::Remorse => {
                format!("aftermath-{}.png", story.combat.player.action_ticks / 60)
            }
            Stage::Complete => "complete.png".into(),
            _ => return None,
        };
        self.snapshots
            .insert(candidate.clone())
            .then_some(candidate)
    }
}

struct Recorder {
    directory: PathBuf,
    process: Child,
    stdin: Option<ChildStdin>,
    pixels: Vec<u8>,
}

impl Recorder {
    fn new(directory: &Path) -> Result<Self, Box<dyn Error>> {
        fs::create_dir_all(directory)?;
        let mut process = Command::new("ffmpeg")
            .args([
                "-y",
                "-loglevel",
                "error",
                "-f",
                "rawvideo",
                "-pixel_format",
                "rgba",
                "-video_size",
                "1280x720",
                "-framerate",
                "30",
                "-i",
                "-",
                "-vf",
                "vflip",
                "-c:v",
                "libx264",
                "-preset",
                "veryfast",
                "-crf",
                "22",
                "-pix_fmt",
                "yuv420p",
            ])
            .arg(directory.join("adventure-silent.mp4"))
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .spawn()?;
        let stdin = process.stdin.take();
        Ok(Self {
            directory: directory.into(),
            process,
            stdin,
            pixels: Vec::with_capacity((render::WIDTH * render::HEIGHT * 4) as usize),
        })
    }
    fn frame(&mut self, target: &RenderTexture2D) -> Result<(), Box<dyn Error>> {
        let image = target.texture().load_image()?;
        let colors = image.get_image_data();
        self.pixels.clear();
        for color in colors.iter() {
            self.pixels
                .extend_from_slice(&[color.r, color.g, color.b, color.a]);
        }
        self.stdin
            .as_mut()
            .ok_or("recording input closed")?
            .write_all(&self.pixels)?;
        Ok(())
    }
    fn finish(mut self) -> Result<(), Box<dyn Error>> {
        self.stdin.take();
        if !self.process.wait()?.success() {
            return Err("FFmpeg failed while recording adventure".into());
        }
        Ok(())
    }
}

fn export(target: &RenderTexture2D, path: &Path) -> Result<(), Box<dyn Error>> {
    let mut image = target.texture().load_image()?;
    image.flip_vertical();
    image.export_image(&path.to_string_lossy());
    if !path.is_file() {
        return Err(format!("could not export {}", path.display()).into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn renderer_review_policy_wins_through_real_contacts_and_waits_for_remorse() {
        let mut story = Story::new();
        story.advance_scene();
        story.advance_scene();
        let review = Review::default();
        for _ in 0..60 * 90 {
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
