//! Animates street traffic, its scripted accident and the kite child's escape.
//!
//! System: Adventure scenery. These fixed-update clocks and visual positions
//! consume the encounter's awakening signal without owning bodies or contacts.

use super::combat::{Facing, LEVEL_WIDTH, TICKS_PER_SECOND};
use crate::math::vec2::Vec2;

/// The child's starting horizontal position, behind Rust's approach to danger.
pub const KID_ORIGIN_X: f32 = 1000.0;
/// Feet baseline of the background sidewalk, above the playable street.
pub const KID_FLOOR_Y: f32 = 350.0;
/// Near cycling lane baseline, separated in depth from Rust's floor.
pub const CYCLE_LANE_FLOOR_Y: f32 = 480.0;
/// Wheels baseline for the incident car, behind the protected cycling lane.
pub const CAR_ROAD_FLOOR_Y: f32 = 430.0;
/// Wheels baseline for ordinary traffic travelling in the other road lane.
pub const FAR_ROAD_FLOOR_Y: f32 = 393.0;
/// Fixed pole contact point in background world coordinates.
pub const CRASH_POLE_X: f32 = 1330.0;
/// Reaction age at which the approaching driver first sounds the horn.
pub const CAR_HORN_TICK: u32 = 38;
/// Reaction age at which braking begins, shared with the tire sound.
pub const CAR_SKID_TICK: u32 = 78;
/// Reaction age at which the car hits the pole and stays wrecked.
pub const CAR_IMPACT_TICK: u32 = 112;

const INCIDENT_START_X: f32 = -160.0;
const INCIDENT_STOP_X: f32 = CRASH_POLE_X - 80.0;
const BRAKING_TICKS: u32 = CAR_IMPACT_TICK - CAR_SKID_TICK;
const IMPACT_SPEED: f32 = 4.0;
// Constant approach speed followed by continuous deceleration. There is still
// forward speed at contact, so the pole causes the abrupt stop and deformation.
const APPROACH_SPEED: f32 =
    (INCIDENT_STOP_X - INCIDENT_START_X - IMPACT_SPEED * BRAKING_TICKS as f32 * 0.5)
        / (CAR_SKID_TICK as f32 + BRAKING_TICKS as f32 * 0.5);

/// Center where braking begins, anchoring tire marks to the asphalt.
pub const CAR_SKID_START_X: f32 = INCIDENT_START_X + CAR_SKID_TICK as f32 * APPROACH_SPEED;

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

/// A purely visual car travelling through the far road lane.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TrafficCar {
    /// Center between the wheels at ground level, in world coordinates.
    pub position: Vec2,
    /// Driving direction, also used to mirror the illustration.
    pub facing: Facing,
    /// Wheel animation clock with a stable offset for each vehicle.
    pub animation_ticks: u32,
    /// Stable appearance index from zero through two.
    pub style: usize,
}

/// Ordered phases of the approaching driver's response to the erratic entity.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IncidentPhase {
    /// Entering from beyond the left world edge and sounding the horn.
    Approaching,
    /// Braking without enough distance to stop before the pole.
    Braking,
    /// In contact with the pole; deformation and aftermath remain visible.
    Crashed,
}

