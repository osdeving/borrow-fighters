//! Renders complete painted biographies with gentle camera tracks and external captions.
//!
//! The shared piece renderer loads each PNG once. A transactional F5 reload can
//! replace artwork, composition and animation without touching narrative clocks.

use super::{
    assets::Assets,
    pieces::{PiecePose, StreetPieces},
    typography::paragraph,
};
use crate::{
    adventure::{
        biography::{Biographies, sample_keys},
        text::TextCatalog,
    },
    runtime_paths::asset_path,
};
use raylib::prelude::*;
use std::error::Error;

pub struct BiographyAssets {
    pub pieces: StreetPieces,
    pub scenes: Biographies,
}

impl BiographyAssets {
    pub fn load(
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        text: &TextCatalog,
    ) -> Result<Self, Box<dyn Error>> {
        let pieces =
            StreetPieces::load_catalog(rl, thread, "assets/adventure/opening/scenes/catalog.json")?;
        let scenes = Biographies::load(
            &asset_path("assets/adventure/opening/scenes/scenes.json"),
            &pieces.catalog,
        )?;
        for shot in &scenes.shots {
            if !text.contains_key(&shot.caption) {
                return Err(format!("missing biography copy: {}", shot.caption).into());
            }
        }
        Ok(Self { pieces, scenes })
    }

    pub fn reload(
        &mut self,
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        text: &TextCatalog,
    ) -> Result<(), Box<dyn Error>> {
        let candidate = Self::load(rl, thread, text)?;
        *self = candidate;
        Ok(())
    }
}

pub fn draw(d: &mut impl RaylibDraw, a: &Assets, id: &str, ticks: u32) {
    let assets = &a.opening.biographies;
    let (shot, local) = assets.scenes.sample(id, ticks);
    let camera = sample_keys(&shot.camera, local);
    {
        let mut world = d.begin_mode2D(Camera2D {
            offset: Vector2::new(640.0, 360.0),
            target: Vector2::new(camera.position[0], camera.position[1]),
            rotation: 0.0,
            zoom: camera.scale,
        });
        assets.pieces.draw(
            &mut world,
            &shot.illustration,
            0,
            &PiecePose::at(Vector2::zero()),
        );
    }
    let ink = Color::new(14, 19, 26, 255);
    let paper = Color::new(239, 224, 192, 255);
    let accent = if id == "duke" {
        Color::new(248, 177, 64, 255)
    } else {
        Color::new(83, 199, 182, 255)
    };
    d.draw_rectangle(0, 28, 1280, 44, Color::new(14, 19, 26, 195));
    paragraph(
        d,
        &a.body,
        a.text.get(&format!("opening.{id}.role")),
        Rectangle::new(50.0, 40.0, 1170.0, 28.0),
        21.0,
        accent,
    );
    d.draw_rectangle_gradient_v(0, 556, 1280, 108, Color::BLANK, ink);
    paragraph(
        d,
        &a.title,
        a.text.get(&format!("opening.{id}.name")),
        Rectangle::new(48.0, 570.0, 1160.0, 40.0),
        32.0,
        paper,
    );
    paragraph(
        d,
        &a.body,
        a.text.get(&shot.caption),
        Rectangle::new(50.0, 615.0, 1170.0, 44.0),
        23.0,
        paper,
    );
    if local < 16 {
        d.draw_rectangle(
            0,
            28,
            1280,
            636,
            Color::new(14, 19, 26, ((1.0 - local as f32 / 16.0) * 255.0) as u8),
        );
    }
}
