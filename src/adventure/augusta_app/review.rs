//! Reviews the chapter by issuing ordinary controls and records auditable state.
//!
//! System: Augusta review boundary. No health, outcomes or checkpoints are forced;
//! the same input policy can drive deterministic domain tests and native capture.

use crate::adventure::{
    augusta::{Chapter, ChapterInput, Phase},
    production::{Input, Team},
};

pub(crate) fn input(chapter: &Chapter) -> ChapterInput {
    if chapter.defeated() {
        return ChapterInput {
            retry: true,
            ..ChapterInput::default()
        };
    }
    if let Some((key, index)) = chapter.dialogue() {
        let characters = chapter
            .texts
            .line(key, index)
            .map_or(0, |line| line.text.chars().count());
        let hold = (48 + characters as u32 * 3).clamp(120, 480);
        return ChapterInput {
            advance: chapter.dialogue_ticks() >= hold,
            ..ChapterInput::default()
        };
    }
    let player = chapter.simulation().player();
    let mut combat = Input::default();
    let mut interact = false;
    match chapter.phase() {
        Phase::Approach | Phase::Exit => combat.movement = 1.0,
        Phase::FindJulia => {
            let delta = chapter.world.julia_x - player.position.x;
            combat.movement = if delta.abs() < 75.0 {
                0.0
            } else {
                delta.signum()
            };
            interact = delta.abs() < 100.0;
        }
        Phase::GuardsFight | Phase::ErraticsFight => {
            if let Some(enemy) = chapter
                .simulation()
                .actors()
                .iter()
                .filter(|a| a.team == Team::Enemy && a.hp > 0)
                .min_by(|a, b| {
                    (a.position.x - player.position.x)
                        .abs()
                        .total_cmp(&(b.position.x - player.position.x).abs())
                })
            {
                let delta = enemy.position.x - player.position.x;
                combat.movement = if delta.abs() > 128.0 || delta.signum() != player.facing.sign() {
                    delta.signum()
                } else {
                    0.0
                };
                let sim = chapter.simulation();
                let incoming = sim
                    .actors()
                    .iter()
                    .filter(|a| a.team == Team::Enemy && a.hp > 0)
                    .any(|a| {
                        (a.position.x - player.position.x) * player.facing.sign() > 0.0
                            && (a.position.x - player.position.x).abs() < 139.0
                            && a.move_id
                                .as_ref()
                                .and_then(|id| sim.content.moves.get(id))
                                .is_some_and(|m| {
                                    a.action_ticks + 5 >= m.startup
                                        && a.action_ticks < m.startup + m.active
                                })
                    });
                let surrounded = [-1.0, 1.0].iter().all(|side| {
                    sim.actors().iter().any(|a| {
                        a.team == Team::Enemy
                            && a.hp > 0
                            && (a.position.x - player.position.x) * side > 0.0
                            && (a.position.x - player.position.x).abs() < 138.0
                    })
                });
                if let Some(id) = &player.move_id {
                    let age = player.action_ticks;
                    combat.light =
                        id == "cpp.light-1" && age == 8 || id == "cpp.light-2" && age == 9;
                } else if incoming {
                    combat.guard = true;
                } else if surrounded {
                    combat.spin = true;
                } else if delta.abs() < 90.0 {
                    combat.light = true;
                } else if delta.abs() < 144.0 {
                    combat.kick = true;
                } else if chapter.phase_ticks().is_multiple_of(181) {
                    combat.linker = true;
                }
            }
        }
        _ => {}
    }
    ChapterInput {
        combat,
        interact,
        ..ChapterInput::default()
    }
}

pub(super) fn telemetry(
    chapter: &Chapter,
    frame: u32,
    seconds: f64,
    paused: bool,
    audio_cues: &[&str],
    audio_reset: bool,
) -> serde_json::Value {
    let sim = chapter.simulation();
    serde_json::json!({"frame":frame,"seconds":seconds,"audio_cues":audio_cues,"audio_reset":audio_reset,"ticks":chapter.ticks(),"phase":format!("{:?}", chapter.phase()),
        "phase_ticks":chapter.phase_ticks(),"paused":paused,"checkpoint":chapter.checkpoint(),"camera_x":chapter.camera_x(),
        "actors":sim.actors().iter().map(|a| serde_json::json!({"id":a.id.0,"character":a.character,"x":a.position.x,"y":a.position.y,"hp":a.hp,"active":a.active,"clip":a.clip_id(),"action_ticks":a.action_ticks})).collect::<Vec<_>>(),
        "outcome":format!("{:?}",sim.outcome()),"dialogue":chapter.dialogue()})
}
