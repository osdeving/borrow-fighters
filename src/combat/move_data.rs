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
pub const SWEEP_KICK_DAMAGE: i32 = 11;
pub const OVERHEAD_PUNCH_DAMAGE: i32 = 14;
pub const RISING_ANTI_AIR_DAMAGE: i32 = 13;
pub const AIR_PUNCH_DAMAGE: i32 = 9;
pub const AIR_KICK_DAMAGE: i32 = 12;
pub const CLOSE_THROW_DAMAGE: i32 = 10;
pub const RUST_BORROW_JAB_DAMAGE: i32 = 7;
pub const DUKE_BOILERPLATE_POKE_DAMAGE: i32 = 18;
pub const LIGHT_ATTACK_WHIFF_RECOVERY: FrameCount = FrameCount::new(4);
pub const HEAVY_ATTACK_WHIFF_RECOVERY: FrameCount = FrameCount::new(10);
pub const KICK_WHIFF_RECOVERY: FrameCount = FrameCount::new(8);
pub const SWEEP_KICK_WHIFF_RECOVERY: FrameCount = FrameCount::new(12);
pub const OVERHEAD_PUNCH_WHIFF_RECOVERY: FrameCount = FrameCount::new(12);
pub const RISING_ANTI_AIR_WHIFF_RECOVERY: FrameCount = FrameCount::new(14);
pub const AIR_ATTACK_WHIFF_RECOVERY: FrameCount = FrameCount::new(6);
pub const CLOSE_THROW_WHIFF_RECOVERY: FrameCount = FrameCount::new(16);
pub const RUST_BORROW_JAB_WHIFF_RECOVERY: FrameCount = FrameCount::new(4);
pub const DUKE_BOILERPLATE_POKE_WHIFF_RECOVERY: FrameCount = FrameCount::new(12);
pub const LIGHT_ATTACK_REACTION: HitReaction = HitReaction {
    hitstun: FrameCount::new(12),
    blockstun: FrameCount::new(8),
    hit_pushback: 22.0,
    block_pushback: 14.0,
};
pub const HEAVY_ATTACK_REACTION: HitReaction = HitReaction {
    hitstun: FrameCount::new(18),
    blockstun: FrameCount::new(12),
    hit_pushback: 38.0,
    block_pushback: 26.0,
};
pub const KICK_REACTION: HitReaction = HitReaction {
    hitstun: FrameCount::new(16),
    blockstun: FrameCount::new(10),
    hit_pushback: 34.0,
    block_pushback: 22.0,
};
pub const SWEEP_REACTION: HitReaction = HitReaction {
    hitstun: FrameCount::new(20),
    blockstun: FrameCount::new(12),
    hit_pushback: 42.0,
    block_pushback: 24.0,
};
pub const OVERHEAD_REACTION: HitReaction = HitReaction {
    hitstun: FrameCount::new(18),
    blockstun: FrameCount::new(12),
    hit_pushback: 32.0,
    block_pushback: 20.0,
};
pub const RISING_ANTI_AIR_REACTION: HitReaction = HitReaction {
    hitstun: FrameCount::new(18),
    blockstun: FrameCount::new(10),
    hit_pushback: 28.0,
    block_pushback: 18.0,
};
pub const AIR_ATTACK_REACTION: HitReaction = HitReaction {
    hitstun: FrameCount::new(14),
    blockstun: FrameCount::new(10),
    hit_pushback: 28.0,
    block_pushback: 18.0,
};
pub const CLOSE_THROW_REACTION: HitReaction = HitReaction {
    hitstun: FrameCount::new(22),
    blockstun: FrameCount::ZERO,
    hit_pushback: 54.0,
    block_pushback: 0.0,
};

/// Stable identifier for a close-range move.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MoveId {
    LightPunch,
    HeavyPunch,
    Kick,
    SweepKick,
    OverheadPunch,
    RisingAntiAir,
    AirPunch,
    AirKick,
    CloseThrow,
    RustBorrowJab,
    DukeBoilerplatePoke,
}

