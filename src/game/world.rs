//! Owns the greybox fight simulation.
//!
//! System: Match runtime. This module owns round-level state and consumes
//! character specs, but delegates combat primitives to `combat/*`.
//!
//! This world is intentionally small and deterministic enough to unit test
//! without Raylib.

use crate::audio::AudioEvent;
use crate::characters::{CharacterBodyMetricsCatalog, CharacterId, character_spec};
use crate::combat::collision::hitbox_hits_hurtbox;
use crate::combat::fighter::{
    ActiveAttack, DamageResult, Fighter, FighterInput, FighterUpdateEvents, GuardRule, HitReaction,
    PlayerSlot,
};
use crate::combat::projectile::Projectile;
use crate::config::{ARENA_LEFT, ARENA_RIGHT};
use crate::engine::sprites::{
    FighterSpriteClip, ProjectedSpriteCombat, SpriteManifest, projected_fighter_combat,
    projected_projectile_origin_for_clip,
};
use crate::game::combat_log::{CombatLog, CombatLogEvent, CombatLogKind};
use crate::game::feature_flags::{FeatureFlag, FeatureFlags};
use crate::math::rect::Rect;
use crate::math::vec2::Vec2;

const HIT_EFFECT_LIFETIME: f32 = 0.35;
const BODY_COLLISION_EFFECT_LIFETIME: f32 = 0.12;
pub const SPAWN_INTRO_DURATION_SECONDS: f32 = 2.7;
pub const ROUND_COUNTDOWN_STEP_SECONDS: f32 = 0.75;
pub const ROUND_COUNTDOWN_TOTAL_SECONDS: f32 = ROUND_COUNTDOWN_STEP_SECONDS * 4.0;
pub const MIN_BODY_GAP: f32 = 8.0;

const ROUND_COUNTDOWN_LABELS: [&str; 4] = ["11", "10", "01", "Fight!"];

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

/// Optional sprite manifests used to override combat boxes from frame metadata.
#[derive(Clone, Debug, Default)]
pub struct WorldSpriteCombatManifests {
    pub player_one: Option<SpriteManifest>,
    pub player_two: Option<SpriteManifest>,
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
    audio_events: Vec<AudioEvent>,
    combat_log: CombatLog,
    spawn_intro_timer: f32,
    countdown_timer: f32,
    countdown_audio_step: Option<usize>,
    sprite_combat_manifests: WorldSpriteCombatManifests,
}

impl World {
    /// Creates the initial greybox fight.
    pub fn new_greybox() -> Self {
        Self::new_with_characters(CharacterId::Rust, CharacterId::Duke)
    }

    /// Creates a greybox fight from explicit character specs.
    pub fn new_with_characters(player_one: CharacterId, player_two: CharacterId) -> Self {
        let body_metrics = CharacterBodyMetricsCatalog::default();
        Self::new_with_character_body_metrics(player_one, player_two, &body_metrics)
    }

    /// Creates a fight with explicit character specs and loaded body metrics.
    pub fn new_with_character_body_metrics(
        player_one: CharacterId,
        player_two: CharacterId,
        body_metrics: &CharacterBodyMetricsCatalog,
    ) -> Self {
        let mut world = Self {
            player_one: fighter_from_character(PlayerSlot::One, player_one, body_metrics, 232.0),
            player_two: fighter_from_character(PlayerSlot::Two, player_two, body_metrics, 676.0),
            player_one_character: player_one,
            player_two_character: player_two,
            outcome: None,
            hit_effects: Vec::new(),
            projectiles: Vec::new(),
            body_collision_timer: 0.0,
            elapsed_seconds: 0.0,
            audio_events: Vec::new(),
            combat_log: CombatLog::default(),
            spawn_intro_timer: 0.0,
            countdown_timer: 0.0,
            countdown_audio_step: None,
            sprite_combat_manifests: WorldSpriteCombatManifests::default(),
        };
        world.record_combat(CombatLogKind::RoundStarted {
            player_one,
            player_two,
        });
        world.update_facing();
        world
    }

