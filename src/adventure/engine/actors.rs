//! Draws the adventure's existing locomotion, combat and compassionate actor poses.
//!
//! System: Adventure actors. Prologue and chapter reuse the same body animation;
//! narrative attachments and world triggers remain outside this drawing module.

use super::assets::Assets;
use crate::adventure::combat::{Action, Actor, FLOOR_Y, Facing};
use raylib::prelude::*;
const INK: Color = Color::new(16, 23, 28, 255);
const GOLD: Color = Color::new(232, 177, 92, 255);
fn alpha(mut color: Color, opacity: f32) -> Color {
    color.a = (opacity.clamp(0.0, 1.0) * color.a as f32) as u8;
    color
}

pub fn actor_shadow(d: &mut impl RaylibDraw, actor: &Actor, camera: f32) {
    d.draw_ellipse(
        (actor.position.x - camera) as i32,
        FLOOR_Y as i32 + 2,
        37.0,
        7.0,
        alpha(INK, 0.22),
    );
}

pub fn rust(d: &mut impl RaylibDraw, a: &Assets, actor: &Actor, camera: f32) {
    rust_scaled(d, a, actor, camera, 1.0);
}

/// Draws existing locomotion at the depth selected by a chapter approach path.
pub fn rust_scaled(d: &mut impl RaylibDraw, a: &Assets, actor: &Actor, camera: f32, depth: f32) {
    let pos = Vector2::new(actor.position.x - camera, actor.position.y);
    if actor.action == Action::Remorse {
        let frame = match actor.action_ticks {
            0..=29 => 8,
            30..=59 => 9,
            60..=89 => 10,
            90..=119 => 9,
            _ => 11,
        };
        let tallest = a
            .morning_bounds
            .iter()
            .skip(6)
            .map(|r| r.height)
            .fold(1.0, f32::max);
        pose(
            d,
            &a.morning,
            a.morning_bounds[frame],
            pos,
            174.0 / tallest * depth,
            actor.facing == Facing::Left,
            Color::WHITE,
        );
        return;
    }
    let frame = match actor.action {
        Action::Walk => 2 + (actor.action_ticks / 7 % 4) as usize,
        Action::Jump if actor.velocity.y < 0.0 => 6,
        Action::Jump => 7,
        Action::LightAttack if actor.action_ticks < 5 => 8,
        Action::LightAttack if actor.action_ticks < 10 => 9,
        Action::LightAttack => 10,
        Action::HeavyAttack if actor.action_ticks < 13 => 11,
        Action::HeavyAttack if actor.action_ticks < 20 => 12,
        Action::Block => 13,
        Action::Hurt => 14,
        Action::Defeated => 15,
        _ => (actor.action_ticks / 30 % 2) as usize,
    };
    let tallest = a
        .action_bounds
        .iter()
        .take(15)
        .map(|r| r.height)
        .fold(1.0, f32::max);
    pose(
        d,
        &a.actions,
        a.action_bounds[frame],
        pos,
        174.0 / tallest * depth,
        actor.facing == Facing::Left,
        Color::WHITE,
    );
    if actor.action == Action::HeavyAttack && (13..20).contains(&actor.action_ticks) {
        let x = pos.x + actor.facing.sign() * 65.0;
        let y = pos.y - 115.0;
        let t = (actor.action_ticks - 13) as f32;
        d.draw_circle_lines(
            x as i32,
            y as i32,
            14.0 + t * 2.0,
            alpha(GOLD, 1.0 - t / 8.0),
        );
        for i in 0..4 {
            let angle = i as f32 * std::f32::consts::FRAC_PI_2 + t * 0.13;
            let p = Vector2::new(x + angle.cos() * 23.0, y + angle.sin() * 23.0);
            d.draw_rectangle(p.x as i32 - 2, p.y as i32 - 2, 4, 4, GOLD);
        }
    }
}

pub fn creature(d: &mut impl RaylibDraw, a: &Assets, actor: &Actor, camera: f32) {
    let frame = match actor.action {
        Action::Walk => 1 + (actor.action_ticks / 10 % 2) as usize,
        Action::Telegraph => 3,
        Action::Lunge => 4,
        Action::Hurt => 5,
        Action::Defeated if actor.action_ticks < 24 => 6,
        Action::Defeated => 7,
        _ => 0,
    };
    let height = a
        .erratic_bounds
        .iter()
        .take(6)
        .map(|r| r.height)
        .fold(1.0, f32::max);
    let tremor = if actor.hp > 0 {
        (actor.action_ticks as f32 * 0.7).sin() * 1.2
    } else {
        0.0
    };
    pose(
        d,
        &a.erratic,
        a.erratic_bounds[frame],
        Vector2::new(actor.position.x - camera + tremor, actor.position.y),
        187.0 / height,
        actor.facing == Facing::Left,
        Color::WHITE,
    );
}

fn pose(
    d: &mut impl RaylibDraw,
    texture: &Texture2D,
    source: Rectangle,
    feet: Vector2,
    scale: f32,
    flip: bool,
    tint: Color,
) {
    let width = source.width * scale;
    let height = source.height * scale;
    let src = Rectangle::new(
        source.x,
        source.y,
        if flip { -source.width } else { source.width },
        source.height,
    );
    d.draw_texture_pro(
        texture,
        src,
        Rectangle::new(feet.x, feet.y, width, height),
        Vector2::new(width * 0.5, height),
        0.0,
        tint,
    );
}
