//! Directs C++'s independent Augusta encounters and reconstructible checkpoints.
//!
//! System: Augusta story. Dialogue, civilian staging and descent use one fixed
//! clock; all damage, movement and victory decisions belong to production combat.

use super::{ChapterSpec, Checkpoint, CheckpointStage, Texts, World};
use crate::{
    adventure::production::{
        ActorId, Bounds, CombatCatalog, EnemySpawn, Event, Facing, Input, Outcome, Simulation,
    },
    math::vec2::Vec2,
};
use std::sync::Arc;

#[cfg(test)]
#[path = "tests.rs"]
mod tests;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Phase {
    Introduction,
    Approach,
    JuliaAttempt,
    Confrontation,
    GuardsArrival,
    GuardsFight,
    AfterGuards,
    Regroup,
    ErraticsArrival,
    ErraticsFight,
    BrokerEscape,
    FindJulia,
    RescueDialogue,
    Exit,
    Complete,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ChapterInput {
    pub combat: Input,
    pub interact: bool,
    pub advance: bool,
    pub skip: bool,
    pub retry: bool,
}

/// Civilian actors have presentation poses and cannot enter the damage simulation.
#[derive(Clone, Copy, Debug)]
pub struct NpcPose {
    pub character: &'static str,
    pub position: Vec2,
    pub facing: Facing,
    pub clip: &'static str,
    pub ticks: u32,
    /// Positive depth places an actor behind the gameplay lane; y includes it.
    pub depth: f32,
    pub visible: bool,
}

/// Authored, short arm chain shared by both views of the restrained pair.
#[derive(Clone, Copy, Debug)]
pub struct RestraintContact {
    pub broker_shoulder: Vec2,
    pub broker_elbow: Vec2,
    pub julia_wrist: Vec2,
    pub julia_shoulder: Vec2,
    pub depth: f32,
    /// Zero at rest, one when Julia pulls away against the grip.
    pub tension: f32,
}

/// A staging override for an existing, inactive combat actor with a stable ID.
#[derive(Clone, Copy, Debug)]
pub struct ArrivalPose {
    pub position: Vec2,
    pub facing: Facing,
    pub depth: f32,
    pub visible: bool,
    pub scale: f32,
    pub clip: &'static str,
    pub ticks: u32,
    pub impact_age: Option<u32>,
}

pub struct Chapter {
    pub spec: ChapterSpec,
    pub world: World,
    pub texts: Texts,
    simulation: Simulation,
    phase: Phase,
    phase_ticks: u32,
    ticks: u64,
    line: usize,
    line_ticks: u32,
    checkpoint: Checkpoint,
    staged_enemies: Vec<ActorId>,
    broker_flee_ticks: Option<u32>,
    threat_ticks: Option<u64>,
    julia_escape_ticks: u32,
    landing_tick: Option<u64>,
}

impl Chapter {
    pub fn new(
        spec: ChapterSpec,
        world: World,
        texts: Texts,
        catalog: Arc<CombatCatalog>,
    ) -> Result<Self, String> {
        if [&spec.title_key, &spec.summary_key].iter().any(|key| {
            texts
                .strings
                .get(*key)
                .is_none_or(|text| text.trim().is_empty())
        }) {
            return Err("Augusta chapter entry references missing text".into());
        }
        for character in ["cpp", "security", "erratic"] {
            if !catalog.characters.contains_key(character) {
                return Err(format!("Augusta requires {character}"));
            }
        }
        let simulation = Simulation::new(
            catalog,
            Self::bounds(&world, false),
            "cpp",
            world.player_spawn_x,
        )?;
        Ok(Self {
            broker_flee_ticks: None,
            threat_ticks: None,
            spec,
            world,
            texts,
            simulation,
            phase: Phase::Introduction,
            phase_ticks: 0,
            ticks: 0,
            line: 0,
            line_ticks: 0,
            checkpoint: Checkpoint::default(),
            staged_enemies: vec![],
            julia_escape_ticks: 0,
            landing_tick: None,
        })
    }

