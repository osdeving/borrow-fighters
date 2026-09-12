//! Deforms one continuous trouser leg around two bones and a supported knee.
//!
//! System: Adventure animation content. A small authored grid and local knee
//! correction preserve cloth continuity; no graphics or combat state lives here.

use super::run::{LegPose, rotate};
use crate::math::vec2::Vec2;
use serde::Deserialize;
use std::{error::Error, fs, path::Path};

/// A textured point: scene-local position and normalized source coordinates.
#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    pub position: Vec2,
    pub uv: Vec2,
}

/// One protected or abbreviated vertical band of the original body art.
#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BodyBand {
    pub end: f32,
    pub scale: f32,
}

/// Specific source geometry for Rust's continuous leg and protected upper body.
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RunMesh {
    version: u32,
    pub leg_piece: String,
    pub source_size: [f32; 2],
    pub hip: [f32; 2],
    pub knee: [f32; 2],
    pub ankle: [f32; 2],
    pub rows: Vec<f32>,
    pub columns: u32,
    pub cloth_width: f32,
    pub knee_blend: f32,
    pub knee_volume: f32,
    pub far_tint: [u8; 3],
    pub body_bands: Vec<BodyBand>,
}

impl RunMesh {
    /// Loads source landmarks, weight falloff and the protected body bands.
    pub fn load(path: &Path) -> Result<Self, Box<dyn Error>> {
        let mesh: Self = serde_json::from_str(&fs::read_to_string(path)?)?;
        mesh.validate()?;
        Ok(mesh)
    }

    fn validate(&self) -> Result<(), Box<dyn Error>> {
        let [width, height] = self.source_size;
        let valid_point = |p: [f32; 2]| {
            p.iter().all(|v| v.is_finite())
                && p[0] >= 0.0
                && p[0] <= width
                && p[1] >= 0.0
                && p[1] <= height
        };
        if self.version != 1
            || self.leg_piece.trim().is_empty()
            || !(32.0..=4096.0).contains(&width)
            || !(32.0..=4096.0).contains(&height)
            || !valid_point(self.hip)
            || !valid_point(self.knee)
            || !valid_point(self.ankle)
            || self.hip[1] + 32.0 >= self.knee[1]
            || self.knee[1] + 32.0 >= self.ankle[1]
            || !(2..=8).contains(&self.columns)
            || !(4..=40).contains(&self.rows.len())
            || self.rows.first() != Some(&0.0)
            || self.rows.last() != Some(&height)
            || self.rows.iter().any(|y| !y.is_finite())
            || self.rows.windows(2).any(|p| p[0] >= p[1])
            || !(0.5..=2.0).contains(&self.cloth_width)
            || !(16.0..=240.0).contains(&self.knee_blend)
            || !(0.0..=1.0).contains(&self.knee_volume)
            || self.body_bands.is_empty()
            || self
                .body_bands
                .iter()
                .any(|b| !b.end.is_finite() || b.end <= 0.0 || !(0.1..=1.0).contains(&b.scale))
            || self.body_bands.windows(2).any(|p| p[0].end >= p[1].end)
        {
            return Err("invalid Rust leg mesh or body bands".into());
        }
        Ok(())
    }

    /// Cross-checks the source crops before accepting an edited catalog. Missing
    /// body bands would clip the torso; excess bands would create invalid UVs.
    pub fn validate_crops(
        &self,
        leg_size: [f32; 2],
        body_height: f32,
    ) -> Result<(), Box<dyn Error>> {
        if leg_size != self.source_size {
            return Err("running mesh source dimensions differ from its catalog crop".into());
        }
        if self.body_bands.last().is_none_or(|b| b.end != body_height) {
            return Err("running body bands must cover exactly the body crop height".into());
        }
        Ok(())
    }

    /// Keeps protected bands rigid; only explicitly authored plain cloth shortens.
    pub fn body_y(&self, source_y: f32) -> f32 {
        let mut start = 0.0;
        let mut mapped = 0.0;
        for band in &self.body_bands {
            let end = source_y.min(band.end);
            if end > start {
                mapped += (end - start) * band.scale;
            }
            if source_y <= band.end {
                return mapped;
            }
            start = band.end;
        }
        mapped + (source_y - start)
    }

    /// Two-bone skinning with a local corrective that prevents the half-weight
    /// knee from losing its transverse volume when the bones bend.
    pub fn vertex(&self, source: Vec2, pose: LegPose) -> Vertex {
        let hip = point(self.hip);
        let knee = point(self.knee);
        let ankle = point(self.ankle);
        let upper_rest = sub(knee, hip);
        let lower_rest = sub(ankle, knee);
        let upper = sub(pose.knee, pose.hip);
        let lower = sub(pose.ankle, pose.knee);
        let upper_scale = length(upper) / length(upper_rest);
        let lower_scale = length(lower) / length(lower_rest);
        let rest_center = if source.y <= knee.y {
            hip.x + (knee.x - hip.x) * (source.y - hip.y) / (knee.y - hip.y)
        } else {
            knee.x + (ankle.x - knee.x) * (source.y - knee.y) / (ankle.y - knee.y)
        };
        let local = Vec2::new(
            rest_center + (source.x - rest_center) * self.cloth_width,
            source.y,
        );
        let upper_angle = angle(upper) - angle(upper_rest);
        let lower_angle = angle(lower) - angle(lower_rest);
        let transform = |v: Vec2, origin: Vec2, scale: f32, degrees: f32, target: Vec2| {
            let delta = sub(v, origin);
            let rotated = rotate(Vec2::new(delta.x * scale, delta.y * scale), degrees);
            add(target, rotated)
        };
        let t = ((source.y - knee.y) / (2.0 * self.knee_blend) + 0.5).clamp(0.0, 1.0);
        let weight = t * t * (3.0 - 2.0 * t);
        let blend = |v: Vec2| {
            lerp(
                transform(v, hip, upper_scale, upper_angle, pose.hip),
                transform(v, knee, lower_scale, lower_angle, pose.knee),
                weight,
            )
        };
        let center = blend(Vec2::new(rest_center, source.y));
        let mut offset = sub(blend(local), center);
        let cosine = ((upper.x * lower.x + upper.y * lower.y) / (length(upper) * length(lower)))
            .clamp(-1.0, 1.0);
        let retained = (1.0 - 2.0 * weight * (1.0 - weight) * (1.0 - cosine))
            .sqrt()
            .max(0.45);
        let correction = 1.0 + self.knee_volume * (1.0 / retained - 1.0);
        offset.x *= correction;
        offset.y *= correction;
        Vertex {
            position: add(center, offset),
            uv: Vec2::new(
                source.x / self.source_size[0],
                source.y / self.source_size[1],
            ),
        }
    }

