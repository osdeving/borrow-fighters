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
}

impl LocalInput {
    /// Reads the current keyboard state from Raylib.
    pub fn read(raylib: &RaylibHandle) -> Self {
        Self {
            player_one: FighterInput {
                left: raylib.is_key_down(KeyboardKey::KEY_A),
                right: raylib.is_key_down(KeyboardKey::KEY_D),
                jump: raylib.is_key_pressed(KeyboardKey::KEY_W),
                attack: raylib.is_key_pressed(KeyboardKey::KEY_F),
            },
            player_two: FighterInput {
                left: raylib.is_key_down(KeyboardKey::KEY_LEFT),
                right: raylib.is_key_down(KeyboardKey::KEY_RIGHT),
                jump: raylib.is_key_pressed(KeyboardKey::KEY_UP),
                attack: raylib.is_key_pressed(KeyboardKey::KEY_ENTER),
            },
            restart: raylib.is_key_pressed(KeyboardKey::KEY_R),
        }
    }
}
