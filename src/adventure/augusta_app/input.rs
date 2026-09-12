//! Maps chapter controls and consumes edges once per fixed update.
//!
//! System: Augusta input boundary. The production simulation receives the same
//! commands as the character lab; dialogue and pause remain host concerns.

use crate::adventure::{augusta::ChapterInput, production::Input};
use raylib::prelude::*;

pub(super) fn read(rl: &RaylibHandle) -> ChapterInput {
    use GamepadButton::*;
    use KeyboardKey::*;
    let pad = |b| rl.is_gamepad_available(0) && rl.is_gamepad_button_pressed(0, b);
    let held = |b| rl.is_gamepad_available(0) && rl.is_gamepad_button_down(0, b);
    let down = |a, b| rl.is_key_down(a) || rl.is_key_down(b);
    let mut movement = f32::from(down(KEY_D, KEY_RIGHT) || held(GAMEPAD_BUTTON_LEFT_FACE_RIGHT))
        - f32::from(down(KEY_A, KEY_LEFT) || held(GAMEPAD_BUTTON_LEFT_FACE_LEFT));
    if movement == 0.0 && rl.is_gamepad_available(0) {
        let axis = rl.get_gamepad_axis_movement(0, GamepadAxis::GAMEPAD_AXIS_LEFT_X);
        if axis.abs() > 0.2 {
            movement = axis;
        }
    }
    ChapterInput {
        combat: Input {
            movement,
            jump: rl.is_key_pressed(KEY_SPACE) || pad(GAMEPAD_BUTTON_RIGHT_FACE_RIGHT),
            light: rl.is_key_pressed(KEY_J) || pad(GAMEPAD_BUTTON_RIGHT_FACE_LEFT),
            kick: rl.is_key_pressed(KEY_V) || pad(GAMEPAD_BUTTON_RIGHT_TRIGGER_2),
            spin: rl.is_key_pressed(KEY_K) || pad(GAMEPAD_BUTTON_RIGHT_FACE_UP),
            linker: rl.is_key_pressed(KEY_L) || pad(GAMEPAD_BUTTON_LEFT_TRIGGER_2),
            guard: rl.is_key_down(KEY_Q) || held(GAMEPAD_BUTTON_LEFT_TRIGGER_1),
        },
        interact: rl.is_key_pressed(KEY_E) || pad(GAMEPAD_BUTTON_RIGHT_FACE_DOWN),
        advance: rl.is_key_pressed(KEY_ENTER) || pad(GAMEPAD_BUTTON_RIGHT_TRIGGER_1),
        skip: rl.is_key_pressed(KEY_BACKSPACE) || pad(GAMEPAD_BUTTON_MIDDLE_LEFT),
        retry: rl.is_key_pressed(KEY_R) || pad(GAMEPAD_BUTTON_RIGHT_FACE_DOWN),
    }
}

pub(super) fn merge(pending: &mut ChapterInput, next: ChapterInput) {
    pending.combat.movement = next.combat.movement;
    pending.combat.guard = next.combat.guard;
    pending.combat.jump |= next.combat.jump;
    pending.combat.light |= next.combat.light;
    pending.combat.kick |= next.combat.kick;
    pending.combat.spin |= next.combat.spin;
    pending.combat.linker |= next.combat.linker;
    pending.interact |= next.interact;
    pending.advance |= next.advance;
    pending.skip |= next.skip;
    pending.retry |= next.retry;
}

pub(super) fn consumed(input: ChapterInput) -> ChapterInput {
    ChapterInput {
        combat: Input {
            movement: input.combat.movement,
            guard: input.combat.guard,
            ..Input::default()
        },
        ..ChapterInput::default()
    }
}
