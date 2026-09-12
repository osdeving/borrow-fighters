//! Draws chapter spaces from the same dimensions and solids used by gameplay.
//!
//! System: Adventure chapter scenery. The aftermath reuses the evacuated street;
//! later spaces reuse one empty background with independently placed props.

use super::assets::ChapterAssets;
use crate::adventure::{
    ambient::AmbientState,
    chapter::{Chapter, Scene},
    engine::{pieces::PiecePose, street},
    neighborhood::{DOOR_HEIGHT, DOOR_WIDTH, NEIGHBOR_FLOOR_Y},
};
use raylib::prelude::*;

pub(super) fn draw(d: &mut impl RaylibDraw, a: &ChapterAssets, chapter: &Chapter, wake_tick: u32) {
    let geometry = chapter.world.scene(chapter.scene);
    let id = match chapter.scene {
        Scene::Street => "street",
        Scene::Lane => "lane",
        Scene::Passage => "passage",
    };
    let camera = chapter.camera();
    crate::adventure::engine::landscape::draw(
        d,
        &a.common,
        id,
        geometry.width,
        crate::adventure::engine::landscape::View {
            left: camera.target.x - 640.0 / camera.zoom,
            width: 1280.0 / camera.zoom,
            origin: 0.0,
        },
    );
    if chapter.scene == Scene::Street {
        let hub = a.common.landscape.map.scene("street").hub_origin;
        street::draw_without_neighbours(
            d,
            &AmbientState::settled_after_reaction_in_bounds(
                wake_tick,
                600 + chapter.ticks,
                [-hub, geometry.width - hub],
            ),
            &a.common,
            -hub,
        );
        d.draw_rectangle_rec(door(chapter), Color::new(14, 24, 20, 160));
        return;
    }
    debris(d, a, chapter);
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

fn debris(d: &mut impl RaylibDraw, a: &ChapterAssets, chapter: &Chapter) {
    for prop in &chapter.world.scene(chapter.scene).loose_props {
        let mut pose = PiecePose::at(Vector2::new(prop.position.x, prop.position.y));
        pose.scale = prop.scale;
        pose.rotation = prop.rotation;
        a.pieces.draw(d, &prop.piece, 0, &pose);
    }
    for (index, piece) in chapter.debris.iter().enumerate() {
        let r = piece.rect();
        let age = piece
            .hit_tick
            .map(|tick| chapter.ticks.saturating_sub(tick));
        if piece.hp > 0 {
            let impact = age.filter(|age| *age < 12).map_or(0.0, |age| {
                (age as f32 * 1.6).sin() * (1.0 - age as f32 / 12.0) * 3.0
            });
            let mut pose = PiecePose::at(Vector2::new(r.center_x() + impact, r.bottom()));
            pose.scale = r.width / a.pieces.size(&piece.spec.piece, 0).x;
            pose.flip = index % 2 == 1;
            pose.rotation = piece.spec.rotation;
            pose.tint = if piece.hp < piece.spec.hp {
                Color::new(221, 202, 163, 255)
            } else {
                Color::WHITE
            };
            d.draw_ellipse(
                r.center_x() as i32,
                r.bottom() as i32 + 2,
                r.width * 0.48,
                4.0,
                Color::new(38, 34, 25, 65),
            );
            a.pieces.draw(d, &piece.spec.piece, 0, &pose);
            if piece.hp < piece.spec.hp {
                // Thin surface fractures follow this item's current gravity position.
                let center = Vector2::new(r.center_x() + impact, r.y + r.height * 0.42);
                for (dx, dy) in [(-22.0, -15.0), (8.0, 11.0), (28.0, 18.0), (-14.0, 20.0)] {
                    d.draw_line_ex(
                        center,
                        Vector2::new(center.x + dx, center.y + dy),
                        1.8,
                        Color::new(62, 43, 24, 220),
                    );
                }
            }
        } else {
            let elapsed = age.unwrap_or(90).min(90) as f32 / 60.0;
            for shard in 0..4 {
                let travel = elapsed.min(0.8);
                let spread = (shard as f32 - 1.5) * 20.0;
                let x = r.center_x()
                    + spread
                    + piece.hit_direction * travel * (35.0 + shard as f32 * 15.0);
                let y = (r.bottom() - 16.0 - elapsed * (135.0 + shard as f32 * 12.0)
                    + 420.0 * elapsed * elapsed)
                    .min(584.0 + shard as f32 * 2.0);
                let mut pose = PiecePose::at(Vector2::new(x, y));
                pose.scale = 0.62 + shard as f32 * 0.12;
                pose.rotation = if y < 583.0 {
                    elapsed * (160.0 + shard as f32 * 65.0)
                } else {
                    spread * 0.5
                };
                pose.tint = Color::new(181, 160, 126, 255);
                a.pieces.draw(d, &piece.spec.fragment, 0, &pose);
            }
        }
        if let Some(age) = age.filter(|age| *age < 36) {
            let t = age as f32 / 36.0;
            for puff in 0..5 {
                let side = puff as f32 - 2.0;
                let center = Vector2::new(
                    r.center_x() + side * (8.0 + t * 31.0),
                    r.bottom() - 16.0 - t * (18.0 + puff as f32 * 6.0),
                );
                d.draw_circle_v(
                    center,
                    (6.0 + t * 18.0) * (1.0 + (puff % 2) as f32 * 0.2),
                    Color::new(196, 178, 137, ((1.0 - t) * 105.0) as u8),
                );
            }
        }
    }
}

pub(super) fn door(chapter: &Chapter) -> Rectangle {
    let center = chapter
        .world
        .scene(Scene::Street)
        .poi("shop")
        .expect("validated shop")
        .position
        .x;
    Rectangle::new(
        center - DOOR_WIDTH * 0.5,
        NEIGHBOR_FLOOR_Y - DOOR_HEIGHT,
        DOOR_WIDTH,
        DOOR_HEIGHT,
    )
}

pub(super) fn shutter(d: &mut impl RaylibDraw, a: &ChapterAssets, chapter: &Chapter) {
    if chapter.scene != Scene::Street {
        return;
    }
    let r = door(chapter);
    let bottom = r.y + r.height * chapter.shutter_progress();
    a.common.street.draw_clipped(
        d,
        "shop.shutter",
        0,
        &PiecePose::at(Vector2::new(r.x + r.width * 0.5, bottom)),
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
    for piece in chapter.debris.iter().filter(|piece| piece.hp > 0) {
        let r = piece.rect();
        d.draw_rectangle_lines_ex(
            Rectangle::new(r.x, r.y, r.width, r.height),
            1.5,
            Color::ORANGE,
        );
    }

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
