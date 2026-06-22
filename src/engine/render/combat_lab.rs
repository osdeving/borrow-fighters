//! Draws the isolated Combat Lab scene.
//!
//! System: Raylib render boundary. This module draws lab snapshots produced by
//! `scenes::combat_lab` and does not own combat rules.
//!
//! The lab renderer stays next to the Raylib boundary while the lab state stays
//! testable under `scenes/combat_lab.rs`.

use raylib::prelude::*;

use crate::characters::CharacterId;
use crate::combat::{fighter::Facing, projectile::Projectile};
use crate::config::{FLOOR_Y, WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::engine::{assets::GameAssets, sprites};
use crate::game::arena::ArenaId;
use crate::scenes::combat_lab::{CombatLab, CombatLabPose};
use crate::ui::combat_debug;

use super::{
    BACKGROUND, FighterDrawOptions, PLAYER_GO, PLAYER_ONE, PLAYER_TWO, PROJECTILE, PROJECTILE_FILL,
    UI_MUTED, draw_arena, draw_fighter, outline_rect,
};

/// Draws the isolated Combat Lab scene.
pub fn draw_combat_lab(draw: &mut RaylibDrawHandle<'_>, lab: &CombatLab, assets: &GameAssets) {
    draw.clear_background(BACKGROUND);
    if lab.show_background() {
        draw_arena(draw, assets.arenas.get(ArenaId::STARTING_ARENA));
        draw_lab_grid(draw, Color::new(44, 49, 60, 118));
    } else {
        draw_lab_grid(draw, Color::new(44, 49, 60, 255));
    }

    let (body_color, sprite_atlas, projectile_texture) = match lab.character() {
        CharacterId::Rust => (
            PLAYER_ONE,
            assets.rust_fighter.as_ref(),
            assets.rust_projectile.as_ref(),
        ),
        CharacterId::Duke => (
            PLAYER_TWO,
            assets.duke_fighter.as_ref(),
            assets.duke_projectile.as_ref(),
        ),
        CharacterId::Go => (
            PLAYER_GO,
            assets.go_fighter.as_ref(),
            assets.go_projectile.as_ref(),
        ),
    };

    draw_fighter(
        draw,
        lab.fighter(),
        FighterDrawOptions {
            body_color,
            show_debug: false,
            sprite_atlas,
            spritesheet: assets.fighter_spritesheet.as_ref(),
            world_elapsed_seconds: lab.elapsed_seconds(),
            forced_clip: forced_clip_for_pose(lab.pose()),
        },
    );

    draw_lab_projectiles(draw, lab.projectiles(), projectile_texture);
    combat_debug::draw_combat_lab_debug(draw, lab);
}

fn draw_lab_grid(draw: &mut RaylibDrawHandle<'_>, line_color: Color) {
    for x in (0..=WINDOW_WIDTH).step_by(80) {
        draw.draw_line(x, 0, x, WINDOW_HEIGHT, line_color);
    }
    for y in (0..=WINDOW_HEIGHT).step_by(60) {
        draw.draw_line(0, y, WINDOW_WIDTH, y, line_color);
    }
    draw.draw_line(0, FLOOR_Y as i32, WINDOW_WIDTH, FLOOR_Y as i32, UI_MUTED);
}

fn draw_lab_projectiles(
    draw: &mut RaylibDrawHandle<'_>,
    projectiles: &[Projectile],
    texture: Option<&Texture2D>,
) {
    for projectile in projectiles {
        let rect = projectile.rect();
        if let Some(texture) = texture {
            let facing = if projectile.velocity.x < 0.0 {
                Facing::Left
            } else {
                Facing::Right
            };
            let center = rect.center();
            sprites::draw_projectile_texture(
                draw,
                texture,
                Vector2::new(center.x, center.y),
                facing,
                Color::WHITE,
            );
        } else {
            draw.draw_rectangle(
                rect.x.round() as i32,
                rect.y.round() as i32,
                rect.width.round() as i32,
                rect.height.round() as i32,
                PROJECTILE_FILL,
            );
        }
        outline_rect(draw, rect, PROJECTILE);
    }
}

fn forced_clip_for_pose(pose: CombatLabPose) -> Option<sprites::FighterSpriteClip> {
    match pose {
        CombatLabPose::Move => None,
        CombatLabPose::Idle => Some(sprites::FighterSpriteClip::Idle),
        CombatLabPose::Crouch => Some(sprites::FighterSpriteClip::Crouch),
        CombatLabPose::Jump => Some(sprites::FighterSpriteClip::Jump),
        CombatLabPose::Block => Some(sprites::FighterSpriteClip::Block),
        CombatLabPose::Hit => Some(sprites::FighterSpriteClip::Hit),
        CombatLabPose::Victory => Some(sprites::FighterSpriteClip::Taunt),
    }
}
