//! Exercises testable greybox combat rules without opening a Raylib window.

use borrow_fighters::characters::{CharacterId, character_spec};
use borrow_fighters::combat::fighter::{
    AttackKind, Fighter, FighterInput, HEAVY_PUNCH_DAMAGE, KICK_DAMAGE, LIGHT_PUNCH_DAMAGE,
    PlayerSlot,
};
use borrow_fighters::combat::move_data::MoveId;
use borrow_fighters::combat::projectile::{PROJECTILE_DAMAGE, PROJECTILE_SPEED};
use borrow_fighters::game::ai::BasicCpu;
use borrow_fighters::game::feature_flags::{FeatureFlag, FeatureFlags};
use borrow_fighters::game::world::{
    MIN_BODY_GAP, MatchOutcome, SPAWN_INTRO_DURATION_SECONDS, World,
};

const DT: f32 = 1.0 / 60.0;
const KICK_ONLY_MOVES: [MoveId; 1] = [MoveId::Kick];

#[test]
fn basic_attack_deals_damage_once_per_swing() {
    let mut world = World::new_greybox();
    let player_two_health = world.player_two.max_health;
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
        world.player_two.health,
        player_two_health - LIGHT_PUNCH_DAMAGE
    );
    assert_eq!(world.hit_effects.len(), 1);
}

#[test]
fn heavy_punch_reaches_farther_than_light_punch() {
    let mut world = World::new_greybox();
    let player_two_health = world.player_two.max_health;
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

    assert_eq!(world.player_two.health, player_two_health);

    let mut world = World::new_greybox();
    let player_two_health = world.player_two.max_health;
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

    assert_eq!(
        world.player_two.health,
        player_two_health - HEAVY_PUNCH_DAMAGE
    );
}

#[test]
fn kick_has_its_own_damage() {
    let mut world = World::new_greybox();
    let player_two_health = world.player_two.max_health;
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

    assert_eq!(world.player_two.health, player_two_health - KICK_DAMAGE);
}

#[test]
fn block_reduces_incoming_damage() {
    let mut world = World::new_greybox();
    let player_two_health = world.player_two.max_health;
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

    assert_eq!(
        world.player_two.health,
        player_two_health - LIGHT_PUNCH_DAMAGE / 4
    );
    assert_eq!(world.hit_effects.len(), 1);
    assert!(world.hit_effects[0].blocked);
}

#[test]
fn greybox_world_uses_character_specs_for_match_setup() {
    let world = World::new_greybox();
    let rust = character_spec(CharacterId::Rust);
    let duke = character_spec(CharacterId::Duke);

    assert_eq!(world.player_one_character(), CharacterId::Rust);
    assert_eq!(world.player_two_character(), CharacterId::Duke);
    assert_eq!(
        world.character_for_slot(world.player_one.slot),
        CharacterId::Rust
    );
    assert_eq!(
        world.character_for_slot(world.player_two.slot),
        CharacterId::Duke
    );
    assert_eq!(world.player_one.name, rust.fighter_name);
    assert_eq!(world.player_two.name, duke.fighter_name);
    assert_eq!(world.player_one.max_health, rust.stats.max_health);
    assert_eq!(world.player_two.max_health, duke.stats.max_health);
    assert_eq!(world.player_one.move_ids(), rust.move_ids);
    assert_eq!(world.player_two.move_ids(), duke.move_ids);
    assert_eq!(world.player_one.health, world.player_one.max_health);
    assert_eq!(world.player_two.health, world.player_two.max_health);
}

#[test]
fn greybox_world_can_swap_character_specs_between_slots() {
    let world = World::new_with_characters(CharacterId::Duke, CharacterId::Rust);

    assert_eq!(world.player_one_character(), CharacterId::Duke);
    assert_eq!(world.player_two_character(), CharacterId::Rust);
    assert_eq!(
        world.player_one.max_health,
        character_spec(CharacterId::Duke).stats.max_health
    );
    assert_eq!(
        world.player_two.max_health,
        character_spec(CharacterId::Rust).stats.max_health
    );
}

