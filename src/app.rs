//! Runs the application loop for the greybox prototype.
//!
//! This layer owns Raylib handles, translates platform input, advances fixed
//! gameplay steps, and delegates drawing to the render module.

use raylib::prelude::*;

use crate::audio::{AudioEvent, MusicTrack};
use crate::cli::{LaunchMode, LaunchOptions, MatchOptions};
use crate::combat::fighter::{FighterInput, PlayerSlot};
use crate::config::{FIXED_TIMESTEP, MAX_FIXED_STEPS_PER_FRAME, MAX_FRAME_TIME, TARGET_FPS};
use crate::engine::{
    assets::GameAssets,
    audio::{AUDIO_MANIFEST_PATH, AudioPlayer},
    input::LocalInput,
    render::{self, GamepadStatus},
};
use crate::game::ai::BasicCpu;
use crate::game::arena::ArenaId;
use crate::game::feature_flags::{FeatureFlag, FeatureFlags};
use crate::game::world::World;
use crate::scenes::{
    AppScene,
    combat_lab::{CombatLab, CombatLabInput},
    preferences::{PreferencesAction, PreferencesMenu},
};

/// Top-level application state outside the testable game world.
pub struct App {
    world: World,
    player_one_cpu: BasicCpu,
    player_two_cpu: BasicCpu,
    feature_flags: FeatureFlags,
    scene: AppScene,
    preferences_menu: PreferencesMenu,
    combat_lab: CombatLab,
    match_options: MatchOptions,
    current_arena: ArenaId,
    advance_arena_on_next_match: bool,
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
        let (scene, combat_lab) = match options.mode {
            LaunchMode::Game => (
                if options.start_fight {
                    AppScene::Fight
                } else {
                    AppScene::Preferences
                },
                CombatLab::default(),
            ),
            LaunchMode::CombatLab(options) => (AppScene::CombatLab, CombatLab::new(options)),
        };

        Self {
            world: World::new_greybox_with_intro_for_characters(
                match_options.player_one,
                match_options.player_two,
            ),
            player_one_cpu: BasicCpu::for_slot(PlayerSlot::One),
            player_two_cpu: BasicCpu::for_slot(PlayerSlot::Two),
            feature_flags: FeatureFlags::default(),
            scene,
            preferences_menu: PreferencesMenu::default(),
            combat_lab,
            match_options,
            current_arena: ArenaId::STARTING_ARENA,
            advance_arena_on_next_match: false,
            accumulator: 0.0,
        }
    }

    /// Runs the Raylib-backed game loop until the window closes.
    pub fn run(mut self, raylib: &mut RaylibHandle, thread: &RaylibThread) {
        raylib.set_target_fps(TARGET_FPS);
        let assets = GameAssets::load(raylib, thread);
        let audio_device = RaylibAudio::init_audio_device();
        let mut audio_player = match &audio_device {
            Ok(audio_device) => AudioPlayer::load(audio_device, AUDIO_MANIFEST_PATH),
            Err(error) => {
                eprintln!("warning: audio disabled: {error}");
                AudioPlayer::disabled()
            }
        };
        audio_player.play_music(music_track_for_scene(self.scene));

        while !raylib.window_should_close() {
            audio_player.update_streams();

            let input = LocalInput::read(
                raylib,
                self.feature_flags.enabled(FeatureFlag::GamepadInput),
            );
            let gamepad_status = GamepadStatus {
                player_one: input.player_one_gamepad_connected,
                player_two: input.player_two_gamepad_connected,
            };

            if self.scene == AppScene::CombatLab {
                self.update_combat_lab(
                    raylib.get_frame_time().min(MAX_FRAME_TIME),
                    input.combat_lab,
                );

                let mut draw = raylib.begin_drawing(thread);
                render::draw_combat_lab(&mut draw, &self.combat_lab, &assets);
                continue;
            }

            if self.scene == AppScene::Preferences {
                play_preferences_audio_feedback(&mut audio_player, input.preferences);
                if self
                    .preferences_menu
                    .update(input.preferences, &mut self.feature_flags)
                    == PreferencesAction::StartFight
                {
                    if self.world.outcome.is_some() {
                        self.restart_match();
                    }
                    self.scene = AppScene::Fight;
                    audio_player.play_music(MusicTrack::Combat);
                }

                let mut draw = raylib.begin_drawing(thread);
                render::draw_preferences(
                    &mut draw,
                    &self.preferences_menu,
                    self.current_arena,
                    self.feature_flags,
                    gamepad_status,
                    &assets,
                );
                continue;
            }

            if input.open_preferences {
                self.scene = AppScene::Preferences;
                self.preferences_menu.ignore_next_input();
                audio_player.play(&AudioEvent::ui_back());
                audio_player.play_music(MusicTrack::Menu);
                let mut draw = raylib.begin_drawing(thread);
                render::draw_preferences(
                    &mut draw,
                    &self.preferences_menu,
                    self.current_arena,
                    self.feature_flags,
                    gamepad_status,
                    &assets,
                );
                continue;
            }

            if input.restart {
                self.restart_match();
            }

            if input.toggle_cpu {
                self.feature_flags.toggle(FeatureFlag::PlayerTwoCpu);
            }

            self.accumulator += raylib.get_frame_time().min(MAX_FRAME_TIME);
            let mut fixed_steps = 0;

            while self.accumulator >= FIXED_TIMESTEP && fixed_steps < MAX_FIXED_STEPS_PER_FRAME {
                let mut player_one = if self.feature_flags.enabled(FeatureFlag::PlayerOneCpu) {
                    self.player_one_cpu
                        .next_input(&self.world, PlayerSlot::One, FIXED_TIMESTEP)
                } else {
                    input.player_one
                };
                let mut player_two = if self.feature_flags.enabled(FeatureFlag::PlayerTwoCpu) {
                    self.player_two_cpu
                        .next_input(&self.world, PlayerSlot::Two, FIXED_TIMESTEP)
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

            let mut draw = raylib.begin_drawing(thread);
            render::draw_fight(
                &mut draw,
                &self.world,
                self.current_arena,
                self.feature_flags,
                gamepad_status,
                &assets,
            );
        }
    }

    fn restart_match(&mut self) {
        if self.advance_arena_on_next_match || self.world.outcome.is_some() {
            self.current_arena = self.current_arena.next();
        }
        self.world = World::new_greybox_with_intro_for_characters(
            self.match_options.player_one,
            self.match_options.player_two,
        );
        self.player_one_cpu = BasicCpu::for_slot(PlayerSlot::One);
        self.player_two_cpu = BasicCpu::for_slot(PlayerSlot::Two);
        self.advance_arena_on_next_match = false;
        self.accumulator = 0.0;
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

fn play_preferences_audio_feedback<'aud>(
    audio_player: &mut AudioPlayer<'aud>,
    input: crate::scenes::preferences::PreferencesInput,
) {
    if input.up || input.down {
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
    }
}
