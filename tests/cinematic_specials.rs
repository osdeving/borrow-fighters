//! Verifies Go/Python local cinematics and shared six-character preview controls.
//!
//! System: Combat integration. Full-screen presentation must not grant reach,
//! invulnerability, extra contacts, or stale state after interruption and reset.

use borrow_fighters::{
    characters::{CharacterId, character_spec},
    cli::{LaunchMode, LaunchOptions},
    combat::{
        fighter::{AttackKind, FighterInput, PlayerSlot},
        move_data::{MoveInputKind, move_spec_for_input},
    },
    config::{FIXED_TIMESTEP as DT, world_px},
    engine::sprites::{
        SpriteManifest, fighter_clip_elapsed_seconds, fighter_sprite_clip,
        frame_for_fighter_clip_at,
    },
    game::{
        combat_log::CombatLogKind,
        world::{World, WorldSpriteCombatManifests},
    },
    scenes::{
        combat_lab::{CombatLab, CombatLabInput, CombatLabMove, CombatLabOptions},
        move_showcase::{MoveShowcase, MoveShowcaseOptions, ShowcaseResult},
    },
};

const ROSTER: [CharacterId; 6] = [
    CharacterId::Rust,
    CharacterId::Duke,
    CharacterId::Go,
    CharacterId::C,
    CharacterId::Python,
    CharacterId::Cpp,
];

const LOCAL_CINEMATICS: [CharacterId; 2] = [CharacterId::Go, CharacterId::Python];

fn baseline(character: CharacterId) -> SpriteManifest {
    SpriteManifest::load(format!(
        "assets/placeholder/{}-fighter.sprite.json",
        character.audio_key()
    ))
    .unwrap()
}

fn arranged(character: CharacterId, reverse: bool, metadata: bool, gap: f32) -> World {
    let mut world = World::new_with_characters(character, CharacterId::Rust);
    let left = world_px(120.0);
    let right = left + world.player_one.body_rect().width + gap;
    world.player_one.position.x = if reverse { right } else { left };
    world.player_two.position.x = if reverse { left } else { right };
    if metadata {
        world.set_sprite_combat_manifests(WorldSpriteCombatManifests {
            player_one: Some(baseline(character)),
            player_two: Some(baseline(CharacterId::Rust)),
        });
    }
    world
}

fn special() -> FighterInput {
    FighterInput {
        cinematic_special: true,
        ..FighterInput::default()
    }
}

#[test]
fn go_and_python_land_once_locally_and_both_guard_heights_reduce_damage() {
    for character in LOCAL_CINEMATICS {
        for reverse in [false, true] {
            for metadata in [false, true] {
                for guard in [None, Some(false), Some(true)] {
                    let mut world = arranged(character, reverse, metadata, world_px(10.0));
                    let spec = move_spec_for_input(
                        character_spec(character).move_ids,
                        MoveInputKind::CinematicSpecial,
                    )
                    .unwrap();
                    let mut scene_seen = false;
                    for tick in 0..160 {
                        world.update(
                            DT,
                            if tick == 0 {
                                special()
                            } else {
                                FighterInput::default()
                            },
                            FighterInput {
                                block: guard.is_some(),
                                crouch: guard == Some(true),
                                ..FighterInput::default()
                            },
                        );
                        if let Some(state) = world.player_one.cinematic_special() {
                            scene_seen = true;
                            assert_eq!(state.character, character);
                            assert_eq!(state.move_id, spec.id);
                            assert!(state.progress() <= 1.0);
                        }
                    }
                    assert!(scene_seen);
                    let damage = world.player_two.max_health - world.player_two.health;
                    if guard.is_none() {
                        assert_eq!(damage, spec.damage, "{character:?}/{reverse}/{metadata}");
                    } else {
                        assert!(
                            damage > 0 && damage < spec.damage,
                            "mid strike must respect either guard height"
                        );
                    }
                    assert_eq!(world.player_one.health, world.player_one.max_health);
                    assert!(world.player_one.cinematic_special().is_none());
                    assert!(world.projectiles.is_empty() && world.signature_effects.is_empty());
                    let contacts = world
                        .combat_log()
                        .iter()
                        .filter(|event| {
                            matches!(
                                event.kind,
                                CombatLogKind::CloseAttackResolved {
                                    attacker: PlayerSlot::One,
                                    ..
                                }
                            )
                        })
                        .count();
                    assert_eq!(contacts, 1);
                }
            }
        }
    }
}

