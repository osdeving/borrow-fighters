//! Simulates Rust's first adventure encounter with an erratic entity.
//!
//! System: Adventure domain. Movement, contact, enemy intent and defeat belong
//! here; this module has no dependency on arena fighting rules or rendering.

use super::locomotion::Gait;
use crate::math::{rect::Rect, vec2::Vec2};

/// Simulation updates per second, independent from display frame rate.
pub const TICKS_PER_SECOND: u32 = 60;
/// Ground coordinate; actor positions describe the center of their feet.
pub const FLOOR_Y: f32 = 580.0;
/// Width of the first explorable stretch, in world pixels.
pub const LEVEL_WIDTH: f32 = 2200.0;
/// Position at which the erratic entity notices Rust.
pub const ENCOUNTER_TRIGGER_X: f32 = 1100.0;
/// Enemy warning duration before its committed forward attack.
pub const TELEGRAPH_TICKS: u32 = 32;

const DT: f32 = 1.0 / TICKS_PER_SECOND as f32;
/// Default player traversal speed; authored approaches keep their own deliberate pace.
pub const PLAYER_RUN_SPEED: f32 = 455.0;
const GRAVITY: f32 = 1700.0;

/// Actor identity used by animation and contact feedback.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActorKind {
    /// Rust, controlled by the player.
    Player,
    /// The unstable creature attacking Rust.
    Erratic,
}

/// Horizontal orientation, retained throughout a committed attack.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Facing {
    /// Facing decreasing world coordinates.
    Left,
    /// Facing increasing world coordinates.
    Right,
}

impl Facing {
    /// Returns the horizontal multiplier for this direction.
    pub const fn sign(self) -> f32 {
        match self {
            Self::Left => -1.0,
            Self::Right => 1.0,
        }
    }
}

/// Visible action states owned by the adventure simulation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Action {
    /// Standing and able to accept a new command.
    Idle,
    /// Moving along the ground.
    Walk,
    /// Airborne through a player-controlled jump or fall.
    Jump,
    /// Quick melee strike with a short recovery.
    LightAttack,
    /// Slower, stronger melee strike.
    HeavyAttack,
    /// Forward kick with longer reach and committed recovery.
    Kick,
    /// Guarding against attacks arriving from the front.
    Block,
    /// Enemy preparing a visible, committed attack.
    Telegraph,
    /// Enemy lunging in its previously signaled direction.
    Lunge,
    /// Enemy recovering after its lunge.
    Recovery,
    /// Interrupted by a damaging contact.
    Hurt,
    /// Unable to act after losing all health.
    Defeated,
    /// Rust looking down and showing regret after the encounter.
    Remorse,
}

/// Platform-independent commands sampled once for each simulation update.
#[derive(Clone, Copy, Debug, Default)]
pub struct CombatInput {
    /// Horizontal intent in the range -1 to 1.
    pub movement: f32,
    /// Edge-triggered jump request; holding jump does not repeat it.
    pub jump_pressed: bool,
    /// Edge-triggered quick attack request.
    pub light_pressed: bool,
    /// Edge-triggered strong attack request.
    pub heavy_pressed: bool,
    /// Edge-triggered forward kick request.
    pub kick_pressed: bool,
    /// Held guard request, effective when standing and facing the attacker.
    pub blocking: bool,
}

/// A body's authoritative physical state and animation clock.
#[derive(Clone, Debug)]
pub struct Actor {
    /// Identity for presentation and body dimensions.
    pub kind: ActorKind,
    /// Center of the feet in world coordinates.
    pub position: Vec2,
    /// Velocity in pixels per second.
    pub velocity: Vec2,
    /// Remaining health, clamped to zero on defeat.
    pub hp: u32,
    /// Starting health for the health display.
    pub max_hp: u32,
    /// Current horizontal direction.
    pub facing: Facing,
    /// Current visible action.
    pub action: Action,
    /// Number of fixed updates elapsed in the current action.
    pub action_ticks: u32,
    /// Ground travel at reference body scale; authored depth paths normalize each step.
    pub stride_distance: f32,
    /// Explicit authored walk or player-controlled run intent.
    pub gait: Gait,
    /// Whether the feet are touching the floor.
    pub grounded: bool,
    hit_registered: bool,
    invulnerable_ticks: u32,
    stun_ticks: u32,
    tuning: EnemyTuning,
}

