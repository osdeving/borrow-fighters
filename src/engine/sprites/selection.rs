//! Maps fighter gameplay state to sprite clips.
//!
//! Combat stays authoritative; this file only translates visible fighter state
//! into animation names.

use crate::{
    combat::fighter::{AttackKind, Fighter, PlayerSlot},
    game::world::MatchOutcome,
};

/// Visual animation clips expected by the current fighter sprite manifest.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FighterSpriteClip {
    Spawn,
    Idle,
    Walk,
    Crouch,
    Jump,
    Block,
    CrouchBlock,
    Hit,
    PunchLight,
    PunchHeavy,
    Kick,
    Sweep,
    Overhead,
    AntiAir,
    AirPunch,
    AirKick,
    Throw,
    Special,
    Taunt,
    Victory,
    Defeat,
}

impl FighterSpriteClip {
    /// Clips required before a candidate can replace a complete fighter atlas.
    ///
    /// Legacy atlases may still use visual aliases; partial production belongs
    /// in the Sprite Viewer until each implemented action has its own clip.
    pub const REQUIRED: [Self; 20] = [
        Self::Spawn,
        Self::Idle,
        Self::Walk,
        Self::Crouch,
        Self::Jump,
        Self::Block,
        Self::CrouchBlock,
        Self::Hit,
        Self::PunchLight,
        Self::PunchHeavy,
        Self::Kick,
        Self::Sweep,
        Self::Overhead,
        Self::AntiAir,
        Self::AirPunch,
        Self::AirKick,
        Self::Throw,
        Self::Special,
        Self::Victory,
        Self::Defeat,
    ];

    /// Returns the clip name used by sprite manifests.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Spawn => "spawn",
            Self::Idle => "idle",
            Self::Walk => "walk",
            Self::Crouch => "crouch",
            Self::Jump => "jump",
            Self::Block => "block",
            Self::CrouchBlock => "crouch_block",
            Self::Hit => "hit",
            Self::PunchLight => "punch_light",
            Self::PunchHeavy => "punch_heavy",
            Self::Kick => "kick",
            Self::Sweep => "sweep",
            Self::Overhead => "overhead",
            Self::AntiAir => "anti_air",
            Self::AirPunch => "air_punch",
            Self::AirKick => "air_kick",
            Self::Throw => "throw",
            Self::Special => "special",
            Self::Taunt => "taunt",
            Self::Victory => "victory",
            Self::Defeat => "defeat",
        }
    }
}

/// Selects non-interactive entrance and outcome clips for either player.
///
/// Entrance also works when `spawn` lives in the main fighter manifest.
pub fn match_fighter_sprite_clip(
    outcome: Option<MatchOutcome>,
    slot: PlayerSlot,
    spawn_intro: bool,
) -> Option<FighterSpriteClip> {
    if let Some(outcome) = outcome {
        return Some(match outcome {
            MatchOutcome::Winner(winner) if winner == slot => FighterSpriteClip::Victory,
            MatchOutcome::Winner(_) | MatchOutcome::Draw => FighterSpriteClip::Defeat,
        });
    }
    spawn_intro.then_some(FighterSpriteClip::Spawn)
}

/// Frames in `fighter-greybox-spritesheet.png`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FighterSpriteFrame {
    Idle,
    Walk,
    Crouch,
    Jump,
    Block,
    LightPunch,
    HeavyPunch,
    Kick,
}

impl FighterSpriteFrame {
    pub(crate) const fn index(self) -> f32 {
        match self {
            Self::Idle => 0.0,
            Self::Walk => 1.0,
            Self::Crouch => 2.0,
            Self::Jump => 3.0,
            Self::Block => 4.0,
            Self::LightPunch => 5.0,
            Self::HeavyPunch => 6.0,
            Self::Kick => 7.0,
        }
    }
}

/// Returns the sprite clip matching the current fighter state.
pub fn fighter_sprite_clip(fighter: &Fighter) -> FighterSpriteClip {
    if fighter.in_hitstun() {
        return FighterSpriteClip::Hit;
    }

    if fighter.blocking {
        return if fighter.crouching {
            FighterSpriteClip::CrouchBlock
        } else {
            FighterSpriteClip::Block
        };
    }

    if fighter.special_elapsed_seconds().is_some() {
        return FighterSpriteClip::Special;
    }

    if let Some(kind) = fighter.attack_kind() {
        return match kind {
            AttackKind::LightPunch => FighterSpriteClip::PunchLight,
            AttackKind::HeavyPunch => FighterSpriteClip::PunchHeavy,
            AttackKind::Kick => FighterSpriteClip::Kick,
            AttackKind::Sweep => FighterSpriteClip::Sweep,
            AttackKind::Overhead => FighterSpriteClip::Overhead,
            AttackKind::AntiAir => FighterSpriteClip::AntiAir,
            AttackKind::AirPunch => FighterSpriteClip::AirPunch,
            AttackKind::AirKick => FighterSpriteClip::AirKick,
            AttackKind::Throw => FighterSpriteClip::Throw,
        };
    }

    if fighter.crouching {
        FighterSpriteClip::Crouch
    } else if !fighter.grounded {
        FighterSpriteClip::Jump
    } else if fighter.velocity.x.abs() > 8.0 {
        FighterSpriteClip::Walk
    } else {
        FighterSpriteClip::Idle
    }
}

