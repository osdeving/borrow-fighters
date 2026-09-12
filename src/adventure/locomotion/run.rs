//! Samples the small running rig with planted feet and alternating knee recovery.
//!
//! System: Adventure locomotion. Anatomical pieces remain independent assets;
//! this pure solver owns only editable limb lengths and distance-driven tracks.

use crate::math::vec2::Vec2;
use serde::Deserialize;
use std::{error::Error, fs, path::Path};

/// One anatomical leg, with explicit segment lengths at gameplay scale.
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Leg {
    pub boot: String,
    pub hip_offset: [f32; 2],
    /// Horizontal footprint lane relative to the shared contact track.
    pub foot_offset: f32,
    pub thigh_length: f32,
    pub shin_length: f32,
}

/// Small torso compression/rise sampled twice per complete cycle.
#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BodyKey {
    pub phase: f32,
    pub height: f32,
    pub lean: f32,
}

/// Guide key used to place the airborne ankle. The final boot may articulate
/// after IK; contact endpoints remain derived from the stride automatically.
#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FootKey {
    pub phase: f32,
    pub position: [f32; 2],
    pub rotation: f32,
}

/// Flight-only ankle articulation. The existing track first places the ankle;
/// this profile then rotates the rigid boot without feeding back into the IK.
#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AirAnkle {
    pub follow_in: [f32; 2],
    pub follow_out: [f32; 2],
    pub flexion: f32,
}

/// Independent content for one complete run cycle, without combat or world rules.
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RunRig {
    version: u32,
    pub clip: String,
    pub body_piece: String,
    pub near_arm_piece: String,
    pub far_arm_piece: String,
    pub near: Leg,
    pub far: Leg,
    pub hip_x: f32,
    pub contact_x: f32,
    pub stance_end: f32,
    pub toe_degrees: f32,
    pub arms: [f32; 2],
    pub body_keys: Vec<BodyKey>,
    pub flight_keys: Vec<FootKey>,
    pub air_ankle: AirAnkle,
}

/// Foot ground reference and whether this leg is carrying weight.
#[derive(Clone, Copy, Debug)]
pub struct FootPose {
    pub position: Vec2,
    pub rotation: f32,
    pub planted: bool,
}

/// Solved joints for one leg; the boot retains an independent ankle rotation.
#[derive(Clone, Copy, Debug)]
pub struct LegPose {
    pub hip: Vec2,
    pub knee: Vec2,
    pub ankle: Vec2,
    pub foot: FootPose,
}

/// Complete body snapshot in local world units, facing right before mirroring.
#[derive(Clone, Copy, Debug)]
pub struct RunPose {
    pub hip: Vec2,
    pub lean: f32,
    pub near: LegPose,
    pub far: LegPose,
    pub near_arm: f32,
    pub far_arm: f32,
}

impl RunRig {
    /// Reads editable geometry and rejects discontinuous or nonfinite tracks.
    pub fn load(path: &Path) -> Result<Self, Box<dyn Error>> {
        let rig: Self = serde_json::from_str(&fs::read_to_string(path)?)?;
        rig.validate()?;
        Ok(rig)
    }

