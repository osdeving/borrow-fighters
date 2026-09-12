//! Validates the chapter's three small spaces and their interaction geometry.
//!
//! System: Adventure chapter. The same world coordinates feed physics, authored
//! approaches, interaction prompts and debug drawing; images never define triggers.

use super::debris::DebrisSpec;
use crate::adventure::combat::EnemyTuning;
use crate::math::{rect::Rect, vec2::Vec2};
use serde::{Deserialize, Serialize};

/// One independently bounded stretch of the chapter.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Scene {
    /// The evacuated street from the prologue.
    Street,
    /// A short residential lane with a jumpable obstruction.
    Lane,
    /// A passage Rust must make safe before continuing.
    Passage,
}

/// A serializable world point, normally an actor's feet.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Point {
    /// Horizontal coordinate.
    pub x: f32,
    /// Vertical coordinate.
    pub y: f32,
}

impl Point {
    /// Converts data coordinates into neutral simulation geometry.
    pub fn vec(self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }
}

/// A data-owned collision or interaction rectangle.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Region {
    /// Left edge.
    pub x: f32,
    /// Top edge.
    pub y: f32,
    /// Horizontal extent.
    pub width: f32,
    /// Vertical extent.
    pub height: f32,
}

impl Region {
    /// Returns neutral geometry for contact and debug drawing.
    pub fn rect(self) -> Rect {
        Rect::new(self.x, self.y, self.width, self.height)
    }

    /// Tests the feet point, including the edge of an interaction zone.
    pub fn contains(self, point: Vec2) -> bool {
        point.x >= self.x
            && point.x <= self.x + self.width
            && point.y >= self.y
            && point.y <= self.y + self.height
    }
}

/// A person or event with a reachable foreground interaction zone.
#[derive(Clone, Debug, Deserialize)]
pub struct InterestPoint {
    /// Stable identifier used by the chapter, never a sprite filename.
    pub id: String,
    /// Position of the person or event in the same world space.
    pub position: Point,
    /// Foreground region where Rust may initiate this interaction.
    pub region: Region,
    /// Ordered approach points, beginning on the foreground floor.
    pub path: Vec<Point>,
}

/// One chapter enemy with independent placement and reviewed difficulty.
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EnemySpawn {
    /// Stable identity within the local encounter.
    pub id: String,
    /// Safe feet position before the encounter becomes visible.
    pub position: Point,
    /// Explicit difficulty parameters, independent of rendering.
    pub tuning: EnemyTuning,
}

/// An independently replaceable, nonblocking piece of loose foreground debris.
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LooseProp {
    /// Stable instance identity.
    pub id: String,
    /// Sprite reference from the chapter catalog.
    pub piece: String,
    /// Feet/anchor in chapter world coordinates.
    pub position: Point,
    /// Uniform size multiplier.
    pub scale: f32,
    /// Rotation in degrees around the catalog anchor.
    pub rotation: f32,
}

/// Geometry of a single bounded chapter scene.
#[derive(Clone, Debug, Deserialize)]
pub struct SceneGeometry {
    /// Which authored stretch these data belong to.
    pub id: Scene,
    /// Width in world pixels, independent of viewport width.
    pub width: f32,
    /// Foreground floor coordinate.
    pub floor_y: f32,
    /// Minimum reachable feet coordinate.
    pub walk_min: f32,
    /// Maximum reachable feet coordinate.
    pub walk_max: f32,
    /// Safe arrival point when entering this scene.
    pub spawn: Point,
    /// Ordered scenery targets and event regions.
    pub points: Vec<InterestPoint>,
    /// Solid rectangles; the current lane contains one low obstruction.
    pub obstacles: Vec<Region>,
    /// Independent breakable cargo pieces.
    #[serde(default)]
    pub debris: Vec<DebrisSpec>,
    /// Loose boards and small fragments, never included in body collision.
    #[serde(default)]
    pub loose_props: Vec<LooseProp>,
    /// Explicit encounter roster, empty outside the passage.
    #[serde(default)]
    pub enemies: Vec<EnemySpawn>,
    /// Foreground region leading to the next authored stretch.
    pub exit: Region,
}

impl SceneGeometry {
    /// Looks up a validated target by stable identifier.
    pub fn poi(&self, id: &str) -> Option<&InterestPoint> {
        self.points.iter().find(|point| point.id == id)
    }
}

/// Validated data for this chapter, without a general map or scripting system.
#[derive(Clone, Debug, Deserialize)]
pub struct World {
    /// Spatial data schema version.
    pub version: u32,
    /// Exactly one instance of each authored scene.
    pub scenes: Vec<SceneGeometry>,
}

impl World {
    /// Parses and validates externally editable chapter geometry.
    pub fn from_json(source: &str) -> Result<Self, String> {
        let world: Self = serde_json::from_str(source).map_err(|e| e.to_string())?;
        world.validate()?;
        Ok(world)
    }

    /// Loads the reviewed default geometry without filesystem I/O.
    pub fn bundled() -> Self {
        Self::from_json(include_str!("../../../assets/adventure/chapter/world.json"))
            .expect("bundled chapter geometry must be valid")
    }

    /// Returns the unique scene established during validation.
    pub fn scene(&self, scene: Scene) -> &SceneGeometry {
        self.scenes
            .iter()
            .find(|entry| entry.id == scene)
            .expect("validated scene")
    }

