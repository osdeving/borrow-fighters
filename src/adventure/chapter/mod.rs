//! Owns the playable chapter following Rust's first encounter.
//!
//! System: Adventure chapter. Geometry, safe checkpoints and short authored
//! actions remain pure; the app omits ticks during pause and owns all I/O.

mod checkpoint;
mod direction;
pub mod phone;
mod progression;
mod texts;
pub mod world;

pub use checkpoint::{Checkpoint, CheckpointStage};
pub use texts::ChapterTexts;
pub use world::{Scene, World};

use super::combat::{Action, Actor, Combat, CombatInput, Facing};
use crate::math::vec2::Vec2;
use phone::PhoneView;

/// Updates spent raising or lowering the shop shutter during a conversation.
pub const SHOP_SHUTTER_TICKS: u32 = 90;
/// Time for the neighbour's complete 80px-wide run pose to clear the door jamb.
pub const SHOP_EXIT_CLEARANCE_TICKS: u32 = 48;
/// Length of the quiet opening before the player receives control.
pub const INTRO_TICKS: u32 = 600;

/// The current scene-local narrative or playable action.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Phase {
    /// Quiet continuation after the prologue's events.
    Intro,
    /// Rust may walk to the driver interaction zone.
    ExploreDriver,
    /// Authored walk to the car's depth.
    DriverApproach,
    /// Brief conversation beside the car.
    DriverDialogue,
    /// Return along the same path to the foreground.
    DriverReturn,
    /// Rust may walk to the shop.
    ExploreShop,
    /// Walk to the door while the shutter rises.
    ShopApproach,
    /// Conversation with the shopkeeper.
    ShopDialogue,
    /// Return while the shutter closes.
    ShopReturn,
    /// Rust may find the neighbour who knows Python's work.
    ExploreNeighbour,
    /// Walk to the neighbour's sidewalk depth.
    NeighbourApproach,
    /// Conversation introducing Python.
    NeighbourDialogue,
    /// Return to the foreground before using the phone.
    NeighbourReturn,
    /// Continuous in-world phone acting and messenger panel.
    Phone,
    /// Leave the street through its exit region.
    LeaveStreet,
    /// Jump over the lane obstruction and reach its resident.
    LaneExplore,
    /// Walk clear of the resident before speaking at the same depth.
    LaneApproach,
    /// Learn that the nearby passage needs help.
    LaneDialogue,
    /// Reach the lane exit after hearing the resident.
    LaneExit,
    /// Approach the blocked passage.
    PassageExplore,
    /// A real local encounter; only health and contact can resolve it.
    PassageCombat,
    /// The enemy is defeated and the exit is now reachable.
    PassageClear,
    /// Rust turns toward the next investigation.
    Departure,
    /// The chapter is complete; the app decides when to return to the menu.
    Complete,
}

/// Device-independent commands for one fixed update.
#[derive(Clone, Copy, Debug, Default)]
pub struct ChapterInput {
    /// Held horizontal intent; non-finite values are ignored by movement.
    pub movement: f32,
    /// Fresh jump press.
    pub jump: bool,
    /// Fresh light attack press.
    pub light: bool,
    /// Fresh heavy attack press.
    pub heavy: bool,
    /// Held guard.
    pub block: bool,
    /// Fresh interaction press, distinct from jumping and dialogue advance.
    pub interact: bool,
    /// Fresh dialogue/acting advance press.
    pub advance: bool,
    /// Complete the current noncombat acting sequence at a safe pose.
    pub skip: bool,
    /// Retry a lost passage encounter from its local checkpoint.
    pub retry: bool,
}

impl ChapterInput {
    fn combat(self) -> CombatInput {
        CombatInput {
            movement: self.movement,
            jump_pressed: self.jump,
            light_pressed: self.light,
            heavy_pressed: self.heavy,
            blocking: self.block,
        }
    }
}

/// A world-space camera, always constrained to the current scene.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ChapterCamera {
    /// World point placed at the center of the viewport.
    pub target: Vec2,
    /// Uniform world magnification.
    pub zoom: f32,
}

/// One subtitle line, selected by the model and resolved through ChapterTexts.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Dialogue {
    /// Text key naming the speaker.
    pub speaker: &'static str,
    /// Text key for the current line.
    pub key: &'static str,
    /// Zero-based position in the short conversation.
    pub line_index: usize,
    /// Number of lines in this conversation.
    pub line_count: usize,
}

/// A reachable, currently relevant interaction and its prompt.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Interaction {
    /// Stable identifier matching the world POI.
    pub id: &'static str,
    /// Text key for its input hint.
    pub prompt_key: &'static str,
}

/// One decorative person sampled in the same coordinates as Rust.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ResidentView {
    /// Stable role used by the renderer: driver, shopkeeper, neighbour, lane_resident.
    pub id: &'static str,
    /// Feet or authored support in world coordinates.
    pub position: Vec2,
    /// Horizontal facing.
    pub facing: Facing,
    /// Whether this person should currently be drawn.
    pub visible: bool,
    /// Whether the current line belongs to this resident.
    pub talking: bool,
    /// Whether the person is walking along an authored path.
    pub walking: bool,
    /// Local action clock for their pose animation.
    pub action_ticks: u32,
}

/// Complete pure chapter state; callers sample it but only tick advances it.
#[derive(Clone, Debug)]
pub struct Chapter {
    /// Current separately bounded space.
    pub scene: Scene,
    /// Current narrative or playable action.
    pub phase: Phase,
    /// Updates elapsed inside this action.
    pub phase_ticks: u32,
    /// Total simulation updates since this instance began.
    pub ticks: u32,
    /// Reused adventure bodies; the enemy is active only in the passage.
    pub combat: Combat,
    /// Validated scene geometry used by both model and presentation.
    pub world: World,
    pub(super) camera: ChapterCamera,
    pub(super) saved: Checkpoint,
    pub(super) route: Vec<Vec2>,
    pub(super) route_index: usize,
    pub(super) return_route: Vec<Vec2>,
    pub(super) dialogue_line: usize,
    pub(super) neighbour_ticks: Option<u32>,
    pub(super) residents_escape_ticks: Option<u32>,
}

