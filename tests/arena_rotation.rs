//! Validates arena identity and rotation without opening a Raylib window.

use std::path::Path;

use borrow_fighters::game::arena::ArenaId;
use borrow_fighters::scenes::combat_lab::{CombatLab, CombatLabInput};

#[test]
fn arena_rotation_starts_at_sirius_and_cycles() {
    assert_eq!(ArenaId::STARTING_ARENA, ArenaId::Sirius);
    assert_eq!(
        ArenaId::ROTATION,
        [
            ArenaId::Sirius,
            ArenaId::Fortaleza,
            ArenaId::JavaStreet,
            ArenaId::BioTic,
            ArenaId::PortoDigital,
            ArenaId::ValeDoPinhao,
        ]
    );
    assert_eq!(ArenaId::Sirius.next(), ArenaId::Fortaleza);
    assert_eq!(ArenaId::Fortaleza.next(), ArenaId::JavaStreet);
    assert_eq!(ArenaId::JavaStreet.next(), ArenaId::BioTic);
    assert_eq!(ArenaId::BioTic.next(), ArenaId::PortoDigital);
    assert_eq!(ArenaId::PortoDigital.next(), ArenaId::ValeDoPinhao);
    assert_eq!(ArenaId::ValeDoPinhao.next(), ArenaId::Sirius);
    assert_eq!(ArenaId::Sirius.previous(), ArenaId::ValeDoPinhao);
    assert_eq!(ArenaId::BioTic.previous(), ArenaId::JavaStreet);
}

#[test]
fn arenas_have_context_names_and_locations() {
    for arena in ArenaId::ROTATION {
        assert!(!arena.label().is_empty());
        assert!(!arena.location().is_empty());
        assert!(!arena.concept().is_empty());
    }

    assert_eq!(ArenaId::Sirius.label(), "Sirius Light Ring");
    assert_eq!(ArenaId::Sirius.location(), "Campinas, SP");
    assert_eq!(ArenaId::PortoDigital.label(), "Porto Digital Cache");
    assert_eq!(ArenaId::PortoDigital.location(), "Recife, PE");
}

#[test]
fn arena_placeholder_assets_exist() {
    for path in [
        "assets/placeholder/arena-sirius.png",
        "assets/placeholder/arena-fortaleza.png",
        "assets/placeholder/arena-java-street.png",
        "assets/placeholder/arena-biotic.png",
        "assets/placeholder/arena-porto-digital.png",
        "assets/placeholder/arena-vale-pinhao.png",
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
