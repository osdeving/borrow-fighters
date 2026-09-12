//! Stores only safe chapter boundaries and the provenance of the prologue result.
//!
//! System: Adventure chapter persistence model. Serialization is pure; file
//! locations, atomic writes and menu routing belong to the hosting application.

use super::{Chapter, Phase, Scene, World};
use crate::adventure::combat::{Action, Combat, Outcome};
use crate::math::vec2::Vec2;
use serde::{Deserialize, Serialize};

/// Stable milestones at which quitting or defeat can safely resume.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckpointStage {
    /// Before checking the driver, after the quiet introductory shot.
    StreetStart,
    /// Driver conversation completed; approach the shop next.
    DriverChecked,
    /// Shop conversation completed; the neighbour has come outside.
    ShopChecked,
    /// Neighbour conversation completed; phone sequence may begin again safely.
    ContactReady,
    /// All three messages sent/received; Rust may leave the street.
    PhoneDone,
    /// Entered the lane before its small platforming task.
    LaneStart,
    /// Heard the lane resident and may continue to the passage.
    LaneCleared,
    /// Entered the passage before the enemy notices Rust.
    PassageStart,
    /// The local encounter has started; retry does not replay conversations.
    PassageFight,
    /// The passage was cleared through actual health/contact resolution.
    PassageCleared,
    /// Reached the end of this chapter.
    Complete,
}

/// Versioned save payload with no transient actors or resource handles.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Checkpoint {
    /// Save schema version; incompatible versions are rejected.
    pub version: u32,
    /// Most recent complete safe milestone.
    pub stage: CheckpointStage,
    /// True only if the hosting flow observed the player win the prologue.
    pub prologue_played_victory: bool,
}

impl Checkpoint {
    /// Starts the chapter's canonical aftermath independently of prologue statistics.
    pub fn new(prologue_played_victory: bool) -> Self {
        Self {
            version: 1,
            stage: CheckpointStage::StreetStart,
            prologue_played_victory,
        }
    }

    /// Rejects incompatible saves before the app attempts to resume them.
    pub fn validate(&self) -> Result<(), String> {
        if self.version != 1 {
            return Err(format!("Unsupported chapter save version {}", self.version));
        }
        Ok(())
    }
}

impl Chapter {
    /// Recreates a safe pose and local scene, never a half-played phone gesture.
    pub fn from_checkpoint(world: World, checkpoint: Checkpoint) -> Result<Self, String> {
        checkpoint.validate()?;
        let mut chapter = Self::new(world, checkpoint.prologue_played_victory);
        chapter.saved = checkpoint;
        use CheckpointStage as C;
        let (scene, phase, point): (Scene, Phase, Option<&str>) = match checkpoint.stage {
            C::StreetStart => (Scene::Street, Phase::ExploreDriver, None),
            C::DriverChecked => (Scene::Street, Phase::ExploreShop, Some("driver")),
            C::ShopChecked => (Scene::Street, Phase::ExploreNeighbour, Some("shop")),
            C::ContactReady => (Scene::Street, Phase::Phone, Some("neighbour")),
            C::PhoneDone => (Scene::Street, Phase::LeaveStreet, Some("neighbour")),
            C::LaneStart => (Scene::Lane, Phase::LaneExplore, None),
            C::LaneCleared => (Scene::Lane, Phase::LaneExit, Some("lane_resident")),
            C::PassageStart => (Scene::Passage, Phase::PassageExplore, None),
            C::PassageFight => (Scene::Passage, Phase::PassageCombat, Some("enemy")),
            C::PassageCleared => (Scene::Passage, Phase::PassageClear, Some("enemy")),
            C::Complete => (Scene::Passage, Phase::Complete, None),
        };
        chapter.scene = scene;
        chapter.reset_scene_content();
        chapter.enter(phase);
        let geometry = chapter.world.scene(scene);
        chapter.combat.player.position = point
            .map(|id| {
                let region = geometry
                    .poi(id)
                    .expect("validated checkpoint target")
                    .region;
                Vec2::new(region.x + region.width * 0.5, geometry.floor_y)
            })
            .unwrap_or(geometry.spawn.vec());
        if scene == Scene::Passage {
            chapter.combat.enemy_awake = phase == Phase::PassageCombat;
            if matches!(phase, Phase::PassageClear | Phase::Complete) {
                for enemy in chapter.combat.enemies_mut() {
                    enemy.hp = 0;
                    enemy.action = Action::Defeated;
                }
                chapter.combat.outcome = Outcome::Victory;
                chapter.residents_escape_ticks = Some(300);
            }
            if phase == Phase::Complete {
                chapter.combat.player.position.x = geometry.exit.x;
            }
        }
        if checkpoint.stage == C::LaneCleared {
            for piece in &mut chapter.debris {
                piece.hp = 0;
            }
        }
        if !matches!(checkpoint.stage, C::StreetStart | C::DriverChecked) {
            chapter.neighbour_ticks = Some(120);
        }
        chapter.camera.target.x = chapter
            .player()
            .position
            .x
            .clamp(640.0, geometry.width - 640.0);
        chapter.camera.target.y = 360.0;
        chapter.camera.zoom = 1.0;
        Ok(chapter)
    }

    pub(super) fn save_at(&mut self, stage: CheckpointStage) {
        self.saved.stage = stage;
    }

    pub(super) fn begin_scene(&mut self, scene: Scene, phase: Phase, stage: CheckpointStage) {
        self.scene = scene;
        self.combat = Combat::new();
        self.combat.player.position = self.world.scene(scene).spawn.vec();
        self.reset_scene_content();
        self.enter(phase);
        self.save_at(stage);
        self.camera.target = Vec2::new(
            self.player()
                .position
                .x
                .clamp(640.0, self.world.scene(scene).width - 640.0),
            360.0,
        );
        self.camera.zoom = 1.0;
    }

    pub(super) fn reset_scene_content(&mut self) {
        let geometry = self.world.scene(self.scene);
        self.combat.set_bounds(geometry.walk_min, geometry.walk_max);
        self.debris = geometry
            .debris
            .iter()
            .cloned()
            .map(super::debris::Debris::new)
            .collect();
        if !geometry.enemies.is_empty() {
            self.combat.configure_enemies(
                &geometry
                    .enemies
                    .iter()
                    .map(|entry| (entry.position.vec(), entry.tuning))
                    .collect::<Vec<_>>(),
            );
        }
    }

    /// Restores only a genuine local defeat, leaving live combat health unchanged.
    pub fn retry(&mut self) {
        if self.phase == Phase::PassageCombat && self.combat.outcome == Outcome::Defeat {
            *self = Self::from_checkpoint(self.world.clone(), self.saved)
                .expect("own checkpoint is valid");
        }
    }
}
