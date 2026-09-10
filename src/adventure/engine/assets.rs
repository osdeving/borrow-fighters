//! Loads the adventure's illustrations, typography and visual-only Rust atlas.
//!
//! System: Adventure presentation. This small catalog never reads fighting
//! manifests, boxes, character selection or gameplay tuning.

use raylib::prelude::*;
use serde::Deserialize;
use std::{collections::BTreeMap, error::Error, fs};

use crate::runtime_paths::asset_path;

/// One visual frame; the adventure simulation supplies all combat geometry.
#[derive(Debug, Deserialize)]
pub struct Frame {
    /// Visual action key local to this catalog.
    pub clip: String,
    /// Local texture filename.
    pub image: String,
    /// Atlas rectangle in pixels.
    pub rect: [f32; 4],
    /// Feet anchor relative to the rectangle.
    pub pivot: [f32; 2],
    /// Duration of this frame.
    pub duration_ms: u32,
}

/// Visual frames copied independently from the reviewed identity artwork.
#[derive(Debug, Deserialize)]
pub struct Catalog {
    /// Frames in playback order, grouped by clip.
    pub frames: Vec<Frame>,
}

impl Catalog {
    /// Selects a visual frame, holding the final pose for non-looping actions.
    pub fn frame(&self, clip: &str, seconds: f32, looping: bool) -> Option<&Frame> {
        let duration: u32 = self
            .frames
            .iter()
            .filter(|f| f.clip == clip)
            .map(|f| f.duration_ms)
            .sum();
        if duration == 0 {
            return None;
        }
        let elapsed = (seconds.max(0.0) * 1000.0) as u32;
        let mut elapsed = if looping {
            elapsed % duration
        } else {
            elapsed.min(duration - 1)
        };
        for frame in self.frames.iter().filter(|f| f.clip == clip) {
            if elapsed < frame.duration_ms {
                return Some(frame);
            }
            elapsed -= frame.duration_ms;
        }
        None
    }
}

/// Assets exclusively owned by the adventure executable.
pub struct Assets {
    /// Six chronological illustrations of Ada's first contact.
    pub ada: Texture2D,
    /// Twelve waking and compassionate poses of Rust.
    pub morning: Texture2D,
    /// Bedroom and street, arranged vertically.
    pub environments: Texture2D,
    /// Eight poses of the original erratic creature.
    pub erratic: Texture2D,
    /// Transparent-pixel bounds within each waking pose.
    pub morning_bounds: Vec<Rectangle>,
    /// Transparent-pixel bounds within each creature pose.
    pub erratic_bounds: Vec<Rectangle>,
    /// Independent catalog of Rust's combat visuals.
    pub rust: Catalog,
    /// Textures addressed by that catalog.
    pub rust_textures: BTreeMap<String, Texture2D>,
    /// Readable Portuguese interface font.
    pub body: Font,
    /// Narrative title font.
    pub title: Font,
}

impl Assets {
    /// Loads required assets, returning a useful error instead of silently substituting art.
    pub fn load(rl: &mut RaylibHandle, thread: &RaylibThread) -> Result<Self, Box<dyn Error>> {
        let rust: Catalog = serde_json::from_str(&fs::read_to_string(asset_path(
            "assets/adventure/rust/animation.json",
        ))?)?;
        let mut rust_textures = BTreeMap::new();
        for frame in &rust.frames {
            if !rust_textures.contains_key(&frame.image) {
                let texture = texture(rl, thread, &format!("rust/{}", frame.image))?;
                rust_textures.insert(frame.image.clone(), texture);
            }
        }
        let glyphs: String = (32..=255).filter_map(char::from_u32).collect();
        let body = rl.load_font_ex(
            thread,
            &asset_path("assets/adventure/fonts/Barlow-Regular.ttf").to_string_lossy(),
            48,
            Some(&glyphs),
        )?;
        let title = rl.load_font_ex(
            thread,
            &asset_path("assets/adventure/fonts/Lora-Variable.ttf").to_string_lossy(),
            64,
            Some(&glyphs),
        )?;
        Ok(Self {
            ada: texture(rl, thread, "ada-prologue.png")?,
            morning: texture(rl, thread, "rust-morning.png")?,
            environments: texture(rl, thread, "adventure-environments.png")?,
            erratic: texture(rl, thread, "erratic.png")?,
            morning_bounds: pose_bounds("rust-morning-poses.json", 12)?,
            erratic_bounds: pose_bounds("erratic-poses.json", 8)?,
            rust,
            rust_textures,
            body,
            title,
        })
    }
}

fn pose_bounds(name: &str, count: usize) -> Result<Vec<Rectangle>, Box<dyn Error>> {
    #[derive(Deserialize)]
    struct Poses {
        rects: Vec<[f32; 4]>,
        image_size: [f32; 2],
    }
    let poses: Poses = serde_json::from_str(&fs::read_to_string(asset_path(format!(
        "assets/adventure/{name}"
    )))?)?;
    if poses.rects.len() != count {
        return Err(format!("{name}: expected {count} poses").into());
    }
    poses
        .rects
        .into_iter()
        .map(|[x, y, w, h]| {
            if ![x, y, w, h].iter().all(|v| v.is_finite())
                || x < 0.0
                || y < 0.0
                || w <= 0.0
                || h <= 0.0
                || x + w > poses.image_size[0]
                || y + h > poses.image_size[1]
            {
                Err(format!("invalid pose bounds in {name}").into())
            } else {
                Ok(Rectangle::new(x, y, w, h))
            }
        })
        .collect()
}

fn texture(
    rl: &mut RaylibHandle,
    thread: &RaylibThread,
    name: &str,
) -> Result<Texture2D, Box<dyn Error>> {
    let path = asset_path(format!("assets/adventure/{name}"));
    let image = rl
        .load_texture(thread, &path.to_string_lossy())
        .map_err(|e| format!("{}: {e}", path.display()))?;
    image.set_texture_filter(thread, TextureFilter::TEXTURE_FILTER_BILINEAR);
    Ok(image)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn independent_catalog_has_required_actions_and_only_local_textures() {
        let catalog: Catalog = serde_json::from_str(include_str!(
            "../../../assets/adventure/rust/animation.json"
        ))
        .unwrap();
        for clip in [
            "idle",
            "walk",
            "jump",
            "block",
            "punch_light",
            "punch_heavy",
            "hit",
            "defeat",
        ] {
            assert!(catalog.frame(clip, 0.0, false).is_some(), "{clip}");
        }
        for frame in &catalog.frames {
            assert!(frame.duration_ms > 0);
            assert!(!frame.image.contains('/') && !frame.image.contains('\\'));
            assert!(frame.rect[2] > 0.0 && frame.rect[3] > 0.0);
        }
    }
}
