//! Defines table-driven close-range move data.
//!
//! System: Combat data. This module stores tunable move specs consumed by
//! fighters, Combat Lab, and future character specs.
//!
//! Character specs can now select input-compatible move ids with different
//! timings, reach, and damage while keeping the same prototype controls.

use super::frame::FrameCount;

pub const LIGHT_PUNCH_DAMAGE: i32 = 8;
pub const HEAVY_PUNCH_DAMAGE: i32 = 16;
pub const KICK_DAMAGE: i32 = 12;
pub const RUST_BORROW_JAB_DAMAGE: i32 = 7;
pub const DUKE_BOILERPLATE_POKE_DAMAGE: i32 = 18;

/// Stable identifier for a close-range move.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MoveId {
    LightPunch,
    HeavyPunch,
    Kick,
    RustBorrowJab,
    DukeBoilerplatePoke,
}

impl MoveId {
    const fn index(self) -> usize {
        match self {
            Self::LightPunch => 0,
            Self::HeavyPunch => 1,
            Self::Kick => 2,
            Self::RustBorrowJab => 3,
            Self::DukeBoilerplatePoke => 4,
        }
    }
}

/// Default close-range move ids used by current prototype characters.
pub const DEFAULT_CLOSE_RANGE_MOVE_IDS: [MoveId; 3] =
    [MoveId::LightPunch, MoveId::HeavyPunch, MoveId::Kick];

/// Input family that starts a move.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MoveInputKind {
    LightPunch,
    HeavyPunch,
    Kick,
}

/// Whole-frame timing data for one close-range attack.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AttackFrameData {
    pub duration: FrameCount,
    pub active_start: FrameCount,
    pub active_end: FrameCount,
}

/// Local attack box dimensions relative to the fighter body.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HitboxSpec {
    pub width: f32,
    pub height: f32,
    pub y_offset: f32,
}

/// Data required to play and resolve one close-range move.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MoveSpec {
    pub id: MoveId,
    pub input: MoveInputKind,
    pub label: &'static str,
    pub frames: AttackFrameData,
    pub hitbox: HitboxSpec,
    pub damage: i32,
}

/// Prototype 0.1 close-range move table.
pub const CLOSE_RANGE_MOVE_SPECS: [MoveSpec; 5] = [
    MoveSpec {
        id: MoveId::LightPunch,
        input: MoveInputKind::LightPunch,
        label: "LP",
        frames: AttackFrameData {
            duration: FrameCount::new(18),
            active_start: FrameCount::new(5),
            active_end: FrameCount::new(10),
        },
        hitbox: HitboxSpec {
            width: 58.0,
            height: 34.0,
            y_offset: 62.0,
        },
        damage: LIGHT_PUNCH_DAMAGE,
    },
    MoveSpec {
        id: MoveId::HeavyPunch,
        input: MoveInputKind::HeavyPunch,
        label: "HP",
        frames: AttackFrameData {
            duration: FrameCount::new(35),
            active_start: FrameCount::new(11),
            active_end: FrameCount::new(20),
        },
        hitbox: HitboxSpec {
            width: 96.0,
            height: 42.0,
            y_offset: 58.0,
        },
        damage: HEAVY_PUNCH_DAMAGE,
    },
    MoveSpec {
        id: MoveId::Kick,
        input: MoveInputKind::Kick,
        label: "KICK",
        frames: AttackFrameData {
            duration: FrameCount::new(28),
            active_start: FrameCount::new(9),
            active_end: FrameCount::new(16),
        },
        hitbox: HitboxSpec {
            width: 100.0,
            height: 36.0,
            y_offset: 108.0,
        },
        damage: KICK_DAMAGE,
    },
    MoveSpec {
        id: MoveId::RustBorrowJab,
        input: MoveInputKind::LightPunch,
        label: "Borrow Jab",
        frames: AttackFrameData {
            duration: FrameCount::new(16),
            active_start: FrameCount::new(4),
            active_end: FrameCount::new(8),
        },
        hitbox: HitboxSpec {
            width: 48.0,
            height: 30.0,
            y_offset: 62.0,
        },
        damage: RUST_BORROW_JAB_DAMAGE,
    },
    MoveSpec {
        id: MoveId::DukeBoilerplatePoke,
        input: MoveInputKind::HeavyPunch,
        label: "Boilerplate",
        frames: AttackFrameData {
            duration: FrameCount::new(40),
            active_start: FrameCount::new(13),
            active_end: FrameCount::new(22),
        },
        hitbox: HitboxSpec {
            width: 112.0,
            height: 44.0,
            y_offset: 60.0,
        },
        damage: DUKE_BOILERPLATE_POKE_DAMAGE,
    },
];

/// Returns close-range move data by stable move id.
pub const fn move_spec(id: MoveId) -> MoveSpec {
    CLOSE_RANGE_MOVE_SPECS[id.index()]
}

/// Returns the first move in a character loadout that matches an input family.
pub fn move_spec_for_input(move_ids: &[MoveId], input: MoveInputKind) -> Option<MoveSpec> {
    move_ids
        .iter()
        .copied()
        .map(move_spec)
        .find(|spec| spec.input == input)
}
