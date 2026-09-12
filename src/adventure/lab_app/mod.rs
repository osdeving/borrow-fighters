//! Hosts the external production laboratory without loading a campaign or street.
//!
//! The tool loads the same character packs and renderer as gameplay. Preview
//! scrubbing and arena controls never create separate animation or damage rules.

mod options;
mod review;
#[cfg(test)]
mod tests;

use crate::{
    adventure::{
        engine::production::{
            actors::{draw_actor, draw_projectile},
            assets::{ActorAssets, ActorContent},
        },
        production::{
            ActorId, Bounds, CombatCatalog, EnemySpawn, Event, Facing, Input, Simulation, Team,
            animation::Animator,
        },
    },
    runtime_paths::asset_path,
};
use options::Options;
use raylib::prelude::*;
use std::{collections::BTreeMap, error::Error, fs, path::Path, sync::Arc, time::Instant};

const WIDTH: i32 = 1280;
const HEIGHT: i32 = 720;
const FLOOR: f32 = 565.0;
const BACK: Color = Color::new(29, 35, 45, 255);
const INK: Color = Color::new(232, 236, 240, 255);
const MUTED: Color = Color::new(162, 178, 195, 255);

struct Labels(BTreeMap<String, String>);
impl Labels {
    fn load() -> Result<Self, Box<dyn Error>> {
        let entries: BTreeMap<String, String> = serde_json::from_str(&fs::read_to_string(
            asset_path("assets/adventure/production-lab.json"),
        )?)?;
        for id in [
            "title",
            "preview",
            "arena",
            "paused",
            "playing",
            "tools",
            "controls",
            "preview_controls",
            "arena_controls",
            "reloaded",
            "reloaded_arena",
            "reload_failed",
            "pose",
            "boxes",
            "source",
            "dummy",
        ] {
            if entries.get(id).is_none_or(|s| s.trim().is_empty()) {
                return Err(format!("missing lab text {id}").into());
            }
        }
        Ok(Self(entries))
    }
    fn get(&self, id: &str) -> &str {
        &self.0[id]
    }
}

struct Session {
    assets: ActorAssets,
    enemy_assets: Option<ActorAssets>,
    sim: Simulation,
    animators: BTreeMap<ActorId, Animator>,
    clip: String,
    preview_ticks: f32,
    facing: Facing,
    arena: bool,
    paused: bool,
    debug: bool,
}

impl Session {
    fn load(
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        options: &Options,
        clip: &str,
    ) -> Result<Self, Box<dyn Error>> {
        let assets = ActorAssets::load(rl, thread, &options.actor)?;
        if !assets.clips.clips.contains_key(clip) {
            return Err(format!("character has no clip {clip}").into());
        }
        let enemy_assets = options
            .enemy
            .as_ref()
            .map(|path| ActorAssets::load(rl, thread, path))
            .transpose()?;
        let mut packs = vec![assets.pack.clone()];
        if let Some(enemy) = &enemy_assets {
            if enemy.pack.character.id == assets.pack.character.id {
                return Err(
                    "--enemy must have a distinct character ID; omit it to use a training copy"
                        .into(),
                );
            }
            packs.push(enemy.pack.clone());
        }
        let catalog = Arc::new(CombatCatalog::new(packs)?);
        let sim = Simulation::new(
            catalog,
            Bounds {
                left: 0.0,
                right: 3000.0,
                floor_y: FLOOR,
            },
            &assets.pack.character.id,
            500.0,
        )?;
        let preview_ticks = options.phase * assets.clips.clips[clip].duration_ticks as f32;
        Ok(Self {
            assets,
            enemy_assets,
            sim,
            animators: BTreeMap::new(),
            clip: clip.into(),
            preview_ticks,
            facing: Facing::Right,
            arena: false,
            paused: true,
            debug: false,
        })
    }
    fn character_assets(&self, id: &str) -> &ActorAssets {
        self.enemy_assets
            .as_ref()
            .filter(|a| a.pack.character.id == id)
            .unwrap_or(&self.assets)
    }
    fn set_arena(&mut self, enabled: bool) -> Result<(), String> {
        self.arena = enabled;
        self.animators.clear();
        if enabled {
            let id = self
                .enemy_assets
                .as_ref()
                .unwrap_or(&self.assets)
                .pack
                .character
                .id
                .clone();
            self.sim.begin_encounter(&[EnemySpawn {
                character: id,
                x: self.sim.player().position.x + 240.0,
                facing: Facing::Left,
            }])?;
        } else {
            self.sim.clear_encounter();
        }
        Ok(())
    }
    fn tick(&mut self, input: Input, opponent_attack: bool) -> Vec<Event> {
        let events = if self.arena && opponent_attack {
            let mut controls = vec![(self.sim.player_id(), input)];
            for actor in self.sim.actors().iter().filter(|a| a.team == Team::Enemy) {
                controls.push((
                    actor.id,
                    Input {
                        light: true,
                        ..Input::default()
                    },
                ));
            }
            self.sim.tick_with_controls(&controls)
        } else {
            self.sim.tick(input)
        };
        for actor in self.sim.actors() {
            let assets = if let Some(enemy) = self
                .enemy_assets
                .as_ref()
                .filter(|a| a.pack.character.id == actor.character)
            {
                enemy
            } else {
                &self.assets
            };
            self.animators
                .entry(actor.id)
                .or_insert_with(|| Animator::new(&assets.clips))
                .tick(
                    &assets.clips,
                    actor.clip_id(),
                    actor.action_ticks as f32,
                    actor.stride_distance,
                );
        }
        events
    }
    fn preview_phase(&self) -> f32 {
        let clip = &self.assets.clips.clips[&self.clip];
        let phase = self.preview_ticks / clip.duration_ticks as f32;
        if clip.looping {
            phase.rem_euclid(1.0)
        } else {
            phase.clamp(0.0, 1.0)
        }
    }
    fn step_preview(&mut self, amount: f32) {
        let duration = self.assets.clips.clips[&self.clip].duration_ticks as f32;
        self.preview_ticks = if self.assets.clips.clips[&self.clip].looping {
            (self.preview_ticks + amount).rem_euclid(duration)
        } else {
            (self.preview_ticks + amount).clamp(0.0, duration)
        };
    }
}

