//! Owns the chronological prologue, Rust's morning and the encounter's aftermath.
//!
//! System: Adventure domain. This explicit sequence advances at 60 Hz and accepts
//! scene skipping without granting a combat victory. The platform owns pause.

use super::ambient::AmbientState;
#[cfg(test)]
use super::arrival::ARRIVAL_TICKS;
use super::arrival::{ArrivalShot, EpArrival, StreetArrivalSpec};
use super::combat::{Action, Combat, CombatInput, Outcome, TICKS_PER_SECOND};

/// Duration of Rust's automatic waking and morning sequence.
pub const MORNING_TICKS: u32 = 14 * TICKS_PER_SECOND;
/// Duration of the newspaper, character and title presentation.
pub const OPENING_TICKS: u32 = 60 * TICKS_PER_SECOND;
/// Duration reserved for approaching the creature and showing regret.
pub const AFTERMATH_TICKS: u32 = 7 * TICKS_PER_SECOND;
/// Minimum visible regret before an explicit scene advance may complete the slice.
pub const MIN_REMORSE_TICKS: u32 = 2 * TICKS_PER_SECOND;

/// Major narrative stage, including the player-controlled encounter.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stage {
    /// Ada's ordinary life, discovery and the first entity's awakening.
    AdaPrologue,
    /// Much later, Rust wakes and begins an otherwise ordinary morning.
    RustMorning,
    /// The camera arrives, then the player explores and fights for survival.
    Encounter,
    /// Rust approaches the fallen creature and shows compassion.
    Aftermath,
    /// Newspapers, character histories and the title after Rust shows compassion.
    Opening,
    /// The playable opening has completed its final gesture and presentation.
    Complete,
}

/// Ordered visual beats; their text and illustrations belong to presentation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PrologueBeat {
    /// Ada appears as an ordinary human in her familiar surroundings.
    AdaOrdinaryLife,
    /// An unexplained message interrupts the ordinary scene.
    StrangeMessage,
    /// Ada studies and follows the connection she has begun to perceive.
    FollowingTheSignal,
    /// Her contact awakens Assembly, the first programmatic entity.
    AssemblyAwakens,
    /// The encounter leaves a mysterious change, without explaining Ada's nature.
    AfterTheContact,
    /// A long interval separates Ada's contact from Rust's present.
    LongYears,
}

impl PrologueBeat {
    /// Ordered prologue beats for presentation and deterministic progression.
    pub const ALL: [Self; 6] = [
        Self::AdaOrdinaryLife,
        Self::StrangeMessage,
        Self::FollowingTheSignal,
        Self::AssemblyAwakens,
        Self::AfterTheContact,
        Self::LongYears,
    ];

    /// Duration of this visual beat in simulation updates.
    pub const fn duration_ticks(self) -> u32 {
        let seconds = match self {
            Self::AdaOrdinaryLife => 6,
            Self::StrangeMessage => 7,
            Self::FollowingTheSignal => 8,
            Self::AssemblyAwakens => 8,
            Self::AfterTheContact => 7,
            Self::LongYears => 4,
        };
        seconds * TICKS_PER_SECOND
    }
}

/// Complete prologue duration, including the transition across the years.
pub const PROLOGUE_TICKS: u32 = 40 * TICKS_PER_SECOND;

/// Story progress and the independent encounter it owns.
#[derive(Clone, Debug)]
pub struct Story {
    /// World translation of the original neighborhood and its ambient actors.
    pub hub_origin: f32,
    /// Physical street bounds and event locations, independent of presentation.
    pub map: super::landscape::PrologueMap,
    /// Current stage, directly available to the renderer.
    pub stage: Stage,
    /// Fixed updates elapsed since the current stage began.
    pub stage_ticks: u32,
    /// Authoritative player, enemy and encounter state.
    pub combat: Combat,
    /// Background animation that observes the encounter without changing combat.
    pub ambient: AmbientState,
    /// Independently authored descent whose ground contact starts the turmoil.
    pub ep_arrival: EpArrival,
    /// Replaceable calm-street camera track, frozen for this story instance.
    pub street_arrival: StreetArrivalSpec,
}

impl Default for Story {
    fn default() -> Self {
        Self::new()
    }
}

