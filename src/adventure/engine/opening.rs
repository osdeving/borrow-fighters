//! Composes the post-encounter newspaper, character-history and rotating-title montage.
//!
//! System: Adventure presentation. Its visual copies are local assets, with no
//! fighting catalog or rules; all displayed wording comes from the editable text.

use super::{
    assets::Assets,
    typography::{centered, paragraph},
};
use crate::{
    adventure::story::{OPENING_TICKS, Stage, Story},
    runtime_paths::asset_path,
};
use raylib::prelude::*;
use serde::Deserialize;
use std::{
    error::Error,
    fs,
    path::{Component, Path},
};

const INK: Color = Color::new(14, 19, 26, 255);
const PAPER: Color = Color::new(239, 224, 192, 255);
const GOLD: Color = Color::new(248, 177, 64, 255);
const TEAL: Color = Color::new(83, 199, 182, 255);
const TITLE_ROSTER: [&str; 5] = ["c", "duke", "cpp", "python", "rust"];

struct Portrait {
    id: String,
    image: Texture2D,
    frames: Vec<Rectangle>,
    height: f32,
}

/// Images and display font belonging exclusively to the presentation montage.
pub struct OpeningAssets {
    /// Editable layered corporate and old-school biographies.
    pub biographies: super::biography::BiographyAssets,
    cpp: Texture2D,
    python: Texture2D,
    roster: Vec<Portrait>,
    display: Font,
}

impl OpeningAssets {
    /// Loads checked local artwork independently from any fighting-game manifest.
    pub fn load(
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        text: &crate::adventure::text::TextCatalog,
    ) -> Result<Self, Box<dyn Error>> {
        #[derive(Deserialize)]
        struct Entry {
            id: String,
            image: String,
            rects: Vec<[f32; 4]>,
        }
        #[derive(Deserialize)]
        struct Manifest {
            characters: Vec<Entry>,
        }
        let manifest: Manifest = serde_json::from_str(&fs::read_to_string(asset_path(
            "assets/adventure/opening/roster.json",
        ))?)?;
        let mut roster = Vec::new();
        for entry in manifest.characters {
            if Path::new(&entry.image)
                .components()
                .any(|p| !matches!(p, Component::Normal(_)))
            {
                return Err("opening portrait must use a relative local image".into());
            }
            let image = texture(rl, thread, &entry.image)?;
            if entry.rects.is_empty() || entry.rects.len() > 16 {
                return Err("invalid opening portrait frame count".into());
            }
            let mut frames = Vec::new();
            for [x, y, w, h] in entry.rects {
                if ![x, y, w, h].iter().all(|v| v.is_finite())
                    || x < 0.0
                    || y < 0.0
                    || w <= 0.0
                    || h <= 0.0
                    || x + w > image.width() as f32
                    || y + h > image.height() as f32
                {
                    return Err(format!("invalid opening portrait bounds: {}", entry.id).into());
                }
                frames.push(Rectangle::new(x, y, w, h));
            }
            let height = frames.iter().map(|r| r.height).fold(1.0, f32::max);
            roster.push(Portrait {
                id: entry.id,
                image,
                frames,
                height,
            });
        }
        if roster.len() != TITLE_ROSTER.len()
            || TITLE_ROSTER
                .iter()
                .any(|id| roster.iter().filter(|p| p.id == *id).count() != 1)
        {
            return Err("opening requires the five unique character portraits".into());
        }
        let mut glyphs: String = (32..=591).filter_map(char::from_u32).collect();
        glyphs.push_str("—–“”‘’…");
        let display = rl.load_font_ex(
            thread,
            &asset_path("assets/adventure/fonts/BarlowCondensed-SemiBold.ttf").to_string_lossy(),
            120,
            Some(&glyphs),
        )?;
        Ok(Self {
            biographies: super::biography::BiographyAssets::load(rl, thread, text)?,
            cpp: texture(rl, thread, "cpp-origin.png")?,
            python: texture(rl, thread, "python-teacher.png")?,
            roster,
            display,
        })
    }
}

