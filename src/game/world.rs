//! Owns the greybox fight simulation.
//!
//! This world is intentionally small and deterministic enough to unit test
//! without Raylib.

use crate::combat::collision::hitbox_hits_hurtbox;
use crate::combat::fighter::{Fighter, FighterInput, PlayerSlot};

/// Final result of a greybox match.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MatchOutcome {
    Winner(PlayerSlot),
    Draw,
}

/// Two-fighter world state for Prototype 0.1.
#[derive(Clone, Debug)]
pub struct World {
    pub player_one: Fighter,
    pub player_two: Fighter,
    pub outcome: Option<MatchOutcome>,
}

impl World {
    /// Creates the initial greybox fight.
    pub fn new_greybox() -> Self {
        let mut world = Self {
            player_one: Fighter::new(PlayerSlot::One, "Rust", 232.0),
            player_two: Fighter::new(PlayerSlot::Two, "Java", 676.0),
            outcome: None,
        };
        world.update_facing();
        world
    }

    /// Advances one fixed gameplay step.
    pub fn update(&mut self, dt: f32, player_one: FighterInput, player_two: FighterInput) {
        if self.outcome.is_some() {
            return;
        }

        self.update_facing();
        self.player_one.update(dt, player_one);
        self.player_two.update(dt, player_two);
        self.update_facing();
        self.resolve_hits();
        self.resolve_outcome();
    }

    fn update_facing(&mut self) {
        let p1 = self.player_one.clone();
        let p2 = self.player_two.clone();
        self.player_one.face_toward(&p2);
        self.player_two.face_toward(&p1);
    }

    fn resolve_hits(&mut self) {
        let p1_hits = self.player_one.active_hitbox().is_some_and(|hitbox| {
            self.player_one.can_register_hit()
                && hitbox_hits_hurtbox(hitbox, self.player_two.hurtbox())
        });

        let p2_hits = self.player_two.active_hitbox().is_some_and(|hitbox| {
            self.player_two.can_register_hit()
                && hitbox_hits_hurtbox(hitbox, self.player_one.hurtbox())
        });

        if p1_hits {
            self.player_two.take_basic_hit();
            self.player_one.mark_attack_hit();
        }

        if p2_hits {
            self.player_one.take_basic_hit();
            self.player_two.mark_attack_hit();
        }
    }

    fn resolve_outcome(&mut self) {
        self.outcome = match (self.player_one.is_defeated(), self.player_two.is_defeated()) {
            (true, true) => Some(MatchOutcome::Draw),
            (true, false) => Some(MatchOutcome::Winner(PlayerSlot::Two)),
            (false, true) => Some(MatchOutcome::Winner(PlayerSlot::One)),
            (false, false) => None,
        };
    }
}
