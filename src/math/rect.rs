//! Defines rectangle helpers for collision and debug drawing.
//!
//! Hitboxes, hurtboxes, and fighter bodies use this type so combat logic stays
//! detached from the renderer.

use crate::math::vec2::Vec2;

/// Axis-aligned rectangle in screen coordinates.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rect {
    /// Creates a rectangle.
    pub const fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    /// Returns true when two rectangles overlap.
    pub fn intersects(self, other: Self) -> bool {
        self.x < other.x + other.width
            && self.x + self.width > other.x
            && self.y < other.y + other.height
            && self.y + self.height > other.y
    }

    /// Returns the horizontal center.
    pub fn center_x(self) -> f32 {
        self.x + self.width * 0.5
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
    pub fn center(self) -> Vec2 {
        Vec2::new(self.center_x(), self.y + self.height * 0.5)
    }
}