impl Actor {
    fn new(kind: ActorKind, x: f32, facing: Facing) -> Self {
        let max_hp = if kind == ActorKind::Player { 100 } else { 96 };
        Self {
            kind,
            position: Vec2::new(x, FLOOR_Y),
            velocity: Vec2::ZERO,
            hp: max_hp,
            max_hp,
            facing,
            action: Action::Idle,
            action_ticks: 0,
            stride_distance: 0.0,
            gait: Gait::Walk,
            grounded: true,
            hit_registered: false,
            invulnerable_ticks: 0,
            stun_ticks: 0,
            tuning: EnemyTuning::default(),
        }
    }

    /// Returns the actor's vulnerable body in world coordinates.
    pub fn hurtbox(&self) -> Rect {
        // These first-pass dimensions leave a small silhouette margin against
        // Rust's ~170 px and the creature's ~187 px adventure presentation.
        // They remain domain-owned measurements to calibrate with the overlay.
        let (width, height) = match self.kind {
            ActorKind::Player => (60.0, 160.0),
            ActorKind::Erratic => (76.0, 174.0),
        };
        Rect::new(
            self.position.x - width * 0.5,
            self.position.y - height,
            width,
            height,
        )
    }

    /// Begins another gait at a grounded contact without reusing the old stride's phase.
    pub fn set_gait(&mut self, gait: Gait) {
        if self.gait != gait {
            self.gait = gait;
            self.stride_distance = 0.0;
        }
    }

    /// Returns the active melee volume; a resolved strike cannot hit again.
    pub fn attack_hitbox(&self) -> Option<Rect> {
        if self.hp == 0 || self.hit_registered {
            return None;
        }
        let (start, end, reach, height, above_feet) = match self.action {
            Action::LightAttack => (5, 10, 66.0, 68.0, 120.0),
            Action::HeavyAttack => (13, 19, 92.0, 78.0, 120.0),
            Action::Kick => (10, 17, 112.0, 84.0, 128.0),
            Action::Lunge => (0, 12, 62.0, 80.0, 125.0),
            _ => return None,
        };
        if !(start..end).contains(&self.action_ticks) {
            return None;
        }
        let edge = self.position.x + self.facing.sign() * (self.hurtbox().width * 0.5 - 8.0);
        let x = if self.facing == Facing::Right {
            edge
        } else {
            edge - reach
        };
        Some(Rect::new(x, self.position.y - above_feet, reach, height))
    }

    /// Damage applied by this action to bodies and breakable scenery alike.
    pub fn strike_damage(&self) -> u32 {
        match self.action {
            Action::LightAttack => 12,
            Action::HeavyAttack => 24,
            Action::Kick => 18,
            Action::Lunge => self.tuning.damage,
            _ => 0,
        }
    }

    /// Consumes the active strike after a scenery collision.
    pub fn register_scenery_hit(&mut self) {
        self.hit_registered = true;
    }

    fn enter(&mut self, action: Action) {
        if self.action != action {
            self.action = action;
            self.action_ticks = 0;
            self.hit_registered = false;
        }
    }

    fn tick_clock(&mut self) {
        self.action_ticks = self.action_ticks.saturating_add(1);
        self.invulnerable_ticks = self.invulnerable_ticks.saturating_sub(1);
    }

    fn locked(&self) -> bool {
        let duration = match self.action {
            Action::LightAttack => 25,
            Action::HeavyAttack => 42,
            Action::Kick => 34,
            Action::Hurt => self.stun_ticks,
            Action::Defeated | Action::Remorse => return true,
            _ => return false,
        };
        self.action_ticks < duration
    }
}

/// Reviewed enemy tuning loaded from scene data for chapter encounters.
#[derive(Clone, Copy, Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EnemyTuning {
    /// Maximum and starting health.
    pub hp: u32,
    /// Approach speed in world pixels per second.
    pub speed: f32,
    /// Explicit warning before a committed attack.
    pub telegraph_ticks: u32,
    /// Vulnerable recovery after a lunge.
    pub recovery_ticks: u32,
    /// Damage of an unguarded lunge.
    pub damage: u32,
}