    /// Creates a greybox fight that starts with the cinematic spawn clips.
    pub fn new_greybox_with_intro() -> Self {
        Self::new_greybox_with_intro_for_characters(CharacterId::Rust, CharacterId::Duke)
    }

    /// Creates an explicit-character fight with cinematic spawn clips.
    pub fn new_greybox_with_intro_for_characters(
        player_one: CharacterId,
        player_two: CharacterId,
    ) -> Self {
        let body_metrics = CharacterBodyMetricsCatalog::default();
        Self::new_greybox_with_intro_for_characters_and_metrics(
            player_one,
            player_two,
            &body_metrics,
        )
    }

    /// Creates an explicit-character fight with cinematic intro and body metrics.
    pub fn new_greybox_with_intro_for_characters_and_metrics(
        player_one: CharacterId,
        player_two: CharacterId,
        body_metrics: &CharacterBodyMetricsCatalog,
    ) -> Self {
        let mut world = Self::new_with_character_body_metrics(player_one, player_two, body_metrics);
        world.spawn_intro_timer = SPAWN_INTRO_DURATION_SECONDS;
        world.countdown_timer = ROUND_COUNTDOWN_TOTAL_SECONDS;
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

    /// Returns whether the pre-fight countdown is blocking gameplay.
    pub fn countdown_active(&self) -> bool {
        !self.spawn_intro_active() && self.countdown_timer > 0.0
    }

    /// Returns the visible pre-fight countdown label.
    pub fn countdown_label(&self) -> Option<&'static str> {
        if self.countdown_active() {
            Some(ROUND_COUNTDOWN_LABELS[self.countdown_step_index()])
        } else {
            None
        }
    }