/// Runs the external tool, or validates its chosen manifest without a graphics context.
pub fn run(args: impl IntoIterator<Item = String>) -> Result<(), Box<dyn Error>> {
    let options = Options::parse(args)?;
    if options.help {
        println!(
            "borrow-actor-lab [--actor character.json] [--enemy character.json] [--clip ID] [--phase 0..1] [--frames N] [--hidden] [--review NEW_DIRECTORY] [--validate]\nSpace pauses; Tab selects clip; Left/Right step preview; Enter switches arena; F3 boxes; F5 reloads; R resets. Arena: A/D move, W jump, J light, V kick, K spin, L Linker, Q guard, I opponent attack."
        );
        return Ok(());
    }
    if options.validate {
        raylib::logging::set_trace_log(TraceLogLevel::LOG_NONE);
    }
    let content = ActorContent::load(&options.actor)?;
    if !content.clips.clips.contains_key(&options.clip) {
        return Err(format!("character has no clip {}", options.clip).into());
    }
    if options.validate {
        let enemy = options
            .enemy
            .as_ref()
            .map(|p| ActorContent::load(p))
            .transpose()?;
        if enemy
            .as_ref()
            .is_some_and(|e| e.pack.character.id == content.pack.character.id)
        {
            return Err(
                "--enemy must have a distinct character ID; omit it to use a training copy".into(),
            );
        }
        let mut packs = vec![content.pack.clone()];
        if let Some(enemy) = &enemy {
            packs.push(enemy.pack.clone());
        }
        CombatCatalog::new(packs)?;
        println!(
            "{}",
            serde_json::to_string_pretty(
                &serde_json::json!({"schema_version":1,"tick_hz":60,"character":content.pack.character.id,"method":content.rig.method,"height":content.rig.height,"clips":content.clips.clips.keys().collect::<Vec<_>>(),"moves":content.pack.combat.moves.iter().map(|m|&m.id).collect::<Vec<_>>(),"image_extents":content.image_extents,"enemy":enemy.map(|e|e.pack.character.id)})
            )?
        );
        return Ok(());
    }
    let labels = Labels::load()?;
    let mut builder = raylib::init();
    builder
        .size(WIDTH, HEIGHT)
        .title(labels.get("title"))
        .msaa_4x();
    if options.hidden {
        builder.hidden();
    }
    let (mut rl, thread) = builder.build();
    rl.set_target_fps(if options.frames.is_some() { 0 } else { 60 });
    let glyphs: String = (32..=591)
        .filter_map(char::from_u32)
        .chain("←→".chars())
        .collect();
    let font = rl.load_font_ex(
        &thread,
        &asset_path("assets/adventure/fonts/Barlow-Regular.ttf").to_string_lossy(),
        40,
        Some(&glyphs),
    )?;
    let mut session = Session::load(&mut rl, &thread, &options, &options.clip)?;
    let mut target = rl.load_render_texture(&thread, WIDTH as u32, HEIGHT as u32)?;
    let mut recorder = if let Some(directory) = &options.review {
        let recorder = review::Recorder::new(directory, &options, &session)?;
        review::sheets(&mut rl, &thread, &session.assets, &font, directory)?;
        Some(recorder)
    } else {
        None
    };
    let mut frame = 0_u32;
    let mut accumulator = 0.0_f32;
    let mut last = Instant::now();
    let mut pending = Input::default();
    let mut pending_opponent = false;
    let mut notice = String::new();
    while !rl.window_should_close() && options.frames.is_none_or(|limit| frame < limit) {
        let reviewing = recorder.is_some();
        let now = Instant::now();
        let elapsed = if options.frames.is_some() {
            1.0 / 60.0
        } else {
            now.duration_since(last).as_secs_f32().min(0.25)
        };
        last = now;
        let mut single_step = false;
        if !reviewing {
            use KeyboardKey::*;
            if rl.is_key_pressed(KEY_SPACE) {
                session.paused = !session.paused;
            }
            if rl.is_key_pressed(KEY_F3) {
                session.debug = !session.debug;
            }
            if rl.is_key_pressed(KEY_ENTER) {
                session.set_arena(!session.arena)?;
                session.paused = false;
            }
            if rl.is_key_pressed(KEY_F) {
                session.facing = if session.facing == Facing::Right {
                    Facing::Left
                } else {
                    Facing::Right
                };
            }
            if rl.is_key_pressed(KEY_TAB) {
                let clips: Vec<_> = session.assets.clips.clips.keys().cloned().collect();
                let index = clips.iter().position(|id| *id == session.clip).unwrap_or(0);
                session.clip = clips[(index + 1) % clips.len()].clone();
                session.preview_ticks = 0.0;
            }
            if rl.is_key_pressed(KEY_R) {
                session.sim.reset_player(500.0)?;
                session.set_arena(session.arena)?;
                session.preview_ticks = 0.0;
            }
            if rl.is_key_pressed(KEY_F5) {
                match Session::load(&mut rl, &thread, &options, &session.clip).and_then(
                    |mut candidate| {
                        candidate.set_arena(session.arena)?;
                        candidate.preview_ticks = reloaded_preview_ticks(
                            session.preview_ticks,
                            &session.assets.clips.clips[&session.clip],
                            &candidate.assets.clips.clips[&candidate.clip],
                        );
                        candidate.paused = session.paused;
                        candidate.debug = session.debug;
                        candidate.facing = session.facing;
                        Ok(candidate)
                    },
                ) {
                    Ok(candidate) => {
                        session = candidate;
                        notice = labels
                            .get(if session.arena {
                                "reloaded_arena"
                            } else {
                                "reloaded"
                            })
                            .into();
                    }
                    Err(error) => {
                        eprintln!("{error}");
                        notice = format!("{}: {error}", labels.get("reload_failed"));
                    }
                }
            }
            if session.paused {
                if rl.is_key_pressed(KEY_RIGHT) {
                    if session.arena {
                        single_step = true;
                    } else {
                        session.step_preview(1.0);
                    }
                }
                if !session.arena && rl.is_key_pressed(KEY_LEFT) {
                    session.step_preview(-1.0);
                }
                if !session.arena && rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
                    let point = rl.get_mouse_position();
                    if (610.0..=636.0).contains(&point.y) && (30.0..=1250.0).contains(&point.x) {
                        session.preview_ticks = ((point.x - 30.0) / 1220.0)
                            * session.assets.clips.clips[&session.clip].duration_ticks as f32;
                    }
                }
            }
            merge(&mut pending, keyboard(&rl));
            pending_opponent |= rl.is_key_pressed(KEY_I);
        }
        let mut events = vec![];
        let mut applied = Input::default();
        if reviewing {
            if frame == 450 {
                session.set_arena(true)?;
            }
            applied = review::input(&session.sim, frame);
            events = session.tick(applied, false);
        } else if !session.paused || single_step {
            accumulator += if single_step { 1.0 / 60.0 } else { elapsed };
            while accumulator + 0.000001 >= 1.0 / 60.0 {
                if session.arena {
                    applied = pending;
                    events.extend(session.tick(pending, pending_opponent));
                } else {
                    session.step_preview(1.0);
                }
                pending = Input {
                    movement: pending.movement,
                    guard: pending.guard,
                    ..Input::default()
                };
                pending_opponent = false;
                accumulator = (accumulator - 1.0 / 60.0).max(0.0);
            }
        } else {
            accumulator = 0.0;
            pending = Input::default();
            pending_opponent = false;
        }
        {
            let mut d = rl.begin_texture_mode(&thread, &mut target);
            draw(
                &mut d,
                &session,
                &font,
                &labels,
                &options.actor,
                &notice,
                reviewing,
            );
        }
        if let Some(recorder) = &mut recorder {
            recorder.frame(&target, &session, frame, applied, &events)?;
        }
        {
            let mut d = rl.begin_drawing(&thread);
            d.clear_background(BACK);
            d.draw_texture_rec(
                target.texture(),
                Rectangle::new(0.0, 0.0, WIDTH as f32, -(HEIGHT as f32)),
                Vector2::zero(),
                Color::WHITE,
            );
        }
        frame += 1;
    }
    if let Some(recorder) = recorder {
        recorder.finish(frame, &session)?;
    }
    Ok(())
}

