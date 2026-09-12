//! Samples Rust's shared stride from distance traveled instead of elapsed time.
//!
//! System: Adventure animation. Editable clip ids and stride length coordinate
//! gameplay, authored paths and footsteps without moving the physical actor.

use serde::Deserialize;
use std::{error::Error, fs, path::Path};

pub mod mesh;
pub mod run;

/// Explicit locomotion intent, independent from physical attack/airborne state.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Gait {
    /// Deliberate small movements during authored approaches and in the bedroom.
    #[default]
    Walk,
    /// Default agile traversal while the player holds a horizontal direction.
    Run,
}

/// Camera-relative view of deliberate movement through scene depth.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TravelView {
    /// The usual lateral gameplay silhouette.
    #[default]
    Side,
    /// Moving away toward the far pavement.
    Back,
    /// Returning toward the near pavement.
    Front,
}

impl TravelView {
    /// Sideways route segments keep the ordinary silhouette; crossing the road
    /// shows the back on the way away and the front on the return.
    pub fn from_delta(dx: f32, dy: f32) -> Self {
        if dy.abs() < 0.01 || !dx.is_finite() || !dy.is_finite() {
            Self::Side
        } else if dy < 0.0 {
            Self::Back
        } else {
            Self::Front
        }
    }
}

/// External tuning for the shared walk, run and kick clips.
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Motion {
    version: u32,
    /// Distance for one complete pair of alternating steps at full character size.
    pub stride_pixels: f32,
    /// Ground distance of a complete running cycle, including both flight phases.
    pub run_stride_pixels: f32,
    /// Catalog entry for the looping walk.
    pub walk_clip: String,
    /// Authored walking away from the viewer while crossing the road.
    pub back_walk_clip: String,
    /// Authored walking toward the viewer on the reverse route.
    pub front_walk_clip: String,
    /// Composite clip defined by the independent rig and its anatomical pieces.
    pub run_clip: String,
    /// Catalog entry for the finite kick.
    pub kick_clip: String,
}

impl Motion {
    /// Loads and validates animation tuning before the scene starts.
    pub fn load(path: &Path) -> Result<Self, Box<dyn Error>> {
        Self::from_json(&fs::read_to_string(path)?)
    }

    /// Parses content independently of filesystem access or graphics.
    pub fn from_json(json: &str) -> Result<Self, Box<dyn Error>> {
        let motion: Self = serde_json::from_str(json)?;
        if motion.version != 1
            || !motion.stride_pixels.is_finite()
            || !(64.0..=240.0).contains(&motion.stride_pixels)
            || !motion.run_stride_pixels.is_finite()
            || !(180.0..=360.0).contains(&motion.run_stride_pixels)
            || motion.walk_clip.trim().is_empty()
            || motion.back_walk_clip.trim().is_empty()
            || motion.front_walk_clip.trim().is_empty()
            || motion.run_clip.trim().is_empty()
            || motion.kick_clip.trim().is_empty()
        {
            return Err("invalid Rust locomotion tuning".into());
        }
        Ok(motion)
    }

    /// Converts accumulated ground travel to a clip cursor. Depth scales the
    /// stride as well as the character so small actors do not skate over paths.
    pub fn walk_cursor(&self, distance: f32, depth: f32, duration: u32) -> u32 {
        let phase = self.stride_phase(distance, depth);
        (phase * duration as f32).floor() as u32
    }

    /// Normalized phase shared by interpolation, frame selection and Foley.
    pub fn stride_phase(&self, distance: f32, depth: f32) -> f32 {
        if !distance.is_finite() || !depth.is_finite() || depth <= 0.0 {
            return 0.0;
        }
        (distance / (self.stride_pixels * depth)).rem_euclid(1.0)
    }

    /// Step boundary counter; callers emit only after actual grounded travel.
    pub fn footfall(&self, distance: f32, depth: f32) -> u32 {
        if !distance.is_finite() || !depth.is_finite() || depth <= 0.0 {
            return 0;
        }
        (distance.max(0.0) / (self.stride_pixels * depth * 0.5)).floor() as u32
    }

