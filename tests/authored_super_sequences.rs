//! Exercises confirmed super capture, authoritative contacts and restoration.
//!
//! System: Match integration. Visual coverage, delayed input and frame chunking
//! cannot add damage, skip authored phases, or change the captured defense.

use borrow_fighters::{
    audio::AudioCue,
    characters::CharacterId,
    combat::{
        fighter::{Facing, Fighter, FighterInput, PlayerSlot},
        projectile::Projectile,
        super_sequence::{
            CPP_BARRAGE_START, CPP_CHARGE_START, CPP_FINISHER_TICK, CPP_REBOOT_TICK,
            PYTHON_SWALLOW_TICK, PYTHON_TARGET_RETURN_TICK, RUST_ARENA_COMMIT_TICK, SuperPhase,
            super_spec,
        },
    },
    config::{ARENA_LEFT, ARENA_RIGHT, FIXED_TIMESTEP as DT, FLOOR_Y, world_px},
    game::{
        arena::ArenaId,
        combat_log::CombatLogKind,
        feature_flags::{FeatureFlag, FeatureFlags},
        world::{MIN_BODY_GAP, MatchOutcome, World},
    },
};

const AUTHORED: [CharacterId; 5] = [
    CharacterId::Rust,
    CharacterId::Duke,
    CharacterId::C,
    CharacterId::Cpp,
    CharacterId::Python,
];

fn request() -> FighterInput {
    FighterInput {
        cinematic_special: true,
        ..FighterInput::default()
    }
}
fn fighter(world: &World, slot: PlayerSlot) -> &Fighter {
    match slot {
        PlayerSlot::One => &world.player_one,
        PlayerSlot::Two => &world.player_two,
    }
}
fn opposite(slot: PlayerSlot) -> PlayerSlot {
    match slot {
        PlayerSlot::One => PlayerSlot::Two,
        PlayerSlot::Two => PlayerSlot::One,
    }
}
fn arrangement(character: CharacterId, slot: PlayerSlot, reverse: bool) -> World {
    let mut world = match slot {
        PlayerSlot::One => World::new_with_characters(character, CharacterId::Go),
        PlayerSlot::Two => World::new_with_characters(CharacterId::Go, character),
    };
    world.player_one.position.x = if reverse {
        ARENA_RIGHT - world.player_one.body_rect().width
    } else {
        ARENA_LEFT
    };
    world.player_two.position.x = if reverse {
        ARENA_LEFT
    } else {
        ARENA_RIGHT - world.player_two.body_rect().width
    };
    world
}
fn input(world: &mut World, slot: PlayerSlot, attacker: FighterInput, target: FighterInput) {
    match slot {
        PlayerSlot::One => world.update(DT, attacker, target),
        PlayerSlot::Two => world.update(DT, target, attacker),
    }
}
fn tick(world: &mut World) {
    world.update(DT, FighterInput::default(), FighterInput::default());
}
fn contacts(world: &World) -> usize {
    world
        .combat_log()
        .iter()
        .filter(|event| matches!(event.kind, CombatLogKind::CloseAttackResolved { .. }))
        .count()
}

#[test]
fn every_authored_capture_reaches_both_corners_from_either_slot_and_only_scheduled_ticks_damage() {
    for character in AUTHORED {
        let spec = super_spec(character).unwrap();
        for slot in [PlayerSlot::One, PlayerSlot::Two] {
            for reverse in [false, true] {
                let mut world = arrangement(character, slot, reverse);
                input(&mut world, slot, request(), FighterInput::default());
                let sequence = world.super_sequence().unwrap();
                assert_eq!(sequence.attacker, slot);
                assert_eq!(sequence.target, opposite(slot));
                assert_eq!(sequence.tick, 0);
                assert_eq!(sequence.phase(), SuperPhase::Freeze);
                let mut expected_damage = 0;
                let mut reaction_seen = false;
                for frame in 1..spec.duration_frames {
                    tick(&mut world);
                    expected_damage += spec
                        .contacts
                        .iter()
                        .filter(|contact| contact.tick == frame)
                        .map(|contact| contact.damage)
                        .sum::<i32>();
                    let sequence = world.super_sequence().unwrap();
                    assert_eq!(sequence.tick, frame);
                    assert!(sequence.phase_progress() >= 0.0 && sequence.phase_progress() < 1.0);
                    assert_eq!(
                        fighter(&world, opposite(slot)).max_health
                            - fighter(&world, opposite(slot)).health,
                        expected_damage,
                        "{character:?}/{slot:?}/{reverse}/tick{frame}"
                    );
                    assert_eq!(
                        fighter(&world, slot).health,
                        fighter(&world, slot).max_health,
                        "footshot never damages its owner"
                    );
                    assert!(world.projectiles.is_empty() && world.signature_effects.is_empty());
                    assert!(world.outcome.is_none());
                    reaction_seen |= fighter(&world, opposite(slot)).in_air_reaction()
                        || fighter(&world, opposite(slot)).in_knockdown();
                }
                assert!(
                    reaction_seen,
                    "every confirmed super must show a real damage reaction"
                );
                assert_eq!(contacts(&world), spec.contacts.len());
                tick(&mut world);
                assert!(!world.super_sequence_active());
                for _ in 0..70 {
                    tick(&mut world);
                }
                assert!(!fighter(&world, opposite(slot)).in_knockdown());
                assert_eq!(contacts(&world), spec.contacts.len());
            }
        }
    }
}

