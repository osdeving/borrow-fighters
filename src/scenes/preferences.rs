//! Owns navigation state for the preferences screen.
//!
//! The menu changes feature flags only through the central feature flag API.

use crate::game::feature_flags::{FeatureFlags, PREFERENCE_FLAGS};

/// Menu input commands for one frame.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PreferencesInput {
    pub up: bool,
    pub down: bool,
    pub activate: bool,
    pub start: bool,
}

/// Result of handling one preferences input frame.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PreferencesAction {
    Stay,
    StartFight,
}

/// Stateful preferences screen cursor.
#[derive(Clone, Debug, Default)]
pub struct PreferencesMenu {
    selected: usize,
    accepting_input: bool,
}

impl PreferencesMenu {
    /// Number of selectable rows in the preferences screen.
    pub const fn row_count() -> usize {
        PREFERENCE_FLAGS.len() + 1
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

        if input.activate {
            if self.selected == 0 {
                return PreferencesAction::StartFight;
            }

            let flag = PREFERENCE_FLAGS[self.selected - 1];
            flags.toggle(flag);
        }

        PreferencesAction::Stay
    }
}
