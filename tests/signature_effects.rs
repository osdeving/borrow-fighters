//! Exercises thematic special entities, projectile travel and defensive responses.
//!
//! System: Match integration. Assertions inspect damage, geometry and control
//! instead of accepting a special clip as proof that an attack exists.

use borrow_fighters::{
    characters::CharacterId,
    combat::{
        fighter::{FighterInput, HitReactionKind},
        move_data::{MoveInputKind, move_spec_for_input},
        signature::SignatureEffectKind,
    },
    config::{FIXED_TIMESTEP as DT, FLOOR_Y, world_px},
    game::{ai::BasicCpu, world::World},
};

fn near(character: CharacterId, reverse: bool) -> World {
    let mut world = World::new_with_characters(character, CharacterId::Rust);
    let left = world_px(330.0);
    let right = left + world.player_one.body_rect().width + world_px(10.0);
    world.player_one.position.x = if reverse { right } else { left };
    world.player_two.position.x = if reverse { left } else { right };
    world
}

#[test]
fn simultaneous_vortices_trade_and_fortresses_guard_without_slot_priority() {
    use borrow_fighters::{
        engine::sprites::SpriteManifest,
        game::{combat_log::CombatLogKind, world::WorldSpriteCombatManifests},
    };
    for character in [CharacterId::Python, CharacterId::Rust] {
        for metadata in [false, true] {
            for reverse in [false, true] {
                let mut world = World::new_with_characters(character, character);
                let left = world_px(360.0);
                let right = left + world.player_one.body_rect().width + world_px(12.0);
                world.player_one.position.x = if reverse { right } else { left };
                world.player_two.position.x = if reverse { left } else { right };
                if metadata {
                    let manifest = SpriteManifest::load(format!(
                        "assets/placeholder/{}-fighter.sprite.json",
                        character.audio_key()
                    ))
                    .unwrap();
                    world.set_sprite_combat_manifests(WorldSpriteCombatManifests {
                        player_one: Some(manifest.clone()),
                        player_two: Some(manifest),
                    });
                }
                let mut both_launched = false;
                for tick in 0..150 {
                    let input = FighterInput {
                        signature_special: tick == 0,
                        ..FighterInput::default()
                    };
                    world.update(DT, input, input);
                    both_launched |=
                        world.player_one.in_air_reaction() && world.player_two.in_air_reaction();
                }
                let expected = if character == CharacterId::Python {
                    20
                } else {
                    0
                };
                assert_eq!(
                    world.player_one.max_health - world.player_one.health,
                    expected,
                    "{character:?} metadata={metadata} reverse={reverse}: P1 damage"
                );
                assert_eq!(
                    world.player_two.max_health - world.player_two.health,
                    expected,
                    "{character:?} metadata={metadata} reverse={reverse}: P2 damage"
                );
                assert_eq!(both_launched, character == CharacterId::Python);
                assert!(world.player_one.grounded && world.player_two.grounded);
                assert!(!world.player_one.in_hitstun() && !world.player_two.in_hitstun());
                let contacts = world.combat_log().iter().filter(|event| matches!(event.kind,
                    CombatLogKind::CloseAttackResolved { blocked, .. } if blocked == (character == CharacterId::Rust))).count();
                assert_eq!(
                    contacts, 2,
                    "both committed effects must resolve exactly once"
                );
            }
        }
    }
}

#[test]
fn normal_strike_interrupts_vortex_before_same_tick_effect_resolution() {
    let mut world = near(CharacterId::Python, false);
    let mut effect_seen = false;
    for tick in 0..100 {
        world.update(
            DT,
            FighterInput {
                signature_special: tick == 0,
                ..FighterInput::default()
            },
            FighterInput {
                light_punch: tick == 22,
                ..FighterInput::default()
            },
        );
        effect_seen |= !world.signature_effects.is_empty();
    }
    assert!(world.player_one.health < world.player_one.max_health);
    assert_eq!(world.player_two.health, world.player_two.max_health);
    assert!(
        !effect_seen,
        "the jab resolves before channels are snapshotted"
    );
}

#[test]
fn java_emits_three_travelling_sheets_and_retains_non_damaging_impact_frames() {
    for reverse in [false, true] {
        let mut world = near(CharacterId::Duke, reverse);
        let mut contacts = 0;
        let mut previous_health = world.player_two.health;
        let mut impact_seen = false;
        for tick in 0..150 {
            world.update(
                DT,
                FighterInput {
                    signature_special: tick == 0,
                    ..FighterInput::default()
                },
                FighterInput::default(),
            );
            if world.player_two.health < previous_health {
                contacts += 1;
            }
            previous_health = world.player_two.health;
            impact_seen |= world.signature_effects.iter().any(|effect| {
                effect.kind == SignatureEffectKind::DukeCodeSheet
                    && effect.has_connected()
                    && effect.velocity.x == 0.0
            });
        }
        assert_eq!(contacts, 3);
        assert_eq!(world.player_two.max_health - world.player_two.health, 24);
        assert!(impact_seen);
        assert!(world.signature_effects.is_empty());
    }
}

