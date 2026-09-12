//! Samples Augusta's pedestrians, bar conversations and traffic from story time.
//!
//! System: C++ adventure ambience. Decorative adults have stable identities and
//! articulated movement; the first EP arrival starts a one-way evacuation.

use std::f32::consts::{PI, TAU};

/// Street extent shared by the authored Augusta scene.
pub const STREET_WIDTH: f32 = 3600.0;
/// Adult profile width relative to the articulated body's vertical scale.
pub const PERSON_WIDTH_RATIO: f32 = 0.82;
const OFFSCREEN_MARGIN: f32 = 220.0;
// Whole cars must clear the street before wrapping or disappearing in escape.
const VEHICLE_MARGIN: f32 = 320.0;

/// Adult wardrobe and silhouette variants, separate from narrative characters.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wardrobe {
    Leather,
    PlumDress,
    AmberJacket,
    Denim,
    TealDress,
    WhiteShirt,
    RedBlouse,
    LongCoat,
}

/// Everyday activity visible before the street reacts to the EPs.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Activity {
    Walking,
    Conversation,
    Seated,
    Phone,
}

/// Articulated local points evaluated without a renderer or elapsed wall time.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PersonPose {
    pub feet: [[f32; 2]; 2],
    pub hands: [[f32; 2]; 2],
    pub bob: f32,
    pub lean: f32,
    pub seated: f32,
    pub alarm: f32,
}

/// One stable decorative person in Augusta's world coordinates.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pedestrian {
    pub id: usize,
    pub x: f32,
    pub ground_y: f32,
    pub scale: f32,
    pub facing: f32,
    pub wardrobe: Wardrobe,
    pub activity: Activity,
    pub skin: usize,
    pub hair: usize,
    pub pose: PersonPose,
    pub fleeing: bool,
}

/// Traffic silhouettes differ in shape as well as paint.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VehicleKind {
    Hatch,
    Taxi,
    DeliveryScooter,
}

impl VehicleKind {
    fn scale(self) -> f32 {
        match self {
            Self::Hatch => 2.55,
            Self::Taxi => 2.35,
            Self::DeliveryScooter => 2.15,
        }
    }

    fn wheel_radius(self) -> f32 {
        if self == Self::DeliveryScooter {
            9.0
        } else {
            11.0
        }
    }
}

/// A vehicle's position and wheel rotation share the same traveled distance.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vehicle {
    pub id: usize,
    pub kind: VehicleKind,
    pub x: f32,
    pub ground_y: f32,
    pub facing: f32,
    pub wheel_angle: f32,
    pub scale: f32,
    pub fleeing: bool,
}

/// A snapshot reused unchanged by cinematic cameras and side-view gameplay.
#[derive(Clone, Debug, PartialEq)]
pub struct Nightlife {
    pub people: Vec<Pedestrian>,
    pub vehicles: Vec<Vehicle>,
    pub ticks: u64,
    pub threat_age: Option<u64>,
}

#[derive(Clone, Copy)]
struct PersonSpec {
    x: f32,
    y: f32,
    facing: f32,
    wardrobe: Wardrobe,
    activity: Activity,
}

