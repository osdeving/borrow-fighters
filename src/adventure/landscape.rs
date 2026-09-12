//! Defines reusable street composition and the prologue's physical map.
//!
//! System: Adventure world. Ground bounds and story markers are independent of
//! images; facade instances share world coordinates and retain their base socket.

use serde::Deserialize;

/// Physical extent and named story locations for the morning street.
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PrologueMap {
    /// Full traversable scene extent, independent of the camera.
    pub width: f32,
    /// Reachable feet interval.
    pub bounds: [f32; 2],
    /// Rust's starting feet coordinate on the foreground pavement.
    pub spawn_x: f32,
    /// Crossing this coordinate begins the EP arrival.
    pub arrival_x: f32,
    /// The same landing anchor is used by physics and cinematic rendering.
    pub enemy_x: f32,
}

/// A facade or existing street prop with a horizontal base socket.
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Facade {
    /// Stable editable instance identity.
    pub id: String,
    /// Piece catalog key; `prop.*` entries reuse the street catalog.
    pub piece: String,
    /// World position of the bottom center anchor.
    pub position: [f32; 2],
    /// Uniform scale applied after the catalog's authored scale.
    pub scale: f32,
}

/// Art composition with no duplicate of the chapter's physical width or triggers.
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LandscapeScene {
    /// Street, lane or passage.
    pub id: String,
    /// Translation of the existing shop, people, traffic and aftermath group.
    pub hub_origin: f32,
    /// Individually replaceable facades, walls and street furniture.
    pub instances: Vec<Facade>,
}

/// External landscape composition plus the first street's physical geometry.
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Landscape {
    /// Content schema version.
    pub version: u32,
    /// Background scroll relative to grounded facades; one locks both planes.
    pub distance_scroll: f32,
    /// Authoritative prologue geometry.
    pub prologue: PrologueMap,
    /// Reusable scene compositions.
    pub scenes: Vec<LandscapeScene>,
}

impl Landscape {
    /// Parses content transactionally before replacing any live map.
    pub fn from_json(source: &str) -> Result<Self, String> {
        let map: Self = serde_json::from_str(source).map_err(|e| e.to_string())?;
        let p = &map.prologue;
        if map.version != 1
            || !map.distance_scroll.is_finite()
            || !(0.0..=1.0).contains(&map.distance_scroll)
            || !p.width.is_finite()
            || !(1280.0..=32768.0).contains(&p.width)
            || [p.bounds[0], p.bounds[1], p.spawn_x, p.arrival_x, p.enemy_x]
                .iter()
                .any(|v| !v.is_finite())
            || p.bounds[0] < 40.0
            || p.bounds[1] > p.width - 40.0
            || !(p.bounds[0] < p.spawn_x
                && p.spawn_x < p.arrival_x
                && p.arrival_x < p.enemy_x
                && p.enemy_x < p.bounds[1])
            || map.scenes.len() != 3
        {
            return Err("Invalid prologue world bounds or ordered event markers".into());
        }
        for id in ["street", "lane", "passage"] {
            if map.scenes.iter().filter(|s| s.id == id).count() != 1 {
                return Err(format!("Missing or duplicate landscape {id}"));
            }
            let scene = map.scene(id);
            if !scene.hub_origin.is_finite() || scene.hub_origin < 0.0 || scene.hub_origin > 32768.0
            {
                return Err(format!("Invalid hub in {id}"));
            }
            for (index, item) in scene.instances.iter().enumerate() {
                if item.id.is_empty()
                    || item.piece.is_empty()
                    || scene.instances[..index].iter().any(|p| p.id == item.id)
                    || item.position.iter().any(|v| !v.is_finite())
                    || !(0.0..=32768.0).contains(&item.position[0])
                    || !(300.0..=600.0).contains(&item.position[1])
                    || !item.scale.is_finite()
                    || !(0.1..=3.0).contains(&item.scale)
                {
                    return Err(format!("Invalid facade {} in {id}", item.id));
                }
            }
        }
        Ok(map)
    }

    /// Deterministic default for simulation and tests, without platform I/O.
    pub fn bundled() -> Self {
        Self::from_json(include_str!("../../assets/adventure/world/map.json"))
            .expect("bundled landscape is valid")
    }

