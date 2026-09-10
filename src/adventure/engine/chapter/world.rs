//! Draws chapter spaces from the same dimensions and solids used by gameplay.
//!
//! System: Adventure chapter scenery. The aftermath reuses the evacuated street;
//! later spaces reuse one empty background with independently placed props.

use super::assets::ChapterAssets;
use crate::adventure::{
    ambient::AmbientState,
    chapter::{Chapter, Scene},
    engine::{pieces::PiecePose, street, typography},
    neighborhood::{DOOR_CENTER_X, DOOR_HEIGHT, DOOR_WIDTH, NEIGHBOR_FLOOR_Y},
};
use raylib::prelude::*;

pub(super) fn draw(d: &mut impl RaylibDraw, a: &ChapterAssets, chapter: &Chapter, wake_tick: u32) {
    if chapter.scene == Scene::Street {
        street::background(d, &a.common, 0.0);
        street::draw_without_neighbours(
            d,
            &AmbientState::settled_after_reaction(wake_tick, 600 + chapter.ticks),
            &a.common,
            0.0,
        );
        d.draw_rectangle_rec(door(), Color::new(14, 24, 20, 160));
        return;
    }
    let geometry = chapter.world.scene(chapter.scene);
    d.draw_texture_pro(
        &a.lane,
        Rectangle::new(0.0, 0.0, a.lane.width as f32, a.lane.height as f32),
        Rectangle::new(0.0, 0.0, geometry.width, 720.0),
        Vector2::zero(),
        0.0,
        if chapter.scene == Scene::Passage {
            Color::new(228, 235, 225, 255)
        } else {
            Color::WHITE
        },
    );
    let mut corner = PiecePose::at(Vector2::new(
        if chapter.scene == Scene::Lane {
            355.0
        } else {
            1470.0
        },
        464.0,
    ));
    corner.scale = 1.45;
    a.common.street.draw(d, "prop.corner", 0, &corner);
    // Lettering is a separate, editable wall sign, with drawn arrow and bolts.
    let panel = Rectangle::new(1400.0, 358.0, 245.0, 42.0);
    d.draw_rectangle_rec(
        Rectangle::new(panel.x + 2.0, panel.y + 3.0, panel.width, panel.height),
        Color::new(31, 42, 31, 60),
    );
    d.draw_rectangle_rec(panel, Color::new(223, 213, 174, 255));
    d.draw_rectangle_lines_ex(panel, 2.0, Color::new(99, 111, 84, 255));
    typography::centered(
        d,
        &a.common.signage,
        if chapter.scene == Scene::Lane {
            "TRAVESSA DO SOL"
        } else {
            "PASSAGEM DA VILA"
        },
        Vector2::new(panel.x + 108.0, panel.y + 10.0),
        188.0,
        22.0,
        Color::new(43, 69, 60, 255),
    );
    let arrow = Vector2::new(panel.x + 220.0, panel.y + 21.0);
    for end in [
        Vector2::new(arrow.x - 16.0, arrow.y),
        Vector2::new(arrow.x - 6.0, arrow.y - 6.0),
        Vector2::new(arrow.x - 6.0, arrow.y + 6.0),
    ] {
        d.draw_line_ex(arrow, end, 2.0, Color::new(43, 69, 60, 255));
    }
    for x in [panel.x + 5.0, panel.x + panel.width - 5.0] {
        for y in [panel.y + 5.0, panel.y + panel.height - 5.0] {
            d.draw_circle_v(Vector2::new(x, y), 1.2, Color::new(78, 91, 70, 255));
        }
    }
    for solid in &geometry.obstacles {
        let mut pose = PiecePose::at(Vector2::new(
            solid.x + solid.width * 0.5,
            solid.y + solid.height,
        ));
        pose.scale = solid.width / 120.0;
        d.draw_ellipse(
            pose.position.x as i32,
            (pose.position.y + 2.0) as i32,
            solid.width * 0.55,
            6.0,
            Color::new(40, 39, 30, 55),
        );
        a.pieces.draw(d, "prop.crate", 0, &pose);
    }
}

pub(super) fn door() -> Rectangle {
    Rectangle::new(
        DOOR_CENTER_X - DOOR_WIDTH * 0.5,
        NEIGHBOR_FLOOR_Y - DOOR_HEIGHT,
        DOOR_WIDTH,
        DOOR_HEIGHT,
    )
}

pub(super) fn shutter(d: &mut impl RaylibDraw, a: &ChapterAssets, chapter: &Chapter) {
    if chapter.scene != Scene::Street {
        return;
    }
    let r = door();
    let bottom = r.y + r.height * chapter.shutter_progress();
    a.common.street.draw_clipped(
        d,
        "shop.shutter",
        0,
        &PiecePose::at(Vector2::new(DOOR_CENTER_X, bottom)),
        r,
    );
    d.draw_line_ex(
        Vector2::new(r.x, bottom),
        Vector2::new(r.x + r.width, bottom),
        1.8,
        Color::new(49, 48, 41, 230),
    );
}

pub(super) fn debug(d: &mut impl RaylibDraw, chapter: &Chapter) {
    let geometry = chapter.world.scene(chapter.scene);
    for point in &geometry.points {
        let r = &point.region;
        d.draw_rectangle_lines_ex(
            Rectangle::new(r.x, r.y, r.width, r.height),
            2.0,
            Color::YELLOW,
        );
        d.draw_text(&point.id, r.x as i32, r.y as i32 - 20, 16, Color::YELLOW);
        let mut previous = None;
        for node in &point.path {
            let p = Vector2::new(node.x, node.y);
            d.draw_circle_lines(p.x as i32, p.y as i32, 5.0, Color::MAGENTA);
            if let Some(last) = previous {
                d.draw_line_ex(last, p, 2.0, Color::MAGENTA);
            }
            previous = Some(p);
        }
    }
    for r in &geometry.obstacles {
        d.draw_rectangle_lines_ex(Rectangle::new(r.x, r.y, r.width, r.height), 2.0, Color::RED);
    }
    let r = &geometry.exit;
    d.draw_rectangle_lines_ex(
        Rectangle::new(r.x, r.y, r.width, r.height),
        2.0,
        Color::GREEN,
    );
    d.draw_line_ex(
        Vector2::new(geometry.walk_min, geometry.floor_y),
        Vector2::new(geometry.walk_max, geometry.floor_y),
        2.0,
        Color::SKYBLUE,
    );
    let body = chapter.player().hurtbox();
    d.draw_rectangle_lines_ex(
        Rectangle::new(body.x, body.y, body.width, body.height),
        2.0,
        Color::LIME,
    );
}
