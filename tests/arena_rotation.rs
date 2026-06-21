//! Validates arena identity and rotation without opening a Raylib window.

use std::path::Path;

use borrow_fighters::game::arena::ArenaId;
use borrow_fighters::scenes::combat_lab::{CombatLab, CombatLabInput};

#[test]
fn arena_rotation_starts_at_sirius_and_cycles() {
    assert_eq!(ArenaId::STARTING_ARENA, ArenaId::Sirius);
    assert_eq!(
        ArenaId::ROTATION,
        [ArenaId::Sirius, ArenaId::Fortaleza, ArenaId::JavaStreet]
    );
    assert_eq!(ArenaId::Sirius.next(), ArenaId::Fortaleza);
    assert_eq!(ArenaId::Fortaleza.next(), ArenaId::JavaStreet);
    assert_eq!(ArenaId::JavaStreet.next(), ArenaId::Sirius);
}

#[test]
fn arena_placeholder_assets_exist() {
    for path in [
        "assets/placeholder/arena-sirius.png",
        "assets/placeholder/arena-fortaleza.png",
        "assets/placeholder/arena-java-street.png",
    ] {
        assert!(Path::new(path).exists(), "missing arena asset {path}");
    }
}

#[test]
fn combat_lab_background_can_be_toggled() {
    let mut lab = CombatLab::default();
    assert!(lab.show_background());

    lab.update(CombatLabInput {
        toggle_background: true,
        ..CombatLabInput::default()
    });
    assert!(!lab.show_background());

    lab.update(CombatLabInput {
        toggle_background: true,
        ..CombatLabInput::default()
    });
    assert!(lab.show_background());
}