#[test]
fn lethal_paper_impact_finishes_without_freezing_or_continuing_the_round() {
    let mut world = near(CharacterId::Duke, false);
    world.player_two.health = 1;
    for tick in 0..100 {
        world.update(
            DT,
            FighterInput {
                signature_special: tick == 0,
                ..FighterInput::default()
            },
            FighterInput::default(),
        );
        if world.outcome.is_some() {
            break;
        }
    }
    let outcome = world.outcome.expect("the first sheet must end the round");
    assert!(world.signature_effects.iter().any(|fx| fx.has_connected()));
    let positions = (world.player_one.position, world.player_two.position);
    let health = (world.player_one.health, world.player_two.health);
    for _ in 0..90 {
        world.update(
            DT,
            FighterInput {
                right: true,
                signature_special: true,
                ..FighterInput::default()
            },
            FighterInput {
                left: true,
                light_punch: true,
                ..FighterInput::default()
            },
        );
    }
    assert!(world.signature_effects.is_empty());
    assert_eq!(world.outcome, Some(outcome));
    assert_eq!(
        (world.player_one.position, world.player_two.position),
        positions
    );
    assert_eq!((world.player_one.health, world.player_two.health), health);
}

#[test]
fn every_signature_restarts_its_impact_clock_without_granting_extra_hits() {
    for character in [
        CharacterId::Rust,
        CharacterId::Duke,
        CharacterId::C,
        CharacterId::Python,
        CharacterId::Cpp,
    ] {
        let mut world = near(character, false);
        let mut first_impact = false;
        for tick in 0..120 {
            world.update(
                DT,
                FighterInput {
                    signature_special: tick == 0,
                    ..FighterInput::default()
                },
                FighterInput::default(),
            );
            if let Some(effect) = world.signature_effects.iter().find(|fx| fx.has_connected()) {
                assert_eq!(effect.elapsed_seconds, 0.0, "{character:?}");
                assert_eq!(effect.lifetime_seconds, 0.25, "{character:?}");
                first_impact = true;
                break;
            }
        }
        assert!(first_impact, "{character:?} should connect");
        let damage = world.player_two.max_health - world.player_two.health;
        // Java's later sheets are deliberately separate contacts. Other
        // effects remain visible but must never strike twice after the reset.
        for _ in 0..120 {
            world.update(DT, FighterInput::default(), FighterInput::default());
        }
        assert_eq!(
            world.player_two.max_health - world.player_two.health,
            if character == CharacterId::Duke {
                24
            } else {
                damage
            }
        );
    }
}

#[test]
fn bazooka_rocket_travels_down_before_its_ground_explosion_launches_the_target() {
    for reverse in [false, true] {
        let mut world = near(CharacterId::Cpp, reverse);
        let mut rocket_start = None;
        let mut rocket_last = None;
        let mut explosion_seen = false;
        let mut launch_seen = false;
        for tick in 0..150 {
            world.update(
                DT,
                FighterInput {
                    signature_special: tick == 0,
                    ..FighterInput::default()
                },
                FighterInput::default(),
            );
            for effect in &world.signature_effects {
                if effect.kind == SignatureEffectKind::CppRocket {
                    rocket_start.get_or_insert(effect.position);
                    rocket_last = Some(effect.position);
                    assert!(effect.hitbox().is_none());
                    assert_eq!(world.player_two.health, world.player_two.max_health);
                }
                if effect.kind == SignatureEffectKind::CppExplosion {
                    explosion_seen = true;
                    assert_eq!(effect.position.y, FLOOR_Y);
                    assert_eq!(effect.hitbox().unwrap().bottom(), FLOOR_Y);
                }
            }
            launch_seen |= world.player_two.hit_reaction_kind() == HitReactionKind::Launched
                && world.player_two.in_air_reaction();
        }
        let start = rocket_start.unwrap();
        let last = rocket_last.unwrap();
        assert!(last.y > start.y + world_px(10.0));
        assert!((last.x - start.x).abs() > world_px(20.0));
        assert!(explosion_seen && launch_seen);
        assert_eq!(world.player_two.max_health - world.player_two.health, 24);
    }
}

#[test]
fn all_signatures_can_be_used_again_after_recovery_without_a_resource() {
    for character in [
        CharacterId::Rust,
        CharacterId::Duke,
        CharacterId::C,
        CharacterId::Python,
        CharacterId::Cpp,
    ] {
        let mut world = near(character, false);
        let expected =
            move_spec_for_input(world.player_one.move_ids(), MoveInputKind::SignatureSpecial)
                .unwrap()
                .id;
        for _ in 0..2 {
            world.update(
                DT,
                FighterInput {
                    signature_special: true,
                    ..FighterInput::default()
                },
                FighterInput {
                    block: true,
                    crouch: matches!(character, CharacterId::C | CharacterId::Cpp),
                    ..FighterInput::default()
                },
            );
            assert_eq!(world.player_one.attack_move_spec().unwrap().id, expected);
            for _ in 0..150 {
                world.update(
                    DT,
                    FighterInput::default(),
                    FighterInput {
                        block: true,
                        crouch: matches!(character, CharacterId::C | CharacterId::Cpp),
                        ..FighterInput::default()
                    },
                );
            }
            assert!(world.player_one.attack_kind().is_none());
        }
    }
}