impl MoveId {
    const fn index(self) -> usize {
        match self {
            Self::LightPunch => 0,
            Self::HeavyPunch => 1,
            Self::Kick => 2,
            Self::SweepKick => 3,
            Self::OverheadPunch => 4,
            Self::RisingAntiAir => 5,
            Self::AirPunch => 6,
            Self::AirKick => 7,
            Self::CloseThrow => 8,
            Self::RustBorrowJab => 9,
            Self::DukeBoilerplatePoke => 10,
        }
    }

    /// Returns the stable manifest key used by audio and data files.
    pub const fn audio_key(self) -> &'static str {
        match self {
            Self::LightPunch => "light_punch",
            Self::HeavyPunch => "heavy_punch",
            Self::Kick => "kick",
            Self::SweepKick => "sweep_kick",
            Self::OverheadPunch => "overhead_punch",
            Self::RisingAntiAir => "rising_anti_air",
            Self::AirPunch => "air_punch",
            Self::AirKick => "air_kick",
            Self::CloseThrow => "close_throw",
            Self::RustBorrowJab => "rust_borrow_jab",
            Self::DukeBoilerplatePoke => "duke_boilerplate_poke",
        }
    }

    /// Parses a stable manifest key.
    pub fn from_audio_key(value: &str) -> Option<Self> {
        match value {
            "light_punch" => Some(Self::LightPunch),
            "heavy_punch" => Some(Self::HeavyPunch),
            "kick" => Some(Self::Kick),
            "sweep_kick" => Some(Self::SweepKick),
            "overhead_punch" => Some(Self::OverheadPunch),
            "rising_anti_air" => Some(Self::RisingAntiAir),
            "air_punch" => Some(Self::AirPunch),
            "air_kick" => Some(Self::AirKick),
            "close_throw" => Some(Self::CloseThrow),
            "rust_borrow_jab" => Some(Self::RustBorrowJab),
            "duke_boilerplate_poke" => Some(Self::DukeBoilerplatePoke),
            _ => None,
        }
    }
}

/// Default close-range move ids used by current prototype characters.
pub const DEFAULT_CLOSE_RANGE_MOVE_IDS: [MoveId; 9] = [
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

/// Input family that starts a move.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MoveInputKind {
    LightPunch,
    HeavyPunch,
    Kick,
    Sweep,
    Overhead,
    AntiAir,
    AirPunch,
    AirKick,
    Throw,
}

/// How an incoming hit can be guarded.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GuardRule {
    High,
    Mid,
    Low,
    Throw,
    Projectile,
}

impl GuardRule {
    /// Returns whether this rule is blocked by the current defensive stance.
    pub const fn is_blocked_by(self, blocking: bool, crouching: bool) -> bool {
        if !blocking {
            return false;
        }

        match self {
            Self::High => !crouching,
            Self::Mid | Self::Projectile => true,
            Self::Low => crouching,
            Self::Throw => false,
        }
    }
}

/// Initial reaction applied on hit or block.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HitReaction {
    pub hitstun: FrameCount,
    pub blockstun: FrameCount,
    pub hit_pushback: f32,
    pub block_pushback: f32,
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
    pub guard_rule: GuardRule,
    pub hit_reaction: HitReaction,
    pub whiff_recovery: FrameCount,
}

