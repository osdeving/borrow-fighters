//! Verifies table-driven close-range move data.

use borrow_fighters::combat::fighter::{
    AttackKind, DUKE_BOILERPLATE_POKE_DAMAGE, HEAVY_PUNCH_DAMAGE, KICK_DAMAGE, LIGHT_PUNCH_DAMAGE,
    RUST_BORROW_JAB_DAMAGE,
};
use borrow_fighters::combat::frame::FrameCount;
use borrow_fighters::combat::move_data::{
    CLOSE_RANGE_MOVE_SPECS, DUKE_BOILERPLATE_POKE_WHIFF_RECOVERY, GuardRule, HEAVY_ATTACK_REACTION,
    HEAVY_ATTACK_WHIFF_RECOVERY, KICK_REACTION, KICK_WHIFF_RECOVERY, LIGHT_ATTACK_REACTION,
    LIGHT_ATTACK_WHIFF_RECOVERY, MoveId, MoveInputKind, RUST_BORROW_JAB_WHIFF_RECOVERY, move_spec,
    move_spec_for_input,
};

#[test]
fn close_range_moves_are_registered_in_table_order() {
    assert_eq!(CLOSE_RANGE_MOVE_SPECS.len(), 5);
    assert_eq!(CLOSE_RANGE_MOVE_SPECS[0].id, MoveId::LightPunch);
    assert_eq!(CLOSE_RANGE_MOVE_SPECS[1].id, MoveId::HeavyPunch);
    assert_eq!(CLOSE_RANGE_MOVE_SPECS[2].id, MoveId::Kick);
    assert_eq!(CLOSE_RANGE_MOVE_SPECS[3].id, MoveId::RustBorrowJab);
    assert_eq!(CLOSE_RANGE_MOVE_SPECS[4].id, MoveId::DukeBoilerplatePoke);
}

#[test]
fn move_specs_preserve_current_tuning_values() {
    let light = move_spec(MoveId::LightPunch);
    assert_eq!(light.input, MoveInputKind::LightPunch);
    assert_eq!(light.label, "LP");
    assert_eq!(light.damage, LIGHT_PUNCH_DAMAGE);
    assert_eq!(light.frames.duration, FrameCount::new(18));
    assert_eq!(light.frames.active_start, FrameCount::new(5));
    assert_eq!(light.frames.active_end, FrameCount::new(10));
    assert_eq!(light.hitbox.width, 58.0);
    assert_eq!(light.hitbox.height, 34.0);
    assert_eq!(light.hitbox.y_offset, 62.0);
    assert_eq!(light.guard_rule, GuardRule::Mid);
    assert_eq!(light.hit_reaction, LIGHT_ATTACK_REACTION);
    assert_eq!(light.hit_reaction.hit_pushback, 22.0);
    assert_eq!(light.hit_reaction.block_pushback, 14.0);
    assert_eq!(light.whiff_recovery, LIGHT_ATTACK_WHIFF_RECOVERY);

    let heavy = move_spec(MoveId::HeavyPunch);
    assert_eq!(heavy.input, MoveInputKind::HeavyPunch);
    assert_eq!(heavy.label, "HP");
    assert_eq!(heavy.damage, HEAVY_PUNCH_DAMAGE);
    assert_eq!(heavy.frames.duration, FrameCount::new(35));
    assert_eq!(heavy.frames.active_start, FrameCount::new(11));
    assert_eq!(heavy.frames.active_end, FrameCount::new(20));
    assert_eq!(heavy.hitbox.width, 96.0);
    assert_eq!(heavy.hitbox.height, 42.0);
    assert_eq!(heavy.hitbox.y_offset, 58.0);
    assert_eq!(heavy.guard_rule, GuardRule::Mid);
    assert_eq!(heavy.hit_reaction, HEAVY_ATTACK_REACTION);
    assert_eq!(heavy.hit_reaction.hit_pushback, 38.0);
    assert_eq!(heavy.hit_reaction.block_pushback, 26.0);
    assert_eq!(heavy.whiff_recovery, HEAVY_ATTACK_WHIFF_RECOVERY);

    let kick = move_spec(MoveId::Kick);
    assert_eq!(kick.input, MoveInputKind::Kick);
    assert_eq!(kick.label, "KICK");
    assert_eq!(kick.damage, KICK_DAMAGE);
    assert_eq!(kick.frames.duration, FrameCount::new(28));
    assert_eq!(kick.frames.active_start, FrameCount::new(9));
    assert_eq!(kick.frames.active_end, FrameCount::new(16));
    assert_eq!(kick.hitbox.width, 100.0);
    assert_eq!(kick.hitbox.height, 36.0);
    assert_eq!(kick.hitbox.y_offset, 108.0);
    assert_eq!(kick.guard_rule, GuardRule::Mid);
    assert_eq!(kick.hit_reaction, KICK_REACTION);
    assert_eq!(kick.hit_reaction.hit_pushback, 34.0);
    assert_eq!(kick.hit_reaction.block_pushback, 22.0);
    assert_eq!(kick.whiff_recovery, KICK_WHIFF_RECOVERY);
}

