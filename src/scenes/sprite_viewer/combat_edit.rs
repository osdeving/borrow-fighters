//! Edits frame-local combat metadata for the sprite viewer.
//!
//! System: Sprite combat viewer. This module keeps pure box-editing helpers
//! away from the larger viewer state machine.

use crate::engine::sprites::{SpriteCombatBox, SpriteCombatPoint, SpriteFrame, SpriteFrameCombat};

use super::SpriteFrameCombatBoxKind;

const FRAME_COMBAT_BOX_MIN_SIZE: i32 = 2;
const DEFAULT_FRAME_HURTBOX_WIDTH: i32 = 48;
const DEFAULT_FRAME_HURTBOX_HEIGHT: i32 = 96;
const DEFAULT_FRAME_HITBOX_WIDTH: i32 = 44;
const DEFAULT_FRAME_HITBOX_HEIGHT: i32 = 30;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FrameCombatBoxDragMode {
    Move,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

pub fn edit_combat_box(
    combat_box: &mut SpriteCombatBox,
    mode: FrameCombatBoxDragMode,
    local: SpriteCombatPoint,
    local_offset_x: i32,
    local_offset_y: i32,
    frame_width: i32,
    frame_height: i32,
) {
    let old_right = combat_box.x + combat_box.w;
    let old_bottom = combat_box.y + combat_box.h;

    match mode {
        FrameCombatBoxDragMode::Move => {
            combat_box.x =
                (local.x - local_offset_x).clamp(0, frame_width.saturating_sub(combat_box.w));
            combat_box.y =
                (local.y - local_offset_y).clamp(0, frame_height.saturating_sub(combat_box.h));
        }
        FrameCombatBoxDragMode::TopLeft => {
            combat_box.x = local.x.clamp(0, old_right - FRAME_COMBAT_BOX_MIN_SIZE);
            combat_box.y = local.y.clamp(0, old_bottom - FRAME_COMBAT_BOX_MIN_SIZE);
            combat_box.w = old_right - combat_box.x;
            combat_box.h = old_bottom - combat_box.y;
        }
        FrameCombatBoxDragMode::TopRight => {
            let next_right = local
                .x
                .clamp(combat_box.x + FRAME_COMBAT_BOX_MIN_SIZE, frame_width);
            combat_box.y = local.y.clamp(0, old_bottom - FRAME_COMBAT_BOX_MIN_SIZE);
            combat_box.w = next_right - combat_box.x;
            combat_box.h = old_bottom - combat_box.y;
        }
        FrameCombatBoxDragMode::BottomLeft => {
            let next_bottom = local
                .y
                .clamp(combat_box.y + FRAME_COMBAT_BOX_MIN_SIZE, frame_height);
            combat_box.x = local.x.clamp(0, old_right - FRAME_COMBAT_BOX_MIN_SIZE);
            combat_box.w = old_right - combat_box.x;
            combat_box.h = next_bottom - combat_box.y;
        }
        FrameCombatBoxDragMode::BottomRight => {
            let next_right = local
                .x
                .clamp(combat_box.x + FRAME_COMBAT_BOX_MIN_SIZE, frame_width);
            let next_bottom = local
                .y
                .clamp(combat_box.y + FRAME_COMBAT_BOX_MIN_SIZE, frame_height);
            combat_box.w = next_right - combat_box.x;
            combat_box.h = next_bottom - combat_box.y;
        }
    }
}

pub fn default_frame_combat_box(
    frame: &SpriteFrame,
    center: SpriteCombatPoint,
    kind: SpriteFrameCombatBoxKind,
) -> SpriteCombatBox {
    let (width, height, label) = match kind {
        SpriteFrameCombatBoxKind::Hurtbox => (
            DEFAULT_FRAME_HURTBOX_WIDTH,
            DEFAULT_FRAME_HURTBOX_HEIGHT,
            "body",
        ),
        SpriteFrameCombatBoxKind::Hitbox => (
            DEFAULT_FRAME_HITBOX_WIDTH,
            DEFAULT_FRAME_HITBOX_HEIGHT,
            "strike",
        ),
    };
    let width = width.min(frame.frame.w).max(FRAME_COMBAT_BOX_MIN_SIZE);
    let height = height.min(frame.frame.h).max(FRAME_COMBAT_BOX_MIN_SIZE);
    SpriteCombatBox {
        x: (center.x - width / 2).clamp(0, frame.frame.w - width),
        y: (center.y - height / 2).clamp(0, frame.frame.h - height),
        w: width,
        h: height,
        label: Some(label.to_string()),
    }
}

pub fn clear_empty_frame_combat(frame: &mut SpriteFrame) {
    if frame.combat.as_ref().is_some_and(frame_combat_is_empty) {
        frame.combat = None;
    }
}

fn frame_combat_is_empty(combat: &SpriteFrameCombat) -> bool {
    combat.hurtboxes.is_empty() && combat.hitboxes.is_empty() && combat.projectile_origin.is_none()
}
