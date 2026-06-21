//! Exercises the Combat Lab scene without the Raylib renderer.

use borrow_fighters::characters::CharacterId;
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

#[test]
fn lab_reports_character_specific_close_move_advantage() {
    let rust = CombatLab::new(CombatLabOptions {
        character: CharacterId::Rust,
        selected_move: CombatLabMove::LightPunch,
        ..CombatLabOptions::default()
    });
    let rust_advantage = rust.advantage().expect("move playback should be analyzed");

    assert_eq!(rust_advantage.contact_frame, FrameCount::new(4));
    assert_eq!(
        rust_advantage.attacker_recovery_after_contact,
        FrameCount::new(12)
    );
    assert_eq!(rust_advantage.hit_advantage, 0);
    assert_eq!(rust_advantage.block_advantage, -4);
    assert_eq!(rust_advantage.hit_pushback, 22.0);
    assert_eq!(rust_advantage.block_pushback, 14.0);
    assert!(
        rust_advantage.hit_body_gap_after_pushback > rust_advantage.block_body_gap_after_pushback
    );

    let duke = CombatLab::new(CombatLabOptions {
        character: CharacterId::Duke,
        selected_move: CombatLabMove::HeavyPunch,
        ..CombatLabOptions::default()
    });
    let duke_advantage = duke.advantage().expect("move playback should be analyzed");

    assert_eq!(duke_advantage.contact_frame, FrameCount::new(13));
    assert_eq!(
        duke_advantage.attacker_recovery_after_contact,
        FrameCount::new(27)
    );
    assert_eq!(duke_advantage.hit_advantage, -9);
    assert_eq!(duke_advantage.block_advantage, -15);
}

#[test]
fn lab_reports_projectile_advantage_without_action_recovery() {
    let lab = CombatLab::new(CombatLabOptions {
        selected_move: CombatLabMove::Projectile,
        ..CombatLabOptions::default()
    });
    let advantage = lab.advantage().expect("projectile should be analyzed");

    assert_eq!(advantage.contact_frame, FrameCount::ZERO);
    assert_eq!(advantage.attacker_recovery_after_contact, FrameCount::ZERO);
    assert_eq!(
        advantage.projectile_cooldown_after_contact,
        FrameCount::new(57)
    );
    assert_eq!(advantage.hit_advantage, 16);
    assert_eq!(advantage.block_advantage, 12);
}

#[test]
fn lab_advantage_is_only_for_move_playback() {
    let lab = CombatLab::new(CombatLabOptions {
        pose: CombatLabPose::Idle,
        ..CombatLabOptions::default()
    });

    assert!(lab.advantage().is_none());
}

#[test]
fn lab_contact_dummy_tracks_selected_move_reach() {
    let light = CombatLab::new(CombatLabOptions {
        selected_move: CombatLabMove::LightPunch,
        ..CombatLabOptions::default()
    });
    let heavy = CombatLab::new(CombatLabOptions {
        selected_move: CombatLabMove::HeavyPunch,
        ..CombatLabOptions::default()
    });

    assert!(heavy.dummy_body_rect().x > light.dummy_body_rect().x);
}
