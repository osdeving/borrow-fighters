//! Plays the adventure's original ambience and contact sounds through Raylib.
//!
//! System: Adventure audio boundary. Borrowed device lifetimes keep streams safe;
//! missing audio is optional, and no arena-fighting audio rules are imported.

use raylib::prelude::{Music, RaylibAudio, Sound};

use crate::adventure::{
    ambient::{
        BICYCLE_FALL_TICK, CAR_HORN_TICK, CAR_IMPACT_TICK, CAR_SKID_TICK, STREET_EVACUATED_TICK,
    },
    combat::ActorKind,
    neighborhood::{DOG_STARTLE_TICK, SHUTTER_CLOSED_TICK, SHUTTER_START_TICK},
    story::{Stage, Story},
};
use crate::runtime_paths::asset_path;

const VOLUME: f32 = 0.3;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Track {
    Ada,
    Morning,
    Street,
    Remorse,
    Opening,
}

impl Track {
    const ALL: [Self; 5] = [
        Self::Ada,
        Self::Morning,
        Self::Street,
        Self::Remorse,
        Self::Opening,
    ];

    fn file(self) -> &'static str {
        match self {
            Self::Ada => "ada.wav",
            Self::Morning => "morning_ambience.wav",
            Self::Street => "street_air.wav",
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
    TrafficEscape,
    BicycleFall,
    CarHorn,
    CarSkid,
    CarCrash,
    DogAlert,
    ShutterRoll,
    ShutterClack,
    EpDescent,
    EpImpact,
}

impl Cue {
    const ALL: [Self; 14] = [
        Self::Strike,
        Self::Block,
        Self::Hurt,
        Self::Transition,
        Self::TrafficEscape,
        Self::BicycleFall,
        Self::CarHorn,
        Self::CarSkid,
        Self::CarCrash,
        Self::DogAlert,
        Self::ShutterRoll,
        Self::ShutterClack,
        Self::EpDescent,
        Self::EpImpact,
    ];

    fn file(self) -> &'static str {
        match self {
            Self::Strike => "strike.wav",
            Self::Block => "block.wav",
            Self::Hurt => "hurt.wav",
            Self::Transition => "transition.wav",
            Self::TrafficEscape => "traffic_escape.wav",
            Self::BicycleFall => "bicycle_fall.wav",
            Self::CarHorn => "car_horn.wav",
            Self::CarSkid => "car_skid.wav",
            Self::CarCrash => "car_crash.wav",
            Self::DogAlert => "dog_alert.wav",
            Self::ShutterRoll => "shutter_roll.wav",
            Self::ShutterClack => "shutter_clack.wav",
            Self::EpDescent => "ep_descent.wav",
            Self::EpImpact => "ep_impact.wav",
        }
    }

    fn is_traffic(self) -> bool {
        matches!(
            self,
            Self::TrafficEscape
                | Self::BicycleFall
                | Self::CarHorn
                | Self::CarSkid
                | Self::CarCrash
                | Self::DogAlert
                | Self::ShutterRoll
                | Self::ShutterClack
                | Self::EpDescent
                | Self::EpImpact
        )
    }
}

