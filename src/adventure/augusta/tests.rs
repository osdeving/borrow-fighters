//! Verifies independent checkpoints against real production combat and content.

use super::*;
use crate::adventure::augusta::store;
use crate::adventure::{
    augusta_app::review,
    production::{CharacterPack, CombatCatalog, Input, Outcome, Team},
};
use std::{fs, path::PathBuf, sync::Arc};

fn chapter() -> Chapter {
    let directory =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets/adventure/chapters/cpp-augusta");
    let spec = ChapterSpec::load(&directory.join("chapter.json")).unwrap();
    let world = World::load(&directory.join(&spec.world)).unwrap();
    let texts = Texts::load(&directory.join(&spec.texts)).unwrap();
    let mut packs = vec![];
    for id in ["cpp", "security", "erratic"] {
        let actor = directory.join(format!("../../actors/{id}"));
        packs.push(
            CharacterPack::from_json(
                &fs::read_to_string(actor.join("character.json")).unwrap(),
                &fs::read_to_string(actor.join("combat.json")).unwrap(),
            )
            .unwrap(),
        );
    }
    Chapter::new(
        spec,
        world,
        texts,
        Arc::new(CombatCatalog::new(packs).unwrap()),
    )
    .unwrap()
}

#[test]
fn chapter_requires_real_two_wave_victories_and_respects_julias_choice() {
    let mut chapter = chapter();
    let mut phases = vec![];
    let mut defeats = 0;
    let mut guard_knockouts = 0;
    let mut erratic_knockouts = 0;
    let mut previous_threat_age = None;
    for _ in 0..24000 {
        let phase = chapter.phase();
        if phases.last() != Some(&phase) {
            phases.push(phase);
        }
        if chapter.defeated() {
            defeats += 1;
        }
        for event in chapter.tick(review::input(&chapter)).unwrap() {
            if let crate::adventure::production::Event::Knockout { actor, .. } = event
                && actor != chapter.simulation().player_id()
            {
                if phase == Phase::GuardsFight {
                    guard_knockouts += 1;
                }
                if phase == Phase::ErraticsFight {
                    erratic_knockouts += 1;
                }
            }
        }
        if let Some(age) = previous_threat_age {
            assert_eq!(chapter.threat_age(), Some(age + 1));
        }
        previous_threat_age = chapter.threat_age();
        if chapter.complete() {
            break;
        }
    }
    assert!(
        chapter.complete(),
        "stalled at {:?}, health {}, defeats {defeats}",
        chapter.phase(),
        chapter.simulation().player().hp
    );
    assert_eq!(
        defeats, 0,
        "review must win through normal inputs without retry loops"
    );
    assert_eq!(guard_knockouts, 3);
    assert_eq!(erratic_knockouts, 2);
    assert!(phases.contains(&Phase::JuliaAttempt));
    assert!(phases.contains(&Phase::ErraticsArrival));
    assert!(!phases.contains(&Phase::BrokerEscape));
    assert!(phases.contains(&Phase::RescueDialogue));
    assert_eq!(chapter.checkpoint().stage, CheckpointStage::Complete);
    assert_eq!(chapter.texts.speakers["julia"].age, Some(22));
    assert!(
        chapter
            .simulation()
            .actors()
            .iter()
            .all(|a| a.character != "julia" && a.character != "broker")
    );
}

#[test]
fn skip_cannot_win_encounters_and_checkpoint_recreates_both_flanks() {
    let mut chapter = chapter();
    chapter.checkpoint = Checkpoint {
        version: 1,
        stage: CheckpointStage::ErraticsFight,
    };
    chapter.retry().unwrap();
    let x = chapter.simulation().player().position.x;
    let enemies: Vec<_> = chapter
        .simulation()
        .actors()
        .iter()
        .filter(|a| a.team == Team::Enemy)
        .collect();
    assert_eq!(enemies.len(), 2);
    assert!(enemies[0].position.x < x && enemies[1].position.x > x);
    assert!(chapter.npcs().iter().all(|npc| npc.character != "broker"));
    for _ in 0..30 {
        chapter
            .tick(ChapterInput {
                skip: true,
                ..ChapterInput::default()
            })
            .unwrap();
    }
    assert_eq!(chapter.phase(), Phase::ErraticsFight);
    assert_eq!(chapter.simulation().outcome(), Outcome::Ongoing);
    chapter.retry().unwrap();
    assert_eq!(
        chapter.simulation().player().hp,
        chapter.simulation().player().max_hp
    );
    assert!(
        chapter
            .simulation()
            .actors()
            .iter()
            .all(|a| a.active && a.grounded && a.hp == a.max_hp)
    );
}

