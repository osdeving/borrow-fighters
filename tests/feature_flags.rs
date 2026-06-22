//! Exercises the prototype feature flag and preferences menu contract.

use borrow_fighters::game::feature_flags::{FeatureFlag, FeatureFlags};
use borrow_fighters::scenes::preferences::{
    CycleDirection, PreferencesAction, PreferencesInput, PreferencesMenu,
};

#[test]
fn feature_flags_start_with_playtest_friendly_defaults() {
    let flags = FeatureFlags::default();

    assert!(!flags.enabled(FeatureFlag::PlayerOneCpu));
    assert!(flags.enabled(FeatureFlag::PlayerTwoCpu));
    assert!(flags.enabled(FeatureFlag::CpuCanAttack));
    assert!(flags.enabled(FeatureFlag::PlayerOneTakesDamage));
    assert!(flags.enabled(FeatureFlag::PlayerTwoTakesDamage));
    assert!(flags.enabled(FeatureFlag::ShowHud));
    assert!(!flags.enabled(FeatureFlag::ShowControlsHelp));
    assert!(!flags.enabled(FeatureFlag::ShowCombatDebug));
    assert!(flags.enabled(FeatureFlag::GamepadInput));
}

#[test]
fn feature_flags_toggle_through_central_api() {
    let mut flags = FeatureFlags::default();

    flags.toggle(FeatureFlag::ShowCombatDebug);
    assert!(flags.enabled(FeatureFlag::ShowCombatDebug));

    flags.set(FeatureFlag::ShowCombatDebug, false);
    assert!(!flags.enabled(FeatureFlag::ShowCombatDebug));
}

#[test]
fn preferences_menu_toggles_selected_feature_flag() {
    let mut flags = FeatureFlags::default();
    let mut menu = PreferencesMenu::default();

    menu.update(PreferencesInput::default(), &mut flags);
    for _ in 0..PreferencesMenu::FIRST_FLAG_ROW {
        menu.update(
            PreferencesInput {
                down: true,
                ..PreferencesInput::default()
            },
            &mut flags,
        );
    }
    let action = menu.update(
        PreferencesInput {
            activate: true,
            ..PreferencesInput::default()
        },
        &mut flags,
    );

    assert_eq!(action, PreferencesAction::Stay);
    assert!(flags.enabled(FeatureFlag::PlayerOneCpu));
}

#[test]
fn preferences_menu_cycles_character_rows() {
    let mut flags = FeatureFlags::default();
    let mut menu = PreferencesMenu::default();

    menu.update(PreferencesInput::default(), &mut flags);
    menu.update(
        PreferencesInput {
            down: true,
            ..PreferencesInput::default()
        },
        &mut flags,
    );

    assert_eq!(
        menu.update(
            PreferencesInput {
                right: true,
                ..PreferencesInput::default()
            },
            &mut flags,
        ),
        PreferencesAction::CyclePlayerOne(CycleDirection::Next)
    );
    assert_eq!(
        menu.update(
            PreferencesInput {
                left: true,
                ..PreferencesInput::default()
            },
            &mut flags,
        ),
        PreferencesAction::CyclePlayerOne(CycleDirection::Previous)
    );

    menu.update(
        PreferencesInput {
            down: true,
            ..PreferencesInput::default()
        },
        &mut flags,
    );
    assert_eq!(
        menu.update(
            PreferencesInput {
                activate: true,
                ..PreferencesInput::default()
            },
            &mut flags,
        ),
        PreferencesAction::CyclePlayerTwo(CycleDirection::Next)
    );
}

#[test]
fn preferences_menu_start_row_enters_fight() {
    let mut flags = FeatureFlags::default();
    let mut menu = PreferencesMenu::default();

    menu.update(PreferencesInput::default(), &mut flags);
    let action = menu.update(
        PreferencesInput {
            activate: true,
            ..PreferencesInput::default()
        },
        &mut flags,
    );

    assert_eq!(action, PreferencesAction::StartFight);
}

#[test]
fn preferences_menu_recording_row_requests_capture_toggle() {
    let mut flags = FeatureFlags::default();
    let mut menu = PreferencesMenu::default();

    menu.update(PreferencesInput::default(), &mut flags);
    for _ in 0..PreferencesMenu::RECORDING_ROW {
        menu.update(
            PreferencesInput {
                down: true,
                ..PreferencesInput::default()
            },
            &mut flags,
        );
    }
    let action = menu.update(
        PreferencesInput {
            activate: true,
            ..PreferencesInput::default()
        },
        &mut flags,
    );

    assert_eq!(action, PreferencesAction::ToggleRecording);
}

#[test]
fn preferences_menu_ignores_first_frame_input() {
    let mut flags = FeatureFlags::default();
    let mut menu = PreferencesMenu::default();

    let action = menu.update(
        PreferencesInput {
            start: true,
            ..PreferencesInput::default()
        },
        &mut flags,
    );

    assert_eq!(action, PreferencesAction::Stay);
    assert_eq!(menu.selected(), 0);
}