#[test]
fn fighter_loadout_blocks_unlisted_close_moves() {
    let mut fighter =
        Fighter::new_with_loadout(PlayerSlot::One, "Test", 100, &KICK_ONLY_MOVES, 300.0);

    fighter.update(
        DT,
        FighterInput {
            light_punch: true,
            ..FighterInput::default()
        },
    );
    assert_eq!(fighter.attack_kind(), None);

    fighter.update(
        DT,
        FighterInput {
            kick: true,
            ..FighterInput::default()
        },
    );
    assert_eq!(fighter.attack_kind(), Some(AttackKind::Kick));
    assert_eq!(fighter.move_ids(), &KICK_ONLY_MOVES);
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

    assert_eq!(world.player_one.health, world.player_one.max_health);
    assert_eq!(world.hit_effects.len(), 1);
    assert_eq!(world.hit_effects[0].damage, 0);
    assert_eq!(world.outcome, None);
}

#[test]
fn player_two_damage_flag_prevents_damage_from_attacks() {
    let mut flags = FeatureFlags::default();
    flags.set(FeatureFlag::PlayerTwoTakesDamage, false);
    let mut world = World::new_greybox();
    world.player_one.position.x = 420.0;
    world.player_two.position.x = 475.0;

    world.update_with_flags(
        DT,
        FighterInput {
            light_punch: true,
            ..FighterInput::default()
        },
        FighterInput::default(),
        flags,
    );

    for _ in 0..20 {
        world.update_with_flags(DT, FighterInput::default(), FighterInput::default(), flags);
    }

    assert_eq!(world.player_two.health, world.player_two.max_health);
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
    let player_two_health = world.player_two.max_health;
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

    assert_eq!(
        world.player_two.health,
        player_two_health - PROJECTILE_DAMAGE
    );
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
fn cpu_slots_use_different_opening_profiles() {
    let world = World::new_greybox();
    let mut player_one_cpu = BasicCpu::for_slot(world.player_one.slot);
    let mut player_two_cpu = BasicCpu::for_slot(world.player_two.slot);

    let player_one = player_one_cpu.next_input(&world, world.player_one.slot, DT);
    let player_two = player_two_cpu.next_input(&world, world.player_two.slot, DT);

    assert_ne!(player_one.jump, player_two.jump);
    assert!(player_one.right);
    assert!(player_two.left);
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

#[test]
fn basic_cpu_varies_movement_attacks_projectiles_and_defense() {
    let mut world = World::new_greybox();
    let mut cpu = BasicCpu::for_slot(world.player_two.slot);
    let mut seen = CpuActionSet::default();

    world.player_one.position.x = 340.0;
    world.player_two.position.x = 650.0;
    for _ in 0..180 {
        seen.observe(cpu.next_input(&world, world.player_two.slot, DT));
    }

    world.player_one.position.x = 520.0;
    world.player_two.position.x = 580.0;
    for _ in 0..180 {
        seen.observe(cpu.next_input(&world, world.player_two.slot, DT));
    }

    world
        .projectiles
        .push(borrow_fighters::combat::projectile::Projectile::from_fighter(&world.player_one));
    for _ in 0..20 {
        seen.observe(cpu.next_input(&world, world.player_two.slot, DT));
    }

    assert!(seen.moved, "CPU should walk or reposition");
    assert!(seen.jumped, "CPU should sometimes jump");
    assert!(seen.close_attack, "CPU should use close attacks");
    assert!(seen.kick, "CPU should visibly use kicks");
    assert!(
        seen.projectile,
        "CPU should sometimes use special/projectile"
    );
    assert!(seen.defense, "CPU should sometimes block, crouch, or evade");
}

fn cpu_is_attacking(input: FighterInput) -> bool {
    input.light_punch || input.heavy_punch || input.kick
}

#[derive(Default)]
struct CpuActionSet {
    moved: bool,
    jumped: bool,
    close_attack: bool,
    kick: bool,
    projectile: bool,
    defense: bool,
}

impl CpuActionSet {
    fn observe(&mut self, input: FighterInput) {
        self.moved |= input.left || input.right;
        self.jumped |= input.jump;
        self.close_attack |= cpu_is_attacking(input);
        self.kick |= input.kick;
        self.projectile |= input.projectile;
        self.defense |= input.block || input.crouch;
    }
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
