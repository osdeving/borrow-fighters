//! Samples the four painted Duke and Old C biography shots and their gentle cameras.
//!
//! Each shot owns one replaceable illustration, editable caption and camera keys.
//! The pure scene clock never advances story/combat; artwork and copy stay external.

use super::scenery::PieceCatalog;
use serde::Deserialize;
use std::{collections::BTreeSet, error::Error, fs, path::Path};

/// One camera key on the 60 Hz scene clock; horizons stay level.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Key {
    pub tick: u32,
    pub position: [f32; 2],
    pub scale: f32,
}

/// One complete painted illustration with its own caption and camera movement.
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Shot {
    pub id: String,
    pub character: String,
    pub duration_ticks: u32,
    pub caption: String,
    pub illustration: String,
    pub camera: Vec<Key>,
}

/// Validated external biographies. Each character occupies twelve seconds.
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Biographies {
    version: u32,
    pub shots: Vec<Shot>,
}

impl Biographies {
    pub fn load(path: &Path, catalog: &PieceCatalog) -> Result<Self, Box<dyn Error>> {
        Self::from_json(&fs::read_to_string(path)?, catalog)
    }

    pub fn from_json(json: &str, catalog: &PieceCatalog) -> Result<Self, Box<dyn Error>> {
        let data: Self = serde_json::from_str(json)?;
        if data.version != 2 || data.shots.len() != 4 {
            return Err("painted biographies require version 2 and four shots".into());
        }
        let mut shot_ids = BTreeSet::new();
        let mut images = BTreeSet::new();
        for shot in &data.shots {
            if !shot_ids.insert(&shot.id)
                || shot.id.trim().is_empty()
                || !["duke", "c"].contains(&shot.character.as_str())
                || !(60..=660).contains(&shot.duration_ticks)
                || shot.caption.trim().is_empty()
            {
                return Err("invalid biography shot".into());
            }
            let piece = catalog
                .pieces
                .get(&shot.illustration)
                .ok_or("missing biography illustration")?;
            if piece.frames.len() != 1 || piece.width != 1280.0 {
                return Err(
                    "biography illustrations require one frame at reference width 1280".into(),
                );
            }
            let frame = &piece.frames[0];
            if frame.anchor != [0.0, 0.0] || !images.insert(&frame.image) {
                return Err(
                    "each biography shot requires a distinct illustration anchored at its top left"
                        .into(),
                );
            }
            validate_keys(&shot.camera, shot.duration_ticks)?;
            let image_height = frame.source[3] * piece.width / frame.source[2];
            for camera in &shot.camera {
                let half = [640.0 / camera.scale, 360.0 / camera.scale];
                if camera.position[0] - half[0] < 0.0
                    || camera.position[1] - half[1] < 0.0
                    || camera.position[0] + half[0] > 1280.0
                    || camera.position[1] + half[1] > image_height
                {
                    return Err("biography camera exposes the edge of its illustration".into());
                }
            }
        }
        for id in ["duke", "c"] {
            let shots: Vec<_> = data.shots.iter().filter(|s| s.character == id).collect();
            if shots.len() != 2 || shots.iter().map(|s| s.duration_ticks).sum::<u32>() != 720 {
                return Err("each biography requires two shots totaling 720 ticks".into());
            }
        }
        Ok(data)
    }

    /// Selects the current shot; the last key holds at the biography boundary.
    pub fn sample(&self, character: &str, ticks: u32) -> (&Shot, u32) {
        let mut local = ticks.min(719);
        for shot in self.shots.iter().filter(|s| s.character == character) {
            if local < shot.duration_ticks {
                return (shot, local);
            }
            local -= shot.duration_ticks;
        }
        unreachable!("renderer only selects validated biography characters")
    }
}

fn validate_keys(keys: &[Key], duration: u32) -> Result<(), Box<dyn Error>> {
    if keys.is_empty() || keys[0].tick != 0 || keys.len() > 64 {
        return Err("camera requires a bounded key list starting at tick zero".into());
    }
    for (i, key) in keys.iter().enumerate() {
        if key.tick > duration
            || i > 0 && key.tick <= keys[i - 1].tick
            || !key.position.iter().all(|v| v.is_finite())
            || !key.scale.is_finite()
            || !(1.0..=2.0).contains(&key.scale)
        {
            return Err("invalid biography camera keyframe".into());
        }
    }
    Ok(())
}

