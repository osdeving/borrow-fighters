//! Owns the chronological prologue, Rust's morning and the encounter's aftermath.
//!
//! System: Adventure domain. This explicit sequence advances at 60 Hz and accepts
//! scene skipping without granting a combat victory. The platform owns pause.

use super::combat::{Action, Combat, CombatInput, Outcome, TICKS_PER_SECOND};

/// Duration of Rust's automatic waking and morning sequence.
pub const MORNING_TICKS: u32 = 14 * TICKS_PER_SECOND;
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
    /// The player explores, encounters the creature and fights for survival.
    Encounter,
    /// Rust approaches the fallen creature and shows compassion.
    Aftermath,
    /// The playable opening has completed its final gesture.
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
    /// Current stage, directly available to the renderer.
    pub stage: Stage,
    /// Fixed updates elapsed since the current stage began.
    pub stage_ticks: u32,
    /// Authoritative player, enemy and encounter state.
    pub combat: Combat,
}

impl Default for Story {
    fn default() -> Self {
        Self::new()
    }
}

impl Story {
    /// Begins with Ada before the mysterious message.
    pub fn new() -> Self {
        Self {
            stage: Stage::AdaPrologue,
            stage_ticks: 0,
            combat: Combat::new(),
        }
    }

    /// Advances one fixed update; the caller must omit this call while paused.
    pub fn tick(&mut self, input: CombatInput) {
        if self.stage == Stage::Complete {
            return;
        }
        self.stage_ticks = self.stage_ticks.saturating_add(1);
        match self.stage {
            Stage::AdaPrologue if self.stage_ticks >= PROLOGUE_TICKS => {
                self.enter(Stage::RustMorning);
            }
            Stage::RustMorning if self.stage_ticks >= MORNING_TICKS => {
                self.enter(Stage::Encounter);
            }
            Stage::Encounter => {
                self.combat.tick(input);
                if self.combat.outcome == Outcome::Victory {
                    self.enter(Stage::Aftermath);
                }
            }
            Stage::Aftermath => {
                self.combat.tick_aftermath();
                if self.stage_ticks >= AFTERMATH_TICKS && self.gesture_visible() {
                    self.enter(Stage::Complete);
                }
            }
            _ => {}
        }
    }

    /// Skips a cinematic stage; this never removes health or wins an encounter.
    pub fn advance_scene(&mut self) {
        match self.stage {
            Stage::AdaPrologue => self.enter(Stage::RustMorning),
            Stage::RustMorning => self.enter(Stage::Encounter),
            Stage::Aftermath if self.gesture_visible() => self.enter(Stage::Complete),
            _ => {}
        }
    }

    /// Restarts only a lost or completed encounter, preserving the seen prologue.
    pub fn retry(&mut self) {
        if self.combat.outcome == Outcome::Defeat || self.stage == Stage::Complete {
            self.combat = Combat::at_checkpoint();
            self.enter(Stage::Encounter);
        }
    }

    /// Restarts the entire opening, including Ada and Rust's morning.
    pub fn restart(&mut self) {
        *self = Self::new();
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

    fn enter(&mut self, stage: Stage) {
        self.stage = stage;
        self.stage_ticks = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(story.combat.player.position.x, 340.0);
        assert_eq!(story.combat.player.action, Action::Idle);
        assert!(!story.combat.enemy_awake);
        story.tick(CombatInput {
            movement: 1.0,
            ..CombatInput::default()
        });
        assert!(story.combat.player.position.x > 340.0);
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
        assert_eq!(story.combat.ticks, 0);
        story.restart();
        assert_eq!(story.stage, Stage::AdaPrologue);
        assert!(!story.combat.enemy_awake);
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
}
