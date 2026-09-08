//! Runs the application loop for the greybox prototype.
//!
//! This layer owns Raylib handles, translates platform input, advances fixed
//! gameplay steps, and delegates drawing to the render module.

use raylib::prelude::*;
use std::path::PathBuf;

use crate::audio::{AudioEvent, MusicTrack};
use crate::characters::{CHARACTER_BODY_METRICS_PATH, CharacterBodyMetricsCatalog, CharacterId};
use crate::cli::{LaunchMode, LaunchOptions, MatchOptions};
use crate::combat::fighter::{FighterInput, PlayerSlot};
use crate::config::{
    FIXED_TIMESTEP, MAX_FIXED_STEPS_PER_FRAME, MAX_FRAME_TIME, TARGET_FPS, WINDOW_HEIGHT,
    WINDOW_WIDTH,
};
use crate::engine::{
    assets::GameAssets,
    audio::{AUDIO_MANIFEST_PATH, AudioPlayer},
    input::LocalInput,
    render::{self, GamepadStatus},
    sprites::C_FIGHTER_MANIFEST_PATH,
    video_capture::VideoCapture,
};
use crate::game::ai::BasicCpu;
use crate::game::arena::ArenaId;
use crate::game::feature_flags::{FeatureFlag, FeatureFlags};
use crate::game::world::{World, WorldSpriteCombatManifests};
use crate::scenes::{
    AppScene,
    combat_lab::{CombatLab, CombatLabInput, CombatLabMove, CombatLabOptions},
    move_showcase::{MoveShowcase, MoveShowcaseOptions},
    preferences::{CycleDirection, PreferencesAction, PreferencesMenu},
    sprite_viewer::{SpriteViewer, SpriteViewerInput, SpriteViewerOptions, ViewerPoint},
};

const CAPTURE_SMOKE_SECONDS_ENV: &str = "BORROW_FIGHTERS_CAPTURE_SMOKE_SECONDS";
const DEFAULT_MUSIC_VOLUME_PERCENT: u8 = 50;
const MUSIC_VOLUME_STEP_PERCENT: u8 = 10;

/// Retains button edges until the next fixed tick while refreshing held directions.
#[derive(Clone, Copy, Debug, Default)]
struct PendingFighterInput(FighterInput);

impl PendingFighterInput {
    fn push(&mut self, next: FighterInput) {
        self.0 = FighterInput {
            jump: self.0.jump || next.jump,
            light_punch: self.0.light_punch || next.light_punch,
            heavy_punch: self.0.heavy_punch || next.heavy_punch,
            kick: self.0.kick || next.kick,
            projectile: self.0.projectile || next.projectile,
            signature_special: self.0.signature_special || next.signature_special,
            ..next
        };
    }

    fn take_tick(&mut self) -> FighterInput {
        let input = self.0;
        self.0 = FighterInput {
            jump: false,
            ..input.without_attacks()
        };
        input
    }
}

/// Top-level application state outside the testable game world.
pub struct App {
    world: World,
    player_one_cpu: BasicCpu,
    player_two_cpu: BasicCpu,
    feature_flags: FeatureFlags,
    scene: AppScene,
    sprite_viewer_options: Option<SpriteViewerOptions>,
    preferences_menu: PreferencesMenu,
    combat_lab: CombatLab,
    move_showcase: MoveShowcase,
    pending_showcase_input: CombatLabInput,
    pending_fight_input: [PendingFighterInput; 2],
    character_body_metrics: CharacterBodyMetricsCatalog,
    match_options: MatchOptions,
    match_options_dirty: bool,
    current_arena: ArenaId,
    arena_selection_dirty: bool,
    advance_arena_on_next_match: bool,
    music_volume_percent: u8,
    video_capture: VideoCapture,
    accumulator: f32,
    visual_time_seconds: f32,
}

impl Default for App {
    fn default() -> Self {
        Self::new(LaunchOptions::default())
    }
}

impl App {
    /// Creates app state for the selected startup mode.
    pub fn new(options: LaunchOptions) -> Self {
        let match_options = options.match_options;
        let mut move_showcase = MoveShowcase::default();
        let (scene, combat_lab, sprite_viewer_options) = match options.mode {
            LaunchMode::Game => (
                if options.start_fight {
                    AppScene::Fight
                } else {
                    AppScene::Preferences
                },
                CombatLab::default(),
                None,
            ),
            LaunchMode::CombatLab(options) => (AppScene::CombatLab, CombatLab::new(options), None),
            LaunchMode::MoveShowcase(options) => {
                move_showcase = MoveShowcase::new(MoveShowcaseOptions {
                    character: options.character,
                });
                move_showcase.select_move(options.selected_move);
                if options.repeat_current {
                    move_showcase.toggle_repeat();
                }
                if options.sides_reversed {
                    move_showcase.switch_sides();
                }
                (AppScene::MoveShowcase, CombatLab::default(), None)
            }
            LaunchMode::SpriteViewer(options) => {
                (AppScene::SpriteViewer, CombatLab::default(), Some(options))
            }
        };
        let character_body_metrics = CharacterBodyMetricsCatalog::load(CHARACTER_BODY_METRICS_PATH)
            .unwrap_or_else(|error| {
                eprintln!("warning: using built-in character body metrics: {error}");
                CharacterBodyMetricsCatalog::default()
            });
        move_showcase.set_body_metrics(character_body_metrics.clone());

        Self {
            world: World::new_greybox_with_intro_for_characters_and_metrics(
                match_options.player_one,
                match_options.player_two,
                &character_body_metrics,
            ),
            player_one_cpu: BasicCpu::for_slot(PlayerSlot::One),
            player_two_cpu: BasicCpu::for_slot(PlayerSlot::Two),
            feature_flags: FeatureFlags::default(),
            scene,
            sprite_viewer_options,
            preferences_menu: PreferencesMenu::default(),
            combat_lab,
            move_showcase,
            pending_showcase_input: CombatLabInput::default(),
            pending_fight_input: [PendingFighterInput::default(); 2],
            character_body_metrics,
            match_options,
            match_options_dirty: false,
            current_arena: ArenaId::STARTING_ARENA,
            arena_selection_dirty: false,
            advance_arena_on_next_match: false,
            music_volume_percent: DEFAULT_MUSIC_VOLUME_PERCENT,
            video_capture: VideoCapture::default(),
            accumulator: 0.0,
            visual_time_seconds: 0.0,
        }
    }

