//! Validates runtime sprite manifest loading without opening a window.

use std::{fs, path::Path};

use borrow_fighters::{
    combat::{
        fighter::{Facing, Fighter, PlayerSlot},
        move_data::{MoveId, move_spec},
    },
    config::RESOLUTION_SCALE,
    engine::sprites::{
        C_BITSTREAM_PROJECTILE_PATH, C_FIGHTER_MANIFEST_PATH, C_START_MANIFEST_PATH,
        DUKE_FIGHTER_MANIFEST_PATH, DUKE_START_MANIFEST_PATH, GO_CHANNEL_PROJECTILE_PATH,
        GO_FIGHTER_MANIFEST_PATH, GO_START_MANIFEST_PATH, PYTHON_DATA_PROJECTILE_PATH,
        PYTHON_FIGHTER_MANIFEST_PATH, PYTHON_START_MANIFEST_PATH, RUST_FIGHTER_MANIFEST_PATH,
        RUST_START_MANIFEST_PATH, SPRITE_SCHEMA, SpriteManifest, frame_for_clip_at,
        project_frame_combat,
    },
};

fn assert_f32_close(actual: f32, expected: f32, context: &str) {
    assert!(
        (actual - expected).abs() <= 0.001,
        "{context}: expected {expected}, got {actual}"
    );
}

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
    assert_f32_close(
        manifest.scale.expect("Go manifest should declare scale"),
        1.08 * RESOLUTION_SCALE,
        "Go manifest scale",
    );
    assert_eq!(manifest.frames.len(), 36);
    assert_eq!(manifest.cell.w, 384);
    assert!(manifest.clip_named("idle").is_some());
    assert!(manifest.clip_named("kick").is_some());
    assert!(manifest.clip_named("special").is_some());
}

#[test]
fn c_fighter_manifest_loads() {
    let manifest = SpriteManifest::load(C_FIGHTER_MANIFEST_PATH).expect("manifest should load");

    assert_eq!(manifest.schema, SPRITE_SCHEMA);
    assert_eq!(manifest.image, "c-fighter-atlas.png");
    assert_eq!(manifest.frames.len(), 94);
    assert_eq!(manifest.cell.w, 384);
    assert!(manifest.clip_named("idle").is_some());
    assert!(manifest.clip_named("punch_light").is_some());
    assert!(manifest.clip_named("kick").is_some());
    assert!(manifest.clip_named("special").is_some());
}

#[test]
fn python_fighter_manifest_candidate_loads() {
    let manifest =
        SpriteManifest::load(PYTHON_FIGHTER_MANIFEST_PATH).expect("manifest should load");

    assert_eq!(manifest.schema, SPRITE_SCHEMA);
    assert_eq!(manifest.image, "python-fighter-atlas.png");
    assert_eq!(manifest.frames.len(), 94);
    assert_eq!(manifest.cell.w, 384);
    assert!(manifest.clip_named("idle").is_some());
    assert!(manifest.clip_named("punch_light").is_some());
    assert!(manifest.clip_named("punch_heavy").is_some());
    assert!(manifest.clip_named("taunt").is_some());
    assert!(manifest.clip_named("special").is_some());
}

#[test]
fn python_runtime_sprite_assets_exist() {
    let fighter = SpriteManifest::load(PYTHON_FIGHTER_MANIFEST_PATH).expect("manifest should load");
    let start = SpriteManifest::load(PYTHON_START_MANIFEST_PATH).expect("manifest should load");

    assert!(fighter.image_path(PYTHON_FIGHTER_MANIFEST_PATH).exists());
    assert!(start.image_path(PYTHON_START_MANIFEST_PATH).exists());
    assert!(Path::new(PYTHON_DATA_PROJECTILE_PATH).exists());
}