    pub fn from_checkpoint(
        spec: ChapterSpec,
        world: World,
        texts: Texts,
        catalog: Arc<CombatCatalog>,
        checkpoint: Checkpoint,
    ) -> Result<Self, String> {
        checkpoint.validate()?;
        let mut chapter = Self::new(spec, world, texts, catalog)?;
        chapter.checkpoint = checkpoint;
        chapter.retry()?;
        Ok(chapter)
    }

    fn bounds(world: &World, arena: bool) -> Bounds {
        let [left, right] = if arena {
            world.guards_arena
        } else {
            world.walk_bounds
        };
        Bounds {
            left,
            right,
            floor_y: world.ground_y,
        }
    }

    pub fn simulation(&self) -> &Simulation {
        &self.simulation
    }
    pub fn phase(&self) -> Phase {
        self.phase
    }
    pub fn phase_ticks(&self) -> u32 {
        self.phase_ticks
    }
    pub fn ticks(&self) -> u64 {
        self.ticks
    }
    pub fn dialogue_ticks(&self) -> u32 {
        self.line_ticks
    }
    /// Panic and running start during EP arrival and remain completed on retry.
    pub fn broker_escape_age(&self) -> Option<u32> {
        self.broker_flee_ticks
    }
    /// One evacuation clock spans the descent, combat and rescue camera cuts.
    pub fn threat_age(&self) -> Option<u64> {
        self.threat_ticks
    }
    /// The physical contact clock survives the camera handoff into live combat.
    pub fn landing_age(&self) -> Option<u32> {
        self.landing_tick
            .map(|tick| self.ticks.saturating_sub(tick).min(u32::MAX as u64) as u32)
    }
    pub fn checkpoint(&self) -> Checkpoint {
        self.checkpoint
    }
    pub fn defeated(&self) -> bool {
        self.simulation.outcome() == Outcome::Defeat
    }
    pub fn complete(&self) -> bool {
        self.phase == Phase::Complete
    }
    pub fn camera_x(&self) -> f32 {
        self.world.camera_x(self.simulation.player().position.x)
    }

    pub fn dialogue(&self) -> Option<(&'static str, usize)> {
        let key = match self.phase {
            Phase::Confrontation => "confrontation",
            Phase::AfterGuards => "after_guards",
            Phase::RescueDialogue => "rescue",
            _ => return None,
        };
        Some((key, self.line))
    }

    pub fn objective_key(&self) -> &'static str {
        match self.phase {
            Phase::Introduction | Phase::Approach | Phase::Confrontation => "objective.approach",
            Phase::JuliaAttempt => "julia.attempt",
            Phase::GuardsArrival => "arrival.guards",
            Phase::GuardsFight => "objective.guards",
            Phase::AfterGuards => "objective.after_guards",
            Phase::Regroup => "arrival.erratics",
            Phase::ErraticsFight => "objective.erratics",
            Phase::ErraticsArrival => "arrival.erratics",
            Phase::BrokerEscape => "broker.escape",
            Phase::FindJulia | Phase::RescueDialogue => "objective.rescue",
            Phase::Exit => "objective.exit",
            Phase::Complete => "chapter.complete",
        }
    }

    fn enter(&mut self, phase: Phase) {
        self.phase = phase;
        self.phase_ticks = 0;
        self.line = 0;
        self.line_ticks = 0;
    }

    fn enemies(&self, erratics: bool) -> Vec<EnemySpawn> {
        let locations: &[f32] = if erratics {
            &self.world.erratic_landings_x
        } else {
            &self.world.guards_spawn_x
        };
        locations
            .iter()
            .map(|x| EnemySpawn {
                character: if erratics { "erratic" } else { "security" }.into(),
                x: *x,
                facing: if *x < self.simulation.player().position.x {
                    Facing::Right
                } else {
                    Facing::Left
                },
            })
            .collect()
    }

