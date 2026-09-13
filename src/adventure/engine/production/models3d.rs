//! Loads authored GLB actors and vehicles into Augusta's existing presentation.
//!
//! System: Adventure Raylib boundary. Models share the chapter's positions and
//! fixed animation clock; their bones never determine damage or story progress.

use crate::adventure::{
    augusta::ambient::{Activity, Pedestrian, Vehicle, VehicleKind, Wardrobe},
    production::animation::safe_relative,
};
use raylib::prelude::*;
use serde::Deserialize;
use std::{
    cell::RefCell,
    collections::BTreeMap,
    error::Error,
    fs,
    path::{Path, PathBuf},
};

const VERTEX: &str = r#"#version 330
in vec3 vertexPosition;
in vec2 vertexTexCoord;
in vec3 vertexNormal;
in vec4 vertexColor;
uniform mat4 mvp;
uniform mat4 matNormal;
out vec2 uv;
out vec3 normal;
out vec4 color;
void main() {
    uv = vertexTexCoord;
    normal = normalize((matNormal * vec4(vertexNormal, 0.0)).xyz);
    color = vertexColor;
    gl_Position = mvp * vec4(vertexPosition, 1.0);
}
"#;
const FRAGMENT: &str = r#"#version 330
in vec2 uv;
in vec3 normal;
in vec4 color;
uniform sampler2D texture0;
uniform vec4 colDiffuse;
out vec4 finalColor;
void main() {
    vec4 texel = texture(texture0, uv);
    // glTF factors are linear; embedded base-color PNGs are sRGB.
    vec3 linearTexel = mix(texel.rgb / 12.92,
        pow((texel.rgb + 0.055) / 1.055, vec3(2.4)), step(vec3(0.04045), texel.rgb));
    vec4 paint = vec4(linearTexel, texel.a) * color * colDiffuse;
    if (paint.a < 0.08) discard;
    vec3 n = normalize(normal);
    float key = max(dot(n, normalize(vec3(-0.45, 0.8, 0.75))), 0.0);
    float rim = max(dot(n, normalize(vec3(0.7, 0.4, -0.8))), 0.0);
    vec3 light = vec3(0.40, 0.43, 0.51) + key * vec3(0.67, 0.57, 0.42)
                 + rim * vec3(0.10, 0.21, 0.28);
    vec3 linearColor = max(paint.rgb * light, vec3(0.0));
    vec3 srgb = mix(linearColor * 12.92,
        1.055 * pow(linearColor, vec3(1.0 / 2.4)) - 0.055,
        step(vec3(0.0031308), linearColor));
    finalColor = vec4(srgb, paint.a);
}
"#;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Entry {
    pub file: String,
    pub height_m: f32,
    pub animation_fps: f32,
    pub clips: BTreeMap<String, String>,
    #[serde(default)]
    pub strides_m: BTreeMap<String, f32>,
    #[serde(default)]
    pub length_m: Option<f32>,
    #[serde(default)]
    pub paint_material: Option<usize>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Catalog {
    pub schema_version: u32,
    pub entries: BTreeMap<String, Entry>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CatalogPaths {
    pub humans: String,
    pub vehicles: Option<String>,
}

impl Catalog {
    /// Validates catalog structure and resource containment without a GPU context.
    pub fn load(path: &Path) -> Result<Self, Box<dyn Error>> {
        let root = path.parent().ok_or("3D catalog has no parent")?;
        let catalog = Self::from_json(
            &fs::read_to_string(path)
                .map_err(|error| format!("cannot read 3D catalog {}: {error}", path.display()))?,
        )?;
        for entry in catalog.entries.values() {
            super::assets::contained(root, &entry.file)?;
        }
        Ok(catalog)
    }

    pub fn from_json(source: &str) -> Result<Self, Box<dyn Error>> {
        let result: Self = serde_json::from_str(source)?;
        if result.schema_version != 1 || result.entries.is_empty() {
            return Err("3D model catalog requires schema_version 1 and entries".into());
        }
        for (id, entry) in &result.entries {
            if id.is_empty()
                || !safe_relative(&entry.file)
                || !entry.file.ends_with(".glb")
                || !entry.height_m.is_finite()
                || !(0.3..=4.0).contains(&entry.height_m)
                || entry.animation_fps != 60.0
                || entry
                    .length_m
                    .is_some_and(|n| !n.is_finite() || !(0.3..=8.0).contains(&n))
                || entry
                    .clips
                    .iter()
                    .any(|(id, clip)| id.is_empty() || clip.is_empty())
                || (!matches!(id.as_str(), "hatchback" | "taxi" | "scooter")
                    && !entry.clips.contains_key("idle"))
                || entry.strides_m.iter().any(|(id, n)| {
                    !entry.clips.contains_key(id) || !n.is_finite() || !(0.1..=5.0).contains(n)
                })
            {
                return Err(format!("invalid 3D model registration: {id}").into());
            }
        }
        Ok(result)
    }
}

struct AnimatedModel {
    model: Model,
    animations: Option<ModelAnimations>,
    clips: BTreeMap<String, usize>,
    entry: Entry,
    phone_bone: Option<usize>,
}

/// Models own their buffers; the shared shader outlives every material borrowing it.
pub struct Models3d {
    entries: RefCell<BTreeMap<String, AnimatedModel>>,
    _textures: ImportedTextures,
    _shader: Shader,
    pub resources: Vec<String>,
}

#[derive(Default)]
struct ImportedTextures(BTreeMap<u32, raylib::ffi::Texture2D>);
impl Drop for ImportedTextures {
    fn drop(&mut self) {
        // Raylib's UnloadModel intentionally leaves imported textures to the caller.
        for texture in self.0.values() {
            unsafe {
                raylib::ffi::UnloadTexture(*texture);
            }
        }
    }
}

#[derive(Clone, Copy)]
pub struct ArmContact {
    pub wrist: [f32; 3],
    pub elbow: [f32; 3],
    pub side: &'static str,
    /// Distance from the holder's wrist to its palm contact, in authored meters.
    pub palm_offset: f32,
}

/// A presentation sample in world pixels, independent of wall-clock time.
#[derive(Clone, Copy)]
pub struct DrawModel<'a> {
    pub id: &'a str,
    pub at: [f32; 3],
    pub height: f32,
    pub length: Option<f32>,
    pub yaw: f32,
    pub clip: &'a str,
    pub ticks: f32,
    pub distance: Option<f32>,
    /// When provided, remaps an existing authored clip onto the imported action.
    pub phase: Option<f32>,
    pub looping: bool,
    pub paint: Option<Color>,
    pub contact: Option<ArmContact>,
    pub phone: bool,
}

