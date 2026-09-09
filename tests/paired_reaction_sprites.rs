//! Verifies the Python/C++ authored drawings at each real contact and recovery phase.
//!
//! System: Sprite/combat integration. Optional reaction artwork must advance within
//! each impact window while the original combat metadata and fallback stay intact.

use std::{collections::BTreeSet, fs::File, io::Read};

use borrow_fighters::{
    characters::{CHARACTER_BODY_METRICS_PATH, CharacterBodyMetricsCatalog, CharacterId},
    combat::{
        fighter::{ContactReactionProfile as Profile, FighterInput},
        super_sequence::{
            CPP_BARRAGE_CADENCE, CPP_BARRAGE_HITS, CPP_BARRAGE_START, cpp_barrage_pose_index,
            super_spec,
        },
    },
    config::{FIXED_TIMESTEP as DT, world_px},
    engine::sprites::{
        SpriteCombatBox, SpriteFrameCombat, SpriteManifest, contact_reaction_clip_name,
        fighter_sprite_clip, frame_for_contact_reaction, frame_for_fighter_state,
        projected_fighter_combat,
    },
    game::world::{World, WorldSpriteCombatManifests},
    scenes::{
        combat_lab::{CombatLabInput, CombatLabMove},
        move_showcase::{MoveShowcase, MoveShowcaseOptions, ShowcaseScenario},
    },
};

const PILOTS: [CharacterId; 2] = [CharacterId::Python, CharacterId::Cpp];
const PROFILES: [Profile; 8] = [
    Profile::Head,
    Profile::Body,
    Profile::Low,
    Profile::GuardHigh,
    Profile::GuardLow,
    Profile::Launch,
    Profile::Fall,
    Profile::Rise,
];

fn candidate(character: CharacterId) -> SpriteManifest {
    SpriteManifest::load(candidate_path(character)).unwrap()
}

fn candidate_path(character: CharacterId) -> String {
    format!(
        "assets/candidates/{0}/{0}-fighter.sprite.json",
        character.audio_key()
    )
}

fn baseline(character: CharacterId) -> SpriteManifest {
    SpriteManifest::load(format!(
        "assets/placeholder/{}-fighter.sprite.json",
        character.audio_key()
    ))
    .unwrap()
}

fn without_optional_reactions(manifest: &SpriteManifest) -> SpriteManifest {
    let mut fallback = manifest.clone();
    fallback
        .frames
        .retain(|frame| !frame.clip.starts_with("reaction_"));
    fallback
        .clips
        .retain(|clip| !clip.name.starts_with("reaction_"));
    fallback.validate().unwrap();
    fallback
}

fn special() -> FighterInput {
    FighterInput {
        cinematic_special: true,
        ..FighterInput::default()
    }
}

fn tick(world: &mut World) {
    world.update(DT, FighterInput::default(), FighterInput::default());
}

#[test]
fn both_pilots_have_four_distinct_drawings_per_profile_without_new_combat_boxes() {
    for character in PILOTS {
        let manifest = candidate(character);
        let mut names = BTreeSet::new();
        for profile in PROFILES {
            let clip_name = contact_reaction_clip_name(profile);
            let clip = manifest
                .clip_named(clip_name)
                .unwrap_or_else(|| panic!("{character:?}: missing {clip_name}"));
            assert_eq!(clip.frames.len(), 4, "{character:?}/{clip_name}");
            assert!(
                !clip.r#loop,
                "recovery must hold rather than loop to impact"
            );
            let mut sources = BTreeSet::new();
            for name in &clip.frames {
                assert!(
                    names.insert(name.clone()),
                    "reaction profiles reuse the same drawing key"
                );
                let frame = manifest.frame_named(name).unwrap();
                assert_eq!(frame.clip, clip_name);
                assert!(
                    frame.combat.is_none(),
                    "optional art must not introduce collision data"
                );
                let path = manifest.image_path_for_frame(candidate_path(character), frame);
                let mut header = [0; 26];
                File::open(&path)
                    .unwrap_or_else(|error| panic!("{}: {error}", path.display()))
                    .read_exact(&mut header)
                    .unwrap();
                assert_eq!(&header[..8], b"\x89PNG\r\n\x1a\n");
                assert_eq!(header[25], 6, "reaction sheets require RGBA");
                let width = u32::from_be_bytes(header[16..20].try_into().unwrap()) as i32;
                let height = u32::from_be_bytes(header[20..24].try_into().unwrap()) as i32;
                assert!(frame.frame.x >= 0 && frame.frame.y >= 0);
                assert!(frame.frame.x + frame.frame.w <= width);
                assert!(frame.frame.y + frame.frame.h <= height);
                sources.insert((
                    path,
                    frame.frame.x,
                    frame.frame.y,
                    frame.frame.w,
                    frame.frame.h,
                ));
            }
            assert_eq!(
                sources.len(),
                4,
                "{character:?}/{clip_name}: static source reused"
            );
        }
        assert_eq!(names.len(), 32);
    }
}

