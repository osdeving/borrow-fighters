//! Defines fighter state, movement, defense, and attack timing.
//!
//! System: Combat runtime. This module owns the mutable fighter model used by
//! matches and tools; data tables live in `move_data` and `characters`.
//!
//! Fighters are still greybox primitives, but their body, hurtboxes, and moves
//! are split enough to test traditional fighting-game verbs without sprites.

use crate::config::{ARENA_LEFT, ARENA_RIGHT, FLOOR_Y};
use crate::math::{rect::Rect, vec2::Vec2};

use super::frame::FrameCount;
use super::projectile::{PROJECTILE_SPEC, ProjectileFrameData, ProjectileSpec};
pub use crate::combat::move_set::{
    AIR_KICK_DAMAGE, AIR_PUNCH_DAMAGE, ActiveAttack, AttackFrameData, AttackKind,
    CLOSE_THROW_DAMAGE, DEFAULT_CLOSE_RANGE_MOVE_IDS, DUKE_BOILERPLATE_POKE_DAMAGE, GuardRule,
    HEAVY_PUNCH_DAMAGE, HitReaction, KICK_DAMAGE, LIGHT_PUNCH_DAMAGE, MoveId, MoveInputKind,
    MoveSpec, OVERHEAD_PUNCH_DAMAGE, RISING_ANTI_AIR_DAMAGE, RUST_BORROW_JAB_DAMAGE,
    SWEEP_KICK_DAMAGE, move_spec_for_input,
};

const WIDTH: f32 = 76.0;
const STANDING_HEIGHT: f32 = 168.0;
const CROUCH_HEIGHT: f32 = 96.0;
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
const BLOCK_DAMAGE_DIVISOR: i32 = 4;
const TIMER_EPSILON: f32 = 0.0001;

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
    WhiffRecovery,
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

/// Events produced while updating one fighter.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FighterUpdateEvents {
    pub close_attack_started: Option<MoveId>,
    pub close_attack_whiffed: Option<MoveId>,
}

/// Result of applying a hit to a defender.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DamageResult {
    pub damage: i32,
    pub blocked: bool,
    pub pushback: f32,
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
    pub max_health: i32,
    move_ids: &'static [MoveId],
    projectile_spec: ProjectileSpec,
    pub facing: Facing,
    pub grounded: bool,
    pub crouching: bool,
    pub blocking: bool,
    projectile_cooldown: f32,
    special_visual_timer: f32,
    hitstun_timer: f32,
    blockstun_timer: f32,
    whiff_recovery_timer: f32,
    attack: Option<AttackState>,
}

#[derive(Clone, Copy, Debug)]
struct AttackState {
    spec: MoveSpec,
    elapsed: f32,
    has_hit: bool,
}

impl Fighter {
    /// Creates a fighter standing on the arena floor.
    pub fn new(slot: PlayerSlot, name: &'static str, x: f32) -> Self {
        Self::new_with_max_health(slot, name, 100, x)
    }

    /// Creates a fighter with character-provided maximum health.
    pub fn new_with_max_health(
        slot: PlayerSlot,
        name: &'static str,
        max_health: i32,
        x: f32,
    ) -> Self {
        Self::new_with_loadout(slot, name, max_health, &DEFAULT_CLOSE_RANGE_MOVE_IDS, x)
    }

    /// Creates a fighter with character-provided stats and close move ids.
    pub fn new_with_loadout(
        slot: PlayerSlot,
        name: &'static str,
        max_health: i32,
        move_ids: &'static [MoveId],
        x: f32,
    ) -> Self {
        Self::new_with_projectile_loadout(slot, name, max_health, move_ids, PROJECTILE_SPEC, x)
    }

    /// Creates a fighter with character-provided close moves and projectile data.
    pub fn new_with_projectile_loadout(
        slot: PlayerSlot,
        name: &'static str,
        max_health: i32,
        move_ids: &'static [MoveId],
        projectile_spec: ProjectileSpec,
        x: f32,
    ) -> Self {
        let max_health = max_health.max(1);
        Self {
            slot,
            name,
            position: Vec2::new(x, FLOOR_Y - STANDING_HEIGHT),
            velocity: Vec2::ZERO,
            health: max_health,
            max_health,
            move_ids,
            projectile_spec,
            facing: Facing::Right,
            grounded: true,
            crouching: false,
            blocking: false,
            projectile_cooldown: 0.0,
            special_visual_timer: 0.0,
            hitstun_timer: 0.0,
            blockstun_timer: 0.0,
            whiff_recovery_timer: 0.0,
            attack: None,
        }
    }

