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
    assert!(phases.contains(&Phase::ErraticsArrival));
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
        if chapter.phase() == Phase::Confrontation {
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
    assert_eq!(chapter.phase(), Phase::Confrontation);
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
