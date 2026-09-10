//! Draws chapter objectives, interaction prompts and the save-aware menu overlay.
//!
//! System: Adventure chapter interface. These screen-space widgets consume
//! sampled state without owning camera, actors, progress or input.

use super::{GOLD, INK, PAPER, assets::ChapterAssets};
use crate::adventure::{
    chapter::{Chapter, Phase, Scene},
    engine::typography,
};
use raylib::prelude::*;

pub(super) fn objective(d: &mut impl RaylibDraw, a: &ChapterAssets, chapter: &Chapter) {
    d.draw_rectangle_rounded(
        Rectangle::new(30.0, 25.0, 640.0, 77.0),
        0.12,
        8,
        Color::new(20, 33, 29, 228),
    );
    let location = match chapter.scene {
        Scene::Street => "01  /  CASA NOSSA",
        Scene::Lane => "01  /  TRAVESSA DO SOL",
        Scene::Passage => "01  /  PASSAGEM DA VILA",
    };
    d.draw_text_ex(
        &a.common.signage,
        location,
        Vector2::new(48.0, 35.0),
        18.0,
        1.1,
        GOLD,
    );
    typography::paragraph(
        d,
        &a.common.body,
        a.texts.get(chapter.objective_key()),
        Rectangle::new(48.0, 62.0, 598.0, 30.0),
        24.0,
        PAPER,
    );
    if chapter.phase == Phase::PassageCombat {
        for (x, hp, max, label) in [
            (48, chapter.player().hp, chapter.player().max_hp, "RUST"),
            (
                958,
                chapter.combat.enemy.hp,
                chapter.combat.enemy.max_hp,
                "ERRÁTICO",
            ),
        ] {
            d.draw_rectangle(x, 125, 250, 8, INK);
            d.draw_rectangle(
                x,
                125,
                (250.0 * (hp as f32 / max as f32).clamp(0.0, 1.0)) as i32,
                8,
                GOLD,
            );
            d.draw_text_ex(
                &a.common.signage,
                label,
                Vector2::new(x as f32, 105.0),
                16.0,
                1.0,
                INK,
            );
        }
    }
}

pub(super) fn prompt(d: &mut impl RaylibDraw, a: &ChapterAssets, text: &str) {
    let width = a.common.body.measure_text(text, 22.0, 0.2).x + 44.0;
    d.draw_rectangle_rounded(
        Rectangle::new(640.0 - width * 0.5, 625.0, width, 47.0),
        0.25,
        8,
        INK,
    );
    typography::centered(
        d,
        &a.common.body,
        text,
        Vector2::new(640.0, 637.0),
        width - 30.0,
        22.0,
        PAPER,
    );
}

pub(super) fn overlay(d: &mut impl RaylibDraw, a: &ChapterAssets, title: &str, detail: &str) {
    d.draw_rectangle(0, 0, 1280, 720, Color::new(11, 24, 23, 200));
    typography::centered(
        d,
        &a.common.title,
        title,
        Vector2::new(640.0, 293.0),
        1060.0,
        41.0,
        PAPER,
    );
    typography::centered(
        d,
        &a.common.body,
        detail,
        Vector2::new(640.0, 374.0),
        1080.0,
        24.0,
        PAPER,
    );
}

/// A compact chapter menu over the still in-game camera; no illustrated panels.
pub fn menu(
    d: &mut impl RaylibDraw,
    a: &ChapterAssets,
    title: &str,
    rows: &[&str],
    selected: usize,
    detail: &str,
) {
    d.draw_rectangle(0, 0, 1280, 720, Color::new(10, 25, 25, 208));
    typography::centered(
        d,
        &a.common.title,
        title,
        Vector2::new(640.0, 140.0),
        980.0,
        43.0,
        PAPER,
    );
    for (index, row) in rows.iter().enumerate() {
        let y = 255.0 + index as f32 * 65.0;
        if selected == index {
            d.draw_rectangle_rounded(
                Rectangle::new(390.0, y - 9.0, 500.0, 52.0),
                0.16,
                8,
                Color::new(76, 93, 68, 240),
            );
        }
        typography::centered(
            d,
            &a.common.body,
            row,
            Vector2::new(640.0, y),
            455.0,
            27.0,
            if selected == index { GOLD } else { PAPER },
        );
    }
    typography::centered(
        d,
        &a.common.body,
        detail,
        Vector2::new(640.0, 576.0),
        1060.0,
        19.0,
        Color::new(188, 206, 190, 255),
    );
    typography::centered(
        d,
        &a.common.body,
        "Setas / direcional   Enter / A selecionar   Esc / B voltar",
        Vector2::new(640.0, 665.0),
        1000.0,
        18.0,
        PAPER,
    );
}
