//! Captures Rust's native public actor renderer without depending on a run technique.
//!
//! Contact sheets sample distance at nominal running speed, not rig joints. The
//! video keeps real exploration velocity and shows the same state at 1x and 2x.

use borrow_fighters::adventure::{
    chapter::{Chapter, ChapterInput, Checkpoint},
    combat::{Action, Actor, CombatInput, Facing, PLAYER_RUN_SPEED},
    engine::{actors, assets::Assets, chapter, chapter::assets::ChapterAssets, render},
    locomotion::Gait,
    story::Story,
};
use raylib::prelude::*;
use std::{
    error::Error,
    fs,
    io::Write,
    path::Path,
    process::{Command, Stdio},
};

const BACK: Color = Color::new(185, 193, 196, 255);
const INK: Color = Color::new(24, 31, 39, 255);
const GRID: Color = Color::new(135, 148, 155, 255);
const ALERT: Color = Color::new(139, 52, 18, 255);
const RECOVERY_PHASES: [f32; 8] = [0.45, 0.5, 0.55, 0.5742512, 0.6, 0.625, 0.65, 0.7];

fn export(target: &RenderTexture2D, path: &Path) -> Result<(), Box<dyn Error>> {
    let mut image = target.texture().load_image()?;
    image.flip_vertical();
    image.export_image(path.to_str().ok_or("non-UTF8 output path")?);
    Ok(())
}

fn pose(d: &mut impl RaylibDraw, assets: &Assets, original: &Actor, at: Vector2, depth: f32) {
    let mut actor = original.clone();
    actor.position.x = at.x;
    actor.position.y = at.y;
    actors::rust_scaled(d, assets, &actor, 0.0, depth);
}

fn floor(d: &mut impl RaylibDraw, x: f32, y: f32, width: f32) {
    d.draw_line_ex(Vector2::new(x, y), Vector2::new(x + width, y), 1.0, GRID);
}

fn sheet(
    rl: &mut RaylibHandle,
    thread: &RaylibThread,
    assets: &Assets,
    base: &Actor,
    out: &Path,
    phases: usize,
    depth: f32,
    facing: Facing,
) -> Result<(), Box<dyn Error>> {
    let cell_w = if depth == 1.0 { 320 } else { 384 };
    let cell_h = if depth == 1.0 { 252 } else { 432 };
    let rows = phases.div_ceil(4);
    let mut target = rl.load_render_texture(thread, cell_w * 4, cell_h * rows as u32)?;
    {
        let mut d = rl.begin_texture_mode(thread, &mut target);
        d.clear_background(BACK);
        for i in 0..phases {
            let phase = i as f32 / phases as f32;
            let x = (i % 4) as f32 * cell_w as f32;
            let y = (i / 4) as f32 * cell_h as f32;
            let mut actor = base.clone();
            actor.action = Action::Walk;
            actor.gait = Gait::Run;
            actor.facing = facing;
            actor.velocity.x = PLAYER_RUN_SPEED * facing.sign();
            actor.velocity.y = 0.0;
            actor.grounded = true;
            actor.stride_distance = phase * assets.locomotion.motion.run_stride_pixels;
            let emphasized = [0.0, 0.125, 0.375].contains(&phase);
            d.draw_rectangle_lines(x as i32, y as i32, cell_w as i32, cell_h as i32, GRID);
            d.draw_text(
                &format!(
                    "phase {phase:.4}   distance {:.2}   {depth:.0}x",
                    actor.stride_distance
                ),
                x as i32 + 12,
                y as i32 + 12,
                16,
                if emphasized { ALERT } else { INK },
            );
            d.draw_text(
                &format!("nominal vx {:+.0} px/s", actor.velocity.x),
                x as i32 + 12,
                y as i32 + 32,
                15,
                INK,
            );
            let support_y = y + cell_h as f32 - 22.0;
            floor(&mut d, x + 8.0, support_y, cell_w as f32 - 16.0);
            pose(
                &mut d,
                assets,
                &actor,
                Vector2::new(x + cell_w as f32 * 0.5, support_y),
                depth,
            );
        }
    }
    let direction = if facing == Facing::Right {
        "right"
    } else {
        "left"
    };
    export(
        &target,
        &out.join(format!("phases-{direction}-{}x.png", depth as u32)),
    )
}

