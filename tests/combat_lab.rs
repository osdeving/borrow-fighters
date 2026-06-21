//! Exercises the Combat Lab scene without the Raylib renderer.

use borrow_fighters::combat::fighter::AttackPhase;
use borrow_fighters::combat::frame::FrameCount;
use borrow_fighters::scenes::combat_lab::{
    CombatLab, CombatLabInput, CombatLabMove, CombatLabOptions,
};

#[test]
fn lab_advances_selected_close_move_in_frames() {
    let mut lab = CombatLab::new(CombatLabOptions {
        selected_move: CombatLabMove::LightPunch,
        ..CombatLabOptions::default()
    });

    lab.update(CombatLabInput::default());

    assert_eq!(lab.current_frame(), FrameCount::new(1));
    assert_eq!(lab.fighter().attack_phase(), AttackPhase::Startup);
    assert_eq!(
        lab.fighter().attack_elapsed_frames(),
        Some(FrameCount::new(1))
    );
}

#[test]
fn pause_and_step_advance_exactly_one_frame() {
    let mut lab = CombatLab::default();

    lab.update(CombatLabInput {
        pause_toggle: true,
        ..CombatLabInput::default()
    });
    assert!(lab.paused());
    assert_eq!(lab.current_frame(), FrameCount::ZERO);

    lab.update(CombatLabInput {
        step_frame: true,
        ..CombatLabInput::default()
    });
    assert_eq!(lab.current_frame(), FrameCount::new(1));

    lab.update(CombatLabInput::default());
    assert_eq!(lab.current_frame(), FrameCount::new(1));
}

#[test]
fn move_selection_restarts_playback() {
    let mut lab = CombatLab::default();

    lab.update(CombatLabInput::default());
    assert_eq!(lab.current_frame(), FrameCount::new(1));

    lab.update(CombatLabInput {
        next_move: true,
        pause_toggle: true,
        ..CombatLabInput::default()
    });

    assert_eq!(lab.selected_move(), CombatLabMove::HeavyPunch);
    assert_eq!(lab.current_frame(), FrameCount::ZERO);
    assert!(lab.paused());
}

#[test]
fn projectile_move_spawns_projectile_and_special_timing() {
    let mut lab = CombatLab::new(CombatLabOptions {
        selected_move: CombatLabMove::Projectile,
        ..CombatLabOptions::default()
    });

    lab.update(CombatLabInput::default());

    assert_eq!(lab.current_frame(), FrameCount::new(1));
    assert_eq!(lab.projectiles().len(), 1);
    assert_eq!(
        lab.fighter().special_elapsed_frames(),
        Some(FrameCount::new(1))
    );
    assert!(!lab.fighter().can_fire_projectile());
}