#[test]
fn guard_is_captured_at_entry_for_both_heights_and_never_chip_kills() {
    for character in AUTHORED {
        for slot in [PlayerSlot::One, PlayerSlot::Two] {
            for crouch in [false, true] {
                for initial_hp in [100, 1] {
                    let mut world = arrangement(character, slot, false);
                    match slot {
                        PlayerSlot::One => world.player_two.health = initial_hp,
                        PlayerSlot::Two => world.player_one.health = initial_hp,
                    };
                    input(
                        &mut world,
                        slot,
                        request(),
                        FighterInput {
                            block: true,
                            crouch,
                            ..FighterInput::default()
                        },
                    );
                    assert!(world.super_sequence().unwrap().guarded);
                    assert_eq!(world.super_sequence().unwrap().target_crouching, crouch);
                    let spec = super_spec(character).unwrap();
                    for _ in 0..spec.duration_frames {
                        tick(&mut world);
                    }
                    let chip: i32 = spec
                        .contacts
                        .iter()
                        .map(|contact| (contact.damage / 4).max(1))
                        .sum();
                    assert_eq!(
                        fighter(&world, opposite(slot)).health,
                        (initial_hp - chip).max(1)
                    );
                    assert!(world.outcome.is_none());
                    assert!(!fighter(&world, opposite(slot)).blocking);
                }
            }
        }
        let mut world = arrangement(character, PlayerSlot::One, false);
        world.update(DT, request(), FighterInput::default());
        for _ in 0..super_spec(character).unwrap().duration_frames {
            world.update(
                DT,
                FighterInput::default(),
                FighterInput {
                    block: true,
                    ..FighterInput::default()
                },
            );
        }
        let total: i32 = super_spec(character)
            .unwrap()
            .contacts
            .iter()
            .map(|contact| contact.damage)
            .sum();
        assert_eq!(
            world.player_two.max_health - world.player_two.health,
            total,
            "late guard cannot rewrite entry"
        );
    }
}

#[test]
fn knockouts_wait_for_restoration_and_training_invincibility_applies_to_both_slots() {
    for character in AUTHORED {
        let spec = super_spec(character).unwrap();
        for slot in [PlayerSlot::One, PlayerSlot::Two] {
            for invincible in [false, true] {
                let mut world = arrangement(character, slot, false);
                let target_flag = match slot {
                    PlayerSlot::One => {
                        world.player_two.health = 1;
                        FeatureFlag::PlayerTwoTakesDamage
                    }
                    PlayerSlot::Two => {
                        world.player_one.health = 1;
                        FeatureFlag::PlayerOneTakesDamage
                    }
                };
                let mut flags = FeatureFlags::default();
                flags.set(target_flag, !invincible);
                input(&mut world, slot, request(), FighterInput::default());
                for frame in 1..=spec.duration_frames {
                    world.update_with_flags(
                        DT,
                        FighterInput::default(),
                        FighterInput::default(),
                        flags,
                    );
                    if frame < spec.duration_frames {
                        assert!(world.outcome.is_none());
                        assert!(world.super_sequence_active());
                    }
                }
                assert_eq!(
                    fighter(&world, opposite(slot)).health,
                    i32::from(invincible)
                );
                assert_eq!(
                    world.outcome,
                    if invincible {
                        None
                    } else {
                        Some(MatchOutcome::Winner(slot))
                    }
                );
                let cues = world.take_audio_events();
                assert_eq!(
                    cues.iter()
                        .filter(|event| event.cue == AudioCue::SuperStart)
                        .count(),
                    1
                );
                assert_eq!(
                    cues.iter()
                        .filter(|event| event.cue == AudioCue::SuperEnd)
                        .count(),
                    1
                );
            }
        }
    }
}

