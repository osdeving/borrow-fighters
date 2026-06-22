//! Registers prototype characters and their move lists.
//!
//! System: Character data. This module describes which combat data belongs to
//! each character without resolving attacks, drawing sprites, or owning match
//! state.

use crate::combat::move_data::MoveId;

const RUST_STATS: CharacterStats = CharacterStats { max_health: 100 };
const DUKE_STATS: CharacterStats = CharacterStats { max_health: 112 };
const GO_STATS: CharacterStats = CharacterStats { max_health: 92 };
const RUST_MOVE_IDS: [MoveId; 9] = [
    MoveId::RustBorrowJab,
    MoveId::HeavyPunch,
    MoveId::Kick,
    MoveId::SweepKick,
    MoveId::OverheadPunch,
    MoveId::RustLifetimeAntiAir,
    MoveId::AirPunch,
    MoveId::AirKick,
    MoveId::RustOwnershipThrow,
];
const DUKE_MOVE_IDS: [MoveId; 9] = [
    MoveId::LightPunch,
    MoveId::DukeBoilerplatePoke,
    MoveId::Kick,
    MoveId::DukeGarbageCollectorSweep,
    MoveId::DukeAbstractFactoryOverhead,
    MoveId::RisingAntiAir,
    MoveId::AirPunch,
    MoveId::AirKick,
    MoveId::DukeEnterpriseThrow,
];
const GO_MOVE_IDS: [MoveId; 9] = [
    MoveId::GoGoroutineJab,
    MoveId::HeavyPunch,
    MoveId::GoDeferKick,
    MoveId::SweepKick,
    MoveId::GoChannelOverhead,
    MoveId::RisingAntiAir,
    MoveId::AirPunch,
    MoveId::GoHopkick,
    MoveId::CloseThrow,
];

/// Stable identifier for playable or testable characters.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CharacterId {
    #[default]
    Rust,
    Duke,
    Go,
}

impl CharacterId {
    /// Returns the next prototype roster character.
    pub const fn next(self) -> Self {
        match self {
            Self::Rust => Self::Duke,
            Self::Duke => Self::Go,
            Self::Go => Self::Rust,
        }
    }

    /// Returns the previous prototype roster character.
    pub const fn previous(self) -> Self {
        match self {
            Self::Rust => Self::Go,
            Self::Duke => Self::Rust,
            Self::Go => Self::Duke,
        }
    }

    /// Parses a CLI character name.
    pub fn from_cli(value: &str) -> Option<Self> {
        match value {
            "rust" | "rustacean" => Some(Self::Rust),
            "duke" | "java" => Some(Self::Duke),
            "go" | "golang" | "gopher" => Some(Self::Go),
            _ => None,
        }
    }

    /// Returns the stable manifest key used by audio and data files.
    pub const fn audio_key(self) -> &'static str {
        match self {
            Self::Rust => "rust",
            Self::Duke => "duke",
            Self::Go => "go",
        }
    }

    /// Parses a stable manifest key.
    pub fn from_audio_key(value: &str) -> Option<Self> {
        match value {
            "rust" => Some(Self::Rust),
            "duke" | "java" => Some(Self::Duke),
            "go" | "golang" | "gopher" => Some(Self::Go),
            _ => None,
        }
    }
}

/// Broad gameplay role used for design and lab labeling.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CharacterArchetype {
    AllRounder,
    MidrangePressure,
    Rushdown,
}

/// Tunable character-level stats consumed by match setup.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CharacterStats {
    pub max_health: i32,
}

/// Static character data used by gameplay tooling.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CharacterSpec {
    pub id: CharacterId,
    pub display_name: &'static str,
    pub fighter_name: &'static str,
    pub archetype: CharacterArchetype,
    pub stats: CharacterStats,
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
            stats: RUST_STATS,
            move_ids: &RUST_MOVE_IDS,
        },
        CharacterId::Duke => CharacterSpec {
            id,
            display_name: "Duke / Java",
            fighter_name: "Java",
            archetype: CharacterArchetype::MidrangePressure,
            stats: DUKE_STATS,
            move_ids: &DUKE_MOVE_IDS,
        },
        CharacterId::Go => CharacterSpec {
            id,
            display_name: "Go",
            fighter_name: "Go",
            archetype: CharacterArchetype::Rushdown,
            stats: GO_STATS,
            move_ids: &GO_MOVE_IDS,
        },
    }
}
