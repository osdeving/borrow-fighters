//! Describes the street's opening camera and the erratic entity's landing.
//!
//! System: Adventure presentation data. This fixed-clock shot is independent of
//! Raylib, player input and combat; its last transform is the playable view.

use crate::math::vec2::Vec2;
use serde::Deserialize;
use std::{error::Error, fs};

/// Default eighteen-second street establishment; runtime uses the loaded track's end.
pub const ARRIVAL_TICKS: u32 = 18 * 60;

/// Editable camera beats, including the fully covered cut back to Rust.
pub const STREET_ARRIVAL_PATH: &str = "assets/adventure/street/arrival-camera.json";

/// World framing and cinematic border opacity at one fixed update.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ArrivalShot {
    /// Point in the untransformed street view placed at screen center.
    pub target: Vec2,
    /// Uniform magnification of the complete world.
    pub zoom: f32,
    /// Letterbox opacity, removed smoothly before controls are handed over.
    pub matte: f32,
}

impl ArrivalShot {
    /// Exact identity transform of normal play, also used for checkpoint retry.
    pub fn settled() -> Self {
        Self {
            target: Vec2::new(640.0, 360.0),
            zoom: 1.0,
            matte: 0.0,
        }
    }
}

/// Coordinate system of one shot; changing anchors requires a fully covered cut.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
enum StreetAnchor {
    Hub,
    Gameplay,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct StreetCameraKey {
    tick: u32,
    anchor: StreetAnchor,
    target: [f32; 2],
    zoom: f32,
    matte: f32,
    blackout: f32,
}

/// Slow observation of ordinary street life, followed by a covered cut to Rust.
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StreetArrivalSpec {
    version: u32,
    keys: Vec<StreetCameraKey>,
}

impl Default for StreetArrivalSpec {
    fn default() -> Self {
        Self::load().unwrap_or_else(|error| {
            eprintln!("Street arrival: {error}; using bundled camera track");
            Self::bundled()
        })
    }
}

impl StreetArrivalSpec {
    /// Reads an independently replaceable camera track before a new story begins.
    pub fn load() -> Result<Self, Box<dyn Error>> {
        Self::parse(&fs::read_to_string(crate::runtime_paths::asset_path(
            STREET_ARRIVAL_PATH,
        ))?)
    }

    /// Deterministic reference used by pure tests and recovery from invalid edits.
    pub fn bundled() -> Self {
        Self::parse(include_str!(
            "../../assets/adventure/street/arrival-camera.json"
        ))
        .expect("bundled street camera must validate")
    }

    fn parse(source: &str) -> Result<Self, Box<dyn Error>> {
        let spec: Self = serde_json::from_str(source)?;
        if spec.version != 1
            || spec.keys.len() < 2
            || spec.keys.first().is_none_or(|key| {
                key.tick != 0 || key.anchor != StreetAnchor::Hub || key.blackout != 0.0
            })
            || spec.keys.windows(2).any(|pair| {
                pair[0].tick >= pair[1].tick
                    || (pair[0].anchor != pair[1].anchor
                        && (pair[0].blackout != 1.0 || pair[1].blackout != 1.0))
            })
            || spec.keys.iter().any(|key| {
                ![
                    key.target[0],
                    key.target[1],
                    key.zoom,
                    key.matte,
                    key.blackout,
                ]
                .iter()
                .all(|value| value.is_finite())
                    || !(1.0..=5.0).contains(&key.zoom)
                    || !(0.0..=1.0).contains(&key.matte)
                    || !(0.0..=1.0).contains(&key.blackout)
                    || key.target[1] - 360.0 / key.zoom < 0.0
                    || key.target[1] + 360.0 / key.zoom > 720.001
            })
            || spec.keys.last().is_none_or(|key| {
                !(600..=3600).contains(&key.tick)
                    || key.anchor != StreetAnchor::Gameplay
                    || key.target != [640.0, 360.0]
                    || key.zoom != 1.0
                    || key.matte != 0.0
                    || key.blackout != 0.0
            })
        {
            return Err("invalid street camera: ordered keys, covered anchor cuts and exact gameplay handoff required".into());
        }
        Ok(spec)
    }