    /// Retrieves a unique composition after validation.
    pub fn scene(&self, id: &str) -> &LandscapeScene {
        self.scenes
            .iter()
            .find(|s| s.id == id)
            .expect("validated landscape id")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adventure::combat::{
        ActorKind, Combat, CombatInput, LEVEL_WIDTH, PLAYER_RUN_SPEED, TICKS_PER_SECOND,
    };
    use crate::adventure::story::{Stage, Story};

    #[test]
    fn moving_the_spawn_preserves_the_kite_target_and_camera_handoff() {
        use crate::adventure::story::Story;
        let mut story = Story::new();
        story.map.spawn_x = 3000.0;
        story.combat.player.position.x = story.map.spawn_x;
        for tick in 0..=crate::adventure::arrival::ARRIVAL_TICKS {
            story.stage_ticks = tick;
            let shot = story.initial_shot();
            let center = story.camera_left() + shot.target.x;
            let half = 640.0 / shot.zoom;
            assert!(center - half >= -0.001);
            assert!(center + half <= story.map.width + 0.001);
            if tick == 0 {
                assert!((center - (story.hub_origin + 1108.0)).abs() < 0.01);
            }
            if tick == crate::adventure::arrival::ARRIVAL_TICKS {
                assert_eq!(shot, crate::adventure::arrival::ArrivalShot::settled());
            }
        }
    }

    #[test]
    fn extended_streets_keep_the_camera_and_arrival_on_the_loaded_map() {
        let source = include_str!("../../assets/adventure/world/map.json");
        let original: serde_json::Value = serde_json::from_str(source).unwrap();
        for extension in [0.0, 2048.0] {
            // Moving the event and extending the map requires only validated data.
            let mut edited = original.clone();
            for key in ["width", "arrival_x", "enemy_x"] {
                edited["prologue"][key] =
                    serde_json::json!(original["prologue"][key].as_f64().unwrap() + extension);
            }
            edited["prologue"]["bounds"][1] =
                serde_json::json!(original["prologue"]["bounds"][1].as_f64().unwrap() + extension);
            let map = Landscape::from_json(&edited.to_string()).unwrap().prologue;
            assert!(map.arrival_x > LEVEL_WIDTH);
            let mut story = Story::new();
            story.configure_map(map.clone());
            story.advance_scene();
            story.advance_scene();
            story.advance_scene(); // Release the kite camera, then traverse normally.
            assert_eq!(story.stage, Stage::Encounter);
            assert_eq!(story.combat.player.position.x, map.spawn_x);
            let limit = ((map.arrival_x - map.spawn_x) / PLAYER_RUN_SPEED * TICKS_PER_SECOND as f32)
                .ceil() as usize
                + 120;
            for _ in 0..limit {
                if story.ep_arrival_active() {
                    break;
                }
                assert!(story.combat.player.position.x < map.arrival_x);
                assert!(!story.combat.enemy_awake);
                assert_eq!(story.combat.enemy.position.x, map.enemy_x);
                assert_eq!(story.ambient.accident_ticks(), None);
                story.tick(CombatInput {
                    movement: 1.0,
                    ..Default::default()
                });
                assert!(story.camera_left() >= 0.0);
                assert!(story.camera_left() + 1280.0 <= map.width);
            }
            assert!(
                story.ep_arrival_active(),
                "must reach the relocated arrival"
            );
            assert!(story.combat.player.position.x >= map.arrival_x);
            assert!(
                story.combat.player.position.x
                    <= map.arrival_x + PLAYER_RUN_SPEED / TICKS_PER_SECOND as f32
            );
            for _ in 0..story.ep_arrival.spec.impact_tick() {
                assert!(!story.combat.enemy_awake);
                assert_eq!(story.ambient.accident_ticks(), None);
                let sample = story
                    .ep_arrival
                    .sample(story.combat.enemy.position.x - story.camera_left())
                    .unwrap();
                let focus_x = story.camera_left() + sample.shot.target.x;
                assert!(focus_x - 640.0 / sample.shot.zoom >= 0.0);
                assert!(focus_x + 640.0 / sample.shot.zoom <= map.width);
                story.tick(CombatInput::default());
            }
            assert!(story.ep_arrival.impacted());
            assert!(story.combat.enemy_awake);
            assert_eq!(story.ambient.accident_ticks(), Some(0));
        }
    }

    #[test]
    fn wide_combat_bounds_apply_to_running_and_real_contact_knockback() {
        let map = Landscape::bundled().prologue;
        let mut combat = Combat::new();
        combat.set_bounds(map.bounds[0], map.bounds[1]);
        combat.player.position.x = map.spawn_x;
        combat.enemy.position.x = map.enemy_x;
        let limit = ((map.bounds[1] - map.spawn_x) / PLAYER_RUN_SPEED * TICKS_PER_SECOND as f32)
            .ceil() as usize
            + 120;
        for _ in 0..limit {
            combat.tick_exploration(CombatInput {
                movement: 1.0,
                ..Default::default()
            });
            assert!((map.bounds[0]..=map.bounds[1]).contains(&combat.player.position.x));
        }
        assert_eq!(combat.player.position.x, map.bounds[1]);
        assert!(combat.player.position.x > LEVEL_WIDTH);
        assert!(!combat.enemy_awake);

        combat.player.position.x = map.bounds[1] - 120.0;
        combat.player.velocity.x = 0.0;
        combat.enemy.position.x = map.bounds[1] - 12.0;
        let target_before = combat.enemy.position.x;
        let hp_before = combat.enemy.hp;
        combat.tick(CombatInput {
            kick_pressed: true,
            ..Default::default()
        });
        for _ in 0..10 {
            combat.tick(CombatInput::default());
        }
        assert_eq!(combat.last_hit.unwrap().target, ActorKind::Erratic);
        assert!(combat.enemy.hp < hp_before);
        assert!(combat.enemy.velocity.x > 0.0);
        for _ in 0..8 {
            combat.tick(CombatInput::default());
            assert!(combat.player.position.x > LEVEL_WIDTH);
            assert!((target_before..=map.bounds[1]).contains(&combat.enemy.position.x));
        }
        assert_eq!(combat.enemy.position.x, map.bounds[1]);
    }

    #[test]
    fn rejects_unreachable_trigger_and_duplicate_instance_before_loading() {
        let source = include_str!("../../assets/adventure/world/map.json");
        let mut value: serde_json::Value = serde_json::from_str(source).unwrap();
        value["prologue"]["arrival_x"] = serde_json::json!(40000);
        assert!(Landscape::from_json(&value.to_string()).is_err());
        let mut value: serde_json::Value = serde_json::from_str(source).unwrap();
        let duplicate = value["scenes"][0]["instances"][0].clone();
        value["scenes"][0]["instances"]
            .as_array_mut()
            .unwrap()
            .push(duplicate);
        assert!(Landscape::from_json(&value.to_string()).is_err());
    }
}