// Leave the Limiar doorway (1514) and Julia/broker blocking free of stationary
// extras. The three pairs outside the mural belong to the adult night crowd.
const PEOPLE: [PersonSpec; 22] = [
    PersonSpec {
        x: 155.0,
        y: 502.0,
        facing: 1.0,
        wardrobe: Wardrobe::Denim,
        activity: Activity::Walking,
    },
    PersonSpec {
        x: 402.0,
        y: 500.0,
        facing: 1.0,
        wardrobe: Wardrobe::RedBlouse,
        activity: Activity::Conversation,
    },
    PersonSpec {
        x: 440.0,
        y: 503.0,
        facing: -1.0,
        wardrobe: Wardrobe::Leather,
        activity: Activity::Conversation,
    },
    PersonSpec {
        x: 586.0,
        y: 501.0,
        facing: 1.0,
        wardrobe: Wardrobe::WhiteShirt,
        activity: Activity::Seated,
    },
    PersonSpec {
        x: 644.0,
        y: 505.0,
        facing: -1.0,
        wardrobe: Wardrobe::TealDress,
        activity: Activity::Seated,
    },
    PersonSpec {
        x: 802.0,
        y: 507.0,
        facing: -1.0,
        wardrobe: Wardrobe::AmberJacket,
        activity: Activity::Walking,
    },
    PersonSpec {
        x: 1030.0,
        y: 501.0,
        facing: 1.0,
        wardrobe: Wardrobe::PlumDress,
        activity: Activity::Conversation,
    },
    PersonSpec {
        x: 1068.0,
        y: 504.0,
        facing: -1.0,
        wardrobe: Wardrobe::Leather,
        activity: Activity::Conversation,
    },
    PersonSpec {
        x: 1200.0,
        y: 500.0,
        facing: -1.0,
        wardrobe: Wardrobe::LongCoat,
        activity: Activity::Phone,
    },
    PersonSpec {
        x: 1360.0,
        y: 503.0,
        facing: 1.0,
        wardrobe: Wardrobe::WhiteShirt,
        activity: Activity::Walking,
    },
    PersonSpec {
        x: 1870.0,
        y: 501.0,
        facing: 1.0,
        wardrobe: Wardrobe::TealDress,
        activity: Activity::Conversation,
    },
    PersonSpec {
        x: 1910.0,
        y: 504.0,
        facing: -1.0,
        wardrobe: Wardrobe::RedBlouse,
        activity: Activity::Conversation,
    },
    PersonSpec {
        x: 2118.0,
        y: 501.0,
        facing: -1.0,
        wardrobe: Wardrobe::Denim,
        activity: Activity::Walking,
    },
    PersonSpec {
        x: 2290.0,
        y: 501.0,
        facing: 1.0,
        wardrobe: Wardrobe::AmberJacket,
        activity: Activity::Conversation,
    },
    PersonSpec {
        x: 2329.0,
        y: 504.0,
        facing: -1.0,
        wardrobe: Wardrobe::PlumDress,
        activity: Activity::Conversation,
    },
    PersonSpec {
        x: 2480.0,
        y: 501.0,
        facing: 1.0,
        wardrobe: Wardrobe::LongCoat,
        activity: Activity::Walking,
    },
    PersonSpec {
        x: 2762.0,
        y: 501.0,
        facing: 1.0,
        wardrobe: Wardrobe::Leather,
        activity: Activity::Seated,
    },
    PersonSpec {
        x: 2820.0,
        y: 505.0,
        facing: -1.0,
        wardrobe: Wardrobe::WhiteShirt,
        activity: Activity::Seated,
    },
    PersonSpec {
        x: 3006.0,
        y: 504.0,
        facing: -1.0,
        wardrobe: Wardrobe::RedBlouse,
        activity: Activity::Walking,
    },
    PersonSpec {
        x: 3200.0,
        y: 501.0,
        facing: 1.0,
        wardrobe: Wardrobe::TealDress,
        activity: Activity::Conversation,
    },
    PersonSpec {
        x: 3239.0,
        y: 504.0,
        facing: -1.0,
        wardrobe: Wardrobe::AmberJacket,
        activity: Activity::Conversation,
    },
    PersonSpec {
        x: 3430.0,
        y: 501.0,
        facing: -1.0,
        wardrobe: Wardrobe::Denim,
        activity: Activity::Phone,
    },
];

fn wrap(x: f32) -> f32 {
    (x + OFFSCREEN_MARGIN).rem_euclid(STREET_WIDTH + OFFSCREEN_MARGIN * 2.0) - OFFSCREEN_MARGIN
}

fn on_street(x: f32) -> bool {
    (-OFFSCREEN_MARGIN..=STREET_WIDTH + OFFSCREEN_MARGIN).contains(&x)
}

fn calm_distance(id: usize, ticks: u64) -> f32 {
    ticks as f32 / 60.0 * (28.0 + (id % 4) as f32 * 5.0)
}

fn calm_x(id: usize, spec: PersonSpec, ticks: u64) -> f32 {
    if spec.activity == Activity::Walking {
        wrap(spec.x + calm_distance(id, ticks) * spec.facing)
    } else {
        spec.x
    }
}