/// Prototype 0.1 close-range move table.
pub const CLOSE_RANGE_MOVE_SPECS: [MoveSpec; 11] = [
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
        guard_rule: GuardRule::Mid,
        hit_reaction: LIGHT_ATTACK_REACTION,
        whiff_recovery: LIGHT_ATTACK_WHIFF_RECOVERY,
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
        guard_rule: GuardRule::Mid,
        hit_reaction: HEAVY_ATTACK_REACTION,
        whiff_recovery: HEAVY_ATTACK_WHIFF_RECOVERY,
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
        guard_rule: GuardRule::Mid,
        hit_reaction: KICK_REACTION,
        whiff_recovery: KICK_WHIFF_RECOVERY,
    },
    MoveSpec {
        id: MoveId::SweepKick,
        input: MoveInputKind::Sweep,
        label: "Sweep",
        frames: AttackFrameData {
            duration: FrameCount::new(32),
            active_start: FrameCount::new(10),
            active_end: FrameCount::new(18),
        },
        hitbox: HitboxSpec {
            width: 112.0,
            height: 30.0,
            y_offset: 66.0,
        },
        damage: SWEEP_KICK_DAMAGE,
        guard_rule: GuardRule::Low,
        hit_reaction: SWEEP_REACTION,
        whiff_recovery: SWEEP_KICK_WHIFF_RECOVERY,
    },
    MoveSpec {
        id: MoveId::OverheadPunch,
        input: MoveInputKind::Overhead,
        label: "Overhead",
        frames: AttackFrameData {
            duration: FrameCount::new(34),
            active_start: FrameCount::new(12),
            active_end: FrameCount::new(18),
        },
        hitbox: HitboxSpec {
            width: 82.0,
            height: 50.0,
            y_offset: 42.0,
        },
        damage: OVERHEAD_PUNCH_DAMAGE,
        guard_rule: GuardRule::High,
        hit_reaction: OVERHEAD_REACTION,
        whiff_recovery: OVERHEAD_PUNCH_WHIFF_RECOVERY,
    },
    MoveSpec {
        id: MoveId::RisingAntiAir,
        input: MoveInputKind::AntiAir,
        label: "Anti-Air",
        frames: AttackFrameData {
            duration: FrameCount::new(30),
            active_start: FrameCount::new(7),
            active_end: FrameCount::new(14),
        },
        hitbox: HitboxSpec {
            width: 70.0,
            height: 82.0,
            y_offset: -86.0,
        },
        damage: RISING_ANTI_AIR_DAMAGE,
        guard_rule: GuardRule::Mid,
        hit_reaction: RISING_ANTI_AIR_REACTION,
        whiff_recovery: RISING_ANTI_AIR_WHIFF_RECOVERY,
    },
    MoveSpec {
        id: MoveId::AirPunch,
        input: MoveInputKind::AirPunch,
        label: "Air Punch",
        frames: AttackFrameData {
            duration: FrameCount::new(22),
            active_start: FrameCount::new(5),
            active_end: FrameCount::new(13),
        },
        hitbox: HitboxSpec {
            width: 72.0,
            height: 42.0,
            y_offset: 70.0,
        },
        damage: AIR_PUNCH_DAMAGE,
        guard_rule: GuardRule::High,
        hit_reaction: AIR_ATTACK_REACTION,
        whiff_recovery: AIR_ATTACK_WHIFF_RECOVERY,
    },
    MoveSpec {
        id: MoveId::AirKick,
        input: MoveInputKind::AirKick,
        label: "Air Kick",
        frames: AttackFrameData {
            duration: FrameCount::new(26),
            active_start: FrameCount::new(7),
            active_end: FrameCount::new(16),
        },
        hitbox: HitboxSpec {
            width: 88.0,
            height: 46.0,
            y_offset: 88.0,
        },
        damage: AIR_KICK_DAMAGE,
        guard_rule: GuardRule::High,
        hit_reaction: AIR_ATTACK_REACTION,
        whiff_recovery: AIR_ATTACK_WHIFF_RECOVERY,
    },
    MoveSpec {
        id: MoveId::CloseThrow,
        input: MoveInputKind::Throw,
        label: "Throw",
        frames: AttackFrameData {
            duration: FrameCount::new(22),
            active_start: FrameCount::new(6),
            active_end: FrameCount::new(8),
        },
        hitbox: HitboxSpec {
            width: 46.0,
            height: 120.0,
            y_offset: 30.0,
        },
        damage: CLOSE_THROW_DAMAGE,
        guard_rule: GuardRule::Throw,
        hit_reaction: CLOSE_THROW_REACTION,
        whiff_recovery: CLOSE_THROW_WHIFF_RECOVERY,
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
        guard_rule: GuardRule::Mid,
        hit_reaction: LIGHT_ATTACK_REACTION,
        whiff_recovery: RUST_BORROW_JAB_WHIFF_RECOVERY,
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
        guard_rule: GuardRule::Mid,
        hit_reaction: HEAVY_ATTACK_REACTION,
        whiff_recovery: DUKE_BOILERPLATE_POKE_WHIFF_RECOVERY,
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
