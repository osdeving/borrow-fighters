//! Captures deterministic movement and optional attack sequences through draw_fight.
//!
//! System: Development examples at the Raylib boundary. Uses normal World inputs,
//! opposing facings and baseline combat data; writes GPU images and simulation
//! timestamps for review without changing game rules or authoring sprite pixels.

use std::{collections::BTreeSet, error::Error, fs, path::Path, process::Command};

use borrow_fighters::{
    characters::{CHARACTER_BODY_METRICS_PATH, CharacterBodyMetricsCatalog, CharacterId},
    combat::fighter::{Facing, Fighter, FighterInput},
    config::{FIXED_TIMESTEP, WINDOW_HEIGHT, WINDOW_WIDTH},
    engine::{
        assets::GameAssets,
        render::{GamepadStatus, draw_fight, draw_render_target_to_window},
        sprites::{
            FighterSpriteClip, SpriteManifest, fighter_clip_elapsed_seconds, fighter_sprite_clip,
            frame_for_fighter_clip_at, match_fighter_sprite_clip,
        },
    },
    game::{
        arena::ArenaId,
        feature_flags::{FeatureFlag, FeatureFlags},
        world::{World, WorldSpriteCombatManifests},
    },
};
use raylib::prelude::*;
use serde_json::{Value, json};

const MOTION_TICKS: u32 = 240;
const MOVE_TICKS: u32 = 120;
const MOVES: [&str; 10] = [
    "punch_light",
    "punch_heavy",
    "kick",
    "sweep",
    "overhead",
    "anti_air",
    "air_punch",
    "air_kick",
    "throw",
    "special",
];

fn move_at(live_tick: u32, attacks: bool) -> Option<(&'static str, u32)> {
    let tick = live_tick.checked_sub(MOTION_TICKS)?;
    attacks.then_some(())?;
    MOVES
        .get((tick / MOVE_TICKS) as usize)
        .map(|name| (*name, tick % MOVE_TICKS))
}

fn move_input(name: &str, tick: u32, facing: Facing) -> FighterInput {
    let mut input = FighterInput::default();
    let airborne = matches!(name, "air_punch" | "air_kick");
    if airborne && tick == 0 {
        input.jump = true;
    }
    if tick != if airborne { 8 } else { 0 } {
        return input;
    }
    match name {
        "punch_light" | "air_punch" => input.light_punch = true,
        "punch_heavy" => input.heavy_punch = true,
        "kick" | "air_kick" => input.kick = true,
        "sweep" => {
            input.crouch = true;
            input.kick = true;
        }
        "overhead" => {
            input.heavy_punch = true;
            input.right = facing == Facing::Right;
            input.left = facing == Facing::Left;
        }
        "anti_air" => {
            input.crouch = true;
            input.heavy_punch = true;
        }
        "throw" => {
            input.block = true;
            input.light_punch = true;
        }
        "special" => input.projectile = true,
        _ => unreachable!("move names come from MOVES"),
    }
    input
}

