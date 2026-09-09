//! Emits and resolves the physical effects of the five signature attacks.
//!
//! System: Match combat. Fortress, code barrage, floor rupture, vortex and rocket
//! all collide through the same defender shape and guard rules as ordinary hits.

use super::*;
use crate::{
    combat::{
        fighter::{AttackKind, GRAVITY, HitReactionKind},
        signature::{SignatureEffect, SignatureEffectKind},
    },
    config::FLOOR_Y,
};

impl World {
    pub(super) fn spawn_signature_effects(&mut self) {
        for owner in [PlayerSlot::One, PlayerSlot::Two] {
            let actor = match owner {
                PlayerSlot::One => &mut self.player_one,
                PlayerSlot::Two => &mut self.player_two,
            };
            while let Some(move_id) = actor.take_signature_emission() {
                let body = actor.body_rect();
                let direction = if actor.facing == Facing::Right {
                    1.0
                } else {
                    -1.0
                };
                let front = if direction > 0.0 {
                    body.right()
                } else {
                    body.x
                };
                let (kind, position, velocity, lifetime_seconds) = match move_id {
                    MoveId::RustBorrowFortress => (
                        SignatureEffectKind::RustFortress,
                        Vec2::new(front + direction * world_px(65.0), FLOOR_Y),
                        Vec2::ZERO,
                        19.0 / 60.0,
                    ),
                    MoveId::DukePrintlnBarrage => (
                        SignatureEffectKind::DukeCodeSheet,
                        Vec2::new(front + direction * world_px(16.0), body.y + world_px(85.0)),
                        Vec2::new(direction * world_px(520.0), 0.0),
                        460.0 / 520.0,
                    ),
                    MoveId::CSegmentationFault => (
                        SignatureEffectKind::CMemoryRupture,
                        Vec2::new(front + direction * world_px(70.0), FLOOR_Y),
                        Vec2::ZERO,
                        20.0 / 60.0,
                    ),
                    MoveId::PythonImportAntigravity => (
                        SignatureEffectKind::PythonVortex,
                        Vec2::new(body.center_x() + direction * world_px(65.0), FLOOR_Y),
                        Vec2::ZERO,
                        33.0 / 60.0,
                    ),
                    MoveId::CppUndefinedBazooka => (
                        SignatureEffectKind::CppRocket,
                        // Calibrated to the reviewed low bazooka muzzle in
                        // runtime pixels. Its 28-degree shot reaches floor in12f.
                        Vec2::new(body.center_x() + direction * 187.5, FLOOR_Y - 25.8),
                        Vec2::new(direction * world_px(180.0), 130.0),
                        1.0,
                    ),
                    _ => continue,
                };
                self.signature_effects.push(SignatureEffect {
                    owner,
                    move_id,
                    kind,
                    position,
                    velocity,
                    elapsed_seconds: 0.0,
                    lifetime_seconds,
                    direction,
                    alive: true,
                    has_hit: false,
                    fresh: true,
                });
            }
        }
    }

