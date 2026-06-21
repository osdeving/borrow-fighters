//! Verifies whole-frame combat timing for close-range attacks.

use borrow_fighters::characters::{CharacterId, character_spec};
use borrow_fighters::combat::fighter::{
    AttackKind, AttackPhase, Fighter, FighterInput, PlayerSlot,
};
use borrow_fighters::combat::frame::FrameCount;
use borrow_fighters::combat::move_data::{MoveId, move_spec};
use borrow_fighters::combat::projectile::PROJECTILE_FRAME_DATA;

const DT: f32 = 1.0 / 60.0;

#[test]
fn close_attacks_expose_integer_frame_data() {
    let light = AttackKind::LightPunch.frame_data();
    assert_eq!(light.duration, FrameCount::new(18));
    assert_eq!(light.active_start, FrameCount::new(5));
    assert_eq!(light.active_end, FrameCount::new(10));

    let heavy = AttackKind::HeavyPunch.frame_data();
    assert_eq!(heavy.duration, FrameCount::new(35));
    assert_eq!(heavy.active_start, FrameCount::new(11));
    assert_eq!(heavy.active_end, FrameCount::new(20));

    let kick = AttackKind::Kick.frame_data();
    assert_eq!(kick.duration, FrameCount::new(28));
    assert_eq!(kick.active_start, FrameCount::new(9));
    assert_eq!(kick.active_end, FrameCount::new(16));
}

#[test]
fn projectile_special_exposes_integer_frame_data() {
    assert_eq!(PROJECTILE_FRAME_DATA.startup, FrameCount::ZERO);
    assert_eq!(PROJECTILE_FRAME_DATA.spawn_frame, FrameCount::ZERO);
    assert_eq!(PROJECTILE_FRAME_DATA.visual_duration, FrameCount::new(21));
    assert_eq!(PROJECTILE_FRAME_DATA.cooldown, FrameCount::new(57));
}

#[test]
fn projectile_special_uses_declared_frame_timers() {
    let mut fighter = Fighter::new(PlayerSlot::One, "Rust", 300.0);

    fighter.mark_projectile_fired();

    assert_eq!(fighter.special_elapsed_frames(), Some(FrameCount::ZERO));
    assert_eq!(
        fighter.projectile_cooldown_remaining_frames(),
        FrameCount::new(57)
    );
    assert!(!fighter.can_fire_projectile());

    for _ in 0..20 {
        fighter.update(DT, FighterInput::default());
    }

    assert_eq!(fighter.special_elapsed_frames(), Some(FrameCount::new(20)));
    assert_eq!(
        fighter.projectile_cooldown_remaining_frames(),
        FrameCount::new(37)
    );

    fighter.update(DT, FighterInput::default());
    assert_eq!(fighter.special_elapsed_frames(), None);

    for _ in 0..36 {
        fighter.update(DT, FighterInput::default());
    }

    assert_eq!(
        fighter.projectile_cooldown_remaining_frames(),
        FrameCount::ZERO
    );
    assert!(fighter.can_fire_projectile());
}

#[test]
fn frame_count_converts_elapsed_seconds_to_current_combat_frame() {
    assert_eq!(FrameCount::from_elapsed_seconds(0.0), FrameCount::ZERO);
    assert_eq!(
        FrameCount::from_elapsed_seconds(DT * 0.5),
        FrameCount::new(1)
    );
    assert_eq!(FrameCount::from_elapsed_seconds(DT), FrameCount::new(1));
    assert_eq!(
        FrameCount::from_elapsed_seconds(DT * 4.01),
        FrameCount::new(5)
    );
}