    /// Runs the Raylib-backed game loop until the window closes.
    pub fn run(mut self, raylib: &mut RaylibHandle, thread: &RaylibThread) {
        raylib.set_target_fps(TARGET_FPS);
        if let Some(options) = self.sprite_viewer_options.take() {
            run_sprite_viewer(raylib, thread, options);
            return;
        }

        let assets = GameAssets::load(raylib, thread);
        self.sync_world_sprite_combat(&assets);
        self.combat_lab
            .set_combat_manifest(fighter_manifest_for_character(
                self.combat_lab.character(),
                &assets,
            ));
        self.sync_showcase_sprite_combat(&assets);
        let software_cursor_requested = software_cursor_enabled_for_env();
        let audio_device = RaylibAudio::init_audio_device();
        let mut audio_player = match &audio_device {
            Ok(audio_device) => AudioPlayer::load(audio_device, AUDIO_MANIFEST_PATH),
            Err(error) => {
                eprintln!("warning: audio disabled: {error}");
                AudioPlayer::disabled()
            }
        };
        audio_player.set_music_volume(self.music_volume_multiplier());
        audio_player.play_music(music_track_for_scene(self.scene, self.current_arena));
        let mut frame_target = load_frame_target(raylib, thread);
        let mut capture_smoke_test = CaptureSmokeTest::from_env();

        while !raylib.window_should_close() {
            let frame_time = raylib.get_frame_time().min(MAX_FRAME_TIME);
            self.visual_time_seconds += frame_time;
            sync_system_cursor(raylib);
            let mouse_position = raylib.get_mouse_position();
            let software_cursor_enabled = software_cursor_requested
                && raylib.is_window_focused()
                && raylib.is_cursor_on_screen();
            audio_player.update_streams();
            update_video_capture_status(&mut self.video_capture);

            let input = LocalInput::read(
                raylib,
                self.feature_flags.enabled(FeatureFlag::GamepadInput),
            );
            handle_video_capture_shortcuts(
                &mut self.video_capture,
                input.start_recording,
                input.stop_recording,
            );
            capture_smoke_test.update(frame_time, &mut self.video_capture);
            let gamepad_status = GamepadStatus {
                player_one: input.player_one_gamepad_connected,
                player_two: input.player_two_gamepad_connected,
            };

            match self.scene {
                AppScene::CombatLab => {
                    if input.open_preferences {
                        self.scene = AppScene::Preferences;
                        self.preferences_menu.ignore_next_input();
                        self.accumulator = 0.0;
                        audio_player.play(&AudioEvent::ui_back());
                        audio_player.play_music(MusicTrack::Menu);
                        sync_system_cursor(raylib);
                        {
                            let mut draw = raylib.begin_texture_mode(thread, &mut frame_target);
                            render::draw_preferences(
                                &mut draw,
                                render::PreferencesDrawOptions {
                                    menu: &self.preferences_menu,
                                    player_one_character: self.match_options.player_one,
                                    player_two_character: self.match_options.player_two,
                                    arena: self.current_arena,
                                    music_volume_percent: self.music_volume_percent,
                                    visual_time_seconds: self.visual_time_seconds,
                                    flags: self.feature_flags,
                                    gamepad_status,
                                    recording: self.video_capture.is_recording(),
                                    assets: &assets,
                                },
                            );
                            render::draw_video_capture_overlay(
                                &mut draw,
                                self.video_capture.is_recording(),
                                self.video_capture.last_message(),
                            );
                        }
                    } else {
                        self.update_combat_lab(frame_time, input.combat_lab);

                        {
                            let mut draw = raylib.begin_texture_mode(thread, &mut frame_target);
                            render::draw_combat_lab(&mut draw, &self.combat_lab, &assets);
                            render::draw_video_capture_overlay(
                                &mut draw,
                                self.video_capture.is_recording(),
                                self.video_capture.last_message(),
                            );
                        }
                    }
                    finish_frame(
                        raylib,
                        thread,
                        &frame_target,
                        &mut self.video_capture,
                        software_cursor_for_position(
                            software_cursor_enabled,
                            mouse_position,
                            self.visual_time_seconds,
                            &assets,
                        ),
                    );
                }
                AppScene::MoveShowcase => {
                    if input.open_preferences {
                        self.scene = AppScene::Preferences;
                        self.preferences_menu.ignore_next_input();
                        self.accumulator = 0.0;
                        audio_player.play(&AudioEvent::ui_back());
                        audio_player.play_music(MusicTrack::Menu);
                        sync_system_cursor(raylib);
                        {
                            let mut draw = raylib.begin_texture_mode(thread, &mut frame_target);
                            render::draw_preferences(
                                &mut draw,
                                render::PreferencesDrawOptions {
                                    menu: &self.preferences_menu,
                                    player_one_character: self.match_options.player_one,
                                    player_two_character: self.match_options.player_two,
                                    arena: self.current_arena,
                                    music_volume_percent: self.music_volume_percent,
                                    visual_time_seconds: self.visual_time_seconds,
                                    flags: self.feature_flags,
                                    gamepad_status,
                                    recording: self.video_capture.is_recording(),
                                    assets: &assets,
                                },
                            );
                            render::draw_video_capture_overlay(
                                &mut draw,
                                self.video_capture.is_recording(),
                                self.video_capture.last_message(),
                            );
                        }
                    } else {
                        if raylib.is_key_pressed(KeyboardKey::KEY_L) {
                            self.move_showcase.toggle_repeat();
                        }
                        if raylib.is_key_pressed(KeyboardKey::KEY_X) {
                            self.move_showcase.switch_sides();
                            self.pending_showcase_input = CombatLabInput::default();
                        }
                        if input.combat_lab.next_pose {
                            self.cycle_showcase_character(CycleDirection::Next);
                            self.sync_showcase_sprite_combat(&assets);
                        } else if input.combat_lab.previous_pose {
                            self.cycle_showcase_character(CycleDirection::Previous);
                            self.sync_showcase_sprite_combat(&assets);
                        }
                        self.update_move_showcase(frame_time, input.combat_lab, &mut audio_player);

                        {
                            let mut draw = raylib.begin_texture_mode(thread, &mut frame_target);
                            render::draw_move_showcase(
                                &mut draw,
                                &self.move_showcase,
                                self.current_arena,
                                self.visual_time_seconds,
                                &assets,
                            );
                            render::draw_video_capture_overlay(
                                &mut draw,
                                self.video_capture.is_recording(),
                                self.video_capture.last_message(),
                            );
                        }
                    }
                    finish_frame(
                        raylib,
                        thread,
                        &frame_target,
                        &mut self.video_capture,
                        software_cursor_for_position(
                            software_cursor_enabled,
                            mouse_position,
                            self.visual_time_seconds,
                            &assets,
                        ),
                    );
                }
                AppScene::Preferences => {
                    if input.open_preferences && self.preferences_menu.back() {
                        audio_player.play(&AudioEvent::ui_back());
                    } else {
                        let mut preferences_input = input.preferences;
                        preferences_input.pointer = crate::engine::input::read_preferences_pointer(
                            raylib,
                            &self.preferences_menu,
                        );
                        play_preferences_audio_feedback(
                            &mut audio_player,
                            preferences_input,
                            self.preferences_menu.selected(),
                        );
                        let preferences_action = self
                            .preferences_menu
                            .update(preferences_input, &mut self.feature_flags);
                        match preferences_action {
                            PreferencesAction::Stay => {}
                            PreferencesAction::CyclePlayerOne(direction) => {
                                self.match_options.player_one =
                                    cycle_character(self.match_options.player_one, direction);
                                self.match_options_dirty = true;
                            }
                            PreferencesAction::CyclePlayerTwo(direction) => {
                                self.match_options.player_two =
                                    cycle_character(self.match_options.player_two, direction);
                                self.match_options_dirty = true;
                            }
                            PreferencesAction::CycleArena(direction) => {
                                self.current_arena = cycle_arena(self.current_arena, direction);
                                self.arena_selection_dirty = true;
                                self.advance_arena_on_next_match = false;
                            }
                            PreferencesAction::AdjustMusicVolume(direction) => {
                                self.adjust_music_volume(direction);
                                audio_player.set_music_volume(self.music_volume_multiplier());
                            }
                            PreferencesAction::CycleLoreChapter(direction) => {
                                self.preferences_menu.cycle_lore_chapter(
                                    direction,
                                    assets.lore_book.chapter_count_for_menu(),
                                );
                            }
                            PreferencesAction::CycleLoreCharacter(direction) => {
                                self.preferences_menu.cycle_lore_character(
                                    direction,
                                    assets.lore_book.character_count_for_menu(),
                                );
                            }
                            PreferencesAction::ToggleRecording => {
                                toggle_video_capture(&mut self.video_capture);
                            }
                            PreferencesAction::OpenCombatLab => {
                                self.combat_lab = CombatLab::new(CombatLabOptions {
                                    character: self.match_options.player_one,
                                    selected_move: CombatLabMove::Projectile,
                                    ..CombatLabOptions::default()
                                });
                                self.combat_lab.set_combat_manifest(
                                    fighter_manifest_for_character(
                                        self.combat_lab.character(),
                                        &assets,
                                    ),
                                );
                                self.scene = AppScene::CombatLab;
                                self.accumulator = 0.0;
                                audio_player.play_music(MusicTrack::CombatDeterminedPursuit);
                            }
                            PreferencesAction::OpenMoveShowcase => {
                                self.move_showcase = MoveShowcase::new(MoveShowcaseOptions {
                                    character: self.match_options.player_one,
                                });
                                self.sync_showcase_sprite_combat(&assets);
                                self.pending_showcase_input = CombatLabInput::default();
                                self.scene = AppScene::MoveShowcase;
                                self.accumulator = 0.0;
                                audio_player.play_music(MusicTrack::CombatDeterminedPursuit);
                            }
                            PreferencesAction::OpenSpriteViewer => {
                                run_sprite_viewer(raylib, thread, default_sprite_viewer_options());
                                if raylib.window_should_close() {
                                    return;
                                }
                                self.preferences_menu.ignore_next_input();
                                audio_player.play_music(MusicTrack::Menu);
                            }
                            PreferencesAction::StartFight => {
                                if self.world.outcome.is_some()
                                    || self.match_options_dirty
                                    || self.arena_selection_dirty
                                {
                                    self.restart_match(&assets);
                                }
                                self.scene = AppScene::Fight;
                                audio_player.play_music(music_track_for_arena(self.current_arena));
                            }
                            PreferencesAction::Exit => return,
                        }
                    }
                    {
                        let mut draw = raylib.begin_texture_mode(thread, &mut frame_target);
                        render::draw_preferences(
                            &mut draw,
                            render::PreferencesDrawOptions {
                                menu: &self.preferences_menu,
                                player_one_character: self.match_options.player_one,
                                player_two_character: self.match_options.player_two,
                                arena: self.current_arena,
                                music_volume_percent: self.music_volume_percent,
                                visual_time_seconds: self.visual_time_seconds,
                                flags: self.feature_flags,
                                gamepad_status,
                                recording: self.video_capture.is_recording(),
                                assets: &assets,
                            },
                        );
                        render::draw_video_capture_overlay(
                            &mut draw,
                            self.video_capture.is_recording(),
                            self.video_capture.last_message(),
                        );
                    }
                    self.preferences_menu.tick_visuals();
                    finish_frame(
                        raylib,
                        thread,
                        &frame_target,
                        &mut self.video_capture,
                        software_cursor_for_position(
                            software_cursor_enabled,
                            mouse_position,
                            self.visual_time_seconds,
                            &assets,
                        ),
                    );
                }
                AppScene::Fight => {
                    if input.open_preferences {
                        self.pending_fight_input = [PendingFighterInput::default(); 2];
                        self.scene = AppScene::Preferences;
                        self.preferences_menu.ignore_next_input();
                        audio_player.play(&AudioEvent::ui_back());
                        audio_player.play_music(MusicTrack::Menu);
                        {
                            let mut draw = raylib.begin_texture_mode(thread, &mut frame_target);
                            render::draw_preferences(
                                &mut draw,
                                render::PreferencesDrawOptions {
                                    menu: &self.preferences_menu,
                                    player_one_character: self.match_options.player_one,
                                    player_two_character: self.match_options.player_two,
                                    arena: self.current_arena,
                                    music_volume_percent: self.music_volume_percent,
                                    visual_time_seconds: self.visual_time_seconds,
                                    flags: self.feature_flags,
                                    gamepad_status,
                                    recording: self.video_capture.is_recording(),
                                    assets: &assets,
                                },
                            );
                            render::draw_video_capture_overlay(
                                &mut draw,
                                self.video_capture.is_recording(),
                                self.video_capture.last_message(),
                            );
                        }
                        finish_frame(
                            raylib,
                            thread,
                            &frame_target,
                            &mut self.video_capture,
                            software_cursor_for_position(
                                software_cursor_enabled,
                                mouse_position,
                                self.visual_time_seconds,
                                &assets,
                            ),
                        );
                    } else {
                        if input.restart {
                            self.restart_match(&assets);
                            audio_player.play_music(music_track_for_arena(self.current_arena));
                        }

                        if input.toggle_cpu {
                            self.feature_flags.toggle(FeatureFlag::PlayerTwoCpu);
                        }

                        self.pending_fight_input[0].push(input.player_one);
                        self.pending_fight_input[1].push(input.player_two);
                        self.accumulator += frame_time;
                        let mut fixed_steps = 0;

                        while self.accumulator >= FIXED_TIMESTEP
                            && fixed_steps < MAX_FIXED_STEPS_PER_FRAME
                        {
                            let manual_one = self.pending_fight_input[0].take_tick();
                            let manual_two = self.pending_fight_input[1].take_tick();
                            let mut player_one =
                                if self.feature_flags.enabled(FeatureFlag::PlayerOneCpu) {
                                    self.player_one_cpu.next_input(
                                        &self.world,
                                        PlayerSlot::One,
                                        FIXED_TIMESTEP,
                                    )
                                } else {
                                    manual_one
                                };
                            let mut player_two =
                                if self.feature_flags.enabled(FeatureFlag::PlayerTwoCpu) {
                                    self.player_two_cpu.next_input(
                                        &self.world,
                                        PlayerSlot::Two,
                                        FIXED_TIMESTEP,
                                    )
                                } else {
                                    manual_two
                                };

                            player_one = cpu_attack_filtered_input(
                                player_one,
                                self.feature_flags,
                                FeatureFlag::PlayerOneCpu,
                            );
                            player_two = cpu_attack_filtered_input(
                                player_two,
                                self.feature_flags,
                                FeatureFlag::PlayerTwoCpu,
                            );

                            self.world.update_with_flags(
                                FIXED_TIMESTEP,
                                player_one,
                                player_two,
                                self.feature_flags,
                            );
                            self.remember_finished_match();
                            audio_player.play_events(self.world.drain_audio_events());
                            self.accumulator -= FIXED_TIMESTEP;
                            fixed_steps += 1;
                        }

                        audio_player.set_music_ducking(self.world.countdown_active());

                        if fixed_steps == MAX_FIXED_STEPS_PER_FRAME {
                            self.accumulator = 0.0;
                        }

                        {
                            let mut draw = raylib.begin_texture_mode(thread, &mut frame_target);
                            render::draw_fight(
                                &mut draw,
                                &self.world,
                                self.current_arena,
                                self.visual_time_seconds,
                                self.feature_flags,
                                gamepad_status,
                                &assets,
                            );
                            render::draw_video_capture_overlay(
                                &mut draw,
                                self.video_capture.is_recording(),
                                self.video_capture.last_message(),
                            );
                        }
                        finish_frame(
                            raylib,
                            thread,
                            &frame_target,
                            &mut self.video_capture,
                            software_cursor_for_position(
                                software_cursor_enabled,
                                mouse_position,
                                self.visual_time_seconds,
                                &assets,
                            ),
                        );
                    }
                }
                AppScene::SpriteViewer => unreachable!("sprite viewer has a separate app loop"),
            }
        }
    }