/// Draws the montage or holds its final title while the completion menu is visible.
pub fn draw(d: &mut impl RaylibDraw, story: &Story, a: &Assets) {
    let ticks = if story.stage == Stage::Complete {
        OPENING_TICKS - 1
    } else {
        story.stage_ticks
    };
    let t = ticks as f32 / 60.0;
    d.clear_background(INK);
    match t {
        t if t < 9.0 => newspaper(d, a, t),
        t if t < 19.0 => history(d, a, "cpp", t - 9.0),
        t if t < 29.0 => history(d, a, "python", t - 19.0),
        t if t < 41.0 => super::biography::draw(d, a, "duke", ticks.saturating_sub(29 * 60)),
        t if t < 53.0 => super::biography::draw(d, a, "c", ticks.saturating_sub(41 * 60)),
        t if t < 57.0 => character(d, a, t - 53.0),
        _ => title(d, a, t - 57.0),
    }
    // Film cadence: warm dust, restrained scan lines and letterboxing.
    for i in 0..24 {
        let f = i as f32;
        d.draw_circle_v(
            Vector2::new(
                (f * 183.7 + t * 9.0) % 1280.0,
                (f * 97.1 - t * 13.0).rem_euclid(720.0),
            ),
            1.0,
            tint(PAPER, 0.17),
        );
    }
    d.draw_rectangle(0, 0, 1280, 28, INK);
    d.draw_rectangle(0, 664, 1280, 56, INK);
    if story.stage != Stage::Complete {
        centered(
            d,
            &a.body,
            a.text.get("opening.controls"),
            Vector2::new(640.0, 683.0),
            1200.0,
            19.0,
            tint(PAPER, 0.65),
        );
    }
    if t < 0.7 {
        d.draw_rectangle(0, 0, 1280, 720, tint(INK, 1.0 - t / 0.7));
    }
}

fn newspaper(d: &mut impl RaylibDraw, a: &Assets, t: f32) {
    let index = (t / 3.0).floor() as usize;
    let local = t % 3.0;
    // Scattered sheets reveal a larger press landscape behind the current headline.
    for i in 0..5 {
        d.draw_rectangle_pro(
            Rectangle::new(270.0 + i as f32 * 190.0, 360.0, 750.0, 525.0),
            Vector2::new(375.0, 262.0),
            -18.0 + i as f32 * 8.0,
            tint(PAPER, 0.12),
        );
    }
    let arrival = (local / 0.45).clamp(0.0, 1.0);
    let settle = 1.0 - (1.0 - arrival).powi(3);
    let camera = Camera2D {
        offset: Vector2::new(640.0, 345.0),
        target: Vector2::zero(),
        rotation: (1.0 - settle) * -20.0 + [1.5, -1.8, 0.8][index.min(2)],
        zoom: 0.65 + 0.35 * settle,
    };
    let mut paper = d.begin_mode2D(camera);
    paper.draw_rectangle(-550, -290, 1100, 580, PAPER);
    for y in (0..550).step_by(7) {
        paper.draw_line(-548, y - 287, 547, y - 287, tint(INK, 0.025));
    }
    centered(
        &mut paper,
        &a.title,
        a.text.get("opening.newspaper"),
        Vector2::new(0.0, -259.0),
        990.0,
        52.0,
        INK,
    );
    paper.draw_line(-505, -188, 505, -188, INK);
    paper.draw_line(-505, -184, 505, -184, INK);
    paragraph(
        &mut paper,
        &a.body,
        a.text.get(&format!("opening.news.{index}.section")),
        Rectangle::new(-503.0, -168.0, 1000.0, 34.0),
        20.0,
        INK,
    );
    paragraph(
        &mut paper,
        &a.opening.display,
        a.text.get(&format!("opening.news.{index}.headline")),
        Rectangle::new(-500.0, -111.0, 1000.0, 175.0),
        69.0,
        INK,
    );
    paragraph(
        &mut paper,
        &a.title,
        a.text.get(&format!("opening.news.{index}.deck")),
        Rectangle::new(-498.0, 90.0, 950.0, 95.0),
        27.0,
        INK,
    );
    for column in 0..3 {
        for row in 0..5 {
            paper.draw_rectangle(
                -500 + column * 344,
                211 + row * 10,
                300 - (row % 3) * 26,
                2,
                tint(INK, 0.19),
            );
        }
    }
}