    /// Last authored update, shared by camera sampling, input ownership and skip.
    pub fn duration_ticks(&self) -> u32 {
        self.keys.last().expect("validated camera keys").tick
    }

    /// The blackout belongs to the same fixed clock, so pause cannot uncover a cut.
    pub fn blackout(&self, ticks: u32) -> f32 {
        self.segment(ticks).map_or(0.0, |(from, to, blend)| {
            from.blackout + (to.blackout - from.blackout) * blend
        })
    }

    /// Samples visible motion in local map coordinates, clamping every viewport.
    pub fn sample(&self, ticks: u32, hub: f32, camera_left: f32, world_width: f32) -> ArrivalShot {
        let Some((from, to, blend)) = self.segment(ticks) else {
            return ArrivalShot::settled();
        };
        let lerp = |a: f32, b: f32| a + (b - a) * blend;
        let zoom = lerp(from.zoom, to.zoom);
        let offset = match from.anchor {
            StreetAnchor::Hub => hub - camera_left,
            StreetAnchor::Gameplay => 0.0,
        };
        let center_x = lerp(from.target[0], to.target[0]) + offset + camera_left;
        let half_width = 640.0 / zoom;
        ArrivalShot {
            target: Vec2::new(
                center_x.clamp(half_width, world_width - half_width) - camera_left,
                lerp(from.target[1], to.target[1]),
            ),
            zoom,
            matte: lerp(from.matte, to.matte),
        }
    }

    fn segment(&self, ticks: u32) -> Option<(&StreetCameraKey, &StreetCameraKey, f32)> {
        let pair = self.keys.windows(2).find(|pair| ticks < pair[1].tick)?;
        let progress = (ticks - pair[0].tick) as f32 / (pair[1].tick - pair[0].tick) as f32;
        Some((&pair[0], &pair[1], smooth(progress)))
    }
}

fn smooth(t: f32) -> f32 {
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}

/// Editable path for the independent EP actor, camera and landing effects.
pub const EP_ARRIVAL_PATH: &str = "assets/adventure/street/ep-arrival.json";

/// One authored trajectory/camera point; the EP's feet retain their world pivot.
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DescentKey {
    /// Fixed update from the start of the descent.
    pub tick: u32,
    /// Feet height in street coordinates.
    pub feet_y: f32,
    /// Vertical tangent in pixels per second, for a continuous falling velocity.
    pub speed_y: f32,
    /// Camera focus in the untransformed street.
    pub camera_y: f32,
    /// Camera magnification; one is the gameplay view.
    pub zoom: f32,
    /// Horizontal camera focus follows the EP at one and settles at zero.
    pub tracking: f32,
    /// Cinematic border opacity.
    pub matte: f32,
    /// Perspective size: the distant body grows continuously into its play scale.
    pub body_scale: f32,
}

/// Data for a single reusable landing, separate from the scene background.
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EpArrivalSpec {
    /// Trajectory through the air, ending at the instant of ground contact.
    pub descent: Vec<DescentKey>,
    /// Update when the wide gameplay view has fully returned.
    pub gameplay_tick: u32,
    /// Duration of the grounded kneeling pose before standing back up.
    pub kneel_ticks: u32,
    /// Total contact/recovery duration before the enemy may attack.
    pub recovery_ticks: u32,
    /// Replaceable pose indices in the independent erratic atlas.
    pub airborne_pose: usize,
    /// Grounded, one-knee/hand contact pose.
    pub landing_pose: usize,
    /// Crouching bridge from the grounded pose into the normal idle pose.
    pub rising_pose: usize,
    /// Final dust radius in world pixels.
    pub dust_radius: f32,
    /// Lifetime of the dust after contact.
    pub dust_ticks: u32,
    /// Short decaying camera movement at impact, in world pixels.
    pub shake_pixels: f32,
}

impl EpArrivalSpec {
    /// Loads and validates one complete revision without changing live state.
    pub fn load() -> Result<Self, Box<dyn Error>> {
        Self::parse(&fs::read_to_string(crate::runtime_paths::asset_path(
            EP_ARRIVAL_PATH,
        ))?)
    }

