//! Owns navigation state for the preferences screen.
//!
//! The menu changes feature flags only through the central feature flag API.

use crate::game::feature_flags::{FeatureFlags, PREFERENCE_FLAGS};

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
    CyclePlayerOne(CycleDirection),
    CyclePlayerTwo(CycleDirection),
    ToggleRecording,
}

/// Stateful preferences screen cursor.
#[derive(Clone, Debug, Default)]
pub struct PreferencesMenu {
    selected: usize,
    accepting_input: bool,
}

impl PreferencesMenu {
    pub const START_ROW: usize = 0;
    pub const PLAYER_ONE_CHARACTER_ROW: usize = 1;
    pub const PLAYER_TWO_CHARACTER_ROW: usize = 2;
    pub const RECORDING_ROW: usize = 3;
    pub const FIRST_FLAG_ROW: usize = 4;

    /// Number of selectable rows in the preferences screen.
    pub const fn row_count() -> usize {
        PREFERENCE_FLAGS.len() + Self::FIRST_FLAG_ROW
    }

    /// Returns the selected row index.
    pub const fn selected(&self) -> usize {
        self.selected
    }

    /// Ignores the next frame of input after entering the preferences scene.
    pub fn ignore_next_input(&mut self) {
        self.accepting_input = false;
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
            self.selected = (self.selected + 1).min(Self::row_count() - 1);
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
            if self.selected == Self::START_ROW {
                return PreferencesAction::StartFight;
            }

            if let Some(action) = self.character_cycle_action(CycleDirection::Next) {
                return action;
            }

            if self.selected == Self::RECORDING_ROW {
                return PreferencesAction::ToggleRecording;
            }

            let flag = PREFERENCE_FLAGS[self.selected - Self::FIRST_FLAG_ROW];
            flags.toggle(flag);
        }

        PreferencesAction::Stay
    }

    fn cycle_action(&self, direction: CycleDirection) -> PreferencesAction {
        self.character_cycle_action(direction)
            .unwrap_or(PreferencesAction::Stay)
    }

    fn character_cycle_action(&self, direction: CycleDirection) -> Option<PreferencesAction> {
        match self.selected {
            Self::PLAYER_ONE_CHARACTER_ROW => Some(PreferencesAction::CyclePlayerOne(direction)),
            Self::PLAYER_TWO_CHARACTER_ROW => Some(PreferencesAction::CyclePlayerTwo(direction)),
            _ => None,
        }
    }
}
