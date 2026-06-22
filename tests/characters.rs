//! Verifies prototype character registry data.

use borrow_fighters::characters::{CharacterArchetype, CharacterId, character_spec};
use borrow_fighters::combat::move_data::MoveId;

#[test]
fn rust_spec_points_to_current_prototype_moves() {
    let rust = character_spec(CharacterId::Rust);

    assert_eq!(rust.display_name, "Rust");
    assert_eq!(rust.fighter_name, "Rust");
    assert_eq!(rust.archetype, CharacterArchetype::AllRounder);
    assert_eq!(rust.stats.max_health, 100);
    assert_eq!(
        rust.move_ids,
        &[
            MoveId::RustBorrowJab,
            MoveId::HeavyPunch,
            MoveId::Kick,
            MoveId::SweepKick,
            MoveId::OverheadPunch,
            MoveId::RustLifetimeAntiAir,
            MoveId::AirPunch,
            MoveId::AirKick,
            MoveId::RustOwnershipThrow,
        ]
    );
}

#[test]
fn duke_spec_points_to_current_prototype_moves() {
    let duke = character_spec(CharacterId::Duke);

    assert_eq!(duke.display_name, "Duke / Java");
    assert_eq!(duke.fighter_name, "Java");
    assert_eq!(duke.archetype, CharacterArchetype::MidrangePressure);
    assert_eq!(duke.stats.max_health, 112);
    assert_eq!(
        duke.move_ids,
        &[
            MoveId::LightPunch,
            MoveId::DukeBoilerplatePoke,
            MoveId::Kick,
            MoveId::DukeGarbageCollectorSweep,
            MoveId::DukeAbstractFactoryOverhead,
            MoveId::RisingAntiAir,
            MoveId::AirPunch,
            MoveId::AirKick,
            MoveId::DukeEnterpriseThrow,
        ]
    );
}

#[test]
fn go_spec_points_to_current_prototype_moves() {
    let go = character_spec(CharacterId::Go);

    assert_eq!(go.display_name, "Go");
    assert_eq!(go.fighter_name, "Go");
    assert_eq!(go.archetype, CharacterArchetype::Rushdown);
    assert_eq!(go.stats.max_health, 92);
    assert_eq!(
        go.move_ids,
        &[
            MoveId::GoGoroutineJab,
            MoveId::HeavyPunch,
            MoveId::GoDeferKick,
            MoveId::SweepKick,
            MoveId::GoChannelOverhead,
            MoveId::RisingAntiAir,
            MoveId::AirPunch,
            MoveId::GoHopkick,
            MoveId::CloseThrow,
        ]
    );
}

#[test]
fn character_cli_aliases_are_stable() {
    assert_eq!(CharacterId::from_cli("rust"), Some(CharacterId::Rust));
    assert_eq!(CharacterId::from_cli("rustacean"), Some(CharacterId::Rust));
    assert_eq!(CharacterId::from_cli("duke"), Some(CharacterId::Duke));
    assert_eq!(CharacterId::from_cli("java"), Some(CharacterId::Duke));
    assert_eq!(CharacterId::from_cli("go"), Some(CharacterId::Go));
    assert_eq!(CharacterId::from_cli("golang"), Some(CharacterId::Go));
    assert_eq!(CharacterId::from_cli("gopher"), Some(CharacterId::Go));
    assert_eq!(CharacterId::from_audio_key("go"), Some(CharacterId::Go));
}

#[test]
fn character_roster_cycles_in_menu_order() {
    assert_eq!(CharacterId::Rust.next(), CharacterId::Duke);
    assert_eq!(CharacterId::Duke.next(), CharacterId::Go);
    assert_eq!(CharacterId::Go.next(), CharacterId::Rust);
    assert_eq!(CharacterId::Rust.previous(), CharacterId::Go);
    assert_eq!(CharacterId::Duke.previous(), CharacterId::Rust);
    assert_eq!(CharacterId::Go.previous(), CharacterId::Duke);
}
