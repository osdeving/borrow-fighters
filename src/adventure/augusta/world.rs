//! Loads Augusta's physical markers, scene pieces and chapter timing.
//!
//! System: C++ adventure content. Coordinates and narration references are
//! independent from textures, combat specifications and save files.

use serde::Deserialize;
use std::{error::Error, fs, path::Path};

/// A reusable environment piece placed in the chapter's world coordinates.
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WorldPiece {
    pub id: String,
    pub piece: String,
    pub position: [f32; 2],
    pub scale: f32,
    pub layer: i32,
}

/// Physical scene and narrative landmarks, all independent of image sizes.
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct World {
    version: u32,
    pub width: f32,
    pub ground_y: f32,
    pub walk_bounds: [f32; 2],
    pub player_spawn_x: f32,
    pub confrontation_x: f32,
    pub guards_arena: [f32; 2],
    pub guards_spawn_x: Vec<f32>,
    /// One physical threshold under LIMIAR; every guard exits this opening.
    pub bar_door: [f32; 2],
    pub erratic_center_x: f32,
    pub erratic_landings_x: [f32; 2],
    pub julia_initial_x: f32,
    pub julia_x: f32,
    pub broker_x: f32,
    pub exit_x: f32,
    pub pieces: Vec<WorldPiece>,
}

impl World {
    /// Rejects markers outside their physical scene before a session starts.
    pub fn load(path: &Path) -> Result<Self, Box<dyn Error>> {
        Self::from_json(&fs::read_to_string(path)?)
    }

    pub fn from_json(json: &str) -> Result<Self, Box<dyn Error>> {
        let world: Self = serde_json::from_str(json)?;
        let inside =
            |x: f32| x.is_finite() && (world.walk_bounds[0]..=world.walk_bounds[1]).contains(&x);
        if world.version != 1
            || !(1280.0..=32000.0).contains(&world.width)
            || !(400.0..=640.0).contains(&world.ground_y)
            || world.walk_bounds.iter().any(|x| !x.is_finite())
            || world.walk_bounds[0] < 0.0
            || world.walk_bounds[1] > world.width
            || world.walk_bounds[1] - world.walk_bounds[0] < 1000.0
            || [
                world.player_spawn_x,
                world.confrontation_x,
                world.erratic_center_x,
                world.julia_initial_x,
                world.julia_x,
                world.broker_x,
                world.exit_x,
            ]
            .into_iter()
            .chain(world.guards_arena)
            .chain(world.guards_spawn_x.iter().copied())
            .chain(world.erratic_landings_x)
            .any(|x| !inside(x))
            || world.guards_arena[1] - world.guards_arena[0] < 700.0
            || !(2..=4).contains(&world.guards_spawn_x.len())
            || !world
                .guards_spawn_x
                .iter()
                .all(|x| (world.guards_arena[0]..=world.guards_arena[1]).contains(x))
            || !world
                .erratic_landings_x
                .iter()
                .all(|x| (world.guards_arena[0]..=world.guards_arena[1]).contains(x))
            || !(world.guards_arena[0]..=world.guards_arena[1]).contains(&world.confrontation_x)
            || !(world.guards_arena[0]..=world.guards_arena[1]).contains(&world.erratic_center_x)
            || world.erratic_landings_x[0] >= world.erratic_center_x - 120.0
            || world.erratic_landings_x[1] <= world.erratic_center_x + 120.0
            || world.player_spawn_x >= world.confrontation_x
            || !inside(world.bar_door[0])
            || !(world.ground_y - 100.0..=world.ground_y).contains(&world.bar_door[1])
            || !(44.0..=100.0).contains(&(world.julia_initial_x - world.broker_x))
            || world.julia_x <= world.guards_arena[1]
            || world.exit_x <= world.julia_x
        {
            return Err("invalid Augusta world bounds or narrative markers".into());
        }
        let mut ids = std::collections::BTreeSet::new();
        for piece in &world.pieces {
            if piece.id.trim().is_empty()
                || piece.piece.trim().is_empty()
                || !ids.insert(&piece.id)
                || piece.position.iter().any(|p| !p.is_finite())
                || !piece.scale.is_finite()
                || piece.scale <= 0.0
            {
                return Err("invalid or duplicate Augusta scenery instance".into());
            }
        }
        Ok(world)
    }

    /// Gameplay camera center, including at the extreme ends of the street.
    pub fn camera_x(&self, actor_x: f32) -> f32 {
        actor_x.clamp(640.0, self.width - 640.0)
    }
}

/// Short authored beats; dialogue progression remains controlled by the player.
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Timing {
    pub introduction_ticks: u32,
    pub julia_attempt_ticks: u32,
    pub guards_arrival_ticks: u32,
    pub erratics_arrival_ticks: u32,
    pub broker_escape_ticks: u32,
}

/// One chapter entry, with relative files that can be replaced independently.
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChapterSpec {
    version: u32,
    pub id: String,
    pub protagonist: String,
    pub title_key: String,
    pub summary_key: String,
    pub world: String,
    pub texts: String,
    pub art: String,
    pub timing: Timing,
}

impl ChapterSpec {
    pub fn load(path: &Path) -> Result<Self, Box<dyn Error>> {
        let spec: Self = serde_json::from_str(&fs::read_to_string(path)?)?;
        let local_file = |name: &str| {
            let path = Path::new(name);
            !name.is_empty()
                && path
                    .components()
                    .all(|part| matches!(part, std::path::Component::Normal(_)))
        };
        if spec.version != 1
            || spec.id != "cpp-augusta"
            || spec.protagonist != "cpp"
            || spec.title_key.trim().is_empty()
            || spec.summary_key.trim().is_empty()
            || !local_file(&spec.world)
            || !local_file(&spec.texts)
            || !local_file(&spec.art)
            || !(60..=3600).contains(&spec.timing.introduction_ticks)
            || !(120..=1200).contains(&spec.timing.julia_attempt_ticks)
            || !(120..=1800).contains(&spec.timing.guards_arrival_ticks)
            || !(120..=1200).contains(&spec.timing.erratics_arrival_ticks)
            || !(30..=600).contains(&spec.timing.broker_escape_ticks)
            || spec.timing.broker_escape_ticks > spec.timing.erratics_arrival_ticks * 7 / 8
        {
            return Err("invalid C++ chapter entry or timing".into());
        }
        Ok(spec)
    }
}
