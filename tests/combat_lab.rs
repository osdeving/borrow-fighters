//! Exercises the Combat Lab scene without the Raylib renderer.

use borrow_fighters::characters::CharacterId;
use borrow_fighters::combat::fighter::{AttackPhase, Facing, FighterInput, PlayerSlot};
use borrow_fighters::combat::frame::FrameCount;
use borrow_fighters::config::{FIXED_TIMESTEP, world_px};
use borrow_fighters::engine::sprites::{SpriteManifest, projected_fighter_combat};
use borrow_fighters::game::world::{World, WorldSpriteCombatManifests};
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
fn entrance_defeat_and_crouched_guard_can_be_reviewed_and_restarted() {
    for (name, expected) in [
        ("spawn", CombatLabPose::Spawn),
        ("defeat", CombatLabPose::Defeat),
        ("crouch_block", CombatLabPose::CrouchBlock),
    ] {
        assert_eq!(CombatLabPose::from_cli(name), Some(expected));
        let mut lab = CombatLab::new(CombatLabOptions {
            pose: expected,
            ..CombatLabOptions::default()
        });
        lab.update(CombatLabInput::default());
        assert_eq!(lab.current_frame(), FrameCount::new(1));
        assert_eq!(lab.fighter().attack_phase(), AttackPhase::Idle);
        if expected == CombatLabPose::CrouchBlock {
            assert!(lab.fighter().blocking && lab.fighter().crouching);
        }
        lab.update(CombatLabInput {
            pause_toggle: true,
            reset: true,
            ..CombatLabInput::default()
        });
        assert_eq!(lab.current_frame(), FrameCount::ZERO);
    }
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
    assert_eq!(rust_advantage.whiff_recovery, FrameCount::new(4));
    assert_eq!(rust_advantage.hit_advantage, 0);
    assert_eq!(rust_advantage.block_advantage, -4);
    assert_eq!(rust_advantage.hit_pushback, world_px(22.0));
    assert_eq!(rust_advantage.block_pushback, world_px(14.0));
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
    assert_eq!(duke_advantage.whiff_recovery, FrameCount::new(12));
    assert_eq!(duke_advantage.hit_advantage, -9);
    assert_eq!(duke_advantage.block_advantage, -15);

    let go = CombatLab::new(CombatLabOptions {
        character: CharacterId::Go,
        selected_move: CombatLabMove::LightPunch,
        ..CombatLabOptions::default()
    });
    let go_advantage = go.advantage().expect("move playback should be analyzed");

    assert_eq!(go_advantage.contact_frame, FrameCount::new(3));
    assert_eq!(
        go_advantage.attacker_recovery_after_contact,
        FrameCount::new(11)
    );
    assert_eq!(go_advantage.whiff_recovery, FrameCount::new(3));
    assert_eq!(go_advantage.hit_advantage, -1);
    assert_eq!(go_advantage.block_advantage, -4);
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
    assert_eq!(advantage.whiff_recovery, FrameCount::ZERO);
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

#[test]
fn lab_projectile_emission_matches_world_with_baseline_metadata_and_fallback() {
    for character in [
        CharacterId::Rust,
        CharacterId::Duke,
        CharacterId::Go,
        CharacterId::C,
        CharacterId::Python,
        CharacterId::Cpp,
    ] {
        let baseline = SpriteManifest::load(format!(
            "assets/placeholder/{}-fighter.sprite.json",
            character.audio_key()
        ))
        .unwrap();
        let mut without_metadata = baseline.clone();
        for frame in &mut without_metadata.frames {
            frame.combat = None;
        }
        let mut centers = Vec::new();
        for manifest in [None, Some(baseline), Some(without_metadata)] {
            let mut lab = CombatLab::new(CombatLabOptions {
                character,
                selected_move: CombatLabMove::Projectile,
                ..CombatLabOptions::default()
            });
            lab.set_combat_manifest(manifest.clone());
            let mut world = World::new_with_characters(character, character);
            let slot = lab.fighter().slot;
            let opponent_x = match lab.fighter().facing {
                Facing::Right => 1100.0,
                Facing::Left => 0.0,
            };
            match slot {
                PlayerSlot::One => {
                    world.player_one = lab.fighter().clone();
                    world.player_two.position.x = opponent_x;
                }
                PlayerSlot::Two => {
                    world.player_two = lab.fighter().clone();
                    world.player_one.position.x = opponent_x;
                }
            }
            world.set_sprite_combat_manifests(WorldSpriteCombatManifests {
                player_one: manifest.clone(),
                player_two: manifest,
            });
            let fire = FighterInput {
                projectile: true,
                ..FighterInput::default()
            };
            let idle = FighterInput::default();
            let (one, two) = match slot {
                PlayerSlot::One => (fire, idle),
                PlayerSlot::Two => (idle, fire),
            };
            world.update(FIXED_TIMESTEP, one, two);
            lab.update(CombatLabInput::default());
            let projectile = &lab.projectiles()[0];
            assert_eq!(
                projectile.rect(),
                world.projectiles[0].rect(),
                "{character:?}"
            );
            assert_eq!(projectile.velocity, world.projectiles[0].velocity);
            assert_eq!(projectile.damage, world.projectiles[0].damage);
            centers.push(projectile.rect().center());
        }
        assert_eq!(
            centers[0], centers[2],
            "missing metadata must retain fallback"
        );
        if character == CharacterId::Python {
            assert_ne!(
                centers[0], centers[1],
                "baseline Python origin must override the old Lab default"
            );
        }
    }
}

#[test]
fn lab_overlays_use_baseline_boxes_and_restore_fallback_after_metadata_is_cleared() {
    let baseline = SpriteManifest::load("assets/placeholder/rust-fighter.sprite.json").unwrap();
    let mut metadata_frames = 0;
    for selected_move in [
        CombatLabMove::LightPunch,
        CombatLabMove::HeavyPunch,
        CombatLabMove::Kick,
    ] {
        let mut lab = CombatLab::new(CombatLabOptions {
            selected_move,
            ..CombatLabOptions::default()
        });
        lab.set_combat_manifest(Some(baseline.clone()));
        for _ in 0..40 {
            lab.update(CombatLabInput::default());
            if let Some(projected) =
                projected_fighter_combat(&baseline, lab.fighter(), lab.elapsed_seconds())
            {
                if !projected.hitboxes.is_empty() {
                    assert_eq!(lab.attack_boxes(), projected.hitboxes);
                    metadata_frames += 1;
                }
                if !projected.hurtboxes.is_empty() {
                    assert_eq!(lab.hurtboxes(), projected.hurtboxes);
                }
            }
        }
        lab.set_combat_manifest(None);
        lab.update(CombatLabInput {
            replay: true,
            ..CombatLabInput::default()
        });
        for _ in 0..20 {
            assert_eq!(
                lab.attack_boxes(),
                lab.fighter().attack_box().into_iter().collect::<Vec<_>>()
            );
            assert_eq!(lab.hurtboxes(), lab.fighter().hurtboxes().rects().to_vec());
            lab.update(CombatLabInput::default());
        }
    }
    assert!(
        metadata_frames > 0,
        "test must sample authored baseline boxes"
    );
}

#[test]
fn signature_lab_playback_uses_real_loadout_and_keeps_go_cycle_supported() {
    use borrow_fighters::combat::fighter::AttackKind;
    for character in [
        CharacterId::Rust,
        CharacterId::Duke,
        CharacterId::C,
        CharacterId::Python,
        CharacterId::Cpp,
    ] {
        let mut lab = CombatLab::new(CombatLabOptions {
            character,
            selected_move: CombatLabMove::SignatureSpecial,
            ..CombatLabOptions::default()
        });
        lab.update(CombatLabInput::default());
        assert_eq!(
            lab.fighter().attack_kind(),
            Some(AttackKind::SignatureSpecial)
        );
        lab.update(CombatLabInput {
            toggle_dummy: true,
            ..CombatLabInput::default()
        });
        assert!(lab.is_signature_actor_preview());
        assert!(
            lab.advantage().is_none(),
            "effect contacts need World simulation"
        );
        assert!(!lab.show_dummy());
        for _ in 0..120 {
            assert!(lab.attack_boxes().is_empty());
            lab.update(CombatLabInput::default());
        }
    }
    let mut go = CombatLab::new(CombatLabOptions {
        character: CharacterId::Go,
        ..CombatLabOptions::default()
    });
    go.update(CombatLabInput {
        previous_move: true,
        ..CombatLabInput::default()
    });
    assert_eq!(go.selected_move(), CombatLabMove::Projectile);
    go.update(CombatLabInput {
        next_move: true,
        ..CombatLabInput::default()
    });
    assert_eq!(go.selected_move(), CombatLabMove::LightPunch);
    assert_eq!(
        CombatLabMove::from_cli("signature_special"),
        Some(CombatLabMove::SignatureSpecial)
    );
    assert_eq!(
        CombatLabMove::from_cli("special"),
        Some(CombatLabMove::Projectile)
    );
}

#[test]
fn lab_sweeps_use_the_same_low_geometry_as_the_match_with_baseline_metadata() {
    for character in [
        CharacterId::Rust,
        CharacterId::Duke,
        CharacterId::C,
        CharacterId::Python,
        CharacterId::Cpp,
    ] {
        let baseline = SpriteManifest::load(format!(
            "assets/placeholder/{}-fighter.sprite.json",
            character.audio_key()
        ))
        .unwrap();
        let mut lab = CombatLab::new(CombatLabOptions {
            character,
            selected_move: CombatLabMove::Sweep,
            ..CombatLabOptions::default()
        });
        lab.set_combat_manifest(Some(baseline));
        let mut active_frames = 0;
        for _ in 0..70 {
            lab.update(CombatLabInput::default());
            if lab.fighter().active_hitbox().is_some() {
                active_frames += 1;
                assert_eq!(
                    lab.attack_boxes(),
                    lab.fighter().attack_box().into_iter().collect::<Vec<_>>()
                );
                assert_eq!(lab.hurtboxes(), lab.fighter().hurtboxes().rects());
                assert!(
                    lab.attack_boxes()
                        .iter()
                        .all(|area| area.y >= borrow_fighters::config::FLOOR_Y - world_px(43.0))
                );
            }
        }
        assert!(active_frames > 0, "{character:?}");
    }
}