fn history(d: &mut impl RaylibDraw, a: &Assets, id: &str, t: f32) {
    let image = if id == "cpp" {
        &a.opening.cpp
    } else {
        &a.opening.python
    };
    let phase = usize::from(t >= 5.0);
    let local = t % 5.0;
    panel(d, image, phase, local / 5.0);
    let accent = if id == "cpp" { GOLD } else { TEAL };
    if phase == 1 {
        for i in 0..13 {
            let n = i as f32;
            let x = 850.0 + (t * 0.8 + n * 0.65).cos() * 200.0;
            let y = 300.0 + (t * 0.55 + n).sin() * 170.0;
            d.draw_circle_v(Vector2::new(x, y), 2.5, tint(accent, 0.5));
        }
    }
    d.draw_rectangle_gradient_v(0, 410, 1280, 280, Color::BLANK, tint(INK, 0.98));
    d.draw_rectangle(0, 28, 1280, 62, tint(INK, 0.75));
    paragraph(
        d,
        &a.body,
        a.text.get(&format!("opening.{id}.role")),
        Rectangle::new(50.0, 47.0, 1180.0, 32.0),
        23.0,
        accent,
    );
    paragraph(
        d,
        &a.opening.display,
        a.text.get(&format!("opening.{id}.name")),
        Rectangle::new(52.0, 446.0, 1080.0, 91.0),
        74.0,
        PAPER,
    );
    paragraph(
        d,
        &a.body,
        a.text.get(&format!(
            "opening.{id}.{}",
            if phase == 0 { "before" } else { "after" }
        )),
        Rectangle::new(56.0, 550.0, 1115.0, 85.0),
        30.0,
        PAPER,
    );
    let transition = (local / 0.28).min(1.0);
    if transition < 1.0 {
        d.draw_rectangle(0, 28, 1280, 636, tint(INK, 1.0 - transition));
    }
}

fn character(d: &mut impl RaylibDraw, a: &Assets, t: f32) {
    let id = "rust";
    let accent = GOLD;
    let local = t % 4.0;
    for i in 0..8 {
        let x = -250.0 + i as f32 * 250.0 - local * 35.0;
        d.draw_rectangle_pro(
            Rectangle::new(x, 390.0, 105.0, 1000.0),
            Vector2::new(52.0, 500.0),
            27.0,
            tint(accent, 0.06),
        );
    }
    let entrance = 1.0 - (1.0 - (local / 0.5).min(1.0)).powi(3);
    avatar(
        d,
        a,
        id,
        Vector2::new(860.0 + (1.0 - entrance) * 380.0, 650.0),
        530.0,
        local,
        Color::WHITE,
    );
    paragraph(
        d,
        &a.opening.display,
        a.text.get(&format!("opening.{id}.name")),
        Rectangle::new(65.0, 254.0, 645.0, 134.0),
        102.0,
        PAPER,
    );
    d.draw_rectangle(68, 404, (540.0 * entrance) as i32, 5, accent);
    paragraph(
        d,
        &a.body,
        a.text.get(&format!("opening.{id}.role")),
        Rectangle::new(70.0, 438.0, 600.0, 95.0),
        30.0,
        accent,
    );
    if local < 0.15 {
        d.draw_rectangle(0, 28, 1280, 636, tint(PAPER, (1.0 - local / 0.15) * 0.4));
    }
}