#[test]
fn fighter_special_first_frames_declare_projectile_origins() {
    let cases = [
        (RUST_FIGHTER_MANIFEST_PATH, "special_0", 216, 148),
        (DUKE_FIGHTER_MANIFEST_PATH, "special_0", 222, 148),
        (GO_FIGHTER_MANIFEST_PATH, "special_0", 180, 150),
        (C_FIGHTER_MANIFEST_PATH, "special_0", 208, 145),
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
fn rust_primary_sprite_hitboxes_match_current_move_reach() {
    let manifest = SpriteManifest::load(RUST_FIGHTER_MANIFEST_PATH).expect("manifest should load");
    let fighter = Fighter::new(PlayerSlot::One, "Rust", 200.0);
    let body = fighter.body_rect();
    let cases: &[(MoveId, &[&str])] = &[
        (MoveId::RustBorrowJab, &["punch_0", "punch_1"]),
        (MoveId::HeavyPunch, &["punch_2"]),
        (MoveId::Kick, &["kick_1", "kick_2"]),
    ];

    for (move_id, frame_names) in cases {
        let spec = move_spec(*move_id);
        for frame_name in *frame_names {
            let frame = manifest.frame_named(frame_name).unwrap();
            let projected = project_frame_combat(&manifest, frame, &fighter).unwrap();
            let hitbox = projected.hitboxes[0];

            assert_f32_close(hitbox.x, body.right(), &format!("{frame_name} x"));
            assert_f32_close(
                hitbox.y,
                body.y + spec.hitbox.y_offset,
                &format!("{frame_name} y"),
            );
            assert_f32_close(
                hitbox.width,
                spec.hitbox.width,
                &format!("{frame_name} width"),
            );
            assert_f32_close(
                hitbox.height,
                spec.hitbox.height,
                &format!("{frame_name} height"),
            );
        }
    }

    assert!(
        manifest
            .frame_named("kick_0")
            .and_then(|frame| frame.combat.as_ref())
            .is_none_or(|combat| combat.hitboxes.is_empty()),
        "kick_0 remains startup without an active hitbox"
    );
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
fn c_start_manifest_loads_spawn_clip() {
    let manifest = SpriteManifest::load(C_START_MANIFEST_PATH).expect("manifest should load");
    let spawn = manifest
        .clip_named("spawn")
        .expect("spawn clip should exist");

    assert_eq!(manifest.image, "c-start-atlas.png");
    assert_eq!(manifest.frames.len(), 7);
    assert!(!spawn.r#loop);
    assert_eq!(spawn.frames.first().map(String::as_str), Some("spawn_00"));
    assert_eq!(spawn.frames.last().map(String::as_str), Some("spawn_06"));
}

#[test]
fn python_start_manifest_loads_spawn_clip() {
    let manifest = SpriteManifest::load(PYTHON_START_MANIFEST_PATH).expect("manifest should load");
    let spawn = manifest
        .clip_named("spawn")
        .expect("spawn clip should exist");

    assert_eq!(manifest.image, "python-start-atlas.png");
    assert_eq!(manifest.frames.len(), 12);
    assert_eq!(manifest.cell.w, 512);
    assert!(!spawn.r#loop);
    assert_eq!(spawn.frames.first().map(String::as_str), Some("spawn_00"));
    assert_eq!(spawn.frames.last().map(String::as_str), Some("spawn_11"));
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
fn c_runtime_sprite_assets_exist() {
    let fighter = SpriteManifest::load(C_FIGHTER_MANIFEST_PATH).expect("manifest should load");
    let start = SpriteManifest::load(C_START_MANIFEST_PATH).expect("manifest should load");

    assert!(fighter.image_path(C_FIGHTER_MANIFEST_PATH).exists());
    assert!(start.image_path(C_START_MANIFEST_PATH).exists());
    assert!(Path::new(C_BITSTREAM_PROJECTILE_PATH).exists());
}

#[test]
fn c_bitstream_projectile_keeps_readable_runtime_size() {
    let manifest = SpriteManifest::load(C_FIGHTER_MANIFEST_PATH).expect("manifest should load");
    let frame = manifest
        .frame_named("projectile_0")
        .expect("projectile frame should exist");
    let bounds = frame
        .trimmed_bounds
        .expect("generated projectile should declare trimmed bounds");
    let (width, height) = png_dimensions(C_BITSTREAM_PROJECTILE_PATH);

    assert!(
        bounds.w >= 260,
        "atlas projectile should be visually readable"
    );
    assert!(
        bounds.h >= 70,
        "atlas projectile should be visually readable"
    );
    assert!(
        width >= 260,
        "runtime projectile should keep readable 0/1 glyphs"
    );
    assert!(
        height >= 70,
        "runtime projectile should keep readable 0/1 glyphs"
    );
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
    let body = fighter.body_rect();
    let anchor_x = body.center_x();
    let anchor_y = body.bottom();

    assert_eq!(projected.frame_name, "idle_0");
    assert_f32_close(projected.hurtboxes[0].x, anchor_x - 40.0, "hurtbox x");
    assert_f32_close(projected.hurtboxes[0].y, anchor_y - 112.0, "hurtbox y");
    assert_f32_close(projected.hitboxes[0].x, anchor_x + 12.0, "hitbox x");
    assert_f32_close(projected.hitboxes[0].y, anchor_y - 82.0, "hitbox y");
    assert_f32_close(
        projected.projectile_origin.unwrap().x,
        anchor_x + 34.0,
        "projectile origin x",
    );
    assert_f32_close(
        projected.projectile_origin.unwrap().y,
        anchor_y - 76.0,
        "projectile origin y",
    );

    fighter.facing = Facing::Left;
    let mirrored = project_frame_combat(&manifest, frame, &fighter).unwrap();

    assert_f32_close(
        mirrored.hurtboxes[0].x,
        anchor_x - 8.0,
        "mirrored hurtbox x",
    );
    assert_f32_close(mirrored.hitboxes[0].x, anchor_x - 40.0, "mirrored hitbox x");
    assert_f32_close(
        mirrored.projectile_origin.unwrap().x,
        anchor_x - 34.0,
        "mirrored projectile origin x",
    );
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

fn png_dimensions(path: &str) -> (u32, u32) {
    let bytes = fs::read(path).expect("png should be readable");
    assert_eq!(&bytes[0..8], b"\x89PNG\r\n\x1a\n");
    let width = u32::from_be_bytes(bytes[16..20].try_into().unwrap());
    let height = u32::from_be_bytes(bytes[20..24].try_into().unwrap());
    (width, height)
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
