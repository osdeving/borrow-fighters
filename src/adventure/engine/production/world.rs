//! Composes the Augusta map, cinematic staging and readable combat presentation.
//!
//! System: Adventure production renderer. Scene pieces share one physical frame
//! of reference; only distant scenery receives the authored parallax factor.

use super::{
    actors::{attachment, draw_actor},
    assets::ProductionAssets,
};
use crate::adventure::{
    augusta::{Chapter, Phase},
    production::{Action, Team},
};
use raylib::prelude::*;

const INK: Color = Color::new(10, 15, 25, 242);
const GOLD: Color = Color::new(247, 201, 112, 255);
const PAPER: Color = Color::new(235, 237, 235, 255);

fn text(
    d: &mut impl RaylibDraw,
    a: &ProductionAssets,
    copy: &str,
    x: f32,
    y: f32,
    size: f32,
    color: Color,
) {
    d.draw_text_ex(&a.font, copy, Vector2::new(x, y), size, 0.35, color);
}

fn wrapped(
    d: &mut impl RaylibDraw,
    a: &ProductionAssets,
    copy: &str,
    x: f32,
    y: f32,
    width: f32,
    size: f32,
) {
    let mut row = String::new();
    let mut yy = y;
    for word in copy.split_whitespace() {
        let next = if row.is_empty() {
            word.into()
        } else {
            format!("{row} {word}")
        };
        if a.font.measure_text(&next, size, 0.35).x > width && !row.is_empty() {
            text(d, a, &row, x, yy, size, PAPER);
            yy += size * 1.3;
            row = word.into();
        } else {
            row = next;
        }
    }
    if !row.is_empty() {
        text(d, a, &row, x, yy, size, PAPER);
    }
}

fn backdrop(d: &mut impl RaylibDraw, a: &ProductionAssets, camera: f32) {
    let bg = &a.art.pieces[&a.art.background];
    let offset = (camera - 640.0) * a.art.parallax;
    let start = (offset / bg.size[0]).floor() as i32;
    for i in start..=start + 2 {
        attachment(
            d,
            &a.textures[&bg.image],
            bg,
            Vector2::new(i as f32 * bg.size[0] - offset, 0.0),
            1.0,
            false,
            0.0,
            Color::WHITE,
        );
    }
    // A continuous masonry backing prevents sky slits between adjacent façades.
    d.draw_rectangle(0, 330, 1280, 220, Color::new(22, 26, 33, 255));
    for p in &a.world.pieces {
        if p.layer >= 20 {
            continue;
        }
        let sprite = &a.art.pieces[&p.piece];
        let x = p.position[0] - camera + 640.0;
        if x + sprite.size[0] * p.scale < 0.0 || x - sprite.size[0] * p.scale > 1280.0 {
            continue;
        }
        attachment(
            d,
            &a.textures[&sprite.image],
            sprite,
            Vector2::new(x, p.position[1]),
            p.scale,
            false,
            0.0,
            Color::WHITE,
        );
    }
    let ground = &a.art.pieces[&a.art.ground];
    let offset = camera - 640.0;
    let start = (offset / ground.size[0]).floor() as i32;
    for i in start..=start + 4 {
        let flip = a.art.mirror_ground_tiles && i.rem_euclid(2) == 1;
        // Reflection preserves the tile's occupied interval despite its pivot.
        // Adjacent tiles then share the same painted edge, including lighting.
        let pivot_shift = if flip {
            (ground.source[2] - 2.0 * ground.anchor[0]) * ground.size[0] / ground.source[2]
        } else {
            0.0
        };
        attachment(
            d,
            &a.textures[&ground.image],
            ground,
            Vector2::new(
                i as f32 * ground.size[0] - offset + pivot_shift,
                a.art.facade_baseline,
            ),
            1.0,
            flip,
            0.0,
            Color::WHITE,
        );
    }
    for sign in &a.art.signs {
        let x = sign.position[0] - camera + 640.0;
        if !(-400.0..=1400.0).contains(&x) {
            continue;
        }
        let c = Color::new(sign.color[0], sign.color[1], sign.color[2], 255);
        let width = a.font.measure_text(&sign.text, sign.size, 1.0).x;
        d.draw_text_ex(
            &a.font,
            &sign.text,
            Vector2::new(x - width / 2.0, sign.position[1]),
            sign.size,
            1.0,
            c,
        );
    }
}

