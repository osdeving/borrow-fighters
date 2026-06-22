//! Tracks the standalone sprite combat viewer state.
//!
//! System: Tooling scene. This module owns testable viewer state for sprite
//! atlas inspection; Raylib input and drawing stay in the app/render boundary.

use std::{
    error::Error,
    fmt::{Display, Formatter},
    path::{Path, PathBuf},
};

use crate::{
    config::FLOOR_Y,
    engine::sprites::{SpriteFrame, SpriteManifest, SpriteManifestError},
};

const DEFAULT_ANCHOR_X: f32 = 480.0;

/// Launch data for the standalone sprite viewer.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SpriteViewerOptions {
    pub manifest_path: PathBuf,
    pub initial_clip: Option<String>,
}

/// Input snapshot consumed by the sprite viewer.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SpriteViewerInput {
    pub next_clip: bool,
    pub previous_clip: bool,
    pub next_frame: bool,
    pub previous_frame: bool,
    pub toggle_playback: bool,
    pub toggle_grid: bool,
    pub toggle_pivot: bool,
    pub toggle_bounds: bool,
    pub reset_position: bool,
    pub mouse_position: ViewerPoint,
    pub mouse_pressed: bool,
    pub mouse_down: bool,
    pub mouse_released: bool,
}

/// Screen-space point used by the viewer without depending on Raylib.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ViewerPoint {
    pub x: f32,
    pub y: f32,
}

impl ViewerPoint {
    /// Creates a screen-space point.
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

/// Screen-space rectangle used by the viewer without depending on Raylib.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ViewerRect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl ViewerRect {
    /// Returns true when a point is inside this rectangle.
    pub fn contains(self, point: ViewerPoint) -> bool {
        point.x >= self.x
            && point.x <= self.x + self.width
            && point.y >= self.y
            && point.y <= self.y + self.height
    }
}

/// Standalone state for inspecting a sprite manifest and atlas.
#[derive(Debug)]
pub struct SpriteViewer {
    options: SpriteViewerOptions,
    manifest: SpriteManifest,
    image_path: PathBuf,
    clip_index: usize,
    frame_index: usize,
    frame_elapsed_ms: f32,
    playing: bool,
    show_grid: bool,
    show_pivot: bool,
    show_bounds: bool,
    anchor: ViewerPoint,
    dragging: bool,
    drag_offset: ViewerPoint,
    texture_error: Option<String>,
}

/// Error returned when the viewer cannot load a manifest.
#[derive(Debug)]
pub enum SpriteViewerError {
    Manifest {
        path: PathBuf,
        source: SpriteManifestError,
    },
    UnknownInitialClip {
        clip: String,
        path: PathBuf,
    },
}

impl SpriteViewer {
    /// Loads a sprite manifest and creates viewer state.
    pub fn load(options: SpriteViewerOptions) -> Result<Self, SpriteViewerError> {
        let manifest = SpriteManifest::load(&options.manifest_path).map_err(|source| {
            SpriteViewerError::Manifest {
                path: options.manifest_path.clone(),
                source,
            }
        })?;
        let image_path = manifest.image_path(&options.manifest_path);
        let clip_index = match options.initial_clip.as_deref() {
            Some(clip_name) => manifest
                .clips
                .iter()
                .position(|clip| clip.name == clip_name)
                .ok_or_else(|| SpriteViewerError::UnknownInitialClip {
                    clip: clip_name.to_string(),
                    path: options.manifest_path.clone(),
                })?,
            None => 0,
        };

        Ok(Self {
            options,
            manifest,
            image_path,
            clip_index,
            frame_index: 0,
            frame_elapsed_ms: 0.0,
            playing: true,
            show_grid: true,
            show_pivot: true,
            show_bounds: true,
            anchor: ViewerPoint::new(DEFAULT_ANCHOR_X, FLOOR_Y),
            dragging: false,
            drag_offset: ViewerPoint::default(),
            texture_error: None,
        })
    }

    /// Advances animation and applies viewer controls.
    pub fn update(&mut self, input: SpriteViewerInput, delta_seconds: f32) {
        if input.previous_clip {
            self.step_clip(-1);
        }
        if input.next_clip {
            self.step_clip(1);
        }
        if input.previous_frame {
            self.playing = false;
            self.step_frame(-1);
        }
        if input.next_frame {
            self.playing = false;
            self.step_frame(1);
        }
        if input.toggle_playback {
            self.playing = !self.playing;
        }
        if input.toggle_grid {
            self.show_grid = !self.show_grid;
        }
        if input.toggle_pivot {
            self.show_pivot = !self.show_pivot;
        }
        if input.toggle_bounds {
            self.show_bounds = !self.show_bounds;
        }
        if input.reset_position {
            self.anchor = ViewerPoint::new(DEFAULT_ANCHOR_X, FLOOR_Y);
        }

        self.update_drag(input);

        if self.playing {
            self.advance_animation(delta_seconds);
        }
    }

    /// Records a texture loading warning that should be visible in the viewer.
    pub fn set_texture_error(&mut self, message: impl Into<String>) {
        self.texture_error = Some(message.into());
    }

    /// Returns the manifest path passed to the viewer.
    pub fn manifest_path(&self) -> &Path {
        &self.options.manifest_path
    }

