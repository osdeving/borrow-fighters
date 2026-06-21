//! Validates runtime sprite manifest loading without opening a window.

use borrow_fighters::engine::sprites::{
    RUST_FIGHTER_MANIFEST_PATH, SPRITE_SCHEMA, SpriteManifest, frame_for_clip_at,
};

#[test]
fn rust_fighter_manifest_loads() {
    let manifest = SpriteManifest::load(RUST_FIGHTER_MANIFEST_PATH).expect("manifest should load");

    assert_eq!(manifest.schema, SPRITE_SCHEMA);
    assert_eq!(manifest.image, "rust-fighter-atlas.png");
    assert_eq!(manifest.frames.len(), 33);
    assert!(manifest.clip_named("idle").is_some());
    assert!(manifest.clip_named("kick").is_some());
    assert!(manifest.clip_named("projectile").is_some());
}

#[test]
fn looping_clip_wraps_by_frame_duration() {
    let manifest = SpriteManifest::load(RUST_FIGHTER_MANIFEST_PATH).expect("manifest should load");
    let first = frame_for_clip_at(&manifest, "idle", 0.0).expect("frame should resolve");
    let wrapped = frame_for_clip_at(&manifest, "idle", 0.700).expect("frame should resolve");

    assert_eq!(first.name, "idle_0");
    assert_eq!(wrapped.name, "idle_0");
}

#[test]
fn non_looping_clip_clamps_to_last_frame() {
    let manifest = SpriteManifest::load(RUST_FIGHTER_MANIFEST_PATH).expect("manifest should load");
    let frame = frame_for_clip_at(&manifest, "kick", 99.0).expect("frame should resolve");

    assert_eq!(frame.name, "kick_2");
}

#[test]
fn manifest_resolves_atlas_next_to_manifest_file() {
    let manifest = SpriteManifest::load(RUST_FIGHTER_MANIFEST_PATH).expect("manifest should load");
    let atlas_path = manifest.image_path(RUST_FIGHTER_MANIFEST_PATH);

    assert!(atlas_path.ends_with("assets/placeholder/rust-fighter-atlas.png"));
}
