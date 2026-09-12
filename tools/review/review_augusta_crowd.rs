//! Captures continuous painted crowd performances using the actual runtime renderer.
//!
//! Each motion reuses one uninterrupted story pose sequence across all eight
//! paintings and both facings, so source extraction defects remain easy to compare.

use borrow_fighters::adventure::{
    augusta::ambient::{Nightlife, Wardrobe},
    engine::production::{assets::ProductionAssets, nightlife},
};
use raylib::prelude::*;
use std::{
    error::Error,
    fs,
    io::Write,
    path::PathBuf,
    process::{Command, Stdio},
};

fn main() -> Result<(), Box<dyn Error>> {
    let output = PathBuf::from(std::env::args().nth(1).ok_or("expected output directory")?);
    fs::create_dir_all(&output)?;
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
            "libx264rgb",
            "-preset",
            "ultrafast",
            "-crf",
            "0",
            "-pix_fmt",
            "rgb24",
        ])
        .arg(output.join("crowd-lossless.mkv"))
        .stdin(Stdio::piped())
        .spawn()?;
    let mut pipe = encoder.stdin.take().ok_or("missing FFmpeg input")?;
    let (mut rl, thread) = raylib::init()
        .size(1280, 720)
        .title("Augusta — painted crowd frame review")
        .build();
    rl.set_window_state(WindowState::default().set_window_hidden(true));
    let assets = ProductionAssets::load(
        &mut rl,
        &thread,
        std::path::Path::new("assets/adventure/chapters/cpp-augusta/chapter.json"),
    )?;
    let mut target = rl.load_render_texture(&thread, 1280, 720)?;
    let cast = [
        (Wardrobe::Leather, "Leather"),
        (Wardrobe::PlumDress, "Plum"),
        (Wardrobe::AmberJacket, "Amber"),
        (Wardrobe::Denim, "Denim"),
        (Wardrobe::TealDress, "Teal"),
        (Wardrobe::WhiteShirt, "White"),
        (Wardrobe::RedBlouse, "Red"),
        (Wardrobe::LongCoat, "Coat"),
    ];
    let mut manifest = vec![];
    for (section, (motion, id)) in [
        ("walking", 0),
        ("conversation", 1),
        ("seated", 3),
        ("phone", 8),
        ("running", 10),
    ]
    .into_iter()
    .enumerate()
    {
        for frame in 0..180_u64 {
            let age = 30 + frame * 2;
            let sample = if motion == "running" {
                Nightlife::sample(100 + age, Some(age))
            } else {
                Nightlife::sample(240 + frame * 2, None)
            };
            let source = *sample.people.iter().find(|p| p.id == id).ok_or_else(|| {
                format!("continuous source {id} vanished during {motion} frame {frame}")
            })?;
            {
                let mut window = rl.begin_drawing(&thread);
                window.clear_background(Color::BLACK);
                let mut d = window.begin_texture_mode(&thread, &mut target);
                d.clear_background(Color::new(36, 41, 48, 255));
                for row in 0..2 {
                    let baseline = 315. + row as f32 * 360.;
                    d.draw_line(
                        0,
                        baseline as i32,
                        1280,
                        baseline as i32,
                        Color::new(88, 94, 105, 255),
                    );
                    d.draw_text(
                        &format!(
                            "{motion} - frame {frame:03} / 179 - {}",
                            if row == 0 { "right" } else { "left" }
                        ),
                        16,
                        12 + row * 360,
                        20,
                        Color::RAYWHITE,
                    );
                    for (index, (wardrobe, label)) in cast.iter().enumerate() {
                        let mut person = source;
                        person.wardrobe = *wardrobe;
                        person.x = 78. + index as f32 * 159.;
                        person.ground_y = baseline;
                        person.scale = 2.2;
                        person.facing = if row == 0 { 1. } else { -1. };
                        nightlife::draw_person(&mut d, &assets, &person, 640.);
                        d.draw_text(
                            label,
                            42 + index as i32 * 159,
                            baseline as i32 + 12,
                            16,
                            Color::LIGHTGRAY,
                        );
                    }
                }
            }
            let number = section as u64 * 180 + frame;
            let image = target.texture().load_image()?;
            let colors = image.get_image_data();
            const {
                assert!(std::mem::size_of::<raylib::ffi::Color>() == 4);
            }
            // SAFETY: Raylib Color is repr(C), four initialized u8 channels.
            // The byte slice is read only and remains within this owning image.
            let bytes = unsafe {
                std::slice::from_raw_parts(colors.as_ptr().cast::<u8>(), colors.len() * 4)
            };
            pipe.write_all(bytes)?;
            manifest.push(serde_json::json!({
                "frame": number, "motion": motion, "motion_frame": frame,
                "source_id": id, "ticks": sample.ticks,
                "threat_age": sample.threat_age,
                "feet": source.pose.feet, "hands": source.pose.hands,
                "facings": [1, -1], "file": format!("frame{number:04}.png"),
            }));
        }
    }
    fs::write(
        output.join("frames.json"),
        serde_json::to_string_pretty(&manifest)?,
    )?;
    drop(pipe);
    if !encoder.wait()?.success() {
        return Err("lossless crowd capture failed".into());
    }
    Ok(())
}
