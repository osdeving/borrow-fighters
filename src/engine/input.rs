//! Maps local keyboard, mouse and gamepad state into game commands.
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
use crate::scenes::preferences::{PreferencesInput, PreferencesMenu, PreferencesPointerInput};
use crate::ui::menu_layout::MenuLayout;

/// Local two-player input for one simulation step.
#[derive(Clone, Copy, Debug, Default)]
pub struct LocalInput {
    pub player_one: FighterInput,
    pub player_two: FighterInput,
    pub preferences: PreferencesInput,
    pub combat_lab: CombatLabInput,
    pub restart: bool,
    pub pause: bool,
    pub toggle_cpu: bool,
    pub open_preferences: bool,
    pub start_recording: bool,
    pub stop_recording: bool,
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
            restart: raylib.is_key_pressed(KeyboardKey::KEY_R),
            pause: raylib.is_key_pressed(KeyboardKey::KEY_ESCAPE)
                || (gamepad_input_enabled
                    && (gamepad::restart_pressed(raylib, gamepad::PLAYER_ONE_GAMEPAD)
                        || gamepad::restart_pressed(raylib, gamepad::PLAYER_TWO_GAMEPAD))),
            toggle_cpu: raylib.is_key_pressed(KeyboardKey::KEY_C)
                || (gamepad_input_enabled
                    && (gamepad::toggle_cpu_pressed(raylib, gamepad::PLAYER_ONE_GAMEPAD)
                        || gamepad::toggle_cpu_pressed(raylib, gamepad::PLAYER_TWO_GAMEPAD))),
            open_preferences: raylib.is_key_pressed(KeyboardKey::KEY_ESCAPE),
            start_recording: raylib.is_key_pressed(KeyboardKey::KEY_F9),
            stop_recording: raylib.is_key_pressed(KeyboardKey::KEY_F10),
            player_one_gamepad_connected,
            player_two_gamepad_connected,
        }
    }
}