/// Returns elapsed clip time for the fighter's current visual state.
pub fn fighter_clip_elapsed_seconds(fighter: &Fighter, world_elapsed_seconds: f32) -> f32 {
    if fighter.in_hitstun() || fighter.in_blockstun() {
        return fighter.reaction_visual_elapsed_seconds();
    }
    if fighter.blocking {
        return fighter.guard_visual_elapsed_seconds();
    }
    if let Some(elapsed) = fighter.special_elapsed_seconds() {
        return elapsed;
    }
    if let Some(elapsed) = fighter.attack_elapsed_seconds() {
        return elapsed;
    }
    if fighter.crouching {
        return fighter.crouch_visual_elapsed_seconds();
    }
    if !fighter.grounded {
        return fighter.jump_visual_elapsed_seconds();
    }
    fighter_combat_clip_elapsed_seconds(fighter, world_elapsed_seconds)
}

/// Preserves the existing metadata clock independently from presentation fixes.
///
/// Advancing reaction, guard, crouch, or jump artwork must not silently switch
/// collision boxes. Recalibrating metadata timing requires a gameplay review.
pub(crate) fn fighter_combat_clip_elapsed_seconds(
    fighter: &Fighter,
    world_elapsed_seconds: f32,
) -> f32 {
    if fighter.in_hitstun() || fighter.in_blockstun() {
        return 0.0;
    }

    if let Some(elapsed) = fighter.special_elapsed_seconds() {
        return elapsed;
    }

    if let Some(elapsed) = fighter.attack_elapsed_seconds() {
        return elapsed;
    }

    if !fighter.grounded {
        if fighter.velocity.y < -80.0 {
            return 0.0;
        }
        if fighter.velocity.y < 140.0 {
            return 0.18;
        }
        return 0.36;
    }

    if fighter.crouching {
        return 999.0;
    }

    world_elapsed_seconds
}

/// Returns the placeholder sprite frame matching the current fighter state.
pub fn fighter_sprite_frame(fighter: &Fighter) -> FighterSpriteFrame {
    match fighter_sprite_clip(fighter) {
        FighterSpriteClip::Spawn => FighterSpriteFrame::Idle,
        FighterSpriteClip::Idle => FighterSpriteFrame::Idle,
        FighterSpriteClip::Walk => FighterSpriteFrame::Walk,
        FighterSpriteClip::Crouch => FighterSpriteFrame::Crouch,
        FighterSpriteClip::Jump => FighterSpriteFrame::Jump,
        FighterSpriteClip::Block => FighterSpriteFrame::Block,
        FighterSpriteClip::CrouchBlock => FighterSpriteFrame::Block,
        FighterSpriteClip::Hit => FighterSpriteFrame::Idle,
        FighterSpriteClip::PunchLight => FighterSpriteFrame::LightPunch,
        FighterSpriteClip::PunchHeavy => FighterSpriteFrame::HeavyPunch,
        FighterSpriteClip::Kick => FighterSpriteFrame::Kick,
        FighterSpriteClip::Sweep => FighterSpriteFrame::Kick,
        FighterSpriteClip::Overhead => FighterSpriteFrame::HeavyPunch,
        FighterSpriteClip::AntiAir => FighterSpriteFrame::HeavyPunch,
        FighterSpriteClip::AirPunch => FighterSpriteFrame::LightPunch,
        FighterSpriteClip::AirKick => FighterSpriteFrame::Kick,
        FighterSpriteClip::Throw => FighterSpriteFrame::LightPunch,
        FighterSpriteClip::Special => FighterSpriteFrame::Idle,
        FighterSpriteClip::Taunt => FighterSpriteFrame::Idle,
        FighterSpriteClip::Victory => FighterSpriteFrame::Idle,
        FighterSpriteClip::Defeat => FighterSpriteFrame::Idle,
    }
}
