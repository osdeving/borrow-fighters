//! Exercises placeholder sprite frame selection without opening a Raylib window.

use borrow_fighters::combat::fighter::{Fighter, FighterInput, PlayerSlot};
use borrow_fighters::engine::sprites::{FighterSpriteFrame, fighter_sprite_frame};

const DT: f32 = 1.0 / 60.0;

#[test]
fn idle_fighter_uses_idle_sprite_frame() {
    let fighter = Fighter::new(PlayerSlot::One, "Rust", 320.0);

    assert_eq!(fighter_sprite_frame(&fighter), FighterSpriteFrame::Idle);
}

#[test]
fn crouching_fighter_uses_crouch_sprite_frame() {
    let mut fighter = Fighter::new(PlayerSlot::One, "Rust", 320.0);

    fighter.update(
        DT,
        FighterInput {
            crouch: true,
            ..FighterInput::default()
        },
    );

    assert_eq!(fighter_sprite_frame(&fighter), FighterSpriteFrame::Crouch);
}

#[test]
fn jumping_fighter_uses_jump_sprite_frame() {
    let mut fighter = Fighter::new(PlayerSlot::One, "Rust", 320.0);

    fighter.update(
        DT,
        FighterInput {
            jump: true,
            ..FighterInput::default()
        },
    );

    assert_eq!(fighter_sprite_frame(&fighter), FighterSpriteFrame::Jump);
}

#[test]
fn light_punch_uses_light_punch_sprite_frame() {
    let mut fighter = Fighter::new(PlayerSlot::One, "Rust", 320.0);

    fighter.update(
        DT,
        FighterInput {
            light_punch: true,
            ..FighterInput::default()
        },
    );

    assert_eq!(
        fighter_sprite_frame(&fighter),
        FighterSpriteFrame::LightPunch
    );
}
