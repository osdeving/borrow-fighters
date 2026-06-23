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
    characters::{
        CHARACTER_BODY_METRICS_PATH, CharacterBodyMetricsCatalog, CharacterBodyMetricsError,
        CharacterId, character_spec,
    },
    combat::{
        fighter::{Facing, Fighter, FighterBodyMetrics, FighterBodyParts, PlayerSlot},
        move_data::{MoveInputKind, MoveSpec, move_spec_for_input},
        projectile::Projectile,
    },
    config::{FLOOR_Y, WINDOW_WIDTH, world_px},
    engine::sprites::{
        SpriteCombatBox, SpriteCombatPoint, SpriteFrame, SpriteFrameCombat, SpriteManifest,
        SpriteManifestError,
    },
    math::rect::Rect,
    scenes::combat_lab::CombatLabMove,
};

mod combat_edit;

use self::combat_edit::{
    FrameCombatBoxDragMode, clear_empty_frame_combat, default_frame_combat_box, edit_combat_box,
};

const DEFAULT_ANCHOR_X: f32 = world_px(480.0);
const DEFAULT_DUMMY_ANCHOR_X: f32 = world_px(680.0);
const ZOOM_MIN: f32 = 0.25;
const ZOOM_MAX: f32 = 4.0;
const ZOOM_STEP: f32 = 0.12;
const MANIFEST_SCALE_MIN: f32 = 0.25;
const MANIFEST_SCALE_MAX: f32 = 2.0;
const MANIFEST_SCALE_STEP: f32 = 0.025;
const FRAME_COMBAT_HANDLE_RADIUS: f32 = world_px(8.0);
const FRAME_COMBAT_BOX_MIN_SIZE: i32 = 2;

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
    pub next_character: bool,
    pub previous_character: bool,
    pub next_move: bool,
    pub previous_move: bool,
    pub sync_clip_to_move: bool,
    pub next_frame: bool,
    pub previous_frame: bool,
    pub toggle_playback: bool,
    pub toggle_grid: bool,
    pub toggle_pivot: bool,
    pub toggle_bounds: bool,
    pub toggle_dummy: bool,
    pub toggle_combat_overlay: bool,
    pub toggle_projectile_trajectory: bool,
    pub reload_manifest: bool,
    pub save_manifest: bool,
    pub seed_frame_combat: bool,
    pub add_frame_hurtbox: bool,
    pub add_frame_hitbox: bool,
    pub delete_frame_combat: bool,
    pub increase_manifest_scale: bool,
    pub decrease_manifest_scale: bool,
    pub reset_zoom: bool,
    pub screenshot_requested: bool,
    pub zoom_delta: f32,
    pub nudge_pivot_x: i32,
    pub nudge_pivot_y: i32,
    pub nudge_body_width: i32,
    pub nudge_standing_height: i32,
    pub nudge_crouch_height: i32,
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

    /// Returns the right edge.
    pub fn right(self) -> f32 {
        self.x + self.width
    }

    /// Returns the bottom edge.
    pub fn bottom(self) -> f32 {
        self.y + self.height
    }

    /// Returns the center point.
    pub fn center(self) -> ViewerPoint {
        ViewerPoint::new(self.x + self.width * 0.5, self.y + self.height * 0.5)
    }
}

/// Standalone state for inspecting a sprite manifest and atlas.
#[derive(Debug)]
pub struct SpriteViewer {
    options: SpriteViewerOptions,
    manifest: SpriteManifest,
    body_metrics: CharacterBodyMetricsCatalog,
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
    show_projectile_trajectory: bool,
    zoom: f32,
    anchor: ViewerPoint,
    dummy_anchor: ViewerPoint,
    mouse_position: ViewerPoint,
    dragging: Option<DragTarget>,
    drag_offset: ViewerPoint,
    texture_error: Option<String>,
    status_message: Option<String>,
    manifest_dirty: bool,
    body_metrics_dirty: bool,
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

/// Predicted projectile path projected into viewer screen coordinates.
#[derive(Clone, Debug, PartialEq)]
pub struct SpriteProjectileTrajectory {
    pub origin: ViewerPoint,
    pub end: ViewerPoint,
    pub samples: Vec<Rect>,
    pub travel_distance: f32,
}

/// Mouse position converted into current frame-local sprite coordinates.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SpriteFrameCursor {
    pub screen_position: ViewerPoint,
    pub local_x: i32,
    pub local_y: i32,
    pub atlas_x: i32,
    pub atlas_y: i32,
}

/// One frame-local combat box projected into viewer screen coordinates.
#[derive(Clone, Debug, PartialEq)]
pub struct SpriteFrameCombatBoxOverlay {
    pub kind: SpriteFrameCombatBoxKind,
    pub index: usize,
    pub rect: ViewerRect,
    pub label: Option<String>,
}

/// Identifies which frame-local combat box collection an overlay belongs to.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpriteFrameCombatBoxKind {
    Hurtbox,
    Hitbox,
}