    fn validate(&self) -> Result<(), String> {
        if self.version != 1 || self.scenes.len() != 3 {
            return Err("Chapter geometry requires version 1 and three scenes".into());
        }
        for id in [Scene::Street, Scene::Lane, Scene::Passage] {
            if self.scenes.iter().filter(|s| s.id == id).count() != 1 {
                return Err(format!("Missing or duplicated scene: {id:?}"));
            }
            let scene = self.scene(id);
            if !scene.width.is_finite()
                || !(1280.0..=32768.0).contains(&scene.width)
                || scene.floor_y != 580.0
                || !scene.walk_min.is_finite()
                || !scene.walk_max.is_finite()
                || scene.walk_min < 40.0
                || scene.walk_max > scene.width - 40.0
                || scene.walk_min >= scene.walk_max
            {
                return Err(format!("Invalid bounds in {id:?}"));
            }
            let point_valid = |p: Point| {
                p.x.is_finite()
                    && p.y.is_finite()
                    && (scene.walk_min..=scene.walk_max).contains(&p.x)
                    && (0.0..=scene.floor_y).contains(&p.y)
            };
            let region_valid = |r: Region| {
                [r.x, r.y, r.width, r.height].iter().all(|v| v.is_finite())
                    && r.width > 0.0
                    && r.height > 0.0
                    && r.x >= 0.0
                    && r.y >= 0.0
                    && r.x + r.width <= scene.width
                    && r.y + r.height <= 720.0
            };
            let reachable = |r: Region| {
                region_valid(r)
                    && r.y <= scene.floor_y
                    && r.y + r.height >= scene.floor_y
                    && r.x + r.width >= scene.walk_min
                    && r.x <= scene.walk_max
            };
            if !point_valid(scene.spawn) || scene.spawn.y != scene.floor_y || !reachable(scene.exit)
            {
                return Err(format!("Unreachable spawn or exit in {id:?}"));
            }
            for (index, point) in scene.points.iter().enumerate() {
                if point.id.is_empty()
                    || scene.points[..index].iter().any(|p| p.id == point.id)
                    || !point_valid(point.position)
                    || !reachable(point.region)
                    || point.path.iter().any(|p| !point_valid(*p))
                    || point.path.first().is_some_and(|p| p.y != scene.floor_y)
                {
                    return Err(format!("Invalid interaction {} in {id:?}", point.id));
                }
            }
            for obstacle in &scene.obstacles {
                if !region_valid(*obstacle)
                    || obstacle.y + obstacle.height != scene.floor_y
                    || obstacle.height > 90.0
                    || obstacle.width > 130.0
                    || obstacle.rect().intersects(scene.exit.rect())
                    || scene
                        .points
                        .iter()
                        .any(|p| obstacle.rect().intersects(p.region.rect()))
                    || obstacle.contains(scene.spawn.vec())
                {
                    return Err(format!("Unreachable or unsupported obstruction in {id:?}"));
                }
            }
            for (index, piece) in scene.debris.iter().enumerate() {
                if piece.id.is_empty()
                    || piece.piece.is_empty()
                    || piece.fragment.is_empty()
                    || scene.debris[..index].iter().any(|p| p.id == piece.id)
                    || !piece.rotation.is_finite()
                    || piece.rotation.abs() > 12.0
                    || !(12..=96).contains(&piece.hp)
                    || !region_valid(piece.region)
                    || piece.region.y + piece.region.height > scene.floor_y
                    || piece.region.height > 120.0
                    || piece.region.width > 150.0
                    || piece.region.rect().intersects(scene.exit.rect())
                    || piece.region.contains(scene.spawn.vec())
                    || scene
                        .points
                        .iter()
                        .any(|p| piece.region.rect().intersects(p.region.rect()))
                    || scene.debris[..index]
                        .iter()
                        .any(|p| p.region.rect().intersects(piece.region.rect()))
                {
                    return Err(format!("Invalid destructible {} in {id:?}", piece.id));
                }
            }
            for (index, prop) in scene.loose_props.iter().enumerate() {
                if prop.id.is_empty()
                    || prop.piece.is_empty()
                    || scene.loose_props[..index].iter().any(|p| p.id == prop.id)
                    || !prop.position.x.is_finite()
                    || !prop.position.y.is_finite()
                    || !(scene.walk_min..=scene.walk_max).contains(&prop.position.x)
                    || !(0.0..=scene.floor_y + 30.0).contains(&prop.position.y)
                    || !prop.scale.is_finite()
                    || !(0.1..=2.0).contains(&prop.scale)
                    || !prop.rotation.is_finite()
                    || prop.rotation.abs() > 90.0
                {
                    return Err(format!("Invalid loose prop {} in {id:?}", prop.id));
                }
            }
            if (id == Scene::Passage && scene.enemies.len() != 2)
                || (id != Scene::Passage && !scene.enemies.is_empty())
            {
                return Err("The passage requires exactly two opponents".into());
            }
            for (index, enemy) in scene.enemies.iter().enumerate() {
                if enemy.id.is_empty()
                    || !point_valid(enemy.position)
                    || enemy.position.y != scene.floor_y
                    || !enemy.tuning.is_valid()
                    || scene.enemies[..index]
                        .iter()
                        .any(|e| e.id == enemy.id || (e.position.x - enemy.position.x).abs() < 90.0)
                {
                    return Err(format!("Invalid opponent {} in {id:?}", enemy.id));
                }
            }
            let required: &[&str] = match id {
                Scene::Street => &["driver", "shop", "neighbour"],
                Scene::Lane => &["lane_resident"],
                Scene::Passage => &["enemy"],
            };
            if required.iter().any(|key| scene.poi(key).is_none()) {
                return Err(format!("Missing chapter target in {id:?}"));
            }
            if id == Scene::Street && scene.points.iter().any(|point| point.path.is_empty()) {
                return Err("Street conversations require explicit approach paths".into());
            }
        }
        Ok(())
    }
}