/// Reload uses the current scrub position, including keyboard steps after launch.
fn reloaded_preview_ticks(
    previous_ticks: f32,
    previous: &crate::adventure::production::animation::Clip,
    candidate: &crate::adventure::production::animation::Clip,
) -> f32 {
    if previous.duration_ticks == candidate.duration_ticks {
        return previous_ticks;
    }
    let phase = previous_ticks / previous.duration_ticks as f32;
    let phase = if previous.looping {
        phase.rem_euclid(1.0)
    } else {
        phase.clamp(0.0, 1.0)
    };
    phase * candidate.duration_ticks as f32
}

fn keyboard(rl: &RaylibHandle) -> Input {
    use KeyboardKey::*;
    Input {
        movement: f32::from(rl.is_key_down(KEY_D)) - f32::from(rl.is_key_down(KEY_A)),
        jump: rl.is_key_pressed(KEY_W),
        light: rl.is_key_pressed(KEY_J),
        kick: rl.is_key_pressed(KEY_V),
        spin: rl.is_key_pressed(KEY_K),
        linker: rl.is_key_pressed(KEY_L),
        guard: rl.is_key_down(KEY_Q),
    }
}
fn merge(pending: &mut Input, input: Input) {
    pending.movement = input.movement;
    pending.guard = input.guard;
    pending.jump |= input.jump;
    pending.light |= input.light;
    pending.kick |= input.kick;
    pending.spin |= input.spin;
    pending.linker |= input.linker;
}
fn label(
    d: &mut impl RaylibDraw,
    font: &Font,
    text: &str,
    x: f32,
    y: f32,
    size: f32,
    color: Color,
) {
    d.draw_text_ex(font, text, Vector2::new(x, y), size, 0.5, color);
}

