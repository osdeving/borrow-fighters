//! Exercises the prototype feature flag and preferences menu contract.

use borrow_fighters::game::feature_flags::{FeatureFlag, FeatureFlags, PREFERENCE_FLAGS};
use borrow_fighters::scenes::preferences::{
    CycleDirection, MenuPage, PlayMode, PreferencesAction, PreferencesInput, PreferencesMenu,
    PreferencesPointerInput,
};
use borrow_fighters::ui::menu_layout::MenuLayout;

fn pointer_on_row(menu: &PreferencesMenu, row: usize, activate: bool) -> PreferencesInput {
    let layout = MenuLayout::for_page(menu.page());
    let bounds = layout.row_bounds(row);
    PreferencesInput {
        pointer: PreferencesPointerInput {
            hovered_row: layout.hovered_row(
                (bounds.x + bounds.width / 2) as f32,
                (bounds.y + bounds.height / 2) as f32,
                menu.row_count(),
            ),
            moved: true,
            activate,
            previous: false,
        },
        ..PreferencesInput::default()
    }
}

#[test]
fn mouse_opens_every_submenu_and_its_back_row() {
    for (row, page) in [
        (PreferencesMenu::MAIN_TRAINING_ROW, MenuPage::Training),
        (PreferencesMenu::MAIN_LORE_ROW, MenuPage::Lore),
        (PreferencesMenu::MAIN_OPTIONS_ROW, MenuPage::Options),
    ] {
        let mut flags = FeatureFlags::default();
        let mut menu = PreferencesMenu::default();
        menu.update(PreferencesInput::default(), &mut flags);
        let click = pointer_on_row(&menu, row, true);
        assert_eq!(menu.update(click, &mut flags), PreferencesAction::Stay);
        assert_eq!(menu.page(), page);
        let click_back = pointer_on_row(&menu, menu.row_count() - 1, true);
        assert_eq!(menu.update(click_back, &mut flags), PreferencesAction::Stay);
        assert_eq!(menu.page(), MenuPage::Main);
    }
}

#[test]
fn mouse_clicks_only_activate_visible_rows_including_exit() {
    let mut flags = FeatureFlags::default();
    let mut menu = PreferencesMenu::default();
    menu.update(PreferencesInput::default(), &mut flags);
    menu.update(
        pointer_on_row(&menu, PreferencesMenu::MAIN_EXIT_ROW, false),
        &mut flags,
    );
    assert_eq!(menu.selected(), PreferencesMenu::MAIN_EXIT_ROW);

    for hovered_row in [None, Some(menu.row_count())] {
        let outside = PreferencesInput {
            pointer: PreferencesPointerInput {
                hovered_row,
                activate: true,
                previous: true,
                ..PreferencesPointerInput::default()
            },
            ..PreferencesInput::default()
        };
        assert_eq!(menu.update(outside, &mut flags), PreferencesAction::Stay);
    }
    assert_eq!(
        menu.update(
            pointer_on_row(&menu, PreferencesMenu::MAIN_EXIT_ROW, true),
            &mut flags
        ),
        PreferencesAction::Exit,
    );
}

#[test]
fn stationary_pointer_preserves_keyboard_selection_until_it_moves_or_clicks() {
    let mut flags = FeatureFlags::default();
    let mut menu = PreferencesMenu::default();
    menu.update(PreferencesInput::default(), &mut flags);
    let hover = pointer_on_row(&menu, PreferencesMenu::MAIN_VERSUS_ROW, false);
    menu.update(hover, &mut flags);
    assert_eq!(menu.selected(), PreferencesMenu::MAIN_VERSUS_ROW);

    let mut keyboard = hover;
    keyboard.down = true;
    menu.update(keyboard, &mut flags);
    assert_eq!(menu.selected(), PreferencesMenu::MAIN_TRAINING_ROW);
    let mut stationary = hover;
    stationary.pointer.moved = false;
    menu.update(stationary, &mut flags);
    assert_eq!(menu.selected(), PreferencesMenu::MAIN_TRAINING_ROW);

    menu.update(hover, &mut flags);
    assert_eq!(menu.selected(), PreferencesMenu::MAIN_VERSUS_ROW);
    let mut click = pointer_on_row(&menu, PreferencesMenu::MAIN_OPTIONS_ROW, true);
    click.pointer.moved = false;
    menu.update(click, &mut flags);
    assert_eq!(menu.page(), MenuPage::Options);
}

