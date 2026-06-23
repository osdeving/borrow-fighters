//! Verifies table-driven close-range move data.

use borrow_fighters::combat::fighter::{
    AIR_KICK_DAMAGE, AIR_PUNCH_DAMAGE, AttackKind, CLOSE_THROW_DAMAGE,
    DUKE_BOILERPLATE_POKE_DAMAGE, HEAVY_PUNCH_DAMAGE, KICK_DAMAGE, LIGHT_PUNCH_DAMAGE,
    OVERHEAD_PUNCH_DAMAGE, RISING_ANTI_AIR_DAMAGE, RUST_BORROW_JAB_DAMAGE, SWEEP_KICK_DAMAGE,
};
use borrow_fighters::combat::frame::FrameCount;
use borrow_fighters::combat::move_data::{
    AIR_ATTACK_REACTION, AIR_ATTACK_WHIFF_RECOVERY, C_ANTI_AIR_REACTION, C_HEAVY_REACTION,
    C_INTERRUPT_VECTOR_DAMAGE, C_INTERRUPT_VECTOR_WHIFF_RECOVERY, C_KICK_REACTION,
    C_LIGHT_REACTION, C_NULL_STEP_KICK_DAMAGE, C_NULL_STEP_KICK_WHIFF_RECOVERY,
    C_OVERHEAD_REACTION, C_POINTER_JAB_DAMAGE, C_POINTER_JAB_WHIFF_RECOVERY,
    C_SEGFAULT_SWEEP_DAMAGE, C_SEGFAULT_SWEEP_WHIFF_RECOVERY, C_STACK_OVERFLOW_DAMAGE,
    C_STACK_OVERFLOW_WHIFF_RECOVERY, C_SWEEP_REACTION, C_THROW_REACTION, C_UNDEFINED_THROW_DAMAGE,
    C_UNDEFINED_THROW_WHIFF_RECOVERY, C_UNSAFE_POKE_DAMAGE, C_UNSAFE_POKE_WHIFF_RECOVERY,
    CLOSE_RANGE_MOVE_SPECS, CLOSE_THROW_REACTION, CLOSE_THROW_WHIFF_RECOVERY,
    DUKE_ABSTRACT_FACTORY_OVERHEAD_DAMAGE, DUKE_ABSTRACT_FACTORY_OVERHEAD_REACTION,
    DUKE_ABSTRACT_FACTORY_OVERHEAD_WHIFF_RECOVERY, DUKE_BOILERPLATE_POKE_WHIFF_RECOVERY,
    DUKE_ENTERPRISE_THROW_DAMAGE, DUKE_ENTERPRISE_THROW_REACTION,
    DUKE_ENTERPRISE_THROW_WHIFF_RECOVERY, DUKE_GARBAGE_COLLECTOR_SWEEP_DAMAGE,
    DUKE_GARBAGE_COLLECTOR_SWEEP_REACTION, DUKE_GARBAGE_COLLECTOR_SWEEP_WHIFF_RECOVERY,
    GO_CHANNEL_OVERHEAD_DAMAGE, GO_CHANNEL_OVERHEAD_WHIFF_RECOVERY, GO_DEFER_KICK_DAMAGE,
    GO_DEFER_KICK_WHIFF_RECOVERY, GO_GOROUTINE_JAB_DAMAGE, GO_GOROUTINE_JAB_WHIFF_RECOVERY,
    GO_HOPKICK_DAMAGE, GO_HOPKICK_WHIFF_RECOVERY, GO_KICK_REACTION, GO_LIGHT_REACTION,
    GO_OVERHEAD_REACTION, GuardRule, HEAVY_ATTACK_REACTION, HEAVY_ATTACK_WHIFF_RECOVERY,
    KICK_REACTION, KICK_WHIFF_RECOVERY, LIGHT_ATTACK_REACTION, LIGHT_ATTACK_WHIFF_RECOVERY, MoveId,
    MoveInputKind, OVERHEAD_PUNCH_WHIFF_RECOVERY, OVERHEAD_REACTION, PYTHON_ANTI_AIR_REACTION,
    PYTHON_CONSTRICT_THROW_DAMAGE, PYTHON_CONSTRICT_THROW_WHIFF_RECOVERY,
    PYTHON_DATA_STRIKE_DAMAGE, PYTHON_DATA_STRIKE_WHIFF_RECOVERY, PYTHON_HEAVY_REACTION,
    PYTHON_HEEL_KICK_DAMAGE, PYTHON_HEEL_KICK_WHIFF_RECOVERY, PYTHON_INDENT_SWEEP_DAMAGE,
    PYTHON_INDENT_SWEEP_WHIFF_RECOVERY, PYTHON_KICK_REACTION, PYTHON_LIGHT_REACTION,
    PYTHON_OVERHEAD_REACTION, PYTHON_SNAKE_BITE_DAMAGE, PYTHON_SNAKE_BITE_WHIFF_RECOVERY,
    PYTHON_SWEEP_REACTION, PYTHON_THROW_REACTION, PYTHON_TRACEBACK_OVERHEAD_DAMAGE,
    PYTHON_TRACEBACK_OVERHEAD_WHIFF_RECOVERY, PYTHON_VISION_ANTI_AIR_DAMAGE,
    PYTHON_VISION_ANTI_AIR_WHIFF_RECOVERY, RISING_ANTI_AIR_REACTION,
    RISING_ANTI_AIR_WHIFF_RECOVERY, RUST_BORROW_JAB_WHIFF_RECOVERY, RUST_LIFETIME_ANTI_AIR_DAMAGE,
    RUST_LIFETIME_ANTI_AIR_REACTION, RUST_LIFETIME_ANTI_AIR_WHIFF_RECOVERY,
    RUST_OWNERSHIP_THROW_DAMAGE, RUST_OWNERSHIP_THROW_REACTION,
    RUST_OWNERSHIP_THROW_WHIFF_RECOVERY, SWEEP_KICK_WHIFF_RECOVERY, SWEEP_REACTION, move_spec,
    move_spec_for_input,
};
use borrow_fighters::config::world_px;