/// Reads roster navigation without merging the ownership of local controllers.
pub fn read_character_select_input(
    raylib: &RaylibHandle,
    selection: &crate::scenes::character_select::CharacterSelect,
    gamepad_enabled: bool,
) -> crate::scenes::character_select::SelectInput {
    use crate::scenes::character_select::{SelectCommand, SelectInput};
    use crate::scenes::preferences::PlayMode;
    use crate::ui::roster_layout as layout;
    let key = |key| raylib.is_key_pressed(key);
    let local = selection.mode == PlayMode::LocalDuel;
    let mut result = SelectInput {
        shared: SelectCommand {
            horizontal: if local {
                0
            } else {
                i8::from(key(KeyboardKey::KEY_RIGHT) || key(KeyboardKey::KEY_D))
                    - i8::from(key(KeyboardKey::KEY_LEFT) || key(KeyboardKey::KEY_A))
            },
            vertical: if local {
                0
            } else {
                i8::from(key(KeyboardKey::KEY_DOWN) || key(KeyboardKey::KEY_S))
                    - i8::from(key(KeyboardKey::KEY_UP) || key(KeyboardKey::KEY_W))
            },
            confirm: key(KeyboardKey::KEY_SPACE)
                || (!local && (key(KeyboardKey::KEY_ENTER) || key(KeyboardKey::KEY_F))),
            back: key(KeyboardKey::KEY_ESCAPE),
        },
        cycle_mode: key(KeyboardKey::KEY_TAB),
        arena_direction: i8::from(key(KeyboardKey::KEY_E)) - i8::from(key(KeyboardKey::KEY_Q)),
        ..SelectInput::default()
    };
    if local {
        result.players = [
            SelectCommand {
                horizontal: i8::from(key(KeyboardKey::KEY_D)) - i8::from(key(KeyboardKey::KEY_A)),
                vertical: i8::from(key(KeyboardKey::KEY_S)) - i8::from(key(KeyboardKey::KEY_W)),
                confirm: key(KeyboardKey::KEY_F),
                back: false,
            },
            SelectCommand {
                horizontal: i8::from(key(KeyboardKey::KEY_RIGHT))
                    - i8::from(key(KeyboardKey::KEY_LEFT)),
                vertical: i8::from(key(KeyboardKey::KEY_DOWN)) - i8::from(key(KeyboardKey::KEY_UP)),
                confirm: key(KeyboardKey::KEY_ENTER),
                back: false,
            },
        ];
    }
    if gamepad_enabled {
        // Match setup belongs to P1; P2 keeps independent fighter navigation.
        let primary = gamepad::PLAYER_ONE_GAMEPAD;
        if gamepad::is_connected(raylib, primary) {
            let pressed = |button| raylib.is_gamepad_button_pressed(primary, button);
            result.cycle_mode |= pressed(GamepadButton::GAMEPAD_BUTTON_MIDDLE_LEFT);
            let arena_direction = i8::from(pressed(GamepadButton::GAMEPAD_BUTTON_RIGHT_TRIGGER_1))
                - i8::from(pressed(GamepadButton::GAMEPAD_BUTTON_LEFT_TRIGGER_1));
            result.arena_direction = (result.arena_direction + arena_direction).clamp(-1, 1);
        }
        for owner in 0..2 {
            let pad = owner as i32;
            let command = SelectCommand {
                horizontal: i8::from(gamepad::menu_right_pressed(raylib, pad))
                    - i8::from(gamepad::menu_left_pressed(raylib, pad)),
                vertical: i8::from(gamepad::menu_down_pressed(raylib, pad))
                    - i8::from(gamepad::menu_up_pressed(raylib, pad)),
                confirm: gamepad::menu_activate_pressed(raylib, pad),
                back: gamepad::is_connected(raylib, pad)
                    && raylib.is_gamepad_button_pressed(
                        pad,
                        GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_RIGHT,
                    ),
            };
            let target = if !local && owner == 0 {
                &mut result.shared
            } else {
                &mut result.players[owner]
            };
            target.horizontal = (target.horizontal + command.horizontal).clamp(-1, 1);
            target.vertical = (target.vertical + command.vertical).clamp(-1, 1);
            target.confirm |= command.confirm;
            target.back |= command.back;
            result.launch |= gamepad::menu_start_pressed(raylib, pad);
        }
    }
    if raylib.is_window_focused() && raylib.is_cursor_on_screen() {
        let pos = raylib.get_mouse_position();
        let delta = raylib.get_mouse_delta();
        let clicked = raylib.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT);
        if clicked || delta.x != 0.0 || delta.y != 0.0 {
            result.hover = layout::hovered_cell(pos.x, pos.y);
        }
        if clicked {
            result.click = result.hover.is_some();
            result.owner = (0..2).find(|&owner| layout::preview(owner).contains(pos.x, pos.y));
            result.cycle_mode |= layout::MODE.contains(pos.x, pos.y);
            result.launch |= layout::LAUNCH.contains(pos.x, pos.y);
            result.shared.back |= layout::BACK.contains(pos.x, pos.y);
            if layout::ARENA.contains(pos.x, pos.y) {
                let midpoint = (layout::ARENA.x + layout::ARENA.w / 2) as f32;
                result.arena_direction = if pos.x < midpoint { -1 } else { 1 };
            }
        }
        result.shared.back |= raylib.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_RIGHT);
    }
    result
}

/// Reads mouse navigation only while the pointer belongs to the focused window.
pub fn read_preferences_pointer(
    raylib: &RaylibHandle,
    menu: &PreferencesMenu,
) -> PreferencesPointerInput {
    if !raylib.is_window_focused() || !raylib.is_cursor_on_screen() {
        return PreferencesPointerInput::default();
    }
    let position = raylib.get_mouse_position();
    let delta = raylib.get_mouse_delta();
    PreferencesPointerInput {
        hovered_row: MenuLayout::for_page(menu.page()).hovered_row(
            position.x,
            position.y,
            menu.row_count(),
        ),
        moved: delta.x != 0.0 || delta.y != 0.0,
        activate: raylib.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT),
        previous: raylib.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_RIGHT),
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
        next_pose: raylib.is_key_pressed(KeyboardKey::KEY_PAGE_DOWN),
        previous_pose: raylib.is_key_pressed(KeyboardKey::KEY_PAGE_UP),
        toggle_hurtboxes: raylib.is_key_pressed(KeyboardKey::KEY_H),
        toggle_hitboxes: raylib.is_key_pressed(KeyboardKey::KEY_B),
        toggle_pivot: raylib.is_key_pressed(KeyboardKey::KEY_P),
        toggle_dummy: raylib.is_key_pressed(KeyboardKey::KEY_D),
        toggle_background: raylib.is_key_pressed(KeyboardKey::KEY_A),
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
        signature_special: raylib.is_key_pressed(KeyboardKey::KEY_T),
        cinematic_special: raylib.is_key_pressed(KeyboardKey::KEY_Y),
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
        signature_special: raylib.is_key_pressed(KeyboardKey::KEY_BACKSLASH),
        cinematic_special: raylib.is_key_pressed(KeyboardKey::KEY_RIGHT_BRACKET),
    }
}