#[test]
fn every_cpp_barrage_hit_starts_the_correct_python_impact_and_plays_all_four_drawings() {
    let manifest = candidate(CharacterId::Python);
    let fallback = without_optional_reactions(&manifest);
    for reversed in [false, true] {
        let mut world = World::new_with_characters(CharacterId::Cpp, CharacterId::Python);
        world.player_one.position.x = if reversed { 820.0 } else { 420.0 };
        world.player_two.position.x = if reversed {
            world.player_one.position.x - world.player_two.body_rect().width - world_px(4.0)
        } else {
            world.player_one.body_rect().right() + world_px(4.0)
        };
        world.update(DT, special(), FighterInput::default());
        for _ in 1..CPP_BARRAGE_START {
            tick(&mut world);
        }
        for beat in 0..CPP_BARRAGE_HITS {
            let expected_profile = Profile::Head;
            let expected_clip = manifest
                .clip_named(contact_reaction_clip_name(expected_profile))
                .unwrap();
            let mut seen = BTreeSet::new();
            for age in 0..CPP_BARRAGE_CADENCE {
                let previous_health = world.player_two.health;
                tick(&mut world);
                let sequence = world.super_sequence().unwrap();
                assert_eq!(
                    sequence.tick,
                    CPP_BARRAGE_START + beat * CPP_BARRAGE_CADENCE + age
                );
                assert_eq!(
                    cpp_barrage_pose_index(sequence.tick),
                    [1, 3, 2, 4][beat as usize % 4]
                );
                let target = &world.player_two;
                let reaction = target.contact_reaction_state().unwrap();
                assert_eq!(reaction.profile, expected_profile);
                let drawn = frame_for_fighter_state(
                    &manifest,
                    target,
                    fighter_sprite_clip(target),
                    world.elapsed_seconds,
                )
                .unwrap();
                assert_eq!(drawn.clip, expected_clip.name);
                if age == 0 {
                    assert_eq!(previous_health - target.health, 3);
                    assert_eq!(reaction.elapsed_seconds, 0.0);
                    assert_eq!(
                        drawn.name, expected_clip.frames[0],
                        "impact drawing lagged the damage tick"
                    );
                } else {
                    assert_eq!(previous_health, target.health);
                }
                seen.insert(drawn.name.clone());
                assert_eq!(
                    projected_fighter_combat(&manifest, target, world.elapsed_seconds),
                    projected_fighter_combat(&fallback, target, world.elapsed_seconds),
                    "visual replacement changed combat metadata sampling"
                );
            }
            assert_eq!(seen, expected_clip.frames.iter().cloned().collect());
        }
    }
}

