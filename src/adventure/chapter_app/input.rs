//! Translates fresh device edges into chapter commands and menu navigation.
//!
//! System: Adventure input boundary. Held movement survives fixed-step batching;
//! one-shot actions are consumed once and discarded when control is suspended.

use crate::adventure::chapter::ChapterInput;
use raylib::prelude::*;

#[derive(Default)]
pub(super) struct Controls {
    pub chapter: ChapterInput,
    pub pause: bool,
    pub confirm: bool,
    pub back: bool,
    pub up: bool,
    pub down: bool,
    pub pointer: Option<Vector2>,
    pub pointer_moved: bool,
    pub click: bool,
}

pub(super) fn read(rl: &RaylibHandle) -> Controls {
    let pad = |button| rl.is_gamepad_available(0) && rl.is_gamepad_button_pressed(0, button);
    let held = |button| rl.is_gamepad_available(0) && rl.is_gamepad_button_down(0, button);
    let pressed = |keys: &[KeyboardKey]| keys.iter().any(|k| rl.is_key_pressed(*k));
    let down = |keys: &[KeyboardKey]| keys.iter().any(|k| rl.is_key_down(*k));
    use GamepadButton::*;
    use KeyboardKey::*;
    let mut movement = f32::from(down(&[KEY_D, KEY_RIGHT]) || held(GAMEPAD_BUTTON_LEFT_FACE_RIGHT))
        - f32::from(down(&[KEY_A, KEY_LEFT]) || held(GAMEPAD_BUTTON_LEFT_FACE_LEFT));
    if movement == 0.0 && rl.is_gamepad_available(0) {
        let axis = rl.get_gamepad_axis_movement(0, GamepadAxis::GAMEPAD_AXIS_LEFT_X);
        if axis.abs() > 0.2 {
            movement = axis;
        }
    }
    let scale = (rl.get_screen_width() as f32 / 1280.0).min(rl.get_screen_height() as f32 / 720.0);
    let mouse = rl.get_mouse_position();
    let offset = Vector2::new(
        (rl.get_screen_width() as f32 - 1280.0 * scale) * 0.5,
        (rl.get_screen_height() as f32 - 720.0 * scale) * 0.5,
    );
    Controls {
        pointer: (rl.is_window_focused() && rl.is_cursor_on_screen() && scale > 0.0)
            .then(|| Vector2::new((mouse.x - offset.x) / scale, (mouse.y - offset.y) / scale)),
        pointer_moved: rl.get_mouse_delta().length_sqr() > 0.0,
        click: rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT),
        chapter: ChapterInput {
            movement,
            jump: pressed(&[KEY_SPACE, KEY_W, KEY_UP]) || pad(GAMEPAD_BUTTON_RIGHT_FACE_RIGHT),
            light: pressed(&[KEY_J, KEY_F]) || pad(GAMEPAD_BUTTON_RIGHT_FACE_LEFT),
            heavy: pressed(&[KEY_K, KEY_H]) || pad(GAMEPAD_BUTTON_RIGHT_FACE_UP),
            block: down(&[KEY_Q, KEY_L]) || held(GAMEPAD_BUTTON_LEFT_TRIGGER_1),
            interact: pressed(&[KEY_E]) || pad(GAMEPAD_BUTTON_RIGHT_FACE_DOWN),
            advance: pressed(&[KEY_ENTER]) || pad(GAMEPAD_BUTTON_RIGHT_TRIGGER_1),
            skip: pressed(&[KEY_BACKSPACE]) || pad(GAMEPAD_BUTTON_MIDDLE_LEFT),
            retry: pressed(&[KEY_R]) || pad(GAMEPAD_BUTTON_RIGHT_FACE_DOWN),
        },
        pause: pressed(&[KEY_ESCAPE]) || pad(GAMEPAD_BUTTON_MIDDLE_RIGHT),
        confirm: pressed(&[KEY_ENTER]) || pad(GAMEPAD_BUTTON_RIGHT_FACE_DOWN),
        back: pressed(&[KEY_ESCAPE]) || pad(GAMEPAD_BUTTON_RIGHT_FACE_RIGHT),
        up: pressed(&[KEY_W, KEY_UP]) || pad(GAMEPAD_BUTTON_LEFT_FACE_UP),
        down: pressed(&[KEY_S, KEY_DOWN]) || pad(GAMEPAD_BUTTON_LEFT_FACE_DOWN),
    }
}

pub(super) fn merge(pending: &mut ChapterInput, next: ChapterInput) {
    pending.movement = next.movement;
    pending.block = next.block;
    pending.jump |= next.jump;
    pending.light |= next.light;
    pending.heavy |= next.heavy;
    pending.interact |= next.interact;
    pending.advance |= next.advance;
    pending.skip |= next.skip;
    pending.retry |= next.retry;
}

pub(super) fn consumed(input: ChapterInput) -> ChapterInput {
    ChapterInput {
        movement: input.movement,
        block: input.block,
        ..ChapterInput::default()
    }
}
