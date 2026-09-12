//! Plays original street ambience and shared combat events for production chapters.
//!
//! System: Production audio boundary. Event observation happens once per fixed
//! update; pausing suspends active sounds and retries discard abandoned cues.

use crate::{
    adventure::{
        augusta::{Chapter, Phase},
        production::Event,
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
    air: Sample,
    effects: BTreeMap<String, Sample>,
}

impl Catalog {
    fn load(path: &Path) -> Result<Self, Box<dyn Error>> {
        let catalog: Self = serde_json::from_str(&fs::read_to_string(path)?)?;
        if catalog.version != 1
            || catalog.provenance.is_empty()
            || [
                "swish",
                "impact",
                "parry",
                "projectile",
                "landing",
                "bar-door",
                "guard-step",
                "ep-rupture",
                "panic",
            ]
            .iter()
            .any(|key| !catalog.effects.contains_key(*key))
            || catalog
                .effects
                .values()
                .chain([&catalog.ambience, &catalog.air])
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
        for sample in self.effects.values().chain([&self.ambience, &self.air]) {
            if !directory.join(&sample.file).is_file() {
                return Err(format!("missing production sound {}", sample.file).into());
            }
        }
        Ok(())
    }
}

#[derive(Clone, Copy)]
struct StorySoundFrame {
    phase: Phase,
    phase_ticks: u32,
    guard_duration: u32,
    threat_age: Option<u64>,
    landing_age: Option<u32>,
}

impl StorySoundFrame {
    fn from_chapter(chapter: &Chapter) -> Self {
        Self {
            phase: chapter.phase(),
            phase_ticks: chapter.phase_ticks(),
            guard_duration: chapter.spec.timing.guards_arrival_ticks,
            threat_age: chapter.threat_age(),
            landing_age: chapter.landing_age(),
        }
    }
}

#[derive(Default)]
struct StorySounds {
    previous: Option<StorySoundFrame>,
}

impl StorySounds {
    fn observe(&mut self, now: StorySoundFrame) -> BTreeSet<&'static str> {
        let mut cues = BTreeSet::new();
        let previous = self.previous;
        if now.phase == Phase::GuardsArrival {
            let before = previous
                .filter(|p| p.phase == now.phase)
                .map(|p| p.phase_ticks);
            if before.is_none() {
                cues.insert("bar-door");
            }
            let start = now.guard_duration / 10;
            let finish = now.guard_duration * 9 / 10;
            if (start..finish)
                .step_by(19)
                .any(|tick| now.phase_ticks >= tick && before.is_none_or(|last| last < tick))
            {
                cues.insert("guard-step");
            }
        }
        if let Some(age) = now.threat_age {
            let last_age = previous.and_then(|p| p.threat_age);
            if last_age.is_none() {
                cues.insert("ep-rupture");
            }
            if age >= 8 && last_age.is_none_or(|last| last < 8) {
                cues.insert("panic");
            }
        }
        // Landing belongs to the contact clock and can survive a phase handoff.
        if now.landing_age.is_some() && previous.is_none_or(|p| p.landing_age.is_none()) {
            cues.insert("landing");
        }
        self.previous = Some(now);
        cues
    }

    fn synchronize(&mut self, now: StorySoundFrame) {
        self.previous = Some(now);
    }
}

fn traffic_gain(threat_age: Option<u64>) -> f32 {
    threat_age.map_or(1.0, |age| 1.0 - (age as f32 / 360.0).clamp(0.0, 1.0))
}

pub struct ProductionAudio<'a> {
    ambience: Option<Music<'a>>,
    air: Option<Music<'a>>,
    traffic_volume: f32,
    sounds: BTreeMap<String, Sound<'a>>,
    paused: bool,
    started: bool,
    suspended: BTreeSet<String>,
    last_tick: u64,
    story: StorySounds,
}