    /// Returns elapsed time inside the pre-fight countdown.
    pub fn countdown_elapsed_seconds(&self) -> f32 {
        (ROUND_COUNTDOWN_TOTAL_SECONDS - self.countdown_timer).max(0.0)
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

    /// Replaces optional sprite combat metadata used by hit/hurt resolution.
    pub fn set_sprite_combat_manifests(&mut self, manifests: WorldSpriteCombatManifests) {
        self.sprite_combat_manifests = manifests;
    }

    /// Drains audio events generated since the previous call.
    pub fn take_audio_events(&mut self) -> Vec<AudioEvent> {
        std::mem::take(&mut self.audio_events)
    }

    /// Returns diagnostic combat events for the current match.
    pub fn combat_log(&self) -> &[CombatLogEvent] {
        self.combat_log.events()
    }

    /// Clears diagnostic combat events collected so far.
    pub fn clear_combat_log(&mut self) {
        self.combat_log.clear();
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
        if self.countdown_active() {
            self.queue_countdown_audio_event();
            self.countdown_timer = (self.countdown_timer - dt).max(0.0);
            return;
        }

        let player_one_events = self.player_one.update(dt, player_one);
        self.queue_fighter_audio_events(PlayerSlot::One, player_one_events);
        let player_two_events = self.player_two.update(dt, player_two);
        self.queue_fighter_audio_events(PlayerSlot::Two, player_two_events);
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
        let p1_sprite_combat = self.sprite_combat_for_slot(PlayerSlot::One);
        let p2_sprite_combat = self.sprite_combat_for_slot(PlayerSlot::Two);
        let p1_attack = landed_attack(
            &self.player_one,
            &self.player_two,
            p1_sprite_combat.as_ref(),
            p2_sprite_combat.as_ref(),
        );
        let p2_attack = landed_attack(
            &self.player_two,
            &self.player_one,
            p2_sprite_combat.as_ref(),
            p1_sprite_combat.as_ref(),
        );

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
            self.queue_close_hit_audio(PlayerSlot::One, PlayerSlot::Two, attack, result);
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
            self.queue_close_hit_audio(PlayerSlot::Two, PlayerSlot::One, attack, result);
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
                .push(self.projectile_from_fighter(PlayerSlot::One));
            self.player_one.mark_projectile_fired();
            self.audio_events.push(AudioEvent::fighter_projectile_cast(
                PlayerSlot::One,
                self.player_one_character,
            ));
            self.record_combat(CombatLogKind::ProjectileSpawned {
                slot: PlayerSlot::One,
                character: self.player_one_character,
                damage: self.player_one.projectile_spec().damage,
            });
        }

        if player_two.projectile && self.player_two.can_fire_projectile() {
            self.projectiles
                .push(self.projectile_from_fighter(PlayerSlot::Two));
            self.player_two.mark_projectile_fired();
            self.audio_events.push(AudioEvent::fighter_projectile_cast(
                PlayerSlot::Two,
                self.player_two_character,
            ));
            self.record_combat(CombatLogKind::ProjectileSpawned {
                slot: PlayerSlot::Two,
                character: self.player_two_character,
                damage: self.player_two.projectile_spec().damage,
            });
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
        let player_one_character = self.player_one_character;
        let player_two_character = self.player_two_character;
        let player_one_sprite_combat = self.sprite_combat_for_slot(PlayerSlot::One);
        let player_two_sprite_combat = self.sprite_combat_for_slot(PlayerSlot::Two);
        let mut audio_events = Vec::new();
        let mut combat_events = Vec::new();

        for projectile in &mut self.projectiles {
            if !projectile.alive {
                continue;
            }

            let rect = projectile.rect();
            match projectile.owner {
                PlayerSlot::One
                    if projectile_hits_fighter(
                        rect,
                        &self.player_two,
                        player_two_sprite_combat.as_ref(),
                    ) =>
                {
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
                    queue_projectile_hit_audio(
                        &mut audio_events,
                        PlayerSlot::One,
                        player_one_character,
                        PlayerSlot::Two,
                        player_two_character,
                        result,
                    );
                    combat_events.push(CombatLogKind::ProjectileResolved {
                        attacker: PlayerSlot::One,
                        defender: PlayerSlot::Two,
                        attacker_character: player_one_character,
                        defender_character: player_two_character,
                        damage: result.damage,
                        blocked: result.blocked,
                    });
                    self.hit_effects.push(HitEffect::new(
                        self.player_two.hurtbox().center(),
                        result.damage,
                        result.blocked,
                    ));
                }
                PlayerSlot::Two
                    if projectile_hits_fighter(
                        rect,
                        &self.player_one,
                        player_one_sprite_combat.as_ref(),
                    ) =>
                {
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
                    queue_projectile_hit_audio(
                        &mut audio_events,
                        PlayerSlot::Two,
                        player_two_character,
                        PlayerSlot::One,
                        player_one_character,
                        result,
                    );
                    combat_events.push(CombatLogKind::ProjectileResolved {
                        attacker: PlayerSlot::Two,
                        defender: PlayerSlot::One,
                        attacker_character: player_two_character,
                        defender_character: player_one_character,
                        damage: result.damage,
                        blocked: result.blocked,
                    });
                    self.hit_effects.push(HitEffect::new(
                        self.player_one.hurtbox().center(),
                        result.damage,
                        result.blocked,
                    ));
                }
                _ => {}
            }
        }

        self.audio_events.extend(audio_events);
        for event in combat_events {
            self.record_combat(event);
        }
        self.projectiles.retain(|projectile| projectile.alive);
    }

    fn resolve_outcome(&mut self) {
        let previous = self.outcome;
        self.outcome = match (self.player_one.is_defeated(), self.player_two.is_defeated()) {
            (true, true) => Some(MatchOutcome::Draw),
            (true, false) => Some(MatchOutcome::Winner(PlayerSlot::Two)),
            (false, true) => Some(MatchOutcome::Winner(PlayerSlot::One)),
            (false, false) => None,
        };
        if previous.is_none()
            && let Some(outcome) = self.outcome
        {
            match outcome {
                MatchOutcome::Winner(slot) => {
                    let character = self.character_for_slot(slot);
                    self.audio_events
                        .push(AudioEvent::match_victory(slot, character));
                    self.record_combat(CombatLogKind::MatchEnded {
                        winner: Some(slot),
                        winner_character: Some(character),
                    });
                }
                MatchOutcome::Draw => {
                    self.record_combat(CombatLogKind::MatchEnded {
                        winner: None,
                        winner_character: None,
                    });
                }
            }
        }
    }

    fn queue_fighter_audio_events(&mut self, slot: PlayerSlot, events: FighterUpdateEvents) {
        let character = self.character_for_slot(slot);
        if let Some(move_id) = events.close_attack_started {
            self.audio_events
                .push(AudioEvent::fighter_attack_start(slot, character, move_id));
            self.record_combat(CombatLogKind::CloseAttackStarted {
                slot,
                character,
                move_id,
            });
        }
        if let Some(move_id) = events.close_attack_whiffed {
            self.audio_events
                .push(AudioEvent::fighter_attack_whiff(slot, character, move_id));
            self.record_combat(CombatLogKind::CloseAttackWhiffed {
                slot,
                character,
                move_id,
            });
        }
    }

    fn queue_countdown_audio_event(&mut self) {
        let step = self.countdown_step_index();
        if self.countdown_audio_step == Some(step) {
            return;
        }

        self.countdown_audio_step = Some(step);
        self.audio_events.push(match step {
            0 => AudioEvent::match_countdown_eleven(),
            1 => AudioEvent::match_countdown_ten(),
            2 => AudioEvent::match_countdown_one(),
            _ => AudioEvent::match_countdown_fight(),
        });
        self.record_combat(CombatLogKind::CountdownStep {
            label: ROUND_COUNTDOWN_LABELS[step],
        });
    }

    fn queue_close_hit_audio(
        &mut self,
        attacker: PlayerSlot,
        defender: PlayerSlot,
        attack: ActiveAttack,
        result: DamageResult,
    ) {
        let attacker_character = self.character_for_slot(attacker);
        let defender_character = self.character_for_slot(defender);
        if result.blocked {
            self.audio_events.push(AudioEvent::combat_block(
                attacker,
                attacker_character,
                attack.move_id,
            ));
            self.audio_events
                .push(AudioEvent::fighter_block(defender, defender_character));
        } else {
            self.audio_events.push(AudioEvent::combat_hit(
                attacker,
                attacker_character,
                attack.move_id,
            ));
            if result.damage > 0 {
                self.audio_events
                    .push(AudioEvent::fighter_hurt(defender, defender_character));
            }
        }
        self.record_combat(CombatLogKind::CloseAttackResolved {
            attacker,
            defender,
            attacker_character,
            defender_character,
            move_id: attack.move_id,
            damage: result.damage,
            blocked: result.blocked,
        });
    }

    fn countdown_step_index(&self) -> usize {
        let elapsed = self
            .countdown_elapsed_seconds()
            .min(ROUND_COUNTDOWN_TOTAL_SECONDS - f32::EPSILON);
        (elapsed / ROUND_COUNTDOWN_STEP_SECONDS).floor() as usize
    }

    fn record_combat(&mut self, kind: CombatLogKind) {
        self.combat_log.record(self.elapsed_seconds, kind);
    }

    fn sprite_combat_for_slot(&self, slot: PlayerSlot) -> Option<ProjectedSpriteCombat> {
        let manifest = match slot {
            PlayerSlot::One => self.sprite_combat_manifests.player_one.as_ref(),
            PlayerSlot::Two => self.sprite_combat_manifests.player_two.as_ref(),
        }?;
        let fighter = match slot {
            PlayerSlot::One => &self.player_one,
            PlayerSlot::Two => &self.player_two,
        };

        projected_fighter_combat(manifest, fighter, self.elapsed_seconds)
    }

    fn projectile_from_fighter(&self, slot: PlayerSlot) -> Projectile {
        let (fighter, manifest) = match slot {
            PlayerSlot::One => (
                &self.player_one,
                self.sprite_combat_manifests.player_one.as_ref(),
            ),
            PlayerSlot::Two => (
                &self.player_two,
                self.sprite_combat_manifests.player_two.as_ref(),
            ),
        };
        let origin = manifest.and_then(|manifest| {
            projected_projectile_origin_for_clip(manifest, fighter, FighterSpriteClip::Special, 0.0)
        });

        match origin {
            Some(origin) => Projectile::from_fighter_with_origin(fighter, origin),
            None => Projectile::from_fighter(fighter),
        }
    }
}

fn queue_projectile_hit_audio(
    events: &mut Vec<AudioEvent>,
    attacker: PlayerSlot,
    attacker_character: CharacterId,
    defender: PlayerSlot,
    defender_character: CharacterId,
    result: DamageResult,
) {
    events.push(AudioEvent::projectile_impact(attacker, attacker_character));
    if result.blocked {
        events.push(AudioEvent::fighter_block(defender, defender_character));
    } else {
        if result.damage > 0 {
            events.push(AudioEvent::fighter_hurt(defender, defender_character));
        }
    }
}

fn fighter_from_character(
    slot: PlayerSlot,
    character: CharacterId,
    body_metrics: &CharacterBodyMetricsCatalog,
    x: f32,
) -> Fighter {
    let spec = character_spec(character);
    Fighter::new_with_projectile_loadout_and_body_metrics(
        slot,
        spec.fighter_name,
        spec.stats.max_health,
        spec.move_ids,
        spec.projectile,
        body_metrics.body_metrics_for(character),
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

fn landed_attack(
    attacker: &Fighter,
    defender: &Fighter,
    attacker_sprite_combat: Option<&ProjectedSpriteCombat>,
    defender_sprite_combat: Option<&ProjectedSpriteCombat>,
) -> Option<ActiveAttack> {
    let attack = attacker.active_attack()?;
    if !attacker.can_register_hit() {
        return None;
    }

    let sprite_hitboxes = attacker_sprite_combat
        .map(|combat| combat.hitboxes.as_slice())
        .filter(|hitboxes| !hitboxes.is_empty());

    if let Some(hitboxes) = sprite_hitboxes {
        hitboxes
            .iter()
            .copied()
            .find(|hitbox| hitbox_hits_defender(*hitbox, defender, defender_sprite_combat))
            .map(|hitbox| ActiveAttack { hitbox, ..attack })
    } else if hitbox_hits_defender(attack.hitbox, defender, defender_sprite_combat) {
        Some(attack)
    } else {
        None
    }
}

fn hitbox_hits_defender(
    hitbox: Rect,
    defender: &Fighter,
    defender_sprite_combat: Option<&ProjectedSpriteCombat>,
) -> bool {
    let sprite_hurtboxes = defender_sprite_combat
        .map(|combat| combat.hurtboxes.as_slice())
        .filter(|hurtboxes| !hurtboxes.is_empty());

    if let Some(hurtboxes) = sprite_hurtboxes {
        hurtboxes
            .iter()
            .copied()
            .any(|hurtbox| hitbox_hits_hurtbox(hitbox, hurtbox))
    } else {
        defender
            .hurtboxes()
            .rects()
            .into_iter()
            .any(|hurtbox| hitbox_hits_hurtbox(hitbox, hurtbox))
    }
}

fn projectile_hits_fighter(
    projectile: Rect,
    fighter: &Fighter,
    sprite_combat: Option<&ProjectedSpriteCombat>,
) -> bool {
    let sprite_hurtboxes = sprite_combat
        .map(|combat| combat.hurtboxes.as_slice())
        .filter(|hurtboxes| !hurtboxes.is_empty());

    if let Some(hurtboxes) = sprite_hurtboxes {
        hurtboxes
            .iter()
            .copied()
            .any(|hurtbox| projectile.intersects(hurtbox))
    } else {
        fighter
            .hurtboxes()
            .rects()
            .into_iter()
            .any(|hurtbox| projectile.intersects(hurtbox))
    }
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