impl Default for EnemyTuning {
    fn default() -> Self {
        Self {
            hp: 96,
            speed: 155.0,
            telegraph_ticks: TELEGRAPH_TICKS,
            recovery_ticks: 44,
            damage: 14,
        }
    }
}

impl EnemyTuning {
    /// Rejects nonfinite or unreadable encounter parameters before loading.
    pub fn is_valid(self) -> bool {
        (24..=300).contains(&self.hp)
            && self.speed.is_finite()
            && (80.0..=240.0).contains(&self.speed)
            && (24..=90).contains(&self.telegraph_ticks)
            && (30..=90).contains(&self.recovery_ticks)
            && (1..=25).contains(&self.damage)
    }
}

/// Terminal condition of this single encounter.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Outcome {
    /// Exploration or combat is still in progress.
    Ongoing,
    /// The creature lost all health; the story still owes Rust's reaction.
    Victory,
    /// Rust lost all health and can retry at the encounter checkpoint.
    Defeat,
}

/// Latest resolved contact, retained briefly for impact and guard effects.
#[derive(Clone, Copy, Debug)]
pub struct HitFeedback {
    /// Actor receiving the contact.
    pub target: ActorKind,
    /// Contact position in world coordinates.
    pub position: Vec2,
    /// Whether a frontal guard prevented damage.
    pub blocked: bool,
    /// Health removed by this contact.
    pub damage: u32,
    /// Simulation updates since contact.
    pub age_ticks: u32,
}

/// Self-contained exploration and melee encounter state.
#[derive(Clone, Debug)]
pub struct Combat {
    /// Reachable horizontal feet interval supplied by the owning map.
    pub bounds: [f32; 2],
    /// Rust's physical and animation state.
    pub player: Actor,
    /// The erratic entity's physical and animation state.
    pub enemy: Actor,
    /// Additional opponents, empty during the original prologue.
    pub extra_enemies: Vec<Actor>,
    /// Outcome established only through contact and health.
    pub outcome: Outcome,
    /// Whether the creature has noticed Rust and begun attacking.
    pub enemy_awake: bool,
    /// Total simulation updates in this encounter.
    pub ticks: u32,
    /// Latest impact for presentation, cleared after its short effect window.
    pub last_hit: Option<HitFeedback>,
}

impl Default for Combat {
    fn default() -> Self {
        Self::new()
    }
}

impl Combat {
    /// Starts the explorable morning stretch before the creature notices Rust.
    pub fn new() -> Self {
        Self {
            bounds: [40.0, LEVEL_WIDTH - 40.0],
            player: Actor::new(ActorKind::Player, 340.0, Facing::Right),
            enemy: Actor::new(ActorKind::Erratic, 1500.0, Facing::Left),
            extra_enemies: Vec::new(),
            outcome: Outcome::Ongoing,
            enemy_awake: false,
            ticks: 0,
            last_hit: None,
        }
    }

    /// Applies validated scene limits without tying physics to the viewport.
    pub fn set_bounds(&mut self, min: f32, max: f32) {
        assert!(min.is_finite() && max.is_finite() && min < max);
        self.bounds = [min, max];
    }

    /// Replaces the encounter roster while preserving Rust's current state.
    pub fn configure_enemies(&mut self, entries: &[(Vec2, EnemyTuning)]) {
        assert!(!entries.is_empty(), "encounter requires an opponent");
        let mut actors = entries.iter().map(|(position, tuning)| {
            let mut actor = Actor::new(ActorKind::Erratic, position.x, Facing::Left);
            actor.position = *position;
            actor.hp = tuning.hp;
            actor.max_hp = tuning.hp;
            actor.tuning = *tuning;
            actor
        });
        self.enemy = actors.next().expect("nonempty roster");
        self.extra_enemies = actors.collect();
        self.enemy_awake = false;
        self.outcome = Outcome::Ongoing;
    }

    /// Every opponent, including defeated bodies retained for scene continuity.
    pub fn enemies(&self) -> impl Iterator<Item = &Actor> {
        std::iter::once(&self.enemy).chain(self.extra_enemies.iter())
    }

    /// Every mutable opponent, used by chapter geometry and safe restoration.
    pub fn enemies_mut(&mut self) -> impl Iterator<Item = &mut Actor> {
        std::iter::once(&mut self.enemy).chain(self.extra_enemies.iter_mut())
    }

