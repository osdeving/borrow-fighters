//! Advances capture and ballistic damage reactions independently of player input.
//!
//! System: Fighter combat state. World coordinates paired throws; this module
//! keeps airborne victims under gravity until a protected ground recovery begins.

use super::{CaptureRole, Fighter, FrameCount, GRAVITY, HitReactionKind, MAX_FALL_SPEED};
use crate::{
    config::{ARENA_LEFT, ARENA_RIGHT, FLOOR_Y},
    math::vec2::Vec2,
};

impl Fighter {
    /// Returns whether an opponent currently holds this fighter for a throw.
    pub fn in_capture(&self) -> bool {
        self.capture_role == Some(CaptureRole::Victim)
    }

    /// Returns whether this fighter currently holds an opponent.
    pub fn is_throwing(&self) -> bool {
        self.capture_role == Some(CaptureRole::Attacker)
    }

    /// Returns whether damage, rather than input, controls the current flight.
    pub fn in_air_reaction(&self) -> bool {
        !self.grounded
            && matches!(
                self.hit_reaction_kind,
                HitReactionKind::Launched | HitReactionKind::Thrown
            )
    }

    /// Recovery/capture victims cannot be repeatedly hit before they can respond.
    pub fn has_protected_reaction(&self) -> bool {
        self.in_knockdown() || self.in_air_reaction() || self.capture_role.is_some()
    }

    /// Begins a gravity-driven reaction without changing the current position.
    pub fn begin_launch(&mut self, velocity: Vec2, kind: HitReactionKind) {
        debug_assert!(matches!(
            kind,
            HitReactionKind::Launched | HitReactionKind::Thrown
        ));
        self.capture_role = None;
        self.attack = None;
        self.blocking = false;
        self.crouching = false;
        self.grounded = false;
        self.special_visual_timer = 0.0;
        self.blockstun_timer = 0.0;
        self.whiff_recovery_timer = 0.0;
        self.hitstun_timer = 0.0;
        self.hit_reaction_kind = kind;
        self.reaction_visual_elapsed = 0.0;
        self.velocity = velocity;
    }

    /// Marks a grounded impact as heavy without changing its physical stun.
    pub(crate) fn mark_heavy_reaction(&mut self) {
        self.hit_reaction_kind = HitReactionKind::HeavyHit;
    }

    pub(crate) fn begin_throw_capture(&mut self, is_attacker: bool) {
        self.capture_role = Some(if is_attacker {
            CaptureRole::Attacker
        } else {
            CaptureRole::Victim
        });
        self.velocity = Vec2::ZERO;
        self.crouching = false;
        self.blocking = false;
        self.blockstun_timer = 0.0;
        self.special_visual_timer = 0.0;
        self.reaction_visual_elapsed = 0.0;
        if !is_attacker {
            self.attack = None;
            self.hit_reaction_kind = HitReactionKind::Thrown;
        }
    }

    pub(crate) fn release_throw_capture(&mut self) {
        self.capture_role = None;
        if let Some(attack) = &mut self.attack {
            attack.elapsed += self.reaction_visual_elapsed;
        }
        self.whiff_recovery_timer = FrameCount::new(12).as_seconds();
    }

    pub(super) fn advance_air_reaction(&mut self, dt: f32) {
        self.reaction_visual_elapsed += dt;
        self.velocity.y = (self.velocity.y + GRAVITY * dt).min(MAX_FALL_SPEED);
        self.position.x += self.velocity.x * dt;
        self.position.y += self.velocity.y * dt;
        let max_x = ARENA_RIGHT - self.body_metrics.width;
        if self.position.x < ARENA_LEFT || self.position.x > max_x {
            self.position.x = self.position.x.clamp(ARENA_LEFT, max_x);
            self.velocity.x = 0.0;
        }
        if self.position.y + self.body_metrics.standing_height >= FLOOR_Y {
            self.start_knockdown();
            // Airborne victims already completed their fall. Skip the initial
            // upright fall key while preserving the complete recovery timer.
            self.reaction_visual_elapsed = 0.1;
        }
    }
}
