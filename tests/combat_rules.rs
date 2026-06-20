//! Exercises testable greybox combat rules without opening a Raylib window.

use borrow_fighters::combat::fighter::FighterInput;
use borrow_fighters::game::world::{MIN_BODY_GAP, MatchOutcome, World};

const DT: f32 = 1.0 / 60.0;

#[test]
fn basic_attack_deals_damage_once_per_swing() {
    let mut world = World::new_greybox();
    world.player_one.position.x = 420.0;
    world.player_two.position.x = 470.0;

    world.update(
        DT,
        FighterInput {
            attack: true,
            ..FighterInput::default()
        },
        FighterInput::default(),
    );

    for _ in 0..20 {
        world.update(DT, FighterInput::default(), FighterInput::default());
    }

    assert_eq!(world.player_two.health, 88);
    assert_eq!(world.hit_effects.len(), 1);
}

#[test]
fn match_ends_when_health_reaches_zero() {
    let mut world = World::new_greybox();
    world.player_two.health = 12;
    world.player_one.position.x = 420.0;
    world.player_two.position.x = 470.0;

    world.update(
        DT,
        FighterInput {
            attack: true,
            ..FighterInput::default()
        },
        FighterInput::default(),
    );

    for _ in 0..20 {
        world.update(DT, FighterInput::default(), FighterInput::default());
    }

    assert_eq!(
        world.outcome,
        Some(MatchOutcome::Winner(world.player_one.slot))
    );
}

#[test]
fn hit_feedback_expires_after_short_lifetime() {
    let mut world = World::new_greybox();
    world.player_one.position.x = 420.0;
    world.player_two.position.x = 470.0;

    world.update(
        DT,
        FighterInput {
            attack: true,
            ..FighterInput::default()
        },
        FighterInput::default(),
    );

    for _ in 0..20 {
        world.update(DT, FighterInput::default(), FighterInput::default());
    }

    assert_eq!(world.hit_effects.len(), 1);

    for _ in 0..60 {
        world.update(DT, FighterInput::default(), FighterInput::default());
    }

    assert!(world.hit_effects.is_empty());
}

#[test]
fn fighters_cannot_walk_through_each_other() {
    let mut world = World::new_greybox();
    world.player_one.position.x = 420.0;
    world.player_two.position.x = 455.0;

    for _ in 0..30 {
        world.update(
            DT,
            FighterInput {
                right: true,
                ..FighterInput::default()
            },
            FighterInput {
                left: true,
                ..FighterInput::default()
            },
        );
    }

    assert!(
        !world
            .player_one
            .body_rect()
            .intersects(world.player_two.body_rect())
    );
    assert_body_gap(&world);
    assert!(world.body_collision_timer > 0.0);
}

#[test]
fn fighters_keep_body_gap_when_pinned_to_arena_edge() {
    let mut world = World::new_greybox();
    world.player_one.position.x = 820.0;
    world.player_two.position.x = 876.0;

    for _ in 0..120 {
        world.update(
            DT,
            FighterInput {
                right: true,
                ..FighterInput::default()
            },
            FighterInput {
                left: true,
                ..FighterInput::default()
            },
        );
    }

    assert_body_gap(&world);
}

fn assert_body_gap(world: &World) {
    let p1 = world.player_one.body_rect();
    let p2 = world.player_two.body_rect();
    let gap = if p1.center_x() <= p2.center_x() {
        p2.x - p1.right()
    } else {
        p1.x - p2.right()
    };
    assert!(gap >= MIN_BODY_GAP - 0.001, "body gap was {gap}");
}
