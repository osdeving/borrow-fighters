//! Plays manifest-routed audio events and music through Raylib.
//!
//! System: Raylib audio boundary. This module owns loaded `Sound` and `Music`
//! resources and maps pure gameplay audio events to raylib playback calls.

use std::collections::HashMap;

use raylib::prelude::*;

use crate::audio::{
    AudioBank, AudioClipDefinition, AudioCue, AudioEvent, AudioMusicDefinition, MusicTrack,
};
use crate::runtime_paths::asset_path;

#[cfg(test)]
mod live_review;

pub const AUDIO_MANIFEST_PATH: &str = "assets/audio/audio_manifest.json";

/// Raylib-backed audio event player.
pub struct AudioPlayer<'aud> {
    bank: AudioBank,
    sounds: HashMap<String, LoadedSound<'aud>>,
    music: HashMap<String, LoadedMusic<'aud>>,
    binding_cursors: Vec<usize>,
    current_music: Option<String>,
    music_ducked: bool,
    cinematic_paused: bool,
    match_paused: bool,
    paused_sounds: Vec<String>,
    resume_music_after_pause: bool,
    music_volume: f32,
    enabled: bool,
}

struct LoadedSound<'aud> {
    sound: Sound<'aud>,
    volume: f32,
    pitch: f32,
    pan: f32,
}

struct LoadedMusic<'aud> {
    music: Music<'aud>,
    volume: f32,
    pitch: f32,
}

impl<'aud> AudioPlayer<'aud> {
    /// Creates a disabled audio player used when the device cannot initialize.
    pub fn disabled() -> Self {
        Self {
            bank: AudioBank::default(),
            sounds: HashMap::new(),
            music: HashMap::new(),
            binding_cursors: Vec::new(),
            current_music: None,
            music_ducked: false,
            cinematic_paused: false,
            match_paused: false,
            paused_sounds: Vec::new(),
            resume_music_after_pause: false,
            music_volume: 1.0,
            enabled: false,
        }
    }

    /// Loads audio manifest and optional sound files.
    pub fn load(audio: &'aud RaylibAudio, manifest_path: &str) -> Self {
        let bank = match AudioBank::load(asset_path(manifest_path)) {
            Ok(bank) => bank,
            Err(error) => {
                eprintln!("warning: audio disabled: {error}");
                return Self::disabled();
            }
        };

        let mut sounds = HashMap::new();
        for clip in bank.clips() {
            let clip_path = asset_path(&clip.file);
            if !clip_path.exists() {
                if clip.required {
                    eprintln!(
                        "warning: required audio clip {} is missing at {}",
                        clip.id, clip.file
                    );
                }
                continue;
            }

            match audio.new_sound(&clip_path.to_string_lossy()) {
                Ok(sound) => {
                    sounds.insert(clip.id.clone(), LoadedSound::new(sound, clip));
                }
                Err(error) => {
                    eprintln!(
                        "warning: could not load audio clip {} from {}: {:?}",
                        clip.id, clip.file, error
                    );
                }
            }
        }

        let mut music = HashMap::new();
        for track in bank.music_tracks() {
            let track_path = asset_path(&track.file);
            if !track_path.exists() {
                if track.required {
                    eprintln!(
                        "warning: required music track {} is missing at {}",
                        track.id, track.file
                    );
                }
                continue;
            }

            match audio.new_music(&track_path.to_string_lossy()) {
                Ok(mut loaded) => {
                    loaded.set_looping(track.looping);
                    music.insert(track.id.clone(), LoadedMusic::new(loaded, track));
                }
                Err(error) => {
                    eprintln!(
                        "warning: could not load music track {} from {}: {:?}",
                        track.id, track.file, error
                    );
                }
            }
        }

        let binding_count = bank.bindings().len();
        Self {
            bank,
            sounds,
            music,
            binding_cursors: vec![0; binding_count],
            current_music: None,
            music_ducked: false,
            cinematic_paused: false,
            match_paused: false,
            paused_sounds: Vec::new(),
            resume_music_after_pause: false,
            music_volume: 1.0,
            enabled: true,
        }
    }

    /// Updates the active music stream.
    pub fn update_streams(&self) {
        if !self.enabled || self.music_paused() {
            return;
        }

        if let Some(current) = self
            .current_music
            .as_ref()
            .and_then(|id| self.music.get(id))
        {
            current.music.update_stream();
        }
    }

