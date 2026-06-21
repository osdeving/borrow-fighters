//! Runs the application loop for the greybox prototype.
//!
//! This layer owns Raylib handles, translates platform input, advances fixed
//! gameplay steps, and delegates drawing to the render module.

use raylib::prelude::*;

use crate::config::{FIXED_TIMESTEP, MAX_FIXED_STEPS_PER_FRAME, MAX_FRAME_TIME, TARGET_FPS};
use crate::engine::{
    assets::GameAssets,
    input::LocalInput,
    render::{self, GamepadStatus},
};
use crate::game::ai::BasicCpu;
use crate::game::feature_flags::{FeatureFlag, FeatureFlags};
use crate::game::world::World;
use crate::scenes::{
    AppScene,
    preferences::{PreferencesAction, PreferencesMenu},
};

/// Top-level application state outside the testable game world.
pub struct App {
    world: World,
    player_two_cpu: BasicCpu,
    feature_flags: FeatureFlags,
    scene: AppScene,
    preferences_menu: PreferencesMenu,
    accumulator: f32,
}

impl Default for App {
    fn default() -> Self {
        Self {
            world: World::new_greybox_with_intro(),
            player_two_cpu: BasicCpu::default(),
            feature_flags: FeatureFlags::default(),
            scene: AppScene::Preferences,
            preferences_menu: PreferencesMenu::default(),
            accumulator: 0.0,
        }
    }
}

impl App {
    /// Runs the Raylib-backed game loop until the window closes.
    pub fn run(mut self, raylib: &mut RaylibHandle, thread: &RaylibThread) {
        raylib.set_target_fps(TARGET_FPS);
        let assets = GameAssets::load(raylib, thread);

        while !raylib.window_should_close() {
            let input = LocalInput::read(
                raylib,
                self.feature_flags.enabled(FeatureFlag::GamepadInput),
            );
            let gamepad_status = GamepadStatus {
                player_one: input.player_one_gamepad_connected,
                player_two: input.player_two_gamepad_connected,
            };

            if self.scene == AppScene::Preferences {
                if self
                    .preferences_menu
                    .update(input.preferences, &mut self.feature_flags)
                    == PreferencesAction::StartFight
                {
                    self.scene = AppScene::Fight;
                }

                let mut draw = raylib.begin_drawing(thread);
                render::draw_preferences(
                    &mut draw,
                    &self.preferences_menu,
                    self.feature_flags,
                    gamepad_status,
                    &assets,
                );
                continue;
            }

            if input.open_preferences {
                self.scene = AppScene::Preferences;
                self.preferences_menu.ignore_next_input();
                let mut draw = raylib.begin_drawing(thread);
                render::draw_preferences(
                    &mut draw,
                    &self.preferences_menu,
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
                let mut player_two = if self.feature_flags.enabled(FeatureFlag::PlayerTwoCpu) {
                    self.player_two_cpu
                        .next_player_two_input(&self.world, FIXED_TIMESTEP)
                } else {
                    input.player_two
                };

                if self.feature_flags.enabled(FeatureFlag::PlayerTwoCpu)
                    && !self.feature_flags.enabled(FeatureFlag::CpuCanAttack)
                {
                    player_two = player_two.without_attacks();
                }

                self.world.update_with_flags(
                    FIXED_TIMESTEP,
                    input.player_one,
                    player_two,
                    self.feature_flags,
                );
                self.accumulator -= FIXED_TIMESTEP;
                fixed_steps += 1;
            }

            if fixed_steps == MAX_FIXED_STEPS_PER_FRAME {
                self.accumulator = 0.0;
            }

            let mut draw = raylib.begin_drawing(thread);
            render::draw_fight(
                &mut draw,
                &self.world,
                self.feature_flags,
                gamepad_status,
                &assets,
            );
        }
    }

    fn restart_match(&mut self) {
        self.world = World::new_greybox_with_intro();
        self.player_two_cpu = BasicCpu::default();
        self.accumulator = 0.0;
    }
}
