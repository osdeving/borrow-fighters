//! Draws the shared circuit panels, typography and scene transition treatment.
//!
//! These effects use the UI clock and never advance combat or authored sequences.

use super::{DrawTarget, GameAssets};
use raylib::prelude::*;

pub const CYAN: Color = Color::new(76, 231, 228, 255);
pub const GOLD: Color = Color::new(255, 190, 96, 255);
pub const INK: Color = Color::new(7, 14, 25, 245);
pub const MUTED: Color = Color::new(135, 160, 177, 255);

pub fn label(
    draw: &mut impl DrawTarget,
    assets: &GameAssets,
    text: &str,
    x: i32,
    y: i32,
    size_px: f32,
    color: Color,
) {
    super::draw_menu_text(
        draw,
        assets.menu_font.as_ref(),
        text,
        x,
        y,
        size_px / crate::config::RESOLUTION_SCALE,
        color,
    );
}

pub fn centered(
    draw: &mut impl DrawTarget,
    assets: &GameAssets,
    text: &str,
    cx: i32,
    y: i32,
    size_px: f32,
    color: Color,
) {
    super::draw_centered_menu_text(
        draw,
        assets.menu_font.as_ref(),
        text,
        cx,
        y,
        size_px / crate::config::RESOLUTION_SCALE,
        color,
    );
}

pub fn panel(draw: &mut impl DrawTarget, bounds: Rectangle, accent: Color) {
    draw.draw_rectangle_rec(bounds, INK);
    draw.draw_rectangle_lines_ex(bounds, 1.0, Color::new(accent.r, accent.g, accent.b, 65));
    let x = bounds.x as i32;
    let y = bounds.y as i32;
    let right = (bounds.x + bounds.width) as i32;
    let bottom = (bounds.y + bounds.height) as i32;
    for (px, py, dx, dy) in [
        (x, y, 1, 1),
        (right, y, -1, 1),
        (x, bottom, 1, -1),
        (right, bottom, -1, -1),
    ] {
        draw.draw_line(px, py, px + 15 * dx, py, accent);
        draw.draw_line(px, py, px, py + 9 * dy, accent);
    }
}

pub fn circuit_background(draw: &mut impl DrawTarget, time: f32) {
    draw.clear_background(Color::new(5, 10, 20, 255));
    draw.draw_rectangle_gradient_v(
        0,
        0,
        1280,
        720,
        Color::new(10, 25, 43, 255),
        Color::new(5, 10, 20, 255),
    );
    draw.draw_circle_gradient(190, 355, 365.0, Color::new(0, 124, 147, 36), Color::BLANK);
    draw.draw_circle_gradient(1090, 355, 365.0, Color::new(184, 92, 35, 30), Color::BLANK);
    for i in 0..22 {
        let x = i * 64 - 32;
        draw.draw_line(x, 0, x, 720, Color::new(69, 111, 141, 10));
        let y = i * 40;
        draw.draw_line(0, y, 1280, y, Color::new(69, 111, 141, 10));
    }
    for i in 0..12 {
        let y = 90 + i * 48;
        let elbow = 65 + (i % 4) * 24;
        for mirror in [false, true] {
            let flip = |x| if mirror { 1280 - x } else { x };
            let color = if mirror {
                Color::new(205, 145, 73, 42)
            } else {
                Color::new(63, 185, 190, 42)
            };
            draw.draw_line(flip(0), y, flip(elbow), y, color);
            draw.draw_line(flip(elbow), y, flip(elbow + 38), y + 38, color);
            draw.draw_line(flip(elbow + 38), y + 38, flip(elbow + 112), y + 38, color);
            let travel = ((time * 45.0 + i as f32 * 31.0) % 170.0) as i32;
            let (px, py) = if travel < elbow {
                (travel, y)
            } else if travel < elbow + 38 {
                (travel, y + travel - elbow)
            } else {
                (travel, y + 38)
            };
            draw.draw_circle(flip(px), py, 2.0, if mirror { GOLD } else { CYAN });
        }
    }
    for i in 0..10 {
        let y = ((time * 13.0 + i as f32 * 77.0) % 760.0) as i32 - 20;
        draw.draw_rectangle(338, y, 2, 12, Color::new(76, 231, 228, 35));
        draw.draw_rectangle(940, 720 - y, 2, 12, Color::new(255, 190, 96, 35));
    }
}

/// Opens two circuit shutters; callers discard scene-entry input separately.
pub fn transition(draw: &mut impl DrawTarget, elapsed: f32) {
    let progress = (elapsed / 0.42).clamp(0.0, 1.0);
    if progress >= 1.0 {
        return;
    }
    let remaining = (1.0 - progress).powi(3);
    let width = (640.0 * remaining) as i32;
    draw.draw_rectangle(0, 0, width, 720, Color::new(5, 13, 23, 255));
    draw.draw_rectangle(1280 - width, 0, width, 720, Color::new(5, 13, 23, 255));
    draw.draw_rectangle(
        width,
        0,
        2,
        720,
        Color::new(76, 231, 228, (255.0 * remaining) as u8),
    );
    draw.draw_rectangle(
        1278 - width,
        0,
        2,
        720,
        Color::new(255, 190, 96, (255.0 * remaining) as u8),
    );
}
