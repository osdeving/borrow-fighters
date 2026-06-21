//! Defines close-range move data for the greybox prototype.
//!
//! The current moves are hard-coded because the prototype needs readable combat
//! behavior before a data-driven character system.

use crate::math::rect::Rect;

use super::frame::FrameCount;

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

/// Whole-frame timing data for one close-range attack.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AttackFrameData {
    pub duration: FrameCount,
    pub active_start: FrameCount,
    pub active_end: FrameCount,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct AttackSpec {
    pub frames: AttackFrameData,
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

    /// Returns whole-frame startup, active, and duration data.
    pub const fn frame_data(self) -> AttackFrameData {
        self.spec().frames
    }

    pub(crate) const fn spec(self) -> AttackSpec {
        match self {
            Self::LightPunch => AttackSpec {
                frames: AttackFrameData {
                    duration: FrameCount::new(18),
                    active_start: FrameCount::new(5),
                    active_end: FrameCount::new(10),
                },
                hitbox_width: 58.0,
                hitbox_height: 34.0,
                hitbox_y_offset: 62.0,
                damage: LIGHT_PUNCH_DAMAGE,
            },
            Self::HeavyPunch => AttackSpec {
                frames: AttackFrameData {
                    duration: FrameCount::new(35),
                    active_start: FrameCount::new(11),
                    active_end: FrameCount::new(20),
                },
                hitbox_width: 96.0,
                hitbox_height: 42.0,
                hitbox_y_offset: 58.0,
                damage: HEAVY_PUNCH_DAMAGE,
            },
            Self::Kick => AttackSpec {
                frames: AttackFrameData {
                    duration: FrameCount::new(28),
                    active_start: FrameCount::new(9),
                    active_end: FrameCount::new(16),
                },
                hitbox_width: 100.0,
                hitbox_height: 36.0,
                hitbox_y_offset: 108.0,
                damage: KICK_DAMAGE,
            },
        }
    }
}
