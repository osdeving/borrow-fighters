//! Composes the chapter's world camera, physical actors and readable interface.
//!
//! System: Adventure chapter presentation. World transforms apply once to every
//! actor and prop; captions and the messenger enlargement remain screen-space.

mod actors;
pub mod assets;
mod phone;
mod ui;
mod world;
pub use ui::menu;
use ui::{objective, overlay, prompt};
const GOLD: Color = Color::new(237, 183, 91, 255);

use crate::adventure::{
    chapter::{Chapter, Phase},
    combat::Outcome,
    engine::typography,
};
use assets::ChapterAssets;
use raylib::prelude::*;

const INK: Color = Color::new(19, 29, 29, 245);
const PAPER: Color = Color::new(245, 236, 213, 255);

/// Draws one sampled state without advancing simulation or animation clocks.
pub fn draw(
    d: &mut impl RaylibDraw,
    a: &ChapterAssets,
    chapter: &Chapter,
    wake_tick: u32,
    debug: bool,
) {
    let camera = chapter.camera();
    let transform = Camera2D {
        offset: Vector2::new(640.0, 360.0),
        target: Vector2::new(camera.target.x, camera.target.y),
        rotation: 0.0,
        zoom: camera.zoom,
    };
    d.clear_background(Color::new(179, 200, 195, 255));
    {
        let mut scene = d.begin_mode2D(transform);
        world::draw(&mut scene, a, chapter, wake_tick);
        actors::draw(&mut scene, a, chapter, debug);
        if debug {
            world::debug(&mut scene, chapter);
        }
    }
    let project = |x: f32, y: f32| {
        Vector2::new(
            (x - camera.target.x) * camera.zoom + 640.0,
            (y - camera.target.y) * camera.zoom + 360.0,
        )
    };
    if let Some(phone) = chapter.phone() {
        phone::draw(
            d,
            &a.common,
            &a.skin,
            phone,
            [
                a.texts.get("phone.first"),
                a.texts.get("phone.python"),
                a.texts.get("phone.last"),
            ],
            project(chapter.player().position.x, chapter.player().position.y),
        );
    }
    if chapter.phase == Phase::Intro {
        let reveal = (chapter.phase_ticks as f32 / 60.0).min(1.0);
        let alpha = (255.0 * reveal) as u8;
        d.draw_rectangle(0, 0, 1280, 52, INK);
        d.draw_rectangle(0, 660, 1280, 60, INK);
        d.draw_text_ex(
            &a.common.signage,
            "CAPÍTULO 01",
            Vector2::new(48.0, 77.0),
            21.0,
            2.0,
            Color::new(245, 216, 162, alpha),
        );
        d.draw_text_ex(
            &a.common.title,
            a.texts.get("chapter.title"),
            Vector2::new(46.0, 109.0),
            42.0,
            0.1,
            Color::new(250, 242, 222, alpha),
        );
        if chapter.phase_ticks < 30 {
            d.draw_rectangle(
                0,
                0,
                1280,
                720,
                Color::new(
                    17,
                    26,
                    26,
                    (255.0 * (1.0 - chapter.phase_ticks as f32 / 30.0)) as u8,
                ),
            );
        }
    } else {
        objective(d, a, chapter);
    }
    if let Some(dialogue) = chapter.active_dialogue() {
        let top = if chapter.phase == Phase::Intro {
            588.0
        } else {
            599.0
        };
        d.draw_rectangle_rounded(Rectangle::new(34.0, top, 1212.0, 101.0), 0.08, 8, INK);
        d.draw_rectangle(34, top as i32 + 12, 3, 73, GOLD);
        d.draw_text_ex(
            &a.common.signage,
            a.texts.get(dialogue.speaker),
            Vector2::new(57.0, top + 10.0),
            22.0,
            0.5,
            GOLD,
        );
        typography::paragraph(
            d,
            &a.common.body,
            a.texts.get(dialogue.key),
            Rectangle::new(57.0, top + 40.0, 1020.0, 53.0),
            25.0,
            PAPER,
        );
        d.draw_text_ex(
            &a.common.body,
            "Enter / RB",
            Vector2::new(1110.0, top + 69.0),
            16.0,
            0.2,
            Color::new(171, 191, 180, 255),
        );
    } else if let Some(interaction) = chapter.nearby_interaction() {
        prompt(d, a, a.texts.get(interaction.prompt_key));
    } else if chapter.controls_active() {
        let hint = if chapter.phase == Phase::PassageCombat {
            a.texts.get("controls.combat")
        } else if chapter.scene == crate::adventure::chapter::Scene::Lane
            && chapter.debris.iter().any(|piece| piece.hp > 0)
        {
            a.texts.get("controls.debris")
        } else {
            a.texts.get("controls.explore")
        };
        d.draw_rectangle(0, 681, 1280, 39, Color::new(20, 30, 28, 225));
        typography::centered(
            d,
            &a.common.body,
            hint,
            Vector2::new(640.0, 690.0),
            1150.0,
            19.0,
            PAPER,
        );
    } else if chapter.phase == Phase::Phone {
        typography::centered(
            d,
            &a.common.body,
            a.texts.get("controls.phone"),
            Vector2::new(640.0, 681.0),
            900.0,
            18.0,
            INK,
        );
    }
    if chapter.combat.outcome == Outcome::Defeat {
        overlay(
            d,
            a,
            "Rust precisa tentar novamente",
            a.texts.get("controls.retry"),
        );
    } else if chapter.phase == Phase::Complete {
        overlay(
            d,
            a,
            a.texts.get("objective.complete"),
            a.texts.get("chapter.complete"),
        );
        typography::centered(
            d,
            &a.common.body,
            a.texts.get("controls.complete"),
            Vector2::new(640.0, 451.0),
            900.0,
            21.0,
            GOLD,
        );
    }
    if matches!(chapter.phase, Phase::LaneExplore | Phase::PassageExplore)
        && chapter.phase_ticks < 24
    {
        d.draw_rectangle(
            0,
            0,
            1280,
            720,
            Color::new(
                16,
                25,
                24,
                (255.0 * (1.0 - chapter.phase_ticks as f32 / 24.0)) as u8,
            ),
        );
    }
    if debug {
        d.draw_rectangle(30, 121, 750, 27, Color::new(0, 0, 0, 180));
        d.draw_text(
            &format!(
                "{:?} / {:?} | feet {:.1},{:.1} | camera {:.2}",
                chapter.scene,
                chapter.phase,
                chapter.player().position.x,
                chapter.player().position.y,
                camera.zoom
            ),
            37,
            126,
            16,
            Color::LIME,
        );
    }
}
