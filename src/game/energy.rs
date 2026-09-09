//! Tracks contact-earned energy and cinematic costs independently from presentation.
//!
//! System: Match runtime. The World reports confirmed non-cinematic contacts;
//! tools can opt out of costs without changing damage, moves, or their timing.

use crate::combat::fighter::PlayerSlot;

pub const MAX_ENERGY: u16 = 100;
pub const INITIAL_ENERGY: u16 = 50;
pub const CINEMATIC_ENERGY_COST: u16 = 100;
pub const HIT_DEALT_ENERGY: u16 = 12;
pub const HIT_RECEIVED_ENERGY: u16 = 8;
pub const BLOCKED_ATTACK_ENERGY: u16 = 4;
pub const BLOCK_ENERGY: u16 = 6;

/// Determines whether cinematics require a full meter.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum EnergyPolicy {
    /// Lab, showcase and low-level simulation retain unrestricted move access.
    #[default]
    Unlimited,
    /// Matches earn energy through confirmed contacts and spend it on entry.
    Metered,
}

/// Read-only snapshot of one fighter's bounded energy reserve.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnergyMeter {
    amount: u16,
}

impl Default for EnergyMeter {
    fn default() -> Self {
        Self {
            amount: INITIAL_ENERGY,
        }
    }
}

impl EnergyMeter {
    /// Current energy in the inclusive range zero to `MAX_ENERGY`.
    pub const fn amount(self) -> u16 {
        self.amount
    }

    /// Filled fraction of the energy bar, between zero and one.
    pub fn fraction(self) -> f32 {
        self.amount as f32 / MAX_ENERGY as f32
    }

    /// Whether this reserve can pay the cinematic cost.
    pub const fn ready(self) -> bool {
        self.amount >= CINEMATIC_ENERGY_COST
    }

    fn gain(&mut self, amount: u16) {
        self.amount = self.amount.saturating_add(amount).min(MAX_ENERGY);
    }
}

/// Per-match policy and reserves; contacts are provided directly by resolution.
#[derive(Clone, Debug, Default)]
pub(crate) struct MatchEnergy {
    pub policy: EnergyPolicy,
    meters: [EnergyMeter; 2],
}

impl MatchEnergy {
    pub fn meter(&self, slot: PlayerSlot) -> EnergyMeter {
        self.meters[slot_index(slot)]
    }

    pub fn ready(&self, slot: PlayerSlot) -> bool {
        self.policy == EnergyPolicy::Unlimited || self.meter(slot).ready()
    }

    pub fn reset(&mut self) {
        self.meters = [EnergyMeter::default(); 2];
    }

    pub fn spend_cinematic(&mut self, slot: PlayerSlot) -> bool {
        if self.policy == EnergyPolicy::Unlimited {
            return true;
        }
        let meter = &mut self.meters[slot_index(slot)];
        if !meter.ready() {
            return false;
        }
        meter.amount -= CINEMATIC_ENERGY_COST;
        true
    }

    pub fn record_contact(&mut self, attacker: PlayerSlot, defender: PlayerSlot, blocked: bool) {
        if self.policy == EnergyPolicy::Unlimited {
            return;
        }
        let (attack_gain, defend_gain) = if blocked {
            (BLOCKED_ATTACK_ENERGY, BLOCK_ENERGY)
        } else {
            (HIT_DEALT_ENERGY, HIT_RECEIVED_ENERGY)
        };
        self.meters[slot_index(attacker)].gain(attack_gain);
        self.meters[slot_index(defender)].gain(defend_gain);
    }
}

const fn slot_index(slot: PlayerSlot) -> usize {
    match slot {
        PlayerSlot::One => 0,
        PlayerSlot::Two => 1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn meter_clamps_large_gains_and_spends_only_a_full_reserve() {
        let mut energy = MatchEnergy {
            policy: EnergyPolicy::Metered,
            ..Default::default()
        };
        assert_eq!(energy.meter(PlayerSlot::One).fraction(), 0.5);
        assert!(!energy.spend_cinematic(PlayerSlot::One));
        assert_eq!(energy.meter(PlayerSlot::One).amount(), INITIAL_ENERGY);
        energy.meters[0].gain(u16::MAX);
        assert_eq!(energy.meter(PlayerSlot::One).fraction(), 1.0);
        assert!(energy.spend_cinematic(PlayerSlot::One));
        assert_eq!(energy.meter(PlayerSlot::One).amount(), 0);
        assert!(!energy.spend_cinematic(PlayerSlot::One));
        energy.reset();
        assert_eq!(energy.policy, EnergyPolicy::Metered);
        assert_eq!(energy.meter(PlayerSlot::One).amount(), INITIAL_ENERGY);
    }
}