    /// Distance between repeated contacts of the same foot for this gait.
    pub fn stride_for(&self, gait: Gait) -> f32 {
        match gait {
            Gait::Walk => self.stride_pixels,
            Gait::Run => self.run_stride_pixels,
        }
    }

    /// Stable normalized cursor shared by the rig and the step observer.
    pub fn phase_for(&self, gait: Gait, distance: f32) -> f32 {
        if !distance.is_finite() {
            return 0.0;
        }
        (distance / self.stride_for(gait)).rem_euclid(1.0)
    }
}

/// One supported waking pose; scene coordinates stay separate from its image.
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WakingPose {
    /// Scene tick at which this drawing begins.
    pub start_tick: u32,
    /// Existing morning atlas pose index.
    pub source: usize,
    /// Anatomical support measured within the source crop.
    pub anchor: [f32; 2],
    /// Matching mattress, seat or sole position in the room.
    pub support: [f32; 2],
    /// Per-pose correction that retains Rust's apparent head size.
    pub scale: f32,
    /// Small initial rotation settling around the weight-bearing support.
    pub settle_degrees: f32,
}

/// External waking performance and the short route from bed toward the doorway.
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Waking {
    version: u32,
    /// Existing authored drawings, with independent timings and supports.
    pub poses: Vec<WakingPose>,
    /// Scene tick at which the standing Rust begins walking.
    pub exit_start_tick: u32,
    /// Duration of the bedroom route in simulation ticks.
    pub exit_duration_ticks: u32,
    /// Scale of the shared street body inside the close bedroom view.
    pub exit_depth: f32,
    /// Polyline coordinates along the floor; editable without changing sprites.
    pub exit_path: Vec<[f32; 2]>,
}

impl Waking {
    /// Loads the bedroom performance, rejecting invalid supports and chronology.
    pub fn load(path: &Path) -> Result<Self, Box<dyn Error>> {
        let track: Self = serde_json::from_str(&fs::read_to_string(path)?)?;
        if track.version != 1
            || track.poses.len() != 8
            || track.poses[0].start_tick != 0
            || track
                .poses
                .windows(2)
                .any(|p| p[0].start_tick >= p[1].start_tick)
            || track.poses.iter().any(|p| {
                p.source >= 8
                    || !p.anchor.iter().chain(&p.support).all(|v| v.is_finite())
                    || !p.scale.is_finite()
                    || p.scale <= 0.0
                    || !p.settle_degrees.is_finite()
                    || p.settle_degrees.abs() > 8.0
            })
            || track.exit_start_tick < track.poses[7].start_tick + 45
            || track.exit_duration_ticks == 0
            || !track.exit_depth.is_finite()
            || track.exit_depth <= 0.0
            || track.exit_path.len() < 2
            || !track.exit_path.iter().flatten().all(|v| v.is_finite())
        {
            return Err("invalid Rust waking performance".into());
        }
        Ok(track)
    }

    /// Current drawing and contact-centered settling angle, with no frame ghosting.
    pub fn pose(&self, ticks: u32) -> (&WakingPose, f32) {
        let pose = self
            .poses
            .iter()
            .rev()
            .find(|p| p.start_tick <= ticks)
            .expect("first pose at zero");
        let t = (ticks.saturating_sub(pose.start_tick) as f32 / 28.0).min(1.0);
        (pose, pose.settle_degrees * (1.0 - smooth(t)))
    }

