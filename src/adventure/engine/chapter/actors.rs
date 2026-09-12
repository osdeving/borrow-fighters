//! Composes narrative body clips, independent residents and hand-held attachments.
//!
//! System: Adventure chapter actors. Locomotion stays shared with the prologue;
//! named sockets use the very same frame, pivot, mirror and scale as their body.

use super::{assets::ChapterAssets, phone, world};
use crate::adventure::{
    chapter::{Chapter, Phase, ResidentView, Scene},
    combat::Facing,
    engine::{actors, pieces::PiecePose},
};
use raylib::prelude::*;

pub(super) fn draw(d: &mut impl RaylibDraw, a: &ChapterAssets, chapter: &Chapter, debug: bool) {
    let mut people: Vec<_> = chapter
        .residents()
        .into_iter()
        .filter(|p| p.visible)
        .collect();
    people.sort_by(|a, b| a.position.y.total_cmp(&b.position.y));
    let mut rust_drawn = false;
    for person in people {
        if !rust_drawn && chapter.player().position.y < person.position.y {
            rust(d, a, chapter, debug);
            rust_drawn = true;
        }
        resident(d, a, chapter, person);
    }
    // The roll-down panel occludes its keeper, while Rust remains outside it.
    world::shutter(d, a, chapter);
    if !rust_drawn {
        rust(d, a, chapter, debug);
    }
    if chapter.scene == Scene::Passage {
        for enemy in chapter.combat.enemies() {
            actors::actor_shadow(d, enemy, 0.0);
            actors::creature(d, &a.common, enemy, 0.0);
        }
    }
}

fn resident(d: &mut impl RaylibDraw, a: &ChapterAssets, chapter: &Chapter, p: ResidentView) {
    let mut pose = PiecePose::at(Vector2::new(p.position.x, p.position.y));
    let ticks = p.action_ticks;
    let (id, custom) = match p.id {
        "driver" => {
            pose.scale = 0.65;
            (
                if p.talking {
                    "driver.talk"
                } else {
                    "driver.idle"
                },
                true,
            )
        }
        "shopkeeper" => (
            if chapter.phase == Phase::ShopReturn {
                "shopkeeper.pull"
            } else if p.talking {
                "shopkeeper.alert"
            } else {
                "shopkeeper.idle"
            },
            false,
        ),
        "neighbour" | "passage_resident_1" => (
            if p.walking {
                "resident.1.run"
            } else {
                "resident.1.idle"
            },
            false,
        ),
        _ => (
            if p.walking {
                "resident.0.run"
            } else {
                "resident.0.idle"
            },
            false,
        ),
    };
    if p.id != "driver" && p.id != "shopkeeper" {
        pose.flip = if p.walking {
            p.facing == Facing::Right
        } else {
            p.facing == Facing::Left
        };
        if chapter.scene != Scene::Street {
            pose.scale = 1.58;
        }
    }
    if p.talking {
        pose.rotation = (ticks as f32 * 0.085).sin() * 0.75;
    }
    d.draw_ellipse(
        p.position.x as i32,
        p.position.y as i32 + 1,
        20.0 * pose.scale,
        3.3 * pose.scale,
        Color::new(36, 42, 31, 45),
    );
    if p.id == "shopkeeper" {
        pose.rotation = 0.0;
        a.common
            .street
            .draw_clipped(d, id, ticks, &pose, world::door());
    } else if custom {
        a.pieces.draw(d, id, ticks, &pose);
    } else {
        a.common
            .street
            .draw(d, id, if p.walking { ticks / 2 } else { ticks }, &pose);
    }
}

fn rust(d: &mut impl RaylibDraw, a: &ChapterAssets, chapter: &Chapter, debug: bool) {
    let actor = chapter.player();
    let scale = chapter.player_scale();
    d.draw_ellipse(
        actor.position.x as i32,
        if actor.grounded {
            actor.position.y as i32 + 2
        } else {
            582
        },
        34.0 * scale,
        6.0 * scale,
        Color::new(16, 23, 28, 55),
    );
    let selected = if let Some(phone) = chapter.phone() {
        Some(phone::body_clip(phone))
    } else if let Some(line) = chapter
        .active_dialogue()
        .filter(|_| chapter.phase != Phase::Intro && chapter.phase != Phase::Departure)
    {
        Some((
            if chapter.phase == Phase::DriverDialogue && line.line_index == 0 {
                "rust.inspect"
            } else if line.speaker == "speaker.rust" {
                "rust.talk"
            } else {
                "rust.listen"
            },
            chapter.phase_ticks,
        ))
    } else {
        None
    };
    let Some((id, ticks)) = selected else {
        actors::rust_scaled(d, &a.common, actor, 0.0, scale);
        return;
    };
    let mut pose = PiecePose::at(Vector2::new(actor.position.x, actor.position.y));
    pose.scale = scale;
    pose.flip = actor.facing == Facing::Left;
    if chapter.phone().is_some_and(|p| p.device_visible)
        && let (Some(hand), Some(tip)) = (
            a.pieces.socket(id, ticks, &pose, "phone"),
            a.pieces.socket(id, ticks, &pose, "phone_tip"),
        )
    {
        let angle = (tip.y - hand.y).atan2(tip.x - hand.x).to_degrees() + 90.0;
        phone::handset(d, hand, scale, angle);
        if debug {
            d.draw_circle_lines(hand.x as i32, hand.y as i32, 5.0, Color::MAGENTA);
        }
    }
    a.pieces.draw(d, id, ticks, &pose);
}
