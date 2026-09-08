//! Exports deterministic contact and reaction evidence from the contextual showcase.
//!
//! System: Development examples at the Raylib boundary. Uses the same World,
//! sprite selection and renderer as Training, and refuses unsuccessful examples.

use std::{error::Error, fs, path::Path};

use borrow_fighters::{
    characters::{CHARACTER_BODY_METRICS_PATH, CharacterBodyMetricsCatalog, CharacterId},
    config::{WINDOW_HEIGHT, WINDOW_WIDTH},
    engine::{
        assets::{GameAssets, SpriteAtlasAsset},
        render::{draw_move_showcase, draw_render_target_to_window},
        sprites::{fighter_clip_elapsed_seconds, fighter_sprite_clip, frame_for_fighter_clip_at},
    },
    game::{arena::ArenaId, world::WorldSpriteCombatManifests},
    scenes::{
        combat_lab::{CombatLabInput, CombatLabMove},
        move_showcase::{MoveShowcase, MoveShowcaseOptions, SCENARIO_FRAMES, ShowcaseResult},
    },
};
use raylib::prelude::*;
use serde_json::json;

type CaptureResult<T> = Result<T, Box<dyn Error>>;

fn main() -> CaptureResult<()> {
    let raw_args: Vec<_> = std::env::args().skip(1).collect();
    let reversed = raw_args.iter().any(|arg| arg == "--reverse");
    let every_frame = raw_args.iter().any(|arg| arg == "--every-frame");
    let args: Vec<_> = raw_args
        .into_iter()
        .filter(|arg| arg != "--reverse" && arg != "--every-frame")
        .collect();
    if !(2..=3).contains(&args.len()) {
        return Err("usage: cargo run --example capture_showcase_review -- <character|all> <output-directory> [move] [--reverse] [--every-frame]".into());
    }
    let characters = if args[0] == "all" {
        vec![
            CharacterId::Rust,
            CharacterId::Duke,
            CharacterId::C,
            CharacterId::Python,
            CharacterId::Cpp,
        ]
    } else {
        vec![CharacterId::from_cli(&args[0]).ok_or("unknown character")?]
    };
    let filter = args
        .get(2)
        .map(|name| CombatLabMove::from_cli(name).ok_or("unknown move"))
        .transpose()?;
    if characters.contains(&CharacterId::Go) {
        return Err("this review covers the five public fighters; Go is unchanged".into());
    }
    let output = Path::new(&args[1]);
    fs::create_dir_all(output)?;
    let (mut raylib, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Borrow Fighters - contextual showcase review")
        .build();
    raylib.set_exit_key(None);
    let assets = GameAssets::load(&mut raylib, &thread);
    let metrics = CharacterBodyMetricsCatalog::load(CHARACTER_BODY_METRICS_PATH)?;
    let mut target =
        raylib.load_render_texture(&thread, WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32)?;
    let mut report = Vec::new();
    for character in characters {
        if assets.signature_atlas(character).is_none() {
            return Err(format!("{} signature effects did not load", character.audio_key()).into());
        }
        let mut scene = MoveShowcase::new(MoveShowcaseOptions { character });
        if reversed {
            scene.switch_sides();
        }
        scene.set_body_metrics(metrics.clone());
        scene.set_sprite_combat_manifests(WorldSpriteCombatManifests {
            player_one: Some(atlas(&assets, character)?.combat_manifest.clone()),
            player_two: Some(
                atlas(&assets, scene.opponent_character())?
                    .combat_manifest
                    .clone(),
            ),
        });
        for _ in 0..scene.move_count() {
            if filter.is_some_and(|selected| {
                scene.selected_move() != selected || scene.scenario().is_defense()
            }) {
                scene.update(CombatLabInput {
                    next_move: true,
                    ..CombatLabInput::default()
                });
                continue;
            }
            // Replay consumes one tick when unpaused, exactly as the interactive scene.
            scene.update(CombatLabInput {
                replay: true,
                ..CombatLabInput::default()
            });
            let mut contact_frame = None;
            let mut captures = Vec::new();
            let scenario = scene.scenario();
            let scenario_key = format!(
                "{}-{:02}-{}",
                character.audio_key(),
                scene.move_number(),
                format!("{scenario:?}")
                    .replace(['(', ')'], "")
                    .to_lowercase()
            );
            let mut last_actor_frames = [String::new(), String::new()];
            let mut last_effect_frames = String::new();
            while scene.current_frame() < SCENARIO_FRAMES {
                if raylib.window_should_close() {
                    return Err("capture window closed before review completed".into());
                }
                if contact_frame.is_none()
                    && matches!(
                        scene.result(),
                        ShowcaseResult::Hit { .. } | ShowcaseResult::Blocked { .. }
                    )
                {
                    contact_frame = Some(scene.current_frame());
                }
                let tick = scene.current_frame();
                let contact_offset = contact_frame.map(|frame| tick.saturating_sub(frame));
                let current_actor_frames = [&scene.world().player_one, &scene.world().player_two]
                    .into_iter()
                    .zip([character, scene.opponent_character()])
                    .map(|(fighter, id)| {
                        let clip = fighter_sprite_clip(fighter);
                        let time =
                            fighter_clip_elapsed_seconds(fighter, scene.world().elapsed_seconds);
                        frame_for_fighter_clip_at(
                            &atlas(&assets, id).expect("loaded actor atlas").manifest,
                            clip,
                            time,
                        )
                        .map(|frame| frame.name.clone())
                        .unwrap_or_default()
                    })
                    .collect::<Vec<_>>();
                let effect_frames = scene
                    .world()
                    .signature_effects
                    .iter()
                    .map(|effect| {
                        format!(
                            "{:?}:{}",
                            effect.kind,
                            (effect.elapsed_seconds * 15.0) as u32
                        )
                    })
                    .collect::<Vec<_>>()
                    .join(",");
                let scheduled = every_frame
                    || [1, 31, 48, SCENARIO_FRAMES - 1].contains(&tick)
                    || contact_offset
                        .is_some_and(|offset| [0, 6, 12, 22, 30, 36, 48, 64].contains(&offset))
                    || (tick >= 31
                        && (last_actor_frames.as_slice() != current_actor_frames
                            || last_effect_frames != effect_frames));
                last_actor_frames = current_actor_frames.try_into().expect("two actors");
                last_effect_frames = effect_frames;
                if scheduled {
                    {
                        let mut draw = raylib.begin_texture_mode(&thread, &mut target);
                        draw_move_showcase(
                            &mut draw,
                            &scene,
                            ArenaId::Sirius,
                            scene.world().elapsed_seconds,
                            &assets,
                        );
                    }
                    {
                        let mut draw = raylib.begin_drawing(&thread);
                        draw_render_target_to_window(&mut draw, &target);
                    }
                    let filename = format!("{scenario_key}-f{tick:03}.png");
                    let mut image = target.texture().load_image()?;
                    image.flip_vertical();
                    let bytes = image.export_image_to_memory(".png")?;
                    fs::write(output.join(&filename), &*bytes)?;
                    let actors = [(&scene.world().player_one, character), (&scene.world().player_two, scene.opponent_character())].into_iter().map(|(fighter, id)| {
                        let clip = fighter_sprite_clip(fighter);
                        let seconds = fighter_clip_elapsed_seconds(fighter, scene.world().elapsed_seconds);
                        let loaded = atlas(&assets, id)?;
                        if loaded.manifest.clip_named(clip.as_str()).is_none() {
                            return Err(format!("{} is missing the actual {} clip", id.audio_key(), clip.as_str()).into());
                        }
                        let frame = frame_for_fighter_clip_at(&loaded.manifest, clip, seconds).ok_or("missing rendered animation frame")?;
                        Ok(json!({ "character": id.audio_key(), "clip": clip.as_str(), "frame": frame.name, "frame_clip": frame.clip, "health": fighter.health, "grounded": fighter.grounded, "crouching": fighter.crouching, "blocking": fighter.blocking, "hitstun": fighter.in_hitstun(), "blockstun": fighter.in_blockstun(), "captured": fighter.in_capture(), "air_reaction": fighter.in_air_reaction(), "position": [fighter.position.x, fighter.position.y] }))
                    }).collect::<CaptureResult<Vec<_>>>()?;
                    let effects = scene.world().signature_effects.iter().map(|effect| json!({ "kind": format!("{:?}", effect.kind), "position": [effect.position.x,effect.position.y], "elapsed":effect.elapsed_seconds })).collect::<Vec<_>>();
                    captures.push(json!({ "image": filename, "tick": tick, "result": format!("{:?}", scene.result()), "actors": actors, "effects": effects }));
                }
                scene.update(CombatLabInput::default());
            }
            if !matches!(
                (scene.scenario().is_defense(), scene.result()),
                (false, ShowcaseResult::Hit { .. }) | (true, ShowcaseResult::Blocked { .. })
            ) {
                return Err(format!("{scenario_key} failed: {:?}", scene.result()).into());
            }
            report.push(json!({ "character": character.audio_key(), "scenario": format!("{scenario:?}"), "scenario_label": scene.scenario_label(), "result": format!("{:?}", scene.result()), "contact_frame": contact_frame, "captures": captures }));
            scene.update(CombatLabInput::default());
        }
    }
    fs::write(
        output.join("showcase-review.json"),
        serde_json::to_string_pretty(
            &json!({ "render_size": [WINDOW_WIDTH, WINDOW_HEIGHT], "reversed": reversed, "scenarios": report }),
        )? + "\n",
    )?;
    println!(
        "Captured {} successful contextual scenarios in {}",
        report.len(),
        output.display()
    );
    Ok(())
}

fn atlas(assets: &GameAssets, character: CharacterId) -> CaptureResult<&SpriteAtlasAsset> {
    match character {
        CharacterId::Rust => assets.rust_fighter.as_ref(),
        CharacterId::Duke => assets.duke_fighter.as_ref(),
        CharacterId::Go => assets.go_fighter.as_ref(),
        CharacterId::C => assets.c_fighter.as_ref(),
        CharacterId::Python => assets.python_fighter.as_ref(),
        CharacterId::Cpp => assets.cpp_fighter.as_ref(),
    }
    .ok_or_else(|| format!("{} atlas did not load", character.audio_key()).into())
}
