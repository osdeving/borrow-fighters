//! Owns navigation state for the preferences screen.
//!
//! The menu changes feature flags only through the central feature flag API.

use crate::game::feature_flags::{FeatureFlags, PREFERENCE_FLAGS};
use crate::ui::binary_text::DEFAULT_BINARY_REVEAL_FRAMES;

/// Top-level prototype menu pages.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MenuPage {
    #[default]
    Main,
    Versus,
    Training,
    Lore,
    Options,
}

/// Menu input commands for one frame.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PreferencesInput {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub scroll_up: bool,
    pub scroll_down: bool,
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
    CycleLoreChapter(CycleDirection),
    CycleLoreCharacter(CycleDirection),
    CyclePlayerOne(CycleDirection),
    CyclePlayerTwo(CycleDirection),
    CycleArena(CycleDirection),
    AdjustMusicVolume(CycleDirection),
    ToggleRecording,
    Exit,
}

/// Stateful preferences screen cursor.
#[derive(Clone, Debug, Default)]
pub struct PreferencesMenu {
    page: MenuPage,
    selected: usize,
    lore_chapter: usize,
    lore_character: usize,
    lore_chapter_scroll: usize,
    lore_character_scroll: usize,
    accepting_input: bool,
    selection_pulse_frames: u16,
}

const LORE_SCROLL_STEP_LINES: usize = 3;

impl PreferencesMenu {
    pub const MAIN_START_ROW: usize = 0;
    pub const MAIN_VERSUS_ROW: usize = 1;
    pub const MAIN_TRAINING_ROW: usize = 2;
    pub const MAIN_LORE_ROW: usize = 3;
    pub const MAIN_OPTIONS_ROW: usize = 4;
    pub const MAIN_EXIT_ROW: usize = 5;
    pub const VERSUS_START_ROW: usize = 0;
    pub const VERSUS_PLAYER_ONE_CHARACTER_ROW: usize = 1;
    pub const VERSUS_PLAYER_TWO_CHARACTER_ROW: usize = 2;
    pub const VERSUS_ARENA_ROW: usize = 3;
    pub const VERSUS_BACK_ROW: usize = 4;
    pub const TRAINING_COMBAT_LAB_ROW: usize = 0;
    pub const TRAINING_SPRITE_VIEWER_ROW: usize = 1;
    pub const TRAINING_BACK_ROW: usize = 2;
    pub const LORE_CHAPTER_ROW: usize = 0;
    pub const LORE_CHARACTER_ROW: usize = 1;
    pub const LORE_BACK_ROW: usize = 2;
    pub const OPTIONS_RECORDING_ROW: usize = 0;
    pub const OPTIONS_MUSIC_VOLUME_ROW: usize = 1;
    pub const OPTIONS_FIRST_FLAG_ROW: usize = 2;

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

    /// Returns the selected lore chapter index.
    pub const fn lore_chapter(&self) -> usize {
        self.lore_chapter
    }

    /// Returns the selected lore character index.
    pub const fn lore_character(&self) -> usize {
        self.lore_character
    }

    /// Returns the current lore chapter scroll offset in text lines.
    pub const fn lore_chapter_scroll(&self) -> usize {
        self.lore_chapter_scroll
    }

    /// Returns the current lore character profile scroll offset in text lines.
    pub const fn lore_character_scroll(&self) -> usize {
        self.lore_character_scroll
    }

    /// Remaining frames for the selected-row binary reveal animation.
    pub const fn selection_pulse_frames(&self) -> u16 {
        self.selection_pulse_frames
    }

    /// Advances non-gameplay menu visuals by one rendered frame.
    pub fn tick_visuals(&mut self) {
        self.selection_pulse_frames = self.selection_pulse_frames.saturating_sub(1);
    }

    /// Ignores the next frame of input after entering the preferences scene.
    pub fn ignore_next_input(&mut self) {
        self.accepting_input = false;
    }

    /// Moves back one page when possible.
    pub fn back(&mut self) -> bool {
        match self.page {
            MenuPage::Main => false,
            MenuPage::Versus | MenuPage::Training | MenuPage::Lore | MenuPage::Options => {
                self.enter_page(MenuPage::Main);
                true
            }
        }
    }

    /// Cycles the selected lore chapter with a runtime count from the lore file.
    pub fn cycle_lore_chapter(&mut self, direction: CycleDirection, chapter_count: usize) {
        self.lore_chapter = cycle_index(self.lore_chapter, direction, chapter_count.max(1));
        self.lore_chapter_scroll = 0;
    }