#[test]
fn cpp_runs_to_the_real_target_then_delivers_eight_hits_finisher_and_inherited_reboot() {
    for slot in [PlayerSlot::One, PlayerSlot::Two] {
        for reverse in [false, true] {
            let mut world = arrangement(CharacterId::Cpp, slot, reverse);
            input(&mut world, slot, request(), FighterInput::default());
            let start = fighter(&world, slot).position.x;
            let mut previous = start;
            for frame in 1..=CPP_BARRAGE_START {
                tick(&mut world);
                let sequence = world.super_sequence().unwrap();
                let x = fighter(&world, slot).position.x;
                if frame <= CPP_CHARGE_START {
                    assert_eq!(x, start);
                } else {
                    assert_ne!(x, previous, "the rush must move each tick");
                    assert!((x - previous).abs() < world_px(20.0), "no teleport");
                    assert_eq!(
                        (x - previous).is_sign_positive(),
                        sequence.facing == Facing::Right
                    );
                }
                assert_eq!(
                    sequence.attacker_anchor.x,
                    fighter(&world, slot).body_rect().center_x()
                );
                previous = x;
            }
            let actor = fighter(&world, slot).body_rect();
            let target = fighter(&world, opposite(slot)).body_rect();
            let gap = if actor.x < target.x {
                target.x - actor.right()
            } else {
                actor.x - target.right()
            };
            assert!((gap - MIN_BODY_GAP).abs() < 0.01);
            assert_eq!(contacts(&world), 1);
            for _ in CPP_BARRAGE_START + 1..=CPP_FINISHER_TICK {
                tick(&mut world);
            }
            assert_eq!(contacts(&world), 9);
            assert!(fighter(&world, opposite(slot)).in_air_reaction());
            for _ in CPP_FINISHER_TICK + 1..=CPP_REBOOT_TICK {
                tick(&mut world);
            }
            assert_eq!(contacts(&world), 10);
            assert!(fighter(&world, opposite(slot)).in_air_reaction());
        }
    }
}

#[test]
fn existing_projectiles_freeze_and_commands_cannot_interrupt_or_add_contacts() {
    for character in AUTHORED {
        let mut world = arrangement(character, PlayerSlot::One, false);
        world
            .projectiles
            .push(Projectile::from_fighter(&world.player_two));
        let before = world.projectiles.clone();
        let target_position = world.player_two.position;
        world.update(
            DT,
            FighterInput {
                block: true,
                signature_special: true,
                light_punch: true,
                ..request()
            },
            FighterInput::default(),
        );
        assert!(
            !world.player_one.blocking,
            "LB super chord clears owner guard"
        );
        for frame in 1..=super_spec(character).unwrap().duration_frames {
            let noisy = FighterInput {
                left: true,
                jump: true,
                heavy_punch: true,
                projectile: true,
                signature_special: true,
                light_punch: true,
                ..request()
            };
            world.update(DT, noisy, noisy);
            assert_eq!(world.projectiles, before);
            if frame < super_spec(character).unwrap().contacts[0].tick {
                assert_eq!(world.player_two.position, target_position);
            }
            assert!(world.player_one.attack_kind().is_none());
            assert!(world.player_two.attack_kind().is_none());
        }
        assert_eq!(
            contacts(&world),
            super_spec(character).unwrap().contacts.len()
        );
        tick(&mut world);
        assert_ne!(
            world.projectiles, before,
            "projectiles resume after release"
        );
    }
}

#[test]
fn simultaneous_eligible_supers_clash_symmetrically_and_can_be_retried_next_tick() {
    for one in AUTHORED {
        for two in AUTHORED {
            let mut world = World::new_with_characters(one, two);
            world.update(DT, request(), request());
            assert!(!world.super_sequence_active());
            assert_eq!(contacts(&world), 0);
            assert!(
                world
                    .take_audio_events()
                    .iter()
                    .all(|event| event.cue != AudioCue::SuperStart)
            );
            world.update(DT, FighterInput::default(), request());
            assert_eq!(world.super_sequence().unwrap().attacker, PlayerSlot::Two);
        }
    }
}