impl DrawModel<'_> {
    pub fn traveled(mut self, distance: f32) -> Self {
        self.distance = Some(distance);
        self
    }
}

fn name(raw: &[std::ffi::c_char]) -> String {
    let bytes: Vec<_> = raw
        .iter()
        .take_while(|c| **c != 0)
        .map(|c| *c as u8)
        .collect();
    String::from_utf8_lossy(&bytes).into_owned()
}

fn frame(ticks: f32, phase: Option<f32>, count: i32, looping: bool) -> f32 {
    let last = (count - 1).max(0) as f32;
    let value = phase.map_or(ticks, |p| p * last);
    if looping && last > 0. {
        value.rem_euclid(last)
    } else {
        value.clamp(0., last)
    }
}

impl Models3d {
    /// Candidate mode is explicit until the native scene comparison is approved.
    pub fn load_candidate(
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        paths: Option<&CatalogPaths>,
    ) -> Result<Option<Self>, Box<dyn Error>> {
        if std::env::var("BORROW_AUGUSTA_MODELS_3D").as_deref() != Ok("1") {
            return Ok(None);
        }
        let paths = paths.ok_or("3D candidate requires models_3d catalogs in world-art")?;
        Self::load(rl, thread, paths).map(Some)
    }

    pub fn load(
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        paths: &CatalogPaths,
    ) -> Result<Self, Box<dyn Error>> {
        let root = crate::runtime_paths::asset_path("assets/adventure");
        let paths = std::iter::once(&paths.humans)
            .chain(paths.vehicles.iter())
            .map(|path| super::assets::contained(&root, path).map_err(Into::into))
            .collect::<Result<Vec<_>, Box<dyn Error>>>()?;
        Self::load_catalog_files(rl, thread, &paths)
    }

    /// Loads an explicit lab catalog, keeping each GLB inside its package root.
    pub fn load_catalog(
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        path: &Path,
    ) -> Result<Self, Box<dyn Error>> {
        Self::load_catalog_files(rl, thread, &[path.canonicalize()?])
    }

