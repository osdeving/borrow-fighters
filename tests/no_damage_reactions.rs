//! Keeps playtest health protection independent of confirmed contact reactions.
//!
//! System: World regression. A protected fighter at one HP must move, guard,
//! animate and sound like a healthy fighter receiving the same real contact.

use borrow_fighters::{
    audio::AudioCue,
    characters::CharacterId,
    combat::{
        fighter::{Fighter, FighterInput, PlayerSlot},
        super_sequence::{CPP_BARRAGE_START, CPP_FINISHER_TICK, super_spec},
    },
    config::{FIXED_TIMESTEP as DT, world_px},
    engine::sprites::{SpriteManifest, fighter_sprite_clip, frame_for_contact_reaction},
    game::{
        feature_flags::{FeatureFlag, FeatureFlags},
        world::World,
    },
    scenes::combat_lab::CombatLabMove,
};

fn fighter(world: &World, slot: PlayerSlot) -> &Fighter {
    match slot {
        PlayerSlot::One => &world.player_one,
        PlayerSlot::Two => &world.player_two,
    }
}

fn fighter_mut(world: &mut World, slot: PlayerSlot) -> &mut Fighter {
    match slot {
        PlayerSlot::One => &mut world.player_one,
        PlayerSlot::Two => &mut world.player_two,
    }
}

fn opposite(slot: PlayerSlot) -> PlayerSlot {
    match slot {
        PlayerSlot::One => PlayerSlot::Two,
        PlayerSlot::Two => PlayerSlot::One,
    }
}

fn damage_flag(slot: PlayerSlot) -> FeatureFlag {
    match slot {
        PlayerSlot::One => FeatureFlag::PlayerOneTakesDamage,
        PlayerSlot::Two => FeatureFlag::PlayerTwoTakesDamage,
    }
}

fn update(
    world: &mut World,
    slot: PlayerSlot,
    attack: FighterInput,
    guard: FighterInput,
    flags: FeatureFlags,
) {
    let (one, two) = match slot {
        PlayerSlot::One => (attack, guard),
        PlayerSlot::Two => (guard, attack),
    };
    world.update_with_flags(DT, one, two, flags);
}

fn pair() -> World {
    let mut world = World::new_with_characters(CharacterId::Cpp, CharacterId::Python);
    world.player_one.position.x = 420.0;
    world.player_two.position.x = world.player_one.body_rect().right() + world_px(4.0);
    world
}

fn attack(selected: CombatLabMove) -> FighterInput {
    let mut input = FighterInput::default();
    match selected {
        CombatLabMove::LightPunch => input.light_punch = true,
        CombatLabMove::Sweep => {
            input.crouch = true;
            input.kick = true;
        }
        CombatLabMove::AntiAir => {
            input.crouch = true;
            input.heavy_punch = true;
        }
        CombatLabMove::Throw => {
            input.light_punch = true;
            input.block = true;
        }
        CombatLabMove::Projectile => input.projectile = true,
        CombatLabMove::SignatureSpecial => input.signature_special = true,
        CombatLabMove::CinematicSpecial => input.cinematic_special = true,
        _ => unreachable!("only regression cases are scheduled"),
    }
    input
}

fn assert_same_response(protected: &Fighter, healthy: &Fighter) {
    assert_eq!(protected.health, 1);
    assert!(!protected.is_defeated());
    assert_eq!(protected.position, healthy.position);
    assert_eq!(protected.velocity, healthy.velocity);
    assert_eq!(protected.grounded, healthy.grounded);
    assert_eq!(protected.blocking, healthy.blocking);
    assert_eq!(protected.crouching, healthy.crouching);
    assert_eq!(protected.in_capture(), healthy.in_capture());
    assert_eq!(
        protected.reaction_visual_state(),
        healthy.reaction_visual_state()
    );
    assert_eq!(
        protected.contact_reaction_state(),
        healthy.contact_reaction_state()
    );
    assert_eq!(fighter_sprite_clip(protected), fighter_sprite_clip(healthy));
}

#[test]
fn protected_one_hp_contacts_keep_guard_pushback_throws_launches_audio_and_whiffs() {
    use CombatLabMove::*;
    let cases = [
        (LightPunch, None, false),
        (LightPunch, Some(false), false),
        (Sweep, Some(false), false),
        (Sweep, Some(true), false),
        (Throw, Some(false), false),
        (AntiAir, None, false),
        (Projectile, None, false),
        (Projectile, Some(false), false),
        (SignatureSpecial, None, false),
        (LightPunch, None, true),
    ];
    for slot in [PlayerSlot::One, PlayerSlot::Two] {
        for (selected, guard, whiff) in cases {
            let target_slot = opposite(slot);
            let mut healthy = pair();
            if selected == Projectile {
                fighter_mut(&mut healthy, target_slot).position.x += if slot == PlayerSlot::One {
                    100.0
                } else {
                    -100.0
                };
            }
            if selected == AntiAir {
                let target = fighter_mut(&mut healthy, target_slot);
                target.grounded = false;
                target.position.y -= world_px(90.0);
            }
            if whiff {
                fighter_mut(&mut healthy, target_slot).position.x += if slot == PlayerSlot::One {
                    450.0
                } else {
                    -350.0
                };
            }
            let mut protected = healthy.clone();
            fighter_mut(&mut protected, target_slot).health = 1;
            let mut flags = FeatureFlags::default();
            flags.set(damage_flag(target_slot), false);
            let mut contact_seen = false;
            let mut reaction_seen = false;
            for frame in 0..180 {
                let input = if frame == 0 {
                    attack(selected)
                } else {
                    FighterInput::default()
                };
                let guard = FighterInput {
                    block: guard.is_some(),
                    crouch: guard == Some(true),
                    ..FighterInput::default()
                };
                update(&mut healthy, slot, input, guard, FeatureFlags::default());
                update(&mut protected, slot, input, guard, flags);
                assert_same_response(
                    fighter(&protected, target_slot),
                    fighter(&healthy, target_slot),
                );
                let events = protected.take_audio_events();
                contact_seen |= events.iter().any(|event| {
                    matches!(event.cue, AudioCue::FighterHurt | AudioCue::FighterBlock)
                });
                assert_eq!(
                    events,
                    healthy.take_audio_events(),
                    "{slot:?}/{selected:?}/{guard:?}"
                );
                reaction_seen |= fighter(&protected, target_slot)
                    .contact_reaction_state()
                    .is_some();
                assert!(protected.hit_effects.iter().all(|hit| hit.damage == 0));
                assert!(protected.outcome.is_none());
            }
            assert_eq!(contact_seen, !whiff, "{slot:?}/{selected:?}");
            assert_eq!(reaction_seen, !whiff, "{slot:?}/{selected:?}");
        }
    }
}

