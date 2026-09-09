//! Models the five thematic signature effects as explicit combat entities.
//!
//! System: Combat primitives. These shapes and trajectories are shared by real
//! collision, debug drawing and visual effects; there is no meter or script VM.

use crate::{
    combat::{fighter::PlayerSlot, move_data::MoveId},
    config::{FLOOR_Y, world_px},
    math::{rect::Rect, vec2::Vec2},
};

/// Distinct physical manifestations of a fighter's signature attack.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SignatureEffectKind {
    RustFortress,
    DukeCodeSheet,
    CMemoryRupture,
    PythonVortex,
    CppRocket,
    CppExplosion,
}

/// One currently visible effect, including its actual offensive geometry.
#[derive(Clone, Debug)]
pub struct SignatureEffect {
    pub owner: PlayerSlot,
    pub move_id: MoveId,
    pub kind: SignatureEffectKind,
    /// Center for sheets/explosions, ground anchor for columns and fortress.
    pub position: Vec2,
    pub velocity: Vec2,
    pub elapsed_seconds: f32,
    pub lifetime_seconds: f32,
    pub direction: f32,
    pub alive: bool,
    pub(crate) has_hit: bool,
    pub(crate) fresh: bool,
}

impl SignatureEffect {
    /// Returns whether the effect has already delivered its one contact.
    pub fn has_connected(&self) -> bool {
        self.has_hit
    }
    /// Returns the offensive shape; an aimed rocket explodes only at the floor.
    pub fn hitbox(&self) -> Option<Rect> {
        let (width, height, y) = match self.kind {
            SignatureEffectKind::RustFortress => {
                (world_px(130.0), world_px(160.0), FLOOR_Y - world_px(160.0))
            }
            SignatureEffectKind::DukeCodeSheet => (
                world_px(90.0),
                world_px(62.0),
                self.position.y - world_px(31.0),
            ),
            SignatureEffectKind::CMemoryRupture => {
                (world_px(150.0), world_px(180.0), FLOOR_Y - world_px(180.0))
            }
            SignatureEffectKind::PythonVortex => {
                (world_px(150.0), world_px(320.0), FLOOR_Y - world_px(320.0))
            }
            SignatureEffectKind::CppExplosion => {
                (world_px(260.0), world_px(95.0), FLOOR_Y - world_px(95.0))
            }
            SignatureEffectKind::CppRocket => return None,
        };
        Some(Rect::new(self.position.x - width * 0.5, y, width, height))
    }

    /// Whether this effect is a launched object that a fortress can cancel.
    pub fn is_projectile(&self) -> bool {
        matches!(
            self.kind,
            SignatureEffectKind::DukeCodeSheet | SignatureEffectKind::CppRocket
        )
    }

    /// Advances movement and turns the bazooka's floor impact into an explosion.
    pub fn update(&mut self, dt: f32) {
        if self.fresh {
            self.fresh = false;
            return;
        }
        self.elapsed_seconds += dt;
        self.position.x += self.velocity.x * dt;
        self.position.y += self.velocity.y * dt;
        if self.kind == SignatureEffectKind::CppRocket && self.position.y >= FLOOR_Y {
            self.position.y = FLOOR_Y;
            self.kind = SignatureEffectKind::CppExplosion;
            self.velocity = Vec2::ZERO;
            self.elapsed_seconds = 0.0;
            self.lifetime_seconds = 11.0 / 60.0;
        }
        if self.elapsed_seconds >= self.lifetime_seconds {
            self.alive = false;
        }
    }
}

/// Returns the emission frames for the named attack. Java emits three real sheets.
pub fn signature_emission_frames(id: MoveId) -> &'static [u16] {
    match id {
        MoveId::RustBorrowFortress => &[24],
        MoveId::DukePrintlnBarrage => &[26, 36, 46],
        MoveId::CSegmentationFault => &[34],
        MoveId::PythonImportAntigravity => &[26],
        MoveId::CppUndefinedBazooka => &[32],
        _ => &[],
    }
}
