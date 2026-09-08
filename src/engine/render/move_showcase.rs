//! Draws contextual combat demonstrations with both actors and real contact feedback.
//!
//! System: Raylib render boundary. Reuses match sprites, projectiles and effects;
//! the scene supplies stable scenario and hit/guard results without debug boxes.

use raylib::prelude::*;

use crate::{
    characters::character_spec,
    config::{WINDOW_HEIGHT, WINDOW_WIDTH, screen_px},
    engine::assets::GameAssets,
    game::arena::ArenaId,
    scenes::move_showcase::{MoveShowcase, ShowcaseResult},
};

use super::{
    BACKGROUND, FighterDrawOptions, UI_MUTED, UI_TEXT, draw_arena, draw_fighter,
    draw_fighter_ground_lights, draw_health_bar, draw_hit_effects, draw_menu_text,
    draw_projectiles,
};

/// Draws two simulated fighters and the current move's practical situation.
pub fn draw_move_showcase(
    draw: &mut impl super::DrawTarget,
    showcase: &MoveShowcase,
    arena: ArenaId,
    visual_time_seconds: f32,
    assets: &GameAssets,
) {
    draw.clear_background(BACKGROUND);
    draw_arena(draw, arena, assets.arenas.get(arena), visual_time_seconds);
    let world = showcase.world();
    draw_fighter_ground_lights(draw, world);
    draw_projectiles(draw, world, false, assets);
    for (fighter, character) in [
        (&world.player_one, world.player_one_character()),
        (&world.player_two, world.player_two_character()),
    ] {
        let visuals = super::character_visuals(character, assets);
        draw_fighter(
            draw,
            fighter,
            FighterDrawOptions {
                body_color: visuals.body_color,
                show_debug: false,
                sprite_atlas: visuals.fight_atlas,
                spritesheet: assets.fighter_spritesheet.as_ref(),
                world_elapsed_seconds: world.elapsed_seconds,
                forced_clip: None,
            },
        );
    }
    draw_hit_effects(draw, world);
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
        "PLAYING"
    };
    let cycle = if showcase.repeat_current() {
        "REPEAT"
    } else {
        "ALL"
    };
    draw.draw_rectangle(0, 0, WINDOW_WIDTH, screen_px(159), Color::new(0, 0, 0, 188));
    let title = format!(
        "MOVE SHOWCASE / {} / {:02}/{:02} / {}   {}  [{}]",
        spec.display_name,
        showcase.move_number(),
        showcase.move_count(),
        showcase.move_label(),
        state,
        cycle
    );
    draw_menu_text(
        draw,
        font,
        &title,
        screen_px(24),
        screen_px(13),
        17.0,
        UI_TEXT,
    );
    draw_menu_text(
        draw,
        font,
        showcase.scenario_label(),
        screen_px(24),
        screen_px(41),
        18.0,
        Color::new(135, 255, 192, 255),
    );
    draw_menu_text(
        draw,
        font,
        showcase.scenario_description(),
        screen_px(24),
        screen_px(68),
        13.0,
        UI_MUTED,
    );
    let world = showcase.world();
    let (left, right) = if showcase.sides_reversed() {
        (&world.player_two, &world.player_one)
    } else {
        (&world.player_one, &world.player_two)
    };
    draw_health_bar(
        draw,
        screen_px(24),
        screen_px(120),
        left.health,
        left.max_health,
        left.name,
    );
    draw_health_bar(
        draw,
        WINDOW_WIDTH - screen_px(324),
        screen_px(120),
        right.health,
        right.max_health,
        right.name,
    );
    let (result, color) = match showcase.result() {
        ShowcaseResult::Pending => ("Waiting for contact".to_owned(), UI_MUTED),
        ShowcaseResult::Hit { damage } => {
            (format!("HIT  -{damage} HP"), Color::new(255, 215, 132, 255))
        }
        ShowcaseResult::Blocked { damage } => (
            format!("BLOCK  -{damage} HP chip"),
            Color::new(139, 219, 255, 255),
        ),
        ShowcaseResult::Whiff => (
            "MISS / replay to inspect".to_owned(),
            Color::new(255, 140, 130, 255),
        ),
    };
    draw_menu_text(
        draw,
        font,
        &result,
        screen_px(359),
        screen_px(120),
        14.0,
        color,
    );
    draw.draw_rectangle(
        0,
        WINDOW_HEIGHT - screen_px(47),
        WINDOW_WIDTH,
        screen_px(47),
        Color::new(0, 0, 0, 200),
    );
    draw_menu_text(
        draw,
        font,
        "Tab / Shift+Tab: next / previous   Enter: replay   Space: pause   . : frame step   Home: reset",
        screen_px(24),
        WINDOW_HEIGHT - screen_px(37),
        12.0,
        UI_TEXT,
    );
    let footer = format!(
        "L: repeat   X: switch sides   PgUp/PgDn: character   Esc: menu     Frame {:03}",
        showcase.current_frame()
    );
    draw_menu_text(
        draw,
        font,
        &footer,
        screen_px(24),
        WINDOW_HEIGHT - screen_px(19),
        12.0,
        UI_MUTED,
    );
}