    fn parse(json: &str) -> Result<Self, Box<dyn Error>> {
        let spec: Self = serde_json::from_str(json)?;
        if spec.descent.len() < 2
            || spec.descent.first().is_none_or(|key| key.tick != 0)
            || spec
                .descent
                .windows(2)
                .any(|pair| pair[0].tick >= pair[1].tick)
            || spec.descent.iter().any(|key| {
                ![
                    key.feet_y,
                    key.speed_y,
                    key.camera_y,
                    key.zoom,
                    key.tracking,
                    key.matte,
                    key.body_scale,
                ]
                .iter()
                .all(|value| value.is_finite())
                    || !(1.0..=5.0).contains(&key.zoom)
                    || !(0.0..=1.0).contains(&key.tracking)
                    || !(0.0..=1.0).contains(&key.matte)
                    || key.camera_y < 360.0 / key.zoom
                    || key.camera_y + 360.0 / key.zoom > 720.001
                    || key.feet_y > super::combat::FLOOR_Y
                    || key.speed_y < 0.0
                    || !(0.1..=1.2).contains(&key.body_scale)
            })
            || spec.descent.last().is_none_or(|key| {
                key.feet_y != super::combat::FLOOR_Y
                    || key.zoom != 1.0
                    || key.tracking != 0.0
                    || key.matte != 0.0
                    || key.camera_y != 360.0
                    || key.tick > 1200
                    || key.body_scale != 1.0
            })
            || spec.gameplay_tick >= spec.impact_tick()
            || !spec
                .descent
                .iter()
                .any(|key| key.tick == spec.gameplay_tick && key.zoom == 1.0 && key.tracking == 0.0)
            || spec.kneel_ticks < 12
            || spec.recovery_ticks <= spec.kneel_ticks + 12
            || spec.recovery_ticks > 600
            || [spec.airborne_pose, spec.landing_pose, spec.rising_pose]
                .iter()
                .any(|pose| *pose >= 8)
            || !spec.dust_radius.is_finite()
            || !(40.0..=400.0).contains(&spec.dust_radius)
            || !(30..=600).contains(&spec.dust_ticks)
            || !spec.shake_pixels.is_finite()
            || !(0.0..=16.0).contains(&spec.shake_pixels)
        {
            return Err("invalid EP arrival: check ordered trajectory, grounded final key, camera, poses and effects".into());
        }
        Ok(spec)
    }

    /// Ground contact is the final airborne key, shared by picture and reaction.
    pub fn impact_tick(&self) -> u32 {
        self.descent.last().map_or(0, |key| key.tick)
    }
}

/// Visual sample of the same body before and after the camera hands off.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EpArrivalSample {
    /// Actor feet, independent of camera framing.
    pub feet_y: f32,
    /// Actual derivative of the authored fall, driving streak length and blur.
    pub speed_y: f32,
    /// World framing, including the short ground impact shake.
    pub shot: ArrivalShot,
    /// Sprite frame in the existing erratic atlas.
    pub pose: usize,
    /// Local horizontal and vertical body scale, preserving the foot pivot.
    pub scale: Vec2,
    /// Grounded effect age; none while the body is airborne.
    pub impact_age: Option<u32>,
}

/// Fixed-clock ownership of the EP's authored descent and recovery.
#[derive(Clone, Debug)]
pub struct EpArrival {
    /// Validated independently replaceable timeline and effect data.
    pub spec: EpArrivalSpec,
    ticks: Option<u32>,
    pending_spec: Option<EpArrivalSpec>,
}