fn draw(
    d: &mut impl RaylibDraw,
    s: &Session,
    font: &Font,
    text: &Labels,
    path: &Path,
    notice: &str,
    reviewing: bool,
) {
    d.clear_background(BACK);
    d.draw_rectangle(0, 95, WIDTH, 490, Color::new(43, 51, 64, 255));
    for x in (0..WIDTH).step_by(80) {
        d.draw_line(x, 95, x, 585, Color::new(51, 61, 75, 255));
    }
    d.draw_line(
        0,
        FLOOR as i32,
        WIDTH,
        FLOOR as i32,
        Color::new(102, 125, 147, 255),
    );
    label(
        d,
        font,
        &format!("{}  /  {}", text.get("title"), s.assets.pack.character.id),
        28.0,
        18.0,
        28.0,
        INK,
    );
    label(
        d,
        font,
        if s.arena {
            text.get("arena")
        } else if reviewing {
            text.get("controls")
        } else {
            text.get("preview")
        },
        28.0,
        57.0,
        20.0,
        MUTED,
    );
    label(
        d,
        font,
        if s.paused && !reviewing {
            text.get("paused")
        } else {
            text.get("playing")
        },
        1050.0,
        26.0,
        20.0,
        MUTED,
    );
    if s.arena || reviewing {
        let camera = (s.sim.player().position.x - 470.0).clamp(0.0, 1720.0);
        for actor in s.sim.actors() {
            let assets = s.character_assets(&actor.character);
            let pose = s.animators.get(&actor.id).map_or_else(
                || {
                    assets
                        .clips
                        .sample(
                            actor.clip_id(),
                            actor.action_ticks as f32,
                            actor.stride_distance,
                        )
                        .pose
                },
                |a| a.pose,
            );
            let at = [actor.position.x - camera, actor.position.y];
            draw_actor(
                d,
                assets,
                pose,
                actor.clip_id(),
                actor.action_ticks as f32,
                at,
                actor.facing,
                1.0,
                s.debug,
            );
            label(
                d,
                font,
                &format!(
                    "{} #{}  {}/{}  {}:{}",
                    actor.character,
                    actor.id.0,
                    actor.hp,
                    actor.max_hp,
                    actor.clip_id(),
                    actor.action_ticks
                ),
                at[0] - 65.0,
                at[1]
                    - assets.rig.height
                    - if actor.team == Team::Player {
                        32.0
                    } else {
                        58.0
                    },
                17.0,
                INK,
            );
            if s.debug {
                for rect in s.sim.hurtboxes(actor.id) {
                    d.draw_rectangle_lines_ex(
                        Rectangle::new(rect.x - camera, rect.y, rect.width, rect.height),
                        1.0,
                        Color::LIME,
                    );
                }
                for rect in s.sim.hitboxes(actor.id) {
                    d.draw_rectangle_lines_ex(
                        Rectangle::new(rect.x - camera, rect.y, rect.width, rect.height),
                        2.0,
                        Color::RED,
                    );
                }
            }
        }
        for projectile in s.sim.projectiles() {
            if let Some(owner) = s.sim.actor(projectile.owner) {
                draw_projectile(
                    d,
                    s.character_assets(&owner.character),
                    &projectile.visual_id,
                    [projectile.position.x - camera, projectile.position.y],
                    projectile.facing,
                    s.sim.ticks as f32,
                );
            }
        }
        label(
            d,
            font,
            &format!("tick {}  {:?}", s.sim.ticks, s.sim.outcome()),
            28.0,
            102.0,
            18.0,
            MUTED,
        );
    } else {
        let c = &s.assets.clips.clips[&s.clip];
        let distance = c.stride_pixels.unwrap_or(0.0) * s.preview_ticks / c.duration_ticks as f32;
        let sample = s.assets.clips.sample(&s.clip, s.preview_ticks, distance);
        draw_actor(
            d,
            &s.assets,
            sample.pose,
            &s.clip,
            s.preview_ticks,
            [400.0, FLOOR],
            s.facing,
            1.0,
            s.debug,
        );
        let large = (380.0 / s.assets.rig.height).min(2.0);
        draw_actor(
            d,
            &s.assets,
            sample.pose,
            &s.clip,
            s.preview_ticks,
            [880.0, FLOOR],
            s.facing,
            large,
            s.debug,
        );
        label(
            d,
            font,
            &format!(
                "{}  {}  {:.3}  /  {} ticks",
                text.get("pose"),
                s.clip,
                s.preview_phase(),
                c.duration_ticks
            ),
            28.0,
            104.0,
            22.0,
            INK,
        );
        label(d, font, "1x", 390.0, 575.0, 17.0, MUTED);
        label(d, font, &format!("{large:.2}x"), 860.0, 575.0, 17.0, MUTED);
        d.draw_rectangle(30, 616, 1220, 10, Color::new(81, 95, 111, 255));
        d.draw_rectangle(
            30,
            616,
            (1220.0 * s.preview_phase()) as i32,
            10,
            Color::SKYBLUE,
        );
    }
    label(d, font, text.get("tools"), 28.0, 648.0, 18.0, INK);
    label(
        d,
        font,
        text.get(if s.arena {
            "arena_controls"
        } else {
            "preview_controls"
        }),
        28.0,
        675.0,
        16.0,
        MUTED,
    );
    if !notice.is_empty() {
        label(
            d,
            font,
            &notice.chars().take(150).collect::<String>(),
            28.0,
            590.0,
            17.0,
            Color::GOLD,
        );
    } else if s.arena {
        label(
            d,
            font,
            &format!("{}: {}", text.get("source"), path.display()),
            28.0,
            612.0,
            16.0,
            MUTED,
        );
    }
}
