//! Checks the Python/C++ reaction pilot at each contact and across recovery phases.
//!
//! System: Combat regression. These tests keep visual cadence independent of stun,
//! damage and geometry, and exercise the same World used by matches and tools.

use std::collections::BTreeSet;

use borrow_fighters::{
    characters::CharacterId,
    combat::{
        fighter::{ContactReactionProfile as Profile, FighterInput, PlayerSlot},
        super_sequence::{
            CPP_BARRAGE_CADENCE, CPP_BARRAGE_HITS, CPP_BARRAGE_START, CPP_FINISHER_TICK,
            cpp_barrage_pose_index, cpp_barrage_reaction_profile, super_spec,
        },
    },
    config::{FIXED_TIMESTEP as DT, world_px},
    game::world::World,
    scenes::{
        combat_lab::{CombatLabInput, CombatLabMove},
        move_showcase::{MoveShowcase, MoveShowcaseOptions},
    },
};

fn pair(attacker: CharacterId, reverse: bool) -> World {
    let defender = if attacker == CharacterId::Cpp {
        CharacterId::Python
    } else {
        CharacterId::Cpp
    };
    let mut world = World::new_with_characters(attacker, defender);
    world.player_one.position.x = if reverse { 730.0 } else { 430.0 };
    world.player_two.position.x = if reverse {
        world.player_one.position.x - world.player_two.body_rect().width - world_px(4.0)
    } else {
        world.player_one.body_rect().right() + world_px(4.0)
    };
    world
}

fn special() -> FighterInput {
    FighterInput {
        cinematic_special: true,
        ..FighterInput::default()
    }
}

fn tick(world: &mut World) {
    world.update(DT, FighterInput::default(), FighterInput::default());
}

#[test]
fn each_barrage_contact_restarts_impact_and_finishes_visual_recovery_before_the_next_hit() {
    for reverse in [false, true] {
        for slot in [PlayerSlot::One, PlayerSlot::Two] {
            for guard in [None, Some(false), Some(true)] {
                let mut world = pair(CharacterId::Cpp, reverse);
                if slot == PlayerSlot::Two {
                    world = World::new_with_characters(CharacterId::Python, CharacterId::Cpp);
                    world.player_two.position.x = if reverse { 730.0 } else { 430.0 };
                    world.player_one.position.x = if reverse {
                        world.player_two.position.x
                            - world.player_one.body_rect().width
                            - world_px(4.0)
                    } else {
                        world.player_two.body_rect().right() + world_px(4.0)
                    };
                }
                let defense = FighterInput {
                    block: guard.is_some(),
                    crouch: guard == Some(true),
                    ..FighterInput::default()
                };
                if guard.is_some() {
                    match slot {
                        PlayerSlot::One => world.player_two.health = 1,
                        PlayerSlot::Two => world.player_one.health = 1,
                    }
                }
                match slot {
                    PlayerSlot::One => world.update(DT, special(), defense),
                    PlayerSlot::Two => world.update(DT, defense, special()),
                }
                for _ in 1..CPP_BARRAGE_START {
                    tick(&mut world);
                }
                for hit in 0..CPP_BARRAGE_HITS {
                    let contact_tick = CPP_BARRAGE_START + hit * CPP_BARRAGE_CADENCE;
                    let expected = match guard {
                        Some(false) => Profile::GuardHigh,
                        Some(true) => Profile::GuardLow,
                        None => Profile::Head,
                    };
                    let mut drawings = BTreeSet::new();
                    for age in 0..CPP_BARRAGE_CADENCE {
                        tick(&mut world);
                        assert_eq!(world.super_sequence().unwrap().tick, contact_tick + age);
                        let victim = match slot {
                            PlayerSlot::One => &world.player_two,
                            PlayerSlot::Two => &world.player_one,
                        };
                        let state = victim.contact_reaction_state().expect("contact must react");
                        assert_eq!(state.profile, expected);
                        assert!(victim.attack_kind().is_none());
                        if guard.is_some() {
                            assert_eq!(victim.health, 1, "zero-damage chip must still react");
                        }
                        assert!(victim.in_hitstun() || victim.in_blockstun());
                        assert!((state.duration_seconds - 9.0 * DT).abs() < 0.0001);
                        if age == 0 {
                            assert_eq!(state.elapsed_seconds, 0.0, "impact must not lag contact");
                            assert_eq!(state.progress(), 0.0);
                        }
                        if age == 9 {
                            assert!(state.progress() >= 0.999, "recovery must precede next hit");
                        }
                        drawings.insert(((state.progress() * 4.0) as usize).min(3));
                    }
                    assert_eq!(drawings, BTreeSet::from([0, 1, 2, 3]));
                    assert_eq!(
                        cpp_barrage_pose_index(contact_tick),
                        [1, 3, 2, 4][hit as usize % 4]
                    );
                }
                tick(&mut world);
                assert_eq!(world.super_sequence().unwrap().tick, CPP_FINISHER_TICK);
                let victim = if slot == PlayerSlot::One {
                    &world.player_two
                } else {
                    &world.player_one
                };
                if guard.is_none() {
                    assert_eq!(
                        victim.contact_reaction_state().unwrap().profile,
                        Profile::Launch
                    );
                }
            }
        }
    }
}

