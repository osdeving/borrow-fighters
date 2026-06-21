//! Owns the greybox fight simulation.
//!
//! System: Match runtime. This module owns round-level state and consumes
//! character specs, but delegates combat primitives to `combat/*`.
//!
//! This world is intentionally small and deterministic enough to unit test
//! without Raylib.

use crate::characters::{CharacterId, character_spec};
use crate::combat::collision::hitbox_hits_hurtbox;
use crate::combat::fighter::{
    ActiveAttack, DamageResult, Fighter, FighterInput, GuardRule, HitReaction, PlayerSlot,
};
use crate::combat::projectile::Projectile;
use crate::config::{ARENA_LEFT, ARENA_RIGHT};
use crate::game::feature_flags::{FeatureFlag, FeatureFlags};
use crate::math::rect::Rect;
use crate::math::vec2::Vec2;

const HIT_EFFECT_LIFETIME: f32 = 0.35;
const BODY_COLLISION_EFFECT_LIFETIME: f32 = 0.12;
pub const SPAWN_INTRO_DURATION_SECONDS: f32 = 2.7;
pub const MIN_BODY_GAP: f32 = 8.0;

/// Final result of a greybox match.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MatchOutcome {
    Winner(PlayerSlot),
    Draw,
}

/// Transient hit feedback drawn by the greybox renderer.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HitEffect {
    pub position: Vec2,
    pub timer: f32,
    pub damage: i32,
    pub blocked: bool,
}

/// Two-fighter world state for Prototype 0.1.
#[derive(Clone, Debug)]
pub struct World {
    pub player_one: Fighter,
    pub player_two: Fighter,
    player_one_character: CharacterId,
    player_two_character: CharacterId,
    pub outcome: Option<MatchOutcome>,
    pub hit_effects: Vec<HitEffect>,
    pub projectiles: Vec<Projectile>,
    pub body_collision_timer: f32,
    pub elapsed_seconds: f32,
    spawn_intro_timer: f32,
}

impl World {
    /// Creates the initial greybox fight.
    pub fn new_greybox() -> Self {
        Self::new_with_characters(CharacterId::Rust, CharacterId::Duke)
    }

    /// Creates a greybox fight from explicit character specs.
    pub fn new_with_characters(player_one: CharacterId, player_two: CharacterId) -> Self {
        let mut world = Self {
            player_one: fighter_from_character(PlayerSlot::One, player_one, 232.0),
            player_two: fighter_from_character(PlayerSlot::Two, player_two, 676.0),
            player_one_character: player_one,
            player_two_character: player_two,
            outcome: None,
            hit_effects: Vec::new(),
            projectiles: Vec::new(),
            body_collision_timer: 0.0,
            elapsed_seconds: 0.0,
            spawn_intro_timer: 0.0,
        };
        world.update_facing();
        world
    }

    /// Creates a greybox fight that starts with the cinematic spawn clips.
    pub fn new_greybox_with_intro() -> Self {
        let mut world = Self::new_greybox();
        world.spawn_intro_timer = SPAWN_INTRO_DURATION_SECONDS;
        world
    }

    /// Returns whether the non-interactive spawn animation is still active.
    pub fn spawn_intro_active(&self) -> bool {
        self.spawn_intro_timer > 0.0
    }

    /// Returns elapsed time inside the current spawn intro.
    pub fn spawn_intro_elapsed_seconds(&self) -> f32 {
        (SPAWN_INTRO_DURATION_SECONDS - self.spawn_intro_timer).max(0.0)
    }

    /// Returns the character id used by Player 1.
    pub const fn player_one_character(&self) -> CharacterId {
        self.player_one_character
    }

    /// Returns the character id used by Player 2.
    pub const fn player_two_character(&self) -> CharacterId {
        self.player_two_character
    }

    /// Returns the character id assigned to a player slot.
    pub const fn character_for_slot(&self, slot: PlayerSlot) -> CharacterId {
        match slot {
            PlayerSlot::One => self.player_one_character,
            PlayerSlot::Two => self.player_two_character,
        }
    }

    /// Advances one fixed gameplay step.
    pub fn update(&mut self, dt: f32, player_one: FighterInput, player_two: FighterInput) {
        self.update_with_flags(dt, player_one, player_two, FeatureFlags::default());
    }

    /// Advances one fixed gameplay step with runtime feature flags.
    pub fn update_with_flags(
        &mut self,
        dt: f32,
        player_one: FighterInput,
        player_two: FighterInput,
        flags: FeatureFlags,
    ) {
        self.elapsed_seconds += dt;
        self.update_transient_feedback(dt);

        if self.outcome.is_some() {
            return;
        }

        self.update_facing();
        if self.spawn_intro_active() {
            self.spawn_intro_timer = (self.spawn_intro_timer - dt).max(0.0);
            return;
        }

        self.player_one.update(dt, player_one);
        self.player_two.update(dt, player_two);
        self.spawn_projectiles(player_one, player_two);
        self.update_projectiles(dt);
        self.resolve_body_collision();
        self.update_facing();
        self.resolve_hits(flags);
        self.resolve_projectile_hits(flags);
        self.resolve_outcome();
    }