    fn start_wave(&mut self, erratics: bool, staged: bool) -> Result<(), String> {
        self.landing_tick = None;
        self.simulation
            .set_bounds(Self::bounds(&self.world, true))?;
        self.simulation.clear_encounter();
        self.staged_enemies.clear();
        if erratics {
            self.threat_ticks = Some(if staged { 0 } else { 600 });
            self.broker_flee_ticks = if staged {
                None
            } else {
                Some(self.spec.timing.broker_escape_ticks)
            };
        }
        for spawn in self.enemies(erratics) {
            self.staged_enemies
                .push(self.simulation.spawn_enemy(&spawn, !staged)?);
        }
        self.checkpoint.stage = if erratics {
            CheckpointStage::ErraticsFight
        } else {
            CheckpointStage::GuardsFight
        };
        self.enter(match (erratics, staged) {
            (true, true) => Phase::ErraticsArrival,
            (true, false) => Phase::ErraticsFight,
            (false, true) => Phase::GuardsArrival,
            (false, false) => Phase::GuardsFight,
        });
        Ok(())
    }

    /// Rebuilds a whole safe encounter without serializing actors, health or dialogue offsets.
    pub fn retry(&mut self) -> Result<(), String> {
        self.landing_tick = None;
        self.simulation
            .set_bounds(Self::bounds(&self.world, false))?;
        self.staged_enemies.clear();
        self.julia_escape_ticks = 600;
        self.broker_flee_ticks = match self.checkpoint.stage {
            CheckpointStage::Arrival | CheckpointStage::GuardsFight => None,
            _ => Some(self.spec.timing.broker_escape_ticks),
        };
        self.threat_ticks = match self.checkpoint.stage {
            CheckpointStage::Arrival | CheckpointStage::GuardsFight => None,
            _ => Some(600),
        };
        match self.checkpoint.stage {
            CheckpointStage::Arrival => {
                self.simulation.reset_player(self.world.player_spawn_x)?;
                self.julia_escape_ticks = 0;
                self.enter(Phase::Introduction);
            }
            CheckpointStage::GuardsFight => {
                self.simulation.reset_player(self.world.confrontation_x)?;
                self.start_wave(false, false)?;
            }
            CheckpointStage::ErraticsFight => {
                self.simulation.reset_player(self.world.erratic_center_x)?;
                self.start_wave(true, false)?;
            }
            CheckpointStage::Rescue => {
                self.simulation.reset_player(self.world.erratic_center_x)?;
                self.enter(Phase::FindJulia);
            }
            CheckpointStage::Complete => {
                self.simulation.reset_player(self.world.exit_x)?;
                self.enter(Phase::Complete);
            }
        }
        Ok(())
    }