#[test]
fn spectacle_cannot_damage_a_distant_opponent_or_spawn_a_travelling_hitbox() {
    for character in LOCAL_CINEMATICS {
        for reverse in [false, true] {
            for metadata in [false, true] {
                let mut world = arranged(character, reverse, metadata, world_px(490.0));
                for tick in 0..170 {
                    world.update(
                        DT,
                        if tick == 0 {
                            special()
                        } else {
                            FighterInput::default()
                        },
                        FighterInput::default(),
                    );
                    assert_eq!(world.player_two.health, world.player_two.max_health);
                    assert!(world.projectiles.is_empty() && world.signature_effects.is_empty());
                    if let Some(hit) = world.player_one.active_attack() {
                        assert!(hit.hitbox.width <= world_px(145.0));
                    }
                }
                assert!(
                    world.combat_log().iter().any(|event| matches!(
                        event.kind,
                        CombatLogKind::CloseAttackWhiffed { .. }
                    ))
                );
            }
        }
    }
}

#[test]
fn startup_jab_interrupts_the_entire_cinematic_and_cannot_leave_delayed_damage() {
    for character in LOCAL_CINEMATICS {
        for reverse in [false, true] {
            let mut world = arranged(character, reverse, true, world_px(10.0));
            world.update(DT, special(), FighterInput::default());
            assert!(world.player_one.cinematic_special().is_some());
            let mut interrupted = false;
            for tick in 0..150 {
                world.update(
                    DT,
                    FighterInput::default(),
                    FighterInput {
                        light_punch: tick == 7,
                        ..FighterInput::default()
                    },
                );
                if world.player_one.health < world.player_one.max_health {
                    interrupted = true;
                    assert!(world.player_one.cinematic_special().is_none());
                }
            }
            assert!(interrupted);
            assert_eq!(world.player_two.health, world.player_two.max_health);
        }
    }
}

#[test]
fn commitment_blocks_movement_and_other_actions_and_reset_discards_the_clock() {
    for character in LOCAL_CINEMATICS {
        let mut world = arranged(character, false, false, world_px(490.0));
        world.update(
            DT,
            FighterInput {
                signature_special: true,
                block: true,
                light_punch: true,
                ..special()
            },
            FighterInput::default(),
        );
        assert_eq!(
            world.player_one.attack_kind(),
            Some(AttackKind::CinematicSpecial)
        );
        assert!(!world.player_one.blocking);
        let x = world.player_one.position.x;
        for _ in 0..80 {
            world.update(
                DT,
                FighterInput {
                    right: true,
                    jump: true,
                    projectile: true,
                    light_punch: true,
                    ..special()
                },
                FighterInput::default(),
            );
            assert_eq!(world.player_one.position.x, x);
            assert!(world.player_one.grounded);
            assert_eq!(
                world.player_one.attack_kind(),
                Some(AttackKind::CinematicSpecial)
            );
            assert!(world.projectiles.is_empty());
        }
        world = World::new_with_characters(character, CharacterId::Rust);
        assert!(world.player_one.cinematic_special().is_none());
        world.update(DT, special(), FighterInput::default());
        assert_eq!(
            world.player_one.cinematic_special().unwrap().elapsed_frames,
            1
        );
    }
}

