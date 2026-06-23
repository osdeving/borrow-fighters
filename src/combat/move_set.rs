//! Defines close-range move runtime types for the greybox prototype.
//!
//! System: Combat runtime. This module exposes the attack enum used by fighter
//! state while delegating tunable values to `move_data`.
//!
//! `AttackKind` remains the runtime compatibility enum while move details now
//! live in table-driven `MoveSpec` data.

use crate::math::rect::Rect;

pub use super::move_data::{
    AIR_ATTACK_REACTION, AIR_ATTACK_WHIFF_RECOVERY, AIR_KICK_DAMAGE, AIR_PUNCH_DAMAGE,
    AttackFrameData, CLOSE_THROW_DAMAGE, CLOSE_THROW_REACTION, CLOSE_THROW_WHIFF_RECOVERY,
    DEFAULT_CLOSE_RANGE_MOVE_IDS, DUKE_ABSTRACT_FACTORY_OVERHEAD_DAMAGE,
    DUKE_ABSTRACT_FACTORY_OVERHEAD_REACTION, DUKE_ABSTRACT_FACTORY_OVERHEAD_WHIFF_RECOVERY,
    DUKE_BOILERPLATE_POKE_DAMAGE, DUKE_BOILERPLATE_POKE_WHIFF_RECOVERY,
    DUKE_ENTERPRISE_THROW_DAMAGE, DUKE_ENTERPRISE_THROW_REACTION,
    DUKE_ENTERPRISE_THROW_WHIFF_RECOVERY, DUKE_GARBAGE_COLLECTOR_SWEEP_DAMAGE,
    DUKE_GARBAGE_COLLECTOR_SWEEP_REACTION, DUKE_GARBAGE_COLLECTOR_SWEEP_WHIFF_RECOVERY,
    GO_CHANNEL_OVERHEAD_DAMAGE, GO_CHANNEL_OVERHEAD_WHIFF_RECOVERY, GO_DEFER_KICK_DAMAGE,
    GO_DEFER_KICK_WHIFF_RECOVERY, GO_GOROUTINE_JAB_DAMAGE, GO_GOROUTINE_JAB_WHIFF_RECOVERY,
    GO_HOPKICK_DAMAGE, GO_HOPKICK_WHIFF_RECOVERY, GO_KICK_REACTION, GO_LIGHT_REACTION,
    GO_OVERHEAD_REACTION, GuardRule, HEAVY_ATTACK_REACTION, HEAVY_ATTACK_WHIFF_RECOVERY,
    HEAVY_PUNCH_DAMAGE, HitReaction, KICK_DAMAGE, KICK_REACTION, KICK_WHIFF_RECOVERY,
    LIGHT_ATTACK_REACTION, LIGHT_ATTACK_WHIFF_RECOVERY, LIGHT_PUNCH_DAMAGE, MoveId, MoveInputKind,
    MoveSpec, OVERHEAD_PUNCH_DAMAGE, OVERHEAD_PUNCH_WHIFF_RECOVERY, OVERHEAD_REACTION,
    RISING_ANTI_AIR_DAMAGE, RISING_ANTI_AIR_REACTION, RISING_ANTI_AIR_WHIFF_RECOVERY,
    RUST_BORROW_JAB_DAMAGE, RUST_BORROW_JAB_WHIFF_RECOVERY, RUST_LIFETIME_ANTI_AIR_DAMAGE,
    RUST_LIFETIME_ANTI_AIR_REACTION, RUST_LIFETIME_ANTI_AIR_WHIFF_RECOVERY,
    RUST_OWNERSHIP_THROW_DAMAGE, RUST_OWNERSHIP_THROW_REACTION,
    RUST_OWNERSHIP_THROW_WHIFF_RECOVERY, SWEEP_KICK_DAMAGE, SWEEP_KICK_WHIFF_RECOVERY,
    SWEEP_REACTION, move_spec, move_spec_for_input,
};

/// Close-range attacks available in the greybox prototype.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AttackKind {
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