#[test]
fn pointer_click_is_suppressed_on_scene_entry_and_does_not_repeat_across_pages() {
    let mut flags = FeatureFlags::default();
    let mut menu = PreferencesMenu::default();
    let click = pointer_on_row(&menu, PreferencesMenu::MAIN_TRAINING_ROW, true);
    assert_eq!(menu.update(click, &mut flags), PreferencesAction::Stay);
    assert_eq!(menu.page(), MenuPage::Main);
    menu.update(click, &mut flags);
    assert_eq!(menu.page(), MenuPage::Training);

    // The engine sends only button-press edges: a held button is inactive.
    let mut held = pointer_on_row(&menu, PreferencesMenu::TRAINING_COMBAT_LAB_ROW, false);
    held.pointer.moved = false;
    assert_eq!(menu.update(held, &mut flags), PreferencesAction::Stay);
    menu.ignore_next_input();
    let click = pointer_on_row(&menu, PreferencesMenu::TRAINING_SPRITE_VIEWER_ROW, true);
    assert_eq!(menu.update(click, &mut flags), PreferencesAction::Stay);
    assert_eq!(menu.selected(), PreferencesMenu::TRAINING_COMBAT_LAB_ROW);
    assert_eq!(
        menu.update(click, &mut flags),
        PreferencesAction::OpenSpriteViewer
    );
}

#[test]
fn mouse_can_open_versus_and_each_training_tool() {
    let mut flags = FeatureFlags::default();
    let mut menu = PreferencesMenu::default();
    menu.update(PreferencesInput::default(), &mut flags);
    assert_eq!(
        menu.update(
            pointer_on_row(&menu, PreferencesMenu::MAIN_VERSUS_ROW, true),
            &mut flags
        ),
        PreferencesAction::OpenCharacterSelect
    );
    menu.update(
        pointer_on_row(&menu, PreferencesMenu::MAIN_TRAINING_ROW, true),
        &mut flags,
    );
    for (row, action) in [
        (
            PreferencesMenu::TRAINING_COMBAT_LAB_ROW,
            PreferencesAction::OpenCombatLab,
        ),
        (
            PreferencesMenu::TRAINING_MOVE_SHOWCASE_ROW,
            PreferencesAction::OpenMoveShowcase,
        ),
        (
            PreferencesMenu::TRAINING_SPRITE_VIEWER_ROW,
            PreferencesAction::OpenSpriteViewer,
        ),
    ] {
        assert_eq!(
            menu.update(pointer_on_row(&menu, row, true), &mut flags),
            action
        );
    }
}

#[test]
fn story_placeholder_is_inert_for_mouse_keyboard_and_controller_commands() {
    for input_kind in 0..6 {
        let mut flags = FeatureFlags::default();
        let original_flags = flags;
        let mut menu = PreferencesMenu::default();
        menu.update(PreferencesInput::default(), &mut flags);
        let input = match input_kind {
            0 => pointer_on_row(&menu, PreferencesMenu::MAIN_STORY_ROW, true),
            1 => PreferencesInput {
                // Keyboard Enter and gamepad A map to this same command.
                activate: true,
                ..PreferencesInput::default()
            },
            2 => PreferencesInput {
                start: true,
                ..PreferencesInput::default()
            },
            3 => PreferencesInput {
                right: true,
                ..PreferencesInput::default()
            },
            4 => PreferencesInput {
                left: true,
                ..PreferencesInput::default()
            },
            _ => {
                let mut input = pointer_on_row(&menu, PreferencesMenu::MAIN_STORY_ROW, false);
                input.pointer.previous = true;
                input
            }
        };
        assert_eq!(menu.update(input, &mut flags), PreferencesAction::Stay);
        assert_eq!(menu.page(), MenuPage::Main);
        assert_eq!(menu.selected(), PreferencesMenu::MAIN_STORY_ROW);
        assert_eq!(flags, original_flags);
    }
}