#[test]
fn close_range_moves_are_registered_in_table_order() {
    assert_eq!(CLOSE_RANGE_MOVE_SPECS.len(), 34);
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
    assert_eq!(CLOSE_RANGE_MOVE_SPECS[20].id, MoveId::CPointerJab);
    assert_eq!(CLOSE_RANGE_MOVE_SPECS[21].id, MoveId::CUnsafePoke);
    assert_eq!(CLOSE_RANGE_MOVE_SPECS[22].id, MoveId::CNullStepKick);
    assert_eq!(CLOSE_RANGE_MOVE_SPECS[23].id, MoveId::CSegfaultSweep);
    assert_eq!(CLOSE_RANGE_MOVE_SPECS[24].id, MoveId::CStackOverflow);
    assert_eq!(CLOSE_RANGE_MOVE_SPECS[25].id, MoveId::CInterruptVector);
    assert_eq!(CLOSE_RANGE_MOVE_SPECS[26].id, MoveId::CUndefinedThrow);
    assert_eq!(CLOSE_RANGE_MOVE_SPECS[27].id, MoveId::PythonSnakeBite);
    assert_eq!(CLOSE_RANGE_MOVE_SPECS[28].id, MoveId::PythonDataStrike);
    assert_eq!(CLOSE_RANGE_MOVE_SPECS[29].id, MoveId::PythonHeelKick);
    assert_eq!(CLOSE_RANGE_MOVE_SPECS[30].id, MoveId::PythonIndentSweep);
    assert_eq!(
        CLOSE_RANGE_MOVE_SPECS[31].id,
        MoveId::PythonTracebackOverhead
    );
    assert_eq!(CLOSE_RANGE_MOVE_SPECS[32].id, MoveId::PythonVisionAntiAir);
    assert_eq!(CLOSE_RANGE_MOVE_SPECS[33].id, MoveId::PythonConstrictThrow);
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
    assert_eq!(light.hitbox.width, world_px(58.0));
    assert_eq!(light.hitbox.height, world_px(34.0));
    assert_eq!(light.hitbox.y_offset, world_px(62.0));
    assert_eq!(light.guard_rule, GuardRule::Mid);
    assert_eq!(light.hit_reaction, LIGHT_ATTACK_REACTION);
    assert_eq!(light.hit_reaction.hit_pushback, world_px(22.0));
    assert_eq!(light.hit_reaction.block_pushback, world_px(14.0));
    assert_eq!(light.whiff_recovery, LIGHT_ATTACK_WHIFF_RECOVERY);

    let heavy = move_spec(MoveId::HeavyPunch);
    assert_eq!(heavy.input, MoveInputKind::HeavyPunch);
    assert_eq!(heavy.label, "HP");
    assert_eq!(heavy.damage, HEAVY_PUNCH_DAMAGE);
    assert_eq!(heavy.frames.duration, FrameCount::new(35));
    assert_eq!(heavy.frames.active_start, FrameCount::new(11));
    assert_eq!(heavy.frames.active_end, FrameCount::new(20));
    assert_eq!(heavy.hitbox.width, world_px(96.0));
    assert_eq!(heavy.hitbox.height, world_px(42.0));
    assert_eq!(heavy.hitbox.y_offset, world_px(58.0));
    assert_eq!(heavy.guard_rule, GuardRule::Mid);
    assert_eq!(heavy.hit_reaction, HEAVY_ATTACK_REACTION);
    assert_eq!(heavy.hit_reaction.hit_pushback, world_px(38.0));
    assert_eq!(heavy.hit_reaction.block_pushback, world_px(26.0));
    assert_eq!(heavy.whiff_recovery, HEAVY_ATTACK_WHIFF_RECOVERY);

    let kick = move_spec(MoveId::Kick);
    assert_eq!(kick.input, MoveInputKind::Kick);
    assert_eq!(kick.label, "KICK");
    assert_eq!(kick.damage, KICK_DAMAGE);
    assert_eq!(kick.frames.duration, FrameCount::new(28));
    assert_eq!(kick.frames.active_start, FrameCount::new(9));
    assert_eq!(kick.frames.active_end, FrameCount::new(16));
    assert_eq!(kick.hitbox.width, world_px(100.0));
    assert_eq!(kick.hitbox.height, world_px(36.0));
    assert_eq!(kick.hitbox.y_offset, world_px(108.0));
    assert_eq!(kick.guard_rule, GuardRule::Mid);
    assert_eq!(kick.hit_reaction, KICK_REACTION);
    assert_eq!(kick.hit_reaction.hit_pushback, world_px(34.0));
    assert_eq!(kick.hit_reaction.block_pushback, world_px(22.0));
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
    assert_eq!(rust_jab.hitbox.width, world_px(48.0));
    assert_eq!(rust_jab.hitbox.height, world_px(30.0));
    assert_eq!(rust_jab.hitbox.y_offset, world_px(62.0));
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
    assert_eq!(rust_anti_air.hitbox.width, world_px(62.0));
    assert_eq!(rust_anti_air.hitbox.height, world_px(90.0));
    assert_eq!(rust_anti_air.hitbox.y_offset, world_px(-92.0));
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
    assert_eq!(rust_throw.hitbox.width, world_px(42.0));
    assert_eq!(rust_throw.hitbox.height, world_px(118.0));
    assert_eq!(rust_throw.hitbox.y_offset, world_px(30.0));
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
    assert_eq!(duke_poke.hitbox.width, world_px(112.0));
    assert_eq!(duke_poke.hitbox.height, world_px(44.0));
    assert_eq!(duke_poke.hitbox.y_offset, world_px(60.0));
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
    assert_eq!(duke_sweep.hitbox.width, world_px(128.0));
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
    assert_eq!(duke_overhead.hitbox.width, world_px(96.0));
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
    assert_eq!(duke_throw.hitbox.width, world_px(56.0));
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
    assert_eq!(go_jab.hitbox.width, world_px(42.0));
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
    assert_eq!(go_kick.hitbox.width, world_px(86.0));
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
    assert_eq!(go_overhead.hitbox.width, world_px(70.0));
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
    assert_eq!(go_hopkick.hitbox.width, world_px(78.0));
    assert_eq!(go_hopkick.guard_rule, GuardRule::High);
    assert_eq!(go_hopkick.hit_reaction, GO_KICK_REACTION);
    assert_eq!(go_hopkick.whiff_recovery, GO_HOPKICK_WHIFF_RECOVERY);

    let c_jab = move_spec(MoveId::CPointerJab);
    assert_eq!(c_jab.input, MoveInputKind::LightPunch);
    assert_eq!(c_jab.label, "Pointer Jab");
    assert_eq!(c_jab.damage, C_POINTER_JAB_DAMAGE);
    assert_eq!(c_jab.frames.duration, FrameCount::new(18));
    assert_eq!(c_jab.frames.active_start, FrameCount::new(5));
    assert_eq!(c_jab.frames.active_end, FrameCount::new(10));
    assert_eq!(c_jab.hitbox.width, world_px(64.0));
    assert_eq!(c_jab.guard_rule, GuardRule::Mid);
    assert_eq!(c_jab.hit_reaction, C_LIGHT_REACTION);
    assert_eq!(c_jab.whiff_recovery, C_POINTER_JAB_WHIFF_RECOVERY);

    let c_poke = move_spec(MoveId::CUnsafePoke);
    assert_eq!(c_poke.input, MoveInputKind::HeavyPunch);
    assert_eq!(c_poke.label, "Unsafe Poke");
    assert_eq!(c_poke.damage, C_UNSAFE_POKE_DAMAGE);
    assert_eq!(c_poke.frames.duration, FrameCount::new(38));
    assert_eq!(c_poke.frames.active_start, FrameCount::new(12));
    assert_eq!(c_poke.frames.active_end, FrameCount::new(20));
    assert_eq!(c_poke.hitbox.width, world_px(108.0));
    assert_eq!(c_poke.guard_rule, GuardRule::Mid);
    assert_eq!(c_poke.hit_reaction, C_HEAVY_REACTION);
    assert_eq!(c_poke.whiff_recovery, C_UNSAFE_POKE_WHIFF_RECOVERY);

    let c_kick = move_spec(MoveId::CNullStepKick);
    assert_eq!(c_kick.input, MoveInputKind::Kick);
    assert_eq!(c_kick.label, "Null Step");
    assert_eq!(c_kick.damage, C_NULL_STEP_KICK_DAMAGE);
    assert_eq!(c_kick.frames.duration, FrameCount::new(30));
    assert_eq!(c_kick.guard_rule, GuardRule::Mid);
    assert_eq!(c_kick.hit_reaction, C_KICK_REACTION);
    assert_eq!(c_kick.whiff_recovery, C_NULL_STEP_KICK_WHIFF_RECOVERY);

    let c_sweep = move_spec(MoveId::CSegfaultSweep);
    assert_eq!(c_sweep.input, MoveInputKind::Sweep);
    assert_eq!(c_sweep.label, "Segfault");
    assert_eq!(c_sweep.damage, C_SEGFAULT_SWEEP_DAMAGE);
    assert_eq!(c_sweep.hitbox.width, world_px(122.0));
    assert_eq!(c_sweep.guard_rule, GuardRule::Low);
    assert_eq!(c_sweep.hit_reaction, C_SWEEP_REACTION);
    assert_eq!(c_sweep.whiff_recovery, C_SEGFAULT_SWEEP_WHIFF_RECOVERY);

    let c_overhead = move_spec(MoveId::CStackOverflow);
    assert_eq!(c_overhead.input, MoveInputKind::Overhead);
    assert_eq!(c_overhead.label, "Stack OH");
    assert_eq!(c_overhead.damage, C_STACK_OVERFLOW_DAMAGE);
    assert_eq!(c_overhead.guard_rule, GuardRule::High);
    assert_eq!(c_overhead.hit_reaction, C_OVERHEAD_REACTION);
    assert_eq!(c_overhead.whiff_recovery, C_STACK_OVERFLOW_WHIFF_RECOVERY);

    let c_anti_air = move_spec(MoveId::CInterruptVector);
    assert_eq!(c_anti_air.input, MoveInputKind::AntiAir);
    assert_eq!(c_anti_air.label, "Interrupt");
    assert_eq!(c_anti_air.damage, C_INTERRUPT_VECTOR_DAMAGE);
    assert_eq!(c_anti_air.guard_rule, GuardRule::Mid);
    assert_eq!(c_anti_air.hit_reaction, C_ANTI_AIR_REACTION);
    assert_eq!(c_anti_air.whiff_recovery, C_INTERRUPT_VECTOR_WHIFF_RECOVERY);

    let c_throw = move_spec(MoveId::CUndefinedThrow);
    assert_eq!(c_throw.input, MoveInputKind::Throw);
    assert_eq!(c_throw.label, "Undefined");
    assert_eq!(c_throw.damage, C_UNDEFINED_THROW_DAMAGE);
    assert_eq!(c_throw.guard_rule, GuardRule::Throw);
    assert_eq!(c_throw.hit_reaction, C_THROW_REACTION);
    assert_eq!(c_throw.whiff_recovery, C_UNDEFINED_THROW_WHIFF_RECOVERY);

    let python_jab = move_spec(MoveId::PythonSnakeBite);
    assert_eq!(python_jab.input, MoveInputKind::LightPunch);
    assert_eq!(python_jab.label, "Snake Bite");
    assert_eq!(python_jab.damage, PYTHON_SNAKE_BITE_DAMAGE);
    assert_eq!(python_jab.frames.duration, FrameCount::new(17));
    assert_eq!(python_jab.guard_rule, GuardRule::Mid);
    assert_eq!(python_jab.hit_reaction, PYTHON_LIGHT_REACTION);
    assert_eq!(python_jab.whiff_recovery, PYTHON_SNAKE_BITE_WHIFF_RECOVERY);

    let python_poke = move_spec(MoveId::PythonDataStrike);
    assert_eq!(python_poke.input, MoveInputKind::HeavyPunch);
    assert_eq!(python_poke.label, "Data Strike");
    assert_eq!(python_poke.damage, PYTHON_DATA_STRIKE_DAMAGE);
    assert_eq!(python_poke.frames.duration, FrameCount::new(31));
    assert_eq!(python_poke.guard_rule, GuardRule::Mid);
    assert_eq!(python_poke.hit_reaction, PYTHON_HEAVY_REACTION);
    assert_eq!(
        python_poke.whiff_recovery,
        PYTHON_DATA_STRIKE_WHIFF_RECOVERY
    );

    let python_kick = move_spec(MoveId::PythonHeelKick);
    assert_eq!(python_kick.input, MoveInputKind::Kick);
    assert_eq!(python_kick.label, "Heel Kick");
    assert_eq!(python_kick.damage, PYTHON_HEEL_KICK_DAMAGE);
    assert_eq!(python_kick.hit_reaction, PYTHON_KICK_REACTION);
    assert_eq!(python_kick.whiff_recovery, PYTHON_HEEL_KICK_WHIFF_RECOVERY);

    let python_sweep = move_spec(MoveId::PythonIndentSweep);
    assert_eq!(python_sweep.input, MoveInputKind::Sweep);
    assert_eq!(python_sweep.label, "Indent Low");
    assert_eq!(python_sweep.damage, PYTHON_INDENT_SWEEP_DAMAGE);
    assert_eq!(python_sweep.guard_rule, GuardRule::Low);
    assert_eq!(python_sweep.hit_reaction, PYTHON_SWEEP_REACTION);
    assert_eq!(
        python_sweep.whiff_recovery,
        PYTHON_INDENT_SWEEP_WHIFF_RECOVERY
    );

    let python_overhead = move_spec(MoveId::PythonTracebackOverhead);
    assert_eq!(python_overhead.input, MoveInputKind::Overhead);
    assert_eq!(python_overhead.label, "Traceback");
    assert_eq!(python_overhead.damage, PYTHON_TRACEBACK_OVERHEAD_DAMAGE);
    assert_eq!(python_overhead.guard_rule, GuardRule::High);
    assert_eq!(python_overhead.hit_reaction, PYTHON_OVERHEAD_REACTION);
    assert_eq!(
        python_overhead.whiff_recovery,
        PYTHON_TRACEBACK_OVERHEAD_WHIFF_RECOVERY
    );

    let python_anti_air = move_spec(MoveId::PythonVisionAntiAir);
    assert_eq!(python_anti_air.input, MoveInputKind::AntiAir);
    assert_eq!(python_anti_air.label, "Vision AA");
    assert_eq!(python_anti_air.damage, PYTHON_VISION_ANTI_AIR_DAMAGE);
    assert_eq!(python_anti_air.guard_rule, GuardRule::Mid);
    assert_eq!(python_anti_air.hit_reaction, PYTHON_ANTI_AIR_REACTION);
    assert_eq!(
        python_anti_air.whiff_recovery,
        PYTHON_VISION_ANTI_AIR_WHIFF_RECOVERY
    );

    let python_throw = move_spec(MoveId::PythonConstrictThrow);
    assert_eq!(python_throw.input, MoveInputKind::Throw);
    assert_eq!(python_throw.label, "Constrict");
    assert_eq!(python_throw.damage, PYTHON_CONSTRICT_THROW_DAMAGE);
    assert_eq!(python_throw.guard_rule, GuardRule::Throw);
    assert_eq!(python_throw.hit_reaction, PYTHON_THROW_REACTION);
    assert_eq!(
        python_throw.whiff_recovery,
        PYTHON_CONSTRICT_THROW_WHIFF_RECOVERY
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
    let c_moves = [
        MoveId::CPointerJab,
        MoveId::CUnsafePoke,
        MoveId::CNullStepKick,
        MoveId::CSegfaultSweep,
        MoveId::CStackOverflow,
        MoveId::CInterruptVector,
        MoveId::AirPunch,
        MoveId::AirKick,
        MoveId::CUndefinedThrow,
    ];
    let python_moves = [
        MoveId::PythonSnakeBite,
        MoveId::PythonDataStrike,
        MoveId::PythonHeelKick,
        MoveId::PythonIndentSweep,
        MoveId::PythonTracebackOverhead,
        MoveId::PythonVisionAntiAir,
        MoveId::AirPunch,
        MoveId::AirKick,
        MoveId::PythonConstrictThrow,
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
    assert_eq!(
        move_spec_for_input(&c_moves, MoveInputKind::LightPunch),
        Some(move_spec(MoveId::CPointerJab))
    );
    assert_eq!(
        move_spec_for_input(&c_moves, MoveInputKind::HeavyPunch),
        Some(move_spec(MoveId::CUnsafePoke))
    );
    assert_eq!(
        move_spec_for_input(&c_moves, MoveInputKind::Kick),
        Some(move_spec(MoveId::CNullStepKick))
    );
    assert_eq!(
        move_spec_for_input(&c_moves, MoveInputKind::Sweep),
        Some(move_spec(MoveId::CSegfaultSweep))
    );
    assert_eq!(
        move_spec_for_input(&c_moves, MoveInputKind::Overhead),
        Some(move_spec(MoveId::CStackOverflow))
    );
    assert_eq!(
        move_spec_for_input(&c_moves, MoveInputKind::AntiAir),
        Some(move_spec(MoveId::CInterruptVector))
    );
    assert_eq!(
        move_spec_for_input(&c_moves, MoveInputKind::Throw),
        Some(move_spec(MoveId::CUndefinedThrow))
    );
    assert_eq!(
        move_spec_for_input(&python_moves, MoveInputKind::LightPunch),
        Some(move_spec(MoveId::PythonSnakeBite))
    );
    assert_eq!(
        move_spec_for_input(&python_moves, MoveInputKind::HeavyPunch),
        Some(move_spec(MoveId::PythonDataStrike))
    );
    assert_eq!(
        move_spec_for_input(&python_moves, MoveInputKind::Kick),
        Some(move_spec(MoveId::PythonHeelKick))
    );
    assert_eq!(
        move_spec_for_input(&python_moves, MoveInputKind::Sweep),
        Some(move_spec(MoveId::PythonIndentSweep))
    );
    assert_eq!(
        move_spec_for_input(&python_moves, MoveInputKind::Overhead),
        Some(move_spec(MoveId::PythonTracebackOverhead))
    );
    assert_eq!(
        move_spec_for_input(&python_moves, MoveInputKind::AntiAir),
        Some(move_spec(MoveId::PythonVisionAntiAir))
    );
    assert_eq!(
        move_spec_for_input(&python_moves, MoveInputKind::Throw),
        Some(move_spec(MoveId::PythonConstrictThrow))
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
        AttackKind::from_move_id(MoveId::CPointerJab),
        AttackKind::LightPunch
    );
    assert_eq!(
        AttackKind::from_move_id(MoveId::CUnsafePoke),
        AttackKind::HeavyPunch
    );
    assert_eq!(
        AttackKind::from_move_id(MoveId::CNullStepKick),
        AttackKind::Kick
    );
    assert_eq!(
        AttackKind::from_move_id(MoveId::CSegfaultSweep),
        AttackKind::Sweep
    );
    assert_eq!(
        AttackKind::from_move_id(MoveId::CStackOverflow),
        AttackKind::Overhead
    );
    assert_eq!(
        AttackKind::from_move_id(MoveId::CInterruptVector),
        AttackKind::AntiAir
    );
    assert_eq!(
        AttackKind::from_move_id(MoveId::CUndefinedThrow),
        AttackKind::Throw
    );
    assert_eq!(
        AttackKind::from_move_id(MoveId::PythonSnakeBite),
        AttackKind::LightPunch
    );
    assert_eq!(
        AttackKind::from_move_id(MoveId::PythonDataStrike),
        AttackKind::HeavyPunch
    );
    assert_eq!(
        AttackKind::from_move_id(MoveId::PythonHeelKick),
        AttackKind::Kick
    );
    assert_eq!(
        AttackKind::from_move_id(MoveId::PythonIndentSweep),
        AttackKind::Sweep
    );
    assert_eq!(
        AttackKind::from_move_id(MoveId::PythonTracebackOverhead),
        AttackKind::Overhead
    );
    assert_eq!(
        AttackKind::from_move_id(MoveId::PythonVisionAntiAir),
        AttackKind::AntiAir
    );
    assert_eq!(
        AttackKind::from_move_id(MoveId::PythonConstrictThrow),
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
