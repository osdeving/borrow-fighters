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
    characters::{CharacterId, character_spec},
    combat::{
        fighter::{Facing, Fighter, FighterBodyParts, PlayerSlot},
        move_data::{MoveInputKind, MoveSpec, move_spec_for_input},
        projectile::Projectile,
    },
    config::FLOOR_Y,
    engine::sprites::{SpriteFrame, SpriteManifest, SpriteManifestError},
    math::rect::Rect,
    scenes::combat_lab::CombatLabMove,
};

const DEFAULT_ANCHOR_X: f32 = 480.0;
const DEFAULT_DUMMY_ANCHOR_X: f32 = 680.0;
const ZOOM_MIN: f32 = 0.25;
const ZOOM_MAX: f32 = 4.0;
const ZOOM_STEP: f32 = 0.12;

/// Launch data for the standalone sprite viewer.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SpriteViewerOptions {
    pub manifest_path: PathBuf,
    pub initial_clip: Option<String>,
    pub character: Option<CharacterId>,
    pub selected_move: CombatLabMove,
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
    pub toggle_dummy: bool,
    pub toggle_combat_overlay: bool,
    pub reload_manifest: bool,
    pub reset_zoom: bool,
    pub screenshot_requested: bool,
    pub zoom_delta: f32,
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
    show_dummy: bool,
    show_combat_overlay: bool,
    zoom: f32,
    anchor: ViewerPoint,
    dummy_anchor: ViewerPoint,
    dragging: Option<DragTarget>,
    drag_offset: ViewerPoint,
    texture_error: Option<String>,
    status_message: Option<String>,
}

