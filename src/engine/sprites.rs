//! Draws placeholder fighter sprites.
//!
//! The first spritesheet is intentionally tiny and state-driven. It makes
//! limbs and attacks readable before a real animation pipeline exists.

use raylib::prelude::*;

use crate::combat::fighter::{AttackKind, Facing, Fighter};

pub const FIGHTER_SPRITESHEET_PATH: &str = "assets/placeholder/fighter-greybox-spritesheet.png";

const FRAME_WIDTH: f32 = 96.0;
const FRAME_HEIGHT: f32 = 128.0;

/// Draws one fighter from the placeholder spritesheet.
pub fn draw_fighter_sprite(
    draw: &mut RaylibDrawHandle<'_>,
    texture: &Texture2D,
    fighter: &Fighter,
    tint: Color,
) {
    let frame = fighter_sprite_frame(fighter);
    let body = fighter.body_rect();
    let mut source = Rectangle::new(frame.index() * FRAME_WIDTH, 0.0, FRAME_WIDTH, FRAME_HEIGHT);

    if fighter.facing == Facing::Left {
        source.x += FRAME_WIDTH;
        source.width = -FRAME_WIDTH;
    }

    let dest = Rectangle::new(
        body.center_x() - FRAME_WIDTH * 0.5,
        body.bottom() - FRAME_HEIGHT,
        FRAME_WIDTH,
        FRAME_HEIGHT,
    );

    draw.draw_texture_pro(texture, source, dest, Vector2::new(0.0, 0.0), 0.0, tint);
}

/// Returns the placeholder sprite frame matching the current fighter state.
pub fn fighter_sprite_frame(fighter: &Fighter) -> FighterSpriteFrame {
    if fighter.blocking {
        return FighterSpriteFrame::Block;
    }

    if let Some(kind) = fighter.attack_kind() {
        return match kind {
            AttackKind::LightPunch => FighterSpriteFrame::LightPunch,
            AttackKind::HeavyPunch => FighterSpriteFrame::HeavyPunch,
            AttackKind::Kick => FighterSpriteFrame::Kick,
        };
    }

    if fighter.crouching {
        FighterSpriteFrame::Crouch
    } else if !fighter.grounded {
        FighterSpriteFrame::Jump
    } else if fighter.velocity.x.abs() > 8.0 {
        FighterSpriteFrame::Walk
    } else {
        FighterSpriteFrame::Idle
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
    const fn index(self) -> f32 {
        match self {
            FighterSpriteFrame::Idle => 0.0,
            FighterSpriteFrame::Walk => 1.0,
            FighterSpriteFrame::Crouch => 2.0,
            FighterSpriteFrame::Jump => 3.0,
            FighterSpriteFrame::Block => 4.0,
            FighterSpriteFrame::LightPunch => 5.0,
            FighterSpriteFrame::HeavyPunch => 6.0,
            FighterSpriteFrame::Kick => 7.0,
        }
    }
}