    fn validate(&self) -> Result<(), Box<dyn Error>> {
        let valid_leg = |leg: &Leg| {
            !leg.boot.trim().is_empty()
                && leg.hip_offset.iter().all(|v| v.is_finite())
                && (-16.0..=16.0).contains(&leg.foot_offset)
                && (20.0..=60.0).contains(&leg.thigh_length)
                && (20.0..=60.0).contains(&leg.shin_length)
        };
        if self.version != 1
            || self.clip.is_empty()
            || self.body_piece.is_empty()
            || self.near_arm_piece.is_empty()
            || self.far_arm_piece.is_empty()
            || !valid_leg(&self.near)
            || !valid_leg(&self.far)
            || ![self.hip_x, self.contact_x, self.toe_degrees]
                .iter()
                .chain(&self.arms)
                .all(|v| v.is_finite())
            || !(0.15..=0.40).contains(&self.stance_end)
            || self.body_keys.len() < 2
            || self.flight_keys.is_empty()
            || !self
                .air_ankle
                .follow_in
                .iter()
                .chain(&self.air_ankle.follow_out)
                .all(|v| v.is_finite())
            || !(self.stance_end <= self.air_ankle.follow_in[0]
                && self.air_ankle.follow_in[0] < self.air_ankle.follow_in[1]
                && self.air_ankle.follow_in[1] <= self.air_ankle.follow_out[0]
                && self.air_ankle.follow_out[0] < self.air_ankle.follow_out[1]
                && self.air_ankle.follow_out[1] <= 1.0)
            || !(-45.0..=45.0).contains(&self.air_ankle.flexion)
            || self.body_keys.first().is_none_or(|k| k.phase != 0.0)
            || self.body_keys.last().is_none_or(|k| k.phase != 1.0)
            || self.body_keys.iter().any(|k| {
                ![k.phase, k.height, k.lean].iter().all(|v| v.is_finite())
                    || !(-90.0..=-48.0).contains(&k.height)
            })
            || self.body_keys.windows(2).any(|w| w[0].phase >= w[1].phase)
            || self.flight_keys.iter().any(|k| {
                ![k.phase, k.position[0], k.position[1], k.rotation]
                    .iter()
                    .all(|v| v.is_finite())
                    || k.phase <= self.stance_end
                    || k.phase >= 1.0
                    || k.position[1] >= 0.0
            })
            || self
                .flight_keys
                .windows(2)
                .any(|w| w[0].phase >= w[1].phase)
        {
            return Err("invalid Rust running rig".into());
        }
        let first = self.body_keys[0];
        let last = self.body_keys[self.body_keys.len() - 1];
        if first.height != last.height || first.lean != last.lean {
            return Err("running torso loop must join continuously".into());
        }
        Ok(())
    }

    /// Every image consumed by this composed clip, including all anatomical parts.
    pub fn pieces(&self) -> [&str; 5] {
        [
            &self.body_piece,
            &self.near_arm_piece,
            &self.far_arm_piece,
            &self.near.boot,
            &self.far.boot,
        ]
    }

    /// Checks the complete performance against actual boot landmarks. A reload
    /// must reject unreachable targets instead of silently stretching a shin.
    pub fn validate_performance(
        &self,
        stride: f32,
        soles: [Vec2; 2],
    ) -> Result<(), Box<dyn Error>> {
        if !stride.is_finite()
            || stride <= 0.0
            || soles.iter().any(|p| !p.x.is_finite() || !p.y.is_finite())
        {
            return Err("running stride and sole landmarks must be finite".into());
        }
        for i in 0..2048 {
            let phase = i as f32 / 2048.0;
            let pose = self.sample(phase, stride, soles);
            for (name, spec, leg) in [
                ("near", &self.near, pose.near),
                ("far", &self.far, pose.far),
            ] {
                if leg.foot.position.y > 0.001 {
                    return Err(format!(
                        "{name} flight sole crosses the floor at run phase {phase:.4}"
                    )
                    .into());
                }
                let reach = (leg.ankle.x - leg.hip.x).hypot(leg.ankle.y - leg.hip.y);
                if reach >= spec.thigh_length + spec.shin_length - 0.001
                    || reach <= (spec.thigh_length - spec.shin_length).abs() + 0.001
                {
                    return Err(format!("unreachable {name} ankle at run phase {phase:.4}").into());
                }
                let knee = ((spec.thigh_length.powi(2) + spec.shin_length.powi(2) - reach.powi(2))
                    / (2.0 * spec.thigh_length * spec.shin_length))
                    .clamp(-1.0, 1.0)
                    .acos()
                    .to_degrees();
                if knee < if leg.foot.planted { 115.0 } else { 60.0 } {
                    return Err(format!("compressed {name} knee at run phase {phase:.4}").into());
                }
                if ((name == "near" && i == 0) || (name == "far" && i == 1024))
                    && !(145.0..=165.0).contains(&knee)
                {
                    return Err(format!(
                        "{name} contact knee must open between 145 and 165 degrees"
                    )
                    .into());
                }
            }
        }
        Ok(())
    }

