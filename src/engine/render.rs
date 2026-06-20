//! Draws the greybox prototype.
//!
//! Rendering intentionally uses primitive shapes and debug overlays so gameplay
//! problems are visible before art production starts.

use raylib::prelude::*;

use crate::combat::fighter::{AttackPhase, PlayerSlot};
use crate::config::{ARENA_LEFT, ARENA_RIGHT, FLOOR_Y, WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::game::world::{MatchOutcome, World};
use crate::math::rect::Rect;

const BACKGROUND: Color = Color::new(18, 20, 26, 255);
const FLOOR: Color = Color::new(72, 76, 88, 255);
const PLAYER_ONE: Color = Color::new(112, 181, 255, 255);
const PLAYER_TWO: Color = Color::new(255, 178, 104, 255);
const BODY_OUTLINE: Color = Color::new(238, 241, 247, 255);
const HURTBOX: Color = Color::new(105, 240, 174, 255);
const HITBOX: Color = Color::new(255, 82, 82, 255);
const HITBOX_FILL: Color = Color::new(255, 82, 82, 82);
const HITSPARK: Color = Color::new(255, 235, 59, 255);
const BODY_COLLISION: Color = Color::new(218, 112, 214, 255);
const UI_TEXT: Color = Color::new(238, 241, 247, 255);
const UI_MUTED: Color = Color::new(165, 172, 185, 255);
const HEALTH_BACK: Color = Color::new(60, 62, 70, 255);
const HEALTH_FILL: Color = Color::new(76, 217, 100, 255);
const HEALTH_DANGER: Color = Color::new(255, 82, 82, 255);

/// Draws the current world state.
pub fn draw(draw: &mut RaylibDrawHandle<'_>, world: &World) {
    draw.clear_background(BACKGROUND);
    draw_arena(draw);
    draw_fighter(draw, &world.player_one, PLAYER_ONE);
    draw_fighter(draw, &world.player_two, PLAYER_TWO);
    draw_body_collision(draw, world);
    draw_hit_effects(draw, world);
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
    let phase = fighter.attack_phase();
    let body = match phase {
        AttackPhase::Idle => body_color,
        AttackPhase::Startup => lighten(body_color, 30),
        AttackPhase::Active => Color::new(255, 222, 89, 255),
        AttackPhase::Recovery => dim(body_color, 25),
    };

    fill_rect(draw, fighter.body_rect(), body);
    outline_rect(draw, fighter.body_rect(), BODY_OUTLINE);
    outline_rect(draw, fighter.hurtbox(), HURTBOX);

    if phase != AttackPhase::Idle {
        let attack_box = fighter.attack_box();
        draw.draw_rectangle(
            attack_box.x.round() as i32,
            attack_box.y.round() as i32,
            attack_box.width.round() as i32,
            attack_box.height.round() as i32,
            if phase == AttackPhase::Active {
                HITBOX_FILL
            } else {
                Color::new(255, 82, 82, 34)
            },
        );
        outline_rect(draw, attack_box, HITBOX);
    }

    if let Some(hitbox) = fighter.active_hitbox() {
        draw.draw_text(
            "ACTIVE",
            hitbox.x as i32,
            (hitbox.y - 22.0) as i32,
            16,
            HITBOX,
        );
    }

    let label_x = fighter.position.x as i32;
    let label_y = (fighter.position.y - 22.0) as i32;
    draw.draw_text(fighter.name, label_x, label_y, 16, UI_TEXT);

    if phase != AttackPhase::Idle {
        let text = match phase {
            AttackPhase::Startup => "PUNCH",
            AttackPhase::Active => "HITBOX",
            AttackPhase::Recovery => "RECOVER",
            AttackPhase::Idle => "",
        };
        draw.draw_text(text, label_x, label_y - 22, 18, HITSPARK);
    }
}

fn draw_hud(draw: &mut RaylibDrawHandle<'_>, world: &World) {
    draw.draw_text(
        "Borrow Fighters / Prototype 0.1 Greybox",
        24,
        18,
        20,
        UI_TEXT,
    );

    draw_health_bar(draw, 24, 50, world.player_one.health, "Rust");
    draw_health_bar(
        draw,
        WINDOW_WIDTH - 324,
        50,
        world.player_two.health,
        "Java",
    );

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
        "White = body, green = hurtbox, red = punch, yellow = hit, magenta = body block",
        WINDOW_WIDTH - 690,
        WINDOW_HEIGHT - 34,
        18,
        UI_MUTED,
    );
}

fn draw_health_bar(draw: &mut RaylibDrawHandle<'_>, x: i32, y: i32, health: i32, label: &str) {
    let width = 300;
    let height = 18;
    let fill_width = (width as f32 * (health.max(0) as f32 / 100.0)).round() as i32;
    let fill = if health <= 24 {
        HEALTH_DANGER
    } else {
        HEALTH_FILL
    };

    draw.draw_rectangle(x, y, width, height, HEALTH_BACK);
    draw.draw_rectangle(x, y, fill_width, height, fill);
    draw.draw_rectangle_lines(x, y, width, height, UI_TEXT);

    let text = format!("{label} HP {health:03}");
    draw.draw_text(&text, x, y - 24, 20, UI_TEXT);
}

fn draw_body_collision(draw: &mut RaylibDrawHandle<'_>, world: &World) {
    if world.body_collision_timer <= 0.0 {
        return;
    }

    let p1 = world.player_one.body_rect();
    let p2 = world.player_two.body_rect();
    let left_right = p1.right().min(p2.right());
    let right_left = p1.x.max(p2.x);
    let x = ((left_right + right_left) * 0.5).round() as i32;
    let top = p1.y.max(p2.y).round() as i32;
    let bottom = p1.bottom().min(p2.bottom()).round() as i32;

    draw.draw_line_ex(
        Vector2::new(x as f32, top as f32),
        Vector2::new(x as f32, bottom as f32),
        6.0,
        BODY_COLLISION,
    );
    draw.draw_text("BODY COLLISION", x - 76, top - 24, 18, BODY_COLLISION);
}

fn draw_hit_effects(draw: &mut RaylibDrawHandle<'_>, world: &World) {
    for effect in &world.hit_effects {
        let x = effect.position.x.round() as i32;
        let y = effect.position.y.round() as i32;
        let radius = (10.0 + effect.timer * 32.0).round() as i32;
        draw.draw_circle_lines(x, y, radius as f32, HITSPARK);
        draw.draw_line(x - radius, y, x + radius, y, HITSPARK);
        draw.draw_line(x, y - radius, x, y + radius, HITSPARK);

        let damage = format!("-{}", effect.damage);
        draw.draw_text(&damage, x + 14, y - 18, 24, HITSPARK);
        draw.draw_text("HIT", x - 18, y - 42, 20, HITSPARK);
    }
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

fn lighten(color: Color, amount: u8) -> Color {
    Color::new(
        color.r.saturating_add(amount),
        color.g.saturating_add(amount),
        color.b.saturating_add(amount),
        color.a,
    )
}

fn dim(color: Color, amount: u8) -> Color {
    Color::new(
        color.r.saturating_sub(amount),
        color.g.saturating_sub(amount),
        color.b.saturating_sub(amount),
        color.a,
    )
}
