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
        && chapter.player().grounded
        && (620.0..810.0).contains(&chapter.player().position.x)
    {
        input.jump = true;
    }
    if chapter.phase == Phase::PassageCombat {
        if chapter.combat.outcome == Outcome::Defeat {
            input.retry = true;
            return input;
        }
        let actor = chapter.player();
        let enemy = &chapter.combat.enemy;
        let distance = enemy.position.x - actor.position.x;
        let guarding = matches!(enemy.action, Action::Telegraph | Action::Lunge);
        input.block = guarding;
        if guarding {
            input.movement = 0.0;
        } else if distance.abs() > 92.0 {
            input.movement = distance.signum();
        } else if matches!(actor.action, Action::Idle | Action::Walk | Action::Block) {
            input.movement = distance.signum();
            input.heavy = chapter.ticks.is_multiple_of(3);
            input.light = !input.heavy;
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