    /// Returns the atlas image path resolved from the manifest.
    pub fn image_path(&self) -> &Path {
        &self.image_path
    }

    /// Returns the loaded sprite manifest.
    pub const fn manifest(&self) -> &SpriteManifest {
        &self.manifest
    }

    /// Returns the selected clip name.
    pub fn current_clip_name(&self) -> &str {
        &self.manifest.clips[self.clip_index].name
    }

    /// Returns the selected frame metadata.
    pub fn current_frame(&self) -> &SpriteFrame {
        let frame_name = &self.manifest.clips[self.clip_index].frames[self.frame_index];
        self.manifest
            .frame_named(frame_name)
            .expect("validated sprite clip references must resolve")
    }

    /// Returns the selected clip index and clip count.
    pub fn clip_position(&self) -> (usize, usize) {
        (self.clip_index, self.manifest.clips.len())
    }

    /// Returns the selected frame index and frame count for the current clip.
    pub fn frame_position(&self) -> (usize, usize) {
        (
            self.frame_index,
            self.manifest.clips[self.clip_index].frames.len(),
        )
    }

    /// Returns the runtime scale from the manifest, falling back to 1.0.
    pub fn scale(&self) -> f32 {
        self.manifest.scale.unwrap_or(1.0).max(0.1)
    }

    /// Returns the current anchor/pivot target in screen space.
    pub const fn anchor(&self) -> ViewerPoint {
        self.anchor
    }

    /// Returns whether the animation is currently playing.
    pub const fn playing(&self) -> bool {
        self.playing
    }

    /// Returns whether grid drawing is enabled.
    pub const fn show_grid(&self) -> bool {
        self.show_grid
    }

    /// Returns whether pivot drawing is enabled.
    pub const fn show_pivot(&self) -> bool {
        self.show_pivot
    }

    /// Returns whether frame/trim bounds drawing is enabled.
    pub const fn show_bounds(&self) -> bool {
        self.show_bounds
    }

    /// Returns a texture loading warning, if one happened.
    pub fn texture_error(&self) -> Option<&str> {
        self.texture_error.as_deref()
    }

    /// Returns the current sprite frame rectangle in screen space.
    pub fn sprite_screen_rect(&self) -> ViewerRect {
        let frame = self.current_frame();
        let scale = self.scale();
        ViewerRect {
            x: self.anchor.x - frame.pivot.x as f32 * scale,
            y: self.anchor.y - frame.pivot.y as f32 * scale,
            width: frame.frame.w as f32 * scale,
            height: frame.frame.h as f32 * scale,
        }
    }

    fn update_drag(&mut self, input: SpriteViewerInput) {
        if input.mouse_pressed && self.sprite_screen_rect().contains(input.mouse_position) {
            self.dragging = true;
            self.drag_offset = ViewerPoint::new(
                input.mouse_position.x - self.anchor.x,
                input.mouse_position.y - self.anchor.y,
            );
        }

        if self.dragging && input.mouse_down {
            self.anchor = ViewerPoint::new(
                input.mouse_position.x - self.drag_offset.x,
                input.mouse_position.y - self.drag_offset.y,
            );
        }

        if input.mouse_released {
            self.dragging = false;
        }
    }

    fn advance_animation(&mut self, delta_seconds: f32) {
        self.frame_elapsed_ms += delta_seconds.max(0.0) * 1000.0;
        loop {
            let duration = self.current_frame().duration_ms as f32;
            if self.frame_elapsed_ms < duration {
                break;
            }
            self.frame_elapsed_ms -= duration;
            if !self.advance_one_frame() {
                self.playing = false;
                self.frame_elapsed_ms = 0.0;
                break;
            }
        }
    }

    fn step_clip(&mut self, direction: i32) {
        let len = self.manifest.clips.len();
        self.clip_index = wrap_index(self.clip_index, len, direction);
        self.frame_index = 0;
        self.frame_elapsed_ms = 0.0;
    }

    fn step_frame(&mut self, direction: i32) {
        let len = self.manifest.clips[self.clip_index].frames.len();
        self.frame_index = wrap_index(self.frame_index, len, direction);
        self.frame_elapsed_ms = 0.0;
    }

    fn advance_one_frame(&mut self) -> bool {
        let clip = &self.manifest.clips[self.clip_index];
        if self.frame_index + 1 < clip.frames.len() {
            self.frame_index += 1;
            return true;
        }

        if clip.r#loop {
            self.frame_index = 0;
            true
        } else {
            false
        }
    }
}

impl Display for SpriteViewerError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Manifest { path, source } => {
                write!(
                    formatter,
                    "could not load sprite manifest {}: {source}",
                    path.display()
                )
            }
            Self::UnknownInitialClip { clip, path } => {
                write!(
                    formatter,
                    "sprite manifest {} does not contain clip '{clip}'",
                    path.display()
                )
            }
        }
    }
}

impl Error for SpriteViewerError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Manifest { source, .. } => Some(source),
            Self::UnknownInitialClip { .. } => None,
        }
    }
}

fn wrap_index(current: usize, len: usize, direction: i32) -> usize {
    debug_assert!(len > 0);
    let len = len as i32;
    (current as i32 + direction).rem_euclid(len) as usize
}