    /// Advances only when the host is unpaused. Skip never bypasses a live encounter.
    pub fn tick(&mut self, input: ChapterInput) -> Result<Vec<Event>, String> {
        if input.retry && self.defeated() {
            self.retry()?;
            return Ok(vec![]);
        }
        if self.defeated() || self.complete() {
            return Ok(vec![]);
        }
        self.ticks += 1;
        self.phase_ticks += 1;
        if let Some(age) = &mut self.broker_flee_ticks {
            *age = age.saturating_add(1);
        }
        if let Some(age) = &mut self.threat_ticks {
            *age = age.saturating_add(1);
        }
        if self.checkpoint.stage != CheckpointStage::Arrival {
            self.julia_escape_ticks = self.julia_escape_ticks.saturating_add(1);
        }
        if let Some((key, _)) = self.dialogue() {
            self.line_ticks = self.line_ticks.saturating_add(1);
            // Idle breathing and physical clocks continue while dialogue owns input.
            let events = self.simulation.tick(Input::default());
            if input.advance || input.interact || input.skip {
                self.line += 1;
                self.line_ticks = 0;
                if self.texts.line(key, self.line).is_none() {
                    match self.phase {
                        Phase::Confrontation => {
                            self.start_wave(false, true)?;
                        }
                        Phase::AfterGuards => {
                            self.enter(Phase::Regroup);
                        }
                        Phase::RescueDialogue => {
                            self.enter(Phase::Exit);
                        }
                        _ => unreachable!(),
                    }
                }
            }
            return Ok(events);
        }
        let mut events = vec![];
        match self.phase {
            Phase::Introduction => {
                events = self.simulation.tick(Input::default());
                if input.skip || self.phase_ticks >= self.spec.timing.introduction_ticks {
                    self.enter(Phase::Approach);
                }
            }
            Phase::JuliaAttempt => {
                events = self.simulation.tick(Input::default());
                if input.skip || self.phase_ticks >= self.spec.timing.julia_attempt_ticks {
                    self.enter(Phase::Confrontation);
                }
            }
            Phase::Approach | Phase::FindJulia | Phase::Exit => {
                events = self.simulation.tick(input.combat);
                let x = self.simulation.player().position.x;
                match self.phase {
                    Phase::Approach
                        if x >= self.world.confrontation_x - 100.0
                            && self.simulation.player().grounded =>
                    {
                        // An overshooting jump must land before staging; preserve
                        // the established C++ -> broker -> Julia screen direction.
                        let x = x.min(self.world.broker_x - 150.0);
                        self.simulation.stage_player(x, Facing::Right)?;
                        self.enter(Phase::JuliaAttempt);
                    }
                    Phase::FindJulia
                        if input.interact
                            && (x - self.world.julia_x).abs() < 130.0
                            && self.simulation.player().grounded =>
                    {
                        self.simulation
                            .stage_player(x, facing_toward(x, self.world.julia_x))?;
                        self.enter(Phase::RescueDialogue);
                    }
                    Phase::Exit if x >= self.world.exit_x && self.simulation.player().grounded => {
                        self.simulation.stage_player(x, Facing::Right)?;
                        self.checkpoint.stage = CheckpointStage::Complete;
                        self.enter(Phase::Complete);
                    }
                    _ => {}
                }
            }
            Phase::GuardsArrival | Phase::ErraticsArrival => {
                events = self.simulation.tick(Input::default());
                if self.phase == Phase::ErraticsArrival
                    && self.broker_flee_ticks.is_none()
                    && self.phase_ticks >= self.spec.timing.erratics_arrival_ticks / 8
                {
                    self.broker_flee_ticks = Some(0);
                }
                if self.phase == Phase::ErraticsArrival
                    && self.landing_tick.is_none()
                    && self.phase_ticks >= self.spec.timing.erratics_arrival_ticks * 3 / 4
                {
                    self.landing_tick = Some(self.ticks);
                }
                let duration = if self.phase == Phase::GuardsArrival {
                    self.spec.timing.guards_arrival_ticks
                } else {
                    self.spec.timing.erratics_arrival_ticks
                };
                if input.skip || self.phase_ticks >= duration {
                    if self.phase == Phase::ErraticsArrival {
                        self.broker_flee_ticks = Some(self.spec.timing.broker_escape_ticks);
                        if input.skip {
                            self.threat_ticks = Some(u64::from(duration));
                        }
                    } else {
                        self.julia_escape_ticks = self.julia_escape_ticks.max(230);
                    }
                    for id in &self.staged_enemies {
                        self.simulation.set_actor_active(*id, true)?;
                    }
                    self.enter(if self.phase == Phase::GuardsArrival {
                        Phase::GuardsFight
                    } else {
                        Phase::ErraticsFight
                    });
                }
            }
            Phase::GuardsFight | Phase::ErraticsFight => {
                events = self.simulation.tick(input.combat);
                if self.simulation.outcome() == Outcome::Victory
                    && self.simulation.player().grounded
                {
                    self.simulation.clear_encounter();
                    if self.phase == Phase::GuardsFight {
                        let x = self.simulation.player().position.x;
                        self.simulation
                            .stage_player(x, facing_toward(x, self.world.broker_x))?;
                        self.enter(Phase::AfterGuards);
                    } else {
                        self.simulation
                            .set_bounds(Self::bounds(&self.world, false))?;
                        self.checkpoint.stage = CheckpointStage::Rescue;
                        self.enter(Phase::FindJulia);
                    }
                }
            }
            Phase::Regroup => {
                let delta = self.world.erratic_center_x - self.simulation.player().position.x;
                if delta.abs() < 5.0 {
                    self.simulation
                        .stage_player(self.world.erratic_center_x, Facing::Right)?;
                    self.start_wave(true, true)?;
                } else {
                    events = self.simulation.tick(Input {
                        movement: delta.signum() * 0.65,
                        ..Input::default()
                    });
                }
            }
            Phase::BrokerEscape => {
                events = self.simulation.tick(Input::default());
                if input.skip || self.phase_ticks >= self.spec.timing.broker_escape_ticks {
                    self.enter(Phase::FindJulia);
                }
            }
            _ => {}
        }
        Ok(events)
    }