#[test]
fn lab_and_showcase_expose_all_six_and_pause_keeps_the_same_clock() {
    for character in ROSTER {
        let parsed = LaunchOptions::parse(
            [
                "game",
                "--showcase",
                "--character",
                character.audio_key(),
                "--move",
                "cinematic_special",
                "--repeat",
            ]
            .map(String::from),
        )
        .unwrap();
        assert!(
            matches!(parsed.mode, LaunchMode::MoveShowcase(options) if options.selected_move == CombatLabMove::CinematicSpecial)
        );
        let mut lab = CombatLab::new(CombatLabOptions {
            character,
            selected_move: CombatLabMove::CinematicSpecial,
            ..CombatLabOptions::default()
        });
        lab.update(CombatLabInput::default());
        assert!(
            lab.fighter().cinematic_special().is_some()
                || lab
                    .super_preview_world()
                    .is_some_and(|world| world.super_sequence_active())
        );
        lab.update(CombatLabInput {
            pause_toggle: true,
            ..CombatLabInput::default()
        });
        let before = preview_tick(&lab);
        for _ in 0..60 {
            lab.update(CombatLabInput::default());
        }
        assert_eq!(preview_tick(&lab), before);
        lab.update(CombatLabInput {
            step_frame: true,
            ..CombatLabInput::default()
        });
        assert_ne!(preview_tick(&lab), before);
        for reverse in [false, true] {
            let mut scene = MoveShowcase::new(MoveShowcaseOptions { character });
            scene.select_move(CombatLabMove::CinematicSpecial);
            if reverse {
                scene.switch_sides();
            }
            for _ in 0..scene.scenario_frames() - 1 {
                scene.update(CombatLabInput::default());
                assert_ne!(
                    scene.result(),
                    ShowcaseResult::Whiff,
                    "authored preparation must never report an early miss"
                );
            }
            assert!(matches!(scene.result(), ShowcaseResult::Hit { damage } if damage > 0));
        }
    }
}

#[test]
fn reused_actor_poses_are_available_and_retimed_at_the_new_contact_frame() {
    for character in LOCAL_CINEMATICS {
        let mut world = arranged(character, false, false, world_px(490.0));
        let spec = move_spec_for_input(
            character_spec(character).move_ids,
            MoveInputKind::CinematicSpecial,
        )
        .unwrap();
        let source = move_spec_for_input(
            character_spec(character).move_ids,
            MoveInputKind::SignatureSpecial,
        )
        .or_else(|| {
            move_spec_for_input(
                character_spec(character).move_ids,
                MoveInputKind::HeavyPunch,
            )
        })
        .unwrap();
        let manifest = SpriteManifest::load(format!(
            "assets/candidates/{0}/{0}-fighter.sprite.json",
            character.audio_key()
        ))
        .unwrap();
        for tick in 0..spec.frames.duration.get() {
            world.update(
                DT,
                if tick == 0 {
                    special()
                } else {
                    FighterInput::default()
                },
                FighterInput::default(),
            );
            let fighter = &world.player_one;
            let elapsed = fighter_clip_elapsed_seconds(fighter, world.elapsed_seconds);
            assert!(
                frame_for_fighter_clip_at(&manifest, fighter_sprite_clip(fighter), elapsed)
                    .is_some()
            );
            if fighter.attack_elapsed_frames() == Some(spec.frames.active_start) {
                assert!((elapsed - source.frames.active_start.as_seconds()).abs() < 0.0001);
            }
        }
    }
}

#[test]
fn simultaneous_cinematics_trade_without_player_slot_priority() {
    for character in LOCAL_CINEMATICS {
        for reverse in [false, true] {
            let mut world = World::new_with_characters(character, character);
            let left = world_px(300.0);
            let right = left + world.player_one.body_rect().width + world_px(10.0);
            world.player_one.position.x = if reverse { right } else { left };
            world.player_two.position.x = if reverse { left } else { right };
            let damage = move_spec_for_input(
                character_spec(character).move_ids,
                MoveInputKind::CinematicSpecial,
            )
            .unwrap()
            .damage;
            for tick in 0..150 {
                let input = if tick == 0 {
                    special()
                } else {
                    FighterInput::default()
                };
                world.update(DT, input, input);
            }
            assert_eq!(
                world.player_one.max_health - world.player_one.health,
                damage
            );
            assert_eq!(
                world.player_two.max_health - world.player_two.health,
                damage
            );
            assert!(world.player_one.cinematic_special().is_none());
            assert!(world.player_two.cinematic_special().is_none());
        }
    }
}

fn preview_tick(lab: &CombatLab) -> u32 {
    lab.super_preview_world()
        .and_then(|world| world.super_sequence())
        .map_or_else(
            || lab.fighter().cinematic_special().unwrap().elapsed_frames,
            |sequence| sequence.tick,
        )
}
