//! Exercises playable routes, acting, interaction order and safe resumption.
//!
//! System: Adventure chapter verification. These tests drive the public input
//! contract and geometry without rendering, filesystem state or simulated wins.

use super::phone::*;
use super::*;
use crate::adventure::combat::{Action, Outcome};

fn fresh() -> Chapter {
    Chapter::new(World::bundled(), false)
}
fn resumed(stage: CheckpointStage) -> Chapter {
    Chapter::from_checkpoint(
        World::bundled(),
        Checkpoint {
            version: 1,
            stage,
            prologue_played_victory: false,
        },
    )
    .unwrap()
}
fn until(chapter: &mut Chapter, phase: Phase, limit: usize) {
    for _ in 0..limit {
        if chapter.phase == phase {
            return;
        }
        chapter.tick(ChapterInput::default());
    }
    assert_eq!(chapter.phase, phase);
}
fn walk_to(chapter: &mut Chapter, x: f32) {
    for _ in 0..1500 {
        let distance = x - chapter.player().position.x;
        if distance.abs() < 6.0 {
            return;
        }
        chapter.tick(ChapterInput {
            movement: distance.signum(),
            ..Default::default()
        });
    }
    panic!(
        "Could not reach {x}; phase={:?}, at={:?}",
        chapter.phase,
        chapter.player().position
    );
}
fn talk(chapter: &mut Chapter, x: f32, dialogue: Phase, next: Phase) {
    walk_to(chapter, x);
    let origin = chapter.player().position;
    chapter.tick(ChapterInput {
        interact: true,
        ..Default::default()
    });
    let mut before = origin;
    for _ in 0..400 {
        if chapter.phase == dialogue {
            break;
        }
        chapter.tick(ChapterInput {
            movement: 1.0,
            jump: true,
            heavy: true,
            interact: true,
            ..Default::default()
        });
        let now = chapter.player().position;
        assert!((now.x - before.x).hypot(now.y - before.y) <= 3.751);
        before = now;
    }
    assert_eq!(chapter.phase, dialogue);
    assert!(chapter.player().position.y < 580.0);
    assert!(chapter.player_scale() < 1.0);
    for _ in 0..3 {
        chapter.tick(ChapterInput {
            advance: true,
            ..Default::default()
        });
    }
    for _ in 0..400 {
        if chapter.phase == next {
            break;
        }
        chapter.tick(ChapterInput::default());
        let now = chapter.player().position;
        assert!((now.x - before.x).hypot(now.y - before.y) <= 3.751);
        before = now;
    }
    assert_eq!(chapter.phase, next);
    assert_eq!(chapter.player().position, origin);
    assert_eq!(chapter.player().velocity, Vec2::ZERO);
}

#[test]
fn geometry_rejects_unreachable_regions_and_unclear_jump_obstacles() {
    let source = include_str!("../../../assets/adventure/chapter/world.json");
    let mut value: serde_json::Value = serde_json::from_str(source).unwrap();
    assert!(World::from_json(source).is_ok());
    value["scenes"][0]["points"][0]["region"]["y"] = serde_json::json!(100);
    assert!(World::from_json(&value.to_string()).is_err());
    let mut value: serde_json::Value = serde_json::from_str(source).unwrap();
    value["scenes"][1]["obstacles"][0]["width"] = serde_json::json!(800);
    assert!(World::from_json(&value.to_string()).is_err());
    let mut value: serde_json::Value = serde_json::from_str(source).unwrap();
    value["scenes"][1]["id"] = serde_json::json!("street");
    assert!(World::from_json(&value.to_string()).is_err());
}

#[test]
fn introduction_blocks_input_and_camera_hands_off_inside_the_world() {
    let mut chapter = fresh();
    let position = chapter.player().position;
    let mut previous = chapter.camera();
    for _ in 0..INTRO_TICKS {
        assert_eq!(chapter.phase, Phase::Intro);
        chapter.tick(ChapterInput {
            movement: 1.0,
            jump: true,
            light: true,
            interact: true,
            ..Default::default()
        });
        assert_eq!(chapter.player().position, position);
        assert_eq!(chapter.combat.ticks, 0);
        assert!(!chapter.combat.enemy_awake);
        let camera = chapter.camera();
        assert!(camera.target.x - 640.0 / camera.zoom >= -0.001);
        assert!(
            camera.target.x + 640.0 / camera.zoom
                <= chapter.world.scene(chapter.scene).width + 0.001
        );
        assert!((camera.target.x - previous.target.x).abs() < 3.0);
        assert!((camera.zoom - previous.zoom).abs() < 0.01);
        previous = camera;
    }
    assert_eq!(chapter.phase, Phase::ExploreDriver);
    chapter.tick(ChapterInput {
        movement: 1.0,
        ..Default::default()
    });
    assert!(chapter.player().position.x > position.x);
}

