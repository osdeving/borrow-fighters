//! Verifies state-local sprite playback without changing combat sampling.
//!
//! System: Sprite runtime regression tests for reactions, stances, outcomes,
//! and compatibility with manifests that do not yet have dedicated artwork.

use borrow_fighters::{
    combat::{
        fighter::{Fighter, FighterInput, GuardRule, PlayerSlot},
        move_data::LIGHT_ATTACK_REACTION,
    },
    engine::sprites::{
        FighterSpriteClip, SpriteClip, SpriteManifest, fighter_clip_elapsed_seconds,
        fighter_sprite_clip, frame_for_fighter_clip_at, match_fighter_sprite_clip,
        projected_fighter_combat,
    },
    game::world::{MatchOutcome, World},
};

const DT: f32 = 1.0 / 60.0;

fn manifest_with_clips(names: &[&str]) -> SpriteManifest {
    let mut manifest = SpriteManifest::load("tests/fixtures/sprite-viewer-combat.sprite.json")
        .expect("test fixture should load");
    let template = manifest.frames[0].clone();
    manifest.frames.clear();
    manifest.clips.clear();
    for name in names {
        let mut frames = Vec::new();
        for index in 0..2 {
            let mut frame = template.clone();
            frame.name = format!("{name}_{index}");
            frame.clip = name.to_string();
            frame.duration_ms = 10;
            frame.combat.as_mut().unwrap().hurtboxes[0].x += index;
            frames.push(frame.name.clone());
            manifest.frames.push(frame);
        }
        manifest.clips.push(SpriteClip {
            name: name.to_string(),
            r#loop: false,
            frames,
        });
    }
    manifest
        .validate()
        .expect("playback fixture should validate");
    manifest
}

#[test]
fn hit_frames_advance_and_restart_without_switching_combat_metadata() {
    let manifest = manifest_with_clips(&["hit"]);
    let mut fighter = Fighter::new(PlayerSlot::One, "Rust", 320.0);
    fighter.take_hit(1, GuardRule::Mid, LIGHT_ATTACK_REACTION);
    let initial_boxes = projected_fighter_combat(&manifest, &fighter, 20.0).unwrap();

    fighter.update(DT, FighterInput::default());

    assert!(fighter.in_hitstun());
    let visual = frame_for_fighter_clip_at(
        &manifest,
        fighter_sprite_clip(&fighter),
        fighter_clip_elapsed_seconds(&fighter, 20.0),
    )
    .unwrap();
    assert_eq!(visual.name, "hit_1");
    assert_eq!(
        projected_fighter_combat(&manifest, &fighter, 20.0).unwrap(),
        initial_boxes
    );
    fighter.take_hit(1, GuardRule::Mid, LIGHT_ATTACK_REACTION);
    assert_eq!(fighter_clip_elapsed_seconds(&fighter, 20.0), 0.0);
}

#[test]
fn guard_and_blocked_impact_have_local_playback_clocks() {
    let mut fighter = Fighter::new(PlayerSlot::One, "Rust", 320.0);
    let guard = FighterInput {
        block: true,
        ..FighterInput::default()
    };
    fighter.update(DT, guard);
    assert_eq!(fighter_clip_elapsed_seconds(&fighter, 20.0), 0.0);
    fighter.update(DT, guard);
    assert_eq!(fighter_clip_elapsed_seconds(&fighter, 20.0), DT);
    fighter.update(
        DT,
        FighterInput {
            crouch: true,
            ..guard
        },
    );
    assert_eq!(
        fighter_sprite_clip(&fighter),
        FighterSpriteClip::CrouchBlock
    );
    assert_eq!(fighter_clip_elapsed_seconds(&fighter, 20.0), 0.0);
    fighter.update(DT, FighterInput::default());
    fighter.update(DT, guard);
    assert_eq!(fighter_clip_elapsed_seconds(&fighter, 20.0), 0.0);

    let result = fighter.take_hit(4, GuardRule::Mid, LIGHT_ATTACK_REACTION);
    assert!(result.blocked);
    assert_eq!(fighter_clip_elapsed_seconds(&fighter, 20.0), 0.0);
    fighter.update(DT, guard);
    assert!(fighter.in_blockstun());
    assert_eq!(fighter_clip_elapsed_seconds(&fighter, 20.0), DT);
}

#[test]
fn crouched_guard_uses_its_art_but_preserves_standing_guard_metadata() {
    let manifest = manifest_with_clips(&["block", "crouch_block"]);
    let mut fighter = Fighter::new(PlayerSlot::One, "Rust", 320.0);
    fighter.update(
        DT,
        FighterInput {
            crouch: true,
            block: true,
            ..FighterInput::default()
        },
    );
    assert_eq!(
        fighter_sprite_clip(&fighter),
        FighterSpriteClip::CrouchBlock
    );
    assert_eq!(
        projected_fighter_combat(&manifest, &fighter, 20.0)
            .unwrap()
            .frame_name,
        "block_1"
    );
    assert_eq!(
        frame_for_fighter_clip_at(&manifest, FighterSpriteClip::CrouchBlock, 0.0)
            .unwrap()
            .name,
        "crouch_block_0"
    );
}

