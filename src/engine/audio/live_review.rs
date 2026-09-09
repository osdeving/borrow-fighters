//! Manually verifies stream pause, resume and cancellation against a real device.
//!
//! This ignored test can inspect private player resources without exposing audio
//! diagnostics in production. It records actual Raylib stream positions.

use std::time::{Duration, Instant};

use super::*;
use crate::{audio::AudioCue, characters::CharacterId, combat::fighter::PlayerSlot};

#[test]
#[ignore = "requires an audio device; run manually with --ignored --nocapture"]
fn live_music_pause_resume_and_cancel_preserve_stream_position() {
    let device = RaylibAudio::init_audio_device().expect("audio device must initialize");
    let mut player = AudioPlayer::load(&device, AUDIO_MANIFEST_PATH);
    assert!(player.enabled);
    let mut records = Vec::new();
    for (scene, track) in [
        ("fight", MusicTrack::Combat),
        ("combat_lab", MusicTrack::CombatDeterminedPursuit),
        ("move_showcase", MusicTrack::CombatDeterminedPursuit),
    ] {
        player.play_music(track);
        pump(&player, 350);
        let before_pause = position(&player);
        assert!(before_pause > 0.15, "{scene}: stream did not advance");
        player.set_cinematic_paused(true);
        let paused_at = position(&player);
        assert!(!playing(&player));
        player.play(
            &AudioEvent::new(AudioCue::SuperStart).with_fighter(PlayerSlot::One, CharacterId::C),
        );
        assert!(
            any_sound_playing(&player),
            "phase cue must play while music pauses"
        );
        pump(&player, 250);
        player.play_music(track);
        player.set_cinematic_paused(true);
        let during_pause = position(&player);
        assert!(
            !playing(&player),
            "{scene}: scene refresh restarted paused music"
        );
        assert!((during_pause - paused_at).abs() < 0.02);
        player.set_cinematic_paused(false);
        let resumed_at = position(&player);
        assert!((resumed_at - paused_at).abs() < 0.04);
        pump(&player, 300);
        let after_resume = position(&player);
        assert!(
            after_resume > resumed_at + 0.15,
            "{scene}: music did not resume"
        );
        player.set_cinematic_paused(true);
        let cancel_paused_at = position(&player);
        player.play(
            &AudioEvent::new(AudioCue::SuperStart).with_fighter(PlayerSlot::One, CharacterId::C),
        );
        assert!(any_sound_playing(&player));
        pump(&player, 60);
        player.cancel_cinematic();
        assert!(
            !any_sound_playing(&player),
            "{scene}: aborted cue still plays"
        );
        assert!(playing(&player));
        let cancel_resumed_at = position(&player);
        assert!((cancel_resumed_at - cancel_paused_at).abs() < 0.04);
        pump(&player, 250);
        let after_cancel = position(&player);
        assert!(after_cancel > cancel_resumed_at + 0.1);
        records.push(serde_json::json!({
            "scene": scene, "music": track.key(), "before_pause": before_pause,
            "paused_at": paused_at, "during_pause": during_pause,
            "resumed_at": resumed_at, "after_resume": after_resume,
            "cancel_paused_at": cancel_paused_at, "cancel_resumed_at": cancel_resumed_at,
            "after_cancel": after_cancel, "phase_cue_while_paused": true,
            "cancel_stops_phase_cue": true,
        }));
    }
    // A scene transition may choose its track before releasing an old sequence.
    player.set_cinematic_paused(true);
    player.play_music(MusicTrack::Menu);
    assert!(!playing(&player));
    let menu_paused_at = position(&player);
    pump(&player, 200);
    assert!((position(&player) - menu_paused_at).abs() < 0.02);
    player.cancel_cinematic();
    pump(&player, 250);
    let menu_after_cancel = position(&player);
    assert!(menu_after_cancel > menu_paused_at + 0.1);

    // Rust replaces Java Street's music during the super, then starts Sirius
    // from zero only when the sequence releases its music pause.
    player.play_music(MusicTrack::CombatConsoleFloor);
    pump(&player, 300);
    let arena_before_pause = position(&player);
    assert!(arena_before_pause > 0.15);
    player.set_cinematic_paused(true);
    player.play_music(MusicTrack::Combat);
    let sirius_paused_at = position(&player);
    assert!(sirius_paused_at < 0.04);
    assert!(!playing(&player));
    pump(&player, 250);
    player.play_music(MusicTrack::Combat);
    let sirius_during_pause = position(&player);
    assert!(!playing(&player));
    assert!((sirius_during_pause - sirius_paused_at).abs() < 0.02);
    player.set_cinematic_paused(false);
    let sirius_resumed_at = position(&player);
    assert!((sirius_resumed_at - sirius_paused_at).abs() < 0.04);
    pump(&player, 300);
    let sirius_after_resume = position(&player);
    assert!(sirius_after_resume > sirius_resumed_at + 0.15);
    player.play_music(MusicTrack::CombatConsoleFloor);
    let reset_at = position(&player);
    assert!(reset_at < 0.04);
    pump(&player, 250);
    let reset_after_resume = position(&player);
    assert!(reset_after_resume > reset_at + 0.1);
    let report = serde_json::json!({
        "kind": "live Raylib stream observation; same player calls used by three scenes",
        "scenes": records,
        "track_change_while_paused": {
            "music": "menu", "paused_at": menu_paused_at,
            "after_cancel": menu_after_cancel, "remained_paused_until_cancel": true
        },
        "arena_music_change_while_paused": {
            "previous_music": "combat-console-floor", "music": "combat",
            "previous_position": arena_before_pause,
            "paused_at": sirius_paused_at, "during_pause": sirius_during_pause,
            "resumed_at": sirius_resumed_at, "after_resume": sirius_after_resume,
            "started_from_zero_after_sequence": true,
            "reset_music": "combat-console-floor", "reset_at": reset_at,
            "reset_after_resume": reset_after_resume
        }
    });
    let text = serde_json::to_string_pretty(&report).unwrap();
    println!("{text}");
    if let Ok(path) = std::env::var("BORROW_AUDIO_REVIEW_OUTPUT") {
        let path = Path::new(&path);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).unwrap();
        }
        std::fs::write(path, format!("{text}\n")).unwrap();
    }
}

fn position(player: &AudioPlayer<'_>) -> f32 {
    player.music[player.current_music.as_deref().unwrap()]
        .music
        .get_time_played()
}

fn playing(player: &AudioPlayer<'_>) -> bool {
    player.music[player.current_music.as_deref().unwrap()]
        .music
        .is_stream_playing()
}

fn any_sound_playing(player: &AudioPlayer<'_>) -> bool {
    player
        .sounds
        .values()
        .any(|loaded| loaded.sound.is_playing())
}

fn pump(player: &AudioPlayer<'_>, milliseconds: u64) {
    let start = Instant::now();
    while start.elapsed() < Duration::from_millis(milliseconds) {
        player.update_streams();
        std::thread::sleep(Duration::from_millis(10));
    }
}
