//! Samples neighbours taking shelter, their shop shutter and a safely escaping dog.
//!
//! System: Adventure scenery. Every pose derives from AmbientState's existing
//! clocks; these residents never add combat bodies, a second clock or pathfinding.

use super::{
    ambient::AmbientState,
    combat::{Facing, TICKS_PER_SECOND},
};
use crate::math::vec2::Vec2;

/// Horizontal center of the mercearia's actual doorway in background coordinates.
pub const DOOR_CENTER_X: f32 = 603.0;
/// Feet baseline shared by the shop entrance and bus-stop neighbours.
pub const NEIGHBOR_FLOOR_Y: f32 = 355.0;
/// Clear opening inside the doorway's frame.
pub const DOOR_WIDTH: f32 = 97.0;
/// Distance from the lintel to the threshold.
pub const DOOR_HEIGHT: f32 = 122.0;
/// Resident index reserved for the shopkeeper who closes the shutter last.
pub const SHOPKEEPER_ID: usize = 3;
/// Updates spent moving behind the inside edge of the doorway.
pub const ENTERING_TICKS: u32 = 24;
/// Reaction age when the shopkeeper starts pulling the shutter down.
pub const SHUTTER_START_TICK: u32 = 270;
/// Reaction age when the shutter reaches the threshold and stays shut.
pub const SHUTTER_CLOSED_TICK: u32 = 330;
/// Reaction age of the dog's brief startled bark.
pub const DOG_STARTLE_TICK: u32 = 8;
/// Reaction age when the startled dog starts escaping along the sidewalk.
pub const DOG_RUN_TICK: u32 = 24;

const RESIDENT_ORIGINS: [f32; 4] = [1390.0, 1470.0, 685.0, DOOR_CENTER_X];
const RESIDENT_STARTS: [u32; 3] = [12, 26, 18];
const RESIDENT_RUN_SPEED: f32 = 260.0 / TICKS_PER_SECOND as f32;
// The complete sprite has passed behind the left jamb before visibility ends.
const SHELTERED_X: f32 = DOOR_CENTER_X - DOOR_WIDTH * 0.5 - 60.0;
const DOG_ORIGIN_X: f32 = 805.0;
const DOG_EXIT_X: f32 = -160.0;
const DOG_ESCAPE_SPEED: f32 = 360.0 / TICKS_PER_SECOND as f32;

/// A resident's ordinary pose or progress toward shelter.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResidentPhase {
    /// Waiting or chatting before the EP appears.
    Idle,
    /// Briefly noticing the alarm before beginning the run.
    Startled,
    /// Crossing the sidewalk toward the real shop entrance.
    Running,
    /// Passing behind the doorway's frame into the dark interior.
    Entering,
    /// Fully inside the shop, without returning during this encounter.
    Sheltered,
    /// The shopkeeper watches the other residents enter before closing.
    Waiting,
    /// The shopkeeper pulls the shutter and is progressively covered by it.
    Closing,
}

/// One resident, including the final shopkeeper instance.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Resident {
    /// Stable identity: two bus-stop residents, a shop-front neighbour, shopkeeper.
    pub id: usize,
    /// Feet in background world coordinates, including the path inside the doorway.
    pub position: Vec2,
    /// Horizontal orientation used by the authored running pieces.
    pub facing: Facing,
    /// Current sheltering action.
    pub phase: ResidentPhase,
    /// Updates elapsed inside the current action.
    pub phase_ticks: u32,
    /// False only after the person is occluded inside or behind the closed shutter.
    pub visible: bool,
}

/// The caramelo's ordinary sidewalk behavior or escape action.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DogPhase {
    /// Relaxing on the sidewalk.
    Idle,
    /// Sitting between short walks.
    Sitting,
    /// Sniffing at a stationary spot.
    Sniffing,
    /// Walking a small distance in the calm street.
    Wandering,
    /// Stopping and looking at the sudden disturbance.
    Startled,
    /// Running toward the safe left exit.
    Running,
    /// Beyond the camera limit, without respawning during the encounter.
    Gone,
}

