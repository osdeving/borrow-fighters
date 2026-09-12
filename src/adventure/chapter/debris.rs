//! Simulates independently replaceable cargo pieces and their progressive breakage.
//!
//! System: Adventure chapter. Hit volumes, health, gravity and local effect clocks
//! are pure data; the renderer chooses intact pieces, splinters and dust assets.

use super::world::Region;
use crate::math::rect::Rect;
use serde::Deserialize;

/// One editable piece in the collapsed cargo pile.
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DebrisSpec {
    /// Stable instance identity, independent of the sprite filename.
    pub id: String,
    /// Replaceable intact sprite from the chapter catalog.
    pub piece: String,
    /// Replaceable fragment sprite from the chapter catalog.
    pub fragment: String,
    /// Authoritative initial solid region.
    pub region: Region,
    /// Damage required to destroy this piece.
    pub hp: u32,
    /// Small authored tilt, independent from conservative solid geometry.
    #[serde(default)]
    pub rotation: f32,
}

/// Runtime state for a single destructible piece, including independent effects.
#[derive(Clone, Debug)]
pub struct Debris {
    /// Immutable content identity and initial placement.
    pub spec: DebrisSpec,
    /// Remaining structural health; zero pieces no longer collide.
    pub hp: u32,
    /// Current top, falling when its support is removed.
    pub y: f32,
    /// Vertical speed while settling after a neighbouring piece breaks.
    pub velocity_y: f32,
    /// Tick of its latest genuine contact, retained for local effects.
    pub hit_tick: Option<u32>,
    /// Direction of the hit, used by splinters and dust.
    pub hit_direction: f32,
}

impl Debris {
    /// Creates a clean piece from reviewed scene data.
    pub fn new(spec: DebrisSpec) -> Self {
        Self {
            hp: spec.hp,
            y: spec.region.y,
            spec,
            velocity_y: 0.0,
            hit_tick: None,
            hit_direction: 1.0,
        }
    }

    /// Current collision bounds; destroyed pieces are queried only for effects.
    pub fn rect(&self) -> Rect {
        Rect::new(
            self.spec.region.x,
            self.y,
            self.spec.region.width,
            self.spec.region.height,
        )
    }

    /// Applies a single consumed attack and reports actual structural damage.
    pub fn hit(&mut self, damage: u32, tick: u32, direction: f32) -> u32 {
        let removed = self.hp.min(damage);
        self.hp -= removed;
        self.hit_tick = Some(tick);
        self.hit_direction = direction;
        removed
    }
}

/// Settles unsupported pieces without teleporting them or resetting effect clocks.
pub fn settle(pieces: &mut [Debris], floor: f32) {
    // Lowest first keeps each upper piece supported by the current lower surface.
    let mut order: Vec<usize> = (0..pieces.len()).filter(|&i| pieces[i].hp > 0).collect();
    order.sort_by(|&a, &b| pieces[b].y.total_cmp(&pieces[a].y));
    for index in order {
        let rect = pieces[index].rect();
        let support = pieces
            .iter()
            .enumerate()
            .filter(|(i, p)| {
                *i != index
                    && p.hp > 0
                    && p.y >= rect.bottom() - 0.01
                    && p.spec.region.x < rect.right()
                    && p.spec.region.x + p.spec.region.width > rect.x
            })
            .map(|(_, p)| p.y)
            .fold(floor, f32::min);
        let piece = &mut pieces[index];
        piece.velocity_y += 1300.0 / 60.0;
        piece.y = (piece.y + piece.velocity_y / 60.0).min(support - rect.height);
        if piece.y + rect.height >= support - 0.01 {
            piece.velocity_y = 0.0;
        }
    }
}
