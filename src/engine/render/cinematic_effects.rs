//! Composes six screen-wide special presentations from the live attack clock.
//!
//! System: Raylib presentation. These shapes never own collision or damage;
//! foreground contact emphasis stays around the fighter's actual local strike.

use std::f32::consts::{PI, TAU};

use raylib::prelude::*;

use crate::{
    characters::CharacterId,
    combat::{cinematic::CinematicSpecialState, fighter::Fighter},
    config::{FLOOR_Y, WINDOW_HEIGHT, WINDOW_WIDTH},
    engine::assets::GameAssets,
};

use super::{DrawTarget, draw_menu_text};

fn palette(character: CharacterId) -> (Color, Color, &'static str) {
    match character {
        CharacterId::Rust => (
            Color::new(255, 151, 64, 255),
            Color::new(255, 219, 133, 255),
            "OWN / BORROW / RELEASE",
        ),
        CharacterId::Duke => (
            Color::new(255, 87, 57, 255),
            Color::new(97, 206, 255, 255),
            "JVM // ALL THREADS ONLINE",
        ),
        CharacterId::Go => (
            Color::new(50, 224, 218, 255),
            Color::new(193, 255, 239, 255),
            "go func() // CHANNELS OPEN",
        ),
        CharacterId::C => (
            Color::new(255, 70, 98, 255),
            Color::new(88, 209, 255, 255),
            "0xDEADBEEF // CORE DUMP",
        ),
        CharacterId::Python => (
            Color::new(83, 158, 255, 255),
            Color::new(255, 214, 80, 255),
            "import universe // escape velocity = 0",
        ),
        CharacterId::Cpp => (
            Color::new(168, 129, 255, 255),
            Color::new(255, 214, 129, 255),
            "template <class Universe>",
        ),
    }
}

fn alpha(mut color: Color, opacity: f32) -> Color {
    color.a = (opacity.clamp(0.0, 1.0) * 255.0) as u8;
    color
}

fn envelope(state: CinematicSpecialState) -> f32 {
    let p = state.progress();
    (p / 0.12).min(1.0) * ((1.0 - p) / 0.28).clamp(0.0, 1.0)
}

fn polar(center: Vector2, radius: f32, angle: f32) -> Vector2 {
    Vector2::new(
        center.x + angle.cos() * radius,
        center.y + angle.sin() * radius,
    )
}

