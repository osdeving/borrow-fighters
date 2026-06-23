//! Defines prototype-wide constants.
//!
//! Values here are deliberately small and explicit while the project is still a
//! greybox prototype.

pub const WINDOW_TITLE: &str = "Borrow Fighters - Greybox Prototype";
pub const BASE_WINDOW_WIDTH: i32 = 960;
pub const BASE_WINDOW_HEIGHT: i32 = 540;
pub const WINDOW_WIDTH: i32 = 1280;
pub const WINDOW_HEIGHT: i32 = 720;
pub const RESOLUTION_SCALE: f32 = 4.0 / 3.0;
pub const TARGET_FPS: u32 = 60;

pub const FIXED_TIMESTEP: f32 = 1.0 / 60.0;
pub const MAX_FRAME_TIME: f32 = 0.25;
pub const MAX_FIXED_STEPS_PER_FRAME: usize = 5;

pub const ARENA_LEFT: f32 = world_px(32.0);
pub const ARENA_RIGHT: f32 = world_px(928.0);
pub const FLOOR_Y: f32 = world_px(462.0);

pub const fn world_px(value: f32) -> f32 {
    value * RESOLUTION_SCALE
}

pub const fn screen_px(value: i32) -> i32 {
    if value >= 0 {
        (value * 4 + 1) / 3
    } else {
        (value * 4 - 1) / 3
    }
}