    /// Starts or switches the active background music track.
    pub fn play_music(&mut self, track: MusicTrack) {
        if !self.enabled {
            return;
        }

        let next_id = track.key();
        if keeps_current_music(
            self.current_music.as_deref(),
            next_id,
            self.music_paused(),
            self.music
                .get(next_id)
                .is_some_and(|loaded| loaded.music.is_stream_playing()),
        ) {
            return;
        }

        if let Some(current_id) = self.current_music.take()
            && let Some(current) = self.music.get(&current_id)
        {
            current.music.stop_stream();
        }
        self.resume_music_after_pause = false;

        let Some(next) = self.music.get(next_id) else {
            return;
        };
        let volume = music_output_volume(next.volume, self.music_ducked, self.music_volume);
        // A scene can select a different track while paused. Prime it silently,
        // then leave it paused until the sequence or menu transition releases it.
        next.music
            .set_volume(if self.music_paused() { 0.0 } else { volume });
        next.music.set_pitch(next.pitch);
        next.music.play_stream();
        if self.music_paused() {
            next.music.pause_stream();
            next.music.set_volume(volume);
            self.resume_music_after_pause = true;
        }
        self.current_music = Some(next_id.to_owned());
    }

    /// Pauses music at its current position while authored sequence cues continue.
    ///
    /// Entering the sequence cuts older voices/SFX once. Further phase cues are
    /// allowed to play normally; leaving resumes rather than restarts the track.
    pub fn set_cinematic_paused(&mut self, paused: bool) {
        if self.cinematic_paused == paused {
            return;
        }
        let music_was_paused = self.music_paused();
        self.cinematic_paused = paused;
        if !self.enabled {
            return;
        }
        if paused {
            for loaded in self.sounds.values() {
                loaded.sound.stop();
            }
            self.paused_sounds.clear();
        }
        self.sync_music_pause(music_was_paused);
    }

    /// Freezes current music and playing sounds while the match overlay is open.
    ///
    /// Resume continues only sounds that this pause suspended. A cinematic's
    /// music pause remains independent. New combat events are ignored without
    /// advancing variation cursors; new UI feedback remains available.
    pub fn set_match_paused(&mut self, paused: bool) {
        if self.match_paused == paused {
            return;
        }
        let music_was_paused = self.music_paused();
        self.match_paused = paused;
        if paused {
            self.paused_sounds.clear();
            for (id, loaded) in &self.sounds {
                if loaded.sound.is_playing() {
                    loaded.sound.pause();
                    self.paused_sounds.push(id.clone());
                }
            }
        } else {
            for id in self.paused_sounds.drain(..) {
                if let Some(loaded) = self.sounds.get(&id) {
                    loaded.sound.resume();
                }
            }
        }
        self.sync_music_pause(music_was_paused);
    }

    fn music_paused(&self) -> bool {
        self.cinematic_paused || self.match_paused
    }

    fn sync_music_pause(&mut self, was_paused: bool) {
        let paused = self.music_paused();
        if !self.enabled || paused == was_paused {
            return;
        }
        if let Some(current) = self
            .current_music
            .as_ref()
            .and_then(|id| self.music.get(id))
        {
            if paused {
                self.resume_music_after_pause = current.music.is_stream_playing();
                if self.resume_music_after_pause {
                    current.music.pause_stream();
                }
            } else if std::mem::take(&mut self.resume_music_after_pause) {
                current.music.resume_stream();
            }
        }
    }

    /// Cuts the unfinished sequence's sounds and restores music after an abort.
    ///
    /// Reset and scene changes use this instead of the normal completion path,
    /// which lets the final impact or end cue finish playing. It also discards
    /// paused sounds so a scene change cannot resume voices from the old fight.
    /// Call `set_match_paused(false)` when leaving the pause overlay itself.
    pub fn cancel_cinematic(&mut self) {
        for loaded in self.sounds.values() {
            loaded.sound.stop();
        }
        self.paused_sounds.clear();
        self.set_cinematic_paused(false);
    }

    /// Lowers the active music while a foreground cue needs priority.
    pub fn set_music_ducking(&mut self, ducked: bool) {
        if !self.enabled {
            return;
        }

        if self.music_ducked == ducked {
            return;
        }
        self.music_ducked = ducked;

        if let Some(current) = self
            .current_music
            .as_ref()
            .and_then(|id| self.music.get(id))
        {
            current.music.set_volume(music_output_volume(
                current.volume,
                self.music_ducked,
                self.music_volume,
            ));
        }
    }

    /// Sets the global music volume multiplier for all streamed tracks.
    pub fn set_music_volume(&mut self, volume: f32) {
        if !self.enabled {
            return;
        }

        let volume = volume.clamp(0.0, 1.0);
        if (self.music_volume - volume).abs() <= f32::EPSILON {
            return;
        }
        self.music_volume = volume;

        if let Some(current) = self
            .current_music
            .as_ref()
            .and_then(|id| self.music.get(id))
        {
            current.music.set_volume(music_output_volume(
                current.volume,
                self.music_ducked,
                self.music_volume,
            ));
        }
    }