    fn restart_match(&mut self, assets: &GameAssets) {
        if self.arena_selection_dirty {
            self.advance_arena_on_next_match = false;
        } else if self.advance_arena_on_next_match || self.world.outcome.is_some() {
            self.current_arena = self.current_arena.next();
        }
        self.world = World::new_greybox_with_intro_for_characters_and_metrics(
            self.match_options.player_one,
            self.match_options.player_two,
            &self.character_body_metrics,
        );
        self.player_one_cpu = BasicCpu::for_slot(PlayerSlot::One);
        self.player_two_cpu = BasicCpu::for_slot(PlayerSlot::Two);
        self.match_options_dirty = false;
        self.arena_selection_dirty = false;
        self.advance_arena_on_next_match = false;
        self.accumulator = 0.0;
        self.pending_fight_input = [PendingFighterInput::default(); 2];
        self.sync_world_sprite_combat(assets);
    }

    fn sync_world_sprite_combat(&mut self, assets: &GameAssets) {
        self.world
            .set_sprite_combat_manifests(WorldSpriteCombatManifests {
                player_one: fighter_manifest_for_character(
                    self.world.player_one_character(),
                    assets,
                ),
                player_two: fighter_manifest_for_character(
                    self.world.player_two_character(),
                    assets,
                ),
            });
    }