#[test]
fn conversations_require_their_regions_and_return_continuously_to_control() {
    let mut chapter = resumed(CheckpointStage::StreetStart);
    walk_to(&mut chapter, 600.0);
    chapter.tick(ChapterInput {
        interact: true,
        ..Default::default()
    });
    assert_eq!(
        chapter.phase,
        Phase::ExploreDriver,
        "shop cannot precede the driver"
    );
    talk(
        &mut chapter,
        1250.0,
        Phase::DriverDialogue,
        Phase::ExploreShop,
    );
    assert_eq!(chapter.checkpoint().stage, CheckpointStage::DriverChecked);
    talk(
        &mut chapter,
        580.0,
        Phase::ShopDialogue,
        Phase::ExploreNeighbour,
    );
    assert_eq!(chapter.shutter_progress(), 1.0);
    talk(&mut chapter, 760.0, Phase::NeighbourDialogue, Phase::Phone);
    assert_eq!(chapter.checkpoint().stage, CheckpointStage::ContactReady);
}

#[test]
fn phone_preserves_the_exact_three_messages_and_blocks_queued_actions() {
    let copy = ChapterTexts::bundled();
    assert_eq!(
        PHONE_MESSAGE_KEYS.map(|key| copy.get(key)),
        [
            "voltou a acontecer.",
            "sim, eu senti algo...",
            "Vou resolver isso."
        ]
    );
    let mut chapter = resumed(CheckpointStage::ContactReady);
    let position = chapter.player().position;
    for age in 0..PHONE_DONE_TICK {
        let phone = chapter.phone().unwrap();
        assert_eq!(phone.ticks, age);
        assert_eq!(
            phone.message_count,
            usize::from(age >= PHONE_FIRST_SENT_TICK)
                + usize::from(age >= PHONE_REPLY_TICK)
                + usize::from(age >= PHONE_LAST_SENT_TICK)
        );
        let paused_snapshot = (
            chapter.ticks,
            chapter.phase_ticks,
            chapter.player().position,
            chapter.phone(),
            chapter.camera(),
            chapter.residents(),
        );
        assert_eq!(
            paused_snapshot,
            (
                chapter.ticks,
                chapter.phase_ticks,
                chapter.player().position,
                chapter.phone(),
                chapter.camera(),
                chapter.residents()
            )
        );
        chapter.tick(ChapterInput {
            movement: 1.0,
            jump: true,
            heavy: true,
            light: true,
            block: true,
            interact: true,
            ..Default::default()
        });
        assert_eq!(chapter.player().position, position);
    }
    assert_eq!(chapter.phase, Phase::LeaveStreet);
    assert_eq!(chapter.phone(), None);
    assert_eq!(chapter.checkpoint().stage, CheckpointStage::PhoneDone);
    chapter.tick(ChapterInput::default());
    assert_eq!(chapter.player().action, Action::Idle);
    assert_eq!(chapter.player().position, position);
}

#[test]
fn skip_and_saved_boundaries_do_not_fabricate_victories_or_half_pocketed_phones() {
    for age in [0, 85, 200, 400, 625, 835] {
        let mut chapter = resumed(CheckpointStage::ContactReady);
        for _ in 0..age {
            chapter.tick(ChapterInput::default());
        }
        let safe = chapter.checkpoint();
        let json = serde_json::to_string(&safe).unwrap();
        let restored =
            Chapter::from_checkpoint(World::bundled(), serde_json::from_str(&json).unwrap())
                .unwrap();
        assert_eq!(restored.phone().unwrap().ticks, 0);
        chapter.tick(ChapterInput {
            skip: true,
            light: true,
            movement: 1.0,
            ..Default::default()
        });
        assert_eq!(chapter.phone(), None);
        assert_eq!(chapter.player().action, Action::Idle);
        assert_eq!(chapter.combat.outcome, Outcome::Ongoing);
        assert!(!chapter.checkpoint().prologue_played_victory);
    }
    let mut invalid = Checkpoint::new(false);
    invalid.version = 2;
    assert!(Chapter::from_checkpoint(World::bundled(), invalid).is_err());
}

