//! Maps Raylib gamepad state into fighter commands.
//!
//! The prototype uses the first connected gamepad for Player 1 and the second
//! connected gamepad for Player 2 when manual Player 2 control is enabled.

use raylib::prelude::*;

use crate::combat::fighter::FighterInput;

pub const PLAYER_ONE_GAMEPAD: i32 = 0;
pub const PLAYER_TWO_GAMEPAD: i32 = 1;

const STICK_DEADZONE: f32 = 0.35;
const TRIGGER_THRESHOLD: f32 = 0.35;

/// Returns true when Raylib sees the gamepad slot as connected.
pub fn is_connected(raylib: &RaylibHandle, gamepad: i32) -> bool {
    raylib.is_gamepad_available(gamepad)
}

/// Reads one connected gamepad as fighting-game commands.
pub fn read_fighter_input(raylib: &RaylibHandle, gamepad: i32) -> Option<FighterInput> {
    if !is_connected(raylib, gamepad) {
        return None;
    }

    let left_x = raylib.get_gamepad_axis_movement(gamepad, GamepadAxis::GAMEPAD_AXIS_LEFT_X);
    let left_y = raylib.get_gamepad_axis_movement(gamepad, GamepadAxis::GAMEPAD_AXIS_LEFT_Y);
    let left_trigger =
        raylib.get_gamepad_axis_movement(gamepad, GamepadAxis::GAMEPAD_AXIS_LEFT_TRIGGER);

    Some(FighterInput {
        left: left_x < -STICK_DEADZONE || button_down(raylib, gamepad, dpad_left()),
        right: left_x > STICK_DEADZONE || button_down(raylib, gamepad, dpad_right()),
        jump: button_pressed(raylib, gamepad, face_down())
            || button_pressed(raylib, gamepad, dpad_up()),
        crouch: left_y > STICK_DEADZONE || button_down(raylib, gamepad, dpad_down()),
        block: left_trigger > TRIGGER_THRESHOLD
            || button_down(raylib, gamepad, left_bumper())
            || button_down(raylib, gamepad, left_trigger_button()),
        light_punch: button_pressed(raylib, gamepad, face_left()),
        heavy_punch: button_pressed(raylib, gamepad, face_up()),
        kick: button_pressed(raylib, gamepad, face_right()),
        projectile: button_pressed(raylib, gamepad, right_bumper())
            || button_pressed(raylib, gamepad, right_trigger_button()),
    })
}

/// Returns true when the gamepad requested a match restart this frame.
pub fn restart_pressed(raylib: &RaylibHandle, gamepad: i32) -> bool {
    is_connected(raylib, gamepad) && button_pressed(raylib, gamepad, start_button())
}

/// Returns true when the gamepad requested CPU/manual toggle this frame.
pub fn toggle_cpu_pressed(raylib: &RaylibHandle, gamepad: i32) -> bool {
    is_connected(raylib, gamepad) && button_pressed(raylib, gamepad, select_button())
}

/// Returns true when the gamepad moved menu selection upward this frame.
pub fn menu_up_pressed(raylib: &RaylibHandle, gamepad: i32) -> bool {
    is_connected(raylib, gamepad) && button_pressed(raylib, gamepad, dpad_up())
}

/// Returns true when the gamepad moved menu selection downward this frame.
pub fn menu_down_pressed(raylib: &RaylibHandle, gamepad: i32) -> bool {
    is_connected(raylib, gamepad) && button_pressed(raylib, gamepad, dpad_down())
}

/// Returns true when the gamepad activated a menu row this frame.
pub fn menu_activate_pressed(raylib: &RaylibHandle, gamepad: i32) -> bool {
    is_connected(raylib, gamepad) && button_pressed(raylib, gamepad, face_down())
}

/// Returns true when the gamepad requested starting the fight this frame.
pub fn menu_start_pressed(raylib: &RaylibHandle, gamepad: i32) -> bool {
    restart_pressed(raylib, gamepad)
}

fn button_down(raylib: &RaylibHandle, gamepad: i32, button: GamepadButton) -> bool {
    raylib.is_gamepad_button_down(gamepad, button)
}

fn button_pressed(raylib: &RaylibHandle, gamepad: i32, button: GamepadButton) -> bool {
    raylib.is_gamepad_button_pressed(gamepad, button)
}

fn dpad_up() -> GamepadButton {
    GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_UP
}

fn dpad_right() -> GamepadButton {
    GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_RIGHT
}

fn dpad_down() -> GamepadButton {
    GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_DOWN
}

fn dpad_left() -> GamepadButton {
    GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_LEFT
}

fn face_up() -> GamepadButton {
    GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_UP
}

fn face_right() -> GamepadButton {
    GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_RIGHT
}

fn face_down() -> GamepadButton {
    GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_DOWN
}

fn face_left() -> GamepadButton {
    GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_LEFT
}

fn left_bumper() -> GamepadButton {
    GamepadButton::GAMEPAD_BUTTON_LEFT_TRIGGER_1
}

fn left_trigger_button() -> GamepadButton {
    GamepadButton::GAMEPAD_BUTTON_LEFT_TRIGGER_2
}

fn right_bumper() -> GamepadButton {
    GamepadButton::GAMEPAD_BUTTON_RIGHT_TRIGGER_1
}

fn right_trigger_button() -> GamepadButton {
    GamepadButton::GAMEPAD_BUTTON_RIGHT_TRIGGER_2
}

fn select_button() -> GamepadButton {
    GamepadButton::GAMEPAD_BUTTON_MIDDLE_LEFT
}

fn start_button() -> GamepadButton {
    GamepadButton::GAMEPAD_BUTTON_MIDDLE_RIGHT
}
