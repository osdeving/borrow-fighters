//! Verifies table-driven close-range move data.

use borrow_fighters::combat::fighter::{
    AIR_KICK_DAMAGE, AIR_PUNCH_DAMAGE, AttackKind, CLOSE_THROW_DAMAGE,
    DUKE_BOILERPLATE_POKE_DAMAGE, HEAVY_PUNCH_DAMAGE, KICK_DAMAGE, LIGHT_PUNCH_DAMAGE,
    OVERHEAD_PUNCH_DAMAGE, RISING_ANTI_AIR_DAMAGE, RUST_BORROW_JAB_DAMAGE, SWEEP_KICK_DAMAGE,
};
use borrow_fighters::combat::frame::FrameCount;
use borrow_fighters::combat::move_data::{
    AIR_ATTACK_REACTION, AIR_ATTACK_WHIFF_RECOVERY, CLOSE_RANGE_MOVE_SPECS, CLOSE_THROW_REACTION,
    CLOSE_THROW_WHIFF_RECOVERY, DUKE_ABSTRACT_FACTORY_OVERHEAD_DAMAGE,
    DUKE_ABSTRACT_FACTORY_OVERHEAD_REACTION, DUKE_ABSTRACT_FACTORY_OVERHEAD_WHIFF_RECOVERY,
    DUKE_BOILERPLATE_POKE_WHIFF_RECOVERY, DUKE_ENTERPRISE_THROW_DAMAGE,
    DUKE_ENTERPRISE_THROW_REACTION, DUKE_ENTERPRISE_THROW_WHIFF_RECOVERY,
    DUKE_GARBAGE_COLLECTOR_SWEEP_DAMAGE, DUKE_GARBAGE_COLLECTOR_SWEEP_REACTION,
    DUKE_GARBAGE_COLLECTOR_SWEEP_WHIFF_RECOVERY, GO_CHANNEL_OVERHEAD_DAMAGE,
    GO_CHANNEL_OVERHEAD_WHIFF_RECOVERY, GO_DEFER_KICK_DAMAGE, GO_DEFER_KICK_WHIFF_RECOVERY,
    GO_GOROUTINE_JAB_DAMAGE, GO_GOROUTINE_JAB_WHIFF_RECOVERY, GO_HOPKICK_DAMAGE,
    GO_HOPKICK_WHIFF_RECOVERY, GO_KICK_REACTION, GO_LIGHT_REACTION, GO_OVERHEAD_REACTION,
    GuardRule, HEAVY_ATTACK_REACTION, HEAVY_ATTACK_WHIFF_RECOVERY, KICK_REACTION,
    KICK_WHIFF_RECOVERY, LIGHT_ATTACK_REACTION, LIGHT_ATTACK_WHIFF_RECOVERY, MoveId, MoveInputKind,
    OVERHEAD_PUNCH_WHIFF_RECOVERY, OVERHEAD_REACTION, RISING_ANTI_AIR_REACTION,
    RISING_ANTI_AIR_WHIFF_RECOVERY, RUST_BORROW_JAB_WHIFF_RECOVERY, RUST_LIFETIME_ANTI_AIR_DAMAGE,
    RUST_LIFETIME_ANTI_AIR_REACTION, RUST_LIFETIME_ANTI_AIR_WHIFF_RECOVERY,
    RUST_OWNERSHIP_THROW_DAMAGE, RUST_OWNERSHIP_THROW_REACTION,
    RUST_OWNERSHIP_THROW_WHIFF_RECOVERY, SWEEP_KICK_WHIFF_RECOVERY, SWEEP_REACTION, move_spec,
    move_spec_for_input,
};

