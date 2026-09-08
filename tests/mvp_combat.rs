//! Guards against combat exploits and verifies the five signature moves in a real World.
//!
//! System: Combat regression tests. These scenarios exercise action locks,
//! counterplay and recovery rather than relying on generated sprite metadata.

use borrow_fighters::characters::{CharacterId, character_spec};
use borrow_fighters::combat::fighter::{
    AttackKind, Facing, Fighter, FighterInput, HitReactionKind, PlayerSlot,
};
use borrow_fighters::combat::move_data::{
    GuardRule, MoveId, MoveInputKind, move_spec, move_spec_for_input,
};
use borrow_fighters::config::{FIXED_TIMESTEP as DT, FLOOR_Y, world_px};
use borrow_fighters::game::world::World;

const ROSTER: [CharacterId; 5] = [
    CharacterId::Rust,
    CharacterId::Duke,
    CharacterId::C,
    CharacterId::Python,
    CharacterId::Cpp,
];
const NONE: FighterInput = FighterInput {
    left: false,
    right: false,
    jump: false,
    crouch: false,
    block: false,
    light_punch: false,
    heavy_punch: false,
    kick: false,
    projectile: false,
    signature_special: false,
};

fn close_world(character: CharacterId) -> World {
    let mut world = World::new_with_characters(character, CharacterId::Rust);
    world.player_one.position.x = world_px(240.0);
    world.player_two.position.x = world.player_one.body_rect().right() + world_px(10.0);
    world
}

#[test]
fn all_five_have_one_unique_special_with_block_and_whiff_counterplay() {
    let mut ids = Vec::new();
    for character in ROSTER {
        let moves = character_spec(character).move_ids;
        let special = move_spec_for_input(moves, MoveInputKind::SignatureSpecial).unwrap();
        assert!(!ids.contains(&special.id));
        ids.push(special.id);
        assert!(special.damage <= 20);
        assert!(special.frames.active_start.get() >= 9);
        assert!(
            special.frames.duration.get() - special.frames.active_end.get()
                > special.hit_reaction.blockstun.get() + 6,
            "{character:?} must be punishable even on late block"
        );
        assert!(special.whiff_recovery.get() >= 12);
        assert_ne!(special.guard_rule, GuardRule::Throw);
    }
    assert!(
        move_spec_for_input(
            character_spec(CharacterId::Go).move_ids,
            MoveInputKind::SignatureSpecial
        )
        .is_none()
    );
}

#[test]
fn every_signature_can_connect_in_its_intended_situation_once() {
    for character in ROSTER {
        let mut world = close_world(character);
        let special = move_spec_for_input(
            character_spec(character).move_ids,
            MoveInputKind::SignatureSpecial,
        )
        .unwrap();
        if character == CharacterId::Cpp {
            world.player_two.position.y -= world_px(90.0);
            world.player_two.grounded = false;
        }
        world.update(
            DT,
            FighterInput {
                signature_special: true,
                ..NONE
            },
            NONE,
        );
        for _ in 0..special.frames.duration.get() {
            world.update(DT, NONE, NONE);
        }
        assert_eq!(
            world.player_two.health,
            world.player_two.max_health - special.damage,
            "{character:?} special should deal exactly one hit"
        );
    }
}

#[test]
fn low_and_overhead_specials_respect_guard_height() {
    for (character, crouching, should_block) in [
        (CharacterId::Duke, false, true),
        (CharacterId::Duke, true, false),
        (CharacterId::Python, true, true),
        (CharacterId::Python, false, false),
        (CharacterId::Rust, true, true),
        (CharacterId::C, false, true),
    ] {
        let mut world = close_world(character);
        let special = move_spec_for_input(
            character_spec(character).move_ids,
            MoveInputKind::SignatureSpecial,
        )
        .unwrap();
        let guard = FighterInput {
            block: true,
            crouch: crouching,
            ..NONE
        };
        world.update(
            DT,
            FighterInput {
                signature_special: true,
                ..NONE
            },
            guard,
        );
        for _ in 0..special.frames.duration.get() {
            world.update(DT, NONE, guard);
        }
        let expected = if should_block {
            special.damage / 4
        } else {
            special.damage
        };
        assert_eq!(
            world.player_two.max_health - world.player_two.health,
            expected,
            "{character:?} crouch={crouching}"
        );
    }
}

