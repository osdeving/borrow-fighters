//! Records chapter evidence and drives only public gameplay commands in review mode.
//!
//! System: Adventure review boundary. Telemetry follows actual sampled state;
//! the optional policy never changes positions, health, checkpoints or triggers.

use crate::adventure::{
    chapter::{Chapter, ChapterInput, Phase},
    combat::{Action, Outcome},
};
use std::{error::Error, fs::File, io::Write};

pub(super) fn input(chapter: &Chapter) -> ChapterInput {
    let mut input = ChapterInput::default();
    let geometry = chapter.world.scene(chapter.scene);
    let target = match chapter.phase {
        Phase::ExploreDriver => Some("driver"),
        Phase::ExploreShop => Some("shop"),
        Phase::ExploreNeighbour => Some("neighbour"),
        Phase::LaneExplore => Some("lane_resident"),
        _ => None,
    };
    if let Some(id) = target {
        let region = geometry.poi(id).expect("validated POI").region;
        let target = region.x + region.width * 0.5;
        input.movement = (target - chapter.player().position.x).signum();
        if chapter.nearby_interaction().is_some() {
            input.interact = true;
            input.movement = 0.0;
        }
    } else if matches!(
        chapter.phase,
        Phase::LeaveStreet | Phase::LaneExit | Phase::PassageExplore | Phase::PassageClear
    ) {
        input.movement = 1.0;
    } else if chapter.active_dialogue().is_some() && chapter.phase != Phase::Intro {
        input.advance = chapter.phase_ticks >= 150;
    }
    if chapter.phase == Phase::LaneExplore
        && let Some(piece) = chapter
            .debris
            .iter()
            .filter(|p| p.hp > 0)
            .min_by(|a, b| a.spec.region.x.total_cmp(&b.spec.region.x))
    {
        let distance = piece.rect().x - chapter.player().position.x;
        input.movement = if distance > 58.0 { 1.0 } else { 0.0 };
        if distance <= 76.0 {
            input.kick = chapter.ticks.is_multiple_of(3);
            input.heavy = chapter.ticks % 3 == 1;
            input.light = chapter.ticks % 3 == 2;
        }
    }
    if chapter.phase == Phase::PassageCombat {
        if chapter.combat.outcome == Outcome::Defeat {
            input.retry = true;
            return input;
        }
        let actor = chapter.player();
        let Some(enemy) = chapter
            .combat
            .enemies()
            .filter(|enemy| enemy.hp > 0)
            .min_by(|a, b| {
                (a.position.x - actor.position.x)
                    .abs()
                    .total_cmp(&(b.position.x - actor.position.x).abs())
            })
        else {
            return input;
        };
        let distance = enemy.position.x - actor.position.x;
        let guarding = matches!(enemy.action, Action::Telegraph | Action::Lunge);
        input.block = guarding;
        if guarding {
            input.movement = 0.0;
        } else if distance.abs() > 92.0 {
            input.movement = distance.signum();
        } else if matches!(actor.action, Action::Idle | Action::Walk | Action::Block) {
            input.movement = distance.signum();
            input.kick = chapter.ticks.is_multiple_of(3);
            input.heavy = !input.kick;
        }
    }
    input
}

pub(super) fn trace(
    file: &mut File,
    chapter: &Chapter,
    frame: u32,
    seconds: f32,
    wall_seconds: f64,
    menu: Option<&str>,
    paused: bool,
) -> Result<(), Box<dyn Error>> {
    let camera = chapter.camera();
    let player = chapter.player();
    let phone = chapter.phone();
    let dialogue = chapter.active_dialogue();
    serde_json::to_writer(
        &mut *file,
        &serde_json::json!({
            "frame":frame,"capture_seconds":seconds,"wall_seconds":wall_seconds,"stage":"Chapter","scene":format!("{:?}",chapter.scene),"phase":format!("{:?}",chapter.phase),"phase_ticks":chapter.phase_ticks,"ticks":chapter.ticks,"paused":paused,"menu":menu,
            "player":{"x":player.position.x,"y":player.position.y,"hp":player.hp,"action":format!("{:?}",player.action),"facing":format!("{:?}",player.facing),"grounded":player.grounded,"scale":chapter.player_scale()},
            "debris":chapter.debris.iter().map(|piece| serde_json::json!({"id":piece.spec.id,"hp":piece.hp,"y":piece.y,"hit_tick":piece.hit_tick})).collect::<Vec<_>>(),
            "enemies":chapter.combat.enemies().map(|enemy| serde_json::json!({"x":enemy.position.x,"hp":enemy.hp,"action":format!("{:?}",enemy.action)})).collect::<Vec<_>>(),
            "enemy":{"x":chapter.combat.enemy.position.x,"hp":chapter.combat.enemy.hp,"action":format!("{:?}",chapter.combat.enemy.action)},"outcome":format!("{:?}",chapter.combat.outcome),
            "camera":{"x":camera.target.x,"y":camera.target.y,"zoom":camera.zoom},"checkpoint":chapter.checkpoint(),"controls_active":chapter.controls_active(),"shutter":chapter.shutter_progress(),
            "dialogue":dialogue.map(|d| d.key),"line":dialogue.map(|d| d.line_index),"interaction":chapter.nearby_interaction().map(|i|i.id),
            "phone":phone.map(|p|serde_json::json!({"phase":format!("{:?}",p.phase),"ticks":p.ticks,"messages":p.message_count,"panel":p.panel_visible,"device":p.device_visible,"typing":p.python_typing}))
        }),
    )?;
    file.write_all(b"\n")?;
    file.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adventure::chapter::{Checkpoint, CheckpointStage, World};

    #[test]
    fn review_policy_breaks_every_piece_then_defeats_both_opponents() {
        let mut checkpoint = Checkpoint::new(false);
        checkpoint.stage = CheckpointStage::LaneStart;
        let mut chapter = Chapter::from_checkpoint(World::bundled(), checkpoint).unwrap();
        let mut broke_cargo = false;
        let mut saw_two = false;
        for _ in 0..12000 {
            let command = input(&chapter);
            chapter.tick(command);
            broke_cargo |=
                !chapter.debris.is_empty() && chapter.debris.iter().all(|piece| piece.hp == 0);
            saw_two |=
                chapter.phase == Phase::PassageCombat && chapter.combat.enemies().count() == 2;
            if chapter.phase == Phase::Complete {
                break;
            }
        }
        assert!(broke_cargo);
        assert!(saw_two);
        assert_eq!(chapter.phase, Phase::Complete);
        assert!(chapter.combat.enemies().all(|enemy| enemy.hp == 0));
    }
}
