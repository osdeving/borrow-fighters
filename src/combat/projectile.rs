//! Defines simple horizontal projectiles for the greybox prototype.
//!
//! System: Combat runtime. This module owns projectile state and timing data,
//! but does not draw effects or resolve full match flow.
//!
//! Projectiles are domain data, not render objects, so their collisions can be
//! tested without opening a Raylib window.

use crate::combat::fighter::{Facing, Fighter, GuardRule, HitReaction, PlayerSlot};
use crate::config::world_px;
use crate::math::{rect::Rect, vec2::Vec2};

use super::frame::FrameCount;

pub const PROJECTILE_SPEED: f32 = RUST_PROJECTILE_SPEC.speed;
pub const PROJECTILE_DAMAGE: i32 = RUST_PROJECTILE_SPEC.damage;
pub const PROJECTILE_HIT_REACTION: HitReaction = RUST_PROJECTILE_SPEC.hit_reaction;
pub const PROJECTILE_GUARD_RULE: GuardRule = RUST_PROJECTILE_SPEC.guard_rule;
pub const PROJECTILE_FRAME_DATA: ProjectileFrameData = RUST_PROJECTILE_SPEC.frame_data;
pub const PROJECTILE_SPEC: ProjectileSpec = RUST_PROJECTILE_SPEC;

const RUST_PROJECTILE_HIT_REACTION: HitReaction = HitReaction {
    hitstun: FrameCount::new(16),
    blockstun: FrameCount::new(12),
    hit_pushback: world_px(30.0),
    block_pushback: world_px(24.0),
};
const DUKE_PROJECTILE_HIT_REACTION: HitReaction = HitReaction {
    hitstun: FrameCount::new(18),
    blockstun: FrameCount::new(14),
    hit_pushback: world_px(38.0),
    block_pushback: world_px(30.0),
};
const GO_PROJECTILE_HIT_REACTION: HitReaction = HitReaction {
    hitstun: FrameCount::new(12),
    blockstun: FrameCount::new(8),
    hit_pushback: world_px(18.0),
    block_pushback: world_px(14.0),
};
const C_PROJECTILE_HIT_REACTION: HitReaction = HitReaction {
    hitstun: FrameCount::new(15),
    blockstun: FrameCount::new(10),
    hit_pushback: world_px(26.0),
    block_pushback: world_px(18.0),
};

/// Whole-frame timing data for the current projectile special.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ProjectileFrameData {
    pub startup: FrameCount,
    pub spawn_frame: FrameCount,
    pub visual_duration: FrameCount,
    pub cooldown: FrameCount,
}

/// Tunable projectile data selected by each character spec.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProjectileSpec {
    pub width: f32,
    pub height: f32,
    pub front_spawn_offset: f32,
    pub center_y_from_body_bottom: f32,
    pub speed: f32,
    pub damage: i32,
    pub guard_rule: GuardRule,
    pub hit_reaction: HitReaction,
    pub frame_data: ProjectileFrameData,
    pub max_travel: Option<f32>,
}

/// Rust keeps the current balanced gear projectile as the default.
pub const RUST_PROJECTILE_SPEC: ProjectileSpec = ProjectileSpec {
    width: world_px(44.0),
    height: world_px(30.0),
    front_spawn_offset: world_px(66.0),
    center_y_from_body_bottom: world_px(88.0),
    speed: world_px(340.0),
    damage: 8,
    guard_rule: GuardRule::Projectile,
    hit_reaction: RUST_PROJECTILE_HIT_REACTION,
    frame_data: ProjectileFrameData {
        startup: FrameCount::ZERO,
        spawn_frame: FrameCount::ZERO,
        visual_duration: FrameCount::new(21),
        cooldown: FrameCount::new(57),
    },
    max_travel: None,
};

/// Duke uses a slower, heavier zoning projectile with clearer recovery.
pub const DUKE_PROJECTILE_SPEC: ProjectileSpec = ProjectileSpec {
    width: world_px(54.0),
    height: world_px(34.0),
    front_spawn_offset: world_px(72.0),
    center_y_from_body_bottom: world_px(88.0),
    speed: world_px(270.0),
    damage: 10,
    guard_rule: GuardRule::Projectile,
    hit_reaction: DUKE_PROJECTILE_HIT_REACTION,
    frame_data: ProjectileFrameData {
        startup: FrameCount::ZERO,
        spawn_frame: FrameCount::ZERO,
        visual_duration: FrameCount::new(25),
        cooldown: FrameCount::new(72),
    },
    max_travel: None,
};

