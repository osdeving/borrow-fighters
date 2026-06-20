//! Runs the application loop for the greybox prototype.
//!
//! This layer owns Raylib handles, translates platform input, advances fixed
//! gameplay steps, and delegates drawing to the render module.

use raylib::prelude::*;

use crate::config::{FIXED_TIMESTEP, MAX_FIXED_STEPS_PER_FRAME, MAX_FRAME_TIME, TARGET_FPS};
use crate::engine::{input::LocalInput, render};
use crate::game::world::World;

/// Top-level application state outside the testable game world.
pub struct App {
    world: World,
    accumulator: f32,
}

impl Default for App {
    fn default() -> Self {
        Self {
            world: World::new_greybox(),
            accumulator: 0.0,
        }
    }
}

impl App {
    /// Runs the Raylib-backed game loop until the window closes.
    pub fn run(mut self, raylib: &mut RaylibHandle, thread: &RaylibThread) {
        raylib.set_target_fps(TARGET_FPS);

        while !raylib.window_should_close() {
            let input = LocalInput::read(raylib);
            if input.restart {
                self.world = World::new_greybox();
                self.accumulator = 0.0;
            }

            self.accumulator += raylib.get_frame_time().min(MAX_FRAME_TIME);
            let mut fixed_steps = 0;

            while self.accumulator >= FIXED_TIMESTEP && fixed_steps < MAX_FIXED_STEPS_PER_FRAME {
                self.world
                    .update(FIXED_TIMESTEP, input.player_one, input.player_two);
                self.accumulator -= FIXED_TIMESTEP;
                fixed_steps += 1;
            }

            if fixed_steps == MAX_FIXED_STEPS_PER_FRAME {
                self.accumulator = 0.0;
            }

            let mut draw = raylib.begin_drawing(thread);
            render::draw(&mut draw, &self.world);
        }
    }
}