fn shadow(d: &mut impl RaylibDraw, x: f32, y: f32, width: f32, height: f32) {
    d.draw_ellipse(x as i32, y as i32, width, height, Color::new(4, 8, 13, 115));
}

fn dust(d: &mut impl RaylibDraw, x: f32, y: f32, age: u32) {
    let t = age as f32 / 60.0;
    if t > 1.6 {
        return;
    }
    let alpha = ((1.0 - t / 1.6) * 170.0).max(0.0) as u8;
    for i in 0..12 {
        let sign = if i % 2 == 0 { -1.0 } else { 1.0 };
        let travel = sign * (20.0 + t * (60.0 + i as f32 * 10.0));
        d.draw_ellipse(
            (x + travel) as i32,
            (y - 8.0 - t * 20.0 - (i % 3) as f32 * 6.0) as i32,
            10.0 + t * 26.0,
            8.0 + t * 12.0,
            Color::new(146, 141, 128, alpha),
        );
    }
    d.draw_ellipse_lines(
        x as i32,
        y as i32,
        25.0 + t * 210.0,
        8.0 + t * 24.0,
        Color::new(103, 219, 229, alpha),
    );
}

/// Draws into a 1280×720 target, also used unchanged by the chapter review capture.
pub fn draw_chapter(d: &mut impl RaylibDraw, a: &ProductionAssets, chapter: &Chapter, debug: bool) {
    d.clear_background(Color::new(8, 14, 26, 255));
    let shake = chapter
        .landing_age()
        .filter(|age| *age < 9)
        .map_or(0.0, |age| {
            let direction = if age.is_multiple_of(2) { 1.0 } else { -1.0 };
            direction * 5.0 * (1.0 - age as f32 / 9.0)
        });
    let camera = chapter.camera_x() + shake;
    backdrop(d, a, camera);
    let sim = chapter.simulation();
    if let Some(age) = chapter.landing_age() {
        for x in chapter.world.erratic_landings_x {
            dust(d, x - camera + 640.0, chapter.world.ground_y, age);
        }
    }
    for npc in chapter.npcs() {
        let x = npc.position.x - camera + 640.0;
        let assets = &a.actors[npc.character];
        let pose = assets
            .clips
            .sample(npc.clip, npc.ticks as f32, npc.ticks as f32 * 3.0)
            .pose;
        shadow(d, x, npc.position.y, 27.0, 7.0);
        draw_actor(
            d,
            assets,
            pose,
            npc.clip,
            npc.ticks as f32,
            [x, npc.position.y],
            npc.facing,
            1.0,
            false,
        );
    }
    let states = a.animations.borrow();
    for actor in sim.actors() {
        let assets = &a.actors[&actor.character];
        let arrival = chapter.arrival_pose(actor.id);
        let (position, scale, clip, ticks) = if let Some(p) = arrival {
            (
                [p.position.x - camera + 640.0, p.position.y],
                p.scale,
                p.clip,
                p.ticks as f32,
            )
        } else {
            (
                [actor.position.x - camera + 640.0, actor.position.y],
                1.0,
                actor.clip_id(),
                actor.action_ticks as f32,
            )
        };
        let pose = if arrival.is_some() {
            assets.clips.sample(clip, ticks, 0.0).pose
        } else {
            states
                .get(&actor.id.0)
                .map(|s| s.pose)
                .unwrap_or_else(|| assets.clips.sample(clip, ticks, actor.stride_distance).pose)
        };
        let ground = sim.bounds.floor_y;
        shadow(d, position[0], ground, 30.0 * scale, 7.0 * scale);
        if arrival.is_some() && position[1] < ground - 30.0 {
            for i in 0..7 {
                let x = position[0] + (i as f32 - 3.0) * 9.0;
                d.draw_line_ex(
                    Vector2::new(x, position[1] - 120.0 - i as f32 * 8.0),
                    Vector2::new(x - 12.0, position[1] - 285.0 - i as f32 * 8.0),
                    2.0,
                    Color::new(102, 219, 233, 100),
                );
            }
        }
        draw_actor(
            d,
            assets,
            pose,
            clip,
            ticks,
            position,
            actor.facing,
            scale,
            debug,
        );
        if actor.action == Action::Parry {
            d.draw_circle_lines_v(
                Vector2::new(
                    position[0] + actor.facing.sign() * 38.0,
                    position[1] - 117.0,
                ),
                34.0,
                GOLD,
            );
        }
        if let Some(id) = &actor.move_id {
            let m = &sim.content.moves[id];
            if m.active_at(actor.action_ticks) {
                let center = Vector2::new(
                    position[0] + actor.facing.sign() * 46.0,
                    position[1] - 100.0,
                );
                d.draw_ring(
                    center,
                    42.0,
                    46.0,
                    if actor.facing.sign() > 0.0 {
                        -80.0
                    } else {
                        100.0
                    },
                    if actor.facing.sign() > 0.0 {
                        65.0
                    } else {
                        245.0
                    },
                    18,
                    Color::new(255, 210, 111, 175),
                );
            }
        }
        if actor.team == Team::Enemy && actor.hp > 0 && actor.active {
            d.draw_rectangle(
                (position[0] - 32.0) as i32,
                (position[1] - assets.rig.height - 20.0) as i32,
                64,
                5,
                INK,
            );
            d.draw_rectangle(
                (position[0] - 32.0) as i32,
                (position[1] - assets.rig.height - 20.0) as i32,
                (64.0 * actor.hp as f32 / actor.max_hp as f32) as i32,
                5,
                Color::new(112, 200, 210, 255),
            );
        }
        if debug {
            let c = &sim.content.characters[&actor.character];
            for r in &c.hurtboxes {
                let x = position[0]
                    + if actor.facing.sign() > 0.0 {
                        r[0]
                    } else {
                        -r[0] - r[2]
                    };
                d.draw_rectangle_lines_ex(
                    Rectangle::new(x, position[1] + r[1], r[2], r[3]),
                    1.0,
                    Color::SKYBLUE,
                );
            }
            if let Some(id) = &actor.move_id {
                for b in &sim.content.moves[id].hitboxes {
                    if (b.start..b.end).contains(&actor.action_ticks) {
                        let r = b.rect;
                        let x = position[0]
                            + if actor.facing.sign() > 0.0 {
                                r[0]
                            } else {
                                -r[0] - r[2]
                            };
                        d.draw_rectangle_lines_ex(
                            Rectangle::new(x, position[1] + r[1], r[2], r[3]),
                            2.0,
                            Color::RED,
                        );
                    }
                }
            }
        }
    }
    for projectile in sim.projectiles() {
        let p = [
            projectile.position.x - camera + 640.0,
            projectile.position.y,
        ];
        if let Some(owner) = sim.actor(projectile.owner) {
            super::actors::draw_projectile(
                d,
                &a.actors[&owner.character],
                &projectile.visual_id,
                p,
                projectile.facing,
                sim.ticks as f32,
            );
        }
    }
    for p in &a.world.pieces {
        if p.layer < 20 {
            continue;
        }
        let sprite = &a.art.pieces[&p.piece];
        let x = p.position[0] - camera + 640.0;
        if (-300.0..=1600.0).contains(&x) {
            attachment(
                d,
                &a.textures[&sprite.image],
                sprite,
                Vector2::new(x, p.position[1]),
                p.scale,
                false,
                0.0,
                Color::WHITE,
            );
        }
    }
    ui(d, a, chapter);
}

