//! Defines prototype-wide constants.
//!
//! Values here are deliberately small and explicit while the project is still a
//! greybox prototype.

pub const WINDOW_TITLE: &str = "Borrow Fighters - Greybox Prototype";
pub const WINDOW_WIDTH: i32 = 960;
pub const WINDOW_HEIGHT: i32 = 540;
pub const TARGET_FPS: u32 = 60;

pub const FIXED_TIMESTEP: f32 = 1.0 / 60.0;
pub const MAX_FRAME_TIME: f32 = 0.25;
pub const MAX_FIXED_STEPS_PER_FRAME: usize = 5;

pub const ARENA_LEFT: f32 = 32.0;
pub const ARENA_RIGHT: f32 = 928.0;
pub const FLOOR_Y: f32 = 462.0;