#[test]
fn showcase_pause_step_and_replay_preserve_and_restart_the_contact_clock() {
    let mut scene = MoveShowcase::new(MoveShowcaseOptions {
        character: CharacterId::Cpp,
    });
    scene.select_move(CombatLabMove::CinematicSpecial);
    for _ in 0..500 {
        scene.update(CombatLabInput::default());
        if scene
            .world()
            .super_sequence()
            .is_some_and(|s| s.tick == CPP_BARRAGE_START + 5)
        {
            break;
        }
    }
    let before = scene.world().player_two.contact_reaction_state().unwrap();
    assert_eq!(before.profile, Profile::Head);
    assert!((before.elapsed_seconds - 5.0 * DT).abs() < 0.0001);
    scene.update(CombatLabInput {
        pause_toggle: true,
        ..CombatLabInput::default()
    });
    for _ in 0..40 {
        scene.update(CombatLabInput::default());
    }
    assert_eq!(
        scene.world().player_two.contact_reaction_state(),
        Some(before)
    );
    scene.update(CombatLabInput {
        step_frame: true,
        ..CombatLabInput::default()
    });
    let stepped = scene.world().player_two.contact_reaction_state().unwrap();
    assert!((stepped.elapsed_seconds - before.elapsed_seconds - DT).abs() < 0.0001);
    scene.update(CombatLabInput {
        replay: true,
        ..CombatLabInput::default()
    });
    assert!(scene.paused());
    assert!(scene.world().player_two.contact_reaction_state().is_none());
    for _ in 0..500 {
        scene.update(CombatLabInput {
            step_frame: true,
            ..CombatLabInput::default()
        });
        if scene
            .world()
            .super_sequence()
            .is_some_and(|s| s.tick == CPP_BARRAGE_START)
        {
            break;
        }
    }
    let replayed = scene.world().player_two.contact_reaction_state().unwrap();
    assert_eq!(replayed.profile, Profile::Head);
    assert_eq!(replayed.elapsed_seconds, 0.0);
}

fn attack(selected: CombatLabMove, reverse: bool) -> FighterInput {
    use CombatLabMove::*;
    let mut input = FighterInput::default();
    match selected {
        LightPunch | AirPunch => input.light_punch = true,
        HeavyPunch => input.heavy_punch = true,
        Kick | AirKick => input.kick = true,
        Sweep => {
            input.crouch = true;
            input.kick = true;
        }
        Overhead => {
            input.heavy_punch = true;
            input.left = reverse;
            input.right = !reverse;
        }
        AntiAir => {
            input.heavy_punch = true;
            input.crouch = true;
        }
        Throw => {
            input.light_punch = true;
            input.block = true;
        }
        Projectile => input.projectile = true,
        SignatureSpecial => input.signature_special = true,
        CinematicSpecial => input.cinematic_special = true,
    }
    input
}

