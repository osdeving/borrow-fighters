//! Maps keyboard state into game commands.
//!
//! This module is the input boundary between Raylib and the testable combat
//! model.

use raylib::prelude::*;

use crate::combat::fighter::FighterInput;

/// Local two-player keyboard input for one simulation step.
#[derive(Clone, Copy, Debug, Default)]
pub struct LocalInput {
    pub player_one: FighterInput,
    pub player_two: FighterInput,
    pub restart: bool,
    pub toggle_cpu: bool,
}

impl LocalInput {
    /// Reads the current keyboard state from Raylib.
    pub fn read(raylib: &RaylibHandle) -> Self {
        Self {
            player_one: FighterInput {
                left: raylib.is_key_down(KeyboardKey::KEY_A),
                right: raylib.is_key_down(KeyboardKey::KEY_D),
                jump: raylib.is_key_pressed(KeyboardKey::KEY_W),
                crouch: raylib.is_key_down(KeyboardKey::KEY_S),
                block: raylib.is_key_down(KeyboardKey::KEY_Q),
                light_punch: raylib.is_key_pressed(KeyboardKey::KEY_F),
                heavy_punch: raylib.is_key_pressed(KeyboardKey::KEY_H),
                kick: raylib.is_key_pressed(KeyboardKey::KEY_V),
                projectile: raylib.is_key_pressed(KeyboardKey::KEY_G),
            },
            player_two: FighterInput {
                left: raylib.is_key_down(KeyboardKey::KEY_LEFT)
                    || raylib.is_key_down(KeyboardKey::KEY_J),
                right: raylib.is_key_down(KeyboardKey::KEY_RIGHT)
                    || raylib.is_key_down(KeyboardKey::KEY_L),
                jump: raylib.is_key_pressed(KeyboardKey::KEY_UP)
                    || raylib.is_key_pressed(KeyboardKey::KEY_I),
                crouch: raylib.is_key_down(KeyboardKey::KEY_DOWN)
                    || raylib.is_key_down(KeyboardKey::KEY_K),
                block: raylib.is_key_down(KeyboardKey::KEY_U),
                light_punch: raylib.is_key_pressed(KeyboardKey::KEY_ENTER)
                    || raylib.is_key_pressed(KeyboardKey::KEY_O),
                heavy_punch: raylib.is_key_pressed(KeyboardKey::KEY_RIGHT_SHIFT)
                    || raylib.is_key_pressed(KeyboardKey::KEY_P),
                kick: raylib.is_key_pressed(KeyboardKey::KEY_SEMICOLON)
                    || raylib.is_key_pressed(KeyboardKey::KEY_SLASH),
                projectile: raylib.is_key_pressed(KeyboardKey::KEY_RIGHT_CONTROL)
                    || raylib.is_key_pressed(KeyboardKey::KEY_KP_0),
            },
            restart: raylib.is_key_pressed(KeyboardKey::KEY_R),
            toggle_cpu: raylib.is_key_pressed(KeyboardKey::KEY_C),
        }
    }
}
