//! Maps local keyboard and gamepad state into game commands.
//!
//! System: Raylib input boundary. This module translates device state into
//! scene and combat input structs without owning gameplay decisions.
//!
//! This module is the input boundary between Raylib and the testable combat
//! model.

use raylib::prelude::*;

use crate::combat::fighter::FighterInput;
use crate::engine::gamepad;
use crate::scenes::combat_lab::CombatLabInput;
use crate::scenes::preferences::PreferencesInput;

/// Local two-player input for one simulation step.
#[derive(Clone, Copy, Debug, Default)]
pub struct LocalInput {
    pub player_one: FighterInput,
    pub player_two: FighterInput,
    pub preferences: PreferencesInput,
    pub combat_lab: CombatLabInput,
    pub restart: bool,
    pub toggle_cpu: bool,
    pub open_preferences: bool,
    pub player_one_gamepad_connected: bool,
    pub player_two_gamepad_connected: bool,
}

impl LocalInput {
    /// Reads the current keyboard and gamepad state from Raylib.
    pub fn read(raylib: &RaylibHandle, gamepad_input_enabled: bool) -> Self {
        let keyboard_player_one = keyboard_player_one(raylib);
        let keyboard_player_two = keyboard_player_two(raylib);
        let gamepad_player_one = gamepad_input_enabled
            .then(|| gamepad::read_fighter_input(raylib, gamepad::PLAYER_ONE_GAMEPAD))
            .flatten()
            .unwrap_or_default();
        let gamepad_player_two = gamepad_input_enabled
            .then(|| gamepad::read_fighter_input(raylib, gamepad::PLAYER_TWO_GAMEPAD))
            .flatten()
            .unwrap_or_default();
        let player_one_gamepad_connected =
            gamepad::is_connected(raylib, gamepad::PLAYER_ONE_GAMEPAD);
        let player_two_gamepad_connected =
            gamepad::is_connected(raylib, gamepad::PLAYER_TWO_GAMEPAD);
        let keyboard_preferences = keyboard_preferences(raylib);
        let combat_lab = keyboard_combat_lab(raylib);
        let gamepad_preferences = if gamepad_input_enabled {
            gamepad_preferences(raylib)
        } else {
            PreferencesInput::default()
        };

        Self {
            player_one: merge_fighter_input(keyboard_player_one, gamepad_player_one),
            player_two: merge_fighter_input(keyboard_player_two, gamepad_player_two),
            preferences: merge_preferences_input(keyboard_preferences, gamepad_preferences),
            combat_lab,
            restart: raylib.is_key_pressed(KeyboardKey::KEY_R)
                || (gamepad_input_enabled
                    && (gamepad::restart_pressed(raylib, gamepad::PLAYER_ONE_GAMEPAD)
                        || gamepad::restart_pressed(raylib, gamepad::PLAYER_TWO_GAMEPAD))),
            toggle_cpu: raylib.is_key_pressed(KeyboardKey::KEY_C)
                || (gamepad_input_enabled
                    && (gamepad::toggle_cpu_pressed(raylib, gamepad::PLAYER_ONE_GAMEPAD)
                        || gamepad::toggle_cpu_pressed(raylib, gamepad::PLAYER_TWO_GAMEPAD))),
            open_preferences: raylib.is_key_pressed(KeyboardKey::KEY_ESCAPE),
            player_one_gamepad_connected,
            player_two_gamepad_connected,
        }
    }
}

fn keyboard_combat_lab(raylib: &RaylibHandle) -> CombatLabInput {
    let shift_down = raylib.is_key_down(KeyboardKey::KEY_LEFT_SHIFT)
        || raylib.is_key_down(KeyboardKey::KEY_RIGHT_SHIFT);
    let tab_pressed = raylib.is_key_pressed(KeyboardKey::KEY_TAB);

    CombatLabInput {
        next_move: tab_pressed && !shift_down,
        previous_move: tab_pressed && shift_down,
        replay: raylib.is_key_pressed(KeyboardKey::KEY_ENTER),
        pause_toggle: raylib.is_key_pressed(KeyboardKey::KEY_SPACE),
        step_frame: raylib.is_key_pressed(KeyboardKey::KEY_PERIOD),
        reset: raylib.is_key_pressed(KeyboardKey::KEY_HOME),
        toggle_hurtboxes: raylib.is_key_pressed(KeyboardKey::KEY_H),
        toggle_hitboxes: raylib.is_key_pressed(KeyboardKey::KEY_B),
        toggle_pivot: raylib.is_key_pressed(KeyboardKey::KEY_P),
        toggle_dummy: raylib.is_key_pressed(KeyboardKey::KEY_D),
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

fn keyboard_preferences(raylib: &RaylibHandle) -> PreferencesInput {
    PreferencesInput {
        up: raylib.is_key_pressed(KeyboardKey::KEY_UP) || raylib.is_key_pressed(KeyboardKey::KEY_W),
        down: raylib.is_key_pressed(KeyboardKey::KEY_DOWN)
            || raylib.is_key_pressed(KeyboardKey::KEY_S),
        activate: raylib.is_key_pressed(KeyboardKey::KEY_SPACE)
            || raylib.is_key_pressed(KeyboardKey::KEY_ENTER),
        start: raylib.is_key_pressed(KeyboardKey::KEY_ENTER),
    }
}

fn gamepad_preferences(raylib: &RaylibHandle) -> PreferencesInput {
    PreferencesInput {
        up: gamepad::menu_up_pressed(raylib, gamepad::PLAYER_ONE_GAMEPAD)
            || gamepad::menu_up_pressed(raylib, gamepad::PLAYER_TWO_GAMEPAD),
        down: gamepad::menu_down_pressed(raylib, gamepad::PLAYER_ONE_GAMEPAD)
            || gamepad::menu_down_pressed(raylib, gamepad::PLAYER_TWO_GAMEPAD),
        activate: gamepad::menu_activate_pressed(raylib, gamepad::PLAYER_ONE_GAMEPAD)
            || gamepad::menu_activate_pressed(raylib, gamepad::PLAYER_TWO_GAMEPAD),
        start: gamepad::menu_start_pressed(raylib, gamepad::PLAYER_ONE_GAMEPAD)
            || gamepad::menu_start_pressed(raylib, gamepad::PLAYER_TWO_GAMEPAD),
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

fn merge_preferences_input(first: PreferencesInput, second: PreferencesInput) -> PreferencesInput {
    PreferencesInput {
        up: first.up || second.up,
        down: first.down || second.down,
        activate: first.activate || second.activate,
        start: first.start || second.start,
    }
}
