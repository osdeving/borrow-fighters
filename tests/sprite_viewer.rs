//! Verifies the standalone sprite viewer state without opening a window.

use std::{fs, path::PathBuf};

use borrow_fighters::{
    characters::CharacterId,
    engine::sprites::SpriteManifest,
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
fn viewer_can_tune_manifest_scale_and_pivot_then_save() {
    let mut manifest_path = std::env::temp_dir();
    manifest_path.push(format!(
        "borrow-fighters-sprite-viewer-{}-{}.json",
        std::process::id(),
        line!()
    ));
    fs::copy(
        "assets/placeholder/rust-fighter.sprite.json",
        &manifest_path,
    )
    .unwrap();

    let mut viewer = SpriteViewer::load(SpriteViewerOptions {
        manifest_path: manifest_path.clone(),
        initial_clip: Some("idle".to_string()),
        character: Some(CharacterId::Rust),
        selected_move: CombatLabMove::LightPunch,
    })
    .unwrap();
    let original_scale = viewer.manifest_scale();
    let original_pivot = viewer.current_frame().pivot;

    viewer.update(
        SpriteViewerInput {
            increase_manifest_scale: true,
            nudge_pivot_x: 2,
            nudge_pivot_y: -1,
            ..SpriteViewerInput::default()
        },
        0.0,
    );

    assert!(viewer.manifest_dirty());
    assert!(viewer.manifest_scale() > original_scale);
    assert_eq!(viewer.current_frame().pivot.x, original_pivot.x + 2);
    assert_eq!(viewer.current_frame().pivot.y, original_pivot.y - 1);

    viewer.update(
        SpriteViewerInput {
            save_manifest: true,
            ..SpriteViewerInput::default()
        },
        0.0,
    );

    assert!(!viewer.manifest_dirty());
    let saved = SpriteManifest::load(&manifest_path).unwrap();
    let saved_frame = saved.frame_named("idle_0").unwrap();
    assert_eq!(saved.scale, Some(viewer.manifest_scale()));
    assert_eq!(saved_frame.pivot, viewer.current_frame().pivot);

    fs::remove_file(manifest_path).unwrap();
}

#[test]
fn viewer_can_tune_selected_character_body_metrics_in_memory() {
    let mut viewer = SpriteViewer::load(SpriteViewerOptions {
        manifest_path: PathBuf::from("assets/placeholder/go-fighter.sprite.json"),
        initial_clip: Some("idle".to_string()),
        character: Some(CharacterId::Go),
        selected_move: CombatLabMove::LightPunch,
    })
    .unwrap();
    let original = viewer.selected_body_metrics().unwrap();

    viewer.update(
        SpriteViewerInput {
            nudge_body_width: 3,
            nudge_standing_height: -4,
            nudge_crouch_height: -2,
            ..SpriteViewerInput::default()
        },
        0.0,
    );

    let tuned = viewer.selected_body_metrics().unwrap();
    assert!(viewer.body_metrics_dirty());
    assert_eq!(tuned.width, original.width + 3.0);
    assert_eq!(tuned.standing_height, original.standing_height - 4.0);
    assert_eq!(tuned.crouch_height, original.crouch_height - 2.0);
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
fn frame_cursor_reports_local_and_atlas_coordinates() {
    let mut viewer = SpriteViewer::load(SpriteViewerOptions {
        manifest_path: PathBuf::from("tests/fixtures/sprite-viewer-combat.sprite.json"),
        initial_clip: Some("idle".to_string()),
        character: None,
        selected_move: CombatLabMove::LightPunch,
    })
    .unwrap();

    viewer.update(
        SpriteViewerInput {
            mouse_position: ViewerPoint::new(452.0, 382.0),
            ..SpriteViewerInput::default()
        },
        0.0,
    );

    let cursor = viewer
        .frame_cursor()
        .expect("mouse should be inside sprite");

    assert_eq!(cursor.local_x, 22);
    assert_eq!(cursor.local_y, 40);
    assert_eq!(cursor.atlas_x, 22);
    assert_eq!(cursor.atlas_y, 40);

    viewer.update(
        SpriteViewerInput {
            mouse_position: ViewerPoint::new(10.0, 10.0),
            ..SpriteViewerInput::default()
        },
        0.0,
    );

    assert!(viewer.frame_cursor().is_none());
}

#[test]
fn sync_clip_to_move_selects_first_known_clip_for_move() {
    let mut viewer = SpriteViewer::load(SpriteViewerOptions {
        manifest_path: PathBuf::from("assets/placeholder/rust-fighter.sprite.json"),
        initial_clip: Some("idle".to_string()),
        character: Some(CharacterId::Rust),
        selected_move: CombatLabMove::Projectile,
    })
    .unwrap();

    viewer.update(
        SpriteViewerInput {
            sync_clip_to_move: true,
            ..SpriteViewerInput::default()
        },
        0.0,
    );

    assert_eq!(viewer.current_clip_name(), "special");

    viewer.update(
        SpriteViewerInput {
            previous_move: true,
            sync_clip_to_move: true,
            ..SpriteViewerInput::default()
        },
        0.0,
    );

    assert_eq!(viewer.selected_move(), CombatLabMove::Throw);
    assert_eq!(viewer.current_clip_name(), "punch_light");
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
fn frame_combat_hurtbox_can_be_dragged_in_frame_local_coordinates() {
    let mut viewer = SpriteViewer::load(SpriteViewerOptions {
        manifest_path: PathBuf::from("tests/fixtures/sprite-viewer-combat.sprite.json"),
        initial_clip: Some("idle".to_string()),
        character: None,
        selected_move: CombatLabMove::LightPunch,
    })
    .unwrap();

    viewer.update(
        SpriteViewerInput {
            mouse_position: ViewerPoint::new(464.0, 398.0),
            mouse_pressed: true,
            mouse_down: true,
            ..SpriteViewerInput::default()
        },
        0.0,
    );
    viewer.update(
        SpriteViewerInput {
            mouse_position: ViewerPoint::new(476.0, 404.0),
            mouse_down: true,
            ..SpriteViewerInput::default()
        },
        0.0,
    );

    let combat = viewer.current_frame().combat.as_ref().unwrap();
    assert!(viewer.manifest_dirty());
    assert_eq!(combat.hurtboxes[0].x, 22);
    assert_eq!(combat.hurtboxes[0].y, 14);
    assert_eq!(combat.hurtboxes[0].w, 48);
    assert_eq!(combat.hurtboxes[0].h, 96);
}

#[test]
fn frame_combat_hitbox_corner_can_be_resized() {
    let mut viewer = SpriteViewer::load(SpriteViewerOptions {
        manifest_path: PathBuf::from("tests/fixtures/sprite-viewer-combat.sprite.json"),
        initial_clip: Some("idle".to_string()),
        character: None,
        selected_move: CombatLabMove::LightPunch,
    })
    .unwrap();

    viewer.update(
        SpriteViewerInput {
            mouse_position: ViewerPoint::new(520.0, 402.0),
            mouse_pressed: true,
            mouse_down: true,
            ..SpriteViewerInput::default()
        },
        0.0,
    );
    viewer.update(
        SpriteViewerInput {
            mouse_position: ViewerPoint::new(528.0, 410.0),
            mouse_down: true,
            ..SpriteViewerInput::default()
        },
        0.0,
    );

    let combat = viewer.current_frame().combat.as_ref().unwrap();
    assert!(viewer.manifest_dirty());
    assert_eq!(combat.hitboxes[0].x, 62);
    assert_eq!(combat.hitboxes[0].y, 38);
    assert_eq!(combat.hitboxes[0].w, 36);
    assert_eq!(combat.hitboxes[0].h, 30);
}

#[test]
fn projectile_origin_can_be_dragged_and_saved() {
    let mut manifest_path = std::env::temp_dir();
    manifest_path.push(format!(
        "borrow-fighters-sprite-viewer-combat-{}-{}.json",
        std::process::id(),
        line!()
    ));
    fs::copy(
        "tests/fixtures/sprite-viewer-combat.sprite.json",
        &manifest_path,
    )
    .unwrap();

    let mut viewer = SpriteViewer::load(SpriteViewerOptions {
        manifest_path: manifest_path.clone(),
        initial_clip: Some("idle".to_string()),
        character: None,
        selected_move: CombatLabMove::Projectile,
    })
    .unwrap();

    viewer.update(
        SpriteViewerInput {
            mouse_position: ViewerPoint::new(514.0, 386.0),
            mouse_pressed: true,
            mouse_down: true,
            ..SpriteViewerInput::default()
        },
        0.0,
    );
    viewer.update(
        SpriteViewerInput {
            mouse_position: ViewerPoint::new(520.0, 390.0),
            mouse_down: true,
            ..SpriteViewerInput::default()
        },
        0.0,
    );
    viewer.update(
        SpriteViewerInput {
            save_manifest: true,
            ..SpriteViewerInput::default()
        },
        0.0,
    );

    assert!(!viewer.manifest_dirty());
    let saved = SpriteManifest::load(&manifest_path).unwrap();
    let origin = saved
        .frame_named("idle_0")
        .unwrap()
        .combat
        .as_ref()
        .unwrap()
        .projectile_origin
        .unwrap();
    assert_eq!(origin.x, 90);
    assert_eq!(origin.y, 48);

    fs::remove_file(manifest_path).unwrap();
}

#[test]
fn seed_frame_combat_creates_editable_metadata_from_runtime_overlay() {
    let mut viewer = SpriteViewer::load(SpriteViewerOptions {
        manifest_path: PathBuf::from("tests/fixtures/sprite-viewer-combat.sprite.json"),
        initial_clip: Some("idle".to_string()),
        character: Some(CharacterId::Rust),
        selected_move: CombatLabMove::LightPunch,
    })
    .unwrap();

    viewer.update(
        SpriteViewerInput {
            seed_frame_combat: true,
            ..SpriteViewerInput::default()
        },
        0.0,
    );

    let combat = viewer.current_frame().combat.as_ref().unwrap();
    assert!(viewer.manifest_dirty());
    assert!(!combat.hurtboxes.is_empty());
    assert!(!combat.hitboxes.is_empty());
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
