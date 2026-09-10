//! Draws the terminal main menu and the cinematic Borrow Fighters wordmark.
//!
//! System: Fighting presentation. Colors and type follow the game's opening
//! identity, using only the fighting domain's fonts and menu navigation state.

use super::{
    DrawTarget, PreferencesDrawOptions, draw_centered_menu_text, draw_menu_text, menu_text_width,
};
use crate::{
    config::{WINDOW_HEIGHT, WINDOW_WIDTH, screen_px, world_px},
    scenes::preferences::{MenuPage, PreferencesMenu},
    ui::{
        binary_text::{DEFAULT_BINARY_REVEAL_FRAMES, binary_reveal_text_with_seed},
        menu_layout::MenuLayout,
    },
};
use raylib::prelude::*;

const INK: Color = Color::new(14, 19, 26, 255);
const PAPER: Color = Color::new(239, 224, 192, 255);
const GOLD: Color = Color::new(248, 177, 64, 255);
const TEAL: Color = Color::new(83, 199, 182, 255);
const MUTED: Color = Color::new(148, 165, 164, 255);
const ROWS: [(&str, &str); 7] = [
    ("MODO HISTÓRIA", "A jornada de Rust continua"),
    ("VERSUS SETUP", "Escolha lutadores e cenário"),
    ("TRAINING", "Domine golpes e especiais"),
    ("LORE / ROSTER", "Conheça quem está no ringue"),
    ("OPTIONS", "Áudio, controles e preferências"),
    ("COMO JOGAR", "Controles, duelo local e modo solo"),
    ("EXIT", "Até o próximo round"),
];

pub(super) fn draw(draw: &mut impl DrawTarget, options: &PreferencesDrawOptions<'_>) {
    backdrop(draw, options.visual_time_seconds);
    wordmark(draw, options);
    terminal(draw, options);

    let entry = (options.menu.main_entry_frames() as f32 / 30.0).clamp(0.0, 1.0);
    if entry < 1.0 {
        draw.draw_rectangle(
            0,
            0,
            WINDOW_WIDTH,
            WINDOW_HEIGHT,
            INK.alpha((1.0 - entry).powi(2)),
        );
    }
}

fn backdrop(draw: &mut impl DrawTarget, t: f32) {
    draw.clear_background(INK);
    draw.draw_rectangle_gradient_v(
        0,
        0,
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        Color::new(22, 35, 41, 255),
        Color::new(8, 13, 19, 255),
    );
    for index in 0..8 {
        let x = screen_px(24 + index * 61);
        let bend = screen_px(88 + (index * 37) % 160);
        let end_x = x + screen_px(44);
        let bottom = WINDOW_HEIGHT - screen_px(28 + (index * 17) % 50);
        let color = if index % 2 == 0 { TEAL } else { GOLD }.alpha(0.065);
        draw.draw_line(x, screen_px(28), x, bend, color);
        draw.draw_line(x, bend, end_x, bend + screen_px(44), color);
        draw.draw_line(end_x, bend + screen_px(44), end_x, bottom, color);
        draw.draw_circle_lines(end_x, bottom, world_px(3.0), color);
    }
    for index in 0..24 {
        let seed = index as f32;
        let x = ((seed * 137.0 + 41.0) % 460.0) + (t * 0.12 + seed).sin() * 5.0;
        let y = (seed * 71.0 - t * 6.0).rem_euclid(510.0) + 15.0;
        let alpha = 0.10 + (t * 0.6 + seed).sin().abs() * 0.13;
        draw.draw_circle(
            world_px(x) as i32,
            world_px(y) as i32,
            world_px(if index % 3 == 0 { 1.5 } else { 0.8 }),
            if index % 2 == 0 { TEAL } else { GOLD }.alpha(alpha),
        );
    }
    draw.draw_line(
        screen_px(40),
        screen_px(518),
        screen_px(920),
        screen_px(518),
        TEAL.alpha(0.13),
    );
}

fn wordmark(draw: &mut impl DrawTarget, options: &PreferencesDrawOptions<'_>) {
    let font = options.assets.menu_font.as_ref();
    let center = screen_px(244);
    draw.draw_rectangle_pro(
        Rectangle::new(
            center as f32,
            world_px(245.0),
            world_px(415.0),
            world_px(178.0),
        ),
        Vector2::new(world_px(207.5), world_px(89.0)),
        -4.0,
        INK.alpha(0.93),
    );
    for y in [169, 328] {
        draw.draw_line_ex(
            Vector2::new(world_px(51.0), world_px(y as f32)),
            Vector2::new(world_px(437.0), world_px(y as f32)),
            world_px(1.5),
            GOLD,
        );
    }
    draw_centered_menu_text(draw, font, "BORROW", center, screen_px(168), 74.0, PAPER);
    draw_centered_menu_text(draw, font, "FIGHTERS", center, screen_px(239), 79.0, GOLD);
    draw_centered_menu_text(
        draw,
        font,
        "ENTRE DOIS MUNDOS",
        center,
        screen_px(352),
        25.0,
        PAPER,
    );
    draw_centered_menu_text(
        draw,
        options.assets.lore_body_font.as_ref(),
        "Entidades Programáticas e Humanos.",
        center,
        screen_px(390),
        12.0,
        TEAL,
    );
}