fn visual(fighter: &Fighter, world: &World, manifest: &SpriteManifest) -> Value {
    let forced = match_fighter_sprite_clip(world.outcome, fighter.slot, world.spawn_intro_active());
    let clip = forced.unwrap_or_else(|| fighter_sprite_clip(fighter));
    let seconds = if world.outcome.is_some() {
        world.outcome_elapsed_seconds()
    } else if world.spawn_intro_active() {
        world.spawn_intro_elapsed_seconds()
    } else {
        fighter_clip_elapsed_seconds(fighter, world.elapsed_seconds)
    };
    let frame = frame_for_fighter_clip_at(manifest, clip, seconds).unwrap();
    let body = fighter.body_rect();
    json!({"clip": clip.as_str(), "frame": frame.name, "clip_seconds": seconds,
        "facing": format!("{:?}", fighter.facing), "position": [fighter.position.x, fighter.position.y],
        "velocity": [fighter.velocity.x, fighter.velocity.y], "grounded": fighter.grounded,
        "body": [body.x, body.y, body.width, body.height], "health": fighter.health,
        "attack_phase": format!("{:?}", fighter.attack_phase()),
        "in_whiff_recovery": fighter.in_whiff_recovery()})
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    const USAGE: &str = "usage: BORROW_FIGHTERS_SPRITE_CANDIDATES=1 cargo run --example capture_motion_review -- <fresh-output-directory> <character> [--attacks] [--every-tick] [--debug-boxes]";
    if args.iter().any(|arg| arg == "--help" || arg == "-h") {
        println!("{USAGE}");
        return Ok(());
    }
    if args.len() < 2
        || args[2..]
            .iter()
            .any(|arg| !matches!(arg.as_str(), "--attacks" | "--every-tick" | "--debug-boxes"))
    {
        return Err(USAGE.into());
    }
    let attacks = args.iter().any(|arg| arg == "--attacks");
    let every_tick = args.iter().any(|arg| arg == "--every-tick");
    let debug_boxes = args.iter().any(|arg| arg == "--debug-boxes");
    assert_eq!(
        std::env::var("BORROW_FIGHTERS_SPRITE_CANDIDATES").as_deref(),
        Ok("1")
    );
    let output = Path::new(&args[0]);
    if output.exists() {
        return Err("output already exists; keep previous evidence intact".into());
    }
    let character = CharacterId::from_cli(&args[1]).ok_or("unknown character")?;
    let key = character.audio_key();
    let manifest_name = format!("assets/candidates/{key}/{key}-fighter.sprite.json");
    let manifest_path = manifest_name.as_str();
    let manifest_bytes = fs::read(manifest_path)?;
    let expected = SpriteManifest::load(manifest_path)?;
    let image_path = Path::new(manifest_path)
        .parent()
        .unwrap()
        .join(&expected.image);
    let image_bytes = fs::read(&image_path)?;
    let image_hash = Command::new("sha256sum").arg(&image_path).output()?;
    assert!(image_hash.status.success());
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Borrow Fighters - deterministic motion review")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();
    let assets = GameAssets::load(&mut rl, &thread);
    let atlas = match character {
        CharacterId::Rust => &assets.rust_fighter,
        CharacterId::Duke => &assets.duke_fighter,
        CharacterId::Go => &assets.go_fighter,
        CharacterId::C => &assets.c_fighter,
        CharacterId::Python => &assets.python_fighter,
        CharacterId::Cpp => &assets.cpp_fighter,
    }
    .as_ref()
    .ok_or("requested fighter atlas did not load")?;
    assert_eq!(
        serde_json::to_value(&expected)?,
        serde_json::to_value(&atlas.manifest)?,
        "candidate fallback is not a review capture"
    );
    for required in FighterSpriteClip::REQUIRED {
        let clip = atlas
            .manifest
            .clip_named(required.as_str())
            .ok_or("required clip missing")?;
        for name in &clip.frames {
            let frame = atlas
                .manifest
                .frame_named(name)
                .ok_or("required frame missing")?;
            if frame.clip != required.as_str() {
                return Err("candidate aliases a required clip to another action".into());
            }
        }
    }
    for frame in &atlas.manifest.frames {
        let texture = atlas
            .texture_for_frame(frame)
            .ok_or("frame texture missing")?;
        assert!(frame.frame.x + frame.frame.w <= texture.width());
        assert!(frame.frame.y + frame.frame.h <= texture.height());
    }
    let metrics = CharacterBodyMetricsCatalog::load(CHARACTER_BODY_METRICS_PATH)?;
    let mut world =
        World::new_greybox_with_intro_for_characters_and_metrics(character, character, &metrics);
    world.set_sprite_combat_manifests(WorldSpriteCombatManifests {
        player_one: Some(atlas.combat_manifest.clone()),
        player_two: Some(atlas.combat_manifest.clone()),
    });
    let mut flags = FeatureFlags::default();
    flags.set(FeatureFlag::PlayerOneCpu, false);
    flags.set(FeatureFlag::PlayerTwoCpu, false);
    flags.set(FeatureFlag::GamepadInput, false);
    flags.set(FeatureFlag::ShowCombatDebug, debug_boxes);
    let mut target = rl.load_render_texture(&thread, WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32)?;
    fs::create_dir_all(output)?;
    let mut captures = Vec::new();
    let mut states = Vec::new();
    let mut tick = 0_u32;
    let mut live_tick = 0_u32;
    let mut previous = Value::Null;
    let last_live_tick = MOTION_TICKS
        + if attacks {
            MOVE_TICKS * MOVES.len() as u32 + 60
        } else {
            0
        };
    let mut seen_moves: [BTreeSet<String>; 2] = Default::default();
    let mut returned_to_idle: Vec<Value> = Vec::new();
    loop {
        if rl.window_should_close() {
            return Err("capture window closed before review completed".into());
        }
        let live = !world.spawn_intro_active() && !world.countdown_active();
        let phase = if world.spawn_intro_active() {
            "spawn"
        } else if world.countdown_active() {
            "countdown"
        } else if live_tick < 30 {
            "idle_before_walk"
        } else if live_tick < 60 {
            "walk_in"
        } else if live_tick < 90 {
            "walk_out"
        } else if live_tick < 150 {
            "return_idle"
        } else if live_tick < 210 {
            "jump_arc"
        } else if let Some((name, _)) = move_at(live_tick, attacks) {
            name
        } else {
            "idle_after_jump"
        };
        let p1 = visual(&world.player_one, &world, &atlas.manifest);
        let p2 = visual(&world.player_two, &world, &atlas.manifest);
        assert_eq!(world.player_one.facing, Facing::Right);
        assert_eq!(world.player_two.facing, Facing::Left);
        assert!(world.outcome.is_none());
        let mut input_one = FighterInput::default();
        let mut input_two = FighterInput::default();
        match phase {
            "walk_in" => {
                input_one.right = true;
                input_two.left = true;
            }
            "walk_out" => {
                input_one.left = true;
                input_two.right = true;
            }
            "jump_arc" if live_tick == 150 => {
                input_one.jump = true;
                input_two.jump = true;
            }
            _ => {}
        }
        if let Some((name, move_tick)) = move_at(live_tick, attacks) {
            input_one = move_input(name, move_tick, world.player_one.facing);
            input_two = move_input(name, move_tick, world.player_two.facing);
            for (index, fighter) in [&world.player_one, &world.player_two]
                .into_iter()
                .enumerate()
            {
                if fighter_sprite_clip(fighter).as_str() == name {
                    seen_moves[index].insert(name.to_string());
                }
                if move_tick == MOVE_TICKS - 1 {
                    assert_eq!(
                        fighter_sprite_clip(fighter),
                        FighterSpriteClip::Idle,
                        "{key} {name} failed to return to idle"
                    );
                    assert!(fighter.grounded && !fighter.in_whiff_recovery());
                }
            }
            if move_tick == MOVE_TICKS - 1 {
                returned_to_idle.push(json!({"move": name, "tick": tick, "p1": p1, "p2": p2}));
            }
        }
        let state = json!({"tick": tick, "live_tick": live.then_some(live_tick), "phase": phase,
            "seconds": world.elapsed_seconds, "p1": p1, "p2": p2,
            "countdown_label": world.countdown_label(),
            "projectiles": world.projectiles.iter().map(|projectile| {
                let rect = projectile.rect();
                json!({"owner":format!("{:?}", projectile.owner),"rect":[rect.x,rect.y,rect.width,rect.height]})
            }).collect::<Vec<_>>(),
            "next_input": {"p1":format!("{input_one:?}"),"p2":format!("{input_two:?}")}});
        let signature = json!([
            phase,
            p1["clip"],
            p1["frame"],
            p2["clip"],
            p2["frame"],
            p1["attack_phase"],
            p2["attack_phase"],
            world.projectiles.len(),
            world.countdown_label(),
            world.player_one.grounded,
            world.player_one.velocity.y >= 0.0
        ]);
        let last = live && live_tick == last_live_tick;
        if every_tick || signature != previous || (live && live_tick.is_multiple_of(3)) || last {
            for _ in 0..2 {
                let mut draw = rl.begin_drawing(&thread);
                {
                    let mut texture = draw.begin_texture_mode(&thread, &mut target);
                    draw_fight(
                        &mut texture,
                        &world,
                        ArenaId::Sirius,
                        world.elapsed_seconds,
                        flags,
                        GamepadStatus::default(),
                        &assets,
                    );
                }
                draw_render_target_to_window(&mut draw, &target);
            }
            let mut image = target.texture().load_image()?;
            image.flip_vertical();
            let bytes = image.export_image_to_memory(".png")?;
            let filename = format!("f{tick:04}-{phase}.png");
            fs::write(output.join(&filename), &*bytes)?;
            let mut captured = state.clone();
            captured["image"] = json!(filename);
            captures.push(captured);
        }
        previous = signature;
        states.push(state);
        if last {
            break;
        }
        world.update_with_flags(FIXED_TIMESTEP, input_one, input_two, flags);
        if live {
            live_tick += 1;
        }
        tick += 1;
        assert!(
            tick < last_live_tick + 1000,
            "intro or review scenario did not finish"
        );
    }
    for fighter in [&world.player_one, &world.player_two] {
        assert_eq!(fighter_sprite_clip(fighter), FighterSpriteClip::Idle);
        if !attacks {
            assert_eq!(fighter.health, fighter.max_health);
        }
        assert!(fighter.grounded && fighter.health > 0);
        assert_eq!(fighter.velocity.x, 0.0);
    }
    if attacks {
        let expected_moves: BTreeSet<_> = MOVES.iter().map(|name| name.to_string()).collect();
        for actual in &seen_moves {
            assert_eq!(
                actual, &expected_moves,
                "not every attack played in each facing"
            );
        }
    }
    assert_eq!(
        fs::read(manifest_path)?,
        manifest_bytes,
        "manifest changed during capture"
    );
    assert_eq!(
        fs::read(image_path)?,
        image_bytes,
        "atlas changed during capture"
    );
    fs::write(
        output.join("capture-report.json"),
        serde_json::to_string_pretty(&json!({
        "character": key, "scenario": "World same-character mirror: real intro/countdown, idle, 30 ticks forward, 30 ticks backward, 60 ticks settling/idle, one jump input followed through the complete arc and return to idle; optional ten attacks use normal inputs with recovery between moves",
        "renderer": "draw_fight -> RenderTexture -> GPU readback", "ai_or_keyboard": false,
        "candidate_loaded_matches_disk": true, "candidate_image_sha256": String::from_utf8(image_hash.stdout)?,
        "loaded_manifest": atlas.manifest, "loaded_combat_manifest": atlas.combat_manifest,
        "physical_metrics": CHARACTER_BODY_METRICS_PATH,
        "combat_metadata": "baseline combat_manifest, same wiring as App", "fixed_timestep": FIXED_TIMESTEP,
        "capture_sampling": if every_tick { "every 60 Hz simulation tick" } else { "each pose/frame/phase/countdown change, plus every 3 live simulation ticks" },
        "every_tick": every_tick, "debug_boxes": debug_boxes, "attacks": attacks,
        "moves_seen_per_player": seen_moves, "returned_to_idle_after_moves": returned_to_idle,
        "last_tick": tick,
        "limitations": ["Controlled World inputs; not an interactive keyboard playtest.",
            "Close attacks are whiffs at the scenario spacing; use Combat Lab for contact alignment.",
            "With --attacks, the final projectiles may hit normally; health is recorded and never reset.",
            "This recording does not evaluate combat balance, audio or input devices."],
        "captures": captures, "states": states }))?,
    )?;
    println!(
        "Captured {} frames with {} simulation states",
        captures.len(),
        states.len()
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal_inputs_play_every_move_in_both_facings_and_finish_grounded() {
        let metrics = CharacterBodyMetricsCatalog::load(CHARACTER_BODY_METRICS_PATH).unwrap();
        for character in [
            CharacterId::Rust,
            CharacterId::Duke,
            CharacterId::Go,
            CharacterId::C,
            CharacterId::Python,
            CharacterId::Cpp,
        ] {
            let key = character.audio_key();
            let baseline =
                SpriteManifest::load(format!("assets/placeholder/{key}-fighter.sprite.json"))
                    .unwrap();
            let mut world = World::new_with_character_body_metrics(character, character, &metrics);
            world.set_sprite_combat_manifests(WorldSpriteCombatManifests {
                player_one: Some(baseline.clone()),
                player_two: Some(baseline),
            });
            let mut flags = FeatureFlags::default();
            flags.set(FeatureFlag::PlayerOneCpu, false);
            flags.set(FeatureFlag::PlayerTwoCpu, false);
            for name in MOVES {
                let mut seen = [false; 2];
                for tick in 0..MOVE_TICKS {
                    world.update_with_flags(
                        FIXED_TIMESTEP,
                        move_input(name, tick, Facing::Right),
                        move_input(name, tick, Facing::Left),
                        flags,
                    );
                    for (index, fighter) in [&world.player_one, &world.player_two]
                        .into_iter()
                        .enumerate()
                    {
                        seen[index] |= fighter_sprite_clip(fighter).as_str() == name;
                        assert!(fighter.health > 0);
                    }
                    assert_eq!(world.player_one.facing, Facing::Right);
                    assert_eq!(world.player_two.facing, Facing::Left);
                }
                assert_eq!(seen, [true, true], "{key} never played {name}");
                for fighter in [&world.player_one, &world.player_two] {
                    assert_eq!(
                        fighter_sprite_clip(fighter),
                        FighterSpriteClip::Idle,
                        "{key} failed to settle after {name}"
                    );
                    assert!(fighter.grounded && !fighter.in_whiff_recovery());
                }
            }
        }
    }
}
