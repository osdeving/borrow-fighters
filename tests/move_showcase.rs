//! Verifies that contextual examples really collide through match rules and stay reproducible.

use borrow_fighters::{
    characters::{CHARACTER_BODY_METRICS_PATH, CharacterBodyMetricsCatalog, CharacterId},
    combat::fighter::PlayerSlot,
    engine::sprites::SpriteManifest,
    game::{combat_log::CombatLogKind, world::WorldSpriteCombatManifests},
    scenes::{
        combat_lab::{CombatLabInput, CombatLabMove},
        move_showcase::{
            MoveShowcase, MoveShowcaseOptions, SCENARIO_FRAMES, ShowcaseResult, ShowcaseScenario,
        },
    },
};

const ROSTER: [CharacterId; 5] = [
    CharacterId::Rust,
    CharacterId::Duke,
    CharacterId::C,
    CharacterId::Python,
    CharacterId::Cpp,
];

fn baseline(character: CharacterId) -> SpriteManifest {
    SpriteManifest::load(format!(
        "assets/placeholder/{}-fighter.sprite.json",
        character.audio_key()
    ))
    .unwrap()
}

fn showcase(character: CharacterId, with_metadata: bool) -> MoveShowcase {
    let mut showcase = MoveShowcase::new(MoveShowcaseOptions { character });
    showcase
        .set_body_metrics(CharacterBodyMetricsCatalog::load(CHARACTER_BODY_METRICS_PATH).unwrap());
    if with_metadata {
        showcase.set_sprite_combat_manifests(WorldSpriteCombatManifests {
            player_one: Some(baseline(character)),
            player_two: Some(baseline(showcase.opponent_character())),
        });
    }
    showcase
}

#[test]
fn every_public_move_hits_in_its_context_on_both_sides_with_match_metadata_and_fallback() {
    let mut failures = Vec::new();
    for metadata in [false, true] {
        for character in ROSTER {
            for selected in CombatLabMove::ALL {
                for reversed in [false, true] {
                    let mut scene = showcase(character, metadata);
                    scene.select_move(selected);
                    if reversed {
                        scene.switch_sides();
                    }
                    let mut airborne_contact = false;
                    let mut reaction_seen = false;
                    let mut previous_result = ShowcaseResult::Pending;
                    for _ in 0..scene.scenario_frames() - 1 {
                        scene.update(CombatLabInput::default());
                        if previous_result == ShowcaseResult::Pending
                            && matches!(scene.result(), ShowcaseResult::Hit { .. })
                        {
                            airborne_contact = !scene.world().player_two.grounded;
                        }
                        reaction_seen |= scene.world().player_two.in_hitstun()
                            || scene.world().player_two.in_air_reaction()
                            || scene.world().player_two.in_capture();
                        previous_result = scene.result();
                    }
                    if !matches!(scene.result(), ShowcaseResult::Hit { damage } if damage > 0) {
                        failures.push(format!("{character:?}/{selected:?} metadata={metadata} reversed={reversed}: {:?}, log={:?}", scene.result(), scene.world().combat_log()));
                        continue;
                    }
                    assert!(
                        reaction_seen,
                        "{character:?}/{selected:?}: missing hit reaction"
                    );
                    if selected == CombatLabMove::AntiAir {
                        assert!(
                            airborne_contact,
                            "{character:?}/{selected:?}: anti-air must contact before landing"
                        );
                    }
                    let ShowcaseResult::Hit { damage } = scene.result() else {
                        unreachable!()
                    };
                    assert_eq!(
                        scene.world().player_two.max_health - scene.world().player_two.health,
                        damage
                    );
                    assert_eq!(
                        scene.world().player_one.health,
                        scene.world().player_one.max_health
                    );
                    let contacts = scene
                        .world()
                        .combat_log()
                        .iter()
                        .filter(|event| {
                            matches!(
                                event.kind,
                                CombatLogKind::CloseAttackResolved {
                                    attacker: PlayerSlot::One,
                                    ..
                                } | CombatLogKind::ProjectileResolved {
                                    attacker: PlayerSlot::One,
                                    ..
                                }
                            )
                        })
                        .count();
                    let expected = if selected == CombatLabMove::CinematicSpecial {
                        borrow_fighters::combat::super_sequence::super_spec(character)
                            .map_or(1, |spec| spec.contacts.len())
                    } else if selected == CombatLabMove::SignatureSpecial
                        && character == CharacterId::Duke
                    {
                        3
                    } else {
                        1
                    };
                    assert_eq!(contacts, expected, "each authored emission resolves once");
                }
            }
        }
    }
    assert!(
        failures.is_empty(),
        "{} scenario failures:\n{}",
        failures.len(),
        failures.join("\n")
    );
}

