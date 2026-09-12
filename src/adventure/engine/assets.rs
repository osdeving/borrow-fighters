//! Loads the adventure's illustrations, typography and visual-only Rust atlas.
//!
//! System: Adventure presentation. This small catalog never reads fighting
//! manifests, boxes, character selection or gameplay tuning.

use raylib::core::{AsRawMut, text::RaylibFont};
use raylib::prelude::*;
use serde::Deserialize;
use std::{error::Error, fs};

use crate::{adventure::text::TextCatalog, runtime_paths::asset_path};

/// Assets exclusively owned by the adventure executable.
pub struct Assets {
    /// Shared walking and kick clips with editable stride and support anchors.
    pub locomotion: super::locomotion::LocomotionAssets,
    /// Independent imagery for the newspaper/character/title presentation.
    pub opening: super::opening::OpeningAssets,
    /// Editable on-disk narrative and interface copy.
    pub text: TextCatalog,
    /// Six chronological illustrations of Ada's first contact.
    pub ada: Texture2D,
    /// Twelve waking and compassionate poses of Rust.
    pub morning: Texture2D,
    /// Bedroom and street, arranged vertically.
    pub environments: Texture2D,
    /// Replaceable actors and props, with a separate scene composition.
    pub street: super::pieces::StreetPieces,
    /// Eight poses of the original erratic creature.
    pub erratic: Texture2D,
    /// Transparent-pixel bounds within each waking pose.
    pub morning_bounds: Vec<Rectangle>,
    /// Transparent-pixel bounds within each creature pose.
    pub erratic_bounds: Vec<Rectangle>,
    /// Sixteen actions authored for this adventure, matching the morning.
    pub actions: Texture2D,
    /// Independently measured action poses.
    pub action_bounds: Vec<Rectangle>,
    /// Readable Portuguese interface font.
    pub body: Font,
    /// Narrative title font.
    pub title: Font,
    /// Heavier, filtered lettering for small signs inside the street scene.
    pub signage: Font,
}

impl Assets {
    /// Loads required assets, returning a useful error instead of silently substituting art.
    pub fn load(
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        text: TextCatalog,
    ) -> Result<Self, Box<dyn Error>> {
        let mut glyphs: String = (32..=591).filter_map(char::from_u32).collect();
        glyphs.push_str("—–“”‘’…←→↑↓");
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
        let mut signage = rl.load_font_ex(
            thread,
            &asset_path("assets/adventure/fonts/BarlowCondensed-SemiBold.ttf").to_string_lossy(),
            64,
            Some(&glyphs),
        )?;
        // SAFETY: the live font owns this texture. Raylib updates its mipmap
        // count without changing glyph pointers or transferring ownership.
        unsafe { raylib::ffi::GenTextureMipmaps(&mut signage.as_raw_mut().texture) };
        signage
            .texture()
            .set_texture_filter(thread, TextureFilter::TEXTURE_FILTER_TRILINEAR);
        Ok(Self {
            locomotion: super::locomotion::LocomotionAssets::load(rl, thread)?,
            opening: super::opening::OpeningAssets::load(rl, thread, &text)?,
            text,
            ada: texture(rl, thread, "ada-prologue.png")?,
            morning: texture(rl, thread, "rust-morning.png")?,
            environments: texture(rl, thread, "prologue-environments.png")?,
            street: super::pieces::StreetPieces::load(rl, thread)?,
            erratic: texture(rl, thread, "erratic.png")?,
            morning_bounds: pose_bounds("rust-morning-poses.json", 12)?,
            erratic_bounds: pose_bounds("erratic-poses.json", 8)?,
            actions: texture(rl, thread, "rust-actions.png")?,
            action_bounds: pose_bounds("rust-actions-poses.json", 16)?,
            body,
            title,
            signage,
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
    fn every_authored_action_has_valid_independent_pose_bounds() {
        for (name, count) in [
            ("rust-morning-poses.json", 12),
            ("erratic-poses.json", 8),
            ("rust-actions-poses.json", 16),
            ("street-life.json", 12),
            ("street-traffic.json", 4),
        ] {
            let frames = pose_bounds(name, count).unwrap();
            assert_eq!(frames.len(), count);
            assert!(frames.iter().all(|f| f.width > 0.0 && f.height > 0.0));
        }
    }
}
