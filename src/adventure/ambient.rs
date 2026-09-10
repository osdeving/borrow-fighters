//! Animates ordinary street life and its one-way evacuation after the EP appears.
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
/// Updates spent slowing a bicycle before its rider gets off.
pub const CYCLIST_BRAKE_TICKS: u32 = 24;
/// Updates spent stepping off and abandoning the bicycle.
pub const CYCLIST_DISMOUNT_TICKS: u32 = 30;
/// Reaction age when bicycles first become independently abandoned objects.
pub const BICYCLE_DROP_TICK: u32 = CYCLIST_BRAKE_TICKS;
/// Reaction age when both former cyclists start escaping on foot.
pub const CYCLIST_RUN_TICK: u32 = CYCLIST_BRAKE_TICKS + CYCLIST_DISMOUNT_TICKS;
/// Reaction age when the released bicycles finish tipping onto the pavement.
pub const BICYCLE_FALL_TICK: u32 = CYCLIST_RUN_TICK + 16;
/// Updates over which ordinary vehicles accelerate to their escape speed.
pub const TRAFFIC_ACCELERATION_TICKS: u32 = 120;
/// Conservative age after which all moving background people and traffic are gone.
pub const STREET_EVACUATED_TICK: u32 = 6 * TICKS_PER_SECOND;

const TRAFFIC_MARGIN: f32 = 260.0;
const TRAFFIC_CRUISE_SPEED: f32 = 4.0;
const TRAFFIC_ESCAPE_SPEED: f32 = 11.0;
const CYCLIST_RUN_SPEED: f32 = 430.0 / TICKS_PER_SECOND as f32;
const RUNNER_EXIT_MARGIN: f32 = 160.0;

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

/// Ordered actions of a cyclist who abandons the bicycle and escapes on foot.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CyclistPhase {
    /// Ordinary cycling before the threat appears.
    Riding,
    /// Slowing continuously from the position occupied at the first alarm.
    Braking,
    /// Leaving the stopped bicycle and turning toward the nearest street exit.
    Dismounting,
    /// Running away from the abandoned bicycle.
    Running,
    /// Beyond the street boundary, without respawning during this encounter.
    Gone,
}

/// A purely visual sample of one cyclist's current pass.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cyclist {
    /// Stable actor index, retaining its visual identity after dismounting.
    pub id: usize,
    /// Rider feet or wheel center before dismounting, in world coordinates.
    pub position: Vec2,
    /// Rider orientation; running may turn away from the original cycling direction.
    pub facing: Facing,
    /// Pedaling clock, with independent phase for the second cyclist.
    pub animation_ticks: u32,
    /// Cycling, abandonment or escape action.
    pub phase: CyclistPhase,
    /// Elapsed updates within the current action.
    pub phase_ticks: u32,
    /// Bicycle's wheel center, fixed permanently once braking finishes.
    pub bicycle_position: Vec2,
    /// Whether the rider remains in the scene; abandoned bicycles are separate.
    pub visible: bool,
}

