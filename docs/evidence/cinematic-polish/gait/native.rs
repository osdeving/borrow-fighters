//! Captures proportional comparison and actual public-input chapter approaches.
use borrow_fighters::adventure::{
    chapter::{Chapter, ChapterInput, Phase, World},
    combat::{Action, Facing},
    engine::{
        actors,
        chapter::{self, assets::ChapterAssets},
    },
    locomotion::{Gait, TravelView},
};
use raylib::prelude::*;
use std::{
    io::Write,
    path::Path,
    process::{Command, Stdio},
};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut rl, thread) = raylib::init()
        .size(1280, 720)
        .title("Rust proportions and crossing native review")
        .hidden()
        .build();
    let a = ChapterAssets::load(&mut rl, &thread)?;
    let mut target = rl.load_render_texture(&thread, 1280, 720)?;
    let out = Path::new("docs/evidence/cinematic-polish/gait");
    std::fs::create_dir_all(out)?;
    let mut state = Chapter::new(World::bundled(), false);
    {
        let mut d = rl.begin_texture_mode(&thread, &mut target);
        d.clear_background(Color::new(164, 173, 170, 255));
        for i in 0..5 {
            let mut actor = state.combat.player.clone();
            actor.position.x = 130.0 + i as f32 * 250.0;
            actor.position.y = 495.0;
            actor.facing = Facing::Right;
            actor.action = if i == 0 { Action::Idle } else { Action::Walk };
            actor.gait = Gait::Run;
            actor.stride_distance = (i as f32 - 1.0) * 272.0 / 8.0;
            actors::rust_scaled(&mut d, &a.common, &actor, 0.0, 1.7);
        }
        d.draw_line(0, 495, 1280, 495, Color::GRAY);
        d.draw_text("IDLE                      RUN 0.00                  RUN 0.125                  RUN 0.25                   RUN 0.375",45,530,20,Color::BLACK);
    }
    let mut img = target.texture().load_image()?;
    img.flip_vertical();
    img.export_image(out.join("idle-run-native.png").to_str().unwrap());
    {
        let mut d = rl.begin_texture_mode(&thread, &mut target);
        d.clear_background(Color::new(164, 173, 170, 255));
        for i in 0..8 {
            let mut actor = state.combat.player.clone();
            actor.action = Action::Walk;
            actor.gait = Gait::Walk;
            actor.position.x = 155.0 + (i % 4) as f32 * 315.0;
            actor.position.y = 340.0 + (i / 4) as f32 * 350.0;
            actor.stride_distance = (i % 4) as f32 * 32.0;
            a.common.locomotion.draw_crossing(
                &mut d,
                &actor,
                if i < 4 {
                    TravelView::Back
                } else {
                    TravelView::Front
                },
                1.65,
            );
        }
    }
    let mut img = target.texture().load_image()?;
    img.flip_vertical();
    img.export_image(out.join("crossing-poses-native.png").to_str().unwrap());
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
        .arg(out.join("crossing-native.mp4"))
        .stdin(Stdio::piped())
        .spawn()?;
    let mut pipe = encoder.stdin.take().unwrap();
    let mut trace = std::fs::File::create(out.join("crossing.jsonl"))?;
    let mut seen = std::collections::HashSet::new();
    state.tick(ChapterInput {
        skip: true,
        ..Default::default()
    });
    for frame in 0..800 {
        for _ in 0..2 {
            let mut input = ChapterInput::default();
            let target = match state.phase {
                Phase::ExploreDriver => Some("driver"),
                Phase::ExploreShop => Some("shop"),
                Phase::ExploreNeighbour => Some("neighbour"),
                _ => None,
            };
            if let Some(id) = target {
                let region = state.world.scene(state.scene).poi(id).unwrap().region;
                input.movement =
                    (region.x + region.width * 0.5 - state.player().position.x).signum();
                if state.nearby_interaction().is_some() {
                    input.interact = true;
                    input.movement = 0.0;
                }
            } else if state.active_dialogue().is_some() {
                input.advance = state.phase_ticks >= 60;
            }
            state.tick(input);
        }
        {
            let mut d = rl.begin_texture_mode(&thread, &mut target);
            chapter::draw(&mut d, &a, &state, 0, false);
        }
        let mut img = target.texture().load_image()?;
        let bytes: Vec<u8> = img
            .get_image_data()
            .iter()
            .flat_map(|p| [p.r, p.g, p.b, p.a])
            .collect();
        pipe.write_all(&bytes)?;
        writeln!(
            trace,
            "{{\"frame\":{frame},\"phase\":\"{:?}\",\"view\":\"{:?}\",\"x\":{},\"y\":{}}}",
            state.phase,
            state.travel_view(),
            state.player().position.x,
            state.player().position.y
        )?;
        let key = format!("{:?}-{:?}", state.phase, state.travel_view());
        if state.travel_view() != TravelView::Side
            && state.phase_ticks > 30
            && seen.insert(key.clone())
        {
            img.flip_vertical();
            img.export_image(out.join(format!("{key}.png")).to_str().unwrap());
        }
        if state.phase == Phase::Phone {
            break;
        }
    }
    drop(pipe);
    assert!(encoder.wait()?.success());
    Ok(())
}
