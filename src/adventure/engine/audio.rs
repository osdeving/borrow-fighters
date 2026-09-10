//! Plays the adventure's original ambience and contact sounds through Raylib.
//!
//! System: Adventure audio boundary. Borrowed device lifetimes keep streams safe;
//! missing audio is optional, and no arena-fighting audio rules are imported.

use raylib::prelude::{Music, RaylibAudio, Sound};

use crate::adventure::{
    combat::{ActorKind, Outcome},
    story::{Stage, Story},
};
use crate::runtime_paths::asset_path;

const VOLUME: f32 = 0.3;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Track {
    Ada,
    Morning,
    Threat,
    Remorse,
    Opening,
}

impl Track {
    const ALL: [Self; 5] = [
        Self::Ada,
        Self::Morning,
        Self::Threat,
        Self::Remorse,
        Self::Opening,
    ];

    fn file(self) -> &'static str {
        match self {
            Self::Ada => "ada.wav",
            Self::Morning => "morning.wav",
            Self::Threat => "threat.wav",
            Self::Remorse => "remorse.wav",
            Self::Opening => "opening.wav",
        }
    }

    fn looping(self) -> bool {
        self != Self::Opening
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Cue {
    Strike,
    Block,
    Hurt,
    Transition,
}

impl Cue {
    const ALL: [Self; 4] = [Self::Strike, Self::Block, Self::Hurt, Self::Transition];

    fn file(self) -> &'static str {
        match self {
            Self::Strike => "strike.wav",
            Self::Block => "block.wav",
            Self::Hurt => "hurt.wav",
            Self::Transition => "transition.wav",
        }
    }
}

/// Adventure-only audio whose resources cannot outlive the platform's device.
pub struct AdventureAudio<'aud> {
    music: Vec<(Track, Music<'aud>)>,
    sounds: Vec<(Cue, Sound<'aud>)>,
    current_track: Option<Track>,
    paused: bool,
    suspended_sounds: Vec<Cue>,
    observed: ObservedAudio,
}

impl<'aud> AdventureAudio<'aud> {
    /// Loads available audio, or remains silent when device creation failed.
    ///
    /// The app owns `RaylibAudio::init_audio_device().ok()` for the loop's lifetime
    /// and passes its `as_ref()` here; no self-reference or leaked device is used.
    pub fn new(device: Option<&'aud RaylibAudio>) -> Self {
        let mut player = Self {
            music: Vec::new(),
            sounds: Vec::new(),
            current_track: None,
            paused: false,
            suspended_sounds: Vec::new(),
            observed: ObservedAudio::default(),
        };
        let Some(device) = device else {
            return player;
        };
        for track in Track::ALL {
            let path = asset_path(format!("assets/adventure/audio/{}", track.file()));
            if path.is_file()
                && let Ok(mut music) = device.new_music(&path.to_string_lossy())
            {
                music.set_looping(track.looping());
                music.set_volume(VOLUME);
                player.music.push((track, music));
            }
        }
        for cue in Cue::ALL {
            let path = asset_path(format!("assets/adventure/audio/{}", cue.file()));
            if path.is_file()
                && let Ok(sound) = device.new_sound(&path.to_string_lossy())
            {
                sound.set_volume(VOLUME);
                player.sounds.push((cue, sound));
            }
        }
        player
    }

    /// Updates the selected ambience and emits each new contact only once.
    pub fn update(&mut self, story: &Story, paused: bool) {
        let next_track = background_for(story);
        if self.current_track != Some(next_track) {
            if let Some(current) = self.active_music() {
                current.stop_stream();
            }
            self.current_track = Some(next_track);
            if let Some(next) = self.active_music() {
                // Selecting a stage from a paused overlay must stay inaudible.
                next.set_volume(if paused { 0.0 } else { VOLUME });
                next.play_stream();
                if paused {
                    next.pause_stream();
                    next.set_volume(VOLUME);
                }
            }
        }
        self.set_paused(paused);
        if paused {
            return;
        }
        if let Some(music) = self.active_music() {
            music.update_stream();
        }
        for cue in self.observed.observe(story) {
            if let Some((_, sound)) = self.sounds.iter().find(|(id, _)| *id == cue) {
                sound.play();
            }
        }
    }

    fn active_music(&self) -> Option<&Music<'aud>> {
        self.music
            .iter()
            .find(|(track, _)| Some(*track) == self.current_track)
            .map(|(_, music)| music)
    }

    fn set_paused(&mut self, paused: bool) {
        if paused == self.paused {
            return;
        }
        self.paused = paused;
        if let Some(music) = self.active_music() {
            if paused {
                music.pause_stream();
            } else {
                music.resume_stream();
            }
        }
        if paused {
            self.suspended_sounds.clear();
            for (cue, sound) in &self.sounds {
                if sound.is_playing() {
                    sound.pause();
                    self.suspended_sounds.push(*cue);
                }
            }
        } else {
            for cue in self.suspended_sounds.drain(..) {
                if let Some((_, sound)) = self.sounds.iter().find(|(id, _)| *id == cue) {
                    sound.resume();
                }
            }
        }
    }
}