fn focus(
    rl: &mut RaylibHandle,
    thread: &RaylibThread,
    assets: &Assets,
    base: &Actor,
    out: &Path,
    facing: Facing,
) -> Result<(), Box<dyn Error>> {
    let mut target = rl.load_render_texture(thread, 1536, 432)?;
    {
        let mut d = rl.begin_texture_mode(thread, &mut target);
        d.clear_background(BACK);
        for (i, phase) in [None, Some(0.0), Some(0.125), Some(0.375)]
            .into_iter()
            .enumerate()
        {
            let x = i as f32 * 384.0;
            let mut actor = base.clone();
            actor.facing = facing;
            let label = if let Some(phase) = phase {
                actor.action = Action::Walk;
                actor.gait = Gait::Run;
                actor.velocity.x = PLAYER_RUN_SPEED * facing.sign();
                actor.velocity.y = 0.0;
                actor.grounded = true;
                actor.stride_distance = phase * assets.locomotion.motion.run_stride_pixels;
                format!("RUN phase {phase:.3} - 2x")
            } else {
                actor.action = Action::Idle;
                actor.action_ticks = 0;
                "IDLE reference - 2x".into()
            };
            d.draw_text(&label, x as i32 + 12, 12, 18, INK);
            d.draw_text(
                &format!("vx {:+.0} px/s", actor.velocity.x),
                x as i32 + 12,
                34,
                15,
                INK,
            );
            d.draw_rectangle_lines(x as i32, 0, 384, 432, GRID);
            floor(&mut d, x + 8.0, 410.0, 368.0);
            pose(&mut d, assets, &actor, Vector2::new(x + 192.0, 410.0), 2.0);
        }
    }
    let direction = if facing == Facing::Right {
        "right"
    } else {
        "left"
    };
    export(
        &target,
        &out.join(format!("idle-and-reported-phases-{direction}-2x.png")),
    )
}

/// Keeps the shin and both complete boots visible around the reported recovery
/// pose. Sampling stays on the public actor API and uses no private joint data.
fn ankle_recovery(
    rl: &mut RaylibHandle,
    thread: &RaylibThread,
    assets: &Assets,
    base: &Actor,
    out: &Path,
    facing: Facing,
) -> Result<(), Box<dyn Error>> {
    let mut target = rl.load_render_texture(thread, 1536, 864)?;
    {
        let mut d = rl.begin_texture_mode(thread, &mut target);
        d.clear_background(BACK);
        for (i, phase) in RECOVERY_PHASES.into_iter().enumerate() {
            let x = (i % 4) as f32 * 384.0;
            let y = (i / 4) as f32 * 432.0;
            let mut actor = base.clone();
            actor.action = Action::Walk;
            actor.gait = Gait::Run;
            actor.facing = facing;
            actor.velocity.x = PLAYER_RUN_SPEED * facing.sign();
            actor.velocity.y = 0.0;
            actor.grounded = true;
            actor.stride_distance = phase * assets.locomotion.motion.run_stride_pixels;
            d.draw_rectangle_lines(x as i32, y as i32, 384, 432, GRID);
            d.draw_text(
                &format!("recovery phase {phase:.7} - 2x"),
                x as i32 + 12,
                y as i32 + 12,
                17,
                if i == 3 { ALERT } else { INK },
            );
            d.draw_text(
                &format!(
                    "vx {:+.0} px/s{}",
                    actor.velocity.x,
                    if i == 3 { " | reported tick 359" } else { "" }
                ),
                x as i32 + 12,
                y as i32 + 34,
                15,
                INK,
            );
            floor(&mut d, x + 8.0, y + 410.0, 368.0);
            pose(
                &mut d,
                assets,
                &actor,
                Vector2::new(x + 192.0, y + 410.0),
                2.0,
            );
        }
    }
    let direction = if facing == Facing::Right {
        "right"
    } else {
        "left"
    };
    export(
        &target,
        &out.join(format!("ankle-recovery-{direction}-2x.png")),
    )
}

fn movement(tick: u32) -> (f32, &'static str) {
    match tick {
        0..60 => (0.0, "IDLE"),
        60..180 => (1.0, "START / RUN RIGHT"),
        180..240 => (0.0, "STOP"),
        240..360 => (-1.0, "START / RUN LEFT"),
        360..480 => (1.0, "DIRECT REVERSE TO RIGHT"),
        480..540 => (0.0, "STOP"),
        540..660 => (-1.0, "START / RUN LEFT"),
        _ => (0.0, "FINAL STOP"),
    }
}