fn terminal(draw: &mut impl DrawTarget, options: &PreferencesDrawOptions<'_>) {
    let font = options.assets.menu_font.as_ref();
    let body = options.assets.lore_body_font.as_ref();
    let layout = MenuLayout::for_page(MenuPage::Main);
    let panel = layout.panel;
    draw.draw_rectangle(
        panel.x + screen_px(7),
        panel.y + screen_px(9),
        panel.width,
        panel.height,
        Color::BLACK.alpha(0.23),
    );
    draw.draw_rectangle(panel.x, panel.y, panel.width, panel.height, INK);
    draw.draw_rectangle_lines(
        panel.x,
        panel.y,
        panel.width,
        panel.height,
        TEAL.alpha(0.44),
    );
    draw.draw_rectangle(
        panel.x + 1,
        panel.y + 1,
        panel.width - 2,
        screen_px(31),
        Color::new(28, 42, 47, 255),
    );
    for (index, color) in [PAPER, GOLD, TEAL].into_iter().enumerate() {
        draw.draw_circle(
            panel.x + screen_px(17 + index as i32 * 12),
            panel.y + screen_px(16),
            world_px(2.5),
            color.alpha(0.70),
        );
    }
    draw_centered_menu_text(
        draw,
        font,
        "borrow_fighters",
        panel.x + panel.width / 2,
        panel.y + screen_px(7),
        14.0,
        PAPER,
    );
    draw_menu_text(
        draw,
        body,
        "MENU PRINCIPAL",
        panel.x + screen_px(24),
        panel.y + screen_px(45),
        10.0,
        MUTED,
    );
    for (index, (label, description)) in ROWS.iter().enumerate() {
        let row = layout.row_bounds(index);
        let selected = options.menu.selected() == index;
        let pulse = options.menu.main_entry_pulse_frames().max(if selected {
            options.menu.selection_pulse_frames()
        } else {
            0
        });
        if selected {
            draw.draw_rectangle(row.x, row.y, row.width, row.height, TEAL.alpha(0.085));
            draw.draw_line(
                row.x + screen_px(32),
                row.y + row.height,
                row.x + row.width,
                row.y + row.height,
                TEAL.alpha(0.23),
            );
            let cursor_lit = options.visual_time_seconds.rem_euclid(1.05) < 0.65 || pulse > 0;
            if cursor_lit {
                draw.draw_rectangle(
                    row.x + screen_px(10),
                    row.y + screen_px(7),
                    screen_px(10),
                    screen_px(19),
                    TEAL,
                );
            }
        }
        let revealed =
            binary_reveal_text_with_seed(label, pulse, DEFAULT_BINARY_REVEAL_FRAMES, index as u32);
        draw_menu_text(
            draw,
            font,
            &revealed,
            row.x + screen_px(33),
            row.y + screen_px(2),
            22.0,
            if pulse > 0 || selected { TEAL } else { PAPER },
        );
        draw_menu_text(
            draw,
            body,
            description,
            row.x + screen_px(34),
            row.y + screen_px(27),
            10.0,
            MUTED,
        );
        if index == PreferencesMenu::MAIN_STORY_ROW && !options.menu.story_available() {
            let label = "EM BREVE";
            let width = menu_text_width(font, label, 10.0, 1.0);
            draw_menu_text(
                draw,
                font,
                label,
                row.x + row.width - width - screen_px(12),
                row.y + screen_px(9),
                10.0,
                GOLD,
            );
        }
    }
    let footer_y = panel.y + panel.height - screen_px(35);
    draw.draw_line(
        panel.x + screen_px(22),
        footer_y,
        panel.x + panel.width - screen_px(22),
        footer_y,
        TEAL.alpha(0.17),
    );
    draw_menu_text(
        draw,
        body,
        "Mouse / Setas: navegar   ·   Clique / Enter / A: escolher",
        panel.x + screen_px(24),
        footer_y + screen_px(12),
        9.0,
        MUTED,
    );
}
