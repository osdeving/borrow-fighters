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
    preferences::{PreferencesAction, PreferencesMenu},
    sprite_viewer::{SpriteViewer, SpriteViewerInput, SpriteViewerOptions, ViewerPoint},
};

const CAPTURE_SMOKE_SECONDS_ENV: &str = "BORROW_FIGHTERS_CAPTURE_SMOKE_SECONDS";

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
    character_body_metrics: CharacterBodyMetricsCatalog,
    match_options: MatchOptions,
    match_options_dirty: bool,
    current_arena: ArenaId,
    advance_arena_on_next_match: bool,
    video_capture: VideoCapture,
    accumulator: f32,
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
            LaunchMode::SpriteViewer(options) => {
                (AppScene::SpriteViewer, CombatLab::default(), Some(options))
            }
        };
        let character_body_metrics = CharacterBodyMetricsCatalog::load(CHARACTER_BODY_METRICS_PATH)
            .unwrap_or_else(|error| {
                eprintln!("warning: using built-in character body metrics: {error}");
                CharacterBodyMetricsCatalog::default()
            });

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
            character_body_metrics,
            match_options,
            match_options_dirty: false,
            current_arena: ArenaId::STARTING_ARENA,
            advance_arena_on_next_match: false,
            video_capture: VideoCapture::default(),
            accumulator: 0.0,
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
        let audio_device = RaylibAudio::init_audio_device();
        let mut audio_player = match &audio_device {
            Ok(audio_device) => AudioPlayer::load(audio_device, AUDIO_MANIFEST_PATH),
            Err(error) => {
                eprintln!("warning: audio disabled: {error}");
                AudioPlayer::disabled()
            }
        };
        audio_player.play_music(music_track_for_scene(self.scene));
        let mut frame_target = load_frame_target(raylib, thread);
        let mut capture_smoke_test = CaptureSmokeTest::from_env();

        while !raylib.window_should_close() {
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
            capture_smoke_test.update(
                raylib.get_frame_time().min(MAX_FRAME_TIME),
                &mut self.video_capture,
            );
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
                        {
                            let mut draw = raylib.begin_texture_mode(thread, &mut frame_target);
                            render::draw_preferences(
                                &mut draw,
                                render::PreferencesDrawOptions {
                                    menu: &self.preferences_menu,
                                    player_one_character: self.match_options.player_one,
                                    player_two_character: self.match_options.player_two,
                                    arena: self.current_arena,
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
                        self.update_combat_lab(
                            raylib.get_frame_time().min(MAX_FRAME_TIME),
                            input.combat_lab,
                        );

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
                    finish_frame(raylib, thread, &frame_target, &mut self.video_capture);
                }
                AppScene::Preferences => {
                    if input.open_preferences && self.preferences_menu.back() {
                        audio_player.play(&AudioEvent::ui_back());
                    } else {
                        play_preferences_audio_feedback(&mut audio_player, input.preferences);
                        let preferences_action = self
                            .preferences_menu
                            .update(input.preferences, &mut self.feature_flags);
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
                            PreferencesAction::ToggleRecording => {
                                toggle_video_capture(&mut self.video_capture);
                            }
                            PreferencesAction::OpenCombatLab => {
                                self.combat_lab = CombatLab::new(CombatLabOptions {
                                    character: self.match_options.player_one,
                                    selected_move: CombatLabMove::Projectile,
                                    ..CombatLabOptions::default()
                                });
                                self.scene = AppScene::CombatLab;
                                self.accumulator = 0.0;
                                audio_player.play_music(MusicTrack::Combat);
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
                                if self.world.outcome.is_some() || self.match_options_dirty {
                                    self.restart_match(&assets);
                                }
                                self.scene = AppScene::Fight;
                                audio_player.play_music(MusicTrack::Combat);
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
                    finish_frame(raylib, thread, &frame_target, &mut self.video_capture);
                }
                AppScene::Fight => {
                    if input.open_preferences {
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
                        finish_frame(raylib, thread, &frame_target, &mut self.video_capture);
                    } else {
                        if input.restart {
                            self.restart_match(&assets);
                        }

                        if input.toggle_cpu {
                            self.feature_flags.toggle(FeatureFlag::PlayerTwoCpu);
                        }

                        self.accumulator += raylib.get_frame_time().min(MAX_FRAME_TIME);
                        let mut fixed_steps = 0;

                        while self.accumulator >= FIXED_TIMESTEP
                            && fixed_steps < MAX_FIXED_STEPS_PER_FRAME
                        {
                            let mut player_one =
                                if self.feature_flags.enabled(FeatureFlag::PlayerOneCpu) {
                                    self.player_one_cpu.next_input(
                                        &self.world,
                                        PlayerSlot::One,
                                        FIXED_TIMESTEP,
                                    )
                                } else {
                                    input.player_one
                                };
                            let mut player_two =
                                if self.feature_flags.enabled(FeatureFlag::PlayerTwoCpu) {
                                    self.player_two_cpu.next_input(
                                        &self.world,
                                        PlayerSlot::Two,
                                        FIXED_TIMESTEP,
                                    )
                                } else {
                                    input.player_two
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
                            audio_player.play_events(self.world.take_audio_events());
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
                        finish_frame(raylib, thread, &frame_target, &mut self.video_capture);
                    }
                }
                AppScene::SpriteViewer => unreachable!("sprite viewer has a separate app loop"),
            }
        }
    }

    fn restart_match(&mut self, assets: &GameAssets) {
        if self.advance_arena_on_next_match || self.world.outcome.is_some() {
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
        self.advance_arena_on_next_match = false;
        self.accumulator = 0.0;
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

fn cycle_character(
    character: crate::characters::CharacterId,
    direction: crate::scenes::preferences::CycleDirection,
) -> crate::characters::CharacterId {
    match direction {
        crate::scenes::preferences::CycleDirection::Previous => character.previous(),
        crate::scenes::preferences::CycleDirection::Next => character.next(),
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
    }
    .map(|atlas| atlas.manifest.clone())
}

fn play_preferences_audio_feedback<'aud>(
    audio_player: &mut AudioPlayer<'aud>,
    input: crate::scenes::preferences::PreferencesInput,
) {
    if input.up || input.down || input.left || input.right {
        audio_player.play(&AudioEvent::ui_navigate());
    }

    if input.activate || input.start {
        audio_player.play(&AudioEvent::ui_confirm());
    }
}

const fn music_track_for_scene(scene: AppScene) -> MusicTrack {
    match scene {
        AppScene::Preferences => MusicTrack::Menu,
        AppScene::Fight | AppScene::CombatLab => MusicTrack::Combat,
        AppScene::SpriteViewer => MusicTrack::Menu,
    }
}

fn run_sprite_viewer(
    raylib: &mut RaylibHandle,
    thread: &RaylibThread,
    options: SpriteViewerOptions,
) {
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
        finish_frame(raylib, thread, &frame_target, &mut video_capture);

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
) {
    if let Err(error) = video_capture.capture_render_texture(frame_target) {
        eprintln!("warning: could not capture frame: {error}");
        video_capture.set_error_message(&error);
    }

    let mut draw = raylib.begin_drawing(thread);
    render::draw_render_target_to_window(&mut draw, frame_target);
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