#[test]
fn airborne_and_busy_attackers_cannot_start_a_super() {
    for character in AUTHORED {
        let mut world = arrangement(character, PlayerSlot::One, false);
        world.update(
            DT,
            FighterInput {
                jump: true,
                ..FighterInput::default()
            },
            FighterInput::default(),
        );
        world.update(DT, request(), FighterInput::default());
        assert!(!world.super_sequence_active());
        assert!(world.player_one.attack_kind().is_none());
        let mut world = arrangement(character, PlayerSlot::One, false);
        world.update(
            DT,
            FighterInput {
                light_punch: true,
                ..FighterInput::default()
            },
            FighterInput::default(),
        );
        world.update(DT, request(), FighterInput::default());
        assert!(!world.super_sequence_active());
    }
}

#[test]
fn frame_chunking_and_half_steps_emit_each_contact_and_phase_cue_once() {
    for character in AUTHORED {
        let spec = super_spec(character).unwrap();
        let mut fine = arrangement(character, PlayerSlot::One, false);
        let mut coarse = arrangement(character, PlayerSlot::One, false);
        fine.update(DT, request(), FighterInput::default());
        coarse.update(DT, request(), FighterInput::default());
        for _ in 0..spec.duration_frames * 2 {
            fine.update(DT * 0.5, FighterInput::default(), FighterInput::default());
        }
        coarse.update(
            DT * spec.duration_frames as f32,
            FighterInput::default(),
            FighterInput::default(),
        );
        assert_eq!(fine.player_two.health, coarse.player_two.health);
        assert_eq!(fine.take_audio_events(), coarse.take_audio_events());
        assert_eq!(contacts(&fine), spec.contacts.len());
        assert!(!fine.super_sequence_active() && !coarse.super_sequence_active());
        let events = coarse.combat_log();
        assert_eq!(
            events
                .iter()
                .filter(|event| matches!(event.kind, CombatLogKind::CloseAttackStarted { .. }))
                .count(),
            1
        );
    }
}

#[test]
fn trash_collection_has_twelve_cadenced_clones_and_reset_discards_the_capture() {
    let mut world = arrangement(CharacterId::Duke, PlayerSlot::One, false);
    world.update(DT, request(), FighterInput::default());
    let mut collect_ticks = Vec::new();
    let mut slam_ticks = Vec::new();
    for frame in 1..350 {
        tick(&mut world);
        for event in world.take_audio_events() {
            if event.cue == AudioCue::SuperCollect {
                collect_ticks.push(frame);
            }
            if event.cue == AudioCue::SuperGiantDrop {
                slam_ticks.push(frame);
            }
        }
    }
    assert_eq!(
        slam_ticks,
        vec![264],
        "the slam must coincide with the giant contact"
    );
    assert_eq!(
        collect_ticks,
        (0..12).map(|index| 84 + index * 12).collect::<Vec<_>>()
    );
    assert_eq!(world.super_sequence().unwrap().phase(), SuperPhase::Restore);
    world = arrangement(CharacterId::Duke, PlayerSlot::One, false);
    assert!(!world.super_sequence_active());
    assert!(
        world
            .take_audio_events()
            .iter()
            .all(|event| event.cue != AudioCue::SuperEnd)
    );
    tick(&mut world);
    assert_eq!(contacts(&world), 0);
    world.update(DT, request(), FighterInput::default());
    assert_eq!(world.super_sequence().unwrap().tick, 0);
}

#[test]
fn launched_signature_effects_keep_their_positions_and_lifetime_during_capture() {
    let mut world = World::new_with_characters(CharacterId::Rust, CharacterId::Duke);
    world.player_one.position.x = ARENA_LEFT;
    world.player_two.position.x = ARENA_RIGHT - world.player_two.body_rect().width;
    for frame in 0..27 {
        world.update(
            DT,
            FighterInput::default(),
            FighterInput {
                signature_special: frame == 0,
                ..FighterInput::default()
            },
        );
    }
    assert!(!world.signature_effects.is_empty());
    let before = world.signature_effects.clone();
    world.update(DT, request(), FighterInput::default());
    assert!(world.super_sequence_active());
    for _ in 0..300 {
        tick(&mut world);
        assert_eq!(world.signature_effects.len(), before.len());
        for (effect, previous) in world.signature_effects.iter().zip(&before) {
            assert_eq!(effect.position, previous.position);
            assert_eq!(effect.elapsed_seconds, previous.elapsed_seconds);
            assert_eq!(effect.has_connected(), previous.has_connected());
        }
    }
    tick(&mut world);
    assert!(
        world
            .signature_effects
            .iter()
            .zip(&before)
            .any(|(effect, previous)| effect.elapsed_seconds > previous.elapsed_seconds)
    );
}

