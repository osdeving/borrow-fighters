//! Verifies exported visual candidates against their reviewed production frames.
//!
//! System: Asset regressions check complete clip coverage and pose phases at
//! actual combat ticks, without promoting candidate metadata into gameplay.

use std::{fs, path::Path};

use borrow_fighters::{
    characters::{CharacterId, character_spec},
    combat::{fighter::AttackKind, frame::FrameCount, move_data::move_spec},
    engine::sprites::{FighterSpriteClip, SpriteManifest, frame_for_fighter_clip_at},
};
use serde_json::Value;

struct Candidate {
    character: CharacterId,
    manifest: SpriteManifest,
    production: Value,
}

fn available_candidates() -> Vec<Candidate> {
    let mut candidates = Vec::new();
    for character in [
        CharacterId::Rust,
        CharacterId::Duke,
        CharacterId::Go,
        CharacterId::C,
        CharacterId::Python,
        CharacterId::Cpp,
    ] {
        let key = character.audio_key();
        let manifest_path = format!("assets/candidates/{key}/{key}-fighter.sprite.json");
        if character != CharacterId::Rust && !Path::new(&manifest_path).exists() {
            continue;
        }
        let manifest = SpriteManifest::load(&manifest_path)
            .unwrap_or_else(|error| panic!("{key}: candidate must load: {error}"));
        let production_path = format!("assets/production/{key}/production.json");
        let production: Value = serde_json::from_str(
            &fs::read_to_string(&production_path)
                .unwrap_or_else(|error| panic!("{key}: production source must load: {error}")),
        )
        .unwrap_or_else(|error| panic!("{key}: production JSON must parse: {error}"));
        assert_eq!(production["character"].as_str(), Some(key));
        candidates.push(Candidate {
            character,
            manifest,
            production,
        });
    }
    candidates
}

fn attack_clip(kind: AttackKind) -> FighterSpriteClip {
    match kind {
        AttackKind::LightPunch => FighterSpriteClip::PunchLight,
        AttackKind::HeavyPunch => FighterSpriteClip::PunchHeavy,
        AttackKind::Kick => FighterSpriteClip::Kick,
        AttackKind::Sweep => FighterSpriteClip::Sweep,
        AttackKind::Overhead => FighterSpriteClip::Overhead,
        AttackKind::AntiAir => FighterSpriteClip::AntiAir,
        AttackKind::AirPunch => FighterSpriteClip::AirPunch,
        AttackKind::AirKick => FighterSpriteClip::AirKick,
        AttackKind::Throw => FighterSpriteClip::Throw,
        AttackKind::SignatureSpecial | AttackKind::CinematicSpecial => {
            FighterSpriteClip::SignatureSpecial
        }
    }
}

#[test]
fn candidates_have_every_required_clip_and_preserve_visual_source_order() {
    for candidate in available_candidates() {
        let key = candidate.character.audio_key();
        for frame in &candidate.manifest.frames {
            assert!(
                frame.combat.is_none(),
                "{key}/{}: visual preparation must not introduce combat metadata",
                frame.name
            );
        }
        for required in FighterSpriteClip::required_for_character(candidate.character) {
            let clip_name = required.as_str();
            let clip = candidate
                .manifest
                .clip_named(clip_name)
                .unwrap_or_else(|| panic!("{key}: missing required clip {clip_name}"));
            let source = &candidate.production["actions"][clip_name];
            let source_frames = source["frames"]
                .as_array()
                .unwrap_or_else(|| panic!("{key}/{clip_name}: missing reviewed frames"));
            let source_names: Vec<_> = source_frames
                .iter()
                .map(|frame| frame["name"].as_str().expect("source frame needs a name"))
                .collect();
            assert_eq!(clip.frames, source_names, "{key}/{clip_name}: frame order");
            assert_eq!(source["loop"].as_bool(), Some(clip.r#loop));
            for source_frame in source_frames {
                let name = source_frame["name"].as_str().unwrap();
                let frame = candidate.manifest.frame_named(name).unwrap();
                assert_eq!(
                    source_frame["duration_ms"].as_u64(),
                    Some(u64::from(frame.duration_ms)),
                    "{key}/{name}: exported duration differs from reviewed source"
                );
                assert!(
                    source_frame.get("combat").is_none(),
                    "{key}/{name}: visual preparation cannot transform combat metadata"
                );
            }
        }
    }
}

#[test]
fn candidate_attack_poses_match_every_combat_tick_including_active_boundaries() {
    for candidate in available_candidates() {
        let key = candidate.character.audio_key();
        for &move_id in character_spec(candidate.character).move_ids {
            // Cinematic moves reuse phase-retimed actor clips; their presentation
            // contract is covered separately in cinematic_specials.rs.
            if AttackKind::from_move_id(move_id) == AttackKind::CinematicSpecial {
                continue;
            }
            let timing = move_spec(move_id).frames;
            let requested_clip = attack_clip(AttackKind::from_move_id(move_id));
            let clip_name = requested_clip.as_str();
            let clip = candidate.manifest.clip_named(clip_name).unwrap();
            assert!(!clip.r#loop, "{key}/{clip_name}: an attack must finish");
            let source_frames = candidate.production["actions"][clip_name]["frames"]
                .as_array()
                .unwrap();
            let duration_ms: u32 = clip
                .frames
                .iter()
                .map(|name| candidate.manifest.frame_named(name).unwrap().duration_ms)
                .sum();
            let combat_duration_ms = timing.duration.as_seconds() * 1000.0;
            assert!(
                (duration_ms as f32 - combat_duration_ms).abs() <= 1.0,
                "{key}/{move_id:?}: visual {duration_ms} ms vs combat {combat_duration_ms} ms"
            );

            // Use runtime seconds, not rounded integer milliseconds: rounding a
            // 4-tick startup to 67 ms would hide contact at Borrow Jab's first active tick.
            for tick in 0..=timing.duration.get() {
                let frame = frame_for_fighter_clip_at(
                    &candidate.manifest,
                    requested_clip,
                    FrameCount::new(tick).as_seconds(),
                )
                .unwrap();
                let source = source_frames
                    .iter()
                    .find(|source| source["name"].as_str() == Some(frame.name.as_str()))
                    .unwrap_or_else(|| panic!("{key}/{clip_name}: {} has no source", frame.name));
                // A landed throw freezes its contact clock while its authored
                // capture keys continue through tick21. The same keys play on a
                // whiff, but they create no extra active collision frames.
                let capture_pose = requested_clip == FighterSpriteClip::Throw
                    && candidate.character != CharacterId::Go
                    && (10..22).contains(&tick);
                if capture_pose {
                    assert!(
                        matches!(source["phase"].as_str(), Some("capture" | "active")),
                        "{key}/{move_id:?}: capture tick {tick} samples {}",
                        frame.name
                    );
                    continue;
                }
                let expected_phase = if tick < timing.active_start.get() {
                    "startup"
                } else if tick <= timing.active_end.get() {
                    "active"
                } else {
                    "recovery"
                };
                assert_eq!(
                    source["phase"].as_str(),
                    Some(expected_phase),
                    "{key}/{move_id:?}: tick {tick} samples {}",
                    frame.name
                );
            }
        }
    }
}
