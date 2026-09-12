//! Evaluates authored poses, view changes and continuous cloth outside the renderer.
//!
//! System: Adventure production. The lab and campaign share these versioned rig
//! and clip contracts; neither texture pixels nor frame rate decide combat hits.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub type Point = [f32; 2];

/// A reusable image with its explicit source-pixel pivot and world dimensions.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Attachment {
    pub image: String,
    pub source: [f32; 4],
    pub anchor: Point,
    pub size: Point,
}

/// Limb dimensions and image landmarks, independent from any particular clip.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Skeleton {
    pub hips: [Point; 2],
    pub shoulders: [Point; 2],
    pub thigh: f32,
    pub shin: f32,
    pub upper_arm: f32,
    pub forearm: f32,
    pub ankle_height: f32,
    pub leg_width: f32,
    /// UV centers and heights of hip, knee and ankle in the continuous leg image.
    pub leg_centers: [f32; 3],
    pub leg_joints: [f32; 3],
    pub knee_blend: f32,
}

/// A painted camera-facing view. Angle zero is profile looking forward.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct View {
    pub angle: f32,
    pub body: String,
    pub leg: String,
    pub boot: String,
    pub flip: bool,
    /// Optional per-view registration, in world pixels from the body pivot.
    pub shoulders: Option<[Point; 2]>,
    pub hips: Option<[Point; 2]>,
    pub bag: Option<Point>,
    /// Local Y interval where the painted hip transitions to the thigh bones.
    pub body_leg_blend: Option<Point>,
    /// Visible half-width of thigh paint in the leg image's normalized U.
    pub leg_alpha_half_width: Option<f32>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Method {
    Skeletal,
    Frames,
}

/// A reusable Linker/projectile appearance, optionally backed by a painted sprite.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectileArt {
    pub color: [u8; 4],
    pub core: [u8; 4],
    pub trail_length: f32,
    pub trail_width: f32,
    pub radius: f32,
    pub sides: u32,
    pub spin: f32,
    pub sprite: Option<String>,
}
impl ProjectileArt {
    pub fn validate(&self) -> Result<(), String> {
        if !finite([self.trail_length, self.trail_width, self.radius, self.spin])
            || !(0.0..=500.0).contains(&self.trail_length)
            || !(0.1..=100.0).contains(&self.trail_width)
            || !(0.1..=200.0).contains(&self.radius)
            || !(3..=32).contains(&self.sides)
        {
            return Err("invalid projectile appearance".into());
        }
        Ok(())
    }
}

/// Two supported authoring strategies under the same loading/review contract.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Rig {
    pub schema_version: u32,
    pub character_id: String,
    pub method: Method,
    pub height: f32,
    pub attachments: BTreeMap<String, Attachment>,
    pub skeleton: Option<Skeleton>,
    #[serde(default)]
    pub views: Vec<View>,
    #[serde(default)]
    pub effects: BTreeMap<String, ProjectileArt>,
}

fn finite(values: impl IntoIterator<Item = f32>) -> bool {
    values
        .into_iter()
        .all(|v| v.is_finite() && v.abs() <= 32000.0)
}

pub fn safe_relative(value: &str) -> bool {
    !value.is_empty()
        && !value.starts_with('/')
        && !value.contains(['\\', ':'])
        && value
            .split('/')
            .all(|p| !p.is_empty() && p != ".." && p != ".")
}