/// A currently active offensive shape.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ActiveAttack {
    pub kind: AttackKind,
    pub move_id: MoveId,
    pub hitbox: Rect,
    pub damage: i32,
    pub guard_rule: GuardRule,
    pub hit_reaction: HitReaction,
}

impl AttackKind {
    /// Returns the runtime attack kind represented by a stable move id.
    pub const fn from_move_id(id: MoveId) -> Self {
        match id {
            MoveId::LightPunch
            | MoveId::RustBorrowJab
            | MoveId::GoGoroutineJab
            | MoveId::CPointerJab
            | MoveId::PythonSnakeBite => Self::LightPunch,
            MoveId::HeavyPunch
            | MoveId::DukeBoilerplatePoke
            | MoveId::CUnsafePoke
            | MoveId::PythonDataStrike => Self::HeavyPunch,
            MoveId::Kick | MoveId::GoDeferKick | MoveId::CNullStepKick | MoveId::PythonHeelKick => {
                Self::Kick
            }
            MoveId::SweepKick
            | MoveId::DukeGarbageCollectorSweep
            | MoveId::CSegfaultSweep
            | MoveId::PythonIndentSweep => Self::Sweep,
            MoveId::OverheadPunch
            | MoveId::DukeAbstractFactoryOverhead
            | MoveId::GoChannelOverhead
            | MoveId::CStackOverflow
            | MoveId::PythonTracebackOverhead => Self::Overhead,
            MoveId::RisingAntiAir
            | MoveId::RustLifetimeAntiAir
            | MoveId::CInterruptVector
            | MoveId::PythonVisionAntiAir => Self::AntiAir,
            MoveId::AirPunch => Self::AirPunch,
            MoveId::AirKick | MoveId::GoHopkick => Self::AirKick,
            MoveId::CloseThrow
            | MoveId::RustOwnershipThrow
            | MoveId::DukeEnterpriseThrow
            | MoveId::CUndefinedThrow
            | MoveId::PythonConstrictThrow => Self::Throw,
        }
    }

    /// Returns the runtime attack kind represented by an input family.
    pub const fn from_input_kind(input: MoveInputKind) -> Self {
        match input {
            MoveInputKind::LightPunch => Self::LightPunch,
            MoveInputKind::HeavyPunch => Self::HeavyPunch,
            MoveInputKind::Kick => Self::Kick,
            MoveInputKind::Sweep => Self::Sweep,
            MoveInputKind::Overhead => Self::Overhead,
            MoveInputKind::AntiAir => Self::AntiAir,
            MoveInputKind::AirPunch => Self::AirPunch,
            MoveInputKind::AirKick => Self::AirKick,
            MoveInputKind::Throw => Self::Throw,
        }
    }

    /// Returns the stable move id represented by this runtime attack kind.
    pub const fn move_id(self) -> MoveId {
        match self {
            Self::LightPunch => MoveId::LightPunch,
            Self::HeavyPunch => MoveId::HeavyPunch,
            Self::Kick => MoveId::Kick,
            Self::Sweep => MoveId::SweepKick,
            Self::Overhead => MoveId::OverheadPunch,
            Self::AntiAir => MoveId::RisingAntiAir,
            Self::AirPunch => MoveId::AirPunch,
            Self::AirKick => MoveId::AirKick,
            Self::Throw => MoveId::CloseThrow,
        }
    }

    /// Returns the short label used by debug rendering.
    pub const fn label(self) -> &'static str {
        self.move_spec().label
    }

    /// Returns the damage applied by this move before guard reduction.
    pub const fn damage(self) -> i32 {
        self.move_spec().damage
    }

    /// Returns whole-frame startup, active, and duration data.
    pub const fn frame_data(self) -> AttackFrameData {
        self.move_spec().frames
    }

    /// Returns the full table-driven move spec.
    pub const fn move_spec(self) -> MoveSpec {
        move_spec(self.move_id())
    }
}
