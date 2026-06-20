//! Loads prototype assets at the Raylib boundary.
//!
//! Assets stay optional in the greybox phase so the game can still run with
//! procedural debug drawing if a local file is missing.

use raylib::prelude::*;

pub const ARENA_BACKGROUND_PATH: &str = "assets/placeholder/arena-terminal-compiler-lab.png";

/// Runtime textures used by the prototype renderer.
pub struct GameAssets {
    pub arena_background: Option<Texture2D>,
}

impl GameAssets {
    /// Loads all optional prototype assets.
    pub fn load(raylib: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        Self {
            arena_background: load_texture_optional(raylib, thread, ARENA_BACKGROUND_PATH),
        }
    }
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