impl Default for EpArrival {
    fn default() -> Self {
        let spec = EpArrivalSpec::load().unwrap_or_else(|error| {
            eprintln!("EP arrival: {error}; using bundled timeline");
            EpArrivalSpec::parse(include_str!(
                "../../assets/adventure/street/ep-arrival.json"
            ))
            .expect("bundled EP timeline must validate")
        });
        Self {
            spec,
            ticks: None,
            pending_spec: None,
        }
    }
}

impl EpArrival {
    /// Rewinds the shot for a new street, or omits it at an awakened checkpoint.
    pub fn reset(&mut self, already_landed: bool) {
        if let Some(spec) = self.pending_spec.take() {
            self.spec = spec;
        }
        self.ticks = None;
        if already_landed {
            self.finish();
        }
    }

    /// Starts once; subsequent attempts cannot reset a running landing.
    pub fn start(&mut self) {
        if self.ticks.is_none() {
            self.ticks = Some(0);
        }
    }

    /// Advances one owned fixed update. Pause omits this call.
    pub fn tick(&mut self) {
        if let Some(ticks) = &mut self.ticks {
            *ticks = ticks
                .saturating_add(1)
                .min(self.spec.impact_tick() + self.spec.recovery_ticks.max(self.spec.dust_ticks));
        }
    }

    /// Elapsed fixed updates, absent until the encounter trigger is crossed.
    pub fn ticks(&self) -> Option<u32> {
        self.ticks
    }

    /// Combat and input remain held until the landing pose has recovered.
    pub fn active(&self) -> bool {
        self.ticks
            .is_some_and(|ticks| ticks < self.spec.impact_tick() + self.spec.recovery_ticks)
    }

    /// The collision moment that starts all neighbourhood reactions.
    pub fn impacted(&self) -> bool {
        self.ticks
            .is_some_and(|ticks| ticks >= self.spec.impact_tick())
    }

    /// Finishes only this arrival; story progression and health stay untouched.
    pub fn finish(&mut self) {
        self.ticks =
            Some(self.spec.impact_tick() + self.spec.recovery_ticks.max(self.spec.dust_ticks));
    }

    /// Adopts valid disk edits before or after the shot; in-flight edits wait.
    ///
    /// Keeping the active revision avoids moving the impact event across the
    /// current tick and replaying an evacuation when F5 is used mid-descent.
    pub fn reload(&mut self) -> Result<(), Box<dyn Error>> {
        let spec = EpArrivalSpec::load()?;
        if !self.active() {
            let completed = self.ticks.is_some();
            self.spec = spec;
            if completed {
                self.finish();
            }
        } else {
            self.pending_spec = Some(spec);
        }
        Ok(())
    }

