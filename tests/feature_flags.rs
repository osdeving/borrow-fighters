//! Exercises the prototype feature flag and preferences menu contract.

use borrow_fighters::game::feature_flags::{FeatureFlag, FeatureFlags};
use borrow_fighters::scenes::preferences::{
    CycleDirection, MenuPage, PreferencesAction, PreferencesInput, PreferencesMenu,
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
    for _ in 0..PreferencesMenu::MAIN_OPTIONS_ROW {
        menu.update(
            PreferencesInput {
                down: true,
                ..PreferencesInput::default()
            },
            &mut flags,
        );
    }
    menu.update(
        PreferencesInput {
            activate: true,
            ..PreferencesInput::default()
        },
        &mut flags,
    );
    for _ in 0..PreferencesMenu::OPTIONS_FIRST_FLAG_ROW {
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
fn preferences_menu_cycles_arena_row() {
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
    menu.update(
        PreferencesInput {
            activate: true,
            ..PreferencesInput::default()
        },
        &mut flags,
    );
    for _ in 0..PreferencesMenu::VERSUS_ARENA_ROW {
        menu.update(
            PreferencesInput {
                down: true,
                ..PreferencesInput::default()
            },
            &mut flags,
        );
    }

    assert_eq!(
        menu.update(
            PreferencesInput {
                right: true,
                ..PreferencesInput::default()
            },
            &mut flags,
        ),
        PreferencesAction::CycleArena(CycleDirection::Next)
    );
    assert_eq!(
        menu.update(
            PreferencesInput {
                activate: true,
                ..PreferencesInput::default()
            },
            &mut flags,
        ),
        PreferencesAction::CycleArena(CycleDirection::Next)
    );
}

#[test]
fn preferences_menu_adjusts_music_volume_row() {
    let mut flags = FeatureFlags::default();
    let mut menu = PreferencesMenu::default();

    menu.update(PreferencesInput::default(), &mut flags);
    for _ in 0..PreferencesMenu::MAIN_OPTIONS_ROW {
        menu.update(
            PreferencesInput {
                down: true,
                ..PreferencesInput::default()
            },
            &mut flags,
        );
    }
    menu.update(
        PreferencesInput {
            activate: true,
            ..PreferencesInput::default()
        },
        &mut flags,
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
                left: true,
                ..PreferencesInput::default()
            },
            &mut flags,
        ),
        PreferencesAction::AdjustMusicVolume(CycleDirection::Previous)
    );
    assert_eq!(
        menu.update(
            PreferencesInput {
                activate: true,
                ..PreferencesInput::default()
            },
            &mut flags,
        ),
        PreferencesAction::AdjustMusicVolume(CycleDirection::Next)
    );
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
    menu.update(
        PreferencesInput {
            activate: true,
            ..PreferencesInput::default()
        },
        &mut flags,
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
    for _ in 0..PreferencesMenu::MAIN_OPTIONS_ROW {
        menu.update(
            PreferencesInput {
                down: true,
                ..PreferencesInput::default()
            },
            &mut flags,
        );
    }
    menu.update(
        PreferencesInput {
            activate: true,
            ..PreferencesInput::default()
        },
        &mut flags,
    );
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
fn preferences_menu_opens_training_tools() {
    let mut flags = FeatureFlags::default();
    let mut menu = PreferencesMenu::default();

    menu.update(PreferencesInput::default(), &mut flags);
    for _ in 0..PreferencesMenu::MAIN_TRAINING_ROW {
        menu.update(
            PreferencesInput {
                down: true,
                ..PreferencesInput::default()
            },
            &mut flags,
        );
    }
    assert_eq!(
        menu.update(
            PreferencesInput {
                activate: true,
                ..PreferencesInput::default()
            },
            &mut flags,
        ),
        PreferencesAction::Stay
    );
    assert_eq!(menu.page(), MenuPage::Training);

    assert_eq!(
        menu.update(
            PreferencesInput {
                activate: true,
                ..PreferencesInput::default()
            },
            &mut flags,
        ),
        PreferencesAction::OpenCombatLab
    );

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
                activate: true,
                ..PreferencesInput::default()
            },
            &mut flags,
        ),
        PreferencesAction::OpenSpriteViewer
    );
}

#[test]
fn preferences_menu_opens_lore_and_cycles_book_entries() {
    let mut flags = FeatureFlags::default();
    let mut menu = PreferencesMenu::default();

    menu.update(PreferencesInput::default(), &mut flags);
    for _ in 0..PreferencesMenu::MAIN_LORE_ROW {
        menu.update(
            PreferencesInput {
                down: true,
                ..PreferencesInput::default()
            },
            &mut flags,
        );
    }
    assert_eq!(
        menu.update(
            PreferencesInput {
                activate: true,
                ..PreferencesInput::default()
            },
            &mut flags,
        ),
        PreferencesAction::Stay
    );
    assert_eq!(menu.page(), MenuPage::Lore);

    assert_eq!(
        menu.update(
            PreferencesInput {
                right: true,
                ..PreferencesInput::default()
            },
            &mut flags,
        ),
        PreferencesAction::CycleLoreChapter(CycleDirection::Next)
    );
    menu.cycle_lore_chapter(CycleDirection::Next, 5);
    assert_eq!(menu.lore_chapter(), 1);

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
        PreferencesAction::CycleLoreCharacter(CycleDirection::Next)
    );
    menu.cycle_lore_character(CycleDirection::Next, 4);
    assert_eq!(menu.lore_character(), 1);
}

#[test]
fn preferences_menu_scrolls_lore_text_areas() {
    let mut flags = FeatureFlags::default();
    let mut menu = PreferencesMenu::default();

    menu.update(PreferencesInput::default(), &mut flags);
    for _ in 0..PreferencesMenu::MAIN_LORE_ROW {
        menu.update(
            PreferencesInput {
                down: true,
                ..PreferencesInput::default()
            },
            &mut flags,
        );
    }
    menu.update(
        PreferencesInput {
            activate: true,
            ..PreferencesInput::default()
        },
        &mut flags,
    );

    menu.update(
        PreferencesInput {
            scroll_down: true,
            ..PreferencesInput::default()
        },
        &mut flags,
    );
    assert!(menu.lore_chapter_scroll() > 0);
    assert_eq!(menu.lore_character_scroll(), 0);

    menu.update(
        PreferencesInput {
            scroll_up: true,
            ..PreferencesInput::default()
        },
        &mut flags,
    );
    assert_eq!(menu.lore_chapter_scroll(), 0);

    menu.update(
        PreferencesInput {
            down: true,
            ..PreferencesInput::default()
        },
        &mut flags,
    );
    menu.update(
        PreferencesInput {
            scroll_down: true,
            ..PreferencesInput::default()
        },
        &mut flags,
    );
    assert!(menu.lore_character_scroll() > 0);

    menu.cycle_lore_character(CycleDirection::Next, 4);
    assert_eq!(menu.lore_character_scroll(), 0);
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

#[test]
fn preferences_menu_restarts_selection_pulse_when_cursor_moves() {
    let mut flags = FeatureFlags::default();
    let mut menu = PreferencesMenu::default();

    menu.update(PreferencesInput::default(), &mut flags);
    assert_eq!(menu.selection_pulse_frames(), 0);

    menu.update(
        PreferencesInput {
            down: true,
            ..PreferencesInput::default()
        },
        &mut flags,
    );
    let pulse_frames = menu.selection_pulse_frames();

    assert!(pulse_frames > 0);
    menu.tick_visuals();
    assert!(menu.selection_pulse_frames() < pulse_frames);
}