#[test]
fn arrival_keeps_ids_inactive_until_landing_and_freezes_without_ticks() {
    let mut chapter = chapter();
    chapter
        .simulation
        .stage_player(
            chapter.world.erratic_center_x,
            crate::adventure::production::Facing::Right,
        )
        .unwrap();
    chapter.start_wave(true, true).unwrap();
    let ids: Vec<_> = chapter
        .simulation()
        .actors()
        .iter()
        .filter(|a| a.team == Team::Enemy)
        .map(|a| a.id)
        .collect();
    let hp = chapter.simulation().player().hp;
    let duration = chapter.spec.timing.erratics_arrival_ticks;
    for tick in 0..duration {
        assert!(
            ids.iter()
                .all(|id| !chapter.simulation().actor(*id).unwrap().active)
        );
        let pose = chapter.arrival_pose(ids[0]).unwrap();
        assert!(pose.position.y <= chapter.world.ground_y);
        if tick >= duration * 3 / 4 {
            assert_eq!(pose.position.y, chapter.world.ground_y);
        }
        let before = (chapter.ticks(), pose.position);
        assert_eq!(
            before,
            (
                chapter.ticks(),
                chapter.arrival_pose(ids[0]).unwrap().position
            )
        );
        chapter
            .tick(ChapterInput {
                combat: Input {
                    light: true,
                    ..Input::default()
                },
                ..ChapterInput::default()
            })
            .unwrap();
    }
    assert_eq!(chapter.phase(), Phase::ErraticsFight);
    assert_eq!(chapter.simulation().player().hp, hp);
    assert!(
        ids.iter()
            .all(|id| chapter.simulation().actor(*id).unwrap().active)
    );
    assert_eq!(chapter.landing_age(), Some(duration - duration * 3 / 4));
    chapter.tick(ChapterInput::default()).unwrap();
    assert_eq!(chapter.landing_age(), Some(duration - duration * 3 / 4 + 1));
    chapter.retry().unwrap();
    assert_eq!(
        chapter.landing_age(),
        None,
        "retry must not invent another impact"
    );
}

#[test]
fn cpp_profile_roundtrip_does_not_write_legacy_rust_progress() {
    let directory =
        std::env::temp_dir().join(format!("borrow-augusta-save-{}", std::process::id()));
    fs::create_dir_all(&directory).unwrap();
    let rust = directory.join("campaign-v1.json");
    let sentinel = b"legacy Rust checkpoint remains byte-for-byte unchanged";
    fs::write(&rust, sentinel).unwrap();
    let path = directory.join("cpp-augusta-v1.json");
    let progress = store::Progress {
        checkpoint: Some(Checkpoint {
            version: 1,
            stage: CheckpointStage::Rescue,
        }),
        ..store::Progress::default()
    };
    store::save(&path, &progress).unwrap();
    assert_eq!(store::load(&path).unwrap(), progress);
    assert_eq!(fs::read(&rust).unwrap(), sentinel);
    let mut invalid = progress.clone();
    invalid.protagonist = "rust".into();
    assert!(store::save(&path, &invalid).is_err());
    assert_eq!(store::load(&path).unwrap(), progress);
    fs::remove_dir_all(directory).unwrap();
}

