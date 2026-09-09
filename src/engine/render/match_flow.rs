//! Draws the pause card and a result overlay around the fighters.
//!
//! Menus reuse scene hit regions and the UI clock; drawing never advances combat.

use raylib::prelude::*;

use super::presentation::{CYAN, GOLD, MUTED, centered, label, panel};
use super::{DrawTarget, GameAssets};
use crate::characters::character_spec;
use crate::combat::fighter::PlayerSlot;
use crate::config::{WINDOW_HEIGHT, WINDOW_WIDTH, screen_px};
use crate::game::world::{MatchOutcome, World};
use crate::math::rect::Rect;
use crate::scenes::match_flow::{MatchFlow, MatchFlowKind};

const TEXT: Color = Color::new(236, 244, 250, 255);

/// Overlays the existing arena; result controls stay beneath the fighters.
pub fn draw_match_flow(
    draw: &mut impl DrawTarget,
    flow: &MatchFlow,
    world: &World,
    time: f32,
    assets: &GameAssets,
) {
    let entrance = (flow.elapsed_seconds() / 0.3).clamp(0.0, 1.0);
    let reveal = 1.0 - (1.0 - entrance).powi(3);
    let accent = match flow.kind() {
        MatchFlowKind::Pause => CYAN,
        MatchFlowKind::Result => GOLD,
    };
    match flow.kind() {
        MatchFlowKind::Pause => draw_pause(draw, assets, reveal),
        MatchFlowKind::Result => draw_result(draw, assets, world, reveal),
    }
    for (index, action) in flow.actions().iter().enumerate() {
        let bounds = flow.row_bounds(index).expect("known menu row");
        let selected = index == flow.selected();
        let row_reveal = ((flow.elapsed_seconds() - index as f32 * 0.035) / 0.18).clamp(0.0, 1.0);
        let pulse = if selected {
            0.82 + (time * 3.2).sin() * 0.12
        } else {
            0.0
        };
        draw.draw_rectangle_rec(
            ray_rect(bounds),
            if selected {
                faded(accent, 0.16 * row_reveal)
            } else {
                Color::new(17, 29, 41, (130.0 * row_reveal) as u8)
            },
        );
        draw.draw_rectangle_lines_ex(
            ray_rect(bounds),
            1.0,
            faded(accent, if selected { pulse } else { 0.16 } * row_reveal),
        );
        let color = faded(if selected { TEXT } else { MUTED }, row_reveal);
        match flow.kind() {
            MatchFlowKind::Pause => {
                if selected {
                    label(
                        draw,
                        assets,
                        ">",
                        bounds.x as i32 + 17,
                        bounds.y as i32 + 10,
                        23.0,
                        faded(accent, row_reveal),
                    );
                    label(
                        draw,
                        assets,
                        "ENTER / A",
                        bounds.right() as i32 - 107,
                        bounds.y as i32 + 17,
                        12.0,
                        faded(accent, row_reveal),
                    );
                }
                label(
                    draw,
                    assets,
                    action.label(flow.kind()),
                    bounds.x as i32 + 45,
                    bounds.y as i32 + 11,
                    23.0,
                    color,
                );
            }
            MatchFlowKind::Result => {
                if selected {
                    draw.draw_rectangle(
                        bounds.x as i32 + 12,
                        bounds.bottom() as i32 - 3,
                        bounds.width as i32 - 24,
                        2,
                        faded(accent, row_reveal),
                    );
                }
                centered(
                    draw,
                    assets,
                    action.label(flow.kind()),
                    bounds.center_x() as i32,
                    bounds.y as i32 + 10,
                    21.0,
                    color,
                );
            }
        }
    }
}