#[test]
fn both_pilots_cover_every_move_with_immediate_profiles_and_complete_ground_recovery() {
    for character in [CharacterId::Python, CharacterId::Cpp] {
        for reverse in [false, true] {
            for selected in CombatLabMove::ALL {
                let mut world = pair(character, reverse);
                if selected == CombatLabMove::Projectile {
                    world.player_two.position.x += if reverse { -100.0 } else { 100.0 };
                }
                if selected == CombatLabMove::AntiAir {
                    world.player_two.grounded = false;
                    world.player_two.position.y -= world_px(90.0);
                }
                if matches!(selected, CombatLabMove::AirPunch | CombatLabMove::AirKick) {
                    world.player_one.grounded = false;
                    world.player_one.position.y -= world_px(65.0);
                    world.player_one.velocity.y = world_px(-120.0);
                }
                let mut contact_seen = false;
                let mut fall_seen = false;
                let mut rise_seen = false;
                for frame in 0..800 {
                    let before = world.player_two.health;
                    world.update(
                        DT,
                        if frame == 0 {
                            attack(selected, reverse)
                        } else {
                            FighterInput::default()
                        },
                        FighterInput::default(),
                    );
                    if world.player_two.health < before {
                        contact_seen = true;
                        let state = world.player_two.contact_reaction_state().unwrap();
                        assert_eq!(state.elapsed_seconds, 0.0, "{character:?}/{selected:?}");
                        assert!(!world.player_two.is_defeated());
                        let expected = match selected {
                            CombatLabMove::Overhead
                            | CombatLabMove::AirPunch
                            | CombatLabMove::AirKick => Profile::Head,
                            CombatLabMove::Sweep => Profile::Low,
                            CombatLabMove::AntiAir
                            | CombatLabMove::Throw
                            | CombatLabMove::SignatureSpecial => Profile::Launch,
                            CombatLabMove::CinematicSpecial => state.profile,
                            _ => Profile::Body,
                        };
                        assert_eq!(state.profile, expected, "{character:?}/{selected:?}");
                    }
                    if let Some(state) = world.player_two.contact_reaction_state() {
                        assert!(state.duration_seconds > 0.0);
                        fall_seen |= state.profile == Profile::Fall;
                        rise_seen |= state.profile == Profile::Rise;
                    }
                    if contact_seen
                        && !world.super_sequence_active()
                        && !world.player_two.in_hitstun()
                    {
                        break;
                    }
                }
                assert!(
                    contact_seen,
                    "no contact: {character:?}/{selected:?}/{reverse}"
                );
                assert!(world.player_two.grounded);
                assert!(world.player_two.contact_reaction_state().is_none());
                if matches!(
                    selected,
                    CombatLabMove::Sweep
                        | CombatLabMove::AntiAir
                        | CombatLabMove::Throw
                        | CombatLabMove::SignatureSpecial
                        | CombatLabMove::CinematicSpecial
                ) {
                    assert!(
                        fall_seen && rise_seen,
                        "incomplete floor recovery: {character:?}/{selected:?}"
                    );
                }
            }
        }
    }
}