    fn load_catalog_files(
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        catalogs: &[PathBuf],
    ) -> Result<Self, Box<dyn Error>> {
        let mut entries = BTreeMap::new();
        let mut resources = vec![];
        let mut textures = ImportedTextures::default();
        let shader = rl.load_shader_from_memory(thread, Some(VERTEX), Some(FRAGMENT));
        if !shader.is_shader_valid() || shader.get_shader_location("matNormal") < 0 {
            return Err("could not compile Augusta's 3D material lighting shader".into());
        }
        for path in catalogs {
            let root = path.parent().ok_or("3D catalog has no parent")?;
            let catalog = Catalog::load(path)?;
            resources.push(path.display().to_string());
            for (id, entry) in catalog.entries {
                if entries.contains_key(&id) {
                    return Err(format!("duplicate 3D model {id}").into());
                }
                let path = super::assets::contained(root, &entry.file)?;
                let mut model = rl.load_model(thread, &path.to_string_lossy())?;
                let default_texture = unsafe { raylib::ffi::rlGetTextureIdDefault() };
                for material in model.materials() {
                    for map in material.maps() {
                        if map.texture.id != 0 && map.texture.id != default_texture {
                            // Match the painted asset loader's smooth minification;
                            // glTF import otherwise leaves Raylib's point filtering.
                            unsafe {
                                raylib::ffi::SetTextureFilter(
                                    map.texture,
                                    TextureFilter::TEXTURE_FILTER_BILINEAR as i32,
                                );
                            }
                            textures.0.insert(map.texture.id, map.texture);
                        }
                    }
                }
                if entry
                    .paint_material
                    .is_some_and(|i| i >= model.materials().len())
                {
                    return Err(format!("invalid paint material index for {id}").into());
                }
                for material in model.materials_mut() {
                    // Materials hold non-owning shader views; Models3d drops models first.
                    unsafe {
                        material.as_raw_mut().shader = *shader.as_ref();
                    }
                }
                let animations = if entry.clips.is_empty() {
                    None
                } else {
                    Some(rl.load_model_animations(thread, &path.to_string_lossy())?)
                };
                let mut clips = BTreeMap::new();
                if let Some(book) = &animations {
                    if model.meshes().iter().any(|mesh| {
                        !mesh.as_ref().boneWeights.is_null() && mesh.as_ref().animVertices.is_null()
                    }) {
                        return Err(
                            "3D lighting currently requires Raylib CPU skinning buffers".into()
                        );
                    }
                    for (logical, action) in &entry.clips {
                        let index = book
                            .iter()
                            .position(|a| name(&a.as_ref().name) == *action)
                            .ok_or_else(|| {
                                format!("3D model {id} has no action {action} for {logical}")
                            })?;
                        if !unsafe {
                            raylib::ffi::IsModelAnimationValid(
                                *model.as_ref(),
                                *book[index].as_ref(),
                            )
                        } {
                            return Err(
                                format!("3D action {action} skeleton differs from {id}").into()
                            );
                        }
                        clips.insert(logical.clone(), index);
                    }
                }
                if matches!(id.as_str(), "julia" | "broker") {
                    let side = if id == "broker" { "r" } else { "l" };
                    for part in ["upperarm", "lowerarm", "hand"] {
                        let required = format!("{part}_{side}");
                        if model
                            .bones()
                            .is_none_or(|bones| !bones.iter().any(|b| name(&b.name) == required))
                        {
                            return Err(format!(
                                "3D restraint actor {id} requires bone {required}"
                            )
                            .into());
                        }
                    }
                }
                let phone_bone = model
                    .bones()
                    .and_then(|bones| bones.iter().position(|bone| name(&bone.name) == "hand_r"));
                if entry.clips.contains_key("phone") && phone_bone.is_none() {
                    return Err(format!("3D phone actor {id} requires bone hand_r").into());
                }
                resources.push(path.display().to_string());
                entries.insert(
                    id,
                    AnimatedModel {
                        model,
                        animations,
                        clips,
                        entry,
                        phone_bone,
                    },
                );
            }
        }
        Ok(Self {
            entries: RefCell::new(entries),
            _textures: textures,
            _shader: shader,
            resources,
        })
    }

    pub fn contains(&self, id: &str) -> bool {
        self.entries.borrow().contains_key(id)
    }

