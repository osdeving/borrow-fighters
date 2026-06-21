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
use crate::game::arena::ArenaId;

pub const ARENA_SIRIUS_PATH: &str = "assets/placeholder/arena-sirius.png";
pub const ARENA_FORTALEZA_PATH: &str = "assets/placeholder/arena-fortaleza.png";
pub const ARENA_JAVA_STREET_PATH: &str = "assets/placeholder/arena-java-street.png";

pub const COUNTDOWN_11_PATH: &str = "assets/placeholder/countdown-11.png";
pub const COUNTDOWN_10_PATH: &str = "assets/placeholder/countdown-10.png";
pub const COUNTDOWN_01_PATH: &str = "assets/placeholder/countdown-01.png";
pub const COUNTDOWN_FIGHT_PATH: &str = "assets/placeholder/countdown-fight.png";

/// Texture and metadata for one atlas-driven sprite set.
pub struct SpriteAtlasAsset {
    pub manifest: SpriteManifest,
    pub texture: Texture2D,
}

/// Runtime textures used by the prototype renderer.
pub struct GameAssets {
    pub arenas: ArenaAssets,
    pub fighter_spritesheet: Option<Texture2D>,
    pub rust_fighter: Option<SpriteAtlasAsset>,
    pub rust_start: Option<SpriteAtlasAsset>,
    pub duke_fighter: Option<SpriteAtlasAsset>,
    pub duke_start: Option<SpriteAtlasAsset>,
    pub rust_projectile: Option<Texture2D>,
    pub duke_projectile: Option<Texture2D>,
    pub countdown_11: Option<Texture2D>,
    pub countdown_10: Option<Texture2D>,
    pub countdown_01: Option<Texture2D>,
    pub countdown_fight: Option<Texture2D>,
}

/// Arena background textures loaded at the Raylib boundary.
pub struct ArenaAssets {
    pub sirius: Option<Texture2D>,
    pub fortaleza: Option<Texture2D>,
    pub java_street: Option<Texture2D>,
}

impl ArenaAssets {
    /// Returns the texture for a selected arena.
    pub fn get(&self, arena: ArenaId) -> Option<&Texture2D> {
        match arena {
            ArenaId::Sirius => self.sirius.as_ref(),
            ArenaId::Fortaleza => self.fortaleza.as_ref(),
            ArenaId::JavaStreet => self.java_street.as_ref(),
        }
    }
}

impl GameAssets {
    /// Loads all optional prototype assets.
    pub fn load(raylib: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        Self {
            arenas: ArenaAssets {
                sirius: load_texture_optional(raylib, thread, ARENA_SIRIUS_PATH),
                fortaleza: load_texture_optional(raylib, thread, ARENA_FORTALEZA_PATH),
                java_street: load_texture_optional(raylib, thread, ARENA_JAVA_STREET_PATH),
            },
            fighter_spritesheet: load_texture_optional(raylib, thread, FIGHTER_SPRITESHEET_PATH),
            rust_fighter: load_sprite_atlas_optional(raylib, thread, RUST_FIGHTER_MANIFEST_PATH),
            rust_start: load_sprite_atlas_optional(raylib, thread, RUST_START_MANIFEST_PATH),
            duke_fighter: load_sprite_atlas_optional(raylib, thread, DUKE_FIGHTER_MANIFEST_PATH),
            duke_start: load_sprite_atlas_optional(raylib, thread, DUKE_START_MANIFEST_PATH),
            rust_projectile: load_texture_optional(raylib, thread, RUST_GEAR_PROJECTILE_PATH),
            duke_projectile: load_texture_optional(raylib, thread, DUKE_BEAN_PROJECTILE_PATH),
            countdown_11: load_texture_optional(raylib, thread, COUNTDOWN_11_PATH),
            countdown_10: load_texture_optional(raylib, thread, COUNTDOWN_10_PATH),
            countdown_01: load_texture_optional(raylib, thread, COUNTDOWN_01_PATH),
            countdown_fight: load_texture_optional(raylib, thread, COUNTDOWN_FIGHT_PATH),
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
