//! Defines close-range move data for the greybox prototype.
//!
//! The current moves are hard-coded because the prototype needs readable combat
//! behavior before a data-driven character system.

use crate::math::rect::Rect;

pub const LIGHT_PUNCH_DAMAGE: i32 = 8;
pub const HEAVY_PUNCH_DAMAGE: i32 = 16;
pub const KICK_DAMAGE: i32 = 12;

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

#[derive(Clone, Copy, Debug)]
pub(crate) struct AttackSpec {
    pub duration: f32,
    pub active_start: f32,
    pub active_end: f32,
    pub hitbox_width: f32,
    pub hitbox_height: f32,
    pub hitbox_y_offset: f32,
    pub damage: i32,
}

impl AttackKind {
    /// Returns the short label used by debug rendering.
    pub const fn label(self) -> &'static str {
        match self {
            Self::LightPunch => "LP",
            Self::HeavyPunch => "HP",
            Self::Kick => "KICK",
        }
    }

    /// Returns the damage applied by this move before guard reduction.
    pub const fn damage(self) -> i32 {
        self.spec().damage
    }

    pub(crate) const fn spec(self) -> AttackSpec {
        match self {
            Self::LightPunch => AttackSpec {
                duration: 0.30,
                active_start: 0.08,
                active_end: 0.17,
                hitbox_width: 58.0,
                hitbox_height: 34.0,
                hitbox_y_offset: 62.0,
                damage: LIGHT_PUNCH_DAMAGE,
            },
            Self::HeavyPunch => AttackSpec {
                duration: 0.58,
                active_start: 0.18,
                active_end: 0.34,
                hitbox_width: 96.0,
                hitbox_height: 42.0,
                hitbox_y_offset: 58.0,
                damage: HEAVY_PUNCH_DAMAGE,
            },
            Self::Kick => AttackSpec {
                duration: 0.46,
                active_start: 0.14,
                active_end: 0.28,
                hitbox_width: 100.0,
                hitbox_height: 36.0,
                hitbox_y_offset: 108.0,
                damage: KICK_DAMAGE,
            },
        }
    }
}
