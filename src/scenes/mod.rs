//! Defines high-level screen states.
//!
//! Scenes keep application flow explicit without introducing a full screen
//! framework during the prototype.

pub mod combat_lab;
pub mod preferences;

/// Top-level screen currently owned by the application loop.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AppScene {
    Preferences,
    Fight,
    CombatLab,
}
