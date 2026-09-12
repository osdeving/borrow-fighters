//! Articulates the three Augusta running silhouettes from their existing painted frames.
//!
//! Runtime texture cutouts replace both baked legs with two fixed-length chains.
//! The torso, clothes, face and shoes retain the original art in both renderers;
//! no texture files, combat rules or persistent animation state are introduced.

use super::{ActorAssets, Transform, debug_chain};
use crate::adventure::production::{
    Facing,
    animation::{self as a, Attachment, Chain, Point},
};
use raylib::prelude::*;
use std::f32::consts::{PI, TAU};

#[derive(Clone, Copy)]
struct Arm {
    shoulder: Point,
    elbow: Point,
    wrist: Point,
    radius: f32,
}

#[derive(Clone, Copy)]
struct Profile {
    frame: &'static str,
    dimensions: [f32; 2],
    torso_bottom: f32,
    coat_tail: Option<[f32; 4]>,
    hip: Point,
    hip_gap: f32,
    source_leg: Chain,
    leg_widths: [f32; 3],
    foot: [f32; 4],
    sole: Point,
    stride: f32,
    lowering: f32,
    lift: f32,
    period: f32,
    stance: f32,
    arms: [Arm; 2],
    arm_swing: f32,
}

fn profile(character: &str) -> Option<Profile> {
    Some(match character {
        "security" => Profile {
            frame: "idle",
            dimensions: [304., 342.],
            torso_bottom: 160.,
            coat_tail: None,
            hip: [147., 159.],
            hip_gap: 15.,
            source_leg: Chain {
                root: [170., 166.],
                joint: [211., 225.],
                tip: [236., 282.],
            },
            leg_widths: [34., 25., 18.],
            foot: [210., 277., 94., 65.],
            sole: [246., 328.],
            stride: 44.,
            lowering: 10.,
            lift: 25.,
            period: 40.,
            stance: 0.61,
            arms: [
                Arm {
                    shoulder: [111., 63.],
                    elbow: [89., 105.],
                    wrist: [154., 66.],
                    radius: 23.,
                },
                Arm {
                    shoulder: [195., 66.],
                    elbow: [225., 105.],
                    wrist: [215., 62.],
                    radius: 20.,
                },
            ],
            arm_swing: 8.,
        },
        "broker" => Profile {
            frame: "flee",
            dimensions: [308., 340.],
            torso_bottom: 154.,
            coat_tail: Some([0., 154., 122., 20.]),
            hip: [158., 164.],
            hip_gap: 12.,
            source_leg: Chain {
                root: [158., 160.],
                joint: [216., 211.],
                tip: [238., 283.],
            },
            leg_widths: [30., 23., 16.],
            foot: [215., 279., 93., 61.],
            sole: [248., 331.],
            stride: 62.,
            lowering: 0.,
            lift: 55.,
            period: 29.,
            stance: 0.5,
            arms: [
                Arm {
                    shoulder: [144., 60.],
                    elbow: [99., 78.],
                    wrist: [63., 125.],
                    radius: 18.,
                },
                Arm {
                    shoulder: [209., 75.],
                    elbow: [235., 121.],
                    wrist: [247., 88.],
                    radius: 18.,
                },
            ],
            arm_swing: 13.,
        },
        "julia" => Profile {
            frame: "run",
            dimensions: [280., 340.],
            torso_bottom: 158.,
            coat_tail: Some([0., 158., 119., 31.]),
            hip: [158., 174.],
            hip_gap: 11.,
            source_leg: Chain {
                root: [161., 167.],
                joint: [194., 215.],
                tip: [216., 289.],
            },
            leg_widths: [27., 21., 12.],
            foot: [205., 287., 75., 53.],
            sole: [230., 331.],
            stride: 46.,
            lowering: 0.,
            lift: 49.,
            period: 33.,
            stance: 0.52,
            arms: [
                Arm {
                    shoulder: [119., 80.],
                    elbow: [88., 109.],
                    wrist: [73., 147.],
                    radius: 17.,
                },
                Arm {
                    shoulder: [190., 91.],
                    elbow: [207., 135.],
                    wrist: [236., 116.],
                    radius: 16.,
                },
            ],
            arm_swing: 11.,
        },
        _ => return None,
    })
}

fn ease(p: f32) -> f32 {
    let p = p.clamp(0., 1.);
    p * p * (3. - 2. * p)
}

