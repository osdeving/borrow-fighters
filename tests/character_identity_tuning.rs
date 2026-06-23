//! Verifies early character identity tuning without growing broad combat tests.

use borrow_fighters::characters::{CharacterId, character_spec};
use borrow_fighters::combat::move_data::{MoveId, MoveInputKind, move_spec, move_spec_for_input};

#[test]
fn characters_resolve_identity_moves_from_shared_inputs() {
    let rust = character_spec(CharacterId::Rust);
    let duke = character_spec(CharacterId::Duke);
    let go = character_spec(CharacterId::Go);
    let c = character_spec(CharacterId::C);
    let python = character_spec(CharacterId::Python);

    assert_eq!(
        move_spec_for_input(rust.move_ids, MoveInputKind::LightPunch).map(|spec| spec.id),
        Some(MoveId::RustBorrowJab)
    );
    assert_eq!(
        move_spec_for_input(rust.move_ids, MoveInputKind::AntiAir).map(|spec| spec.id),
        Some(MoveId::RustLifetimeAntiAir)
    );
    assert_eq!(
        move_spec_for_input(rust.move_ids, MoveInputKind::Throw).map(|spec| spec.id),
        Some(MoveId::RustOwnershipThrow)
    );

    assert_eq!(
        move_spec_for_input(duke.move_ids, MoveInputKind::HeavyPunch).map(|spec| spec.id),
        Some(MoveId::DukeBoilerplatePoke)
    );
    assert_eq!(
        move_spec_for_input(duke.move_ids, MoveInputKind::Sweep).map(|spec| spec.id),
        Some(MoveId::DukeGarbageCollectorSweep)
    );
    assert_eq!(
        move_spec_for_input(duke.move_ids, MoveInputKind::Overhead).map(|spec| spec.id),
        Some(MoveId::DukeAbstractFactoryOverhead)
    );
    assert_eq!(
        move_spec_for_input(duke.move_ids, MoveInputKind::Throw).map(|spec| spec.id),
        Some(MoveId::DukeEnterpriseThrow)
    );

    assert_eq!(
        move_spec_for_input(go.move_ids, MoveInputKind::LightPunch).map(|spec| spec.id),
        Some(MoveId::GoGoroutineJab)
    );
    assert_eq!(
        move_spec_for_input(go.move_ids, MoveInputKind::Kick).map(|spec| spec.id),
        Some(MoveId::GoDeferKick)
    );
    assert_eq!(
        move_spec_for_input(go.move_ids, MoveInputKind::Overhead).map(|spec| spec.id),
        Some(MoveId::GoChannelOverhead)
    );
    assert_eq!(
        move_spec_for_input(go.move_ids, MoveInputKind::AirKick).map(|spec| spec.id),
        Some(MoveId::GoHopkick)
    );

    assert_eq!(
        move_spec_for_input(c.move_ids, MoveInputKind::LightPunch).map(|spec| spec.id),
        Some(MoveId::CPointerJab)
    );
    assert_eq!(
        move_spec_for_input(c.move_ids, MoveInputKind::HeavyPunch).map(|spec| spec.id),
        Some(MoveId::CUnsafePoke)
    );
    assert_eq!(
        move_spec_for_input(c.move_ids, MoveInputKind::Kick).map(|spec| spec.id),
        Some(MoveId::CNullStepKick)
    );
    assert_eq!(
        move_spec_for_input(c.move_ids, MoveInputKind::Sweep).map(|spec| spec.id),
        Some(MoveId::CSegfaultSweep)
    );
    assert_eq!(
        move_spec_for_input(c.move_ids, MoveInputKind::Overhead).map(|spec| spec.id),
        Some(MoveId::CStackOverflow)
    );
    assert_eq!(
        move_spec_for_input(c.move_ids, MoveInputKind::AntiAir).map(|spec| spec.id),
        Some(MoveId::CInterruptVector)
    );
    assert_eq!(
        move_spec_for_input(c.move_ids, MoveInputKind::Throw).map(|spec| spec.id),
        Some(MoveId::CUndefinedThrow)
    );

    assert_eq!(
        move_spec_for_input(python.move_ids, MoveInputKind::LightPunch).map(|spec| spec.id),
        Some(MoveId::PythonSnakeBite)
    );
    assert_eq!(
        move_spec_for_input(python.move_ids, MoveInputKind::HeavyPunch).map(|spec| spec.id),
        Some(MoveId::PythonDataStrike)
    );
    assert_eq!(
        move_spec_for_input(python.move_ids, MoveInputKind::Kick).map(|spec| spec.id),
        Some(MoveId::PythonHeelKick)
    );
    assert_eq!(
        move_spec_for_input(python.move_ids, MoveInputKind::Sweep).map(|spec| spec.id),
        Some(MoveId::PythonIndentSweep)
    );
    assert_eq!(
        move_spec_for_input(python.move_ids, MoveInputKind::Overhead).map(|spec| spec.id),
        Some(MoveId::PythonTracebackOverhead)
    );
    assert_eq!(
        move_spec_for_input(python.move_ids, MoveInputKind::AntiAir).map(|spec| spec.id),
        Some(MoveId::PythonVisionAntiAir)
    );
    assert_eq!(
        move_spec_for_input(python.move_ids, MoveInputKind::Throw).map(|spec| spec.id),
        Some(MoveId::PythonConstrictThrow)
    );
}

