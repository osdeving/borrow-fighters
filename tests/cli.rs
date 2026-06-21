//! Verifies startup argument parsing without opening a Raylib window.

use borrow_fighters::cli::{LaunchMode, LaunchOptions};
use borrow_fighters::scenes::combat_lab::{CombatLabCharacter, CombatLabMove};

#[test]
fn no_args_start_regular_game() {
    let options = LaunchOptions::parse(["borrow-fighters"].map(String::from)).unwrap();

    assert_eq!(options.mode, LaunchMode::Game);
}

#[test]
fn combat_lab_args_select_character_and_move() {
    let options = LaunchOptions::parse(
        [
            "borrow-fighters",
            "--lab",
            "combat",
            "--character",
            "duke",
            "--move",
            "projectile",
        ]
        .map(String::from),
    )
    .unwrap();

    let LaunchMode::CombatLab(lab) = options.mode else {
        panic!("expected combat lab mode");
    };
    assert_eq!(lab.character, CombatLabCharacter::Duke);
    assert_eq!(lab.selected_move, CombatLabMove::Projectile);
}

#[test]
fn unknown_move_is_rejected() {
    let error = LaunchOptions::parse(
        [
            "borrow-fighters",
            "--lab",
            "combat",
            "--move",
            "segfault_uppercut",
        ]
        .map(String::from),
    )
    .unwrap_err();

    assert!(error.to_string().contains("unknown move"));
}