    pub fn texture_footprint(&self) -> (usize, u64) {
        (
            self._textures.0.len(),
            self._textures
                .0
                .values()
                .map(|texture| texture.width.max(0) as u64 * texture.height.max(0) as u64 * 4)
                .sum(),
        )
    }

    pub fn has_pair(&self) -> bool {
        self.contains("julia") && self.contains("broker")
    }

    /// Draw inside the already selected stage camera, preserving painted layer order.
    pub fn draw(&self, d: &mut impl RaylibDraw3D, sample: DrawModel<'_>) -> bool {
        let mut entries = self.entries.borrow_mut();
        let Some(actor) = entries.get_mut(sample.id) else {
            return false;
        };
        unsafe {
            raylib::ffi::rlDrawRenderBatchActive();
            raylib::ffi::rlEnableDepthTest();
        }
        let scale = sample
            .length
            .zip(actor.entry.length_m)
            .map_or(sample.height / actor.entry.height_m, |(pixels, meters)| {
                pixels / meters
            });
        if let Some(index) = actor
            .clips
            .get(sample.clip)
            .or_else(|| actor.clips.get("idle"))
        {
            let anim = &actor.animations.as_ref().expect("loaded animation map")[*index];
            let phase = sample
                .distance
                .zip(actor.entry.strides_m.get(sample.clip))
                .map(|(distance, stride)| distance / (scale * stride))
                .or(sample.phase);
            let at = frame(
                sample.ticks,
                phase,
                anim.as_ref().keyframeCount,
                sample.looping,
            );
            // The exclusive catalog borrow spans update and immediate GPU submission.
            unsafe {
                raylib::ffi::UpdateModelAnimation(*actor.model.as_ref(), *anim.as_ref(), at);
            }
        }
        if let Some(contact) = sample.contact {
            pose_contact(&actor.model, sample.at, sample.yaw, scale, contact);
        }
        let mut old_paint = None;
        if let (Some(index), Some(paint)) = (actor.entry.paint_material, sample.paint) {
            let map = &mut actor.model.materials_mut()[index].maps_mut()[0];
            old_paint = Some((index, map.color));
            let linear = |channel: u8| {
                let value = channel as f32 / 255.;
                let linear = if value <= 0.04045 {
                    value / 12.92
                } else {
                    ((value + 0.055) / 1.055).powf(2.4)
                };
                (linear * 255.).round() as u8
            };
            map.color = Color::new(linear(paint.r), linear(paint.g), linear(paint.b), paint.a);
        }
        d.draw_model_ex(
            &actor.model,
            Vector3::from(sample.at),
            Vector3::new(0., 1., 0.),
            sample.yaw,
            Vector3::new(scale, scale, scale),
            Color::WHITE,
        );
        if sample.phone
            && let Some(bone) = actor
                .phone_bone
                .filter(|_| !actor.model.as_ref().currentPose.is_null())
        {
            // currentPose contains the interpolated global hand transform;
            // the prop follows the same wrist as the imported phone action.
            let hand = unsafe { *actor.model.as_ref().currentPose.add(bone) };
            draw_phone(d, hand, sample.at, sample.yaw, scale);
        }
        if let Some((index, color)) = old_paint {
            actor.model.materials_mut()[index].maps_mut()[0].color = color;
        }
        let rider = if sample.id == "scooter" {
            entries.get("delivery-rider").map(|rider| DrawModel {
                id: "delivery-rider",
                at: sample.at,
                height: rider.entry.height_m * scale,
                length: None,
                yaw: sample.yaw,
                clip: "riding",
                ticks: sample.ticks,
                distance: None,
                phase: None,
                looping: true,
                paint: None,
                contact: None,
                phone: false,
            })
        } else {
            None
        };
        drop(entries);
        if let Some(rider) = rider {
            self.draw(d, rider);
        }
        unsafe {
            raylib::ffi::rlDisableDepthTest();
        }
        true
    }

    /// Orthographic framing maps one world pixel to one reference framebuffer pixel.
    pub fn draw_screen(&self, d: &mut impl RaylibDraw, sample: DrawModel<'_>) -> bool {
        self.draw_viewport(d, sample, [1280., 720.])
    }

