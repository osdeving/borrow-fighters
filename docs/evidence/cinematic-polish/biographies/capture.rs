//! Captures the four editable biographies through the game's native renderer.
use borrow_fighters::adventure::{
    engine::{assets::Assets, opening},
    story::{Stage, Story},
    text::TextCatalog,
};
use raylib::prelude::*;
use std::{
    error::Error,
    io::Write,
    process::{Command, Stdio},
};
fn main() -> Result<(), Box<dyn Error>> {
    let (mut rl, thread) = raylib::init()
        .size(1280, 720)
        .title("Biography review")
        .build();
    rl.set_window_state(WindowState::default().set_window_hidden(true));
    let assets = Assets::load(
        &mut rl,
        &thread,
        TextCatalog::load("assets/adventure/texts/pt-BR.json")?,
    )?;
    let mut target = rl.load_render_texture(&thread, 1280, 720)?;
    let mut state = Story::new();
    state.stage = Stage::Opening;
    let out = "docs/evidence/cinematic-polish/biographies";
    for (id, tick, label) in [
        ("duke", 180, "duke-paulista"),
        ("duke", 359, "duke-before-cut"),
        ("duke", 368, "duke-crossing-cut"),
        ("duke", 560, "duke-boardroom"),
        ("c", 140, "old-c-workshop"),
        ("c", 560, "old-c-foundations"),
    ] {
        {
            let mut d = rl.begin_texture_mode(&thread, &mut target);
            d.clear_background(Color::BLACK);
            state.stage_ticks = if id == "duke" {
                1740 + tick
            } else {
                2460 + tick
            };
            opening::draw(&mut d, &state, &assets);
        }
        let mut image = target.texture().load_image()?;
        image.flip_vertical();
        image.export_image(&format!("{out}/{label}.png"));
    }
    if std::env::args().any(|a| a == "--video") {
        let mut video = Command::new("ffmpeg")
            .args([
                "-y",
                "-loglevel",
                "error",
                "-f",
                "rawvideo",
                "-pix_fmt",
                "rgba",
                "-s",
                "1280x720",
                "-r",
                "30",
                "-i",
                "-",
                "-an",
                "-c:v",
                "libx264",
                "-preset",
                "fast",
                "-crf",
                "20",
                "-pix_fmt",
                "yuv420p",
                "-movflags",
                "+faststart",
            ])
            .arg(format!("{out}/biographies-native.mp4"))
            .stdin(Stdio::piped())
            .spawn()?;
        let mut input = video.stdin.take().unwrap();
        for id in ["duke", "c"] {
            for tick in (0..720).step_by(2) {
                {
                    let mut d = rl.begin_texture_mode(&thread, &mut target);
                    d.clear_background(Color::BLACK);
                    state.stage_ticks = if id == "duke" {
                        1740 + tick
                    } else {
                        2460 + tick
                    };
                    opening::draw(&mut d, &state, &assets);
                }
                let mut image = target.texture().load_image()?;
                image.flip_vertical();
                let bytes: Vec<u8> = image
                    .get_image_data()
                    .iter()
                    .flat_map(|c| [c.r, c.g, c.b, c.a])
                    .collect();
                input.write_all(&bytes)?;
            }
        }
        drop(input);
        assert!(video.wait()?.success());
    }
    Ok(())
}