#[test]
fn all_moves_react_only_to_real_contacts_across_movement_attacks_crouch_air_and_guards() {
    let states = [
        "moving",
        "attacking",
        "crouched",
        "airborne",
        "guard_high",
        "guard_low",
        "distant",
    ];
    let mut cases = 0;
    let mut contacts_by_state = [0; 7];
    let mut airborne_reaction_seen = false;
    let mut guarded_profiles = BTreeSet::new();
    let mut whiffs = 0;
    for character in [CharacterId::Python, CharacterId::Cpp] {
        for reverse in [false, true] {
            for selected in CombatLabMove::ALL {
                for (state_index, state) in states.iter().enumerate() {
                    let mut world = pair(character, reverse);
                    if *state == "airborne" {
                        world.player_two.grounded = false;
                        world.player_two.position.y -= world_px(80.0);
                        world.player_two.velocity.y = world_px(-150.0);
                    }
                    if *state == "distant" {
                        world.player_two.position.x = if reverse { 80.0 } else { 1180.0 };
                    }
                    if matches!(selected, CombatLabMove::AirPunch | CombatLabMove::AirKick) {
                        world.player_one.grounded = false;
                        world.player_one.position.y -= world_px(65.0);
                        world.player_one.velocity.y = world_px(-120.0);
                    }
                    let mut contact_seen = false;
                    for frame in 0..760 {
                        let before = world.player_two.health;
                        let target_input = match *state {
                            "moving" => FighterInput {
                                left: !reverse,
                                right: reverse,
                                ..FighterInput::default()
                            },
                            "attacking" if frame == 0 => FighterInput {
                                heavy_punch: true,
                                ..FighterInput::default()
                            },
                            "crouched" | "guard_low" => FighterInput {
                                crouch: true,
                                block: *state == "guard_low",
                                ..FighterInput::default()
                            },
                            "guard_high" => FighterInput {
                                block: true,
                                ..FighterInput::default()
                            },
                            _ => FighterInput::default(),
                        };
                        world.update(
                            DT,
                            if frame == 0 {
                                attack(selected, reverse)
                            } else {
                                FighterInput::default()
                            },
                            target_input,
                        );
                        if world.player_two.health < before {
                            contact_seen = true;
                            contacts_by_state[state_index] += 1;
                            let reaction = world
                                .player_two
                                .contact_reaction_state()
                                .expect("an actual hit must expose the pilot reaction immediately");
                            assert_eq!(
                                reaction.elapsed_seconds, 0.0,
                                "{character:?}/{selected:?}/{state}"
                            );
                            assert!(world.player_two.attack_kind().is_none());
                            assert!(world.player_two.special_elapsed_seconds().is_none());
                            airborne_reaction_seen |=
                                reaction.profile == Profile::Launch && *state == "airborne";
                            if world.player_two.in_blockstun() {
                                let expected = if *state == "guard_low" {
                                    Profile::GuardLow
                                } else {
                                    Profile::GuardHigh
                                };
                                assert_eq!(reaction.profile, expected);
                                guarded_profiles.insert(format!("{:?}", reaction.profile));
                            }
                        }
                        if !contact_seen {
                            assert!(
                                world.player_two.contact_reaction_state().is_none(),
                                "reaction before contact: {character:?}/{selected:?}/{state}"
                            );
                        }
                    }
                    if *state == "distant"
                        && !matches!(
                            selected,
                            CombatLabMove::Projectile
                                | CombatLabMove::SignatureSpecial
                                | CombatLabMove::CinematicSpecial
                        )
                    {
                        assert!(!contact_seen, "out-of-range close move acquired a reaction");
                        whiffs += 1;
                    }
                    cases += 1;
                }
            }
        }
    }
    assert_eq!(cases, 336);
    assert!(contacts_by_state.into_iter().all(|count| count > 0));
    assert_eq!(guarded_profiles.len(), 2);
    assert!(airborne_reaction_seen);
    assert_eq!(whiffs, 36);
}

#[test]
fn lethal_pilot_supers_hold_the_final_fall_pose_and_reset_clears_contact_state() {
    for character in [CharacterId::Python, CharacterId::Cpp] {
        let mut world = pair(character, false);
        world.player_two.health = 1;
        world.update(DT, special(), FighterInput::default());
        for _ in 0..super_spec(character).unwrap().duration_frames + 80 {
            tick(&mut world);
        }
        assert!(world.outcome.is_some());
        let state = world.player_two.contact_reaction_state().unwrap();
        assert_eq!(state.profile, Profile::Fall);
        assert_eq!(state.progress(), 1.0);
        assert!(world.player_two.grounded);
        world = pair(character, false);
        assert!(world.player_two.contact_reaction_state().is_none());
        assert!(world.player_one.contact_reaction_state().is_none());
    }
}

#[test]
fn pilot_contract_is_absent_for_other_fighters_and_outside_the_barrage() {
    for character in [
        CharacterId::Rust,
        CharacterId::Duke,
        CharacterId::Go,
        CharacterId::C,
    ] {
        let mut world = World::new_with_characters(CharacterId::Cpp, character);
        world.update(DT, special(), FighterInput::default());
        for _ in 0..CPP_FINISHER_TICK + 60 {
            tick(&mut world);
            assert!(world.player_two.contact_reaction_state().is_none());
        }
    }
    assert_eq!(cpp_barrage_reaction_profile(CPP_BARRAGE_START - 1), None);
    assert_eq!(cpp_barrage_reaction_profile(CPP_FINISHER_TICK), None);
}
