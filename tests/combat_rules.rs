//! Exercises testable greybox combat rules without opening a Raylib window.

use borrow_fighters::audio::AudioCue;
use borrow_fighters::characters::{CharacterId, character_spec};
use borrow_fighters::combat::fighter::{
    AttackKind, AttackPhase, Fighter, FighterInput, GuardRule, HEAVY_PUNCH_DAMAGE, KICK_DAMAGE,
    PlayerSlot, RUST_BORROW_JAB_DAMAGE,
};
use borrow_fighters::combat::move_data::{LIGHT_ATTACK_REACTION, MoveId, move_spec};
use borrow_fighters::combat::projectile::{
    DUKE_PROJECTILE_SPEC, GO_PROJECTILE_SPEC, PROJECTILE_DAMAGE, PROJECTILE_GUARD_RULE,
    PROJECTILE_HIT_REACTION, PROJECTILE_SPEED,
};
use borrow_fighters::engine::sprites::SpriteManifest;
use borrow_fighters::game::ai::BasicCpu;
use borrow_fighters::game::combat_log::CombatLogKind;
use borrow_fighters::game::feature_flags::{FeatureFlag, FeatureFlags};
use borrow_fighters::game::world::{
    MIN_BODY_GAP, MatchOutcome, ROUND_COUNTDOWN_STEP_SECONDS, ROUND_COUNTDOWN_TOTAL_SECONDS,
    SPAWN_INTRO_DURATION_SECONDS, World, WorldSpriteCombatManifests,
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
        player_two_health - RUST_BORROW_JAB_DAMAGE
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
fn sprite_frame_hitbox_metadata_can_drive_close_hit_resolution() {
    let mut world = World::new_greybox();
    let player_two_health = world.player_two.max_health;
    world.player_one.position.x = 200.0;
    world.player_two.position.x = 600.0;
    world.set_sprite_combat_manifests(WorldSpriteCombatManifests {
        player_one: Some(sprite_combat_manifest(
            r#""hitboxes": [{ "x": 380, "y": 30, "w": 80, "h": 60, "label": "long_sprite_strike" }]"#,
        )),
        player_two: None,
    });

    world.update(
        DT,
        FighterInput {
            light_punch: true,
            ..FighterInput::default()
        },
        FighterInput::default(),
    );

    for _ in 0..8 {
        world.update(DT, FighterInput::default(), FighterInput::default());
    }

    assert_eq!(
        world.player_two.health,
        player_two_health - RUST_BORROW_JAB_DAMAGE
    );
}

#[test]
fn sprite_projectile_origin_metadata_overrides_default_spawn_position() {
    let mut world = World::new_greybox();
    world.player_one.position.x = 200.0;
    world.player_two.position.x = 850.0;
    world.set_sprite_combat_manifests(WorldSpriteCombatManifests {
        player_one: Some(sprite_combat_manifest(
            r#""projectile_origin": { "x": 90, "y": 44 }"#,
        )),
        player_two: None,
    });

    world.update(
        DT,
        FighterInput {
            projectile: true,
            ..FighterInput::default()
        },
        FighterInput::default(),
    );

    let projectile = world.projectiles.first().expect("projectile should spawn");
    let body = world.player_one.body_rect();
    let expected_origin_x = body.center_x() - 50.0 + 90.0;
    let expected_origin_y = body.bottom() - 120.0 + 44.0;
    assert_eq!(
        projectile.position.x,
        expected_origin_x + projectile.velocity.x * DT
    );
    assert_eq!(
        projectile.position.y,
        expected_origin_y - projectile.height * 0.5
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
fn close_attack_queues_audio_events_for_start_and_hit() {
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

    let events = world.take_audio_events();
    assert!(events.iter().any(|event| {
        event.cue == AudioCue::FighterAttackStart
            && event.character == Some(CharacterId::Rust)
            && event.move_id == Some(MoveId::RustBorrowJab)
    }));

    for _ in 0..20 {
        world.update(DT, FighterInput::default(), FighterInput::default());
    }

    let events = world.take_audio_events();
    assert!(events.iter().any(|event| event.cue == AudioCue::CombatHit));
    assert!(events.iter().any(|event| {
        event.cue == AudioCue::FighterHurt && event.character == Some(CharacterId::Duke)
    }));
}

#[test]
fn combat_log_records_round_close_attack_hit_and_whiff() {
    let mut world = World::new_greybox();
    assert!(world.combat_log().iter().any(|event| {
        matches!(
            event.kind,
            CombatLogKind::RoundStarted {
                player_one: CharacterId::Rust,
                player_two: CharacterId::Duke
            }
        )
    }));

    world.clear_combat_log();
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

    assert!(world.combat_log().iter().any(|event| {
        matches!(
            event.kind,
            CombatLogKind::CloseAttackStarted {
                slot: PlayerSlot::One,
                character: CharacterId::Rust,
                move_id: MoveId::RustBorrowJab
            }
        )
    }));

    for _ in 0..20 {
        world.update(DT, FighterInput::default(), FighterInput::default());
    }

    assert!(world.combat_log().iter().any(|event| {
        matches!(
            event.kind,
            CombatLogKind::CloseAttackResolved {
                attacker: PlayerSlot::One,
                defender: PlayerSlot::Two,
                attacker_character: CharacterId::Rust,
                defender_character: CharacterId::Duke,
                move_id: MoveId::RustBorrowJab,
                damage: RUST_BORROW_JAB_DAMAGE,
                blocked: false,
            }
        )
    }));

    let mut whiff_world = World::new_greybox();
    whiff_world.clear_combat_log();
    whiff_world.player_one.position.x = 300.0;
    whiff_world.player_two.position.x = 760.0;
    whiff_world.update(
        DT,
        FighterInput {
            light_punch: true,
            ..FighterInput::default()
        },
        FighterInput::default(),
    );

    for _ in 0..30 {
        whiff_world.update(DT, FighterInput::default(), FighterInput::default());
    }

    assert!(whiff_world.combat_log().iter().any(|event| {
        matches!(
            event.kind,
            CombatLogKind::CloseAttackWhiffed {
                slot: PlayerSlot::One,
                character: CharacterId::Rust,
                move_id: MoveId::RustBorrowJab
            }
        )
    }));
}

#[test]
fn blocked_close_attack_queues_guard_audio_events() {
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

    let events = world.take_audio_events();
    assert!(
        events
            .iter()
            .any(|event| event.cue == AudioCue::CombatBlock)
    );
    assert!(events.iter().any(|event| {
        event.cue == AudioCue::FighterBlock && event.character == Some(CharacterId::Duke)
    }));
}

#[test]
fn no_damage_flag_suppresses_hurt_audio_event() {
    let mut world = World::new_greybox();
    world.player_one.position.x = 420.0;
    world.player_two.position.x = 470.0;
    let mut flags = FeatureFlags::default();
    flags.set(FeatureFlag::PlayerTwoTakesDamage, false);

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

    let events = world.take_audio_events();
    assert!(events.iter().any(|event| event.cue == AudioCue::CombatHit));
    assert!(
        events
            .iter()
            .all(|event| event.cue != AudioCue::FighterHurt)
    );
}

#[test]
fn projectile_cast_queues_audio_event() {
    let mut world = World::new_greybox();

    world.update(
        DT,
        FighterInput {
            projectile: true,
            ..FighterInput::default()
        },
        FighterInput::default(),
    );

    let events = world.take_audio_events();
    assert!(events.iter().any(|event| {
        event.cue == AudioCue::FighterProjectileCast && event.character == Some(CharacterId::Rust)
    }));
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
        player_two_health - RUST_BORROW_JAB_DAMAGE / 4
    );
    assert_eq!(world.hit_effects.len(), 1);
    assert!(world.hit_effects[0].blocked);
}

#[test]
fn guard_rule_controls_blockability_and_reaction() {
    let mut defender = Fighter::new(PlayerSlot::Two, "Guard", 500.0);
    defender.update(
        DT,
        FighterInput {
            block: true,
            ..FighterInput::default()
        },
    );

    let blocked = defender.take_hit(20, GuardRule::Mid, LIGHT_ATTACK_REACTION);
    assert_eq!(blocked.damage, 5);
    assert!(blocked.blocked);
    assert_eq!(blocked.pushback, LIGHT_ATTACK_REACTION.block_pushback);
    assert!(defender.in_blockstun());
    assert!(!defender.in_hitstun());

    let mut defender = Fighter::new(PlayerSlot::Two, "Throw Target", 500.0);
    defender.update(
        DT,
        FighterInput {
            block: true,
            crouch: true,
            ..FighterInput::default()
        },
    );

    let thrown = defender.take_hit(20, GuardRule::Throw, LIGHT_ATTACK_REACTION);
    assert_eq!(thrown.damage, 20);
    assert!(!thrown.blocked);
    assert_eq!(thrown.pushback, LIGHT_ATTACK_REACTION.hit_pushback);
    assert!(defender.in_hitstun());
}

#[test]
fn hitstun_and_blockstun_lock_out_actions_temporarily() {
    let mut hit = Fighter::new(PlayerSlot::Two, "Hit", 500.0);
    hit.take_hit(10, GuardRule::Mid, LIGHT_ATTACK_REACTION);
    assert!(hit.in_hitstun());
    assert_eq!(
        hit.hitstun_remaining_frames(),
        LIGHT_ATTACK_REACTION.hitstun
    );

    hit.update(
        DT,
        FighterInput {
            heavy_punch: true,
            projectile: true,
            right: true,
            ..FighterInput::default()
        },
    );

    assert_eq!(hit.attack_kind(), None);
    assert!(!hit.can_fire_projectile());
    assert!(hit.velocity.x.abs() < 0.01);

    for _ in 0..LIGHT_ATTACK_REACTION.hitstun.get() {
        hit.update(DT, FighterInput::default());
    }

    assert!(!hit.in_hitstun());
    hit.update(
        DT,
        FighterInput {
            heavy_punch: true,
            ..FighterInput::default()
        },
    );
    assert_eq!(hit.attack_kind(), Some(AttackKind::HeavyPunch));

    let mut blocked = Fighter::new(PlayerSlot::Two, "Block", 500.0);
    blocked.update(
        DT,
        FighterInput {
            block: true,
            ..FighterInput::default()
        },
    );
    blocked.take_hit(10, GuardRule::Mid, LIGHT_ATTACK_REACTION);

    assert!(blocked.in_blockstun());
    assert!(blocked.blocking);
    blocked.update(
        DT,
        FighterInput {
            light_punch: true,
            ..FighterInput::default()
        },
    );
    assert_eq!(blocked.attack_kind(), None);
}

#[test]
fn whiff_recovery_locks_out_actions_after_missing() {
    let spec = move_spec(MoveId::HeavyPunch);
    let mut fighter = Fighter::new(PlayerSlot::One, "Whiff", 300.0);

    fighter.update(
        DT,
        FighterInput {
            heavy_punch: true,
            ..FighterInput::default()
        },
    );

    while fighter.attack_elapsed_frames().is_some() {
        fighter.update(DT, FighterInput::default());
    }

    assert!(fighter.in_whiff_recovery());
    assert_eq!(fighter.attack_phase(), AttackPhase::WhiffRecovery);
    assert_eq!(
        fighter.whiff_recovery_remaining_frames(),
        spec.whiff_recovery
    );

    fighter.update(
        DT,
        FighterInput {
            light_punch: true,
            projectile: true,
            right: true,
            ..FighterInput::default()
        },
    );

    assert_eq!(fighter.attack_kind(), None);
    assert!(!fighter.can_fire_projectile());
    assert!(fighter.velocity.x.abs() < 0.01);

    for _ in 0..spec.whiff_recovery.get() {
        fighter.update(DT, FighterInput::default());
    }

    assert!(!fighter.in_whiff_recovery());
    fighter.update(
        DT,
        FighterInput {
            light_punch: true,
            ..FighterInput::default()
        },
    );
    assert_eq!(fighter.attack_kind(), Some(AttackKind::LightPunch));
}

#[test]
fn projectile_guard_rule_blocks_like_a_projectile() {
    let mut defender = Fighter::new(PlayerSlot::Two, "Projectile Target", 500.0);
    defender.update(
        DT,
        FighterInput {
            block: true,
            ..FighterInput::default()
        },
    );

    let result = defender.take_hit(
        PROJECTILE_DAMAGE,
        PROJECTILE_GUARD_RULE,
        PROJECTILE_HIT_REACTION,
    );

    assert!(result.blocked);
    assert_eq!(result.damage, PROJECTILE_DAMAGE / 4);
    assert_eq!(result.pushback, PROJECTILE_HIT_REACTION.block_pushback);
    assert_eq!(
        defender.blockstun_remaining_frames(),
        PROJECTILE_HIT_REACTION.blockstun
    );
}

#[test]
fn hit_and_block_pushback_move_defender_away_from_attacker() {
    let mut hit_world = World::new_greybox();
    hit_world.player_one.position.x = 380.0;
    hit_world.player_two.position.x = 465.0;
    let hit_start_x = hit_world.player_two.position.x;

    hit_world.update(
        DT,
        FighterInput {
            light_punch: true,
            ..FighterInput::default()
        },
        FighterInput::default(),
    );

    for _ in 0..20 {
        hit_world.update(DT, FighterInput::default(), FighterInput::default());
    }

    let hit_delta = hit_world.player_two.position.x - hit_start_x;
    assert!(hit_delta >= LIGHT_ATTACK_REACTION.hit_pushback);

    let mut block_world = World::new_greybox();
    block_world.player_one.position.x = 380.0;
    block_world.player_two.position.x = 465.0;
    let block_start_x = block_world.player_two.position.x;

    block_world.update(
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
        block_world.update(
            DT,
            FighterInput::default(),
            FighterInput {
                block: true,
                ..FighterInput::default()
            },
        );
    }

    let block_delta = block_world.player_two.position.x - block_start_x;
    assert!(block_delta >= LIGHT_ATTACK_REACTION.block_pushback);
    assert!(
        hit_delta > block_delta,
        "hit pushback should be larger than block pushback"
    );
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
fn greybox_intro_world_can_use_explicit_match_characters() {
    let world = World::new_greybox_with_intro_for_characters(CharacterId::Go, CharacterId::Duke);

    assert_eq!(world.player_one_character(), CharacterId::Go);
    assert_eq!(world.player_two_character(), CharacterId::Duke);
    assert_eq!(
        world.player_one.name,
        character_spec(CharacterId::Go).fighter_name
    );
    assert_eq!(
        world.player_one.max_health,
        character_spec(CharacterId::Go).stats.max_health
    );
    assert!(world.spawn_intro_active());
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
    world.player_two.health = RUST_BORROW_JAB_DAMAGE;
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
    assert!(world.combat_log().iter().any(|event| {
        matches!(
            event.kind,
            CombatLogKind::MatchEnded {
                winner: Some(PlayerSlot::One),
                winner_character: Some(CharacterId::Rust)
            }
        )
    }));
}

#[test]
fn close_attack_tuning_comes_from_character_loadout() {
    let mut rust = Fighter::new_with_loadout(
        PlayerSlot::One,
        "Rust",
        100,
        character_spec(CharacterId::Rust).move_ids,
        300.0,
    );

    rust.update(
        DT,
        FighterInput {
            light_punch: true,
            ..FighterInput::default()
        },
    );

    assert_eq!(rust.attack_kind(), Some(AttackKind::LightPunch));
    assert_eq!(
        rust.attack_move_spec().map(|spec| spec.id),
        Some(MoveId::RustBorrowJab)
    );

    let mut duke = Fighter::new_with_loadout(
        PlayerSlot::Two,
        "Java",
        112,
        character_spec(CharacterId::Duke).move_ids,
        500.0,
    );

    duke.update(
        DT,
        FighterInput {
            heavy_punch: true,
            ..FighterInput::default()
        },
    );

    assert_eq!(duke.attack_kind(), Some(AttackKind::HeavyPunch));
    assert_eq!(
        duke.attack_move_spec().map(|spec| spec.id),
        Some(MoveId::DukeBoilerplatePoke)
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
fn combat_log_records_projectile_spawn_and_hit() {
    let mut world = World::new_greybox();
    world.clear_combat_log();
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

    assert!(world.combat_log().iter().any(|event| {
        matches!(
            event.kind,
            CombatLogKind::ProjectileSpawned {
                slot: PlayerSlot::One,
                character: CharacterId::Rust,
                damage: PROJECTILE_DAMAGE
            }
        )
    }));

    for _ in 0..45 {
        world.update(DT, FighterInput::default(), FighterInput::default());
    }

    assert!(world.combat_log().iter().any(|event| {
        matches!(
            event.kind,
            CombatLogKind::ProjectileResolved {
                attacker: PlayerSlot::One,
                defender: PlayerSlot::Two,
                attacker_character: CharacterId::Rust,
                defender_character: CharacterId::Duke,
                damage: PROJECTILE_DAMAGE,
                blocked: false,
            }
        )
    }));
}

#[test]
fn match_projectiles_use_character_specific_specs() {
    let mut world = World::new_with_characters(CharacterId::Go, CharacterId::Duke);

    world.update(
        DT,
        FighterInput {
            projectile: true,
            ..FighterInput::default()
        },
        FighterInput::default(),
    );

    assert_eq!(world.projectiles.len(), 1);
    assert_eq!(world.projectiles[0].damage, GO_PROJECTILE_SPEC.damage);
    assert_eq!(world.projectiles[0].rect().width, GO_PROJECTILE_SPEC.width);
    assert_eq!(
        world.projectiles[0].velocity.x.abs(),
        GO_PROJECTILE_SPEC.speed
    );
    assert_eq!(
        world.player_one.projectile_cooldown_remaining_frames(),
        GO_PROJECTILE_SPEC.frame_data.cooldown
    );

    let mut world = World::new_with_characters(CharacterId::Go, CharacterId::Duke);
    world.update(
        DT,
        FighterInput::default(),
        FighterInput {
            projectile: true,
            ..FighterInput::default()
        },
    );

    assert_eq!(world.projectiles.len(), 1);
    assert_eq!(world.projectiles[0].damage, DUKE_PROJECTILE_SPEC.damage);
    assert_eq!(
        world.projectiles[0].velocity.x.abs(),
        DUKE_PROJECTILE_SPEC.speed
    );
    assert_eq!(
        world.player_two.projectile_cooldown_remaining_frames(),
        DUKE_PROJECTILE_SPEC.frame_data.cooldown
    );
}

#[test]
fn spawn_intro_and_countdown_block_gameplay_until_finished() {
    let mut world = World::new_greybox_with_intro();

    assert!(world.spawn_intro_active());
    assert!(!world.countdown_active());
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
    assert!(world.countdown_active());
    assert_eq!(world.countdown_label(), Some("11"));
    world.update(
        DT,
        FighterInput {
            projectile: true,
            ..FighterInput::default()
        },
        FighterInput::default(),
    );

    assert!(world.projectiles.is_empty());
    assert!(
        world
            .take_audio_events()
            .iter()
            .any(|event| event.cue == AudioCue::MatchCountdownEleven)
    );

    let countdown_steps = (ROUND_COUNTDOWN_TOTAL_SECONDS / DT).ceil() as usize + 1;
    for _ in 0..countdown_steps {
        world.update(DT, FighterInput::default(), FighterInput::default());
    }

    assert!(!world.countdown_active());
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
fn countdown_advances_labels_and_audio_events() {
    let mut world = World::new_greybox_with_intro();
    let intro_steps = (SPAWN_INTRO_DURATION_SECONDS / DT).ceil() as usize + 1;
    for _ in 0..intro_steps {
        world.update(DT, FighterInput::default(), FighterInput::default());
    }

    for (label, cue) in [
        ("11", AudioCue::MatchCountdownEleven),
        ("10", AudioCue::MatchCountdownTen),
        ("01", AudioCue::MatchCountdownOne),
        ("Fight!", AudioCue::MatchCountdownFight),
    ] {
        assert_eq!(world.countdown_label(), Some(label));
        world.update(DT, FighterInput::default(), FighterInput::default());
        assert!(
            world
                .take_audio_events()
                .iter()
                .any(|event| event.cue == cue),
            "expected countdown cue {cue:?}"
        );

        let step_steps = (ROUND_COUNTDOWN_STEP_SECONDS / DT).ceil() as usize;
        for _ in 1..=step_steps {
            world.update(DT, FighterInput::default(), FighterInput::default());
        }
    }

    assert!(!world.countdown_active());
    assert_eq!(world.countdown_label(), None);
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

fn sprite_combat_manifest(combat_body: &str) -> SpriteManifest {
    let json = format!(
        r#"{{
  "schema": "borrow-fighters.sprite.v1",
  "image": "missing.png",
  "cell": {{ "w": 500, "h": 120 }},
  "default_pivot": {{ "x": 50, "y": 120 }},
  "frames": [
    {{
      "name": "punch_light_0",
      "clip": "punch_light",
      "duration_ms": 1000,
      "pivot": {{ "x": 50, "y": 120 }},
      "frame": {{ "x": 0, "y": 0, "w": 500, "h": 120 }},
      "combat": {{ {combat_body} }}
    }},
    {{
      "name": "special_0",
      "clip": "special",
      "duration_ms": 1000,
      "pivot": {{ "x": 50, "y": 120 }},
      "frame": {{ "x": 0, "y": 0, "w": 500, "h": 120 }},
      "combat": {{ {combat_body} }}
    }}
  ],
  "clips": [
    {{ "name": "punch_light", "loop": false, "frames": ["punch_light_0"] }},
    {{ "name": "special", "loop": false, "frames": ["special_0"] }}
  ]
}}"#
    );
    let manifest: SpriteManifest = serde_json::from_str(&json).unwrap();
    manifest.validate().unwrap();
    manifest
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
