//! Calculates Combat Lab frame advantage and contact spacing.
//!
//! System: Combat Lab scene. This module turns existing combat data into
//! inspection numbers without drawing UI or advancing the live lab playback.

use crate::characters::{CharacterId, character_spec};
use crate::combat::{
    fighter::{Fighter, FighterInput, PlayerSlot},
    frame::FrameCount,
    move_data::{HitReaction, MoveInputKind, move_spec_for_input},
    projectile::Projectile,
};
use crate::config::{FIXED_TIMESTEP, FLOOR_Y};
use crate::math::rect::Rect;

const CONTACT_OVERLAP: f32 = 1.0;
const DEFAULT_DUMMY_X: f32 = 690.0;
const DEFAULT_DUMMY_WIDTH: f32 = 76.0;
const DEFAULT_DUMMY_HEIGHT: f32 = 168.0;
const AIR_ATTACK_PREVIEW_HEIGHT: f32 = 92.0;

type ContactAnalysis = (
    FrameCount,
    FrameCount,
    FrameCount,
    FrameCount,
    HitReaction,
    Rect,
);

/// Attack source selected by the Combat Lab for analysis.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CombatLabAnalysisMove {
    Close(MoveInputKind),
    Projectile,
}

/// Estimated frame advantage and spacing for a lab move.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CombatLabAdvantage {
    pub contact_frame: FrameCount,
    pub attacker_recovery_after_contact: FrameCount,
    pub whiff_recovery: FrameCount,
    pub projectile_cooldown_after_contact: FrameCount,
    pub hitstun: FrameCount,
    pub blockstun: FrameCount,
    pub hit_advantage: i16,
    pub block_advantage: i16,
    pub hit_pushback: f32,
    pub block_pushback: f32,
    pub body_gap_on_contact: f32,
    pub hit_body_gap_after_pushback: f32,
    pub block_body_gap_after_pushback: f32,
}

/// Calculates estimated frame advantage and post-pushback spacing.
pub fn analyze_advantage(
    character: CharacterId,
    attacker: &Fighter,
    selected_move: CombatLabAnalysisMove,
) -> Option<CombatLabAdvantage> {
    let (contact_frame, recovery, whiff_recovery, cooldown, reaction, contact_box) =
        contact_analysis(attacker.clone(), selected_move)?;
    let dummy = dummy_body_for_contact_box(character, contact_box);
    let gap = horizontal_body_gap(attacker.body_rect(), dummy);

    Some(CombatLabAdvantage {
        contact_frame,
        attacker_recovery_after_contact: recovery,
        whiff_recovery,
        projectile_cooldown_after_contact: cooldown,
        hitstun: reaction.hitstun,
        blockstun: reaction.blockstun,
        hit_advantage: signed_frame_delta(reaction.hitstun, recovery),
        block_advantage: signed_frame_delta(reaction.blockstun, recovery),
        hit_pushback: reaction.hit_pushback,
        block_pushback: reaction.block_pushback,
        body_gap_on_contact: gap,
        hit_body_gap_after_pushback: gap + reaction.hit_pushback,
        block_body_gap_after_pushback: gap + reaction.block_pushback,
    })
}

/// Returns the dummy body placed where the selected move would first connect.
pub fn contact_dummy_body(
    character: CharacterId,
    attacker: &Fighter,
    selected_move: CombatLabAnalysisMove,
) -> Option<Rect> {
    let (_, _, _, _, _, contact_box) = contact_analysis(attacker.clone(), selected_move)?;

    Some(dummy_body_for_contact_box(character, contact_box))
}

fn dummy_body_for_contact_box(character: CharacterId, contact_box: Rect) -> Rect {
    let defender = dummy_fighter_for(character);
    let defender_body = defender.body_rect();
    let defender_hurtbox = defender.hurtbox();
    let hurtbox_left_offset = defender_hurtbox.x - defender_body.x;
    let x = contact_box.right() - hurtbox_left_offset - CONTACT_OVERLAP;

    Rect::new(
        x,
        defender_body.y,
        defender_body.width,
        defender_body.height,
    )
}

/// Returns the fallback dummy body used outside move playback.
pub fn default_dummy_body() -> Rect {
    Rect::new(
        DEFAULT_DUMMY_X,
        FLOOR_Y - DEFAULT_DUMMY_HEIGHT,
        DEFAULT_DUMMY_WIDTH,
        DEFAULT_DUMMY_HEIGHT,
    )
}