/// Combat shapes projected into viewer screen coordinates.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SpriteCombatOverlay {
    pub character: CharacterId,
    pub selected_move: CombatLabMove,
    pub move_label: &'static str,
    pub body: Rect,
    pub hurtboxes: FighterBodyParts,
    pub hitbox: Option<Rect>,
    pub projectile: Option<Rect>,
    pub projectile_origin: Option<ViewerPoint>,
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DragTarget {
    Main,
    Dummy,
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

        let show_combat_overlay = options.character.is_some();

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
            show_dummy: true,
            show_combat_overlay,
            zoom: 1.0,
            anchor: ViewerPoint::new(DEFAULT_ANCHOR_X, FLOOR_Y),
            dummy_anchor: ViewerPoint::new(DEFAULT_DUMMY_ANCHOR_X, FLOOR_Y),
            dragging: None,
            drag_offset: ViewerPoint::default(),
            texture_error: None,
            status_message: None,
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
        if input.toggle_dummy {
            self.show_dummy = !self.show_dummy;
        }
        if input.toggle_combat_overlay {
            self.show_combat_overlay = !self.show_combat_overlay;
        }
        if input.reset_position {
            self.anchor = ViewerPoint::new(DEFAULT_ANCHOR_X, FLOOR_Y);
            self.dummy_anchor = ViewerPoint::new(DEFAULT_DUMMY_ANCHOR_X, FLOOR_Y);
        }
        if input.reset_zoom {
            self.zoom = 1.0;
        }
        if input.zoom_delta.abs() > f32::EPSILON {
            let factor = 1.0 + input.zoom_delta * ZOOM_STEP;
            self.zoom = (self.zoom * factor.max(0.25)).clamp(ZOOM_MIN, ZOOM_MAX);
        }

        self.update_drag(input);

        if self.playing {
            self.advance_animation(delta_seconds);
        }
    }

    /// Reloads the manifest from disk, preserving clip/frame where possible.
    pub fn reload_manifest(&mut self) -> Result<bool, SpriteViewerError> {
        let previous_image_path = self.image_path.clone();
        let previous_clip = self.current_clip_name().to_string();
        let previous_frame_index = self.frame_index;
        let manifest = SpriteManifest::load(&self.options.manifest_path).map_err(|source| {
            SpriteViewerError::Manifest {
                path: self.options.manifest_path.clone(),
                source,
            }
        })?;
        let image_path = manifest.image_path(&self.options.manifest_path);
        let clip_index = select_clip_index(
            &manifest,
            Some(previous_clip.as_str()),
            self.options.initial_clip.as_deref(),
        );
        let frame_len = manifest.clips[clip_index].frames.len();

        self.manifest = manifest;
        self.image_path = image_path;
        self.clip_index = clip_index;
        self.frame_index = previous_frame_index.min(frame_len.saturating_sub(1));
        self.frame_elapsed_ms = 0.0;
        self.texture_error = None;
        self.status_message = Some("Manifesto recarregado.".to_string());

        Ok(previous_image_path != self.image_path)
    }

    /// Records a texture loading warning that should be visible in the viewer.
    pub fn set_texture_error(&mut self, message: impl Into<String>) {
        self.texture_error = Some(message.into());
    }

    /// Records a transient viewer status message.
    pub fn set_status_message(&mut self, message: impl Into<String>) {
        self.status_message = Some(message.into());
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
        self.manifest_scale() * self.zoom
    }

    /// Returns the scale stored in the manifest without the viewer zoom.
    pub fn manifest_scale(&self) -> f32 {
        self.manifest.scale.unwrap_or(1.0).max(0.1)
    }

    /// Returns the viewer zoom multiplier.
    pub const fn zoom(&self) -> f32 {
        self.zoom
    }

    /// Returns the current anchor/pivot target in screen space.
    pub const fn anchor(&self) -> ViewerPoint {
        self.anchor
    }

    /// Returns the dummy anchor/pivot target in screen space.
    pub const fn dummy_anchor(&self) -> ViewerPoint {
        self.dummy_anchor
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

    /// Returns whether the mirrored dummy is visible.
    pub const fn show_dummy(&self) -> bool {
        self.show_dummy
    }

    /// Returns whether combat metrics should be drawn.
    pub const fn show_combat_overlay(&self) -> bool {
        self.show_combat_overlay
    }

    /// Returns a texture loading warning, if one happened.
    pub fn texture_error(&self) -> Option<&str> {
        self.texture_error.as_deref()
    }

    /// Returns the latest viewer status message, if one exists.
    pub fn status_message(&self) -> Option<&str> {
        self.status_message.as_deref()
    }

    /// Returns the current sprite frame rectangle in screen space.
    pub fn sprite_screen_rect(&self) -> ViewerRect {
        self.sprite_screen_rect_at(self.anchor, false)
    }

    /// Returns the mirrored dummy frame rectangle in screen space.
    pub fn dummy_screen_rect(&self) -> ViewerRect {
        self.sprite_screen_rect_at(self.dummy_anchor, true)
    }

    /// Returns the horizontal distance between main and dummy anchors.
    pub fn dummy_distance(&self) -> f32 {
        (self.dummy_anchor.x - self.anchor.x).abs()
    }

    /// Returns combat boxes aligned to the current main anchor.
    pub fn combat_overlay(&self) -> Option<SpriteCombatOverlay> {
        if !self.show_combat_overlay {
            return None;
        }
        let character = self.options.character?;
        let spec = character_spec(character);
        let mut fighter = Fighter::new_with_projectile_loadout(
            PlayerSlot::One,
            spec.fighter_name,
            spec.stats.max_health,
            spec.move_ids,
            spec.projectile,
            0.0,
        );
        fighter.facing = Facing::Right;
        if matches!(
            self.options.selected_move,
            CombatLabMove::Sweep | CombatLabMove::AntiAir
        ) {
            fighter.crouching = true;
        }
        if matches!(
            self.options.selected_move,
            CombatLabMove::AirPunch | CombatLabMove::AirKick
        ) {
            fighter.grounded = false;
            fighter.position.y -= 92.0;
        }
        align_fighter_to_anchor(&mut fighter, self.anchor);

        let body = fighter.body_rect();
        let hurtboxes = fighter.hurtboxes();
        let (move_label, hitbox, projectile, projectile_origin) =
            self.combat_attack_shapes(&fighter, spec.move_ids, spec.projectile);

        Some(SpriteCombatOverlay {
            character,
            selected_move: self.options.selected_move,
            move_label,
            body,
            hurtboxes,
            hitbox,
            projectile,
            projectile_origin,
        })
    }

    fn sprite_screen_rect_at(&self, anchor: ViewerPoint, mirrored: bool) -> ViewerRect {
        let frame = self.current_frame();
        let scale = self.scale();
        let width = frame.frame.w as f32 * scale;
        let pivot_x = frame.pivot.x as f32 * scale;
        let x = if mirrored {
            anchor.x - (width - pivot_x)
        } else {
            anchor.x - pivot_x
        };
        ViewerRect {
            x,
            y: anchor.y - frame.pivot.y as f32 * scale,
            width,
            height: frame.frame.h as f32 * scale,
        }
    }

    fn update_drag(&mut self, input: SpriteViewerInput) {
        if input.mouse_pressed {
            let target =
                if self.show_dummy && self.dummy_screen_rect().contains(input.mouse_position) {
                    Some(DragTarget::Dummy)
                } else if self.sprite_screen_rect().contains(input.mouse_position) {
                    Some(DragTarget::Main)
                } else {
                    None
                };

            if let Some(target) = target {
                self.dragging = Some(target);
                let anchor = self.anchor_for_target(target);
                self.drag_offset = ViewerPoint::new(
                    input.mouse_position.x - anchor.x,
                    input.mouse_position.y - anchor.y,
                );
            }
        }

        if let Some(target) = self.dragging
            && input.mouse_down
        {
            let anchor = ViewerPoint::new(
                input.mouse_position.x - self.drag_offset.x,
                input.mouse_position.y - self.drag_offset.y,
            );
            self.set_anchor_for_target(target, anchor);
        }

        if input.mouse_released {
            self.dragging = None;
        }
    }

    fn anchor_for_target(&self, target: DragTarget) -> ViewerPoint {
        match target {
            DragTarget::Main => self.anchor,
            DragTarget::Dummy => self.dummy_anchor,
        }
    }

    fn set_anchor_for_target(&mut self, target: DragTarget, anchor: ViewerPoint) {
        match target {
            DragTarget::Main => self.anchor = anchor,
            DragTarget::Dummy => self.dummy_anchor = anchor,
        }
    }

    fn combat_attack_shapes(
        &self,
        fighter: &Fighter,
        move_ids: &[crate::combat::move_data::MoveId],
        projectile_spec: crate::combat::projectile::ProjectileSpec,
    ) -> (
        &'static str,
        Option<Rect>,
        Option<Rect>,
        Option<ViewerPoint>,
    ) {
        if self.options.selected_move == CombatLabMove::Projectile {
            let projectile = Projectile::from_fighter_with_spec(fighter, projectile_spec);
            let rect = projectile.rect();
            return (
                "Projectile",
                None,
                Some(rect),
                Some(ViewerPoint::new(rect.x, rect.y + rect.height * 0.5)),
            );
        }

        let Some(input_kind) = input_kind_for_move(self.options.selected_move) else {
            return ("None", None, None, None);
        };
        let Some(spec) = move_spec_for_input(move_ids, input_kind) else {
            return ("Unmapped", None, None, None);
        };

        (spec.label, Some(hitbox_for_move(fighter, spec)), None, None)
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

fn select_clip_index(
    manifest: &SpriteManifest,
    preferred_clip: Option<&str>,
    fallback_clip: Option<&str>,
) -> usize {
    preferred_clip
        .and_then(|clip| {
            manifest
                .clips
                .iter()
                .position(|candidate| candidate.name == clip)
        })
        .or_else(|| {
            fallback_clip.and_then(|clip| {
                manifest
                    .clips
                    .iter()
                    .position(|candidate| candidate.name == clip)
            })
        })
        .unwrap_or(0)
}

fn align_fighter_to_anchor(fighter: &mut Fighter, anchor: ViewerPoint) {
    let body = fighter.body_rect();
    fighter.position.x += anchor.x - body.center_x();
    fighter.position.y += anchor.y - body.bottom();
}

fn input_kind_for_move(selected_move: CombatLabMove) -> Option<MoveInputKind> {
    match selected_move {
        CombatLabMove::LightPunch => Some(MoveInputKind::LightPunch),
        CombatLabMove::HeavyPunch => Some(MoveInputKind::HeavyPunch),
        CombatLabMove::Kick => Some(MoveInputKind::Kick),
        CombatLabMove::Sweep => Some(MoveInputKind::Sweep),
        CombatLabMove::Overhead => Some(MoveInputKind::Overhead),
        CombatLabMove::AntiAir => Some(MoveInputKind::AntiAir),
        CombatLabMove::AirPunch => Some(MoveInputKind::AirPunch),
        CombatLabMove::AirKick => Some(MoveInputKind::AirKick),
        CombatLabMove::Throw => Some(MoveInputKind::Throw),
        CombatLabMove::Projectile => None,
    }
}

fn hitbox_for_move(fighter: &Fighter, spec: MoveSpec) -> Rect {
    let body = fighter.body_rect();
    let hitbox = spec.hitbox;
    let x = match fighter.facing {
        Facing::Right => body.right(),
        Facing::Left => body.x - hitbox.width,
    };
    Rect::new(x, body.y + hitbox.y_offset, hitbox.width, hitbox.height)
}
