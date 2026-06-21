//! Maps fighter gameplay state to sprite clips.
//!
//! Combat stays authoritative; this file only translates visible fighter state
//! into animation names.

use crate::combat::fighter::{AttackKind, Fighter};

/// Visual animation clips expected by the current fighter sprite manifest.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FighterSpriteClip {
    Spawn,
    Idle,
    Walk,
    Crouch,
    Jump,
    Block,
    Hit,
    PunchLight,
    PunchHeavy,
    Kick,
    Special,
    Taunt,
}

impl FighterSpriteClip {
    /// Returns the clip name used by sprite manifests.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Spawn => "spawn",
            Self::Idle => "idle",
            Self::Walk => "walk",
            Self::Crouch => "crouch",
            Self::Jump => "jump",
            Self::Block => "block",
            Self::Hit => "hit",
            Self::PunchLight => "punch_light",
            Self::PunchHeavy => "punch_heavy",
            Self::Kick => "kick",
            Self::Special => "special",
            Self::Taunt => "taunt",
        }
    }
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
        return FighterSpriteClip::Block;
    }

    if fighter.special_elapsed_seconds().is_some() {
        return FighterSpriteClip::Special;
    }

    if let Some(kind) = fighter.attack_kind() {
        return match kind {
            AttackKind::LightPunch | AttackKind::AirPunch | AttackKind::Throw => {
                FighterSpriteClip::PunchLight
            }
            AttackKind::HeavyPunch | AttackKind::Overhead | AttackKind::AntiAir => {
                FighterSpriteClip::PunchHeavy
            }
            AttackKind::Kick | AttackKind::Sweep | AttackKind::AirKick => FighterSpriteClip::Kick,
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
        FighterSpriteClip::Hit => FighterSpriteFrame::Idle,
        FighterSpriteClip::PunchLight => FighterSpriteFrame::LightPunch,
        FighterSpriteClip::PunchHeavy => FighterSpriteFrame::HeavyPunch,
        FighterSpriteClip::Kick => FighterSpriteFrame::Kick,
        FighterSpriteClip::Special => FighterSpriteFrame::Idle,
        FighterSpriteClip::Taunt => FighterSpriteFrame::Idle,
    }
}