/// The support foot travels linearly backwards; recovery clears the floor and
/// returns with zero vertical velocity. Left and right feet share one clock.
fn foot_sample(p: Profile, ticks: f32, leg: usize) -> (Point, f32) {
    let phase = (ticks / p.period + leg as f32 * 0.5).rem_euclid(1.);
    if phase < p.stance {
        ([p.stride * (1. - 2. * phase / p.stance), 0.], 0.)
    } else {
        let recovery = (phase - p.stance) / (1. - p.stance);
        let lift = (PI * recovery).sin().powi(2);
        (
            [-p.stride + 2. * p.stride * ease(recovery), -p.lift * lift],
            -14. * lift,
        )
    }
}

fn body_bob(p: Profile, ticks: f32) -> f32 {
    p.lowering + 3.0 - 3.0 * (ticks / p.period * TAU * 2.).cos()
}

fn distance_clock(p: Profile, sprite: &Attachment, signed_distance: f32) -> f32 {
    let source_scale = sprite.size[0] / sprite.source[2];
    signed_distance * p.period * p.stance / (2. * p.stride * source_scale)
}

pub(super) fn stride_ticks(assets: &ActorAssets, signed_distance: f32) -> Option<f32> {
    let p = profile(&assets.rig.character_id)?;
    let sprite = assets.rig.attachments.get(p.frame)?;
    if !signed_distance.is_finite() || [sprite.source[2], sprite.source[3]] != p.dimensions {
        return None;
    }
    Some(distance_clock(p, sprite, signed_distance))
}

fn target_leg(p: Profile, sprite: &Attachment, ticks: f32, index: usize) -> (Chain, Point, f32) {
    let (foot, angle) = foot_sample(p, ticks, index);
    let foot = [p.hip[0] + foot[0], sprite.anchor[1] + foot[1]];
    let ankle = a::add(foot, a::rotate(a::sub(p.source_leg.tip, p.sole), angle));
    let hip = [
        p.hip[0] + if index == 0 { p.hip_gap } else { -p.hip_gap },
        p.hip[1] + body_bob(p, ticks),
    ];
    let upper = a::length(a::sub(p.source_leg.joint, p.source_leg.root));
    let lower = a::length(a::sub(p.source_leg.tip, p.source_leg.joint));
    (a::solve_chain(hip, ankle, upper, lower, -1.), foot, angle)
}

fn normalized(p: Point) -> Point {
    a::mul(p, 1. / a::length(p).max(0.001))
}

fn lerp(a: Point, b: Point, t: f32) -> Point {
    [a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t]
}

fn ribbon_point(chain: Chain, widths: [f32; 3], t: f32, across: f32) -> Point {
    let upper = normalized(a::sub(chain.joint, chain.root));
    let lower = normalized(a::sub(chain.tip, chain.joint));
    let middle = normalized(a::add([upper[1], -upper[0]], [lower[1], -lower[0]]));
    let (center, normal, width) = if t <= 0.5 {
        let q = t * 2.;
        (
            lerp(chain.root, chain.joint, q),
            normalized(lerp([1., 0.], middle, q)),
            widths[0] + (widths[1] - widths[0]) * q,
        )
    } else {
        let q = (t - 0.5) * 2.;
        (
            lerp(chain.joint, chain.tip, q),
            normalized(lerp(middle, [lower[1], -lower[0]], q)),
            widths[1] + (widths[2] - widths[1]) * q,
        )
    };
    a::add(center, a::mul(normal, width * across))
}

fn segment_distance(point: Point, a: Point, b: Point) -> f32 {
    let delta = a::sub(b, a);
    let from = a::sub(point, a);
    let squared = delta[0] * delta[0] + delta[1] * delta[1];
    let along = ((from[0] * delta[0] + from[1] * delta[1]) / squared.max(0.001)).clamp(0., 1.);
    a::length(a::sub(point, a::add(a, a::mul(delta, along))))
}