    /// Solves both legs around a stable pelvis. Foot offsets are read from each
    /// boot's `sole` socket, so replacing a boot preserves its real ground contact.
    pub fn sample(&self, phase: f32, stride: f32, soles: [Vec2; 2]) -> RunPose {
        let phase = phase.rem_euclid(1.0);
        let body = self.body((phase * 2.0).rem_euclid(1.0));
        let hip = Vec2::new(self.hip_x, body.height);
        let near = self.leg(hip, &self.near, self.foot(phase, stride), soles[0], phase);
        let far_phase = (phase + 0.5).rem_euclid(1.0);
        let far = self.leg(
            hip,
            &self.far,
            self.foot(far_phase, stride),
            soles[1],
            far_phase,
        );
        let arm = |p: f32| {
            self.arms[0]
                + (self.arms[1] - self.arms[0]) * (0.5 - 0.5 * (p * std::f32::consts::TAU).cos())
        };
        RunPose {
            hip,
            lean: body.lean,
            near,
            far,
            near_arm: arm(phase),
            far_arm: arm(phase + 0.5),
        }
    }

    /// During stance, root travel and this backward sole travel cancel exactly.
    /// In flight this is the guide used to solve the ankle, before the rigid boot
    /// follows the shin; `sample` returns the resulting actual sole position.
    pub fn foot(&self, phase: f32, stride: f32) -> FootPose {
        if phase <= self.stance_end {
            let toe = ((phase / self.stance_end - 0.45) / 0.55).clamp(0.0, 1.0);
            return FootPose {
                position: Vec2::new(self.contact_x - phase * stride, 0.0),
                rotation: self.toe_degrees * smooth(toe),
                planted: true,
            };
        }
        let start = FootKey {
            phase: self.stance_end,
            position: [self.contact_x - self.stance_end * stride, 0.0],
            rotation: self.toe_degrees,
        };
        let end = FootKey {
            phase: 1.0,
            position: [self.contact_x, 0.0],
            rotation: 0.0,
        };
        let mut previous = start;
        for next in self.flight_keys.iter().copied().chain(std::iter::once(end)) {
            if phase <= next.phase {
                let t = smooth((phase - previous.phase) / (next.phase - previous.phase));
                return FootPose {
                    position: Vec2::new(
                        lerp(previous.position[0], next.position[0], t),
                        lerp(previous.position[1], next.position[1], t),
                    ),
                    rotation: lerp(previous.rotation, next.rotation, t),
                    planted: false,
                };
            }
            previous = next;
        }
        unreachable!("normalized phase is inside validated run track")
    }

    fn body(&self, phase: f32) -> BodyKey {
        for pair in self.body_keys.windows(2) {
            if phase <= pair[1].phase {
                let t = smooth((phase - pair[0].phase) / (pair[1].phase - pair[0].phase));
                return BodyKey {
                    phase,
                    height: lerp(pair[0].height, pair[1].height, t),
                    lean: lerp(pair[0].lean, pair[1].lean, t),
                };
            }
        }
        self.body_keys[0]
    }

    fn leg(&self, pelvis: Vec2, leg: &Leg, mut foot: FootPose, sole: Vec2, phase: f32) -> LegPose {
        foot.position.x += leg.foot_offset;
        let hip = Vec2::new(pelvis.x + leg.hip_offset[0], pelvis.y + leg.hip_offset[1]);
        let offset = rotate(sole, foot.rotation);
        let ankle = Vec2::new(foot.position.x - offset.x, foot.position.y - offset.y);
        let dx = ankle.x - hip.x;
        let dy = ankle.y - hip.y;
        let distance = dx.hypot(dy).max(0.001);
        let reach = distance.clamp(
            (leg.thigh_length - leg.shin_length).abs() + 0.001,
            leg.thigh_length + leg.shin_length - 0.001,
        );
        let along =
            (leg.thigh_length.powi(2) - leg.shin_length.powi(2) + reach.powi(2)) / (2.0 * reach);
        let bend = (leg.thigh_length.powi(2) - along.powi(2)).max(0.0).sqrt();
        // Positive perpendicular puts the knee forward in both support and heel recovery.
        let knee = Vec2::new(
            hip.x + dx / distance * along + dy / distance * bend,
            hip.y + dy / distance * along - dx / distance * bend,
        );
        if !foot.planted {
            let [start, full] = self.air_ankle.follow_in;
            let [release, end] = self.air_ankle.follow_out;
            let weight = smooth((phase - start) / (full - start))
                * (1.0 - smooth((phase - release) / (end - release)));
            let shin_rotation = (ankle.y - knee.y).atan2(ankle.x - knee.x).to_degrees() - 90.0;
            let relative = shin_rotation + self.air_ankle.flexion;
            let turn = (relative - foot.rotation + 180.0).rem_euclid(360.0) - 180.0;
            foot.rotation += turn * weight;
            // Airborne boot orientation follows the solved shin. Keeping the
            // ankle fixed avoids a cyclic solve and preserves both bone lengths.
            let offset = rotate(sole, foot.rotation);
            foot.position = Vec2::new(ankle.x + offset.x, ankle.y + offset.y);
        }
        LegPose {
            hip,
            knee,
            ankle,
            foot,
        }
    }
}

