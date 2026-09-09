//! Captures roster animation, confirmation and match overlays with the real renderer.
//!
//! Output stays outside the repository unless a reviewed frame is explicitly kept.

use borrow_fighters::{
    characters::CharacterId,
    combat::fighter::{FighterInput, PlayerSlot},
    config::{FIXED_TIMESTEP, WINDOW_HEIGHT, WINDOW_WIDTH, world_px},
    engine::{
        assets::GameAssets,
        render::{self, GamepadStatus},
    },
    game::{
        arena::ArenaId,
        energy::EnergyPolicy,
        feature_flags::{FeatureFlag, FeatureFlags},
        world::{MatchOutcome, World},
    },
    scenes::{
        character_select::{CharacterSelect, SelectCommand, SelectInput},
        match_flow::MatchFlow,
        preferences::PlayMode,
    },
};
use raylib::prelude::*;
use std::{error::Error, fs, path::Path};

fn main() -> Result<(), Box<dyn Error>> {
    let destination = std::env::args()
        .nth(1)
        .ok_or("usage: capture_roster_flow_review <output-directory> [--frames]")?;
    let frames = std::env::args().any(|arg| arg == "--frames");
    let output = Path::new(&destination);
    fs::create_dir_all(output)?;
    let (mut raylib, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Borrow Fighters - roster review")
        .hidden()
        .msaa_4x()
        .build();
    let assets = GameAssets::load(&mut raylib, &thread);
    for atlas in [
        &assets.rust_fighter,
        &assets.duke_fighter,
        &assets.c_fighter,
        &assets.python_fighter,
        &assets.cpp_fighter,
    ] {
        assert!(atlas.is_some(), "public roster art must load");
    }
    let mut target =
        raylib.load_render_texture(&thread, WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32)?;
    for (name, characters) in [
        ("rust-java", [CharacterId::Rust, CharacterId::Duke]),
        ("old-c-python", [CharacterId::C, CharacterId::Python]),
        ("python-cpp", [CharacterId::Python, CharacterId::Cpp]),
    ] {
        let mut selection =
            CharacterSelect::new(characters, ArenaId::Sirius, PlayMode::LocalDuel, 17);
        selection.update(1.0, SelectInput::default());
        {
            let mut draw = raylib.begin_texture_mode(&thread, &mut target);
            render::draw_character_select(&mut draw, &selection, &assets);
        }
        export(&target, &output.join(format!("roster-{name}.png")))?;
    }
    let mut selection = CharacterSelect::new(
        [CharacterId::Rust, CharacterId::Duke],
        ArenaId::PortoDigital,
        PlayMode::AgainstCpu,
        991,
    );
    selection.update(0.0, SelectInput::default());
    for frame in 0..150 {
        let input = match frame {
            40 => SelectInput {
                hover: Some(5),
                ..Default::default()
            },
            75 | 105 => SelectInput {
                shared: SelectCommand {
                    confirm: true,
                    ..Default::default()
                },
                ..Default::default()
            },
            _ => SelectInput::default(),
        };
        selection.update(1.0 / 60.0, input);
        {
            let mut draw = raylib.begin_texture_mode(&thread, &mut target);
            render::draw_character_select(&mut draw, &selection, &assets);
        }
        if frames {
            export(&target, &output.join(format!("selection-{frame:04}.png")))?;
        }
        if [5, 65, 120].contains(&frame) {
            export(
                &target,
                &output.join(format!("selection-state-{frame:03}.png")),
            )?;
        }
    }
    let flags = FeatureFlags::default();
    let mut world = World::new_greybox();
    world.set_energy_policy(EnergyPolicy::Metered);
    for (name, flow) in [
        ("pause", Some(MatchFlow::pause())),
        ("energy", None),
        ("result", Some(MatchFlow::result())),
    ] {
        let mut flow = flow;
        if name == "result" {
            world.player_two.health = 0;
            world.outcome = Some(MatchOutcome::Winner(PlayerSlot::One));
            for _ in 0..120 {
                world.update_with_flags(
                    1.0 / 60.0,
                    FighterInput::default(),
                    FighterInput::default(),
                    flags,
                );
            }
        }
        if let Some(flow) = &mut flow {
            for _ in 0..5 {
                flow.tick(0.2);
            }
        }
        {
            let mut draw = raylib.begin_texture_mode(&thread, &mut target);
            render::draw_fight(
                &mut draw,
                &world,
                ArenaId::Sirius,
                7.0,
                flags,
                GamepadStatus::default(),
                &assets,
            );
            if let Some(flow) = &flow {
                render::draw_match_flow(&mut draw, flow, &world, 7.0, &assets);
            }
        }
        export(&target, &output.join(format!("fight-{name}.png")))?;
    }
    // Charge through accepted melee contacts so the HUD review uses actual energy.
    let mut world = World::new_with_characters(CharacterId::C, CharacterId::Rust);
    world.set_energy_policy(EnergyPolicy::Metered);
    let mut protected = flags;
    protected.set(FeatureFlag::PlayerOneTakesDamage, false);
    protected.set(FeatureFlag::PlayerTwoTakesDamage, false);
    for _ in 0..5 {
        world.player_one.position.x = world_px(300.0);
        world.player_two.position.x = world.player_one.body_rect().right() + world_px(10.0);
        for tick in 0..120 {
            world.update_with_flags(
                FIXED_TIMESTEP,
                FighterInput {
                    light_punch: tick == 0,
                    ..Default::default()
                },
                FighterInput::default(),
                protected,
            );
        }
    }
    assert_eq!(world.energy(PlayerSlot::One).amount(), 100);
    for spent in [false, true] {
        if spent {
            world.player_two.position.x = world.player_one.body_rect().right() + world_px(10.0);
            world.update_with_flags(
                FIXED_TIMESTEP,
                FighterInput {
                    cinematic_special: true,
                    ..Default::default()
                },
                FighterInput::default(),
                protected,
            );
            assert_eq!(world.energy(PlayerSlot::One).amount(), 0);
        }
        {
            let mut draw = raylib.begin_texture_mode(&thread, &mut target);
            render::draw_fight(
                &mut draw,
                &world,
                ArenaId::Sirius,
                7.0,
                protected,
                GamepadStatus::default(),
                &assets,
            );
        }
        export(
            &target,
            &output.join(if spent {
                "energy-spent.png"
            } else {
                "energy-ready.png"
            }),
        )?;
    }
    println!("Roster and flow review: {}", output.display());
    Ok(())
}

fn export(target: &RenderTexture2D, path: &Path) -> Result<(), Box<dyn Error>> {
    let mut image = target.texture().load_image()?;
    image.flip_vertical();
    image.export_image(path.to_str().ok_or("output path must be UTF-8")?);
    Ok(())
}
