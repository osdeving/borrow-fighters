//! Verifies impact feedback against the offensive shape that actually made contact.
//!
//! System: Combat visual coherence. Real showcase inputs cover low, aerial and
//! projectile contact with both baseline metadata and fallback geometry.

use borrow_fighters::{
    characters::CharacterId,
    config::FIXED_TIMESTEP,
    engine::sprites::{SpriteManifest, projected_fighter_combat},
    game::world::WorldSpriteCombatManifests,
    math::rect::Rect,
    scenes::{
        combat_lab::{CombatLabInput, CombatLabMove},
        move_showcase::{MoveShowcase, MoveShowcaseOptions},
    },
};

#[test]
fn sweep_anti_air_and_projectile_sparks_follow_actual_contact_on_both_sides() {
    for character in [
        CharacterId::Rust,
        CharacterId::Duke,
        CharacterId::C,
        CharacterId::Python,
        CharacterId::Cpp,
    ] {
        for selected in [
            CombatLabMove::Sweep,
            CombatLabMove::AntiAir,
            CombatLabMove::Projectile,
        ] {
            for metadata in [false, true] {
                for reversed in [false, true] {
                    let mut scene = MoveShowcase::new(MoveShowcaseOptions { character });
                    let attacker_manifest = SpriteManifest::load(format!(
                        "assets/placeholder/{}-fighter.sprite.json",
                        character.audio_key()
                    ))
                    .unwrap();
                    if metadata {
                        scene.set_sprite_combat_manifests(WorldSpriteCombatManifests {
                            player_one: Some(attacker_manifest.clone()),
                            player_two: Some(
                                SpriteManifest::load(format!(
                                    "assets/placeholder/{}-fighter.sprite.json",
                                    scene.opponent_character().audio_key()
                                ))
                                .unwrap(),
                            ),
                        });
                    }
                    scene.select_move(selected);
                    if reversed {
                        scene.switch_sides();
                    }
                    let mut contact_seen = false;
                    for _ in 0..190 {
                        let projectiles_before = scene.world().projectiles.clone();
                        scene.update(CombatLabInput::default());
                        let Some(effect) = scene.world().hit_effects.first() else {
                            continue;
                        };
                        let fighter = &scene.world().player_one;
                        let mut shapes = Vec::new();
                        if selected == CombatLabMove::Projectile {
                            for mut projectile in projectiles_before {
                                projectile.update(FIXED_TIMESTEP);
                                shapes.push(projectile.rect());
                            }
                        } else {
                            if metadata
                                && !fighter.uses_move_spec_hitbox()
                                && let Some(projected) = projected_fighter_combat(
                                    &attacker_manifest,
                                    fighter,
                                    scene.world().elapsed_seconds,
                                )
                            {
                                shapes.extend(projected.hitboxes);
                            }
                            if shapes.is_empty() {
                                shapes.extend(fighter.active_hitbox());
                            }
                        }
                        assert!(
                            shapes.iter().any(|shape| contains(
                                *shape,
                                effect.position.x,
                                effect.position.y
                            )),
                            "{character:?}/{selected:?} metadata={metadata} reversed={reversed}: {:?} outside {:?}",
                            effect.position,
                            shapes
                        );
                        if selected == CombatLabMove::Sweep {
                            assert!(
                                effect.position.y > borrow_fighters::config::FLOOR_Y - 44.0,
                                "sweep spark must stay at ankle/shin height"
                            );
                        }
                        contact_seen = true;
                        break;
                    }
                    assert!(contact_seen, "{character:?}/{selected:?} must connect");
                }
            }
        }
    }
}

fn contains(rect: Rect, x: f32, y: f32) -> bool {
    x >= rect.x && x <= rect.right() && y >= rect.y && y <= rect.bottom()
}