    /// Restores both actors near the encounter without replaying the morning.
    pub fn at_checkpoint() -> Self {
        let mut combat = Self::new();
        combat.player.position.x = ENCOUNTER_TRIGGER_X;
        combat.enemy_awake = true;
        combat
    }

    /// Advances Rust alone while a chapter owns its geometry and progression.
    /// No enemy intent, encounter trigger, contact or victory is evaluated.
    pub fn tick_exploration(&mut self, input: CombatInput) {
        self.tick_clocks();
        self.update_player(input);
        integrate(&mut self.player, self.bounds);
        let bounds = self.bounds;
        for enemy in self.enemies_mut().filter(|enemy| enemy.hp == 0) {
            integrate(enemy, bounds);
        }
    }

    /// Advances movement, enemy intent and contact by one sixtieth of a second.
    pub fn tick(&mut self, input: CombatInput) {
        if self.outcome == Outcome::Defeat {
            // The loss is final, but Rust's fall and defeat animation still play.
            // Neither input nor the enemy can produce further combat contacts.
            self.tick_clocks();
            integrate(&mut self.player, self.bounds);
            return;
        }
        if self.outcome != Outcome::Ongoing {
            return;
        }
        self.tick_clocks();
        self.update_player(input);
        if self.player.position.x >= ENCOUNTER_TRIGGER_X {
            self.enemy_awake = true;
        }
        update_enemy(&mut self.enemy, self.player.position, self.enemy_awake);
        for enemy in &mut self.extra_enemies {
            update_enemy(enemy, self.player.position, self.enemy_awake);
        }
        integrate(&mut self.player, self.bounds);
        integrate(&mut self.enemy, self.bounds);
        separate_bodies(&mut self.player, &mut self.enemy);
        let bounds = self.bounds;
        for enemy in &mut self.extra_enemies {
            integrate(enemy, bounds);
            separate_bodies(&mut self.player, enemy);
        }
        // Keep the two silhouettes distinct while both approach on the same side.
        for enemy in &mut self.extra_enemies {
            separate_bodies(&mut self.enemy, enemy);
        }

        if let Some(hit) = contact(&mut self.player, &mut self.enemy) {
            self.last_hit = Some(hit);
            self.enemy_awake = true;
        }
        // A timely player hit interrupts the lunge before its contact is resolved.
        if let Some(hit) = contact(&mut self.enemy, &mut self.player) {
            self.last_hit = Some(hit);
        }
        for enemy in &mut self.extra_enemies {
            if let Some(hit) = contact(&mut self.player, enemy) {
                self.last_hit = Some(hit);
                self.enemy_awake = true;
            }
            if let Some(hit) = contact(enemy, &mut self.player) {
                self.last_hit = Some(hit);
            }
        }
        self.player.position.x = self.player.position.x.clamp(self.bounds[0], self.bounds[1]);
        let bounds = self.bounds;
        for enemy in self.enemies_mut() {
            enemy.position.x = enemy.position.x.clamp(bounds[0], bounds[1]);
        }
        if self.player.hp == 0 {
            self.outcome = Outcome::Defeat;
        } else if self.enemies().all(|enemy| enemy.hp == 0) {
            self.outcome = Outcome::Victory;
        }
    }

    /// Advances only the approach and compassionate gesture after victory.
    pub fn tick_aftermath(&mut self) {
        if self.outcome != Outcome::Victory {
            return;
        }
        self.tick_clocks();
        let distance = self.enemy.position.x - self.player.position.x;
        if self.player.action != Action::Remorse && distance.abs() > 102.0 {
            self.player.set_gait(Gait::Walk);
            self.player.facing = facing_towards(distance);
            self.player.enter(Action::Walk);
            self.player.velocity.x = distance.signum() * 100.0;
        } else {
            self.player.facing = facing_towards(distance);
            self.player.enter(Action::Remorse);
            self.player.velocity.x = 0.0;
        }
        integrate(&mut self.player, self.bounds);
        integrate(&mut self.enemy, self.bounds);
    }

    fn tick_clocks(&mut self) {
        self.ticks = self.ticks.saturating_add(1);
        self.player.tick_clock();
        self.enemy.tick_clock();
        for enemy in &mut self.extra_enemies {
            enemy.tick_clock();
        }
        if let Some(hit) = &mut self.last_hit {
            hit.age_ticks += 1;
            if hit.age_ticks > 18 {
                self.last_hit = None;
            }
        }
    }