    pub fn draw_viewport(
        &self,
        d: &mut impl RaylibDraw,
        mut sample: DrawModel<'_>,
        size: [f32; 2],
    ) -> bool {
        if !self.contains(sample.id) {
            return false;
        }
        sample.at[1] = size[1] - sample.at[1];
        if let Some(contact) = &mut sample.contact {
            contact.wrist[1] = size[1] - contact.wrist[1];
            contact.elbow[1] = size[1] - contact.elbow[1];
        }
        let camera = Camera3D::orthographic(
            Vector3::new(size[0] * 0.5, size[1] * 0.5, 2000.),
            Vector3::new(size[0] * 0.5, size[1] * 0.5, 0.),
            Vector3::new(0., 1., 0.),
            size[1],
        );
        let mut view = d.begin_mode3D(camera);
        self.draw(&mut view, sample)
    }
}

pub fn person_id(person: &Pedestrian) -> &'static str {
    match person.wardrobe {
        Wardrobe::Leather => "crowd-leather",
        Wardrobe::PlumDress => "crowd-plum-dress",
        Wardrobe::AmberJacket => "crowd-amber-jacket",
        Wardrobe::Denim => "crowd-denim",
        Wardrobe::TealDress => "crowd-teal-dress",
        Wardrobe::WhiteShirt => "crowd-white-shirt",
        Wardrobe::RedBlouse => "crowd-red-blouse",
        Wardrobe::LongCoat => "crowd-long-coat",
    }
}

pub fn person_sample(person: &Pedestrian, ticks: u64) -> DrawModel<'static> {
    let moving = person.fleeing || person.activity == Activity::Walking;
    DrawModel {
        id: person_id(person),
        at: [person.x, person.ground_y, -120.],
        height: 96. * person.scale,
        length: None,
        yaw: if moving {
            90. * person.facing
        } else {
            28. * person.facing
        },
        clip: if person.fleeing {
            "run"
        } else {
            match person.activity {
                Activity::Walking => "walk",
                Activity::Conversation => "conversation",
                Activity::Seated => "seated",
                Activity::Phone => "phone",
            }
        },
        ticks: ticks as f32 + person.id as f32 * 17.,
        distance: moving.then_some(person.x * person.facing),
        phase: None,
        looping: true,
        paint: None,
        contact: None,
        phone: person.activity == Activity::Phone && !person.fleeing && person.pose.alarm == 0.,
    }
}

pub fn vehicle_sample(vehicle: &Vehicle) -> DrawModel<'static> {
    let (id, height) = match vehicle.kind {
        VehicleKind::Hatch => ("hatchback", 63. * vehicle.scale),
        VehicleKind::Taxi => ("taxi", 72. * vehicle.scale),
        VehicleKind::DeliveryScooter => ("scooter", 79. * vehicle.scale),
    };
    DrawModel {
        id,
        at: [
            vehicle.x,
            vehicle.ground_y,
            if vehicle.facing > 0. { 600. } else { 500. },
        ],
        height,
        length: match vehicle.kind {
            VehicleKind::Hatch => Some(172. * vehicle.scale),
            VehicleKind::Taxi => Some(187. * vehicle.scale),
            VehicleKind::DeliveryScooter => None,
        },
        yaw: 90. * vehicle.facing,
        clip: "drive",
        ticks: vehicle.wheel_angle / std::f32::consts::TAU * 10.,
        distance: None,
        phase: Some(vehicle.wheel_angle / std::f32::consts::TAU),
        looping: true,
        paint: (vehicle.kind == VehicleKind::Hatch).then_some(if vehicle.id == 0 {
            Color::new(61, 84, 110, 255)
        } else {
            Color::new(131, 60, 57, 255)
        }),
        contact: None,
        phone: false,
    }
}

pub fn actor_sample<'a>(
    assets: &'a super::assets::ActorAssets,
    clip: &'a str,
    ticks: f32,
    at: [f32; 3],
    facing: f32,
    yaw: f32,
    scale: f32,
) -> DrawModel<'a> {
    let action = &assets.clips.clips[clip];
    DrawModel {
        id: &assets.pack.character.id,
        at,
        height: assets.rig.height * scale,
        length: None,
        yaw: 90. * facing + yaw * facing,
        clip,
        ticks,
        // Preview scrubbing names an action phase, independently of display scale.
        // Moving world/arena actors explicitly provide their traveled distance.
        distance: None,
        phase: Some(ticks / action.duration_ticks as f32),
        looping: action.looping,
        paint: None,
        contact: None,
        phone: false,
    }
}

