//! Verifies prototype character registry data.

use borrow_fighters::characters::{
    CHARACTER_BODY_METRICS_PATH, CharacterArchetype, CharacterBodyMetricsCatalog, CharacterId,
    character_spec,
};
use borrow_fighters::combat::move_data::MoveId;
use borrow_fighters::combat::projectile::{
    C_PROJECTILE_SPEC, DUKE_PROJECTILE_SPEC, GO_PROJECTILE_SPEC, RUST_PROJECTILE_SPEC,
};
use borrow_fighters::game::world::World;

#[test]
fn rust_spec_points_to_current_prototype_moves() {
    let rust = character_spec(CharacterId::Rust);

    assert_eq!(rust.display_name, "Rust");
    assert_eq!(rust.fighter_name, "Rust");
    assert_eq!(rust.archetype, CharacterArchetype::AllRounder);
    assert_eq!(rust.stats.max_health, 100);
    assert_eq!(
        rust.move_ids,
        &[
            MoveId::RustBorrowJab,
            MoveId::HeavyPunch,
            MoveId::Kick,
            MoveId::SweepKick,
            MoveId::OverheadPunch,
            MoveId::RustLifetimeAntiAir,
            MoveId::AirPunch,
            MoveId::AirKick,
            MoveId::RustOwnershipThrow,
        ]
    );
}

#[test]
fn duke_spec_points_to_current_prototype_moves() {
    let duke = character_spec(CharacterId::Duke);

    assert_eq!(duke.display_name, "Duke / Java");
    assert_eq!(duke.fighter_name, "Java");
    assert_eq!(duke.archetype, CharacterArchetype::MidrangePressure);
    assert_eq!(duke.stats.max_health, 112);
    assert_eq!(
        duke.move_ids,
        &[
            MoveId::LightPunch,
            MoveId::DukeBoilerplatePoke,
            MoveId::Kick,
            MoveId::DukeGarbageCollectorSweep,
            MoveId::DukeAbstractFactoryOverhead,
            MoveId::RisingAntiAir,
            MoveId::AirPunch,
            MoveId::AirKick,
            MoveId::DukeEnterpriseThrow,
        ]
    );
}

#[test]
fn go_spec_points_to_current_prototype_moves() {
    let go = character_spec(CharacterId::Go);

    assert_eq!(go.display_name, "Go");
    assert_eq!(go.fighter_name, "Go");
    assert_eq!(go.archetype, CharacterArchetype::Rushdown);
    assert_eq!(go.stats.max_health, 92);
    assert_eq!(
        go.move_ids,
        &[
            MoveId::GoGoroutineJab,
            MoveId::HeavyPunch,
            MoveId::GoDeferKick,
            MoveId::SweepKick,
            MoveId::GoChannelOverhead,
            MoveId::RisingAntiAir,
            MoveId::AirPunch,
            MoveId::GoHopkick,
            MoveId::CloseThrow,
        ]
    );
}

#[test]
fn c_spec_points_to_current_prototype_moves() {
    let c = character_spec(CharacterId::C);

    assert_eq!(c.display_name, "C");
    assert_eq!(c.fighter_name, "C");
    assert_eq!(c.archetype, CharacterArchetype::MidrangePressure);
    assert_eq!(c.stats.max_health, 104);
    assert_eq!(
        c.move_ids,
        &[
            MoveId::LightPunch,
            MoveId::HeavyPunch,
            MoveId::Kick,
            MoveId::SweepKick,
            MoveId::OverheadPunch,
            MoveId::RisingAntiAir,
            MoveId::AirPunch,
            MoveId::AirKick,
            MoveId::CloseThrow,
        ]
    );
}

#[test]
fn character_body_metrics_manifest_loads_go_as_leaner_mascot_body() {
    let catalog = CharacterBodyMetricsCatalog::load(CHARACTER_BODY_METRICS_PATH)
        .expect("character body metrics should load");
    let rust = catalog.body_metrics_for(CharacterId::Rust);
    let go = catalog.body_metrics_for(CharacterId::Go);

    assert_eq!(rust, character_spec(CharacterId::Rust).body_metrics);
    assert_eq!(go, character_spec(CharacterId::Go).body_metrics);
    assert_eq!(go.width, rust.width);
    assert_eq!(go.standing_height, rust.standing_height);
    assert_eq!(go.crouch_height, rust.crouch_height);
}

