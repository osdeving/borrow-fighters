//! Exercises every supported attack against all six defenders through real World contacts.
//!
//! System: Combat/presentation regression tests. Damage, pose priority, local clocks,
//! frame coverage and mirrored recoil are checked together without a graphics device.

use borrow_fighters::{
    characters::{CharacterId, character_spec},
    combat::fighter::{Facing, FighterInput, HitReactionKind},
    combat::move_data::{GuardRule, LIGHT_ATTACK_REACTION, MoveInputKind, move_spec_for_input},
    config::{FIXED_TIMESTEP as DT, world_px},
    engine::sprites::{
        SpriteManifest, fighter_reaction_transform, fighter_sprite_clip, frame_for_fighter_state,
    },
    game::world::{World, WorldSpriteCombatManifests},
    scenes::combat_lab::CombatLabMove,
};
use std::collections::BTreeSet;

const ROSTER: [CharacterId; 6] = [
    CharacterId::Rust,
    CharacterId::Duke,
    CharacterId::Go,
    CharacterId::C,
    CharacterId::Python,
    CharacterId::Cpp,
];

fn input(selected: CombatLabMove, reverse: bool) -> FighterInput {
    use CombatLabMove::*;
    let mut input = FighterInput::default();
    match selected {
        LightPunch | AirPunch => input.light_punch = true,
        HeavyPunch => input.heavy_punch = true,
        Kick | AirKick => input.kick = true,
        Sweep => {
            input.crouch = true;
            input.kick = true;
        }
        Overhead => {
            input.heavy_punch = true;
            input.left = reverse;
            input.right = !reverse;
        }
        AntiAir => {
            input.heavy_punch = true;
            input.crouch = true;
        }
        Throw => {
            input.light_punch = true;
            input.block = true;
        }
        Projectile => input.projectile = true,
        SignatureSpecial => input.signature_special = true,
        CinematicSpecial => input.cinematic_special = true,
    }
    input
}

fn setup(
    attacker: CharacterId,
    defender: CharacterId,
    selected: CombatLabMove,
    reverse: bool,
) -> World {
    let mut world = World::new_with_characters(attacker, defender);
    let gap = if selected == CombatLabMove::Projectile {
        world_px(120.0)
    } else {
        world_px(4.0)
    };
    world.player_one.position.x = if reverse { 630.0 } else { 420.0 };
    world.player_two.position.x = if reverse {
        world.player_one.position.x - world.player_two.body_rect().width - gap
    } else {
        world.player_one.body_rect().right() + gap
    };
    world.player_one.facing = if reverse { Facing::Left } else { Facing::Right };
    world.player_two.facing = if reverse { Facing::Right } else { Facing::Left };
    if selected == CombatLabMove::AntiAir {
        world.player_two.grounded = false;
        world.player_two.position.y -= world_px(90.0);
    }
    if matches!(selected, CombatLabMove::AirPunch | CombatLabMove::AirKick) {
        world.player_one.grounded = false;
        world.player_one.position.y -= world_px(65.0);
        world.player_one.velocity.y = world_px(-120.0);
    }
    world
}

#[test]
fn every_move_reacts_with_every_defender_and_in_both_directions() {
    let mut cases = 0;
    for attacker in ROSTER {
        for defender in ROSTER {
            let manifest = SpriteManifest::load(format!(
                "assets/candidates/{0}/{0}-fighter.sprite.json",
                defender.audio_key()
            ))
            .unwrap();
            for selected in CombatLabMove::ALL {
                if selected == CombatLabMove::SignatureSpecial
                    && move_spec_for_input(
                        character_spec(attacker).move_ids,
                        MoveInputKind::SignatureSpecial,
                    )
                    .is_none()
                {
                    continue;
                }
                for (reverse, metadata) in
                    [(false, false), (true, false), (false, true), (true, true)]
                {
                    let context = format!(
                        "{attacker:?}/{selected:?} -> {defender:?} reversed={reverse} metadata={metadata}"
                    );
                    let mut world = setup(attacker, defender, selected, reverse);
                    if metadata {
                        world.set_sprite_combat_manifests(WorldSpriteCombatManifests {
                            player_one: Some(
                                SpriteManifest::load(format!(
                                    "assets/placeholder/{}-fighter.sprite.json",
                                    attacker.audio_key()
                                ))
                                .unwrap(),
                            ),
                            player_two: Some(
                                SpriteManifest::load(format!(
                                    "assets/placeholder/{}-fighter.sprite.json",
                                    defender.audio_key()
                                ))
                                .unwrap(),
                            ),
                        });
                    }
                    let mut frames = BTreeSet::new();
                    let mut transforms = BTreeSet::new();
                    let mut contact = false;
                    let mut reaction_ticks = 0;
                    for tick in 0..760 {
                        world.update(
                            DT,
                            if tick == 0 {
                                input(selected, reverse)
                            } else {
                                FighterInput::default()
                            },
                            FighterInput::default(),
                        );
                        let victim = &world.player_two;
                        if victim.health < victim.max_health {
                            contact = true;
                        }
                        if contact && victim.reaction_visual_state().is_some() {
                            reaction_ticks += 1;
                            assert!(
                                victim.attack_kind().is_none(),
                                "attack survived contact: {context}"
                            );
                            assert!(
                                victim.special_elapsed_seconds().is_none(),
                                "projectile survived contact: {context}"
                            );
                            let clip = fighter_sprite_clip(victim);
                            assert!(
                                matches!(
                                    clip.as_str(),
                                    "hit" | "heavy_hit" | "launched" | "thrown" | "knockdown"
                                ),
                                "neutral pose after contact: {context}/{clip:?}"
                            );
                            let frame = frame_for_fighter_state(
                                &manifest,
                                victim,
                                clip,
                                world.elapsed_seconds,
                            )
                            .unwrap();
                            frames.insert(frame.name.clone());
                            let transform = fighter_reaction_transform(
                                victim,
                                manifest.clip_named(clip.as_str()).is_some(),
                            );
                            transforms.insert(format!("{transform:?}"));
                        }
                        if contact && !world.super_sequence_active() && !victim.in_hitstun() {
                            break;
                        }
                    }
                    assert!(contact, "no contact: {context}");
                    assert!(reaction_ticks >= 6, "reaction too short: {context}");
                    assert!(
                        frames.len() >= 2,
                        "static atlas reaction: {context}/{frames:?}"
                    );
                    assert!(
                        transforms.len() >= 4,
                        "static recoil: {context}/{transforms:?}"
                    );
                    assert!(
                        world.player_two.grounded,
                        "flight failed to land: {context}"
                    );
                    cases += 1;
                }
            }
        }
    }
    assert_eq!(cases, 1704);
}

