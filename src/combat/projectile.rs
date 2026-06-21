//! Defines simple horizontal projectiles for the greybox prototype.
//!
//! System: Combat runtime. This module owns projectile state and timing data,
//! but does not draw effects or resolve full match flow.
//!
//! Projectiles are domain data, not render objects, so their collisions can be
//! tested without opening a Raylib window.

use crate::combat::fighter::{Facing, Fighter, GuardRule, HitReaction, PlayerSlot};
use crate::math::{rect::Rect, vec2::Vec2};

use super::frame::FrameCount;

const WIDTH: f32 = 44.0;
const HEIGHT: f32 = 30.0;
const FRONT_SPAWN_OFFSET: f32 = 66.0;
const CENTER_Y_FROM_BODY_BOTTOM: f32 = 88.0;
pub const PROJECTILE_SPEED: f32 = 340.0;
pub const PROJECTILE_DAMAGE: i32 = 8;
pub const PROJECTILE_HIT_REACTION: HitReaction = HitReaction {
    hitstun: FrameCount::new(16),
    blockstun: FrameCount::new(12),
    hit_pushback: 30.0,
    block_pushback: 24.0,
};
pub const PROJECTILE_GUARD_RULE: GuardRule = GuardRule::Projectile;

/// Whole-frame timing data for the current projectile special.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ProjectileFrameData {
    pub startup: FrameCount,
    pub spawn_frame: FrameCount,
    pub visual_duration: FrameCount,
    pub cooldown: FrameCount,
}

/// Fireball frame data for Prototype 0.1.
///
/// The projectile currently spawns immediately; Combat Lab work will decide if
/// startup/recovery should become real lockout frames instead of only cooldown.
pub const PROJECTILE_FRAME_DATA: ProjectileFrameData = ProjectileFrameData {
    startup: FrameCount::ZERO,
    spawn_frame: FrameCount::ZERO,
    visual_duration: FrameCount::new(21),
    cooldown: FrameCount::new(57),
};

/// A hadouken-like projectile moving horizontally across the arena.
#[derive(Clone, Debug, PartialEq)]
pub struct Projectile {
    pub owner: PlayerSlot,
    pub position: Vec2,
    pub velocity: Vec2,
    pub damage: i32,
    pub guard_rule: GuardRule,
    pub hit_reaction: HitReaction,
    pub alive: bool,
}

impl Projectile {
    /// Spawns a projectile from the front side of a fighter.
    pub fn from_fighter(fighter: &Fighter) -> Self {
        let body = fighter.body_rect();
        let direction = match fighter.facing {
            Facing::Left => -1.0,
            Facing::Right => 1.0,
        };
        let x = if fighter.facing == Facing::Right {
            body.right() + FRONT_SPAWN_OFFSET
        } else {
            body.x - WIDTH - FRONT_SPAWN_OFFSET
        };
        let center_y = body.bottom() - CENTER_Y_FROM_BODY_BOTTOM;

        Self {
            owner: fighter.slot,
            position: Vec2::new(x, center_y - HEIGHT * 0.5),
            velocity: Vec2::new(direction * PROJECTILE_SPEED, 0.0),
            damage: PROJECTILE_DAMAGE,
            guard_rule: PROJECTILE_GUARD_RULE,
            hit_reaction: PROJECTILE_HIT_REACTION,
            alive: true,
        }
    }

    /// Advances projectile position.
    pub fn update(&mut self, dt: f32) {
        self.position.x += self.velocity.x * dt;
        self.position.y += self.velocity.y * dt;
    }

    /// Returns the collision rectangle.
    pub fn rect(&self) -> Rect {
        Rect::new(self.position.x, self.position.y, WIDTH, HEIGHT)
    }
}
