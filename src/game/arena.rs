//! Defines the playable arena rotation.
//!
//! System: Match runtime. This module keeps arena identity and rotation order
//! testable without Raylib textures or render code.

/// Prototype arena identifiers.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArenaId {
    Sirius,
    Fortaleza,
    JavaStreet,
}

impl ArenaId {
    /// First arena used when the application starts.
    pub const STARTING_ARENA: Self = Self::Sirius;

    /// Ordered arena cycle used after each finished match.
    pub const ROTATION: [Self; 3] = [Self::Sirius, Self::Fortaleza, Self::JavaStreet];

    /// Returns the next arena in the prototype rotation.
    pub const fn next(self) -> Self {
        match self {
            Self::Sirius => Self::Fortaleza,
            Self::Fortaleza => Self::JavaStreet,
            Self::JavaStreet => Self::Sirius,
        }
    }

    /// Returns a short label for UI and docs.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Sirius => "Sirius",
            Self::Fortaleza => "Fortaleza Tech Coast",
            Self::JavaStreet => "Java Street",
        }
    }
}
