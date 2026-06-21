//! Loads prototype assets at the Raylib boundary.
//!
//! Assets stay optional in the greybox phase so the game can still run with
//! procedural debug drawing if a local file is missing.

use raylib::prelude::*;

use crate::engine::sprites::{
    DUKE_BEAN_PROJECTILE_PATH, DUKE_FIGHTER_MANIFEST_PATH, DUKE_START_MANIFEST_PATH,
    FIGHTER_SPRITESHEET_PATH, RUST_FIGHTER_MANIFEST_PATH, RUST_GEAR_PROJECTILE_PATH,
    RUST_START_MANIFEST_PATH, SpriteManifest,
};

pub const ARENA_BACKGROUND_PATH: &str = "assets/placeholder/arena-terminal-compiler-lab.png";

/// Texture and metadata for one atlas-driven sprite set.
pub struct SpriteAtlasAsset {
    pub manifest: SpriteManifest,
    pub texture: Texture2D,
}

/// Runtime textures used by the prototype renderer.
pub struct GameAssets {
    pub arena_background: Option<Texture2D>,
    pub fighter_spritesheet: Option<Texture2D>,
    pub rust_fighter: Option<SpriteAtlasAsset>,
    pub rust_start: Option<SpriteAtlasAsset>,
    pub duke_fighter: Option<SpriteAtlasAsset>,
    pub duke_start: Option<SpriteAtlasAsset>,
    pub rust_projectile: Option<Texture2D>,
    pub duke_projectile: Option<Texture2D>,
}

impl GameAssets {
    /// Loads all optional prototype assets.
    pub fn load(raylib: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        Self {
            arena_background: load_texture_optional(raylib, thread, ARENA_BACKGROUND_PATH),
            fighter_spritesheet: load_texture_optional(raylib, thread, FIGHTER_SPRITESHEET_PATH),
            rust_fighter: load_sprite_atlas_optional(raylib, thread, RUST_FIGHTER_MANIFEST_PATH),
            rust_start: load_sprite_atlas_optional(raylib, thread, RUST_START_MANIFEST_PATH),
            duke_fighter: load_sprite_atlas_optional(raylib, thread, DUKE_FIGHTER_MANIFEST_PATH),
            duke_start: load_sprite_atlas_optional(raylib, thread, DUKE_START_MANIFEST_PATH),
            rust_projectile: load_texture_optional(raylib, thread, RUST_GEAR_PROJECTILE_PATH),
            duke_projectile: load_texture_optional(raylib, thread, DUKE_BEAN_PROJECTILE_PATH),
        }
    }
}

fn load_sprite_atlas_optional(
    raylib: &mut RaylibHandle,
    thread: &RaylibThread,
    manifest_path: &str,
) -> Option<SpriteAtlasAsset> {
    let manifest = match SpriteManifest::load(manifest_path) {
        Ok(manifest) => manifest,
        Err(error) => {
            eprintln!("warning: could not load sprite manifest {manifest_path}: {error}");
            return None;
        }
    };
    let texture_path = manifest.image_path(manifest_path);
    let texture = load_texture_optional(raylib, thread, &texture_path.to_string_lossy())?;

    Some(SpriteAtlasAsset { manifest, texture })
}

fn load_texture_optional(
    raylib: &mut RaylibHandle,
    thread: &RaylibThread,
    path: &str,
) -> Option<Texture2D> {
    match raylib.load_texture(thread, path) {
        Ok(texture) => Some(texture),
        Err(error) => {
            eprintln!("warning: could not load texture {path}: {error:?}");
            None
        }
    }
}
