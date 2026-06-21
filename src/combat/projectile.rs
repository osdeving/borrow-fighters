//! Defines simple horizontal projectiles for the greybox prototype.
//!
//! Projectiles are domain data, not render objects, so their collisions can be
//! tested without opening a Raylib window.

use crate::combat::fighter::{Facing, Fighter, PlayerSlot};
use crate::math::{rect::Rect, vec2::Vec2};

const WIDTH: f32 = 44.0;
const HEIGHT: f32 = 30.0;
const FRONT_SPAWN_OFFSET: f32 = 66.0;
const CENTER_Y_FROM_BODY_BOTTOM: f32 = 88.0;
pub const PROJECTILE_SPEED: f32 = 340.0;
pub const PROJECTILE_DAMAGE: i32 = 8;

/// A hadouken-like projectile moving horizontally across the arena.
#[derive(Clone, Debug, PartialEq)]
pub struct Projectile {
    pub owner: PlayerSlot,
    pub position: Vec2,
    pub velocity: Vec2,
    pub damage: i32,
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
