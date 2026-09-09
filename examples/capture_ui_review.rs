//! Captures the menu pages and fight HUD through the actual game renderer.
//!
//! System: Development examples at the Raylib boundary. Keeps typography,
//! pointer-layout and presentation review reproducible without input automation.

use std::{error::Error, fs, path::Path};

use borrow_fighters::{
    characters::CharacterId,
    combat::fighter::PlayerSlot,
    config::{WINDOW_HEIGHT, WINDOW_WIDTH},
    engine::{
        assets::GameAssets,
        render::{GamepadStatus, PreferencesDrawOptions, draw_fight, draw_preferences},
    },
    game::{
        arena::ArenaId,
        feature_flags::{FeatureFlag, FeatureFlags},
        world::{MatchOutcome, World},
    },
    scenes::preferences::{PreferencesInput, PreferencesMenu, PreferencesPointerInput},
};
use raylib::core::text::RaylibFont;
use raylib::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let destination = std::env::args()
        .nth(1)
        .ok_or("usage: cargo run --example capture_ui_review -- <output-directory>")?;
    let output = Path::new(&destination);
    fs::create_dir_all(output)?;
    let (mut raylib, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Borrow Fighters - UI review")
        .hidden()
        .msaa_4x()
        .build();
    let assets = GameAssets::load(&mut raylib, &thread);
    for font in [&assets.menu_font, &assets.lore_font, &assets.lore_body_font] {
        let font = font.as_ref().ok_or("bundled UI font failed to load")?;
        // A successful ASCII atlas can still silently replace every accent by '?'.
        for character in "ÁáÂâÃãÀàÉéÊêÍíÓóÔôÕõÚúÇç".chars() {
            if font.get_glyph_index(character) == font.get_glyph_index('?') {
                return Err(format!("font is missing Portuguese glyph {character}").into());
            }
        }
    }
    let mut target =
        raylib.load_render_texture(&thread, WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32)?;
    let mut flags = FeatureFlags::default();
    for (name, row) in [
        ("main", None),
        ("versus", Some(1)),
        ("training", Some(2)),
        ("lore", Some(3)),
        ("options", Some(4)),
    ] {
        let mut menu = PreferencesMenu::default();
        menu.update(PreferencesInput::default(), &mut flags);
        if let Some(row) = row {
            menu.update(
                PreferencesInput {
                    pointer: PreferencesPointerInput {
                        hovered_row: Some(row),
                        moved: true,
                        activate: true,
                        previous: false,
                    },
                    ..PreferencesInput::default()
                },
                &mut flags,
            );
        }
        for _ in 0..30 {
            menu.tick_visuals();
        }
        {
            let mut draw = raylib.begin_texture_mode(&thread, &mut target);
            draw_preferences(
                &mut draw,
                PreferencesDrawOptions {
                    menu: &menu,
                    player_one_character: CharacterId::Rust,
                    player_two_character: CharacterId::Duke,
                    arena: ArenaId::JavaStreet,
                    music_volume_percent: 60,
                    visual_time_seconds: 7.0,
                    flags,
                    gamepad_status: GamepadStatus::default(),
                    recording: false,
                    assets: &assets,
                },
            );
        }
        export(&target, &output.join(format!("{name}.png")))?;
    }
    let mut world = World::new_greybox();
    world.player_one.health = 67;
    world.player_two.health = 22;
    for (name, outcome, controls) in [
        ("hud", None, false),
        ("controls", None, true),
        (
            "victory",
            Some(MatchOutcome::Winner(PlayerSlot::One)),
            false,
        ),
    ] {
        world.outcome = outcome;
        flags.set(FeatureFlag::ShowControlsHelp, controls);
        {
            let mut draw = raylib.begin_texture_mode(&thread, &mut target);
            draw_fight(
                &mut draw,
                &world,
                ArenaId::JavaStreet,
                7.0,
                flags,
                GamepadStatus::default(),
                &assets,
            );
        }
        export(&target, &output.join(format!("{name}.png")))?;
    }
    println!("UI review written to {}", output.display());
    Ok(())
}

fn export(target: &RenderTexture2D, path: &Path) -> Result<(), Box<dyn Error>> {
    let mut image = target.texture().load_image()?;
    image.flip_vertical();
    image.export_image(path.to_str().ok_or("output path must be UTF-8")?);
    Ok(())
}