#[test]
fn main_menu_replays_its_entry_reveal_when_returning_without_mutating_flags() {
    let mut flags = FeatureFlags::default();
    let original_flags = flags;
    let mut menu = PreferencesMenu::default();
    assert_eq!(menu.main_entry_frames(), 0);
    assert!(menu.main_entry_pulse_frames() > 0);
    menu.update(PreferencesInput::default(), &mut flags);
    for _ in 0..60 {
        menu.tick_visuals();
    }
    assert_eq!(menu.main_entry_pulse_frames(), 0);

    menu.update(
        pointer_on_row(&menu, PreferencesMenu::MAIN_TRAINING_ROW, true),
        &mut flags,
    );
    assert!(menu.back());
    assert_eq!(menu.main_entry_frames(), 0);
    assert!(menu.main_entry_pulse_frames() > 0);
    for _ in 0..60 {
        menu.tick_visuals();
    }
    menu.ignore_next_input();
    assert_eq!(menu.main_entry_frames(), 0);
    assert!(menu.main_entry_pulse_frames() > 0);
    assert_eq!(flags, original_flags);
}

#[test]
fn mouse_cycles_lore_and_volume_in_both_directions() {
    for (main_row, row, next, previous) in [
        (
            PreferencesMenu::MAIN_LORE_ROW,
            PreferencesMenu::LORE_CHAPTER_ROW,
            PreferencesAction::CycleLoreChapter(CycleDirection::Next),
            PreferencesAction::CycleLoreChapter(CycleDirection::Previous),
        ),
        (
            PreferencesMenu::MAIN_LORE_ROW,
            PreferencesMenu::LORE_CHARACTER_ROW,
            PreferencesAction::CycleLoreCharacter(CycleDirection::Next),
            PreferencesAction::CycleLoreCharacter(CycleDirection::Previous),
        ),
        (
            PreferencesMenu::MAIN_OPTIONS_ROW,
            PreferencesMenu::OPTIONS_MUSIC_VOLUME_ROW,
            PreferencesAction::AdjustMusicVolume(CycleDirection::Next),
            PreferencesAction::AdjustMusicVolume(CycleDirection::Previous),
        ),
    ] {
        let mut flags = FeatureFlags::default();
        let mut menu = PreferencesMenu::default();
        menu.update(PreferencesInput::default(), &mut flags);
        menu.update(pointer_on_row(&menu, main_row, true), &mut flags);
        assert_eq!(
            menu.update(pointer_on_row(&menu, row, true), &mut flags),
            next
        );
        let mut right_click = pointer_on_row(&menu, row, false);
        right_click.pointer.previous = true;
        assert_eq!(menu.update(right_click, &mut flags), previous);
    }
}

