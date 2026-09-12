//! Renders layered biographies with independent sprites, camera tracks and attached text.
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
            for key in std::iter::once(&shot.caption).chain(shot.labels.iter().map(|l| &l.text)) {
                if !text.contains_key(key) {
                    return Err(format!("missing biography copy: {key}").into());
                }
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
            rotation: camera.rotation,
            zoom: camera.scale,
        });
        for track in &shot.tracks {
            let key = sample_keys(&track.keys, local);
            assets.pieces.draw(
                &mut world,
                &track.piece,
                local,
                &PiecePose {
                    position: Vector2::new(key.position[0], key.position[1]),
                    scale: key.scale,
                    flip: track.flip,
                    rotation: key.rotation,
                    tint: Color::new(255, 255, 255, (key.opacity * 255.0) as u8),
                },
            );
            for label in shot.labels.iter().filter(|l| l.parent == track.id) {
                let direction = if track.flip { -1.0 } else { 1.0 };
                let [ox, oy] = [
                    label.offset[0] * key.scale * direction,
                    label.offset[1] * key.scale,
                ];
                let angle = key.rotation.to_radians();
                let at = Vector2::new(
                    key.position[0] + ox * angle.cos() - oy * angle.sin(),
                    key.position[1] + ox * angle.sin() + oy * angle.cos(),
                );
                let mut ink = label.color;
                ink[3] = (ink[3] as f32 * key.opacity) as u8;
                let maximum = label.size * key.scale;
                let measured = a
                    .body
                    .measure_text(a.text.get(&label.text), maximum, 0.5)
                    .x
                    .max(1.0);
                world.draw_text_pro(
                    &a.body,
                    a.text.get(&label.text),
                    at,
                    Vector2::zero(),
                    key.rotation,
                    maximum * (label.width * key.scale / measured).min(1.0),
                    0.5,
                    Color::new(ink[0], ink[1], ink[2], ink[3]),
                );
            }
        }
    }
    let ink = Color::new(14, 19, 26, 255);
    let paper = Color::new(239, 224, 192, 255);
    let accent = if id == "duke" {
        Color::new(248, 177, 64, 255)
    } else {
        Color::new(83, 199, 182, 255)
    };
    d.draw_rectangle(0, 28, 1280, 56, Color::new(14, 19, 26, 195));
    paragraph(
        d,
        &a.body,
        a.text.get(&format!("opening.{id}.role")),
        Rectangle::new(50.0, 44.0, 1170.0, 35.0),
        24.0,
        accent,
    );
    d.draw_rectangle_gradient_v(0, 510, 1280, 160, Color::BLANK, ink);
    paragraph(
        d,
        &a.title,
        a.text.get(&format!("opening.{id}.name")),
        Rectangle::new(48.0, 539.0, 1160.0, 48.0),
        39.0,
        paper,
    );
    paragraph(
        d,
        &a.body,
        a.text.get(&shot.caption),
        Rectangle::new(50.0, 601.0, 1170.0, 56.0),
        26.0,
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
