//! Owns navigation state for the preferences screen.
//!
//! The menu changes feature flags only through the central feature flag API.

use crate::game::feature_flags::{FeatureFlags, PREFERENCE_FLAGS};

/// Top-level prototype menu pages.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MenuPage {
    #[default]
    Main,
    Versus,
    Training,
    Options,
}

/// Menu input commands for one frame.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PreferencesInput {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub activate: bool,
    pub start: bool,
}

/// Direction used by menu rows that cycle through discrete options.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CycleDirection {
    Previous,
    Next,
}

/// Result of handling one preferences input frame.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PreferencesAction {
    Stay,
    StartFight,
    OpenCombatLab,
    OpenSpriteViewer,
    CyclePlayerOne(CycleDirection),
    CyclePlayerTwo(CycleDirection),
    ToggleRecording,
    Exit,
}

/// Stateful preferences screen cursor.
#[derive(Clone, Debug, Default)]
pub struct PreferencesMenu {
    page: MenuPage,
    selected: usize,
    accepting_input: bool,
}

impl PreferencesMenu {
    pub const MAIN_START_ROW: usize = 0;
    pub const MAIN_VERSUS_ROW: usize = 1;
    pub const MAIN_TRAINING_ROW: usize = 2;
    pub const MAIN_OPTIONS_ROW: usize = 3;
    pub const MAIN_EXIT_ROW: usize = 4;
    pub const VERSUS_START_ROW: usize = 0;
    pub const VERSUS_PLAYER_ONE_CHARACTER_ROW: usize = 1;
    pub const VERSUS_PLAYER_TWO_CHARACTER_ROW: usize = 2;
    pub const VERSUS_BACK_ROW: usize = 3;
    pub const TRAINING_COMBAT_LAB_ROW: usize = 0;
    pub const TRAINING_SPRITE_VIEWER_ROW: usize = 1;
    pub const TRAINING_BACK_ROW: usize = 2;
    pub const OPTIONS_RECORDING_ROW: usize = 0;
    pub const OPTIONS_FIRST_FLAG_ROW: usize = 1;

    /// Number of selectable rows in the preferences screen.
    pub fn row_count(&self) -> usize {
        self.page_row_count()
    }

    /// Returns the active menu page.
    pub const fn page(&self) -> MenuPage {
        self.page
    }

    /// Returns the selected row index.
    pub const fn selected(&self) -> usize {
        self.selected
    }

    /// Ignores the next frame of input after entering the preferences scene.
    pub fn ignore_next_input(&mut self) {
        self.accepting_input = false;
    }

    /// Moves back one page when possible.
    pub fn back(&mut self) -> bool {
        match self.page {
            MenuPage::Main => false,
            MenuPage::Versus | MenuPage::Training | MenuPage::Options => {
                self.enter_page(MenuPage::Main);
                true
            }
        }
    }

    /// Handles menu input and mutates feature flags when a toggle row is used.
    pub fn update(
        &mut self,
        input: PreferencesInput,
        flags: &mut FeatureFlags,
    ) -> PreferencesAction {
        if !self.accepting_input {
            self.accepting_input = true;
            return PreferencesAction::Stay;
        }

        if input.up {
            self.selected = self.selected.saturating_sub(1);
        }

        if input.down {
            self.selected = (self.selected + 1).min(self.page_row_count() - 1);
        }

        if input.start {
            return PreferencesAction::StartFight;
        }

        if input.left {
            return self.cycle_action(CycleDirection::Previous);
        }

        if input.right {
            return self.cycle_action(CycleDirection::Next);
        }

        if input.activate {
            return self.activate_selected(flags);
        }

        PreferencesAction::Stay
    }

    fn cycle_action(&self, direction: CycleDirection) -> PreferencesAction {
        self.character_cycle_action(direction)
            .unwrap_or(PreferencesAction::Stay)
    }

    fn character_cycle_action(&self, direction: CycleDirection) -> Option<PreferencesAction> {
        if self.page != MenuPage::Versus {
            return None;
        }

        match self.selected {
            Self::VERSUS_PLAYER_ONE_CHARACTER_ROW => {
                Some(PreferencesAction::CyclePlayerOne(direction))
            }
            Self::VERSUS_PLAYER_TWO_CHARACTER_ROW => {
                Some(PreferencesAction::CyclePlayerTwo(direction))
            }
            _ => None,
        }
    }

    fn activate_selected(&mut self, flags: &mut FeatureFlags) -> PreferencesAction {
        match self.page {
            MenuPage::Main => match self.selected {
                Self::MAIN_START_ROW => PreferencesAction::StartFight,
                Self::MAIN_VERSUS_ROW => {
                    self.enter_page(MenuPage::Versus);
                    PreferencesAction::Stay
                }
                Self::MAIN_TRAINING_ROW => {
                    self.enter_page(MenuPage::Training);
                    PreferencesAction::Stay
                }
                Self::MAIN_OPTIONS_ROW => {
                    self.enter_page(MenuPage::Options);
                    PreferencesAction::Stay
                }
                Self::MAIN_EXIT_ROW => PreferencesAction::Exit,
                _ => PreferencesAction::Stay,
            },
            MenuPage::Versus => match self.selected {
                Self::VERSUS_START_ROW => PreferencesAction::StartFight,
                Self::VERSUS_PLAYER_ONE_CHARACTER_ROW => {
                    PreferencesAction::CyclePlayerOne(CycleDirection::Next)
                }
                Self::VERSUS_PLAYER_TWO_CHARACTER_ROW => {
                    PreferencesAction::CyclePlayerTwo(CycleDirection::Next)
                }
                Self::VERSUS_BACK_ROW => {
                    self.enter_page(MenuPage::Main);
                    PreferencesAction::Stay
                }
                _ => PreferencesAction::Stay,
            },
            MenuPage::Training => match self.selected {
                Self::TRAINING_COMBAT_LAB_ROW => PreferencesAction::OpenCombatLab,
                Self::TRAINING_SPRITE_VIEWER_ROW => PreferencesAction::OpenSpriteViewer,
                Self::TRAINING_BACK_ROW => {
                    self.enter_page(MenuPage::Main);
                    PreferencesAction::Stay
                }
                _ => PreferencesAction::Stay,
            },
            MenuPage::Options => {
                if self.selected == Self::OPTIONS_RECORDING_ROW {
                    return PreferencesAction::ToggleRecording;
                }
                if self.selected == self.page_row_count() - 1 {
                    self.enter_page(MenuPage::Main);
                    return PreferencesAction::Stay;
                }

                let flag = PREFERENCE_FLAGS[self.selected - Self::OPTIONS_FIRST_FLAG_ROW];
                flags.toggle(flag);
                PreferencesAction::Stay
            }
        }
    }

    fn enter_page(&mut self, page: MenuPage) {
        self.page = page;
        self.selected = 0;
    }

    fn page_row_count(&self) -> usize {
        match self.page {
            MenuPage::Main => 5,
            MenuPage::Versus => 4,
            MenuPage::Training => 3,
            MenuPage::Options => PREFERENCE_FLAGS.len() + Self::OPTIONS_FIRST_FLAG_ROW + 1,
        }
    }
}
