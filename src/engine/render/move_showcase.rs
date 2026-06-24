//! Draws the single-character move showcase.
//!
//! System: Raylib render boundary. This module renders clean snapshots from
//! `scenes::move_showcase` and leaves combat playback to the scene model.

use raylib::prelude::*;

use crate::{
    characters::character_spec,
    combat::{fighter::Facing, projectile::Projectile},
    config::{FLOOR_Y, WINDOW_WIDTH, screen_px},
    engine::{assets::GameAssets, sprites},
    game::arena::ArenaId,
    scenes::move_showcase::MoveShowcase,
};

use super::{
    BACKGROUND, FighterDrawOptions, PROJECTILE_FILL, UI_MUTED, UI_TEXT, draw_arena, draw_fighter,
    draw_menu_text,
};

/// Draws a clean autoplay showcase for one character and every move.
pub fn draw_move_showcase(
    draw: &mut impl super::DrawTarget,
    showcase: &MoveShowcase,
    arena: ArenaId,
    visual_time_seconds: f32,
    assets: &GameAssets,
) {
    draw.clear_background(BACKGROUND);
    draw_arena(draw, arena, assets.arenas.get(arena), visual_time_seconds);
    draw.draw_rectangle(0, 0, WINDOW_WIDTH, screen_px(72), Color::new(0, 0, 0, 142));
    draw.draw_rectangle(
        0,
        screen_px(72),
        WINDOW_WIDTH,
        screen_px(2),
        Color::new(95, 255, 174, 120),
    );
    draw.draw_line(
        0,
        FLOOR_Y as i32,
        WINDOW_WIDTH,
        FLOOR_Y as i32,
        Color::new(255, 255, 255, 60),
    );

    let lab = showcase.lab();
    let visuals = super::character_visuals(showcase.character(), assets);
    draw_fighter(
        draw,
        lab.fighter(),
        FighterDrawOptions {
            body_color: visuals.body_color,
            show_debug: false,
            sprite_atlas: visuals.fight_atlas,
            spritesheet: assets.fighter_spritesheet.as_ref(),
            world_elapsed_seconds: lab.elapsed_seconds(),
            forced_clip: None,
        },
    );
    draw_showcase_projectiles(draw, lab.projectiles(), visuals.projectile_texture);
    draw_showcase_label(draw, showcase, assets);
}

fn draw_showcase_label(
    draw: &mut impl super::DrawTarget,
    showcase: &MoveShowcase,
    assets: &GameAssets,
) {
    let font = assets.menu_font.as_ref();
    let spec = character_spec(showcase.character());
    let state = if showcase.paused() {
        "PAUSED"
    } else if showcase.resting() {
        "READY"
    } else {
        "AUTO"
    };
    let title = format!(
        "MOVE SHOWCASE  /  {}  /  {:02}/{:02}",
        spec.display_name,
        showcase.move_number(),
        showcase.move_count()
    );
    let move_label = format!("{}  [{}]", showcase.selected_move().label(), state);
    draw_menu_text(
        draw,
        font,
        &title,
        screen_px(28),
        screen_px(18),
        18.0,
        UI_TEXT,
    );
    draw_menu_text(
        draw,
        font,
        &move_label,
        screen_px(28),
        screen_px(43),
        15.0,
        UI_MUTED,
    );
}

fn draw_showcase_projectiles(
    draw: &mut impl super::DrawTarget,
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
    }
}