    fn update_player(&mut self, input: CombatInput) {
        if self.player.locked() {
            self.player.velocity.x *= 0.84;
            return;
        }
        let movement = if input.movement.is_finite() {
            input.movement.clamp(-1.0, 1.0)
        } else {
            0.0
        };
        if movement.abs() > 0.05 {
            self.player.facing = facing_towards(movement);
        }
        if input.blocking && self.player.grounded {
            self.player.enter(Action::Block);
            self.player.velocity.x *= 0.6;
            return;
        }
        if input.heavy_pressed || input.light_pressed || input.kick_pressed {
            self.player.enter(if input.kick_pressed {
                Action::Kick
            } else if input.heavy_pressed {
                Action::HeavyAttack
            } else {
                Action::LightAttack
            });
            // A second attack after recovery begins a new swing of the same kind.
            self.player.action_ticks = 0;
            self.player.hit_registered = false;
            self.player.velocity.x *= 0.4;
            return;
        }
        if input.jump_pressed && self.player.grounded {
            self.player.velocity.y = -640.0;
            self.player.grounded = false;
        }
        self.player.set_gait(Gait::Run);
        let desired = movement * PLAYER_RUN_SPEED;
        let braking = movement.abs() <= 0.05 || desired * self.player.velocity.x < 0.0;
        let acceleration = if !self.player.grounded {
            1900.0
        } else if braking {
            3800.0
        } else {
            3300.0
        };
        self.player.velocity.x +=
            (desired - self.player.velocity.x).clamp(-acceleration * DT, acceleration * DT);
        // During a reversal the planted boot still follows actual travel until
        // braking reaches zero; turning on input would drag that support.
        if self.player.velocity.x.abs() > 8.0 {
            self.player.facing = facing_towards(self.player.velocity.x);
        }
        self.player.enter(if !self.player.grounded {
            Action::Jump
        } else if self.player.velocity.x.abs() > 8.0 {
            Action::Walk
        } else {
            Action::Idle
        });
    }
}

fn update_enemy(enemy: &mut Actor, player_position: Vec2, awake: bool) {
    if enemy.hp == 0 || !awake {
        return;
    }
    match enemy.action {
        Action::Hurt if enemy.locked() => {
            enemy.velocity.x *= 0.84;
            return;
        }
        Action::Telegraph if enemy.action_ticks < enemy.tuning.telegraph_ticks => {
            enemy.velocity.x = 0.0;
            return;
        }
        Action::Telegraph => {
            enemy.enter(Action::Lunge);
            enemy.velocity.x = enemy.facing.sign() * 610.0;
            return;
        }
        Action::Lunge if enemy.action_ticks < 12 => return,
        Action::Lunge => {
            enemy.enter(Action::Recovery);
            enemy.velocity.x = 0.0;
            return;
        }
        Action::Recovery if enemy.action_ticks < enemy.tuning.recovery_ticks => return,
        _ => {}
    }
    let distance = player_position.x - enemy.position.x;
    enemy.facing = facing_towards(distance);
    if distance.abs() <= 198.0 {
        enemy.enter(Action::Telegraph);
        enemy.velocity.x = 0.0;
    } else {
        enemy.enter(Action::Walk);
        enemy.velocity.x = distance.signum() * enemy.tuning.speed;
    }
}

fn separate_bodies(player: &mut Actor, enemy: &mut Actor) {
    if player.hp == 0 || enemy.hp == 0 {
        return;
    }
    if !player.hurtbox().intersects(enemy.hurtbox()) {
        return;
    }
    let distance = enemy.position.x - player.position.x;
    let spacing = (player.hurtbox().width + enemy.hurtbox().width) * 0.5;
    let push = (spacing - distance.abs()).max(0.0) * 0.5;
    let direction = if distance >= 0.0 { 1.0 } else { -1.0 };
    player.position.x -= direction * push;
    enemy.position.x += direction * push;
}
fn facing_towards(distance: f32) -> Facing {
    if distance < 0.0 {
        Facing::Left
    } else {
        Facing::Right
    }
}

