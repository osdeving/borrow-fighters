//! Exercises testable greybox combat rules without opening a Raylib window.

use borrow_fighters::combat::fighter::{
    FighterInput, HEAVY_PUNCH_DAMAGE, KICK_DAMAGE, LIGHT_PUNCH_DAMAGE,
};
use borrow_fighters::combat::projectile::{PROJECTILE_DAMAGE, PROJECTILE_SPEED};
use borrow_fighters::game::ai::BasicCpu;
use borrow_fighters::game::feature_flags::{FeatureFlag, FeatureFlags};
use borrow_fighters::game::world::{
    MIN_BODY_GAP, MatchOutcome, SPAWN_INTRO_DURATION_SECONDS, World,
};

const DT: f32 = 1.0 / 60.0;

#[test]
fn basic_attack_deals_damage_once_per_swing() {
    let mut world = World::new_greybox();
    world.player_one.position.x = 420.0;
    world.player_two.position.x = 470.0;

    world.update(
        DT,
        FighterInput {
            light_punch: true,
            ..FighterInput::default()
        },
        FighterInput::default(),
    );

    for _ in 0..20 {
        world.update(DT, FighterInput::default(), FighterInput::default());
    }

    assert_eq!(world.player_two.health, 100 - LIGHT_PUNCH_DAMAGE);
    assert_eq!(world.hit_effects.len(), 1);
}

#[test]
fn heavy_punch_reaches_farther_than_light_punch() {
    let mut world = World::new_greybox();
    world.player_one.position.x = 390.0;
    world.player_two.position.x = 540.0;

    world.update(
        DT,
        FighterInput {
            light_punch: true,
            ..FighterInput::default()
        },
        FighterInput::default(),
    );

    for _ in 0..20 {
        world.update(DT, FighterInput::default(), FighterInput::default());
    }

    assert_eq!(world.player_two.health, 100);

    let mut world = World::new_greybox();
    world.player_one.position.x = 390.0;
    world.player_two.position.x = 540.0;

    world.update(
        DT,
        FighterInput {
            heavy_punch: true,
            ..FighterInput::default()
        },
        FighterInput::default(),
    );

    for _ in 0..24 {
        world.update(DT, FighterInput::default(), FighterInput::default());
    }

    assert_eq!(world.player_two.health, 100 - HEAVY_PUNCH_DAMAGE);
}

#[test]
fn kick_has_its_own_damage() {
    let mut world = World::new_greybox();
    world.player_one.position.x = 420.0;
    world.player_two.position.x = 475.0;

    world.update(
        DT,
        FighterInput {
            kick: true,
            ..FighterInput::default()
        },
        FighterInput::default(),
    );

    for _ in 0..24 {
        world.update(DT, FighterInput::default(), FighterInput::default());
    }

    assert_eq!(world.player_two.health, 100 - KICK_DAMAGE);
}

#[test]
fn block_reduces_incoming_damage() {
    let mut world = World::new_greybox();
    world.player_one.position.x = 420.0;
    world.player_two.position.x = 475.0;

    world.update(
        DT,
        FighterInput {
            light_punch: true,
            ..FighterInput::default()
        },
        FighterInput {
            block: true,
            ..FighterInput::default()
        },
    );

    for _ in 0..20 {
        world.update(
            DT,
            FighterInput::default(),
            FighterInput {
                block: true,
                ..FighterInput::default()
            },
        );
    }

    assert_eq!(world.player_two.health, 100 - LIGHT_PUNCH_DAMAGE / 4);
    assert_eq!(world.hit_effects.len(), 1);
    assert!(world.hit_effects[0].blocked);
}

#[test]
fn player_one_damage_flag_prevents_damage_from_attacks() {
    let mut flags = FeatureFlags::default();
    flags.set(FeatureFlag::PlayerOneTakesDamage, false);
    let mut world = World::new_greybox();
    world.player_one.position.x = 420.0;
    world.player_two.position.x = 475.0;

    world.update_with_flags(
        DT,
        FighterInput::default(),
        FighterInput {
            light_punch: true,
            ..FighterInput::default()
        },
        flags,
    );

    for _ in 0..20 {
        world.update_with_flags(DT, FighterInput::default(), FighterInput::default(), flags);
    }

    assert_eq!(world.player_one.health, 100);
    assert_eq!(world.hit_effects.len(), 1);
    assert_eq!(world.hit_effects[0].damage, 0);
    assert_eq!(world.outcome, None);
}

#[test]
fn crouch_reduces_the_vulnerable_body_height() {
    let mut world = World::new_greybox();
    let standing_height = world.player_one.hurtbox().height;

    world.update(
        DT,
        FighterInput {
            crouch: true,
            ..FighterInput::default()
        },
        FighterInput::default(),
    );

    assert!(world.player_one.crouching);
    assert!(world.player_one.hurtbox().height < standing_height);
}