fn draw_pause(draw: &mut impl DrawTarget, assets: &GameAssets, reveal: f32) {
    draw.draw_rectangle(
        0,
        0,
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        Color::new(2, 7, 14, (160.0 * reveal) as u8),
    );
    let card = scaled_rect(240, 92, 480, 366);
    panel(draw, card, faded(CYAN, reveal));
    draw.draw_rectangle_gradient_v(
        card.x as i32 + 1,
        card.y as i32 + 1,
        card.width as i32 - 2,
        screen_px(99),
        faded(CYAN, 0.08 * reveal),
        Color::BLANK,
    );
    centered(
        draw,
        assets,
        "EXECUÇÃO SUSPENSA",
        WINDOW_WIDTH / 2,
        screen_px(110),
        14.0,
        faded(CYAN, reveal),
    );
    centered(
        draw,
        assets,
        "PAUSA",
        WINDOW_WIDTH / 2,
        screen_px(130),
        45.0,
        faded(TEXT, reveal),
    );
    centered(
        draw,
        assets,
        "A luta espera. Escolha como continuar.",
        WINDOW_WIDTH / 2,
        screen_px(173),
        18.0,
        faded(MUTED, reveal),
    );
    let width = (screen_px(360) as f32 * reveal) as i32;
    draw.draw_rectangle(
        WINDOW_WIDTH / 2 - width / 2,
        screen_px(199),
        width,
        1,
        faded(CYAN, 0.45 * reveal),
    );
    centered(
        draw,
        assets,
        "SETAS / DIRECIONAL   navegar     ENTER / A   confirmar",
        WINDOW_WIDTH / 2,
        screen_px(419),
        13.0,
        faded(MUTED, reveal),
    );
    centered(
        draw,
        assets,
        "ESC / B   continuar",
        WINDOW_WIDTH / 2,
        screen_px(437),
        13.0,
        faded(CYAN, reveal),
    );
}

fn draw_result(draw: &mut impl DrawTarget, assets: &GameAssets, world: &World, reveal: f32) {
    let (title, owner) = match world.outcome {
        Some(MatchOutcome::Winner(slot)) => (
            format!(
                "{} venceu!",
                character_spec(world.character_for_slot(slot)).display_name
            ),
            match slot {
                PlayerSlot::One => "JOGADOR 1  //  BUILD APROVADO",
                PlayerSlot::Two => "JOGADOR 2  //  BUILD APROVADO",
            },
        ),
        Some(MatchOutcome::Draw) => ("EMPATE".to_owned(), "DOIS PROCESSOS. O MESMO DESTINO."),
        None => ("FIM DE LUTA".to_owned(), "ROUND CONCLUÍDO"),
    };
    // Only this non-interactive banner slides. Clickable rows keep stable bounds.
    let mut banner = scaled_rect(210, 78, 540, 86);
    banner.y -= (1.0 - reveal) * 14.0;
    panel(draw, banner, faded(GOLD, reveal));
    centered(
        draw,
        assets,
        "ROUND CONCLUÍDO",
        WINDOW_WIDTH / 2,
        banner.y as i32 + 12,
        12.0,
        faded(GOLD, reveal),
    );
    centered(
        draw,
        assets,
        &title,
        WINDOW_WIDTH / 2,
        banner.y as i32 + 32,
        39.0,
        faded(TEXT, reveal),
    );
    centered(
        draw,
        assets,
        owner,
        WINDOW_WIDTH / 2,
        banner.y as i32 + 85,
        13.0,
        faded(MUTED, reveal),
    );
    let completion = (world.outcome_elapsed_seconds() / 0.6).clamp(0.0, 1.0) * reveal;
    draw.draw_rectangle(
        banner.x as i32 + 24,
        (banner.y + banner.height) as i32 - 2,
        ((banner.width - 48.0) * completion) as i32,
        2,
        faded(GOLD, reveal),
    );

    panel(draw, scaled_rect(72, 464, 816, 68), faded(GOLD, reveal));
    centered(
        draw,
        assets,
        "SETAS / DIRECIONAL   escolher     ENTER / A   confirmar     ESC / B   menu",
        WINDOW_WIDTH / 2,
        screen_px(515),
        13.0,
        faded(MUTED, reveal),
    );
}

fn scaled_rect(x: i32, y: i32, width: i32, height: i32) -> Rectangle {
    Rectangle::new(
        screen_px(x) as f32,
        screen_px(y) as f32,
        screen_px(width) as f32,
        screen_px(height) as f32,
    )
}

fn ray_rect(bounds: Rect) -> Rectangle {
    Rectangle::new(bounds.x, bounds.y, bounds.width, bounds.height)
}

fn faded(color: Color, opacity: f32) -> Color {
    Color::new(
        color.r,
        color.g,
        color.b,
        (color.a as f32 * opacity.clamp(0.0, 1.0)) as u8,
    )
}