#[test]
fn fortress_stops_a_frontal_jab_but_its_recovery_is_vulnerable() {
    for reverse in [false, true] {
        let mut world = near(CharacterId::Rust, reverse);
        for tick in 0..35 {
            // Rust's jab becomes active on its fourth tick, matching fortress24.
            world.update(
                DT,
                FighterInput {
                    signature_special: tick == 0,
                    ..FighterInput::default()
                },
                FighterInput {
                    light_punch: tick == 20,
                    ..FighterInput::default()
                },
            );
        }
        assert_eq!(world.player_one.health, world.player_one.max_health);
        assert!(world.hit_effects.iter().any(|hit| hit.blocked));
        for _ in 0..24 {
            world.update(DT, FighterInput::default(), FighterInput::default());
        }
        // Walk through the pushback before punishing the still-committed caster.
        for _ in 0..12 {
            world.update(
                DT,
                FighterInput::default(),
                FighterInput {
                    right: reverse,
                    left: !reverse,
                    ..FighterInput::default()
                },
            );
        }
        world.update(
            DT,
            FighterInput::default(),
            FighterInput {
                light_punch: true,
                ..FighterInput::default()
            },
        );
        for _ in 0..6 {
            world.update(DT, FighterInput::default(), FighterInput::default());
        }
        assert!(world.player_one.health < world.player_one.max_health);
    }
}

#[test]
fn cpu_sees_detached_paper_and_rocket_threats_with_the_correct_guard_height() {
    for (character, emission_frame, low) in
        [(CharacterId::Duke, 26, false), (CharacterId::Cpp, 32, true)]
    {
        let mut world = near(character, false);
        world.player_two.position.x += world_px(100.0);
        for tick in 0..emission_frame {
            world.update(
                DT,
                FighterInput {
                    signature_special: tick == 0,
                    ..FighterInput::default()
                },
                FighterInput::default(),
            );
        }
        // Interrupt the caster: the detached projectile must still be visible
        // to the CPU without relying on the actor's windup as its warning.
        let jab = borrow_fighters::combat::move_data::move_spec(
            borrow_fighters::combat::move_data::MoveId::LightPunch,
        );
        world
            .player_one
            .take_hit(jab.damage, jab.guard_rule, jab.hit_reaction);
        assert!(world.player_one.attack_kind().is_none());
        let mut cpu = BasicCpu::for_slot(world.player_two.slot);
        let input = cpu.next_input(&world, world.player_two.slot, DT);
        assert!(input.block, "{character:?} projectile should be recognized");
        assert_eq!(input.crouch, low);
    }
}

#[test]
fn fortress_cancels_a_projectile_but_can_be_grabbed_during_its_guard_window() {
    for throw in [false, true] {
        let mut world = near(CharacterId::Rust, false);
        for tick in 0..26 {
            world.update(
                DT,
                FighterInput {
                    signature_special: tick == 0,
                    ..FighterInput::default()
                },
                FighterInput {
                    // Throw active10 and projectile instant spawn both meet guard24.
                    block: throw && tick == 14,
                    light_punch: throw && tick == 14,
                    projectile: !throw && tick == 23,
                    ..FighterInput::default()
                },
            );
        }
        if throw {
            assert!(world.player_one.in_capture());
            assert!(world.player_two.is_throwing());
            assert!(world.player_one.health < world.player_one.max_health);
        } else {
            assert_eq!(world.player_one.health, world.player_one.max_health);
            assert!(world.projectiles.is_empty());
            assert!(world.combat_log().iter().any(|event| matches!(
                event.kind,
                borrow_fighters::game::combat_log::CombatLogKind::ProjectileResolved {
                    damage: 0,
                    blocked: true,
                    ..
                }
            )));
        }
    }
}

#[test]
fn fortress_does_not_protect_the_attackers_back() {
    let mut world = near(CharacterId::Rust, false);
    for tick in 0..28 {
        if tick == 15 {
            world.player_two.position.x =
                world.player_one.position.x - world.player_two.body_rect().width - world_px(10.0);
        }
        world.update(
            DT,
            FighterInput {
                signature_special: tick == 0,
                ..FighterInput::default()
            },
            FighterInput {
                light_punch: tick == 20,
                ..FighterInput::default()
            },
        );
    }
    assert!(world.player_one.health < world.player_one.max_health);
    assert!(world.player_one.attack_kind().is_none());
}