    /// Samples an actor in pre-camera street coordinates.
    pub fn sample(&self, enemy_screen_x: f32) -> Option<EpArrivalSample> {
        let ticks = self.ticks?;
        let impact_tick = self.spec.impact_tick();
        if ticks >= impact_tick {
            let age = ticks - impact_tick;
            let compression = if age < 8 {
                (age as f32 / 8.0 * std::f32::consts::PI).sin()
            } else {
                0.0
            };
            let pose = if age < self.spec.kneel_ticks {
                self.spec.landing_pose
            } else if age < self.spec.recovery_ticks - 12 {
                self.spec.rising_pose
            } else {
                0
            };
            let mut shot = ArrivalShot::settled();
            if age < 22 {
                let falloff = (1.0 - age as f32 / 22.0).powi(2);
                shot.target.x += (age as f32 * 2.3).sin() * self.spec.shake_pixels * falloff;
                shot.target.y += (age as f32 * 2.7).sin() * self.spec.shake_pixels * 0.6 * falloff;
            }
            return Some(EpArrivalSample {
                feet_y: super::combat::FLOOR_Y,
                speed_y: 0.0,
                shot,
                pose,
                scale: Vec2::new(1.0 + compression * 0.12, 1.0 - compression * 0.18),
                impact_age: Some(age),
            });
        }
        let pair = self
            .spec
            .descent
            .windows(2)
            .find(|pair| ticks < pair[1].tick)?;
        let (from, to) = (&pair[0], &pair[1]);
        let span = (to.tick - from.tick) as f32;
        let t = (ticks - from.tick) as f32 / span;
        let blend = smooth(t);
        let lerp = |a: f32, b: f32| a + (b - a) * blend;
        let t2 = t * t;
        let t3 = t2 * t;
        let feet_y = (2.0 * t3 - 3.0 * t2 + 1.0) * from.feet_y
            + (t3 - 2.0 * t2 + t) * from.speed_y * span / 60.0
            + (-2.0 * t3 + 3.0 * t2) * to.feet_y
            + (t3 - t2) * to.speed_y * span / 60.0;
        let speed_y = ((6.0 * t2 - 6.0 * t) * from.feet_y
            + (3.0 * t2 - 4.0 * t + 1.0) * from.speed_y * span / 60.0
            + (-6.0 * t2 + 6.0 * t) * to.feet_y
            + (3.0 * t2 - 2.0 * t) * to.speed_y * span / 60.0)
            * 60.0
            / span;
        let zoom = lerp(from.zoom, to.zoom);
        let tracking = lerp(from.tracking, to.tracking);
        let target_x = (640.0 + (enemy_screen_x - 640.0) * tracking)
            .clamp(640.0 / zoom, 1280.0 - 640.0 / zoom);
        Some(EpArrivalSample {
            feet_y,
            speed_y,
            shot: ArrivalShot {
                target: Vec2::new(target_x, lerp(from.camera_y, to.camera_y)),
                zoom,
                matte: lerp(from.matte, to.matte),
            },
            pose: self.spec.airborne_pose,
            scale: Vec2::new(
                lerp(from.body_scale, to.body_scale),
                lerp(from.body_scale, to.body_scale),
            ),
            impact_age: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shot_stays_inside_the_painted_world_and_hands_off_without_a_jump() {
        let track = StreetArrivalSpec::bundled();
        assert_eq!(track.duration_ticks(), ARRIVAL_TICKS);
        let mut previous = track.sample(0, 2048.0, 0.0, 4608.0);
        for tick in 1..=ARRIVAL_TICKS {
            let shot = track.sample(tick, 2048.0, 0.0, 4608.0);
            assert!(shot.target.x - 640.0 / shot.zoom >= -0.001);
            assert!(shot.target.y - 360.0 / shot.zoom >= -0.001);
            assert!(shot.target.x + 640.0 / shot.zoom <= 4608.001);
            assert!(shot.target.y + 360.0 / shot.zoom <= 720.001);
            if track.blackout(tick) < 1.0 || track.blackout(tick - 1) < 1.0 {
                assert!((shot.target.x - previous.target.x).abs() < 5.0);
                assert!((shot.target.y - previous.target.y).abs() < 3.0);
                assert!((shot.zoom - previous.zoom).abs() < 0.03);
            }
            assert!((track.blackout(tick) - track.blackout(tick - 1)).abs() < 0.04);
            previous = shot;
        }
        assert_eq!(previous, ArrivalShot::settled());
        assert_eq!(track.sample(u32::MAX, 2048.0, 0.0, 4608.0), previous);
        assert_eq!(track.blackout(ARRIVAL_TICKS), 0.0);
    }

    #[test]
    fn external_street_timing_owns_handoff_and_rejects_uncovered_cuts() {
        use crate::adventure::story::Story;
        let source = include_str!("../../assets/adventure/street/arrival-camera.json");
        let mut value: serde_json::Value = serde_json::from_str(source).unwrap();
        value["keys"][6]["blackout"] = serde_json::json!(0);
        assert!(StreetArrivalSpec::parse(&value.to_string()).is_err());
        let mut value: serde_json::Value = serde_json::from_str(source).unwrap();
        value["keys"][9]["target"] = serde_json::json!([600, 360]);
        assert!(StreetArrivalSpec::parse(&value.to_string()).is_err());
        let mut value: serde_json::Value = serde_json::from_str(source).unwrap();
        value["keys"][9]["tick"] = serde_json::json!(1200);
        let mut story = Story::new();
        story.street_arrival = StreetArrivalSpec::parse(&value.to_string()).unwrap();
        story.advance_scene();
        story.advance_scene();
        story.stage_ticks = ARRIVAL_TICKS;
        assert!(story.arrival_active());
        story.skip_segment();
        assert_eq!(story.stage_ticks, 1200);
        assert!(!story.arrival_active());
        assert_eq!(story.initial_shot(), ArrivalShot::settled());
        assert_eq!(story.initial_blackout(), 0.0);
        assert!(!story.combat.enemy_awake);
    }

    #[test]
    fn ep_descent_keeps_falling_after_the_gameplay_camera_returns() {
        let mut arrival = EpArrival::default();
        arrival.start();
        let mut previous = arrival.sample(845.0).unwrap();
        for tick in 1..arrival.spec.impact_tick() {
            arrival.tick();
            let sample = arrival.sample(845.0).unwrap();
            assert!(sample.feet_y >= previous.feet_y, "upward jump at {tick}");
            assert!((sample.feet_y - previous.feet_y).abs() < 14.0);
            assert!(sample.speed_y > previous.speed_y);
            assert!(sample.scale.y >= previous.scale.y);
            assert!(sample.feet_y < super::super::combat::FLOOR_Y);
            assert!(sample.impact_age.is_none());
            assert!(sample.shot.target.y - 360.0 / sample.shot.zoom >= -0.01);
            assert!(sample.shot.target.y + 360.0 / sample.shot.zoom <= 720.01);
            assert!((sample.shot.zoom - previous.shot.zoom).abs() < 0.18);
            if tick >= arrival.spec.gameplay_tick {
                assert_eq!(sample.shot, ArrivalShot::settled());
            }
            previous = sample;
        }
        assert!(!arrival.impacted());
        arrival.tick();
        let impact = arrival.sample(845.0).unwrap();
        assert_eq!(impact.feet_y, super::super::combat::FLOOR_Y);
        assert_eq!(impact.impact_age, Some(0));
        assert_eq!(impact.pose, arrival.spec.landing_pose);
        assert!(
            arrival.spec.impact_tick() < 60,
            "fall must stay fast, without hovering"
        );
        assert!(previous.speed_y > 750.0);
        assert!(arrival.active());
        for _ in 0..arrival.spec.recovery_ticks {
            arrival.tick();
        }
        assert!(!arrival.active());
        assert_eq!(arrival.sample(845.0).unwrap().pose, 0);
    }

    #[test]
    fn authored_ep_track_rejects_backward_time_bad_camera_and_missing_ground_contact() {
        let source = include_str!("../../assets/adventure/street/ep-arrival.json");
        for broken in [
            source.replacen("\"tick\": 20", "\"tick\": 0", 1),
            source.replacen("\"camera_y\": 150", "\"camera_y\": 5", 1),
            source.replacen("\"feet_y\": 580.0", "\"feet_y\": 590.0", 1),
            source.replacen("\"landing_pose\": 6", "\"landing_pose\": 8", 1),
        ] {
            assert!(EpArrivalSpec::parse(&broken).is_err());
        }
    }

    #[test]
    fn retry_and_skip_do_not_leave_frozen_dust_or_repeat_a_landing() {
        let mut arrival = EpArrival::default();
        arrival.start();
        arrival.finish();
        let end = arrival.ticks();
        assert!(!arrival.active());
        arrival.start();
        arrival.tick();
        assert_eq!(arrival.ticks(), end);
        assert!(arrival.sample(800.0).unwrap().impact_age.unwrap() >= arrival.spec.dust_ticks);
        arrival.reset(false);
        assert!(arrival.ticks().is_none());
        arrival.start();
        assert!(arrival.active());
        arrival.reset(true);
        assert!(!arrival.active());
    }
}