#[test]
fn lane_obstruction_stops_the_body_and_a_real_jump_reaches_the_resident() {
    let mut chapter = resumed(CheckpointStage::LaneStart);
    for _ in 0..220 {
        chapter.tick(ChapterInput {
            movement: 1.0,
            ..Default::default()
        });
    }
    assert_eq!(chapter.player().position.x, 670.0);
    assert_eq!(chapter.player().position.y, 580.0);
    chapter.tick(ChapterInput {
        movement: 1.0,
        jump: true,
        ..Default::default()
    });
    for _ in 0..65 {
        chapter.tick(ChapterInput {
            movement: 1.0,
            ..Default::default()
        });
        assert_eq!(
            chapter.player_scale(),
            1.0,
            "jumping is not a change of depth"
        );
        assert!(
            !chapter
                .player()
                .hurtbox()
                .intersects(chapter.world.scene(Scene::Lane).obstacles[0].rect())
        );
    }
    assert!(chapter.player().position.x > 824.0);
    assert_eq!(chapter.player().position.y, 580.0);
    walk_to(&mut chapter, 1160.0);
    chapter.tick(ChapterInput {
        interact: true,
        ..Default::default()
    });
    assert_eq!(chapter.phase, Phase::LaneApproach);
    for _ in 0..90 {
        chapter.tick(ChapterInput::default());
        if chapter.phase == Phase::LaneDialogue {
            break;
        }
    }
    assert_eq!(chapter.phase, Phase::LaneDialogue);
    assert!(
        chapter
            .world
            .scene(Scene::Lane)
            .poi("lane_resident")
            .unwrap()
            .position
            .x
            - chapter.player().position.x
            >= 90.0
    );
    for _ in 0..3 {
        chapter.tick(ChapterInput {
            advance: true,
            ..Default::default()
        });
    }
    assert_eq!(chapter.phase, Phase::LaneExit);
    for _ in 0..200 {
        if chapter.scene == Scene::Passage {
            break;
        }
        chapter.tick(ChapterInput {
            movement: 1.0,
            ..Default::default()
        });
    }
    assert_eq!(chapter.scene, Scene::Passage);
}

#[test]
fn passage_loss_retry_and_clear_depend_on_played_contact() {
    let mut chapter = resumed(CheckpointStage::PassageStart);
    for _ in 0..150 {
        if chapter.phase == Phase::PassageCombat {
            break;
        }
        chapter.tick(ChapterInput {
            movement: 1.0,
            jump: true,
            ..Default::default()
        });
    }
    assert_eq!(
        chapter.phase,
        Phase::PassageCombat,
        "jump cannot bypass the threat region"
    );
    let hp = chapter.player().hp;
    chapter.tick(ChapterInput {
        retry: true,
        ..Default::default()
    });
    assert_eq!(chapter.player().hp, hp);
    for _ in 0..2400 {
        if chapter.combat.outcome == Outcome::Defeat {
            break;
        }
        chapter.tick(ChapterInput::default());
    }
    assert_eq!(chapter.combat.outcome, Outcome::Defeat);
    chapter.tick(ChapterInput {
        retry: true,
        ..Default::default()
    });
    assert_eq!(chapter.phase, Phase::PassageCombat);
    assert_eq!(chapter.player().hp, 100);
    assert_eq!(chapter.combat.enemy.hp, 96);
    assert_eq!(chapter.phone(), None);
    chapter.tick(ChapterInput {
        skip: true,
        ..Default::default()
    });
    assert_eq!(chapter.phase, Phase::PassageCombat);
    assert_eq!(chapter.combat.outcome, Outcome::Ongoing);
    for _ in 0..6000 {
        if chapter.phase == Phase::PassageClear {
            break;
        }
        let distance = chapter.combat.enemy.position.x - chapter.player().position.x;
        let danger = matches!(
            chapter.combat.enemy.action,
            Action::Telegraph | Action::Lunge
        );
        chapter.tick(ChapterInput {
            movement: if distance.abs() > 100.0 {
                distance.signum()
            } else {
                0.0
            },
            block: danger,
            heavy: !danger && distance.abs() <= 110.0,
            ..Default::default()
        });
    }
    assert_eq!(chapter.phase, Phase::PassageClear);
    assert_eq!(chapter.combat.enemy.hp, 0);
    assert_eq!(chapter.checkpoint().stage, CheckpointStage::PassageCleared);
    assert!(!chapter.checkpoint().prologue_played_victory);
    for _ in 0..1000 {
        if chapter.phase == Phase::Departure {
            break;
        }
        chapter.tick(ChapterInput {
            movement: 1.0,
            ..Default::default()
        });
    }
    assert_eq!(chapter.phase, Phase::Departure);
    until(&mut chapter, Phase::Complete, 181);
    assert_eq!(chapter.checkpoint().stage, CheckpointStage::Complete);
}