    /// Plays every event in order.
    pub fn play_events(&mut self, events: impl IntoIterator<Item = AudioEvent>) {
        for event in events {
            self.play(&event);
        }
    }

    /// Resolves and plays one event if a loaded clip is available.
    pub fn play(&mut self, event: &AudioEvent) {
        if !self.enabled || (self.match_paused && !pause_overlay_feedback(event.cue)) {
            return;
        }

        if event.cue == AudioCue::SuperStart {
            // Super variants follow authored phases (for example morph, then
            // grow). A replay after an abort must restart that order while
            // ordinary fighter voices keep their independent variation.
            for (binding, cursor) in self.bank.bindings().iter().zip(&mut self.binding_cursors) {
                if binding.cue.starts_with("super.") {
                    *cursor = 0;
                }
            }
        }

        let Some(binding_index) = self.bank.binding_index_for_event(event) else {
            return;
        };
        let Some(clip_ids) = self.bank.binding_clip_ids(binding_index) else {
            return;
        };
        let Some(cursor) = self.binding_cursors.get_mut(binding_index) else {
            return;
        };
        let Some(clip_id) =
            next_loaded_clip_id(clip_ids, |id| self.sounds.contains_key(id), cursor)
        else {
            return;
        };
        let Some(loaded) = self.sounds.get(clip_id) else {
            return;
        };

        if self.match_paused {
            // A fresh UI cue replaces any earlier instance of the same sound;
            // it must not be resumed again when the fight overlay closes.
            self.paused_sounds.retain(|id| id != clip_id);
        }

        loaded.sound.set_volume(loaded.volume);
        loaded.sound.set_pitch(loaded.pitch);
        loaded.sound.set_pan(loaded.pan);
        loaded.sound.play();
    }
}

fn pause_overlay_feedback(cue: AudioCue) -> bool {
    matches!(
        cue,
        AudioCue::UiNavigate | AudioCue::UiConfirm | AudioCue::UiBack
    )
}

impl<'aud> LoadedSound<'aud> {
    fn new(sound: Sound<'aud>, clip: &AudioClipDefinition) -> Self {
        Self {
            sound,
            volume: clip.volume.clamp(0.0, 1.0),
            pitch: clip.pitch.max(0.01),
            pan: clip.pan.clamp(0.0, 1.0),
        }
    }
}

impl<'aud> LoadedMusic<'aud> {
    fn new(music: Music<'aud>, track: &AudioMusicDefinition) -> Self {
        Self {
            music,
            volume: track.volume.clamp(0.0, 1.0),
            pitch: track.pitch.max(0.01),
        }
    }
}

fn next_loaded_clip_id<'clips>(
    clip_ids: &'clips [String],
    is_loaded: impl Fn(&str) -> bool,
    cursor: &mut usize,
) -> Option<&'clips str> {
    if clip_ids.is_empty() {
        return None;
    }

    let start = *cursor % clip_ids.len();
    for offset in 0..clip_ids.len() {
        let index = (start + offset) % clip_ids.len();
        let clip_id = clip_ids[index].as_str();
        if is_loaded(clip_id) {
            *cursor = (index + 1) % clip_ids.len();
            return Some(clip_id);
        }
    }

    None
}

fn keeps_current_music(current: Option<&str>, next: &str, paused: bool, playing: bool) -> bool {
    current == Some(next) && (paused || playing)
}

