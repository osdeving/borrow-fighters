//! Generates lightweight CPU fighters for the greybox prototype.
//!
//! This is deterministic playtest behavior, not a general AI system. It exists
//! so one person can watch movement, attacks, blocking, jumping, and projectiles
//! without controlling both fighters.

use crate::combat::fighter::{Fighter, FighterInput, PlayerSlot};
use crate::game::world::World;
use crate::math::rect::Rect;

const BACK_AWAY_GAP: f32 = 48.0;
const LIGHT_ATTACK_GAP: f32 = 42.0;
const HEAVY_ATTACK_GAP: f32 = 82.0;
const FIREBALL_MIN_GAP: f32 = 145.0;
const FIREBALL_MAX_GAP: f32 = 295.0;
const PROJECTILE_GUARD_DISTANCE: f32 = 185.0;

/// Deterministic CPU controller for one fighter slot.
#[derive(Clone, Debug)]
pub struct BasicCpu {
    action_cooldown: f32,
    intent_timer: f32,
    guard_timer: f32,
    pattern_index: usize,
    rng_state: u32,
    intent: CpuIntent,
    profile: CpuProfile,
}

#[derive(Clone, Copy, Debug)]
struct CpuProfile {
    seed: u32,
    preferred_gap: f32,
    aggression: u32,
    projectile_bias: u32,
    jump_bias: u32,
    retreat_bias: u32,
    guard_reaction: u32,
    starting_action_cooldown: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CpuIntent {
    Hold,
    Approach,
    Retreat,
    Crouch,
    JumpToward,
    JumpAway,
}

impl Default for BasicCpu {
    fn default() -> Self {
        Self::for_slot(PlayerSlot::Two)
    }
}

impl BasicCpu {
    /// Creates a CPU profile tuned for a specific player slot.
    pub fn for_slot(slot: PlayerSlot) -> Self {
        let profile = CpuProfile::for_slot(slot);
        Self {
            action_cooldown: profile.starting_action_cooldown,
            intent_timer: 0.0,
            guard_timer: 0.0,
            pattern_index: 0,
            rng_state: profile.seed,
            intent: CpuIntent::Hold,
            profile,
        }
    }

    /// Returns CPU input for one fixed simulation step.
    pub fn next_input(&mut self, world: &World, slot: PlayerSlot, dt: f32) -> FighterInput {
        self.action_cooldown = (self.action_cooldown - dt).max(0.0);
        self.intent_timer = (self.intent_timer - dt).max(0.0);
        self.guard_timer = (self.guard_timer - dt).max(0.0);

        let (cpu, target) = fighters_for_slot(world, slot);
        if world.outcome.is_some() || cpu.is_defeated() {
            return FighterInput::default();
        }

        let gap = horizontal_gap(cpu.body_rect(), target.body_rect());
        let target_offset = target.body_rect().center_x() - cpu.body_rect().center_x();
        let mut input = FighterInput::default();

        if self.guard_timer > 0.0 {
            input.block = true;
            return input;
        }

        if incoming_projectile_threat(world, cpu) {
            self.react_to_projectile(&mut input, target_offset);
            return input;
        }

        if self.intent_timer <= 0.0 {
            self.choose_intent(gap);
        }
        apply_intent(&mut input, self.intent, target_offset);

        if self.action_cooldown <= 0.0 {
            self.choose_action(&mut input, gap);
        }

        input
    }

    /// Returns Player 2 input for one fixed simulation step.
    pub fn next_player_two_input(&mut self, world: &World, dt: f32) -> FighterInput {
        self.next_input(world, PlayerSlot::Two, dt)
    }

    fn react_to_projectile(&mut self, input: &mut FighterInput, target_offset: f32) {
        let roll = self.next_roll(100);
        if roll < self.profile.guard_reaction {
            self.guard_timer = self.random_duration(0.18, 0.34);
            input.block = true;
        } else if roll < self.profile.guard_reaction + self.profile.jump_bias {
            input.jump = true;
            if self.chance(55) {
                move_away(input, target_offset);
            }
        } else {
            move_away(input, target_offset);
        }
    }

    fn choose_intent(&mut self, gap: f32) {
        let roll = self.next_roll(100);
        self.intent = if gap < BACK_AWAY_GAP {
            if roll < self.profile.retreat_bias + 35 {
                CpuIntent::Retreat
            } else if roll < self.profile.retreat_bias + self.profile.jump_bias + 35 {
                CpuIntent::JumpAway
            } else {
                CpuIntent::Crouch
            }
        } else if gap > self.profile.preferred_gap + 46.0 {
            if roll < self.profile.jump_bias {
                CpuIntent::JumpToward
            } else if roll < 88 {
                CpuIntent::Approach
            } else {
                CpuIntent::Hold
            }
        } else if gap < self.profile.preferred_gap - 34.0 {
            if roll < self.profile.retreat_bias {
                CpuIntent::Retreat
            } else if roll < self.profile.retreat_bias + 18 {
                CpuIntent::JumpAway
            } else if roll < self.profile.retreat_bias + 34 {
                CpuIntent::Crouch
            } else {
                CpuIntent::Hold
            }
        } else if roll < self.profile.jump_bias {
            CpuIntent::JumpToward
        } else if roll < self.profile.jump_bias + self.profile.retreat_bias {
            CpuIntent::Retreat
        } else if roll
            < self.profile.jump_bias + self.profile.retreat_bias + self.profile.aggression
        {
            CpuIntent::Approach
        } else {
            CpuIntent::Hold
        };

        self.intent_timer = self.random_duration(0.22, 0.62);
    }