fn contact_analysis(
    attacker: Fighter,
    selected_move: CombatLabAnalysisMove,
) -> Option<ContactAnalysis> {
    match selected_move {
        CombatLabAnalysisMove::Close(input) => {
            let spec = move_spec_for_input(attacker.move_ids(), input)?;
            let contact_frame = spec.frames.active_start;
            let recovery = frames_between(contact_frame, spec.frames.duration);
            let contact_box = close_attack_contact_box(attacker, input, contact_frame)?;

            Some((
                contact_frame,
                recovery,
                spec.whiff_recovery,
                FrameCount::ZERO,
                spec.hit_reaction,
                contact_box,
            ))
        }
        CombatLabAnalysisMove::Projectile => {
            let projectile_spec = attacker.projectile_spec();
            let frame_data = projectile_spec.frame_data;
            let projectile = Projectile::from_fighter(&attacker);

            Some((
                frame_data.spawn_frame,
                FrameCount::ZERO,
                FrameCount::ZERO,
                frames_between(frame_data.spawn_frame, frame_data.cooldown),
                projectile_spec.hit_reaction,
                projectile.rect(),
            ))
        }
    }
}

fn close_attack_contact_box(
    mut attacker: Fighter,
    input: MoveInputKind,
    contact_frame: FrameCount,
) -> Option<Rect> {
    prepare_attacker_for_close_move(&mut attacker, input);

    for frame in 0..contact_frame.get() {
        attacker.update(
            FIXED_TIMESTEP,
            if frame == 0 {
                input_for_close_move(input)
            } else {
                FighterInput::default()
            },
        );
    }

    attacker.attack_box()
}

fn prepare_attacker_for_close_move(attacker: &mut Fighter, input: MoveInputKind) {
    if matches!(input, MoveInputKind::AirPunch | MoveInputKind::AirKick) {
        attacker.grounded = false;
        attacker.position.y -= AIR_ATTACK_PREVIEW_HEIGHT;
    }
}

fn input_for_close_move(input: MoveInputKind) -> FighterInput {
    match input {
        MoveInputKind::LightPunch => FighterInput {
            light_punch: true,
            ..FighterInput::default()
        },
        MoveInputKind::HeavyPunch => FighterInput {
            heavy_punch: true,
            ..FighterInput::default()
        },
        MoveInputKind::Kick => FighterInput {
            kick: true,
            ..FighterInput::default()
        },
        MoveInputKind::Sweep => FighterInput {
            crouch: true,
            kick: true,
            ..FighterInput::default()
        },
        MoveInputKind::Overhead => FighterInput {
            right: true,
            heavy_punch: true,
            ..FighterInput::default()
        },
        MoveInputKind::AntiAir => FighterInput {
            crouch: true,
            heavy_punch: true,
            ..FighterInput::default()
        },
        MoveInputKind::AirPunch => FighterInput {
            light_punch: true,
            ..FighterInput::default()
        },
        MoveInputKind::AirKick => FighterInput {
            kick: true,
            ..FighterInput::default()
        },
        MoveInputKind::Throw => FighterInput {
            block: true,
            light_punch: true,
            ..FighterInput::default()
        },
    }
}

fn dummy_fighter_for(attacker_character: CharacterId) -> Fighter {
    let character = match attacker_character {
        CharacterId::Rust => CharacterId::Duke,
        CharacterId::Duke => CharacterId::Rust,
        CharacterId::Go => CharacterId::Duke,
    };
    let spec = character_spec(character);
    Fighter::new_with_projectile_loadout(
        PlayerSlot::Two,
        spec.fighter_name,
        spec.stats.max_health,
        spec.move_ids,
        spec.projectile,
        DEFAULT_DUMMY_X,
    )
}

fn frames_between(start: FrameCount, end: FrameCount) -> FrameCount {
    FrameCount::new(end.get().saturating_sub(start.get()))
}

fn signed_frame_delta(stun: FrameCount, recovery: FrameCount) -> i16 {
    stun.get() as i16 - recovery.get() as i16
}

fn horizontal_body_gap(attacker: Rect, defender: Rect) -> f32 {
    if attacker.center_x() <= defender.center_x() {
        defender.x - attacker.right()
    } else {
        attacker.x - defender.right()
    }
}