#[test]
fn airborne_targets_freeze_then_settle_without_teleport_and_stay_on_the_authored_floor() {
    for character in AUTHORED {
        for slot in [PlayerSlot::One, PlayerSlot::Two] {
            for invincible in [false, true] {
                let mut world = arrangement(character, slot, false);
                let target = match slot {
                    PlayerSlot::One => &mut world.player_two,
                    PlayerSlot::Two => &mut world.player_one,
                };
                target.grounded = false;
                target.position.y -= world_px(180.0);
                let original = target.position;
                let mut flags = FeatureFlags::default();
                flags.set(
                    if slot == PlayerSlot::One {
                        FeatureFlag::PlayerTwoTakesDamage
                    } else {
                        FeatureFlag::PlayerOneTakesDamage
                    },
                    !invincible,
                );
                input(&mut world, slot, request(), FighterInput::default());
                assert_eq!(fighter(&world, opposite(slot)).position, original);
                let mut previous_y = original.y;
                for frame in 1..=super_spec(character).unwrap().duration_frames {
                    world.update_with_flags(
                        DT,
                        FighterInput::default(),
                        FighterInput::default(),
                        flags,
                    );
                    let target = fighter(&world, opposite(slot));
                    if frame < super_spec(character).unwrap().contacts[0].tick || invincible {
                        assert_eq!(target.position.x, original.x);
                    }
                    if frame <= 8 {
                        assert_eq!(target.position.y, original.y);
                        assert!(!target.grounded);
                    } else if frame < 20 {
                        assert!(target.position.y > previous_y);
                        assert!(target.body_rect().bottom() < FLOOR_Y);
                        assert!(!target.grounded);
                    } else if frame < super_spec(character).unwrap().contacts[0].tick || invincible
                    {
                        assert!(target.grounded);
                        assert!((target.body_rect().bottom() - FLOOR_Y).abs() < 0.01);
                    }
                    previous_y = target.position.y;
                }
            }
        }
    }
}

#[test]
fn rust_commits_sirius_after_the_last_build_block_and_keeps_it_through_guard_ko_and_reset() {
    for base in ArenaId::ROTATION {
        for guarded in [false, true] {
            let mut world = arrangement(CharacterId::Rust, PlayerSlot::One, false);
            world.player_two.health = 1;
            world.update(
                DT,
                request(),
                FighterInput {
                    block: guarded,
                    ..FighterInput::default()
                },
            );
            for frame in 1..RUST_ARENA_COMMIT_TICK {
                assert_eq!(
                    world.effective_arena(base),
                    base,
                    "mutation must not commit early at tick {frame}"
                );
                assert_eq!(world.arena_override(), None);
                tick(&mut world);
            }
            assert_eq!(
                world.super_sequence().unwrap().tick,
                RUST_ARENA_COMMIT_TICK - 1
            );
            tick(&mut world);
            assert_eq!(world.arena_override(), Some(ArenaId::Sirius));
            assert_eq!(world.effective_arena(base), ArenaId::Sirius);
            for _ in RUST_ARENA_COMMIT_TICK..super_spec(CharacterId::Rust).unwrap().duration_frames
            {
                tick(&mut world);
            }
            assert!(!world.super_sequence_active());
            assert_eq!(world.effective_arena(base), ArenaId::Sirius);
            assert_eq!(world.outcome.is_some(), !guarded);
            tick(&mut world);
            assert_eq!(world.effective_arena(base), ArenaId::Sirius);
            world = arrangement(CharacterId::Rust, PlayerSlot::One, false);
            assert_eq!(world.effective_arena(base), base);
            assert_eq!(world.arena_override(), None);
        }
    }
}