impl Story {
    /// Begins with Ada before the mysterious message.
    pub fn new() -> Self {
        let landscape = super::landscape::Landscape::bundled();
        let hub_origin = landscape.scene("street").hub_origin;
        let map = landscape.prologue;
        let mut combat = Combat::new();
        combat.set_bounds(map.bounds[0], map.bounds[1]);
        combat.player.position.x = map.spawn_x;
        combat.enemy.position.x = map.enemy_x;
        Self {
            hub_origin,
            map,
            stage: Stage::AdaPrologue,
            stage_ticks: 0,
            combat,
            ambient: AmbientState::default(),
            ep_arrival: EpArrival::default(),
            street_arrival: StreetArrivalSpec::default(),
        }
    }

    /// Applies a validated runtime map before the street is first entered.
    pub fn configure_map(&mut self, map: super::landscape::PrologueMap) {
        self.combat.set_bounds(map.bounds[0], map.bounds[1]);
        self.combat.player.position.x = map.spawn_x;
        self.combat.enemy.position.x = map.enemy_x;
        self.map = map;
        if self.stage == Stage::Encounter {
            self.ambient = AmbientState::new_in_bounds(
                self.combat.enemy_awake,
                [-self.hub_origin, self.map.width - self.hub_origin],
            );
        }
    }

    /// Left edge of the gameplay viewport within the physical map.
    pub fn camera_left(&self) -> f32 {
        (self.combat.player.position.x - 450.0).clamp(0.0, self.map.width - 1280.0)
    }

    /// Initial camera in the same viewport coordinates used by drawing and traces.
    pub fn initial_shot(&self) -> ArrivalShot {
        self.street_arrival.sample(
            self.stage_ticks,
            self.hub_origin,
            self.camera_left(),
            self.map.width,
        )
    }

    /// Fade opacity that covers the authored cut from neighbours back to Rust.
    pub fn initial_blackout(&self) -> f32 {
        self.street_arrival.blackout(self.stage_ticks)
    }

    /// Data-owned end of the street establishment and its input lock.
    pub fn arrival_duration(&self) -> u32 {
        self.street_arrival.duration_ticks()
    }

    /// Advances one fixed update; the caller must omit this call while paused.
    pub fn tick(&mut self, input: CombatInput) {
        if self.stage == Stage::Complete {
            return;
        }
        let arriving = self.arrival_active();
        self.stage_ticks = self.stage_ticks.saturating_add(1);
        match self.stage {
            Stage::AdaPrologue if self.stage_ticks >= PROLOGUE_TICKS => {
                self.enter(Stage::RustMorning);
            }
            Stage::RustMorning if self.stage_ticks >= MORNING_TICKS => {
                self.enter(Stage::Encounter);
            }
            Stage::Encounter => {
                if !arriving {
                    self.tick_encounter(input);
                }
                self.ambient.tick(self.combat.enemy_awake);
                if self.combat.outcome == Outcome::Victory {
                    self.enter(Stage::Aftermath);
                }
            }
            Stage::Aftermath => {
                self.combat.tick_aftermath();
                self.ep_arrival.tick();
                self.ambient.tick(self.combat.enemy_awake);
                if self.stage_ticks >= AFTERMATH_TICKS && self.gesture_visible() {
                    self.enter(Stage::Opening);
                }
            }
            Stage::Opening if self.stage_ticks >= OPENING_TICKS => self.enter(Stage::Complete),
            _ => {}
        }
    }

    /// Skips a cinematic stage; this never removes health or wins an encounter.
    pub fn advance_scene(&mut self) {
        match self.stage {
            Stage::AdaPrologue => self.enter(Stage::RustMorning),
            Stage::RustMorning => self.enter(Stage::Encounter),
            Stage::Encounter if self.arrival_active() => self.stage_ticks = self.arrival_duration(),
            Stage::Encounter if self.ep_arrival_active() => self.finish_ep_arrival(),
            Stage::Aftermath if self.gesture_visible() => self.enter(Stage::Opening),
            Stage::Opening => self.enter(Stage::Complete),
            _ => {}
        }
    }