    fn remember_finished_match(&mut self) {
        if self.world.outcome.is_some() {
            self.advance_arena_on_next_match = true;
        }
    }

    fn sync_showcase_sprite_combat(&mut self, assets: &GameAssets) {
        self.move_showcase
            .set_body_metrics(self.character_body_metrics.clone());
        self.move_showcase
            .set_sprite_combat_manifests(WorldSpriteCombatManifests {
                player_one: fighter_manifest_for_character(self.move_showcase.character(), assets),
                player_two: fighter_manifest_for_character(
                    self.move_showcase.opponent_character(),
                    assets,
                ),
            });
    }

    fn cycle_showcase_character(&mut self, direction: CycleDirection) {
        let scenario = self.move_showcase.scenario();
        let repeat = self.move_showcase.repeat_current();
        let reversed = self.move_showcase.sides_reversed();
        let paused = self.move_showcase.paused();
        let character = cycle_character(self.move_showcase.character(), direction);
        self.move_showcase = MoveShowcase::new(MoveShowcaseOptions { character });
        self.move_showcase.select_scenario(scenario);
        if repeat {
            self.move_showcase.toggle_repeat();
        }
        if reversed {
            self.move_showcase.switch_sides();
        }
        self.move_showcase
            .set_body_metrics(self.character_body_metrics.clone());
        if paused {
            self.move_showcase.update(CombatLabInput {
                pause_toggle: true,
                ..CombatLabInput::default()
            });
        }
        self.pending_showcase_input = CombatLabInput::default();
        self.accumulator = 0.0;
    }

