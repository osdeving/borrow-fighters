//! Defines close-range move runtime types for the greybox prototype.
//!
//! `AttackKind` remains the runtime compatibility enum while move details now
//! live in table-driven `MoveSpec` data.

use crate::math::rect::Rect;

pub use super::move_data::{
    AttackFrameData, HEAVY_PUNCH_DAMAGE, KICK_DAMAGE, LIGHT_PUNCH_DAMAGE, MoveId, MoveSpec,
    move_spec,
};

/// Close-range attacks available in the greybox prototype.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AttackKind {
    LightPunch,
    HeavyPunch,
    Kick,
}

/// A currently active offensive shape.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ActiveAttack {
    pub kind: AttackKind,
    pub hitbox: Rect,
    pub damage: i32,
}

impl AttackKind {
    /// Returns the stable move id represented by this runtime attack kind.
    pub const fn move_id(self) -> MoveId {
        match self {
            Self::LightPunch => MoveId::LightPunch,
            Self::HeavyPunch => MoveId::HeavyPunch,
            Self::Kick => MoveId::Kick,
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
