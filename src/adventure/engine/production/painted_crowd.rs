//! Articulates Augusta's painted adult cast from registered source landmarks.
//!
//! System: Adventure production renderer. The same textured limb meshes follow
//! the deterministic pedestrians in gameplay and cinematic perspective shots.

use super::assets::ProductionAssets;
use crate::adventure::{
    augusta::ambient::{Activity, PERSON_WIDTH_RATIO, Pedestrian, Wardrobe},
    production::animation::Attachment,
};
use raylib::prelude::*;
use serde::Deserialize;
use std::{cell::RefCell, collections::BTreeMap, error::Error, fs, path::Path};

type Point = [f32; 2];

const WARDROBES: [&str; 8] = [
    "leather",
    "plum_dress",
    "amber_jacket",
    "denim",
    "teal_dress",
    "white_shirt",
    "red_blouse",
    "long_coat",
];

const CHROMA: &str = r#"#version 330
in vec2 fragTexCoord;
in vec4 fragColor;
uniform sampler2D texture0;
uniform vec4 colDiffuse;
uniform float chromaCutoff;
uniform vec4 spriteBounds;
out vec4 finalColor;
void main() {
    if (fragTexCoord.x < spriteBounds.x || fragTexCoord.y < spriteBounds.y ||
        fragTexCoord.x > spriteBounds.z || fragTexCoord.y > spriteBounds.w) discard;
    vec4 paint = texture(texture0, fragTexCoord);
    float excess = paint.g - max(paint.r, paint.b);
    float alpha = paint.a * (1.0 - smoothstep(chromaCutoff, 0.20, excess));
    if (alpha < 0.015) discard;
    if (excess > 0.025) paint.g = min(paint.g, max(paint.r, paint.b) + 0.015);
    finalColor = vec4(paint.rgb, alpha) * fragColor * colDiffuse;
}
"#;

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Landmarks {
    shoulder_back: Point,
    shoulder_front: Point,
    elbow_back: Point,
    elbow_front: Point,
    hand_back: Point,
    hand_front: Point,
    hip_back: Point,
    hip_front: Point,
    knee_back: Point,
    knee_front: Point,
    ankle_back: Point,
    ankle_front: Point,
    #[serde(default)]
    head: Option<Point>,
    #[serde(default)]
    foot_back: Option<Point>,
    #[serde(default)]
    foot_front: Option<Point>,
}