/// Data-driven combat metadata projected from the current sprite frame.
#[derive(Clone, Debug, PartialEq)]
pub struct SpriteFrameCombatOverlay {
    pub hurtboxes: Vec<SpriteFrameCombatBoxOverlay>,
    pub hitboxes: Vec<SpriteFrameCombatBoxOverlay>,
    pub projectile_origin: Option<ViewerPoint>,
}

/// Approximate combat phase used to color the clip timeline.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpriteTimelinePhase {
    Startup,
    Active,
    Recovery,
}

/// Error returned when the viewer cannot load a manifest.
#[derive(Debug)]
pub enum SpriteViewerError {
    Manifest {
        path: PathBuf,
        source: SpriteManifestError,
    },
    ManifestSave {
        path: PathBuf,
        source: SpriteManifestError,
    },
    BodyMetricsSave {
        path: PathBuf,
        source: CharacterBodyMetricsError,
    },
    UnknownInitialClip {
        clip: String,
        path: PathBuf,
    },
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum DragTarget {
    Main,
    Dummy,
    FrameCombatBox {
        kind: SpriteFrameCombatBoxKind,
        index: usize,
        mode: FrameCombatBoxDragMode,
        local_offset_x: i32,
        local_offset_y: i32,
    },
    ProjectileOrigin,
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
        let body_metrics =
            CharacterBodyMetricsCatalog::load(CHARACTER_BODY_METRICS_PATH).unwrap_or_default();
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

        let show_combat_overlay = options.character.is_some()
            || manifest.frames.iter().any(|frame| frame.combat.is_some());

        Ok(Self {
            options,
            manifest,
            body_metrics,
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
            show_projectile_trajectory: true,
            zoom: 1.0,
            anchor: ViewerPoint::new(DEFAULT_ANCHOR_X, FLOOR_Y),
            dummy_anchor: ViewerPoint::new(DEFAULT_DUMMY_ANCHOR_X, FLOOR_Y),
            mouse_position: ViewerPoint::default(),
            dragging: None,
            drag_offset: ViewerPoint::default(),
            texture_error: None,
            status_message: None,
            manifest_dirty: false,
            body_metrics_dirty: false,
        })
    }

