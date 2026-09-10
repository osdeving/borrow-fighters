//! Places Rust's waking poses against the bedroom's mattress and floor.
//!
//! System: Adventure presentation. Authored anatomical anchors keep the shoulder,
//! seated pelvis and planted boot supported while the original drawings change.

use raylib::prelude::*;

use super::assets::Assets;
use crate::adventure::story::Story;

/// Adds editable Rust references and quiet monitor/fan animation to the room.
pub fn draw_room_details(d: &mut impl RaylibDraw, story: &Story, a: &Assets) {
    let t = story.stage_ticks as f32 / 60.0;
    let orange = Color::new(230, 126, 65, 255);
    let cream = Color::new(244, 225, 188, 255);
    let mint = Color::new(130, 201, 185, 255);
    d.draw_text_ex(
        &a.body,
        a.text.get("morning.poster.rust"),
        Vector2::new(342.0, 207.0),
        27.0,
        2.0,
        orange,
    );
    super::typography::paragraph(
        d,
        &a.body,
        a.text.get("morning.poster.slogan"),
        Rectangle::new(495.0, 193.0, 126.0, 28.0),
        11.0,
        Color::new(69, 64, 49, 255),
    );
    d.draw_rectangle(569, 232, 120, 65, Color::new(20, 30, 34, 255));
    d.draw_text_ex(
        &a.body,
        a.text.get("morning.setup.command"),
        Vector2::new(575.0, 239.0),
        12.0,
        0.0,
        mint,
    );
    super::typography::paragraph(
        d,
        &a.body,
        a.text.get("morning.setup.status"),
        Rectangle::new(575.0, 257.0, 108.0, 31.0),
        8.0,
        cream,
    );
    if story.stage_ticks % 60 < 32 {
        d.draw_rectangle(575, 288, 6, 2, mint);
    }
    // Restrained light on the tower: fixed centers, rotating thin fan blades.
    for y in [417.0, 480.0] {
        let center = Vector2::new(859.0, y);
        for blade in 0..5 {
            let angle = t * 4.5 + blade as f32 * std::f32::consts::TAU / 5.0;
            d.draw_line_ex(
                Vector2::new(center.x + angle.cos() * 4.0, center.y + angle.sin() * 4.0),
                Vector2::new(
                    center.x + (angle + 0.25).cos() * 12.0,
                    center.y + (angle + 0.25).sin() * 12.0,
                ),
                1.2,
                Color::new(236, 151, 77, 85),
            );
        }
    }
}

/// Draws Rust waking on the bed and standing on the rug, without moving the room.
pub fn draw_morning_character(d: &mut impl RaylibDraw, story: &Story, a: &Assets) {
    let index = pose_index(story.stage_ticks);
    let source = a.morning_bounds[index];
    let placement = placement(index);
    let base_scale = 335.0 / a.morning_bounds[7].height;
    let scale = base_scale * placement.scale;
    let seconds = story.stage_ticks as f32 / 60.0;

    // Breathing expands around the point carrying weight. Translating the whole
    // cutout would lift the forearm, pelvis or sole away from its support.
    let breath = (seconds * 1.7).sin() * 0.0025;
    let scale_x = scale * (1.0 - breath * 0.25);
    let scale_y = scale * (1.0 + breath);

    draw_contact_shadow(d, index);
    d.draw_texture_pro(
        &a.morning,
        source,
        Rectangle::new(
            placement.world_anchor.x - placement.pose_anchor.x * scale_x,
            placement.world_anchor.y - placement.pose_anchor.y * scale_y,
            source.width * scale_x,
            source.height * scale_y,
        ),
        Vector2::zero(),
        0.0,
        Color::WHITE,
    );
}

struct Placement {
    pose_anchor: Vector2,
    world_anchor: Vector2,
    scale: f32,
}

fn pose_index(ticks: u32) -> usize {
    match ticks {
        0..=149 => 0,
        150..=224 => 1,
        225..=289 => 2,
        290..=354 => 3,
        355..=424 => 4,
        425..=499 => 5,
        500..=584 => 6,
        _ => 7,
    }
}

fn placement(index: usize) -> Placement {
    // Source anchors are pixels relative to the measured pose rectangles in
    // rust-morning-poses.json. Targets refer to the bedroom at 1280 x 720:
    // mattress surface around y=408, front seating edge y=426, rug y=552.
    // If the illustrations change, these contact landmarks must be reviewed.
    let (pose_anchor, world_anchor, scale) = match index {
        // The tucked hip and supporting forearm rest across the mattress; the
        // head sits over the pillow rather than above the hanging bed sheet.
        0 => ((220.0, 134.0), (408.0, 410.0), 1.0),
        1 => ((221.0, 133.0), (408.0, 410.0), 1.0),
        // The left forearm remains on the bed while the torso rises.
        2 => ((78.0, 207.0), (264.0, 409.0), 1.0),
        // This cross-legged drawing has a larger head; the modest correction
        // preserves its apparent size relative to the neighboring seated pose.
        3 => ((99.0, 241.0), (406.0, 416.0), 0.9),
        // Anchor the underside of the seated pelvis, not the dangling boots.
        4 => ((90.0, 199.0), (413.0, 426.0), 1.0),
        5 => ((132.0, 218.0), (413.0, 426.0), 1.0),
        // The lowered boot plants on the rug as the supporting hand comes down.
        6 => ((60.0, 254.0), (438.0, 552.0), 1.0),
        // Keep the same planted boot while Rust straightens his legs.
        _ => ((54.0, 337.0), (438.0, 552.0), 1.0),
    };
    Placement {
        pose_anchor: Vector2::new(pose_anchor.0, pose_anchor.1),
        world_anchor: Vector2::new(world_anchor.0, world_anchor.1),
        scale,
    }
}

fn draw_contact_shadow(d: &mut impl RaylibDraw, index: usize) {
    let shadow = Color::new(48, 35, 25, 35);
    match index {
        0 | 1 => d.draw_ellipse(351, 412, 145.0, 5.0, shadow),
        2 => d.draw_ellipse(381, 413, 120.0, 4.0, shadow),
        3 => d.draw_ellipse(406, 418, 58.0, 4.0, shadow),
        4 | 5 => d.draw_ellipse(414, 429, 34.0, 4.0, shadow),
        6 => {
            d.draw_ellipse(438, 554, 28.0, 4.0, shadow);
            d.draw_ellipse(566, 537, 22.0, 3.0, shadow);
        }
        _ => d.draw_ellipse(482, 555, 66.0, 5.0, shadow),
    }
}