fn hand_point(
    hand: raylib::ffi::Transform,
    point: Vector3,
    at: [f32; 3],
    yaw: f32,
    scale: f32,
) -> Vector3 {
    let model = hand.translation + point.rotate_by_quaternion(hand.rotation);
    Vector3::from(at)
        + model.rotate_by_axis_angle(Vector3::new(0., 1., 0.), yaw.to_radians()) * scale
}

fn draw_phone(
    d: &mut impl RaylibDraw3D,
    hand: raylib::ffi::Transform,
    at: [f32; 3],
    yaw: f32,
    scale: f32,
) {
    // MPFB hand local +Y follows the palm toward the fingers. The rigid case
    // sits against the palm, retaining its volume from every cinematic angle.
    let point = |x, y, z| hand_point(hand, Vector3::new(x, y, z), at, yaw, scale);
    let vertices = [
        point(-0.037, -0.014, 0.012),
        point(0.037, -0.014, 0.012),
        point(0.037, 0.134, 0.012),
        point(-0.037, 0.134, 0.012),
        point(-0.037, -0.014, 0.024),
        point(0.037, -0.014, 0.024),
        point(0.037, 0.134, 0.024),
        point(-0.037, 0.134, 0.024),
    ];
    for (indices, color) in [
        ([0, 3, 2, 1], Color::new(24, 31, 38, 255)),
        ([4, 5, 6, 7], Color::new(35, 44, 51, 255)),
        ([0, 4, 7, 3], Color::new(47, 55, 61, 255)),
        ([1, 2, 6, 5], Color::new(47, 55, 61, 255)),
        ([0, 1, 5, 4], Color::new(24, 31, 38, 255)),
        ([3, 7, 6, 2], Color::new(68, 81, 87, 255)),
    ] {
        let [a, b, c, e] = indices.map(|i| vertices[i]);
        d.draw_triangle3D(a, b, c, color);
        d.draw_triangle3D(a, c, e, color);
    }
    let screen = [
        point(-0.030, -0.001, 0.0242),
        point(0.030, -0.001, 0.0242),
        point(0.030, 0.118, 0.0242),
        point(-0.030, 0.118, 0.0242),
    ];
    let color = Color::new(65, 91, 100, 255);
    d.draw_triangle3D(screen[0], screen[1], screen[2], color);
    d.draw_triangle3D(screen[0], screen[2], screen[3], color);
    // Submit the small immediate-mode prop before restoring painted depth order.
    unsafe { raylib::ffi::rlDrawRenderBatchActive() };
}

fn elbow(
    root: Vector3,
    hand: Vector3,
    pole: Vector3,
    upper: f32,
    lower: f32,
) -> (Vector3, Vector3) {
    let delta = hand - root;
    let distance = delta
        .length()
        .clamp((upper - lower).abs() + 0.001, upper + lower - 0.001);
    let direction = delta.normalize();
    let hand = root + direction * distance;
    let along = (upper * upper - lower * lower + distance * distance) / (2. * distance);
    let bend = (upper * upper - along * along).max(0.).sqrt();
    let projected = pole - root - direction * (pole - root).dot(direction);
    let normal = if projected.length_sqr() > 0.00001 {
        projected.normalize()
    } else {
        direction.perpendicular().normalize()
    };
    (root + direction * along + normal * bend, hand)
}

fn grip_wrist(root: Vector3, contact: Vector3, palm_offset: f32) -> Vector3 {
    contact - (contact - root).normalize() * palm_offset
}

fn descendant(bones: &[BoneInfo], mut index: usize, ancestor: usize) -> bool {
    for _ in 0..bones.len() {
        if index == ancestor {
            return true;
        }
        let parent = bones[index].parent;
        if parent < 0 || parent as usize >= bones.len() {
            return false;
        }
        index = parent as usize;
    }
    false
}

