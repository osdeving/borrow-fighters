//! Generates a minimal CPU opponent for the greybox prototype.
//!
//! This is a deterministic sparring controller, not a general AI system. It
//! exists so one person can test movement, attacks, blocking, and projectiles.

use crate::combat::fighter::{Fighter, FighterInput, PlayerSlot};
use crate::game::world::World;
use crate::math::rect::Rect;

const STARTING_ACTION_COOLDOWN: f32 = 0.45;
const BACK_AWAY_GAP: f32 = 48.0;
const LIGHT_ATTACK_GAP: f32 = 42.0;
const HEAVY_ATTACK_GAP: f32 = 82.0;
const FIREBALL_MIN_GAP: f32 = 145.0;
const FIREBALL_MAX_GAP: f32 = 260.0;
const APPROACH_GAP: f32 = 185.0;
const PROJECTILE_GUARD_DISTANCE: f32 = 170.0;

/// Deterministic CPU controller for one fighter slot.
#[derive(Clone, Debug)]
pub struct BasicCpu {
    action_cooldown: f32,
    pattern_index: usize,
}

impl Default for BasicCpu {
    fn default() -> Self {
        Self {
            action_cooldown: STARTING_ACTION_COOLDOWN,
            pattern_index: 0,
        }
    }
}

impl BasicCpu {
    /// Returns CPU input for one fixed simulation step.
    pub fn next_input(&mut self, world: &World, slot: PlayerSlot, dt: f32) -> FighterInput {
        self.action_cooldown = (self.action_cooldown - dt).max(0.0);

        let (cpu, target) = fighters_for_slot(world, slot);
        if world.outcome.is_some() || cpu.is_defeated() {
            return FighterInput::default();
        }

        if incoming_projectile_threat(world, cpu) {
            return FighterInput {
                block: true,
                ..FighterInput::default()
            };
        }

        let gap = horizontal_gap(cpu.body_rect(), target.body_rect());
        let target_offset = target.body_rect().center_x() - cpu.body_rect().center_x();
        let mut input = FighterInput::default();

        if gap < BACK_AWAY_GAP {
            move_away(&mut input, target_offset);
        } else if gap > APPROACH_GAP {
            move_toward(&mut input, target_offset);
        }

        if self.action_cooldown <= 0.0 {
            self.choose_action(&mut input, gap);
        }

        input
    }

    /// Returns Player 2 input for one fixed simulation step.
    pub fn next_player_two_input(&mut self, world: &World, dt: f32) -> FighterInput {
        self.next_input(world, PlayerSlot::Two, dt)
    }

    fn choose_action(&mut self, input: &mut FighterInput, gap: f32) {
        if gap <= LIGHT_ATTACK_GAP {
            match self.pattern_index % 3 {
                0 => input.light_punch = true,
                1 => input.kick = true,
                _ => input.heavy_punch = true,
            }
            self.advance_pattern(0.82);
        } else if gap <= HEAVY_ATTACK_GAP {
            if self.pattern_index.is_multiple_of(2) {
                input.heavy_punch = true;
            } else {
                input.kick = true;
            }
            self.advance_pattern(1.05);
        } else if (FIREBALL_MIN_GAP..=FIREBALL_MAX_GAP).contains(&gap) {
            input.projectile = true;
            self.advance_pattern(1.35);
        }
    }

    fn advance_pattern(&mut self, cooldown: f32) {
        self.pattern_index = self.pattern_index.wrapping_add(1);
        self.action_cooldown = cooldown;
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