#[test]
fn guarded_low_keeps_crouch_height_through_blockstun_and_guard_break_clears_blockstun() {
    let mut fighter = Fighter::new(PlayerSlot::One, "defender", 300.0);
    let low = move_spec(MoveId::SweepKick);
    fighter.update(
        DT,
        FighterInput {
            crouch: true,
            block: true,
            ..NONE
        },
    );
    assert!(
        fighter
            .take_hit(low.damage, low.guard_rule, low.hit_reaction)
            .blocked
    );
    fighter.update(
        DT,
        FighterInput {
            crouch: true,
            block: true,
            ..NONE
        },
    );
    assert!(fighter.crouching);
    assert!(
        fighter
            .take_hit(low.damage, low.guard_rule, low.hit_reaction)
            .blocked
    );
    let high = move_spec(MoveId::OverheadPunch);
    assert!(
        !fighter
            .take_hit(high.damage, high.guard_rule, high.hit_reaction)
            .blocked
    );
    assert!(!fighter.in_blockstun());
    fighter.update(
        DT,
        FighterInput {
            block: true,
            ..NONE
        },
    );
    assert!(!fighter.blocking);
}

#[test]
fn sweep_stays_low_and_cannot_hit_a_jumping_opponent() {
    let mut world = close_world(CharacterId::Rust);
    world.update(
        DT,
        FighterInput {
            crouch: true,
            kick: true,
            ..NONE
        },
        FighterInput { jump: true, ..NONE },
    );
    for _ in 0..28 {
        world.update(DT, NONE, NONE);
        if let Some(hitbox) = world.player_one.active_hitbox() {
            assert!(world.player_one.crouching);
            assert!(hitbox.y >= FLOOR_Y - world_px(34.0));
        }
    }
    assert_eq!(world.player_two.health, world.player_two.max_health);
}

#[test]
fn ground_throw_misses_an_airborne_target_even_with_overlapping_body() {
    let mut world = close_world(CharacterId::Rust);
    world.update(
        DT,
        FighterInput {
            block: true,
            light_punch: true,
            ..NONE
        },
        FighterInput { jump: true, ..NONE },
    );
    for _ in 0..12 {
        world.update(DT, NONE, NONE);
    }
    assert_eq!(world.player_two.health, world.player_two.max_health);
}

#[test]
fn knockdown_plays_once_protects_the_floor_and_allows_wake_jump() {
    let mut world = close_world(CharacterId::Rust);
    world.update(
        DT,
        FighterInput {
            block: true,
            light_punch: true,
            ..NONE
        },
        FighterInput {
            block: true,
            ..NONE
        },
    );
    for _ in 0..10 {
        world.update(DT, NONE, NONE);
    }
    assert!(world.player_two.in_knockdown());
    assert_eq!(
        world.player_two.hit_reaction_kind(),
        HitReactionKind::Knockdown
    );
    let health = world.player_two.health;
    // A fresh attacker and projectile overlap the downed fighter deliberately.
    world.player_one = close_world(CharacterId::Rust).player_one;
    world.player_one.position.x =
        world.player_two.position.x - world.player_one.body_rect().width - world_px(10.0);
    for _ in 0..16 {
        world.update(
            DT,
            FighterInput {
                light_punch: true,
                projectile: true,
                ..NONE
            },
            NONE,
        );
    }
    assert_eq!(world.player_two.health, health);
    while world.player_two.in_knockdown() {
        world.update(DT, NONE, NONE);
    }
    assert!(!world.player_two.can_be_thrown());
    world.update(DT, NONE, FighterInput { jump: true, ..NONE });
    assert!(!world.player_two.grounded);
}

