//! Exercises the Combat Lab scene without the Raylib renderer.

use borrow_fighters::combat::fighter::AttackPhase;
use borrow_fighters::combat::frame::FrameCount;
use borrow_fighters::scenes::combat_lab::{
    CombatLab, CombatLabInput, CombatLabMove, CombatLabOptions, CombatLabPose,
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
fn pose_selection_restarts_playback() {
    let mut lab = CombatLab::default();

    lab.update(CombatLabInput::default());
    assert_eq!(lab.current_frame(), FrameCount::new(1));

    lab.update(CombatLabInput {
        next_pose: true,
        ..CombatLabInput::default()
    });

    assert_eq!(lab.pose(), CombatLabPose::Idle);
    assert_eq!(lab.current_frame(), FrameCount::new(1));
    assert_eq!(lab.fighter().attack_phase(), AttackPhase::Idle);
}

#[test]
fn static_poses_keep_expected_fighter_state() {
    let mut crouch = CombatLab::new(CombatLabOptions {
        pose: CombatLabPose::Crouch,
        ..CombatLabOptions::default()
    });
    assert!(crouch.fighter().crouching);
    let crouch_height = crouch.fighter().hurtbox().height;
    crouch.update(CombatLabInput::default());
    assert_eq!(crouch.current_frame(), FrameCount::new(1));
    assert!(crouch.fighter().crouching);
    assert_eq!(crouch.fighter().hurtbox().height, crouch_height);

    let jump = CombatLab::new(CombatLabOptions {
        pose: CombatLabPose::Jump,
        ..CombatLabOptions::default()
    });
    assert!(!jump.fighter().grounded);

    let block = CombatLab::new(CombatLabOptions {
        pose: CombatLabPose::Block,
        ..CombatLabOptions::default()
    });
    assert!(block.fighter().blocking);
}

#[test]
fn pose_cli_aliases_are_stable() {
    assert_eq!(CombatLabPose::from_cli("move"), Some(CombatLabPose::Move));
    assert_eq!(
        CombatLabPose::from_cli("playback"),
        Some(CombatLabPose::Move)
    );
    assert_eq!(
        CombatLabPose::from_cli("crouch"),
        Some(CombatLabPose::Crouch)
    );
    assert_eq!(CombatLabPose::from_cli("air"), Some(CombatLabPose::Jump));
    assert_eq!(CombatLabPose::from_cli("guard"), Some(CombatLabPose::Block));
    assert_eq!(CombatLabPose::from_cli("hurt"), Some(CombatLabPose::Hit));
    assert_eq!(
        CombatLabPose::from_cli("taunt"),
        Some(CombatLabPose::Victory)
    );
    assert_eq!(CombatLabPose::from_cli("teleport"), None);
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
