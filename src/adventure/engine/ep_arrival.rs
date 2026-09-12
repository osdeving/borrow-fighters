//! Draws the EP's independent airborne body, grounded pose and impact particles.
//!
//! System: Adventure presentation. The domain timeline supplies every position
//! and event age; this adapter reuses the erratic sprite without baking the street.

use super::assets::Assets;
use crate::adventure::{
    arrival::{EpArrival, EpArrivalSample},
    combat::{Actor, FLOOR_Y, Facing},
};
use raylib::prelude::*;

fn tint(mut color: Color, opacity: f32) -> Color {
    color.a = (255.0 * opacity.clamp(0.0, 1.0)) as u8;
    color
}

/// Draws the landing body with a stable foot pivot and independent contact pose.
pub fn body(
    d: &mut impl RaylibDraw,
    assets: &Assets,
    actor: &Actor,
    camera: f32,
    sample: EpArrivalSample,
) {
    let x = actor.position.x - camera;
    let approach = (1.0 - (FLOOR_Y - sample.feet_y) / 600.0).clamp(0.0, 1.0);
    d.draw_ellipse(
        x as i32,
        FLOOR_Y as i32 + 2,
        17.0 + approach * 25.0,
        3.0 + approach * 5.0,
        tint(Color::new(16, 23, 28, 255), 0.08 + approach * 0.22),
    );
    let source = assets.erratic_bounds[sample.pose];
    let height = assets
        .erratic_bounds
        .iter()
        .take(6)
        .map(|bounds| bounds.height)
        .fold(1.0, f32::max);
    let scale = 187.0 / height;
    let width = source.width * scale * sample.scale.x;
    let height = source.height * scale * sample.scale.y;
    let flip = actor.facing == Facing::Left;
    d.draw_texture_pro(
        &assets.erratic,
        Rectangle::new(
            source.x,
            source.y,
            if flip { -source.width } else { source.width },
            source.height,
        ),
        Rectangle::new(x, sample.feet_y, width, height),
        Vector2::new(width * 0.5, height),
        0.0,
        Color::WHITE,
    );

    if sample.impact_age.is_none() {
        // Small air streaks follow this actor, never the camera or background.
        let speed = (sample.feet_y / FLOOR_Y).clamp(0.0, 1.0);
        for i in 0..4 {
            let side = if i % 2 == 0 { -1.0 } else { 1.0 };
            let p = Vector2::new(
                x + side * (32.0 + (i / 2) as f32 * 14.0),
                sample.feet_y - 95.0 - i as f32 * 12.0,
            );
            d.draw_line_ex(
                p,
                Vector2::new(p.x + side * 7.0, p.y - 12.0 - speed * 26.0),
                1.4,
                tint(Color::new(163, 216, 207, 255), 0.08 + speed * 0.2),
            );
        }
    }
}

/// Draws configurable dust/fragments independently from the body sprite.
pub fn impact(d: &mut impl RaylibDraw, arrival: &EpArrival, x: f32) {
    let Some(age) = arrival
        .ticks()
        .and_then(|ticks| ticks.checked_sub(arrival.spec.impact_tick()))
    else {
        return;
    };
    if age >= arrival.spec.dust_ticks {
        return;
    }
    let t = age as f32 / arrival.spec.dust_ticks as f32;
    let spread = 1.0 - (1.0 - t).powi(3);
    let fade = (1.0 - t).powi(2);
    let radius = arrival.spec.dust_radius;
    if age < 24 {
        let ring = age as f32 / 24.0;
        d.draw_ellipse_lines(
            x as i32,
            FLOOR_Y as i32 + 3,
            22.0 + ring * radius,
            4.0 + ring * 15.0,
            tint(Color::new(231, 203, 159, 255), (1.0 - ring) * 0.65),
        );
    }
    for i in 0..22 {
        let seed = i as f32;
        let side = if i % 2 == 0 { -1.0 } else { 1.0 };
        let variation = 0.45 + (seed * 1.73).sin().abs() * 0.55;
        let px = x + side * (8.0 + spread * radius * variation);
        let py = FLOOR_Y + 5.0 - (t * std::f32::consts::PI).sin() * (10.0 + variation * 35.0);
        let size = (5.0 + variation * 12.0) * (0.7 + spread * 0.9);
        d.draw_ellipse(
            px as i32,
            py as i32,
            size * 1.65,
            size * 0.66,
            tint(Color::new(188, 167, 137, 255), fade * 0.25),
        );
        d.draw_ellipse(
            (px - side * 4.0) as i32,
            (py - 3.0) as i32,
            size,
            size * 0.56,
            tint(Color::new(216, 197, 166, 255), fade * 0.18),
        );
    }
    for i in 0..12 {
        let f = i as f32;
        let side = if i % 2 == 0 { -1.0 } else { 1.0 };
        let seconds = age as f32 / 60.0;
        let height = (seconds * (75.0 + f * 5.0) - 150.0 * seconds * seconds).max(0.0);
        let px = x + side * (10.0 + seconds.min(0.95) * (28.0 + f * 7.0));
        let py = FLOOR_Y - height;
        d.draw_rectangle_pro(
            Rectangle::new(px, py, 3.0 + (i % 3) as f32, 2.0),
            Vector2::new(1.5, 1.0),
            seconds * (90.0 + f * 13.0),
            tint(Color::new(92, 83, 73, 255), fade * 0.85),
        );
    }
}
