use borrow_fighters::adventure::{
    combat::{Action, Facing},
    engine::{actors, assets::Assets, render},
    locomotion::Gait,
    story::{Stage, Story},
    text::TextCatalog,
};
use borrow_fighters::runtime_paths::asset_path;
use raylib::prelude::*;
use std::{
    io::Write,
    path::Path,
    process::{Command, Stdio},
};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut rl, thread) = raylib::init()
        .size(1280, 720)
        .title("Rust running native review")
        .hidden()
        .build();
    let assets = Assets::load(
        &mut rl,
        &thread,
        TextCatalog::load(asset_path("assets/adventure/texts/pt-BR.json"))?,
    )?;
    let mut target = rl.load_render_texture(&thread, 1280, 720)?;
    let out = Path::new("docs/evidence/modular-running-world/gait");
    std::fs::create_dir_all(out)?;
    let mut story = Story::new();
    story.stage = Stage::Encounter;
    story.stage_ticks = 1000;
    {
        let mut d = rl.begin_texture_mode(&thread, &mut target);
        d.clear_background(Color::new(168, 176, 172, 255));
        for i in 0..12 {
            let mut actor = story.combat.player.clone();
            actor.action = Action::Walk;
            actor.gait = Gait::Run;
            actor.position.x = 90.0 + (i % 6) as f32 * 210.0;
            actor.position.y = 320.0 + (i / 6) as f32 * 350.0;
            actor.stride_distance = i as f32 / 12.0 * 272.0;
            actor.facing = Facing::Right;
            actors::rust_scaled(&mut d, &assets, &actor, 0.0, 1.5);
            d.draw_line(
                0,
                actor.position.y as i32,
                1280,
                actor.position.y as i32,
                Color::GRAY,
            );
            d.draw_text(
                &format!("{:02}", i),
                actor.position.x as i32,
                actor.position.y as i32 + 8,
                18,
                Color::BLACK,
            );
        }
    }
    let mut img = target.texture().load_image()?;
    img.flip_vertical();
    img.export_image(out.join("run-poses-native.png").to_str().unwrap());
    if std::env::args().any(|arg| arg == "--poses-only") {
        return Ok(());
    }
    let mut encoder = Command::new("ffmpeg")
        .args([
            "-y",
            "-loglevel",
            "error",
            "-f",
            "rawvideo",
            "-pixel_format",
            "rgba",
            "-video_size",
            "1280x720",
            "-framerate",
            "30",
            "-i",
            "-",
            "-vf",
            "vflip",
            "-c:v",
            "libx264",
            "-preset",
            "veryfast",
            "-crf",
            "20",
            "-pix_fmt",
            "yuv420p",
        ])
        .arg(out.join("rust-run-native.mp4"))
        .stdin(Stdio::piped())
        .spawn()?;
    let mut pipe = encoder.stdin.take().unwrap();
    for frame in 0..240 {
        let movement = match frame {
            0..=74 => 1.0,
            75..=89 => 0.0,
            90..=164 => -1.0,
            165..=179 => 0.0,
            _ => 1.0,
        };
        for _ in 0..2 {
            story
                .combat
                .tick_exploration(borrow_fighters::adventure::combat::CombatInput {
                    movement,
                    ..Default::default()
                });
        }
        {
            let mut d = rl.begin_texture_mode(&thread, &mut target);
            render::draw(&mut d, &story, &assets, false, false, false);
        }
        let img = target.texture().load_image()?;
        let pixels = img.get_image_data();
        let bytes: Vec<u8> = pixels.iter().flat_map(|p| [p.r, p.g, p.b, p.a]).collect();
        pipe.write_all(&bytes)?;
    }
    drop(pipe);
    assert!(encoder.wait()?.success());
    Ok(())
}
