//! Exercises sprite clip selection from gameplay state.

use borrow_fighters::combat::fighter::{Fighter, FighterInput, PlayerSlot};
use borrow_fighters::engine::sprites::{
    FighterSpriteClip, fighter_clip_elapsed_seconds, fighter_sprite_clip,
};

const DT: f32 = 1.0 / 60.0;

#[test]
fn idle_fighter_uses_idle_clip() {
    let fighter = Fighter::new(PlayerSlot::One, "Rust", 320.0);

    assert_eq!(fighter_sprite_clip(&fighter), FighterSpriteClip::Idle);
    assert_eq!(FighterSpriteClip::Spawn.as_str(), "spawn");
    assert_eq!(FighterSpriteClip::Hit.as_str(), "hit");
}

#[test]
fn light_punch_uses_light_punch_clip_and_attack_time() {
    let mut fighter = Fighter::new(PlayerSlot::One, "Rust", 320.0);

    fighter.update(
        DT,
        FighterInput {
            light_punch: true,
            ..FighterInput::default()
        },
    );

    assert_eq!(fighter_sprite_clip(&fighter), FighterSpriteClip::PunchLight);
    assert_eq!(fighter_clip_elapsed_seconds(&fighter, 10.0), DT);
}

#[test]
fn crouch_clip_clamps_to_finished_crouch_pose() {
    let mut fighter = Fighter::new(PlayerSlot::One, "Rust", 320.0);

    fighter.update(
        DT,
        FighterInput {
            crouch: true,
            ..FighterInput::default()
        },
    );

    assert_eq!(fighter_sprite_clip(&fighter), FighterSpriteClip::Crouch);
    assert_eq!(fighter_clip_elapsed_seconds(&fighter, 10.0), 999.0);
}

#[test]
fn projectile_fire_uses_special_clip() {
    let mut fighter = Fighter::new(PlayerSlot::One, "Rust", 320.0);
    fighter.mark_projectile_fired();

    assert_eq!(fighter_sprite_clip(&fighter), FighterSpriteClip::Special);
    assert_eq!(fighter_clip_elapsed_seconds(&fighter, 10.0), 0.0);
}

#[test]
fn block_input_does_not_override_airborne_jump_clip() {
    let mut fighter = Fighter::new(PlayerSlot::One, "Rust", 320.0);

    fighter.update(
        DT,
        FighterInput {
            right: true,
            jump: true,
            ..FighterInput::default()
        },
    );
    fighter.update(
        DT,
        FighterInput {
            block: true,
            ..FighterInput::default()
        },
    );

    assert!(!fighter.blocking);
    assert_eq!(fighter_sprite_clip(&fighter), FighterSpriteClip::Jump);
}
