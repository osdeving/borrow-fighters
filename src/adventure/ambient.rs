//! Animates the street's cyclists and the kite child's one-way escape.
//!
//! System: Adventure scenery. These fixed-update clocks and visual positions
//! consume the encounter's awakening signal without owning bodies or contacts.

use super::combat::{Facing, LEVEL_WIDTH, TICKS_PER_SECOND};
use crate::math::vec2::Vec2;

/// The child's starting horizontal position, behind Rust's approach to danger.
pub const KID_ORIGIN_X: f32 = 1000.0;
/// Feet baseline of the background sidewalk, above the playable street.
pub const KID_FLOOR_Y: f32 = 420.0;
/// Near cycling lane baseline, separated in depth from Rust's floor.
pub const CYCLE_LANE_FLOOR_Y: f32 = 480.0;

const STARTLED_TICKS: u32 = 24;
const RELEASING_TICKS: u32 = 24;
const RUNNING_TICKS: u32 = 256;
const RUN_START: u32 = STARTLED_TICKS + RELEASING_TICKS;
const GONE_START: u32 = RUN_START + RUNNING_TICKS;
const RUN_SPEED: f32 = 270.0 / TICKS_PER_SECOND as f32;

/// Ordered phases of the child's response to the erratic entity.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KidPhase {
    /// Quietly flying the kite before the creature notices Rust.
    Playing,
    /// Freezing briefly and looking toward the disturbance.
    Startled,
    /// Letting go of the line before turning to run.
    Releasing,
    /// Running left along the background sidewalk, away from the entity.
    Running,
    /// Safely offscreen; the child does not return during this encounter.
    Gone,
}

/// A purely visual sample of one cyclist's current pass.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cyclist {
    /// Center between the wheels at ground level, in world coordinates.
    pub position: Vec2,
    /// Travel direction, also used to mirror the cycling frames.
    pub facing: Facing,
    /// Pedaling clock, with independent phase for the second cyclist.
    pub animation_ticks: u32,
}

/// Deterministic street animation owned by the story, outside combat state.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AmbientState {
    ticks: u32,
    reaction_ticks: Option<u32>,
}

impl Default for AmbientState {
    fn default() -> Self {
        Self::new(false)
    }
}

impl AmbientState {
    /// Starts a fresh street scene, reacting immediately at an awake checkpoint.
    pub fn new(enemy_awake: bool) -> Self {
        Self {
            ticks: 0,
            reaction_ticks: enemy_awake.then_some(0),
        }
    }

    /// Advances once while the street is active; pause omits this call entirely.
    pub fn tick(&mut self, enemy_awake: bool) {
        self.ticks = self.ticks.saturating_add(1);
        self.reaction_ticks = match self.reaction_ticks {
            Some(ticks) => Some(ticks.saturating_add(1)),
            None => enemy_awake.then_some(0),
        };
    }

    /// Elapsed street updates, retained across the transition into aftermath.
    pub fn ticks(&self) -> u32 {
        self.ticks
    }

    /// Current child action, with no return to play after the threat appears.
    pub fn kid_phase(&self) -> KidPhase {
        match self.reaction_ticks {
            None => KidPhase::Playing,
            Some(ticks) if ticks < STARTLED_TICKS => KidPhase::Startled,
            Some(ticks) if ticks < RUN_START => KidPhase::Releasing,
            Some(ticks) if ticks < GONE_START => KidPhase::Running,
            Some(_) => KidPhase::Gone,
        }
    }

    /// Elapsed updates inside the current child action, for its animation frames.
    pub fn kid_phase_ticks(&self) -> u32 {
        let Some(ticks) = self.reaction_ticks else {
            return self.ticks;
        };
        let start = match self.kid_phase() {
            KidPhase::Playing | KidPhase::Startled => 0,
            KidPhase::Releasing => STARTLED_TICKS,
            KidPhase::Running => RUN_START,
            KidPhase::Gone => GONE_START,
        };
        ticks - start
    }

