//! Checks each new roster reaction against real cinematic contacts and KO.
//!
//! Both player slots, facings and protected health use the same World path as a
//! match. Every barrage impact must play its four drawings before the next hit.

use std::collections::BTreeSet;

use borrow_fighters::{
    characters::{CHARACTER_BODY_METRICS_PATH, CharacterBodyMetricsCatalog, CharacterId},
    combat::{
        fighter::{ContactReactionProfile, Fighter, FighterInput, PlayerSlot},
        super_sequence::{CPP_BARRAGE_CADENCE, CPP_BARRAGE_HITS, CPP_BARRAGE_START, super_spec},
    },
    config::{FIXED_TIMESTEP as DT, FLOOR_Y, world_px},
    engine::sprites::{SpriteManifest, contact_reaction_clip_name, frame_for_contact_reaction},
    game::{
        feature_flags::{FeatureFlag, FeatureFlags},
        world::World,
    },
};

const DEFENDERS: [CharacterId; 4] = [
    CharacterId::Rust,
    CharacterId::Duke,
    CharacterId::C,
    CharacterId::Go,
];

fn target(world: &World, attacker: PlayerSlot) -> &Fighter {
    match attacker {
        PlayerSlot::One => &world.player_two,
        PlayerSlot::Two => &world.player_one,
    }
}

fn target_mut(world: &mut World, attacker: PlayerSlot) -> &mut Fighter {
    match attacker {
        PlayerSlot::One => &mut world.player_two,
        PlayerSlot::Two => &mut world.player_one,
    }
}

fn world(defender: CharacterId, attacker: PlayerSlot, reverse: bool) -> World {
    let metrics = CharacterBodyMetricsCatalog::load(CHARACTER_BODY_METRICS_PATH).unwrap();
    let (one, two) = match attacker {
        PlayerSlot::One => (CharacterId::Cpp, defender),
        PlayerSlot::Two => (defender, CharacterId::Cpp),
    };
    let mut world = World::new_with_character_body_metrics(one, two, &metrics);
    let (striker, victim) = match attacker {
        PlayerSlot::One => (&mut world.player_one, &mut world.player_two),
        PlayerSlot::Two => (&mut world.player_two, &mut world.player_one),
    };
    striker.position.x = if reverse { 840.0 } else { 420.0 };
    victim.position.x = if reverse {
        striker.position.x - victim.body_rect().width - world_px(4.0)
    } else {
        striker.body_rect().right() + world_px(4.0)
    };
    world
}

fn update(
    world: &mut World,
    attacker: PlayerSlot,
    attack: FighterInput,
    defense: FighterInput,
    flags: FeatureFlags,
) {
    let (one, two) = match attacker {
        PlayerSlot::One => (attack, defense),
        PlayerSlot::Two => (defense, attack),
    };
    world.update_with_flags(DT, one, two, flags);
}

fn idle(world: &mut World, flags: FeatureFlags) {
    world.update_with_flags(DT, FighterInput::default(), FighterInput::default(), flags);
}

fn manifest(defender: CharacterId) -> SpriteManifest {
    SpriteManifest::load(format!(
        "assets/candidates/{0}/{0}-fighter.sprite.json",
        defender.audio_key()
    ))
    .unwrap()
}

