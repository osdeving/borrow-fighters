//! Runs the resumable chapter in a borrowed window with fixed-step simulation.
//!
//! System: Adventure application boundary. This module owns saves, menus and
//! device lifetimes; chapter geometry, acting and combat remain pure modules.

mod input;
mod review;

use crate::{
    adventure::{
        app::Options,
        chapter::{Chapter, ChapterInput, CheckpointStage, Phase},
        chapter_store::{self, CampaignProgress},
        engine::{
            capture::{Recorder, export},
            chapter::{
                self as render,
                assets::{ChapterAssets, load_world},
            },
            chapter_audio::ChapterAudio,
        },
    },
    runtime_paths,
};
use raylib::prelude::*;
use std::{error::Error, fs, time::Instant};

/// Why the chapter hands its borrowed window back to the application host.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CampaignExit {
    Menu,
    ReplayPrologue,
    Closed,
    FrameLimit,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Menu {
    Story,
    Pause,
}

/// Runs a chapter or its small save-aware selection screen in the existing window.
pub fn run_in_window(
    rl: &mut RaylibHandle,
    thread: &RaylibThread,
    options: Options,
) -> Result<CampaignExit, Box<dyn Error>> {
    let destination = save_path(&options);
    let mut progress = if options.review.is_some() {
        CampaignProgress::default()
    } else {
        chapter_store::load(&destination)?
    };
    let world = load_world()?;
    let mut chapter = if let Some(checkpoint) = progress.checkpoint {
        Chapter::from_checkpoint(world.clone(), checkpoint)?
    } else {
        Chapter::new(world.clone(), progress.prologue_played_victory)
    };
    let direct = options.start.as_deref() == Some("chapter");
    let reviewing = options.review.is_some();
    let mut menu = (!direct).then_some(Menu::Story);
    let mut selected = 0;
    let mut assets = ChapterAssets::load(rl, thread)?;
    let mut target = rl.load_render_texture(thread, 1280, 720)?;
    let device = if options.muted || reviewing {
        None
    } else {
        RaylibAudio::init_audio_device().ok()
    };
    let mut audio = ChapterAudio::new(device.as_ref());
    if menu.is_some() {
        audio.update(&chapter, true);
    }
    audio.synchronize(&chapter);
    let output = options.review.as_ref().or(options.capture.as_ref());
    let mut recorder = output.map(|path| Recorder::new(path)).transpose()?;
    let mut trace = output
        .map(|path| fs::File::create(path.join("telemetry.jsonl")))
        .transpose()?;
    let mut frame = 0_u32;
    let mut accumulator = 0.0_f32;
    let mut capture_accumulator = 0.0_f32;
    let mut seconds = 0.0_f32;
    let mut last_frame = Instant::now();
    let mut pending = ChapterInput::default();
    let mut debug = false;
    let mut notice: Option<(String, u32)> = None;
    let mut previous_image = String::new();
    let mut exit = CampaignExit::Closed;
    let mut saved_checkpoint = progress.checkpoint;
    rl.set_exit_key(None);
    rl.set_window_title(thread, "Borrow Fighters — Depois do silêncio");
    rl.set_target_fps(if reviewing { 0 } else { 60 });
    while !rl.window_should_close() {
        let wall_seconds = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs_f64();
        let now = Instant::now();
        let dt = if frame == 0 || reviewing {
            1.0 / 60.0
        } else {
            now.duration_since(last_frame).as_secs_f32()
        };
        last_frame = now;
        let controls = input::read(rl);
        while rl.get_key_pressed_number().is_some() {}
        let mut suppress = frame == 0;
        if !reviewing && frame > 0 {
            if rl.is_key_pressed(KeyboardKey::KEY_F3) {
                debug = !debug;
            }
            if rl.is_key_pressed(KeyboardKey::KEY_F5) {
                let copy_result = assets.reload_copy();
                let motion_result = assets.common.locomotion.reload(rl, thread);
                if motion_result.is_ok() {
                    audio.reload_motion(&assets.common.locomotion.motion, &chapter);
                }
                notice = Some((
                    match copy_result.and(motion_result) {
                        Ok(()) => assets.common.text.get("editor.reloaded").into(),
                        Err(e) => {
                            eprintln!("{e}");
                            assets.common.text.get("editor.failed").into()
                        }
                    },
                    240,
                ));
            }
            if let Some(active_menu) = menu {
                let continuing = can_continue(&progress);
                let count = if active_menu == Menu::Story {
                    if continuing { 4 } else { 3 }
                } else {
                    3
                };
                let hovered = controls.pointer.and_then(|point| {
                    (0..count).find(|index| {
                        Rectangle::new(390.0, 246.0 + *index as f32 * 65.0, 500.0, 52.0)
                            .check_collision_point_rec(point)
                    })
                });
                if (controls.pointer_moved || controls.click)
                    && let Some(row) = hovered
                {
                    selected = row;
                }
                if controls.up {
                    selected = (selected + count - 1) % count;
                }
                if controls.down {
                    selected = (selected + 1) % count;
                }
                if controls.back {
                    if active_menu == Menu::Story {
                        exit = CampaignExit::Menu;
                        break;
                    }
                    menu = None;
                } else if controls.confirm || controls.click && hovered.is_some() {
                    if active_menu == Menu::Pause {
                        match selected {
                            0 => menu = None,
                            1 => {
                                chapter =
                                    Chapter::from_checkpoint(world.clone(), chapter.checkpoint())?;
                                menu = None;
                                audio.synchronize(&chapter);
                            }
                            _ => {
                                exit = CampaignExit::Menu;
                                break;
                            }
                        }
                    } else {
                        let action = selected + usize::from(!continuing);
                        match action {
                            0 => {
                                chapter = Chapter::from_checkpoint(
                                    world.clone(),
                                    progress.checkpoint.expect("continue is available"),
                                )?;
                                menu = None;
                                audio.synchronize(&chapter);
                            }
                            1 => {
                                chapter =
                                    Chapter::new(world.clone(), progress.prologue_played_victory);
                                menu = None;
                                audio.synchronize(&chapter);
                                saved_checkpoint = None;
                            }
                            2 => {
                                exit = CampaignExit::ReplayPrologue;
                                break;
                            }
                            _ => {
                                exit = CampaignExit::Menu;
                                break;
                            }
                        }
                    }
                }
                suppress = true;
            } else if chapter.phase == Phase::Complete && controls.confirm {
                exit = CampaignExit::Menu;
                break;
            } else if controls.pause {
                menu = Some(Menu::Pause);
                selected = 0;
                suppress = true;
            }
        }
        if menu.is_some() || suppress {
            pending = ChapterInput::default();
            accumulator = 0.0;
        } else {
            input::merge(
                &mut pending,
                if reviewing {
                    review::input(&chapter)
                } else {
                    controls.chapter
                },
            );
            accumulator += dt.clamp(0.0, 0.1);
            while accumulator >= 1.0 / 60.0 {
                let old_phase = chapter.phase;
                let old_ticks = chapter.phase_ticks;
                let synchronizing = pending.skip
                    || (pending.retry
                        && chapter.combat.outcome == crate::adventure::combat::Outcome::Defeat)
                    || pending.advance && chapter.active_dialogue().is_none();
                chapter.tick(pending);
                if synchronizing && (old_phase != chapter.phase || chapter.phase_ticks < old_ticks)
                {
                    audio.synchronize(&chapter);
                }
                // Observe every simulation update so batching never loses a cue.
                audio.update(&chapter, false);
                pending = input::consumed(pending);
                accumulator -= 1.0 / 60.0;
            }
        }
        audio.update(&chapter, menu.is_some());
        if menu.is_none() && saved_checkpoint != Some(chapter.checkpoint()) {
            progress.checkpoint = Some(chapter.checkpoint());
            progress.prologue_seen = true;
            chapter_store::save(&destination, &progress)?;
            saved_checkpoint = progress.checkpoint;
        }
        {
            let mut canvas = rl.begin_texture_mode(thread, &mut target);
            render::draw(
                &mut canvas,
                &assets,
                &chapter,
                progress.street_wake_tick.unwrap_or(600),
                debug,
            );
            if let Some(active_menu) = menu {
                let rows = if active_menu == Menu::Pause {
                    vec!["Continuar", "Retomar checkpoint", "Voltar ao menu"]
                } else if can_continue(&progress) {
                    vec![
                        "Continuar capítulo",
                        "Recomeçar capítulo",
                        "Rever prólogo",
                        "Voltar",
                    ]
                } else {
                    vec!["Iniciar capítulo", "Rever prólogo", "Voltar"]
                };
                render::menu(
                    &mut canvas,
                    &assets,
                    if active_menu == Menu::Pause {
                        "Pausa"
                    } else {
                        "Modo História"
                    },
                    &rows,
                    selected,
                    if active_menu == Menu::Pause {
                        "Seu progresso está salvo no último checkpoint."
                    } else {
                        "Capítulo 01 — Depois do silêncio · Rust"
                    },
                );
            }
            if let Some((text, remaining)) = &mut notice {
                canvas.draw_rectangle(375, 627, 530, 37, Color::new(22, 42, 32, 245));
                canvas.draw_text_ex(
                    &assets.common.body,
                    text,
                    Vector2::new(395.0, 636.0),
                    20.0,
                    0.1,
                    Color::RAYWHITE,
                );
                *remaining = remaining.saturating_sub(1);
                if *remaining == 0 {
                    notice = None;
                }
            }
        }
        if let Some(recorder) = &mut recorder {
            capture_accumulator += if reviewing { 1.0 / 60.0 } else { dt };
            let copies = (capture_accumulator * 30.0).floor() as usize;
            capture_accumulator -= copies as f32 / 30.0;
            if copies > 0 {
                recorder.frames(&target, copies)?;
            }
            let label = format!(
                "{:?}-{}-{}",
                chapter.phase,
                chapter.active_dialogue().map(|v| v.line_index).unwrap_or(0),
                chapter
                    .phone()
                    .map(|v| format!("{:?}", v.phase))
                    .unwrap_or_default()
            );
            if reviewing && label != previous_image {
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
            export(&target, &dir.join(format!("chapter-{frame:06}.png")))?;
        }
        if let Some(trace) = &mut trace {
            review::trace(
                trace,
                &chapter,
                frame,
                seconds,
                wall_seconds,
                menu.map(|m| if m == Menu::Story { "Story" } else { "Pause" }),
                menu.is_some(),
            )?;
        }
        {
            let width = rl.get_screen_width() as f32;
            let height = rl.get_screen_height() as f32;
            let scale = (width / 1280.0).min(height / 720.0);
            let mut screen = rl.begin_drawing(thread);
            screen.clear_background(Color::BLACK);
            screen.draw_texture_pro(
                target.texture(),
                Rectangle::new(0.0, 0.0, 1280.0, -720.0),
                Rectangle::new(
                    (width - 1280.0 * scale) * 0.5,
                    (height - 720.0 * scale) * 0.5,
                    1280.0 * scale,
                    720.0 * scale,
                ),
                Vector2::zero(),
                0.0,
                Color::WHITE,
            );
        }
        frame += 1;
        seconds += dt;
        if options.max_frames.is_some_and(|max| frame >= max) || reviewing && frame >= 60 * 180 {
            exit = CampaignExit::FrameLimit;
            break;
        }
        if reviewing && chapter.phase == Phase::Complete {
            exit = CampaignExit::Menu;
            break;
        }
    }
    if let Some(recorder) = recorder {
        recorder.finish()?;
    }
    Ok(exit)
}

fn can_continue(progress: &CampaignProgress) -> bool {
    progress
        .checkpoint
        .is_some_and(|c| c.stage != CheckpointStage::Complete)
}

fn save_path(options: &Options) -> std::path::PathBuf {
    options
        .review
        .as_ref()
        .map(|directory| directory.join("review-campaign.json"))
        .unwrap_or_else(chapter_store::path)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn automatic_review_never_uses_the_players_campaign_save() {
        let review_dir = std::env::temp_dir().join("borrow-chapter-render-review");
        let options = Options {
            review: Some(review_dir.clone()),
            ..Options::chapter()
        };
        assert_eq!(save_path(&options), review_dir.join("review-campaign.json"));
        assert_eq!(save_path(&Options::chapter()), chapter_store::path());
    }
}
