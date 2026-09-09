//! Captures contact-clock reaction matrices using the actual manifest renderer.
//!
//! System: Development evidence. Drives real World hits against all defenders,
//! then places unchanged snapshots in a labeled sheet; no audio or host input.

use borrow_fighters::{
    characters::CharacterId,
    combat::fighter::{Fighter, FighterInput},
    config::{FIXED_TIMESTEP as DT, FLOOR_Y},
    engine::{
        assets::{GameAssets, SpriteAtlasAsset},
        sprites::{self, FighterSpritePresentation, FighterVisualPlacement},
    },
    game::world::World,
    math::vec2::Vec2,
};
use raylib::prelude::*;
use serde_json::json;
use std::{error::Error, fs, path::Path};

type Result<T> = std::result::Result<T, Box<dyn Error>>;
const ROSTER: [CharacterId; 6] = [
    CharacterId::Rust,
    CharacterId::Duke,
    CharacterId::Go,
    CharacterId::C,
    CharacterId::Python,
    CharacterId::Cpp,
];

fn atlas(assets: &GameAssets, character: CharacterId) -> Result<&SpriteAtlasAsset> {
    match character {
        CharacterId::Rust => assets.rust_fighter.as_ref(),
        CharacterId::Duke => assets.duke_fighter.as_ref(),
        CharacterId::Go => assets.go_fighter.as_ref(),
        CharacterId::C => assets.c_fighter.as_ref(),
        CharacterId::Python => assets.python_fighter.as_ref(),
        CharacterId::Cpp => assets.cpp_fighter.as_ref(),
    }
    .ok_or_else(|| "reviewed atlas missing".into())
}

fn snapshots(character: CharacterId, kind: &str) -> Vec<(u32, Fighter)> {
    let mut world = World::new_with_characters(CharacterId::Duke, character);
    world.player_one.position.x = 440.0;
    world.player_two.position.x = world.player_one.body_rect().right() + 5.0;
    let offsets = if kind == "super" {
        [0, 6, 14, 26, 43, 59]
    } else {
        [0, 2, 4, 7, 10, 14]
    };
    let input = match kind {
        "heavy" => FighterInput {
            heavy_punch: true,
            ..FighterInput::default()
        },
        "super" => FighterInput {
            cinematic_special: true,
            ..FighterInput::default()
        },
        _ => FighterInput {
            light_punch: true,
            ..FighterInput::default()
        },
    };
    let mut contact = None;
    let mut snapshots = Vec::new();
    for tick in 0..500 {
        world.update(
            DT,
            if tick == 0 {
                input
            } else {
                FighterInput::default()
            },
            FighterInput::default(),
        );
        if contact.is_none() && world.player_two.health < world.player_two.max_health {
            contact = Some(tick);
        }
        if let Some(contact) = contact {
            let elapsed = tick - contact;
            if offsets.contains(&elapsed) {
                snapshots.push((elapsed, world.player_two.clone()));
            }
            if elapsed == offsets[5] {
                break;
            }
        }
    }
    assert_eq!(
        snapshots.len(),
        6,
        "missing real contact for {character:?}/{kind}"
    );
    snapshots
}

fn label(
    draw: &mut impl RaylibDraw,
    assets: &GameAssets,
    text: &str,
    x: f32,
    y: f32,
    size: f32,
    color: Color,
) {
    if let Some(font) = assets.menu_font.as_ref() {
        draw.draw_text_ex(font, text, Vector2::new(x, y), size, 0.5, color);
    }
}

fn main() -> Result<()> {
    let output = std::env::args()
        .nth(1)
        .ok_or("usage: capture_reaction_review OUTPUT")?;
    let output = Path::new(&output);
    fs::create_dir_all(output)?;
    let (mut rl, thread) = raylib::init()
        .size(1600, 1000)
        .hidden()
        .msaa_4x()
        .title("Reaction review")
        .build();
    let assets = GameAssets::load(&mut rl, &thread);
    let mut target = rl.load_render_texture(&thread, 1600, 1000)?;
    let mut report = Vec::new();
    for kind in ["hit", "heavy", "super"] {
        let rows = ROSTER.map(|character| (character, snapshots(character, kind)));
        {
            let mut draw = rl.begin_texture_mode(&thread, &mut target);
            draw.clear_background(Color::new(16, 22, 34, 255));
            label(
                &mut draw,
                &assets,
                &format!("REACTION REVIEW / {kind} / real World contacts"),
                22.0,
                18.0,
                27.0,
                Color::WHITE,
            );
            label(
                &mut draw,
                &assets,
                "Each column advances the same hit. Sprite placement is scaled only to compare six defenders.",
                22.0,
                51.0,
                18.0,
                Color::LIGHTGRAY,
            );
            for (row, (character, states)) in rows.iter().enumerate() {
                let sheet = atlas(&assets, *character)?;
                let top = 84.0 + row as f32 * 150.0;
                label(
                    &mut draw,
                    &assets,
                    character.audio_key(),
                    12.0,
                    top + 70.0,
                    20.0,
                    Color::new(255, 198, 93, 255),
                );
                let start_x = states[0].1.body_rect().center_x();
                for (column, (tick, fighter)) in states.iter().enumerate() {
                    let left = 110.0 + column as f32 * 245.0;
                    let baseline = top + 126.0;
                    draw.draw_rectangle(
                        left as i32,
                        top as i32,
                        239,
                        144,
                        Color::new(24, 33, 47, 255),
                    );
                    draw.draw_line(
                        left as i32,
                        baseline as i32,
                        (left + 239.0) as i32,
                        baseline as i32,
                        Color::new(73, 94, 108, 255),
                    );
                    let scale = 0.48;
                    let anchor = Vec2::new(
                        left + 110.0 + (fighter.body_rect().center_x() - start_x) * scale,
                        baseline + (fighter.body_rect().bottom() - FLOOR_Y) * scale,
                    );
                    sprites::draw_manifest_fighter_sprite_placed(
                        &mut draw,
                        &sheet.manifest,
                        fighter,
                        FighterSpritePresentation {
                            elapsed_seconds: *tick as f32 * DT,
                            forced_clip: None,
                            placement: Some(FighterVisualPlacement {
                                anchor,
                                scale,
                                rotation_degrees: 0.0,
                                opacity: 1.0,
                            }),
                        },
                        Color::WHITE,
                        |frame| sheet.texture_for_frame(frame),
                    );
                    let clip = sprites::fighter_sprite_clip(fighter);
                    let frame =
                        sprites::frame_for_fighter_state(&sheet.manifest, fighter, clip, 0.0)
                            .unwrap();
                    label(
                        &mut draw,
                        &assets,
                        &format!("+{tick:02}f {}", clip.as_str()),
                        left + 7.0,
                        top + 4.0,
                        15.0,
                        Color::LIGHTGRAY,
                    );
                    report.push(json!({"kind":kind,"character":character.audio_key(),"tick_since_contact":tick,"health":fighter.health,"frame":frame.name,"clip":clip.as_str(),"feet_y":fighter.body_rect().bottom(),"reaction":format!("{:?}",fighter.reaction_visual_state())}));
                }
            }
        }
        let mut image = target.texture().load_image()?;
        image.flip_vertical();
        fs::write(
            output.join(format!("{kind}-six-defenders.png")),
            image.export_image_to_memory(".png")?.as_ref(),
        )?;
    }
    fs::write(
        output.join("reaction-review.json"),
        serde_json::to_vec_pretty(
            &json!({"method":"Actual World snapshots, shared fighter renderer, 0.48 visual scale for comparison", "samples":report}),
        )?,
    )?;
    Ok(())
}