fn ui(d: &mut impl RaylibDraw, a: &ProductionAssets, chapter: &Chapter) {
    let player = chapter.simulation().player();
    d.draw_rectangle(0, 0, 1280, 76, Color::new(5, 11, 20, 220));
    text(d, a, "C++", 30.0, 15.0, 24.0, GOLD);
    text(
        d,
        a,
        a.texts.get(&a.spec.title_key),
        430.0,
        15.0,
        21.0,
        PAPER,
    );
    d.draw_rectangle(30, 49, 260, 8, Color::new(49, 51, 60, 255));
    d.draw_rectangle(
        30,
        49,
        (260.0 * player.hp as f32 / player.max_hp as f32) as i32,
        8,
        GOLD,
    );
    text(
        d,
        a,
        a.texts.get(chapter.objective_key()),
        430.0,
        43.0,
        17.0,
        Color::new(174, 191, 206, 255),
    );
    if chapter.defeated() {
        d.draw_rectangle(280, 228, 720, 228, INK);
        text(d, a, a.texts.get("defeat.title"), 320.0, 258.0, 34.0, GOLD);
        wrapped(d, a, a.texts.get("defeat"), 320.0, 323.0, 640.0, 24.0);
    } else if let Some((id, index)) = chapter.dialogue() {
        let line = &a.texts.dialogues[id][index];
        d.draw_rectangle(42, 564, 1196, 128, INK);
        d.draw_rectangle(42, 564, 4, 128, GOLD);
        text(
            d,
            a,
            &a.texts.speakers[&line.speaker].name,
            64.0,
            577.0,
            21.0,
            GOLD,
        );
        crate::adventure::engine::typography::paragraph(
            d,
            &a.font,
            &line.text,
            Rectangle::new(64.0, 607.0, 1120.0, 57.0),
            21.0,
            PAPER,
        );
        text(
            d,
            a,
            a.texts.get("dialogue.next"),
            965.0,
            669.0,
            14.0,
            Color::new(151, 165, 178, 255),
        );
    } else if chapter.complete() {
        d.draw_rectangle(280, 220, 720, 286, INK);
        crate::adventure::engine::typography::paragraph(
            d,
            &a.font,
            a.texts.get("chapter.complete_title"),
            Rectangle::new(320.0, 250.0, 640.0, 82.0),
            34.0,
            GOLD,
        );
        crate::adventure::engine::typography::paragraph(
            d,
            &a.font,
            a.texts.get("chapter.complete"),
            Rectangle::new(320.0, 342.0, 640.0, 100.0),
            24.0,
            PAPER,
        );
        text(
            d,
            a,
            a.texts.get("chapter.return"),
            320.0,
            461.0,
            19.0,
            Color::new(185, 194, 206, 255),
        );
    } else {
        d.draw_rectangle(0, 680, 1280, 40, Color::new(4, 9, 16, 235));
        text(
            d,
            a,
            a.texts.get("controls"),
            26.0,
            690.0,
            16.0,
            Color::new(185, 194, 206, 255),
        );
    }
    if chapter.phase() == Phase::Introduction {
        let duration = chapter.spec.timing.introduction_ticks as f32;
        let alpha = (1.0 - chapter.phase_ticks() as f32 / duration).clamp(0.0, 1.0);
        d.draw_rectangle(
            0,
            80,
            1280,
            170,
            Color::new(7, 14, 25, (190.0 * alpha) as u8),
        );
        text(
            d,
            a,
            a.texts.get("arrival.title"),
            48.0,
            111.0,
            44.0,
            Color::new(242, 222, 181, (255.0 * alpha) as u8),
        );
    }
}