fn title(d: &mut impl RaylibDraw, a: &Assets, t: f32) {
    let column_width = 1280.0 / TITLE_ROSTER.len() as f32;
    for (i, id) in TITLE_ROSTER.iter().enumerate() {
        let x = (i as f32 + 0.5) * column_width;
        let slide = (1.0 - (t * 1.4 - i as f32 * 0.08).clamp(0.0, 1.0)) * 200.0;
        d.draw_rectangle(
            (i as f32 * column_width) as i32,
            28,
            column_width as i32 - 1,
            636,
            tint(if i % 2 == 0 { GOLD } else { TEAL }, 0.08),
        );
        avatar(
            d,
            a,
            id,
            Vector2::new(x, 660.0 + slide),
            440.0,
            t,
            tint(Color::WHITE, 0.47),
        );
    }
    d.draw_rectangle(0, 28, 1280, 636, tint(INK, 0.50));
    let arrival = (t / 1.6).clamp(0.0, 1.0);
    let ease = 1.0 - (1.0 - arrival).powi(3);
    let camera = Camera2D {
        offset: Vector2::new(640.0, 322.0),
        target: Vector2::zero(),
        rotation: -360.0 * (1.0 - ease),
        zoom: 0.18 + 0.82 * ease,
    };
    {
        let mut logo = d.begin_mode2D(camera);
        logo.draw_rectangle_pro(
            Rectangle::new(0.0, 0.0, 900.0, 290.0),
            Vector2::new(450.0, 145.0),
            -4.0,
            tint(INK, 0.93),
        );
        logo.draw_line_ex(
            Vector2::new(-420.0, -121.0),
            Vector2::new(420.0, -121.0),
            3.0,
            GOLD,
        );
        centered(
            &mut logo,
            &a.opening.display,
            a.text.get("opening.logo.top"),
            Vector2::new(0.0, -119.0),
            815.0,
            116.0,
            PAPER,
        );
        centered(
            &mut logo,
            &a.opening.display,
            a.text.get("opening.logo.bottom"),
            Vector2::new(0.0, -9.0),
            820.0,
            123.0,
            GOLD,
        );
        logo.draw_line_ex(
            Vector2::new(-420.0, 130.0),
            Vector2::new(420.0, 130.0),
            3.0,
            GOLD,
        );
    }
    let subtitle = ((t - 1.9) / 0.9).clamp(0.0, 1.0);
    centered(
        d,
        &a.opening.display,
        a.text.get("opening.subtitle"),
        Vector2::new(640.0, 492.0 + (1.0 - subtitle) * 24.0),
        1080.0,
        43.0,
        tint(PAPER, subtitle),
    );
    centered(
        d,
        &a.body,
        a.text.get("opening.tagline"),
        Vector2::new(640.0, 552.0),
        1080.0,
        25.0,
        tint(TEAL, ((t - 3.2) / 0.8).clamp(0.0, 1.0)),
    );
    if t < 0.3 {
        d.draw_rectangle(0, 28, 1280, 636, tint(GOLD, (1.0 - t / 0.3) * 0.5));
    }
}

fn avatar(
    d: &mut impl RaylibDraw,
    a: &Assets,
    id: &str,
    feet: Vector2,
    height: f32,
    t: f32,
    color: Color,
) {
    let portrait = a
        .opening
        .roster
        .iter()
        .find(|p| p.id == id)
        .expect("validated opening roster");
    let frame = portrait.frames[((t * 5.0) as usize) % portrait.frames.len()];
    let scale = height / portrait.height;
    d.draw_texture_pro(
        &portrait.image,
        frame,
        Rectangle::new(feet.x, feet.y, frame.width * scale, frame.height * scale),
        Vector2::new(frame.width * scale * 0.5, frame.height * scale),
        0.0,
        color,
    );
}

fn panel(d: &mut impl RaylibDraw, image: &Texture2D, index: usize, t: f32) {
    let w = image.width() as f32;
    let h = image.height() as f32 * 0.5;
    let zoom = 1.0 + t * 0.055;
    let crop = Rectangle::new(
        (w - w / zoom) * 0.5,
        index as f32 * h + (h - h / zoom) * 0.3,
        w / zoom,
        h / zoom,
    );
    d.draw_texture_pro(
        image,
        crop,
        Rectangle::new(0.0, 0.0, 1280.0, 720.0),
        Vector2::zero(),
        0.0,
        Color::WHITE,
    );
}

fn texture(
    rl: &mut RaylibHandle,
    thread: &RaylibThread,
    file: &str,
) -> Result<Texture2D, Box<dyn Error>> {
    let path = asset_path(format!("assets/adventure/opening/{file}"));
    let texture = rl
        .load_texture(thread, &path.to_string_lossy())
        .map_err(|e| format!("{}: {e}", path.display()))?;
    texture.set_texture_filter(thread, TextureFilter::TEXTURE_FILTER_BILINEAR);
    Ok(texture)
}

fn tint(color: Color, opacity: f32) -> Color {
    Color::new(
        color.r,
        color.g,
        color.b,
        (opacity.clamp(0.0, 1.0) * 255.0) as u8,
    )
}
