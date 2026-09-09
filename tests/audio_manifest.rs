//! Validates the data-driven audio manifest contract.

use std::{collections::HashSet, path::Path};

use borrow_fighters::audio::{AudioBank, AudioCue, AudioEvent, AudioManifest, MusicTrack};
use borrow_fighters::characters::{CharacterId, character_spec};
use borrow_fighters::combat::fighter::PlayerSlot;
use borrow_fighters::combat::move_data::MoveId;

#[test]
fn repository_audio_manifest_uses_known_keys() {
    let manifest =
        AudioManifest::load("assets/audio/audio_manifest.json").expect("audio manifest loads");

    assert_eq!(manifest.version, 1);
    assert!(!manifest.clips.is_empty());
    assert!(!manifest.music.is_empty());
    assert!(!manifest.bindings.is_empty());

    for clip in &manifest.clips {
        assert!(
            Path::new(&clip.file).exists(),
            "audio clip {} points to missing file {}",
            clip.id,
            clip.file
        );
    }

    for track in &manifest.music {
        assert!(
            MusicTrack::from_key(&track.id).is_some(),
            "unknown music track key {}",
            track.id
        );
        assert!(
            Path::new(&track.file).exists(),
            "music track {} points to missing file {}",
            track.id,
            track.file
        );
    }

    let clip_ids = manifest
        .clips
        .iter()
        .map(|clip| clip.id.as_str())
        .collect::<HashSet<_>>();

    for binding in &manifest.bindings {
        assert!(
            AudioCue::from_key(&binding.cue).is_some(),
            "unknown audio cue {}",
            binding.cue
        );

        if let Some(character) = &binding.character {
            assert!(
                CharacterId::from_audio_key(character).is_some(),
                "unknown audio character key {character}"
            );
        }

        if let Some(move_key) = &binding.move_key {
            assert!(
                MoveId::from_audio_key(move_key).is_some(),
                "unknown audio move key {move_key}"
            );
        }

        assert!(
            !binding.clips.is_empty(),
            "binding {} must route to at least one clip",
            binding.cue
        );

        for clip_id in &binding.clips {
            assert!(
                clip_ids.contains(clip_id.as_str()),
                "binding {} references missing clip {}",
                binding.cue,
                clip_id
            );
        }
    }
}

#[test]
fn ui_audio_cues_have_stable_manifest_keys() {
    assert_eq!(AudioCue::UiNavigate.key(), "ui.navigate");
    assert_eq!(AudioCue::UiConfirm.key(), "ui.confirm");
    assert_eq!(AudioCue::UiBack.key(), "ui.back");
    assert_eq!(AudioCue::MatchCountdownEleven.key(), "match.countdown.11");
    assert_eq!(AudioCue::MatchCountdownTen.key(), "match.countdown.10");
    assert_eq!(AudioCue::MatchCountdownOne.key(), "match.countdown.01");
    assert_eq!(AudioCue::MatchCountdownFight.key(), "match.countdown.fight");
    assert_eq!(MusicTrack::Menu.key(), "menu");
    assert_eq!(MusicTrack::Combat.key(), "combat");
    assert_eq!(
        MusicTrack::CombatChiptuneBattle.key(),
        "combat-chiptune-battle"
    );
    assert_eq!(MusicTrack::CombatRinsTheme.key(), "combat-rins-theme");
    assert_eq!(MusicTrack::CombatEightBitBattle.key(), "combat-8bit-battle");
    assert_eq!(MusicTrack::CombatConsoleFloor.key(), "combat-console-floor");
    assert_eq!(
        MusicTrack::CombatRandomEncounter.key(),
        "combat-random-encounter"
    );
    assert_eq!(
        MusicTrack::CombatDeterminedPursuit.key(),
        "combat-determined-pursuit"
    );
}

#[test]
fn most_specific_audio_binding_wins() {
    let bank = AudioBank::new(
        AudioManifest::from_json(
            r#"
            {
              "clips": [
                { "id": "generic", "file": "generic.wav" },
                { "id": "rust-jab", "file": "rust-jab.wav" }
              ],
              "bindings": [
                {
                  "cue": "fighter.attack.start",
                  "clips": ["generic"]
                },
                {
                  "cue": "fighter.attack.start",
                  "character": "rust",
                  "move": "rust_borrow_jab",
                  "clips": ["rust-jab"]
                }
              ]
            }
            "#,
        )
        .expect("inline manifest parses"),
    );

    let (_, binding) = bank
        .binding_for_event(&AudioEvent::fighter_attack_start(
            PlayerSlot::One,
            CharacterId::Rust,
            MoveId::RustBorrowJab,
        ))
        .expect("specific binding exists");
    assert_eq!(binding.clips, ["rust-jab"]);

    let (_, binding) = bank
        .binding_for_event(&AudioEvent::fighter_attack_start(
            PlayerSlot::One,
            CharacterId::Duke,
            MoveId::Kick,
        ))
        .expect("generic binding exists");
    assert_eq!(binding.clips, ["generic"]);
}