/// Draws the arena transformation behind both fighters; pause follows combat time.
pub(super) fn draw_background(
    draw: &mut impl DrawTarget,
    fighter: &Fighter,
    state: CinematicSpecialState,
    assets: &GameAssets,
) {
    let (primary, secondary, code) = palette(state.character);
    let strength = envelope(state);
    let p = state.progress();
    let t = state.elapsed_frames as f32 / 60.0;
    let width = WINDOW_WIDTH as f32;
    let height = WINDOW_HEIGHT as f32;
    let center = Vector2::new(width * 0.5, height * 0.48);
    draw.draw_rectangle(
        0,
        0,
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        alpha(Color::new(4, 8, 22, 255), strength * 0.78),
    );
    draw.draw_rectangle_gradient_v(
        0,
        80,
        WINDOW_WIDTH,
        530,
        alpha(primary, strength * 0.12),
        alpha(Color::BLACK, 0.0),
    );

    match state.character {
        CharacterId::Rust => {
            let radius = 150.0 + (p * PI).sin() * 55.0;
            draw.draw_circle_v(
                center,
                radius - 14.0,
                alpha(Color::new(8, 9, 17, 255), strength),
            );
            for ring in 0..3 {
                let r = radius + ring as f32 * 66.0;
                draw.draw_ring(
                    center,
                    r,
                    r + 3.0,
                    0.0,
                    360.0,
                    100,
                    alpha(primary, strength * (0.8 - ring as f32 * 0.22)),
                );
                for tooth in 0..24 {
                    let a = tooth as f32 * TAU / 24.0 + t * (0.3 + ring as f32 * 0.1);
                    let from = polar(center, r, a);
                    let to = polar(center, r + 19.0, a);
                    draw.draw_line_ex(from, to, 12.0, alpha(primary, strength * 0.58));
                }
            }
            for i in 0..8 {
                let a = i as f32 * TAU / 8.0 - t * 0.3;
                let point = polar(center, radius + 100.0, a);
                draw.draw_line_ex(
                    point,
                    polar(center, width, a),
                    2.0,
                    alpha(secondary, strength * 0.32),
                );
                draw.draw_poly_lines_ex(point, 6, 22.0, t * 20.0, 3.0, alpha(secondary, strength));
            }
            draw_menu_text(
                draw,
                assets.menu_font.as_ref(),
                "&mut WORLD",
                523,
                320,
                32.0,
                alpha(secondary, strength),
            );
        }
        CharacterId::Duke => {
            for i in 0..13 {
                let x = i as f32 * 108.0 - 20.0;
                let rise = ((t * 2.5 + i as f32 * 1.7).sin() * 0.5 + 0.5) * 220.0 + 110.0;
                let top = FLOOR_Y - rise;
                draw.draw_rectangle(
                    x as i32,
                    top as i32,
                    78,
                    rise as i32,
                    alpha(primary, strength * 0.14),
                );
                draw.draw_rectangle_lines_ex(
                    Rectangle::new(x, top, 78.0, rise),
                    2.0,
                    alpha(secondary, strength * 0.65),
                );
                for row in 0..5 {
                    let y = top + 20.0 + row as f32 * 30.0;
                    draw.draw_line_ex(
                        Vector2::new(x + 12.0, y),
                        Vector2::new(x + 64.0, y),
                        4.0,
                        alpha(primary, strength * 0.7),
                    );
                }
                let packet_y = 160.0 + (t * 420.0 + i as f32 * 73.0).rem_euclid(400.0);
                draw.draw_rectangle(
                    x as i32 + 26,
                    packet_y as i32,
                    28,
                    10,
                    alpha(secondary, strength),
                );
            }
            for i in 0..4 {
                let y = 160.0 + i as f32 * 102.0;
                let x = (t * 240.0 + i as f32 * 340.0).rem_euclid(width + 400.0) - 400.0;
                draw_menu_text(
                    draw,
                    assets.menu_font.as_ref(),
                    "public static void OVERDRIVE()",
                    x as i32,
                    y as i32,
                    27.0,
                    alpha(secondary, strength * 0.52),
                );
            }
        }
        CharacterId::Go => {
            for lane in 0..14 {
                let y = 157.0 + lane as f32 * 32.0;
                let end_y = y + (lane as f32 * 0.8 + t).sin() * 90.0;
                draw.draw_line_ex(
                    Vector2::new(0.0, y),
                    Vector2::new(width, end_y),
                    2.0,
                    alpha(primary, strength * 0.4),
                );
                for packet in 0..5 {
                    let x = (t * (460.0 + lane as f32 * 16.0)
                        + packet as f32 * 277.0
                        + lane as f32 * 71.0)
                        .rem_euclid(width);
                    let py = y + (end_y - y) * x / width;
                    draw.draw_rectangle(
                        x as i32,
                        py as i32 - 5,
                        25,
                        10,
                        alpha(secondary, strength * 0.75),
                    );
                    draw.draw_line_ex(
                        Vector2::new(x - 40.0, py),
                        Vector2::new(x, py),
                        5.0,
                        alpha(primary, strength * 0.28),
                    );
                }
            }
            for i in 0..3 {
                draw.draw_ring(
                    center,
                    100.0 + i as f32 * 56.0,
                    103.0 + i as f32 * 56.0,
                    t * 80.0 + i as f32 * 90.0,
                    t * 80.0 + i as f32 * 90.0 + 270.0,
                    80,
                    alpha(primary, strength * 0.65),
                );
            }
        }
        CharacterId::C => {
            for row in 0..8 {
                for col in 0..20 {
                    let index = row * 20 + col;
                    let fracture = (t * 2.0 + index as f32 * 1.31).sin();
                    let x = col as f32 * 67.0 + fracture * p * 18.0;
                    let y = 157.0 + row as f32 * 54.0 + fracture * p * 22.0;
                    let color = if index % 4 == 0 { secondary } else { primary };
                    draw.draw_rectangle_lines_ex(
                        Rectangle::new(x, y, 55.0, 40.0),
                        1.5,
                        alpha(color, strength * 0.32),
                    );
                    if index % 7 == 0 {
                        draw_menu_text(
                            draw,
                            assets.menu_font.as_ref(),
                            if index % 2 == 0 { "NULL" } else { "0xFF" },
                            x as i32 + 5,
                            y as i32 + 8,
                            16.0,
                            alpha(color, strength * 0.8),
                        );
                    }
                }
            }
            for branch in 0..3 {
                let mut last = Vector2::new(center.x, 158.0);
                for segment in 1..11 {
                    let next = Vector2::new(
                        center.x
                            + (segment as f32 * 1.9 + branch as f32).sin()
                                * (60.0 + branch as f32 * 100.0),
                        158.0 + segment as f32 * 47.0,
                    );
                    draw.draw_line_ex(last, next, 3.0, alpha(secondary, strength * 0.8));
                    last = next;
                }
            }
        }
        CharacterId::Python => {
            draw.draw_circle_v(center, 114.0, alpha(Color::new(2, 2, 12, 255), strength));
            for orbit in 0..8 {
                let radius = 127.0 + orbit as f32 * 24.0;
                let color = if orbit % 2 == 0 { primary } else { secondary };
                let start = t * (35.0 + orbit as f32 * 11.0) + orbit as f32 * 37.0;
                draw.draw_ring(
                    center,
                    radius,
                    radius + 3.5,
                    start,
                    start + 260.0,
                    80,
                    alpha(color, strength * 0.65),
                );
            }
            for i in 0..74 {
                let distance = 150.0 + ((i as f32 * 43.0 - t * 130.0).rem_euclid(630.0));
                let angle = i as f32 * 2.399 + t * 0.15;
                let point = polar(center, distance, angle);
                let end = polar(center, distance + 35.0, angle - 0.035);
                draw.draw_line_ex(
                    point,
                    end,
                    2.0,
                    alpha(if i % 2 == 0 { primary } else { secondary }, strength * 0.7),
                );
            }
        }
        CharacterId::Cpp => {
            for depth in (0..9).rev() {
                let radius = 45.0 + depth as f32 * 50.0 + (t * 1.2).sin() * 20.0;
                let angle = t * 18.0 + depth as f32 * 11.0;
                draw.draw_poly_lines_ex(
                    center,
                    4,
                    radius,
                    angle,
                    2.5,
                    alpha(
                        if depth % 2 == 0 { primary } else { secondary },
                        strength * 0.6,
                    ),
                );
                for side in 0..4 {
                    let point = polar(center, radius, angle.to_radians() + side as f32 * PI * 0.5);
                    draw_menu_text(
                        draw,
                        assets.menu_font.as_ref(),
                        if side % 2 == 0 { "++" } else { "<>" },
                        point.x as i32 - 12,
                        point.y as i32 - 14,
                        29.0,
                        alpha(secondary, strength * 0.7),
                    );
                }
            }
        }
    }

    // Speed lines travel from both screen edges toward the local attacker.
    let target = Vector2::new(fighter.position.x, FLOOR_Y - 100.0);
    for i in 0..16 {
        let edge = Vector2::new(
            if i % 2 == 0 { 0.0 } else { width },
            160.0 + i as f32 * 30.0,
        );
        let amount = 0.18 + (p * 0.14);
        let end = Vector2::new(
            edge.x + (target.x - edge.x) * amount,
            edge.y + (target.y - edge.y) * amount,
        );
        draw.draw_line_ex(edge, end, 2.0, alpha(secondary, strength * 0.4));
    }
    draw_menu_text(
        draw,
        assets.menu_font.as_ref(),
        code,
        26,
        WINDOW_HEIGHT - 55,
        20.0,
        alpha(secondary, strength * 0.72),
    );
}