    /// Cycles the selected lore character with a runtime count from the lore file.
    pub fn cycle_lore_character(&mut self, direction: CycleDirection, character_count: usize) {
        self.lore_character = cycle_index(self.lore_character, direction, character_count.max(1));
        self.lore_character_scroll = 0;
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

        let previous_selected = self.selected;

        if input.up {
            self.selected = self.selected.saturating_sub(1);
        }

        if input.down {
            self.selected = (self.selected + 1).min(self.page_row_count() - 1);
        }

        if self.selected != previous_selected {
            self.restart_selection_pulse();
        }

        if input.scroll_up && self.scroll_lore(CycleDirection::Previous) {
            return PreferencesAction::Stay;
        }

        if input.scroll_down && self.scroll_lore(CycleDirection::Next) {
            return PreferencesAction::Stay;
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
        self.discrete_cycle_action(direction)
            .unwrap_or(PreferencesAction::Stay)
    }

    fn discrete_cycle_action(&self, direction: CycleDirection) -> Option<PreferencesAction> {
        match self.page {
            MenuPage::Versus => match self.selected {
                Self::VERSUS_PLAYER_ONE_CHARACTER_ROW => {
                    Some(PreferencesAction::CyclePlayerOne(direction))
                }
                Self::VERSUS_PLAYER_TWO_CHARACTER_ROW => {
                    Some(PreferencesAction::CyclePlayerTwo(direction))
                }
                Self::VERSUS_ARENA_ROW => Some(PreferencesAction::CycleArena(direction)),
                _ => None,
            },
            MenuPage::Lore => match self.selected {
                Self::LORE_CHAPTER_ROW => Some(PreferencesAction::CycleLoreChapter(direction)),
                Self::LORE_CHARACTER_ROW => Some(PreferencesAction::CycleLoreCharacter(direction)),
                _ => None,
            },
            MenuPage::Options => {
                if self.selected == Self::OPTIONS_MUSIC_VOLUME_ROW {
                    Some(PreferencesAction::AdjustMusicVolume(direction))
                } else {
                    None
                }
            }
            MenuPage::Main | MenuPage::Training => None,
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
                Self::MAIN_LORE_ROW => {
                    self.enter_page(MenuPage::Lore);
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
                Self::VERSUS_ARENA_ROW => PreferencesAction::CycleArena(CycleDirection::Next),
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
            MenuPage::Lore => match self.selected {
                Self::LORE_CHAPTER_ROW => PreferencesAction::CycleLoreChapter(CycleDirection::Next),
                Self::LORE_CHARACTER_ROW => {
                    PreferencesAction::CycleLoreCharacter(CycleDirection::Next)
                }
                Self::LORE_BACK_ROW => {
                    self.enter_page(MenuPage::Main);
                    PreferencesAction::Stay
                }
                _ => PreferencesAction::Stay,
            },
            MenuPage::Options => {
                if self.selected == Self::OPTIONS_RECORDING_ROW {
                    return PreferencesAction::ToggleRecording;
                }
                if self.selected == Self::OPTIONS_MUSIC_VOLUME_ROW {
                    return PreferencesAction::AdjustMusicVolume(CycleDirection::Next);
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
        if page == MenuPage::Lore {
            self.lore_chapter_scroll = 0;
            self.lore_character_scroll = 0;
        }
        self.restart_selection_pulse();
    }

    fn scroll_lore(&mut self, direction: CycleDirection) -> bool {
        if self.page != MenuPage::Lore {
            return false;
        }

        let scroll = match self.selected {
            Self::LORE_CHAPTER_ROW => &mut self.lore_chapter_scroll,
            Self::LORE_CHARACTER_ROW => &mut self.lore_character_scroll,
            _ => return false,
        };

        *scroll = scroll_line_offset(*scroll, direction);
        true
    }

    fn restart_selection_pulse(&mut self) {
        self.selection_pulse_frames = DEFAULT_BINARY_REVEAL_FRAMES;
    }

    fn page_row_count(&self) -> usize {
        match self.page {
            MenuPage::Main => 6,
            MenuPage::Versus => 5,
            MenuPage::Training => 3,
            MenuPage::Lore => 3,
            MenuPage::Options => PREFERENCE_FLAGS.len() + Self::OPTIONS_FIRST_FLAG_ROW + 1,
        }
    }
}

const fn scroll_line_offset(offset: usize, direction: CycleDirection) -> usize {
    match direction {
        CycleDirection::Previous => offset.saturating_sub(LORE_SCROLL_STEP_LINES),
        CycleDirection::Next => offset + LORE_SCROLL_STEP_LINES,
    }
}

const fn cycle_index(index: usize, direction: CycleDirection, count: usize) -> usize {
    match direction {
        CycleDirection::Previous => {
            if index == 0 {
                count - 1
            } else {
                index - 1
            }
        }
        CycleDirection::Next => (index + 1) % count,
    }
}
