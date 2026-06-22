//! Verifies the standalone sprite viewer state without opening a window.

use std::path::PathBuf;

use borrow_fighters::scenes::sprite_viewer::{
    SpriteViewer, SpriteViewerInput, SpriteViewerOptions, ViewerPoint,
};

#[test]
fn loads_manifest_and_selects_requested_clip() {
    let viewer = SpriteViewer::load(SpriteViewerOptions {
        manifest_path: PathBuf::from("assets/placeholder/rust-fighter.sprite.json"),
        initial_clip: Some("idle".to_string()),
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
    })
    .unwrap_err();

    assert!(error.to_string().contains("segfault_pose"));
}

#[test]
fn dragging_inside_sprite_moves_anchor() {
    let mut viewer = SpriteViewer::load(SpriteViewerOptions {
        manifest_path: PathBuf::from("assets/placeholder/rust-fighter.sprite.json"),
        initial_clip: Some("idle".to_string()),
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
    })
    .unwrap();

    let image_changed = viewer.reload_manifest().unwrap();

    assert!(!image_changed);
    assert_eq!(viewer.current_clip_name(), "special");
}