#[test]
fn rust_defensive_tools_are_faster_but_smaller_than_generic_tools() {
    let generic_anti_air = move_spec(MoveId::RisingAntiAir);
    let rust_anti_air = move_spec(MoveId::RustLifetimeAntiAir);
    let generic_throw = move_spec(MoveId::CloseThrow);
    let rust_throw = move_spec(MoveId::RustOwnershipThrow);

    assert!(rust_anti_air.frames.active_start < generic_anti_air.frames.active_start);
    assert!(rust_anti_air.frames.duration < generic_anti_air.frames.duration);
    assert!(rust_anti_air.whiff_recovery < generic_anti_air.whiff_recovery);
    assert!(rust_anti_air.hitbox.width < generic_anti_air.hitbox.width);
    assert!(rust_anti_air.damage < generic_anti_air.damage);

    assert!(rust_throw.frames.active_start < generic_throw.frames.active_start);
    assert!(rust_throw.frames.duration < generic_throw.frames.duration);
    assert!(rust_throw.whiff_recovery < generic_throw.whiff_recovery);
    assert!(rust_throw.hitbox.width < generic_throw.hitbox.width);
    assert!(rust_throw.damage < generic_throw.damage);
}

#[test]
fn duke_pressure_tools_trade_speed_for_reach_and_damage() {
    let generic_sweep = move_spec(MoveId::SweepKick);
    let duke_sweep = move_spec(MoveId::DukeGarbageCollectorSweep);
    let generic_overhead = move_spec(MoveId::OverheadPunch);
    let duke_overhead = move_spec(MoveId::DukeAbstractFactoryOverhead);
    let generic_throw = move_spec(MoveId::CloseThrow);
    let duke_throw = move_spec(MoveId::DukeEnterpriseThrow);

    assert!(duke_sweep.hitbox.width > generic_sweep.hitbox.width);
    assert!(duke_sweep.damage > generic_sweep.damage);
    assert!(duke_sweep.frames.active_start > generic_sweep.frames.active_start);
    assert!(duke_sweep.whiff_recovery > generic_sweep.whiff_recovery);

    assert!(duke_overhead.hitbox.width > generic_overhead.hitbox.width);
    assert!(duke_overhead.damage > generic_overhead.damage);
    assert!(duke_overhead.frames.active_start > generic_overhead.frames.active_start);
    assert!(duke_overhead.whiff_recovery > generic_overhead.whiff_recovery);

    assert!(duke_throw.hitbox.width > generic_throw.hitbox.width);
    assert!(duke_throw.damage > generic_throw.damage);
    assert!(duke_throw.frames.active_start > generic_throw.frames.active_start);
    assert!(duke_throw.whiff_recovery > generic_throw.whiff_recovery);
}

#[test]
fn go_rushdown_tools_trade_durability_and_reach_for_speed() {
    let rust = character_spec(CharacterId::Rust);
    let duke = character_spec(CharacterId::Duke);
    let go = character_spec(CharacterId::Go);
    let generic_light = move_spec(MoveId::LightPunch);
    let go_jab = move_spec(MoveId::GoGoroutineJab);
    let generic_kick = move_spec(MoveId::Kick);
    let go_kick = move_spec(MoveId::GoDeferKick);
    let generic_overhead = move_spec(MoveId::OverheadPunch);
    let go_overhead = move_spec(MoveId::GoChannelOverhead);
    let generic_air_kick = move_spec(MoveId::AirKick);
    let go_air_kick = move_spec(MoveId::GoHopkick);

    assert!(go.stats.max_health < rust.stats.max_health);
    assert!(go.stats.max_health < duke.stats.max_health);

    assert!(go_jab.frames.active_start < generic_light.frames.active_start);
    assert!(go_jab.frames.duration < generic_light.frames.duration);
    assert!(go_jab.whiff_recovery < generic_light.whiff_recovery);
    assert!(go_jab.hitbox.width < generic_light.hitbox.width);

    assert!(go_kick.frames.active_start < generic_kick.frames.active_start);
    assert!(go_kick.frames.duration < generic_kick.frames.duration);
    assert!(go_kick.whiff_recovery < generic_kick.whiff_recovery);
    assert!(go_kick.hitbox.width < generic_kick.hitbox.width);

    assert!(go_overhead.frames.active_start < generic_overhead.frames.active_start);
    assert!(go_overhead.frames.duration < generic_overhead.frames.duration);
    assert!(go_overhead.hitbox.width < generic_overhead.hitbox.width);

    assert!(go_air_kick.frames.active_start < generic_air_kick.frames.active_start);
    assert!(go_air_kick.frames.duration < generic_air_kick.frames.duration);
    assert!(go_air_kick.whiff_recovery < generic_air_kick.whiff_recovery);
    assert!(go_air_kick.hitbox.width < generic_air_kick.hitbox.width);
}