#[test]
fn approaching_during_a_jump_lands_before_dialogue_takes_control() {
    let mut chapter = chapter();
    chapter
        .tick(ChapterInput {
            skip: true,
            ..ChapterInput::default()
        })
        .unwrap();
    chapter
        .simulation
        .stage_player(
            chapter.world.confrontation_x - 125.0,
            crate::adventure::production::Facing::Right,
        )
        .unwrap();
    chapter
        .tick(ChapterInput {
            combat: Input {
                jump: true,
                movement: 1.0,
                ..Input::default()
            },
            ..ChapterInput::default()
        })
        .unwrap();
    let mut crossed_in_air = false;
    for _ in 0..100 {
        let player = chapter.simulation().player();
        if player.position.x > chapter.world.confrontation_x - 100.0 && !player.grounded {
            crossed_in_air = true;
            assert_eq!(chapter.phase(), Phase::Approach);
        }
        if chapter.phase() == Phase::JuliaAttempt {
            break;
        }
        chapter
            .tick(ChapterInput {
                combat: Input {
                    movement: 1.0,
                    ..Input::default()
                },
                ..ChapterInput::default()
            })
            .unwrap();
    }
    assert!(crossed_in_air);
    assert_eq!(chapter.phase(), Phase::JuliaAttempt);
    assert!(chapter.simulation().player().grounded);
    let x = chapter.simulation().player().position.x;
    for _ in 0..20 {
        chapter
            .tick(ChapterInput {
                combat: Input {
                    movement: 1.0,
                    jump: true,
                    light: true,
                    ..Input::default()
                },
                ..ChapterInput::default()
            })
            .unwrap();
    }
    assert_eq!(chapter.simulation().player().position.x, x);
    assert_eq!(chapter.simulation().player().clip_id(), "idle");
    assert!(chapter.simulation().player().action_ticks >= 20);
    chapter
        .tick(ChapterInput {
            skip: true,
            ..ChapterInput::default()
        })
        .unwrap();
    assert_eq!(chapter.phase(), Phase::Confrontation);
}

#[test]
fn reload_candidate_rebuilds_current_checkpoint_without_mutating_the_live_chapter() {
    let mut live = chapter();
    live.checkpoint.stage = CheckpointStage::ErraticsFight;
    live.retry().unwrap();
    for _ in 0..30 {
        live.tick(ChapterInput::default()).unwrap();
    }
    let original = (
        live.ticks(),
        live.phase(),
        live.simulation().player().position,
        live.simulation().player().hp,
    );
    let mut world = live.world.clone();
    world.erratic_center_x += 50.0;
    world.erratic_landings_x = [1500.0, 2200.0];
    let candidate = Chapter::from_checkpoint(
        live.spec.clone(),
        world.clone(),
        live.texts.clone(),
        live.simulation().content.clone(),
        live.checkpoint(),
    )
    .unwrap();
    assert_eq!(candidate.phase(), Phase::ErraticsFight);
    assert_eq!(
        candidate.simulation().player().position.x,
        world.erratic_center_x
    );
    assert_eq!(
        candidate.simulation().player().hp,
        candidate.simulation().player().max_hp
    );
    let mut bad_entry = live.spec.clone();
    bad_entry.title_key = "missing.chapter.title".into();
    assert!(
        Chapter::from_checkpoint(
            bad_entry,
            world,
            live.texts.clone(),
            live.simulation().content.clone(),
            live.checkpoint()
        )
        .is_err()
    );
    assert_eq!(
        (
            live.ticks(),
            live.phase(),
            live.simulation().player().position,
            live.simulation().player().hp
        ),
        original
    );
}

