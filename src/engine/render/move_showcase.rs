//! Draws contextual combat demonstrations with both actors and real contact feedback.
//!
//! System: Raylib render boundary. Reuses match sprites, projectiles and effects;
//! the scene supplies stable scenario and hit/guard results without debug boxes.

use raylib::prelude::*;

use crate::{
    characters::character_spec,
    config::{WINDOW_HEIGHT, WINDOW_WIDTH},
    engine::assets::GameAssets,
    game::arena::ArenaId,
    scenes::move_showcase::{MoveShowcase, ShowcaseResult},
};

use super::{
    BACKGROUND, FighterDrawOptions, UI_MUTED, UI_TEXT, draw_arena, draw_fighter,
    draw_fighter_ground_lights, draw_hit_effects, draw_menu_text, draw_projectiles,
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
    super::signature_effects::draw_signature_effects(draw, world, false, assets);
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
    // Keep the entire airspace visible: throws and uppercuts can reach above
    // the old top HUD. The showcase controls now occupy only the floor margin.
    let top = WINDOW_HEIGHT - 104;
    draw.draw_rectangle(0, top, WINDOW_WIDTH, 104, Color::new(0, 0, 0, 220));
    let title = format!(
        "MOVE SHOWCASE / {} / {:02}/{:02} / {}   {} [{}]",
        spec.display_name,
        showcase.move_number(),
        showcase.move_count(),
        showcase.move_label(),
        state,
        cycle
    );
    draw_menu_text(draw, font, &title, 18, top + 5, 14.0, UI_TEXT);
    let (result, color) = match showcase.result() {
        ShowcaseResult::Pending => ("Waiting for contact".to_owned(), UI_MUTED),
        ShowcaseResult::Hit { damage } => {
            (format!("HIT -{damage} HP"), Color::new(255, 215, 132, 255))
        }
        ShowcaseResult::Blocked { damage } => (
            format!("BLOCK -{damage} HP chip"),
            Color::new(139, 219, 255, 255),
        ),
        ShowcaseResult::Whiff => (
            "MISS / replay to inspect".to_owned(),
            Color::new(255, 140, 130, 255),
        ),
    };
    let world = showcase.world();
    let situation = format!(
        "{}   |   {}   |   HP {} : {}",
        showcase.scenario_label(),
        result,
        world.player_one.health,
        world.player_two.health
    );
    draw_menu_text(draw, font, &situation, 18, top + 30, 12.0, color);
    draw_menu_text(
        draw,
        font,
        showcase.scenario_description(),
        18,
        top + 53,
        11.0,
        UI_MUTED,
    );
    let footer = format!(
        "Tab/Shift+Tab: move   Enter: replay   Space: pause   .: frame   L: repeat   X: sides   PgUp/PgDn: fighter   Home: reset   Esc: menu   [{:03}]",
        showcase.current_frame()
    );
    draw_menu_text(draw, font, &footer, 18, top + 80, 10.0, UI_TEXT);
}