    pub(super) fn update_signature_effects(
        &mut self,
        dt: f32,
        flags: FeatureFlags,
        gain_energy: bool,
    ) {
        // Strikes resolved earlier in World can interrupt a channel. Effects
        // contacting on this same tick must instead trade symmetrically: the
        // first contact cannot erase the second owner's already-active cast.
        let channel_actors = [
            (
                self.player_one.attack_move_spec().map(|spec| spec.id),
                self.player_one.body_rect(),
            ),
            (
                self.player_two.attack_move_spec().map(|spec| spec.id),
                self.player_two.body_rect(),
            ),
        ];
        let mut effects = std::mem::take(&mut self.signature_effects);
        for effect in &mut effects {
            let (channel_move, actor_body) = match effect.owner {
                PlayerSlot::One => channel_actors[0],
                PlayerSlot::Two => channel_actors[1],
            };
            if matches!(
                effect.kind,
                SignatureEffectKind::RustFortress | SignatureEffectKind::PythonVortex
            ) {
                if channel_move != Some(effect.move_id) {
                    effect.alive = false;
                    continue;
                }
                if effect.kind == SignatureEffectKind::RustFortress {
                    effect.position.x = if effect.direction > 0.0 {
                        actor_body.right() + world_px(65.0)
                    } else {
                        actor_body.x - world_px(65.0)
                    };
                }
            }
            effect.update(dt);
            if !effect.alive || effect.has_hit {
                continue;
            }
            let defender_slot = if effect.owner == PlayerSlot::One {
                PlayerSlot::Two
            } else {
                PlayerSlot::One
            };
            let defender_combat = self.sprite_combat_for_slot(defender_slot);
            let defender = match defender_slot {
                PlayerSlot::One => &mut self.player_one,
                PlayerSlot::Two => &mut self.player_two,
            };
            // The physical rocket can be intercepted by a frontal fortress
            // before its floor explosion, just like the paper projectiles.
            if effect.is_projectile()
                && fortress_stops_projectile(defender, effect.velocity.x)
                && (effect.position.x - defender.body_rect().center_x()).abs() < world_px(105.0)
                && effect.position.y >= defender.body_rect().y
            {
                if gain_energy {
                    self.energy
                        .record_contact(effect.owner, defender_slot, true);
                }
                effect.alive = false;
                continue;
            }
            let Some(hitbox) = effect.hitbox() else {
                continue;
            };
            let Some(contact) =
                hitbox_contact_with_defender(hitbox, defender, defender_combat.as_ref())
            else {
                continue;
            };
            let spec = move_spec(effect.move_id);
            // A wide barrier can extend behind a close defender. Its attack
            // still comes from the caster, like an ordinary frontal strike.
            let source_x = if effect.kind == SignatureEffectKind::RustFortress {
                actor_body.center_x()
            } else {
                effect.position.x
            };
            let result = if fortress_faces(defender, source_x) {
                fortress_block_result()
            } else {
                match defender_slot {
                    PlayerSlot::One => take_player_one_hit(
                        defender,
                        spec.damage,
                        spec.guard_rule,
                        spec.hit_reaction,
                        flags,
                    ),
                    PlayerSlot::Two => take_player_two_hit(
                        defender,
                        spec.damage,
                        spec.guard_rule,
                        spec.hit_reaction,
                        flags,
                    ),
                }
            };
            apply_pushback(defender, effect.direction, result.pushback);
            if !result.blocked {
                if matches!(
                    effect.kind,
                    SignatureEffectKind::CMemoryRupture
                        | SignatureEffectKind::PythonVortex
                        | SignatureEffectKind::CppExplosion
                ) {
                    let rise = (defender.position.y - world_px(66.0))
                        .clamp(world_px(55.0), world_px(200.0));
                    defender.begin_launch(
                        Vec2::new(
                            effect.direction * world_px(100.0),
                            -(2.0 * GRAVITY * rise).sqrt(),
                        ),
                        HitReactionKind::Launched,
                    );
                } else {
                    defender.mark_heavy_reaction();
                }
            }
            let facing = defender.facing;
            effect.has_hit = true;
            // Every connected effect gets a complete visual impact sequence.
            // has_hit prevents this additional lifetime from extending damage.
            effect.elapsed_seconds = 0.0;
            effect.lifetime_seconds = 0.25;
            if effect.kind == SignatureEffectKind::DukeCodeSheet {
                effect.velocity = Vec2::ZERO;
            }
            match effect.owner {
                PlayerSlot::One => self.player_one.mark_attack_hit(),
                PlayerSlot::Two => self.player_two.mark_attack_hit(),
            }
            let attack = ActiveAttack {
                kind: AttackKind::SignatureSpecial,
                move_id: effect.move_id,
                hitbox,
                damage: spec.damage,
                guard_rule: spec.guard_rule,
                hit_reaction: spec.hit_reaction,
            };
            self.record_close_contact(effect.owner, defender_slot, attack, result, gain_energy);
            self.hit_effects.push(HitEffect::new(
                contact,
                result.damage,
                result.blocked,
                facing,
            ));
        }
        effects.retain(|effect| effect.alive);
        self.signature_effects = effects;
    }
}

pub(super) fn fortress_faces(defender: &Fighter, source_x: f32) -> bool {
    defender.fortress_guard_active()
        && match defender.facing {
            Facing::Right => source_x >= defender.body_rect().center_x(),
            Facing::Left => source_x <= defender.body_rect().center_x(),
        }
}

pub(super) fn fortress_block_result() -> DamageResult {
    DamageResult {
        damage: 0,
        blocked: true,
        pushback: 0.0,
    }
}

pub(super) fn fortress_stops_projectile(defender: &Fighter, velocity_x: f32) -> bool {
    // At point-blank range the projectile origin can already overlap the body;
    // travel direction, not its center after movement, identifies the front.
    defender.fortress_guard_active()
        && match defender.facing {
            Facing::Right => velocity_x < 0.0,
            Facing::Left => velocity_x > 0.0,
        }
}