/// Current sample of the one-shot, right-facing blue incident car.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IncidentCar {
    /// Center between the wheels; the undeformed front is eighty pixels ahead.
    pub position: Vec2,
    /// Current approach, braking or persistent wreck action.
    pub phase: IncidentPhase,
    /// Updates elapsed inside this phase, including the age of impact effects.
    pub phase_ticks: u32,
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

    /// Updates since the accident was triggered, shared by image and audio cues.
    pub fn accident_ticks(&self) -> Option<u32> {
        self.reaction_ticks
    }

    /// Three evenly spaced cars wrap beyond both camera limits in the far lane.
    ///
    /// Their paths never change when the entity wakes. The dedicated accident
    /// uses the empty near lane, so passing traffic cannot cross through a wreck.
    pub fn traffic_cars(&self) -> [TrafficCar; 3] {
        let span = f64::from(LEVEL_WIDTH) + 320.0;
        std::array::from_fn(|style| TrafficCar {
            position: Vec2::new(
                (span
                    - 160.0
                    - (f64::from(self.ticks) * 4.0 + 240.0 + style as f64 * span / 3.0) % span)
                    as f32,
                FAR_ROAD_FLOOR_Y,
            ),
            facing: Facing::Left,
            animation_ticks: self.ticks.saturating_add(style as u32 * 8),
            style,
        })
    }

    /// Samples a continuous offscreen approach, braking arc and permanent wreck.
    ///
    /// The incident car is separate from passing traffic and starts beyond the
    /// left world boundary. Awakening never relocates an already visible car.
    pub fn incident_car(&self) -> Option<IncidentCar> {
        let ticks = self.accident_ticks()?;
        let (x, phase, phase_ticks) = if ticks < CAR_SKID_TICK {
            (
                INCIDENT_START_X + ticks as f32 * APPROACH_SPEED,
                IncidentPhase::Approaching,
                ticks,
            )
        } else if ticks < CAR_IMPACT_TICK {
            let braking_age = ticks - CAR_SKID_TICK;
            let elapsed = braking_age as f32;
            let deceleration = (APPROACH_SPEED - IMPACT_SPEED) / BRAKING_TICKS as f32;
            (
                INCIDENT_START_X + CAR_SKID_TICK as f32 * APPROACH_SPEED + APPROACH_SPEED * elapsed
                    - deceleration * elapsed * elapsed * 0.5,
                IncidentPhase::Braking,
                braking_age,
            )
        } else {
            (
                INCIDENT_STOP_X,
                IncidentPhase::Crashed,
                ticks - CAR_IMPACT_TICK,
            )
        };
        Some(IncidentCar {
            position: Vec2::new(x, CAR_ROAD_FLOOR_Y),
            phase,
            phase_ticks,
        })
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
    fn traffic_stays_continuous_and_never_crashes_before_the_threat() {
        let mut quiet = AmbientState::default();
        let mut threatened = AmbientState::new(true);
        let span = LEVEL_WIDTH + 320.0;
        let mut previous = quiet.traffic_cars();
        for _ in 0..3000 {
            quiet.tick(false);
            threatened.tick(true);
            assert_eq!(quiet.incident_car(), None);
            assert_eq!(quiet.accident_ticks(), None);
            assert_eq!(quiet.traffic_cars(), threatened.traffic_cars());
            let current = quiet.traffic_cars();
            for (before, after) in previous.iter().zip(current) {
                assert_eq!(after.facing, Facing::Left);
                assert_eq!(after.position.y, FAR_ROAD_FLOOR_Y);
                let travelled = (before.position.x - after.position.x).rem_euclid(span);
                assert!((travelled - 4.0).abs() < 0.001);
                assert!((-160.0..=LEVEL_WIDTH + 160.0).contains(&after.position.x));
                if after.position.x > before.position.x {
                    assert!(before.position.x < -150.0);
                    assert!(after.position.x > LEVEL_WIDTH + 150.0);
                }
            }
            previous = current;
        }
    }

    #[test]
    fn incident_enters_continuously_brakes_and_leaves_a_permanent_wreck() {
        let mut ambient = AmbientState::default();
        // Waiting to approach the entity does not change the incident timing.
        for _ in 0..517 {
            ambient.tick(false);
        }
        ambient.tick(true);
        let first = ambient.incident_car().unwrap();
        assert_eq!(ambient.accident_ticks(), Some(0));
        assert_eq!(first.phase, IncidentPhase::Approaching);
        assert!(first.position.x + 80.0 < 0.0);

        let mut previous_x = first.position.x;
        let mut previous_speed = APPROACH_SPEED;
        for age in 1..=CAR_IMPACT_TICK {
            // A later false signal cannot cancel or restart the choreography.
            ambient.tick(false);
            let car = ambient.incident_car().unwrap();
            let speed = car.position.x - previous_x;
            assert!(speed > 0.0);
            assert!(speed <= APPROACH_SPEED + 0.001);
            assert_eq!(car.position.y, CAR_ROAD_FLOOR_Y);
            if age > CAR_SKID_TICK {
                assert!(speed < previous_speed);
            }
            if age == CAR_SKID_TICK {
                assert_eq!(car.phase, IncidentPhase::Braking);
                assert_eq!(car.phase_ticks, 0);
            }
            if age < CAR_IMPACT_TICK {
                assert!(car.position.x + 80.0 < CRASH_POLE_X);
                assert_ne!(car.phase, IncidentPhase::Crashed);
            }
            previous_speed = speed;
            previous_x = car.position.x;
        }
        let wreck = ambient.incident_car().unwrap();
        assert_eq!(wreck.phase, IncidentPhase::Crashed);
        assert_eq!(wreck.phase_ticks, 0);
        assert_eq!(wreck.position.x + 80.0, CRASH_POLE_X);
        for age in 1..=600 {
            ambient.tick(age % 2 == 0);
            let car = ambient.incident_car().unwrap();
            assert_eq!(car.position, wreck.position);
            assert_eq!(car.phase, IncidentPhase::Crashed);
            assert_eq!(car.phase_ticks, age);
        }
    }

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