#[test]
fn all_attacks_and_guard_scenarios_use_new_reactions_for_both_characters_with_legacy_fallback() {
    let mut cases = 0;
    let mut profiles_seen = BTreeSet::new();
    for character in PILOTS {
        let mut scenarios: Vec<_> = CombatLabMove::ALL
            .into_iter()
            .map(ShowcaseScenario::Attack)
            .collect();
        scenarios.extend([
            ShowcaseScenario::StandingBlock,
            ShowcaseScenario::OverheadBlock,
            ShowcaseScenario::CrouchingBlock,
            ShowcaseScenario::ProjectileBlock,
        ]);
        for scenario in scenarios {
            for reversed in [false, true] {
                let mut scene = MoveShowcase::new(MoveShowcaseOptions { character });
                scene.set_body_metrics(
                    CharacterBodyMetricsCatalog::load(CHARACTER_BODY_METRICS_PATH).unwrap(),
                );
                scene.set_sprite_combat_manifests(WorldSpriteCombatManifests {
                    player_one: Some(baseline(character)),
                    player_two: Some(baseline(scene.opponent_character())),
                });
                scene.select_scenario(scenario);
                if reversed {
                    scene.switch_sides();
                }
                let defender = if scenario.is_defense() {
                    character
                } else {
                    scene.opponent_character()
                };
                let manifest = candidate(defender);
                let fallback = without_optional_reactions(&manifest);
                let mut contacts = 0;
                let mut reaction_frames = 0;
                for _ in 0..scene.scenario_frames() - 1 {
                    let previous_health = if scenario.is_defense() {
                        scene.world().player_one.health
                    } else {
                        scene.world().player_two.health
                    };
                    scene.update(CombatLabInput::default());
                    let world = scene.world();
                    let target = if scenario.is_defense() {
                        &world.player_one
                    } else {
                        &world.player_two
                    };
                    if previous_health > target.health {
                        contacts += 1;
                    }
                    if let Some(reaction) = target.contact_reaction_state() {
                        reaction_frames += 1;
                        let expected_name = contact_reaction_clip_name(reaction.profile);
                        profiles_seen.insert((defender.audio_key(), expected_name));
                        let frame = frame_for_fighter_state(
                            &manifest,
                            target,
                            fighter_sprite_clip(target),
                            world.elapsed_seconds,
                        )
                        .unwrap();
                        assert_eq!(
                            frame.clip, expected_name,
                            "{character:?}/{scenario:?}/{reversed}"
                        );
                        if previous_health > target.health || target.in_capture() {
                            let source = manifest.clip_named(expected_name).unwrap();
                            assert_eq!(
                                frame.name, source.frames[0],
                                "{character:?}/{scenario:?}: impact must start immediately and stay held until capture releases"
                            );
                        }
                        assert!(frame_for_contact_reaction(&fallback, target).is_none());
                        let old_frame = frame_for_fighter_state(
                            &fallback,
                            target,
                            fighter_sprite_clip(target),
                            world.elapsed_seconds,
                        )
                        .unwrap();
                        assert!(!old_frame.clip.starts_with("reaction_"));
                        assert_eq!(
                            projected_fighter_combat(&manifest, target, world.elapsed_seconds),
                            projected_fighter_combat(&fallback, target, world.elapsed_seconds)
                        );
                    }
                }
                assert!(
                    contacts > 0 && reaction_frames > 0,
                    "scenario never reacted: {character:?}/{scenario:?}/{reversed}"
                );
                cases += 1;
            }
        }
    }
    assert_eq!(cases, 64);
    for character in PILOTS {
        for profile in PROFILES {
            assert!(
                profiles_seen
                    .contains(&(character.audio_key(), contact_reaction_clip_name(profile))),
                "{character:?}/{profile:?} was never rendered"
            );
        }
    }
}

#[test]
fn lethal_super_landing_holds_each_pilots_last_fall_drawing_without_recovery() {
    for attacker in PILOTS {
        let defender = if attacker == CharacterId::Cpp {
            CharacterId::Python
        } else {
            CharacterId::Cpp
        };
        let manifest = candidate(defender);
        let last_fall = manifest
            .clip_named("reaction_fall")
            .unwrap()
            .frames
            .last()
            .unwrap();
        let mut world = World::new_with_characters(attacker, defender);
        world.player_two.health = 1;
        world.update(DT, special(), FighterInput::default());
        for _ in 0..super_spec(attacker).unwrap().duration_frames + 60 {
            tick(&mut world);
        }
        assert!(world.outcome.is_some());
        for _ in 0..60 {
            tick(&mut world);
            let target = &world.player_two;
            let frame = frame_for_fighter_state(
                &manifest,
                target,
                fighter_sprite_clip(target),
                world.elapsed_seconds,
            )
            .unwrap();
            assert_eq!(frame.clip, "reaction_fall");
            assert_eq!(&frame.name, last_fall);
            assert!(target.grounded);
        }
    }
}

#[test]
fn authored_drawings_do_not_replace_independently_supplied_legacy_combat_metadata() {
    let mut manifest = candidate(CharacterId::Python);
    // Current candidate reaction frames have no boxes. Supply a non-empty
    // legacy contract so this assertion cannot pass by comparing two Nones.
    for frame in manifest
        .frames
        .iter_mut()
        .filter(|frame| frame.clip == "heavy_hit")
    {
        frame.combat = Some(SpriteFrameCombat {
            hurtboxes: vec![SpriteCombatBox {
                x: 1,
                y: 2,
                w: 3,
                h: 4,
                label: Some("legacy-hit-contract".into()),
            }],
            ..SpriteFrameCombat::default()
        });
    }
    manifest.validate().unwrap();
    let fallback = without_optional_reactions(&manifest);
    let mut world = World::new_with_characters(CharacterId::Cpp, CharacterId::Python);
    world.update(DT, special(), FighterInput::default());
    for _ in 0..CPP_BARRAGE_START {
        tick(&mut world);
    }
    let target = &world.player_two;
    let drawing = frame_for_contact_reaction(&manifest, target).unwrap();
    assert_eq!(drawing.clip, "reaction_head");
    let combat = projected_fighter_combat(&manifest, target, world.elapsed_seconds).unwrap();
    assert_eq!(combat.hurtboxes.len(), 1);
    assert_ne!(combat.frame_name, drawing.name);
    assert_eq!(
        Some(combat),
        projected_fighter_combat(&fallback, target, world.elapsed_seconds)
    );
}
