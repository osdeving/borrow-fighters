//! Exercises traditional fighting-game verbs added after the first combat slice.

use borrow_fighters::combat::fighter::{AttackKind, Fighter, FighterInput, PlayerSlot};
use borrow_fighters::combat::move_data::{GuardRule, MoveId, move_spec};
use borrow_fighters::game::world::World;

const DT: f32 = 1.0 / 60.0;

#[test]
fn low_sweep_requires_crouch_block() {
    let mut stand_block = World::new_greybox();
    place_for_contact(&mut stand_block);
    let health = stand_block.player_two.health;

    stand_block.update(
        DT,
        FighterInput {
            crouch: true,
            kick: true,
            ..FighterInput::default()
        },
        FighterInput {
            block: true,
            ..FighterInput::default()
        },
    );
    advance(
        &mut stand_block,
        18,
        FighterInput::default(),
        FighterInput::default(),
    );

    assert_eq!(
        stand_block.player_two.health,
        health - move_spec(MoveId::SweepKick).damage
    );
    assert!(stand_block.player_two.in_hitstun());

    let mut crouch_block = World::new_greybox();
    place_for_contact(&mut crouch_block);
    let health = crouch_block.player_two.health;
    let blocking = FighterInput {
        block: true,
        crouch: true,
        ..FighterInput::default()
    };

    crouch_block.update(
        DT,
        FighterInput {
            crouch: true,
            kick: true,
            ..FighterInput::default()
        },
        blocking,
    );
    advance(&mut crouch_block, 18, FighterInput::default(), blocking);

    assert_eq!(
        crouch_block.player_two.health,
        health - move_spec(MoveId::SweepKick).damage / 4
    );
    assert!(crouch_block.player_two.in_blockstun());
}

#[test]
fn overhead_requires_standing_block() {
    let mut crouch_block = World::new_greybox();
    place_for_contact(&mut crouch_block);
    let health = crouch_block.player_two.health;

    crouch_block.update(
        DT,
        FighterInput {
            right: true,
            heavy_punch: true,
            ..FighterInput::default()
        },
        FighterInput {
            block: true,
            crouch: true,
            ..FighterInput::default()
        },
    );
    advance(
        &mut crouch_block,
        18,
        FighterInput::default(),
        FighterInput {
            block: true,
            crouch: true,
            ..FighterInput::default()
        },
    );

    assert_eq!(
        crouch_block.player_two.health,
        health - move_spec(MoveId::OverheadPunch).damage
    );
    assert!(crouch_block.player_two.in_hitstun());

    let mut stand_block = World::new_greybox();
    place_for_contact(&mut stand_block);
    let health = stand_block.player_two.health;
    let blocking = FighterInput {
        block: true,
        ..FighterInput::default()
    };

    stand_block.update(
        DT,
        FighterInput {
            right: true,
            heavy_punch: true,
            ..FighterInput::default()
        },
        blocking,
    );
    advance(&mut stand_block, 18, FighterInput::default(), blocking);

    assert_eq!(
        stand_block.player_two.health,
        health - move_spec(MoveId::OverheadPunch).damage / 4
    );
    assert!(stand_block.player_two.in_blockstun());
}

#[test]
fn close_throw_ignores_block() {
    let mut world = World::new_greybox();
    world.player_one.position.x = 420.0;
    world.player_two.position.x = 482.0;
    let health = world.player_two.health;

    world.update(
        DT,
        FighterInput {
            block: true,
            light_punch: true,
            ..FighterInput::default()
        },
        FighterInput {
            block: true,
            ..FighterInput::default()
        },
    );
    advance(
        &mut world,
        8,
        FighterInput::default(),
        FighterInput {
            block: true,
            ..FighterInput::default()
        },
    );

    assert_eq!(
        world.player_two.health,
        health - move_spec(MoveId::CloseThrow).damage
    );
    assert!(!world.player_two.in_blockstun());
    assert!(world.player_two.in_hitstun());
}

#[test]
fn airborne_buttons_start_air_attacks() {
    let mut punch = airborne_fighter();
    punch.update(
        DT,
        FighterInput {
            light_punch: true,
            ..FighterInput::default()
        },
    );

    assert_eq!(punch.attack_kind(), Some(AttackKind::AirPunch));
    assert_eq!(
        punch.attack_move_spec().map(|spec| spec.id),
        Some(MoveId::AirPunch)
    );

    let mut kick = airborne_fighter();
    kick.update(
        DT,
        FighterInput {
            kick: true,
            ..FighterInput::default()
        },
    );

    assert_eq!(kick.attack_kind(), Some(AttackKind::AirKick));
    assert_eq!(
        kick.attack_move_spec().map(|spec| spec.id),
        Some(MoveId::AirKick)
    );
}

#[test]
fn crouch_heavy_starts_anti_air() {
    let mut fighter = Fighter::new(PlayerSlot::One, "Rust", 320.0);

    fighter.update(
        DT,
        FighterInput {
            crouch: true,
            heavy_punch: true,
            ..FighterInput::default()
        },
    );

    assert_eq!(fighter.attack_kind(), Some(AttackKind::AntiAir));
    assert_eq!(
        fighter.attack_move_spec().map(|spec| spec.id),
        Some(MoveId::RisingAntiAir)
    );
}

#[test]
fn guard_rule_high_low_throw_contract_is_explicit() {
    assert!(GuardRule::High.is_blocked_by(true, false));
    assert!(!GuardRule::High.is_blocked_by(true, true));
    assert!(!GuardRule::Low.is_blocked_by(true, false));
    assert!(GuardRule::Low.is_blocked_by(true, true));
    assert!(!GuardRule::Throw.is_blocked_by(true, false));
    assert!(!GuardRule::Throw.is_blocked_by(true, true));
}

fn place_for_contact(world: &mut World) {
    world.player_one.position.x = 420.0;
    world.player_two.position.x = 500.0;
}

fn advance(world: &mut World, frames: usize, player_one: FighterInput, player_two: FighterInput) {
    for _ in 0..frames {
        world.update(DT, player_one, player_two);
    }
}

fn airborne_fighter() -> Fighter {
    let mut fighter = Fighter::new(PlayerSlot::One, "Rust", 320.0);
    fighter.grounded = false;
    fighter.position.y -= 92.0;
    fighter
}
