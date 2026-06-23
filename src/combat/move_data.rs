//! Defines table-driven close-range move data.
//!
//! System: Combat data. This module stores tunable move specs consumed by
//! fighters, Combat Lab, and future character specs.
//!
//! Character specs can now select input-compatible move ids with different
//! timings, reach, and damage while keeping the same prototype controls.

use crate::config::world_px;

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
pub const RUST_LIFETIME_ANTI_AIR_DAMAGE: i32 = 12;
pub const RUST_OWNERSHIP_THROW_DAMAGE: i32 = 9;
pub const DUKE_BOILERPLATE_POKE_DAMAGE: i32 = 18;
pub const DUKE_GARBAGE_COLLECTOR_SWEEP_DAMAGE: i32 = 13;
pub const DUKE_ABSTRACT_FACTORY_OVERHEAD_DAMAGE: i32 = 16;
pub const DUKE_ENTERPRISE_THROW_DAMAGE: i32 = 12;
pub const GO_GOROUTINE_JAB_DAMAGE: i32 = 6;
pub const GO_DEFER_KICK_DAMAGE: i32 = 10;
pub const GO_CHANNEL_OVERHEAD_DAMAGE: i32 = 12;
pub const GO_HOPKICK_DAMAGE: i32 = 10;
pub const C_POINTER_JAB_DAMAGE: i32 = 8;
pub const C_UNSAFE_POKE_DAMAGE: i32 = 17;
pub const C_NULL_STEP_KICK_DAMAGE: i32 = 12;
pub const C_SEGFAULT_SWEEP_DAMAGE: i32 = 14;
pub const C_STACK_OVERFLOW_DAMAGE: i32 = 15;
pub const C_INTERRUPT_VECTOR_DAMAGE: i32 = 13;
pub const C_UNDEFINED_THROW_DAMAGE: i32 = 11;
pub const PYTHON_SNAKE_BITE_DAMAGE: i32 = 7;
pub const PYTHON_DATA_STRIKE_DAMAGE: i32 = 15;
pub const PYTHON_HEEL_KICK_DAMAGE: i32 = 11;
pub const PYTHON_INDENT_SWEEP_DAMAGE: i32 = 10;
pub const PYTHON_TRACEBACK_OVERHEAD_DAMAGE: i32 = 13;
pub const PYTHON_VISION_ANTI_AIR_DAMAGE: i32 = 11;
pub const PYTHON_CONSTRICT_THROW_DAMAGE: i32 = 10;
pub const LIGHT_ATTACK_WHIFF_RECOVERY: FrameCount = FrameCount::new(4);
pub const HEAVY_ATTACK_WHIFF_RECOVERY: FrameCount = FrameCount::new(10);
pub const KICK_WHIFF_RECOVERY: FrameCount = FrameCount::new(8);
pub const SWEEP_KICK_WHIFF_RECOVERY: FrameCount = FrameCount::new(12);
pub const OVERHEAD_PUNCH_WHIFF_RECOVERY: FrameCount = FrameCount::new(12);
pub const RISING_ANTI_AIR_WHIFF_RECOVERY: FrameCount = FrameCount::new(14);
pub const AIR_ATTACK_WHIFF_RECOVERY: FrameCount = FrameCount::new(6);
pub const CLOSE_THROW_WHIFF_RECOVERY: FrameCount = FrameCount::new(16);
pub const RUST_BORROW_JAB_WHIFF_RECOVERY: FrameCount = FrameCount::new(4);
pub const RUST_LIFETIME_ANTI_AIR_WHIFF_RECOVERY: FrameCount = FrameCount::new(10);
pub const RUST_OWNERSHIP_THROW_WHIFF_RECOVERY: FrameCount = FrameCount::new(12);
pub const DUKE_BOILERPLATE_POKE_WHIFF_RECOVERY: FrameCount = FrameCount::new(12);
pub const DUKE_GARBAGE_COLLECTOR_SWEEP_WHIFF_RECOVERY: FrameCount = FrameCount::new(16);
pub const DUKE_ABSTRACT_FACTORY_OVERHEAD_WHIFF_RECOVERY: FrameCount = FrameCount::new(16);
pub const DUKE_ENTERPRISE_THROW_WHIFF_RECOVERY: FrameCount = FrameCount::new(20);
pub const GO_GOROUTINE_JAB_WHIFF_RECOVERY: FrameCount = FrameCount::new(3);
pub const GO_DEFER_KICK_WHIFF_RECOVERY: FrameCount = FrameCount::new(5);
pub const GO_CHANNEL_OVERHEAD_WHIFF_RECOVERY: FrameCount = FrameCount::new(8);
pub const GO_HOPKICK_WHIFF_RECOVERY: FrameCount = FrameCount::new(4);
pub const C_POINTER_JAB_WHIFF_RECOVERY: FrameCount = FrameCount::new(5);
pub const C_UNSAFE_POKE_WHIFF_RECOVERY: FrameCount = FrameCount::new(13);
pub const C_NULL_STEP_KICK_WHIFF_RECOVERY: FrameCount = FrameCount::new(9);
pub const C_SEGFAULT_SWEEP_WHIFF_RECOVERY: FrameCount = FrameCount::new(15);
pub const C_STACK_OVERFLOW_WHIFF_RECOVERY: FrameCount = FrameCount::new(14);
pub const C_INTERRUPT_VECTOR_WHIFF_RECOVERY: FrameCount = FrameCount::new(12);
pub const C_UNDEFINED_THROW_WHIFF_RECOVERY: FrameCount = FrameCount::new(18);
pub const PYTHON_SNAKE_BITE_WHIFF_RECOVERY: FrameCount = FrameCount::new(5);
pub const PYTHON_DATA_STRIKE_WHIFF_RECOVERY: FrameCount = FrameCount::new(9);
pub const PYTHON_HEEL_KICK_WHIFF_RECOVERY: FrameCount = FrameCount::new(7);
pub const PYTHON_INDENT_SWEEP_WHIFF_RECOVERY: FrameCount = FrameCount::new(10);
pub const PYTHON_TRACEBACK_OVERHEAD_WHIFF_RECOVERY: FrameCount = FrameCount::new(11);
pub const PYTHON_VISION_ANTI_AIR_WHIFF_RECOVERY: FrameCount = FrameCount::new(9);
pub const PYTHON_CONSTRICT_THROW_WHIFF_RECOVERY: FrameCount = FrameCount::new(14);
pub const LIGHT_ATTACK_REACTION: HitReaction = HitReaction {
    hitstun: FrameCount::new(12),
    blockstun: FrameCount::new(8),
    hit_pushback: world_px(22.0),
    block_pushback: world_px(14.0),
};
pub const HEAVY_ATTACK_REACTION: HitReaction = HitReaction {
    hitstun: FrameCount::new(18),
    blockstun: FrameCount::new(12),
    hit_pushback: world_px(38.0),
    block_pushback: world_px(26.0),
};
pub const KICK_REACTION: HitReaction = HitReaction {
    hitstun: FrameCount::new(16),
    blockstun: FrameCount::new(10),
    hit_pushback: world_px(34.0),
    block_pushback: world_px(22.0),
};
pub const SWEEP_REACTION: HitReaction = HitReaction {
    hitstun: FrameCount::new(20),
    blockstun: FrameCount::new(12),
    hit_pushback: world_px(42.0),
    block_pushback: world_px(24.0),
};
pub const OVERHEAD_REACTION: HitReaction = HitReaction {
    hitstun: FrameCount::new(18),
    blockstun: FrameCount::new(12),
    hit_pushback: world_px(32.0),
    block_pushback: world_px(20.0),
};
pub const RISING_ANTI_AIR_REACTION: HitReaction = HitReaction {
    hitstun: FrameCount::new(18),
    blockstun: FrameCount::new(10),
    hit_pushback: world_px(28.0),
    block_pushback: world_px(18.0),
};
pub const AIR_ATTACK_REACTION: HitReaction = HitReaction {
    hitstun: FrameCount::new(14),
    blockstun: FrameCount::new(10),
    hit_pushback: world_px(28.0),
    block_pushback: world_px(18.0),
};
pub const CLOSE_THROW_REACTION: HitReaction = HitReaction {
    hitstun: FrameCount::new(22),
    blockstun: FrameCount::ZERO,
    hit_pushback: world_px(54.0),
    block_pushback: world_px(0.0),
};
pub const RUST_LIFETIME_ANTI_AIR_REACTION: HitReaction = HitReaction {
    hitstun: FrameCount::new(17),
    blockstun: FrameCount::new(9),
    hit_pushback: world_px(30.0),
    block_pushback: world_px(18.0),
};
pub const RUST_OWNERSHIP_THROW_REACTION: HitReaction = HitReaction {
    hitstun: FrameCount::new(18),
    blockstun: FrameCount::ZERO,
    hit_pushback: world_px(42.0),
    block_pushback: world_px(0.0),
};
pub const DUKE_GARBAGE_COLLECTOR_SWEEP_REACTION: HitReaction = HitReaction {
    hitstun: FrameCount::new(22),
    blockstun: FrameCount::new(13),
    hit_pushback: world_px(48.0),
    block_pushback: world_px(28.0),
};
pub const DUKE_ABSTRACT_FACTORY_OVERHEAD_REACTION: HitReaction = HitReaction {
    hitstun: FrameCount::new(20),
    blockstun: FrameCount::new(13),
    hit_pushback: world_px(36.0),
    block_pushback: world_px(22.0),
};
pub const DUKE_ENTERPRISE_THROW_REACTION: HitReaction = HitReaction {
    hitstun: FrameCount::new(24),
    blockstun: FrameCount::ZERO,
    hit_pushback: world_px(60.0),
    block_pushback: world_px(0.0),
};
pub const GO_LIGHT_REACTION: HitReaction = HitReaction {
    hitstun: FrameCount::new(10),
    blockstun: FrameCount::new(7),
    hit_pushback: world_px(18.0),
    block_pushback: world_px(12.0),
};
pub const GO_KICK_REACTION: HitReaction = HitReaction {
    hitstun: FrameCount::new(13),
    blockstun: FrameCount::new(9),
    hit_pushback: world_px(24.0),
    block_pushback: world_px(16.0),
};
pub const GO_OVERHEAD_REACTION: HitReaction = HitReaction {
    hitstun: FrameCount::new(15),
    blockstun: FrameCount::new(10),
    hit_pushback: world_px(24.0),
    block_pushback: world_px(16.0),
};
pub const C_LIGHT_REACTION: HitReaction = HitReaction {
    hitstun: FrameCount::new(12),
    blockstun: FrameCount::new(8),
    hit_pushback: world_px(24.0),
    block_pushback: world_px(14.0),
};
pub const C_HEAVY_REACTION: HitReaction = HitReaction {
    hitstun: FrameCount::new(18),
    blockstun: FrameCount::new(12),
    hit_pushback: world_px(42.0),
    block_pushback: world_px(28.0),
};
pub const C_KICK_REACTION: HitReaction = HitReaction {
    hitstun: FrameCount::new(15),
    blockstun: FrameCount::new(10),
    hit_pushback: world_px(32.0),
    block_pushback: world_px(22.0),
};
pub const C_SWEEP_REACTION: HitReaction = HitReaction {
    hitstun: FrameCount::new(21),
    blockstun: FrameCount::new(13),
    hit_pushback: world_px(46.0),
    block_pushback: world_px(26.0),
};
pub const C_OVERHEAD_REACTION: HitReaction = HitReaction {
    hitstun: FrameCount::new(18),
    blockstun: FrameCount::new(12),
    hit_pushback: world_px(34.0),
    block_pushback: world_px(20.0),
};
pub const C_ANTI_AIR_REACTION: HitReaction = HitReaction {
    hitstun: FrameCount::new(18),
    blockstun: FrameCount::new(10),
    hit_pushback: world_px(30.0),
    block_pushback: world_px(18.0),
};
pub const C_THROW_REACTION: HitReaction = HitReaction {
    hitstun: FrameCount::new(23),
    blockstun: FrameCount::ZERO,
    hit_pushback: world_px(58.0),
    block_pushback: world_px(0.0),
};
pub const PYTHON_LIGHT_REACTION: HitReaction = HitReaction {
    hitstun: FrameCount::new(11),
    blockstun: FrameCount::new(8),
    hit_pushback: world_px(20.0),
    block_pushback: world_px(13.0),
};
pub const PYTHON_HEAVY_REACTION: HitReaction = HitReaction {
    hitstun: FrameCount::new(16),
    blockstun: FrameCount::new(11),
    hit_pushback: world_px(30.0),
    block_pushback: world_px(20.0),
};
pub const PYTHON_KICK_REACTION: HitReaction = HitReaction {
    hitstun: FrameCount::new(14),
    blockstun: FrameCount::new(9),
    hit_pushback: world_px(28.0),
    block_pushback: world_px(18.0),
};
pub const PYTHON_SWEEP_REACTION: HitReaction = HitReaction {
    hitstun: FrameCount::new(16),
    blockstun: FrameCount::new(10),
    hit_pushback: world_px(30.0),
    block_pushback: world_px(18.0),
};
pub const PYTHON_OVERHEAD_REACTION: HitReaction = HitReaction {
    hitstun: FrameCount::new(16),
    blockstun: FrameCount::new(11),
    hit_pushback: world_px(28.0),
    block_pushback: world_px(18.0),
};
pub const PYTHON_ANTI_AIR_REACTION: HitReaction = HitReaction {
    hitstun: FrameCount::new(16),
    blockstun: FrameCount::new(9),
    hit_pushback: world_px(28.0),
    block_pushback: world_px(18.0),
};
pub const PYTHON_THROW_REACTION: HitReaction = HitReaction {
    hitstun: FrameCount::new(20),
    blockstun: FrameCount::ZERO,
    hit_pushback: world_px(48.0),
    block_pushback: world_px(0.0),
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
    RustLifetimeAntiAir,
    RustOwnershipThrow,
    DukeBoilerplatePoke,
    DukeGarbageCollectorSweep,
    DukeAbstractFactoryOverhead,
    DukeEnterpriseThrow,
    GoGoroutineJab,
    GoDeferKick,
    GoChannelOverhead,
    GoHopkick,
    CPointerJab,
    CUnsafePoke,
    CNullStepKick,
    CSegfaultSweep,
    CStackOverflow,
    CInterruptVector,
    CUndefinedThrow,
    PythonSnakeBite,
    PythonDataStrike,
    PythonHeelKick,
    PythonIndentSweep,
    PythonTracebackOverhead,
    PythonVisionAntiAir,
    PythonConstrictThrow,
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
            Self::RustLifetimeAntiAir => 10,
            Self::RustOwnershipThrow => 11,
            Self::DukeBoilerplatePoke => 12,
            Self::DukeGarbageCollectorSweep => 13,
            Self::DukeAbstractFactoryOverhead => 14,
            Self::DukeEnterpriseThrow => 15,
            Self::GoGoroutineJab => 16,
            Self::GoDeferKick => 17,
            Self::GoChannelOverhead => 18,
            Self::GoHopkick => 19,
            Self::CPointerJab => 20,
            Self::CUnsafePoke => 21,
            Self::CNullStepKick => 22,
            Self::CSegfaultSweep => 23,
            Self::CStackOverflow => 24,
            Self::CInterruptVector => 25,
            Self::CUndefinedThrow => 26,
            Self::PythonSnakeBite => 27,
            Self::PythonDataStrike => 28,
            Self::PythonHeelKick => 29,
            Self::PythonIndentSweep => 30,
            Self::PythonTracebackOverhead => 31,
            Self::PythonVisionAntiAir => 32,
            Self::PythonConstrictThrow => 33,
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
            Self::RustLifetimeAntiAir => "rust_lifetime_anti_air",
            Self::RustOwnershipThrow => "rust_ownership_throw",
            Self::DukeBoilerplatePoke => "duke_boilerplate_poke",
            Self::DukeGarbageCollectorSweep => "duke_garbage_collector_sweep",
            Self::DukeAbstractFactoryOverhead => "duke_abstract_factory_overhead",
            Self::DukeEnterpriseThrow => "duke_enterprise_throw",
            Self::GoGoroutineJab => "go_goroutine_jab",
            Self::GoDeferKick => "go_defer_kick",
            Self::GoChannelOverhead => "go_channel_overhead",
            Self::GoHopkick => "go_hopkick",
            Self::CPointerJab => "c_pointer_jab",
            Self::CUnsafePoke => "c_unsafe_poke",
            Self::CNullStepKick => "c_null_step_kick",
            Self::CSegfaultSweep => "c_segfault_sweep",
            Self::CStackOverflow => "c_stack_overflow",
            Self::CInterruptVector => "c_interrupt_vector",
            Self::CUndefinedThrow => "c_undefined_throw",
            Self::PythonSnakeBite => "python_snake_bite",
            Self::PythonDataStrike => "python_data_strike",
            Self::PythonHeelKick => "python_heel_kick",
            Self::PythonIndentSweep => "python_indent_sweep",
            Self::PythonTracebackOverhead => "python_traceback_overhead",
            Self::PythonVisionAntiAir => "python_vision_anti_air",
            Self::PythonConstrictThrow => "python_constrict_throw",
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
            "rust_lifetime_anti_air" => Some(Self::RustLifetimeAntiAir),
            "rust_ownership_throw" => Some(Self::RustOwnershipThrow),
            "duke_boilerplate_poke" => Some(Self::DukeBoilerplatePoke),
            "duke_garbage_collector_sweep" => Some(Self::DukeGarbageCollectorSweep),
            "duke_abstract_factory_overhead" => Some(Self::DukeAbstractFactoryOverhead),
            "duke_enterprise_throw" => Some(Self::DukeEnterpriseThrow),
            "go_goroutine_jab" => Some(Self::GoGoroutineJab),
            "go_defer_kick" => Some(Self::GoDeferKick),
            "go_channel_overhead" => Some(Self::GoChannelOverhead),
            "go_hopkick" => Some(Self::GoHopkick),
            "c_pointer_jab" => Some(Self::CPointerJab),
            "c_unsafe_poke" => Some(Self::CUnsafePoke),
            "c_null_step_kick" => Some(Self::CNullStepKick),
            "c_segfault_sweep" => Some(Self::CSegfaultSweep),
            "c_stack_overflow" => Some(Self::CStackOverflow),
            "c_interrupt_vector" => Some(Self::CInterruptVector),
            "c_undefined_throw" => Some(Self::CUndefinedThrow),
            "python_snake_bite" => Some(Self::PythonSnakeBite),
            "python_data_strike" => Some(Self::PythonDataStrike),
            "python_heel_kick" => Some(Self::PythonHeelKick),
            "python_indent_sweep" => Some(Self::PythonIndentSweep),
            "python_traceback_overhead" => Some(Self::PythonTracebackOverhead),
            "python_vision_anti_air" => Some(Self::PythonVisionAntiAir),
            "python_constrict_throw" => Some(Self::PythonConstrictThrow),
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
pub const CLOSE_RANGE_MOVE_SPECS: [MoveSpec; 34] = [
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
            width: world_px(58.0),
            height: world_px(34.0),
            y_offset: world_px(62.0),
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
            width: world_px(96.0),
            height: world_px(42.0),
            y_offset: world_px(58.0),
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
            width: world_px(100.0),
            height: world_px(36.0),
            y_offset: world_px(108.0),
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
            width: world_px(112.0),
            height: world_px(30.0),
            y_offset: world_px(66.0),
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
            width: world_px(82.0),
            height: world_px(50.0),
            y_offset: world_px(42.0),
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
            width: world_px(70.0),
            height: world_px(82.0),
            y_offset: world_px(-86.0),
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
            width: world_px(72.0),
            height: world_px(42.0),
            y_offset: world_px(70.0),
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
            width: world_px(88.0),
            height: world_px(46.0),
            y_offset: world_px(88.0),
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
            width: world_px(46.0),
            height: world_px(120.0),
            y_offset: world_px(30.0),
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
            width: world_px(48.0),
            height: world_px(30.0),
            y_offset: world_px(62.0),
        },
        damage: RUST_BORROW_JAB_DAMAGE,
        guard_rule: GuardRule::Mid,
        hit_reaction: LIGHT_ATTACK_REACTION,
        whiff_recovery: RUST_BORROW_JAB_WHIFF_RECOVERY,
    },
    MoveSpec {
        id: MoveId::RustLifetimeAntiAir,
        input: MoveInputKind::AntiAir,
        label: "Lifetime AA",
        frames: AttackFrameData {
            duration: FrameCount::new(26),
            active_start: FrameCount::new(6),
            active_end: FrameCount::new(12),
        },
        hitbox: HitboxSpec {
            width: world_px(62.0),
            height: world_px(90.0),
            y_offset: world_px(-92.0),
        },
        damage: RUST_LIFETIME_ANTI_AIR_DAMAGE,
        guard_rule: GuardRule::Mid,
        hit_reaction: RUST_LIFETIME_ANTI_AIR_REACTION,
        whiff_recovery: RUST_LIFETIME_ANTI_AIR_WHIFF_RECOVERY,
    },
    MoveSpec {
        id: MoveId::RustOwnershipThrow,
        input: MoveInputKind::Throw,
        label: "Ownership",
        frames: AttackFrameData {
            duration: FrameCount::new(20),
            active_start: FrameCount::new(5),
            active_end: FrameCount::new(7),
        },
        hitbox: HitboxSpec {
            width: world_px(42.0),
            height: world_px(118.0),
            y_offset: world_px(30.0),
        },
        damage: RUST_OWNERSHIP_THROW_DAMAGE,
        guard_rule: GuardRule::Throw,
        hit_reaction: RUST_OWNERSHIP_THROW_REACTION,
        whiff_recovery: RUST_OWNERSHIP_THROW_WHIFF_RECOVERY,
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
            width: world_px(112.0),
            height: world_px(44.0),
            y_offset: world_px(60.0),
        },
        damage: DUKE_BOILERPLATE_POKE_DAMAGE,
        guard_rule: GuardRule::Mid,
        hit_reaction: HEAVY_ATTACK_REACTION,
        whiff_recovery: DUKE_BOILERPLATE_POKE_WHIFF_RECOVERY,
    },
    MoveSpec {
        id: MoveId::DukeGarbageCollectorSweep,
        input: MoveInputKind::Sweep,
        label: "GC Sweep",
        frames: AttackFrameData {
            duration: FrameCount::new(38),
            active_start: FrameCount::new(13),
            active_end: FrameCount::new(22),
        },
        hitbox: HitboxSpec {
            width: world_px(128.0),
            height: world_px(32.0),
            y_offset: world_px(66.0),
        },
        damage: DUKE_GARBAGE_COLLECTOR_SWEEP_DAMAGE,
        guard_rule: GuardRule::Low,
        hit_reaction: DUKE_GARBAGE_COLLECTOR_SWEEP_REACTION,
        whiff_recovery: DUKE_GARBAGE_COLLECTOR_SWEEP_WHIFF_RECOVERY,
    },
    MoveSpec {
        id: MoveId::DukeAbstractFactoryOverhead,
        input: MoveInputKind::Overhead,
        label: "Factory OH",
        frames: AttackFrameData {
            duration: FrameCount::new(40),
            active_start: FrameCount::new(15),
            active_end: FrameCount::new(22),
        },
        hitbox: HitboxSpec {
            width: world_px(96.0),
            height: world_px(54.0),
            y_offset: world_px(40.0),
        },
        damage: DUKE_ABSTRACT_FACTORY_OVERHEAD_DAMAGE,
        guard_rule: GuardRule::High,
        hit_reaction: DUKE_ABSTRACT_FACTORY_OVERHEAD_REACTION,
        whiff_recovery: DUKE_ABSTRACT_FACTORY_OVERHEAD_WHIFF_RECOVERY,
    },
    MoveSpec {
        id: MoveId::DukeEnterpriseThrow,
        input: MoveInputKind::Throw,
        label: "Enterprise Grab",
        frames: AttackFrameData {
            duration: FrameCount::new(30),
            active_start: FrameCount::new(9),
            active_end: FrameCount::new(11),
        },
        hitbox: HitboxSpec {
            width: world_px(56.0),
            height: world_px(124.0),
            y_offset: world_px(28.0),
        },
        damage: DUKE_ENTERPRISE_THROW_DAMAGE,
        guard_rule: GuardRule::Throw,
        hit_reaction: DUKE_ENTERPRISE_THROW_REACTION,
        whiff_recovery: DUKE_ENTERPRISE_THROW_WHIFF_RECOVERY,
    },
    MoveSpec {
        id: MoveId::GoGoroutineJab,
        input: MoveInputKind::LightPunch,
        label: "Goroutine Jab",
        frames: AttackFrameData {
            duration: FrameCount::new(14),
            active_start: FrameCount::new(3),
            active_end: FrameCount::new(7),
        },
        hitbox: HitboxSpec {
            width: world_px(42.0),
            height: world_px(30.0),
            y_offset: world_px(62.0),
        },
        damage: GO_GOROUTINE_JAB_DAMAGE,
        guard_rule: GuardRule::Mid,
        hit_reaction: GO_LIGHT_REACTION,
        whiff_recovery: GO_GOROUTINE_JAB_WHIFF_RECOVERY,
    },
    MoveSpec {
        id: MoveId::GoDeferKick,
        input: MoveInputKind::Kick,
        label: "Defer Kick",
        frames: AttackFrameData {
            duration: FrameCount::new(23),
            active_start: FrameCount::new(7),
            active_end: FrameCount::new(13),
        },
        hitbox: HitboxSpec {
            width: world_px(86.0),
            height: world_px(34.0),
            y_offset: world_px(106.0),
        },
        damage: GO_DEFER_KICK_DAMAGE,
        guard_rule: GuardRule::Mid,
        hit_reaction: GO_KICK_REACTION,
        whiff_recovery: GO_DEFER_KICK_WHIFF_RECOVERY,
    },
    MoveSpec {
        id: MoveId::GoChannelOverhead,
        input: MoveInputKind::Overhead,
        label: "Channel OH",
        frames: AttackFrameData {
            duration: FrameCount::new(28),
            active_start: FrameCount::new(10),
            active_end: FrameCount::new(15),
        },
        hitbox: HitboxSpec {
            width: world_px(70.0),
            height: world_px(48.0),
            y_offset: world_px(42.0),
        },
        damage: GO_CHANNEL_OVERHEAD_DAMAGE,
        guard_rule: GuardRule::High,
        hit_reaction: GO_OVERHEAD_REACTION,
        whiff_recovery: GO_CHANNEL_OVERHEAD_WHIFF_RECOVERY,
    },
    MoveSpec {
        id: MoveId::GoHopkick,
        input: MoveInputKind::AirKick,
        label: "Hopkick",
        frames: AttackFrameData {
            duration: FrameCount::new(20),
            active_start: FrameCount::new(5),
            active_end: FrameCount::new(12),
        },
        hitbox: HitboxSpec {
            width: world_px(78.0),
            height: world_px(42.0),
            y_offset: world_px(88.0),
        },
        damage: GO_HOPKICK_DAMAGE,
        guard_rule: GuardRule::High,
        hit_reaction: GO_KICK_REACTION,
        whiff_recovery: GO_HOPKICK_WHIFF_RECOVERY,
    },
    MoveSpec {
        id: MoveId::CPointerJab,
        input: MoveInputKind::LightPunch,
        label: "Pointer Jab",
        frames: AttackFrameData {
            duration: FrameCount::new(18),
            active_start: FrameCount::new(5),
            active_end: FrameCount::new(10),
        },
        hitbox: HitboxSpec {
            width: world_px(64.0),
            height: world_px(32.0),
            y_offset: world_px(62.0),
        },
        damage: C_POINTER_JAB_DAMAGE,
        guard_rule: GuardRule::Mid,
        hit_reaction: C_LIGHT_REACTION,
        whiff_recovery: C_POINTER_JAB_WHIFF_RECOVERY,
    },
    MoveSpec {
        id: MoveId::CUnsafePoke,
        input: MoveInputKind::HeavyPunch,
        label: "Unsafe Poke",
        frames: AttackFrameData {
            duration: FrameCount::new(38),
            active_start: FrameCount::new(12),
            active_end: FrameCount::new(20),
        },
        hitbox: HitboxSpec {
            width: world_px(108.0),
            height: world_px(40.0),
            y_offset: world_px(58.0),
        },
        damage: C_UNSAFE_POKE_DAMAGE,
        guard_rule: GuardRule::Mid,
        hit_reaction: C_HEAVY_REACTION,
        whiff_recovery: C_UNSAFE_POKE_WHIFF_RECOVERY,
    },
    MoveSpec {
        id: MoveId::CNullStepKick,
        input: MoveInputKind::Kick,
        label: "Null Step",
        frames: AttackFrameData {
            duration: FrameCount::new(30),
            active_start: FrameCount::new(9),
            active_end: FrameCount::new(16),
        },
        hitbox: HitboxSpec {
            width: world_px(102.0),
            height: world_px(34.0),
            y_offset: world_px(110.0),
        },
        damage: C_NULL_STEP_KICK_DAMAGE,
        guard_rule: GuardRule::Mid,
        hit_reaction: C_KICK_REACTION,
        whiff_recovery: C_NULL_STEP_KICK_WHIFF_RECOVERY,
    },
    MoveSpec {
        id: MoveId::CSegfaultSweep,
        input: MoveInputKind::Sweep,
        label: "Segfault",
        frames: AttackFrameData {
            duration: FrameCount::new(36),
            active_start: FrameCount::new(12),
            active_end: FrameCount::new(20),
        },
        hitbox: HitboxSpec {
            width: world_px(122.0),
            height: world_px(30.0),
            y_offset: world_px(66.0),
        },
        damage: C_SEGFAULT_SWEEP_DAMAGE,
        guard_rule: GuardRule::Low,
        hit_reaction: C_SWEEP_REACTION,
        whiff_recovery: C_SEGFAULT_SWEEP_WHIFF_RECOVERY,
    },
    MoveSpec {
        id: MoveId::CStackOverflow,
        input: MoveInputKind::Overhead,
        label: "Stack OH",
        frames: AttackFrameData {
            duration: FrameCount::new(36),
            active_start: FrameCount::new(13),
            active_end: FrameCount::new(20),
        },
        hitbox: HitboxSpec {
            width: world_px(86.0),
            height: world_px(52.0),
            y_offset: world_px(40.0),
        },
        damage: C_STACK_OVERFLOW_DAMAGE,
        guard_rule: GuardRule::High,
        hit_reaction: C_OVERHEAD_REACTION,
        whiff_recovery: C_STACK_OVERFLOW_WHIFF_RECOVERY,
    },
    MoveSpec {
        id: MoveId::CInterruptVector,
        input: MoveInputKind::AntiAir,
        label: "Interrupt",
        frames: AttackFrameData {
            duration: FrameCount::new(30),
            active_start: FrameCount::new(7),
            active_end: FrameCount::new(14),
        },
        hitbox: HitboxSpec {
            width: world_px(74.0),
            height: world_px(86.0),
            y_offset: world_px(-88.0),
        },
        damage: C_INTERRUPT_VECTOR_DAMAGE,
        guard_rule: GuardRule::Mid,
        hit_reaction: C_ANTI_AIR_REACTION,
        whiff_recovery: C_INTERRUPT_VECTOR_WHIFF_RECOVERY,
    },
    MoveSpec {
        id: MoveId::CUndefinedThrow,
        input: MoveInputKind::Throw,
        label: "Undefined",
        frames: AttackFrameData {
            duration: FrameCount::new(26),
            active_start: FrameCount::new(7),
            active_end: FrameCount::new(9),
        },
        hitbox: HitboxSpec {
            width: world_px(50.0),
            height: world_px(122.0),
            y_offset: world_px(30.0),
        },
        damage: C_UNDEFINED_THROW_DAMAGE,
        guard_rule: GuardRule::Throw,
        hit_reaction: C_THROW_REACTION,
        whiff_recovery: C_UNDEFINED_THROW_WHIFF_RECOVERY,
    },
    MoveSpec {
        id: MoveId::PythonSnakeBite,
        input: MoveInputKind::LightPunch,
        label: "Snake Bite",
        frames: AttackFrameData {
            duration: FrameCount::new(17),
            active_start: FrameCount::new(4),
            active_end: FrameCount::new(9),
        },
        hitbox: HitboxSpec {
            width: world_px(66.0),
            height: world_px(28.0),
            y_offset: world_px(58.0),
        },
        damage: PYTHON_SNAKE_BITE_DAMAGE,
        guard_rule: GuardRule::Mid,
        hit_reaction: PYTHON_LIGHT_REACTION,
        whiff_recovery: PYTHON_SNAKE_BITE_WHIFF_RECOVERY,
    },
    MoveSpec {
        id: MoveId::PythonDataStrike,
        input: MoveInputKind::HeavyPunch,
        label: "Data Strike",
        frames: AttackFrameData {
            duration: FrameCount::new(31),
            active_start: FrameCount::new(10),
            active_end: FrameCount::new(18),
        },
        hitbox: HitboxSpec {
            width: world_px(92.0),
            height: world_px(40.0),
            y_offset: world_px(56.0),
        },
        damage: PYTHON_DATA_STRIKE_DAMAGE,
        guard_rule: GuardRule::Mid,
        hit_reaction: PYTHON_HEAVY_REACTION,
        whiff_recovery: PYTHON_DATA_STRIKE_WHIFF_RECOVERY,
    },
    MoveSpec {
        id: MoveId::PythonHeelKick,
        input: MoveInputKind::Kick,
        label: "Heel Kick",
        frames: AttackFrameData {
            duration: FrameCount::new(25),
            active_start: FrameCount::new(8),
            active_end: FrameCount::new(14),
        },
        hitbox: HitboxSpec {
            width: world_px(86.0),
            height: world_px(34.0),
            y_offset: world_px(108.0),
        },
        damage: PYTHON_HEEL_KICK_DAMAGE,
        guard_rule: GuardRule::Mid,
        hit_reaction: PYTHON_KICK_REACTION,
        whiff_recovery: PYTHON_HEEL_KICK_WHIFF_RECOVERY,
    },
    MoveSpec {
        id: MoveId::PythonIndentSweep,
        input: MoveInputKind::Sweep,
        label: "Indent Low",
        frames: AttackFrameData {
            duration: FrameCount::new(29),
            active_start: FrameCount::new(9),
            active_end: FrameCount::new(16),
        },
        hitbox: HitboxSpec {
            width: world_px(98.0),
            height: world_px(30.0),
            y_offset: world_px(66.0),
        },
        damage: PYTHON_INDENT_SWEEP_DAMAGE,
        guard_rule: GuardRule::Low,
        hit_reaction: PYTHON_SWEEP_REACTION,
        whiff_recovery: PYTHON_INDENT_SWEEP_WHIFF_RECOVERY,
    },
    MoveSpec {
        id: MoveId::PythonTracebackOverhead,
        input: MoveInputKind::Overhead,
        label: "Traceback",
        frames: AttackFrameData {
            duration: FrameCount::new(31),
            active_start: FrameCount::new(11),
            active_end: FrameCount::new(17),
        },
        hitbox: HitboxSpec {
            width: world_px(78.0),
            height: world_px(48.0),
            y_offset: world_px(42.0),
        },
        damage: PYTHON_TRACEBACK_OVERHEAD_DAMAGE,
        guard_rule: GuardRule::High,
        hit_reaction: PYTHON_OVERHEAD_REACTION,
        whiff_recovery: PYTHON_TRACEBACK_OVERHEAD_WHIFF_RECOVERY,
    },
    MoveSpec {
        id: MoveId::PythonVisionAntiAir,
        input: MoveInputKind::AntiAir,
        label: "Vision AA",
        frames: AttackFrameData {
            duration: FrameCount::new(25),
            active_start: FrameCount::new(6),
            active_end: FrameCount::new(12),
        },
        hitbox: HitboxSpec {
            width: world_px(68.0),
            height: world_px(88.0),
            y_offset: world_px(-88.0),
        },
        damage: PYTHON_VISION_ANTI_AIR_DAMAGE,
        guard_rule: GuardRule::Mid,
        hit_reaction: PYTHON_ANTI_AIR_REACTION,
        whiff_recovery: PYTHON_VISION_ANTI_AIR_WHIFF_RECOVERY,
    },
    MoveSpec {
        id: MoveId::PythonConstrictThrow,
        input: MoveInputKind::Throw,
        label: "Constrict",
        frames: AttackFrameData {
            duration: FrameCount::new(24),
            active_start: FrameCount::new(7),
            active_end: FrameCount::new(9),
        },
        hitbox: HitboxSpec {
            width: world_px(52.0),
            height: world_px(118.0),
            y_offset: world_px(30.0),
        },
        damage: PYTHON_CONSTRICT_THROW_DAMAGE,
        guard_rule: GuardRule::Throw,
        hit_reaction: PYTHON_THROW_REACTION,
        whiff_recovery: PYTHON_CONSTRICT_THROW_WHIFF_RECOVERY,
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
