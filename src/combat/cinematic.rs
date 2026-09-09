//! Exposes the clock and identity of a cinematic special without screen geometry.
//!
//! System: Combat presentation data. Rendering may fill the screen from this
//! state; offensive geometry remains solely in the local MoveSpec hitbox.

use super::{
    frame::FrameCount,
    move_data::{MoveId, MoveSpec},
};
use crate::characters::CharacterId;

/// A snapshot derived from the living attack, never an independent damage entity.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CinematicSpecialState {
    pub character: CharacterId,
    pub move_id: MoveId,
    pub label: &'static str,
    pub elapsed_frames: u32,
    pub duration_frames: u32,
    pub active_start: u32,
    pub active_end: u32,
}

impl CinematicSpecialState {
    pub(crate) fn from_move(spec: MoveSpec, elapsed: FrameCount) -> Option<Self> {
        let character = match spec.id {
            MoveId::RustOwnershipEclipse => CharacterId::Rust,
            MoveId::DukeJvmOverdrive => CharacterId::Duke,
            MoveId::GoMillionGoroutines => CharacterId::Go,
            MoveId::CKernelPanic => CharacterId::C,
            MoveId::PythonEventHorizon => CharacterId::Python,
            MoveId::CppTemplateSingularity => CharacterId::Cpp,
            _ => return None,
        };
        Some(Self {
            character,
            move_id: spec.id,
            label: spec.label,
            elapsed_frames: u32::from(elapsed.get()),
            duration_frames: u32::from(spec.frames.duration.get()),
            active_start: u32::from(spec.frames.active_start.get()),
            active_end: u32::from(spec.frames.active_end.get()),
        })
    }

    /// Normalized visual timeline. It advances with simulation, including pause.
    pub fn progress(self) -> f32 {
        (self.elapsed_frames as f32 / self.duration_frames as f32).clamp(0.0, 1.0)
    }
}
