//! Draws the street's separated sidewalks, cycling lane and escaping kite.
//!
//! System: Adventure presentation. Every sprite follows the background camera
//! and a fixed story clock; none creates a physical body or combat contact.

use raylib::prelude::*;

use super::assets::Assets;
use crate::adventure::{
    ambient::{AmbientState, KID_FLOOR_Y, KID_ORIGIN_X, KidPhase},
    combat::Facing,
};

const PARALLAX: f32 = 0.48;
const PAPER: Color = Color::new(243, 226, 189, 255);
const INK: Color = Color::new(46, 61, 56, 255);

/// Aligns the painted sidewalk/lane/planter above the existing playable floor.
pub fn background(d: &mut impl RaylibDraw, a: &Assets, camera: f32) {
    let source_height = a.environments.height() as f32 * 0.5;
    let source_width = a.environments.width() as f32;
    let x = -camera * PARALLAX;
    let width = 1280.0 * 1.35;
    // Raise the neighborhood and its sidewalk to make room for a motor lane.
    // Preserve the original cycling lane, planter and all playable-floor pixels.
    for (from, height, to, drawn_height) in [(0.0, 438.0, 0.0, 368.0), (438.0, 182.0, 438.0, 182.0)]
    {
        d.draw_texture_pro(
            &a.environments,
            Rectangle::new(
                0.0,
                source_height + from / 620.0 * source_height,
                source_width,
                height / 620.0 * source_height,
            ),
            Rectangle::new(x, to, width, drawn_height),
            Vector2::zero(),
            0.0,
            Color::WHITE,
        );
    }
    super::traffic::road(d, camera * PARALLAX);
    // Continue only the foreground paving under the lower cinematic gradient.
    // Rust remains on y580; the cycling lane and planter sit entirely behind him.
    d.draw_texture_pro(
        &a.environments,
        Rectangle::new(0.0, source_height * 2.0 - 55.0, source_width, 55.0),
        Rectangle::new(x, 620.0, width, 100.0),
        Vector2::zero(),
        0.0,
        Color::WHITE,
    );
}

/// Composes scenery behind Rust and the erratic, with separated feet baselines.
pub fn draw(d: &mut impl RaylibDraw, ambient: &AmbientState, a: &Assets, camera: f32) {
    let offset = camera * PARALLAX;
    cycle_lane(d, offset);
    kite(d, ambient, offset);
    child(d, ambient, a, offset);
    super::traffic::draw(d, ambient, a, offset);

    // Far lane first: cyclists never share the child's sidewalk or Rust's plane.
    for cyclist in ambient.cyclists().iter().rev() {
        let frame = (cyclist.animation_ticks / 7 % 4) as usize;
        let feet = Vector2::new(cyclist.position.x - offset, cyclist.position.y);
        let near = cyclist.facing == Facing::Right;
        let scale = if near { 100.0 } else { 92.0 } / a.street_life_bounds[0].height;
        let tint = if near {
            Color::new(241, 228, 209, 255)
        } else {
            Color::new(204, 215, 207, 255)
        };
        d.draw_ellipse(
            feet.x as i32,
            feet.y as i32 + 1,
            54.0,
            3.0,
            Color::new(48, 43, 34, 35),
        );
        sprite(d, a, frame, feet, scale, !near, tint);
        wheel_spokes(
            d,
            a.street_life_bounds[frame],
            feet,
            scale,
            cyclist.animation_ticks,
            !near,
        );
    }
}

fn wheel_spokes(
    d: &mut impl RaylibDraw,
    bounds: Rectangle,
    feet: Vector2,
    scale: f32,
    ticks: u32,
    flip: bool,
) {
    // Measured wheel landmarks in the cycling row; only thin spokes rotate.
    let radius = 67.0 * scale;
    let rotation = ticks as f32 * 2.1 / radius * if flip { -1.0 } else { 1.0 };
    for fraction in [0.207, 0.798] {
        let local_x = (fraction - 0.5) * bounds.width * scale;
        let center = Vector2::new(
            feet.x + local_x * if flip { -1.0 } else { 1.0 },
            feet.y - 72.0 * scale,
        );
        for spoke in 0..4 {
            let angle = rotation + spoke as f32 * std::f32::consts::FRAC_PI_2;
            d.draw_line_ex(
                center,
                Vector2::new(
                    center.x + angle.cos() * radius * 0.88,
                    center.y + angle.sin() * radius * 0.88,
                ),
                0.7,
                Color::new(228, 220, 195, 120),
            );
        }
    }
}

fn cycle_lane(d: &mut impl RaylibDraw, offset: f32) {
    // These small markings reinforce the painted parallel lane, well above
    // the playable floor at y580. The curb belongs entirely to the background.
    d.draw_rectangle(0, 440, 1280, 46, Color::new(150, 79, 63, 32));
    d.draw_line_ex(
        Vector2::new(0.0, 441.0),
        Vector2::new(1280.0, 441.0),
        2.0,
        Color::new(245, 224, 184, 165),
    );
    d.draw_line_ex(
        Vector2::new(0.0, 488.0),
        Vector2::new(1280.0, 488.0),
        3.0,
        Color::new(245, 224, 184, 185),
    );
    for i in -1..10 {
        let x = i as f32 * 182.0 - offset.rem_euclid(182.0);
        d.draw_line_ex(
            Vector2::new(x, 463.0),
            Vector2::new(x + 51.0, 463.0),
            2.0,
            Color::new(245, 224, 184, 150),
        );
    }
}