    fn attempt_tension(&self) -> f32 {
        if self.phase != Phase::JuliaAttempt {
            return 0.0;
        }
        let p = self.phase_ticks as f32 / self.spec.timing.julia_attempt_ticks as f32;
        // She takes a step, meets the grip, then regains her footing. Both
        // endpoints equal the idle staging, so skip never leaves an offset.
        let reach = smooth((p - 0.12) / 0.27);
        let recover = smooth((p - 0.52) / 0.32);
        reach * (1.0 - recover)
    }

    pub fn npcs(&self) -> Vec<NpcPose> {
        let mut result = vec![];
        let escaped = self.checkpoint.stage != CheckpointStage::Arrival;
        let tension = self.attempt_tension();
        let (julia_x, clip) = if matches!(self.phase, Phase::Exit | Phase::Complete) {
            (
                (self.simulation.player().position.x - 78.0).max(self.world.julia_x),
                if self.simulation.player().position.x - 78.0 > self.world.julia_x
                    && self.simulation.player().velocity.x.abs() > 1.0
                {
                    "run"
                } else {
                    "idle"
                },
            )
        } else if escaped {
            let p = (self.julia_escape_ticks as f32 / 230.0).clamp(0.0, 1.0);
            (
                self.world.julia_initial_x + (self.world.julia_x - self.world.julia_initial_x) * p,
                if p < 1.0 { "run" } else { "idle" },
            )
        } else {
            (
                self.world.julia_initial_x - tension * 22.0,
                if tension > 0.04 { "run" } else { "idle" },
            )
        };
        let julia_depth = if escaped {
            16.0 * (1.0 - self.julia_escape_ticks as f32 / 90.0).clamp(0.0, 1.0)
        } else {
            16.0
        };
        result.push(NpcPose {
            character: "julia",
            position: Vec2::new(julia_x, self.world.ground_y - julia_depth),
            facing: if matches!(self.phase, Phase::Confrontation | Phase::RescueDialogue) {
                facing_toward(julia_x, self.simulation.player().position.x)
            } else if !escaped
                || self.phase == Phase::Exit
                    && clip == "run"
                    && self.simulation.player().velocity.x < 0.0
            {
                Facing::Left
            } else {
                Facing::Right
            },
            clip,
            ticks: self.ticks as u32,
            depth: julia_depth,
            visible: true,
        });
        let flee_duration = self.spec.timing.broker_escape_ticks;
        let gone = self
            .broker_flee_ticks
            .is_some_and(|age| age >= flee_duration)
            || matches!(
                self.phase,
                Phase::FindJulia | Phase::RescueDialogue | Phase::Exit | Phase::Complete
            );
        if !gone {
            let age = self.broker_flee_ticks.unwrap_or(0);
            let running = self.broker_flee_ticks.is_some() && age >= 24;
            let p = age.saturating_sub(24) as f32 / (flee_duration - 24) as f32;
            let x = if running {
                self.world.broker_x
                    + (self.world.walk_bounds[0] - 240.0 - self.world.broker_x) * smooth(p)
            } else {
                self.world.broker_x
            };
            result.push(NpcPose {
                character: "broker",
                position: Vec2::new(x, self.world.ground_y),
                facing: if self.broker_flee_ticks.is_some() {
                    Facing::Left
                } else {
                    facing_toward(x, self.simulation.player().position.x)
                },
                clip: if running {
                    "run"
                } else if self.broker_flee_ticks.is_some() {
                    "guard"
                } else {
                    "idle"
                },
                ticks: if self.broker_flee_ticks.is_some() {
                    age
                } else {
                    self.ticks as u32
                },
                depth: 0.0,
                visible: true,
            });
        }
        result
    }

