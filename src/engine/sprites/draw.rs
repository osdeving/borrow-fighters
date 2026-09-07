//! Draws sprite frames through Raylib.
//!
//! The renderer aligns atlas frames by pivot so large character art can move
//! independently from combat hitboxes.

use raylib::prelude::*;

use crate::{
    combat::fighter::{Facing, Fighter},
    config::{FLOOR_Y, RESOLUTION_SCALE, world_px},
    engine::sprites::{
        animation::frame_for_fighter_clip_at,
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
const PROJECTILE_SCALE: f32 = 0.45 * RESOLUTION_SCALE;

/// Draws one fighter from the placeholder spritesheet.
pub fn draw_fighter_sprite(
    draw: &mut impl RaylibDraw,
    texture: &Texture2D,
    fighter: &Fighter,
    tint: Color,
) {
    let frame = fighter_sprite_frame(fighter);
    let body = fighter.body_rect();
    let source = mirrored_source_rect(
        Rectangle::new(
            frame.index() * GREYBOX_FRAME_WIDTH,
            0.0,
            GREYBOX_FRAME_WIDTH,
            GREYBOX_FRAME_HEIGHT,
        ),
        fighter.facing == Facing::Left,
    );

    let dest = Rectangle::new(
        body.center_x() - world_px(GREYBOX_FRAME_WIDTH) * 0.5,
        body.bottom() - world_px(GREYBOX_FRAME_HEIGHT),
        world_px(GREYBOX_FRAME_WIDTH),
        world_px(GREYBOX_FRAME_HEIGHT),
    );

    draw.draw_texture_pro(texture, source, dest, Vector2::new(0.0, 0.0), 0.0, tint);
}

/// Draws one fighter from a sprite manifest and atlas texture.
pub fn draw_manifest_fighter_sprite<'a>(
    draw: &mut impl RaylibDraw,
    manifest: &SpriteManifest,
    fighter: &Fighter,
    world_elapsed_seconds: f32,
    forced_clip: Option<FighterSpriteClip>,
    tint: Color,
    texture_for_frame: impl Fn(&SpriteFrame) -> Option<&'a Texture2D>,
) -> bool {
    let clip = forced_clip.unwrap_or_else(|| fighter_sprite_clip(fighter));
    let clip_time = if forced_clip.is_some() {
        world_elapsed_seconds
    } else {
        fighter_clip_elapsed_seconds(fighter, world_elapsed_seconds)
    };
    let Some(frame) = frame_for_fighter_clip_at(manifest, clip, clip_time) else {
        return false;
    };
    let Some(texture) = texture_for_frame(frame) else {
        return false;
    };

    let (source, dest) = manifest_frame_geometry(manifest, frame, fighter, clip);
    draw.draw_texture_pro(texture, source, dest, Vector2::new(0.0, 0.0), 0.0, tint);
    true
}

/// Draws the current projectile texture centered on a projectile rectangle.
pub fn draw_projectile_texture(
    draw: &mut impl RaylibDraw,
    texture: &Texture2D,
    center: Vector2,
    facing: Facing,
    tint: Color,
) {
    let width = texture.width() as f32 * PROJECTILE_SCALE;
    let height = texture.height() as f32 * PROJECTILE_SCALE;
    let source = mirrored_source_rect(
        Rectangle::new(0.0, 0.0, texture.width() as f32, texture.height() as f32),
        facing == Facing::Left,
    );

    let dest = Rectangle::new(center.x, center.y, width, height);
    let origin = Vector2::new(width * 0.5, height * 0.5);
    draw.draw_texture_pro(texture, source, dest, origin, 0.0, tint);
}

/// Mirrors Raylib UVs within the original crop, without sampling the next cell.
pub(crate) fn mirrored_source_rect(mut source: Rectangle, mirrored: bool) -> Rectangle {
    // DrawTexturePro interprets negative width as a flip at the same source x.
    if mirrored {
        source.width = -source.width;
    }
    source
}

fn manifest_frame_geometry(
    manifest: &SpriteManifest,
    frame: &SpriteFrame,
    fighter: &Fighter,
    clip: FighterSpriteClip,
) -> (Rectangle, Rectangle) {
    let runtime_scale = manifest.scale.unwrap_or(1.0).max(MIN_RUNTIME_FIGHTER_SCALE);
    let body = fighter.body_rect();
    // KO freezes the physical body, including an airborne winner. Grounded
    // outcome artwork gets its own anchor while debug/combat retain that body.
    let anchor_y = if matches!(clip, FighterSpriteClip::Victory | FighterSpriteClip::Defeat) {
        FLOOR_Y
    } else {
        body.bottom()
    };
    let source_width = frame.frame.w as f32;
    let source_height = frame.frame.h as f32;
    let dest_width = source_width * runtime_scale;
    let dest_height = source_height * runtime_scale;
    let pivot_x = frame.pivot.x as f32 * runtime_scale;
    let pivot_y = frame.pivot.y as f32 * runtime_scale;
    let source = mirrored_source_rect(
        Rectangle::new(
            frame.frame.x as f32,
            frame.frame.y as f32,
            source_width,
            source_height,
        ),
        fighter.facing == Facing::Left,
    );

    let dest_x = if fighter.facing == Facing::Left {
        body.center_x() - (dest_width - pivot_x)
    } else {
        body.center_x() - pivot_x
    };

    let dest = Rectangle::new(dest_x, anchor_y - pivot_y, dest_width, dest_height);
    (source, dest)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        combat::fighter::{FighterInput, PlayerSlot},
        engine::sprites::{SpritePivot, SpriteRect},
    };

    #[test]
    fn mirroring_preserves_an_offset_atlas_crop_and_a_full_projectile_texture() {
        for crop in [
            Rectangle::new(355.0, 3080.0, 275.0, 329.0),
            Rectangle::new(0.0, 0.0, 64.0, 48.0),
        ] {
            let mirrored = mirrored_source_rect(crop, true);
            assert_eq!(mirrored.x, crop.x);
            assert_eq!(mirrored.y, crop.y);
            assert_eq!(mirrored.height, crop.height);
            assert_eq!(mirrored.width, -crop.width);
            assert_eq!(mirrored_source_rect(crop, false), crop);
        }
    }

    #[test]
    fn outcome_art_uses_floor_anchor_and_keeps_airborne_combat_body_unchanged() {
        let mut manifest =
            SpriteManifest::load("tests/fixtures/sprite-viewer-combat.sprite.json").unwrap();
        manifest.scale = Some(1.25);
        let mut frame = manifest.frames[0].clone();
        frame.frame = SpriteRect {
            x: 83,
            y: 47,
            w: 96,
            h: 128,
        };
        frame.pivot = SpritePivot { x: 25, y: 115 };
        let mut fighter = Fighter::new(PlayerSlot::One, "Rust", 320.0);
        fighter.update(
            1.0 / 60.0,
            FighterInput {
                jump: true,
                ..FighterInput::default()
            },
        );
        let physical_body = fighter.body_rect();
        let velocity = fighter.velocity;
        assert!(physical_body.bottom() < FLOOR_Y);

        for facing in [Facing::Right, Facing::Left] {
            fighter.facing = facing;
            for clip in [FighterSpriteClip::Victory, FighterSpriteClip::Defeat] {
                let (source, dest) = manifest_frame_geometry(&manifest, &frame, &fighter, clip);
                let pivot_x = if facing == Facing::Left {
                    96.0 - 25.0
                } else {
                    25.0
                };
                assert_eq!(dest.x + pivot_x * 1.25, physical_body.center_x());
                assert_eq!(dest.y + 115.0 * 1.25, FLOOR_Y);
                assert_eq!((source.x, source.y), (83.0, 47.0));
            }
            let (_, jumping) =
                manifest_frame_geometry(&manifest, &frame, &fighter, FighterSpriteClip::Jump);
            assert_eq!(jumping.y + 115.0 * 1.25, physical_body.bottom());
        }
        assert_eq!(fighter.body_rect(), physical_body);
        assert_eq!(fighter.velocity, velocity);
        assert!(!fighter.grounded);
    }
}
