//! Exercises sprite clip selection from gameplay state.

use borrow_fighters::combat::fighter::{Fighter, FighterInput, GuardRule, PlayerSlot};
use borrow_fighters::combat::move_data::LIGHT_ATTACK_REACTION;
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
fn grounded_attacks_use_dedicated_clips() {
    let cases = [
        (
            FighterInput {
                heavy_punch: true,
                ..FighterInput::default()
            },
            FighterSpriteClip::PunchHeavy,
            "punch_heavy",
        ),
        (
            FighterInput {
                kick: true,
                ..FighterInput::default()
            },
            FighterSpriteClip::Kick,
            "kick",
        ),
        (
            FighterInput {
                crouch: true,
                kick: true,
                ..FighterInput::default()
            },
            FighterSpriteClip::Sweep,
            "sweep",
        ),
        (
            FighterInput {
                right: true,
                heavy_punch: true,
                ..FighterInput::default()
            },
            FighterSpriteClip::Overhead,
            "overhead",
        ),
        (
            FighterInput {
                crouch: true,
                heavy_punch: true,
                ..FighterInput::default()
            },
            FighterSpriteClip::AntiAir,
            "anti_air",
        ),
        (
            FighterInput {
                block: true,
                light_punch: true,
                ..FighterInput::default()
            },
            FighterSpriteClip::Throw,
            "throw",
        ),
    ];

    for (input, expected_clip, expected_name) in cases {
        let mut fighter = Fighter::new(PlayerSlot::One, "Rust", 320.0);

        fighter.update(DT, input);

        assert_eq!(fighter_sprite_clip(&fighter), expected_clip);
        assert_eq!(expected_clip.as_str(), expected_name);
    }
}

#[test]
fn airborne_attacks_use_dedicated_air_clips() {
    let mut punch = airborne_fighter();
    punch.update(
        DT,
        FighterInput {
            light_punch: true,
            ..FighterInput::default()
        },
    );

    assert_eq!(fighter_sprite_clip(&punch), FighterSpriteClip::AirPunch);
    assert_eq!(FighterSpriteClip::AirPunch.as_str(), "air_punch");

    let mut kick = airborne_fighter();
    kick.update(
        DT,
        FighterInput {
            kick: true,
            ..FighterInput::default()
        },
    );

    assert_eq!(fighter_sprite_clip(&kick), FighterSpriteClip::AirKick);
    assert_eq!(FighterSpriteClip::AirKick.as_str(), "air_kick");
}

#[test]
fn crouch_clip_starts_at_the_first_frame() {
    let mut fighter = Fighter::new(PlayerSlot::One, "Rust", 320.0);

    fighter.update(
        DT,
        FighterInput {
            crouch: true,
            ..FighterInput::default()
        },
    );

    assert_eq!(fighter_sprite_clip(&fighter), FighterSpriteClip::Crouch);
    assert_eq!(fighter_clip_elapsed_seconds(&fighter, 10.0), 0.0);
}

#[test]
fn projectile_fire_uses_special_clip() {
    let mut fighter = Fighter::new(PlayerSlot::One, "Rust", 320.0);
    fighter.mark_projectile_fired();

    assert_eq!(fighter_sprite_clip(&fighter), FighterSpriteClip::Special);
    assert_eq!(fighter_clip_elapsed_seconds(&fighter, 10.0), 0.0);
}

#[test]
fn hitstun_uses_hit_clip_and_resets_clip_time() {
    let mut fighter = Fighter::new(PlayerSlot::One, "Rust", 320.0);

    fighter.take_hit(10, GuardRule::Mid, LIGHT_ATTACK_REACTION);

    assert_eq!(fighter_sprite_clip(&fighter), FighterSpriteClip::Hit);
    assert_eq!(fighter_clip_elapsed_seconds(&fighter, 10.0), 0.0);
}

#[test]
fn signature_attack_uses_its_own_clip_before_and_during_contact() {
    let mut fighter = borrow_fighters::game::world::World::new_greybox().player_one;
    fighter.update(
        DT,
        FighterInput {
            signature_special: true,
            ..FighterInput::default()
        },
    );
    assert_eq!(
        fighter_sprite_clip(&fighter),
        FighterSpriteClip::SignatureSpecial
    );
    for _ in 0..12 {
        fighter.update(DT, FighterInput::default());
    }
    assert!(fighter.active_attack().is_some());
    assert_eq!(
        fighter_sprite_clip(&fighter),
        FighterSpriteClip::SignatureSpecial
    );
}

#[test]
fn floor_recovery_animates_from_impact_and_returns_to_idle() {
    let mut fighter = Fighter::new(PlayerSlot::One, "Rust", 320.0);
    fighter.take_hit(10, GuardRule::Low, LIGHT_ATTACK_REACTION);
    fighter.start_knockdown();
    assert_eq!(fighter_sprite_clip(&fighter), FighterSpriteClip::Knockdown);
    assert_eq!(fighter_clip_elapsed_seconds(&fighter, 900.0), 0.0);
    for _ in 0..20 {
        fighter.update(DT, FighterInput::default());
    }
    assert_eq!(fighter_sprite_clip(&fighter), FighterSpriteClip::Knockdown);
    assert!((fighter_clip_elapsed_seconds(&fighter, 900.0) - 20.0 * DT).abs() < 0.0001);
    for _ in 0..20 {
        fighter.update(DT, FighterInput::default());
    }
    assert_eq!(fighter_sprite_clip(&fighter), FighterSpriteClip::Idle);
}

#[test]
fn low_blockstun_preserves_crouching_guard_art_until_recovery() {
    let mut fighter = Fighter::new(PlayerSlot::One, "Rust", 320.0);
    fighter.update(
        DT,
        FighterInput {
            block: true,
            crouch: true,
            ..FighterInput::default()
        },
    );
    fighter.take_hit(10, GuardRule::Low, LIGHT_ATTACK_REACTION);
    fighter.update(DT, FighterInput::default());
    assert!(fighter.in_blockstun());
    assert_eq!(
        fighter_sprite_clip(&fighter),
        FighterSpriteClip::CrouchBlock
    );
    assert!((fighter_clip_elapsed_seconds(&fighter, 900.0) - DT).abs() < 0.0001);
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

fn airborne_fighter() -> Fighter {
    let mut fighter = Fighter::new(PlayerSlot::One, "Rust", 320.0);
    fighter.grounded = false;
    fighter.position.y -= 92.0;
    fighter
}