fn music_output_volume(volume: f32, ducked: bool, music_volume: f32) -> f32 {
    let multiplier = if ducked { 0.35 } else { 1.0 };
    (volume * multiplier * music_volume).clamp(0.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn match_and_cinematic_pause_can_be_released_in_either_order() {
        for release_match_first in [false, true] {
            let mut player = AudioPlayer::disabled();
            player.set_cinematic_paused(true);
            player.set_match_paused(true);
            assert!(player.music_paused());
            if release_match_first {
                player.set_match_paused(false);
                assert!(player.cinematic_paused && player.music_paused());
                player.set_cinematic_paused(false);
            } else {
                player.set_cinematic_paused(false);
                assert!(player.match_paused && player.music_paused());
                player.set_match_paused(false);
            }
            assert!(!player.music_paused());
        }
    }

    #[test]
    fn cancelling_a_paused_scene_discards_its_pending_voices() {
        let mut player = AudioPlayer::disabled();
        player.set_cinematic_paused(true);
        player.set_match_paused(true);
        // This records only the pending-resume state, not a fake audio device.
        player.paused_sounds.push("unfinished-phase".to_owned());
        player.cancel_cinematic();
        assert!(player.paused_sounds.is_empty());
        assert!(!player.cinematic_paused);
        assert!(player.match_paused, "scene owns when its overlay closes");
        player.set_match_paused(false);
        assert!(!player.music_paused());
        player.set_match_paused(true);
        player.paused_sounds.push("unfinished-punch".to_owned());
        player.cancel_cinematic();
        player.set_match_paused(false);
        assert!(player.paused_sounds.is_empty());
        assert!(!player.music_paused());
    }

    #[test]
    fn events_during_match_pause_do_not_consume_variations_or_restart_phase_order() {
        let mut player = AudioPlayer::disabled();
        player.bank = AudioBank::load(AUDIO_MANIFEST_PATH).unwrap();
        player.binding_cursors = vec![7; player.bank.bindings().len()];
        player.enabled = true;
        player.set_match_paused(true);
        let before = player.binding_cursors.clone();
        let event = AudioEvent::new(AudioCue::SuperStart);
        player.play(&event);
        assert_eq!(player.binding_cursors, before);
        player.set_match_paused(false);
        player.play(&event);
        assert_ne!(
            player.binding_cursors, before,
            "normal SuperStart resets authored phase order"
        );
    }

    #[test]
    fn paused_event_filter_allows_only_ui_feedback_from_the_audio_manifest() {
        let bank = AudioBank::load(AUDIO_MANIFEST_PATH).unwrap();
        let mut ui_cues = std::collections::HashSet::new();
        for binding in bank.bindings() {
            let cue = AudioCue::from_key(&binding.cue).expect("known manifest cue");
            assert_eq!(
                pause_overlay_feedback(cue),
                binding.cue.starts_with("ui."),
                "paused routing for {}",
                binding.cue,
            );
            if pause_overlay_feedback(cue) {
                ui_cues.insert(binding.cue.as_str());
            }
        }
        assert_eq!(
            ui_cues.len(),
            3,
            "navigate, confirm and back remain audible"
        );
    }

    #[test]
    fn music_output_volume_applies_ducking_and_user_volume() {
        assert_near(music_output_volume(0.5, false, 1.0), 0.5);
        assert_near(music_output_volume(0.5, false, 0.4), 0.2);
        assert_near(music_output_volume(0.5, true, 1.0), 0.175);
        assert_near(music_output_volume(2.0, false, 1.0), 1.0);
    }

    #[test]
    fn scene_track_refresh_never_restarts_paused_music() {
        assert!(keeps_current_music(Some("combat"), "combat", true, false));
        assert!(keeps_current_music(Some("combat"), "combat", false, true));
        assert!(!keeps_current_music(Some("combat"), "menu", true, false));
        assert!(!keeps_current_music(Some("combat"), "combat", false, false));
        assert!(!keeps_current_music(None, "combat", true, false));
    }

    #[test]
    fn replay_after_the_first_mutation_restarts_phase_sounds_without_resetting_voices() {
        use crate::{characters::CharacterId, combat::fighter::PlayerSlot};
        let mut player = AudioPlayer::disabled();
        player.bank = AudioBank::load(AUDIO_MANIFEST_PATH).unwrap();
        player.binding_cursors = vec![0; player.bank.bindings().len()];
        player.enabled = true;
        let voice = AudioEvent::fighter_hurt(PlayerSlot::Two, CharacterId::Rust);
        sample_variation(&mut player, &voice);
        let voice_binding = player.bank.binding_index_for_event(&voice).unwrap();
        let voice_cursor = player.binding_cursors[voice_binding];
        assert_ne!(
            voice_cursor, 0,
            "the hurt voice must have multiple variants"
        );

        for character in [CharacterId::Python, CharacterId::Rust] {
            let start =
                AudioEvent::new(AudioCue::SuperStart).with_fighter(PlayerSlot::One, character);
            let mutation =
                AudioEvent::new(AudioCue::SuperMutation).with_fighter(PlayerSlot::One, character);
            player.play(&start);
            let first = sample_variation(&mut player, &mutation);
            // Reset/replay occurs before the second phase. The next SuperStart
            // must restore the first phase even when the starting sound is absent.
            player.cancel_cinematic();
            player.play(&start);
            assert_eq!(sample_variation(&mut player, &mutation), first);
            assert_ne!(sample_variation(&mut player, &mutation), first);
            assert_eq!(player.binding_cursors[voice_binding], voice_cursor);
        }
    }

    fn sample_variation(player: &mut AudioPlayer<'_>, event: &AudioEvent) -> String {
        let binding = player.bank.binding_index_for_event(event).unwrap();
        next_loaded_clip_id(
            player.bank.binding_clip_ids(binding).unwrap(),
            |_| true,
            &mut player.binding_cursors[binding],
        )
        .unwrap()
        .to_owned()
    }

    fn assert_near(actual: f32, expected: f32) {
        assert!((actual - expected).abs() < 0.0001, "{actual} != {expected}");
    }
}