/// Current pose of the single, purely decorative caramelo dog.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NeighborhoodDog {
    /// Feet in background coordinates.
    pub position: Vec2,
    /// Direction of the current walk, look or escape.
    pub facing: Facing,
    /// Current ordinary or alarm action.
    pub phase: DogPhase,
    /// Elapsed updates inside the action, suitable for the piece animation.
    pub phase_ticks: u32,
    /// Whether the dog remains inside the background street.
    pub visible: bool,
}

/// State of the shop's separate rolling metal shutter.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShutterPhase {
    /// Raised while the neighbours are still outside or entering.
    Open,
    /// Descending after every visitor has reached shelter.
    Closing,
    /// In contact with the threshold for the rest of the encounter.
    Closed,
}

/// A rolling shutter whose exposed height grows without stretching the artwork.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Shutter {
    /// Open, descending or persistently closed.
    pub phase: ShutterPhase,
    /// Updates elapsed in this phase.
    pub phase_ticks: u32,
    /// Fraction of the doorway covered from the top, from zero through one.
    pub progress: f32,
}

/// Complete neighbourhood presentation sampled from one existing ambient clock.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Neighborhood {
    /// Three sheltering visitors and the shopkeeper, with stable indices.
    pub residents: [Resident; 4],
    /// The caramelo's current pose and one-way escape position.
    pub dog: NeighborhoodDog,
    /// Shutter state, shared with the closing sound milestones.
    pub shutter: Shutter,
}

impl Neighborhood {
    /// Samples the scene without advancing clocks or modifying the ambient state.
    pub fn sample(ambient: &AmbientState) -> Self {
        let age = ambient.accident_ticks();
        Self {
            residents: std::array::from_fn(|id| resident(id, ambient.ticks(), age)),
            dog: dog(ambient.ticks(), age),
            shutter: shutter(age),
        }
    }
}

fn resident(id: usize, ticks: u32, age: Option<u32>) -> Resident {
    let mut resident = Resident {
        id,
        position: Vec2::new(RESIDENT_ORIGINS[id], NEIGHBOR_FLOOR_Y),
        facing: if id == SHOPKEEPER_ID {
            Facing::Right
        } else {
            Facing::Left
        },
        phase: ResidentPhase::Idle,
        phase_ticks: ticks,
        visible: true,
    };
    let Some(age) = age else {
        return resident;
    };
    if id == SHOPKEEPER_ID {
        (resident.phase, resident.phase_ticks) = if age < SHUTTER_START_TICK {
            (ResidentPhase::Waiting, age)
        } else if age < SHUTTER_CLOSED_TICK {
            (ResidentPhase::Closing, age - SHUTTER_START_TICK)
        } else {
            resident.visible = false;
            (ResidentPhase::Sheltered, age - SHUTTER_CLOSED_TICK)
        };
        return resident;
    }
    let run_start = RESIDENT_STARTS[id];
    if age < run_start {
        resident.phase = ResidentPhase::Startled;
        resident.phase_ticks = age;
        return resident;
    }
    let distance = RESIDENT_ORIGINS[id] - DOOR_CENTER_X;
    let run_duration = (distance / RESIDENT_RUN_SPEED).ceil() as u32;
    let run_ticks = age - run_start;
    if run_ticks < run_duration {
        resident.position.x -= run_ticks as f32 * RESIDENT_RUN_SPEED;
        resident.phase = ResidentPhase::Running;
        resident.phase_ticks = run_ticks;
        return resident;
    }
    let entering_ticks = run_ticks - run_duration;
    let progress = (entering_ticks as f32 / ENTERING_TICKS as f32).min(1.0);
    resident.position.x = DOOR_CENTER_X + (SHELTERED_X - DOOR_CENTER_X) * progress;
    // A small depth step keeps the feet grounded within the interior, rather
    // than fading an actor out on the public sidewalk.
    resident.position.y = NEIGHBOR_FLOOR_Y - 7.0 * progress;
    resident.visible = entering_ticks < ENTERING_TICKS;
    resident.phase = if resident.visible {
        ResidentPhase::Entering
    } else {
        ResidentPhase::Sheltered
    };
    resident.phase_ticks = if resident.visible {
        entering_ticks
    } else {
        entering_ticks - ENTERING_TICKS
    };
    resident
}