    fn update_combat_lab(&mut self, frame_time: f32, input: CombatLabInput) {
        self.accumulator += frame_time;
        let mut fixed_steps = 0;

        while self.accumulator >= FIXED_TIMESTEP && fixed_steps < MAX_FIXED_STEPS_PER_FRAME {
            let lab_input = if fixed_steps == 0 {
                input
            } else {
                CombatLabInput::default()
            };
            self.combat_lab.update(lab_input);
            self.accumulator -= FIXED_TIMESTEP;
            fixed_steps += 1;
        }

        if fixed_steps == MAX_FIXED_STEPS_PER_FRAME {
            self.accumulator = 0.0;
        }
    }

    fn update_move_showcase(
        &mut self,
        frame_time: f32,
        input: CombatLabInput,
        audio: &mut AudioPlayer<'_>,
    ) {
        self.accumulator += frame_time;
        self.pending_showcase_input = merge_showcase_input(self.pending_showcase_input, input);
        let mut fixed_steps = 0;

        while self.accumulator >= FIXED_TIMESTEP && fixed_steps < MAX_FIXED_STEPS_PER_FRAME {
            let showcase_input = std::mem::take(&mut self.pending_showcase_input);
            self.move_showcase.update(showcase_input);
            audio.play_events(self.move_showcase.take_audio_events());
            self.accumulator -= FIXED_TIMESTEP;
            fixed_steps += 1;
        }

        if fixed_steps == MAX_FIXED_STEPS_PER_FRAME {
            self.accumulator = 0.0;
        }
    }

    fn adjust_music_volume(&mut self, direction: CycleDirection) {
        self.music_volume_percent = match direction {
            CycleDirection::Previous => self
                .music_volume_percent
                .saturating_sub(MUSIC_VOLUME_STEP_PERCENT),
            CycleDirection::Next => {
                (self.music_volume_percent + MUSIC_VOLUME_STEP_PERCENT).min(100)
            }
        };
    }

    fn music_volume_multiplier(&self) -> f32 {
        f32::from(self.music_volume_percent) / 100.0
    }
}

fn cpu_attack_filtered_input(
    input: FighterInput,
    flags: FeatureFlags,
    cpu_flag: FeatureFlag,
) -> FighterInput {
    if flags.enabled(cpu_flag) && !flags.enabled(FeatureFlag::CpuCanAttack) {
        input.without_attacks()
    } else {
        input
    }
}

fn merge_showcase_input(first: CombatLabInput, second: CombatLabInput) -> CombatLabInput {
    // Key presses survive render frames with no simulation tick; subsequent
    // catch-up ticks consume each navigation/pause/step command only once.
    CombatLabInput {
        next_move: first.next_move || second.next_move,
        previous_move: first.previous_move || second.previous_move,
        replay: first.replay || second.replay,
        pause_toggle: first.pause_toggle || second.pause_toggle,
        step_frame: first.step_frame || second.step_frame,
        reset: first.reset || second.reset,
        ..CombatLabInput::default()
    }
}

fn cycle_character(
    character: crate::characters::CharacterId,
    direction: CycleDirection,
) -> crate::characters::CharacterId {
    match direction {
        CycleDirection::Previous => character.demo_previous(),
        CycleDirection::Next => character.demo_next(),
    }
}

const fn cycle_arena(arena: ArenaId, direction: CycleDirection) -> ArenaId {
    match direction {
        CycleDirection::Previous => arena.previous(),
        CycleDirection::Next => arena.next(),
    }
}

fn fighter_manifest_for_character(
    character: CharacterId,
    assets: &GameAssets,
) -> Option<crate::engine::sprites::SpriteManifest> {
    match character {
        CharacterId::Rust => assets.rust_fighter.as_ref(),
        CharacterId::Duke => assets.duke_fighter.as_ref(),
        CharacterId::Go => assets.go_fighter.as_ref(),
        CharacterId::C => assets.c_fighter.as_ref(),
        CharacterId::Python => assets.python_fighter.as_ref(),
        CharacterId::Cpp => assets.cpp_fighter.as_ref(),
    }
    .map(|atlas| atlas.combat_manifest.clone())
}

fn play_preferences_audio_feedback<'aud>(
    audio_player: &mut AudioPlayer<'aud>,
    input: crate::scenes::preferences::PreferencesInput,
    selected_row: usize,
) {
    let pointer_selection_changed = input.pointer.moved
        && input
            .pointer
            .hovered_row
            .is_some_and(|row| row != selected_row);
    if input.up
        || input.down
        || input.left
        || input.right
        || input.scroll_up
        || input.scroll_down
        || pointer_selection_changed
        || (input.pointer.previous && input.pointer.hovered_row.is_some())
    {
        audio_player.play(&AudioEvent::ui_navigate());
    }

    if input.activate
        || input.start
        || (input.pointer.activate && input.pointer.hovered_row.is_some())
    {
        audio_player.play(&AudioEvent::ui_confirm());
    }
}

