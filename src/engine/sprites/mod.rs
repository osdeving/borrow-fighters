//! Coordinates sprite manifests, animation, and Raylib drawing.
//!
//! This module keeps sprite metadata testable while the actual texture drawing
//! stays at the engine boundary.

pub mod animation;
pub mod combat;
mod draw;
pub mod manifest;
mod selection;

pub use animation::frame_for_clip_at;
pub use combat::{
    ProjectedSpriteCombat, project_frame_combat, projected_fighter_combat,
    projected_projectile_origin_for_clip,
};
pub use draw::{draw_fighter_sprite, draw_manifest_fighter_sprite, draw_projectile_texture};
pub use manifest::{
    SPRITE_SCHEMA, SpriteClip, SpriteCombatBox, SpriteCombatPoint, SpriteFrame, SpriteFrameCombat,
    SpriteManifest, SpriteManifestError, SpritePivot, SpriteRect, SpriteSize,
};
pub use selection::{
    FighterSpriteClip, FighterSpriteFrame, fighter_clip_elapsed_seconds, fighter_sprite_clip,
    fighter_sprite_frame,
};

pub const FIGHTER_SPRITESHEET_PATH: &str = "assets/placeholder/fighter-greybox-spritesheet.png";
pub const RUST_FIGHTER_MANIFEST_PATH: &str = "assets/placeholder/rust-fighter.sprite.json";
pub const RUST_START_MANIFEST_PATH: &str = "assets/placeholder/rust-start.sprite.json";
pub const RUST_GEAR_PROJECTILE_PATH: &str = "assets/placeholder/rust-gear-projectile.png";
pub const DUKE_FIGHTER_MANIFEST_PATH: &str = "assets/placeholder/duke-fighter.sprite.json";
pub const DUKE_START_MANIFEST_PATH: &str = "assets/placeholder/duke-start.sprite.json";
pub const DUKE_BEAN_PROJECTILE_PATH: &str = "assets/placeholder/duke-bean-projectile.png";
pub const GO_FIGHTER_MANIFEST_PATH: &str = "assets/placeholder/go-fighter.sprite.json";
pub const GO_START_MANIFEST_PATH: &str = "assets/placeholder/go-start.sprite.json";
pub const GO_CHANNEL_PROJECTILE_PATH: &str = "assets/placeholder/go-channel-projectile.png";
pub const C_FIGHTER_MANIFEST_PATH: &str = "assets/placeholder/c-fighter.sprite.json";
pub const C_START_MANIFEST_PATH: &str = "assets/placeholder/c-start.sprite.json";
pub const C_BITSTREAM_PROJECTILE_PATH: &str = "assets/placeholder/c-bitstream-projectile.png";
pub const PYTHON_FIGHTER_MANIFEST_PATH: &str = "assets/placeholder/python-fighter.sprite.json";
pub const PYTHON_DATA_PROJECTILE_PATH: &str = "assets/placeholder/python-data-projectile.png";