#[test]
fn projectile_cast_cannot_be_cancelled_into_attack_jump_or_guard() {
    let mut world = World::new_greybox();
    world.update(
        DT,
        FighterInput {
            projectile: true,
            ..NONE
        },
        NONE,
    );
    for _ in 0..10 {
        world.update(
            DT,
            FighterInput {
                signature_special: true,
                light_punch: true,
                jump: true,
                block: true,
                ..NONE
            },
            NONE,
        );
        assert!(world.player_one.attack_kind().is_none());
        assert!(world.player_one.grounded);
        assert!(!world.player_one.blocking);
    }
}

#[test]
fn chip_cannot_ko_a_defender() {
    let mut fighter = Fighter::new(PlayerSlot::One, "defender", 300.0);
    fighter.health = 1;
    fighter.update(
        DT,
        FighterInput {
            block: true,
            ..NONE
        },
    );
    let hit = move_spec(MoveId::HeavyPunch);
    let result = fighter.take_hit(hit.damage, hit.guard_rule, hit.hit_reaction);
    assert!(result.blocked);
    assert_eq!(result.damage, 0);
    assert_eq!(fighter.health, 1);
}

#[test]
fn an_attack_commits_its_facing_and_cannot_track_a_jump_over() {
    let mut world = close_world(CharacterId::Rust);
    world.update(
        DT,
        FighterInput {
            signature_special: true,
            ..NONE
        },
        NONE,
    );
    assert_eq!(
        world.player_one.attack_kind(),
        Some(AttackKind::SignatureSpecial)
    );
    world.player_two.position.x = world.player_one.position.x - world_px(170.0);
    world.player_two.position.y -= world_px(200.0);
    world.player_two.grounded = false;
    world.update(DT, NONE, NONE);
    assert_eq!(world.player_one.facing, Facing::Right);
}

#[test]
fn signature_startup_is_interruptible_by_a_close_jab() {
    for character in ROSTER {
        let mut world = close_world(character);
        world.update(
            DT,
            FighterInput {
                signature_special: true,
                ..NONE
            },
            FighterInput {
                light_punch: true,
                ..NONE
            },
        );
        for _ in 0..7 {
            world.update(DT, NONE, NONE);
        }
        assert!(
            world.player_one.health < world.player_one.max_health,
            "{character:?} must be interruptible"
        );
        assert!(world.player_one.attack_kind().is_none());
        assert_eq!(world.player_two.health, world.player_two.max_health);
    }
}

#[test]
fn a_close_blocked_signature_can_be_punished_before_recovery_ends() {
    for character in [
        CharacterId::Rust,
        CharacterId::Duke,
        CharacterId::C,
        CharacterId::Python,
    ] {
        let mut world = close_world(character);
        let guard = FighterInput {
            block: true,
            crouch: character == CharacterId::Python,
            ..NONE
        };
        world.update(
            DT,
            FighterInput {
                signature_special: true,
                ..NONE
            },
            guard,
        );
        for _ in 0..60 {
            if world.player_two.in_blockstun() {
                break;
            }
            world.update(DT, NONE, guard);
        }
        assert!(world.player_two.in_blockstun());
        for _ in 0..60 {
            if !world.player_two.in_blockstun() {
                break;
            }
            world.update(DT, NONE, guard);
        }
        assert!(!world.player_two.in_blockstun());
        world.update(
            DT,
            NONE,
            FighterInput {
                light_punch: true,
                ..NONE
            },
        );
        for _ in 0..5 {
            world.update(DT, NONE, NONE);
        }
        assert!(
            world.player_one.health < world.player_one.max_health,
            "{character:?} recovery should lose to immediate close jab"
        );
    }
}