#[test]
fn python_swallow_hides_both_guard_and_hit_targets_until_return_without_an_extra_contact() {
    for slot in [PlayerSlot::One, PlayerSlot::Two] {
        for reverse in [false, true] {
            for guarded in [false, true] {
                let mut world = arrangement(CharacterId::Python, slot, reverse);
                input(
                    &mut world,
                    slot,
                    request(),
                    FighterInput {
                        block: guarded,
                        ..FighterInput::default()
                    },
                );
                let mut return_health = None;
                for frame in 1..super_spec(CharacterId::Python).unwrap().duration_frames {
                    tick(&mut world);
                    let sequence = world.super_sequence().unwrap();
                    assert_eq!(
                        sequence.target_hidden(),
                        (PYTHON_SWALLOW_TICK..PYTHON_TARGET_RETURN_TICK).contains(&frame)
                    );
                    let target = fighter(&world, opposite(slot));
                    if frame < PYTHON_SWALLOW_TICK {
                        assert_eq!(target.health, target.max_health);
                    }
                    if frame == PYTHON_TARGET_RETURN_TICK - 1 {
                        return_health = Some(target.health);
                    }
                    if frame == PYTHON_TARGET_RETURN_TICK {
                        assert_eq!(Some(target.health), return_health);
                        assert_eq!(contacts(&world), 1);
                        assert!(
                            target.in_air_reaction() || target.in_blockstun(),
                            "return must restart a visible reaction"
                        );
                    }
                }
                tick(&mut world);
                assert_eq!(contacts(&world), 1);
                assert!(fighter(&world, opposite(slot)).grounded);
                assert!(!world.super_sequence_active());
            }
        }
    }
}

#[test]
fn schedules_cover_every_tick_and_never_place_contact_outside_an_authored_phase() {
    for character in AUTHORED {
        let spec = super_spec(character).unwrap();
        assert_eq!(spec.phases[0].phase, SuperPhase::Freeze);
        assert_eq!(spec.phases[0].start, 0);
        assert_eq!(spec.phases.last().unwrap().end, spec.duration_frames);
        for neighbors in spec.phases.windows(2) {
            assert_eq!(
                neighbors[0].end, neighbors[1].start,
                "no skipped or competing phases for {character:?}"
            );
        }
        for contact in spec.contacts {
            assert!(contact.tick < spec.duration_frames);
            assert!(spec.phases.iter().any(|phase| phase.start <= contact.tick
                && contact.tick < phase.end
                && phase.phase != SuperPhase::Freeze));
        }
    }
}

#[test]
fn transformation_and_notebook_audio_follow_authored_actions_once() {
    for (character, mut expected) in [
        (
            CharacterId::Python,
            vec![
                (0, AudioCue::SuperStart),
                (62, AudioCue::SuperMutation),
                (158, AudioCue::SuperMutation),
                (270, AudioCue::SuperLunge),
                (304, AudioCue::SuperSwallow),
                (344, AudioCue::SuperRevert),
                (400, AudioCue::SuperCelebrate),
                (444, AudioCue::SuperPeace),
                (528, AudioCue::SuperEnd),
            ],
        ),
        (
            CharacterId::Cpp,
            vec![
                (0, AudioCue::SuperStart),
                (25, AudioCue::SuperTyping),
                (130, AudioCue::SuperEnter),
                (184, AudioCue::SuperFootshot),
                (424, AudioCue::SuperBarrageHit),
                (450, AudioCue::SuperError),
                (498, AudioCue::SuperError),
                (538, AudioCue::SuperBoot),
                (566, AudioCue::SuperBarrageHit),
                (632, AudioCue::SuperEnd),
            ],
        ),
    ] {
        if character == CharacterId::Cpp {
            expected.extend((0..8).map(|index| (344 + index * 10, AudioCue::SuperBarrageHit)));
            expected.sort_by_key(|(frame, _)| *frame);
        }
        let mut world = arrangement(character, PlayerSlot::Two, true);
        input(
            &mut world,
            PlayerSlot::Two,
            request(),
            FighterInput::default(),
        );
        let mut actual = Vec::new();
        for frame in 0..=super_spec(character).unwrap().duration_frames {
            if frame > 0 {
                tick(&mut world);
            }
            for event in world.take_audio_events() {
                if event.cue.key().starts_with("super.") {
                    assert_eq!(event.character, Some(character));
                    assert_eq!(event.slot, Some(PlayerSlot::Two));
                    assert_eq!(event.move_id, Some(super_spec(character).unwrap().move_id));
                    actual.push((frame, event.cue));
                }
            }
        }
        assert_eq!(actual, expected, "{character:?} phase foley drifted");
    }
}