fn calm_dog(ticks: u32) -> NeighborhoodDog {
    let cycle = ticks % 720;
    let (x, facing, phase, phase_ticks) = match cycle {
        0..=179 => (DOG_ORIGIN_X, Facing::Right, DogPhase::Sniffing, cycle),
        180..=359 => (
            DOG_ORIGIN_X + (cycle - 180) as f32 * (40.0 / 180.0),
            Facing::Right,
            DogPhase::Wandering,
            cycle - 180,
        ),
        360..=479 => (
            DOG_ORIGIN_X + 40.0,
            Facing::Right,
            DogPhase::Sitting,
            cycle - 360,
        ),
        480..=659 => (
            DOG_ORIGIN_X + 40.0 - (cycle - 480) as f32 * (40.0 / 180.0),
            Facing::Left,
            DogPhase::Wandering,
            cycle - 480,
        ),
        _ => (DOG_ORIGIN_X, Facing::Left, DogPhase::Idle, cycle - 660),
    };
    NeighborhoodDog {
        position: Vec2::new(x, NEIGHBOR_FLOOR_Y + 3.0),
        facing,
        phase,
        phase_ticks,
        visible: true,
    }
}

fn dog(ticks: u32, age: Option<u32>) -> NeighborhoodDog {
    let Some(age) = age else {
        return calm_dog(ticks);
    };
    // A normal first alarm increments ambient ticks while reaction age is zero;
    // the awake retry checkpoint starts both at zero. Recover that exact calm
    // origin, so a late alarm does not reposition a dog partway through a walk.
    let origin_ticks = ticks
        .saturating_sub(age)
        .saturating_sub(u32::from(ticks > age));
    let mut dog = calm_dog(origin_ticks);
    dog.phase_ticks = age;
    dog.phase = DogPhase::Startled;
    if age < DOG_RUN_TICK {
        return dog;
    }
    let run_ticks = age - DOG_RUN_TICK;
    let duration = ((dog.position.x - DOG_EXIT_X) / DOG_ESCAPE_SPEED).ceil() as u32;
    dog.position.x = (dog.position.x - run_ticks as f32 * DOG_ESCAPE_SPEED).max(DOG_EXIT_X);
    dog.facing = Facing::Left;
    dog.visible = run_ticks < duration;
    dog.phase = if dog.visible {
        DogPhase::Running
    } else {
        DogPhase::Gone
    };
    dog.phase_ticks = if dog.visible {
        run_ticks
    } else {
        run_ticks - duration
    };
    dog
}

