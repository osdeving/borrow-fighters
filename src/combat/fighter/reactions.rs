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
        self.reaction_landed = false;
        // Positive root of y + vy*t + gravity*t²/2 = floor.
        let height = (FLOOR_Y - self.body_rect().bottom()).max(0.0);
        self.reaction_visual_duration =
            (-velocity.y + (velocity.y * velocity.y + 2.0 * GRAVITY * height).sqrt()) / GRAVITY;
        self.velocity = velocity;
    }

    /// Marks a grounded impact as heavy without changing its physical stun.
    pub(crate) fn mark_heavy_reaction(&mut self) {
        // A heavy strike on an airborne target must retain the ballistic
        // reaction that take_hit already started, rather than falling to idle.
        if !self.in_air_reaction() {
            self.hit_reaction_kind = HitReactionKind::HeavyHit;
        }
        self.reaction_strength = self.reaction_strength.max(1.35);
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
        self.reaction_visual_duration = FrameCount::new(12).as_seconds();
        self.reaction_landed = false;
        self.super_reaction = false;
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
            // Presentation uses `landed` to begin on the floor-impact pose.
            // Keep the actual recovery clock monotonic from zero.
        }
    }
}

/// Contact-local presentation data shared by match, Lab and showcase renderers.
/// Values describe a pose only; sprite deformation never changes a hurtbox.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ReactionVisualState {
    pub kind: HitReactionKind,
    pub elapsed_seconds: f32,
    pub duration_seconds: f32,
    pub strength: f32,
    pub guarded: bool,
    pub crouched: bool,
    pub landed: bool,
    pub cinematic: bool,
}

impl ReactionVisualState {
    /// Progress through the actual stun/recovery, independent of atlas duration.
    pub fn progress(self) -> f32 {
        (self.elapsed_seconds / self.duration_seconds.max(1.0 / 60.0)).clamp(0.0, 1.0)
    }
}

impl Fighter {
    /// Returns the current contact clock and intensity, before idle/attack clocks.
    pub fn reaction_visual_state(&self) -> Option<ReactionVisualState> {
        self.is_reacting().then_some(ReactionVisualState {
            kind: self.hit_reaction_kind,
            elapsed_seconds: self.reaction_visual_elapsed,
            duration_seconds: self.reaction_visual_duration,
            strength: self.reaction_strength,
            guarded: self.in_blockstun(),
            crouched: self.reaction_was_crouching,
            landed: self.reaction_landed,
            cinematic: self.super_reaction,
        })
    }

    /// Applies fall, prone and get-up after a sweep or ballistic floor contact.
    pub fn start_knockdown(&mut self) {
        self.reaction_landed = !self.grounded;
        self.capture_role = None;
        self.attack = None;
        self.special_visual_timer = 0.0;
        self.blocking = false;
        self.blockstun_timer = 0.0;
        self.hit_reaction_kind = HitReactionKind::Knockdown;
        self.hitstun_timer = self.hitstun_timer.max(FrameCount::new(36).as_seconds());
        self.reaction_visual_duration = self.hitstun_timer;
        self.position.y = FLOOR_Y - self.body_metrics.standing_height;
        self.grounded = true;
        self.crouching = false;
        self.velocity = Vec2::ZERO;
        self.reaction_visual_elapsed = 0.0;
    }

    pub(crate) fn begin_super_capture(&mut self, guarded: bool, crouching: bool) {
        self.attack = None;
        self.capture_role = None;
        self.special_visual_timer = 0.0;
        self.hitstun_timer = 0.0;
        self.blockstun_timer = 0.0;
        self.whiff_recovery_timer = 0.0;
        self.velocity = Vec2::ZERO;
        self.blocking = guarded;
        self.crouching = guarded && crouching;
        self.hit_reaction_kind = HitReactionKind::Hit;
        self.reaction_visual_elapsed = 0.0;
        self.reaction_visual_duration = 0.0;
        self.reaction_landed = false;
        self.super_reaction = false;
    }

    pub(crate) fn advance_super_visuals(&mut self, dt: f32) {
        if self.in_air_reaction() {
            self.advance_air_reaction(dt);
            return;
        }
        if self.is_reacting() {
            self.reaction_visual_elapsed += dt;
            // Lethal victims stay prone. Living victims complete recovery even
            // during a long cinematic; the next contact starts a fresh recoil.
            if !self.is_defeated() {
                self.hitstun_timer = super::tick_timer(self.hitstun_timer, dt);
            }
            self.blockstun_timer = super::tick_timer(self.blockstun_timer, dt);
        }
    }

    pub(crate) fn receive_super_contact(&mut self, damage: i32, guarded: bool, knockdown: bool) {
        self.take_damage(damage);
        self.attack = None;
        self.special_visual_timer = 0.0;
        self.capture_role = None;
        self.reaction_visual_elapsed = 0.0;
        self.reaction_strength = 2.0;
        self.reaction_was_crouching = self.crouching;
        self.reaction_landed = false;
        self.super_reaction = true;
        if guarded {
            self.blocking = true;
            self.blockstun_timer = FrameCount::new(16).as_seconds();
            self.reaction_visual_duration = self.blockstun_timer;
        } else if knockdown {
            self.launch_super_target();
        } else {
            self.blocking = false;
            self.blockstun_timer = 0.0;
            self.hit_reaction_kind = HitReactionKind::HeavyHit;
            self.hitstun_timer = FrameCount::new(24).as_seconds();
            self.reaction_visual_duration = self.hitstun_timer;
        }
    }

    fn launch_super_target(&mut self) {
        use crate::config::world_px;
        let direction = if self.facing == super::Facing::Right {
            -1.0
        } else {
            1.0
        };
        let flight_seconds = 2.0 * world_px(340.0) / GRAVITY;
        // Keep the dramatic vertical arc while limiting horizontal travel near
        // a corner. Reviewed art extends beyond its body, especially Duke.
        let room = if direction > 0.0 {
            ARENA_RIGHT - self.body_metrics.width - world_px(24.0) - self.position.x
        } else {
            self.position.x - ARENA_LEFT - world_px(24.0)
        }
        .max(0.0);
        let horizontal_speed = world_px(90.0).min(room / flight_seconds);
        self.begin_launch(
            Vec2::new(direction * horizontal_speed, world_px(-340.0)),
            HitReactionKind::Launched,
        );
        self.super_reaction = true;
        self.reaction_strength = 2.0;
    }

    /// Restarts the return impact after a hidden victim reappears, without damage.
    pub(crate) fn resume_super_target_reaction(&mut self) {
        if !self.super_reaction {
            return;
        }
        if self.blocking {
            self.reaction_visual_elapsed = 0.0;
            self.blockstun_timer = FrameCount::new(16).as_seconds();
            self.reaction_visual_duration = self.blockstun_timer;
        } else {
            self.launch_super_target();
        }
    }

    pub(crate) fn finish_super_capture(&mut self) {
        self.blocking = false;
        self.blockstun_timer = 0.0;
        // A late authored contact must finish its flight/landing in the normal
        // World update rather than being frozen midair by the scene transition.
        if !self.in_air_reaction() && !self.in_knockdown() {
            self.velocity = Vec2::ZERO;
            self.hitstun_timer = 0.0;
        }
        self.throw_protection_timer = FrameCount::new(6).as_seconds();
    }
}
