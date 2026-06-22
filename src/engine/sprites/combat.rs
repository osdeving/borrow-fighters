//! Projects sprite combat metadata into world-space rectangles.
//!
//! System: Sprite runtime. This module bridges atlas-local sprite metadata to
//! combat/debug coordinates without owning match rules or Raylib resources.

use crate::{
    combat::fighter::{Facing, Fighter},
    engine::sprites::{
        animation::frame_for_clip_at,
        manifest::{SpriteCombatBox, SpriteFrame, SpriteManifest},
        selection::{FighterSpriteClip, fighter_clip_elapsed_seconds, fighter_sprite_clip},
    },
    math::{rect::Rect, vec2::Vec2},
};

const MIN_RUNTIME_FIGHTER_SCALE: f32 = 0.1;

/// Sprite combat metadata projected from one visual frame into world space.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ProjectedSpriteCombat {
    pub frame_name: String,
    pub hurtboxes: Vec<Rect>,
    pub hitboxes: Vec<Rect>,
    pub projectile_origin: Option<Vec2>,
}

impl ProjectedSpriteCombat {
    /// Returns true when this frame carries no combat metadata.
    pub fn is_empty(&self) -> bool {
        self.hurtboxes.is_empty() && self.hitboxes.is_empty() && self.projectile_origin.is_none()
    }
}

/// Returns combat metadata for the fighter's current visual frame.
pub fn projected_fighter_combat(
    manifest: &SpriteManifest,
    fighter: &Fighter,
    world_elapsed_seconds: f32,
) -> Option<ProjectedSpriteCombat> {
    let clip = fighter_sprite_clip(fighter);
    let clip_time = fighter_clip_elapsed_seconds(fighter, world_elapsed_seconds);
    let frame = frame_for_clip_at(manifest, clip.as_str(), clip_time)?;

    project_frame_combat(manifest, frame, fighter).filter(|combat| !combat.is_empty())
}

/// Returns a projected projectile origin from a specific visual clip.
pub fn projected_projectile_origin_for_clip(
    manifest: &SpriteManifest,
    fighter: &Fighter,
    clip: FighterSpriteClip,
    elapsed_seconds: f32,
) -> Option<Vec2> {
    let frame = frame_for_clip_at(manifest, clip.as_str(), elapsed_seconds)?;
    project_frame_combat(manifest, frame, fighter)?.projectile_origin
}

/// Projects a specific sprite frame's optional combat metadata into world space.
pub fn project_frame_combat(
    manifest: &SpriteManifest,
    frame: &SpriteFrame,
    fighter: &Fighter,
) -> Option<ProjectedSpriteCombat> {
    let combat = frame.combat.as_ref()?;
    let placement = SpriteFramePlacement::new(manifest, frame, fighter);
    let hurtboxes = combat
        .hurtboxes
        .iter()
        .map(|combat_box| placement.project_box(combat_box))
        .collect::<Vec<_>>();
    let hitboxes = combat
        .hitboxes
        .iter()
        .map(|combat_box| placement.project_box(combat_box))
        .collect::<Vec<_>>();
    let projectile_origin = combat
        .projectile_origin
        .map(|origin| placement.project_point(origin.x, origin.y));

    Some(ProjectedSpriteCombat {
        frame_name: frame.name.clone(),
        hurtboxes,
        hitboxes,
        projectile_origin,
    })
}

struct SpriteFramePlacement {
    frame_width: f32,
    scale: f32,
    dest_x: f32,
    dest_y: f32,
    facing: Facing,
}

impl SpriteFramePlacement {
    fn new(manifest: &SpriteManifest, frame: &SpriteFrame, fighter: &Fighter) -> Self {
        let scale = manifest.scale.unwrap_or(1.0).max(MIN_RUNTIME_FIGHTER_SCALE);
        let body = fighter.body_rect();
        let anchor = Vec2::new(body.center_x(), body.bottom());
        let frame_width = frame.frame.w as f32;
        let dest_width = frame_width * scale;
        let pivot_x = frame.pivot.x as f32 * scale;
        let pivot_y = frame.pivot.y as f32 * scale;
        let dest_x = match fighter.facing {
            Facing::Left => anchor.x - (dest_width - pivot_x),
            Facing::Right => anchor.x - pivot_x,
        };

        Self {
            frame_width,
            scale,
            dest_x,
            dest_y: anchor.y - pivot_y,
            facing: fighter.facing,
        }
    }

    fn project_box(&self, combat_box: &SpriteCombatBox) -> Rect {
        let local_x = match self.facing {
            Facing::Left => self.frame_width - (combat_box.x + combat_box.w) as f32,
            Facing::Right => combat_box.x as f32,
        };
        Rect::new(
            self.dest_x + local_x * self.scale,
            self.dest_y + combat_box.y as f32 * self.scale,
            combat_box.w as f32 * self.scale,
            combat_box.h as f32 * self.scale,
        )
    }

    fn project_point(&self, local_x: i32, local_y: i32) -> Vec2 {
        let local_x = match self.facing {
            Facing::Left => self.frame_width - local_x as f32,
            Facing::Right => local_x as f32,
        };
        Vec2::new(
            self.dest_x + local_x * self.scale,
            self.dest_y + local_y as f32 * self.scale,
        )
    }
}