fn integrate(actor: &mut Actor, bounds: [f32; 2]) {
    let previous_x = actor.position.x;
    actor.position.x = (actor.position.x + actor.velocity.x * DT).clamp(bounds[0], bounds[1]);
    if actor.grounded && actor.action == Action::Walk {
        actor.stride_distance += (actor.position.x - previous_x).abs();
    }
    if !actor.grounded {
        actor.velocity.y += GRAVITY * DT;
        actor.position.y += actor.velocity.y * DT;
    }
    if actor.position.y >= FLOOR_Y {
        actor.position.y = FLOOR_Y;
        actor.velocity.y = 0.0;
        actor.grounded = true;
    }
    if actor.action == Action::Defeated {
        actor.velocity.x *= 0.86;
    }
}

fn contact(attacker: &mut Actor, target: &mut Actor) -> Option<HitFeedback> {
    let hitbox = attacker.attack_hitbox()?;
    if target.hp == 0 || target.invulnerable_ticks > 0 || !hitbox.intersects(target.hurtbox()) {
        return None;
    }
    attacker.hit_registered = true;
    let (knockback, stun) = match attacker.action {
        Action::LightAttack => (185.0, 15),
        Action::HeavyAttack => (275.0, 26),
        Action::Kick => (300.0, 22),
        Action::Lunge => (245.0, 22),
        _ => return None,
    };
    let damage = attacker.strike_damage();
    let from_front = (attacker.position.x - target.position.x) * target.facing.sign() > 0.0;
    let blocked = target.action == Action::Block && target.grounded && from_front;
    target.velocity.x = attacker.facing.sign() * knockback * if blocked { 0.22 } else { 1.0 };
    if !blocked {
        target.hp = target.hp.saturating_sub(damage);
        target.stun_ticks = stun;
        target.invulnerable_ticks = if target.kind == ActorKind::Player {
            24
        } else {
            8
        };
        target.enter(if target.hp == 0 {
            Action::Defeated
        } else {
            Action::Hurt
        });
        target.action_ticks = 0;
        target.velocity.y = -85.0;
        target.grounded = false;
    }
    Some(HitFeedback {
        target: target.kind,
        position: Vec2::new(
            (hitbox.center_x() + target.position.x) * 0.5,
            target.position.y - 72.0,
        ),
        blocked,
        damage: if blocked { 0 } else { damage },
        age_ticks: 0,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_run_accelerates_brakes_and_reverses_without_dragging_a_backwards_boot() {
        let mut combat = Combat::new();
        combat.player.position.x = 500.0;
        let right = CombatInput {
            movement: 1.0,
            ..Default::default()
        };
        for _ in 0..9 {
            combat.tick_exploration(right);
        }
        assert_eq!(combat.player.gait, Gait::Run);
        assert_eq!(combat.player.velocity.x, PLAYER_RUN_SPEED);
        let brake_at = combat.player.position.x;
        for _ in 0..8 {
            combat.tick_exploration(CombatInput::default());
        }
        assert_eq!(combat.player.action, Action::Idle);
        assert_eq!(combat.player.velocity.x, 0.0);
        assert!(combat.player.position.x - brake_at < 30.0);
        for _ in 0..9 {
            combat.tick_exploration(right);
        }
        for _ in 0..16 {
            combat.tick_exploration(CombatInput {
                movement: -1.0,
                ..Default::default()
            });
            if combat.player.velocity.x.abs() > 8.0 {
                assert_eq!(
                    combat.player.facing.sign(),
                    combat.player.velocity.x.signum()
                );
            }
        }
        assert_eq!(combat.player.velocity.x, -PLAYER_RUN_SPEED);
    }

    #[test]
    fn changing_gait_starts_on_a_contact_but_repeated_intent_preserves_distance() {
        let mut player = Actor::new(ActorKind::Player, 500.0, Facing::Right);
        player.stride_distance = 52.0;
        player.set_gait(Gait::Run);
        assert_eq!(player.stride_distance, 0.0);
        player.stride_distance = 93.0;
        player.set_gait(Gait::Run);
        assert_eq!(player.stride_distance, 93.0);
        player.set_gait(Gait::Walk);
        assert_eq!(player.stride_distance, 0.0);
    }

    fn ready_strike(facing: Facing, target_x: f32) -> (Actor, Actor) {
        let mut player = Actor::new(ActorKind::Player, 500.0, facing);
        player.enter(Action::LightAttack);
        player.action_ticks = 5;
        let enemy = Actor::new(ActorKind::Erratic, target_x, Facing::Left);
        (player, enemy)
    }

    #[test]
    fn melee_requires_active_frames_front_and_actual_overlap() {
        for (facing, x, expected) in [
            (Facing::Right, 580.0, true),
            (Facing::Right, 700.0, false),
            (Facing::Right, 420.0, false),
            (Facing::Left, 420.0, true),
        ] {
            let (mut attacker, mut target) = ready_strike(facing, x);
            assert_eq!(contact(&mut attacker, &mut target).is_some(), expected);
        }
        let (mut attacker, mut target) = ready_strike(Facing::Right, 580.0);
        attacker.action_ticks = 4;
        assert!(contact(&mut attacker, &mut target).is_none());
        attacker.action_ticks = 10;
        assert!(contact(&mut attacker, &mut target).is_none());
        attacker.action_ticks = 5;
        target.position.y = FLOOR_Y - 180.0;
        assert!(contact(&mut attacker, &mut target).is_none());
    }

    #[test]
    fn one_swing_can_damage_a_target_only_once() {
        let (mut attacker, mut target) = ready_strike(Facing::Right, 580.0);
        assert_eq!(contact(&mut attacker, &mut target).unwrap().damage, 12);
        target.invulnerable_ticks = 0;
        assert!(contact(&mut attacker, &mut target).is_none());
        assert_eq!(target.hp, target.max_hp - 12);
    }

    #[test]
    fn guard_stops_front_contact_but_not_a_hit_from_behind() {
        for (facing, expected_damage) in [(Facing::Left, 0), (Facing::Right, 12)] {
            let (mut attacker, mut target) = ready_strike(Facing::Right, 580.0);
            target.facing = facing;
            target.enter(Action::Block);
            let hit = contact(&mut attacker, &mut target).unwrap();
            assert_eq!(hit.damage, expected_damage);
            assert_eq!(target.hp, target.max_hp - expected_damage);
            assert_eq!(hit.blocked, expected_damage == 0);
        }
    }

    #[test]
    fn exploration_does_not_run_enemy_ai_before_the_trigger() {
        let mut combat = Combat::new();
        for _ in 0..600 {
            combat.tick(CombatInput::default());
        }
        assert!(!combat.enemy_awake);
        assert_eq!(combat.enemy.position.x, 1500.0);
        combat.player.position.x = ENCOUNTER_TRIGGER_X;
        combat.tick(CombatInput::default());
        assert!(combat.enemy_awake);
        assert_eq!(combat.enemy.action, Action::Walk);
    }

    #[test]
    fn lunge_is_telegraphed_and_does_not_retarget_mid_attack() {
        let mut combat = Combat::at_checkpoint();
        combat.player.position.x = combat.enemy.position.x - 180.0;
        combat.tick(CombatInput::default());
        assert_eq!(combat.enemy.action, Action::Telegraph);
        for _ in 1..TELEGRAPH_TICKS {
            combat.tick(CombatInput::default());
            assert_eq!(combat.player.hp, combat.player.max_hp);
            assert_eq!(combat.enemy.action, Action::Telegraph);
        }
        combat.player.position.x = combat.enemy.position.x + 180.0;
        combat.tick(CombatInput::default());
        assert_eq!(combat.enemy.action, Action::Lunge);
        assert_eq!(combat.enemy.facing, Facing::Left);
        assert!(combat.enemy.velocity.x < 0.0);
    }

    #[test]
    fn defeat_cancels_the_creatures_attack_and_future_damage() {
        let mut combat = Combat::at_checkpoint();
        combat.player.position.x = 1400.0;
        combat.enemy.position.x = 1480.0;
        combat.enemy.hp = 12;
        combat.enemy.enter(Action::Lunge);
        combat.player.enter(Action::LightAttack);
        combat.player.action_ticks = 4;
        combat.tick(CombatInput::default());
        assert_eq!(combat.outcome, Outcome::Victory);
        assert_eq!(combat.enemy.action, Action::Defeated);
        let hp = combat.player.hp;
        for _ in 0..180 {
            combat.tick(CombatInput::default());
            combat.tick_aftermath();
        }
        assert_eq!(combat.player.hp, hp);
        assert!(combat.enemy.attack_hitbox().is_none());
        assert_eq!(combat.player.action, Action::Remorse);
    }

    #[test]
    fn jump_is_ballistic_and_lands_without_a_second_midair_jump() {
        let mut combat = Combat::new();
        combat.tick(CombatInput {
            jump_pressed: true,
            ..CombatInput::default()
        });
        assert!(!combat.player.grounded);
        let velocity = combat.player.velocity.y;
        combat.tick(CombatInput {
            jump_pressed: true,
            ..CombatInput::default()
        });
        assert!(combat.player.velocity.y > velocity);
        for _ in 0..90 {
            combat.tick(CombatInput::default());
        }
        assert!(combat.player.grounded);
        assert_eq!(combat.player.position.y, FLOOR_Y);
    }

    #[test]
    fn lost_encounter_finishes_rusts_fall_without_running_enemy_ai_or_contacts() {
        let mut combat = Combat::at_checkpoint();
        combat.player.position.x = 1420.0;
        combat.enemy.position.x = 1500.0;
        combat.player.hp = 14;
        combat.enemy.enter(Action::Lunge);
        combat.tick(CombatInput::default());
        assert_eq!(combat.outcome, Outcome::Defeat);
        assert_eq!(combat.player.action, Action::Defeated);
        assert!(!combat.player.grounded);
        let enemy_position = combat.enemy.position;
        let enemy_health = combat.enemy.hp;
        let player_start_x = combat.player.position.x;
        for _ in 0..90 {
            combat.tick(CombatInput {
                movement: 1.0,
                jump_pressed: true,
                heavy_pressed: true,
                ..CombatInput::default()
            });
        }
        assert_eq!(combat.player.hp, 0);
        assert_eq!(combat.player.action, Action::Defeated);
        assert_eq!(combat.player.action_ticks, 90);
        assert_eq!(combat.player.position.y, FLOOR_Y);
        assert!(combat.player.grounded);
        assert!(combat.player.position.x < player_start_x);
        assert_eq!(combat.enemy.position, enemy_position);
        assert_eq!(combat.enemy.hp, enemy_health);
        assert_eq!(combat.outcome, Outcome::Defeat);
        assert!(combat.last_hit.is_none());
    }

    #[test]
    fn kick_uses_its_authored_active_window_and_outreaches_the_light_punch() {
        let mut player = Actor::new(ActorKind::Player, 500.0, Facing::Right);
        let mut enemy = Actor::new(ActorKind::Erratic, 648.0, Facing::Left);
        player.enter(Action::LightAttack);
        player.action_ticks = 5;
        assert!(contact(&mut player, &mut enemy).is_none());
        player.enter(Action::Kick);
        player.action_ticks = 9;
        assert!(contact(&mut player, &mut enemy).is_none());
        player.action_ticks = 10;
        assert_eq!(contact(&mut player, &mut enemy).unwrap().damage, 18);
        enemy.invulnerable_ticks = 0;
        assert!(contact(&mut player, &mut enemy).is_none());
        player.hit_registered = false;
        player.action_ticks = 17;
        assert!(player.attack_hitbox().is_none());
    }

    #[test]
    fn two_enemy_victory_waits_for_the_remaining_body_and_fallen_enemies_do_not_block() {
        let mut combat = Combat::new();
        combat.configure_enemies(&[
            (Vec2::new(580.0, FLOOR_Y), EnemyTuning::default()),
            (Vec2::new(660.0, FLOOR_Y), EnemyTuning::default()),
        ]);
        combat.player.position.x = 580.0;
        combat.enemy.hp = 0;
        combat.enemy.action = Action::Defeated;
        combat.extra_enemies[0].hp = 18;
        combat.tick(CombatInput::default());
        assert_eq!(combat.outcome, Outcome::Ongoing);
        assert_eq!(
            combat.player.position.x, 580.0,
            "the fallen primary opponent is no longer solid"
        );
        combat.player.enter(Action::Kick);
        combat.player.action_ticks = 9;
        combat.tick(CombatInput::default());
        assert_eq!(combat.extra_enemies[0].hp, 0);
        assert_eq!(combat.outcome, Outcome::Victory);
    }
}