#[test]
fn crouch_and_jump_play_from_state_entry_and_keep_legacy_box_sampling() {
    let manifest = manifest_with_clips(&["crouch", "jump"]);
    let mut fighter = Fighter::new(PlayerSlot::One, "Rust", 320.0);
    let crouch = FighterInput {
        crouch: true,
        ..FighterInput::default()
    };
    fighter.update(DT, crouch);
    assert_eq!(fighter_clip_elapsed_seconds(&fighter, 20.0), 0.0);
    assert_eq!(
        projected_fighter_combat(&manifest, &fighter, 20.0)
            .unwrap()
            .frame_name,
        "crouch_1"
    );
    fighter.update(DT, crouch);
    assert_eq!(fighter_clip_elapsed_seconds(&fighter, 20.0), DT);
    fighter.update(DT, FighterInput::default());
    fighter.update(DT, crouch);
    assert_eq!(fighter_clip_elapsed_seconds(&fighter, 20.0), 0.0);

    fighter.update(DT, FighterInput::default());
    fighter.update(
        DT,
        FighterInput {
            jump: true,
            ..FighterInput::default()
        },
    );
    assert!(!fighter.grounded);
    assert_eq!(fighter_clip_elapsed_seconds(&fighter, 20.0), 0.0);
    fighter.update(DT, FighterInput::default());
    assert_eq!(fighter_clip_elapsed_seconds(&fighter, 20.0), DT);
    assert_eq!(
        projected_fighter_combat(&manifest, &fighter, 20.0)
            .unwrap()
            .frame_name,
        "jump_0"
    );
}

#[test]
fn absent_action_clips_fall_back_visually_without_borrowing_attack_boxes() {
    let manifest = manifest_with_clips(&["idle", "kick", "punch_light", "punch_heavy", "block"]);
    for (requested, fallback) in [
        (FighterSpriteClip::Throw, "punch_light_0"),
        (FighterSpriteClip::AirPunch, "punch_light_0"),
        (FighterSpriteClip::Sweep, "kick_0"),
        (FighterSpriteClip::AirKick, "kick_0"),
        (FighterSpriteClip::Overhead, "punch_heavy_0"),
        (FighterSpriteClip::AntiAir, "punch_heavy_0"),
        (FighterSpriteClip::CrouchBlock, "block_0"),
    ] {
        assert_eq!(
            frame_for_fighter_clip_at(&manifest, requested, 0.0)
                .unwrap()
                .name,
            fallback
        );
    }
    let mut fighter = Fighter::new(PlayerSlot::One, "Rust", 320.0);
    fighter.update(
        DT,
        FighterInput {
            block: true,
            light_punch: true,
            ..FighterInput::default()
        },
    );
    assert_eq!(fighter_sprite_clip(&fighter), FighterSpriteClip::Throw);
    assert!(projected_fighter_combat(&manifest, &fighter, 20.0).is_none());
}

#[test]
fn outcomes_choose_victory_defeat_and_compatible_legacy_poses() {
    let outcome = Some(MatchOutcome::Winner(PlayerSlot::One));
    assert_eq!(
        match_fighter_sprite_clip(outcome, PlayerSlot::One, false),
        Some(FighterSpriteClip::Victory)
    );
    assert_eq!(
        match_fighter_sprite_clip(outcome, PlayerSlot::Two, false),
        Some(FighterSpriteClip::Defeat)
    );
    for slot in [PlayerSlot::One, PlayerSlot::Two] {
        assert_eq!(
            match_fighter_sprite_clip(Some(MatchOutcome::Draw), slot, false),
            Some(FighterSpriteClip::Defeat)
        );
    }
    for (names, clip, expected) in [
        (
            vec!["victory", "taunt", "idle"],
            FighterSpriteClip::Victory,
            "victory_0",
        ),
        (vec!["taunt", "idle"], FighterSpriteClip::Victory, "taunt_0"),
        (
            vec!["defeat", "hit", "idle"],
            FighterSpriteClip::Defeat,
            "defeat_0",
        ),
        (vec!["hit", "idle"], FighterSpriteClip::Defeat, "hit_0"),
        (vec!["idle"], FighterSpriteClip::Defeat, "idle_0"),
    ] {
        let manifest = manifest_with_clips(&names);
        assert_eq!(
            frame_for_fighter_clip_at(&manifest, clip, 0.0)
                .unwrap()
                .name,
            expected
        );
    }
}

#[test]
fn outcome_clock_starts_at_zero_and_continues_while_combat_is_frozen() {
    let mut world = World::new_greybox();
    world.elapsed_seconds = 35.0;
    world.player_two.health = 0;
    world.update(DT, FighterInput::default(), FighterInput::default());
    assert_eq!(world.outcome, Some(MatchOutcome::Winner(PlayerSlot::One)));
    assert_eq!(world.outcome_elapsed_seconds(), 0.0);
    let previous_position = world.player_one.position;
    world.update(
        DT,
        FighterInput {
            jump: true,
            ..FighterInput::default()
        },
        FighterInput::default(),
    );
    assert_eq!(world.player_one.position, previous_position);
    assert_eq!(world.outcome_elapsed_seconds(), DT);
    assert_eq!(World::new_greybox().outcome_elapsed_seconds(), 0.0);
}
