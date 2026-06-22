//! Verifies the standalone sprite viewer state without opening a window.

use std::path::PathBuf;

use borrow_fighters::{
    characters::CharacterId,
    scenes::{
        combat_lab::CombatLabMove,
        sprite_viewer::{
            SpriteTimelinePhase, SpriteViewer, SpriteViewerInput, SpriteViewerOptions, ViewerPoint,
        },
    },
};

#[test]
fn loads_manifest_and_selects_requested_clip() {
    let viewer = SpriteViewer::load(SpriteViewerOptions {
        manifest_path: PathBuf::from("assets/placeholder/rust-fighter.sprite.json"),
        initial_clip: Some("idle".to_string()),
        character: Some(CharacterId::Rust),
        selected_move: CombatLabMove::LightPunch,
    })
    .unwrap();

    assert_eq!(viewer.current_clip_name(), "idle");
    assert!(viewer.image_path().ends_with("rust-fighter-atlas.png"));
}

#[test]
fn rejects_unknown_initial_clip() {
    let error = SpriteViewer::load(SpriteViewerOptions {
        manifest_path: PathBuf::from("assets/placeholder/rust-fighter.sprite.json"),
        initial_clip: Some("segfault_pose".to_string()),
        character: Some(CharacterId::Rust),
        selected_move: CombatLabMove::LightPunch,
    })
    .unwrap_err();

    assert!(error.to_string().contains("segfault_pose"));
}

#[test]
fn dragging_inside_sprite_moves_anchor() {
    let mut viewer = SpriteViewer::load(SpriteViewerOptions {
        manifest_path: PathBuf::from("assets/placeholder/rust-fighter.sprite.json"),
        initial_clip: Some("idle".to_string()),
        character: Some(CharacterId::Rust),
        selected_move: CombatLabMove::LightPunch,
    })
    .unwrap();
    let start = viewer.anchor();
    let sprite_rect = viewer.sprite_screen_rect();
    let grab = ViewerPoint::new(sprite_rect.x + 10.0, sprite_rect.y + 10.0);

    viewer.update(
        SpriteViewerInput {
            mouse_position: grab,
            mouse_pressed: true,
            mouse_down: true,
            ..SpriteViewerInput::default()
        },
        0.0,
    );
    viewer.update(
        SpriteViewerInput {
            mouse_position: ViewerPoint::new(grab.x + 24.0, grab.y + 16.0),
            mouse_down: true,
            ..SpriteViewerInput::default()
        },
        0.0,
    );

    assert_eq!(
        viewer.anchor(),
        ViewerPoint::new(start.x + 24.0, start.y + 16.0)
    );
}

#[test]
fn mouse_wheel_zoom_changes_sprite_screen_size_and_reset_restores_it() {
    let mut viewer = SpriteViewer::load(SpriteViewerOptions {
        manifest_path: PathBuf::from("assets/placeholder/rust-fighter.sprite.json"),
        initial_clip: Some("idle".to_string()),
        character: Some(CharacterId::Rust),
        selected_move: CombatLabMove::LightPunch,
    })
    .unwrap();
    let original_width = viewer.sprite_screen_rect().width;

    viewer.update(
        SpriteViewerInput {
            zoom_delta: 1.0,
            ..SpriteViewerInput::default()
        },
        0.0,
    );
    assert!(viewer.sprite_screen_rect().width > original_width);

    viewer.update(
        SpriteViewerInput {
            reset_zoom: true,
            ..SpriteViewerInput::default()
        },
        0.0,
    );
    assert_eq!(viewer.sprite_screen_rect().width, original_width);
}

#[test]
fn dummy_can_be_dragged_without_moving_main_anchor() {
    let mut viewer = SpriteViewer::load(SpriteViewerOptions {
        manifest_path: PathBuf::from("assets/placeholder/rust-fighter.sprite.json"),
        initial_clip: Some("idle".to_string()),
        character: Some(CharacterId::Rust),
        selected_move: CombatLabMove::LightPunch,
    })
    .unwrap();
    let main_start = viewer.anchor();
    let dummy_start = viewer.dummy_anchor();
    let dummy_rect = viewer.dummy_screen_rect();
    let grab = ViewerPoint::new(dummy_rect.x + 10.0, dummy_rect.y + 10.0);

    viewer.update(
        SpriteViewerInput {
            mouse_position: grab,
            mouse_pressed: true,
            mouse_down: true,
            ..SpriteViewerInput::default()
        },
        0.0,
    );
    viewer.update(
        SpriteViewerInput {
            mouse_position: ViewerPoint::new(grab.x - 36.0, grab.y + 12.0),
            mouse_down: true,
            ..SpriteViewerInput::default()
        },
        0.0,
    );

    assert_eq!(viewer.anchor(), main_start);
    assert_eq!(
        viewer.dummy_anchor(),
        ViewerPoint::new(dummy_start.x - 36.0, dummy_start.y + 12.0)
    );
}

#[test]
fn reload_preserves_selected_clip_when_manifest_still_contains_it() {
    let mut viewer = SpriteViewer::load(SpriteViewerOptions {
        manifest_path: PathBuf::from("assets/placeholder/rust-fighter.sprite.json"),
        initial_clip: Some("special".to_string()),
        character: Some(CharacterId::Rust),
        selected_move: CombatLabMove::Projectile,
    })
    .unwrap();

    let image_changed = viewer.reload_manifest().unwrap();

    assert!(!image_changed);
    assert_eq!(viewer.current_clip_name(), "special");
}