/// Adventure-only audio whose resources cannot outlive the platform's device.
pub struct AdventureAudio<'aud> {
    music: Vec<(Track, Music<'aud>)>,
    traffic: Option<Music<'aud>>,
    traffic_active: bool,
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
            traffic: None,
            traffic_active: false,
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
        let path = asset_path("assets/adventure/audio/street_traffic.wav");
        if path.is_file()
            && let Ok(mut traffic) = device.new_music(&path.to_string_lossy())
        {
            traffic.set_looping(true);
            traffic.set_volume(VOLUME);
            player.traffic = Some(traffic);
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

    /// Updates ambience and emits each contact or street milestone only once.
    pub fn update(&mut self, story: &Story, paused: bool) {
        let reset = self.observed.timeline_restarted(story);
        // A retry abandons the old crash even if its ringing metal was paused.
        // Natural aftermath keeps the same street clock and the impact tail.
        if reset || !matches!(story.stage, Stage::Encounter | Stage::Aftermath) {
            for (cue, sound) in &self.sounds {
                if cue.is_traffic() {
                    sound.stop();
                }
            }
            self.suspended_sounds.retain(|cue| !cue.is_traffic());
        }
        self.update_traffic(story, paused, reset);
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
        if self.traffic_active
            && let Some(traffic) = &self.traffic
        {
            traffic.update_stream();
        }
        for cue in self.observed.observe(story) {
            if let Some((_, sound)) = self.sounds.iter().find(|(id, _)| *id == cue) {
                sound.play();
            }
        }
    }

    /// Aligns music with a skipped scene without replaying abandoned effects.
    pub fn sync_after_skip(&mut self, story: &Story) {
        for (_, sound) in &self.sounds {
            sound.stop();
        }
        self.suspended_sounds.clear();
        self.observed = ObservedAudio::at_story(story);
        self.update(story, self.paused);
        if let Some(music) = self.active_music() {
            let duration = music.get_time_length();
            if duration > 0.0 && duration.is_finite() {
                let ticks = if background_for(story) == Track::Street {
                    story.ambient.ticks()
                } else {
                    story.stage_ticks
                };
                let elapsed = ticks as f32 / 60.0;
                let position = if background_for(story).looping() {
                    elapsed.rem_euclid(duration)
                } else {
                    elapsed.min(duration)
                };
                music.seek_stream(position);
            }
        }
        if self.traffic_active
            && let Some(traffic) = &self.traffic
        {
            let duration = traffic.get_time_length();
            if duration > 0.0 && duration.is_finite() {
                // The camera can advance stage_ticks without advancing combat.
                // Traffic always follows the living street's own clock.
                let elapsed = story.ambient.ticks() as f32 / 60.0;
                traffic.seek_stream(elapsed.rem_euclid(duration));
            }
        }
    }

    fn update_traffic(&mut self, story: &Story, paused: bool, reset: bool) {
        let gain = traffic_gain(story);
        if self.traffic_active && (reset || gain == 0.0) {
            if let Some(traffic) = &self.traffic {
                traffic.stop_stream();
            }
            self.traffic_active = false;
        }
        if gain == 0.0 {
            return;
        }
        if let Some(traffic) = &self.traffic {
            if !self.traffic_active {
                traffic.set_volume(if paused { 0.0 } else { VOLUME * gain });
                traffic.play_stream();
                if paused {
                    traffic.pause_stream();
                }
            }
            traffic.set_volume(VOLUME * gain);
        }
        self.traffic_active = true;
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
        if self.traffic_active
            && let Some(traffic) = &self.traffic
        {
            if paused {
                traffic.pause_stream();
            } else {
                traffic.resume_stream();
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
        Stage::Encounter => Track::Street,
        Stage::Aftermath | Stage::Complete => Track::Remorse,
        Stage::Opening => Track::Opening,
    }
}

/// Cars recede with the evacuation clock, while the independent air bed stays.
fn traffic_gain(story: &Story) -> f32 {
    if !matches!(story.stage, Stage::Encounter | Stage::Aftermath) {
        return 0.0;
    }
    match story.ambient.accident_ticks() {
        Some(ticks) => (1.0 - ticks as f32 / STREET_EVACUATED_TICK as f32)
            .clamp(0.0, 1.0)
            .powi(2),
        None if story.stage == Stage::Encounter => 1.0,
        None => 0.0,
    }
}

#[derive(Default)]
struct ObservedAudio {
    stage: Option<Stage>,
    stage_ticks: u32,
    combat_ticks: u32,
    ambient_ticks: u32,
    accident_ticks: Option<u32>,
    enemy_awake: bool,
    ep_ticks: Option<u32>,
    hit_tick: Option<u32>,
}

impl ObservedAudio {
    fn at_story(story: &Story) -> Self {
        Self {
            stage: Some(story.stage),
            stage_ticks: story.stage_ticks,
            combat_ticks: story.combat.ticks,
            ambient_ticks: story.ambient.ticks(),
            accident_ticks: story.ambient.accident_ticks(),
            enemy_awake: story.combat.enemy_awake,
            ep_ticks: story.ep_arrival.ticks(),
            hit_tick: story
                .combat
                .last_hit
                .map(|hit| story.combat.ticks.saturating_sub(hit.age_ticks)),
        }
    }

    fn timeline_restarted(&self, story: &Story) -> bool {
        story.combat.ticks < self.combat_ticks
            || story.ambient.ticks() < self.ambient_ticks
            || (self.stage == Some(story.stage) && story.stage_ticks < self.stage_ticks)
    }

    fn observe(&mut self, story: &Story) -> Vec<Cue> {
        let mut cues = Vec::with_capacity(10);
        let reset = self.timeline_restarted(story);
        if reset {
            self.hit_tick = None;
            self.accident_ticks = None;
            self.ep_ticks = None;
        }
        let changed_stage = self.stage.is_some() && self.stage != Some(story.stage);
        let noticed_rust = story.combat.enemy_awake && !self.enemy_awake;
        if changed_stage || noticed_rust || reset {
            cues.push(Cue::Transition);
        }
        if story.ep_arrival_active()
            && let Some(ticks) = story.ep_arrival.ticks()
        {
            if self.ep_ticks.is_none() && !story.ep_arrival.impacted() {
                cues.push(Cue::EpDescent);
            }
            let impact = story.ep_arrival.spec.impact_tick();
            if ticks >= impact && self.ep_ticks.is_none_or(|previous| previous < impact) {
                cues.push(Cue::EpImpact);
            }
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
        if matches!(story.stage, Stage::Encounter | Stage::Aftermath)
            && let Some(ticks) = story.ambient.accident_ticks()
        {
            // Render frames can span several fixed updates. Crossing a milestone
            // emits once even if no render sampled the exact simulation tick.
            for (at, cue) in [
                (0, Cue::TrafficEscape),
                (DOG_STARTLE_TICK, Cue::DogAlert),
                (CAR_HORN_TICK, Cue::CarHorn),
                (BICYCLE_FALL_TICK, Cue::BicycleFall),
                (CAR_SKID_TICK, Cue::CarSkid),
                (CAR_IMPACT_TICK, Cue::CarCrash),
                (SHUTTER_START_TICK, Cue::ShutterRoll),
                (SHUTTER_CLOSED_TICK, Cue::ShutterClack),
            ] {
                if ticks >= at && self.accident_ticks.is_none_or(|previous| previous < at) {
                    cues.push(cue);
                }
            }
        }
        self.stage = Some(story.stage);
        self.stage_ticks = story.stage_ticks;
        self.combat_ticks = story.combat.ticks;
        self.ambient_ticks = story.ambient.ticks();
        self.accident_ticks = story.ambient.accident_ticks();
        self.enemy_awake = story.combat.enemy_awake;
        self.ep_ticks = story.ep_arrival.ticks();
        cues
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        adventure::combat::{CombatInput, HitFeedback, Outcome},
        math::vec2::Vec2,
    };

    fn awake_street() -> Story {
        let mut story = Story::new();
        story.advance_scene();
        story.advance_scene();
        story.combat.enemy_awake = true;
        story.ambient.tick(true);
        story
    }

    #[test]
    fn ep_air_and_ground_cues_fire_once_and_panic_waits_for_the_impact() {
        let mut story = Story::new();
        story.advance_scene();
        story.advance_scene();
        story.skip_segment();
        let mut observed = ObservedAudio::at_story(&story);
        story.combat.player.position.x = crate::adventure::combat::ENCOUNTER_TRIGGER_X;
        story.tick(CombatInput::default());
        let cues = observed.observe(&story);
        assert!(cues.contains(&Cue::EpDescent));
        assert!(!cues.contains(&Cue::TrafficEscape));
        assert!(!cues.contains(&Cue::EpImpact));
        assert!(observed.observe(&story).is_empty());
        while !story.ep_arrival.impacted() {
            story.tick(CombatInput::default());
        }
        let cues = observed.observe(&story);
        assert!(cues.contains(&Cue::EpImpact));
        assert!(cues.contains(&Cue::TrafficEscape));
        assert!(!cues.contains(&Cue::EpDescent));
        assert!(observed.observe(&story).is_empty());
        story.skip_segment();
        observed = ObservedAudio::at_story(&story);
        assert!(observed.observe(&story).is_empty());
        story.combat.outcome = Outcome::Defeat;
        story.retry();
        assert!(!observed.observe(&story).contains(&Cue::EpImpact));
    }

    fn advance_accident_to(story: &mut Story, target: u32) {
        while story.ambient.accident_ticks().unwrap() < target {
            story.ambient.tick(true);
        }
    }

    #[test]
    fn traffic_cues_follow_milestones_once_across_uneven_render_updates() {
        let mut story = awake_street();
        let mut observed = ObservedAudio::at_story(&story);
        for (at, cue) in [
            (DOG_STARTLE_TICK, Cue::DogAlert),
            (CAR_HORN_TICK, Cue::CarHorn),
            (BICYCLE_FALL_TICK, Cue::BicycleFall),
            (CAR_SKID_TICK, Cue::CarSkid),
            (CAR_IMPACT_TICK, Cue::CarCrash),
            (SHUTTER_START_TICK, Cue::ShutterRoll),
            (SHUTTER_CLOSED_TICK, Cue::ShutterClack),
        ] {
            advance_accident_to(&mut story, at - 1);
            assert!(observed.observe(&story).is_empty());
            advance_accident_to(&mut story, at + 3);
            assert_eq!(observed.observe(&story), [cue]);
            assert!(observed.observe(&story).is_empty());
        }
        advance_accident_to(&mut story, CAR_IMPACT_TICK + 1000);
        assert!(observed.observe(&story).is_empty());
    }

    #[test]
    fn one_late_render_retains_all_crossed_traffic_milestones_in_order() {
        let mut story = awake_street();
        let mut observed = ObservedAudio::default();
        advance_accident_to(&mut story, CAR_IMPACT_TICK + 10);
        assert_eq!(
            observed.observe(&story),
            [
                Cue::Transition,
                Cue::TrafficEscape,
                Cue::DogAlert,
                Cue::CarHorn,
                Cue::BicycleFall,
                Cue::CarSkid,
                Cue::CarCrash
            ]
        );
        assert!(observed.observe(&story).is_empty());
    }

    #[test]
    fn collective_escape_starts_once_at_zero_and_does_not_loop_after_the_street_empties() {
        let mut story = awake_street();
        let mut observed = ObservedAudio::default();
        assert_eq!(story.ambient.accident_ticks(), Some(0));
        assert_eq!(
            observed.observe(&story),
            [Cue::Transition, Cue::TrafficEscape]
        );
        assert!(observed.observe(&story).is_empty());
        advance_accident_to(&mut story, BICYCLE_FALL_TICK);
        assert_eq!(
            observed.observe(&story),
            [Cue::DogAlert, Cue::CarHorn, Cue::BicycleFall]
        );
        advance_accident_to(&mut story, crate::adventure::ambient::STREET_EVACUATED_TICK);
        assert_eq!(
            observed.observe(&story),
            [
                Cue::CarSkid,
                Cue::CarCrash,
                Cue::ShutterRoll,
                Cue::ShutterClack
            ]
        );
        for _ in 0..3000 {
            story.ambient.tick(true);
            assert!(observed.observe(&story).is_empty());
        }
        story.combat.outcome = Outcome::Defeat;
        story.retry();
        assert_eq!(story.ambient.accident_ticks(), Some(0));
        assert_eq!(
            observed.observe(&story),
            [Cue::Transition, Cue::TrafficEscape]
        );
        assert!(observed.observe(&story).is_empty());
    }

    #[test]
    fn aftermath_keeps_the_pending_impact_without_replaying_the_horn() {
        let mut story = awake_street();
        advance_accident_to(&mut story, CAR_SKID_TICK);
        let mut observed = ObservedAudio::at_story(&story);
        story.stage = Stage::Aftermath;
        story.stage_ticks = 0;
        advance_accident_to(&mut story, CAR_IMPACT_TICK);
        assert_eq!(observed.observe(&story), [Cue::Transition, Cue::CarCrash]);
        assert!(observed.observe(&story).is_empty());
    }

    #[test]
    fn pause_preserves_pending_traffic_cues_until_resumed() {
        let mut audio = AdventureAudio::new(None);
        let mut story = awake_street();
        advance_accident_to(&mut story, CAR_HORN_TICK - 1);
        audio.update(&story, false);
        advance_accident_to(&mut story, CAR_HORN_TICK);
        for _ in 0..20 {
            audio.update(&story, true);
            assert_eq!(audio.observed.accident_ticks, Some(CAR_HORN_TICK - 1));
        }
        audio.update(&story, false);
        assert_eq!(audio.observed.accident_ticks, Some(CAR_HORN_TICK));
        assert!(audio.observed.observe(&story).is_empty());
    }

    #[test]
    fn retry_rearms_the_accident_and_discards_suspended_wreck_audio() {
        let mut audio = AdventureAudio::new(None);
        let mut story = awake_street();
        advance_accident_to(&mut story, CAR_IMPACT_TICK + 20);
        audio.update(&story, false);
        audio.update(&story, true);
        audio.suspended_sounds.push(Cue::CarCrash);
        audio.suspended_sounds.push(Cue::TrafficEscape);
        audio.suspended_sounds.push(Cue::BicycleFall);
        audio.suspended_sounds.push(Cue::ShutterRoll);
        audio.suspended_sounds.push(Cue::DogAlert);
        story.combat.outcome = Outcome::Defeat;
        story.retry();
        assert_eq!(story.ambient.accident_ticks(), Some(0));
        audio.update(&story, true);
        assert!(audio.suspended_sounds.is_empty());
        audio.update(&story, false);
        assert_eq!(audio.observed.accident_ticks, Some(0));
        assert!(audio.observed.observe(&story).is_empty());
        advance_accident_to(&mut story, CAR_HORN_TICK);
        assert_eq!(
            audio.observed.observe(&story),
            [Cue::DogAlert, Cue::CarHorn]
        );
        advance_accident_to(&mut story, CAR_IMPACT_TICK);
        assert_eq!(
            audio.observed.observe(&story),
            [Cue::BicycleFall, Cue::CarSkid, Cue::CarCrash]
        );
        story.restart();
        assert_eq!(audio.observed.observe(&story), [Cue::Transition]);
        assert_eq!(audio.observed.accident_ticks, None);
    }

    #[test]
    fn skip_abandons_pending_traffic_cues_and_suspended_sound() {
        let mut audio = AdventureAudio::new(None);
        let mut story = awake_street();
        advance_accident_to(&mut story, CAR_HORN_TICK);
        audio.update(&story, false);
        audio.update(&story, true);
        audio.suspended_sounds.push(Cue::CarHorn);
        advance_accident_to(&mut story, CAR_IMPACT_TICK);
        story.skip_segment();
        audio.sync_after_skip(&story);
        assert!(audio.paused);
        assert!(audio.suspended_sounds.is_empty());
        assert_eq!(audio.current_track, Some(Track::Opening));
        assert!(audio.observed.observe(&story).is_empty());
        audio.update(&story, false);
        assert!(audio.observed.observe(&story).is_empty());
    }

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
    fn natural_ambience_covers_morning_and_all_encounter_outcomes() {
        let mut story = Story::new();
        assert_eq!(background_for(&story), Track::Ada);
        story.advance_scene();
        assert_eq!(background_for(&story), Track::Morning);
        story.advance_scene();
        assert_eq!(background_for(&story), Track::Street);
        story.combat.enemy_awake = true;
        assert_eq!(background_for(&story), Track::Street);
        story.combat.outcome = Outcome::Defeat;
        assert_eq!(background_for(&story), Track::Street);
        story.stage = Stage::Aftermath;
        assert_eq!(background_for(&story), Track::Remorse);
    }

    #[test]
    fn traffic_plays_during_camera_arrival_before_combat_clock_starts() {
        let mut story = Story::new();
        story.advance_scene();
        story.advance_scene();
        let mut audio = AdventureAudio::new(None);
        for _ in 0..120 {
            story.tick(CombatInput::default());
            audio.update(&story, false);
            assert_eq!(story.combat.ticks, 0);
            assert_eq!(traffic_gain(&story), 1.0);
            assert!(audio.traffic_active);
            assert_eq!(audio.current_track, Some(Track::Street));
        }
        assert_eq!(story.ambient.ticks(), 120);
    }

    #[test]
    fn traffic_recedes_and_stays_silent_until_retry_without_silencing_the_air() {
        let mut story = awake_street();
        let mut audio = AdventureAudio::new(None);
        let mut previous = 1.0;
        for ticks in 0..=STREET_EVACUATED_TICK + 600 {
            advance_accident_to(&mut story, ticks);
            let gain = traffic_gain(&story);
            assert!(gain <= previous);
            previous = gain;
            audio.update(&story, false);
            assert_eq!(audio.traffic_active, ticks < STREET_EVACUATED_TICK);
            assert_eq!(audio.current_track, Some(Track::Street));
        }
        assert_eq!(traffic_gain(&story), 0.0);
        story.combat.outcome = Outcome::Defeat;
        story.retry();
        audio.update(&story, true);
        assert!(audio.paused);
        assert!(audio.traffic_active);
        assert_eq!(traffic_gain(&story), 1.0);
        audio.update(&story, false);
        assert_eq!(audio.observed.accident_ticks, Some(0));
    }

    #[test]
    fn shutter_pause_and_skip_preserve_only_future_closure_milestones() {
        let mut story = awake_street();
        advance_accident_to(&mut story, SHUTTER_START_TICK - 1);
        let mut audio = AdventureAudio::new(None);
        audio.update(&story, false);
        advance_accident_to(&mut story, SHUTTER_START_TICK + 1);
        audio.update(&story, true);
        assert_eq!(audio.observed.accident_ticks, Some(SHUTTER_START_TICK - 1));
        audio.update(&story, false);
        audio.update(&story, true);
        audio.suspended_sounds.push(Cue::ShutterRoll);
        advance_accident_to(&mut story, SHUTTER_CLOSED_TICK);
        audio.sync_after_skip(&story);
        assert!(audio.suspended_sounds.is_empty());
        audio.update(&story, false);
        assert!(audio.observed.observe(&story).is_empty());
        advance_accident_to(&mut story, STREET_EVACUATED_TICK);
        audio.update(&story, false);
        assert!(!audio.traffic_active);
        story.stage = Stage::Complete;
        audio.update(&story, false);
        assert!(!audio.traffic_active);
    }

    #[test]
    fn skipping_camera_keeps_calm_traffic_and_does_not_invent_a_panic() {
        let mut story = Story::new();
        story.advance_scene();
        story.advance_scene();
        story.tick(CombatInput::default());
        let mut audio = AdventureAudio::new(None);
        audio.update(&story, true);
        story.skip_segment();
        audio.sync_after_skip(&story);
        assert_eq!(story.stage, Stage::Encounter);
        assert!(!story.arrival_active());
        assert_eq!(story.combat.ticks, 0);
        assert!(audio.paused && audio.traffic_active);
        assert!(audio.observed.observe(&story).is_empty());
        assert_eq!(audio.observed.accident_ticks, None);
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

    #[test]
    fn skip_discards_suspended_contacts_and_keeps_the_new_scene_paused() {
        let mut audio = AdventureAudio::new(None);
        let mut story = Story::new();
        story.stage = Stage::Encounter;
        story.combat.ticks = 120;
        story.combat.last_hit = Some(HitFeedback {
            target: ActorKind::Erratic,
            position: Vec2::ZERO,
            blocked: false,
            damage: 12,
            age_ticks: 0,
        });
        audio.update(&story, true);
        audio.suspended_sounds.push(Cue::Strike);
        story.stage = Stage::Opening;
        story.stage_ticks = 19 * 60;
        audio.sync_after_skip(&story);
        assert!(audio.paused);
        assert!(audio.suspended_sounds.is_empty());
        assert_eq!(audio.current_track, Some(Track::Opening));
        assert!(audio.observed.observe(&story).is_empty());
        audio.update(&story, false);
        assert!(!audio.paused);
        assert!(audio.observed.observe(&story).is_empty());
    }
}