#[test]
fn close_range_moves_are_registered_in_table_order() {
    assert_eq!(CLOSE_RANGE_MOVE_SPECS.len(), 20);
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
    assert_eq!(CLOSE_RANGE_MOVE_SPECS[10].id, MoveId::RustLifetimeAntiAir);
    assert_eq!(CLOSE_RANGE_MOVE_SPECS[11].id, MoveId::RustOwnershipThrow);
    assert_eq!(CLOSE_RANGE_MOVE_SPECS[12].id, MoveId::DukeBoilerplatePoke);
    assert_eq!(
        CLOSE_RANGE_MOVE_SPECS[13].id,
        MoveId::DukeGarbageCollectorSweep
    );
    assert_eq!(
        CLOSE_RANGE_MOVE_SPECS[14].id,
        MoveId::DukeAbstractFactoryOverhead
    );
    assert_eq!(CLOSE_RANGE_MOVE_SPECS[15].id, MoveId::DukeEnterpriseThrow);
    assert_eq!(CLOSE_RANGE_MOVE_SPECS[16].id, MoveId::GoGoroutineJab);
    assert_eq!(CLOSE_RANGE_MOVE_SPECS[17].id, MoveId::GoDeferKick);
    assert_eq!(CLOSE_RANGE_MOVE_SPECS[18].id, MoveId::GoChannelOverhead);
    assert_eq!(CLOSE_RANGE_MOVE_SPECS[19].id, MoveId::GoHopkick);
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

    let rust_anti_air = move_spec(MoveId::RustLifetimeAntiAir);
    assert_eq!(rust_anti_air.input, MoveInputKind::AntiAir);
    assert_eq!(rust_anti_air.label, "Lifetime AA");
    assert_eq!(rust_anti_air.damage, RUST_LIFETIME_ANTI_AIR_DAMAGE);
    assert_eq!(rust_anti_air.frames.duration, FrameCount::new(26));
    assert_eq!(rust_anti_air.frames.active_start, FrameCount::new(6));
    assert_eq!(rust_anti_air.frames.active_end, FrameCount::new(12));
    assert_eq!(rust_anti_air.hitbox.width, 62.0);
    assert_eq!(rust_anti_air.hitbox.height, 90.0);
    assert_eq!(rust_anti_air.hitbox.y_offset, -92.0);
    assert_eq!(rust_anti_air.guard_rule, GuardRule::Mid);
    assert_eq!(rust_anti_air.hit_reaction, RUST_LIFETIME_ANTI_AIR_REACTION);
    assert_eq!(
        rust_anti_air.whiff_recovery,
        RUST_LIFETIME_ANTI_AIR_WHIFF_RECOVERY
    );

    let rust_throw = move_spec(MoveId::RustOwnershipThrow);
    assert_eq!(rust_throw.input, MoveInputKind::Throw);
    assert_eq!(rust_throw.label, "Ownership");
    assert_eq!(rust_throw.damage, RUST_OWNERSHIP_THROW_DAMAGE);
    assert_eq!(rust_throw.frames.duration, FrameCount::new(20));
    assert_eq!(rust_throw.frames.active_start, FrameCount::new(5));
    assert_eq!(rust_throw.frames.active_end, FrameCount::new(7));
    assert_eq!(rust_throw.hitbox.width, 42.0);
    assert_eq!(rust_throw.hitbox.height, 118.0);
    assert_eq!(rust_throw.hitbox.y_offset, 30.0);
    assert_eq!(rust_throw.guard_rule, GuardRule::Throw);
    assert_eq!(rust_throw.hit_reaction, RUST_OWNERSHIP_THROW_REACTION);
    assert_eq!(
        rust_throw.whiff_recovery,
        RUST_OWNERSHIP_THROW_WHIFF_RECOVERY
    );

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

    let duke_sweep = move_spec(MoveId::DukeGarbageCollectorSweep);
    assert_eq!(duke_sweep.input, MoveInputKind::Sweep);
    assert_eq!(duke_sweep.label, "GC Sweep");
    assert_eq!(duke_sweep.damage, DUKE_GARBAGE_COLLECTOR_SWEEP_DAMAGE);
    assert_eq!(duke_sweep.frames.duration, FrameCount::new(38));
    assert_eq!(duke_sweep.frames.active_start, FrameCount::new(13));
    assert_eq!(duke_sweep.frames.active_end, FrameCount::new(22));
    assert_eq!(duke_sweep.hitbox.width, 128.0);
    assert_eq!(duke_sweep.guard_rule, GuardRule::Low);
    assert_eq!(
        duke_sweep.hit_reaction,
        DUKE_GARBAGE_COLLECTOR_SWEEP_REACTION
    );
    assert_eq!(
        duke_sweep.whiff_recovery,
        DUKE_GARBAGE_COLLECTOR_SWEEP_WHIFF_RECOVERY
    );

    let duke_overhead = move_spec(MoveId::DukeAbstractFactoryOverhead);
    assert_eq!(duke_overhead.input, MoveInputKind::Overhead);
    assert_eq!(duke_overhead.label, "Factory OH");
    assert_eq!(duke_overhead.damage, DUKE_ABSTRACT_FACTORY_OVERHEAD_DAMAGE);
    assert_eq!(duke_overhead.frames.duration, FrameCount::new(40));
    assert_eq!(duke_overhead.frames.active_start, FrameCount::new(15));
    assert_eq!(duke_overhead.frames.active_end, FrameCount::new(22));
    assert_eq!(duke_overhead.hitbox.width, 96.0);
    assert_eq!(duke_overhead.guard_rule, GuardRule::High);
    assert_eq!(
        duke_overhead.hit_reaction,
        DUKE_ABSTRACT_FACTORY_OVERHEAD_REACTION
    );
    assert_eq!(
        duke_overhead.whiff_recovery,
        DUKE_ABSTRACT_FACTORY_OVERHEAD_WHIFF_RECOVERY
    );

    let duke_throw = move_spec(MoveId::DukeEnterpriseThrow);
    assert_eq!(duke_throw.input, MoveInputKind::Throw);
    assert_eq!(duke_throw.label, "Enterprise Grab");
    assert_eq!(duke_throw.damage, DUKE_ENTERPRISE_THROW_DAMAGE);
    assert_eq!(duke_throw.frames.duration, FrameCount::new(30));
    assert_eq!(duke_throw.frames.active_start, FrameCount::new(9));
    assert_eq!(duke_throw.frames.active_end, FrameCount::new(11));
    assert_eq!(duke_throw.hitbox.width, 56.0);
    assert_eq!(duke_throw.guard_rule, GuardRule::Throw);
    assert_eq!(duke_throw.hit_reaction, DUKE_ENTERPRISE_THROW_REACTION);
    assert_eq!(
        duke_throw.whiff_recovery,
        DUKE_ENTERPRISE_THROW_WHIFF_RECOVERY
    );

    let go_jab = move_spec(MoveId::GoGoroutineJab);
    assert_eq!(go_jab.input, MoveInputKind::LightPunch);
    assert_eq!(go_jab.label, "Goroutine Jab");
    assert_eq!(go_jab.damage, GO_GOROUTINE_JAB_DAMAGE);
    assert_eq!(go_jab.frames.duration, FrameCount::new(14));
    assert_eq!(go_jab.frames.active_start, FrameCount::new(3));
    assert_eq!(go_jab.frames.active_end, FrameCount::new(7));
    assert_eq!(go_jab.hitbox.width, 42.0);
    assert_eq!(go_jab.guard_rule, GuardRule::Mid);
    assert_eq!(go_jab.hit_reaction, GO_LIGHT_REACTION);
    assert_eq!(go_jab.whiff_recovery, GO_GOROUTINE_JAB_WHIFF_RECOVERY);

    let go_kick = move_spec(MoveId::GoDeferKick);
    assert_eq!(go_kick.input, MoveInputKind::Kick);
    assert_eq!(go_kick.label, "Defer Kick");
    assert_eq!(go_kick.damage, GO_DEFER_KICK_DAMAGE);
    assert_eq!(go_kick.frames.duration, FrameCount::new(23));
    assert_eq!(go_kick.frames.active_start, FrameCount::new(7));
    assert_eq!(go_kick.frames.active_end, FrameCount::new(13));
    assert_eq!(go_kick.hitbox.width, 86.0);
    assert_eq!(go_kick.guard_rule, GuardRule::Mid);
    assert_eq!(go_kick.hit_reaction, GO_KICK_REACTION);
    assert_eq!(go_kick.whiff_recovery, GO_DEFER_KICK_WHIFF_RECOVERY);

    let go_overhead = move_spec(MoveId::GoChannelOverhead);
    assert_eq!(go_overhead.input, MoveInputKind::Overhead);
    assert_eq!(go_overhead.label, "Channel OH");
    assert_eq!(go_overhead.damage, GO_CHANNEL_OVERHEAD_DAMAGE);
    assert_eq!(go_overhead.frames.duration, FrameCount::new(28));
    assert_eq!(go_overhead.frames.active_start, FrameCount::new(10));
    assert_eq!(go_overhead.frames.active_end, FrameCount::new(15));
    assert_eq!(go_overhead.hitbox.width, 70.0);
    assert_eq!(go_overhead.guard_rule, GuardRule::High);
    assert_eq!(go_overhead.hit_reaction, GO_OVERHEAD_REACTION);
    assert_eq!(
        go_overhead.whiff_recovery,
        GO_CHANNEL_OVERHEAD_WHIFF_RECOVERY
    );

    let go_hopkick = move_spec(MoveId::GoHopkick);
    assert_eq!(go_hopkick.input, MoveInputKind::AirKick);
    assert_eq!(go_hopkick.label, "Hopkick");
    assert_eq!(go_hopkick.damage, GO_HOPKICK_DAMAGE);
    assert_eq!(go_hopkick.frames.duration, FrameCount::new(20));
    assert_eq!(go_hopkick.frames.active_start, FrameCount::new(5));
    assert_eq!(go_hopkick.frames.active_end, FrameCount::new(12));
    assert_eq!(go_hopkick.hitbox.width, 78.0);
    assert_eq!(go_hopkick.guard_rule, GuardRule::High);
    assert_eq!(go_hopkick.hit_reaction, GO_KICK_REACTION);
    assert_eq!(go_hopkick.whiff_recovery, GO_HOPKICK_WHIFF_RECOVERY);
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
        MoveId::RustLifetimeAntiAir,
        MoveId::AirPunch,
        MoveId::AirKick,
        MoveId::RustOwnershipThrow,
    ];
    let duke_moves = [
        MoveId::LightPunch,
        MoveId::DukeBoilerplatePoke,
        MoveId::Kick,
        MoveId::DukeGarbageCollectorSweep,
        MoveId::DukeAbstractFactoryOverhead,
        MoveId::RisingAntiAir,
        MoveId::AirPunch,
        MoveId::AirKick,
        MoveId::DukeEnterpriseThrow,
    ];
    let go_moves = [
        MoveId::GoGoroutineJab,
        MoveId::HeavyPunch,
        MoveId::GoDeferKick,
        MoveId::SweepKick,
        MoveId::GoChannelOverhead,
        MoveId::RisingAntiAir,
        MoveId::AirPunch,
        MoveId::GoHopkick,
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
        move_spec_for_input(&rust_moves, MoveInputKind::AntiAir),
        Some(move_spec(MoveId::RustLifetimeAntiAir))
    );
    assert_eq!(
        move_spec_for_input(&rust_moves, MoveInputKind::Throw),
        Some(move_spec(MoveId::RustOwnershipThrow))
    );
    assert_eq!(
        move_spec_for_input(&duke_moves, MoveInputKind::Sweep),
        Some(move_spec(MoveId::DukeGarbageCollectorSweep))
    );
    assert_eq!(
        move_spec_for_input(&duke_moves, MoveInputKind::Overhead),
        Some(move_spec(MoveId::DukeAbstractFactoryOverhead))
    );
    assert_eq!(
        move_spec_for_input(&duke_moves, MoveInputKind::Throw),
        Some(move_spec(MoveId::DukeEnterpriseThrow))
    );
    assert_eq!(
        move_spec_for_input(&go_moves, MoveInputKind::LightPunch),
        Some(move_spec(MoveId::GoGoroutineJab))
    );
    assert_eq!(
        move_spec_for_input(&go_moves, MoveInputKind::Kick),
        Some(move_spec(MoveId::GoDeferKick))
    );
    assert_eq!(
        move_spec_for_input(&go_moves, MoveInputKind::Overhead),
        Some(move_spec(MoveId::GoChannelOverhead))
    );
    assert_eq!(
        move_spec_for_input(&go_moves, MoveInputKind::AirKick),
        Some(move_spec(MoveId::GoHopkick))
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
        AttackKind::from_move_id(MoveId::RustLifetimeAntiAir),
        AttackKind::AntiAir
    );
    assert_eq!(
        AttackKind::from_move_id(MoveId::DukeGarbageCollectorSweep),
        AttackKind::Sweep
    );
    assert_eq!(
        AttackKind::from_move_id(MoveId::DukeAbstractFactoryOverhead),
        AttackKind::Overhead
    );
    assert_eq!(
        AttackKind::from_move_id(MoveId::RustOwnershipThrow),
        AttackKind::Throw
    );
    assert_eq!(
        AttackKind::from_move_id(MoveId::GoGoroutineJab),
        AttackKind::LightPunch
    );
    assert_eq!(
        AttackKind::from_move_id(MoveId::GoDeferKick),
        AttackKind::Kick
    );
    assert_eq!(
        AttackKind::from_move_id(MoveId::GoChannelOverhead),
        AttackKind::Overhead
    );
    assert_eq!(
        AttackKind::from_move_id(MoveId::GoHopkick),
        AttackKind::AirKick
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