#[test]
fn match_ends_when_health_reaches_zero() {
    let mut world = World::new_greybox();
    world.player_two.health = LIGHT_PUNCH_DAMAGE;
    world.player_one.position.x = 420.0;
    world.player_two.position.x = 470.0;

    world.update(
        DT,
        FighterInput {
            light_punch: true,
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
            light_punch: true,
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

#[test]
fn diagonal_jump_keeps_horizontal_momentum() {
    let mut world = World::new_greybox();

    world.update(
        DT,
        FighterInput {
            right: true,
            jump: true,
            ..FighterInput::default()
        },
        FighterInput::default(),
    );

    assert!(!world.player_one.grounded);
    assert!(world.player_one.velocity.x > 0.0);
    assert!(world.player_one.velocity.y < 0.0);
}

#[test]
fn projectile_deals_damage_and_disappears() {
    let mut world = World::new_greybox();
    world.player_one.position.x = 300.0;
    world.player_two.position.x = 560.0;

    world.update(
        DT,
        FighterInput {
            projectile: true,
            ..FighterInput::default()
        },
        FighterInput::default(),
    );

    assert_eq!(world.projectiles.len(), 1);
    assert_eq!(world.projectiles[0].velocity.x.abs(), PROJECTILE_SPEED);

    for _ in 0..45 {
        world.update(DT, FighterInput::default(), FighterInput::default());
    }

    assert_eq!(world.player_two.health, 100 - PROJECTILE_DAMAGE);
    assert!(world.projectiles.is_empty());
}

#[test]
fn spawn_intro_blocks_gameplay_until_finished() {
    let mut world = World::new_greybox_with_intro();

    assert!(world.spawn_intro_active());
    world.update(
        DT,
        FighterInput {
            projectile: true,
            ..FighterInput::default()
        },
        FighterInput::default(),
    );

    assert!(world.projectiles.is_empty());
    assert!(world.spawn_intro_elapsed_seconds() > 0.0);

    let intro_steps = (SPAWN_INTRO_DURATION_SECONDS / DT).ceil() as usize + 1;
    for _ in 0..intro_steps {
        world.update(DT, FighterInput::default(), FighterInput::default());
    }

    assert!(!world.spawn_intro_active());
    world.update(
        DT,
        FighterInput {
            projectile: true,
            ..FighterInput::default()
        },
        FighterInput::default(),
    );

    assert_eq!(world.projectiles.len(), 1);
}

#[test]
fn projectile_cooldown_prevents_immediate_spam() {
    let mut world = World::new_greybox();
    world.player_two.position.x = 820.0;

    world.update(
        DT,
        FighterInput {
            projectile: true,
            ..FighterInput::default()
        },
        FighterInput::default(),
    );
    world.update(
        DT,
        FighterInput {
            projectile: true,
            ..FighterInput::default()
        },
        FighterInput::default(),
    );

    assert_eq!(world.projectiles.len(), 1);
}

#[test]
fn basic_cpu_moves_toward_player_when_far() {
    let world = World::new_greybox();
    let mut cpu = BasicCpu::default();

    let input = cpu.next_player_two_input(&world, DT);

    assert!(input.left);
    assert!(!input.right);
}

#[test]
fn basic_cpu_can_drive_player_one_toward_player_two() {
    let world = World::new_greybox();
    let mut cpu = BasicCpu::default();

    let input = cpu.next_input(&world, world.player_one.slot, DT);

    assert!(input.right);
    assert!(!input.left);
}

#[test]
fn basic_cpu_attacks_when_close() {
    let mut world = World::new_greybox();
    world.player_one.position.x = 520.0;
    world.player_two.position.x = 580.0;
    let mut cpu = BasicCpu::default();

    let immediate = cpu.next_player_two_input(&world, DT);
    assert!(!cpu_is_attacking(immediate));

    let mut attacked = false;
    for _ in 0..40 {
        attacked |= cpu_is_attacking(cpu.next_player_two_input(&world, DT));
    }

    assert!(attacked);
}

#[test]
fn basic_cpu_blocks_incoming_projectile() {
    let mut world = World::new_greybox();
    world.player_one.position.x = 500.0;
    world.player_two.position.x = 650.0;
    world
        .projectiles
        .push(borrow_fighters::combat::projectile::Projectile::from_fighter(&world.player_one));
    let mut cpu = BasicCpu::default();

    let input = cpu.next_player_two_input(&world, DT);

    assert!(input.block);
    assert!(!input.light_punch);
    assert!(!input.heavy_punch);
    assert!(!input.kick);
}

#[test]
fn player_one_cpu_blocks_incoming_projectile() {
    let mut world = World::new_greybox();
    world.player_one.position.x = 500.0;
    world.player_two.position.x = 650.0;
    world
        .projectiles
        .push(borrow_fighters::combat::projectile::Projectile::from_fighter(&world.player_two));
    let mut cpu = BasicCpu::default();

    let input = cpu.next_input(&world, world.player_one.slot, DT);

    assert!(input.block);
    assert!(!input.light_punch);
    assert!(!input.heavy_punch);
    assert!(!input.kick);
}

fn cpu_is_attacking(input: FighterInput) -> bool {
    input.light_punch || input.heavy_punch || input.kick
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