fn keyboard_preferences(raylib: &RaylibHandle) -> PreferencesInput {
    let mouse_wheel = if raylib.is_window_focused() && raylib.is_cursor_on_screen() {
        raylib.get_mouse_wheel_move()
    } else {
        0.0
    };
    PreferencesInput {
        up: raylib.is_key_pressed(KeyboardKey::KEY_UP) || raylib.is_key_pressed(KeyboardKey::KEY_W),
        down: raylib.is_key_pressed(KeyboardKey::KEY_DOWN)
            || raylib.is_key_pressed(KeyboardKey::KEY_S),
        left: raylib.is_key_pressed(KeyboardKey::KEY_LEFT)
            || raylib.is_key_pressed(KeyboardKey::KEY_A),
        right: raylib.is_key_pressed(KeyboardKey::KEY_RIGHT)
            || raylib.is_key_pressed(KeyboardKey::KEY_D),
        scroll_up: raylib.is_key_pressed(KeyboardKey::KEY_PAGE_UP) || mouse_wheel > 0.0,
        scroll_down: raylib.is_key_pressed(KeyboardKey::KEY_PAGE_DOWN) || mouse_wheel < 0.0,
        activate: raylib.is_key_pressed(KeyboardKey::KEY_SPACE)
            || raylib.is_key_pressed(KeyboardKey::KEY_ENTER),
        start: false,
        pointer: PreferencesPointerInput::default(),
    }
}

fn gamepad_preferences(raylib: &RaylibHandle) -> PreferencesInput {
    PreferencesInput {
        up: gamepad::menu_up_pressed(raylib, gamepad::PLAYER_ONE_GAMEPAD)
            || gamepad::menu_up_pressed(raylib, gamepad::PLAYER_TWO_GAMEPAD),
        down: gamepad::menu_down_pressed(raylib, gamepad::PLAYER_ONE_GAMEPAD)
            || gamepad::menu_down_pressed(raylib, gamepad::PLAYER_TWO_GAMEPAD),
        left: gamepad::menu_left_pressed(raylib, gamepad::PLAYER_ONE_GAMEPAD)
            || gamepad::menu_left_pressed(raylib, gamepad::PLAYER_TWO_GAMEPAD),
        right: gamepad::menu_right_pressed(raylib, gamepad::PLAYER_ONE_GAMEPAD)
            || gamepad::menu_right_pressed(raylib, gamepad::PLAYER_TWO_GAMEPAD),
        scroll_up: false,
        scroll_down: false,
        activate: gamepad::menu_activate_pressed(raylib, gamepad::PLAYER_ONE_GAMEPAD)
            || gamepad::menu_activate_pressed(raylib, gamepad::PLAYER_TWO_GAMEPAD),
        start: gamepad::menu_start_pressed(raylib, gamepad::PLAYER_ONE_GAMEPAD)
            || gamepad::menu_start_pressed(raylib, gamepad::PLAYER_TWO_GAMEPAD),
        pointer: PreferencesPointerInput::default(),
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
        signature_special: first.signature_special || second.signature_special,
        cinematic_special: first.cinematic_special || second.cinematic_special,
    }
}

fn merge_preferences_input(first: PreferencesInput, second: PreferencesInput) -> PreferencesInput {
    PreferencesInput {
        up: first.up || second.up,
        down: first.down || second.down,
        left: first.left || second.left,
        right: first.right || second.right,
        scroll_up: first.scroll_up || second.scroll_up,
        scroll_down: first.scroll_down || second.scroll_down,
        activate: first.activate || second.activate,
        start: first.start || second.start,
        pointer: PreferencesPointerInput::default(),
    }
}