    /// Skips to the next authored segment instead of discarding its whole stage.
    ///
    /// An arrival skip settles the camera and releases exploration. A later
    /// explicit encounter skip goes to the presentation, preserving
    /// combat health and outcome. It does not invent a victory or show regret for
    /// a fight the player skipped. Likewise, aftermath can be skipped before its
    /// gesture finishes; automatic progression still waits for that gesture.
    pub fn skip_segment(&mut self) {
        match self.stage {
            Stage::AdaPrologue => {
                if let Some((beat, elapsed)) = self.beat_and_ticks() {
                    self.stage_ticks += beat.duration_ticks() - elapsed;
                    if self.stage_ticks >= PROLOGUE_TICKS {
                        self.enter(Stage::RustMorning);
                    }
                }
            }
            Stage::RustMorning => {
                // Wake, lift the torso, sit at the edge, plant the foot and stand.
                // These are authored pose starts in engine/morning.rs at 60 Hz.
                self.skip_to_next_boundary(&[150, 225, 355, 500, 585], Stage::Encounter);
            }
            Stage::Encounter if self.arrival_active() => self.stage_ticks = self.arrival_duration(),
            Stage::Encounter if self.ep_arrival_active() => self.finish_ep_arrival(),
            Stage::Encounter | Stage::Aftermath => self.enter(Stage::Opening),
            Stage::Opening => {
                // Three headlines, both halves of each biography and the final
                // logo. Rust remains in the title without a separate cast card.
                let boundaries = [3, 6, 9, 14, 19, 24, 29, 35, 41, 47, 53]
                    .map(|seconds| seconds * TICKS_PER_SECOND);
                self.skip_to_next_boundary(&boundaries, Stage::Complete);
            }
            Stage::Complete => {}
        }
    }

    /// Begins the presentation directly for review without fabricating a combat victory.
    pub fn presentation() -> Self {
        let mut story = Self::new();
        story.enter(Stage::Opening);
        story
    }

    /// Replays the presentation after completing the slice, keeping encounter health intact.
    pub fn replay_presentation(&mut self) {
        if self.stage == Stage::Complete {
            self.enter(Stage::Opening);
        }
    }

    /// Restarts only a lost or completed encounter, preserving the seen prologue.
    pub fn retry(&mut self) {
        if self.combat.outcome == Outcome::Defeat || self.stage == Stage::Complete {
            self.combat = Combat::at_checkpoint();
            self.combat
                .set_bounds(self.map.bounds[0], self.map.bounds[1]);
            self.combat.player.position.x = self.map.arrival_x;
            self.combat.enemy.position.x = self.map.enemy_x;
            self.enter(Stage::Encounter);
        }
    }

    /// Restarts the entire opening, including Ada and Rust's morning.
    pub fn restart(&mut self) {
        let map = self.map.clone();
        let hub = self.hub_origin;
        *self = Self::new();
        self.hub_origin = hub;
        self.configure_map(map);
    }

    /// Whether the authored camera still owns the encounter's initial view.
    /// Awakened checkpoints resume combat directly, without another camera move.
    pub fn arrival_active(&self) -> bool {
        self.stage == Stage::Encounter
            && !self.combat.enemy_awake
            && self.combat.ticks == 0
            && self.stage_ticks < self.arrival_duration()
    }

    /// Whether the EP descent/recovery currently holds combat input.
    pub fn ep_arrival_active(&self) -> bool {
        self.stage == Stage::Encounter && self.ep_arrival.active()
    }

    fn tick_encounter(&mut self, input: CombatInput) {
        if self.ep_arrival.active() {
            self.ep_arrival.tick();
            self.combat.player.action_ticks = self.combat.player.action_ticks.saturating_add(1);
            if self.ep_arrival.impacted() {
                self.combat.enemy_awake = true;
            }
        } else if !self.combat.enemy_awake && self.combat.outcome == Outcome::Ongoing {
            // The prologue owns this appearance. Standalone encounter rules and
            // chapter encounters retain their own activation policies.
            self.combat.tick_exploration(input);
            if self.combat.player.position.x >= self.map.arrival_x && self.combat.player.grounded {
                self.ep_arrival.start();
                self.combat.player.action = Action::Idle;
                self.combat.player.action_ticks = 0;
                self.combat.player.velocity.x = 0.0;
            }
        } else {
            self.ep_arrival.tick();
            self.combat.tick(input);
        }
    }

    fn finish_ep_arrival(&mut self) {
        self.ep_arrival.finish();
        self.combat.enemy_awake = true;
        // A local scene skip lands safely and starts evacuation at its origin;
        // it cannot grant victory or replay an already-started street reaction.
        if self.ambient.accident_ticks().is_none() {
            self.ambient.tick(true);
        }
    }

    /// Returns the current prologue beat, or none after the historical sequence.
    pub fn prologue_beat(&self) -> Option<PrologueBeat> {
        self.beat_and_ticks().map(|(beat, _)| beat)
    }

    /// Returns elapsed updates within the active prologue beat.
    pub fn beat_ticks(&self) -> u32 {
        self.beat_and_ticks().map_or(0, |(_, ticks)| ticks)
    }