fn body_point(p: Profile, source: Point, ticks: f32) -> Point {
    let mut strongest = 0.;
    let mut movement = [0., 0.];
    for (i, arm) in p.arms.iter().enumerate() {
        let distance = segment_distance(source, arm.shoulder, arm.elbow)
            .min(segment_distance(source, arm.elbow, arm.wrist));
        let weight =
            ease(1. - distance / arm.radius) * ease(a::length(a::sub(source, arm.shoulder)) / 30.);
        if weight > strongest {
            let angle = (ticks / p.period * TAU + i as f32 * PI).sin() * p.arm_swing;
            movement = a::mul(
                a::sub(
                    a::add(arm.shoulder, a::rotate(a::sub(source, arm.shoulder), angle)),
                    source,
                ),
                weight,
            );
            strongest = weight;
        }
    }
    let point = a::add(source, movement);
    let sway = (ticks / p.period * TAU).sin() * 1.3;
    let point = a::add(p.hip, a::rotate(a::sub(point, p.hip), sway));
    // Running source frames place the waistband above the articulated hip.
    // Register that cut six pixels into the thigh roots to keep one continuous
    // silhouette, including when the pelvis bobs or the torso counter-rotates.
    let waist_registration = p.hip[1] - p.torso_bottom + 6.;
    a::add(point, [0., body_bob(p, ticks) + waist_registration])
}

fn local(sprite: &Attachment, source: Point) -> Point {
    [
        (source[0] - sprite.anchor[0]) * sprite.size[0] / sprite.source[2],
        (source[1] - sprite.anchor[1]) * sprite.size[1] / sprite.source[3],
    ]
}

#[derive(Clone, Copy)]
struct Vertex {
    source: Point,
    destination: Point,
}

