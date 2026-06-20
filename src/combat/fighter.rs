//! Defines fighter state, movement, defense, and attack timing.
//!
//! Fighters are still greybox primitives, but their body, hurtboxes, and moves
//! are split enough to test traditional fighting-game verbs without sprites.

use crate::config::{ARENA_LEFT, ARENA_RIGHT, FLOOR_Y};
use crate::math::{rect::Rect, vec2::Vec2};

pub use crate::combat::move_set::{
    ActiveAttack, AttackKind, HEAVY_PUNCH_DAMAGE, KICK_DAMAGE, LIGHT_PUNCH_DAMAGE,
};

const WIDTH: f32 = 52.0;
const STANDING_HEIGHT: f32 = 96.0;
const CROUCH_HEIGHT: f32 = 66.0;
const MAX_RUN_SPEED: f32 = 280.0;
const ATTACK_MOVE_SPEED_FACTOR: f32 = 0.55;
const GROUND_ACCELERATION: f32 = 2200.0;
const AIR_ACCELERATION: f32 = 1350.0;
const GROUND_DECELERATION: f32 = 2600.0;
const AIR_DECELERATION: f32 = 700.0;
const DIAGONAL_JUMP_MIN_SPEED: f32 = 180.0;
const JUMP_SPEED: f32 = -680.0;
const GRAVITY: f32 = 1650.0;
const MAX_FALL_SPEED: f32 = 920.0;
const PROJECTILE_COOLDOWN: f32 = 0.95;
const BLOCK_DAMAGE_DIVISOR: i32 = 4;

pub const BASIC_DAMAGE: i32 = LIGHT_PUNCH_DAMAGE;

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
    pub crouch: bool,
    pub block: bool,
    pub light_punch: bool,
    pub heavy_punch: bool,
    pub kick: bool,
    pub projectile: bool,
}

/// Result of applying a hit to a defender.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DamageResult {
    pub damage: i32,
    pub blocked: bool,
}

/// Composed body or hurtbox pieces for debug drawing and collision checks.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FighterBodyParts {
    pub head: Rect,
    pub torso: Rect,
    pub legs: Rect,
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
    pub crouching: bool,
    pub blocking: bool,
    projectile_cooldown: f32,
    attack: Option<AttackState>,
}

#[derive(Clone, Copy, Debug)]
struct AttackState {
    kind: AttackKind,
    elapsed: f32,
    has_hit: bool,
}

impl Fighter {
    /// Creates a fighter standing on the arena floor.
    pub fn new(slot: PlayerSlot, name: &'static str, x: f32) -> Self {
        Self {
            slot,
            name,
            position: Vec2::new(x, FLOOR_Y - STANDING_HEIGHT),
            velocity: Vec2::ZERO,
            health: 100,
            facing: Facing::Right,
            grounded: true,
            crouching: false,
            blocking: false,
            projectile_cooldown: 0.0,
            attack: None,
        }
    }

