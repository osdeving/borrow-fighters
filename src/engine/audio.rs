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
    binding_cursors: HashMap<usize, usize>,
    current_music: Option<String>,
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
            binding_cursors: HashMap::new(),
            current_music: None,
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

        Self {
            bank,
            sounds,
            music,
            binding_cursors: HashMap::new(),
            current_music: None,
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
        next.music.set_volume(next.volume);
        next.music.set_pitch(next.pitch);
        next.music.play_stream();
        self.current_music = Some(next_id.to_owned());
    }

    /// Lowers the active music while a foreground cue needs priority.
    pub fn set_music_ducking(&self, ducked: bool) {
        if !self.enabled {
            return;
        }

        let multiplier = if ducked { 0.35 } else { 1.0 };

        if let Some(current) = self
            .current_music
            .as_ref()
            .and_then(|id| self.music.get(id))
        {
            current
                .music
                .set_volume((current.volume * multiplier).clamp(0.0, 1.0));
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

        let Some((binding_index, binding)) = self.bank.binding_for_event(event) else {
            return;
        };
        let clip_ids = binding.clips.clone();
        let Some(clip_id) = self.next_loaded_clip_id(binding_index, &clip_ids) else {
            return;
        };
        let Some(loaded) = self.sounds.get(&clip_id) else {
            return;
        };

        loaded.sound.set_volume(loaded.volume);
        loaded.sound.set_pitch(loaded.pitch);
        loaded.sound.set_pan(loaded.pan);
        loaded.sound.play();
    }

    fn next_loaded_clip_id(&mut self, binding_index: usize, clip_ids: &[String]) -> Option<String> {
        if clip_ids.is_empty() {
            return None;
        }

        let start = *self.binding_cursors.get(&binding_index).unwrap_or(&0);
        for offset in 0..clip_ids.len() {
            let index = (start + offset) % clip_ids.len();
            if self.sounds.contains_key(&clip_ids[index]) {
                self.binding_cursors
                    .insert(binding_index, (index + 1) % clip_ids.len());
                return Some(clip_ids[index].clone());
            }
        }

        None
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