#[test]
fn damage_flags_affect_only_the_selected_slot_health() {
    for one_enabled in [false, true] {
        for two_enabled in [false, true] {
            for attacker in [PlayerSlot::One, PlayerSlot::Two] {
                let mut world = pair();
                let target = opposite(attacker);
                let original = fighter(&world, target).health;
                let mut flags = FeatureFlags::default();
                flags.set(FeatureFlag::PlayerOneTakesDamage, one_enabled);
                flags.set(FeatureFlag::PlayerTwoTakesDamage, two_enabled);
                for frame in 0..40 {
                    update(
                        &mut world,
                        attacker,
                        if frame == 0 {
                            attack(CombatLabMove::LightPunch)
                        } else {
                            FighterInput::default()
                        },
                        FighterInput::default(),
                        flags,
                    );
                }
                assert_eq!(
                    fighter(&world, target).health < original,
                    flags.enabled(damage_flag(target))
                );
                assert_eq!(
                    fighter(&world, attacker).health,
                    fighter(&world, attacker).max_health
                );
                assert!(
                    world
                        .take_audio_events()
                        .iter()
                        .any(|event| event.cue == AudioCue::FighterHurt
                            && event.slot == Some(target))
                );
            }
        }
    }
}

#[test]
fn protected_supers_keep_every_barrage_drawing_and_guarded_zero_chip_contact() {
    for attacker in [CharacterId::Cpp, CharacterId::Python] {
        for slot in [PlayerSlot::One, PlayerSlot::Two] {
            for guard in [None, Some(false), Some(true)] {
                let defender = if attacker == CharacterId::Cpp {
                    CharacterId::Python
                } else {
                    CharacterId::Cpp
                };
                let mut healthy = match slot {
                    PlayerSlot::One => World::new_with_characters(attacker, defender),
                    PlayerSlot::Two => World::new_with_characters(defender, attacker),
                };
                let target_slot = opposite(slot);
                let mut protected = healthy.clone();
                fighter_mut(&mut protected, target_slot).health = 1;
                let mut zero_chip = guard.map(|_| protected.clone());
                let mut flags = FeatureFlags::default();
                flags.set(damage_flag(target_slot), false);
                let manifest = SpriteManifest::load(format!(
                    "assets/candidates/{0}/{0}-fighter.sprite.json",
                    defender.audio_key()
                ))
                .unwrap();
                let spec = super_spec(attacker).unwrap();
                let mut contacts = 0;
                for frame in 0..spec.duration_frames + 90 {
                    let input = if frame == 0 {
                        attack(CombatLabMove::CinematicSpecial)
                    } else {
                        FighterInput::default()
                    };
                    let guard = FighterInput {
                        block: guard.is_some(),
                        crouch: guard == Some(true),
                        ..FighterInput::default()
                    };
                    update(&mut healthy, slot, input, guard, FeatureFlags::default());
                    update(&mut protected, slot, input, guard, flags);
                    let victim = fighter(&protected, target_slot);
                    assert_same_response(victim, fighter(&healthy, target_slot));
                    if let Some(sequence) = protected.super_sequence() {
                        if spec
                            .contacts
                            .iter()
                            .any(|contact| contact.tick == sequence.tick)
                        {
                            contacts += 1;
                            assert_eq!(
                                victim.contact_reaction_state().unwrap().elapsed_seconds,
                                0.0
                            );
                        }
                        if attacker == CharacterId::Cpp
                            && (CPP_BARRAGE_START..CPP_FINISHER_TICK).contains(&sequence.tick)
                        {
                            let drawing = frame_for_contact_reaction(&manifest, victim).unwrap();
                            assert!(drawing.clip.starts_with("reaction_"));
                            assert_eq!(
                                Some(drawing.name.as_str()),
                                frame_for_contact_reaction(
                                    &manifest,
                                    fighter(&healthy, target_slot)
                                )
                                .map(|frame| frame.name.as_str())
                            );
                        }
                    }
                    let events = protected.take_audio_events();
                    assert_eq!(events, healthy.take_audio_events());
                    if let Some(zero_chip) = &mut zero_chip {
                        update(zero_chip, slot, input, guard, FeatureFlags::default());
                        assert_same_response(
                            fighter(zero_chip, target_slot),
                            fighter(&protected, target_slot),
                        );
                        assert_eq!(events, zero_chip.take_audio_events());
                    }
                    assert!(protected.hit_effects.iter().all(|hit| hit.damage == 0));
                    assert!(protected.outcome.is_none());
                }
                assert_eq!(contacts, spec.contacts.len());
            }
        }
    }
}