    /// Holds Julia's forearm behind the broker until C++ opens the escape route.
    /// World sockets are stable across camera angles and stay within arm reach.
    pub fn restraint_contact(&self) -> Option<RestraintContact> {
        if self.checkpoint.stage != CheckpointStage::Arrival {
            return None;
        }
        let tension = self.attempt_tension();
        let julia_x = self.world.julia_initial_x - tension * 22.0;
        let shoulder = Vec2::new(self.world.broker_x + 13.0, self.world.ground_y - 151.0);
        let wrist = Vec2::new(julia_x - 24.0, self.world.ground_y - 132.0);
        Some(RestraintContact {
            broker_shoulder: shoulder,
            broker_elbow: Vec2::new((shoulder.x + wrist.x) * 0.5 + 4.0, wrist.y + 10.0),
            julia_wrist: wrist,
            julia_shoulder: Vec2::new(julia_x - 10.0, self.world.ground_y - 145.0),
            depth: 8.0,
            tension,
        })
    }

    pub fn arrival_pose(&self, id: ActorId) -> Option<ArrivalPose> {
        if !self.staged_enemies.contains(&id) {
            return None;
        }
        let actor = self.simulation.actor(id)?;
        if self.phase == Phase::GuardsArrival {
            let index = self
                .staged_enemies
                .iter()
                .position(|staged| *staged == id)? as u32;
            let duration = self.spec.timing.guards_arrival_ticks;
            let start = duration / 10 + index * duration * 3 / 20;
            let finish = duration * 9 / 10;
            let age = self.phase_ticks.saturating_sub(start);
            let travel_ticks = finish - start;
            let emerge_ticks = travel_ticks.min(duration / 9);
            let toward_exit = facing_toward(self.world.bar_door[0], actor.position.x);
            let emerge = smooth(age as f32 / emerge_ticks as f32);
            let walk = (age.saturating_sub(emerge_ticks) as f32
                / (travel_ticks - emerge_ticks) as f32)
                .clamp(0.0, 1.0);
            let threshold_x = self.world.bar_door[0] + toward_exit.sign() * 32.0;
            let x = if age < emerge_ticks {
                self.world.bar_door[0] + toward_exit.sign() * 32.0 * emerge
            } else {
                threshold_x + (actor.position.x - threshold_x) * walk
            };
            // The first guard passes in front of C++ before taking the left
            // flank. A short depth detour avoids walking through her body;
            // both the door and final combat position remain exact endpoints.
            let bypass = if index == 0 && walk > 0.0 && walk < 1.0 {
                25.0 * smooth(walk / 0.18) * (1.0 - smooth((walk - 0.72) / 0.28))
            } else {
                0.0
            };
            let y = self.world.bar_door[1]
                + (self.world.ground_y - self.world.bar_door[1]) * emerge
                + bypass;
            return Some(ArrivalPose {
                position: Vec2::new(x, y),
                facing: if walk >= 1.0 {
                    actor.facing
                } else {
                    toward_exit
                },
                depth: self.world.ground_y - y,
                visible: self.phase_ticks >= start,
                scale: 1.0,
                clip: if walk >= 1.0 { "idle" } else { "run" },
                ticks: age,
                impact_age: None,
            });
        }
        if self.phase != Phase::ErraticsArrival {
            return None;
        }
        let impact = self.spec.timing.erratics_arrival_ticks * 3 / 4;
        let p = (self.phase_ticks as f32 / impact as f32).clamp(0.0, 1.0);
        let descent = p * p * p;
        Some(ArrivalPose {
            position: Vec2::new(
                actor.position.x - actor.facing.sign() * (1.0 - descent) * 260.0,
                actor.position.y - (1.0 - descent) * 1350.0,
            ),
            facing: actor.facing,
            depth: 0.0,
            visible: true,
            scale: 0.32 + 0.68 * descent,
            clip: if self.phase_ticks >= impact {
                "land"
            } else {
                "arrival"
            },
            ticks: if self.phase_ticks >= impact {
                self.phase_ticks - impact
            } else {
                self.phase_ticks
            },
            impact_age: (self.phase_ticks >= impact).then(|| self.phase_ticks - impact),
        })
    }
}

fn facing_toward(x: f32, target: f32) -> Facing {
    if x > target {
        Facing::Left
    } else {
        Facing::Right
    }
}

fn smooth(value: f32) -> f32 {
    let p = value.clamp(0.0, 1.0);
    p * p * (3.0 - 2.0 * p)
}
