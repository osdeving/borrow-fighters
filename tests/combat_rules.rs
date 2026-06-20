//! Exercises testable greybox combat rules without opening a Raylib window.

use borrow_fighters::combat::fighter::FighterInput;
use borrow_fighters::game::world::{MatchOutcome, World};

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