/// Returns false for other frames/packs, preserving their original frame playback.
pub(super) fn draw(
    d: &mut impl RaylibDraw,
    assets: &ActorAssets,
    ticks: f32,
    at: Point,
    facing: Facing,
    scale: f32,
    debug: bool,
) -> bool {
    let Some(p) = profile(&assets.rig.character_id) else {
        return false;
    };
    let Some(sprite) = assets.rig.attachments.get(p.frame) else {
        return false;
    };
    if [sprite.source[2], sprite.source[3]] != p.dimensions {
        return false;
    }
    let transform = Transform {
        at,
        sign: facing.sign(),
        scale,
    };
    let texture = &assets.textures[&sprite.image];
    let legs = [
        target_leg(p, sprite, ticks, 0),
        target_leg(p, sprite, ticks, 1),
    ];
    let mut quads: Vec<([Vertex; 4], Color)> = Vec::new();
    for i in [1, 0] {
        let (chain, foot, angle) = legs[i];
        let tint = if i == 1 {
            Color::new(195, 200, 212, 255)
        } else {
            Color::WHITE
        };
        for row in 0..16 {
            let top = row as f32 / 16.;
            let bottom = (row + 1) as f32 / 16.;
            let nodes = [(top, -1.), (top, 1.), (bottom, -1.), (bottom, 1.)];
            quads.push((
                nodes.map(|(t, cross)| Vertex {
                    source: ribbon_point(p.source_leg, p.leg_widths, t, cross),
                    destination: ribbon_point(chain, p.leg_widths, t, cross),
                }),
                tint,
            ));
        }
        let [x, y, w, h] = p.foot;
        quads.push((
            [[x, y], [x + w, y], [x, y + h], [x + w, y + h]].map(|source| Vertex {
                source,
                destination: a::add(foot, a::rotate(a::sub(source, p.sole), angle)),
            }),
            tint,
        ));
    }
    for rect in std::iter::once([0., 0., p.dimensions[0], p.torso_bottom]).chain(p.coat_tail) {
        let [x, y, w, h] = rect;
        let columns = (w / 7.).ceil() as usize;
        let rows = (h / 7.).ceil() as usize;
        for row in 0..rows {
            for col in 0..columns {
                let left = x + w * col as f32 / columns as f32;
                let right = x + w * (col + 1) as f32 / columns as f32;
                let top = y + h * row as f32 / rows as f32;
                let bottom = y + h * (row + 1) as f32 / rows as f32;
                quads.push((
                    [[left, top], [right, top], [left, bottom], [right, bottom]].map(|source| {
                        Vertex {
                            source,
                            destination: body_point(p, source, ticks),
                        }
                    }),
                    Color::WHITE,
                ));
            }
        }
    }
    d.rl_set_texture(texture);
    d.rl_draw(DrawMode::Triangles, |draw| {
        draw.normal3f(0., 0., 1.);
        for (vertices, tint) in quads {
            draw.color4ub(tint);
            for tri in [[0, 1, 2], [1, 3, 2]] {
                let points = tri.map(|i| transform.point(local(sprite, vertices[i].destination)));
                let cross = (points[1].x - points[0].x) * (points[2].y - points[0].y)
                    - (points[1].y - points[0].y) * (points[2].x - points[0].x);
                let tri = if cross > 0. {
                    [tri[0], tri[2], tri[1]]
                } else {
                    tri
                };
                for i in tri {
                    let v = vertices[i];
                    draw.texcoord2f(
                        (sprite.source[0] + v.source[0]) / texture.width as f32,
                        (sprite.source[1] + v.source[1]) / texture.height as f32,
                    );
                    let point = transform.point(local(sprite, v.destination));
                    draw.vertex2f(point.x, point.y);
                }
            }
        }
    });
    d.rl_disable_texture();
    if debug {
        for (chain, _, _) in legs {
            debug_chain(
                d,
                Chain {
                    root: local(sprite, chain.root),
                    joint: local(sprite, chain.joint),
                    tip: local(sprite, chain.tip),
                },
                transform,
                Color::LIME,
            );
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs, path::PathBuf};

    #[test]
    fn every_step_preserves_bone_lengths_and_reaches_its_authored_ankle() {
        for id in ["security", "broker", "julia"] {
            let p = profile(id).unwrap();
            let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join(format!("assets/adventure/actors/{id}/rig.json"));
            let rig: a::Rig = serde_json::from_str(&fs::read_to_string(path).unwrap()).unwrap();
            let sprite = &rig.attachments[p.frame];
            let upper = a::length(a::sub(p.source_leg.joint, p.source_leg.root));
            let lower = a::length(a::sub(p.source_leg.tip, p.source_leg.joint));
            for sample in 0..400 {
                let tick = sample as f32 * p.period / 200.;
                for i in 0..2 {
                    let (chain, foot, angle) = target_leg(p, sprite, tick, i);
                    let expected = a::add(foot, a::rotate(a::sub(p.source_leg.tip, p.sole), angle));
                    assert!(
                        (a::length(a::sub(chain.joint, chain.root)) - upper).abs() < 0.001,
                        "{id} upper leg"
                    );
                    assert!(
                        (a::length(a::sub(chain.tip, chain.joint)) - lower).abs() < 0.001,
                        "{id} lower leg"
                    );
                    assert!(
                        a::length(a::sub(chain.tip, expected)) < 0.01,
                        "{id} unreachable ankle at tick {tick}, leg {i}: {:?} != {:?}",
                        chain.tip,
                        expected
                    );
                    assert!(foot[1] <= sprite.anchor[1]);
                }
            }
        }
    }

    #[test]
    fn support_is_linear_and_recovery_keeps_the_foot_clear() {
        for id in ["security", "broker", "julia"] {
            let p = profile(id).unwrap();
            let first = foot_sample(p, 2., 0).0;
            let second = foot_sample(p, 3., 0).0;
            let third = foot_sample(p, 4., 0).0;
            assert_eq!(first[1], 0.);
            assert_eq!(second[1], 0.);
            assert!(((second[0] - first[0]) - (third[0] - second[0])).abs() < 0.001);
            let mid = (p.stance + (1. - p.stance) * 0.5) * p.period;
            assert!(foot_sample(p, mid, 0).0[1] < -p.lift * 0.99);
            assert!(
                a::length(a::sub(
                    foot_sample(p, 0., 0).0,
                    foot_sample(p, p.period, 0).0
                )) < 0.001
            );
        }
    }

    #[test]
    fn distance_clock_holds_the_support_foot_on_the_street_in_both_directions() {
        for id in ["security", "broker", "julia"] {
            let p = profile(id).unwrap();
            let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join(format!("assets/adventure/actors/{id}/rig.json"));
            let rig: a::Rig = serde_json::from_str(&fs::read_to_string(path).unwrap()).unwrap();
            let sprite = &rig.attachments[p.frame];
            let scale = sprite.size[0] / sprite.source[2];
            for direction in [-1., 1.] {
                let mut support: Option<f32> = None;
                for step in 0..80 {
                    let distance = p.stride * scale * (0.2 + step as f32 / 80.);
                    let ticks = distance_clock(p, sprite, distance);
                    let foot = foot_sample(p, ticks, 0).0;
                    assert_eq!(foot[1], 0.);
                    let world_x = direction * distance
                        + direction * (p.hip[0] + foot[0] - sprite.anchor[0]) * scale;
                    if let Some(previous) = support {
                        assert!((world_x - previous).abs() < 0.001, "{id} foot slid");
                    }
                    support = Some(world_x);
                }
            }
        }
    }
}