impl<'a> ProductionAudio<'a> {
    pub fn new(device: Option<&'a RaylibAudio>) -> Result<Self, Box<dyn Error>> {
        let mut audio = Self {
            ambience: None,
            air: None,
            traffic_volume: 0.0,
            sounds: BTreeMap::new(),
            paused: false,
            started: false,
            suspended: BTreeSet::new(),
            last_tick: 0,
            story: StorySounds::default(),
        };
        let path = asset_path(CATALOG);
        let catalog = Catalog::load(&path)?;
        let directory = path.parent().ok_or("missing audio directory")?;
        catalog.validate_files(directory)?;
        audio.traffic_volume = catalog.ambience.volume;
        let Some(device) = device else {
            return Ok(audio);
        };
        let mut ambience =
            device.new_music(&directory.join(catalog.ambience.file).to_string_lossy())?;
        ambience.set_looping(true);
        ambience.set_volume(catalog.ambience.volume);
        audio.ambience = Some(ambience);
        let mut air = device.new_music(&directory.join(catalog.air.file).to_string_lossy())?;
        air.set_looping(true);
        air.set_volume(catalog.air.volume);
        audio.air = Some(air);
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
        let mut cues = self.story.observe(StorySoundFrame::from_chapter(chapter));
        self.set_traffic(chapter.threat_age());
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
            for ambience in self.ambience.iter().chain(self.air.iter()) {
                ambience.play_stream();
            }
            self.started = true;
        }
        if self.paused != paused {
            for ambience in self.ambience.iter().chain(self.air.iter()) {
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
        if !paused {
            for ambience in self.ambience.iter().chain(self.air.iter()) {
                ambience.update_stream();
            }
        }
    }

    fn set_traffic(&self, threat_age: Option<u64>) {
        if let Some(ambience) = &self.ambience {
            ambience.set_volume(self.traffic_volume * traffic_gain(threat_age));
        }
    }

    /// An explicit jump in story state cannot replay the old encounter's effects.
    pub fn synchronize(&mut self, chapter: &Chapter) {
        for sound in self.sounds.values() {
            sound.stop();
        }
        self.suspended.clear();
        self.last_tick = chapter.ticks();
        self.story
            .synchronize(StorySoundFrame::from_chapter(chapter));
        self.set_traffic(chapter.threat_age());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalogue_is_complete_and_pcm_samples_are_present() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(CATALOG);
        let catalog = Catalog::load(&path).unwrap();
        for sample in catalog
            .effects
            .values()
            .chain([&catalog.ambience, &catalog.air])
        {
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

    fn frame(
        phase: Phase,
        ticks: u32,
        threat_age: Option<u64>,
        landing_age: Option<u32>,
    ) -> StorySoundFrame {
        StorySoundFrame {
            phase,
            phase_ticks: ticks,
            guard_duration: 540,
            threat_age,
            landing_age,
        }
    }

    #[test]
    fn cinematic_cues_cross_ticks_once_and_contact_survives_handoff() {
        let mut story = StorySounds::default();
        let door = frame(Phase::GuardsArrival, 0, None, None);
        assert_eq!(story.observe(door), BTreeSet::from(["bar-door"]));
        assert!(story.observe(door).is_empty());
        assert!(
            story
                .observe(frame(Phase::GuardsArrival, 120, None, None))
                .contains("guard-step")
        );
        assert!(
            story
                .observe(frame(Phase::GuardsArrival, 120, None, None))
                .is_empty()
        );
        let rupture = frame(Phase::ErraticsArrival, 0, Some(0), None);
        assert_eq!(story.observe(rupture), BTreeSet::from(["ep-rupture"]));
        assert!(story.observe(rupture).is_empty());
        assert_eq!(
            story.observe(frame(Phase::ErraticsArrival, 40, Some(40), None)),
            BTreeSet::from(["panic"])
        );
        // A render frame may contain enough fixed updates to leave arrival.
        let combat = frame(Phase::ErraticsFight, 1, Some(481), Some(121));
        assert_eq!(story.observe(combat), BTreeSet::from(["landing"]));
        assert!(story.observe(combat).is_empty());
    }

    #[test]
    fn synchronization_discards_old_cues_and_rearms_only_new_arrivals() {
        let mut story = StorySounds::default();
        let restored = frame(Phase::ErraticsFight, 0, Some(600), Some(240));
        story.synchronize(restored);
        assert!(story.observe(restored).is_empty());
        assert!(
            story
                .observe(frame(Phase::FindJulia, 0, Some(2400), Some(2040)))
                .is_empty()
        );
        story.synchronize(frame(Phase::Introduction, 0, None, None));
        assert_eq!(
            story.observe(frame(Phase::GuardsArrival, 0, None, None)),
            BTreeSet::from(["bar-door"])
        );
        assert_eq!(traffic_gain(None), 1.0);
        assert_eq!(traffic_gain(Some(180)), 0.5);
        assert_eq!(traffic_gain(Some(360)), 0.0);
        assert_eq!(traffic_gain(Some(36000)), 0.0);
    }
}
