//! Defines fighter state and attack timing.
//!
//! Fighters are greybox rectangles with simple movement, jump, one basic attack,
//! health, and facing.

use crate::config::{ARENA_LEFT, ARENA_RIGHT, FLOOR_Y};
use crate::math::{rect::Rect, vec2::Vec2};

const WIDTH: f32 = 52.0;
const HEIGHT: f32 = 96.0;
const MOVE_SPEED: f32 = 260.0;
const JUMP_SPEED: f32 = -640.0;
const GRAVITY: f32 = 1850.0;
const MAX_FALL_SPEED: f32 = 900.0;
const ATTACK_DURATION: f32 = 0.30;
const ATTACK_ACTIVE_START: f32 = 0.08;
const ATTACK_ACTIVE_END: f32 = 0.18;
const HITBOX_WIDTH: f32 = 58.0;
const HITBOX_HEIGHT: f32 = 34.0;
const HITBOX_Y_OFFSET: f32 = 30.0;
pub const BASIC_DAMAGE: i32 = 12;

/// Identifies a local player slot.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlayerSlot {
    One,
    Two,
}

/// Horizontal direction used by attacks and rendering.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Facing {
    Left,
    Right,
}

/// Visible attack phase used by debug rendering.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AttackPhase {
    Idle,
    Startup,
    Active,
    Recovery,
}

/// Input commands for one fighter during one simulation tick.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FighterInput {
    pub left: bool,
    pub right: bool,
    pub jump: bool,
    pub attack: bool,
}

/// A greybox fighter used by the first vertical slice.
#[derive(Clone, Debug)]
pub struct Fighter {
    pub slot: PlayerSlot,
    pub name: &'static str,
    pub position: Vec2,
    pub velocity: Vec2,
    pub health: i32,
    pub facing: Facing,
    pub grounded: bool,
    attack: Option<AttackState>,
}

#[derive(Clone, Copy, Debug)]
struct AttackState {
    elapsed: f32,
    has_hit: bool,
}

impl Fighter {
    /// Creates a fighter standing on the arena floor.
    pub fn new(slot: PlayerSlot, name: &'static str, x: f32) -> Self {
        Self {
            slot,
            name,
            position: Vec2::new(x, FLOOR_Y - HEIGHT),
            velocity: Vec2::ZERO,
            health: 100,
            facing: Facing::Right,
            grounded: true,
            attack: None,
        }
    }

    /// Advances movement and attack timers.
    pub fn update(&mut self, dt: f32, input: FighterInput) {
        if self.is_defeated() {
            self.velocity = Vec2::ZERO;
            return;
        }

        self.velocity.x = match (input.left, input.right) {
            (true, false) => -MOVE_SPEED,
            (false, true) => MOVE_SPEED,
            _ => 0.0,
        };

        if input.jump && self.grounded {
            self.velocity.y = JUMP_SPEED;
            self.grounded = false;
        }

        if input.attack && self.attack.is_none() {
            self.attack = Some(AttackState {
                elapsed: 0.0,
                has_hit: false,
            });
        }

        self.velocity.y = (self.velocity.y + GRAVITY * dt).min(MAX_FALL_SPEED);
        self.position.x += self.velocity.x * dt;
        self.position.y += self.velocity.y * dt;

        self.clamp_to_arena();
        if self.position.y + HEIGHT >= FLOOR_Y {
            self.position.y = FLOOR_Y - HEIGHT;
            self.velocity.y = 0.0;
            self.grounded = true;
        }

        if let Some(mut attack) = self.attack {
            attack.elapsed += dt;
            self.attack = (attack.elapsed < ATTACK_DURATION).then_some(attack);
        }
    }

    /// Updates facing direction to look toward the opponent.
    pub fn face_toward(&mut self, opponent: &Self) {
        self.facing = if self.body_rect().center_x() <= opponent.body_rect().center_x() {
            Facing::Right
        } else {
            Facing::Left
        };
    }

    /// Keeps the fighter inside the horizontal arena bounds.
    pub fn clamp_to_arena(&mut self) {
        self.position.x = self.position.x.clamp(ARENA_LEFT, ARENA_RIGHT - WIDTH);
    }

    /// Applies basic attack damage.
    pub fn take_basic_hit(&mut self) {
        self.health = (self.health - BASIC_DAMAGE).max(0);
    }

    /// Returns whether this fighter can currently deal a new hit.
    pub fn can_register_hit(&self) -> bool {
        self.attack
            .is_some_and(|attack| attack.is_active() && !attack.has_hit)
    }

    /// Marks the current attack as having hit once.
    pub fn mark_attack_hit(&mut self) {
        if let Some(attack) = &mut self.attack {
            attack.has_hit = true;
        }
    }

    /// Returns the body rectangle used for drawing and simple arena logic.
    pub fn body_rect(&self) -> Rect {
        Rect::new(self.position.x, self.position.y, WIDTH, HEIGHT)
    }

    /// Returns the vulnerable area.
    pub fn hurtbox(&self) -> Rect {
        Rect::new(
            self.position.x + 8.0,
            self.position.y + 6.0,
            WIDTH - 16.0,
            HEIGHT - 8.0,
        )
    }

    /// Returns the active hitbox if the current attack is active.
    pub fn active_hitbox(&self) -> Option<Rect> {
        self.attack
            .and_then(|attack| attack.is_active().then(|| self.attack_box()))
    }

    /// Returns the attack reach box while the punch animation is running.
    pub fn attack_box(&self) -> Rect {
        let x = if self.facing == Facing::Right {
            self.position.x + WIDTH
        } else {
            self.position.x - HITBOX_WIDTH
        };
        Rect::new(
            x,
            self.position.y + HITBOX_Y_OFFSET,
            HITBOX_WIDTH,
            HITBOX_HEIGHT,
        )
    }

    /// Returns the current attack phase for debug rendering.
    pub fn attack_phase(&self) -> AttackPhase {
        self.attack.map_or(AttackPhase::Idle, AttackState::phase)
    }

    /// Returns true when health reached zero.
    pub fn is_defeated(&self) -> bool {
        self.health <= 0
    }
}

impl AttackState {
    fn is_active(self) -> bool {
        self.elapsed >= ATTACK_ACTIVE_START && self.elapsed <= ATTACK_ACTIVE_END
    }

    fn phase(self) -> AttackPhase {
        if self.elapsed < ATTACK_ACTIVE_START {
            AttackPhase::Startup
        } else if self.elapsed <= ATTACK_ACTIVE_END {
            AttackPhase::Active
        } else {
            AttackPhase::Recovery
        }
    }
}