    /// Builds one watertight grid, using the same vertices on adjacent cells.
    pub fn triangles(&self, pose: LegPose) -> Vec<[Vertex; 3]> {
        let mut triangles = Vec::with_capacity((self.rows.len() - 1) * self.columns as usize * 2);
        for pair in self.rows.windows(2) {
            for col in 0..self.columns {
                let left = self.source_size[0] * col as f32 / self.columns as f32;
                let right = self.source_size[0] * (col + 1) as f32 / self.columns as f32;
                let a = self.vertex(Vec2::new(left, pair[0]), pose);
                let b = self.vertex(Vec2::new(left, pair[1]), pose);
                let c = self.vertex(Vec2::new(right, pair[1]), pose);
                let d = self.vertex(Vec2::new(right, pair[0]), pose);
                triangles.extend([[a, b, c], [a, c, d]]);
            }
        }
        triangles
    }
}

fn point(p: [f32; 2]) -> Vec2 {
    Vec2::new(p[0], p[1])
}
fn add(a: Vec2, b: Vec2) -> Vec2 {
    Vec2::new(a.x + b.x, a.y + b.y)
}
fn sub(a: Vec2, b: Vec2) -> Vec2 {
    Vec2::new(a.x - b.x, a.y - b.y)
}
fn length(p: Vec2) -> f32 {
    p.x.hypot(p.y)
}
fn angle(p: Vec2) -> f32 {
    p.y.atan2(p.x).to_degrees()
}
fn lerp(a: Vec2, b: Vec2, t: f32) -> Vec2 {
    Vec2::new(a.x + (b.x - a.x) * t, a.y + (b.y - a.y) * t)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adventure::locomotion::run::FootPose;

    fn mesh() -> RunMesh {
        RunMesh::load(&crate::runtime_paths::asset_path(
            "assets/adventure/locomotion/run-mesh.json",
        ))
        .unwrap()
    }

    fn pose(inner_angle: f32) -> LegPose {
        let knee = Vec2::new(0.0, 38.0);
        let ankle = add(knee, rotate(Vec2::new(0.0, 35.0), 180.0 - inner_angle));
        LegPose {
            hip: Vec2::ZERO,
            knee,
            ankle,
            foot: FootPose {
                position: ankle,
                rotation: 0.0,
                planted: false,
            },
        }
    }

    #[test]
    fn continuous_skin_attaches_to_all_three_anatomical_landmarks() {
        let mesh = mesh();
        for angle in [60.0, 90.0, 120.0, 150.0, 180.0] {
            let pose = pose(angle);
            for (source, target) in [
                (mesh.hip, pose.hip),
                (mesh.knee, pose.knee),
                (mesh.ankle, pose.ankle),
            ] {
                assert!(length(sub(mesh.vertex(point(source), pose).position, target)) < 0.0001);
            }
        }
    }

    #[test]
    fn knee_volume_is_preserved_during_contact_compression_and_recovery() {
        let mesh = mesh();
        let width = |angle| {
            let pose = pose(angle);
            let left = mesh.vertex(Vec2::new(mesh.knee[0] - 100.0, mesh.knee[1]), pose);
            let right = mesh.vertex(Vec2::new(mesh.knee[0] + 100.0, mesh.knee[1]), pose);
            length(sub(right.position, left.position))
        };
        let resting = width(180.0);
        for angle in [60.0, 90.0, 115.0, 145.0, 165.0] {
            let ratio = width(angle) / resting;
            assert!(
                (0.98..=1.02).contains(&ratio),
                "knee width at {angle}: {ratio}"
            );
        }
    }

    #[test]
    fn head_and_emblem_keep_source_height_while_plain_waist_shortens() {
        let mesh = mesh();
        assert_eq!(mesh.body_y(220.0) - mesh.body_y(0.0), 220.0);
        assert_eq!(mesh.body_y(365.0) - mesh.body_y(250.0), 115.0);
        assert!(mesh.body_y(400.0) - mesh.body_y(365.0) < 35.0);
    }

    #[test]
    fn catalog_crop_edits_cannot_clip_or_discard_the_body() {
        let mesh = mesh();
        assert!(mesh.validate_crops([332.0, 1156.0], 455.0).is_ok());
        assert!(mesh.validate_crops([332.0, 1155.0], 455.0).is_err());
        assert!(mesh.validate_crops([332.0, 1156.0], 454.0).is_err());
        assert!(mesh.validate_crops([332.0, 1156.0], 456.0).is_err());
    }
}
