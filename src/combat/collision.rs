//! Resolves hitbox and hurtbox overlap.
//!
//! Collision is intentionally axis-aligned for the prototype so combat feedback
//! can be debugged visually.

use crate::math::rect::Rect;

/// Returns true if the active hitbox overlaps the target hurtbox.
pub fn hitbox_hits_hurtbox(hitbox: Rect, hurtbox: Rect) -> bool {
    hitbox.intersects(hurtbox)
}