    fn beat_and_ticks(&self) -> Option<(PrologueBeat, u32)> {
        if self.stage != Stage::AdaPrologue {
            return None;
        }
        let mut elapsed = self.stage_ticks;
        for beat in PrologueBeat::ALL {
            if elapsed < beat.duration_ticks() {
                return Some((beat, elapsed));
            }
            elapsed -= beat.duration_ticks();
        }
        Some((PrologueBeat::LongYears, 0))
    }

    fn gesture_visible(&self) -> bool {
        self.combat.player.action == Action::Remorse
            && self.combat.player.action_ticks >= MIN_REMORSE_TICKS
    }

    fn skip_to_next_boundary(&mut self, boundaries: &[u32], next_stage: Stage) {
        if let Some(&ticks) = boundaries.iter().find(|&&ticks| ticks > self.stage_ticks) {
            self.stage_ticks = ticks;
        } else {
            self.enter(next_stage);
        }
    }

    fn enter(&mut self, stage: Stage) {
        self.stage = stage;
        self.stage_ticks = 0;
        if stage == Stage::Encounter {
            self.ambient = AmbientState::new_in_bounds(
                self.combat.enemy_awake,
                [-self.hub_origin, self.map.width - self.hub_origin],
            );
            self.ep_arrival.reset(self.combat.enemy_awake);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adventure::ambient::{CyclistPhase, IncidentPhase, KidPhase};

    #[test]
    fn background_reaction_continues_through_victory_and_stops_after_the_street() {
        let mut story = Story::new();
        story.advance_scene();
        for _ in 0..30 {
            story.tick(CombatInput::default());
        }
        assert_eq!(story.ambient, AmbientState::default());
        story.advance_scene();
        story.skip_segment(); // The camera hands control to the player.
        story.combat.player.position.x = story.map.arrival_x;
        story.tick(CombatInput::default());
        while !story.ep_arrival.impacted() {
            assert_eq!(story.ambient.accident_ticks(), None);
            story.tick(CombatInput::default());
        }
        assert_eq!(story.ambient.kid_phase(), KidPhase::Startled);
        assert_eq!(story.ambient.accident_ticks(), Some(0));
        for _ in 0..60 {
            story.tick(CombatInput::default());
        }
        assert_eq!(story.ambient.kid_phase(), KidPhase::Running);
        assert!(
            story
                .ambient
                .cyclists()
                .iter()
                .all(|cyclist| cyclist.phase == CyclistPhase::Running)
        );
        let before_victory = story.ambient.clone();
        story.combat.outcome = Outcome::Victory;
        story.combat.enemy.hp = 0;
        story.combat.enemy.action = Action::Defeated;
        story.tick(CombatInput::default());
        assert_eq!(story.stage, Stage::Aftermath);
        assert_eq!(story.ambient.ticks(), before_victory.ticks() + 1);
        assert!(story.ambient.kid_position().x < before_victory.kid_position().x);
        for _ in 0..300 {
            let previous_x = story.ambient.kid_position().x;
            let previous_ticks = story.ambient.ticks();
            story.tick(CombatInput::default());
            assert_eq!(story.ambient.ticks(), previous_ticks + 1);
            let travelled = previous_x - story.ambient.kid_position().x;
            assert!((0.0..=4.501).contains(&travelled));
            if story.ambient.kid_phase() == KidPhase::Gone {
                assert!(story.ambient.kid_position().x + story.hub_origin < 0.0);
            }
            for cyclist in story
                .ambient
                .cyclists()
                .iter()
                .filter(|actor| !actor.visible)
            {
                let x = cyclist.position.x + story.hub_origin;
                assert!(x < 0.0 || x > story.map.width);
                assert_eq!(cyclist.phase, CyclistPhase::Gone);
            }
            for car in story
                .ambient
                .traffic_cars()
                .iter()
                .filter(|actor| !actor.visible)
            {
                assert!(car.position.x + story.hub_origin < 0.0);
            }
        }
        assert_eq!(
            story
                .ambient
                .abandoned_bicycles()
                .map(|bike| bike.unwrap().position),
            before_victory
                .abandoned_bicycles()
                .map(|bike| bike.unwrap().position)
        );
        let wreck = story.ambient.incident_car().unwrap();
        assert_eq!(wreck.phase, IncidentPhase::Crashed);
        story.tick(CombatInput::default());
        let later_wreck = story.ambient.incident_car().unwrap();
        assert_eq!(later_wreck.position, wreck.position);
        assert_eq!(later_wreck.phase_ticks, wreck.phase_ticks + 1);

        story.skip_segment();
        assert_eq!(story.stage, Stage::Opening);
        let before_opening = story.ambient.clone();
        for _ in 0..60 {
            story.tick(CombatInput::default());
        }
        assert_eq!(story.ambient, before_opening);
    }

    #[test]
    fn retry_reacts_at_the_awake_checkpoint_and_restart_restores_a_quiet_street() {
        let mut story = Story::new();
        story.advance_scene();
        story.advance_scene();
        story.combat.enemy_awake = true;
        story.tick(CombatInput::default());
        let bounds = [-story.hub_origin, story.map.width - story.hub_origin];
        let evacuated = AmbientState::settled_after_reaction_in_bounds(
            story.ambient.reaction_origin().unwrap(),
            0,
            bounds,
        );
        // Wait for the configured street's full escape, then compare the played
        // positions and clocks with the chapter's reconstruction of that alarm.
        for _ in 0..evacuated.accident_ticks().unwrap() {
            let previous_x = story.ambient.kid_position().x;
            story.tick(CombatInput::default());
            let travelled = previous_x - story.ambient.kid_position().x;
            assert!((0.0..=4.501).contains(&travelled));
        }
        assert_eq!(story.ambient, evacuated);
        assert_eq!(story.ambient.kid_phase(), KidPhase::Gone);
        assert!(story.ambient.kid_position().x + story.hub_origin < 0.0);
        assert!(story.ambient.traffic_cars().iter().all(|car| !car.visible));
        for car in story.ambient.traffic_cars() {
            assert!(car.position.x + story.hub_origin < 0.0);
        }
        assert!(
            story
                .ambient
                .cyclists()
                .iter()
                .all(|cyclist| !cyclist.visible)
        );
        for cyclist in story.ambient.cyclists() {
            let x = cyclist.position.x + story.hub_origin;
            assert!(x < 0.0 || x > story.map.width);
        }
        assert!(
            story
                .ambient
                .abandoned_bicycles()
                .iter()
                .all(Option::is_some)
        );
        assert_eq!(
            story.ambient.incident_car().unwrap().phase,
            IncidentPhase::Crashed
        );
        story.combat.outcome = Outcome::Defeat;
        story.retry();
        assert!(story.combat.enemy_awake);
        assert_eq!(story.ambient.ticks(), 0);
        assert_eq!(story.ambient.kid_phase(), KidPhase::Startled);
        assert_eq!(story.ambient.kid_phase_ticks(), 0);
        assert_eq!(story.ambient.kite_release_ticks(), None);
        assert_eq!(story.ambient.accident_ticks(), Some(0));
        assert!(
            story
                .ambient
                .traffic_cars()
                .iter()
                .all(|car| car.visible && car.fleeing)
        );
        assert!(
            story
                .ambient
                .cyclists()
                .iter()
                .all(|cyclist| cyclist.visible && cyclist.phase == CyclistPhase::Braking)
        );
        assert_eq!(story.ambient.abandoned_bicycles(), [None, None]);
        assert_eq!(
            story.ambient.incident_car().unwrap().phase,
            IncidentPhase::Approaching
        );
        story.tick(CombatInput::default());
        assert_eq!(story.ambient.kid_phase_ticks(), 1);
        assert_eq!(story.ambient.accident_ticks(), Some(1));

        let current = story.ambient.clone();
        story.retry();
        assert_eq!(story.ambient, current);
        story.restart();
        story.advance_scene();
        story.advance_scene();
        assert_eq!(story.ambient, AmbientState::new_in_bounds(false, bounds));
        assert_eq!(story.ambient.incident_car(), None);
        assert!(
            story
                .ambient
                .traffic_cars()
                .iter()
                .all(|car| car.visible && !car.fleeing)
        );
        assert!(
            story
                .ambient
                .cyclists()
                .iter()
                .all(|cyclist| cyclist.visible && cyclist.phase == CyclistPhase::Riding)
        );
        assert_eq!(story.ambient.abandoned_bicycles(), [None, None]);
        assert!(!story.combat.enemy_awake);
    }

    #[test]
    fn scenery_changes_cannot_change_encounter_rules_or_progress() {
        let mut story = Story::new();
        story.advance_scene();
        story.advance_scene();
        let mut alternate = story.clone();
        // Force entirely different decoration, leaving combat identical.
        for _ in 0..900 {
            alternate.ambient.tick(true);
        }
        assert_ne!(story.ambient, alternate.ambient);

        for tick in 0..720 {
            let input = CombatInput {
                movement: if tick < 250 { 1.0 } else { -0.5 },
                jump_pressed: tick % 61 == 0,
                light_pressed: tick % 27 == 0,
                heavy_pressed: tick % 47 == 0,
                kick_pressed: false,
                blocking: tick % 99 < 20,
            };
            story.tick(input);
            alternate.tick(input);
            assert_eq!(story.stage, alternate.stage);
            assert_eq!(story.stage_ticks, alternate.stage_ticks);
            assert_eq!(
                format!("{:?}", story.combat),
                format!("{:?}", alternate.combat)
            );
        }
    }

    #[test]
    fn automatic_sequence_preserves_ada_then_morning_before_player_control() {
        let mut story = Story::new();
        let noisy_input = CombatInput {
            movement: 1.0,
            jump_pressed: true,
            light_pressed: true,
            ..CombatInput::default()
        };
        assert_eq!(
            PrologueBeat::ALL
                .iter()
                .map(|beat| beat.duration_ticks())
                .sum::<u32>(),
            PROLOGUE_TICKS
        );
        for beat in PrologueBeat::ALL {
            assert_eq!(story.prologue_beat(), Some(beat));
            assert_eq!(story.beat_ticks(), 0);
            for _ in 0..beat.duration_ticks() {
                story.tick(noisy_input);
            }
        }
        assert_eq!(story.stage, Stage::RustMorning);
        assert_eq!(story.combat.ticks, 0);
        for _ in 0..MORNING_TICKS {
            story.tick(noisy_input);
        }
        assert_eq!(story.stage, Stage::Encounter);
        assert_eq!(story.combat.player.position.x, story.map.spawn_x);
        assert_eq!(story.combat.player.action, Action::Idle);
        assert!(!story.combat.enemy_awake);
        for _ in 0..ARRIVAL_TICKS {
            assert!(story.arrival_active());
            story.tick(noisy_input);
            assert_eq!(story.combat.ticks, 0);
            assert_eq!(story.combat.player.position.x, story.map.spawn_x);
            assert_eq!(story.combat.player.action, Action::Idle);
            assert!(!story.combat.enemy_awake);
        }
        assert!(!story.arrival_active());
        assert_eq!(story.ambient.ticks(), ARRIVAL_TICKS);
        story.tick(CombatInput {
            movement: 1.0,
            ..CombatInput::default()
        });
        assert!(story.combat.player.position.x > story.map.spawn_x);
    }

    #[test]
    fn skipping_cinematics_cannot_skip_the_encounter_or_its_compassionate_gesture() {
        let mut story = Story::new();
        story.advance_scene();
        assert_eq!(story.stage, Stage::RustMorning);
        story.advance_scene();
        assert_eq!(story.stage, Stage::Encounter);
        for _ in 0..20 {
            story.advance_scene();
        }
        assert_eq!(story.stage, Stage::Encounter);
        assert_eq!(story.combat.enemy.hp, story.combat.enemy.max_hp);
        story.combat.outcome = Outcome::Victory;
        story.combat.enemy.hp = 0;
        story.combat.enemy.action = Action::Defeated;
        story.combat.player.position.x = story.combat.enemy.position.x - 90.0;
        story.tick(CombatInput::default());
        assert_eq!(story.stage, Stage::Aftermath);
        story.advance_scene();
        assert_eq!(story.stage, Stage::Aftermath);
        for _ in 0..AFTERMATH_TICKS {
            story.tick(CombatInput::default());
        }
        assert_eq!(story.combat.player.action, Action::Remorse);
        assert_eq!(story.stage, Stage::Opening);
        let health = (story.combat.player.hp, story.combat.enemy.hp);
        let combat_ticks = story.combat.ticks;
        for _ in 0..OPENING_TICKS {
            story.tick(CombatInput {
                light_pressed: true,
                movement: 1.0,
                ..CombatInput::default()
            });
        }
        assert_eq!(story.stage, Stage::Complete);
        assert_eq!(health, (story.combat.player.hp, story.combat.enemy.hp));
        assert_eq!(combat_ticks, story.combat.ticks);
        story.replay_presentation();
        assert_eq!(story.stage, Stage::Opening);
        story.advance_scene();
        assert_eq!(story.stage, Stage::Complete);
    }

    #[test]
    fn retry_after_defeat_restores_health_without_replaying_either_cinematic() {
        let mut story = Story::new();
        story.advance_scene();
        story.advance_scene();
        story.combat.player.hp = 0;
        story.combat.enemy.hp = 24;
        story.combat.outcome = Outcome::Defeat;
        story.retry();
        assert_eq!(story.stage, Stage::Encounter);
        assert_eq!(story.stage_ticks, 0);
        assert_eq!(story.combat.outcome, Outcome::Ongoing);
        assert_eq!(story.combat.player.hp, story.combat.player.max_hp);
        assert_eq!(story.combat.enemy.hp, story.combat.enemy.max_hp);
        assert!(story.combat.enemy_awake);
        assert!(!story.arrival_active());
        assert!(!story.ep_arrival_active());
        assert_eq!(story.combat.ticks, 0);
        story.restart();
        assert_eq!(story.stage, Stage::AdaPrologue);
        assert!(!story.combat.enemy_awake);
    }

    #[test]
    fn ep_landing_holds_combat_and_starts_turmoil_only_at_ground_contact() {
        let mut story = Story::new();
        story.advance_scene();
        story.advance_scene();
        story.skip_segment();
        story.combat.player.position.x = story.map.arrival_x;
        story.tick(CombatInput::default());
        let position = story.combat.player.position;
        let ticks = story.combat.ticks;
        let impact = story.ep_arrival.spec.impact_tick();
        assert!(story.ep_arrival_active());
        assert!(!story.combat.enemy_awake);
        let noisy = CombatInput {
            movement: 1.0,
            jump_pressed: true,
            light_pressed: true,
            heavy_pressed: true,
            kick_pressed: true,
            blocking: true,
        };
        for age in 1..impact {
            story.tick(noisy);
            assert_eq!(story.ep_arrival.ticks(), Some(age));
            assert_eq!(story.combat.player.position, position);
            assert_eq!(story.combat.ticks, ticks);
            assert!(!story.combat.enemy_awake);
            assert_eq!(story.ambient.accident_ticks(), None);
        }
        story.tick(noisy);
        assert!(story.combat.enemy_awake);
        assert_eq!(story.ambient.accident_ticks(), Some(0));
        while story.ep_arrival_active() {
            story.tick(noisy);
            assert_eq!(story.combat.player.position, position);
            assert_eq!(story.combat.player.hp, 100);
            assert_eq!(story.combat.enemy.hp, 96);
        }
        story.tick(CombatInput {
            movement: 1.0,
            ..CombatInput::default()
        });
        assert!(story.combat.player.position.x > position.x);
        assert_eq!(story.combat.ticks, ticks + 1);
    }

    #[test]
    fn skipping_ep_descent_or_recovery_releases_combat_without_victory() {
        for age in [0, 20, 40, 65, 100] {
            let mut story = Story::new();
            story.advance_scene();
            story.advance_scene();
            story.skip_segment();
            story.combat.player.position.x = story.map.arrival_x;
            story.tick(CombatInput::default());
            for _ in 0..age {
                story.tick(CombatInput::default());
            }
            let reaction_before = story.ambient.accident_ticks();
            story.skip_segment();
            assert_eq!(story.stage, Stage::Encounter);
            assert_eq!(story.combat.outcome, Outcome::Ongoing);
            assert_eq!(story.combat.player.hp, 100);
            assert_eq!(story.combat.enemy.hp, 96);
            assert!(story.combat.enemy_awake);
            assert!(!story.ep_arrival_active());
            assert_eq!(story.ambient.accident_ticks(), reaction_before.or(Some(0)));
            story.skip_segment();
            assert_eq!(story.stage, Stage::Opening);
        }
    }

    #[test]
    fn retry_during_live_combat_does_not_restore_health() {
        let mut story = Story::new();
        story.advance_scene();
        story.advance_scene();
        story.combat.player.hp = 30;
        story.retry();
        assert_eq!(story.combat.player.hp, 30);
        assert_eq!(story.stage, Stage::Encounter);
    }

    #[test]
    fn segment_skip_reaches_each_ada_beat_from_its_middle() {
        let mut story = Story::new();
        for next in PrologueBeat::ALL.into_iter().skip(1) {
            for _ in 0..30 {
                story.tick(CombatInput::default());
            }
            story.skip_segment();
            assert_eq!(story.stage, Stage::AdaPrologue);
            assert_eq!(story.prologue_beat(), Some(next));
            assert_eq!(story.beat_ticks(), 0);
        }
        story.skip_segment();
        assert_eq!(story.stage, Stage::RustMorning);
        assert_eq!(story.stage_ticks, 0);
        assert_eq!(story.combat.ticks, 0);
    }

    #[test]
    fn morning_skip_visits_waking_seated_and_standing_poses_before_encounter() {
        let mut story = Story::new();
        story.advance_scene();
        for ticks in [150, 225, 355, 500, 585] {
            story.skip_segment();
            assert_eq!(story.stage, Stage::RustMorning);
            assert_eq!(story.stage_ticks, ticks);
        }
        story.skip_segment();
        assert_eq!(story.stage, Stage::Encounter);
        assert_eq!(story.stage_ticks, 0);
        assert!(!story.combat.enemy_awake);
    }

    #[test]
    fn arrival_skip_releases_control_without_skipping_combat_or_rewinding_the_street() {
        for age in [0, 60, 360, 780, 840, 841, 900, ARRIVAL_TICKS - 1] {
            let mut story = Story::new();
            story.advance_scene();
            story.advance_scene();
            for _ in 0..age {
                story.tick(CombatInput::default());
            }
            let ambient = story.ambient.clone();
            story.skip_segment();
            assert_eq!(story.initial_blackout(), 0.0);
            assert_eq!(story.stage, Stage::Encounter);
            assert!(!story.arrival_active());
            assert_eq!(story.ambient, ambient);
            assert_eq!(story.combat.ticks, 0);
            story.tick(CombatInput {
                movement: 1.0,
                ..CombatInput::default()
            });
            assert!(story.combat.player.position.x > story.map.spawn_x);
            assert_eq!(story.combat.player.action, Action::Walk);
            assert_eq!(story.ambient.ticks(), age + 1);
            story.skip_segment();
            assert_eq!(story.stage, Stage::Opening);
        }
    }

    #[test]
    fn opening_skip_preserves_each_headline_biography_panel_and_character() {
        let mut story = Story::presentation();
        for seconds in [3, 6, 9, 14, 19, 24, 29, 35, 41, 47, 53] {
            story.tick(CombatInput::default());
            story.skip_segment();
            assert_eq!(story.stage, Stage::Opening);
            assert_eq!(story.stage_ticks, seconds * TICKS_PER_SECOND);
        }
        story.skip_segment();
        assert_eq!(story.stage, Stage::Complete);
    }

    #[test]
    fn skipping_an_encounter_preserves_damage_and_outcome_without_showing_regret() {
        for outcome in [Outcome::Ongoing, Outcome::Defeat] {
            let mut story = Story::new();
            story.advance_scene();
            story.advance_scene();
            story.skip_segment(); // Finish the initial camera move first.
            story.combat.player.hp = if outcome == Outcome::Defeat { 0 } else { 37 };
            story.combat.enemy.hp = 54;
            story.combat.outcome = outcome;
            let health = (story.combat.player.hp, story.combat.enemy.hp);
            let combat_ticks = story.combat.ticks;

            story.skip_segment();

            assert_eq!(story.stage, Stage::Opening);
            assert_eq!(story.stage_ticks, 0);
            assert_eq!(story.combat.outcome, outcome);
            assert_eq!(health, (story.combat.player.hp, story.combat.enemy.hp));
            assert_eq!(story.combat.ticks, combat_ticks);
            assert_ne!(story.combat.player.action, Action::Remorse);
        }
    }

    #[test]
    fn explicit_aftermath_skip_does_not_wait_for_the_automatic_remorse_gate() {
        let mut story = Story::new();
        story.advance_scene();
        story.advance_scene();
        story.combat.outcome = Outcome::Victory;
        story.combat.enemy.hp = 0;
        story.combat.enemy.action = Action::Defeated;
        story.tick(CombatInput::default());
        assert_eq!(story.stage, Stage::Aftermath);
        assert!(!story.gesture_visible());

        story.skip_segment();

        assert_eq!(story.stage, Stage::Opening);
        assert_eq!(story.combat.outcome, Outcome::Victory);
        assert_eq!(story.combat.enemy.hp, 0);
    }

    #[test]
    fn repeated_segment_skips_complete_the_sequence_and_stop_at_the_title() {
        let mut story = Story::new();
        for _ in 0..30 {
            story.skip_segment();
        }
        assert_eq!(story.stage, Stage::Complete);
        assert_eq!(story.stage_ticks, 0);
        assert_eq!(story.combat.outcome, Outcome::Ongoing);
        assert_eq!(story.combat.player.hp, story.combat.player.max_hp);
        assert_eq!(story.combat.enemy.hp, story.combat.enemy.max_hp);
        story.skip_segment();
        assert_eq!(story.stage, Stage::Complete);
        assert_eq!(story.stage_ticks, 0);
    }
}