#[test]
fn c_fundamentals_tools_trade_speed_for_reach_and_stability() {
    let rust = character_spec(CharacterId::Rust);
    let c = character_spec(CharacterId::C);
    let generic_light = move_spec(MoveId::LightPunch);
    let c_light = move_spec(MoveId::CPointerJab);
    let generic_heavy = move_spec(MoveId::HeavyPunch);
    let c_heavy = move_spec(MoveId::CUnsafePoke);
    let generic_sweep = move_spec(MoveId::SweepKick);
    let c_sweep = move_spec(MoveId::CSegfaultSweep);
    let generic_throw = move_spec(MoveId::CloseThrow);
    let c_throw = move_spec(MoveId::CUndefinedThrow);

    assert!(c.stats.max_health > rust.stats.max_health);

    assert_eq!(
        c_light.frames.active_start,
        generic_light.frames.active_start
    );
    assert!(c_light.hitbox.width > generic_light.hitbox.width);
    assert!(c_light.whiff_recovery > generic_light.whiff_recovery);

    assert!(c_heavy.hitbox.width > generic_heavy.hitbox.width);
    assert!(c_heavy.damage > generic_heavy.damage);
    assert!(c_heavy.frames.active_start > generic_heavy.frames.active_start);
    assert!(c_heavy.whiff_recovery > generic_heavy.whiff_recovery);

    assert!(c_sweep.hitbox.width > generic_sweep.hitbox.width);
    assert!(c_sweep.damage > generic_sweep.damage);
    assert!(c_sweep.whiff_recovery > generic_sweep.whiff_recovery);

    assert!(c_throw.damage > generic_throw.damage);
    assert!(c_throw.hit_reaction.hit_pushback > generic_throw.hit_reaction.hit_pushback);
    assert!(c_throw.whiff_recovery > generic_throw.whiff_recovery);
}

#[test]
fn python_agile_tools_favor_fast_punishes_over_raw_damage() {
    let c = character_spec(CharacterId::C);
    let python = character_spec(CharacterId::Python);
    let generic_heavy = move_spec(MoveId::HeavyPunch);
    let python_heavy = move_spec(MoveId::PythonDataStrike);
    let generic_kick = move_spec(MoveId::Kick);
    let python_kick = move_spec(MoveId::PythonHeelKick);
    let generic_sweep = move_spec(MoveId::SweepKick);
    let python_sweep = move_spec(MoveId::PythonIndentSweep);
    let generic_throw = move_spec(MoveId::CloseThrow);
    let python_throw = move_spec(MoveId::PythonConstrictThrow);

    assert!(python.stats.max_health < c.stats.max_health);

    assert!(python_heavy.frames.active_start < generic_heavy.frames.active_start);
    assert!(python_heavy.frames.duration < generic_heavy.frames.duration);
    assert!(python_heavy.damage < generic_heavy.damage);
    assert!(python_heavy.whiff_recovery < generic_heavy.whiff_recovery);

    assert!(python_kick.frames.active_start < generic_kick.frames.active_start);
    assert!(python_kick.frames.duration < generic_kick.frames.duration);
    assert!(python_kick.damage < generic_kick.damage);
    assert!(python_kick.whiff_recovery < generic_kick.whiff_recovery);

    assert!(python_sweep.frames.active_start < generic_sweep.frames.active_start);
    assert!(python_sweep.frames.duration < generic_sweep.frames.duration);
    assert!(python_sweep.damage < generic_sweep.damage);
    assert!(python_sweep.whiff_recovery < generic_sweep.whiff_recovery);

    assert_eq!(python_throw.damage, generic_throw.damage);
    assert!(python_throw.frames.duration > generic_throw.frames.duration);
    assert!(python_throw.whiff_recovery < generic_throw.whiff_recovery);
}