    /// Advances movement, stance, defense, and attack timers.
    pub fn update(&mut self, dt: f32, input: FighterInput) {
        if self.is_defeated() {
            self.velocity = Vec2::ZERO;
            self.crouching = false;
            self.blocking = false;
            return;
        }

        self.projectile_cooldown = (self.projectile_cooldown - dt).max(0.0);
        self.crouching = input.crouch && self.grounded && self.attack.is_none();
        self.blocking = input.block && self.grounded && self.attack.is_none();
        self.update_horizontal_velocity(dt, input);

        let can_start_action = !self.crouching && !self.blocking;
        if input.jump && self.grounded && can_start_action {
            self.velocity.y = JUMP_SPEED;
            self.grounded = false;
            self.apply_diagonal_jump_boost(input);
        }

        if let Some(kind) = input.requested_attack()
            && self.attack.is_none()
            && can_start_action
        {
            self.attack = Some(AttackState {
                kind,
                elapsed: 0.0,
                has_hit: false,
            });
        }

        self.velocity.y = (self.velocity.y + GRAVITY * dt).min(MAX_FALL_SPEED);
        self.position.x += self.velocity.x * dt;
        self.position.y += self.velocity.y * dt;

        self.clamp_to_arena();
        if self.position.y + STANDING_HEIGHT >= FLOOR_Y {
            self.position.y = FLOOR_Y - STANDING_HEIGHT;
            self.velocity.y = 0.0;
            self.grounded = true;
        }

        if let Some(mut attack) = self.attack {
            attack.elapsed += dt;
            self.attack = (attack.elapsed < attack.kind.spec().duration).then_some(attack);
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

    /// Applies light-punch damage for legacy tests and callers.
    pub fn take_basic_hit(&mut self) {
        self.take_damage(BASIC_DAMAGE);
    }

    /// Applies arbitrary damage without checking guard.
    pub fn take_damage(&mut self, damage: i32) {
        self.health = (self.health - damage).max(0);
    }

    /// Applies incoming damage with the current block state.
    pub fn take_hit(&mut self, damage: i32) -> DamageResult {
        let blocked = self.blocking && !self.is_defeated();
        let final_damage = if blocked {
            (damage / BLOCK_DAMAGE_DIVISOR).max(1)
        } else {
            damage
        };
        self.take_damage(final_damage);

        DamageResult {
            damage: final_damage,
            blocked,
        }
    }

    /// Returns true if this fighter can fire a projectile this tick.
    pub fn can_fire_projectile(&self) -> bool {
        !self.is_defeated()
            && self.grounded
            && !self.crouching
            && !self.blocking
            && self.attack.is_none()
            && self.projectile_cooldown <= 0.0
    }

    /// Starts the projectile cooldown after firing.
    pub fn mark_projectile_fired(&mut self) {
        self.projectile_cooldown = PROJECTILE_COOLDOWN;
    }

    /// Returns whether this fighter can currently deal a new close hit.
    pub fn can_register_hit(&self) -> bool {
        self.attack
            .is_some_and(|attack| attack.is_active() && !attack.has_hit)
    }

    /// Marks the current close attack as having hit once.
    pub fn mark_attack_hit(&mut self) {
        if let Some(attack) = &mut self.attack {
            attack.has_hit = true;
        }
    }

    /// Returns the body rectangle used for drawing and simple arena logic.
    pub fn body_rect(&self) -> Rect {
        let height = self.body_height();
        Rect::new(
            self.position.x,
            self.position.y + (STANDING_HEIGHT - height),
            WIDTH,
            height,
        )
    }

    /// Returns composed body pieces for greybox readability.
    pub fn body_parts(&self) -> FighterBodyParts {
        let body = self.body_rect();
        if self.crouching {
            FighterBodyParts {
                head: Rect::new(body.x + 16.0, body.y + 4.0, 22.0, 18.0),
                torso: Rect::new(body.x + 9.0, body.y + 24.0, 34.0, 26.0),
                legs: Rect::new(body.x + 5.0, body.y + 52.0, 42.0, 12.0),
            }
        } else {
            FighterBodyParts {
                head: Rect::new(body.x + 14.0, body.y + 4.0, 24.0, 24.0),
                torso: Rect::new(body.x + 10.0, body.y + 32.0, 32.0, 34.0),
                legs: Rect::new(body.x + 7.0, body.y + 72.0, 38.0, 20.0),
            }
        }
    }

    /// Returns composed vulnerable areas.
    pub fn hurtboxes(&self) -> FighterBodyParts {
        let parts = self.body_parts();
        FighterBodyParts {
            head: inset_rect(parts.head, 2.0),
            torso: inset_rect(parts.torso, 2.0),
            legs: inset_rect(parts.legs, 2.0),
        }
    }

    /// Returns the coarse vulnerable area.
    pub fn hurtbox(&self) -> Rect {
        let body = self.body_rect();
        Rect::new(body.x + 8.0, body.y + 5.0, WIDTH - 16.0, body.height - 8.0)
    }

    /// Returns the active hitbox if the current close attack is active.
    pub fn active_hitbox(&self) -> Option<Rect> {
        self.active_attack().map(|attack| attack.hitbox)
    }

    /// Returns the active close attack if the current attack can deal damage.
    pub fn active_attack(&self) -> Option<ActiveAttack> {
        self.attack.and_then(|attack| {
            attack.is_active().then(|| ActiveAttack {
                kind: attack.kind,
                hitbox: self.attack_box_for(attack.kind),
                damage: attack.kind.damage(),
            })
        })
    }

    /// Returns the attack reach box while an attack animation is running.
    pub fn attack_box(&self) -> Option<Rect> {
        self.attack.map(|attack| self.attack_box_for(attack.kind))
    }

    /// Returns the current close attack kind, if any.
    pub fn attack_kind(&self) -> Option<AttackKind> {
        self.attack.map(|attack| attack.kind)
    }

    /// Returns the current attack phase for debug rendering.
    pub fn attack_phase(&self) -> AttackPhase {
        self.attack.map_or(AttackPhase::Idle, AttackState::phase)
    }

    /// Returns the defensive box drawn in front of a blocking fighter.
    pub fn guard_box(&self) -> Rect {
        let body = self.body_rect();
        let width = 12.0;
        let x = if self.facing == Facing::Right {
            body.right() + 4.0
        } else {
            body.x - width - 4.0
        };
        Rect::new(x, body.y + 8.0, width, body.height - 16.0)
    }

    /// Returns true when health reached zero.
    pub fn is_defeated(&self) -> bool {
        self.health <= 0
    }

    fn update_horizontal_velocity(&mut self, dt: f32, input: FighterInput) {
        let axis = if self.crouching || self.blocking {
            0.0
        } else {
            input.horizontal_axis()
        };
        let acceleration = if self.grounded {
            GROUND_ACCELERATION
        } else {
            AIR_ACCELERATION
        };
        let deceleration = if self.grounded {
            GROUND_DECELERATION
        } else {
            AIR_DECELERATION
        };
        let max_speed = if self.attack.is_some() {
            MAX_RUN_SPEED * ATTACK_MOVE_SPEED_FACTOR
        } else {
            MAX_RUN_SPEED
        };

        if axis != 0.0 {
            self.velocity.x = approach(self.velocity.x, axis * max_speed, acceleration * dt);
        } else {
            self.velocity.x = approach(self.velocity.x, 0.0, deceleration * dt);
        }
    }

    fn apply_diagonal_jump_boost(&mut self, input: FighterInput) {
        let axis = input.horizontal_axis();
        if axis == 0.0 {
            return;
        }

        let minimum_speed = axis * DIAGONAL_JUMP_MIN_SPEED;
        if self.velocity.x.abs() < DIAGONAL_JUMP_MIN_SPEED {
            self.velocity.x = minimum_speed;
        }
    }

    fn attack_box_for(&self, kind: AttackKind) -> Rect {
        let body = self.body_rect();
        let spec = kind.spec();
        let x = if self.facing == Facing::Right {
            body.right()
        } else {
            body.x - spec.hitbox_width
        };
        Rect::new(
            x,
            body.y + spec.hitbox_y_offset,
            spec.hitbox_width,
            spec.hitbox_height,
        )
    }

    fn body_height(&self) -> f32 {
        if self.crouching {
            CROUCH_HEIGHT
        } else {
            STANDING_HEIGHT
        }
    }
}

impl AttackState {
    fn is_active(self) -> bool {
        let spec = self.kind.spec();
        self.elapsed >= spec.active_start && self.elapsed <= spec.active_end
    }

    fn phase(self) -> AttackPhase {
        let spec = self.kind.spec();
        if self.elapsed < spec.active_start {
            AttackPhase::Startup
        } else if self.elapsed <= spec.active_end {
            AttackPhase::Active
        } else {
            AttackPhase::Recovery
        }
    }
}

impl FighterInput {
    /// Returns this input with offensive actions removed.
    pub fn without_attacks(self) -> Self {
        Self {
            light_punch: false,
            heavy_punch: false,
            kick: false,
            projectile: false,
            ..self
        }
    }

    fn requested_attack(self) -> Option<AttackKind> {
        if self.heavy_punch {
            Some(AttackKind::HeavyPunch)
        } else if self.kick {
            Some(AttackKind::Kick)
        } else if self.light_punch {
            Some(AttackKind::LightPunch)
        } else {
            None
        }
    }

    fn horizontal_axis(self) -> f32 {
        match (self.left, self.right) {
            (true, false) => -1.0,
            (false, true) => 1.0,
            _ => 0.0,
        }
    }
}

impl FighterBodyParts {
    /// Returns every piece as an array for collision and debug drawing.
    pub const fn rects(self) -> [Rect; 3] {
        [self.head, self.torso, self.legs]
    }
}

fn approach(current: f32, target: f32, max_delta: f32) -> f32 {
    if current < target {
        (current + max_delta).min(target)
    } else if current > target {
        (current - max_delta).max(target)
    } else {
        current
    }
}

fn inset_rect(rect: Rect, amount: f32) -> Rect {
    Rect::new(
        rect.x + amount,
        rect.y + amount,
        (rect.width - amount * 2.0).max(1.0),
        (rect.height - amount * 2.0).max(1.0),
    )
}