#[test]
fn left_click_toggles_options_while_right_click_leaves_toggles_and_actions_unchanged() {
    let mut flags = FeatureFlags::default();
    let mut menu = PreferencesMenu::default();
    menu.update(PreferencesInput::default(), &mut flags);
    let mut exit_right_click = pointer_on_row(&menu, PreferencesMenu::MAIN_EXIT_ROW, false);
    exit_right_click.pointer.previous = true;
    assert_eq!(
        menu.update(exit_right_click, &mut flags),
        PreferencesAction::Stay
    );
    menu.update(
        pointer_on_row(&menu, PreferencesMenu::MAIN_OPTIONS_ROW, true),
        &mut flags,
    );
    assert_eq!(
        menu.update(
            pointer_on_row(&menu, PreferencesMenu::OPTIONS_RECORDING_ROW, true),
            &mut flags
        ),
        PreferencesAction::ToggleRecording
    );
    for (index, flag) in PREFERENCE_FLAGS.into_iter().enumerate() {
        let initial = flags.enabled(flag);
        let row = index + PreferencesMenu::OPTIONS_FIRST_FLAG_ROW;
        menu.update(pointer_on_row(&menu, row, true), &mut flags);
        assert_ne!(flags.enabled(flag), initial);
        let mut right_click = pointer_on_row(&menu, row, false);
        right_click.pointer.previous = true;
        assert_eq!(
            menu.update(right_click, &mut flags),
            PreferencesAction::Stay
        );
        assert_ne!(flags.enabled(flag), initial);
    }
}

#[test]
fn menu_hit_testing_excludes_padding_gaps_and_edges_on_every_page() {
    for (page, rows) in [
        (MenuPage::Main, 7),
        (MenuPage::Versus, 5),
        (MenuPage::Training, 4),
        (MenuPage::Lore, 3),
        (MenuPage::Options, PREFERENCE_FLAGS.len() + 3),
        (MenuPage::HowToPlay, 4),
    ] {
        let layout = MenuLayout::for_page(page);
        for row in 0..rows {
            let bounds = layout.row_bounds(row);
            let x = bounds.x as f32;
            let y = bounds.y as f32;
            assert_eq!(layout.hovered_row(x, y, rows), Some(row));
            assert_eq!(
                layout.hovered_row(
                    x + bounds.width as f32 - 0.1,
                    y + bounds.height as f32 - 0.1,
                    rows
                ),
                Some(row)
            );
            assert_eq!(layout.hovered_row(x - 1.0, y, rows), None);
            assert_eq!(layout.hovered_row(x + bounds.width as f32, y, rows), None);
            assert_eq!(layout.hovered_row(x, y + bounds.height as f32, rows), None);
        }
        assert_eq!(layout.hovered_row(-1.0, -1.0, rows), None);
    }
}

#[test]
fn feature_flags_start_with_playtest_friendly_defaults() {
    let flags = FeatureFlags::default();

    assert!(flags.enabled(FeatureFlag::PlayerOneCpu));
    assert!(flags.enabled(FeatureFlag::PlayerTwoCpu));
    assert!(flags.enabled(FeatureFlag::CpuCanAttack));
    assert!(flags.enabled(FeatureFlag::PlayerOneTakesDamage));
    assert!(flags.enabled(FeatureFlag::PlayerTwoTakesDamage));
    assert!(flags.enabled(FeatureFlag::ShowHud));
    assert!(!flags.enabled(FeatureFlag::ShowControlsHelp));
    assert!(!flags.enabled(FeatureFlag::ShowCombatDebug));
    assert!(flags.enabled(FeatureFlag::ShowStageLife));
    assert!(flags.enabled(FeatureFlag::GamepadInput));
}