/// Smoothstep preserves the camera's position and velocity at its held endpoints.
pub fn sample_keys(keys: &[Key], ticks: u32) -> Key {
    for pair in keys.windows(2) {
        let [a, b] = pair else { unreachable!() };
        if ticks < b.tick {
            let t = ticks.saturating_sub(a.tick) as f32 / (b.tick - a.tick) as f32;
            let t = t * t * (3.0 - 2.0 * t);
            let lerp = |x: f32, y: f32| x + (y - x) * t;
            return Key {
                tick: ticks,
                position: [
                    lerp(a.position[0], b.position[0]),
                    lerp(a.position[1], b.position[1]),
                ],
                scale: lerp(a.scale, b.scale),
            };
        }
    }
    *keys.last().expect("validated nonempty camera")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime_paths::asset_path;

    fn assets() -> (Biographies, PieceCatalog) {
        let catalog =
            PieceCatalog::load(&asset_path("assets/adventure/opening/scenes/catalog.json"))
                .unwrap();
        let data = Biographies::load(
            &asset_path("assets/adventure/opening/scenes/scenes.json"),
            &catalog,
        )
        .unwrap();
        (data, catalog)
    }

    #[test]
    fn biographies_load_only_four_distinct_painted_images() {
        let (data, catalog) = assets();
        assert_eq!(catalog.pieces.len(), 4);
        let paths: BTreeSet<_> = data
            .shots
            .iter()
            .map(|shot| {
                let piece = &catalog.pieces[&shot.illustration];
                assert_eq!(piece.frames.len(), 1);
                &piece.frames[0].image
            })
            .collect();
        assert_eq!(paths.len(), 4);
        for id in ["duke", "c"] {
            assert_ne!(
                data.sample(id, 0).0.illustration,
                data.sample(id, 719).0.illustration
            );
        }
    }

    #[test]
    fn shot_boundaries_preserve_the_two_twelve_second_biographies() {
        let (data, _) = assets();
        for id in ["duke", "c"] {
            assert_eq!(data.sample(id, 0).0.id, data.sample(id, 359).0.id);
            assert_ne!(data.sample(id, 359).0.id, data.sample(id, 360).0.id);
            assert_eq!(data.sample(id, 360).1, 0);
            assert_eq!(data.sample(id, 719).0.id, data.sample(id, u32::MAX).0.id);
            assert_eq!(data.sample(id, u32::MAX).1, 359);
        }
    }

    #[test]
    fn gentle_camera_motion_covers_the_frame_and_holds_at_the_end() {
        let (data, catalog) = assets();
        for shot in &data.shots {
            let frame = &catalog.pieces[&shot.illustration].frames[0];
            let height = frame.source[3] * 1280.0 / frame.source[2];
            for tick in 0..=shot.duration_ticks {
                let camera = sample_keys(&shot.camera, tick);
                assert!(camera.position[0] - 640.0 / camera.scale >= 0.0);
                assert!(camera.position[0] + 640.0 / camera.scale <= 1280.0);
                assert!(camera.position[1] - 360.0 / camera.scale >= 0.0);
                assert!(camera.position[1] + 360.0 / camera.scale <= height);
                let previous = sample_keys(&shot.camera, tick.saturating_sub(1));
                assert!((camera.position[0] - previous.position[0]).abs() < 0.1);
                assert!((camera.position[1] - previous.position[1]).abs() < 0.1);
                assert!((camera.scale - previous.scale).abs() < 0.001);
            }
            assert_eq!(
                sample_keys(&shot.camera, u32::MAX),
                *shot.camera.last().unwrap()
            );
        }
    }

    #[test]
    fn bad_references_timing_and_legacy_layered_scenes_are_rejected() {
        let (_, catalog) = assets();
        let original: serde_json::Value = serde_json::from_str(
            &fs::read_to_string(asset_path("assets/adventure/opening/scenes/scenes.json")).unwrap(),
        )
        .unwrap();
        for (field, value) in [
            ("illustration", serde_json::json!("missing")),
            ("duration_ticks", serde_json::json!(0)),
            ("tracks", serde_json::json!([])),
        ] {
            let mut invalid = original.clone();
            invalid["shots"][0][field] = value;
            assert!(Biographies::from_json(&invalid.to_string(), &catalog).is_err());
        }
        let mut duplicate = original.clone();
        duplicate["shots"][1]["illustration"] = original["shots"][0]["illustration"].clone();
        assert!(Biographies::from_json(&duplicate.to_string(), &catalog).is_err());
        let mut legacy = original;
        legacy["version"] = 1.into();
        assert!(Biographies::from_json(&legacy.to_string(), &catalog).is_err());
    }

    #[test]
    fn invalid_camera_edits_are_rejected_before_reload() {
        let (_, catalog) = assets();
        let original: serde_json::Value = serde_json::from_str(
            &fs::read_to_string(asset_path("assets/adventure/opening/scenes/scenes.json")).unwrap(),
        )
        .unwrap();
        for (field, value) in [
            ("tick", serde_json::json!(1)),
            ("scale", serde_json::json!(0.9)),
            ("position", serde_json::json!([0, 0])),
        ] {
            let mut invalid = original.clone();
            invalid["shots"][0]["camera"][0][field] = value;
            assert!(Biographies::from_json(&invalid.to_string(), &catalog).is_err());
        }
    }
}