/// One abandoned bicycle, remaining where the rider finished braking.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AbandonedBicycle {
    /// Stable index of the original cyclist and its bicycle.
    pub id: usize,
    /// Wheel center at the final stopping location.
    pub position: Vec2,
    /// Original cycling direction, retained while the rider may run the other way.
    pub facing: Facing,
    /// Updates since dismounting began, useful for settling the fallen bicycle.
    pub drop_ticks: u32,
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
    /// Stable silhouette: zero hatch, one sedan, two pickup, three SUV, four bus.
    pub style: usize,
    /// Whether this vehicle is accelerating away from the threat.
    pub fleeing: bool,
    /// Whether it has yet to leave the stage; escaped vehicles never respawn.
    pub visible: bool,
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
    reaction_origin_ticks: u32,
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
            reaction_origin_ticks: 0,
        }
    }

    /// Advances once while the street is active; pause omits this call entirely.
    pub fn tick(&mut self, enemy_awake: bool) {
        if self.reaction_ticks.is_none() && enemy_awake {
            // Capture the exact pre-alarm positions, including on a potential
            // wrapping update. Every subsequent path uses this frozen origin.
            self.reaction_origin_ticks = self.ticks;
        }
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

    /// Five spaced silhouettes cruise normally, then accelerate away without wrapping.
    ///
    /// The first alarm captures every current position, even after a long wait.
    /// All vehicles retain their far lane, separated from the permanent wreck.
    pub fn traffic_cars(&self) -> [TrafficCar; 5] {
        let origin_ticks = self
            .reaction_ticks
            .map_or(self.ticks, |_| self.reaction_origin_ticks);
        let travelled = self.reaction_ticks.map_or(0.0, traffic_escape_distance);
        std::array::from_fn(|style| {
            let x = (calm_car_x(origin_ticks, style) - travelled).max(-TRAFFIC_MARGIN);
            TrafficCar {
                position: Vec2::new(x, FAR_ROAD_FLOOR_Y),
                facing: Facing::Left,
                animation_ticks: self.ticks.saturating_add(style as u32 * 8),
                style,
                fleeing: self.reaction_ticks.is_some(),
                visible: self.reaction_ticks.is_none() || x > -TRAFFIC_MARGIN,
            }
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

    /// Two cycling passes that brake, abandon their bicycles and leave on foot.
    pub fn cyclists(&self) -> [Cyclist; 2] {
        std::array::from_fn(|id| self.cyclist(id))
    }

    /// Bicycles left at the stopping positions, persisting after their owners escape.
    pub fn abandoned_bicycles(&self) -> [Option<AbandonedBicycle>; 2] {
        let drop_ticks = self
            .reaction_ticks
            .and_then(|ticks| ticks.checked_sub(BICYCLE_DROP_TICK));
        std::array::from_fn(|id| {
            drop_ticks.map(|drop_ticks| AbandonedBicycle {
                id,
                position: self.cyclist(id).bicycle_position,
                facing: cycle_facing(id),
                drop_ticks,
            })
        })
    }

    fn cyclist(&self, id: usize) -> Cyclist {
        let origin_ticks = self
            .reaction_ticks
            .map_or(self.ticks, |_| self.reaction_origin_ticks);
        let origin = calm_cycle_position(origin_ticks, id);
        let mut cyclist = Cyclist {
            id,
            position: origin,
            facing: cycle_facing(id),
            animation_ticks: self.ticks.saturating_add(id as u32 * 10),
            phase: CyclistPhase::Riding,
            phase_ticks: self.ticks,
            bicycle_position: origin,
            visible: true,
        };
        let Some(age) = self.reaction_ticks else {
            return cyclist;
        };
        let braking_ticks = age.min(CYCLIST_BRAKE_TICKS) as f32;
        let brake_distance = cycle_speed(id)
            * (braking_ticks - braking_ticks * braking_ticks / (2.0 * CYCLIST_BRAKE_TICKS as f32));
        cyclist.bicycle_position.x += brake_distance * cyclist.facing.sign();
        cyclist.position = cyclist.bicycle_position;
        if age < CYCLIST_BRAKE_TICKS {
            cyclist.phase = CyclistPhase::Braking;
            cyclist.phase_ticks = age;
            return cyclist;
        }
        // Turn while stepping off; no position changes until the first running update.
        cyclist.facing = if cyclist.bicycle_position.x < LEVEL_WIDTH * 0.5 {
            Facing::Left
        } else {
            Facing::Right
        };
        if age < CYCLIST_RUN_TICK {
            cyclist.phase = CyclistPhase::Dismounting;
            cyclist.phase_ticks = age - CYCLIST_BRAKE_TICKS;
            return cyclist;
        }
        let run_ticks = age - CYCLIST_RUN_TICK;
        let exit = if cyclist.facing == Facing::Left {
            -RUNNER_EXIT_MARGIN
        } else {
            LEVEL_WIDTH + RUNNER_EXIT_MARGIN
        };
        let distance = ((exit - cyclist.bicycle_position.x) * cyclist.facing.sign()).max(0.0);
        let travelled = (run_ticks as f32 * CYCLIST_RUN_SPEED).min(distance);
        cyclist.position.x += travelled * cyclist.facing.sign();
        cyclist.visible = travelled < distance;
        cyclist.phase = if cyclist.visible {
            CyclistPhase::Running
        } else {
            CyclistPhase::Gone
        };
        cyclist.phase_ticks = if cyclist.visible {
            run_ticks
        } else {
            run_ticks.saturating_sub((distance / CYCLIST_RUN_SPEED).ceil() as u32)
        };
        cyclist
    }
}

fn calm_car_x(ticks: u32, style: usize) -> f32 {
    let span = f64::from(LEVEL_WIDTH + TRAFFIC_MARGIN * 2.0);
    (span
        - f64::from(TRAFFIC_MARGIN)
        - (f64::from(ticks) * f64::from(TRAFFIC_CRUISE_SPEED) + 240.0 + style as f64 * span / 5.0)
            % span) as f32
}

fn traffic_escape_distance(age: u32) -> f32 {
    let accelerated_ticks = age.min(TRAFFIC_ACCELERATION_TICKS) as f32;
    let acceleration =
        (TRAFFIC_ESCAPE_SPEED - TRAFFIC_CRUISE_SPEED) / TRAFFIC_ACCELERATION_TICKS as f32;
    TRAFFIC_CRUISE_SPEED * accelerated_ticks
        + acceleration * accelerated_ticks * accelerated_ticks * 0.5
        + age.saturating_sub(TRAFFIC_ACCELERATION_TICKS) as f32 * TRAFFIC_ESCAPE_SPEED
}

fn calm_cycle_position(ticks: u32, id: usize) -> Vec2 {
    let span = f64::from(LEVEL_WIDTH) + 320.0;
    let x = if id == 0 {
        (f64::from(ticks) * 2.35 + 520.0) % span - 160.0
    } else {
        span - 160.0 - (f64::from(ticks) * 1.85 + 720.0) % span
    };
    Vec2::new(x as f32, CYCLE_LANE_FLOOR_Y - id as f32 * 13.0)
}

fn cycle_facing(id: usize) -> Facing {
    if id == 0 { Facing::Right } else { Facing::Left }
}

fn cycle_speed(id: usize) -> f32 {
    if id == 0 { 2.35 } else { 1.85 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn traffic_stays_continuous_and_never_crashes_before_the_threat() {
        let mut quiet = AmbientState::default();
        let span = LEVEL_WIDTH + TRAFFIC_MARGIN * 2.0;
        let mut previous = quiet.traffic_cars();
        for _ in 0..3000 {
            quiet.tick(false);
            assert_eq!(quiet.incident_car(), None);
            assert_eq!(quiet.accident_ticks(), None);
            let current = quiet.traffic_cars();
            for (before, after) in previous.iter().zip(current) {
                assert_eq!(after.facing, Facing::Left);
                assert_eq!(after.position.y, FAR_ROAD_FLOOR_Y);
                assert!(!after.fleeing);
                assert!(after.visible);
                let travelled = (before.position.x - after.position.x).rem_euclid(span);
                assert!((travelled - 4.0).abs() < 0.001);
                assert!(
                    (-TRAFFIC_MARGIN..=LEVEL_WIDTH + TRAFFIC_MARGIN).contains(&after.position.x)
                );
                if after.position.x > before.position.x {
                    assert!(before.position.x < -TRAFFIC_MARGIN + 10.0);
                    assert!(after.position.x > LEVEL_WIDTH + TRAFFIC_MARGIN - 10.0);
                }
            }
            assert_eq!(current.map(|car| car.style), [0, 1, 2, 3, 4]);
            for pair in current.windows(2) {
                let spacing = (pair[0].position.x - pair[1].position.x).rem_euclid(span);
                assert!(spacing > 500.0, "The bus needs room between silhouettes");
            }
            previous = current;
        }
    }

    #[test]
    fn late_alarms_capture_exact_positions_then_clear_the_street_without_respawning() {
        for delay in [0, 517, 629, 680, 1661, 12001, u32::MAX - 5] {
            let mut ambient = AmbientState {
                ticks: delay,
                ..AmbientState::default()
            };
            let before_cars = ambient.traffic_cars();
            let before_cyclists = ambient.cyclists();
            ambient.tick(true);
            for (before, after) in before_cars.iter().zip(ambient.traffic_cars()) {
                assert_eq!(before.position, after.position);
                assert!(after.fleeing);
            }
            for (before, after) in before_cyclists.iter().zip(ambient.cyclists()) {
                assert_eq!(before.position, after.position);
                assert_eq!(before.bicycle_position, after.bicycle_position);
                assert_eq!(after.phase, CyclistPhase::Braking);
            }

            let mut previous_cars = ambient.traffic_cars();
            let mut previous_travel = [0.0; 5];
            for age in 1..=STREET_EVACUATED_TICK {
                ambient.tick(false);
                let current = ambient.traffic_cars();
                for (id, (before, after)) in previous_cars.iter().zip(current).enumerate() {
                    assert!(
                        after.position.x <= before.position.x,
                        "Escaping traffic must never wrap"
                    );
                    if !before.visible {
                        assert!(!after.visible);
                        assert_eq!(after.position, before.position);
                    }
                    let travel = before.position.x - after.position.x;
                    if after.visible && age < TRAFFIC_ACCELERATION_TICKS {
                        assert!(travel > previous_travel[id]);
                    }
                    previous_travel[id] = travel;
                }
                previous_cars = current;
            }
            let bicycles = ambient
                .abandoned_bicycles()
                .map(|bike| bike.unwrap().position);
            let escaped_cars = ambient.traffic_cars().map(|car| car.position);
            let escaped_people = ambient.cyclists().map(|cyclist| cyclist.position);
            for tick in 0..3000 {
                ambient.tick(tick % 2 == 0);
                assert!(ambient.traffic_cars().iter().all(|car| !car.visible));
                assert!(
                    ambient
                        .cyclists()
                        .iter()
                        .all(|cyclist| !cyclist.visible && cyclist.phase == CyclistPhase::Gone)
                );
                assert_eq!(ambient.traffic_cars().map(|car| car.position), escaped_cars);
                assert_eq!(
                    ambient.cyclists().map(|cyclist| cyclist.position),
                    escaped_people
                );
                assert_eq!(
                    ambient
                        .abandoned_bicycles()
                        .map(|bike| bike.unwrap().position),
                    bicycles
                );
                assert_eq!(ambient.kid_phase(), KidPhase::Gone);
                assert_eq!(
                    ambient.incident_car().unwrap().phase,
                    IncidentPhase::Crashed
                );
            }
        }
    }

    #[test]
    fn both_cyclists_brake_then_leave_their_bicycles_and_run_to_an_exit() {
        let mut ambient = AmbientState {
            ticks: 500,
            ..AmbientState::default()
        };
        let original = ambient.cyclists();
        ambient.tick(true);
        let mut previous = original;
        let mut previous_speed = [f32::MAX; 2];
        for age in 1..=CYCLIST_BRAKE_TICKS {
            ambient.tick(false);
            let current = ambient.cyclists();
            for (id, (before, after)) in previous.iter().zip(current).enumerate() {
                let speed = (after.position.x - before.position.x).abs();
                assert!(speed > 0.0 && speed < previous_speed[id]);
                assert_eq!(after.bicycle_position, after.position);
                previous_speed[id] = speed;
            }
            if age < BICYCLE_DROP_TICK {
                assert_eq!(ambient.abandoned_bicycles(), [None, None]);
            }
            previous = current;
        }
        let bicycles = ambient.abandoned_bicycles().map(Option::unwrap);
        for (id, bicycle) in bicycles.iter().enumerate() {
            assert_eq!(bicycle.id, original[id].id);
            assert_eq!(bicycle.facing, original[id].facing);
            assert_eq!(bicycle.drop_ticks, 0);
            assert_eq!(ambient.cyclists()[id].phase, CyclistPhase::Dismounting);
        }
        for _ in 0..CYCLIST_DISMOUNT_TICKS {
            ambient.tick(false);
        }
        let runners = ambient.cyclists();
        for (runner, bicycle) in runners.iter().zip(bicycles) {
            assert_eq!(runner.phase, CyclistPhase::Running);
            assert_eq!(runner.phase_ticks, 0);
            assert_eq!(runner.position, bicycle.position);
        }
        ambient.tick(false);
        for (before, after) in runners.iter().zip(ambient.cyclists()) {
            assert!((after.position.x - before.position.x) * after.facing.sign() > 0.0);
            assert_eq!(after.bicycle_position, before.bicycle_position);
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