fn shutter(age: Option<u32>) -> Shutter {
    let age = age.unwrap_or(0);
    if age < SHUTTER_START_TICK {
        Shutter {
            phase: ShutterPhase::Open,
            phase_ticks: age,
            progress: 0.0,
        }
    } else if age < SHUTTER_CLOSED_TICK {
        let elapsed = age - SHUTTER_START_TICK;
        Shutter {
            phase: ShutterPhase::Closing,
            phase_ticks: elapsed,
            progress: elapsed as f32 / (SHUTTER_CLOSED_TICK - SHUTTER_START_TICK) as f32,
        }
    } else {
        Shutter {
            phase: ShutterPhase::Closed,
            phase_ticks: age - SHUTTER_CLOSED_TICK,
            progress: 1.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn everyone_reaches_the_real_door_before_the_shopkeeper_closes() {
        let mut ambient = AmbientState::new(true);
        let mut reached_door = [false; 3];
        let mut previous = Neighborhood::sample(&ambient);
        for age in 1..=SHUTTER_CLOSED_TICK {
            ambient.tick(true);
            let current = Neighborhood::sample(&ambient);
            for (id, reached) in reached_door.iter_mut().enumerate() {
                let before = previous.residents[id];
                let person = current.residents[id];
                assert!(person.position.x <= before.position.x);
                assert!(
                    (person.position.x - before.position.x).abs()
                        <= RESIDENT_RUN_SPEED
                            .max((DOOR_CENTER_X - SHELTERED_X) / ENTERING_TICKS as f32,)
                            + 0.001
                );
                if person.phase == ResidentPhase::Entering {
                    *reached = true;
                    assert!(person.position.x <= DOOR_CENTER_X);
                }
                if !person.visible {
                    assert!(*reached, "Nobody may disappear before reaching the doorway");
                    assert!(person.position.x + 60.0 <= DOOR_CENTER_X - DOOR_WIDTH * 0.5);
                }
            }
            if current.shutter.phase != ShutterPhase::Open {
                assert!(
                    current.residents[..3]
                        .iter()
                        .all(|person| person.phase == ResidentPhase::Sheltered)
                );
            }
            if age < SHUTTER_CLOSED_TICK {
                assert!(current.residents[SHOPKEEPER_ID].visible);
            }
            assert!(current.shutter.progress >= previous.shutter.progress);
            previous = current;
        }
        assert!(reached_door.into_iter().all(|reached| reached));
        assert_eq!(previous.shutter.phase, ShutterPhase::Closed);
        assert!(previous.residents.iter().all(|person| !person.visible));
    }

    #[test]
    fn late_alarm_preserves_the_dog_walk_and_every_resident_position() {
        for delay in [0, 181, 359, 481, 659, 721, 12619] {
            let mut ambient = AmbientState::default();
            for _ in 0..delay {
                ambient.tick(false);
            }
            let before = Neighborhood::sample(&ambient);
            ambient.tick(true);
            let startled = Neighborhood::sample(&ambient);
            assert_eq!(startled.dog.position, before.dog.position);
            assert_eq!(startled.dog.phase, DogPhase::Startled);
            for (before, after) in before.residents.iter().zip(startled.residents) {
                assert_eq!(before.position, after.position);
            }
            let mut last_x = startled.dog.position.x;
            for _ in 0..SHUTTER_CLOSED_TICK {
                ambient.tick(false);
                let sample = Neighborhood::sample(&ambient);
                assert!(sample.dog.position.x <= last_x);
                assert!(last_x - sample.dog.position.x <= DOG_ESCAPE_SPEED + 0.001);
                last_x = sample.dog.position.x;
            }
            let empty = Neighborhood::sample(&ambient);
            assert_eq!(empty.dog.phase, DogPhase::Gone);
            assert!(!empty.dog.visible);
            for _ in 0..3000 {
                ambient.tick(true);
                let later = Neighborhood::sample(&ambient);
                assert_eq!(later.dog.position, empty.dog.position);
                assert_eq!(later.dog.phase, DogPhase::Gone);
                assert_eq!(later.shutter.phase, ShutterPhase::Closed);
                assert_eq!(later.shutter.progress, 1.0);
                assert!(later.residents.iter().all(|person| !person.visible));
            }
        }
    }

    #[test]
    fn sampling_freezes_with_the_ambient_clock_and_fresh_state_restores_the_open_shop() {
        let mut ambient = AmbientState::new(true);
        for _ in 0..SHUTTER_START_TICK + 25 {
            ambient.tick(true);
        }
        let paused = Neighborhood::sample(&ambient);
        for _ in 0..120 {
            assert_eq!(Neighborhood::sample(&ambient), paused);
        }
        assert_eq!(paused.shutter.phase, ShutterPhase::Closing);
        let retry = Neighborhood::sample(&AmbientState::new(true));
        assert_eq!(retry.shutter.phase, ShutterPhase::Open);
        assert!(retry.residents.iter().all(|person| person.visible));
        let restarted = Neighborhood::sample(&AmbientState::default());
        assert_eq!(restarted.shutter.phase, ShutterPhase::Open);
        assert!(
            restarted
                .residents
                .iter()
                .all(|person| person.phase == ResidentPhase::Idle)
        );
        assert!(restarted.dog.visible);
    }
}