/// Samples native world composition after ordinary movement has advanced each
/// real camera. Only the final run phase is selected for the anatomical review.
fn world_context(
    rl: &mut RaylibHandle,
    thread: &RaylibThread,
    assets: &ChapterAssets,
    out: &Path,
) -> Result<(), Box<dyn Error>> {
    let mut target = rl.load_render_texture(thread, 1280, 720)?;
    let mut street = Story::default();
    street.hub_origin = assets.common.landscape.map.scene("street").hub_origin;
    street.configure_map(assets.common.landscape.map.prologue.clone());
    // Use the story's public skips to reach normal, controllable street framing.
    for _ in 0..3 {
        street.advance_scene();
    }
    for _ in 0..110 {
        street.tick(CombatInput {
            movement: 1.0,
            ..Default::default()
        });
    }
    street.combat.player.action = Action::Walk;
    street.combat.player.gait = Gait::Run;
    street.combat.player.facing = Facing::Right;
    street.combat.player.velocity.x = PLAYER_RUN_SPEED;
    street.combat.player.velocity.y = 0.0;
    street.combat.player.grounded = true;
    street.combat.player.stride_distance =
        0.125 * assets.common.locomotion.motion.run_stride_pixels;
    {
        let mut d = rl.begin_texture_mode(thread, &mut target);
        render::draw(&mut d, &street, &assets.common, false, false, true);
        render::navigation(&mut d, &assets.common, &street, false, false, 0.0);
    }
    export(&target, &out.join("context-prologue-right-phase-0125.png"))?;

    let mut chapter =
        Chapter::from_checkpoint(chapter::assets::load_world()?, Checkpoint::new(false))?;
    for _ in 0..60 {
        chapter.tick(ChapterInput {
            movement: -1.0,
            ..Default::default()
        });
    }
    chapter.combat.player.action = Action::Walk;
    chapter.combat.player.gait = Gait::Run;
    chapter.combat.player.facing = Facing::Left;
    chapter.combat.player.velocity.x = -PLAYER_RUN_SPEED;
    chapter.combat.player.velocity.y = 0.0;
    chapter.combat.player.grounded = true;
    chapter.combat.player.stride_distance =
        0.375 * assets.common.locomotion.motion.run_stride_pixels;
    {
        let mut d = rl.begin_texture_mode(thread, &mut target);
        chapter::draw(&mut d, assets, &chapter, 0, false);
    }
    export(&target, &out.join("context-chapter-left-phase-0375.png"))?;
    let camera = chapter.camera();
    fs::write(
        out.join("world-context.json"),
        format!(
            r#"{{"resolution":[1280,720],"prologue":{{"image":"context-prologue-right-phase-0125.png","renderer":"engine::render::draw + navigation","phase":0.125,"movement_ticks":110,"player":[{},{}],"velocity_x":{},"camera_left":{}}},"chapter":{{"image":"context-chapter-left-phase-0375.png","renderer":"engine::chapter::draw","checkpoint":"street_start","phase":0.375,"movement_ticks":60,"player":[{},{}],"velocity_x":{},"camera_target":[{},{}],"camera_zoom":{}}}}}"#,
            street.combat.player.position.x,
            street.combat.player.position.y,
            street.combat.player.velocity.x,
            street.camera_left(),
            chapter.player().position.x,
            chapter.player().position.y,
            chapter.player().velocity.x,
            camera.target.x,
            camera.target.y,
            camera.zoom,
        ),
    )?;
    Ok(())
}