fn child(d: &mut impl RaylibDraw, ambient: &AmbientState, a: &Assets, offset: f32) {
    let frame = match ambient.kid_phase() {
        KidPhase::Playing => 4 + (ambient.kid_phase_ticks() / 38 % 2) as usize,
        KidPhase::Startled => 6,
        KidPhase::Releasing => 7,
        KidPhase::Running => 8 + (ambient.kid_phase_ticks() / 6 % 4) as usize,
        KidPhase::Gone => return,
    };
    let position = ambient.kid_position();
    let feet = Vector2::new(position.x - offset, position.y);
    let scale = 86.0 / a.street_life_bounds[4].height;
    d.draw_ellipse(
        feet.x as i32,
        feet.y as i32 + 1,
        17.0,
        2.5,
        Color::new(48, 43, 34, 50),
    );
    sprite(
        d,
        a,
        frame,
        feet,
        scale,
        false,
        Color::new(247, 236, 217, 255),
    );

    if ambient.kid_phase() == KidPhase::Startled {
        // A short hand-drawn attention gesture, local to the child's silhouette.
        for i in 0..3 {
            let x = feet.x + 14.0 + i as f32 * 6.0;
            let y = feet.y - 85.0 + i as f32 * 4.0;
            d.draw_line_ex(
                Vector2::new(x, y),
                Vector2::new(x + 3.0, y - 9.0),
                2.0,
                PAPER,
            );
        }
    }
}

fn kite(d: &mut impl RaylibDraw, ambient: &AmbientState, offset: f32) {
    let t = ambient.ticks() as f32 / 60.0;
    // The free kite begins at the exact last tethered position. Using release
    // age changes its trajectory without snapping or following the fleeing boy.
    let free = ambient
        .kite_release_ticks()
        .map(|ticks| ticks as f32 / 60.0);
    let flight = free.unwrap_or(0.0);
    let center = Vector2::new(
        KID_ORIGIN_X + 108.0 + (t * 1.25).sin() * 13.0 + flight * 46.0 - offset,
        KID_FLOOR_Y - 223.0 + (t * 1.7).sin() * 8.0 - flight * 37.0,
    );
    if center.y < -160.0 {
        return;
    }
    let angle = (t * 1.8).sin() * 0.13 + flight.min(3.0) * 0.08;
    let local = |x: f32, y: f32| {
        Vector2::new(
            center.x + x * angle.cos() - y * angle.sin(),
            center.y + x * angle.sin() + y * angle.cos(),
        )
    };
    let top = local(0.0, -27.0);
    let left = local(-21.0, 0.0);
    let right = local(21.0, 0.0);
    let bottom = local(0.0, 33.0);
    d.draw_triangle(top, left, bottom, Color::new(230, 126, 67, 255));
    d.draw_triangle(top, bottom, right, Color::new(60, 149, 150, 255));
    for (start, end) in [(top, left), (left, bottom), (bottom, right), (right, top)] {
        d.draw_line_ex(start, end, 1.2, INK);
    }
    d.draw_line_ex(top, bottom, 1.0, PAPER);
    d.draw_line_ex(left, right, 1.0, PAPER);

    let mut last = bottom;
    for i in 1..=15 {
        let n = i as f32;
        let point = Vector2::new(
            bottom.x + (t * 4.0 - n * 0.53).sin() * n * 0.95,
            bottom.y + n * 4.3,
        );
        d.draw_line_ex(last, point, 1.1, PAPER);
        if i % 3 == 0 {
            d.draw_line_ex(
                Vector2::new(point.x - 3.5, point.y - 2.0),
                Vector2::new(point.x + 3.5, point.y + 2.0),
                2.5,
                Color::new(227, 124, 68, 255),
            );
        }
        last = point;
    }

    let start = if free.is_none() {
        let (hand_x, hand_y) = if ambient.kid_phase() == KidPhase::Startled {
            (9.0, -59.0)
        } else if (ambient.kid_phase_ticks() / 38).is_multiple_of(2) {
            (21.0, -81.0)
        } else {
            (26.0, -75.0)
        };
        Vector2::new(KID_ORIGIN_X + hand_x - offset, KID_FLOOR_Y + hand_y)
    } else {
        Vector2::new(center.x - 35.0 - flight * 8.0, center.y + 105.0)
    };
    let control = Vector2::new(
        (start.x + center.x) * 0.5 - 19.0 + (t * 2.0).sin() * 7.0,
        (start.y + center.y) * 0.5 + 16.0,
    );
    let mut previous = start;
    for i in 1..=20 {
        let u = i as f32 / 20.0;
        let v = 1.0 - u;
        let point = Vector2::new(
            v * v * start.x + 2.0 * v * u * control.x + u * u * center.x,
            v * v * start.y + 2.0 * v * u * control.y + u * u * center.y,
        );
        d.draw_line_ex(previous, point, 1.0, Color::new(240, 224, 190, 195));
        previous = point;
    }
}

fn sprite(
    d: &mut impl RaylibDraw,
    a: &Assets,
    frame: usize,
    feet: Vector2,
    scale: f32,
    flip: bool,
    tint: Color,
) {
    let bounds = a.street_life_bounds[frame];
    let width = bounds.width * scale;
    let height = bounds.height * scale;
    d.draw_texture_pro(
        &a.street_life,
        Rectangle::new(
            bounds.x,
            bounds.y,
            if flip { -bounds.width } else { bounds.width },
            bounds.height,
        ),
        Rectangle::new(feet.x, feet.y, width, height),
        Vector2::new(width * 0.5, height),
        0.0,
        tint,
    );
}
