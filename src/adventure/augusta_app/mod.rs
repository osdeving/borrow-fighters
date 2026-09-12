//! Runs the independent C++ chapter using shared production actors and rendering.
//!
//! System: Augusta application boundary. The borrowed window, atomic chapter save,
//! input edges, pause and review recording are owned here, never by combat rules.

mod input;
pub(crate) mod review;

use crate::{
    adventure::{
        app::Options,
        augusta::{
            self, Chapter, CheckpointStage,
            store::{self, Progress},
        },
        campaign::{REGISTRY_PATH, Registry},
        campaign_app::{self as ui},
        chapter_app::CampaignExit,
        engine::{
            capture::{Recorder, export},
            production::{assets::ProductionAssets, audio::ProductionAudio, world::draw_chapter},
        },
    },
    runtime_paths,
};
use raylib::prelude::*;
use std::{error::Error, fs, io::Write, time::Instant};

#[derive(Clone, Copy)]
enum Menu {
    Chapter,
    Pause,
}

pub fn run_in_window(
    rl: &mut RaylibHandle,
    thread: &RaylibThread,
    options: Options,
) -> Result<CampaignExit, Box<dyn Error>> {
    let reviewing = options.review.is_some();
    let output = options.review.as_ref().or(options.capture.as_ref());
    let destination = options
        .review
        .as_ref()
        .map_or_else(store::path, |p| p.join("cpp-augusta-v1.json"));
    let mut progress = if reviewing {
        Progress::default()
    } else {
        store::load(&destination)?
    };
    let content_path = runtime_paths::asset_path(augusta::CONTENT_PATH);
    let registry = Registry::load(&runtime_paths::asset_path(REGISTRY_PATH))?;
    let mut assets = ProductionAssets::load(rl, thread, &content_path)?;
    let device = if options.muted || reviewing {
        None
    } else {
        RaylibAudio::init_audio_device().ok()
    };
    let mut audio = ProductionAudio::new(device.as_ref())?;
    let mut chapter = if let Some(checkpoint) = progress.checkpoint {
        Chapter::from_checkpoint(
            assets.spec.clone(),
            assets.world.clone(),
            assets.texts.clone(),
            assets.catalog.clone(),
            checkpoint,
        )?
    } else {
        Chapter::new(
            assets.spec.clone(),
            assets.world.clone(),
            assets.texts.clone(),
            assets.catalog.clone(),
        )?
    };
    let mut menu =
        (options.start.as_deref() != Some("augusta") && !reviewing).then_some(Menu::Chapter);
    let mut selected = 0;
    let mut target = rl.load_render_texture(thread, 1280, 720)?;
    let mut recorder = output.map(|p| Recorder::new(p)).transpose()?;
    if let Some(output) = output {
        serde_json::to_writer_pretty(
            fs::File::create(output.join("load-report.json"))?,
            &assets.report,
        )?;
    }
    let mut trace = output
        .map(|p| fs::File::create(p.join("telemetry.jsonl")))
        .transpose()?;
    let mut frame = 0_u32;
    let mut complete_frames = 0_u32;
    let mut accumulator = 0.0_f32;
    let mut seconds = 0.0_f64;
    let mut capture_accumulator = 0.0_f32;
    let mut last = Instant::now();
    let mut pending = augusta::ChapterInput::default();
    let mut debug = false;
    let mut notice: Option<(String, u32)> = None;
    let mut saved = progress.checkpoint;
    let mut previous_image = String::new();
    let mut exit = CampaignExit::Closed;
    rl.set_window_title(
        thread,
        &format!("Borrow Fighters — {}", assets.texts.get("chapter.title")),
    );
    rl.set_target_fps(if reviewing { 0 } else { 60 });
    while !rl.window_should_close() {
        let now = Instant::now();
        let dt = if reviewing || frame == 0 {
            1.0 / 60.0
        } else {
            now.duration_since(last).as_secs_f32()
        };
        last = now;
        let controls = ui::menu_input(rl);
        let chapter_input = input::read(rl);
        let mut suppress = frame == 0;
        let mut frame_cues = vec![];
        let mut audio_reset = false;
        if !reviewing && frame > 0 {
            if rl.is_key_pressed(KeyboardKey::KEY_F3) {
                debug = !debug;
            }
            if rl.is_key_pressed(KeyboardKey::KEY_F5) {
                let candidate = (|| -> Result<_, Box<dyn Error>> {
                    let assets = ProductionAssets::load(rl, thread, &content_path)?;
                    let state = Chapter::from_checkpoint(
                        assets.spec.clone(),
                        assets.world.clone(),
                        assets.texts.clone(),
                        assets.catalog.clone(),
                        chapter.checkpoint(),
                    )?;
                    let candidate_audio = ProductionAudio::new(device.as_ref())?;
                    Ok((assets, state, candidate_audio))
                })();
                match candidate {
                    Ok((candidate, state, candidate_audio)) => {
                        assets = candidate;
                        chapter = state;
                        audio = candidate_audio;
                        audio.synchronize(&chapter);
                        audio_reset = true;
                        notice = Some((assets.texts.get("editor.reloaded").into(), 240));
                    }
                    Err(e) => {
                        eprintln!("{e}");
                        notice = Some((assets.texts.get("editor.failed").into(), 240));
                    }
                }
                suppress = true;
            }
            if let Some(active) = menu {
                let has_save = progress.checkpoint.is_some();
                let count = match active {
                    Menu::Pause => 3,
                    Menu::Chapter => {
                        if has_save {
                            3
                        } else {
                            2
                        }
                    }
                };
                ui::navigate(&mut selected, count, &controls);
                if controls.back {
                    match active {
                        Menu::Pause => menu = None,
                        Menu::Chapter => {
                            exit = CampaignExit::Menu;
                            break;
                        }
                    }
                } else if controls.confirm_for(count) {
                    match active {
                        Menu::Pause => match selected {
                            0 => menu = None,
                            1 => {
                                chapter.retry()?;
                                audio.synchronize(&chapter);
                                audio_reset = true;
                                menu = None;
                            }
                            _ => {
                                exit = CampaignExit::Menu;
                                break;
                            }
                        },
                        Menu::Chapter => {
                            if selected == count - 1 {
                                exit = CampaignExit::Menu;
                                break;
                            }
                            if !has_save || selected == 1 {
                                chapter = Chapter::new(
                                    assets.spec.clone(),
                                    assets.world.clone(),
                                    assets.texts.clone(),
                                    assets.catalog.clone(),
                                )?;
                                saved = None;
                                audio.synchronize(&chapter);
                                audio_reset = true;
                            }
                            menu = None;
                        }
                    }
                }
                suppress = true;
            } else if chapter.complete() && controls.confirm {
                exit = CampaignExit::Menu;
                break;
            } else if rl.is_key_pressed(KeyboardKey::KEY_ESCAPE)
                || (rl.is_gamepad_available(0)
                    && rl.is_gamepad_button_pressed(0, GamepadButton::GAMEPAD_BUTTON_MIDDLE_RIGHT))
            {
                menu = Some(Menu::Pause);
                selected = 0;
                suppress = true;
            }
        }
        audio.update(menu.is_some());
        if menu.is_some() || suppress {
            pending = augusta::ChapterInput::default();
            accumulator = 0.0;
        } else {
            input::merge(
                &mut pending,
                if reviewing {
                    review::input(&chapter)
                } else {
                    chapter_input
                },
            );
            accumulator += dt.clamp(0.0, 0.1);
            while accumulator >= 1.0 / 60.0 {
                let synchronizing = pending.skip || pending.retry && chapter.defeated();
                let events = chapter.tick(pending)?;
                assets.advance_animations(chapter.simulation());
                if synchronizing {
                    audio.synchronize(&chapter);
                    audio_reset = true;
                } else {
                    frame_cues.extend(audio.observe(&chapter, &events));
                }
                pending = input::consumed(pending);
                accumulator -= 1.0 / 60.0;
            }
        }
        if menu.is_none() && saved != Some(chapter.checkpoint()) {
            progress.checkpoint = Some(chapter.checkpoint());
            store::save(&destination, &progress)?;
            saved = progress.checkpoint;
        }
        {
            let mut canvas = rl.begin_texture_mode(thread, &mut target);
            draw_chapter(&mut canvas, &assets, &chapter, debug);
            if let Some(active) = menu {
                let rows = match active {
                    Menu::Pause => vec![
                        assets.texts.get("resume"),
                        assets.texts.get("retry"),
                        assets.texts.get("menu"),
                    ],
                    Menu::Chapter if progress.checkpoint.is_some() => vec![
                        registry.continue_label.as_str(),
                        &registry.restart_label,
                        &registry.back_label,
                    ],
                    Menu::Chapter => vec![registry.start_label.as_str(), &registry.back_label],
                };
                let title = match active {
                    Menu::Pause => assets.texts.get("pause"),
                    Menu::Chapter => assets.texts.get("chapter.title"),
                };
                ui::draw_menu(
                    &mut canvas,
                    &assets.font,
                    title,
                    &rows,
                    selected,
                    assets.texts.get("chapter.summary"),
                );
            }
            if let Some((message, remaining)) = &mut notice {
                canvas.draw_rectangle(160, 626, 960, 48, Color::new(17, 35, 32, 250));
                crate::adventure::engine::typography::paragraph(
                    &mut canvas,
                    &assets.font,
                    message,
                    Rectangle::new(180.0, 635.0, 920.0, 34.0),
                    23.0,
                    Color::RAYWHITE,
                );
                *remaining = remaining.saturating_sub(1);
                if *remaining == 0 {
                    notice = None;
                }
            }
        }
        if let Some(recorder) = &mut recorder {
            capture_accumulator += dt;
            let copies = (capture_accumulator * 30.0).floor() as usize;
            capture_accumulator -= copies as f32 / 30.0;
            if copies > 0 {
                recorder.frames(&target, copies)?;
            }
            let label = format!(
                "{:?}-{}-{}",
                chapter.phase(),
                chapter.dialogue().map_or(0, |(_, line)| line),
                augusta::cinema::shot(&chapter).map_or("gameplay", |s| s.name)
            );
            if reviewing && previous_image != label {
                export(
                    &target,
                    &recorder.directory.join(format!("{frame:06}-{label}.png")),
                )?;
                previous_image = label;
            }
        }
        if rl.is_key_pressed(KeyboardKey::KEY_F12) {
            let dir = output
                .cloned()
                .unwrap_or_else(|| runtime_paths::data_dir().join("adventure/screenshots"));
            fs::create_dir_all(&dir)?;
            export(&target, &dir.join(format!("augusta-{frame:06}.png")))?;
        }
        if let Some(trace) = &mut trace {
            serde_json::to_writer(
                &mut *trace,
                &review::telemetry(
                    &chapter,
                    frame,
                    seconds,
                    menu.is_some(),
                    &frame_cues,
                    audio_reset,
                ),
            )?;
            writeln!(trace)?;
        }
        ui::present(rl, thread, &target);
        frame += 1;
        seconds += dt as f64;
        complete_frames = if chapter.complete() {
            complete_frames + 1
        } else {
            0
        };
        if options.max_frames.is_some_and(|limit| frame >= limit) {
            exit = CampaignExit::FrameLimit;
            break;
        }
        if reviewing
            && chapter.complete()
            && chapter.checkpoint().stage == CheckpointStage::Complete
            && complete_frames >= 180
        {
            exit = CampaignExit::FrameLimit;
            break;
        }
    }
    if let Some(recorder) = recorder {
        recorder.finish()?;
    }
    Ok(exit)
}
