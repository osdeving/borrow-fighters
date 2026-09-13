//! Places the imported Julia/broker rigs on their existing restraint contact.
//!
//! System: Augusta presentation. One wrist target drives both real arm chains;
//! camera choice changes projection only, without introducing a second timeline.

use super::{
    assets::ProductionAssets,
    models3d::{ArmContact, DrawModel, actor_sample},
};
use crate::adventure::augusta::Chapter;
use raylib::prelude::*;

fn samples<'a>(a: &'a ProductionAssets, c: &Chapter, camera: Option<f32>) -> Vec<DrawModel<'a>> {
    let Some(contact) = c.restraint_contact() else {
        return vec![];
    };
    if !a.models.as_ref().is_some_and(|models| models.has_pair()) {
        return vec![];
    }
    c.npcs()
        .into_iter()
        .filter(|npc| npc.visible)
        .map(|npc| {
            let broker = npc.character == "broker";
            let screen = camera.is_some();
            let x_offset = camera.map_or(0., |x| 640. - x);
            let y = |value: f32| {
                if screen {
                    value
                } else {
                    c.world.ground_y - value
                }
            };
            let assets = &a.actors[npc.character];
            let at = [
                npc.position.x + x_offset,
                if screen {
                    npc.position.y
                } else {
                    c.world.ground_y - npc.depth - npc.position.y
                },
                -npc.depth * 3.,
            ];
            let mut sample = actor_sample(
                assets,
                "idle",
                npc.ticks as f32,
                at,
                npc.facing.sign(),
                0.,
                1.,
            );
            sample.clip = if broker {
                "restrain"
            } else if contact.tension > 0.04 {
                "pull"
            } else {
                "restrained"
            };
            sample.phase = None;
            sample.distance = None;
            sample.looping = true;
            sample.contact = Some(ArmContact {
                wrist: [
                    contact.julia_wrist.x + x_offset,
                    y(contact.julia_wrist.y),
                    -contact.depth * 3.,
                ],
                elbow: if broker {
                    [
                        contact.broker_elbow.x + x_offset,
                        y(contact.broker_elbow.y),
                        -contact.depth * 3. + 9.,
                    ]
                } else {
                    [
                        (contact.julia_shoulder.x + contact.julia_wrist.x) * 0.5 + x_offset + 10.,
                        y(contact.julia_wrist.y + 16.),
                        -contact.depth * 3. + 9.,
                    ]
                },
                // Both face left: these adjacent shoulders lie near z=-24;
                // the opposite arms start outside the pair's shared depth.
                side: if broker { "r" } else { "l" },
                palm_offset: if broker { 0.06 } else { 0. },
            });
            sample
        })
        .collect()
}

pub(super) fn draw_screen(
    d: &mut impl RaylibDraw,
    a: &ProductionAssets,
    c: &Chapter,
    camera: f32,
) -> bool {
    let samples = samples(a, c, Some(camera));
    if samples.is_empty() {
        return false;
    }
    let models = a.models.as_ref().expect("validated pair models");
    for sample in samples {
        models.draw_screen(d, sample);
    }
    true
}

pub(super) fn draw_stage(d: &mut impl RaylibDraw3D, a: &ProductionAssets, c: &Chapter) -> bool {
    let samples = samples(a, c, None);
    if samples.is_empty() {
        return false;
    }
    let models = a.models.as_ref().expect("validated pair models");
    for sample in samples {
        models.draw(d, sample);
    }
    true
}
