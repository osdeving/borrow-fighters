//! Maps local keyboard and gamepad state into game commands.
//!
//! This module is the input boundary between Raylib and the testable combat
//! model.

use raylib::prelude::*;

use crate::combat::fighter::FighterInput;
use crate::engine::gamepad;

/// Local two-player input for one simulation step.
#[derive(Clone, Copy, Debug, Default)]
pub struct LocalInput {
    pub player_one: FighterInput,
    pub player_two: FighterInput,
    pub restart: bool,
    pub toggle_cpu: bool,
    pub player_one_gamepad_connected: bool,
    pub player_two_gamepad_connected: bool,
}

impl LocalInput {
    /// Reads the current keyboard and gamepad state from Raylib.
    pub fn read(raylib: &RaylibHandle) -> Self {
        let keyboard_player_one = keyboard_player_one(raylib);
        let keyboard_player_two = keyboard_player_two(raylib);
        let gamepad_player_one =
            gamepad::read_fighter_input(raylib, gamepad::PLAYER_ONE_GAMEPAD).unwrap_or_default();
        let gamepad_player_two =
            gamepad::read_fighter_input(raylib, gamepad::PLAYER_TWO_GAMEPAD).unwrap_or_default();
        let player_one_gamepad_connected =
            gamepad::is_connected(raylib, gamepad::PLAYER_ONE_GAMEPAD);
        let player_two_gamepad_connected =
            gamepad::is_connected(raylib, gamepad::PLAYER_TWO_GAMEPAD);

        Self {
            player_one: merge_fighter_input(keyboard_player_one, gamepad_player_one),
            player_two: merge_fighter_input(keyboard_player_two, gamepad_player_two),
            restart: raylib.is_key_pressed(KeyboardKey::KEY_R)
                || gamepad::restart_pressed(raylib, gamepad::PLAYER_ONE_GAMEPAD)
                || gamepad::restart_pressed(raylib, gamepad::PLAYER_TWO_GAMEPAD),
            toggle_cpu: raylib.is_key_pressed(KeyboardKey::KEY_C)
                || gamepad::toggle_cpu_pressed(raylib, gamepad::PLAYER_ONE_GAMEPAD)
                || gamepad::toggle_cpu_pressed(raylib, gamepad::PLAYER_TWO_GAMEPAD),
            player_one_gamepad_connected,
            player_two_gamepad_connected,
        }
    }
}

fn keyboard_player_one(raylib: &RaylibHandle) -> FighterInput {
    FighterInput {
        left: raylib.is_key_down(KeyboardKey::KEY_A),
        right: raylib.is_key_down(KeyboardKey::KEY_D),
        jump: raylib.is_key_pressed(KeyboardKey::KEY_W),
        crouch: raylib.is_key_down(KeyboardKey::KEY_S),
        block: raylib.is_key_down(KeyboardKey::KEY_Q),
        light_punch: raylib.is_key_pressed(KeyboardKey::KEY_F),
        heavy_punch: raylib.is_key_pressed(KeyboardKey::KEY_H),
        kick: raylib.is_key_pressed(KeyboardKey::KEY_V),
        projectile: raylib.is_key_pressed(KeyboardKey::KEY_G),
    }
}

fn keyboard_player_two(raylib: &RaylibHandle) -> FighterInput {
    FighterInput {
        left: raylib.is_key_down(KeyboardKey::KEY_LEFT) || raylib.is_key_down(KeyboardKey::KEY_J),
        right: raylib.is_key_down(KeyboardKey::KEY_RIGHT) || raylib.is_key_down(KeyboardKey::KEY_L),
        jump: raylib.is_key_pressed(KeyboardKey::KEY_UP)
            || raylib.is_key_pressed(KeyboardKey::KEY_I),
        crouch: raylib.is_key_down(KeyboardKey::KEY_DOWN) || raylib.is_key_down(KeyboardKey::KEY_K),
        block: raylib.is_key_down(KeyboardKey::KEY_U),
        light_punch: raylib.is_key_pressed(KeyboardKey::KEY_ENTER)
            || raylib.is_key_pressed(KeyboardKey::KEY_O),
        heavy_punch: raylib.is_key_pressed(KeyboardKey::KEY_RIGHT_SHIFT)
            || raylib.is_key_pressed(KeyboardKey::KEY_P),
        kick: raylib.is_key_pressed(KeyboardKey::KEY_SEMICOLON)
            || raylib.is_key_pressed(KeyboardKey::KEY_SLASH),
        projectile: raylib.is_key_pressed(KeyboardKey::KEY_RIGHT_CONTROL)
            || raylib.is_key_pressed(KeyboardKey::KEY_KP_0),
    }
}

fn merge_fighter_input(first: FighterInput, second: FighterInput) -> FighterInput {
    FighterInput {
        left: first.left || second.left,
        right: first.right || second.right,
        jump: first.jump || second.jump,
        crouch: first.crouch || second.crouch,
        block: first.block || second.block,
        light_punch: first.light_punch || second.light_punch,
        heavy_punch: first.heavy_punch || second.heavy_punch,
        kick: first.kick || second.kick,
        projectile: first.projectile || second.projectile,
    }
}