fn sync_system_cursor(raylib: &mut RaylibHandle) {
    // EnableCursor also warps the pointer to the window center in Raylib.
    // ShowCursor restores the normal cursor without changing its position.
    raylib.show_cursor();
}

#[derive(Clone, Copy)]
struct SoftwareCursorFrame<'a> {
    position: Vector2,
    visual_time_seconds: f32,
    assets: &'a GameAssets,
}

fn software_cursor_enabled_for_env() -> bool {
    match std::env::var("BORROW_FIGHTERS_SOFTWARE_CURSOR") {
        Ok(value) => !matches!(
            value.trim().to_ascii_lowercase().as_str(),
            "0" | "false" | "off" | "no"
        ),
        Err(_) => std::env::var_os("WSL_DISTRO_NAME").is_some(),
    }
}

fn software_cursor_for_position<'a>(
    enabled: bool,
    position: Vector2,
    visual_time_seconds: f32,
    assets: &'a GameAssets,
) -> Option<SoftwareCursorFrame<'a>> {
    enabled.then_some(SoftwareCursorFrame {
        position,
        visual_time_seconds,
        assets,
    })
}

const fn music_track_for_scene(scene: AppScene, arena: ArenaId) -> MusicTrack {
    match scene {
        AppScene::Preferences => MusicTrack::Menu,
        AppScene::Fight => music_track_for_arena(arena),
        AppScene::CombatLab => MusicTrack::CombatDeterminedPursuit,
        AppScene::MoveShowcase => MusicTrack::CombatDeterminedPursuit,
        AppScene::SpriteViewer => MusicTrack::Menu,
    }
}

const fn music_track_for_arena(arena: ArenaId) -> MusicTrack {
    match arena {
        ArenaId::Sirius => MusicTrack::Combat,
        ArenaId::Fortaleza => MusicTrack::CombatRandomEncounter,
        ArenaId::JavaStreet => MusicTrack::CombatConsoleFloor,
        ArenaId::BioTic => MusicTrack::CombatRinsTheme,
        ArenaId::PortoDigital => MusicTrack::CombatChiptuneBattle,
        ArenaId::ValeDoPinhao => MusicTrack::CombatEightBitBattle,
    }
}

fn run_sprite_viewer(
    raylib: &mut RaylibHandle,
    thread: &RaylibThread,
    options: SpriteViewerOptions,
) {
    sync_system_cursor(raylib);
    let mut viewer = match SpriteViewer::load(options) {
        Ok(viewer) => viewer,
        Err(error) => {
            let message = error.to_string();
            while !raylib.window_should_close() {
                if raylib.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
                    break;
                }
                let mut draw = raylib.begin_drawing(thread);
                render::draw_sprite_viewer_error(&mut draw, &message);
            }
            return;
        }
    };

    let mut texture = load_sprite_viewer_texture(raylib, thread, &mut viewer);
    let mut video_capture = VideoCapture::default();
    let mut frame_target = load_frame_target(raylib, thread);

    while !raylib.window_should_close() {
        if raylib.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
            break;
        }
        update_video_capture_status(&mut video_capture);
        handle_video_capture_shortcuts(
            &mut video_capture,
            raylib.is_key_pressed(KeyboardKey::KEY_F9),
            raylib.is_key_pressed(KeyboardKey::KEY_F10),
        );
        let input = read_sprite_viewer_input(raylib);
        if input.reload_manifest {
            match viewer.reload_manifest() {
                Ok(_) => {
                    texture = load_sprite_viewer_texture(raylib, thread, &mut viewer);
                }
                Err(error) => viewer.set_texture_error(error.to_string()),
            }
        }
        let screenshot_requested = input.screenshot_requested;
        viewer.update(input, raylib.get_frame_time().min(MAX_FRAME_TIME));

        {
            let mut draw = raylib.begin_texture_mode(thread, &mut frame_target);
            render::draw_sprite_viewer(&mut draw, &viewer, texture.as_ref());
            render::draw_video_capture_overlay(
                &mut draw,
                video_capture.is_recording(),
                video_capture.last_message(),
            );
        }
        finish_frame(raylib, thread, &frame_target, &mut video_capture, None);

        if screenshot_requested {
            let path = "target/sprite-viewer-capture.png";
            if let Err(error) = std::fs::create_dir_all("target") {
                viewer.set_texture_error(format!("could not create target directory: {error}"));
            } else {
                raylib.take_screenshot(thread, path);
                viewer.set_status_message(format!("Screenshot salvo em {path}."));
            }
        }
    }
}

fn default_sprite_viewer_options() -> SpriteViewerOptions {
    SpriteViewerOptions {
        manifest_path: PathBuf::from(C_FIGHTER_MANIFEST_PATH),
        initial_clip: Some("special".to_owned()),
        character: Some(CharacterId::C),
        selected_move: CombatLabMove::Projectile,
    }
}

fn update_video_capture_status(video_capture: &mut VideoCapture) {
    if let Err(error) = video_capture.update() {
        eprintln!("warning: could not update recording status: {error}");
        video_capture.set_error_message(&error);
    }
}

fn finish_frame(
    raylib: &mut RaylibHandle,
    thread: &RaylibThread,
    frame_target: &RenderTexture2D,
    video_capture: &mut VideoCapture,
    software_cursor: Option<SoftwareCursorFrame<'_>>,
) {
    if let Err(error) = video_capture.capture_render_texture(frame_target) {
        eprintln!("warning: could not capture frame: {error}");
        video_capture.set_error_message(&error);
    }

    let mut draw = raylib.begin_drawing(thread);
    render::draw_render_target_to_window(&mut draw, frame_target);
    if let Some(cursor) = software_cursor {
        render::draw_linker_chip_cursor(
            &mut draw,
            cursor.position,
            cursor.visual_time_seconds,
            cursor.assets,
        );
    }
}