#[test]
fn every_new_defender_restarts_four_drawings_on_each_barrage_hit_with_guard_and_protected_health() {
    let mut cases = 0;
    for defender in DEFENDERS {
        let manifest = manifest(defender);
        for attacker in [PlayerSlot::One, PlayerSlot::Two] {
            for reverse in [false, true] {
                for guard in [None, Some(false), Some(true)] {
                    for protected in [false, true] {
                        let context = format!(
                            "{defender:?}/{attacker:?}/reverse={reverse}/guard={guard:?}/protected={protected}"
                        );
                        let mut world = world(defender, attacker, reverse);
                        let mut flags = FeatureFlags::default();
                        flags.set(
                            match attacker {
                                PlayerSlot::One => FeatureFlag::PlayerTwoTakesDamage,
                                PlayerSlot::Two => FeatureFlag::PlayerOneTakesDamage,
                            },
                            !protected,
                        );
                        if protected || guard.is_some() {
                            target_mut(&mut world, attacker).health = 1;
                        }
                        update(
                            &mut world,
                            attacker,
                            FighterInput {
                                cinematic_special: true,
                                ..Default::default()
                            },
                            FighterInput {
                                block: guard.is_some(),
                                crouch: guard == Some(true),
                                ..Default::default()
                            },
                            flags,
                        );
                        for _ in 1..CPP_BARRAGE_START {
                            idle(&mut world, flags);
                        }
                        let profile = match guard {
                            None => ContactReactionProfile::Head,
                            Some(false) => ContactReactionProfile::GuardHigh,
                            Some(true) => ContactReactionProfile::GuardLow,
                        };
                        let expected = &manifest
                            .clip_named(contact_reaction_clip_name(profile))
                            .unwrap()
                            .frames;
                        assert_eq!(expected.len(), 4, "{context}");
                        for hit in 0..CPP_BARRAGE_HITS {
                            let mut seen = BTreeSet::new();
                            for age in 0..CPP_BARRAGE_CADENCE {
                                let before = target(&world, attacker).health;
                                idle(&mut world, flags);
                                let tick = CPP_BARRAGE_START + hit * CPP_BARRAGE_CADENCE + age;
                                assert_eq!(world.super_sequence().unwrap().tick, tick, "{context}");
                                let victim = target(&world, attacker);
                                let contact = victim
                                    .contact_reaction_state()
                                    .expect("real barrage hit must react");
                                let frame = frame_for_contact_reaction(&manifest, victim)
                                    .expect("new roster reaction must resolve");
                                assert_eq!(
                                    contact.profile, profile,
                                    "{context}/hit={hit}/age={age}"
                                );
                                if age == 0 {
                                    assert_eq!(contact.elapsed_seconds, 0.0, "{context}");
                                    assert_eq!(
                                        &frame.name, &expected[0],
                                        "impact frame lagged: {context}"
                                    );
                                    if !protected && guard.is_none() {
                                        assert!(
                                            victim.health < before,
                                            "missing real damage: {context}"
                                        );
                                    }
                                }
                                if protected || guard.is_some() {
                                    assert_eq!(
                                        victim.health, 1,
                                        "zero damage must preserve reactions: {context}"
                                    );
                                }
                                if age == CPP_BARRAGE_CADENCE - 1 {
                                    assert_eq!(
                                        &frame.name,
                                        expected.last().unwrap(),
                                        "recovery missed its window: {context}"
                                    );
                                }
                                assert!(
                                    victim.attack_kind().is_none(),
                                    "old attack survived contact: {context}"
                                );
                                seen.insert(frame.name.clone());
                            }
                            assert_eq!(
                                seen,
                                expected.iter().cloned().collect::<BTreeSet<_>>(),
                                "barrage did not articulate all four drawings: {context}/hit={hit}"
                            );
                        }
                        cases += 1;
                    }
                }
            }
        }
    }
    assert_eq!(cases, 96);
}

#[test]
fn lethal_contacts_hold_new_roster_fall_frames_at_the_floor_in_both_directions() {
    for defender in DEFENDERS {
        let manifest = manifest(defender);
        for reverse in [false, true] {
            let mut world = world(defender, PlayerSlot::One, reverse);
            world.player_two.health = 1;
            update(
                &mut world,
                PlayerSlot::One,
                FighterInput {
                    cinematic_special: true,
                    ..Default::default()
                },
                FighterInput::default(),
                FeatureFlags::default(),
            );
            for _ in 0..super_spec(CharacterId::Cpp).unwrap().duration_frames + 60 {
                idle(&mut world, FeatureFlags::default());
            }
            assert!(
                world.outcome.is_some(),
                "{defender:?}/{reverse}: KO never resolved"
            );
            for _ in 0..60 {
                idle(&mut world, FeatureFlags::default());
                let victim = &world.player_two;
                let frame = frame_for_contact_reaction(&manifest, victim).unwrap();
                assert_eq!(frame.clip, "reaction_fall");
                assert_eq!(
                    &frame.name,
                    manifest
                        .clip_named("reaction_fall")
                        .unwrap()
                        .frames
                        .last()
                        .unwrap()
                );
                assert!(victim.grounded);
                assert!((victim.body_rect().bottom() - FLOOR_Y).abs() < 0.01);
            }
        }
    }
}