impl Landmarks {
    fn points(&self) -> [Point; 12] {
        [
            self.shoulder_back,
            self.shoulder_front,
            self.elbow_back,
            self.elbow_front,
            self.hand_back,
            self.hand_front,
            self.hip_back,
            self.hip_front,
            self.knee_back,
            self.knee_front,
            self.ankle_back,
            self.ankle_front,
        ]
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Widths {
    arm_back: Point,
    arm_front: Point,
    leg_back: Point,
    leg_front: Point,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct CastMember {
    piece: String,
    top_y: f32,
    sole_y: f32,
    body_polygon: Vec<Point>,
    landmarks: Landmarks,
    limb_widths: Widths,
    #[serde(default)]
    limb_polygons: BTreeMap<String, Vec<Point>>,
    #[serde(skip)]
    profile: bool,
    #[serde(skip)]
    body_triangles: Vec<[Point; 3]>,
    #[serde(skip)]
    masks: BTreeMap<String, Vec<[Point; 3]>>,
}

impl CastMember {
    // Near limbs are on the left of the frontal painting, but the right of
    // the profile. A profile's shoulders may share X, so screen position
    // cannot reliably distinguish their opposite gait phases.
    fn pose_slot(&self, back: bool) -> usize {
        usize::from(back != self.profile)
    }
}

/// Validated source registrations; loading this data does not allocate a GPU.
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CastCatalog {
    schema_version: u32,
    entries: BTreeMap<String, CastMember>,
    profile_entries: BTreeMap<String, CastMember>,
}

fn cross(a: Point, b: Point, c: Point) -> f32 {
    (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0])
}

fn triangulate(points: &[Point]) -> Result<Vec<[Point; 3]>, String> {
    if points.len() < 3 || points.len() > 64 {
        return Err("body polygon must have 3–64 vertices".into());
    }
    let mut remaining: Vec<_> = (0..points.len()).collect();
    let area: f32 = points
        .iter()
        .zip(points.iter().cycle().skip(1))
        .map(|(a, b)| a[0] * b[1] - b[0] * a[1])
        .sum();
    if area.abs() < 1.0 {
        return Err("body polygon has no area".into());
    }
    if area < 0.0 {
        remaining.reverse();
    }
    let mut triangles = Vec::with_capacity(points.len() - 2);
    while remaining.len() > 3 {
        let ear = (0..remaining.len())
            .find(|&i| {
                let indices = [
                    remaining[(i + remaining.len() - 1) % remaining.len()],
                    remaining[i],
                    remaining[(i + 1) % remaining.len()],
                ];
                let [a, b, c] = indices.map(|index| points[index]);
                cross(a, b, c) > 0.001
                    && !remaining.iter().any(|&other| {
                        !indices.contains(&other)
                            && cross(a, b, points[other]) >= -0.001
                            && cross(b, c, points[other]) >= -0.001
                            && cross(c, a, points[other]) >= -0.001
                    })
            })
            .ok_or("body polygon is self-intersecting or degenerate")?;
        let indices = [
            remaining[(ear + remaining.len() - 1) % remaining.len()],
            remaining[ear],
            remaining[(ear + 1) % remaining.len()],
        ];
        triangles.push(indices.map(|index| points[index]));
        remaining.remove(ear);
    }
    triangles.push([
        points[remaining[0]],
        points[remaining[1]],
        points[remaining[2]],
    ]);
    Ok(triangles)
}

// Add interior vertices so a bent elbow, knee or skirt hem can deform smoothly
// without repeating overlapping source pixels in independently capped bands.
fn refine(triangles: Vec<[Point; 3]>) -> Vec<[Point; 3]> {
    refine_affine(triangles, |_| false)
}

fn refine_body(triangles: Vec<[Point; 3]>, member: &CastMember, cloth: bool) -> Vec<[Point; 3]> {
    let units = 96.0 / (member.sole_y - member.top_y);
    let head_clamp = member.sole_y - 72.0 / units;
    let hip = (member.landmarks.hip_back[1] + member.landmarks.hip_front[1]) * 0.5;
    refine_affine(triangles, |triangle| {
        let low = triangle.iter().map(|p| p[1]).fold(f32::INFINITY, f32::min);
        let high = triangle
            .iter()
            .map(|p| p[1])
            .fold(f32::NEG_INFINITY, f32::max);
        // Within one body.high interval the painted torso transform is affine:
        // subdivision cannot change its geometry or texture interpolation.
        // Keep the original 22px detail across clamps and below the cloth root.
        !(low < head_clamp && high > head_clamp
            || low < member.sole_y && high > member.sole_y
            || cloth && high > hip)
    })
}

fn refine_affine(
    triangles: Vec<[Point; 3]>,
    affine: impl Fn(&[Point; 3]) -> bool,
) -> Vec<[Point; 3]> {
    let mut pending = triangles;
    let mut result = Vec::new();
    while let Some(t) = pending.pop() {
        let (edge, length) = (0..3)
            .map(|i| (i, distance(t[i], t[(i + 1) % 3])))
            .max_by(|a, b| a.1.total_cmp(&b.1))
            .unwrap();
        if length <= 22.0 || affine(&t) {
            result.push(t);
        } else {
            let a = t[edge];
            let b = t[(edge + 1) % 3];
            let c = t[(edge + 2) % 3];
            let middle = scale(add(a, b), 0.5);
            pending.push([a, middle, c]);
            pending.push([middle, b, c]);
        }
    }
    result
}

impl CastCatalog {
    /// Rejects missing wardrobes, unsafe registrations or malformed body masks.
    pub fn load(
        path: &Path,
        pieces: &BTreeMap<String, Attachment>,
    ) -> Result<Self, Box<dyn Error>> {
        let source = fs::read_to_string(path)
            .map_err(|e| format!("painted crowd {}: {e}", path.display()))?;
        let mut catalog: Self = serde_json::from_str(&source)
            .map_err(|e| format!("painted crowd {}: {e}", path.display()))?;
        catalog.validate(pieces)?;
        Ok(catalog)
    }

    fn member(&self, person: &Pedestrian) -> &CastMember {
        let view = if person.fleeing || person.activity == Activity::Walking {
            &self.profile_entries
        } else {
            &self.entries
        };
        &view[wardrobe_key(person.wardrobe)]
    }

    fn validate(&mut self, pieces: &BTreeMap<String, Attachment>) -> Result<(), String> {
        if self.schema_version != 1 {
            return Err("painted crowd requires schema version 1".into());
        }
        for (view, members) in [
            ("front", &mut self.entries),
            ("profile", &mut self.profile_entries),
        ] {
            if members.len() != WARDROBES.len() {
                return Err(format!(
                    "painted crowd requires all eight adult wardrobes in {view} view"
                ));
            }
            for id in WARDROBES {
                let member = members
                    .get_mut(id)
                    .ok_or_else(|| format!("painted crowd missing {view} wardrobe {id}"))?;
                member.profile = view == "profile";
                let piece = pieces.get(&member.piece).ok_or_else(|| {
                    format!(
                        "painted crowd {id} references missing piece {}",
                        member.piece
                    )
                })?;
                let inside = |p: Point| {
                    p[0].is_finite()
                        && p[1].is_finite()
                        && (0.0..=piece.source[2]).contains(&p[0])
                        && (0.0..=piece.source[3]).contains(&p[1])
                };
                if !member.top_y.is_finite()
                    || !member.sole_y.is_finite()
                    || member.top_y < 0.0
                    || member.sole_y > piece.source[3]
                    || member.sole_y - member.top_y < 100.0
                    || member
                        .landmarks
                        .points()
                        .into_iter()
                        .chain(member.body_polygon.iter().copied())
                        .any(|p| !inside(p))
                    || [
                        member.landmarks.head,
                        member.landmarks.foot_back,
                        member.landmarks.foot_front,
                    ]
                    .into_iter()
                    .flatten()
                    .any(|p| !inside(p))
                    || [
                        member.limb_widths.arm_back,
                        member.limb_widths.arm_front,
                        member.limb_widths.leg_back,
                        member.limb_widths.leg_front,
                    ]
                    .into_iter()
                    .flatten()
                    .any(|w| !w.is_finite() || w < 2.0 || w > piece.source[2] * 0.5)
                {
                    return Err(format!(
                        "painted crowd {id} has invalid source landmarks or widths"
                    ));
                }
                let l = &member.landmarks;
                if member.profile
                    && [(l.ankle_back, l.foot_back), (l.ankle_front, l.foot_front)]
                        .into_iter()
                        .any(|(ankle, toe)| toe.is_none_or(|toe| toe[0] <= ankle[0]))
                {
                    return Err(format!(
                        "painted crowd {id} profile requires both feet pointing right"
                    ));
                }
                for [a, b, c] in [
                    [l.shoulder_back, l.elbow_back, l.hand_back],
                    [l.shoulder_front, l.elbow_front, l.hand_front],
                    [l.hip_back, l.knee_back, l.ankle_back],
                    [l.hip_front, l.knee_front, l.ankle_front],
                ] {
                    if distance(a, b) < 5.0 || distance(b, c) < 5.0 {
                        return Err(format!("painted crowd {id} has a collapsed limb"));
                    }
                }
                member.body_triangles = refine_body(
                    triangulate(&member.body_polygon)
                        .map_err(|e| format!("painted crowd {id}: {e}"))?,
                    member,
                    matches!(id, "plum_dress" | "teal_dress" | "long_coat"),
                );
                member.masks.clear();
                for name in ["arm_back", "arm_front", "leg_back", "leg_front"] {
                    let polygon = member
                        .limb_polygons
                        .get(name)
                        .ok_or_else(|| format!("painted crowd {id} missing {name} silhouette"))?;
                    if polygon.iter().any(|point| !inside(*point)) {
                        return Err(format!(
                            "painted crowd {id} has an out-of-bounds {name} silhouette"
                        ));
                    }
                    let triangles = triangulate(polygon)
                        .map_err(|e| format!("painted crowd {id} {name}: {e}"))?;
                    member.masks.insert(name.to_owned(), refine(triangles));
                }
            }
        }
        for id in WARDROBES {
            if self.entries[id].piece == self.profile_entries[id].piece {
                return Err(format!(
                    "painted crowd {id} needs a distinct painted profile piece"
                ));
            }
        }
        Ok(())
    }
}

/// Context-owned shader and registrations; textures remain in the world catalog.
pub struct PaintedCrowd {
    catalog: CastCatalog,
    shader: RefCell<Shader>,
    bounds_location: i32,
}

impl PaintedCrowd {
    pub fn new(
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        catalog: CastCatalog,
    ) -> Result<Self, String> {
        let mut shader = rl.load_shader_from_memory(thread, None, Some(CHROMA));
        let cutoff = shader.get_shader_location("chromaCutoff");
        let bounds_location = shader.get_shader_location("spriteBounds");
        if !shader.is_shader_valid() || cutoff < 0 || bounds_location < 0 {
            return Err("painted crowd chroma shader could not be loaded".into());
        }
        shader.set_shader_value(cutoff, 0.04_f32);
        Ok(Self {
            catalog,
            shader: RefCell::new(shader),
            bounds_location,
        })
    }
}

fn wardrobe_key(wardrobe: Wardrobe) -> &'static str {
    match wardrobe {
        Wardrobe::Leather => "leather",
        Wardrobe::PlumDress => "plum_dress",
        Wardrobe::AmberJacket => "amber_jacket",
        Wardrobe::Denim => "denim",
        Wardrobe::TealDress => "teal_dress",
        Wardrobe::WhiteShirt => "white_shirt",
        Wardrobe::RedBlouse => "red_blouse",
        Wardrobe::LongCoat => "long_coat",
    }
}

fn distance(a: Point, b: Point) -> f32 {
    (b[0] - a[0]).hypot(b[1] - a[1])
}
fn add(a: Point, b: Point) -> Point {
    [a[0] + b[0], a[1] + b[1]]
}
fn scale(a: Point, s: f32) -> Point {
    [a[0] * s, a[1] * s]
}
fn sub(a: Point, b: Point) -> Point {
    [a[0] - b[0], a[1] - b[1]]
}

fn joint(start: Point, goal: Point, upper: f32, lower: f32, bend: f32) -> [Point; 3] {
    let delta = sub(goal, start);
    let length = distance(start, goal).max(0.001);
    let reach = length.clamp((upper - lower).abs() + 0.01, upper + lower - 0.01);
    let direction = scale(delta, 1.0 / length);
    let end = add(start, scale(direction, reach));
    let along = (upper * upper - lower * lower + reach * reach) / (2.0 * reach);
    let height = (upper * upper - along * along).max(0.0).sqrt() * bend;
    let middle = add(
        start,
        add(
            scale(direction, along),
            [-direction[1] * height, direction[0] * height],
        ),
    );
    [start, middle, end]
}

#[derive(Clone, Copy)]
struct Vertex {
    source: Point,
    target: Point,
}

fn mix(a: Point, b: Point, weight: f32) -> Point {
    add(scale(a, 1.0 - weight), scale(b, weight))
}

fn smooth(low: f32, high: f32, value: f32) -> f32 {
    let t = ((value - low) / (high - low)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

fn dot(a: Point, b: Point) -> f32 {
    a[0] * b[0] + a[1] * b[1]
}

#[derive(Clone, Copy)]
struct Chain {
    source: [Point; 3],
    target: [Point; 3],
    units: f32,
    softness: f32,
    axes: [[Point; 2]; 2],
    joint_axis: Point,
}

impl Chain {
    fn new(source: [Point; 3], target: [Point; 3], units: f32, softness: f32) -> Self {
        let axes = std::array::from_fn(|i| {
            [
                scale(
                    sub(source[i + 1], source[i]),
                    1.0 / distance(source[i], source[i + 1]),
                ),
                scale(
                    sub(target[i + 1], target[i]),
                    1.0 / distance(target[i], target[i + 1]),
                ),
            ]
        });
        Self {
            source,
            target,
            units,
            softness,
            axes,
            joint_axis: scale(
                sub(source[2], source[0]),
                1.0 / distance(source[0], source[2]),
            ),
        }
    }

    fn bone(&self, point: Point, segment: usize) -> Point {
        let a = self.source[segment];
        let [source, target] = self.axes[segment];
        let delta = sub(point, a);
        let along = dot(delta, source) * self.units;
        let across = dot(delta, [-source[1], source[0]]) * self.units;
        add(
            self.target[segment],
            add(scale(target, along), scale([-target[1], target[0]], across)),
        )
    }

    fn skin(&self, point: Point) -> Point {
        let across_joint = dot(sub(point, self.source[1]), self.joint_axis);
        let weight = smooth(-self.softness, self.softness, across_joint);
        if weight <= 0.0 {
            self.bone(point, 0)
        } else if weight >= 1.0 {
            self.bone(point, 1)
        } else {
            mix(self.bone(point, 0), self.bone(point, 1), weight)
        }
    }
}

struct Deform<'a> {
    person: &'a Pedestrian,
    member: &'a CastMember,
    units: f32,
    center_x: f32,
    settle: f32,
    hem_y: f32,
}

impl<'a> Deform<'a> {
    fn new(person: &'a Pedestrian, member: &'a CastMember) -> Self {
        let mut result = Self {
            person,
            member,
            units: 96.0 / (member.sole_y - member.top_y),
            center_x: (member.landmarks.hip_back[0] + member.landmarks.hip_front[0]) * 0.5,
            settle: 0.0,
            hem_y: member.body_polygon.iter().map(|p| p[1]).fold(0.0, f32::max),
        };
        let l = &member.landmarks;
        for (back, source) in [
            (true, [l.hip_back, l.knee_back, l.ankle_back]),
            (false, [l.hip_front, l.knee_front, l.ankle_front]),
        ] {
            let side = member.pose_slot(back);
            let hip = result.body(source[0]);
            let foot = person.pose.feet[side];
            let ankle = [
                foot[0] + person.pose.seated * 10.0,
                foot[1] - (member.sole_y - source[2][1]).max(3.0) * result.units,
            ];
            let length = (distance(source[0], source[1]) + distance(source[1], source[2]))
                * result.units
                - 0.05;
            let dx = ankle[0] - hip[0];
            let vertical = (length * length - dx * dx).max(0.0).sqrt();
            result.settle = result
                .settle
                .max((ankle[1] - (hip[1] - result.settle) - vertical).max(0.0));
        }
        result
    }

    fn body(&self, source: Point) -> Point {
        let y = (source[1] - self.member.sole_y) * self.units;
        let high = (-y / 72.0).clamp(0.0, 1.0);
        [
            (source[0] - self.center_x) * self.units + self.person.pose.lean * high,
            y + self.person.pose.bob + self.person.pose.seated * 14.0 + self.settle,
        ]
    }

    fn arm_chain(&self, back: bool) -> Chain {
        let l = &self.member.landmarks;
        let source = if back {
            [l.shoulder_back, l.elbow_back, l.hand_back]
        } else {
            [l.shoulder_front, l.elbow_front, l.hand_front]
        };
        let side = self.member.pose_slot(back);
        let outside = if side == 0 { -1.0 } else { 1.0 };
        let shoulder = self.body(source[0]);
        let rest = self.body(source[2]);
        let hand = self.person.pose.hands[side];
        let mut target_hand = rest;
        if self.person.fleeing {
            target_hand[0] += hand[0] * 0.3;
            target_hand[1] += (hand[1] + 44.0) * 0.75;
        } else if self.person.pose.alarm > 0.0 {
            target_hand = [shoulder[0] + outside * 12.0, shoulder[1] + 1.0];
        } else if self.person.activity == Activity::Walking {
            target_hand[0] += hand[0] * 0.3;
            target_hand[1] += (hand[1] + 44.0) * 0.25;
        } else if self.person.activity == Activity::Phone && side == 1 {
            target_hand = [shoulder[0] + outside * 7.0, shoulder[1] - 7.0];
        } else if side == 1 {
            target_hand[0] += (hand[0] - 8.0) * 0.4;
            target_hand[1] += (hand[1] + 49.0) * 0.7;
        } else {
            target_hand[1] += (hand[1] + 41.0) * 0.5;
        }
        // Calm gestures stay on their own side; the running FK below permits
        // natural projected overlap while keeping both painted arms visible.
        target_hand[0] = if side == 0 {
            target_hand[0].min(shoulder[0] - 1.0)
        } else {
            target_hand[0].max(shoulder[0] + 1.0)
        };
        let outward_bend = if (target_hand[1] >= shoulder[1]) == (side == 0) {
            1.0
        } else {
            -1.0
        };
        let upper = distance(source[0], source[1]) * self.units;
        let lower = distance(source[1], source[2]) * self.units;
        let target = if self.person.fleeing {
            // Pump in the travel direction, with opposite phases and elbows
            // below the shoulders. The painted arms remain visible over the
            // chest when their natural projected path overlaps the torso.
            let angle = hand[0] / 24.0 * 0.55 + 0.1;
            let elbow = add(shoulder, [angle.sin() * upper, angle.cos() * upper]);
            let forearm = angle + 1.5;
            [
                shoulder,
                elbow,
                add(elbow, [forearm.sin() * lower, forearm.cos() * lower]),
            ]
        } else if self.member.profile
            && self.person.activity == Activity::Walking
            && self.person.pose.alarm == 0.0
        {
            // Sideways travel uses a small sagittal swing, independent of the
            // authored bind pose. The two sides retain opposite gait phases.
            let angle = hand[0] / 15.0 * 0.36;
            let elbow = add(shoulder, [angle.sin() * upper, angle.cos() * upper]);
            let forearm = angle + 0.12;
            [
                shoulder,
                elbow,
                add(elbow, [forearm.sin() * lower, forearm.cos() * lower]),
            ]
        } else if self.person.activity == Activity::Phone
            && side == 1
            && self.person.pose.alarm == 0.0
        {
            // The screen is held in front of the face, supported by a lowered
            // elbow instead of pulling the whole upper arm up to eye height.
            let elbow = add(
                shoulder,
                [outside * 0.28_f32.sin() * upper, 0.28_f32.cos() * upper],
            );
            [
                shoulder,
                elbow,
                add(
                    elbow,
                    [outside * 2.8_f32.sin() * lower, 2.8_f32.cos() * lower],
                ),
            ]
        } else {
            joint(shoulder, target_hand, upper, lower, outward_bend)
        };
        Chain::new(
            source,
            target,
            self.units,
            distance(source[0], source[1]) * 0.18,
        )
    }

    fn arm(&self, back: bool) -> (Vec<[Vertex; 3]>, Point) {
        let chain = self.arm_chain(back);
        let source = chain.source;
        let root_axis = chain.axes[0][0];
        let root_softness = distance(source[0], source[1]) * 0.25;
        let name = if back { "arm_back" } else { "arm_front" };
        let mesh = self.member.masks[name]
            .iter()
            .map(|triangle| {
                triangle.map(|point| {
                    let weight = smooth(0.0, root_softness, dot(sub(point, source[0]), root_axis));
                    Vertex {
                        source: point,
                        target: if weight <= 0.0 {
                            self.body(point)
                        } else if weight >= 1.0 {
                            chain.skin(point)
                        } else {
                            mix(self.body(point), chain.skin(point), weight)
                        },
                    }
                })
            })
            .collect();
        (mesh, chain.target[2])
    }

    fn leg_chain(&self, back: bool) -> Chain {
        let l = &self.member.landmarks;
        let source = if back {
            [l.hip_back, l.knee_back, l.ankle_back]
        } else {
            [l.hip_front, l.knee_front, l.ankle_front]
        };
        let side = self.member.pose_slot(back);
        let foot = self.person.pose.feet[side];
        let lift = if !self.person.fleeing && self.person.activity == Activity::Walking {
            0.5
        } else {
            1.0
        };
        let ankle = [
            foot[0] + self.person.pose.seated * 10.0,
            foot[1] * lift - (self.member.sole_y - source[2][1]).max(3.0) * self.units,
        ];
        Chain::new(
            source,
            joint(
                self.body(source[0]),
                ankle,
                distance(source[0], source[1]) * self.units,
                distance(source[1], source[2]) * self.units,
                -1.0,
            ),
            self.units,
            distance(source[0], source[1]) * 0.15,
        )
    }

    fn cloth(&self, point: Point, back: &Chain, front: &Chain) -> Point {
        let base = self.body(point);
        if !matches!(
            self.person.wardrobe,
            Wardrobe::PlumDress | Wardrobe::TealDress | Wardrobe::LongCoat
        ) {
            return base;
        }
        let hip_y = (back.source[0][1] + front.source[0][1]) * 0.5;
        let weight = smooth(hip_y, self.hem_y.max(hip_y), point[1]);
        if weight <= 0.0 {
            return base;
        }
        let side = smooth(self.center_x - 18.0, self.center_x + 18.0, point[0]);
        let (left, right) = if back.source[0][0] < front.source[0][0] {
            (back, front)
        } else {
            (front, back)
        };
        let cloth = if side <= 0.0 {
            left.skin(point)
        } else if side >= 1.0 {
            right.skin(point)
        } else {
            mix(left.skin(point), right.skin(point), side)
        };
        mix(base, cloth, weight)
    }

    fn leg(&self, back: bool, chain: &Chain) -> Vec<[Vertex; 3]> {
        let name = if back { "leg_back" } else { "leg_front" };
        let ankle = chain.source[2];
        let width = if back {
            self.member.limb_widths.leg_back[1]
        } else {
            self.member.limb_widths.leg_front[1]
        };
        self.member.masks[name]
            .iter()
            .map(|triangle| {
                triangle.map(|point| {
                    let flat = add(chain.target[2], scale(sub(point, ankle), self.units));
                    let weight = smooth(ankle[1] - width * 0.25, ankle[1] + width * 0.15, point[1]);
                    Vertex {
                        source: point,
                        target: if weight >= 1.0 {
                            flat
                        } else if weight <= 0.0 {
                            chain.skin(point)
                        } else {
                            mix(chain.skin(point), flat, weight)
                        },
                    }
                })
            })
            .collect()
    }
}

/// Draws a painted, articulated adult; no geometric human fallback is used.
pub fn draw_person(
    d: &mut impl RaylibDraw,
    assets: &ProductionAssets,
    person: &Pedestrian,
    camera_x: f32,
) {
    let member = assets.crowd.catalog.member(person);
    let piece = &assets.art.pieces[&member.piece];
    let texture = &assets.textures[&piece.image];
    let deform = Deform::new(person, member);
    let (mut back_arm, back_hand) = deform.arm(true);
    let back_leg = deform.leg_chain(true);
    let front_leg = deform.leg_chain(false);
    let mut mesh = deform.leg(true, &back_leg);
    mesh.extend(deform.leg(false, &front_leg));
    if member.profile {
        // In a true side view the far upper arm passes behind the ribcage.
        mesh.append(&mut back_arm);
    }
    mesh.extend(member.body_triangles.iter().map(|triangle| {
        triangle.map(|point| Vertex {
            source: point,
            target: deform.cloth(point, &back_leg, &front_leg),
        })
    }));
    // Visible sleeves and hands stay on the painted side of the torso. The
    // source masks prevent the near shoulder from carrying a second torso.
    mesh.extend(back_arm);
    let (front_arm, front_hand) = deform.arm(false);
    mesh.extend(front_arm);
    let world = |p: Point| {
        Vector2::new(
            person.x - camera_x + 640.0 + p[0] * person.scale * PERSON_WIDTH_RATIO * person.facing,
            person.ground_y + p[1] * person.scale,
        )
    };
    d.draw_ellipse(
        (person.x - camera_x + 640.0) as i32,
        person.ground_y as i32,
        23.0 * person.scale * PERSON_WIDTH_RATIO,
        4.0 * person.scale,
        Color::new(5, 9, 17, 96),
    );
    {
        let mut shader = assets.crowd.shader.borrow_mut();
        shader.set_shader_value(
            assets.crowd.bounds_location,
            Vector4::new(
                piece.source[0] / texture.width as f32,
                piece.source[1] / texture.height as f32,
                (piece.source[0] + piece.source[2]) / texture.width as f32,
                (piece.source[1] + piece.source[3]) / texture.height as f32,
            ),
        );
        let mut paint = d.begin_shader_mode(&mut shader);
        paint.rl_set_texture(texture);
        paint.rl_draw(DrawMode::Triangles, |draw| {
            draw.normal3f(0.0, 0.0, 1.0);
            draw.color4ub(Color::WHITE);
            for triangle in mesh {
                let targets = triangle.map(|v| world(v.target));
                let area = (targets[1].x - targets[0].x) * (targets[2].y - targets[0].y)
                    - (targets[1].y - targets[0].y) * (targets[2].x - targets[0].x);
                let indices = if area > 0.0 { [0, 2, 1] } else { [0, 1, 2] };
                for i in indices {
                    let vertex = triangle[i];
                    draw.texcoord2f(
                        (piece.source[0] + vertex.source[0]) / texture.width as f32,
                        (piece.source[1] + vertex.source[1]) / texture.height as f32,
                    );
                    let p = targets[i];
                    draw.vertex2f(p.x, p.y);
                }
            }
        });
        paint.rl_disable_texture();
    }
    if person.activity == Activity::Phone && !person.fleeing && person.pose.alarm == 0.0 {
        let hand = world(if member.landmarks.shoulder_front[0] >= deform.center_x {
            front_hand
        } else {
            back_hand
        });
        d.draw_rectangle_pro(
            Rectangle::new(
                hand.x,
                hand.y - 4.0 * person.scale,
                5.0 * person.scale,
                9.0 * person.scale,
            ),
            Vector2::new(2.5 * person.scale, 0.0),
            -10.0 * person.facing,
            Color::new(24, 31, 38, 255),
        );
        d.draw_line_ex(
            hand + Vector2::new(0.0, -3.0 * person.scale),
            hand + Vector2::new(0.0, 2.0 * person.scale),
            2.2 * person.scale,
            Color::new(110, 169, 177, 255),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_painted_catalog_covers_every_adult_and_rejects_bad_registration() {
        let root =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("assets/adventure/chapters/cpp-augusta");
        let art: super::super::assets::WorldArt =
            serde_json::from_str(&fs::read_to_string(root.join("world-art.json")).unwrap())
                .unwrap();
        let catalog = CastCatalog::load(&root.join(&art.nightlife_cast), &art.pieces).unwrap();
        assert_eq!(catalog.entries.len(), 8);
        assert_eq!(catalog.profile_entries.len(), 8);
        let mut invalid = catalog.clone();
        invalid
            .entries
            .get_mut("denim")
            .unwrap()
            .landmarks
            .hand_front[0] = f32::NAN;
        assert!(invalid.validate(&art.pieces).is_err());
        let mut missing = catalog.clone();
        missing.entries.remove("long_coat");
        assert!(missing.validate(&art.pieces).is_err());
        let mut missing_profile = catalog.clone();
        missing_profile.profile_entries.remove("long_coat");
        assert!(missing_profile.validate(&art.pieces).is_err());
    }

    #[test]
    fn displacement_uses_true_profile_for_each_wardrobe_and_both_directions() {
        use crate::adventure::augusta::ambient::Nightlife;
        let root =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("assets/adventure/chapters/cpp-augusta");
        let art: super::super::assets::WorldArt =
            serde_json::from_str(&fs::read_to_string(root.join("world-art.json")).unwrap())
                .unwrap();
        let catalog = CastCatalog::load(&root.join(&art.nightlife_cast), &art.pieces).unwrap();
        let sample = Nightlife::sample(240, None);
        let source = sample.people[0];
        for wardrobe in [
            Wardrobe::Leather,
            Wardrobe::PlumDress,
            Wardrobe::AmberJacket,
            Wardrobe::Denim,
            Wardrobe::TealDress,
            Wardrobe::WhiteShirt,
            Wardrobe::RedBlouse,
            Wardrobe::LongCoat,
        ] {
            for facing in [-1.0, 1.0] {
                let mut person = source;
                person.wardrobe = wardrobe;
                person.facing = facing;
                for activity in [
                    Activity::Walking,
                    Activity::Conversation,
                    Activity::Seated,
                    Activity::Phone,
                ] {
                    person.activity = activity;
                    for fleeing in [false, true] {
                        person.fleeing = fleeing;
                        let expected = fleeing || activity == Activity::Walking;
                        let member = catalog.member(&person);
                        assert_eq!(member.profile, expected);
                        assert_ne!(member.pose_slot(false), member.pose_slot(true));
                        let key = wardrobe_key(wardrobe);
                        assert_ne!(
                            catalog.entries[key].piece,
                            catalog.profile_entries[key].piece
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn real_painted_legs_reach_fixed_supports_across_full_movement_cycles() {
        use crate::adventure::augusta::ambient::Nightlife;
        let root =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("assets/adventure/chapters/cpp-augusta");
        let art: super::super::assets::WorldArt =
            serde_json::from_str(&fs::read_to_string(root.join("world-art.json")).unwrap())
                .unwrap();
        let catalog = CastCatalog::load(&root.join(&art.nightlife_cast), &art.pieces).unwrap();
        for ticks in (0..360).step_by(3) {
            let calm = Nightlife::sample(100 + ticks, None);
            let fleeing = Nightlife::sample(1030 + ticks, Some(30 + ticks));
            for member in catalog
                .entries
                .values()
                .chain(catalog.profile_entries.values())
            {
                for person in calm
                    .people
                    .iter()
                    .filter(|p| [0, 1, 3, 8].contains(&p.id))
                    .chain(fleeing.people.iter().filter(|p| p.id == 10))
                {
                    let deform = Deform::new(person, member);
                    let l = &member.landmarks;
                    for (back, source) in [
                        (true, [l.hip_back, l.knee_back, l.ankle_back]),
                        (false, [l.hip_front, l.knee_front, l.ankle_front]),
                    ] {
                        let side = member.pose_slot(back);
                        let foot = person.pose.feet[side];
                        let goal = [
                            foot[0] + person.pose.seated * 10.0,
                            foot[1] - (member.sole_y - source[2][1]).max(3.0) * deform.units,
                        ];
                        let chain = joint(
                            deform.body(source[0]),
                            goal,
                            distance(source[0], source[1]) * deform.units,
                            distance(source[1], source[2]) * deform.units,
                            -1.0,
                        );
                        assert!(
                            foot[1].abs() > 0.001 || distance(chain[2], goal) < 0.001,
                            "floating support in {} at {ticks}: {:?} vs {:?}",
                            member.piece,
                            chain[2],
                            goal
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn runner_elbows_stay_lowered_and_masks_never_duplicate_source_pixels() {
        use crate::adventure::augusta::ambient::Nightlife;
        let root =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("assets/adventure/chapters/cpp-augusta");
        let art: super::super::assets::WorldArt =
            serde_json::from_str(&fs::read_to_string(root.join("world-art.json")).unwrap())
                .unwrap();
        let catalog = CastCatalog::load(&root.join(&art.nightlife_cast), &art.pieces).unwrap();
        for member in catalog
            .entries
            .values()
            .chain(catalog.profile_entries.values())
        {
            for (name, polygon) in &member.limb_polygons {
                let source_area: f32 = polygon
                    .iter()
                    .zip(polygon.iter().cycle().skip(1))
                    .map(|(a, b)| a[0] * b[1] - b[0] * a[1])
                    .sum::<f32>()
                    .abs()
                    * 0.5;
                let rendered_area: f32 = member.masks[name]
                    .iter()
                    .map(|t| cross(t[0], t[1], t[2]).abs() * 0.5)
                    .sum();
                assert!(
                    (rendered_area - source_area).abs() < 0.5,
                    "{} {name} repeats or loses source pixels",
                    member.piece
                );
            }
            for age in (30..390).step_by(2) {
                let sample = Nightlife::sample(100 + age, Some(age));
                let person = sample.people.iter().find(|p| p.id == 10).unwrap();
                let deform = Deform::new(person, member);
                for back in [false, true] {
                    let chain = deform.arm_chain(back);
                    assert!(
                        chain.target[1][1]
                            > chain.target[0][1]
                                + distance(chain.source[0], chain.source[1]) * deform.units * 0.7,
                        "{} raises running elbow at {age}",
                        member.piece
                    );
                    for i in 0..2 {
                        assert!(
                            (distance(chain.target[i], chain.target[i + 1])
                                - distance(chain.source[i], chain.source[i + 1]) * deform.units)
                                .abs()
                                < 0.001
                        );
                    }
                    assert!(
                        deform
                            .arm(back)
                            .0
                            .iter()
                            .flatten()
                            .flat_map(|v| v.target)
                            .all(f32::is_finite)
                    );
                }
            }
        }
    }

    #[test]
    fn coarse_body_triangles_preserve_the_affine_paint_deformation() {
        use crate::adventure::augusta::ambient::Nightlife;
        let root =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("assets/adventure/chapters/cpp-augusta");
        let art: super::super::assets::WorldArt =
            serde_json::from_str(&fs::read_to_string(root.join("world-art.json")).unwrap())
                .unwrap();
        let catalog = CastCatalog::load(&root.join(&art.nightlife_cast), &art.pieces).unwrap();
        let mut fine_count = 0;
        let mut coarse_count = 0;
        for wardrobe in [
            Wardrobe::Leather,
            Wardrobe::PlumDress,
            Wardrobe::AmberJacket,
            Wardrobe::Denim,
            Wardrobe::TealDress,
            Wardrobe::WhiteShirt,
            Wardrobe::RedBlouse,
            Wardrobe::LongCoat,
        ] {
            let key = wardrobe_key(wardrobe);
            for member in [&catalog.entries[key], &catalog.profile_entries[key]] {
                fine_count += refine(triangulate(&member.body_polygon).unwrap()).len();
                coarse_count += member.body_triangles.len();
                for tick in [0, 21, 84, 147] {
                    let calm = Nightlife::sample(240 + tick, None);
                    let running = Nightlife::sample(130 + tick, Some(30 + tick));
                    for mut person in [
                        calm.people[0],
                        *running.people.iter().find(|p| p.id == 10).unwrap(),
                    ] {
                        person.wardrobe = wardrobe;
                        let deform = Deform::new(&person, member);
                        let back = deform.leg_chain(true);
                        let front = deform.leg_chain(false);
                        let target = |point| deform.cloth(point, &back, &front);
                        for triangle in &member.body_triangles {
                            if (0..3).all(|i| distance(triangle[i], triangle[(i + 1) % 3]) <= 22.0)
                            {
                                continue;
                            }
                            let center =
                                scale(add(add(triangle[0], triangle[1]), triangle[2]), 1.0 / 3.0);
                            let expected = scale(
                                add(
                                    add(target(triangle[0]), target(triangle[1])),
                                    target(triangle[2]),
                                ),
                                1.0 / 3.0,
                            );
                            assert!(
                                distance(target(center), expected) < 0.002,
                                "{} coarse body distorts paint at {tick}",
                                member.piece
                            );
                            for i in 0..3 {
                                let a = triangle[i];
                                let b = triangle[(i + 1) % 3];
                                assert!(
                                    distance(
                                        target(mix(a, b, 0.5)),
                                        mix(target(a), target(b), 0.5)
                                    ) < 0.002
                                );
                            }
                        }
                    }
                }
            }
        }
        assert!(
            coarse_count * 2 < fine_count,
            "body refinement must save meaningful work"
        );
    }

    #[test]
    fn concave_torso_mask_keeps_cutouts_and_rejects_collapsed_polygons() {
        let polygon = [[0., 0.], [8., 0.], [8., 8.], [4., 4.], [0., 8.]];
        let triangles = triangulate(&polygon).unwrap();
        let area: f32 = triangles
            .iter()
            .map(|t| cross(t[0], t[1], t[2]) * 0.5)
            .sum();
        assert!((area - 48.0).abs() < 0.001);
        assert!(triangulate(&[[0., 0.], [1., 0.], [2., 0.]]).is_err());
    }

    #[test]
    fn painted_limb_lengths_hold_through_reachable_gait_and_guard_targets() {
        for target in [[8., 40.], [-20., 25.], [25., 10.], [0., 100.]] {
            let chain = joint([0., 0.], target, 24., 22., -1.);
            assert!((distance(chain[0], chain[1]) - 24.).abs() < 0.001);
            assert!((distance(chain[1], chain[2]) - 22.).abs() < 0.001);
            assert!(chain.into_iter().flatten().all(f32::is_finite));
        }
    }
}
