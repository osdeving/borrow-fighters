//! Captures background actors and local props in the five arenas outside São Paulo.
//!
//! System: Development examples at the Raylib boundary. Uses the actual fight
//! renderer; São Paulo and flag comparisons live in capture_presentation_review.

use std::{error::Error, fs, path::Path};

use borrow_fighters::{
    config::{WINDOW_HEIGHT, WINDOW_WIDTH},
    engine::{
        assets::GameAssets,
        render::{GamepadStatus, draw_fight},
    },
    game::{arena::ArenaId, feature_flags::FeatureFlags, world::World},
};
use raylib::prelude::*;
use serde_json::json;

fn main() -> Result<(), Box<dyn Error>> {
    let destination = std::env::args()
        .nth(1)
        .ok_or("usage: cargo run --example capture_stage_review -- <output-directory>")?;
    let output = Path::new(&destination);
    fs::create_dir_all(output)?;
    let (mut raylib, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Borrow Fighters - stage review")
        .hidden()
        .msaa_4x()
        .build();
    let assets = GameAssets::load(&mut raylib, &thread);
    if assets.caramelo_run.is_none() {
        return Err("the caramelo atlas must load before stage review".into());
    }
    let world = World::new_greybox();
    let mut target =
        raylib.load_render_texture(&thread, WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32)?;
    let mut captures = Vec::new();
    for (arena, name, time) in [
        (ArenaId::Sirius, "sirius", 8.5),
        (ArenaId::Fortaleza, "fortaleza", 11.5),
        (ArenaId::BioTic, "biotic", 13.5),
        (ArenaId::PortoDigital, "porto-digital", 10.5),
        (ArenaId::ValeDoPinhao, "vale-pinhao", 12.5),
    ] {
        if assets.arenas.get(arena).is_none() {
            return Err(format!("{} arena texture failed to load", arena.label()).into());
        }
        {
            let mut draw = raylib.begin_texture_mode(&thread, &mut target);
            draw_fight(
                &mut draw,
                &world,
                arena,
                time,
                FeatureFlags::default(),
                GamepadStatus::default(),
                &assets,
            );
        }
        let filename = format!("{name}.png");
        let mut image = target.texture().load_image()?;
        image.flip_vertical();
        fs::write(
            output.join(&filename),
            image.export_image_to_memory(".png")?.as_ref(),
        )?;
        captures.push(json!({"image":filename,"arena":arena.label(),
            "visual_time_seconds":time,"stage_life":true}));
    }
    fs::write(
        output.join("stage-review.json"),
        serde_json::to_string_pretty(&json!({
            "render_size":[WINDOW_WIDTH,WINDOW_HEIGHT],"captures":captures
        }))? + "\n",
    )?;
    println!("Stage review written to {}", output.display());
    Ok(())
}