fn load_frame_target(raylib: &mut RaylibHandle, thread: &RaylibThread) -> RenderTexture2D {
    raylib
        .load_render_texture(thread, WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32)
        .expect("render texture for frame capture")
}

fn handle_video_capture_shortcuts(
    video_capture: &mut VideoCapture,
    start_recording: bool,
    stop_recording: bool,
) {
    if start_recording
        && let Err(error) = video_capture.start(WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32)
    {
        eprintln!("warning: could not start recording: {error}");
        video_capture.set_error_message(&error);
    }

    if stop_recording && let Err(error) = video_capture.stop() {
        eprintln!("warning: could not stop recording: {error}");
        video_capture.set_error_message(&error);
    }
}

fn toggle_video_capture(video_capture: &mut VideoCapture) {
    if video_capture.is_recording() {
        if let Err(error) = video_capture.stop() {
            eprintln!("warning: could not stop recording: {error}");
            video_capture.set_error_message(&error);
        }
    } else if let Err(error) = video_capture.start(WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32) {
        eprintln!("warning: could not start recording: {error}");
        video_capture.set_error_message(&error);
    }
}

#[derive(Debug, Default)]
struct CaptureSmokeTest {
    remaining_seconds: f32,
    started: bool,
    finished: bool,
}

impl CaptureSmokeTest {
    fn from_env() -> Self {
        let Ok(value) = std::env::var(CAPTURE_SMOKE_SECONDS_ENV) else {
            return Self::default();
        };
        let Ok(seconds) = value.parse::<f32>() else {
            eprintln!("warning: ignoring invalid {CAPTURE_SMOKE_SECONDS_ENV}={value}");
            return Self::default();
        };
        if seconds <= 0.0 {
            eprintln!("warning: ignoring non-positive {CAPTURE_SMOKE_SECONDS_ENV}={value}");
            return Self::default();
        }

        Self {
            remaining_seconds: seconds,
            started: false,
            finished: false,
        }
    }

    fn update(&mut self, frame_time: f32, video_capture: &mut VideoCapture) {
        if self.finished || self.remaining_seconds <= 0.0 {
            return;
        }

        if !self.started {
            self.started = true;
            if let Err(error) = video_capture.start(WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32) {
                eprintln!("warning: could not start smoke recording: {error}");
                video_capture.set_error_message(&error);
                self.finished = true;
                return;
            }
        }

        self.remaining_seconds -= frame_time;
        if self.remaining_seconds <= 0.0 {
            if let Err(error) = video_capture.stop() {
                eprintln!("warning: could not stop smoke recording: {error}");
                video_capture.set_error_message(&error);
            }
            self.finished = true;
        }
    }
}

fn load_sprite_viewer_texture(
    raylib: &mut RaylibHandle,
    thread: &RaylibThread,
    viewer: &mut SpriteViewer,
) -> Option<Texture2D> {
    let texture_path = viewer.image_path().to_string_lossy().to_string();
    match raylib.load_texture(thread, &texture_path) {
        Ok(texture) => {
            viewer.set_status_message(format!("Atlas carregado: {texture_path}"));
            Some(texture)
        }
        Err(error) => {
            viewer.set_texture_error(format!("could not load texture {texture_path}: {error:?}"));
            None
        }
    }
}

