//! Describes the street's opening camera move before handing control to Rust.
//!
//! System: Adventure presentation data. This fixed-clock shot is independent of
//! Raylib, player input and combat; its last transform is the playable view.

use crate::math::vec2::Vec2;

/// Six seconds to linger on the kite, descend to Rust and settle into play.
pub const ARRIVAL_TICKS: u32 = 360;

/// World framing and cinematic border opacity at one fixed update.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ArrivalShot {
    /// Point in the untransformed street view placed at screen center.
    pub target: Vec2,
    /// Uniform magnification of the complete world.
    pub zoom: f32,
    /// Letterbox opacity, removed smoothly before controls are handed over.
    pub matte: f32,
}

impl ArrivalShot {
    /// Exact identity transform of normal play, also used for checkpoint retry.
    pub fn settled() -> Self {
        Self {
            target: Vec2::new(640.0, 360.0),
            zoom: 1.0,
            matte: 0.0,
        }
    }

    /// Samples one continuous shot with zero velocity at each authored join.
    pub fn at(ticks: u32) -> Self {
        if ticks >= ARRIVAL_TICKS {
            return Self::settled();
        }
        if ticks < 60 {
            return Self {
                target: Vec2::new(1108.0, 127.0),
                zoom: 3.8,
                matte: 1.0,
            };
        }
        let (from, to, a, b, start, length) = if ticks < 300 {
            (
                Vec2::new(1108.0, 127.0),
                Vec2::new(570.0, 370.0),
                3.8,
                1.15,
                60,
                240,
            )
        } else {
            (
                Vec2::new(570.0, 370.0),
                Vec2::new(640.0, 360.0),
                1.15,
                1.0,
                300,
                60,
            )
        };
        let t = smooth((ticks - start) as f32 / length as f32);
        Self {
            target: Vec2::new(from.x + (to.x - from.x) * t, from.y + (to.y - from.y) * t),
            zoom: a + (b - a) * t,
            matte: if ticks < 300 { 1.0 } else { 1.0 - t },
        }
    }
}

fn smooth(t: f32) -> f32 {
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shot_stays_inside_the_painted_world_and_hands_off_without_a_jump() {
        let mut previous = ArrivalShot::at(0);
        for tick in 1..=ARRIVAL_TICKS {
            let shot = ArrivalShot::at(tick);
            assert!(shot.target.x - 640.0 / shot.zoom >= -0.001);
            assert!(shot.target.y - 360.0 / shot.zoom >= -0.001);
            assert!(shot.target.x + 640.0 / shot.zoom <= 1728.0);
            assert!(shot.target.y + 360.0 / shot.zoom <= 720.001);
            assert!((shot.target.x - previous.target.x).abs() < 5.0);
            assert!((shot.zoom - previous.zoom).abs() < 0.03);
            previous = shot;
        }
        assert_eq!(previous, ArrivalShot::settled());
        assert_eq!(ArrivalShot::at(u32::MAX), previous);
    }
}
