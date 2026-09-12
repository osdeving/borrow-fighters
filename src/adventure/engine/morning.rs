//! Places Rust's waking poses against the bedroom's mattress and floor.
//!
//! System: Adventure presentation. Authored anatomical anchors keep the shoulder,
//! seated pelvis and planted boot supported while the original drawings change.

use raylib::prelude::*;

use super::assets::Assets;
use crate::{
    adventure::{
        combat::{Action, Facing},
        story::Story,
    },
    math::vec2::Vec2,
};

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
    if let Some(([x, y], distance)) = a.locomotion.waking.exit(story.stage_ticks) {
        let mut actor = story.combat.player.clone();
        actor.action = Action::Walk;
        actor.position = Vec2::new(x, y);
        actor.facing = Facing::Right;
        let depth = a.locomotion.waking.exit_depth;
        actor.stride_distance = distance / depth;
        d.draw_ellipse(
            x as i32,
            y as i32 + 2,
            34.0 * depth,
            4.0 * depth,
            Color::new(48, 35, 25, 35),
        );
        a.locomotion.draw(d, &actor, 0.0, depth);
        return;
    }
    let (placement, rotation) = a.locomotion.waking.pose(story.stage_ticks);
    let index = placement.source;
    let source = a.morning_bounds[index];
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
            placement.support[0],
            placement.support[1],
            source.width * scale_x,
            source.height * scale_y,
        ),
        Vector2::new(placement.anchor[0] * scale_x, placement.anchor[1] * scale_y),
        rotation,
        Color::WHITE,
    );
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