    /// Child feet in world space; escape stops beyond the left camera limit.
    pub fn kid_position(&self) -> Vec2 {
        let running_ticks = self
            .reaction_ticks
            .unwrap_or(0)
            .saturating_sub(RUN_START)
            .min(RUNNING_TICKS);
        Vec2::new(KID_ORIGIN_X - running_ticks as f32 * RUN_SPEED, KID_FLOOR_Y)
    }

    /// Updates since the line was released, continuing after the child is gone.
    pub fn kite_release_ticks(&self) -> Option<u32> {
        self.reaction_ticks?.checked_sub(STARTLED_TICKS)
    }

    /// Two cycling passes at different speeds and depths, wrapping offscreen.
    pub fn cyclists(&self) -> [Cyclist; 2] {
        let span = f64::from(LEVEL_WIDTH) + 320.0;
        let ticks = f64::from(self.ticks);
        [
            Cyclist {
                position: Vec2::new(
                    ((ticks * 2.35 + 520.0) % span - 160.0) as f32,
                    CYCLE_LANE_FLOOR_Y,
                ),
                facing: Facing::Right,
                animation_ticks: self.ticks,
            },
            Cyclist {
                position: Vec2::new(
                    (span - 160.0 - (ticks * 1.85 + 720.0) % span) as f32,
                    CYCLE_LANE_FLOOR_Y - 13.0,
                ),
                facing: Facing::Left,
                animation_ticks: self.ticks.saturating_add(10),
            },
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn threat_releases_the_kite_then_child_escapes_without_returning() {
        let mut ambient = AmbientState::default();
        for _ in 0..600 {
            ambient.tick(false);
        }
        assert_eq!(ambient.kid_phase(), KidPhase::Playing);
        assert_eq!(ambient.kite_release_ticks(), None);
        assert_eq!(ambient.kid_position().x, KID_ORIGIN_X);

        ambient.tick(true);
        assert_eq!(ambient.kid_phase(), KidPhase::Startled);
        assert_eq!(ambient.kid_phase_ticks(), 0);
        for _ in 0..STARTLED_TICKS {
            ambient.tick(false);
        }
        assert_eq!(ambient.kid_phase(), KidPhase::Releasing);
        assert_eq!(ambient.kite_release_ticks(), Some(0));
        for _ in 0..RELEASING_TICKS {
            ambient.tick(true);
        }
        assert_eq!(ambient.kid_phase(), KidPhase::Running);
        assert_eq!(ambient.kid_position().x, KID_ORIGIN_X);

        let mut last_x = KID_ORIGIN_X;
        for _ in 0..RUNNING_TICKS {
            ambient.tick(false);
            let position = ambient.kid_position();
            assert!(position.x < last_x);
            assert_eq!(position.y, KID_FLOOR_Y);
            last_x = position.x;
        }
        assert_eq!(ambient.kid_phase(), KidPhase::Gone);
        assert!(last_x < -150.0);
        let release_age = ambient.kite_release_ticks().unwrap();
        for tick in 0..600 {
            ambient.tick(tick % 2 == 0);
            assert_eq!(ambient.kid_phase(), KidPhase::Gone);
            assert_eq!(ambient.kid_position().x, last_x);
        }
        assert_eq!(ambient.kite_release_ticks(), Some(release_age + 600));
    }

    #[test]
    fn cyclists_move_opposite_ways_and_stay_behind_the_playable_floor() {
        let mut ambient = AmbientState::default();
        let first = ambient.cyclists();
        for _ in 0..60 {
            ambient.tick(false);
        }
        let next = ambient.cyclists();
        assert!(next[0].position.x > first[0].position.x);
        assert!(next[1].position.x < first[1].position.x);
        assert_ne!(next[0].position.y, next[1].position.y);
        for _ in 0..3000 {
            ambient.tick(true);
            for cyclist in ambient.cyclists() {
                assert!((-160.0..=LEVEL_WIDTH + 160.0).contains(&cyclist.position.x));
                assert!(cyclist.position.y <= super::super::combat::FLOOR_Y - 100.0);
            }
        }
    }
}