// Integrate a short acceleration; position and gait consume this same distance.
fn escape_distance(seconds: f32, speed: f32) -> f32 {
    const RAMP: f32 = 0.4;
    if seconds < RAMP {
        0.5 * speed * seconds * seconds / RAMP
    } else {
        speed * (seconds - 0.5 * RAMP)
    }
}

/// A foot stays planted through support and arcs upward through recovery.
fn gait_foot(phase: f32, running: bool) -> [f32; 2] {
    let cycle = phase.rem_euclid(1.0);
    let (stride, support, lift) = if running {
        (112.0, 0.36, 30.0)
    } else {
        (66.0, 0.60, 13.0)
    };
    let reach = stride * support * 0.5;
    if cycle < support {
        [reach - cycle * stride, 0.0]
    } else {
        let t = (cycle - support) / (1.0 - support);
        let eased = t * t * (3.0 - 2.0 * t);
        [-reach + 2.0 * reach * eased, -(t * PI).sin() * lift]
    }
}

fn pose(
    id: usize,
    spec: PersonSpec,
    ticks: u64,
    traveled: f32,
    escape_age: Option<f32>,
    alarm: f32,
) -> PersonPose {
    let t = ticks as f32 / 60.0;
    let running = escape_age.is_some_and(|age| age > 0.0);
    let walking = spec.activity == Activity::Walking || running;
    let seated = if spec.activity == Activity::Seated {
        1.0 - escape_age.map_or(0.0, |age| (age * 5.0).clamp(0.0, 1.0))
    } else {
        0.0
    };
    let phase = traveled / if running { 112.0 } else { 66.0 } + id as f32 * 0.173;
    let wave = (phase * TAU).sin();
    let mut hands = [[-11.0, -43.0], [13.0, -45.0]];
    let mut feet = [[-8.0, 0.0], [9.0, 0.0]];
    if walking {
        feet = [gait_foot(phase, running), gait_foot(phase + 0.5, running)];
        let (reach, raise) = if running { (24.0, 14.0) } else { (15.0, 0.0) };
        hands = [
            [-wave * reach, -45.0 - raise - wave.max(0.0) * 9.0],
            [wave * reach, -44.0 - raise - (-wave).max(0.0) * 9.0],
        ];
    } else if spec.activity == Activity::Phone {
        hands[1] = [17.0, -65.0];
    } else {
        let gesture = ((t * 1.4 + id as f32).sin() * 0.5 + 0.5).powi(3);
        // Keep conversational hands near the chest so neighboring adult
        // silhouettes remain separate at their authored 38–40px spacing.
        hands[1] = [8.0 + gesture * 2.0, -49.0 - gesture * 11.0];
        hands[0] = [-11.0, -41.0 - (t * 0.8).sin() * 2.0];
    }
    if alarm > 0.0 && !running {
        hands = [[-20.0, -68.0], [24.0, -70.0]];
    }
    PersonPose {
        feet,
        hands,
        bob: if walking {
            -(wave * wave) * if running { 3.0 } else { 1.2 }
        } else {
            (t * 1.9 + id as f32).sin() * 0.5
        },
        lean: if running { 8.0 } else { 0.0 },
        seated,
        alarm,
    }
}