#[test]
fn contact_interrupts_attacks_and_projectiles_and_uses_all_authored_hit_keys_before_release() {
    for character in ROSTER {
        let manifest = SpriteManifest::load(format!(
            "assets/candidates/{0}/{0}-fighter.sprite.json",
            character.audio_key()
        ))
        .unwrap();
        let hit_clip = "reaction_body";
        let expected_frames: BTreeSet<_> = manifest
            .clip_named(hit_clip)
            .unwrap()
            .frames
            .iter()
            .cloned()
            .collect();
        for projectile in [false, true] {
            let mut world = setup(character, character, CombatLabMove::HeavyPunch, false);
            let fighter = &mut world.player_two;
            if projectile {
                fighter.mark_projectile_fired();
            } else {
                fighter.update(
                    DT,
                    FighterInput {
                        heavy_punch: true,
                        ..FighterInput::default()
                    },
                );
            }
            fighter.take_hit(8, GuardRule::Mid, LIGHT_ATTACK_REACTION);
            let mut frames = BTreeSet::new();
            while fighter.in_hitstun() {
                assert_eq!(fighter_sprite_clip(fighter).as_str(), "hit");
                frames.insert(
                    frame_for_fighter_state(
                        &manifest,
                        fighter,
                        fighter_sprite_clip(fighter),
                        999.0,
                    )
                    .unwrap()
                    .name
                    .clone(),
                );
                fighter.update(DT, FighterInput::default());
            }
            assert_eq!(
                frames, expected_frames,
                "all hit drawings must fit actual stun: {character:?}"
            );
            assert_eq!(fighter_sprite_clip(fighter).as_str(), "idle");
        }
    }
}

#[test]
fn mirrored_recoil_is_visual_only_and_guard_keeps_the_correct_height() {
    for character in ROSTER {
        for crouched in [false, true] {
            let mut world = setup(character, character, CombatLabMove::LightPunch, false);
            let fighter = &mut world.player_two;
            fighter.update(
                DT,
                FighterInput {
                    block: true,
                    crouch: crouched,
                    ..FighterInput::default()
                },
            );
            let result = fighter.take_hit(8, GuardRule::Mid, LIGHT_ATTACK_REACTION);
            assert!(result.blocked);
            fighter.update(DT, FighterInput::default());
            let before = fighter.body_rect();
            fighter.facing = Facing::Left;
            let right = fighter_reaction_transform(fighter, true);
            fighter.facing = Facing::Right;
            let left = fighter_reaction_transform(fighter, true);
            assert_eq!(left.offset.x, -right.offset.x);
            assert_eq!(left.rotation_degrees, -right.rotation_degrees);
            assert_eq!(before, fighter.body_rect());
            assert_eq!(
                fighter_sprite_clip(fighter).as_str(),
                if crouched { "crouch_block" } else { "block" }
            );
            assert!(fighter.reaction_visual_state().unwrap().guarded);
        }
    }
}

