//! Verifies startup argument parsing without opening a Raylib window.

use borrow_fighters::characters::CharacterId;
use borrow_fighters::cli::{LaunchMode, LaunchOptions, MatchOptions};
use borrow_fighters::scenes::combat_lab::{CombatLabMove, CombatLabPose};

#[test]
fn no_args_start_regular_game() {
    let options = LaunchOptions::parse(["borrow-fighters"].map(String::from)).unwrap();

    assert_eq!(options.mode, LaunchMode::Game);
    assert_eq!(options.match_options, MatchOptions::default());
    assert!(!options.start_fight);
}

#[test]
fn game_args_select_match_characters() {
    let options =
        LaunchOptions::parse(["borrow-fighters", "--p1", "go", "--p2", "rust"].map(String::from))
            .unwrap();

    assert_eq!(options.mode, LaunchMode::Game);
    assert_eq!(options.match_options.player_one, CharacterId::Go);
    assert_eq!(options.match_options.player_two, CharacterId::Rust);
}

#[test]
fn game_args_accept_long_character_flags() {
    let options = LaunchOptions::parse(
        [
            "borrow-fighters",
            "--player-one",
            "golang",
            "--player-two",
            "java",
        ]
        .map(String::from),
    )
    .unwrap();

    assert_eq!(options.match_options.player_one, CharacterId::Go);
    assert_eq!(options.match_options.player_two, CharacterId::Duke);
}

#[test]
fn game_args_accept_c_character_aliases() {
    let options = LaunchOptions::parse(
        [
            "borrow-fighters",
            "--player-one",
            "langc",
            "--player-two",
            "clang",
        ]
        .map(String::from),
    )
    .unwrap();

    assert_eq!(options.match_options.player_one, CharacterId::C);
    assert_eq!(options.match_options.player_two, CharacterId::C);
}

#[test]
fn game_args_can_start_directly_in_fight() {
    let options = LaunchOptions::parse(
        ["borrow-fighters", "--fight", "--p1", "go", "--p2", "duke"].map(String::from),
    )
    .unwrap();

    assert_eq!(options.mode, LaunchMode::Game);
    assert!(options.start_fight);
    assert_eq!(options.match_options.player_one, CharacterId::Go);
    assert_eq!(options.match_options.player_two, CharacterId::Duke);
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
    assert_eq!(lab.character, CharacterId::Duke);
    assert_eq!(lab.selected_move, CombatLabMove::Projectile);
    assert_eq!(lab.pose, CombatLabPose::Move);
}

#[test]
fn combat_lab_args_accept_traditional_move_aliases() {
    for (raw_move, expected) in [
        ("sweep", CombatLabMove::Sweep),
        ("overhead", CombatLabMove::Overhead),
        ("anti-air", CombatLabMove::AntiAir),
        ("air_kick", CombatLabMove::AirKick),
        ("throw", CombatLabMove::Throw),
    ] {
        let options = LaunchOptions::parse(
            ["borrow-fighters", "--lab", "combat", "--move", raw_move].map(String::from),
        )
        .unwrap();

        let LaunchMode::CombatLab(lab) = options.mode else {
            panic!("expected combat lab mode");
        };
        assert_eq!(lab.selected_move, expected);
    }
}

#[test]
fn combat_lab_args_select_static_pose() {
    let options = LaunchOptions::parse(
        [
            "borrow-fighters",
            "--lab",
            "combat",
            "--character",
            "rust",
            "--pose",
            "block",
        ]
        .map(String::from),
    )
    .unwrap();

    let LaunchMode::CombatLab(lab) = options.mode else {
        panic!("expected combat lab mode");
    };
    assert_eq!(lab.character, CharacterId::Rust);
    assert_eq!(lab.pose, CombatLabPose::Block);
}

#[test]
fn combat_lab_args_select_go_character() {
    let options = LaunchOptions::parse(
        ["borrow-fighters", "--lab", "combat", "--character", "go"].map(String::from),
    )
    .unwrap();

    let LaunchMode::CombatLab(lab) = options.mode else {
        panic!("expected combat lab mode");
    };
    assert_eq!(lab.character, CharacterId::Go);
}

#[test]
fn combat_lab_args_select_c_character() {
    let options = LaunchOptions::parse(
        ["borrow-fighters", "--lab", "combat", "--character", "c"].map(String::from),
    )
    .unwrap();

    let LaunchMode::CombatLab(lab) = options.mode else {
        panic!("expected combat lab mode");
    };
    assert_eq!(lab.character, CharacterId::C);
}

#[test]
fn sprite_viewer_args_select_manifest_and_clip() {
    let options = LaunchOptions::parse(
        [
            "borrow-fighters",
            "--tool",
            "sprite-viewer",
            "--manifest",
            "assets/placeholder/rust-fighter.sprite.json",
            "--clip",
            "idle",
        ]
        .map(String::from),
    )
    .unwrap();

    let LaunchMode::SpriteViewer(viewer) = options.mode else {
        panic!("expected sprite viewer mode");
    };
    assert_eq!(
        viewer.manifest_path,
        std::path::PathBuf::from("assets/placeholder/rust-fighter.sprite.json")
    );
    assert_eq!(viewer.initial_clip.as_deref(), Some("idle"));
    assert_eq!(viewer.character, Some(CharacterId::Rust));
    assert_eq!(viewer.selected_move, CombatLabMove::LightPunch);
}

#[test]
fn sprite_viewer_infers_c_from_manifest_path() {
    let options = LaunchOptions::parse(
        [
            "borrow-fighters",
            "--tool",
            "sprite-viewer",
            "--manifest",
            "assets/placeholder/c-fighter.sprite.json",
        ]
        .map(String::from),
    )
    .unwrap();

    let LaunchMode::SpriteViewer(viewer) = options.mode else {
        panic!("expected sprite viewer mode");
    };
    assert_eq!(viewer.character, Some(CharacterId::C));
}

#[test]
fn sprite_viewer_args_accept_character_and_move_for_combat_overlay() {
    let options = LaunchOptions::parse(
        [
            "borrow-fighters",
            "--tool",
            "sprite-viewer",
            "--manifest",
            "assets/placeholder/duke-fighter.sprite.json",
            "--character",
            "duke",
            "--move",
            "projectile",
        ]
        .map(String::from),
    )
    .unwrap();

    let LaunchMode::SpriteViewer(viewer) = options.mode else {
        panic!("expected sprite viewer mode");
    };
    assert_eq!(viewer.character, Some(CharacterId::Duke));
    assert_eq!(viewer.selected_move, CombatLabMove::Projectile);
}

#[test]
fn sprite_viewer_requires_manifest() {
    let error =
        LaunchOptions::parse(["borrow-fighters", "--tool", "sprite-viewer"].map(String::from))
            .unwrap_err();

    assert!(error.to_string().contains("requires --manifest"));
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

#[test]
fn unknown_match_character_is_rejected() {
    let error = LaunchOptions::parse(["borrow-fighters", "--p1", "segfault"].map(String::from))
        .unwrap_err();

    assert!(error.to_string().contains("unknown player one character"));
}

#[test]
fn unknown_pose_is_rejected() {
    let error = LaunchOptions::parse(
        ["borrow-fighters", "--lab", "combat", "--pose", "ragdoll"].map(String::from),
    )
    .unwrap_err();

    assert!(error.to_string().contains("unknown pose"));
}
