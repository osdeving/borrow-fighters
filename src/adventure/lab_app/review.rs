//! Captures native pose sheets and one distinct simulated frame per 60 Hz video frame.
//!
//! Reviews preserve a portable copy of the selected sources and stream pixels to
//! FFmpeg. The scripted arena issues ordinary controls, without forcing outcomes.

use super::{
    ActorAssets, BACK, Event, Facing, HEIGHT, INK, Input, Options, Session, Simulation, WIDTH,
    label,
};
use crate::adventure::{
    engine::{
        capture::export,
        production::{
            actors::draw_actor,
            models3d::{self, Catalog, Models3d},
        },
    },
    production::{Action, Team},
};
use raylib::prelude::*;
use std::{
    collections::BTreeSet,
    error::Error,
    fs,
    io::Write,
    path::{Path, PathBuf},
    process::{Child, ChildStdin, Command, Stdio},
};

pub(super) struct Recorder {
    directory: PathBuf,
    process: Child,
    stdin: Option<ChildStdin>,
    trace: fs::File,
    pixels: Vec<u8>,
}

impl Recorder {
    pub fn new(
        directory: &Path,
        options: &Options,
        session: &Session,
    ) -> Result<Self, Box<dyn Error>> {
        if directory.is_dir() && fs::read_dir(directory)?.next().is_some() {
            return Err(format!("review directory is not empty: {}", directory.display()).into());
        }
        fs::create_dir_all(directory)?;
        preserve_sources(
            &options.actor,
            &session.assets,
            &directory.join("source/player"),
        )?;
        if let (Some(path), Some(assets)) = (&options.enemy, &session.enemy_assets) {
            preserve_sources(path, assets, &directory.join("source/enemy"))?;
        }
        let model_snapshot = session
            .model_catalog
            .as_ref()
            .map(|catalog| {
                preserve_models(catalog, &directory.join("source/models"))
                    .map(|name| format!("source/models/{name}"))
            })
            .transpose()?;
        let rendered_models: Vec<_> = session
            .model_actors
            .iter()
            .filter(|id| {
                session
                    .models
                    .as_ref()
                    .is_some_and(|models| models.contains(id))
            })
            .collect();
        fs::write(
            directory.join("invocation.json"),
            serde_json::to_vec_pretty(&serde_json::json!({
                "schema_version":1,"actor":options.actor,"enemy":options.enemy,"clip":options.clip,
                "phase":options.phase,"requested_frames":options.frames,"simulation_hz":60,"video_fps":60,
                "script":"idle60/run90/stop60/reverse120/stop60/jump60/arena",
                "source_snapshot":"source/player/character.json",
                "models_3d":!rendered_models.is_empty(),
                "model_actors":rendered_models,
                "model_catalog_snapshot":model_snapshot,
                "renderer":if rendered_models.is_empty() {"engine::production::actors::draw_actor"} else {"engine::production::models3d"}
            }))?,
        )?;
        let mut process = Command::new("ffmpeg")
            .args([
                "-n",
                "-loglevel",
                "error",
                "-f",
                "rawvideo",
                "-pixel_format",
                "rgba",
                "-video_size",
                "1280x720",
                "-framerate",
                "60",
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
            .arg(directory.join("lab-simulation-60fps.mp4"))
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .spawn()?;
        let stdin = process.stdin.take();
        let trace = fs::File::create(directory.join("telemetry.jsonl"))?;
        Ok(Self {
            directory: directory.into(),
            process,
            stdin,
            trace,
            pixels: Vec::with_capacity((WIDTH * HEIGHT * 4) as usize),
        })
    }

    pub fn frame(
        &mut self,
        target: &RenderTexture2D,
        session: &Session,
        frame: u32,
        input: Input,
        events: &[Event],
    ) -> Result<(), Box<dyn Error>> {
        let image = target.texture().load_image()?;
        let colors = image.get_image_data();
        if colors.len() != (WIDTH * HEIGHT) as usize {
            return Err("lab framebuffer dimensions changed".into());
        }
        self.pixels.clear();
        self.pixels
            .extend(colors.iter().flat_map(|c| [c.r, c.g, c.b, c.a]));
        self.stdin
            .as_mut()
            .ok_or("review pipe closed")?
            .write_all(&self.pixels)?;
        let sim = &session.sim;
        writeln!(
            self.trace,
            "{}",
            serde_json::json!({"frame":frame,"tick":sim.ticks,"arena":session.arena,
            "input":{"movement":input.movement,"jump":input.jump,"light":input.light,"kick":input.kick,"spin":input.spin,"linker":input.linker,"guard":input.guard},
            "actors":sim.actors().iter().map(|a|serde_json::json!({"id":a.id.0,"character":a.character,"hp":a.hp,"x":a.position.x,"y":a.position.y,"vx":a.velocity.x,"clip":a.clip_id(),"action_tick":a.action_ticks,"distance":a.stride_distance,"facing":format!("{:?}",a.facing)})).collect::<Vec<_>>(),
            "events":events.iter().map(|e|format!("{e:?}")).collect::<Vec<_>>(),"projectiles":sim.projectiles().len(),"outcome":format!("{:?}",sim.outcome())})
        )?;
        if [
            0, 60, 75, 149, 180, 210, 218, 329, 390, 411, 450, 480, 600, 780, 1079,
        ]
        .contains(&frame)
        {
            export(
                target,
                &self.directory.join(format!("simulation-{frame:04}.png")),
            )?;
        }
        Ok(())
    }

    pub fn finish(mut self, frames: u32, session: &Session) -> Result<(), Box<dyn Error>> {
        self.stdin.take();
        self.trace.flush()?;
        if !self.process.wait()?.success() {
            return Err("FFmpeg failed while capturing actor lab".into());
        }
        let zoom_sheets = ["idle", "run", "spin"]
            .iter()
            .filter(|id| session.assets.clips.clips.contains_key(**id))
            .count()
            * 2;
        fs::write(
            self.directory.join("result.json"),
            serde_json::to_vec_pretty(&serde_json::json!({
                "schema_version":1,"exit":"complete","frames":frames,"simulation_ticks":session.sim.ticks,
                "video_fps":60,"duration_seconds":frames as f64/60.0,"video":"lab-simulation-60fps.mp4",
                "pose_sheets":session.assets.clips.clips.len()*2,
                "zoom_sheets":zoom_sheets,
                "outcome":format!("{:?}",session.sim.outcome())
            }))?,
        )?;
        Ok(())
    }
}

impl Drop for Recorder {
    fn drop(&mut self) {
        self.stdin.take();
        if self.process.try_wait().ok().flatten().is_none() {
            let _ = self.process.kill();
            let _ = self.process.wait();
        }
    }
}

/// Every referenced image is copied once, preserving relative paths for replay.
fn preserve_sources(
    path: &Path,
    assets: &ActorAssets,
    output: &Path,
) -> Result<(), Box<dyn Error>> {
    let root = path.parent().ok_or("source manifest has no directory")?;
    let mut files: BTreeSet<_> = [
        assets.pack.character.combat.clone(),
        assets.pack.character.rig.clone(),
        assets.pack.character.clips.clone(),
    ]
    .into_iter()
    .collect();
    files.extend(assets.rig.attachments.values().map(|a| a.image.clone()));
    fs::create_dir_all(output)?;
    fs::copy(path, output.join("character.json"))?;
    for relative in files {
        let target = output.join(&relative);
        fs::create_dir_all(target.parent().ok_or("invalid snapshot path")?)?;
        fs::copy(root.join(relative), target)?;
    }
    Ok(())
}

/// Copy declared GLB paths so the standalone --models catalog remains replayable.
fn preserve_models(path: &Path, output: &Path) -> Result<String, Box<dyn Error>> {
    let root = path.parent().ok_or("model catalog has no directory")?;
    let name = path
        .file_name()
        .and_then(|value| value.to_str())
        .ok_or("invalid catalog filename")?;
    let catalog = Catalog::load(path)?;
    fs::create_dir_all(output)?;
    fs::copy(path, output.join(name))?;
    for entry in catalog.entries.values() {
        let target = output.join(&entry.file);
        fs::create_dir_all(target.parent().ok_or("model snapshot has no parent")?)?;
        fs::copy(root.join(&entry.file), target)?;
    }
    Ok(name.into())
}

pub(super) fn sheets(
    rl: &mut RaylibHandle,
    thread: &RaylibThread,
    assets: &ActorAssets,
    models: Option<&Models3d>,
    font: &Font,
    out: &Path,
) -> Result<(), Box<dyn Error>> {
    for (index, (id, clip)) in assets.clips.clips.iter().enumerate() {
        let name: String = id
            .chars()
            .take(64)
            .map(|c| {
                if c.is_ascii_alphanumeric() || c == '-' {
                    c
                } else {
                    '_'
                }
            })
            .collect();
        for facing in [Facing::Right, Facing::Left] {
            let mut target = rl.load_render_texture(thread, 1280, 960)?;
            {
                let mut d = rl.begin_texture_mode(thread, &mut target);
                d.clear_background(BACK);
                for cell in 0..12 {
                    let phase = cell as f32 / 11.0;
                    let ticks = phase * clip.duration_ticks as f32;
                    let distance = phase * clip.stride_pixels.unwrap_or(0.0);
                    let sample = assets.clips.sample(id, ticks, distance);
                    let x = (cell % 4) as f32 * 320.0;
                    let y = (cell / 4) as f32 * 320.0;
                    let scale = (245.0 / assets.rig.height).min(1.0);
                    d.draw_rectangle_lines(
                        x as i32,
                        y as i32,
                        320,
                        320,
                        Color::new(80, 95, 111, 255),
                    );
                    label(
                        &mut d,
                        font,
                        &format!("{id}  {phase:.3}  {scale:.2}x"),
                        x + 12.0,
                        y + 12.0,
                        18.0,
                        INK,
                    );
                    d.draw_line(
                        (x + 8.0) as i32,
                        (y + 296.0) as i32,
                        (x + 312.0) as i32,
                        (y + 296.0) as i32,
                        Color::GRAY,
                    );
                    let drawn = models.is_some_and(|models| {
                        models.draw_viewport(
                            &mut d,
                            models3d::actor_sample(
                                assets,
                                id,
                                ticks,
                                [x + 160., y + 296., 0.],
                                facing.sign(),
                                sample.pose.yaw,
                                scale,
                            ),
                            [1280., 960.],
                        )
                    });
                    if !drawn {
                        draw_actor(
                            &mut d,
                            assets,
                            sample.pose,
                            id,
                            ticks,
                            [x + 160.0, y + 296.0],
                            facing,
                            scale,
                            false,
                        );
                    }
                }
            }
            let direction = if facing == Facing::Right {
                "right"
            } else {
                "left"
            };
            export(
                &target,
                &out.join(format!("clip-{index:02}-{name}-{direction}.png")),
            )?;
        }
    }
    for id in ["idle", "run", "spin"] {
        if assets.clips.clips.contains_key(id) {
            for facing in [Facing::Right, Facing::Left] {
                zoom_sheet(rl, thread, assets, models, font, out, id, facing)?;
            }
        }
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn zoom_sheet(
    rl: &mut RaylibHandle,
    thread: &RaylibThread,
    assets: &ActorAssets,
    models: Option<&Models3d>,
    font: &Font,
    out: &Path,
    id: &str,
    facing: Facing,
) -> Result<(), Box<dyn Error>> {
    let clip = &assets.clips.clips[id];
    let mut target = rl.load_render_texture(thread, 1536, 960)?;
    {
        let mut d = rl.begin_texture_mode(thread, &mut target);
        d.clear_background(BACK);
        for cell in 0..8 {
            let phase = cell as f32 / 8.0;
            let ticks = phase * clip.duration_ticks as f32;
            let sample = assets
                .clips
                .sample(id, ticks, phase * clip.stride_pixels.unwrap_or(0.0));
            let x = (cell % 4) as f32 * 384.0;
            let y = (cell / 4) as f32 * 480.0;
            let scale = (420.0 / assets.rig.height).min(2.0);
            d.draw_rectangle_lines(x as i32, y as i32, 384, 480, Color::new(80, 95, 111, 255));
            label(
                &mut d,
                font,
                &format!("{id}  {phase:.3}  {scale:.2}x"),
                x + 12.0,
                y + 12.0,
                20.0,
                INK,
            );
            d.draw_line(
                (x + 8.0) as i32,
                (y + 458.0) as i32,
                (x + 376.0) as i32,
                (y + 458.0) as i32,
                Color::GRAY,
            );
            let drawn = models.is_some_and(|models| {
                models.draw_viewport(
                    &mut d,
                    models3d::actor_sample(
                        assets,
                        id,
                        ticks,
                        [x + 192., y + 458., 0.],
                        facing.sign(),
                        sample.pose.yaw,
                        scale,
                    ),
                    [1536., 960.],
                )
            });
            if !drawn {
                draw_actor(
                    &mut d,
                    assets,
                    sample.pose,
                    id,
                    ticks,
                    [x + 192.0, y + 458.0],
                    facing,
                    scale,
                    false,
                );
            }
        }
    }
    let direction = if facing == Facing::Right {
        "right"
    } else {
        "left"
    };
    export(&target, &out.join(format!("zoom-{id}-{direction}.png")))
}

pub(super) fn input(sim: &Simulation, frame: u32) -> Input {
    match frame {
        0..60 | 150..210 | 330..390 => return Input::default(),
        60..150 => {
            return Input {
                movement: 1.0,
                ..Input::default()
            };
        }
        210..330 => {
            return Input {
                movement: -1.0,
                ..Input::default()
            };
        }
        390..450 => {
            return Input {
                movement: 1.0,
                jump: frame == 390,
                ..Input::default()
            };
        }
        _ => {}
    }
    let player = sim.player();
    let Some(enemy) = sim
        .actors()
        .iter()
        .filter(|a| a.team == Team::Enemy && a.hp > 0)
        .min_by(|a, b| {
            (a.position.x - player.position.x)
                .abs()
                .total_cmp(&(b.position.x - player.position.x).abs())
        })
    else {
        return Input::default();
    };
    let delta = enemy.position.x - player.position.x;
    if let Some(id) = &player.move_id {
        let current = &sim.content.moves[id];
        return Input {
            light: current.combo_window.is_some_and(|[start, end]| {
                player.action_ticks + 2 >= start && player.action_ticks + 1 < end
            }),
            ..Input::default()
        };
    }
    if player.action == Action::Hurt {
        return Input {
            guard: true,
            ..Input::default()
        };
    }
    let loadout = &sim.content.characters[&player.character].loadout;
    let movement = if delta.abs() > 85.0 || delta.signum() != player.facing.sign() {
        delta.signum()
    } else {
        0.0
    };
    if delta.abs() > 175.0 {
        return Input {
            movement,
            linker: loadout.linker.is_some() && frame.is_multiple_of(90),
            ..Input::default()
        };
    }
    if delta.abs() > 95.0 {
        return Input {
            movement,
            ..Input::default()
        };
    }
    let spin = loadout.spin.is_some() && frame / 90 % 3 == 1;
    let kick = !spin && loadout.kick.is_some() && frame / 90 % 3 == 2;
    Input {
        movement,
        light: !spin && !kick,
        kick,
        spin,
        ..Input::default()
    }
}

#[cfg(test)]
mod model_snapshot_tests {
    use super::*;

    #[test]
    fn preserved_catalog_and_nested_model_survive_removal_of_original_sources() {
        let root =
            std::env::temp_dir().join(format!("borrow-model-snapshot-{}", std::process::id()));
        let source = root.join("original");
        let snapshot = root.join("review/source/models");
        fs::create_dir_all(source.join("cast")).unwrap();
        let catalog = source.join("humans.json");
        fs::write(&catalog, r#"{"schema_version":1,"entries":{"cpp":{"file":"cast/cpp.glb","height_m":1.764,"animation_fps":60,"clips":{"idle":"idle"}}}}"#).unwrap();
        fs::write(source.join("cast/cpp.glb"), b"preserved GLB bytes").unwrap();
        assert_eq!(preserve_models(&catalog, &snapshot).unwrap(), "humans.json");
        fs::remove_dir_all(&source).unwrap();
        let copied = Catalog::load(&snapshot.join("humans.json")).unwrap();
        assert_eq!(
            fs::read(snapshot.join(&copied.entries["cpp"].file)).unwrap(),
            b"preserved GLB bytes"
        );
        fs::remove_dir_all(root).unwrap();
    }
}
