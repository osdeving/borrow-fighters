//! Samples the editable Duke and Old C biographies as independent moving pieces.
//!
//! Scene tracks own timing and transforms; the shared art catalog owns pixels,
//! anchors and sockets. Sampling is pure and never advances story or combat.

use super::scenery::PieceCatalog;
use serde::Deserialize;
use std::{collections::BTreeSet, error::Error, fs, path::Path};

/// One transform key on a 60 Hz scene clock.
#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Key {
    pub tick: u32,
    pub position: [f32; 2],
    pub scale: f32,
    pub rotation: f32,
    pub opacity: f32,
}

/// An independently replaceable and reusable actor, object or environment.
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Track {
    pub id: String,
    pub piece: String,
    #[serde(default)]
    pub flip: bool,
    pub keys: Vec<Key>,
}

/// Editable text attached to an object; it follows the object's full transform.
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Label {
    pub parent: String,
    pub text: String,
    pub offset: [f32; 2],
    pub width: f32,
    pub size: f32,
    pub color: [u8; 4],
}

/// One shot, with an ordered layer stack and camera keys in world coordinates.
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Shot {
    pub id: String,
    pub character: String,
    pub duration_ticks: u32,
    pub caption: String,
    pub camera: Vec<Key>,
    pub tracks: Vec<Track>,
    #[serde(default)]
    pub labels: Vec<Label>,
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
        if data.version != 1 || data.shots.len() != 4 {
            return Err("biographies require version 1 and four shots".into());
        }
        let mut shot_ids = BTreeSet::new();
        for shot in &data.shots {
            if !shot_ids.insert(&shot.id)
                || shot.id.trim().is_empty()
                || !["duke", "c"].contains(&shot.character.as_str())
                || !(60..=660).contains(&shot.duration_ticks)
                || shot.caption.trim().is_empty()
                || shot.tracks.is_empty()
            {
                return Err("invalid biography shot".into());
            }
            validate_keys(&shot.camera, shot.duration_ticks)?;
            let mut ids = BTreeSet::new();
            for track in &shot.tracks {
                if track.id.trim().is_empty()
                    || !ids.insert(&track.id)
                    || !catalog.pieces.contains_key(&track.piece)
                {
                    return Err(format!("invalid biography piece: {}", track.id).into());
                }
                validate_keys(&track.keys, shot.duration_ticks)?;
            }
            for label in &shot.labels {
                if !ids.contains(&label.parent)
                    || label.text.trim().is_empty()
                    || !label.offset.iter().all(|v| v.is_finite())
                    || !label.width.is_finite()
                    || !(1.0..=1280.0).contains(&label.width)
                    || !label.size.is_finite()
                    || !(8.0..=80.0).contains(&label.size)
                {
                    return Err("invalid biography attached text".into());
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
        return Err("tracks require a bounded key list starting at tick zero".into());
    }
    for (i, key) in keys.iter().enumerate() {
        if key.tick > duration
            || i > 0 && key.tick <= keys[i - 1].tick
            || !key
                .position
                .iter()
                .all(|v| v.is_finite() && v.abs() <= 10000.0)
            || !key.scale.is_finite()
            || !(0.01..=10.0).contains(&key.scale)
            || !key.rotation.is_finite()
            || !(-360.0..=360.0).contains(&key.rotation)
            || !key.opacity.is_finite()
            || !(0.0..=1.0).contains(&key.opacity)
        {
            return Err("invalid biography keyframe".into());
        }
    }
    Ok(())
}

/// Smoothstep preserves position and velocity at held poses and camera stops.
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
                rotation: lerp(a.rotation, b.rotation),
                opacity: lerp(a.opacity, b.opacity),
            };
        }
    }
    *keys.last().expect("validated nonempty track")
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
    fn biographies_have_independent_reusable_props_and_complete_timelines() {
        let (data, catalog) = assets();
        for piece in [
            "limousine",
            "duke.standing",
            "duke.seated",
            "oldc.seated",
            "desk",
            "computer",
            "book.kr",
        ] {
            assert!(catalog.pieces.contains_key(piece));
        }
        for id in ["duke", "c"] {
            assert_ne!(data.sample(id, 0).0.id, data.sample(id, 719).0.id);
            for ticks in 0..720 {
                let (shot, local) = data.sample(id, ticks);
                for track in &shot.tracks {
                    let key = sample_keys(&track.keys, local);
                    assert!(key.scale > 0.0 && key.opacity >= 0.0 && key.opacity <= 1.0);
                }
            }
        }
    }

    #[test]
    fn camera_and_car_stop_without_teleporting_or_losing_their_last_pose() {
        let (data, _) = assets();
        let shot = data.sample("duke", 0).0;
        let track = shot.tracks.iter().find(|t| t.id == "car").unwrap();
        let start = sample_keys(&track.keys, 0);
        let end = sample_keys(&track.keys, 719);
        assert!(start.position[0] > 1280.0 && end.position[0] < 1000.0);
        for t in 1..360 {
            let a = sample_keys(&track.keys, t - 1);
            let b = sample_keys(&track.keys, t);
            assert!((a.position[0] - b.position[0]).abs() < 18.0);
        }
    }

    #[test]
    fn bad_references_timing_and_parent_links_are_rejected_before_reload() {
        let (_, catalog) = assets();
        let json =
            fs::read_to_string(asset_path("assets/adventure/opening/scenes/scenes.json")).unwrap();
        for (from, to) in [
            ("\"limousine\"", "\"missing\""),
            ("\"parent\": \"kr\"", "\"parent\": \"missing\""),
            ("\"duration_ticks\": 360", "\"duration_ticks\": 0"),
        ] {
            assert!(json.contains(from));
            assert!(Biographies::from_json(&json.replace(from, to), &catalog).is_err());
        }
    }
}