impl Nightlife {
    /// Samples a fixed-tick story clock and persistent EP evacuation age.
    ///
    /// `threat_age` must stay `Some` after arrival, including the victory and
    /// rescue. Its origin reconstructs the calm positions without extra state.
    pub fn sample(ticks: u64, threat_age: Option<u64>) -> Self {
        let origin = threat_age.map_or(ticks, |age| ticks.saturating_sub(age));
        let mut people = Vec::with_capacity(PEOPLE.len());
        for (id, spec) in PEOPLE.iter().copied().enumerate() {
            let start_x = calm_x(id, spec, origin);
            let delay = 0.12 + (id % 5) as f32 * 0.07;
            let seconds = threat_age.map(|age| age as f32 / 60.0);
            let escape_age = seconds.map(|age| (age - delay).max(0.0));
            let fleeing = escape_age.is_some_and(|age| age > 0.0);
            let escape_direction = if start_x < STREET_WIDTH * 0.5 {
                -1.0
            } else {
                1.0
            };
            let distance = escape_age.map_or(0.0, |age| {
                escape_distance(age, 255.0 + (id % 4) as f32 * 22.0)
            });
            let x = start_x + distance * escape_direction;
            if !on_street(x) {
                continue;
            }
            let traveled = if fleeing {
                distance
            } else if spec.activity == Activity::Walking {
                calm_distance(id, origin)
            } else {
                0.0
            };
            // The local figure is about 96px tall: these scales put adults at
            // 167–178px beside the chapter's approximately 194px protagonists.
            let scale = 1.74 + (id % 3) as f32 * 0.055;
            people.push(Pedestrian {
                id,
                x,
                ground_y: spec.y,
                scale,
                facing: if fleeing {
                    escape_direction
                } else {
                    spec.facing
                },
                wardrobe: spec.wardrobe,
                activity: spec.activity,
                skin: id % 4,
                hair: (id / 2) % 4,
                pose: pose(
                    id,
                    spec,
                    ticks,
                    traveled / (scale * PERSON_WIDTH_RATIO),
                    escape_age,
                    seconds.map_or(0.0, |age| (age * 7.0).min(1.0)),
                ),
                fleeing,
            });
        }
        // Depth ordering stays stable when a pedestrian walks past a table.
        people.sort_by(|a, b| a.ground_y.total_cmp(&b.ground_y));
        let vehicles = (0..5)
            .filter_map(|id| {
                let facing = if id % 2 == 0 { 1.0 } else { -1.0 };
                let speed = 270.0 + id as f32 * 25.0;
                let kind = match id % 3 {
                    0 => VehicleKind::Hatch,
                    1 => VehicleKind::Taxi,
                    _ => VehicleKind::DeliveryScooter,
                };
                let scale = kind.scale();
                let traveled = origin as f32 / 60.0 * speed;
                let start = (230.0 + id as f32 * 780.0 + traveled * facing + VEHICLE_MARGIN)
                    .rem_euclid(STREET_WIDTH + VEHICLE_MARGIN * 2.0)
                    - VEHICLE_MARGIN;
                let escape = threat_age.map_or(0.0, |age| {
                    let seconds = age as f32 / 60.0;
                    speed * seconds + escape_distance(seconds, 390.0)
                });
                let x = start + escape * facing;
                (-VEHICLE_MARGIN..=STREET_WIDTH + VEHICLE_MARGIN)
                    .contains(&x)
                    .then_some(Vehicle {
                        id,
                        kind,
                        x,
                        ground_y: if facing > 0.0 { 738.0 } else { 712.0 },
                        facing,
                        // Space mirrors the wheel together with the vehicle. The
                        // local angle therefore uses positive travel; its world
                        // angular direction gains the same facing sign as X.
                        wheel_angle: (traveled + escape) / (kind.wheel_radius() * scale),
                        scale,
                        fleeing: threat_age.is_some(),
                    })
            })
            .collect();
        Self {
            people,
            vehicles,
            ticks,
            threat_age,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arrival_preserves_positions_before_the_reaction() {
        for ticks in [0, 128, 600, 4853] {
            let before = Nightlife::sample(ticks, None);
            let arrival = Nightlife::sample(ticks, Some(0));
            for (a, b) in before.people.iter().zip(&arrival.people) {
                assert_eq!((a.id, a.x, a.ground_y), (b.id, b.x, b.ground_y));
                assert_eq!(a.pose, b.pose);
            }
            for (a, b) in before.vehicles.iter().zip(&arrival.vehicles) {
                assert_eq!((a.id, a.x, a.wheel_angle), (b.id, b.x, b.wheel_angle));
            }
        }
    }

    #[test]
    fn every_person_and_vehicle_leaves_without_returning() {
        for onset in [0, 328, 2300, 27000] {
            let initial = Nightlife::sample(onset, Some(0));
            let mut exited_people = [false; PEOPLE.len()];
            let mut exited_vehicles = [false; 5];
            for age in 0..=1200 {
                let current = Nightlife::sample(onset + age, Some(age));
                for person in &current.people {
                    assert!(
                        !exited_people[person.id],
                        "person {} returned at {age}",
                        person.id
                    );
                    let start = initial.people.iter().find(|p| p.id == person.id).unwrap();
                    assert!(
                        (person.x - STREET_WIDTH * 0.5).abs()
                            >= (start.x - STREET_WIDTH * 0.5).abs()
                    );
                }
                for vehicle in &current.vehicles {
                    assert!(
                        !exited_vehicles[vehicle.id],
                        "vehicle {} returned at {age}",
                        vehicle.id
                    );
                }
                for (id, exited) in exited_people.iter_mut().enumerate() {
                    *exited |= !current.people.iter().any(|p| p.id == id);
                }
                for (id, exited) in exited_vehicles.iter_mut().enumerate() {
                    *exited |= !current.vehicles.iter().any(|v| v.id == id);
                }
            }
            assert!(exited_people.into_iter().all(|exited| exited));
            assert!(exited_vehicles.into_iter().all(|exited| exited));
            let later = Nightlife::sample(onset + 120_000, Some(120_000));
            assert!(later.people.is_empty() && later.vehicles.is_empty());
        }
    }

    #[test]
    fn support_foot_cancels_world_travel_and_recovery_lifts() {
        for running in [false, true] {
            let stride = if running { 112.0 } else { 66.0 };
            let foot_a = gait_foot(0.1, running);
            let foot_b = gait_foot(0.2, running);
            assert!((foot_a[0] - (foot_b[0] + stride * 0.1)).abs() < 0.0001);
            assert_eq!(foot_a[1], 0.0);
            assert_eq!(foot_b[1], 0.0);
            assert!(gait_foot(0.8, running)[1] < -5.0);
        }
    }

    #[test]
    fn traffic_has_adult_world_proportions_and_ground_clearance() {
        let scene = Nightlife::sample(0, None);
        for vehicle in &scene.vehicles {
            let (width, height, body_height) = match vehicle.kind {
                VehicleKind::Hatch => (172.0, 63.0, 63.0),
                VehicleKind::Taxi => (187.0, 72.0, 63.0),
                VehicleKind::DeliveryScooter => (76.0, 79.0, 79.0),
            };
            let actual_height = height * vehicle.scale;
            if vehicle.kind == VehicleKind::DeliveryScooter {
                assert!((160.0..=180.0).contains(&actual_height));
            } else {
                assert!((420.0..=470.0).contains(&(width * vehicle.scale)));
                assert!((150.0..=170.0).contains(&actual_height));
                assert!(vehicle.ground_y - body_height * vehicle.scale >= 550.0);
            }
            assert!(VEHICLE_MARGIN > width * vehicle.scale * 0.5);
            assert_eq!(vehicle.scale, vehicle.kind.scale());
        }
        let taxis: Vec<_> = scene
            .vehicles
            .iter()
            .filter(|v| v.kind == VehicleKind::Taxi)
            .collect();
        assert_ne!(taxis[0].facing, taxis[1].facing);
        assert_eq!(taxis[0].scale, taxis[1].scale);
    }

    #[test]
    fn scaled_wheels_roll_the_world_distance_in_both_directions() {
        for onset in [0, 300] {
            for fleeing in [false, true] {
                let a = Nightlife::sample(onset, fleeing.then_some(0));
                let b = Nightlife::sample(onset + 1, fleeing.then_some(1));
                for current in &b.vehicles {
                    let previous = a.vehicles.iter().find(|v| v.id == current.id).unwrap();
                    let displacement = current.x - previous.x;
                    let rolled = (current.wheel_angle - previous.wheel_angle)
                        * current.kind.wheel_radius()
                        * current.scale
                        * current.facing;
                    assert!((displacement - rolled).abs() < 0.005);
                }
            }
        }
    }

    #[test]
    fn sampling_is_stable_when_multiple_cameras_draw_the_same_tick() {
        assert_eq!(Nightlife::sample(518, None), Nightlife::sample(518, None));
        assert_eq!(
            Nightlife::sample(620, Some(102)),
            Nightlife::sample(620, Some(102))
        );
    }
}