/// Rotates a local anatomical landmark without depending on a graphics library.
pub fn rotate(point: Vec2, degrees: f32) -> Vec2 {
    let a = degrees.to_radians();
    Vec2::new(
        point.x * a.cos() - point.y * a.sin(),
        point.x * a.sin() + point.y * a.cos(),
    )
}
fn smooth(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}
fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

#[cfg(test)]
mod tests {
    use super::*;
    fn rig() -> RunRig {
        RunRig::load(&crate::runtime_paths::asset_path(
            "assets/adventure/locomotion/run-rig.json",
        ))
        .unwrap()
    }

    fn content() -> (RunRig, f32, [Vec2; 2]) {
        let rig = rig();
        let catalog = crate::adventure::scenery::PieceCatalog::load(
            &crate::runtime_paths::asset_path("assets/adventure/locomotion/catalog.json"),
        )
        .unwrap();
        let motion = super::super::Motion::load(&crate::runtime_paths::asset_path(
            "assets/adventure/locomotion/motion.json",
        ))
        .unwrap();
        let soles = [&rig.near, &rig.far].map(|leg| {
            let (frame, scale) = catalog.pieces[&leg.boot].sample(0);
            let sole = frame.sockets["sole"];
            Vec2::new(
                (sole[0] - frame.anchor[0]) * scale,
                (sole[1] - frame.anchor[1]) * scale,
            )
        });
        (rig, motion.run_stride_pixels, soles)
    }

    #[test]
    fn supporting_foot_stays_fixed_in_the_world_at_any_acceleration() {
        let rig = rig();
        let stride = 272.0;
        for distance in [0.0, 1.0, 4.0, 13.0, 28.0, 45.0, 70.0] {
            let foot = rig.foot(distance / stride, stride);
            assert!(foot.planted);
            assert!((distance + foot.position.x - rig.contact_x).abs() < 0.001);
            assert_eq!(foot.position.y, 0.0);
        }
    }

    #[test]
    fn supports_alternate_and_flight_never_places_both_feet_on_the_floor() {
        let (rig, stride, soles) = content();
        rig.validate_performance(stride, soles).unwrap();
        for i in 0..1000 {
            let phase = i as f32 / 1000.0;
            let pose = rig.sample(phase, stride, soles);
            assert!(!(pose.near.foot.planted && pose.far.foot.planted));
            for (leg, spec) in [(pose.near, &rig.near), (pose.far, &rig.far)] {
                assert!(leg.foot.position.y <= 0.0);
                let upper = (leg.knee.x - leg.hip.x).hypot(leg.knee.y - leg.hip.y);
                let lower = (leg.ankle.x - leg.knee.x).hypot(leg.ankle.y - leg.knee.y);
                assert!(
                    (upper - spec.thigh_length).abs() < 0.02,
                    "thigh at phase {phase}"
                );
                assert!(
                    (lower - spec.shin_length).abs() < 0.02,
                    "shin at phase {phase}: {lower}"
                );
            }
        }
        for phase in [0.32, 0.40, 0.82, 0.90] {
            let pose = rig.sample(phase, stride, soles);
            assert!(!pose.near.foot.planted && !pose.far.foot.planted);
        }
    }

    #[test]
    fn performance_rejects_unreachable_or_crouched_edits_before_reload() {
        let (mut rig, stride, soles) = content();
        assert!(rig.validate_performance(stride * 2.0, soles).is_err());
        for key in &mut rig.body_keys {
            key.height += 25.0;
        }
        assert!(rig.validate_performance(stride, soles).is_err());
    }

