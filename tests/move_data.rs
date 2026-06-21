//! Verifies table-driven close-range move data.

use borrow_fighters::combat::fighter::{
    AIR_KICK_DAMAGE, AIR_PUNCH_DAMAGE, AttackKind, CLOSE_THROW_DAMAGE,
    DUKE_BOILERPLATE_POKE_DAMAGE, HEAVY_PUNCH_DAMAGE, KICK_DAMAGE, LIGHT_PUNCH_DAMAGE,
    OVERHEAD_PUNCH_DAMAGE, RISING_ANTI_AIR_DAMAGE, RUST_BORROW_JAB_DAMAGE, SWEEP_KICK_DAMAGE,
};
use borrow_fighters::combat::frame::FrameCount;
use borrow_fighters::combat::move_data::{
    AIR_ATTACK_REACTION, AIR_ATTACK_WHIFF_RECOVERY, CLOSE_RANGE_MOVE_SPECS, CLOSE_THROW_REACTION,
    CLOSE_THROW_WHIFF_RECOVERY, DUKE_BOILERPLATE_POKE_WHIFF_RECOVERY, GuardRule,
    HEAVY_ATTACK_REACTION, HEAVY_ATTACK_WHIFF_RECOVERY, KICK_REACTION, KICK_WHIFF_RECOVERY,
    LIGHT_ATTACK_REACTION, LIGHT_ATTACK_WHIFF_RECOVERY, MoveId, MoveInputKind,
    OVERHEAD_PUNCH_WHIFF_RECOVERY, OVERHEAD_REACTION, RISING_ANTI_AIR_REACTION,
    RISING_ANTI_AIR_WHIFF_RECOVERY, RUST_BORROW_JAB_WHIFF_RECOVERY, SWEEP_KICK_WHIFF_RECOVERY,
    SWEEP_REACTION, move_spec, move_spec_for_input,
};

