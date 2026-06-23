//! Registers prototype characters and their move lists.
//!
//! System: Character data. This module describes which combat data belongs to
//! each character without resolving attacks, drawing sprites, or owning match
//! state.

mod body_metrics;

pub use body_metrics::{
    CHARACTER_BODY_METRICS_PATH, CharacterBodyMetricsCatalog, CharacterBodyMetricsError,
};

use crate::combat::{
    fighter::FighterBodyMetrics,
    move_data::MoveId,
    projectile::{
        C_PROJECTILE_SPEC, DUKE_PROJECTILE_SPEC, GO_PROJECTILE_SPEC, PYTHON_PROJECTILE_SPEC,
        ProjectileSpec, RUST_PROJECTILE_SPEC,
    },
};

const RUST_STATS: CharacterStats = CharacterStats { max_health: 100 };
const DUKE_STATS: CharacterStats = CharacterStats { max_health: 112 };
const GO_STATS: CharacterStats = CharacterStats { max_health: 92 };
const C_STATS: CharacterStats = CharacterStats { max_health: 104 };
const PYTHON_STATS: CharacterStats = CharacterStats { max_health: 96 };
const RUST_BODY_METRICS: FighterBodyMetrics = FighterBodyMetrics::DEFAULT;
const DUKE_BODY_METRICS: FighterBodyMetrics = FighterBodyMetrics::DEFAULT;
const GO_BODY_METRICS: FighterBodyMetrics = FighterBodyMetrics::DEFAULT;
const C_BODY_METRICS: FighterBodyMetrics = FighterBodyMetrics::DEFAULT;
const PYTHON_BODY_METRICS: FighterBodyMetrics = FighterBodyMetrics::DEFAULT;
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
const C_MOVE_IDS: [MoveId; 9] = [
    MoveId::LightPunch,
    MoveId::HeavyPunch,
    MoveId::Kick,
    MoveId::SweepKick,
    MoveId::OverheadPunch,
    MoveId::RisingAntiAir,
    MoveId::AirPunch,
    MoveId::AirKick,
    MoveId::CloseThrow,
];
const PYTHON_MOVE_IDS: [MoveId; 9] = [
    MoveId::LightPunch,
    MoveId::HeavyPunch,
    MoveId::Kick,
    MoveId::SweepKick,
    MoveId::OverheadPunch,
    MoveId::RisingAntiAir,
    MoveId::AirPunch,
    MoveId::AirKick,
    MoveId::CloseThrow,
];

/// Stable identifier for playable or testable characters.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CharacterId {
    #[default]
    Rust,
    Duke,
    Go,
    C,
    Python,
}

impl CharacterId {
    /// Returns the next prototype roster character.
    pub const fn next(self) -> Self {
        match self {
            Self::Rust => Self::Duke,
            Self::Duke => Self::Go,
            Self::Go => Self::C,
            Self::C => Self::Python,
            Self::Python => Self::Rust,
        }
    }

    /// Returns the previous prototype roster character.
    pub const fn previous(self) -> Self {
        match self {
            Self::Rust => Self::Python,
            Self::Duke => Self::Rust,
            Self::Go => Self::Duke,
            Self::C => Self::Go,
            Self::Python => Self::C,
        }
    }

    /// Parses a CLI character name.
    pub fn from_cli(value: &str) -> Option<Self> {
        match value {
            "rust" | "rustacean" => Some(Self::Rust),
            "duke" | "java" => Some(Self::Duke),
            "go" | "golang" | "gopher" => Some(Self::Go),
            "c" | "langc" | "c-lang" | "clang" => Some(Self::C),
            "python" | "py" | "python.py" | "fei-fei" | "feifei" => Some(Self::Python),
            _ => None,
        }
    }

    /// Returns the stable manifest key used by audio and data files.
    pub const fn audio_key(self) -> &'static str {
        match self {
            Self::Rust => "rust",
            Self::Duke => "duke",
            Self::Go => "go",
            Self::C => "c",
            Self::Python => "python",
        }
    }

    /// Parses a stable manifest key.
    pub fn from_audio_key(value: &str) -> Option<Self> {
        match value {
            "rust" => Some(Self::Rust),
            "duke" | "java" => Some(Self::Duke),
            "go" | "golang" | "gopher" => Some(Self::Go),
            "c" | "langc" | "c-lang" | "clang" => Some(Self::C),
            "python" | "py" | "python.py" => Some(Self::Python),
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
    pub body_metrics: FighterBodyMetrics,
    pub move_ids: &'static [MoveId],
    pub projectile: ProjectileSpec,
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
            body_metrics: RUST_BODY_METRICS,
            move_ids: &RUST_MOVE_IDS,
            projectile: RUST_PROJECTILE_SPEC,
        },
        CharacterId::Duke => CharacterSpec {
            id,
            display_name: "Duke / Java",
            fighter_name: "Java",
            archetype: CharacterArchetype::MidrangePressure,
            stats: DUKE_STATS,
            body_metrics: DUKE_BODY_METRICS,
            move_ids: &DUKE_MOVE_IDS,
            projectile: DUKE_PROJECTILE_SPEC,
        },
        CharacterId::Go => CharacterSpec {
            id,
            display_name: "Go",
            fighter_name: "Go",
            archetype: CharacterArchetype::Rushdown,
            stats: GO_STATS,
            body_metrics: GO_BODY_METRICS,
            move_ids: &GO_MOVE_IDS,
            projectile: GO_PROJECTILE_SPEC,
        },
        CharacterId::C => CharacterSpec {
            id,
            display_name: "C",
            fighter_name: "C",
            archetype: CharacterArchetype::MidrangePressure,
            stats: C_STATS,
            body_metrics: C_BODY_METRICS,
            move_ids: &C_MOVE_IDS,
            projectile: C_PROJECTILE_SPEC,
        },
        CharacterId::Python => CharacterSpec {
            id,
            display_name: "Python",
            fighter_name: "Python",
            archetype: CharacterArchetype::AllRounder,
            stats: PYTHON_STATS,
            body_metrics: PYTHON_BODY_METRICS,
            move_ids: &PYTHON_MOVE_IDS,
            projectile: PYTHON_PROJECTILE_SPEC,
        },
    }
}