#[test]
fn audio_bank_exposes_binding_clip_ids_by_index() {
    let bank = AudioBank::new(
        AudioManifest::from_json(
            r#"
            {
              "clips": [
                { "id": "generic", "file": "generic.wav" },
                { "id": "python-kick", "file": "python-kick.wav" }
              ],
              "bindings": [
                {
                  "cue": "fighter.attack.start",
                  "clips": ["generic"]
                },
                {
                  "cue": "fighter.attack.start",
                  "character": "python",
                  "move": "python_heel_kick",
                  "clips": ["python-kick"]
                }
              ]
            }
            "#,
        )
        .expect("inline manifest parses"),
    );

    let index = bank
        .binding_index_for_event(&AudioEvent::fighter_attack_start(
            PlayerSlot::Two,
            CharacterId::Python,
            MoveId::PythonHeelKick,
        ))
        .expect("specific binding index exists");

    let clip_ids = bank
        .binding_clip_ids(index)
        .expect("binding clips exist")
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>();
    assert_eq!(clip_ids, ["python-kick"]);
}

#[test]
fn every_character_attack_can_resolve_voice_binding() {
    let bank = AudioBank::new(
        AudioManifest::load("assets/audio/audio_manifest.json").expect("audio manifest loads"),
    );

    for character in [
        CharacterId::Rust,
        CharacterId::Duke,
        CharacterId::Go,
        CharacterId::C,
        CharacterId::Python,
        CharacterId::Cpp,
    ] {
        let spec = character_spec(character);
        for move_id in spec.move_ids {
            let event = AudioEvent::fighter_attack_start(PlayerSlot::One, character, *move_id);
            assert!(
                bank.binding_for_event(&event).is_some(),
                "{character:?} move {move_id:?} must resolve fighter.attack.start"
            );
        }

        let projectile = AudioEvent::fighter_projectile_cast(PlayerSlot::One, character);
        assert!(
            bank.binding_for_event(&projectile).is_some(),
            "{character:?} must resolve fighter.projectile.cast"
        );
    }
}

#[test]
fn rust_and_duke_close_attacks_use_move_specific_voice_bindings() {
    let bank = AudioBank::new(
        AudioManifest::load("assets/audio/audio_manifest.json").expect("audio manifest loads"),
    );

    for character in [CharacterId::Rust, CharacterId::Duke] {
        let spec = character_spec(character);
        for move_id in spec.move_ids {
            let event = AudioEvent::fighter_attack_start(PlayerSlot::One, character, *move_id);
            let (_, binding) = bank
                .binding_for_event(&event)
                .expect("attack voice binding exists");
            assert!(
                binding.move_key.is_some(),
                "{character:?} move {move_id:?} should not fall back to generic attack voice"
            );
        }
    }
}

#[test]
fn audio_clip_defaults_keep_placeholders_lightweight() {
    let manifest = AudioManifest::from_json(
        r#"
        {
          "clips": [{ "id": "clip", "file": "clip.ogg" }],
          "music": [{ "id": "menu", "file": "menu.ogg" }]
        }
        "#,
    )
    .expect("inline manifest parses");

    let clip = &manifest.clips[0];
    assert_eq!(clip.bus, "sfx");
    assert_eq!(clip.volume, 1.0);
    assert_eq!(clip.pitch, 1.0);
    assert_eq!(clip.pan, 0.5);
    assert!(!clip.required);

    let music = &manifest.music[0];
    assert_eq!(music.volume, 0.45);
    assert_eq!(music.pitch, 1.0);
    assert!(music.looping);
    assert!(!music.required);
}

#[test]
fn fighter_voices_only_share_recordings_for_the_authorized_old_c_rust_pair() {
    let manifest = AudioManifest::load("assets/audio/audio_manifest.json").unwrap();
    let bank = AudioBank::new(manifest.clone());
    let mut voice_files = Vec::new();
    for character in [
        CharacterId::Rust,
        CharacterId::Duke,
        CharacterId::Go,
        CharacterId::C,
        CharacterId::Python,
        CharacterId::Cpp,
    ] {
        let expected_directory =
            format!("assets/audio/characters/{}/voice/", character.audio_key());
        let mut ids = HashSet::new();
        for cue in [
            AudioCue::FighterAttackStart,
            AudioCue::FighterHurt,
            AudioCue::FighterBlock,
        ] {
            let event = AudioEvent::new(cue).with_fighter(PlayerSlot::One, character);
            let (_, binding) = bank.binding_for_event(&event).unwrap();
            ids.extend(binding.clips.iter().cloned());
        }
        for &move_id in character_spec(character).move_ids {
            let event = AudioEvent::fighter_attack_start(PlayerSlot::One, character, move_id);
            let (_, binding) = bank.binding_for_event(&event).unwrap();
            ids.extend(binding.clips.iter().cloned());
        }
        let projectile = AudioEvent::fighter_projectile_cast(PlayerSlot::One, character);
        let (_, binding) = bank.binding_for_event(&projectile).unwrap();
        ids.extend(binding.clips.iter().cloned());
        for id in ids {
            let clip = manifest.clips.iter().find(|clip| clip.id == id).unwrap();
            if clip.bus != "voice" {
                continue;
            }
            assert!(
                clip.file.starts_with(&expected_directory),
                "{character:?} uses {}",
                clip.file
            );
            let bytes = std::fs::read(&clip.file).unwrap();
            for (other, other_file, other_bytes) in &voice_files {
                let authorized_shared_voice = matches!(
                    (character, *other),
                    (CharacterId::C, CharacterId::Rust) | (CharacterId::Rust, CharacterId::C)
                );
                if *other != character && !authorized_shared_voice {
                    assert_ne!(
                        &bytes, other_bytes,
                        "{character:?} and {other:?} share the same recording: {} / {other_file}",
                        clip.file
                    );
                }
            }
            voice_files.push((character, clip.file.clone(), bytes));
        }
    }
}

