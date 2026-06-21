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
        &[MoveId::RustBorrowJab, MoveId::HeavyPunch, MoveId::Kick]
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
            MoveId::Kick
        ]
    );
}

#[test]
fn character_cli_aliases_are_stable() {
    assert_eq!(CharacterId::from_cli("rust"), Some(CharacterId::Rust));
    assert_eq!(CharacterId::from_cli("rustacean"), Some(CharacterId::Rust));
    assert_eq!(CharacterId::from_cli("duke"), Some(CharacterId::Duke));
    assert_eq!(CharacterId::from_cli("java"), Some(CharacterId::Duke));
    assert_eq!(CharacterId::from_cli("go"), None);
}