#[test]
fn close_range_moves_are_registered_in_table_order() {
    assert_eq!(CLOSE_RANGE_MOVE_SPECS.len(), 11);
    assert_eq!(CLOSE_RANGE_MOVE_SPECS[0].id, MoveId::LightPunch);
    assert_eq!(CLOSE_RANGE_MOVE_SPECS[1].id, MoveId::HeavyPunch);
    assert_eq!(CLOSE_RANGE_MOVE_SPECS[2].id, MoveId::Kick);
    assert_eq!(CLOSE_RANGE_MOVE_SPECS[3].id, MoveId::SweepKick);
    assert_eq!(CLOSE_RANGE_MOVE_SPECS[4].id, MoveId::OverheadPunch);
    assert_eq!(CLOSE_RANGE_MOVE_SPECS[5].id, MoveId::RisingAntiAir);
    assert_eq!(CLOSE_RANGE_MOVE_SPECS[6].id, MoveId::AirPunch);
    assert_eq!(CLOSE_RANGE_MOVE_SPECS[7].id, MoveId::AirKick);
    assert_eq!(CLOSE_RANGE_MOVE_SPECS[8].id, MoveId::CloseThrow);
    assert_eq!(CLOSE_RANGE_MOVE_SPECS[9].id, MoveId::RustBorrowJab);
    assert_eq!(CLOSE_RANGE_MOVE_SPECS[10].id, MoveId::DukeBoilerplatePoke);
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
fn traditional_move_specs_define_guard_rules_and_reactions() {
    let sweep = move_spec(MoveId::SweepKick);
    assert_eq!(sweep.input, MoveInputKind::Sweep);
    assert_eq!(sweep.label, "Sweep");
    assert_eq!(sweep.damage, SWEEP_KICK_DAMAGE);
    assert_eq!(sweep.guard_rule, GuardRule::Low);
    assert_eq!(sweep.hit_reaction, SWEEP_REACTION);
    assert_eq!(sweep.whiff_recovery, SWEEP_KICK_WHIFF_RECOVERY);

    let overhead = move_spec(MoveId::OverheadPunch);
    assert_eq!(overhead.input, MoveInputKind::Overhead);
    assert_eq!(overhead.label, "Overhead");
    assert_eq!(overhead.damage, OVERHEAD_PUNCH_DAMAGE);
    assert_eq!(overhead.guard_rule, GuardRule::High);
    assert_eq!(overhead.hit_reaction, OVERHEAD_REACTION);
    assert_eq!(overhead.whiff_recovery, OVERHEAD_PUNCH_WHIFF_RECOVERY);

    let anti_air = move_spec(MoveId::RisingAntiAir);
    assert_eq!(anti_air.input, MoveInputKind::AntiAir);
    assert_eq!(anti_air.label, "Anti-Air");
    assert_eq!(anti_air.damage, RISING_ANTI_AIR_DAMAGE);
    assert_eq!(anti_air.guard_rule, GuardRule::Mid);
    assert_eq!(anti_air.hit_reaction, RISING_ANTI_AIR_REACTION);
    assert_eq!(anti_air.whiff_recovery, RISING_ANTI_AIR_WHIFF_RECOVERY);

    let air_punch = move_spec(MoveId::AirPunch);
    assert_eq!(air_punch.input, MoveInputKind::AirPunch);
    assert_eq!(air_punch.damage, AIR_PUNCH_DAMAGE);
    assert_eq!(air_punch.guard_rule, GuardRule::High);
    assert_eq!(air_punch.hit_reaction, AIR_ATTACK_REACTION);
    assert_eq!(air_punch.whiff_recovery, AIR_ATTACK_WHIFF_RECOVERY);

    let air_kick = move_spec(MoveId::AirKick);
    assert_eq!(air_kick.input, MoveInputKind::AirKick);
    assert_eq!(air_kick.damage, AIR_KICK_DAMAGE);
    assert_eq!(air_kick.guard_rule, GuardRule::High);
    assert_eq!(air_kick.hit_reaction, AIR_ATTACK_REACTION);
    assert_eq!(air_kick.whiff_recovery, AIR_ATTACK_WHIFF_RECOVERY);

    let throw = move_spec(MoveId::CloseThrow);
    assert_eq!(throw.input, MoveInputKind::Throw);
    assert_eq!(throw.label, "Throw");
    assert_eq!(throw.damage, CLOSE_THROW_DAMAGE);
    assert_eq!(throw.guard_rule, GuardRule::Throw);
    assert_eq!(throw.hit_reaction, CLOSE_THROW_REACTION);
    assert_eq!(throw.whiff_recovery, CLOSE_THROW_WHIFF_RECOVERY);
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
    assert!(!GuardRule::High.is_blocked_by(true, true));
    assert!(GuardRule::Projectile.is_blocked_by(true, false));
    assert!(GuardRule::Low.is_blocked_by(true, true));
    assert!(!GuardRule::Low.is_blocked_by(true, false));
    assert!(!GuardRule::Throw.is_blocked_by(true, true));
    assert!(!GuardRule::Mid.is_blocked_by(false, false));
}

#[test]
fn loadout_move_selection_prefers_character_specific_specs() {
    let rust_moves = [
        MoveId::RustBorrowJab,
        MoveId::HeavyPunch,
        MoveId::Kick,
        MoveId::SweepKick,
        MoveId::OverheadPunch,
        MoveId::RisingAntiAir,
        MoveId::AirPunch,
        MoveId::AirKick,
        MoveId::CloseThrow,
    ];
    let duke_moves = [
        MoveId::LightPunch,
        MoveId::DukeBoilerplatePoke,
        MoveId::Kick,
        MoveId::SweepKick,
        MoveId::OverheadPunch,
        MoveId::RisingAntiAir,
        MoveId::AirPunch,
        MoveId::AirKick,
        MoveId::CloseThrow,
    ];

    assert_eq!(
        move_spec_for_input(&rust_moves, MoveInputKind::LightPunch),
        Some(move_spec(MoveId::RustBorrowJab))
    );
    assert_eq!(
        move_spec_for_input(&duke_moves, MoveInputKind::HeavyPunch),
        Some(move_spec(MoveId::DukeBoilerplatePoke))
    );
    assert_eq!(
        move_spec_for_input(&rust_moves, MoveInputKind::Sweep),
        Some(move_spec(MoveId::SweepKick))
    );
    assert_eq!(
        move_spec_for_input(&duke_moves, MoveInputKind::Throw),
        Some(move_spec(MoveId::CloseThrow))
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
        AttackKind::from_move_id(MoveId::SweepKick),
        AttackKind::Sweep
    );
    assert_eq!(
        AttackKind::from_move_id(MoveId::CloseThrow),
        AttackKind::Throw
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
    assert_eq!(
        AttackKind::AirPunch.move_spec(),
        move_spec(MoveId::AirPunch)
    );
}
