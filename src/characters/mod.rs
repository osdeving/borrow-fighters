//! Registers prototype characters and their move lists.
//!
//! System: Character data. This module describes which combat data belongs to
//! each character without resolving attacks, drawing sprites, or owning match
//! state.

use crate::combat::move_data::MoveId;

const PROTOTYPE_MOVE_IDS: [MoveId; 3] = [MoveId::LightPunch, MoveId::HeavyPunch, MoveId::Kick];

/// Stable identifier for playable or testable characters.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CharacterId {
    #[default]
    Rust,
    Duke,
}

impl CharacterId {
    /// Parses a CLI character name.
    pub fn from_cli(value: &str) -> Option<Self> {
        match value {
            "rust" | "rustacean" => Some(Self::Rust),
            "duke" | "java" => Some(Self::Duke),
            _ => None,
        }
    }
}

/// Broad gameplay role used for design and lab labeling.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CharacterArchetype {
    AllRounder,
    MidrangePressure,
}

/// Static character data used by gameplay tooling.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CharacterSpec {
    pub id: CharacterId,
    pub display_name: &'static str,
    pub fighter_name: &'static str,
    pub archetype: CharacterArchetype,
    pub move_ids: &'static [MoveId],
}

/// Returns the static character spec for the requested character.
pub const fn character_spec(id: CharacterId) -> CharacterSpec {
    match id {
        CharacterId::Rust => CharacterSpec {
            id,
            display_name: "Rust",
            fighter_name: "Rust",
            archetype: CharacterArchetype::AllRounder,
            move_ids: &PROTOTYPE_MOVE_IDS,
        },
        CharacterId::Duke => CharacterSpec {
            id,
            display_name: "Duke / Java",
            fighter_name: "Java",
            archetype: CharacterArchetype::MidrangePressure,
            move_ids: &PROTOTYPE_MOVE_IDS,
        },
    }
}