#[test]
fn world_uses_loaded_character_body_metrics() {
    let catalog = CharacterBodyMetricsCatalog::from_json_str(
        r#"
        {
          "schema": "borrow-fighters.character-body-metrics.v1",
          "characters": [
            {
              "id": "go",
              "body": {
                "width": 104.0,
                "standing_height": 150.0,
                "crouch_height": 82.0
              }
            }
          ]
        }
        "#,
    )
    .expect("inline body metrics should parse");

    let world =
        World::new_with_character_body_metrics(CharacterId::Go, CharacterId::Rust, &catalog);

    assert_eq!(world.player_one.body_metrics().width, 104.0);
    assert_eq!(world.player_one.body_rect().width, 104.0);
    assert_eq!(world.player_one.body_rect().height, 150.0);
}

#[test]
fn character_cli_aliases_are_stable() {
    assert_eq!(CharacterId::from_cli("rust"), Some(CharacterId::Rust));
    assert_eq!(CharacterId::from_cli("rustacean"), Some(CharacterId::Rust));
    assert_eq!(CharacterId::from_cli("duke"), Some(CharacterId::Duke));
    assert_eq!(CharacterId::from_cli("java"), Some(CharacterId::Duke));
    assert_eq!(CharacterId::from_cli("go"), Some(CharacterId::Go));
    assert_eq!(CharacterId::from_cli("golang"), Some(CharacterId::Go));
    assert_eq!(CharacterId::from_cli("gopher"), Some(CharacterId::Go));
    assert_eq!(CharacterId::from_cli("c"), Some(CharacterId::C));
    assert_eq!(CharacterId::from_cli("langc"), Some(CharacterId::C));
    assert_eq!(CharacterId::from_audio_key("go"), Some(CharacterId::Go));
    assert_eq!(CharacterId::from_audio_key("c"), Some(CharacterId::C));
}

#[test]
fn character_roster_cycles_in_menu_order() {
    assert_eq!(CharacterId::Rust.next(), CharacterId::Duke);
    assert_eq!(CharacterId::Duke.next(), CharacterId::Go);
    assert_eq!(CharacterId::Go.next(), CharacterId::C);
    assert_eq!(CharacterId::C.next(), CharacterId::Rust);
    assert_eq!(CharacterId::Rust.previous(), CharacterId::C);
    assert_eq!(CharacterId::Duke.previous(), CharacterId::Rust);
    assert_eq!(CharacterId::Go.previous(), CharacterId::Duke);
    assert_eq!(CharacterId::C.previous(), CharacterId::Go);
}

#[test]
fn character_projectiles_follow_archetype_intent() {
    let rust = character_spec(CharacterId::Rust).projectile;
    let duke = character_spec(CharacterId::Duke).projectile;
    let go = character_spec(CharacterId::Go).projectile;
    let c = character_spec(CharacterId::C).projectile;

    assert_eq!(rust, RUST_PROJECTILE_SPEC);
    assert_eq!(duke, DUKE_PROJECTILE_SPEC);
    assert_eq!(go, GO_PROJECTILE_SPEC);
    assert_eq!(c, C_PROJECTILE_SPEC);

    assert!(duke.damage > rust.damage);
    assert!(duke.speed < rust.speed);
    assert!(duke.frame_data.cooldown > rust.frame_data.cooldown);
    assert!(duke.width > rust.width);

    assert!(go.damage < rust.damage);
    assert!(go.speed > rust.speed);
    assert!(go.frame_data.cooldown < rust.frame_data.cooldown);
    assert!(go.max_travel.is_some());

    assert_eq!(c.damage, rust.damage);
    assert!(c.speed > rust.speed);
    assert!(c.speed < go.speed);
    assert!(c.width > rust.width);
    assert_eq!(c.height, rust.height);
    assert_eq!(c.max_travel, None);
}