    /// Samples floor position and actual distance along the authored route.
    pub fn exit(&self, ticks: u32) -> Option<([f32; 2], f32)> {
        if ticks < self.exit_start_tick {
            return None;
        }
        let t = (ticks.saturating_sub(self.exit_start_tick) as f32
            / self.exit_duration_ticks as f32)
            .min(1.0);
        let length = |p: &[[f32; 2]]| (p[1][0] - p[0][0]).hypot(p[1][1] - p[0][1]);
        let total: f32 = self.exit_path.windows(2).map(length).sum();
        let distance = total * smooth(t);
        let mut remaining = distance;
        for pair in self.exit_path.windows(2) {
            let segment = length(pair);
            if remaining <= segment && segment > 0.0 {
                let f = remaining / segment;
                return Some((
                    [
                        pair[0][0] + (pair[1][0] - pair[0][0]) * f,
                        pair[0][1] + (pair[1][1] - pair[0][1]) * f,
                    ],
                    distance,
                ));
            }
            remaining -= segment;
        }
        Some((*self.exit_path.last().expect("validated path"), distance))
    }
}

fn smooth(t: f32) -> f32 {
    t * t * (3.0 - 2.0 * t)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn motion() -> Motion {
        Motion::from_json(include_str!(
            "../../assets/adventure/locomotion/motion.json"
        ))
        .unwrap()
    }

    #[test]
    fn distance_clock_preserves_stride_across_speed_pause_and_depth() {
        let motion = motion();
        let stride = motion.stride_pixels;
        assert_eq!(motion.walk_cursor(0.0, 1.0, 8), 0);
        assert_eq!(motion.walk_cursor(stride * 0.25, 1.0, 8), 2);
        assert_eq!(motion.walk_cursor(stride * 0.5, 1.0, 8), 4);
        assert_eq!(motion.walk_cursor(stride, 1.0, 8), 0);
        assert_eq!(motion.walk_cursor(stride * 0.25, 0.5, 8), 4);
        // At the same traveled distance, paused and slower paths show the same
        // support, even after thousands of ticks without physical movement.
        let at_wall = motion.walk_cursor(75.0, 1.0, 8);
        for _ in 0..4000 {
            assert_eq!(motion.walk_cursor(75.0, 1.0, 8), at_wall);
        }
    }

    #[test]
    fn contacts_alternate_twice_per_cycle_and_bad_content_is_rejected() {
        let motion = motion();
        assert_eq!(motion.footfall(motion.stride_pixels * 0.49, 1.0), 0);
        assert_eq!(motion.footfall(motion.stride_pixels * 0.51, 1.0), 1);
        assert_eq!(motion.footfall(motion.stride_pixels, 1.0), 2);
        for length in [0, 63, 241] {
            let mut edited: serde_json::Value = serde_json::from_str(include_str!(
                "../../assets/adventure/locomotion/motion.json"
            ))
            .unwrap();
            edited["stride_pixels"] = length.into();
            assert!(Motion::from_json(&edited.to_string()).is_err());
        }
        for distance in [f32::NAN, f32::INFINITY] {
            assert_eq!(motion.walk_cursor(distance, 1.0, 8), 0);
        }
    }

    #[test]
    fn bedroom_route_preserves_floor_contact_and_monotonic_stride_at_joins() {
        let track = Waking::load(&crate::runtime_paths::asset_path(
            "assets/adventure/locomotion/waking.json",
        ))
        .unwrap();
        assert!(track.exit(track.exit_start_tick - 1).is_none());
        assert_eq!(
            track.exit(track.exit_start_tick).unwrap(),
            (track.exit_path[0], 0.0)
        );
        let mut previous = track.exit(track.exit_start_tick).unwrap();
        for tick in track.exit_start_tick + 1..=track.exit_start_tick + track.exit_duration_ticks {
            let current = track.exit(tick).unwrap();
            let displacement = (current.0[0] - previous.0[0]).hypot(current.0[1] - previous.0[1]);
            assert!(displacement < 6.0, "route teleported at tick {tick}");
            assert!(current.1 >= previous.1);
            assert!(current.1 - previous.1 + 0.001 >= displacement);
            previous = current;
        }
        assert_eq!(previous.0, *track.exit_path.last().unwrap());
        assert_eq!(track.exit(u32::MAX).unwrap(), previous);
        for pose in &track.poses {
            let (settled, rotation) = track.pose(pose.start_tick + 28);
            assert_eq!(settled.support, pose.support);
            assert_eq!(rotation, 0.0, "support motion must settle exactly");
        }
    }
}