#[test]
fn airborne_damage_finishes_descent_and_floor_recovery_instead_of_returning_to_jump() {
    for character in ROSTER {
        let mut world = setup(character, character, CombatLabMove::LightPunch, false);
        let victim = &mut world.player_two;
        victim.grounded = false;
        victim.position.y -= world_px(100.0);
        victim.take_hit(8, GuardRule::Mid, LIGHT_ATTACK_REACTION);
        assert_eq!(victim.hit_reaction_kind(), HitReactionKind::Launched);
        let mut floor_seen = false;
        for _ in 0..120 {
            victim.update(DT, FighterInput::default());
            if victim.in_knockdown() {
                floor_seen = true;
                assert!(victim.reaction_visual_state().unwrap().landed);
            }
        }
        assert!(floor_seen && victim.grounded && !victim.in_hitstun());
    }
}

#[test]
fn lethal_super_lands_and_holds_each_characters_prone_drawing_without_getting_up() {
    for character in ROSTER {
        let manifest = SpriteManifest::load(format!(
            "assets/candidates/{0}/{0}-fighter.sprite.json",
            character.audio_key()
        ))
        .unwrap();
        let mut world = setup(
            CharacterId::Duke,
            character,
            CombatLabMove::CinematicSpecial,
            false,
        );
        world.player_two.health = 1;
        let mut prone_frame = None;
        for tick in 0..500 {
            world.update(
                DT,
                if tick == 0 {
                    input(CombatLabMove::CinematicSpecial, false)
                } else {
                    FighterInput::default()
                },
                FighterInput::default(),
            );
            if world.player_two.in_knockdown() {
                let fighter = &world.player_two;
                let frame = frame_for_fighter_state(
                    &manifest,
                    fighter,
                    fighter_sprite_clip(fighter),
                    world.elapsed_seconds,
                )
                .unwrap();
                if let Some(prone) = prone_frame.as_ref() {
                    assert_eq!(prone, &frame.name, "KO must not stand up: {character:?}");
                } else {
                    prone_frame = Some(frame.name.clone());
                }
            }
        }
        assert!(prone_frame.is_some() && world.outcome.is_some());
        assert!(world.player_two.grounded);
        let pose = fighter_reaction_transform(
            &world.player_two,
            manifest.clip_named("knockdown").is_some(),
        );
        if character == CharacterId::Go {
            // Preserve the procedural fallback when optional contact art is absent.
            assert!(pose.rotation_degrees.abs() > 80.0);
            // A lethal grounded sweep has no flight clock to advance before KO.
            let mut grounded_victim = world.player_two.clone();
            grounded_victim.start_knockdown();
            assert!(
                fighter_reaction_transform(&grounded_victim, false)
                    .rotation_degrees
                    .abs()
                    > 80.0
            );
        }
    }
}

#[test]
fn super_horizontal_launch_preserves_an_inner_margin_and_never_teleports_at_contact() {
    use borrow_fighters::config::{ARENA_LEFT, ARENA_RIGHT};
    for character in ROSTER {
        for reverse in [false, true] {
            let mut world = setup(
                CharacterId::Rust,
                character,
                CombatLabMove::CinematicSpecial,
                reverse,
            );
            world.player_two.position.x = if reverse {
                ARENA_LEFT + world_px(30.0)
            } else {
                ARENA_RIGHT - world.player_two.body_rect().width - world_px(30.0)
            };
            let initial = world.player_two.position.x;
            let mut previous_health = world.player_two.health;
            for tick in 0..420 {
                let previous_position = world.player_two.position.x;
                world.update(
                    DT,
                    if tick == 0 {
                        input(CombatLabMove::CinematicSpecial, reverse)
                    } else {
                        FighterInput::default()
                    },
                    FighterInput::default(),
                );
                if world.player_two.health < previous_health {
                    assert_eq!(
                        world.player_two.position.x, previous_position,
                        "contact cannot teleport the target"
                    );
                }
                previous_health = world.player_two.health;
            }
            let fighter = &world.player_two;
            assert!((fighter.position.x - initial).abs() <= world_px(6.1));
            assert!(fighter.position.x >= ARENA_LEFT + world_px(24.0) - 0.01);
            assert!(fighter.body_rect().right() <= ARENA_RIGHT - world_px(24.0) + 0.01);
        }
    }
}

#[test]
fn heavy_close_hit_does_not_overwrite_an_airborne_victims_ballistic_reaction() {
    for character in ROSTER {
        let mut world = setup(
            CharacterId::Duke,
            character,
            CombatLabMove::HeavyPunch,
            false,
        );
        world.player_two.position.y -= world_px(80.0);
        world.player_two.velocity.y = world_px(-200.0);
        world.player_two.grounded = false;
        let mut connected = false;
        for tick in 0..80 {
            world.update(
                DT,
                if tick == 0 {
                    input(CombatLabMove::HeavyPunch, false)
                } else {
                    FighterInput::default()
                },
                FighterInput::default(),
            );
            if world.player_two.health < world.player_two.max_health {
                connected = true;
                assert!(
                    world.player_two.in_air_reaction(),
                    "heavy override lost airborne state: {character:?}"
                );
                break;
            }
        }
        assert!(
            connected,
            "heavy should intercept airborne target: {character:?}"
        );
    }
}