fn background_for(story: &Story) -> Track {
    match story.stage {
        Stage::AdaPrologue => Track::Ada,
        Stage::RustMorning => Track::Morning,
        Stage::Encounter if story.combat.outcome == Outcome::Defeat => Track::Remorse,
        Stage::Encounter if story.combat.enemy_awake => Track::Threat,
        Stage::Encounter => Track::Morning,
        Stage::Aftermath | Stage::Complete => Track::Remorse,
        Stage::Opening => Track::Opening,
    }
}

#[derive(Default)]
struct ObservedAudio {
    stage: Option<Stage>,
    stage_ticks: u32,
    combat_ticks: u32,
    enemy_awake: bool,
    hit_tick: Option<u32>,
}

impl ObservedAudio {
    fn observe(&mut self, story: &Story) -> Vec<Cue> {
        let mut cues = Vec::with_capacity(2);
        let reset = story.combat.ticks < self.combat_ticks
            || (self.stage == Some(story.stage) && story.stage_ticks < self.stage_ticks);
        if reset {
            self.hit_tick = None;
        }
        let changed_stage = self.stage.is_some() && self.stage != Some(story.stage);
        let noticed_rust = story.combat.enemy_awake && !self.enemy_awake;
        if changed_stage || noticed_rust || reset {
            cues.push(Cue::Transition);
        }
        if let Some(hit) = story.combat.last_hit {
            let contact_tick = story.combat.ticks.saturating_sub(hit.age_ticks);
            if hit.age_ticks <= 1 && self.hit_tick != Some(contact_tick) {
                cues.push(if hit.blocked {
                    Cue::Block
                } else if hit.target == ActorKind::Player {
                    Cue::Hurt
                } else {
                    Cue::Strike
                });
                self.hit_tick = Some(contact_tick);
            }
        }
        self.stage = Some(story.stage);
        self.stage_ticks = story.stage_ticks;
        self.combat_ticks = story.combat.ticks;
        self.enemy_awake = story.combat.enemy_awake;
        cues
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{adventure::combat::HitFeedback, math::vec2::Vec2};

    #[test]
    fn repeated_render_updates_do_not_repeat_the_same_contact() {
        let mut story = Story::new();
        story.advance_scene();
        story.advance_scene();
        story.combat.ticks = 50;
        story.combat.last_hit = Some(HitFeedback {
            target: ActorKind::Erratic,
            position: Vec2::ZERO,
            blocked: false,
            damage: 12,
            age_ticks: 0,
        });
        let mut observed = ObservedAudio::default();
        assert_eq!(observed.observe(&story), [Cue::Strike]);
        assert!(observed.observe(&story).is_empty());
        story.combat.ticks += 1;
        story.combat.last_hit.as_mut().unwrap().age_ticks = 1;
        assert!(observed.observe(&story).is_empty());
        story.combat.ticks += 20;
        story.combat.last_hit.as_mut().unwrap().age_ticks = 0;
        story.combat.last_hit.as_mut().unwrap().blocked = true;
        assert_eq!(observed.observe(&story), [Cue::Block]);
    }

    #[test]
    fn pausing_silent_audio_does_not_consume_or_repeat_pending_cues() {
        let mut audio = AdventureAudio::new(None);
        let mut story = Story::new();
        audio.update(&story, false);
        story.advance_scene();
        audio.update(&story, true);
        audio.update(&story, true);
        assert_eq!(audio.observed.stage, Some(Stage::AdaPrologue));
        audio.update(&story, false);
        assert_eq!(audio.observed.stage, Some(Stage::RustMorning));
        assert!(audio.observed.observe(&story).is_empty());
    }

    #[test]
    fn threat_begins_on_aggro_and_aftermath_uses_a_quiet_track() {
        let mut story = Story::new();
        assert_eq!(background_for(&story), Track::Ada);
        story.advance_scene();
        assert_eq!(background_for(&story), Track::Morning);
        story.advance_scene();
        assert_eq!(background_for(&story), Track::Morning);
        story.combat.enemy_awake = true;
        assert_eq!(background_for(&story), Track::Threat);
        story.stage = Stage::Aftermath;
        assert_eq!(background_for(&story), Track::Remorse);
    }

    #[test]
    fn opening_has_its_own_score_without_looping_or_replacing_aftermath() {
        let mut story = Story::new();
        story.stage = Stage::Aftermath;
        assert_eq!(background_for(&story), Track::Remorse);
        story.stage = Stage::Opening;
        assert_eq!(background_for(&story), Track::Opening);
        assert!(!Track::Opening.looping());
        assert!(Track::Remorse.looping());
        story.stage = Stage::Complete;
        assert_eq!(background_for(&story), Track::Remorse);
    }

    #[test]
    fn paused_opening_keeps_transition_pending_until_resume() {
        let mut audio = AdventureAudio::new(None);
        let mut story = Story::new();
        story.stage = Stage::Aftermath;
        audio.update(&story, false);
        story.stage = Stage::Opening;
        audio.update(&story, true);
        assert_eq!(audio.current_track, Some(Track::Opening));
        assert_eq!(audio.observed.stage, Some(Stage::Aftermath));
        audio.update(&story, false);
        assert_eq!(audio.observed.stage, Some(Stage::Opening));
        assert!(audio.observed.observe(&story).is_empty());
    }
}