    fn choose_action(&mut self, input: &mut FighterInput, gap: f32) {
        let roll = (self.next_roll(100) + (self.pattern_index as u32 * 17)) % 100;
        if gap <= LIGHT_ATTACK_GAP && roll < self.profile.aggression + 32 {
            self.choose_close_attack(input, roll);
            let cooldown = self.random_duration(0.74, 1.18);
            self.advance_pattern(cooldown);
        } else if gap <= HEAVY_ATTACK_GAP {
            if roll < 32 {
                input.kick = true;
            } else if roll < self.profile.aggression + 28 {
                input.heavy_punch = true;
            } else if roll < self.profile.aggression + self.profile.jump_bias + 28 {
                input.jump = true;
            } else {
                return;
            };
            let cooldown = self.random_duration(0.88, 1.34);
            self.advance_pattern(cooldown);
        } else if (FIREBALL_MIN_GAP..=FIREBALL_MAX_GAP).contains(&gap)
            && roll < self.profile.projectile_bias
        {
            input.projectile = true;
            let cooldown = self.random_duration(1.10, 1.62);
            self.advance_pattern(cooldown);
        } else if gap > FIREBALL_MAX_GAP && roll < self.profile.jump_bias {
            input.jump = true;
            let cooldown = self.random_duration(0.72, 1.08);
            self.advance_pattern(cooldown);
        }
    }

    fn choose_close_attack(&mut self, input: &mut FighterInput, roll: u32) {
        match (self.pattern_index + roll as usize) % 4 {
            0 => input.light_punch = true,
            1 => input.kick = true,
            2 => input.heavy_punch = true,
            _ => {
                input.light_punch = true;
                if self.chance(30) {
                    input.jump = true;
                }
            }
        }
    }

    fn advance_pattern(&mut self, cooldown: f32) {
        self.pattern_index = self.pattern_index.wrapping_add(1);
        self.action_cooldown = cooldown;
    }

    fn next_roll(&mut self, upper_bound: u32) -> u32 {
        self.rng_state = self
            .rng_state
            .wrapping_mul(1_664_525)
            .wrapping_add(1_013_904_223);
        (self.rng_state >> 16) % upper_bound.max(1)
    }

    fn chance(&mut self, percent: u32) -> bool {
        self.next_roll(100) < percent
    }

    fn random_duration(&mut self, min: f32, max: f32) -> f32 {
        let t = self.next_roll(1000) as f32 / 999.0;
        min + (max - min) * t
    }
}

impl CpuProfile {
    fn for_slot(slot: PlayerSlot) -> Self {
        match slot {
            PlayerSlot::One => Self {
                seed: 0xB0F1_0001,
                preferred_gap: 165.0,
                aggression: 38,
                projectile_bias: 42,
                jump_bias: 18,
                retreat_bias: 28,
                guard_reaction: 58,
                starting_action_cooldown: 0.62,
            },
            PlayerSlot::Two => Self {
                seed: 0xD0C0_0002,
                preferred_gap: 105.0,
                aggression: 58,
                projectile_bias: 24,
                jump_bias: 24,
                retreat_bias: 16,
                guard_reaction: 46,
                starting_action_cooldown: 0.48,
            },
        }
    }
}

fn fighters_for_slot(world: &World, slot: PlayerSlot) -> (&Fighter, &Fighter) {
    match slot {
        PlayerSlot::One => (&world.player_one, &world.player_two),
        PlayerSlot::Two => (&world.player_two, &world.player_one),
    }
}

fn incoming_projectile_threat(world: &World, fighter: &Fighter) -> bool {
    let fighter_body = fighter.body_rect();
    let fighter_center = fighter_body.center_x();

    world.projectiles.iter().any(|projectile| {
        if projectile.owner == fighter.slot || !projectile.alive {
            return false;
        }

        let projectile_rect = projectile.rect();
        let projectile_center = projectile_rect.center_x();
        let moving_toward_fighter = projectile.velocity.x > 0.0
            && projectile_center < fighter_center
            || projectile.velocity.x < 0.0 && projectile_center > fighter_center;
        let close_enough = (fighter_center - projectile_center).abs() <= PROJECTILE_GUARD_DISTANCE;
        let vertical_threat = projectile_rect.y < fighter_body.bottom()
            && projectile_rect.bottom() > fighter_body.y + 8.0;

        moving_toward_fighter && close_enough && vertical_threat
    })
}

fn horizontal_gap(first: Rect, second: Rect) -> f32 {
    if first.center_x() <= second.center_x() {
        (second.x - first.right()).max(0.0)
    } else {
        (first.x - second.right()).max(0.0)
    }
}

fn move_toward(input: &mut FighterInput, target_offset: f32) {
    if target_offset < 0.0 {
        input.left = true;
    } else if target_offset > 0.0 {
        input.right = true;
    }
}

fn move_away(input: &mut FighterInput, target_offset: f32) {
    if target_offset < 0.0 {
        input.right = true;
    } else if target_offset > 0.0 {
        input.left = true;
    }
}

fn apply_intent(input: &mut FighterInput, intent: CpuIntent, target_offset: f32) {
    match intent {
        CpuIntent::Hold => {}
        CpuIntent::Approach => move_toward(input, target_offset),
        CpuIntent::Retreat => move_away(input, target_offset),
        CpuIntent::Crouch => input.crouch = true,
        CpuIntent::JumpToward => {
            input.jump = true;
            move_toward(input, target_offset);
        }
        CpuIntent::JumpAway => {
            input.jump = true;
            move_away(input, target_offset);
        }
    }
}
