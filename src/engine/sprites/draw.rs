//! Draws sprite frames through Raylib.
//!
//! The renderer aligns atlas frames by pivot so large character art can move
//! independently from combat hitboxes.

use raylib::prelude::*;

use crate::{
    combat::fighter::{Facing, Fighter},
    engine::sprites::{
        animation::frame_for_clip_at,
        manifest::{SpriteFrame, SpriteManifest},
        selection::{
            FighterSpriteClip, fighter_clip_elapsed_seconds, fighter_sprite_clip,
            fighter_sprite_frame,
        },
    },
};

const GREYBOX_FRAME_WIDTH: f32 = 96.0;
const GREYBOX_FRAME_HEIGHT: f32 = 128.0;
const MIN_RUNTIME_FIGHTER_SCALE: f32 = 0.1;
const PROJECTILE_SCALE: f32 = 0.45;

/// Draws one fighter from the placeholder spritesheet.
pub fn draw_fighter_sprite(
    draw: &mut RaylibDrawHandle<'_>,
    texture: &Texture2D,
    fighter: &Fighter,
    tint: Color,
) {
    let frame = fighter_sprite_frame(fighter);
    let body = fighter.body_rect();
    let mut source = Rectangle::new(
        frame.index() * GREYBOX_FRAME_WIDTH,
        0.0,
        GREYBOX_FRAME_WIDTH,
        GREYBOX_FRAME_HEIGHT,
    );

    if fighter.facing == Facing::Left {
        source.x += GREYBOX_FRAME_WIDTH;
        source.width = -GREYBOX_FRAME_WIDTH;
    }

    let dest = Rectangle::new(
        body.center_x() - GREYBOX_FRAME_WIDTH * 0.5,
        body.bottom() - GREYBOX_FRAME_HEIGHT,
        GREYBOX_FRAME_WIDTH,
        GREYBOX_FRAME_HEIGHT,
    );

    draw.draw_texture_pro(texture, source, dest, Vector2::new(0.0, 0.0), 0.0, tint);
}

/// Draws one fighter from a sprite manifest and atlas texture.
pub fn draw_manifest_fighter_sprite(
    draw: &mut RaylibDrawHandle<'_>,
    texture: &Texture2D,
    manifest: &SpriteManifest,
    fighter: &Fighter,
    world_elapsed_seconds: f32,
    forced_clip: Option<FighterSpriteClip>,
    tint: Color,
) -> bool {
    let clip = forced_clip.unwrap_or_else(|| fighter_sprite_clip(fighter));
    let clip_time = if forced_clip.is_some() {
        world_elapsed_seconds
    } else {
        fighter_clip_elapsed_seconds(fighter, world_elapsed_seconds)
    };
    let Some(frame) = frame_for_clip_at(manifest, clip.as_str(), clip_time) else {
        return false;
    };

    draw_manifest_frame(draw, texture, manifest, frame, fighter, tint);
    true
}

/// Draws the current projectile texture centered on a projectile rectangle.
pub fn draw_projectile_texture(
    draw: &mut RaylibDrawHandle<'_>,
    texture: &Texture2D,
    center: Vector2,
    facing: Facing,
    tint: Color,
) {
    let width = texture.width() as f32 * PROJECTILE_SCALE;
    let height = texture.height() as f32 * PROJECTILE_SCALE;
    let mut source = Rectangle::new(0.0, 0.0, texture.width() as f32, texture.height() as f32);

    if facing == Facing::Left {
        source.x += texture.width() as f32;
        source.width = -source.width;
    }

    let dest = Rectangle::new(center.x, center.y, width, height);
    let origin = Vector2::new(width * 0.5, height * 0.5);
    draw.draw_texture_pro(texture, source, dest, origin, 0.0, tint);
}

fn draw_manifest_frame(
    draw: &mut RaylibDrawHandle<'_>,
    texture: &Texture2D,
    manifest: &SpriteManifest,
    frame: &SpriteFrame,
    fighter: &Fighter,
    tint: Color,
) {
    let runtime_scale = manifest.scale.unwrap_or(1.0).max(MIN_RUNTIME_FIGHTER_SCALE);
    let body = fighter.body_rect();
    let anchor = Vector2::new(body.center_x(), body.bottom());
    let source_width = frame.frame.w as f32;
    let source_height = frame.frame.h as f32;
    let dest_width = source_width * runtime_scale;
    let dest_height = source_height * runtime_scale;
    let pivot_x = frame.pivot.x as f32 * runtime_scale;
    let pivot_y = frame.pivot.y as f32 * runtime_scale;
    let mut source = Rectangle::new(
        frame.frame.x as f32,
        frame.frame.y as f32,
        source_width,
        source_height,
    );

    let dest_x = if fighter.facing == Facing::Left {
        source.x += source_width;
        source.width = -source_width;
        anchor.x - (dest_width - pivot_x)
    } else {
        anchor.x - pivot_x
    };

    let dest = Rectangle::new(dest_x, anchor.y - pivot_y, dest_width, dest_height);
    draw.draw_texture_pro(texture, source, dest, Vector2::new(0.0, 0.0), 0.0, tint);
}