#[test]
fn character_specific_move_specs_have_distinct_tuning() {
    let rust_jab = move_spec(MoveId::RustBorrowJab);
    assert_eq!(rust_jab.input, MoveInputKind::LightPunch);
    assert_eq!(rust_jab.label, "Borrow Jab");
    assert_eq!(rust_jab.damage, RUST_BORROW_JAB_DAMAGE);
    assert_eq!(rust_jab.frames.duration, FrameCount::new(16));
    assert_eq!(rust_jab.frames.active_start, FrameCount::new(4));
    assert_eq!(rust_jab.frames.active_end, FrameCount::new(8));
    assert_eq!(rust_jab.hitbox.width, 48.0);
    assert_eq!(rust_jab.hitbox.height, 30.0);
    assert_eq!(rust_jab.hitbox.y_offset, 62.0);
    assert_eq!(rust_jab.guard_rule, GuardRule::Mid);
    assert_eq!(rust_jab.hit_reaction, LIGHT_ATTACK_REACTION);
    assert_eq!(rust_jab.whiff_recovery, RUST_BORROW_JAB_WHIFF_RECOVERY);

    let duke_poke = move_spec(MoveId::DukeBoilerplatePoke);
    assert_eq!(duke_poke.input, MoveInputKind::HeavyPunch);
    assert_eq!(duke_poke.label, "Boilerplate");
    assert_eq!(duke_poke.damage, DUKE_BOILERPLATE_POKE_DAMAGE);
    assert_eq!(duke_poke.frames.duration, FrameCount::new(40));
    assert_eq!(duke_poke.frames.active_start, FrameCount::new(13));
    assert_eq!(duke_poke.frames.active_end, FrameCount::new(22));
    assert_eq!(duke_poke.hitbox.width, 112.0);
    assert_eq!(duke_poke.hitbox.height, 44.0);
    assert_eq!(duke_poke.hitbox.y_offset, 60.0);
    assert_eq!(duke_poke.guard_rule, GuardRule::Mid);
    assert_eq!(duke_poke.hit_reaction, HEAVY_ATTACK_REACTION);
    assert_eq!(
        duke_poke.whiff_recovery,
        DUKE_BOILERPLATE_POKE_WHIFF_RECOVERY
    );
}

#[test]
fn guard_rules_describe_current_blocking_model() {
    assert!(GuardRule::Mid.is_blocked_by(true, false));
    assert!(GuardRule::High.is_blocked_by(true, false));
    assert!(GuardRule::Projectile.is_blocked_by(true, false));
    assert!(GuardRule::Low.is_blocked_by(true, true));
    assert!(!GuardRule::Low.is_blocked_by(true, false));
    assert!(!GuardRule::Throw.is_blocked_by(true, true));
    assert!(!GuardRule::Mid.is_blocked_by(false, false));
}

#[test]
fn loadout_move_selection_prefers_character_specific_specs() {
    let rust_moves = [MoveId::RustBorrowJab, MoveId::HeavyPunch, MoveId::Kick];
    let duke_moves = [
        MoveId::LightPunch,
        MoveId::DukeBoilerplatePoke,
        MoveId::Kick,
    ];

    assert_eq!(
        move_spec_for_input(&rust_moves, MoveInputKind::LightPunch),
        Some(move_spec(MoveId::RustBorrowJab))
    );
    assert_eq!(
        move_spec_for_input(&duke_moves, MoveInputKind::HeavyPunch),
        Some(move_spec(MoveId::DukeBoilerplatePoke))
    );
}

#[test]
fn attack_kind_is_compatibility_layer_over_move_specs() {
    assert_eq!(AttackKind::LightPunch.move_id(), MoveId::LightPunch);
    assert_eq!(
        AttackKind::from_move_id(MoveId::RustBorrowJab),
        AttackKind::LightPunch
    );
    assert_eq!(
        AttackKind::from_move_id(MoveId::DukeBoilerplatePoke),
        AttackKind::HeavyPunch
    );
    assert_eq!(
        AttackKind::LightPunch.move_spec(),
        move_spec(MoveId::LightPunch)
    );
    assert_eq!(
        AttackKind::HeavyPunch.move_spec(),
        move_spec(MoveId::HeavyPunch)
    );
    assert_eq!(AttackKind::Kick.move_spec(), move_spec(MoveId::Kick));
}