/// Draws move identification and a compact energy arc over the actual strike.
pub(super) fn draw_foreground(
    draw: &mut impl DrawTarget,
    fighter: &Fighter,
    state: CinematicSpecialState,
    assets: &GameAssets,
) {
    let (primary, secondary, _) = palette(state.character);
    let strength = envelope(state);
    let active =
        state.elapsed_frames >= state.active_start && state.elapsed_frames <= state.active_end;
    let title_in = (state.elapsed_frames as f32 / 12.0).clamp(0.0, 1.0);
    let x = 28 + ((1.0 - title_in).powi(3) * -170.0) as i32;
    draw.draw_rectangle(
        0,
        125,
        WINDOW_WIDTH,
        66,
        alpha(Color::new(5, 10, 24, 255), strength * 0.86),
    );
    draw.draw_rectangle(0, 125, WINDOW_WIDTH, 2, alpha(primary, strength * 0.85));
    draw.draw_rectangle(0, 189, WINDOW_WIDTH, 2, alpha(secondary, strength * 0.6));
    draw_menu_text(
        draw,
        assets.menu_font.as_ref(),
        state.label,
        x,
        135,
        39.0,
        alpha(Color::WHITE, strength),
    );
    draw_menu_text(
        draw,
        assets.menu_font.as_ref(),
        "LINKER / OVERDRIVE",
        WINDOW_WIDTH - 225,
        147,
        17.0,
        alpha(secondary, strength),
    );
    if active {
        let age = (state.elapsed_frames - state.active_start) as f32
            / (state.active_end - state.active_start + 1).max(1) as f32;
        let direction = if fighter.facing == crate::combat::fighter::Facing::Right {
            1.0
        } else {
            -1.0
        };
        let center = Vector2::new(fighter.position.x + direction * 75.0, FLOOR_Y - 85.0);
        let radius = 50.0 + age * 80.0;
        draw.draw_ring(
            center,
            radius,
            radius + 8.0 * (1.0 - age),
            0.0,
            360.0,
            64,
            alpha(secondary, (1.0 - age) * 0.8),
        );
        for i in 0..12 {
            let angle = i as f32 * TAU / 12.0;
            draw.draw_line_ex(
                polar(center, radius * 0.6, angle),
                polar(center, radius, angle),
                3.0,
                alpha(primary, 1.0 - age),
            );
        }
    }
}
