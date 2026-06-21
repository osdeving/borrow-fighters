//! Loads and validates sprite manifest metadata.
//!
//! Sprite manifests describe atlas frames, pivots, and clips without requiring
//! Raylib, so they can be tested as ordinary data.

use std::{
    collections::HashSet,
    error::Error,
    fmt::{Display, Formatter},
    fs,
    path::{Path, PathBuf},
};

use serde::Deserialize;

pub const SPRITE_SCHEMA: &str = "borrow-fighters.sprite.v1";

/// Metadata for one sprite atlas.
#[derive(Clone, Debug, Deserialize)]
pub struct SpriteManifest {
    pub schema: String,
    pub image: String,
    pub source: Option<String>,
    pub cell: SpriteSize,
    pub default_pivot: SpritePivot,
    pub scale: Option<f32>,
    pub frames: Vec<SpriteFrame>,
    pub clips: Vec<SpriteClip>,
    #[serde(default)]
    pub notes: Vec<String>,
}

/// Pixel size in a sprite manifest.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
pub struct SpriteSize {
    pub w: i32,
    pub h: i32,
}

/// Pivot point inside one atlas frame.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
pub struct SpritePivot {
    pub x: i32,
    pub y: i32,
}

/// Integer atlas rectangle.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
pub struct SpriteRect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

/// One named frame inside the atlas.
#[derive(Clone, Debug, Deserialize)]
pub struct SpriteFrame {
    pub name: String,
    pub clip: String,
    pub duration_ms: u32,
    pub pivot: SpritePivot,
    pub source_crop: Option<SpriteRect>,
    pub trimmed_bounds: Option<SpriteRect>,
    pub frame: SpriteRect,
}

/// Ordered animation clip referencing named frames.
#[derive(Clone, Debug, Deserialize)]
pub struct SpriteClip {
    pub name: String,
    pub r#loop: bool,
    pub frames: Vec<String>,
}

/// Error returned while loading or validating a sprite manifest.
#[derive(Debug)]
pub enum SpriteManifestError {
    Io {
        path: PathBuf,
        source: std::io::Error,
    },
    Json {
        path: PathBuf,
        source: serde_json::Error,
    },
    Invalid(String),
}

impl SpriteManifest {
    /// Loads a sprite manifest from disk and validates its internal references.
    pub fn load(path: impl AsRef<Path>) -> Result<Self, SpriteManifestError> {
        let path = path.as_ref().to_path_buf();
        let content = fs::read_to_string(&path).map_err(|source| SpriteManifestError::Io {
            path: path.clone(),
            source,
        })?;
        let manifest: Self =
            serde_json::from_str(&content).map_err(|source| SpriteManifestError::Json {
                path: path.clone(),
                source,
            })?;
        manifest.validate()?;
        Ok(manifest)
    }

    /// Validates required fields, frame uniqueness, and clip references.
    pub fn validate(&self) -> Result<(), SpriteManifestError> {
        if self.schema != SPRITE_SCHEMA {
            return Err(SpriteManifestError::Invalid(format!(
                "unsupported sprite schema '{}'",
                self.schema
            )));
        }

        if self.image.trim().is_empty() {
            return Err(SpriteManifestError::Invalid(
                "sprite manifest image is required".to_string(),
            ));
        }

        if self.cell.w <= 0 || self.cell.h <= 0 {
            return Err(SpriteManifestError::Invalid(
                "sprite manifest cell size must be positive".to_string(),
            ));
        }

        if self.frames.is_empty() {
            return Err(SpriteManifestError::Invalid(
                "sprite manifest must contain frames".to_string(),
            ));
        }

        let mut frame_names = HashSet::new();
        for frame in &self.frames {
            validate_frame(frame)?;
            if !frame_names.insert(frame.name.as_str()) {
                return Err(SpriteManifestError::Invalid(format!(
                    "duplicate sprite frame '{}'",
                    frame.name
                )));
            }
        }

        if self.clips.is_empty() {
            return Err(SpriteManifestError::Invalid(
                "sprite manifest must contain clips".to_string(),
            ));
        }

        let mut clip_names = HashSet::new();
        for clip in &self.clips {
            if clip.name.trim().is_empty() {
                return Err(SpriteManifestError::Invalid(
                    "sprite clip name is required".to_string(),
                ));
            }
            if !clip_names.insert(clip.name.as_str()) {
                return Err(SpriteManifestError::Invalid(format!(
                    "duplicate sprite clip '{}'",
                    clip.name
                )));
            }
            if clip.frames.is_empty() {
                return Err(SpriteManifestError::Invalid(format!(
                    "sprite clip '{}' must contain frames",
                    clip.name
                )));
            }
            for frame_name in &clip.frames {
                if !frame_names.contains(frame_name.as_str()) {
                    return Err(SpriteManifestError::Invalid(format!(
                        "sprite clip '{}' references missing frame '{}'",
                        clip.name, frame_name
                    )));
                }
            }
        }

        Ok(())
    }

    /// Returns the atlas image path resolved next to the manifest file.
    pub fn image_path(&self, manifest_path: impl AsRef<Path>) -> PathBuf {
        let image = Path::new(&self.image);
        if image.is_absolute() {
            return image.to_path_buf();
        }

        manifest_path
            .as_ref()
            .parent()
            .unwrap_or_else(|| Path::new(""))
            .join(image)
    }

    /// Returns a frame by name.
    pub fn frame_named(&self, name: &str) -> Option<&SpriteFrame> {
        self.frames.iter().find(|frame| frame.name == name)
    }

    /// Returns a clip by name.
    pub fn clip_named(&self, name: &str) -> Option<&SpriteClip> {
        self.clips.iter().find(|clip| clip.name == name)
    }
}

impl Display for SpriteManifestError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io { path, source } => {
                write!(formatter, "could not read {}: {source}", path.display())
            }
            Self::Json { path, source } => {
                write!(formatter, "could not parse {}: {source}", path.display())
            }
            Self::Invalid(message) => formatter.write_str(message),
        }
    }
}

impl Error for SpriteManifestError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Io { source, .. } => Some(source),
            Self::Json { source, .. } => Some(source),
            Self::Invalid(_) => None,
        }
    }
}

fn validate_frame(frame: &SpriteFrame) -> Result<(), SpriteManifestError> {
    if frame.name.trim().is_empty() {
        return Err(SpriteManifestError::Invalid(
            "sprite frame name is required".to_string(),
        ));
    }

    if frame.clip.trim().is_empty() {
        return Err(SpriteManifestError::Invalid(format!(
            "sprite frame '{}' must declare a source clip",
            frame.name
        )));
    }

    if frame.duration_ms == 0 {
        return Err(SpriteManifestError::Invalid(format!(
            "sprite frame '{}' must have a positive duration",
            frame.name
        )));
    }

    if frame.frame.w <= 0 || frame.frame.h <= 0 {
        return Err(SpriteManifestError::Invalid(format!(
            "sprite frame '{}' must have a positive rectangle",
            frame.name
        )));
    }

    if frame.pivot.x < 0
        || frame.pivot.y < 0
        || frame.pivot.x > frame.frame.w
        || frame.pivot.y > frame.frame.h
    {
        return Err(SpriteManifestError::Invalid(format!(
            "sprite frame '{}' pivot must be inside the frame",
            frame.name
        )));
    }

    Ok(())
}