    fn update_transient_feedback(&mut self, dt: f32) {
        for effect in &mut self.hit_effects {
            effect.timer -= dt;
            effect.position.y -= 42.0 * dt;
        }
        self.hit_effects.retain(|effect| effect.timer > 0.0);
        self.body_collision_timer = (self.body_collision_timer - dt).max(0.0);
    }

    fn update_facing(&mut self) {
        let p1 = self.player_one.clone();
        let p2 = self.player_two.clone();
        self.player_one.face_toward(&p2);
        self.player_two.face_toward(&p1);
    }

    fn resolve_body_collision(&mut self) {
        let p1_body = self.player_one.body_rect();
        let p2_body = self.player_two.body_rect();
        let vertical_overlap = p1_body.bottom().min(p2_body.bottom()) - p1_body.y.max(p2_body.y);
        if vertical_overlap <= 0.0 {
            return;
        }

        let player_one_is_left = p1_body.center_x() <= p2_body.center_x();
        let current_gap = if player_one_is_left {
            p2_body.x - p1_body.right()
        } else {
            p1_body.x - p2_body.right()
        };

        if current_gap >= MIN_BODY_GAP {
            return;
        }

        self.body_collision_timer = BODY_COLLISION_EFFECT_LIFETIME;
        if player_one_is_left {
            if self.player_one.velocity.x > 0.0 {
                self.player_one.velocity.x = 0.0;
            }
            if self.player_two.velocity.x < 0.0 {
                self.player_two.velocity.x = 0.0;
            }
            self.place_pair_with_gap(true);
        } else {
            if self.player_one.velocity.x < 0.0 {
                self.player_one.velocity.x = 0.0;
            }
            if self.player_two.velocity.x > 0.0 {
                self.player_two.velocity.x = 0.0;
            }
            self.place_pair_with_gap(false);
        }
    }

    fn place_pair_with_gap(&mut self, player_one_is_left: bool) {
        let p1_width = self.player_one.body_rect().width;
        let p2_width = self.player_two.body_rect().width;
        let total_width = p1_width + MIN_BODY_GAP + p2_width;
        let center =
            (self.player_one.body_rect().center_x() + self.player_two.body_rect().center_x()) * 0.5;
        let left_x = (center - total_width * 0.5).clamp(ARENA_LEFT, ARENA_RIGHT - total_width);
        let right_x = left_x + p1_width + MIN_BODY_GAP;

        if player_one_is_left {
            self.player_one.position.x = left_x;
            self.player_two.position.x = right_x;
        } else {
            self.player_two.position.x = left_x;
            self.player_one.position.x = left_x + p2_width + MIN_BODY_GAP;
        }
    }

    fn resolve_hits(&mut self, flags: FeatureFlags) {
        let p1_attack = landed_attack(&self.player_one, &self.player_two);
        let p2_attack = landed_attack(&self.player_two, &self.player_one);

        if let Some(attack) = p1_attack {
            let pushback_direction = pushback_direction(&self.player_one, &self.player_two);
            let result = take_player_two_hit(
                &mut self.player_two,
                attack.damage,
                attack.guard_rule,
                attack.hit_reaction,
                flags,
            );
            apply_pushback(&mut self.player_two, pushback_direction, result.pushback);
            self.player_one.mark_attack_hit();
            self.hit_effects.push(HitEffect::new(
                self.player_two.hurtbox().center(),
                result.damage,
                result.blocked,
            ));
        }

        if let Some(attack) = p2_attack {
            let pushback_direction = pushback_direction(&self.player_two, &self.player_one);
            let result = take_player_one_hit(
                &mut self.player_one,
                attack.damage,
                attack.guard_rule,
                attack.hit_reaction,
                flags,
            );
            apply_pushback(&mut self.player_one, pushback_direction, result.pushback);
            self.player_two.mark_attack_hit();
            self.hit_effects.push(HitEffect::new(
                self.player_one.hurtbox().center(),
                result.damage,
                result.blocked,
            ));
        }
    }

    fn spawn_projectiles(&mut self, player_one: FighterInput, player_two: FighterInput) {
        if player_one.projectile && self.player_one.can_fire_projectile() {
            self.projectiles
                .push(Projectile::from_fighter(&self.player_one));
            self.player_one.mark_projectile_fired();
        }

        if player_two.projectile && self.player_two.can_fire_projectile() {
            self.projectiles
                .push(Projectile::from_fighter(&self.player_two));
            self.player_two.mark_projectile_fired();
        }
    }

