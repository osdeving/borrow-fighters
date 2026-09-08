//! Verifies real capture, aerial trajectories, safe corners and lethal landings.
//!
//! System: Combat integration. These tests observe physical motion and control
//! recovery across the five public fighters rather than animation labels alone.

use borrow_fighters::{
    characters::CharacterId,
    combat::fighter::{FighterInput, HitReactionKind, PlayerSlot},
    config::{ARENA_LEFT, ARENA_RIGHT, FIXED_TIMESTEP as DT, FLOOR_Y, world_px},
    game::world::{MatchOutcome, ThrowPhase, World},
};

const ROSTER: [CharacterId; 5] = [
    CharacterId::Rust,
    CharacterId::Duke,
    CharacterId::C,
    CharacterId::Python,
    CharacterId::Cpp,
];

fn throw_world(character: CharacterId, reverse: bool, corner: i32) -> World {
    let mut world = World::new_with_characters(character, CharacterId::Rust);
    let width = world.player_one.body_rect().width;
    let left = match corner {
        -1 => ARENA_LEFT,
        1 => ARENA_RIGHT - 2.0 * width - world_px(10.0),
        _ => world_px(360.0),
    };
    world.player_one.position.x = left + if reverse { width + world_px(10.0) } else { 0.0 };
    world.player_two.position.x = left + if reverse { 0.0 } else { width + world_px(10.0) };
    world
}

#[test]
fn throws_capture_lift_cross_above_the_attacker_and_land_safely_for_all_five() {
    for character in ROSTER {
        for reverse in [false, true] {
            for corner in [-1, 0, 1] {
                for low_guard in [false, true] {
                    let mut world = throw_world(character, reverse, corner);
                    let initial_health = world.player_two.health;
                    let initial_side = world.player_two.position.x > world.player_one.position.x;
                    let initial_center_gap = (world.player_two.body_rect().center_x()
                        - world.player_one.body_rect().center_x())
                    .abs();
                    let mut closest_capture_gap = initial_center_gap;
                    let mut capture_seen = false;
                    let mut flight_seen = false;
                    let mut cross_seen = false;
                    let mut previous_side = initial_side;
                    let mut landed = false;
                    for tick in 0..150 {
                        world.update(
                            DT,
                            FighterInput {
                                block: tick == 0,
                                light_punch: tick == 0,
                                ..FighterInput::default()
                            },
                            FighterInput {
                                block: true,
                                crouch: low_guard,
                                jump: tick > 11,
                                kick: tick > 11,
                                ..FighterInput::default()
                            },
                        );
                        capture_seen |= world.player_two.in_capture();
                        flight_seen |= world.player_two.in_air_reaction();
                        if let Some(sequence) = world.throw_sequence() {
                            if sequence.phase == ThrowPhase::Capture {
                                assert!(world.player_one.is_throwing());
                                assert!(!world.player_two.blocking);
                                closest_capture_gap = closest_capture_gap.min(
                                    (world.player_two.body_rect().center_x()
                                        - world.player_one.body_rect().center_x())
                                    .abs(),
                                );
                            } else {
                                let side = world.player_two.body_rect().center_x()
                                    > world.player_one.body_rect().center_x();
                                if side != previous_side {
                                    cross_seen = true;
                                    assert!(
                                        world.player_two.body_rect().bottom()
                                            < world.player_one.body_rect().y,
                                        "crossing must happen above the attacker"
                                    );
                                }
                                previous_side = side;
                            }
                        }
                        if world.player_two.in_knockdown() {
                            landed = true;
                            break;
                        }
                    }
                    assert!(
                        capture_seen && flight_seen && landed,
                        "{character:?}/{reverse}/{corner}/{low_guard}"
                    );
                    assert!(closest_capture_gap < world_px(25.0));
                    assert!(closest_capture_gap < initial_center_gap * 0.5);
                    assert!(world.player_two.health < initial_health);
                    assert_eq!(world.player_two.body_rect().bottom(), FLOOR_Y);
                    assert_eq!(world.player_two.reaction_visual_elapsed_seconds(), 0.1);
                    assert!(world.player_two.position.x >= ARENA_LEFT);
                    assert!(world.player_two.body_rect().right() <= ARENA_RIGHT + 0.01);
                    assert!(world.throw_sequence().is_none());
                    assert!(!world.player_one.is_throwing());
                    if corner == 0 {
                        assert!(cross_seen);
                        assert_ne!(
                            world.player_two.position.x > world.player_one.position.x,
                            initial_side
                        );
                    }
                }
            }
        }
    }
}

#[test]
fn lethal_throw_finishes_its_flight_before_announcing_the_winner() {
    for character in ROSTER {
        for reverse in [false, true] {
            let mut world = throw_world(character, reverse, 0);
            world.player_two.health = 1;
            let mut lethal_flight_seen = false;
            for tick in 0..160 {
                world.update(
                    DT,
                    FighterInput {
                        block: tick == 0,
                        light_punch: tick == 0,
                        ..FighterInput::default()
                    },
                    FighterInput::default(),
                );
                if world.player_two.is_defeated()
                    && (world.player_two.in_capture() || world.player_two.in_air_reaction())
                {
                    lethal_flight_seen = true;
                    assert!(world.outcome.is_none());
                }
                if world.outcome.is_some() {
                    break;
                }
            }
            assert!(lethal_flight_seen);
            assert!(world.player_two.grounded);
            assert_eq!(world.outcome, Some(MatchOutcome::Winner(PlayerSlot::One)));
        }
    }
}

#[test]
fn every_anti_air_launches_under_gravity_then_allows_recovery_and_jump() {
    for character in ROSTER {
        let mut world = throw_world(character, false, 0);
        world.player_two.position.y -= world_px(90.0);
        world.player_two.grounded = false;
        let mut contact_y = None;
        let mut min_y = FLOOR_Y;
        let mut landed = false;
        for tick in 0..180 {
            world.update(
                DT,
                FighterInput {
                    crouch: tick == 0,
                    heavy_punch: tick == 0,
                    ..FighterInput::default()
                },
                FighterInput {
                    block: true,
                    kick: true,
                    ..FighterInput::default()
                },
            );
            if world.player_two.hit_reaction_kind() == HitReactionKind::Launched
                && world.player_two.in_air_reaction()
            {
                contact_y.get_or_insert(world.player_two.position.y);
                min_y = min_y.min(world.player_two.position.y);
                assert!(world.player_two.attack_kind().is_none());
                assert!(!world.player_two.blocking);
            }
            if world.player_two.in_knockdown() {
                landed = true;
                break;
            }
        }
        assert!(landed, "{character:?} must land after launch");
        assert_eq!(world.player_two.reaction_visual_elapsed_seconds(), 0.1);
        assert!(min_y < contact_y.unwrap() - world_px(40.0));
        for _ in 0..40 {
            world.update(DT, FighterInput::default(), FighterInput::default());
        }
        world.update(
            DT,
            FighterInput::default(),
            FighterInput {
                jump: true,
                ..FighterInput::default()
            },
        );
        assert!(!world.player_two.grounded);
        assert!(!world.player_two.in_air_reaction());
    }
}