impl Rig {
    pub fn from_json(source: &str) -> Result<Self, String> {
        let rig: Self = serde_json::from_str(source).map_err(|e| e.to_string())?;
        rig.validate()?;
        Ok(rig)
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.schema_version != 1
            || self.character_id.is_empty()
            || !(24.0..=1000.0).contains(&self.height)
            || self.attachments.is_empty()
        {
            return Err("invalid actor rig version, identity or dimensions".into());
        }
        for (id, a) in &self.attachments {
            if id.is_empty()
                || !safe_relative(&a.image)
                || !finite(a.source.into_iter().chain(a.anchor).chain(a.size))
                || a.source[0] < 0.0
                || a.source[1] < 0.0
                || a.source[2..].iter().chain(a.size.iter()).any(|v| *v <= 0.0)
                || a.anchor[0] < 0.0
                || a.anchor[0] > a.source[2]
                || a.anchor[1] < 0.0
                || a.anchor[1] > a.source[3]
            {
                return Err(format!("invalid attachment {id}"));
            }
        }
        for (id, effect) in &self.effects {
            if id.is_empty() {
                return Err("empty projectile effect identity".into());
            }
            effect.validate()?;
        }
        if self.method == Method::Skeletal {
            let s = self
                .skeleton
                .as_ref()
                .ok_or("skeletal rig has no skeleton")?;
            if !finite(
                s.hips
                    .into_iter()
                    .flatten()
                    .chain(s.shoulders.into_iter().flatten()),
            ) || [
                s.thigh,
                s.shin,
                s.upper_arm,
                s.forearm,
                s.ankle_height,
                s.leg_width,
                s.knee_blend,
            ]
            .iter()
            // World-pixel dimensions must exceed the IK singularity margin;
            // accepting arbitrarily tiny bones can invert solve_chain's clamp.
            .any(|v| !(1.0..=300.0).contains(v))
                || s.leg_centers
                    .iter()
                    .chain(&s.leg_joints)
                    .any(|v| !(0.0..=1.0).contains(v))
                || !(s.leg_joints[0] < s.leg_joints[1] && s.leg_joints[1] < s.leg_joints[2])
                || self.views.is_empty()
            {
                return Err("invalid skeleton dimensions or cloth landmarks".into());
            }
            for id in ["upper-arm", "forearm", "bag"] {
                if !self.attachments.contains_key(id) {
                    return Err(format!("missing attachment {id}"));
                }
            }
            for view in &self.views {
                if !view.angle.is_finite()
                    || !(0.0..360.0).contains(&view.angle)
                    || !finite(
                        view.shoulders
                            .into_iter()
                            .flatten()
                            .flatten()
                            .chain(view.hips.into_iter().flatten().flatten())
                            .chain(view.bag.into_iter().flatten()),
                    )
                    || [&view.body, &view.leg, &view.boot]
                        .iter()
                        .any(|id| !self.attachments.contains_key(*id))
                    || view.body_leg_blend.is_some_and(|[start, end]| {
                        !finite([start, end]) || start >= end || start < -300.0 || end > 300.0
                    })
                    || view.leg_alpha_half_width.is_some_and(|radius| {
                        !radius.is_finite() || !(0.05..=0.5).contains(&radius)
                    })
                {
                    return Err("invalid painted view".into());
                }
            }
        }
        Ok(())
    }

    pub fn view(&self, angle: f32) -> Option<&View> {
        self.views.iter().min_by(|a, b| {
            angular_delta(angle, a.angle)
                .abs()
                .total_cmp(&angular_delta(angle, b.angle).abs())
        })
    }
}

/// End-effector pose in local world pixels, relative to the actor's feet origin.
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Pose {
    pub pelvis: Point,
    pub body_angle: f32,
    /// Unwrapped angle permits a genuine authored 0→360 turn through painted views.
    pub yaw: f32,
    pub feet: [Point; 2],
    pub hands: [Point; 2],
    pub foot_angles: [f32; 2],
    pub bag_angle: f32,
}

impl Pose {
    pub fn blend(self, b: Self, amount: f32) -> Self {
        let t = amount.clamp(0.0, 1.0);
        Self {
            pelvis: lerp(self.pelvis, b.pelvis, t),
            body_angle: mix(self.body_angle, b.body_angle, t),
            yaw: mix(self.yaw, b.yaw, t),
            feet: [
                lerp(self.feet[0], b.feet[0], t),
                lerp(self.feet[1], b.feet[1], t),
            ],
            hands: [
                lerp(self.hands[0], b.hands[0], t),
                lerp(self.hands[1], b.hands[1], t),
            ],
            foot_angles: [
                mix(self.foot_angles[0], b.foot_angles[0], t),
                mix(self.foot_angles[1], b.foot_angles[1], t),
            ],
            bag_angle: mix(self.bag_angle, b.bag_angle, t),
        }
    }