fn read_sprite_viewer_input(raylib: &RaylibHandle) -> SpriteViewerInput {
    let tab_pressed = raylib.is_key_pressed(KeyboardKey::KEY_TAB);
    let shift_down = raylib.is_key_down(KeyboardKey::KEY_LEFT_SHIFT)
        || raylib.is_key_down(KeyboardKey::KEY_RIGHT_SHIFT);
    let control_down = raylib.is_key_down(KeyboardKey::KEY_LEFT_CONTROL)
        || raylib.is_key_down(KeyboardKey::KEY_RIGHT_CONTROL);
    let pivot_step = if shift_down { 8 } else { 1 };
    let arrow_x = raylib.is_key_pressed(KeyboardKey::KEY_RIGHT) as i32
        - raylib.is_key_pressed(KeyboardKey::KEY_LEFT) as i32;
    let arrow_y = raylib.is_key_pressed(KeyboardKey::KEY_DOWN) as i32
        - raylib.is_key_pressed(KeyboardKey::KEY_UP) as i32;
    let mouse = raylib.get_mouse_position();

    SpriteViewerInput {
        next_clip: tab_pressed && !shift_down,
        previous_clip: tab_pressed && shift_down,
        next_character: raylib.is_key_pressed(KeyboardKey::KEY_C) && !shift_down,
        previous_character: raylib.is_key_pressed(KeyboardKey::KEY_C) && shift_down,
        next_move: raylib.is_key_pressed(KeyboardKey::KEY_RIGHT_BRACKET),
        previous_move: raylib.is_key_pressed(KeyboardKey::KEY_LEFT_BRACKET),
        sync_clip_to_move: raylib.is_key_pressed(KeyboardKey::KEY_ENTER),
        next_frame: raylib.is_key_pressed(KeyboardKey::KEY_PERIOD),
        previous_frame: raylib.is_key_pressed(KeyboardKey::KEY_COMMA),
        toggle_playback: raylib.is_key_pressed(KeyboardKey::KEY_SPACE),
        toggle_grid: raylib.is_key_pressed(KeyboardKey::KEY_G),
        toggle_pivot: raylib.is_key_pressed(KeyboardKey::KEY_P),
        toggle_bounds: raylib.is_key_pressed(KeyboardKey::KEY_B),
        toggle_dummy: raylib.is_key_pressed(KeyboardKey::KEY_O),
        toggle_combat_overlay: raylib.is_key_pressed(KeyboardKey::KEY_M),
        toggle_projectile_trajectory: raylib.is_key_pressed(KeyboardKey::KEY_T),
        reload_manifest: raylib.is_key_pressed(KeyboardKey::KEY_F5),
        save_manifest: control_down && raylib.is_key_pressed(KeyboardKey::KEY_S),
        seed_frame_combat: raylib.is_key_pressed(KeyboardKey::KEY_N),
        add_frame_hurtbox: !shift_down && raylib.is_key_pressed(KeyboardKey::KEY_H),
        add_frame_hitbox: !shift_down && raylib.is_key_pressed(KeyboardKey::KEY_J),
        delete_frame_combat: raylib.is_key_pressed(KeyboardKey::KEY_DELETE),
        increase_manifest_scale: raylib.is_key_pressed(KeyboardKey::KEY_EQUAL),
        decrease_manifest_scale: raylib.is_key_pressed(KeyboardKey::KEY_MINUS),
        reset_zoom: raylib.is_key_pressed(KeyboardKey::KEY_ZERO),
        screenshot_requested: raylib.is_key_pressed(KeyboardKey::KEY_F12),
        zoom_delta: raylib.get_mouse_wheel_move(),
        nudge_pivot_x: if control_down {
            0
        } else {
            arrow_x * pivot_step
        },
        nudge_pivot_y: if control_down {
            0
        } else {
            arrow_y * pivot_step
        },
        nudge_body_width: if control_down && !shift_down {
            arrow_x * pivot_step
        } else {
            0
        },
        nudge_standing_height: if control_down && !shift_down {
            -arrow_y * pivot_step
        } else {
            0
        },
        nudge_crouch_height: if control_down && shift_down {
            -arrow_y * pivot_step
        } else {
            0
        },
        reset_position: raylib.is_key_pressed(KeyboardKey::KEY_R),
        mouse_position: ViewerPoint::new(mouse.x, mouse.y),
        mouse_pressed: raylib.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT),
        mouse_down: raylib.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT),
        mouse_released: raylib.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn app_starts_with_music_volume_at_half() {
        let app = App::default();

        assert_eq!(app.music_volume_percent, 50);
        assert_eq!(app.music_volume_multiplier(), 0.5);
    }

    #[test]
    fn app_applies_showcase_launch_move_repeat_and_facing() {
        let options = LaunchOptions::parse(
            [
                "game",
                "--showcase",
                "--character",
                "cpp",
                "--move",
                "signature_special",
                "--repeat",
                "--reverse",
            ]
            .map(String::from),
        )
        .unwrap();
        let app = App::new(options);
        assert_eq!(app.scene, AppScene::MoveShowcase);
        assert_eq!(app.move_showcase.character(), CharacterId::Cpp);
        assert_eq!(
            app.move_showcase.selected_move(),
            CombatLabMove::SignatureSpecial
        );
        assert!(app.move_showcase.repeat_current());
        assert!(app.move_showcase.sides_reversed());
        assert!(
            app.move_showcase.world().player_one.position.x
                > app.move_showcase.world().player_two.position.x
        );
    }

    #[test]
    fn showcase_preserves_pressed_keys_until_a_fixed_tick_and_steps_once_when_paused() {
        let mut app = App::default();
        let mut audio = AudioPlayer::disabled();
        app.update_move_showcase(
            FIXED_TIMESTEP * 0.25,
            CombatLabInput {
                pause_toggle: true,
                ..CombatLabInput::default()
            },
            &mut audio,
        );
        assert!(!app.move_showcase.paused());
        app.update_move_showcase(FIXED_TIMESTEP * 0.75, CombatLabInput::default(), &mut audio);
        assert!(app.move_showcase.paused());
        assert_eq!(app.move_showcase.current_frame(), 0);
        app.update_move_showcase(
            FIXED_TIMESTEP * 3.0,
            CombatLabInput {
                step_frame: true,
                ..CombatLabInput::default()
            },
            &mut audio,
        );
        assert_eq!(app.move_showcase.current_frame(), 1);
        assert!(app.move_showcase.take_audio_events().is_empty());
    }

    #[test]
    fn fight_button_edges_survive_until_tick_without_repeating_during_catch_up() {
        let mut pending = PendingFighterInput::default();
        pending.push(FighterInput {
            signature_special: true,
            jump: true,
            right: true,
            ..FighterInput::default()
        });
        // A render-only frame can refresh held controls before the next fixed tick.
        pending.push(FighterInput {
            block: true,
            left: true,
            ..FighterInput::default()
        });
        let first = pending.take_tick();
        assert!(first.signature_special && first.jump);
        assert!(first.left && first.block);
        assert!(!first.right);
        let next = pending.take_tick();
        assert!(!next.signature_special && !next.jump);
        assert!(next.left && next.block);
    }

    #[test]
    fn showcase_page_navigation_preserves_all_defense_scenarios_and_playback_options() {
        use crate::scenes::move_showcase::ShowcaseScenario;

        for scenario in [
            ShowcaseScenario::StandingBlock,
            ShowcaseScenario::OverheadBlock,
            ShowcaseScenario::CrouchingBlock,
            ShowcaseScenario::ProjectileBlock,
        ] {
            let mut app = App::default();
            app.move_showcase.select_scenario(scenario);
            app.move_showcase.toggle_repeat();
            app.move_showcase.switch_sides();
            app.move_showcase.update(CombatLabInput {
                pause_toggle: true,
                ..CombatLabInput::default()
            });
            // PageDown and PageUp use these same transitions, including wraparound.
            for direction in [CycleDirection::Next, CycleDirection::Previous] {
                for _ in 0..5 {
                    let previous = app.move_showcase.character();
                    app.cycle_showcase_character(direction);
                    assert_ne!(app.move_showcase.character(), previous);
                    assert_ne!(app.move_showcase.character(), CharacterId::Go);
                    assert_eq!(app.move_showcase.scenario(), scenario);
                    assert_eq!(app.move_showcase.current_frame(), 0);
                    assert!(app.move_showcase.paused());
                    assert!(app.move_showcase.repeat_current());
                    assert!(app.move_showcase.sides_reversed());
                }
                assert_eq!(app.move_showcase.character(), CharacterId::Rust);
            }
        }
    }
}