    fn update_projectiles(&mut self, dt: f32) {
        for projectile in &mut self.projectiles {
            projectile.update(dt);
            let rect = projectile.rect();
            if rect.right() < ARENA_LEFT || rect.x > ARENA_RIGHT {
                projectile.alive = false;
            }
        }

        self.projectiles.retain(|projectile| projectile.alive);
    }

    fn resolve_projectile_hits(&mut self, flags: FeatureFlags) {
        for projectile in &mut self.projectiles {
            if !projectile.alive {
                continue;
            }

            let rect = projectile.rect();
            match projectile.owner {
                PlayerSlot::One if projectile_hits_fighter(rect, &self.player_two) => {
                    let pushback_direction = projectile_pushback_direction(projectile);
                    let result = take_player_two_hit(
                        &mut self.player_two,
                        projectile.damage,
                        projectile.guard_rule,
                        projectile.hit_reaction,
                        flags,
                    );
                    apply_pushback(&mut self.player_two, pushback_direction, result.pushback);
                    projectile.alive = false;
                    self.hit_effects.push(HitEffect::new(
                        self.player_two.hurtbox().center(),
                        result.damage,
                        result.blocked,
                    ));
                }
                PlayerSlot::Two if projectile_hits_fighter(rect, &self.player_one) => {
                    let pushback_direction = projectile_pushback_direction(projectile);
                    let result = take_player_one_hit(
                        &mut self.player_one,
                        projectile.damage,
                        projectile.guard_rule,
                        projectile.hit_reaction,
                        flags,
                    );
                    apply_pushback(&mut self.player_one, pushback_direction, result.pushback);
                    projectile.alive = false;
                    self.hit_effects.push(HitEffect::new(
                        self.player_one.hurtbox().center(),
                        result.damage,
                        result.blocked,
                    ));
                }
                _ => {}
            }
        }

        self.projectiles.retain(|projectile| projectile.alive);
    }

    fn resolve_outcome(&mut self) {
        self.outcome = match (self.player_one.is_defeated(), self.player_two.is_defeated()) {
            (true, true) => Some(MatchOutcome::Draw),
            (true, false) => Some(MatchOutcome::Winner(PlayerSlot::Two)),
            (false, true) => Some(MatchOutcome::Winner(PlayerSlot::One)),
            (false, false) => None,
        };
    }
}

fn fighter_from_character(slot: PlayerSlot, character: CharacterId, x: f32) -> Fighter {
    let spec = character_spec(character);
    Fighter::new_with_loadout(
        slot,
        spec.fighter_name,
        spec.stats.max_health,
        spec.move_ids,
        x,
    )
}

impl HitEffect {
    fn new(position: Vec2, damage: i32, blocked: bool) -> Self {
        Self {
            position,
            timer: HIT_EFFECT_LIFETIME,
            damage,
            blocked,
        }
    }
}

fn landed_attack(attacker: &Fighter, defender: &Fighter) -> Option<ActiveAttack> {
    attacker.active_attack().filter(|attack| {
        attacker.can_register_hit()
            && defender
                .hurtboxes()
                .rects()
                .into_iter()
                .any(|hurtbox| hitbox_hits_hurtbox(attack.hitbox, hurtbox))
    })
}

fn projectile_hits_fighter(projectile: Rect, fighter: &Fighter) -> bool {
    fighter
        .hurtboxes()
        .rects()
        .into_iter()
        .any(|hurtbox| projectile.intersects(hurtbox))
}

fn pushback_direction(attacker: &Fighter, defender: &Fighter) -> f32 {
    if attacker.body_rect().center_x() <= defender.body_rect().center_x() {
        1.0
    } else {
        -1.0
    }
}

fn projectile_pushback_direction(projectile: &Projectile) -> f32 {
    if projectile.velocity.x >= 0.0 {
        1.0
    } else {
        -1.0
    }
}

fn apply_pushback(defender: &mut Fighter, direction: f32, amount: f32) {
    if amount <= 0.0 {
        return;
    }

    defender.position.x += direction * amount;
    defender.clamp_to_arena();
}

fn take_player_one_hit(
    player_one: &mut Fighter,
    damage: i32,
    guard_rule: GuardRule,
    hit_reaction: HitReaction,
    flags: FeatureFlags,
) -> DamageResult {
    if flags.enabled(FeatureFlag::PlayerOneTakesDamage) {
        player_one.take_hit(damage, guard_rule, hit_reaction)
    } else {
        DamageResult {
            damage: 0,
            blocked: false,
            pushback: 0.0,
        }
    }
}

fn take_player_two_hit(
    player_two: &mut Fighter,
    damage: i32,
    guard_rule: GuardRule,
    hit_reaction: HitReaction,
    flags: FeatureFlags,
) -> DamageResult {
    if flags.enabled(FeatureFlag::PlayerTwoTakesDamage) {
        player_two.take_hit(damage, guard_rule, hit_reaction)
    } else {
        DamageResult {
            damage: 0,
            blocked: false,
            pushback: 0.0,
        }
    }
}