impl Chapter {
    /// Starts the canonical aftermath, without manufacturing a played prologue win.
    pub fn new(world: World, prologue_played_victory: bool) -> Self {
        let spawn = world.scene(Scene::Street).spawn.vec();
        let mut combat = Combat::new();
        combat.player.position = spawn;
        let geometry = world.scene(Scene::Street);
        let detail_x = geometry.poi("driver").expect("validated driver").position.x;
        let initial_camera = ChapterCamera {
            target: Vec2::new(
                detail_x.clamp(640.0 / 1.5, geometry.width - 640.0 / 1.5),
                340.0,
            ),
            zoom: 1.5,
        };
        Self {
            scene: Scene::Street,
            phase: Phase::Intro,
            phase_ticks: 0,
            ticks: 0,
            combat,
            world,
            camera: initial_camera,
            saved: Checkpoint::new(prologue_played_victory),
            route: Vec::new(),
            route_index: 0,
            return_route: Vec::new(),
            dialogue_line: 0,
            neighbour_ticks: None,
            residents_escape_ticks: None,
        }
    }

    /// Rust's authoritative feet, action, facing and health.
    pub fn player(&self) -> &Actor {
        &self.combat.player
    }

    /// Current constrained world camera.
    pub fn camera(&self) -> ChapterCamera {
        self.camera
    }

    /// Safe serializable progress; no transient pose, phone or renderer state leaks in.
    pub fn checkpoint(&self) -> Checkpoint {
        self.saved
    }

    /// Current foreground/depth scale, derived exclusively from Rust's feet.
    pub fn player_scale(&self) -> f32 {
        if !matches!(
            self.phase,
            Phase::DriverApproach
                | Phase::DriverDialogue
                | Phase::DriverReturn
                | Phase::ShopApproach
                | Phase::ShopDialogue
                | Phase::ShopReturn
                | Phase::NeighbourApproach
                | Phase::NeighbourDialogue
                | Phase::NeighbourReturn
        ) {
            return 1.0;
        }
        (0.58 + (self.player().position.y - 355.0) / 225.0 * 0.42).clamp(0.58, 1.0)
    }

    /// Whether movement and attacks currently belong to the player.
    pub fn controls_active(&self) -> bool {
        self.combat.outcome != super::combat::Outcome::Defeat
            && matches!(
                self.phase,
                Phase::ExploreDriver
                    | Phase::ExploreShop
                    | Phase::ExploreNeighbour
                    | Phase::LeaveStreet
                    | Phase::LaneExplore
                    | Phase::LaneExit
                    | Phase::PassageExplore
                    | Phase::PassageCombat
                    | Phase::PassageClear
            )
    }

    /// The phone is absent outside its authored action, including immediately after skip.
    pub fn phone(&self) -> Option<PhoneView> {
        (self.phase == Phase::Phone)
            .then(|| PhoneView::at(self.phase_ticks))
            .flatten()
    }

    /// Current objective, without embedding product copy in state transitions.
    pub fn objective_key(&self) -> &'static str {
        match self.phase {
            Phase::Intro
            | Phase::ExploreDriver
            | Phase::DriverApproach
            | Phase::DriverDialogue
            | Phase::DriverReturn => "objective.driver",
            Phase::ExploreShop | Phase::ShopApproach | Phase::ShopDialogue | Phase::ShopReturn => {
                "objective.shop"
            }
            Phase::ExploreNeighbour
            | Phase::NeighbourApproach
            | Phase::NeighbourDialogue
            | Phase::NeighbourReturn => "objective.neighbour",
            Phase::Phone => "objective.phone",
            Phase::LeaveStreet => "objective.leave_street",
            Phase::LaneExplore | Phase::LaneApproach | Phase::LaneDialogue => "objective.lane",
            Phase::LaneExit => "objective.lane_exit",
            Phase::PassageExplore => "objective.passage",
            Phase::PassageCombat => "objective.protect",
            Phase::PassageClear | Phase::Departure => "objective.clear",
            Phase::Complete => "objective.complete",
        }
    }

    pub(super) fn enter(&mut self, phase: Phase) {
        if self.phase == Phase::Intro && phase == Phase::ExploreDriver {
            self.camera = ChapterCamera {
                target: Vec2::new(
                    self.player()
                        .position
                        .x
                        .clamp(640.0, self.world.scene(self.scene).width - 640.0),
                    360.0,
                ),
                zoom: 1.0,
            };
        }
        self.phase = phase;
        self.phase_ticks = 0;
        self.dialogue_line = 0;
        self.combat.player.velocity = Vec2::ZERO;
        if phase != Phase::PassageCombat {
            self.combat.player.action = Action::Idle;
            self.combat.player.action_ticks = 0;
        }
        let target = match phase {
            Phase::DriverDialogue => Some("driver"),
            Phase::ShopDialogue => Some("shop"),
            Phase::NeighbourDialogue => Some("neighbour"),
            Phase::LaneDialogue => Some("lane_resident"),
            _ => None,
        };
        if let Some(target) = target {
            let x = self
                .world
                .scene(self.scene)
                .poi(target)
                .expect("validated speaker")
                .position
                .x;
            self.combat.player.facing = if x < self.player().position.x {
                Facing::Left
            } else {
                Facing::Right
            };
        }
    }
}

#[cfg(test)]
mod tests;
