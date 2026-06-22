//! Records compact combat events for debugging and playtest reproduction.
//!
//! System: Match runtime. The combat log is diagnostic data owned by `World`;
//! it does not drive gameplay, audio, rendering, or balancing decisions.

use crate::characters::CharacterId;
use crate::combat::fighter::PlayerSlot;
use crate::combat::move_data::MoveId;

pub const COMBAT_LOG_CAPACITY: usize = 256;

/// One timestamped combat event emitted by the match runtime.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CombatLogEvent {
    pub time_seconds: f32,
    pub kind: CombatLogKind,
}

/// Small set of gameplay events useful for reproducing combat bugs.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CombatLogKind {
    RoundStarted {
        player_one: CharacterId,
        player_two: CharacterId,
    },
    CountdownStep {
        label: &'static str,
    },
    CloseAttackStarted {
        slot: PlayerSlot,
        character: CharacterId,
        move_id: MoveId,
    },
    CloseAttackWhiffed {
        slot: PlayerSlot,
        character: CharacterId,
        move_id: MoveId,
    },
    CloseAttackResolved {
        attacker: PlayerSlot,
        defender: PlayerSlot,
        attacker_character: CharacterId,
        defender_character: CharacterId,
        move_id: MoveId,
        damage: i32,
        blocked: bool,
    },
    ProjectileSpawned {
        slot: PlayerSlot,
        character: CharacterId,
        damage: i32,
    },
    ProjectileResolved {
        attacker: PlayerSlot,
        defender: PlayerSlot,
        attacker_character: CharacterId,
        defender_character: CharacterId,
        damage: i32,
        blocked: bool,
    },
    MatchEnded {
        winner: Option<PlayerSlot>,
        winner_character: Option<CharacterId>,
    },
}

/// Fixed-capacity event log for the current match.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CombatLog {
    events: Vec<CombatLogEvent>,
}

impl CombatLog {
    /// Records one event, dropping the oldest entries when the log is full.
    pub fn record(&mut self, time_seconds: f32, kind: CombatLogKind) {
        self.events.push(CombatLogEvent { time_seconds, kind });
        let overflow = self.events.len().saturating_sub(COMBAT_LOG_CAPACITY);
        if overflow > 0 {
            self.events.drain(0..overflow);
        }
    }

    /// Returns recorded events in chronological order.
    pub fn events(&self) -> &[CombatLogEvent] {
        &self.events
    }

    /// Clears the current diagnostic log.
    pub fn clear(&mut self) {
        self.events.clear();
    }
}