/// Adjust the imported arm and all its finger descendants as one continuous skin.
fn pose_contact(model: &Model, at: [f32; 3], yaw: f32, scale: f32, contact: ArmContact) {
    let Some(bones) = model.bones() else {
        return;
    };
    let find = |part: &str| {
        bones
            .iter()
            .position(|b| name(&b.name) == format!("{part}_{}", contact.side))
    };
    let (Some(upper), Some(lower), Some(hand)) = (find("upperarm"), find("lowerarm"), find("hand"))
    else {
        return;
    };
    let raw = model.as_ref();
    if raw.currentPose.is_null() {
        return;
    }
    // Raylib stores global skeleton transforms after importing the glTF hierarchy.
    let original = unsafe { std::slice::from_raw_parts(raw.currentPose, bones.len()) };
    let mut posed = original.to_vec();
    let local = |p: [f32; 3]| {
        ((Vector3::from(p) - Vector3::from(at)) * (1. / scale))
            .rotate_by_axis_angle(Vector3::new(0., 1., 0.), -yaw.to_radians())
    };
    let root = original[upper].translation;
    let old_elbow = original[lower].translation;
    let old_hand = original[hand].translation;
    let palm_target = local(contact.wrist);
    let (new_elbow, new_hand) = elbow(
        root,
        grip_wrist(root, palm_target, contact.palm_offset),
        local(contact.elbow),
        (old_elbow - root).length(),
        (old_hand - old_elbow).length(),
    );
    let upper_rotation = Quaternion::from_vector3_to_vector3(
        (old_elbow - root).normalize(),
        (new_elbow - root).normalize(),
    );
    let lower_rotation = Quaternion::from_vector3_to_vector3(
        (old_hand - old_elbow).normalize(),
        (new_hand - new_elbow).normalize(),
    );
    for (index, pose) in posed.iter_mut().enumerate() {
        let (rotation, origin, target) = if descendant(bones, index, lower) {
            (lower_rotation, old_elbow, new_elbow)
        } else if descendant(bones, index, upper) {
            (upper_rotation, root, root)
        } else {
            continue;
        };
        pose.translation = target + (pose.translation - origin).rotate_by_quaternion(rotation);
        pose.rotation = unsafe { raylib::ffi::QuaternionMultiply(rotation, pose.rotation) };
    }
    if contact.palm_offset > 0. {
        // The holder's palm reaches Julia's wrist; the two wrist origins remain
        // separate. Rotate the complete hand/finger subtree, preserving the
        // authored grip and the wrist's twist rather than deforming individual digits.
        let palm_axis = Vector3::new(0., 1., 0.).rotate_by_quaternion(posed[hand].rotation);
        let align = Quaternion::from_vector3_to_vector3(
            palm_axis.normalize(),
            (palm_target - new_hand).normalize(),
        );
        for (index, pose) in posed.iter_mut().enumerate() {
            if descendant(bones, index, hand) {
                pose.translation =
                    new_hand + (pose.translation - new_hand).rotate_by_quaternion(align);
                pose.rotation = unsafe { raylib::ffi::QuaternionMultiply(align, pose.rotation) };
            }
        }
    }
    let mut pointer = posed.as_mut_ptr();
    let animation = raylib::ffi::ModelAnimation {
        boneCount: bones.len() as i32,
        keyframeCount: 1,
        keyframePoses: &mut pointer,
        ..Default::default()
    };
    // The borrowed one-frame pose survives the synchronous CPU skin/update call.
    unsafe {
        raylib::ffi::UpdateModelAnimation(*raw, animation, 0.);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn animation_seek_clamps_actions_and_wraps_cycles() {
        assert_eq!(frame(10., None, 61, true), 10.);
        assert_eq!(frame(125., None, 61, true), 5.);
        assert_eq!(frame(125., None, 61, false), 60.);
        assert_eq!(frame(0., Some(0.5), 61, false), 30.);
        assert_eq!(frame(0., Some(-0.25), 61, true), 45.);
    }
    #[test]
    fn preview_scrubbing_keeps_the_same_action_phase_at_both_display_sizes() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("assets/adventure/actors/cpp/character.json");
        let content = super::super::assets::ActorContent::load(&root).unwrap();
        let assets = super::super::assets::ActorAssets {
            pack: content.pack,
            rig: content.rig,
            clips: content.clips,
            textures: BTreeMap::new(),
        };
        let ticks = assets.clips.clips["run"].duration_ticks as f32 * 0.5;
        for scale in [1., 1.85] {
            let sample = actor_sample(&assets, "run", ticks, [400., 565., 0.], 1., 0., scale);
            assert_eq!(sample.phase, Some(0.5));
            assert_eq!(sample.distance, None);
            assert_eq!(frame(sample.ticks, sample.phase, 49, true), 24.);
            assert_eq!(sample.traveled(153.).distance, Some(153.));
        }
    }
    #[test]
    fn model_catalog_rejects_outside_paths_and_invalid_units() {
        let valid = r#"{"schema_version":1,"entries":{"cpp":{"file":"cpp.glb","height_m":1.75,"animation_fps":60,"clips":{"idle":"idle"}}}}"#;
        assert!(Catalog::from_json(valid).is_ok());
        assert!(Catalog::from_json(&valid.replace("cpp.glb", "../cpp.glb")).is_err());
        assert!(Catalog::from_json(&valid.replace("1.75", "175")).is_err());
        assert!(Catalog::from_json(&valid.replace("fps\":60", "fps\":30")).is_err());
    }
    #[test]
    fn phone_belongs_to_the_activity_and_is_put_away_on_alarm() {
        use crate::adventure::augusta::ambient::Nightlife;
        let street = Nightlife::sample(120, None);
        let phones: Vec<_> = street
            .people
            .iter()
            .filter(|person| person_sample(person, street.ticks).phone)
            .map(|person| person.id)
            .collect();
        assert_eq!(phones, [8, 21]);
        let mut person = street
            .people
            .iter()
            .find(|person| person.id == 8)
            .copied()
            .unwrap();
        person.activity = Activity::Walking;
        assert!(!person_sample(&person, 120).phone);
        person.activity = Activity::Phone;
        person.pose.alarm = 0.1;
        assert!(!person_sample(&person, 120).phone);
        person.pose.alarm = 0.;
        person.fleeing = true;
        let escaping = person_sample(&person, 120);
        assert!(!escaping.phone);
        assert_eq!(escaping.clip, "run");
    }
    #[test]
    fn hand_prop_follows_bone_rotation_and_world_facing_at_physical_scale() {
        let hand = raylib::ffi::Transform {
            translation: Vector3::new(0.1, 1.5, 0.2),
            rotation: Quaternion::from_axis_angle(Vector3::new(0., 0., 1.), 0.4),
            scale: Vector3::new(1., 1., 1.),
        };
        for yaw in [-90., -28., 28., 90.] {
            let at = [780., 0., -120.];
            let wrist = hand_point(hand, Vector3::zero(), at, yaw, 110.);
            let tip = hand_point(hand, Vector3::new(0., 0.148, 0.), at, yaw, 110.);
            assert!(((tip - wrist).length() - 0.148 * 110.).abs() < 0.0001);
            let moved = hand_point(hand, Vector3::zero(), [800., 30., -100.], yaw, 110.);
            assert!((moved - wrist - Vector3::new(20., 30., 20.)).length() < 0.0001);
        }
    }
    #[test]
    fn restraint_reaches_contact_with_both_bone_lengths_intact() {
        let root = Vector3::new(0., 1.5, 0.);
        let target = Vector3::new(0.4, 1.2, 0.1);
        let pole = Vector3::new(0.2, 1.05, 0.2);
        let (joint, hand) = elbow(root, target, pole, 0.32, 0.28);
        assert!((hand - target).length() < 0.00001);
        assert!(((joint - root).length() - 0.32).abs() < 0.00001);
        assert!(((hand - joint).length() - 0.28).abs() < 0.00001);
        let (_, far_hand) = elbow(root, Vector3::new(4., 1.2, 0.), pole, 0.32, 0.28);
        assert!((far_hand - root).length() < 0.60);
    }
    #[test]
    fn holder_palm_reaches_the_wrist_without_coinciding_both_wrist_origins() {
        // Measured internal broker arm in the authored resting shot: its palm
        // can cover the 5.55 px deficit while both rigid bones retain their lengths.
        let scale = 105.593;
        let shoulder = Vector3::new(1677.76, 154.73, -20.24) * (1. / scale);
        let contact = Vector3::new(1730., 122., -24.) * (1. / scale);
        let upper = 0.2628;
        let lower = 0.2695;
        assert!((contact - shoulder).length() > upper + lower);
        let wrist = grip_wrist(shoulder, contact, 0.06);
        let (joint, reached) = elbow(
            shoulder,
            wrist,
            shoulder + Vector3::new(0., -0.3, 0.1),
            upper,
            lower,
        );
        assert!((reached - wrist).length() < 0.00001);
        assert!(((joint - shoulder).length() - upper).abs() < 0.00001);
        assert!(((reached - joint).length() - lower).abs() < 0.00001);
        assert!(((contact - reached).length() - 0.06).abs() < 0.00001);
        assert_eq!(grip_wrist(shoulder, contact, 0.), contact);
    }
}