#[test]
fn each_character_demonstrates_real_standing_low_overhead_and_projectile_defense() {
    for character in ROSTER {
        for reversed in [false, true] {
            for offset in 0..4 {
                let mut scene = showcase(character, true);
                if reversed {
                    scene.switch_sides();
                }
                for _ in 0..CombatLabMove::ALL.len() + offset {
                    scene.update(CombatLabInput {
                        next_move: true,
                        ..CombatLabInput::default()
                    });
                }
                let selected_scenario = scene.scenario();
                assert!(selected_scenario.is_defense());
                let mut block_reaction = false;
                let mut crouching_reaction = false;
                for _ in 0..189 {
                    scene.update(CombatLabInput::default());
                    block_reaction |= scene.world().player_one.in_blockstun();
                    crouching_reaction |= scene.world().player_one.in_blockstun()
                        && scene.world().player_one.crouching;
                }
                assert!(
                    matches!(scene.result(), ShowcaseResult::Blocked { .. }),
                    "{character:?}/{selected_scenario:?}: {:?}, {:?}",
                    scene.result(),
                    scene.world().combat_log()
                );
                assert!(block_reaction);
                assert_eq!(
                    crouching_reaction,
                    selected_scenario == ShowcaseScenario::CrouchingBlock
                );
                assert_eq!(
                    scene.world().player_two.health,
                    scene.world().player_two.max_health
                );
            }
        }
    }
}

#[test]
fn fortress_and_vortex_remain_visible_before_the_approaching_opponent_is_hit() {
    let mut observations = Vec::new();
    for character in [CharacterId::Rust, CharacterId::Python] {
        for metadata in [false, true] {
            for reversed in [false, true] {
                let mut scene = showcase(character, metadata);
                scene.select_move(CombatLabMove::SignatureSpecial);
                if reversed {
                    scene.switch_sides();
                }
                let mut visible_before_contact = 0;
                for _ in 0..SCENARIO_FRAMES {
                    scene.update(CombatLabInput::default());
                    if scene
                        .world()
                        .signature_effects
                        .iter()
                        .any(|effect| !effect.has_connected())
                    {
                        assert_eq!(scene.result(), ShowcaseResult::Pending);
                        visible_before_contact += 1;
                    }
                }
                observations.push((character, metadata, reversed, visible_before_contact));
                assert!(matches!(scene.result(), ShowcaseResult::Hit { damage } if damage > 0));
                let contacts = scene
                    .world()
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
    assert!(
        observations
            .iter()
            .all(|(_, _, _, ticks)| (7..=12).contains(ticks)),
        "expected 7–12 visible ticks before contact: {observations:?}"
    );
}

#[test]
fn pause_frame_step_replay_and_side_switch_preserve_deterministic_results() {
    let mut scene = showcase(CharacterId::Python, true);
    scene.select_move(CombatLabMove::Sweep);
    scene.update(CombatLabInput {
        pause_toggle: true,
        ..CombatLabInput::default()
    });
    assert!(scene.paused());
    assert_eq!(scene.current_frame(), 0);
    scene.update(CombatLabInput::default());
    assert_eq!(scene.current_frame(), 0);
    scene.update(CombatLabInput {
        step_frame: true,
        ..CombatLabInput::default()
    });
    assert_eq!(scene.current_frame(), 1);
    scene.update(CombatLabInput {
        reset: true,
        ..CombatLabInput::default()
    });
    assert_eq!(scene.current_frame(), 0);
    scene.update(CombatLabInput {
        pause_toggle: true,
        ..CombatLabInput::default()
    });
    for _ in 0..170 {
        scene.update(CombatLabInput::default());
    }
    let result = scene.result();
    let events = scene.world().combat_log().to_vec();
    scene.update(CombatLabInput {
        replay: true,
        ..CombatLabInput::default()
    });
    for _ in 0..170 {
        scene.update(CombatLabInput::default());
    }
    assert_eq!(scene.result(), result);
    assert_eq!(scene.world().combat_log(), events);
    scene.switch_sides();
    assert!(scene.world().player_one.position.x > scene.world().player_two.position.x);
    for _ in 0..171 {
        scene.update(CombatLabInput::default());
    }
    assert_eq!(scene.result(), result);
}

#[test]
fn cycle_and_repeat_reset_time_result_and_health() {
    let mut scene = showcase(CharacterId::Rust, true);
    let first_count = scene.move_count();
    assert_eq!(first_count, CombatLabMove::ALL.len() + 4);
    for _ in 0..SCENARIO_FRAMES {
        scene.update(CombatLabInput::default());
    }
    assert!(matches!(scene.result(), ShowcaseResult::Hit { .. }));
    scene.update(CombatLabInput::default());
    assert_eq!(scene.selected_move(), CombatLabMove::HeavyPunch);
    assert!(scene.resting());
    assert_eq!(scene.current_frame(), 0);
    assert_eq!(scene.result(), ShowcaseResult::Pending);
    assert_eq!(
        scene.world().player_two.health,
        scene.world().player_two.max_health
    );
    scene.toggle_repeat();
    for _ in 0..=SCENARIO_FRAMES {
        scene.update(CombatLabInput::default());
    }
    assert_eq!(scene.selected_move(), CombatLabMove::HeavyPunch);
    assert_eq!(scene.current_frame(), 0);
    scene.update(CombatLabInput {
        previous_move: true,
        ..CombatLabInput::default()
    });
    assert_eq!(scene.selected_move(), CombatLabMove::LightPunch);
    scene.update(CombatLabInput {
        previous_move: true,
        ..CombatLabInput::default()
    });
    assert_eq!(scene.scenario(), ShowcaseScenario::ProjectileBlock);
}
