//! Validates the data-driven audio manifest contract.

use std::path::Path;

use borrow_fighters::audio::{AudioBank, AudioCue, AudioEvent, AudioManifest, MusicTrack};
use borrow_fighters::characters::CharacterId;
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