#[test]
fn julia_attempt_preserves_the_pair_axis_and_short_contact_on_both_approaches() {
    for approach_x in [1400.0, 1900.0] {
        let mut chapter = chapter();
        chapter.enter(Phase::Approach);
        chapter
            .simulation
            .stage_player(approach_x, Facing::Right)
            .unwrap();
        chapter.tick(ChapterInput::default()).unwrap();
        assert_eq!(chapter.phase(), Phase::JuliaAttempt);
        let player_x = chapter.simulation().player().position.x;
        let mut max_tension = 0.0_f32;
        let mut nearest_julia_x = chapter.world.julia_initial_x;
        for _ in 0..chapter.spec.timing.julia_attempt_ticks {
            let poses = chapter.npcs();
            let julia = poses.iter().find(|npc| npc.character == "julia").unwrap();
            let broker = poses.iter().find(|npc| npc.character == "broker").unwrap();
            assert!(player_x < broker.position.x && broker.position.x < julia.position.x);
            assert_eq!(julia.facing, Facing::Left);
            assert!(julia.depth > broker.depth);
            let contact = chapter.restraint_contact().unwrap();
            for (a, b) in [
                (contact.broker_shoulder, contact.broker_elbow),
                (contact.broker_elbow, contact.julia_wrist),
                (contact.julia_wrist, contact.julia_shoulder),
            ] {
                assert!((a.x - b.x).hypot(a.y - b.y) < 45.0);
            }
            assert_eq!(contact.julia_wrist.x, julia.position.x - 40.0);
            max_tension = max_tension.max(contact.tension);
            nearest_julia_x = nearest_julia_x.min(julia.position.x);
            chapter
                .tick(ChapterInput {
                    combat: Input {
                        movement: 1.0,
                        jump: true,
                        light: true,
                        ..Input::default()
                    },
                    ..ChapterInput::default()
                })
                .unwrap();
            assert_eq!(chapter.simulation().player().position.x, player_x);
        }
        assert_eq!(chapter.phase(), Phase::Confrontation);
        assert!(max_tension > 0.99);
        assert!(nearest_julia_x < chapter.world.julia_initial_x - 20.0);
        assert_eq!(chapter.npcs()[0].position.x, chapter.world.julia_initial_x);
        chapter.start_wave(false, true).unwrap();
        assert!(chapter.restraint_contact().is_none());
    }
}

#[test]
fn guards_exit_one_threshold_in_order_and_match_both_combat_handoffs() {
    for player_x in [1250.0, 2200.0] {
        let mut chapter = chapter();
        chapter
            .simulation
            .stage_player(player_x, Facing::Right)
            .unwrap();
        chapter.start_wave(false, true).unwrap();
        let ids = chapter.staged_enemies.clone();
        let mut first_visible = vec![None; ids.len()];
        let mut bypassed_player = false;
        let hp = chapter.simulation().player().hp;
        for tick in 0..chapter.spec.timing.guards_arrival_ticks {
            for (index, id) in ids.iter().enumerate() {
                let actor = chapter.simulation().actor(*id).unwrap();
                let pose = chapter.arrival_pose(*id).unwrap();
                assert!(!actor.active);
                assert!(pose.position.y >= chapter.world.bar_door[1]);
                assert!(pose.position.y <= chapter.world.ground_y + 25.0);
                assert_eq!(pose.depth, chapter.world.ground_y - pose.position.y);
                if index == 0 && (pose.position.x - player_x).abs() < 10.0 {
                    assert_eq!(pose.depth, -25.0);
                    bypassed_player = true;
                }
                if pose.visible && first_visible[index].is_none() {
                    first_visible[index] = Some(tick);
                    assert_eq!(
                        pose.position,
                        Vec2::new(chapter.world.bar_door[0], chapter.world.bar_door[1])
                    );
                }
                if tick >= chapter.spec.timing.guards_arrival_ticks * 9 / 10 {
                    assert_eq!(pose.position, actor.position);
                    assert_eq!(pose.facing, actor.facing);
                    assert_eq!(pose.depth, 0.0);
                }
            }
            chapter
                .tick(ChapterInput {
                    combat: Input {
                        movement: -1.0,
                        light: true,
                        ..Input::default()
                    },
                    ..ChapterInput::default()
                })
                .unwrap();
        }
        assert_eq!(chapter.phase(), Phase::GuardsFight);
        assert_eq!(chapter.simulation().player().position.x, player_x);
        assert_eq!(chapter.simulation().player().hp, hp);
        assert!(first_visible.iter().all(Option::is_some));
        assert!(first_visible.windows(2).all(|pair| pair[0] < pair[1]));
        if player_x < chapter.world.bar_door[0] {
            assert!(bypassed_player);
        }
        for id in ids {
            assert!(chapter.simulation().actor(id).unwrap().active);
            assert!(chapter.arrival_pose(id).is_none());
        }
    }
}

