//! Loads and draws reusable street pieces from their validated art catalog.
//!
//! System: Adventure rendering. Texture sources, anchors and wheel landmarks
//! belong to content; callers supply positions and phase clocks only.

use crate::{
    adventure::scenery::{PieceCatalog, StreetLayout},
    runtime_paths::asset_path,
};
use raylib::prelude::*;
use std::{collections::BTreeMap, error::Error};

/// Per-instance transform; replacing the image keeps this pose unchanged.
pub struct PiecePose {
    /// Anchor position in screen coordinates.
    pub position: Vector2,
    /// Local size multiplier.
    pub scale: f32,
    /// Mirrors around the content anchor.
    pub flip: bool,
    /// Degrees around that anchor, used for falling bicycles.
    pub rotation: f32,
    /// Lighting tint.
    pub tint: Color,
}

impl PiecePose {
    /// Grounded default transform at a screen position.
    pub fn at(position: Vector2) -> Self {
        Self {
            position,
            scale: 1.0,
            flip: false,
            rotation: 0.0,
            tint: Color::WHITE,
        }
    }
}

/// Textures are cached once per path, independently of how many props use them.
pub struct StreetPieces {
    /// Pure art geometry and animation descriptions.
    pub catalog: PieceCatalog,
    /// Independently editable prop placements.
    pub layout: StreetLayout,
    textures: BTreeMap<String, Texture2D>,
}

impl StreetPieces {
    /// Validates every reference and image extent before accepting the catalog.
    pub fn load(rl: &mut RaylibHandle, thread: &RaylibThread) -> Result<Self, Box<dyn Error>> {
        let catalog = PieceCatalog::load(&asset_path("assets/adventure/street/catalog.json"))?;
        let layout =
            StreetLayout::load(&asset_path("assets/adventure/street/scene.json"), &catalog)?;
        for id in [
            "kid.play",
            "kid.startled",
            "kid.release",
            "kid.run",
            "cyclist.ride",
            "cyclist.brake",
            "cyclist.dismount",
            "cyclist.run",
            "bike.upright",
            "bike.fallen",
            "vehicle.hatch",
            "vehicle.sedan",
            "vehicle.pickup",
            "vehicle.suv",
            "vehicle.bus",
            "incident.intact",
            "incident.crashed",
        ] {
            if !catalog.pieces.contains_key(id) {
                return Err(format!("missing street piece: {id}").into());
            }
        }
        let mut textures = BTreeMap::new();
        for piece in catalog.pieces.values() {
            for frame in &piece.frames {
                if !textures.contains_key(&frame.image) {
                    let image = rl.load_texture(
                        thread,
                        &asset_path(format!("assets/adventure/{}", frame.image)).to_string_lossy(),
                    )?;
                    image.set_texture_filter(thread, TextureFilter::TEXTURE_FILTER_BILINEAR);
                    textures.insert(frame.image.clone(), image);
                }
                let image = &textures[&frame.image];
                if frame.source[0] + frame.source[2] > image.width as f32
                    || frame.source[1] + frame.source[3] > image.height as f32
                {
                    return Err(format!("street frame extends beyond image {}", frame.image).into());
                }
            }
        }
        Ok(Self {
            catalog,
            layout,
            textures,
        })
    }

    /// Draws an authored frame, keeping later poses at the first frame's scale.
    pub fn draw(&self, d: &mut impl RaylibDraw, id: &str, ticks: u32, pose: &PiecePose) {
        let (frame, scale) = self.catalog.pieces[id].sample(ticks);
        let scale = scale * pose.scale;
        let [x, y, w, h] = frame.source;
        let anchor_x = if pose.flip {
            w - frame.anchor[0]
        } else {
            frame.anchor[0]
        };
        d.draw_texture_pro(
            &self.textures[&frame.image],
            Rectangle::new(x, y, if pose.flip { -w } else { w }, h),
            Rectangle::new(pose.position.x, pose.position.y, w * scale, h * scale),
            Vector2::new(anchor_x * scale, frame.anchor[1] * scale),
            pose.rotation,
            pose.tint,
        );
    }

    /// Rotating highlights use each authored frame's own wheel landmarks.
    pub fn wheels(&self, d: &mut impl RaylibDraw, id: &str, ticks: u32, pose: &PiecePose) {
        let (frame, scale) = self.catalog.pieces[id].sample(ticks);
        let scale = scale * pose.scale;
        let direction = if pose.flip { -1.0 } else { 1.0 };
        let angle = ticks as f32 * 0.32 * direction;
        for [x, y, r] in &frame.wheels {
            let center = Vector2::new(
                pose.position.x + (x - frame.anchor[0]) * scale * direction,
                pose.position.y + (y - frame.anchor[1]) * scale,
            );
            let radius = r * scale;
            d.draw_line_ex(
                Vector2::new(
                    center.x - angle.cos() * radius,
                    center.y - angle.sin() * radius,
                ),
                Vector2::new(
                    center.x + angle.cos() * radius,
                    center.y + angle.sin() * radius,
                ),
                0.85,
                Color::new(225, 220, 195, 155),
            );
        }
    }

    /// Current displayed dimensions, useful for a shadow without atlas assumptions.
    pub fn size(&self, id: &str, ticks: u32) -> Vector2 {
        let (frame, scale) = self.catalog.pieces[id].sample(ticks);
        Vector2::new(frame.source[2] * scale, frame.source[3] * scale)
    }
}
