//! Draws the greybox prototype.
//!
//! Rendering intentionally uses primitive shapes and debug overlays so gameplay
//! problems are visible before art production starts.

use raylib::prelude::*;

use crate::combat::fighter::PlayerSlot;
use crate::config::{ARENA_LEFT, ARENA_RIGHT, FLOOR_Y, WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::game::world::{MatchOutcome, World};
use crate::math::rect::Rect;

const BACKGROUND: Color = Color::new(18, 20, 26, 255);
const FLOOR: Color = Color::new(72, 76, 88, 255);
const PLAYER_ONE: Color = Color::new(112, 181, 255, 255);
const PLAYER_TWO: Color = Color::new(255, 178, 104, 255);
const HURTBOX: Color = Color::new(105, 240, 174, 255);
const HITBOX: Color = Color::new(255, 82, 82, 255);
const UI_TEXT: Color = Color::new(238, 241, 247, 255);
const UI_MUTED: Color = Color::new(165, 172, 185, 255);
const HEALTH_BACK: Color = Color::new(60, 62, 70, 255);
const HEALTH_FILL: Color = Color::new(76, 217, 100, 255);

/// Draws the current world state.
pub fn draw(draw: &mut RaylibDrawHandle<'_>, world: &World) {
    draw.clear_background(BACKGROUND);
    draw_arena(draw);
    draw_fighter(draw, &world.player_one, PLAYER_ONE);
    draw_fighter(draw, &world.player_two, PLAYER_TWO);
    draw_hud(draw, world);
    draw_help(draw);
}

fn draw_arena(draw: &mut RaylibDrawHandle<'_>) {
    draw.draw_rectangle(
        0,
        FLOOR_Y as i32,
        WINDOW_WIDTH,
        WINDOW_HEIGHT - FLOOR_Y as i32,
        FLOOR,
    );
    draw.draw_line(
        ARENA_LEFT as i32,
        FLOOR_Y as i32,
        ARENA_RIGHT as i32,
        FLOOR_Y as i32,
        UI_MUTED,
    );
    draw.draw_line(
        ARENA_LEFT as i32,
        96,
        ARENA_LEFT as i32,
        FLOOR_Y as i32,
        UI_MUTED,
    );
    draw.draw_line(
        ARENA_RIGHT as i32,
        96,
        ARENA_RIGHT as i32,
        FLOOR_Y as i32,
        UI_MUTED,
    );
}

fn draw_fighter(
    draw: &mut RaylibDrawHandle<'_>,
    fighter: &crate::combat::fighter::Fighter,
    body_color: Color,
) {
    fill_rect(draw, fighter.body_rect(), body_color);
    outline_rect(draw, fighter.hurtbox(), HURTBOX);

    if let Some(hitbox) = fighter.active_hitbox() {
        outline_rect(draw, hitbox, HITBOX);
    }

    let label_x = fighter.position.x as i32;
    let label_y = (fighter.position.y - 22.0) as i32;
    draw.draw_text(fighter.name, label_x, label_y, 16, UI_TEXT);
}

fn draw_hud(draw: &mut RaylibDrawHandle<'_>, world: &World) {
    draw.draw_text(
        "Borrow Fighters / Prototype 0.1 Greybox",
        24,
        18,
        20,
        UI_TEXT,
    );

    draw_health_bar(draw, 24, 50, world.player_one.health);
    draw_health_bar(draw, WINDOW_WIDTH - 324, 50, world.player_two.health);

    draw.draw_text("Rust", 24, 78, 18, PLAYER_ONE);
    draw.draw_text("Java", WINDOW_WIDTH - 324, 78, 18, PLAYER_TWO);

    if let Some(outcome) = world.outcome {
        let message = match outcome {
            MatchOutcome::Winner(PlayerSlot::One) => "Rust wins - press R",
            MatchOutcome::Winner(PlayerSlot::Two) => "Java wins - press R",
            MatchOutcome::Draw => "Draw - press R",
        };
        let width = draw.measure_text(message, 32);
        draw.draw_text(message, (WINDOW_WIDTH - width) / 2, 118, 32, UI_TEXT);
    }
}

fn draw_help(draw: &mut RaylibDrawHandle<'_>) {
    draw.draw_text(
        "P1: A/D move, W jump, F attack",
        24,
        WINDOW_HEIGHT - 58,
        18,
        UI_TEXT,
    );
    draw.draw_text(
        "P2: arrows move/jump, Enter attack",
        24,
        WINDOW_HEIGHT - 34,
        18,
        UI_TEXT,
    );
    draw.draw_text(
        "Green = hurtbox, red = active hitbox",
        WINDOW_WIDTH - 360,
        WINDOW_HEIGHT - 34,
        18,
        UI_MUTED,
    );
}

fn draw_health_bar(draw: &mut RaylibDrawHandle<'_>, x: i32, y: i32, health: i32) {
    let width = 300;
    let height = 18;
    let fill_width = (width as f32 * (health.max(0) as f32 / 100.0)).round() as i32;

    draw.draw_rectangle(x, y, width, height, HEALTH_BACK);
    draw.draw_rectangle(x, y, fill_width, height, HEALTH_FILL);
    draw.draw_rectangle_lines(x, y, width, height, UI_TEXT);
}

fn fill_rect(draw: &mut RaylibDrawHandle<'_>, rect: Rect, color: Color) {
    draw.draw_rectangle(
        rect.x.round() as i32,
        rect.y.round() as i32,
        rect.width.round() as i32,
        rect.height.round() as i32,
        color,
    );
}

fn outline_rect(draw: &mut RaylibDrawHandle<'_>, rect: Rect, color: Color) {
    draw.draw_rectangle_lines(
        rect.x.round() as i32,
        rect.y.round() as i32,
        rect.width.round() as i32,
        rect.height.round() as i32,
        color,
    );
}