#[test]
fn broker_panics_and_runs_during_arrival_and_skip_and_retry_keep_him_gone() {
    for skip in [false, true] {
        let mut chapter = chapter();
        chapter
            .simulation
            .stage_player(chapter.world.erratic_center_x, Facing::Right)
            .unwrap();
        chapter.start_wave(true, true).unwrap();
        assert_eq!(chapter.broker_escape_age(), None);
        let panic_at = chapter.spec.timing.erratics_arrival_ticks / 8;
        for _ in 0..panic_at {
            chapter.tick(ChapterInput::default()).unwrap();
        }
        assert_eq!(chapter.broker_escape_age(), Some(0));
        assert_eq!(chapter.npcs().last().unwrap().clip, "guard");
        for _ in 0..60 {
            chapter.tick(ChapterInput::default()).unwrap();
        }
        let broker = chapter
            .npcs()
            .into_iter()
            .find(|npc| npc.character == "broker")
            .unwrap();
        assert_eq!(chapter.phase(), Phase::ErraticsArrival);
        assert_eq!(broker.clip, "run");
        assert_eq!(broker.facing, Facing::Left);
        assert!(broker.position.x < chapter.world.broker_x - 100.0);
        if skip {
            chapter
                .tick(ChapterInput {
                    skip: true,
                    ..ChapterInput::default()
                })
                .unwrap();
        } else {
            while chapter.phase() == Phase::ErraticsArrival {
                chapter.tick(ChapterInput::default()).unwrap();
            }
        }
        assert_eq!(chapter.phase(), Phase::ErraticsFight);
        assert_eq!(
            chapter.threat_age(),
            Some(u64::from(chapter.spec.timing.erratics_arrival_ticks))
        );
        assert!(chapter.npcs().iter().all(|npc| npc.character != "broker"));
        assert!(
            chapter
                .simulation()
                .actors()
                .iter()
                .all(|actor| actor.hp == actor.max_hp)
        );
        chapter.retry().unwrap();
        assert!(chapter.threat_age().is_some_and(|age| age >= 600));
        assert!(chapter.npcs().iter().all(|npc| npc.character != "broker"));
        assert_eq!(chapter.phase(), Phase::ErraticsFight);
    }
}

#[test]
fn skipping_each_cinematic_reconstructs_its_safe_endpoint() {
    let mut chapter = chapter();
    let skip = ChapterInput {
        skip: true,
        ..ChapterInput::default()
    };
    chapter.tick(skip).unwrap();
    assert_eq!(chapter.phase(), Phase::Approach);
    chapter
        .simulation
        .stage_player(chapter.world.confrontation_x, Facing::Right)
        .unwrap();
    chapter.tick(ChapterInput::default()).unwrap();
    chapter.tick(skip).unwrap();
    assert_eq!(chapter.phase(), Phase::Confrontation);
    assert_eq!(chapter.npcs()[0].position.x, chapter.world.julia_initial_x);
    chapter.start_wave(false, true).unwrap();
    chapter.tick(skip).unwrap();
    assert_eq!(chapter.phase(), Phase::GuardsFight);
    assert!(
        chapter
            .simulation()
            .actors()
            .iter()
            .all(|actor| actor.active)
    );
    assert!(chapter.restraint_contact().is_none());
    for stage in [
        "arrival",
        "guards_fight",
        "erratics_fight",
        "rescue",
        "complete",
    ] {
        let checkpoint: Checkpoint =
            serde_json::from_value(serde_json::json!({"version": 1, "stage": stage})).unwrap();
        checkpoint.validate().unwrap();
        chapter.checkpoint = checkpoint;
        chapter.retry().unwrap();
        assert_eq!(chapter.checkpoint(), checkpoint);
        if matches!(
            checkpoint.stage,
            CheckpointStage::ErraticsFight | CheckpointStage::Rescue | CheckpointStage::Complete
        ) {
            assert!(chapter.npcs().iter().all(|npc| npc.character != "broker"));
        }
    }
}

#[test]
fn reload_rejects_missing_or_empty_cinematic_copy() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("assets/adventure/chapters/cpp-augusta/texts.json");
    let source: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(path).unwrap()).unwrap();
    for key in [
        "julia.attempt",
        "cinema.skip",
        "cinema.place",
        "cinema.julia",
        "cinema.panic",
    ] {
        let mut missing = source.clone();
        missing["strings"].as_object_mut().unwrap().remove(key);
        assert!(
            Texts::from_json(&missing.to_string()).is_err(),
            "missing {key}"
        );
        let mut empty = source.clone();
        empty["strings"][key] = serde_json::Value::String("  ".into());
        assert!(Texts::from_json(&empty.to_string()).is_err(), "empty {key}");
    }
}
