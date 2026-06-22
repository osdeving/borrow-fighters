//! Validates runtime sprite manifest loading without opening a window.

use std::path::Path;

use borrow_fighters::{
    combat::fighter::{Facing, Fighter, PlayerSlot},
    engine::sprites::{
        DUKE_FIGHTER_MANIFEST_PATH, DUKE_START_MANIFEST_PATH, GO_CHANNEL_PROJECTILE_PATH,
        GO_FIGHTER_MANIFEST_PATH, GO_START_MANIFEST_PATH, RUST_FIGHTER_MANIFEST_PATH,
        RUST_START_MANIFEST_PATH, SPRITE_SCHEMA, SpriteManifest, frame_for_clip_at,
        project_frame_combat,
    },
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
fn duke_fighter_manifest_loads() {
    let manifest = SpriteManifest::load(DUKE_FIGHTER_MANIFEST_PATH).expect("manifest should load");

    assert_eq!(manifest.schema, SPRITE_SCHEMA);
    assert_eq!(manifest.image, "duke-fighter-atlas.png");
    assert_eq!(manifest.frames.len(), 33);
    assert_eq!(manifest.cell.w, 384);
    assert!(manifest.clip_named("special").is_some());
    assert!(manifest.clip_named("taunt").is_some());
}

#[test]
fn go_fighter_manifest_loads() {
    let manifest = SpriteManifest::load(GO_FIGHTER_MANIFEST_PATH).expect("manifest should load");

    assert_eq!(manifest.schema, SPRITE_SCHEMA);
    assert_eq!(manifest.image, "go-fighter-atlas.png");
    assert_eq!(manifest.frames.len(), 36);
    assert_eq!(manifest.cell.w, 384);
    assert!(manifest.clip_named("idle").is_some());
    assert!(manifest.clip_named("kick").is_some());
    assert!(manifest.clip_named("special").is_some());
}

#[test]
fn fighter_special_first_frames_declare_projectile_origins() {
    let cases = [
        (RUST_FIGHTER_MANIFEST_PATH, "special_0", 216, 148),
        (DUKE_FIGHTER_MANIFEST_PATH, "special_0", 222, 148),
        (GO_FIGHTER_MANIFEST_PATH, "special_0", 225, 150),
    ];

    for (path, frame_name, expected_x, expected_y) in cases {
        let manifest = SpriteManifest::load(path).expect("manifest should load");
        let origin = manifest
            .frame_named(frame_name)
            .and_then(|frame| frame.combat.as_ref())
            .and_then(|combat| combat.projectile_origin)
            .expect("special frame should declare projectile origin");

        assert_eq!(origin.x, expected_x, "{path} {frame_name} origin x");
        assert_eq!(origin.y, expected_y, "{path} {frame_name} origin y");
    }
}

#[test]
fn rust_start_manifest_loads_spawn_clip() {
    let manifest = SpriteManifest::load(RUST_START_MANIFEST_PATH).expect("manifest should load");
    let spawn = manifest
        .clip_named("spawn")
        .expect("spawn clip should exist");

    assert_eq!(manifest.image, "rust-start-atlas.png");
    assert_eq!(manifest.frames.len(), 19);
    assert!(!spawn.r#loop);
    assert_eq!(spawn.frames.first().map(String::as_str), Some("spawn_00"));
    assert_eq!(spawn.frames.last().map(String::as_str), Some("spawn_18"));
}

#[test]
fn duke_start_manifest_loads_spawn_clip() {
    let manifest = SpriteManifest::load(DUKE_START_MANIFEST_PATH).expect("manifest should load");
    let spawn = manifest
        .clip_named("spawn")
        .expect("spawn clip should exist");

    assert_eq!(manifest.image, "duke-start-atlas.png");
    assert_eq!(manifest.frames.len(), 18);
    assert!(!spawn.r#loop);
    assert_eq!(spawn.frames.first().map(String::as_str), Some("spawn_00"));
    assert_eq!(spawn.frames.last().map(String::as_str), Some("spawn_17"));
}

#[test]
fn go_start_manifest_loads_spawn_clip() {
    let manifest = SpriteManifest::load(GO_START_MANIFEST_PATH).expect("manifest should load");
    let spawn = manifest
        .clip_named("spawn")
        .expect("spawn clip should exist");

    assert_eq!(manifest.image, "go-start-atlas.png");
    assert_eq!(manifest.frames.len(), 24);
    assert!(!spawn.r#loop);
    assert_eq!(spawn.frames.first().map(String::as_str), Some("spawn_00"));
    assert_eq!(spawn.frames.last().map(String::as_str), Some("spawn_23"));
}

#[test]
fn go_runtime_sprite_assets_exist() {
    let fighter = SpriteManifest::load(GO_FIGHTER_MANIFEST_PATH).expect("manifest should load");
    let start = SpriteManifest::load(GO_START_MANIFEST_PATH).expect("manifest should load");

    assert!(fighter.image_path(GO_FIGHTER_MANIFEST_PATH).exists());
    assert!(start.image_path(GO_START_MANIFEST_PATH).exists());
    assert!(Path::new(GO_CHANNEL_PROJECTILE_PATH).exists());
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

#[test]
fn frame_combat_metadata_projects_to_world_space_and_mirrors_with_facing() {
    let manifest = SpriteManifest::load("tests/fixtures/sprite-viewer-combat.sprite.json")
        .expect("fixture manifest should load");
    let frame = manifest.frame_named("idle_0").unwrap();
    let mut fighter = Fighter::new(PlayerSlot::One, "Test", 200.0);

    let projected = project_frame_combat(&manifest, frame, &fighter).unwrap();

    assert_eq!(projected.frame_name, "idle_0");
    assert_eq!(projected.hurtboxes[0].x, 198.0);
    assert_eq!(projected.hurtboxes[0].y, 350.0);
    assert_eq!(projected.hitboxes[0].x, 250.0);
    assert_eq!(projected.hitboxes[0].y, 380.0);
    assert_eq!(projected.projectile_origin.unwrap().x, 272.0);
    assert_eq!(projected.projectile_origin.unwrap().y, 386.0);

    fighter.facing = Facing::Left;
    let mirrored = project_frame_combat(&manifest, frame, &fighter).unwrap();

    assert_eq!(mirrored.hurtboxes[0].x, 230.0);
    assert_eq!(mirrored.hitboxes[0].x, 198.0);
    assert_eq!(mirrored.projectile_origin.unwrap().x, 204.0);
}

#[test]
fn frame_combat_metadata_validates() {
    let manifest: SpriteManifest = serde_json::from_str(
        r#"
        {
          "schema": "borrow-fighters.sprite.v1",
          "image": "atlas.png",
          "cell": { "w": 100, "h": 120 },
          "default_pivot": { "x": 50, "y": 120 },
          "frames": [
            {
              "name": "punch_0",
              "clip": "punch",
              "duration_ms": 90,
              "pivot": { "x": 50, "y": 120 },
              "frame": { "x": 0, "y": 0, "w": 100, "h": 120 },
              "combat": {
                "hurtboxes": [
                  { "x": 10, "y": 20, "w": 36, "h": 80, "label": "torso" }
                ],
                "hitboxes": [
                  { "x": 60, "y": 36, "w": 30, "h": 24, "label": "fist" }
                ],
                "projectile_origin": { "x": 82, "y": 48 }
              }
            }
          ],
          "clips": [
            { "name": "punch", "loop": false, "frames": ["punch_0"] }
          ]
        }
        "#,
    )
    .expect("manifest json should parse");

    manifest
        .validate()
        .expect("combat metadata should validate");
    let combat = manifest.frames[0]
        .combat
        .as_ref()
        .expect("frame should contain combat metadata");

    assert_eq!(combat.hurtboxes[0].label.as_deref(), Some("torso"));
    assert_eq!(combat.hitboxes[0].w, 30);
    assert_eq!(combat.projectile_origin.unwrap().x, 82);
}

#[test]
fn frame_combat_metadata_rejects_boxes_outside_frame() {
    let manifest: SpriteManifest = serde_json::from_str(
        r#"
        {
          "schema": "borrow-fighters.sprite.v1",
          "image": "atlas.png",
          "cell": { "w": 100, "h": 120 },
          "default_pivot": { "x": 50, "y": 120 },
          "frames": [
            {
              "name": "punch_0",
              "clip": "punch",
              "duration_ms": 90,
              "pivot": { "x": 50, "y": 120 },
              "frame": { "x": 0, "y": 0, "w": 100, "h": 120 },
              "combat": {
                "hitboxes": [
                  { "x": 90, "y": 36, "w": 30, "h": 24, "label": "fist" }
                ]
              }
            }
          ],
          "clips": [
            { "name": "punch", "loop": false, "frames": ["punch_0"] }
          ]
        }
        "#,
    )
    .expect("manifest json should parse");

    let error = manifest.validate().unwrap_err();

    assert!(error.to_string().contains("hitbox must be inside"));
}