fn video(
    rl: &mut RaylibHandle,
    thread: &RaylibThread,
    assets: &Assets,
    mut story: Story,
    out: &Path,
) -> Result<(), Box<dyn Error>> {
    let mut encoder = Command::new("ffmpeg")
        .args([
            "-y",
            "-loglevel",
            "error",
            "-f",
            "rawvideo",
            "-pix_fmt",
            "rgba",
            "-s",
            "1280x900",
            "-r",
            "60",
            "-i",
            "-",
            "-an",
            "-c:v",
            "libx264",
            "-preset",
            "fast",
            "-crf",
            "18",
            "-pix_fmt",
            "yuv420p",
            "-movflags",
            "+faststart",
        ])
        .arg(out.join("start-stop-reverse-60fps.mp4"))
        .stdin(Stdio::piped())
        .spawn()?;
    let mut input = encoder.stdin.take().ok_or("missing FFmpeg stdin")?;
    let mut target = rl.load_render_texture(thread, 1280, 900)?;
    let mut csv = fs::File::create(out.join("timeline.csv"))?;
    writeln!(
        csv,
        "tick,segment,input,x,delta_x,stride_distance,phase,action,facing,gait,velocity_x,speed_ratio"
    )?;
    // Use broad review bounds only to keep this prescribed route away from walls.
    story.combat.set_bounds(-5000.0, 5000.0);
    story.combat.player.position.x = 700.0;
    let idle = story.combat.player.clone();
    for tick in 0..720 {
        let (intent, segment) = movement(tick);
        let before = story.combat.player.position.x;
        let previous_action = story.combat.player.action;
        let previous_facing = story.combat.player.facing;
        story.combat.tick_exploration(CombatInput {
            movement: intent,
            ..Default::default()
        });
        let actor = &story.combat.player;
        let phase = assets
            .locomotion
            .motion
            .phase_for(Gait::Run, actor.stride_distance);
        writeln!(
            csv,
            "{tick},{segment},{intent},{},{},{},{phase},{:?},{:?},{:?},{},{}",
            actor.position.x,
            actor.position.x - before,
            actor.stride_distance,
            actor.action,
            actor.facing,
            actor.gait,
            actor.velocity.x,
            actor.velocity.x.abs() / PLAYER_RUN_SPEED
        )?;
        {
            let mut d = rl.begin_texture_mode(thread, &mut target);
            d.clear_background(BACK);
            d.draw_text(
                &format!(
                    "{segment}  tick {tick:03}  phase {phase:.4}  stride {:.2}  vx {:+.1}  v/max {:.3}",
                    actor.stride_distance,
                    actor.velocity.x,
                    actor.velocity.x.abs() / PLAYER_RUN_SPEED
                ),
                24,
                18,
                22,
                INK,
            );
            d.draw_text(
                "Same authoritative actor at native size / 2x. Floor scrolls with world travel.",
                24,
                50,
                17,
                INK,
            );
            for (depth, y, top) in [(1.0, 315.0, 89), (2.0, 832.0, 416)] {
                d.draw_text(
                    &format!("{depth:.0}x - public actors::rust_scaled"),
                    24,
                    top,
                    18,
                    INK,
                );
                floor(&mut d, 0.0, y, 1280.0);
                let first = ((actor.position.x - 600.0 / depth) / 50.0).floor() as i32;
                for n in first..first + 40 {
                    let x = 530.0 + (n as f32 * 50.0 - actor.position.x) * depth;
                    if (0.0..=930.0).contains(&x) {
                        d.draw_line(x as i32, y as i32, x as i32, y as i32 + 22, GRID);
                    }
                }
                pose(&mut d, assets, actor, Vector2::new(530.0, y), depth);
                let mut reference = idle.clone();
                reference.facing = actor.facing;
                pose(&mut d, assets, &reference, Vector2::new(1110.0, y), depth);
                d.draw_text("idle reference", 1012, y as i32 + 27, 17, INK);
            }
            d.draw_line(0, 393, 1280, 393, GRID);
        }
        if [
            59, 60, 61, 179, 180, 181, 239, 240, 359, 360, 361, 479, 480, 659, 660,
        ]
        .contains(&tick)
        {
            export(&target, &out.join(format!("transition-{tick:03}.png")))?;
        }
        // Stop and turn complete after braking, not at the input's edge. Keep
        // those actual state transitions even if acceleration tuning changes.
        if actor.action != previous_action || actor.facing != previous_facing {
            export(&target, &out.join(format!("state-change-{tick:03}.png")))?;
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
    drop(input);
    if !encoder.wait()?.success() {
        return Err("FFmpeg failed".into());
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = std::env::args().collect();
    let out = Path::new(
        args.get(1)
            .ok_or("usage: capture OUTPUT PHASES [--no-video]")?,
    );
    let phases: usize = args.get(2).map(String::as_str).unwrap_or("16").parse()?;
    if ![12, 16].contains(&phases) {
        return Err("phase count must be 12 or 16".into());
    }
    fs::create_dir_all(out)?;
    let (mut rl, thread) = raylib::init()
        .size(1280, 900)
        .title("Rust run review")
        .build();
    rl.set_window_state(WindowState::default().set_window_hidden(true));
    let chapter_assets = ChapterAssets::load(&mut rl, &thread)?;
    let assets = &chapter_assets.common;
    let story = Story::default();
    fs::write(
        out.join("capture-settings.json"),
        format!(
            r#"{{"harness_version":4,"static_run_speed":{PLAYER_RUN_SPEED},"run_stride_pixels":{},"static_grounded":true,"static_velocity_y":0,"video_uses_real_velocity":true,"world_context_uses_real_camera":true,"ankle_recovery":{{"phases":{:?},"scale":2,"reported_tick":359}}}}"#,
            assets.locomotion.motion.run_stride_pixels, RECOVERY_PHASES,
        ),
    )?;
    for facing in [Facing::Right, Facing::Left] {
        for depth in [1.0, 2.0] {
            sheet(
                &mut rl,
                &thread,
                assets,
                &story.combat.player,
                out,
                phases,
                depth,
                facing,
            )?;
        }
        focus(&mut rl, &thread, assets, &story.combat.player, out, facing)?;
        ankle_recovery(&mut rl, &thread, assets, &story.combat.player, out, facing)?;
    }
    world_context(&mut rl, &thread, &chapter_assets, out)?;
    if !args.iter().any(|arg| arg == "--no-video") {
        video(&mut rl, &thread, assets, story, out)?;
    }
    Ok(())
}
