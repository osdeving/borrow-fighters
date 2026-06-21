//! Represents fixed-step combat timing in whole frames.
//!
//! System: Combat data. This module provides the frame unit used by move specs,
//! projectiles, tests, and debug overlays.
//!
//! Fighting-game tuning should speak in frames first, then convert to seconds
//! only at the boundary where the fixed update loop advances timers.

use crate::config::FIXED_TIMESTEP;

const FRAME_EPSILON: f32 = 0.001;

/// Number of fixed simulation frames used by combat timing.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct FrameCount(u16);

impl FrameCount {
    /// Zero elapsed combat frames.
    pub const ZERO: Self = Self(0);

    /// Creates a frame count from a raw integer.
    pub const fn new(frames: u16) -> Self {
        Self(frames)
    }

    /// Returns the raw frame count.
    pub const fn get(self) -> u16 {
        self.0
    }

    /// Converts fixed simulation frames to seconds.
    pub fn as_seconds(self) -> f32 {
        self.0 as f32 * FIXED_TIMESTEP
    }

    /// Converts elapsed seconds to an elapsed frame count.
    ///
    /// `ceil` matches the current update model: once one fixed step has run,
    /// the move is considered to be on frame 1 for debug and phase checks.
    pub fn from_elapsed_seconds(seconds: f32) -> Self {
        if seconds <= 0.0 {
            Self::ZERO
        } else {
            Self(((seconds / FIXED_TIMESTEP) - FRAME_EPSILON).ceil() as u16)
        }
    }
}
