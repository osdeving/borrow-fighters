//! Plays manifest-routed audio events and music through Raylib.
//!
//! System: Raylib audio boundary. This module owns loaded `Sound` and `Music`
//! resources and maps pure gameplay audio events to raylib playback calls.

use std::{collections::HashMap, path::Path};

use raylib::prelude::*;

use crate::audio::{AudioBank, AudioClipDefinition, AudioEvent, AudioMusicDefinition, MusicTrack};

pub const AUDIO_MANIFEST_PATH: &str = "assets/audio/audio_manifest.json";

/// Raylib-backed audio event player.
pub struct AudioPlayer<'aud> {
    bank: AudioBank,
    sounds: HashMap<String, LoadedSound<'aud>>,
    music: HashMap<String, LoadedMusic<'aud>>,
    binding_cursors: Vec<usize>,
    current_music: Option<String>,
    music_ducked: bool,
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
            music_volume: 1.0,
            enabled: false,
        }
    }

    /// Loads audio manifest and optional sound files.
    pub fn load(audio: &'aud RaylibAudio, manifest_path: &str) -> Self {
        let bank = match AudioBank::load(manifest_path) {
            Ok(bank) => bank,
            Err(error) => {
                eprintln!("warning: audio disabled: {error}");
                return Self::disabled();
            }
        };

        let mut sounds = HashMap::new();
        for clip in bank.clips() {
            if !Path::new(&clip.file).exists() {
                if clip.required {
                    eprintln!(
                        "warning: required audio clip {} is missing at {}",
                        clip.id, clip.file
                    );
                }
                continue;
            }

            match audio.new_sound(&clip.file) {
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
            if !Path::new(&track.file).exists() {
                if track.required {
                    eprintln!(
                        "warning: required music track {} is missing at {}",
                        track.id, track.file
                    );
                }
                continue;
            }

            match audio.new_music(&track.file) {
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
            music_volume: 1.0,
            enabled: true,
        }
    }

    /// Updates the active music stream.
    pub fn update_streams(&self) {
        if !self.enabled {
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
        if self.current_music.as_deref() == Some(next_id)
            && self
                .music
                .get(next_id)
                .is_some_and(|loaded| loaded.music.is_stream_playing())
        {
            return;
        }

        if let Some(current_id) = self.current_music.take()
            && let Some(current) = self.music.get(&current_id)
        {
            current.music.stop_stream();
        }

        let Some(next) = self.music.get(next_id) else {
            return;
        };
        next.music.set_volume(music_output_volume(
            next.volume,
            self.music_ducked,
            self.music_volume,
        ));
        next.music.set_pitch(next.pitch);
        next.music.play_stream();
        self.current_music = Some(next_id.to_owned());
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
        if !self.enabled {
            return;
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
        let Some(clip_id) = next_loaded_clip_id(clip_ids, &self.sounds, cursor) else {
            return;
        };
        let Some(loaded) = self.sounds.get(clip_id) else {
            return;
        };

        loaded.sound.set_volume(loaded.volume);
        loaded.sound.set_pitch(loaded.pitch);
        loaded.sound.set_pan(loaded.pan);
        loaded.sound.play();
    }
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

fn next_loaded_clip_id<'clips, 'aud>(
    clip_ids: &'clips [String],
    sounds: &HashMap<String, LoadedSound<'aud>>,
    cursor: &mut usize,
) -> Option<&'clips str> {
    if clip_ids.is_empty() {
        return None;
    }

    let start = *cursor % clip_ids.len();
    for offset in 0..clip_ids.len() {
        let index = (start + offset) % clip_ids.len();
        let clip_id = clip_ids[index].as_str();
        if sounds.contains_key(clip_id) {
            *cursor = (index + 1) % clip_ids.len();
            return Some(clip_id);
        }
    }

    None
}

fn music_output_volume(volume: f32, ducked: bool, music_volume: f32) -> f32 {
    let multiplier = if ducked { 0.35 } else { 1.0 };
    (volume * multiplier * music_volume).clamp(0.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn music_output_volume_applies_ducking_and_user_volume() {
        assert_near(music_output_volume(0.5, false, 1.0), 0.5);
        assert_near(music_output_volume(0.5, false, 0.4), 0.2);
        assert_near(music_output_volume(0.5, true, 1.0), 0.175);
        assert_near(music_output_volume(2.0, false, 1.0), 1.0);
    }

    fn assert_near(actual: f32, expected: f32) {
        assert!((actual - expected).abs() < 0.0001, "{actual} != {expected}");
    }
}