    #[test]
    fn ankle_to_sole_contact_and_joint_positions_join_at_loop_wrap() {
        let (rig, stride, soles) = content();
        let before = rig.sample(1.0 - 0.00001, stride, soles);
        let after = rig.sample(0.00001, stride, soles);
        for (a, b, sole) in [
            (before.near, after.near, soles[0]),
            (before.far, after.far, soles[1]),
        ] {
            for (p, q) in [(a.hip, b.hip), (a.knee, b.knee), (a.ankle, b.ankle)] {
                assert!((p.x - q.x).hypot(p.y - q.y) < 0.02);
            }
            let offset = rotate(sole, b.foot.rotation);
            assert!((b.ankle.x + offset.x - b.foot.position.x).abs() < 0.001);
            assert!((b.ankle.y + offset.y - b.foot.position.y).abs() < 0.001);
        }
    }

    #[test]
    fn flight_boot_follows_the_shin_without_moving_its_ankle_or_ground_support() {
        let (rig, stride, soles) = content();
        for i in 0..1000 {
            let phase = i as f32 / 1000.0;
            let pose = rig.sample(phase, stride, soles);
            for (p, spec, leg, sole) in [
                (phase, &rig.near, pose.near, soles[0]),
                ((phase + 0.5).rem_euclid(1.0), &rig.far, pose.far, soles[1]),
            ] {
                let guide = rig.foot(p, stride);
                let before = rotate(sole, guide.rotation);
                assert!(
                    (leg.ankle.x - (guide.position.x + spec.foot_offset - before.x)).abs() < 0.001
                );
                assert!((leg.ankle.y - (guide.position.y - before.y)).abs() < 0.001);
                let final_offset = rotate(sole, leg.foot.rotation);
                assert!((leg.ankle.x + final_offset.x - leg.foot.position.x).abs() < 0.001);
                assert!((leg.ankle.y + final_offset.y - leg.foot.position.y).abs() < 0.001);
                if guide.planted {
                    assert_eq!(leg.foot.rotation, guide.rotation);
                    assert_eq!(leg.foot.position.y, 0.0);
                    assert!(
                        (leg.foot.position.x - guide.position.x - spec.foot_offset).abs() < 0.001
                    );
                } else if (rig.air_ankle.follow_in[1]..=rig.air_ankle.follow_out[0]).contains(&p) {
                    let shin = (leg.ankle.y - leg.knee.y)
                        .atan2(leg.ankle.x - leg.knee.x)
                        .to_degrees()
                        - 90.0;
                    let difference = (leg.foot.rotation - shin - rig.air_ankle.flexion + 180.0)
                        .rem_euclid(360.0)
                        - 180.0;
                    assert!(difference.abs() < 0.001);
                }
            }
        }
    }

    #[test]
    fn ankle_follow_blends_join_continuously_and_invalid_windows_are_rejected() {
        let (mut rig, stride, soles) = content();
        for phase in [
            rig.stance_end,
            rig.air_ankle.follow_in[0],
            rig.air_ankle.follow_in[1],
            rig.air_ankle.follow_out[0],
            rig.air_ankle.follow_out[1],
            1.0,
        ] {
            let a = rig.sample(phase - 0.00001, stride, soles).near;
            let b = rig.sample(phase + 0.00001, stride, soles).near;
            assert!(
                (a.foot.position.x - b.foot.position.x)
                    .hypot(a.foot.position.y - b.foot.position.y)
                    < 0.02
            );
            assert!((a.foot.rotation - b.foot.rotation).abs() < 0.05);
        }
        rig.air_ankle.follow_in[0] = rig.stance_end - 0.01;
        assert!(rig.validate().is_err());
        rig.air_ankle.follow_in[0] = rig.air_ankle.follow_in[1];
        assert!(rig.validate().is_err());
    }

    #[test]
    fn late_ankle_flexion_cannot_push_the_landing_sole_below_the_floor() {
        let (mut rig, stride, soles) = content();
        rig.air_ankle.follow_out = [0.98, 1.0];
        rig.air_ankle.flexion = 45.0;
        // Valid profile structure, but physically invalid after articulation.
        rig.validate().unwrap();
        assert!(rig.sample(0.99, stride, soles).near.foot.position.y > 3.0);
        let error = rig.validate_performance(stride, soles).unwrap_err();
        assert!(error.to_string().contains("sole crosses the floor"));
    }
}
