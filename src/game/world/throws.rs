//! Coordinates paired throws and move-specific damage reactions.
//!
//! System: Match combat. Captures lift continuously, then release into the same
//! gravity used by other launches. Landing determines recovery and round ending.

use super::*;
use crate::{
    combat::fighter::{GRAVITY, HitReactionKind},
    config::{FIXED_TIMESTEP, FLOOR_Y},
};

const CAPTURE_SECONDS: f32 = 12.0 * FIXED_TIMESTEP;
const FLIGHT_FRAMES: f32 = 40.0;

/// The paired capture or the released ballistic portion of a throw.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ThrowPhase {
    Capture,
    Flight,
}

/// Throw presentation data; fighter positions remain authoritative.
#[derive(Clone, Copy, Debug)]
pub struct ThrowSequence {
    pub attacker: PlayerSlot,
    pub victim: PlayerSlot,
    pub phase: ThrowPhase,
    pub elapsed_seconds: f32,
    pub landing_x: f32,
    start_x: f32,
    release_x: f32,
    start_y: f32,
    release_y: f32,
}

impl World {
    /// Exposes the active throw for synchronized capture/release artwork.
    pub fn throw_sequence(&self) -> Option<&ThrowSequence> {
        self.throw_sequence.as_ref()
    }

    pub(super) fn apply_close_reaction(
        &mut self,
        attacker: PlayerSlot,
        attack: ActiveAttack,
        result: DamageResult,
        direction: f32,
    ) {
        if result.damage > 0 && !result.blocked && attack.guard_rule == GuardRule::Throw {
            self.begin_throw(attacker);
            return;
        }
        let defender = match attacker {
            PlayerSlot::One => &mut self.player_two,
            PlayerSlot::Two => &mut self.player_one,
        };
        apply_pushback(defender, direction, result.pushback);
        if result.damage <= 0 || result.blocked {
            return;
        }
        match attack.kind {
            crate::combat::fighter::AttackKind::AntiAir => {
                let rise =
                    (defender.position.y - world_px(66.0)).clamp(world_px(45.0), world_px(170.0));
                defender.begin_launch(
                    Vec2::new(direction * world_px(130.0), -(2.0 * GRAVITY * rise).sqrt()),
                    HitReactionKind::Launched,
                );
            }
            crate::combat::fighter::AttackKind::HeavyPunch
            | crate::combat::fighter::AttackKind::Overhead
            | crate::combat::fighter::AttackKind::CinematicSpecial => {
                defender.mark_heavy_reaction()
            }
            _ => apply_knockdown_for_move(defender, attack, result),
        }
    }

    fn begin_throw(&mut self, attacker_slot: PlayerSlot) {
        let (attacker, victim) = match attacker_slot {
            PlayerSlot::One => (&mut self.player_one, &mut self.player_two),
            PlayerSlot::Two => (&mut self.player_two, &mut self.player_one),
        };
        let direction = if victim.position.x >= attacker.position.x {
            1.0
        } else {
            -1.0
        };
        let behind = if direction > 0.0 {
            attacker.position.x - victim.body_rect().width - world_px(70.0)
        } else {
            attacker.body_rect().right() + world_px(70.0)
        };
        let max_x = ARENA_RIGHT - victim.body_rect().width;
        // The middle guarantees a side switch. At the boundary, throw toward
        // open center space instead of teleporting either actor to make room.
        let landing_x = if (ARENA_LEFT..=max_x).contains(&behind) {
            behind
        } else {
            (victim.position.x + direction * world_px(170.0)).clamp(ARENA_LEFT, max_x)
        };
        let release_y =
            attacker.position.y - victim.body_metrics().standing_height - world_px(24.0);
        self.throw_sequence = Some(ThrowSequence {
            attacker: attacker_slot,
            victim: victim.slot,
            phase: ThrowPhase::Capture,
            elapsed_seconds: 0.0,
            landing_x,
            start_x: victim.position.x,
            release_x: attacker.body_rect().center_x() + direction * world_px(15.0)
                - victim.body_rect().width * 0.5,
            start_y: victim.position.y,
            release_y,
        });
        attacker.begin_throw_capture(true);
        victim.begin_throw_capture(false);
    }

    pub(super) fn update_throw_sequence(&mut self, dt: f32) {
        let Some(mut sequence) = self.throw_sequence else {
            return;
        };
        let (attacker, victim) = match sequence.attacker {
            PlayerSlot::One => (&mut self.player_one, &mut self.player_two),
            PlayerSlot::Two => (&mut self.player_two, &mut self.player_one),
        };
        sequence.elapsed_seconds += dt;
        match sequence.phase {
            ThrowPhase::Capture => {
                let t = (sequence.elapsed_seconds / CAPTURE_SECONDS).min(1.0);
                let smooth = t * t * (3.0 - 2.0 * t);
                victim.position.x =
                    sequence.start_x + (sequence.release_x - sequence.start_x) * smooth;
                victim.position.y =
                    sequence.start_y + (sequence.release_y - sequence.start_y) * smooth;
                if t >= 1.0 - 0.0001 {
                    let flight_seconds = FLIGHT_FRAMES * FIXED_TIMESTEP;
                    let floor_position = FLOOR_Y - victim.body_rect().height;
                    // Semi-implicit Euler includes acceleration on the first
                    // step; this velocity lands after exactly forty fixed ticks.
                    let gravity_distance = GRAVITY
                        * FIXED_TIMESTEP.powi(2)
                        * FLIGHT_FRAMES
                        * (FLIGHT_FRAMES + 1.0)
                        * 0.5;
                    let velocity = Vec2::new(
                        (sequence.landing_x - victim.position.x) / flight_seconds,
                        (floor_position - victim.position.y - gravity_distance) / flight_seconds,
                    );
                    attacker.release_throw_capture();
                    victim.begin_launch(velocity, HitReactionKind::Thrown);
                    sequence.phase = ThrowPhase::Flight;
                    sequence.elapsed_seconds = 0.0;
                }
            }
            ThrowPhase::Flight if victim.grounded => {
                self.throw_sequence = None;
                return;
            }
            ThrowPhase::Flight => {}
        }
        self.throw_sequence = Some(sequence);
    }
}