#[test]
fn guide_modes_assign_controls_from_mouse_keyboard_and_gamepad() {
    for (row, mode, p1_cpu, p2_cpu) in [
        (
            PreferencesMenu::GUIDE_CPU_ROW,
            PlayMode::AgainstCpu,
            false,
            true,
        ),
        (
            PreferencesMenu::GUIDE_LOCAL_ROW,
            PlayMode::LocalDuel,
            false,
            false,
        ),
        (
            PreferencesMenu::GUIDE_DEMO_ROW,
            PlayMode::WatchDemo,
            true,
            true,
        ),
    ] {
        for input_kind in 0..3 {
            let mut flags = FeatureFlags::default();
            flags.set(FeatureFlag::ShowCombatDebug, true);
            let mut menu = PreferencesMenu::default();
            menu.open_guide();
            menu.update(PreferencesInput::default(), &mut flags);
            for _ in 0..row {
                menu.update(
                    PreferencesInput {
                        down: true,
                        ..PreferencesInput::default()
                    },
                    &mut flags,
                );
            }
            let input = match input_kind {
                0 => pointer_on_row(&menu, row, true),
                1 => PreferencesInput {
                    activate: true,
                    ..PreferencesInput::default()
                },
                _ => PreferencesInput {
                    start: true,
                    ..PreferencesInput::default()
                },
            };
            assert_eq!(
                menu.update(input, &mut flags),
                PreferencesAction::StartWithMode(mode)
            );
            assert_eq!(menu.page(), MenuPage::Main);
            assert_eq!(flags.enabled(FeatureFlag::PlayerOneCpu), p1_cpu);
            assert_eq!(flags.enabled(FeatureFlag::PlayerTwoCpu), p2_cpu);
            assert!(flags.enabled(FeatureFlag::ShowCombatDebug));
        }
    }
}

#[test]
fn guide_can_be_reopened_and_closed_without_changing_control_assignments() {
    let mut flags = FeatureFlags::default();
    PlayMode::LocalDuel.apply(&mut flags);
    let previous = flags;
    let mut menu = PreferencesMenu::default();
    menu.update(PreferencesInput::default(), &mut flags);
    assert_eq!(
        menu.update(
            pointer_on_row(&menu, PreferencesMenu::MAIN_HOW_TO_PLAY_ROW, true),
            &mut flags
        ),
        PreferencesAction::Stay
    );
    assert_eq!(menu.page(), MenuPage::HowToPlay);
    assert_eq!(
        menu.update(
            pointer_on_row(&menu, PreferencesMenu::GUIDE_BACK_ROW, true),
            &mut flags
        ),
        PreferencesAction::CloseGuide
    );
    assert_eq!(menu.page(), MenuPage::Main);
    assert_eq!(flags, previous);
    menu.open_guide();
    assert!(menu.back());
    assert_eq!(menu.page(), MenuPage::Main);
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
    assert!(!flags.enabled(FeatureFlag::PlayerOneCpu));
}

#[test]
fn versus_opens_character_select_from_mouse_and_confirm() {
    for input_kind in 0..2 {
        let mut flags = FeatureFlags::default();
        PlayMode::LocalDuel.apply(&mut flags);
        flags.set(FeatureFlag::ShowCombatDebug, true);
        let original_flags = flags;
        let mut menu = PreferencesMenu::default();
        menu.update(PreferencesInput::default(), &mut flags);
        for _ in 0..PreferencesMenu::MAIN_VERSUS_ROW {
            menu.update(
                PreferencesInput {
                    down: true,
                    ..PreferencesInput::default()
                },
                &mut flags,
            );
        }
        let input = if input_kind == 0 {
            pointer_on_row(&menu, PreferencesMenu::MAIN_VERSUS_ROW, true)
        } else {
            // Enter and the controller confirm button share this command.
            PreferencesInput {
                activate: true,
                ..PreferencesInput::default()
            }
        };
        assert_eq!(
            menu.update(input, &mut flags),
            PreferencesAction::OpenCharacterSelect
        );
        assert_eq!(menu.page(), MenuPage::Main);
        assert_eq!(flags, original_flags);
    }
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
fn preferences_menu_preserves_start_shortcut_on_versus() {
    let mut flags = FeatureFlags::default();
    let mut menu = PreferencesMenu::default();

    menu.update(PreferencesInput::default(), &mut flags);
    menu.update(
        pointer_on_row(&menu, PreferencesMenu::MAIN_VERSUS_ROW, false),
        &mut flags,
    );
    let action = menu.update(
        PreferencesInput {
            start: true,
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
        PreferencesAction::OpenMoveShowcase
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
