//! Renders reusable facades on a continuous pavement with distant parallax.
//!
//! System: Adventure rendering. Only visible tiles and instances are submitted;
//! physical map length never changes a texture's scale or a facade's ground socket.

use super::{
    assets::Assets,
    pieces::{PiecePose, StreetPieces},
};
use crate::{adventure::landscape::Landscape, runtime_paths::asset_path};
use raylib::prelude::*;
use std::{error::Error, fs};

/// Cached art resources and validated composition, separate from live physics.
pub struct LandscapeAssets {
    /// Editable placement and prologue physical map.
    pub map: Landscape,
    /// Independent facade PNGs, each loaded once.
    pub pieces: StreetPieces,
    distance: Texture2D,
    pavement: Texture2D,
    planter: Texture2D,
}

impl LandscapeAssets {
    /// Loads the map and all referenced production pieces before entering a scene.
    pub fn load(rl: &mut RaylibHandle, thread: &RaylibThread) -> Result<Self, Box<dyn Error>> {
        let map = Landscape::from_json(&fs::read_to_string(asset_path(
            "assets/adventure/world/map.json",
        ))?)?;
        let pieces = StreetPieces::load_catalog(rl, thread, "assets/adventure/world/catalog.json")?;
        let street = crate::adventure::scenery::PieceCatalog::load(&asset_path(
            "assets/adventure/street/catalog.json",
        ))?;
        for scene in &map.scenes {
            for instance in &scene.instances {
                if !pieces.catalog.pieces.contains_key(&instance.piece)
                    && !street.pieces.contains_key(&instance.piece)
                {
                    return Err(format!(
                        "{} references missing facade {}",
                        instance.id, instance.piece
                    )
                    .into());
                }
            }
        }
        let distance = rl.load_texture(
            thread,
            &asset_path("assets/adventure/world/distance.png").to_string_lossy(),
        )?;
        distance.set_texture_filter(thread, TextureFilter::TEXTURE_FILTER_BILINEAR);
        let pavement = rl.load_texture(
            thread,
            &asset_path("assets/adventure/world/pavement.png").to_string_lossy(),
        )?;
        let planter = rl.load_texture(
            thread,
            &asset_path("assets/adventure/world/planter.png").to_string_lossy(),
        )?;
        pavement.set_texture_filter(thread, TextureFilter::TEXTURE_FILTER_BILINEAR);
        planter.set_texture_filter(thread, TextureFilter::TEXTURE_FILTER_BILINEAR);
        Ok(Self {
            pavement,
            planter,
            map,
            pieces,
            distance,
        })
    }
}

/// World interval visible through either a chapter camera or a cinematic shot.
pub struct View {
    /// First visible world coordinate, before clipping.
    pub left: f32,
    /// Horizontal visible extent at the current zoom.
    pub width: f32,
    /// Translation for renderers that already draw in viewport coordinates.
    pub origin: f32,
}

/// Draws road, pavement, distant hills and individually grounded building pieces.
pub fn draw(d: &mut impl RaylibDraw, a: &Assets, scene: &str, width: f32, view: View) {
    let art = &a.landscape;
    let source = Rectangle::new(
        0.0,
        0.0,
        art.distance.width as f32,
        art.distance.height as f32,
    );
    let tile = 960.0;
    // Anchor depth to the camera center, not the left edge of its changing
    // frustum. Zooming a still camera must not slide the hills behind the roofs.
    let center = view.left + view.width * 0.5;
    let drift = (center - 640.0) * (1.0 - art.map.distance_scroll);
    let first = ((view.left - drift) / tile).floor() as i32 - 1;
    let count = (view.width / tile).ceil() as i32 + 3;
    for index in first..first + count {
        let mut region = source;
        if index.rem_euclid(2) == 1 {
            region.width = -region.width;
        }
        d.draw_texture_pro(
            &art.distance,
            region,
            Rectangle::new(
                index as f32 * tile + drift + view.origin,
                -275.0,
                tile,
                640.0,
            ),
            Vector2::zero(),
            0.0,
            Color::WHITE,
        );
    }
    // Texture tiles follow world coordinates; the same seam remains in place
    // while a character, camera or story shot passes over it.
    let start = ((view.left / 512.0).floor() as i32 - 1).max(0);
    let end =
        (((view.left + view.width) / 512.0).ceil() as i32 + 1).min((width / 512.0).ceil() as i32);
    for index in start..end {
        let x = index as f32 * 512.0 + view.origin;
        for row in 0..4 {
            let region = Rectangle::new(
                0.0,
                0.0,
                if index % 2 == 1 { -512.0 } else { 512.0 },
                if row % 2 == 1 { -96.0 } else { 96.0 },
            );
            d.draw_texture_pro(
                &art.pavement,
                region,
                Rectangle::new(x, 355.0 + row as f32 * 96.0, 512.0, 96.0),
                Vector2::zero(),
                0.0,
                Color::WHITE,
            );
        }
    }
    let left = view.left + view.origin - 2.0;
    let extent = view.width + 4.0;
    d.draw_rectangle_gradient_v(
        left as i32,
        368,
        extent as i32,
        70,
        Color::new(89, 87, 78, 255),
        Color::new(114, 109, 93, 255),
    );
    d.draw_rectangle(
        left as i32,
        440,
        extent as i32,
        48,
        Color::new(156, 83, 66, 220),
    );
    for (y, color, thickness) in [
        (355.0, Color::new(217, 205, 175, 255), 4.0),
        (367.0, Color::new(68, 66, 54, 200), 3.0),
        (438.0, Color::new(218, 210, 176, 255), 3.0),
        (488.0, Color::new(221, 211, 177, 255), 4.0),
    ] {
        d.draw_line_ex(
            Vector2::new(left, y),
            Vector2::new(left + extent, y),
            thickness,
            color,
        );
    }
    for index in
        (view.left / 182.0).floor() as i32 - 1..((view.left + view.width) / 182.0).ceil() as i32 + 1
    {
        let x = index as f32 * 182.0 + view.origin;
        d.draw_line_ex(
            Vector2::new(x, 402.0),
            Vector2::new(x + 60.0, 402.0),
            2.0,
            Color::new(231, 220, 167, 200),
        );
        d.draw_line_ex(
            Vector2::new(x, 463.0),
            Vector2::new(x + 51.0, 463.0),
            2.0,
            Color::new(245, 224, 184, 150),
        );
    }
    for index in start..end {
        d.draw_texture_pro(
            &art.planter,
            Rectangle::new(0.0, 0.0, if index % 2 == 1 { -900.0 } else { 900.0 }, 46.0),
            Rectangle::new(index as f32 * 512.0 + view.origin, 488.0, 512.0, 30.0),
            Vector2::zero(),
            0.0,
            Color::WHITE,
        );
    }
    for item in &art.map.scene(scene).instances {
        let pieces = if art.pieces.catalog.pieces.contains_key(&item.piece) {
            &art.pieces
        } else {
            &a.street
        };
        let half = pieces.size(&item.piece, 0).x * item.scale * 0.5;
        if item.position[0] + half < view.left - 10.0
            || item.position[0] - half > view.left + view.width + 10.0
        {
            continue;
        }
        let mut pose = PiecePose::at(Vector2::new(
            item.position[0] + view.origin,
            item.position[1],
        ));
        pose.scale = item.scale;
        d.draw_ellipse(
            pose.position.x as i32,
            pose.position.y as i32 + 2,
            half * 0.96,
            3.0,
            Color::new(58, 52, 35, 80),
        );
        pieces.draw(d, &item.piece, 0, &pose);
    }
}