#[test]
fn old_c_temporarily_reuses_the_selected_rust_clips_without_reprocessing() {
    let mappings = [
        ("attack-pointer-jab-01.ogg", "attack-borrow-jab-01.ogg"),
        ("attack-unsafe-poke-01.ogg", "attack-lifetime-01.ogg"),
        ("attack-heavy-02.ogg", "attack-heavy-01.ogg"),
        ("attack-reboot-02.ogg", "attack-kick-01.ogg"),
        ("hurt-01.ogg", "hurt-01.ogg"),
        ("block-01.ogg", "block-01.ogg"),
    ];
    for (old_c, rust) in mappings {
        assert_eq!(
            std::fs::read(format!("assets/audio/characters/c/voice/{old_c}")).unwrap(),
            std::fs::read(format!("assets/audio/characters/rust/voice/{rust}")).unwrap(),
            "Old C fallback {old_c} must preserve the selected Rust recording"
        );
    }
}

#[test]
fn authored_super_phases_resolve_existing_sounds_for_their_character() {
    let bank = AudioBank::new(AudioManifest::load("assets/audio/audio_manifest.json").unwrap());
    let phases = [
        (
            CharacterId::Rust,
            &[
                AudioCue::SuperStart,
                AudioCue::SuperMutation,
                AudioCue::SuperEnd,
            ][..],
        ),
        (
            CharacterId::Duke,
            &[
                AudioCue::SuperStart,
                AudioCue::SuperTrashRain,
                AudioCue::SuperCollect,
                AudioCue::SuperGiantDrop,
                AudioCue::SuperEnd,
            ][..],
        ),
        (
            CharacterId::C,
            &[
                AudioCue::SuperStart,
                AudioCue::SuperError,
                AudioCue::SuperBoot,
                AudioCue::SuperEnd,
            ][..],
        ),
        (
            CharacterId::Cpp,
            &[
                AudioCue::SuperStart,
                AudioCue::SuperTyping,
                AudioCue::SuperEnter,
                AudioCue::SuperFootshot,
                AudioCue::SuperBarrageHit,
                AudioCue::SuperError,
                AudioCue::SuperBoot,
                AudioCue::SuperEnd,
            ][..],
        ),
        (
            CharacterId::Python,
            &[
                AudioCue::SuperStart,
                AudioCue::SuperMutation,
                AudioCue::SuperLunge,
                AudioCue::SuperSwallow,
                AudioCue::SuperRevert,
                AudioCue::SuperCelebrate,
                AudioCue::SuperPeace,
                AudioCue::SuperEnd,
            ][..],
        ),
    ];
    for (character, cues) in phases {
        for &cue in cues {
            assert_eq!(AudioCue::from_key(cue.key()), Some(cue));
            let event = AudioEvent::new(cue).with_fighter(PlayerSlot::One, character);
            assert!(
                bank.binding_for_event(&event).is_some(),
                "{character:?} {cue:?} needs phase audio"
            );
        }
    }
}

#[test]
fn current_voice_sources_only_share_the_authorized_old_c_rust_recordings() {
    let mut current_outputs = std::collections::BTreeMap::new();
    for path in [
        "assets/audio/production-2026-09-09.json",
        "assets/audio/production-old-c-fallback-2026-09-09.json",
    ] {
        let production: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap();
        for output in production["outputs"].as_array().unwrap() {
            current_outputs.insert(output["file"].as_str().unwrap().to_owned(), output.clone());
        }
    }
    let mut source_owners = std::collections::HashMap::new();
    let mut characters = HashSet::new();
    for output in current_outputs.values() {
        let file = output["file"].as_str().unwrap();
        let Some(voice_path) = file.strip_prefix("assets/audio/characters/") else {
            continue;
        };
        let character = voice_path.split('/').next().unwrap();
        characters.insert(character);
        for source in output["sources"].as_array().unwrap() {
            let hash = source["source_sha256"].as_str().unwrap();
            assert_eq!(hash.len(), 64, "source hash missing for {file}");
            if let Some(other) = source_owners.insert(hash, character) {
                assert!(
                    other == character
                        || matches!((character, other), ("c", "rust") | ("rust", "c")),
                    "{character} and {other} reuse a source outside the authorized Old C/Rust pair"
                );
            }
        }
    }
    assert_eq!(characters, HashSet::from(["rust", "c", "go", "cpp"]));
}