    fn valid(self) -> bool {
        finite(
            self.pelvis
                .into_iter()
                .chain(self.feet.into_iter().flatten())
                .chain(self.hands.into_iter().flatten())
                .chain(self.foot_angles)
                .chain([self.body_angle, self.yaw, self.bag_angle]),
        )
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Curve {
    Linear,
    Smooth,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Key {
    pub time: f32,
    pub pose: Pose,
    pub curve: Curve,
    pub frame: Option<String>,
}

/// Ordinary poses cross zero by the shortest arc; authored spins retain turns.
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AngleMode {
    #[default]
    Shortest,
    Unwrapped,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Clip {
    pub duration_ticks: u32,
    pub looping: bool,
    pub blend_ticks: u32,
    /// Some stride length means distance, rather than wall clock, advances this clip.
    pub stride_pixels: Option<f32>,
    #[serde(default)]
    pub yaw_interpolation: AngleMode,
    pub keys: Vec<Key>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Clips {
    pub schema_version: u32,
    pub tick_hz: u32,
    pub clips: BTreeMap<String, Clip>,
}

pub struct Sample<'a> {
    pub pose: Pose,
    pub frame: Option<&'a str>,
}

impl Clips {
    pub fn from_json(source: &str, rig: &Rig) -> Result<Self, String> {
        let clips: Self = serde_json::from_str(source).map_err(|e| e.to_string())?;
        clips.validate(rig)?;
        Ok(clips)
    }
    pub fn validate(&self, rig: &Rig) -> Result<(), String> {
        if self.schema_version != 1 || self.tick_hz != 60 || !self.clips.contains_key("idle") {
            return Err("invalid clip version/frequency or missing idle".into());
        }
        for (id, c) in &self.clips {
            if id.is_empty()
                || c.duration_ticks == 0
                || c.duration_ticks > 36000
                || c.blend_ticks > 60
                || c.keys.len() < 2
                || c.keys.len() > 4096
                || c.keys[0].time != 0.0
                || c.keys.last().is_none_or(|k| k.time != 1.0)
                || c.stride_pixels.is_some_and(|s| !s.is_finite() || s <= 0.0)
                || (rig.method == Method::Frames && c.stride_pixels.is_some())
                || c.keys.windows(2).any(|p| p[0].time >= p[1].time)
                || c.keys.iter().any(|k| {
                    !(0.0..=1.0).contains(&k.time)
                        || !k.pose.valid()
                        || k.frame
                            .as_ref()
                            .is_some_and(|f| !rig.attachments.contains_key(f))
                        || (rig.method == Method::Frames && k.frame.is_none())
                })
            {
                return Err(format!("invalid animation clip {id}"));
            }
        }
        Ok(())
    }
    pub fn sample(&self, id: &str, ticks: f32, distance: f32) -> Sample<'_> {
        let c = &self.clips[id];
        let raw = c
            .stride_pixels
            .map_or(ticks / c.duration_ticks as f32, |s| distance / s);
        let t = if c.looping {
            raw.rem_euclid(1.0)
        } else {
            raw.clamp(0.0, 1.0)
        };
        let idx = c
            .keys
            .partition_point(|k| k.time <= t)
            .saturating_sub(1)
            .min(c.keys.len() - 2);
        let (a, b) = (&c.keys[idx], &c.keys[idx + 1]);
        let mut f = ((t - a.time) / (b.time - a.time)).clamp(0.0, 1.0);
        if matches!(a.curve, Curve::Smooth) {
            f = f * f * (3.0 - 2.0 * f);
        }
        let mut pose = a.pose.blend(b.pose, f);
        if matches!(c.yaw_interpolation, AngleMode::Shortest) {
            pose.yaw = a.pose.yaw + angular_delta(a.pose.yaw, b.pose.yaw) * f;
        }
        Sample {
            pose,
            frame: if t >= 1.0 {
                b.frame.as_deref()
            } else {
                a.frame.as_deref()
            },
        }
    }
}

/// Small stateful crossfade; the source is captured only when the state changes.
#[derive(Clone, Debug)]
pub struct Animator {
    pub pose: Pose,
    pub clip: String,
    from: Pose,
    elapsed: f32,
}
impl Animator {
    pub fn new(book: &Clips) -> Self {
        let pose = book.sample("idle", 0.0, 0.0).pose;
        Self {
            pose,
            from: pose,
            clip: "idle".into(),
            elapsed: 60.0,
        }
    }
    pub fn tick(&mut self, book: &Clips, clip: &str, ticks: f32, distance: f32) {
        if self.clip != clip {
            self.from = self.pose;
            self.clip = clip.into();
            self.elapsed = 0.0;
        }
        let mut target = book.sample(clip, ticks, distance).pose;
        let duration = book.clips[clip].blend_ticks as f32;
        if self.elapsed < duration {
            target.yaw = self.from.yaw + angular_delta(self.from.yaw, target.yaw);
            let t = (self.elapsed / duration).clamp(0.0, 1.0);
            self.pose = self.from.blend(target, t * t * (3.0 - 2.0 * t));
        } else {
            self.pose = target;
        }
        self.elapsed += 1.0;
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Chain {
    pub root: Point,
    pub joint: Point,
    pub tip: Point,
}

/// Two-bone inverse kinematics retains lengths, including at unreachable targets.
pub fn solve_chain(root: Point, target: Point, upper: f32, lower: f32, bend: f32) -> Chain {
    let delta = sub(target, root);
    let distance = length(delta).clamp((upper - lower).abs() + 0.001, upper + lower - 0.001);
    let direction = if length(delta) > 0.0001 {
        mul(delta, 1.0 / length(delta))
    } else {
        [0.0, 1.0]
    };
    let along = (upper * upper + distance * distance - lower * lower) / (2.0 * distance);
    let height = (upper * upper - along * along).max(0.0).sqrt() * bend.signum();
    Chain {
        root,
        joint: add(
            root,
            add(
                mul(direction, along),
                mul([-direction[1], direction[0]], height),
            ),
        ),
        tip: add(root, mul(direction, distance)),
    }
}

/// Elbow pole points down the body instead of flipping above the shoulder when
/// the wrist passes behind it. Both branches preserve the same limb lengths.
pub fn solve_arm(root: Point, target: Point, upper: f32, lower: f32) -> Chain {
    let a = solve_chain(root, target, upper, lower, 1.0);
    let b = solve_chain(root, target, upper, lower, -1.0);
    if a.joint[1] >= b.joint[1] { a } else { b }
}

#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    pub position: Point,
    pub uv: Point,
    pub opacity: f32,
}

/// Keeps the painted upper body rigid while its hip fabric follows the thighs.
pub fn body_mesh(
    sprite: &Attachment,
    pose: Pose,
    view: &View,
    hips: [Point; 2],
    legs: [Chain; 2],
    leg_width: f32,
) -> Vec<[Vertex; 3]> {
    const ROWS: usize = 40;
    const COLS: usize = 16;
    let flip = if view.flip { -1.0 } else { 1.0 };
    let [start, end] = view.body_leg_blend.expect("weighted body view");
    let hip_x = hips.map(|h| h[0] * flip);
    let left = usize::from(hip_x[0] > hip_x[1]);
    let right = 1 - left;
    let mut grid = Vec::with_capacity((ROWS + 1) * (COLS + 1));
    for row in 0..=ROWS {
        let v = row as f32 / ROWS as f32;
        for col in 0..=COLS {
            let u = col as f32 / COLS as f32;
            let local = [
                (u - sprite.anchor[0] / sprite.source[2]) * sprite.size[0] * flip,
                (v - sprite.anchor[1] / sprite.source[3]) * sprite.size[1],
            ];
            let rigid = add(pose.pelvis, rotate(local, pose.body_angle));
            let weight = ((local[1] - start) / (end - start)).clamp(0.0, 1.0);
            let weight = weight * weight * (3.0 - 2.0 * weight);
            let across = ((local[0] - hip_x[left]) / (hip_x[right] - hip_x[left]).max(0.001))
                .clamp(0.0, 1.0);
            let deformed = legs.map(|leg| {
                let turn = angle_from_down(sub(leg.joint, leg.root)) - pose.body_angle;
                add(leg.root, rotate(sub(rigid, leg.root), turn))
            });
            let position = lerp(rigid, lerp(deformed[left], deformed[right], across), weight);
            // Blend static painted thigh ends into the articulated silhouettes.
            let distance = legs.map(|leg| {
                let axis = sub(leg.joint, leg.root);
                let offset = sub(position, leg.root);
                let along = ((offset[0] * axis[0] + offset[1] * axis[1])
                    / (axis[0] * axis[0] + axis[1] * axis[1]).max(0.001))
                .clamp(0.0, 1.0);
                length(sub(position, add(leg.root, mul(axis, along))))
            });
            let radius = leg_width * view.leg_alpha_half_width.unwrap_or(0.5);
            let coverage = ((radius + 1.0 - distance[0].min(distance[1])) / 2.0).clamp(0.0, 1.0);
            let hip_y = (hips[0][1] + hips[1][1]) * 0.5;
            let mask = ((local[1] - hip_y + 6.0) / 6.0).clamp(0.0, 1.0);
            let mask = mask * mask * (3.0 - 2.0 * mask);
            grid.push(Vertex {
                position,
                uv: [u, v],
                opacity: mix(1.0, coverage, mask),
            });
        }
    }
    let mut triangles = Vec::with_capacity(ROWS * COLS * 2);
    for row in 0..ROWS {
        for col in 0..COLS {
            let a = row * (COLS + 1) + col;
            let b = a + 1;
            let c = a + COLS + 1;
            let d = c + 1;
            triangles.push([grid[a], grid[b], grid[c]]);
            triangles.push([grid[b], grid[d], grid[c]]);
        }
    }
    triangles
}

/// Skin a continuous trouser leg, normalizing the blended normal to retain volume.
pub fn leg_mesh(s: &Skeleton, chain: Chain) -> Vec<[Vertex; 3]> {
    const ROWS: usize = 28;
    const COLS: usize = 4;
    let upper = unit(sub(chain.joint, chain.root));
    let lower = unit(sub(chain.tip, chain.joint));
    let mut grid = Vec::with_capacity((ROWS + 1) * (COLS + 1));
    for row in 0..=ROWS {
        let v = row as f32 / ROWS as f32;
        let (y, center) = if v <= s.leg_joints[1] {
            let t = (v - s.leg_joints[0]) / (s.leg_joints[1] - s.leg_joints[0]);
            (t * s.thigh, mix(s.leg_centers[0], s.leg_centers[1], t))
        } else {
            let t = (v - s.leg_joints[1]) / (s.leg_joints[2] - s.leg_joints[1]);
            (
                s.thigh + t * s.shin,
                mix(s.leg_centers[1], s.leg_centers[2], t),
            )
        };
        let t = ((y - s.thigh + s.knee_blend) / (2.0 * s.knee_blend)).clamp(0.0, 1.0);
        let w = t * t * (3.0 - 2.0 * t);
        let centerline = lerp(
            add(chain.root, mul(upper, y)),
            add(chain.joint, mul(lower, y - s.thigh)),
            w,
        );
        let normal = unit(lerp([upper[1], -upper[0]], [lower[1], -lower[0]], w));
        for col in 0..=COLS {
            let u = col as f32 / COLS as f32;
            grid.push(Vertex {
                position: add(centerline, mul(normal, (u - center) * s.leg_width)),
                uv: [u, v],
                opacity: 1.0,
            });
        }
    }
    let mut triangles = Vec::with_capacity(ROWS * COLS * 2);
    for row in 0..ROWS {
        for col in 0..COLS {
            let a = row * (COLS + 1) + col;
            let b = a + 1;
            let c = a + COLS + 1;
            let d = c + 1;
            triangles.push([grid[a], grid[b], grid[c]]);
            triangles.push([grid[b], grid[d], grid[c]]);
        }
    }
    triangles
}

pub fn add(a: Point, b: Point) -> Point {
    [a[0] + b[0], a[1] + b[1]]
}
pub fn sub(a: Point, b: Point) -> Point {
    [a[0] - b[0], a[1] - b[1]]
}
pub fn mul(a: Point, s: f32) -> Point {
    [a[0] * s, a[1] * s]
}
pub fn length(a: Point) -> f32 {
    a[0].hypot(a[1])
}
pub fn unit(a: Point) -> Point {
    let l = length(a);
    if l < 0.00001 {
        [0.0, 1.0]
    } else {
        mul(a, 1.0 / l)
    }
}
pub fn rotate(a: Point, degrees: f32) -> Point {
    let (s, c) = degrees.to_radians().sin_cos();
    [a[0] * c - a[1] * s, a[0] * s + a[1] * c]
}
pub fn angle_from_down(a: Point) -> f32 {
    (-a[0]).atan2(a[1]).to_degrees()
}
pub fn angular_delta(a: f32, b: f32) -> f32 {
    (b - a + 180.0).rem_euclid(360.0) - 180.0
}
fn mix(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}
fn lerp(a: Point, b: Point, t: f32) -> Point {
    [mix(a[0], b[0], t), mix(a[1], b[1], t)]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cpp_animation() -> (Rig, Clips) {
        let rig = Rig::from_json(include_str!(
            "../../../assets/adventure/actors/cpp/rig.json"
        ))
        .unwrap();
        let clips = Clips::from_json(
            include_str!("../../../assets/adventure/actors/cpp/clips.json"),
            &rig,
        )
        .unwrap();
        (rig, clips)
    }

    #[test]
    fn hip_fabric_moves_without_deforming_the_upper_body_or_losing_vertices() {
        let (rig, clips) = cpp_animation();
        let skeleton = rig.skeleton.as_ref().unwrap();
        let mut moved = 0;
        for id in ["spin", "run", "kick"] {
            let clip = &clips.clips[id];
            for step in 0..=24 {
                let phase = step as f32 / 24.0;
                let pose = clips
                    .sample(
                        id,
                        phase * clip.duration_ticks as f32,
                        phase * clip.stride_pixels.unwrap_or(0.0),
                    )
                    .pose;
                let view = rig.view(pose.yaw).unwrap();
                let sprite = &rig.attachments[&view.body];
                let hips = view.hips.unwrap_or(skeleton.hips);
                let sign = if view.flip { -1.0 } else { 1.0 };
                let legs = std::array::from_fn(|i| {
                    solve_chain(
                        add(
                            pose.pelvis,
                            rotate([hips[i][0] * sign, hips[i][1]], pose.body_angle),
                        ),
                        add(pose.feet[i], [0.0, -skeleton.ankle_height]),
                        skeleton.thigh,
                        skeleton.shin,
                        -1.0,
                    )
                });
                for vertex in body_mesh(sprite, pose, view, hips, legs, skeleton.leg_width)
                    .into_iter()
                    .flatten()
                {
                    assert!(finite(vertex.position));
                    assert!((0.0..=1.0).contains(&vertex.opacity));
                    let local = [
                        (vertex.uv[0] - sprite.anchor[0] / sprite.source[2])
                            * sprite.size[0]
                            * sign,
                        (vertex.uv[1] - sprite.anchor[1] / sprite.source[3]) * sprite.size[1],
                    ];
                    let rigid = add(pose.pelvis, rotate(local, pose.body_angle));
                    let delta = length(sub(vertex.position, rigid));
                    if local[1] <= view.body_leg_blend.unwrap()[0] {
                        assert!(delta < 0.001, "upper body changed in {id} at {phase}");
                    } else if delta > 0.1 {
                        moved += 1;
                    }
                }
            }
        }
        assert!(moved > 0, "the hip fabric must follow moving thighs");
        let mut invalid = rig.clone();
        invalid.views[0].body_leg_blend = Some([12.0, -10.0]);
        assert!(invalid.validate().is_err());
    }

    /// Contact must survive interpolation between the authored keys, not only
    /// the key poses. A clamped leg can otherwise leave its supporting boot afloat.
    #[test]
    fn authored_run_keeps_both_legs_reachable_and_planted_soles_on_the_floor() {
        let (rig, clips) = cpp_animation();
        let skeleton = rig.skeleton.as_ref().unwrap();
        let clip = &clips.clips["run"];
        let stride = clip.stride_pixels.expect("run is distance driven");
        let mut contacts = [0_u32; 2];
        for sample in 0..=4096 {
            let phase = sample as f32 / 4096.0;
            let pose = clips
                .sample("run", phase * clip.duration_ticks as f32, phase * stride)
                .pose;
            let view = rig.view(pose.yaw).unwrap();
            let hips = view.hips.unwrap_or(skeleton.hips);
            let view_sign = if view.flip { -1.0 } else { 1.0 };
            for leg in 0..2 {
                let root = add(
                    pose.pelvis,
                    rotate([hips[leg][0] * view_sign, hips[leg][1]], pose.body_angle),
                );
                let ankle = add(pose.feet[leg], [0.0, -skeleton.ankle_height]);
                let reach = length(sub(ankle, root));
                assert!(
                    reach <= skeleton.thigh + skeleton.shin + 0.001,
                    "unreachable leg {leg} at {phase}: {reach}"
                );
                let solved = solve_chain(root, ankle, skeleton.thigh, skeleton.shin, -1.0);
                assert!(
                    length(sub(solved.tip, ankle)) < 0.02,
                    "ankle was clamped at phase {phase}, leg {leg}"
                );
                let sole_y = solved.tip[1] + skeleton.ankle_height;
                assert!(
                    sole_y <= 0.02,
                    "sole penetrates floor at phase {phase}, leg {leg}: {sole_y}"
                );
                if pose.feet[leg][1].abs() < 0.0001 {
                    contacts[leg] += 1;
                    assert!(
                        sole_y.abs() < 0.02,
                        "support floats at phase {phase}, leg {leg}: {sole_y}"
                    );
                }
            }
        }
        assert!(
            contacts.iter().all(|count| *count > 1000),
            "both legs need a sustained support phase: {contacts:?}"
        );
    }

    /// The reported upper-arm inversion and the modulo-360 reversal are separate
    /// visual failures in the same authored motion package.
    #[test]
    fn run_elbows_stay_below_shoulders_and_spin_yaw_remains_unwrapped() {
        let (rig, clips) = cpp_animation();
        let skeleton = rig.skeleton.as_ref().unwrap();
        let run = &clips.clips["run"];
        for phase in [0.545, 0.636, 0.640, 0.727, 0.818, 0.909] {
            let pose = clips
                .sample(
                    "run",
                    phase * run.duration_ticks as f32,
                    phase * run.stride_pixels.unwrap(),
                )
                .pose;
            let view = rig.view(pose.yaw).unwrap();
            let shoulders = view.shoulders.unwrap_or(skeleton.shoulders);
            let sign = if view.flip { -1.0 } else { 1.0 };
            for (arm, shoulder) in shoulders.into_iter().enumerate() {
                let root = add(
                    pose.pelvis,
                    rotate([shoulder[0] * sign, shoulder[1]], pose.body_angle),
                );
                let solved = solve_arm(root, pose.hands[arm], skeleton.upper_arm, skeleton.forearm);
                assert!(
                    solved.joint[1] >= root[1] - 0.02,
                    "elbow rises above shoulder at {phase}, arm {arm}"
                );
                assert!(length(sub(solved.tip, pose.hands[arm])) < 0.02);
            }
        }
        let mut previous_elbows = [None; 2];
        for sample in 0..=4096 {
            let phase = sample as f32 / 4096.0;
            let pose = clips
                .sample(
                    "run",
                    phase * run.duration_ticks as f32,
                    phase * run.stride_pixels.unwrap(),
                )
                .pose;
            let view = rig.view(pose.yaw).unwrap();
            let sign = if view.flip { -1.0 } else { 1.0 };
            for (arm, shoulder) in view
                .shoulders
                .unwrap_or(skeleton.shoulders)
                .into_iter()
                .enumerate()
            {
                let root = add(
                    pose.pelvis,
                    rotate([shoulder[0] * sign, shoulder[1]], pose.body_angle),
                );
                let elbow =
                    solve_arm(root, pose.hands[arm], skeleton.upper_arm, skeleton.forearm).joint;
                if let Some(previous) = previous_elbows[arm] {
                    let distance = length(sub(elbow, previous));
                    assert!(
                        distance < 1.0,
                        "elbow branch jumps {distance}px at phase {phase}, arm {arm}"
                    );
                }
                previous_elbows[arm] = Some(elbow);
            }
        }
        let spin = &clips.clips["spin"];
        let initial = clips.sample("spin", 0.0, 0.0).pose.yaw;
        let mut previous = initial;
        for sample in 1..=4096 {
            let yaw = clips
                .sample(
                    "spin",
                    sample as f32 / 4096.0 * spin.duration_ticks as f32,
                    0.0,
                )
                .pose
                .yaw;
            assert!(
                yaw >= previous - 0.001,
                "spin reverses from {previous} to {yaw} at sample {sample}"
            );
            previous = yaw;
        }
        assert!(
            previous - initial >= 360.0,
            "spin must complete a full authored turn"
        );
        // Ordinary profile transitions must take the authored short arc. A raw
        // 330→0 blend otherwise turns a right-facing attack away from its target.
        for id in ["start", "stop", "light-1", "kick", "linker"] {
            let clip = &clips.clips[id];
            for sample in 0..=1024 {
                let phase = sample as f32 / 1024.0;
                let yaw = clips
                    .sample(id, phase * clip.duration_ticks as f32, 0.0)
                    .pose
                    .yaw;
                assert!(
                    angular_delta(0.0, yaw).abs() <= 90.0,
                    "{id} turns away from its target at phase {phase}: yaw {yaw}"
                );
            }
        }
    }

    #[test]
    fn ik_keeps_both_lengths_including_unreachable_targets() {
        for target in [[20.0, 60.0], [0.0, 0.0], [300.0, -300.0], [-48.0, -12.0]] {
            let c = solve_chain([0.0, 0.0], target, 48.0, 43.0, -1.0);
            assert!((length(sub(c.joint, c.root)) - 48.0).abs() < 0.01);
            assert!((length(sub(c.tip, c.joint)) - 43.0).abs() < 0.01);
        }
    }
    #[test]
    fn resource_paths_cannot_escape_pack() {
        for path in [
            "../other.png",
            "/tmp/x.png",
            "C:/x.png",
            "x\\y.png",
            "x//y.png",
        ] {
            assert!(!safe_relative(path));
        }
        assert!(safe_relative("sprites/boot-profile.png"));
    }
    #[test]
    fn view_angle_wrap_keeps_full_turn_continuity() {
        assert_eq!(angular_delta(359.0, 1.0), 2.0);
        assert_eq!(angular_delta(1.0, 359.0), -2.0);
    }
}
