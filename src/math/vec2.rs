//! Defines a minimal 2D vector for gameplay rules.
//!
//! This avoids depending on Raylib types in the combat model.

/// Two-dimensional coordinate or velocity.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };

    /// Creates a new vector.
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}