#[test]
fn light_punch_phases_follow_frame_data() {
    let mut fighter = start_attack(FighterInput {
        light_punch: true,
        ..FighterInput::default()
    });

    assert_eq!(fighter.attack_elapsed_frames(), Some(FrameCount::new(1)));
    assert_eq!(fighter.attack_phase(), AttackPhase::Startup);
    assert!(fighter.active_attack().is_none());

    advance_to_frame(&mut fighter, 5);
    assert_eq!(fighter.attack_phase(), AttackPhase::Active);
    assert!(fighter.active_attack().is_some());

    advance_to_frame(&mut fighter, 10);
    assert_eq!(fighter.attack_phase(), AttackPhase::Active);
    assert!(fighter.active_attack().is_some());

    advance_to_frame(&mut fighter, 11);
    assert_eq!(fighter.attack_phase(), AttackPhase::Recovery);
    assert!(fighter.active_attack().is_none());

    advance_to_frame(&mut fighter, 18);
    assert_eq!(fighter.attack_phase(), AttackPhase::Recovery);

    advance_until_attack_finishes(&mut fighter);
    assert_eq!(fighter.attack_elapsed_frames(), None);
    assert_eq!(fighter.attack_phase(), AttackPhase::WhiffRecovery);
    assert_eq!(
        fighter.whiff_recovery_remaining_frames(),
        move_spec(MoveId::LightPunch).whiff_recovery
    );

    for _ in 0..move_spec(MoveId::LightPunch).whiff_recovery.get() {
        fighter.update(DT, FighterInput::default());
    }

    assert_eq!(fighter.attack_phase(), AttackPhase::Idle);
}

#[test]
fn heavy_punch_and_kick_start_on_their_declared_active_frames() {
    let mut heavy = start_attack(FighterInput {
        heavy_punch: true,
        ..FighterInput::default()
    });
    advance_to_frame(&mut heavy, 10);
    assert_eq!(heavy.attack_phase(), AttackPhase::Startup);
    advance_to_frame(&mut heavy, 11);
    assert_eq!(heavy.attack_phase(), AttackPhase::Active);

    let mut kick = start_attack(FighterInput {
        kick: true,
        ..FighterInput::default()
    });
    advance_to_frame(&mut kick, 8);
    assert_eq!(kick.attack_phase(), AttackPhase::Startup);
    advance_to_frame(&mut kick, 9);
    assert_eq!(kick.attack_phase(), AttackPhase::Active);
}

#[test]
fn character_specific_moves_use_their_own_frame_data() {
    let mut rust = start_character_attack(
        CharacterId::Rust,
        FighterInput {
            light_punch: true,
            ..FighterInput::default()
        },
    );
    assert_eq!(
        rust.attack_frame_data(),
        Some(move_spec(MoveId::RustBorrowJab).frames)
    );
    advance_to_frame(&mut rust, 3);
    assert_eq!(rust.attack_phase(), AttackPhase::Startup);
    advance_to_frame(&mut rust, 4);
    assert_eq!(rust.attack_phase(), AttackPhase::Active);

    let mut duke = start_character_attack(
        CharacterId::Duke,
        FighterInput {
            heavy_punch: true,
            ..FighterInput::default()
        },
    );
    assert_eq!(
        duke.attack_frame_data(),
        Some(move_spec(MoveId::DukeBoilerplatePoke).frames)
    );
    advance_to_frame(&mut duke, 12);
    assert_eq!(duke.attack_phase(), AttackPhase::Startup);
    advance_to_frame(&mut duke, 13);
    assert_eq!(duke.attack_phase(), AttackPhase::Active);
}

fn start_attack(input: FighterInput) -> Fighter {
    let mut fighter = Fighter::new(PlayerSlot::One, "Rust", 300.0);
    fighter.update(DT, input);
    fighter
}

fn start_character_attack(character: CharacterId, input: FighterInput) -> Fighter {
    let spec = character_spec(character);
    let mut fighter = Fighter::new_with_loadout(
        PlayerSlot::One,
        spec.fighter_name,
        spec.stats.max_health,
        spec.move_ids,
        300.0,
    );
    fighter.update(DT, input);
    fighter
}

fn advance_to_frame(fighter: &mut Fighter, target_frame: u16) {
    while fighter
        .attack_elapsed_frames()
        .is_some_and(|frame| frame.get() < target_frame)
    {
        fighter.update(DT, FighterInput::default());
    }
}

fn advance_until_attack_finishes(fighter: &mut Fighter) {
    while fighter.attack_elapsed_frames().is_some() {
        fighter.update(DT, FighterInput::default());
    }
}