#[test]
fn combat_overlay_uses_character_move_data() {
    let viewer = SpriteViewer::load(SpriteViewerOptions {
        manifest_path: PathBuf::from("assets/placeholder/duke-fighter.sprite.json"),
        initial_clip: Some("punch_heavy".to_string()),
        character: Some(CharacterId::Duke),
        selected_move: CombatLabMove::HeavyPunch,
    })
    .unwrap();

    let overlay = viewer.combat_overlay().unwrap();

    assert_eq!(overlay.character, CharacterId::Duke);
    assert_eq!(overlay.move_label, "Boilerplate");
    assert!(overlay.hitbox.is_some());
    assert!(overlay.projectile.is_none());
}

#[test]
fn projectile_overlay_exposes_spawn_box_and_origin() {
    let viewer = SpriteViewer::load(SpriteViewerOptions {
        manifest_path: PathBuf::from("assets/placeholder/rust-fighter.sprite.json"),
        initial_clip: Some("special".to_string()),
        character: Some(CharacterId::Rust),
        selected_move: CombatLabMove::Projectile,
    })
    .unwrap();

    let overlay = viewer.combat_overlay().unwrap();

    assert!(overlay.hitbox.is_none());
    assert!(overlay.projectile.is_some());
    assert!(overlay.projectile_origin.is_some());
}

#[test]
fn viewer_can_cycle_character_and_move_at_runtime() {
    let mut viewer = SpriteViewer::load(SpriteViewerOptions {
        manifest_path: PathBuf::from("assets/placeholder/rust-fighter.sprite.json"),
        initial_clip: Some("idle".to_string()),
        character: Some(CharacterId::Rust),
        selected_move: CombatLabMove::LightPunch,
    })
    .unwrap();

    viewer.update(
        SpriteViewerInput {
            next_character: true,
            ..SpriteViewerInput::default()
        },
        0.0,
    );
    assert_eq!(viewer.selected_character(), Some(CharacterId::Duke));

    viewer.update(
        SpriteViewerInput {
            previous_character: true,
            ..SpriteViewerInput::default()
        },
        0.0,
    );
    assert_eq!(viewer.selected_character(), Some(CharacterId::Rust));

    viewer.update(
        SpriteViewerInput {
            next_move: true,
            ..SpriteViewerInput::default()
        },
        0.0,
    );
    assert_eq!(viewer.selected_move(), CombatLabMove::HeavyPunch);

    viewer.update(
        SpriteViewerInput {
            previous_move: true,
            ..SpriteViewerInput::default()
        },
        0.0,
    );
    assert_eq!(viewer.selected_move(), CombatLabMove::LightPunch);
}

#[test]
fn projectile_trajectory_preview_tracks_selected_projectile() {
    let mut viewer = SpriteViewer::load(SpriteViewerOptions {
        manifest_path: PathBuf::from("assets/placeholder/rust-fighter.sprite.json"),
        initial_clip: Some("special".to_string()),
        character: Some(CharacterId::Rust),
        selected_move: CombatLabMove::Projectile,
    })
    .unwrap();

    let overlay = viewer.combat_overlay().unwrap();
    let trajectory = viewer.projectile_trajectory().unwrap();

    assert_eq!(trajectory.samples.len(), 7);
    assert_eq!(trajectory.origin, overlay.projectile_origin.unwrap());
    assert!(trajectory.end.x > trajectory.origin.x);
    assert!(trajectory.travel_distance > 0.0);

    viewer.update(
        SpriteViewerInput {
            toggle_projectile_trajectory: true,
            ..SpriteViewerInput::default()
        },
        0.0,
    );

    assert!(viewer.projectile_trajectory().is_none());
}

#[test]
fn frame_combat_overlay_projects_manifest_metadata_to_screen_space() {
    let viewer = SpriteViewer::load(SpriteViewerOptions {
        manifest_path: PathBuf::from("tests/fixtures/sprite-viewer-combat.sprite.json"),
        initial_clip: Some("idle".to_string()),
        character: None,
        selected_move: CombatLabMove::LightPunch,
    })
    .unwrap();

    let overlay = viewer.frame_combat_overlay().unwrap();

    assert_eq!(overlay.hurtboxes.len(), 1);
    assert_eq!(overlay.hitboxes.len(), 1);
    assert_eq!(overlay.hurtboxes[0].label.as_deref(), Some("body"));
    assert_eq!(overlay.hitboxes[0].label.as_deref(), Some("strike"));
    assert_eq!(overlay.hurtboxes[0].rect.x, 440.0);
    assert_eq!(overlay.hurtboxes[0].rect.y, 350.0);
    assert_eq!(
        overlay.projectile_origin,
        Some(ViewerPoint::new(514.0, 386.0))
    );
}

#[test]
fn timeline_phase_uses_selected_character_move_data() {
    let viewer = SpriteViewer::load(SpriteViewerOptions {
        manifest_path: PathBuf::from("assets/placeholder/rust-fighter.sprite.json"),
        initial_clip: Some("punch_light".to_string()),
        character: Some(CharacterId::Rust),
        selected_move: CombatLabMove::LightPunch,
    })
    .unwrap();

    assert_eq!(
        viewer.timeline_phase_for_frame_index(0),
        Some(SpriteTimelinePhase::Startup)
    );
    assert_eq!(
        viewer.timeline_phase_for_frame_index(4),
        Some(SpriteTimelinePhase::Active)
    );
    assert_eq!(
        viewer.timeline_phase_for_frame_index(9),
        Some(SpriteTimelinePhase::Recovery)
    );
}