/// Go gets a fast, short-lived burst so rushdown does not become full zoning.
pub const GO_PROJECTILE_SPEC: ProjectileSpec = ProjectileSpec {
    width: world_px(36.0),
    height: world_px(24.0),
    front_spawn_offset: world_px(58.0),
    center_y_from_body_bottom: world_px(84.0),
    speed: world_px(430.0),
    damage: 6,
    guard_rule: GuardRule::Projectile,
    hit_reaction: GO_PROJECTILE_HIT_REACTION,
    frame_data: ProjectileFrameData {
        startup: FrameCount::ZERO,
        spawn_frame: FrameCount::ZERO,
        visual_duration: FrameCount::new(16),
        cooldown: FrameCount::new(44),
    },
    max_travel: Some(world_px(320.0)),
};

/// C uses a compact bitstream projectile with moderate speed and recovery.
pub const C_PROJECTILE_SPEC: ProjectileSpec = ProjectileSpec {
    width: world_px(78.0),
    height: world_px(30.0),
    front_spawn_offset: world_px(72.0),
    center_y_from_body_bottom: world_px(90.0),
    speed: world_px(360.0),
    damage: 8,
    guard_rule: GuardRule::Projectile,
    hit_reaction: C_PROJECTILE_HIT_REACTION,
    frame_data: ProjectileFrameData {
        startup: FrameCount::ZERO,
        spawn_frame: FrameCount::ZERO,
        visual_duration: FrameCount::new(20),
        cooldown: FrameCount::new(56),
    },
    max_travel: None,
};

/// Fireball frame data for Prototype 0.1.
pub const RUST_PROJECTILE_FRAME_DATA: ProjectileFrameData = RUST_PROJECTILE_SPEC.frame_data;

/// A hadouken-like projectile moving horizontally across the arena.
#[derive(Clone, Debug, PartialEq)]
pub struct Projectile {
    pub owner: PlayerSlot,
    pub position: Vec2,
    pub velocity: Vec2,
    pub width: f32,
    pub height: f32,
    pub damage: i32,
    pub guard_rule: GuardRule,
    pub hit_reaction: HitReaction,
    pub alive: bool,
    distance_traveled: f32,
    max_travel: Option<f32>,
}

impl Projectile {
    /// Spawns a projectile from the front side of a fighter.
    pub fn from_fighter(fighter: &Fighter) -> Self {
        Self::from_fighter_with_spec(fighter, fighter.projectile_spec())
    }

    /// Spawns a projectile using explicit projectile tuning.
    pub fn from_fighter_with_spec(fighter: &Fighter, spec: ProjectileSpec) -> Self {
        let body = fighter.body_rect();
        let direction = match fighter.facing {
            Facing::Left => -1.0,
            Facing::Right => 1.0,
        };
        let x = if fighter.facing == Facing::Right {
            body.right() + spec.front_spawn_offset
        } else {
            body.x - spec.width - spec.front_spawn_offset
        };
        let center_y = body.bottom() - spec.center_y_from_body_bottom;

        Self {
            owner: fighter.slot,
            position: Vec2::new(x, center_y - spec.height * 0.5),
            velocity: Vec2::new(direction * spec.speed, 0.0),
            width: spec.width,
            height: spec.height,
            damage: spec.damage,
            guard_rule: spec.guard_rule,
            hit_reaction: spec.hit_reaction,
            alive: true,
            distance_traveled: 0.0,
            max_travel: spec.max_travel,
        }
    }

    /// Spawns a projectile from a projected sprite-local origin.
    pub fn from_fighter_with_origin(fighter: &Fighter, origin: Vec2) -> Self {
        Self::from_fighter_with_spec_and_origin(fighter, fighter.projectile_spec(), origin)
    }

    /// Spawns a projectile from a projected sprite-local origin and explicit tuning.
    pub fn from_fighter_with_spec_and_origin(
        fighter: &Fighter,
        spec: ProjectileSpec,
        origin: Vec2,
    ) -> Self {
        let direction = match fighter.facing {
            Facing::Left => -1.0,
            Facing::Right => 1.0,
        };
        let x = match fighter.facing {
            Facing::Left => origin.x - spec.width,
            Facing::Right => origin.x,
        };

        Self {
            owner: fighter.slot,
            position: Vec2::new(x, origin.y - spec.height * 0.5),
            velocity: Vec2::new(direction * spec.speed, 0.0),
            width: spec.width,
            height: spec.height,
            damage: spec.damage,
            guard_rule: spec.guard_rule,
            hit_reaction: spec.hit_reaction,
            alive: true,
            distance_traveled: 0.0,
            max_travel: spec.max_travel,
        }
    }

    /// Advances projectile position.
    pub fn update(&mut self, dt: f32) {
        self.position.x += self.velocity.x * dt;
        self.position.y += self.velocity.y * dt;
        self.distance_traveled += self.velocity.x.abs() * dt;
        if self
            .max_travel
            .is_some_and(|max_travel| self.distance_traveled >= max_travel)
        {
            self.alive = false;
        }
    }

    /// Returns the collision rectangle.
    pub fn rect(&self) -> Rect {
        Rect::new(self.position.x, self.position.y, self.width, self.height)
    }
}
