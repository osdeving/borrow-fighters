//! Plays original street ambience and shared combat events for production chapters.
//!
//! System: Production audio boundary. Event observation happens once per fixed
//! update; pausing suspends active sounds and retries discard abandoned cues.

use crate::{
    adventure::{
        augusta::{Chapter, Phase},
        production::{Event, Team},
    },
    runtime_paths::asset_path,
};
use raylib::prelude::{Music, RaylibAudio, Sound};
use serde::Deserialize;
use std::{
    collections::{BTreeMap, BTreeSet},
    error::Error,
    fs,
    path::Path,
};

const CATALOG: &str = "assets/adventure/audio/production/catalog.json";

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Sample {
    file: String,
    volume: f32,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Catalog {
    version: u32,
    provenance: String,
    ambience: Sample,
    effects: BTreeMap<String, Sample>,
}

impl Catalog {
    fn load(path: &Path) -> Result<Self, Box<dyn Error>> {
        let catalog: Self = serde_json::from_str(&fs::read_to_string(path)?)?;
        if catalog.version != 1
            || catalog.provenance.is_empty()
            || ["swish", "impact", "parry", "projectile", "landing"]
                .iter()
                .any(|key| !catalog.effects.contains_key(*key))
            || catalog
                .effects
                .values()
                .chain([&catalog.ambience])
                .any(|sample| {
                    !sample.volume.is_finite()
                        || !(0.0..=1.0).contains(&sample.volume)
                        || sample.file.is_empty()
                        || !Path::new(&sample.file)
                            .components()
                            .all(|part| matches!(part, std::path::Component::Normal(_)))
                })
        {
            return Err("invalid production audio catalogue".into());
        }
        Ok(catalog)
    }

    fn validate_files(&self, directory: &Path) -> Result<(), Box<dyn Error>> {
        for sample in self.effects.values().chain([&self.ambience]) {
            if !directory.join(&sample.file).is_file() {
                return Err(format!("missing production sound {}", sample.file).into());
            }
        }
        Ok(())
    }
}

pub struct ProductionAudio<'a> {
    ambience: Option<Music<'a>>,
    sounds: BTreeMap<String, Sound<'a>>,
    paused: bool,
    started: bool,
    suspended: BTreeSet<String>,
    last_tick: u64,
    impact_played: bool,
}

impl<'a> ProductionAudio<'a> {
    pub fn new(device: Option<&'a RaylibAudio>) -> Result<Self, Box<dyn Error>> {
        let mut audio = Self {
            ambience: None,
            sounds: BTreeMap::new(),
            paused: false,
            started: false,
            suspended: BTreeSet::new(),
            last_tick: 0,
            impact_played: false,
        };
        let path = asset_path(CATALOG);
        let catalog = Catalog::load(&path)?;
        let directory = path.parent().ok_or("missing audio directory")?;
        catalog.validate_files(directory)?;
        let Some(device) = device else {
            return Ok(audio);
        };
        let mut ambience =
            device.new_music(&directory.join(catalog.ambience.file).to_string_lossy())?;
        ambience.set_looping(true);
        ambience.set_volume(catalog.ambience.volume);
        audio.ambience = Some(ambience);
        for (key, sample) in catalog.effects {
            let sound = device.new_sound(&directory.join(sample.file).to_string_lossy())?;
            sound.set_volume(sample.volume);
            audio.sounds.insert(key, sound);
        }
        Ok(audio)
    }

    /// Called after each simulation update; repeated rendering cannot repeat a hit.
    pub fn observe(&mut self, chapter: &Chapter, events: &[Event]) -> Vec<&'static str> {
        if self.paused || chapter.ticks() == self.last_tick {
            return vec![];
        }
        self.last_tick = chapter.ticks();
        let mut cues = BTreeSet::new();
        for event in events {
            let cue = match event {
                Event::MoveStarted { .. } => Some("swish"),
                Event::Hit { .. } | Event::Blocked { .. } => Some("impact"),
                Event::Parried { .. } => Some("parry"),
                Event::ProjectileSpawned { .. } => Some("projectile"),
                Event::Landed { .. } => Some("landing"),
                _ => None,
            };
            if let Some(cue) = cue {
                cues.insert(cue);
            }
        }
        if chapter.phase() != Phase::ErraticsArrival {
            self.impact_played = false;
        } else if !self.impact_played
            && chapter
                .simulation()
                .actors()
                .iter()
                .filter(|a| a.team == Team::Enemy)
                .any(|a| {
                    chapter
                        .arrival_pose(a.id)
                        .is_some_and(|pose| pose.impact_age.is_some())
                })
        {
            cues.insert("landing");
            self.impact_played = true;
        }
        for key in &cues {
            if let Some(sound) = self.sounds.get(*key) {
                sound.play();
            }
        }
        cues.into_iter().collect()
    }

    /// Updates the stream independently from how many fixed ticks were batched.
    pub fn update(&mut self, paused: bool) {
        if !self.started {
            if let Some(ambience) = &self.ambience {
                ambience.play_stream();
            }
            self.started = true;
        }
        if self.paused != paused {
            if let Some(ambience) = &self.ambience {
                if paused {
                    ambience.pause_stream();
                } else {
                    ambience.resume_stream();
                }
            }
            if paused {
                for (key, sound) in &self.sounds {
                    if sound.is_playing() {
                        sound.pause();
                        self.suspended.insert(key.clone());
                    }
                }
            } else {
                for key in &self.suspended {
                    if let Some(sound) = self.sounds.get(key) {
                        sound.resume();
                    }
                }
                self.suspended.clear();
            }
            self.paused = paused;
        }
        if !paused && let Some(ambience) = &self.ambience {
            ambience.update_stream();
        }
    }

    /// An explicit jump in story state cannot replay the old encounter's effects.
    pub fn synchronize(&mut self, chapter: &Chapter) {
        for sound in self.sounds.values() {
            sound.stop();
        }
        self.suspended.clear();
        self.last_tick = chapter.ticks();
        self.impact_played = chapter.phase() == Phase::ErraticsArrival
            && chapter.simulation().actors().iter().any(|a| {
                chapter
                    .arrival_pose(a.id)
                    .is_some_and(|pose| pose.impact_age.is_some())
            });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalogue_is_complete_and_pcm_samples_are_present() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(CATALOG);
        let catalog = Catalog::load(&path).unwrap();
        for sample in catalog.effects.values().chain([&catalog.ambience]) {
            let bytes = fs::read(path.parent().unwrap().join(&sample.file)).unwrap();
            assert_eq!(&bytes[..4], b"RIFF");
            assert_eq!(&bytes[8..12], b"WAVE");
            assert!(bytes.len() > 2205);
        }
    }

    #[test]
    fn missing_sound_is_rejected_before_allocating_an_audio_device() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(CATALOG);
        let mut candidate = Catalog::load(&path).unwrap();
        assert!(candidate.validate_files(path.parent().unwrap()).is_ok());
        candidate.effects.get_mut("impact").unwrap().file = "missing-impact.wav".into();
        assert!(candidate.validate_files(path.parent().unwrap()).is_err());
        let current = Catalog::load(&path).unwrap();
        assert!(current.validate_files(path.parent().unwrap()).is_ok());
    }
}