    /// Advances movement, stance, defense, and attack timers.
    pub fn update(&mut self, dt: f32, input: FighterInput) -> FighterUpdateEvents {
        let mut events = FighterUpdateEvents::default();
        if self.is_defeated() {
            self.velocity = Vec2::ZERO;
            self.crouching = false;
            self.blocking = false;
            self.hitstun_timer = 0.0;
            self.blockstun_timer = 0.0;
            self.whiff_recovery_timer = 0.0;
            self.special_visual_timer = 0.0;
            return events;
        }

        self.projectile_cooldown = tick_timer(self.projectile_cooldown, dt);
        self.special_visual_timer = tick_timer(self.special_visual_timer, dt);
        self.hitstun_timer = tick_timer(self.hitstun_timer, dt);
        self.blockstun_timer = tick_timer(self.blockstun_timer, dt);
        self.whiff_recovery_timer = tick_timer(self.whiff_recovery_timer, dt);

        let action_locked = self.is_action_locked();
        let requested_move = input.requested_move_spec(self.move_ids, self.facing, self.grounded);
        let wants_attack = requested_move.is_some();
        self.crouching = !action_locked && input.crouch && self.grounded && self.attack.is_none();
        self.blocking = self.in_blockstun()
            || (!action_locked
                && input.block
                && self.grounded
                && self.attack.is_none()
                && !wants_attack);
        self.update_horizontal_velocity(dt, input);

        let can_start_action = !action_locked && !self.blocking && self.attack.is_none();
        if input.jump && self.grounded && can_start_action && !self.crouching && !wants_attack {
            self.velocity.y = JUMP_SPEED;
            self.grounded = false;
            self.apply_diagonal_jump_boost(input);
        }

        if let Some(spec) = requested_move
            && can_start_action
        {
            events.close_attack_started = Some(spec.id);
            self.attack = Some(AttackState {
                spec,
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
            if attack.elapsed_frames() <= attack.spec.frames.duration {
                self.attack = Some(attack);
            } else {
                self.attack = None;
                if !attack.has_hit {
                    events.close_attack_whiffed = Some(attack.spec.id);
                    self.start_whiff_recovery(attack.spec);
                }
            }
        }

        events
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

    /// Applies incoming damage and reaction using the given guard rule.
    pub fn take_hit(
        &mut self,
        damage: i32,
        guard_rule: GuardRule,
        hit_reaction: HitReaction,
    ) -> DamageResult {
        let blocked =
            !self.is_defeated() && guard_rule.is_blocked_by(self.blocking, self.crouching);
        let final_damage = if blocked {
            (damage / BLOCK_DAMAGE_DIVISOR).max(1)
        } else {
            damage
        };
        self.take_damage(final_damage);
        let pushback = self.apply_hit_reaction(hit_reaction, blocked);

        DamageResult {
            damage: final_damage,
            blocked,
            pushback,
        }
    }

    /// Returns true if this fighter can fire a projectile this tick.
    pub fn can_fire_projectile(&self) -> bool {
        !self.is_defeated()
            && self.grounded
            && !self.crouching
            && !self.blocking
            && !self.is_action_locked()
            && self.attack.is_none()
            && self.projectile_cooldown <= 0.0
    }

    /// Starts the projectile cooldown after firing.
    pub fn mark_projectile_fired(&mut self) {
        let frame_data = self.projectile_spec.frame_data;
        self.projectile_cooldown = frame_data.cooldown.as_seconds();
        self.special_visual_timer = frame_data.visual_duration.as_seconds();
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
                head: Rect::new(body.x + 21.0, body.y + 6.0, 34.0, 26.0),
                torso: Rect::new(body.x + 12.0, body.y + 34.0, 52.0, 36.0),
                legs: Rect::new(body.x + 8.0, body.y + 72.0, 60.0, 20.0),
            }
        } else {
            FighterBodyParts {
                head: Rect::new(body.x + 22.0, body.y + 8.0, 32.0, 42.0),
                torso: Rect::new(body.x + 12.0, body.y + 54.0, 52.0, 68.0),
                legs: Rect::new(body.x + 9.0, body.y + 128.0, 58.0, 34.0),
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
                kind: attack.kind(),
                move_id: attack.spec.id,
                hitbox: self.attack_box_for(attack.spec),
                damage: attack.spec.damage,
                guard_rule: attack.spec.guard_rule,
                hit_reaction: attack.spec.hit_reaction,
            })
        })
    }

    /// Returns the attack reach box while an attack animation is running.
    pub fn attack_box(&self) -> Option<Rect> {
        self.attack.map(|attack| self.attack_box_for(attack.spec))
    }

    /// Returns the current close attack kind, if any.
    pub fn attack_kind(&self) -> Option<AttackKind> {
        self.attack.map(AttackState::kind)
    }

    /// Returns the concrete close-range move spec currently being played.
    pub fn attack_move_spec(&self) -> Option<MoveSpec> {
        self.attack.map(|attack| attack.spec)
    }

    /// Returns close-range move ids available to this fighter.
    pub const fn move_ids(&self) -> &'static [MoveId] {
        self.move_ids
    }

    /// Returns projectile tuning available to this fighter.
    pub const fn projectile_spec(&self) -> ProjectileSpec {
        self.projectile_spec
    }

    /// Returns elapsed seconds for the current close attack animation.
    pub fn attack_elapsed_seconds(&self) -> Option<f32> {
        self.attack.map(|attack| attack.elapsed)
    }

    /// Returns elapsed whole frames for the current close attack animation.
    pub fn attack_elapsed_frames(&self) -> Option<FrameCount> {
        self.attack.map(AttackState::elapsed_frames)
    }

    /// Returns whole-frame data for the current close attack.
    pub fn attack_frame_data(&self) -> Option<AttackFrameData> {
        self.attack.map(|attack| attack.spec.frames)
    }

    /// Returns elapsed seconds for the current special animation.
    pub fn special_elapsed_seconds(&self) -> Option<f32> {
        (self.special_visual_timer > 0.0).then_some(
            self.projectile_spec.frame_data.visual_duration.as_seconds()
                - self.special_visual_timer,
        )
    }

    /// Returns elapsed whole frames for the current special animation.
    pub fn special_elapsed_frames(&self) -> Option<FrameCount> {
        self.special_elapsed_seconds()
            .map(FrameCount::from_elapsed_seconds)
    }

    /// Returns whole-frame data for the projectile special.
    pub fn projectile_frame_data(&self) -> ProjectileFrameData {
        self.projectile_spec.frame_data
    }

    /// Returns remaining cooldown for the projectile special in whole frames.
    pub fn projectile_cooldown_remaining_frames(&self) -> FrameCount {
        FrameCount::from_elapsed_seconds(self.projectile_cooldown)
    }

    /// Returns remaining hitstun in whole frames.
    pub fn hitstun_remaining_frames(&self) -> FrameCount {
        FrameCount::from_elapsed_seconds(self.hitstun_timer)
    }

    /// Returns remaining blockstun in whole frames.
    pub fn blockstun_remaining_frames(&self) -> FrameCount {
        FrameCount::from_elapsed_seconds(self.blockstun_timer)
    }

    /// Returns remaining whiff recovery in whole frames.
    pub fn whiff_recovery_remaining_frames(&self) -> FrameCount {
        FrameCount::from_elapsed_seconds(self.whiff_recovery_timer)
    }

    /// Returns whether the fighter is currently locked by hitstun.
    pub fn in_hitstun(&self) -> bool {
        self.hitstun_timer > 0.0
    }

    /// Returns whether the fighter is currently locked by blockstun.
    pub fn in_blockstun(&self) -> bool {
        self.blockstun_timer > 0.0
    }

    /// Returns whether the fighter is locked after missing a close attack.
    pub fn in_whiff_recovery(&self) -> bool {
        self.whiff_recovery_timer > 0.0
    }

    /// Returns the current attack phase for debug rendering.
    pub fn attack_phase(&self) -> AttackPhase {
        if self.in_whiff_recovery() {
            AttackPhase::WhiffRecovery
        } else {
            self.attack.map_or(AttackPhase::Idle, AttackState::phase)
        }
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
        let axis = if self.crouching || self.blocking || self.is_action_locked() {
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

    fn attack_box_for(&self, spec: MoveSpec) -> Rect {
        let body = self.body_rect();
        let spec = spec.hitbox;
        let x = if self.facing == Facing::Right {
            body.right()
        } else {
            body.x - spec.width
        };
        Rect::new(x, body.y + spec.y_offset, spec.width, spec.height)
    }

    fn body_height(&self) -> f32 {
        if self.crouching {
            CROUCH_HEIGHT
        } else {
            STANDING_HEIGHT
        }
    }

    fn apply_hit_reaction(&mut self, hit_reaction: HitReaction, blocked: bool) -> f32 {
        self.velocity.x = 0.0;
        self.whiff_recovery_timer = 0.0;
        if blocked {
            self.blocking = true;
            self.blockstun_timer = hit_reaction.blockstun.as_seconds();
            return hit_reaction.block_pushback;
        }

        self.attack = None;
        self.blocking = false;
        self.hitstun_timer = hit_reaction.hitstun.as_seconds();
        hit_reaction.hit_pushback
    }

    fn is_reacting(&self) -> bool {
        self.in_hitstun() || self.in_blockstun()
    }

    fn is_action_locked(&self) -> bool {
        self.is_reacting() || self.in_whiff_recovery()
    }

    fn start_whiff_recovery(&mut self, spec: MoveSpec) {
        self.velocity.x = 0.0;
        self.whiff_recovery_timer = spec.whiff_recovery.as_seconds();
    }
}

impl AttackState {
    fn kind(self) -> AttackKind {
        AttackKind::from_move_id(self.spec.id)
    }

    fn elapsed_frames(self) -> FrameCount {
        FrameCount::from_elapsed_seconds(self.elapsed)
    }

    fn is_active(self) -> bool {
        let frames = self.spec.frames;
        let current = self.elapsed_frames();
        current >= frames.active_start && current <= frames.active_end
    }

    fn phase(self) -> AttackPhase {
        let frames = self.spec.frames;
        let current = self.elapsed_frames();
        if current < frames.active_start {
            AttackPhase::Startup
        } else if current <= frames.active_end {
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

    fn requested_move_spec(
        self,
        move_ids: &[MoveId],
        facing: Facing,
        grounded: bool,
    ) -> Option<MoveSpec> {
        let input = self.requested_move_kind(facing, grounded)?;
        move_spec_for_input(move_ids, input)
    }

    fn requested_move_kind(self, facing: Facing, grounded: bool) -> Option<MoveInputKind> {
        if !grounded {
            return if self.kick {
                Some(MoveInputKind::AirKick)
            } else if self.light_punch || self.heavy_punch {
                Some(MoveInputKind::AirPunch)
            } else {
                None
            };
        }

        let input = if self.block && self.light_punch {
            MoveInputKind::Throw
        } else if self.crouch && self.kick {
            MoveInputKind::Sweep
        } else if self.crouch && self.heavy_punch {
            MoveInputKind::AntiAir
        } else if self.heavy_punch && self.is_forward(facing) {
            MoveInputKind::Overhead
        } else if self.heavy_punch {
            MoveInputKind::HeavyPunch
        } else if self.kick {
            MoveInputKind::Kick
        } else if self.light_punch {
            MoveInputKind::LightPunch
        } else {
            return None;
        };

        Some(input)
    }

    fn horizontal_axis(self) -> f32 {
        match (self.left, self.right) {
            (true, false) => -1.0,
            (false, true) => 1.0,
            _ => 0.0,
        }
    }

    fn is_forward(self, facing: Facing) -> bool {
        match facing {
            Facing::Right => self.right && !self.left,
            Facing::Left => self.left && !self.right,
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

fn tick_timer(timer: f32, dt: f32) -> f32 {
    let next = timer - dt;
    if next <= TIMER_EPSILON { 0.0 } else { next }
}