    /// Advances animation and applies viewer controls.
    pub fn update(&mut self, input: SpriteViewerInput, delta_seconds: f32) {
        self.mouse_position = input.mouse_position;

        if input.previous_clip {
            self.step_clip(-1);
        }
        if input.next_clip {
            self.step_clip(1);
        }
        if input.previous_character {
            self.step_character(-1);
        }
        if input.next_character {
            self.step_character(1);
        }
        if input.previous_move {
            self.step_move(-1);
        }
        if input.next_move {
            self.step_move(1);
        }
        if input.sync_clip_to_move {
            self.sync_clip_to_selected_move();
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
        if input.toggle_projectile_trajectory {
            self.show_projectile_trajectory = !self.show_projectile_trajectory;
        }
        if input.decrease_manifest_scale {
            self.adjust_manifest_scale(-MANIFEST_SCALE_STEP);
        }
        if input.increase_manifest_scale {
            self.adjust_manifest_scale(MANIFEST_SCALE_STEP);
        }
        if input.nudge_pivot_x != 0 || input.nudge_pivot_y != 0 {
            self.nudge_current_frame_pivot(input.nudge_pivot_x, input.nudge_pivot_y);
        }
        if input.nudge_body_width != 0
            || input.nudge_standing_height != 0
            || input.nudge_crouch_height != 0
        {
            self.nudge_selected_body_metrics(
                input.nudge_body_width,
                input.nudge_standing_height,
                input.nudge_crouch_height,
            );
        }
        if input.save_manifest
            && let Err(error) = self.save_manifest()
        {
            self.status_message = Some(error.to_string());
        }
        if input.seed_frame_combat {
            self.seed_current_frame_combat_from_runtime();
        }
        if input.add_frame_hurtbox {
            self.add_frame_combat_box(SpriteFrameCombatBoxKind::Hurtbox);
        }
        if input.add_frame_hitbox {
            self.add_frame_combat_box(SpriteFrameCombatBoxKind::Hitbox);
        }
        if input.delete_frame_combat {
            self.delete_frame_combat_at_mouse();
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
        self.manifest_dirty = false;
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

    /// Returns the ordered frame names for the selected clip.
    pub fn current_clip_frame_names(&self) -> &[String] {
        &self.manifest.clips[self.clip_index].frames
    }

    /// Returns the mouse coordinates inside the current atlas frame, if hovered.
    pub fn frame_cursor(&self) -> Option<SpriteFrameCursor> {
        let screen = self.sprite_screen_rect();
        if !screen.contains(self.mouse_position) {
            return None;
        }

        let frame = self.current_frame();
        let scale_x = screen.width / frame.frame.w as f32;
        let scale_y = screen.height / frame.frame.h as f32;
        let local_x = ((self.mouse_position.x - screen.x) / scale_x).floor() as i32;
        let local_y = ((self.mouse_position.y - screen.y) / scale_y).floor() as i32;
        let local_x = local_x.clamp(0, frame.frame.w.saturating_sub(1));
        let local_y = local_y.clamp(0, frame.frame.h.saturating_sub(1));

        Some(SpriteFrameCursor {
            screen_position: self.mouse_position,
            local_x,
            local_y,
            atlas_x: frame.frame.x + local_x,
            atlas_y: frame.frame.y + local_y,
        })
    }

    /// Returns the selected runtime combat character, if any.
    pub const fn selected_character(&self) -> Option<CharacterId> {
        self.options.character
    }

    /// Returns the selected runtime move.
    pub const fn selected_move(&self) -> CombatLabMove {
        self.options.selected_move
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

    /// Returns true when manifest tuning changed and should be saved.
    pub const fn manifest_dirty(&self) -> bool {
        self.manifest_dirty
    }

    /// Returns true when body metrics changed and should be saved.
    pub const fn body_metrics_dirty(&self) -> bool {
        self.body_metrics_dirty
    }

    /// Returns loaded body metrics for the selected combat character.
    pub fn selected_body_metrics(&self) -> Option<FighterBodyMetrics> {
        self.options
            .character
            .map(|character| self.body_metrics.body_metrics_for(character))
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

    /// Returns whether projectile trajectory preview is enabled.
    pub const fn show_projectile_trajectory(&self) -> bool {
        self.show_projectile_trajectory
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
        let mut fighter = Fighter::new_with_projectile_loadout_and_body_metrics(
            PlayerSlot::One,
            spec.fighter_name,
            spec.stats.max_health,
            spec.move_ids,
            spec.projectile,
            self.body_metrics.body_metrics_for(character),
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
            fighter.position.y -= world_px(92.0);
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

    /// Returns a simple predicted projectile path for the selected character.
    pub fn projectile_trajectory(&self) -> Option<SpriteProjectileTrajectory> {
        if !self.show_combat_overlay
            || !self.show_projectile_trajectory
            || self.options.selected_move != CombatLabMove::Projectile
        {
            return None;
        }

        let character = self.options.character?;
        let spec = character_spec(character);
        let mut fighter = Fighter::new_with_projectile_loadout_and_body_metrics(
            PlayerSlot::One,
            spec.fighter_name,
            spec.stats.max_health,
            spec.move_ids,
            spec.projectile,
            self.body_metrics.body_metrics_for(character),
            0.0,
        );
        fighter.facing = Facing::Right;
        align_fighter_to_anchor(&mut fighter, self.anchor);

        let projectile = Projectile::from_fighter_with_spec(&fighter, spec.projectile);
        let start = projectile.rect();
        let travel_distance = spec
            .projectile
            .max_travel
            .unwrap_or_else(|| (WINDOW_WIDTH as f32 - start.right() - world_px(32.0)).max(0.0));
        let sample_count = 7;
        let samples = (0..sample_count)
            .map(|index| {
                let t = index as f32 / (sample_count - 1) as f32;
                Rect::new(
                    start.x + travel_distance * t,
                    start.y,
                    start.width,
                    start.height,
                )
            })
            .collect::<Vec<_>>();
        let origin = ViewerPoint::new(start.x, start.y + start.height * 0.5);
        let end = ViewerPoint::new(
            start.x + travel_distance + start.width,
            start.y + start.height * 0.5,
        );

        Some(SpriteProjectileTrajectory {
            origin,
            end,
            samples,
            travel_distance,
        })
    }

    /// Returns data-driven frame combat metadata aligned to the current sprite.
    pub fn frame_combat_overlay(&self) -> Option<SpriteFrameCombatOverlay> {
        if !self.show_combat_overlay {
            return None;
        }

        let frame = self.current_frame();
        let combat = frame.combat.as_ref()?;
        let screen = self.sprite_screen_rect();

        Some(SpriteFrameCombatOverlay {
            hurtboxes: combat
                .hurtboxes
                .iter()
                .enumerate()
                .map(|(index, combat_box)| {
                    self.project_frame_combat_box(
                        screen,
                        frame,
                        SpriteFrameCombatBoxKind::Hurtbox,
                        index,
                        combat_box,
                    )
                })
                .collect(),
            hitboxes: combat
                .hitboxes
                .iter()
                .enumerate()
                .map(|(index, combat_box)| {
                    self.project_frame_combat_box(
                        screen,
                        frame,
                        SpriteFrameCombatBoxKind::Hitbox,
                        index,
                        combat_box,
                    )
                })
                .collect(),
            projectile_origin: combat.projectile_origin.map(|origin| {
                let scale_x = screen.width / frame.frame.w as f32;
                let scale_y = screen.height / frame.frame.h as f32;
                ViewerPoint::new(
                    screen.x + origin.x as f32 * scale_x,
                    screen.y + origin.y as f32 * scale_y,
                )
            }),
        })
    }

    /// Returns the approximate combat phase for a visual frame index.
    pub fn timeline_phase_for_frame_index(&self, index: usize) -> Option<SpriteTimelinePhase> {
        let frame_number = index as u16;
        let character = self.options.character?;
        let spec = character_spec(character);

        if self.options.selected_move == CombatLabMove::Projectile {
            let frame_data = spec.projectile.frame_data;
            if frame_number < frame_data.spawn_frame.get() {
                return Some(SpriteTimelinePhase::Startup);
            }
            if frame_number == frame_data.spawn_frame.get() {
                return Some(SpriteTimelinePhase::Active);
            }
            if frame_number <= frame_data.visual_duration.get() {
                return Some(SpriteTimelinePhase::Recovery);
            }
            return None;
        }

        let input_kind = input_kind_for_move(self.options.selected_move)?;
        let move_spec = move_spec_for_input(spec.move_ids, input_kind)?;
        let frames = move_spec.frames;
        if frame_number < frames.active_start.get() {
            Some(SpriteTimelinePhase::Startup)
        } else if frame_number <= frames.active_end.get() {
            Some(SpriteTimelinePhase::Active)
        } else if frame_number <= frames.duration.get() {
            Some(SpriteTimelinePhase::Recovery)
        } else {
            None
        }
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

    fn project_frame_combat_box(
        &self,
        screen: ViewerRect,
        frame: &SpriteFrame,
        kind: SpriteFrameCombatBoxKind,
        index: usize,
        combat_box: &SpriteCombatBox,
    ) -> SpriteFrameCombatBoxOverlay {
        let scale_x = screen.width / frame.frame.w as f32;
        let scale_y = screen.height / frame.frame.h as f32;
        SpriteFrameCombatBoxOverlay {
            kind,
            index,
            rect: ViewerRect {
                x: screen.x + combat_box.x as f32 * scale_x,
                y: screen.y + combat_box.y as f32 * scale_y,
                width: combat_box.w as f32 * scale_x,
                height: combat_box.h as f32 * scale_y,
            },
            label: combat_box.label.clone(),
        }
    }

    fn update_drag(&mut self, input: SpriteViewerInput) {
        if input.mouse_pressed {
            let target = self
                .frame_combat_drag_target(input.mouse_position)
                .or_else(|| {
                    if self.show_dummy && self.dummy_screen_rect().contains(input.mouse_position) {
                        Some(DragTarget::Dummy)
                    } else if self.sprite_screen_rect().contains(input.mouse_position) {
                        Some(DragTarget::Main)
                    } else {
                        None
                    }
                });

            if let Some(target) = target {
                self.dragging = Some(target);
                if matches!(target, DragTarget::Main | DragTarget::Dummy) {
                    let anchor = self.anchor_for_target(target);
                    self.drag_offset = ViewerPoint::new(
                        input.mouse_position.x - anchor.x,
                        input.mouse_position.y - anchor.y,
                    );
                }
            }
        }

        if let Some(target) = self.dragging
            && input.mouse_down
        {
            match target {
                DragTarget::Main | DragTarget::Dummy => {
                    let anchor = ViewerPoint::new(
                        input.mouse_position.x - self.drag_offset.x,
                        input.mouse_position.y - self.drag_offset.y,
                    );
                    self.set_anchor_for_target(target, anchor);
                }
                DragTarget::FrameCombatBox {
                    kind,
                    index,
                    mode,
                    local_offset_x,
                    local_offset_y,
                } => {
                    self.drag_frame_combat_box(
                        kind,
                        index,
                        mode,
                        local_offset_x,
                        local_offset_y,
                        input.mouse_position,
                    );
                }
                DragTarget::ProjectileOrigin => {
                    self.drag_projectile_origin(input.mouse_position);
                }
            }
        }

        if input.mouse_released {
            self.dragging = None;
        }
    }

    fn anchor_for_target(&self, target: DragTarget) -> ViewerPoint {
        match target {
            DragTarget::Main => self.anchor,
            DragTarget::Dummy => self.dummy_anchor,
            DragTarget::FrameCombatBox { .. } | DragTarget::ProjectileOrigin => self.anchor,
        }
    }

    fn set_anchor_for_target(&mut self, target: DragTarget, anchor: ViewerPoint) {
        match target {
            DragTarget::Main => self.anchor = anchor,
            DragTarget::Dummy => self.dummy_anchor = anchor,
            DragTarget::FrameCombatBox { .. } | DragTarget::ProjectileOrigin => {}
        }
    }

    fn frame_combat_drag_target(&self, point: ViewerPoint) -> Option<DragTarget> {
        if !self.show_combat_overlay {
            return None;
        }

        let overlay = self.frame_combat_overlay()?;
        if let Some(origin) = overlay.projectile_origin
            && point_distance(point, origin) <= FRAME_COMBAT_HANDLE_RADIUS
        {
            return Some(DragTarget::ProjectileOrigin);
        }

        overlay
            .hitboxes
            .iter()
            .rev()
            .chain(overlay.hurtboxes.iter().rev())
            .find_map(|overlay_box| self.drag_target_for_frame_box(point, overlay_box))
    }

    fn drag_target_for_frame_box(
        &self,
        point: ViewerPoint,
        overlay_box: &SpriteFrameCombatBoxOverlay,
    ) -> Option<DragTarget> {
        let mode = frame_box_drag_mode(point, overlay_box.rect)?;
        let local = self.screen_to_frame_local_clamped(point);
        let combat_box = self.current_frame_combat_box(overlay_box.kind, overlay_box.index)?;

        Some(DragTarget::FrameCombatBox {
            kind: overlay_box.kind,
            index: overlay_box.index,
            mode,
            local_offset_x: local.x - combat_box.x,
            local_offset_y: local.y - combat_box.y,
        })
    }

    fn current_frame_combat_box(
        &self,
        kind: SpriteFrameCombatBoxKind,
        index: usize,
    ) -> Option<&SpriteCombatBox> {
        let combat = self.current_frame().combat.as_ref()?;
        match kind {
            SpriteFrameCombatBoxKind::Hurtbox => combat.hurtboxes.get(index),
            SpriteFrameCombatBoxKind::Hitbox => combat.hitboxes.get(index),
        }
    }

    fn drag_frame_combat_box(
        &mut self,
        kind: SpriteFrameCombatBoxKind,
        index: usize,
        mode: FrameCombatBoxDragMode,
        local_offset_x: i32,
        local_offset_y: i32,
        point: ViewerPoint,
    ) {
        let local = self.screen_to_frame_local_clamped(point);
        let frame_name = self.manifest.clips[self.clip_index].frames[self.frame_index].clone();
        let Some(frame) = self.manifest.frame_named_mut(&frame_name) else {
            return;
        };
        let frame_width = frame.frame.w;
        let frame_height = frame.frame.h;
        let combat = frame.combat.get_or_insert_with(SpriteFrameCombat::default);
        let combat_boxes = match kind {
            SpriteFrameCombatBoxKind::Hurtbox => &mut combat.hurtboxes,
            SpriteFrameCombatBoxKind::Hitbox => &mut combat.hitboxes,
        };
        let Some(combat_box) = combat_boxes.get_mut(index) else {
            return;
        };

        edit_combat_box(
            combat_box,
            mode,
            local,
            local_offset_x,
            local_offset_y,
            frame_width,
            frame_height,
        );
        self.manifest_dirty = true;
        self.status_message = Some(format!(
            "{kind:?} #{index}: {},{} {}x{}. Ctrl+S salva.",
            combat_box.x, combat_box.y, combat_box.w, combat_box.h
        ));
    }

    fn drag_projectile_origin(&mut self, point: ViewerPoint) {
        let local = self.screen_to_frame_local_clamped(point);
        let frame_name = self.manifest.clips[self.clip_index].frames[self.frame_index].clone();
        let Some(frame) = self.manifest.frame_named_mut(&frame_name) else {
            return;
        };
        let combat = frame.combat.get_or_insert_with(SpriteFrameCombat::default);
        combat.projectile_origin = Some(local);
        self.manifest_dirty = true;
        self.status_message = Some(format!(
            "Origem de projectile: {},{}. Ctrl+S salva.",
            local.x, local.y
        ));
    }

    fn screen_to_frame_local_clamped(&self, point: ViewerPoint) -> SpriteCombatPoint {
        let screen = self.sprite_screen_rect();
        let frame = self.current_frame();
        let scale_x = frame.frame.w as f32 / screen.width;
        let scale_y = frame.frame.h as f32 / screen.height;
        SpriteCombatPoint {
            x: ((point.x - screen.x) * scale_x).round() as i32,
            y: ((point.y - screen.y) * scale_y).round() as i32,
        }
        .clamped_to_frame(frame)
    }

    fn seed_current_frame_combat_from_runtime(&mut self) {
        self.show_combat_overlay = true;
        let Some(overlay) = self.combat_overlay() else {
            self.status_message =
                Some("Selecione --character para gerar metadata de combate.".to_string());
            return;
        };

        let hurtbox_labels = ["head", "torso", "legs"];
        let hurtboxes = overlay
            .hurtboxes
            .rects()
            .into_iter()
            .zip(hurtbox_labels)
            .filter_map(|(rect, label)| self.screen_rect_to_frame_combat_box(rect, Some(label)))
            .collect::<Vec<_>>();

        let hitboxes = overlay
            .hitbox
            .and_then(|hitbox| self.screen_rect_to_frame_combat_box(hitbox, Some("strike")))
            .into_iter()
            .collect::<Vec<_>>();

        let projectile_origin = overlay
            .projectile_origin
            .map(|origin| self.screen_to_frame_local_clamped(origin));

        if hurtboxes.is_empty() && hitboxes.is_empty() && projectile_origin.is_none() {
            self.status_message =
                Some("Overlay runtime nao cruza o frame atual; nada gerado.".to_string());
            return;
        }

        let frame_name = self.manifest.clips[self.clip_index].frames[self.frame_index].clone();
        let Some(frame) = self.manifest.frame_named_mut(&frame_name) else {
            return;
        };
        frame.combat = Some(SpriteFrameCombat {
            hurtboxes,
            hitboxes,
            projectile_origin,
        });
        self.manifest_dirty = true;
        self.status_message = Some(format!(
            "Metadata de combate gerada para {}. Arraste as boxes e use Ctrl+S.",
            frame.name
        ));
    }

    fn add_frame_combat_box(&mut self, kind: SpriteFrameCombatBoxKind) {
        self.show_combat_overlay = true;
        let local = self.frame_cursor().map_or_else(
            || {
                let frame = self.current_frame();
                SpriteCombatPoint {
                    x: frame.frame.w / 2,
                    y: frame.frame.h / 2,
                }
            },
            |cursor| SpriteCombatPoint {
                x: cursor.local_x,
                y: cursor.local_y,
            },
        );
        let frame_name = self.manifest.clips[self.clip_index].frames[self.frame_index].clone();
        let Some(frame) = self.manifest.frame_named_mut(&frame_name) else {
            return;
        };
        let combat_box = default_frame_combat_box(frame, local, kind);
        let combat = frame.combat.get_or_insert_with(SpriteFrameCombat::default);
        let (label, index) = match kind {
            SpriteFrameCombatBoxKind::Hurtbox => {
                combat.hurtboxes.push(combat_box);
                ("hurtbox", combat.hurtboxes.len() - 1)
            }
            SpriteFrameCombatBoxKind::Hitbox => {
                combat.hitboxes.push(combat_box);
                ("hitbox", combat.hitboxes.len() - 1)
            }
        };

        self.manifest_dirty = true;
        self.status_message = Some(format!(
            "{} #{} adicionada em {}. Ctrl+S salva.",
            label, index, frame.name
        ));
    }

    fn delete_frame_combat_at_mouse(&mut self) {
        let target = self.frame_combat_drag_target(self.mouse_position);
        let removed = match target {
            Some(DragTarget::FrameCombatBox { kind, index, .. }) => {
                self.remove_frame_combat_box(kind, index)
            }
            Some(DragTarget::ProjectileOrigin) => self.clear_frame_projectile_origin(),
            _ => self.remove_last_frame_combat_entry(),
        };

        if !removed {
            self.status_message =
                Some("Nenhuma metadata de combate para remover neste frame.".to_string());
        }
    }

    fn remove_last_frame_combat_entry(&mut self) -> bool {
        let frame_name = self.manifest.clips[self.clip_index].frames[self.frame_index].clone();
        let Some(frame) = self.manifest.frame_named_mut(&frame_name) else {
            return false;
        };
        let Some(combat) = frame.combat.as_mut() else {
            return false;
        };

        let removed = if combat.hitboxes.pop().is_some() {
            Some("ultima hitbox")
        } else if combat.hurtboxes.pop().is_some() {
            Some("ultima hurtbox")
        } else if combat.projectile_origin.take().is_some() {
            Some("origem de projectile")
        } else {
            None
        };

        let Some(label) = removed else {
            return false;
        };
        clear_empty_frame_combat(frame);
        self.manifest_dirty = true;
        self.status_message = Some(format!("Removida {label} de {}. Ctrl+S salva.", frame.name));
        true
    }

    fn remove_frame_combat_box(&mut self, kind: SpriteFrameCombatBoxKind, index: usize) -> bool {
        let frame_name = self.manifest.clips[self.clip_index].frames[self.frame_index].clone();
        let Some(frame) = self.manifest.frame_named_mut(&frame_name) else {
            return false;
        };
        let Some(combat) = frame.combat.as_mut() else {
            return false;
        };
        let boxes = match kind {
            SpriteFrameCombatBoxKind::Hurtbox => &mut combat.hurtboxes,
            SpriteFrameCombatBoxKind::Hitbox => &mut combat.hitboxes,
        };
        if index >= boxes.len() {
            return false;
        }

        boxes.remove(index);
        clear_empty_frame_combat(frame);
        self.manifest_dirty = true;
        self.status_message = Some(format!(
            "{kind:?} #{index} removida de {}. Ctrl+S salva.",
            frame.name
        ));
        true
    }

    fn clear_frame_projectile_origin(&mut self) -> bool {
        let frame_name = self.manifest.clips[self.clip_index].frames[self.frame_index].clone();
        let Some(frame) = self.manifest.frame_named_mut(&frame_name) else {
            return false;
        };
        let Some(combat) = frame.combat.as_mut() else {
            return false;
        };
        if combat.projectile_origin.take().is_none() {
            return false;
        }

        clear_empty_frame_combat(frame);
        self.manifest_dirty = true;
        self.status_message = Some(format!(
            "Origem de projectile removida de {}. Ctrl+S salva.",
            frame.name
        ));
        true
    }

    fn screen_rect_to_frame_combat_box(
        &self,
        rect: Rect,
        label: Option<&str>,
    ) -> Option<SpriteCombatBox> {
        let screen = self.sprite_screen_rect();
        let frame = self.current_frame();
        let scale_x = frame.frame.w as f32 / screen.width;
        let scale_y = frame.frame.h as f32 / screen.height;
        let x0 = ((rect.x - screen.x) * scale_x).floor() as i32;
        let y0 = ((rect.y - screen.y) * scale_y).floor() as i32;
        let x1 = ((rect.right() - screen.x) * scale_x).ceil() as i32;
        let y1 = ((rect.bottom() - screen.y) * scale_y).ceil() as i32;
        let x0 = x0.clamp(0, frame.frame.w);
        let y0 = y0.clamp(0, frame.frame.h);
        let x1 = x1.clamp(0, frame.frame.w);
        let y1 = y1.clamp(0, frame.frame.h);
        let width = x1 - x0;
        let height = y1 - y0;

        if width < FRAME_COMBAT_BOX_MIN_SIZE || height < FRAME_COMBAT_BOX_MIN_SIZE {
            return None;
        }

        Some(SpriteCombatBox {
            x: x0,
            y: y0,
            w: width,
            h: height,
            label: label.map(ToString::to_string),
        })
    }

    fn adjust_manifest_scale(&mut self, delta: f32) {
        let next = quantize_scale(
            (self.manifest_scale() + delta).clamp(MANIFEST_SCALE_MIN, MANIFEST_SCALE_MAX),
        );
        self.manifest.scale = Some(next);
        self.manifest_dirty = true;
        self.status_message = Some(format!("Escala do manifesto: {next:.3}. Ctrl+S salva."));
    }

    fn nudge_current_frame_pivot(&mut self, delta_x: i32, delta_y: i32) {
        let frame_name = self.manifest.clips[self.clip_index].frames[self.frame_index].clone();
        let Some(frame) = self.manifest.frame_named_mut(&frame_name) else {
            return;
        };

        frame.pivot.x = (frame.pivot.x + delta_x).clamp(0, frame.frame.w);
        frame.pivot.y = (frame.pivot.y + delta_y).clamp(0, frame.frame.h);
        self.manifest_dirty = true;
        self.status_message = Some(format!(
            "Pivot de {}: {},{}. Ctrl+S salva.",
            frame.name, frame.pivot.x, frame.pivot.y
        ));
    }

    fn nudge_selected_body_metrics(
        &mut self,
        delta_width: i32,
        delta_standing_height: i32,
        delta_crouch_height: i32,
    ) {
        let Some(character) = self.options.character else {
            self.status_message = Some("Selecione um personagem para ajustar o corpo.".to_string());
            return;
        };

        let current = self.body_metrics.body_metrics_for(character);
        let next = FighterBodyMetrics {
            width: current.width + delta_width as f32,
            standing_height: current.standing_height + delta_standing_height as f32,
            crouch_height: current.crouch_height + delta_crouch_height as f32,
        }
        .sanitized();
        self.body_metrics.set_body_metrics_for(character, next);
        self.body_metrics_dirty = true;
        self.status_message = Some(format!(
            "Corpo {:?}: w {:.0}, h {:.0}, crouch {:.0}. Ctrl+S salva.",
            character, next.width, next.standing_height, next.crouch_height
        ));
    }

    fn save_manifest(&mut self) -> Result<(), SpriteViewerError> {
        if self.manifest_dirty {
            self.manifest
                .save(&self.options.manifest_path)
                .map_err(|source| SpriteViewerError::ManifestSave {
                    path: self.options.manifest_path.clone(),
                    source,
                })?;
        }
        if self.body_metrics_dirty {
            self.body_metrics
                .save(CHARACTER_BODY_METRICS_PATH)
                .map_err(|source| SpriteViewerError::BodyMetricsSave {
                    path: PathBuf::from(CHARACTER_BODY_METRICS_PATH),
                    source,
                })?;
        }
        self.manifest_dirty = false;
        self.body_metrics_dirty = false;
        self.status_message = Some("Arquivos de tuning salvos.".to_string());
        Ok(())
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

    fn step_character(&mut self, direction: i32) {
        let next = match (self.options.character, direction) {
            (Some(character), value) if value < 0 => character.previous(),
            (Some(character), _) => character.next(),
            (None, value) if value < 0 => CharacterId::default().previous(),
            (None, _) => CharacterId::default(),
        };
        self.options.character = Some(next);
        self.show_combat_overlay = true;
        self.status_message = Some(format!(
            "Personagem de combate: {}.",
            character_spec(next).display_name
        ));
    }

    fn step_move(&mut self, direction: i32) {
        let current = CombatLabMove::ALL
            .iter()
            .position(|candidate| *candidate == self.options.selected_move)
            .unwrap_or(0);
        self.options.selected_move =
            CombatLabMove::ALL[wrap_index(current, CombatLabMove::ALL.len(), direction)];
        self.status_message = Some(format!(
            "Golpe selecionado: {}.",
            self.options.selected_move.label()
        ));
    }

    fn sync_clip_to_selected_move(&mut self) {
        let candidates = preferred_clips_for_move(self.options.selected_move);
        let Some(index) = candidates.iter().find_map(|candidate| {
            self.manifest
                .clips
                .iter()
                .position(|clip| clip.name == *candidate)
        }) else {
            self.status_message = Some(format!(
                "Nenhum clip conhecido para {} neste manifesto.",
                self.options.selected_move.label()
            ));
            return;
        };

        self.clip_index = index;
        self.frame_index = 0;
        self.frame_elapsed_ms = 0.0;
        self.playing = true;
        self.status_message = Some(format!(
            "Clip sincronizado: {}.",
            self.manifest.clips[index].name
        ));
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
            Self::ManifestSave { path, source } => {
                write!(
                    formatter,
                    "could not save sprite manifest {}: {source}",
                    path.display()
                )
            }
            Self::BodyMetricsSave { path, source } => {
                write!(
                    formatter,
                    "could not save character body metrics {}: {source}",
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
            Self::ManifestSave { source, .. } => Some(source),
            Self::BodyMetricsSave { source, .. } => Some(source),
            Self::UnknownInitialClip { .. } => None,
        }
    }
}

impl SpriteCombatPoint {
    fn clamped_to_frame(self, frame: &SpriteFrame) -> Self {
        Self {
            x: self.x.clamp(0, frame.frame.w),
            y: self.y.clamp(0, frame.frame.h),
        }
    }
}

fn wrap_index(current: usize, len: usize, direction: i32) -> usize {
    debug_assert!(len > 0);
    let len = len as i32;
    (current as i32 + direction).rem_euclid(len) as usize
}

fn quantize_scale(value: f32) -> f32 {
    (value * 1000.0).round() / 1000.0
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

fn frame_box_drag_mode(point: ViewerPoint, rect: ViewerRect) -> Option<FrameCombatBoxDragMode> {
    if point_near(
        point,
        ViewerPoint::new(rect.x, rect.y),
        FRAME_COMBAT_HANDLE_RADIUS,
    ) {
        return Some(FrameCombatBoxDragMode::TopLeft);
    }
    if point_near(
        point,
        ViewerPoint::new(rect.right(), rect.y),
        FRAME_COMBAT_HANDLE_RADIUS,
    ) {
        return Some(FrameCombatBoxDragMode::TopRight);
    }
    if point_near(
        point,
        ViewerPoint::new(rect.x, rect.bottom()),
        FRAME_COMBAT_HANDLE_RADIUS,
    ) {
        return Some(FrameCombatBoxDragMode::BottomLeft);
    }
    if point_near(
        point,
        ViewerPoint::new(rect.right(), rect.bottom()),
        FRAME_COMBAT_HANDLE_RADIUS,
    ) {
        return Some(FrameCombatBoxDragMode::BottomRight);
    }
    if rect.contains(point) {
        return Some(FrameCombatBoxDragMode::Move);
    }

    None
}

fn point_near(point: ViewerPoint, target: ViewerPoint, radius: f32) -> bool {
    point_distance(point, target) <= radius
}

fn point_distance(a: ViewerPoint, b: ViewerPoint) -> f32 {
    let dx = a.x - b.x;
    let dy = a.y - b.y;
    (dx * dx + dy * dy).sqrt()
}

fn preferred_clips_for_move(selected_move: CombatLabMove) -> &'static [&'static str] {
    match selected_move {
        CombatLabMove::LightPunch => &["punch_light", "light_punch", "punch"],
        CombatLabMove::HeavyPunch => &["punch_heavy", "heavy_punch", "punch"],
        CombatLabMove::Kick => &["kick"],
        CombatLabMove::Sweep => &["sweep", "sweep_kick", "crouch", "kick"],
        CombatLabMove::Overhead => &["overhead", "punch_heavy"],
        CombatLabMove::AntiAir => &["anti_air", "rising_anti_air", "punch_heavy"],
        CombatLabMove::AirPunch => &["air_punch", "jump_punch", "jump", "punch_light"],
        CombatLabMove::AirKick => &["air_kick", "jump_kick", "jump", "kick"],
        CombatLabMove::Throw => &["throw", "grab", "punch_light"],
        CombatLabMove::Projectile => &["special", "projectile"],
    }
}
