//! Plays quiet street ambience, physical Foley and phone cues for the chapter.
//!
//! System: Adventure chapter audio boundary. A pure snapshot observer follows
//! simulation clocks; optional Raylib resources retain their device lifetime.

use raylib::prelude::{Music, RaylibAudio, Sound};

use crate::{
    adventure::{
        chapter::{
            Chapter, Phase, SHOP_EXIT_CLEARANCE_TICKS, SHOP_SHUTTER_TICKS,
            phone::{
                PHONE_FIRST_SENT_TICK, PHONE_LAST_SENT_TICK, PHONE_REPLY_TICK, PHONE_STOW_TICK,
            },
        },
        combat::{Action, ActorKind},
    },
    math::vec2::Vec2,
    runtime_paths::asset_path,
};

const VOLUME: f32 = 0.3;
// Existing Rust walk animation: four poses of seven ticks, two foot contacts.
const STEP_TICKS: u32 = 14;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Cue {
    Pocket,
    Tap,
    Send,
    Receive,
    Step,
    ShutterRoll,
    ShutterClack,
    Strike,
    Block,
    Hurt,
}

impl Cue {
    const ALL: [Self; 10] = [
        Self::Pocket,
        Self::Tap,
        Self::Send,
        Self::Receive,
        Self::Step,
        Self::ShutterRoll,
        Self::ShutterClack,
        Self::Strike,
        Self::Block,
        Self::Hurt,
    ];

    fn path(self) -> &'static str {
        match self {
            Self::Pocket => "assets/adventure/chapter/audio/phone_pocket.wav",
            Self::Tap => "assets/adventure/chapter/audio/phone_tap.wav",
            Self::Send => "assets/adventure/chapter/audio/phone_send.wav",
            Self::Receive => "assets/adventure/chapter/audio/phone_receive.wav",
            Self::Step => "assets/adventure/chapter/audio/footstep.wav",
            Self::ShutterRoll => "assets/adventure/audio/shutter_roll.wav",
            Self::ShutterClack => "assets/adventure/audio/shutter_clack.wav",
            Self::Strike => "assets/adventure/audio/strike.wav",
            Self::Block => "assets/adventure/audio/block.wav",
            Self::Hurt => "assets/adventure/audio/hurt.wav",
        }
    }
}

const PHONE_CUES: [(u32, Cue); 11] = [
    (0, Cue::Pocket),
    (PHONE_FIRST_SENT_TICK - 100, Cue::Tap),
    (PHONE_FIRST_SENT_TICK - 64, Cue::Tap),
    (PHONE_FIRST_SENT_TICK - 26, Cue::Tap),
    (PHONE_FIRST_SENT_TICK, Cue::Send),
    (PHONE_REPLY_TICK, Cue::Receive),
    (PHONE_LAST_SENT_TICK - 94, Cue::Tap),
    (PHONE_LAST_SENT_TICK - 60, Cue::Tap),
    (PHONE_LAST_SENT_TICK - 24, Cue::Tap),
    (PHONE_LAST_SENT_TICK, Cue::Send),
    (PHONE_STOW_TICK, Cue::Pocket),
];

/// Optional chapter audio, with no traffic stream or musical soundtrack.
pub struct ChapterAudio<'aud> {
    air: Option<Music<'aud>>,
    sounds: Vec<(Cue, Sound<'aud>)>,
    started: bool,
    paused: bool,
    suspended: Vec<Cue>,
    observed: Observed,
}

impl<'aud> ChapterAudio<'aud> {
    /// Loads the small phone/Foley set and reuses existing adventure assets.
    pub fn new(device: Option<&'aud RaylibAudio>) -> Self {
        let mut audio = Self {
            air: None,
            sounds: Vec::new(),
            started: false,
            paused: false,
            suspended: Vec::new(),
            observed: Observed::default(),
        };
        let Some(device) = device else { return audio };
        let path = asset_path("assets/adventure/audio/street_air.wav");
        if path.is_file()
            && let Ok(mut air) = device.new_music(&path.to_string_lossy())
        {
            air.set_looping(true);
            air.set_volume(VOLUME);
            audio.air = Some(air);
        }
        for cue in Cue::ALL {
            let path = asset_path(cue.path());
            if path.is_file()
                && let Ok(sound) = device.new_sound(&path.to_string_lossy())
            {
                sound.set_volume(VOLUME);
                if cue == Cue::ShutterRoll {
                    // The original roll is one second. Slow it to the same
                    // fixed-clock interval as the chapter's physical shutter.
                    sound.set_pitch(60.0 / SHOP_SHUTTER_TICKS as f32);
                }
                audio.sounds.push((cue, sound));
            }
        }
        audio
    }

    /// Observes a rendered state; repeated renders never repeat an event.
    pub fn update(&mut self, chapter: &Chapter, paused: bool) {
        self.update_frame(Frame::from(chapter), paused);
    }

    /// Discards abandoned effects after skip, retry or loading a checkpoint.
    ///
    /// The host calls this immediately after replacing/repositioning chapter
    /// state. Ordinary phase transitions use `update`, so their cues still play.
    /// Pause remains active; only future milestones can sound after resuming.
    pub fn synchronize(&mut self, chapter: &Chapter) {
        self.synchronize_frame(Frame::from(chapter));
    }

    fn update_frame(&mut self, frame: Frame, paused: bool) {
        if self.observed.restarted(frame) {
            self.stop_effects();
        }
        self.start_air(paused);
        self.set_paused(paused);
        if paused {
            return;
        }
        if let Some(air) = &self.air {
            air.update_stream();
        }
        for cue in self.observed.observe(frame) {
            if let Some((_, sound)) = self.sounds.iter().find(|(id, _)| *id == cue) {
                sound.play();
            }
        }
    }

    fn synchronize_frame(&mut self, frame: Frame) {
        self.stop_effects();
        self.observed = Observed::at(frame);
        self.start_air(self.paused);
        if let Some(air) = &self.air {
            let duration = air.get_time_length();
            if duration.is_finite() && duration > 0.0 {
                air.seek_stream((frame.ticks as f32 / 60.0).rem_euclid(duration));
            }
        }
    }

    fn start_air(&mut self, paused: bool) {
        if self.started {
            return;
        }
        if let Some(air) = &self.air {
            air.set_volume(if paused { 0.0 } else { VOLUME });
            air.play_stream();
            if paused {
                air.pause_stream();
            }
            air.set_volume(VOLUME);
        }
        self.started = true;
    }

    fn stop_effects(&mut self) {
        for (_, sound) in &self.sounds {
            sound.stop();
        }
        self.suspended.clear();
    }

    fn set_paused(&mut self, paused: bool) {
        if self.paused == paused {
            return;
        }
        self.paused = paused;
        if let Some(air) = &self.air {
            if paused {
                air.pause_stream();
            } else {
                air.resume_stream();
            }
        }
        if paused {
            self.suspended.clear();
            for (cue, sound) in &self.sounds {
                if sound.is_playing() {
                    sound.pause();
                    self.suspended.push(*cue);
                }
            }
        } else {
            for cue in self.suspended.drain(..) {
                if let Some((_, sound)) = self.sounds.iter().find(|(id, _)| *id == cue) {
                    sound.resume();
                }
            }
        }
    }
}

#[derive(Clone, Copy)]
struct Frame {
    ticks: u32,
    phase: Phase,
    phase_ticks: u32,
    phone_ticks: Option<u32>,
    walk_ticks: Option<u32>,
    position: Vec2,
    contact: Option<(u32, Cue)>,
}

impl From<&Chapter> for Frame {
    fn from(chapter: &Chapter) -> Self {
        let player = &chapter.combat.player;
        Self {
            ticks: chapter.ticks,
            phase: chapter.phase,
            phase_ticks: chapter.phase_ticks,
            phone_ticks: chapter.phone().map(|phone| phone.ticks),
            walk_ticks: (player.action == Action::Walk && player.grounded)
                .then_some(player.action_ticks),
            position: player.position,
            contact: chapter
                .combat
                .last_hit
                .filter(|hit| hit.age_ticks <= 1)
                .map(|hit| {
                    let cue = if hit.blocked {
                        Cue::Block
                    } else if hit.target == ActorKind::Player {
                        Cue::Hurt
                    } else {
                        Cue::Strike
                    };
                    (chapter.combat.ticks.saturating_sub(hit.age_ticks), cue)
                }),
        }
    }
}

#[derive(Default)]
struct Observed {
    previous: Option<Frame>,
    contact_tick: Option<u32>,
}

impl Observed {
    fn at(frame: Frame) -> Self {
        Self {
            previous: Some(frame),
            contact_tick: frame.contact.map(|(tick, _)| tick),
        }
    }

    fn restarted(&self, frame: Frame) -> bool {
        self.previous.is_some_and(|previous| {
            frame.ticks < previous.ticks
                || (frame.phase == previous.phase && frame.phase_ticks < previous.phase_ticks)
        })
    }

    fn observe(&mut self, frame: Frame) -> Vec<Cue> {
        if self.restarted(frame) {
            // A restored chapter is a new baseline, not an invitation to play
            // every gesture that happened before the restored checkpoint.
            *self = Self::at(frame);
            return Vec::new();
        }
        let mut cues = Vec::new();
        if let Some(ticks) = frame.phone_ticks {
            let previous = self.previous.and_then(|p| p.phone_ticks);
            for (at, cue) in PHONE_CUES {
                if ticks >= at && previous.is_none_or(|tick| tick < at) {
                    cues.push(cue);
                }
            }
        }
        if matches!(frame.phase, Phase::ShopApproach | Phase::ShopReturn) {
            let previous = self.previous.filter(|p| p.phase == frame.phase);
            let start = if frame.phase == Phase::ShopReturn {
                SHOP_EXIT_CLEARANCE_TICKS
            } else {
                0
            };
            if frame.phase_ticks >= start && previous.is_none_or(|p| p.phase_ticks < start) {
                cues.push(Cue::ShutterRoll);
            }
            if frame.phase == Phase::ShopReturn
                && frame.phase_ticks >= start + SHOP_SHUTTER_TICKS
                && previous.is_none_or(|p| p.phase_ticks < start + SHOP_SHUTTER_TICKS)
            {
                cues.push(Cue::ShutterClack);
            }
        } else if frame.phase == Phase::ExploreNeighbour
            && self.previous.is_some_and(|p| {
                p.phase == Phase::ShopReturn
                    && p.phase_ticks < SHOP_EXIT_CLEARANCE_TICKS + SHOP_SHUTTER_TICKS
            })
        {
            // The fixed update closes the shutter and changes phase together,
            // so its final ShopReturn tick is never exposed to the adapter.
            // Explicit skips synchronize first, discarding this transition.
            cues.push(Cue::ShutterClack);
        }
        if let Some(walk_ticks) = frame.walk_ticks
            && let Some(previous) = self.previous
            && (frame.position.x - previous.position.x).abs()
                + (frame.position.y - previous.position.y).abs()
                > 0.1
            && previous.walk_ticks.is_none_or(|ticks| {
                walk_ticks < ticks || walk_ticks / STEP_TICKS > ticks / STEP_TICKS
            })
        {
            // Coalesce missed foot contacts into one audible step, avoiding
            // stacked footsteps after a slow render; stationary actors stay quiet.
            cues.push(Cue::Step);
        }
        if let Some((tick, cue)) = frame.contact
            && self.contact_tick != Some(tick)
        {
            cues.push(cue);
            self.contact_tick = Some(tick);
        }
        self.previous = Some(frame);
        cues
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn frame(phase: Phase, phase_ticks: u32) -> Frame {
        Frame {
            ticks: 1000,
            phase,
            phase_ticks,
            phone_ticks: None,
            walk_ticks: None,
            position: Vec2::ZERO,
            contact: None,
        }
    }

    fn phone(ticks: u32) -> Frame {
        Frame {
            phone_ticks: Some(ticks),
            ..frame(Phase::Phone, ticks)
        }
    }

    #[test]
    fn phone_events_cross_uneven_renders_once_in_authored_order() {
        let mut observed = Observed::default();
        let mut emitted = Vec::new();
        for ticks in [0, 113, 190, 270, 519, 700, 725, 811, 860] {
            let state = phone(ticks);
            emitted.extend(observed.observe(state));
            assert!(observed.observe(state).is_empty());
        }
        assert_eq!(emitted, PHONE_CUES.map(|(_, cue)| cue));
    }

    #[test]
    fn waiting_and_python_typing_do_not_make_rust_keyboard_noises() {
        let mut observed = Observed::at(phone(PHONE_FIRST_SENT_TICK));
        for ticks in PHONE_FIRST_SENT_TICK..PHONE_REPLY_TICK {
            assert!(observed.observe(phone(ticks)).is_empty());
        }
        assert_eq!(observed.observe(phone(PHONE_REPLY_TICK)), [Cue::Receive]);
    }

    #[test]
    fn pause_defers_the_message_without_consuming_or_replaying_it() {
        let mut audio = ChapterAudio::new(None);
        audio.update_frame(phone(PHONE_FIRST_SENT_TICK - 1), false);
        for _ in 0..10 {
            audio.update_frame(phone(PHONE_FIRST_SENT_TICK), true);
        }
        assert_eq!(
            audio.observed.previous.unwrap().phone_ticks,
            Some(PHONE_FIRST_SENT_TICK - 1)
        );
        audio.update_frame(phone(PHONE_FIRST_SENT_TICK), false);
        assert!(
            audio
                .observed
                .observe(phone(PHONE_FIRST_SENT_TICK))
                .is_empty()
        );
    }

    #[test]
    fn skip_while_paused_discards_voices_and_only_future_phone_events_remain() {
        let mut audio = ChapterAudio::new(None);
        audio.update_frame(phone(120), true);
        audio.suspended.extend([Cue::Tap, Cue::Pocket]);
        audio.synchronize_frame(phone(PHONE_REPLY_TICK));
        assert!(audio.paused && audio.suspended.is_empty());
        assert!(audio.observed.observe(phone(PHONE_REPLY_TICK)).is_empty());
        assert_eq!(
            audio.observed.observe(phone(PHONE_LAST_SENT_TICK)),
            [Cue::Tap, Cue::Tap, Cue::Tap, Cue::Send]
        );
    }

    #[test]
    fn retry_synchronizes_combat_without_replaying_phone_or_old_contact() {
        let mut audio = ChapterAudio::new(None);
        audio.update_frame(phone(PHONE_STOW_TICK), false);
        audio.update_frame(phone(PHONE_STOW_TICK), true);
        audio.suspended.push(Cue::Pocket);
        let mut combat = frame(Phase::PassageCombat, 0);
        combat.contact = Some((3, Cue::Block));
        audio.synchronize_frame(combat);
        assert!(audio.suspended.is_empty());
        audio.update_frame(combat, false);
        assert!(audio.observed.observe(combat).is_empty());
        combat.ticks += 10;
        combat.phase_ticks += 10;
        combat.contact = Some((13, Cue::Strike));
        assert_eq!(audio.observed.observe(combat), [Cue::Strike]);
    }

    #[test]
    fn clock_rollback_does_not_burst_old_conversation_effects() {
        let mut observed = Observed::at(phone(PHONE_STOW_TICK));
        assert!(observed.observe(phone(PHONE_REPLY_TICK - 1)).is_empty());
        assert_eq!(observed.observe(phone(PHONE_REPLY_TICK)), [Cue::Receive]);
        assert!(observed.observe(phone(PHONE_REPLY_TICK)).is_empty());
    }

    #[test]
    fn shutter_rolls_on_each_physical_move_and_only_clacks_when_closed() {
        let mut observed = Observed::at(frame(Phase::ExploreShop, 0));
        assert_eq!(
            observed.observe(frame(Phase::ShopApproach, 0)),
            [Cue::ShutterRoll]
        );
        assert!(
            observed
                .observe(frame(Phase::ShopApproach, SHOP_SHUTTER_TICKS))
                .is_empty()
        );
        assert!(observed.observe(frame(Phase::ShopDialogue, 0)).is_empty());
        assert!(observed.observe(frame(Phase::ShopReturn, 2)).is_empty());
        assert!(
            observed
                .observe(frame(Phase::ShopReturn, SHOP_EXIT_CLEARANCE_TICKS - 1))
                .is_empty()
        );
        assert_eq!(
            observed.observe(frame(Phase::ShopReturn, SHOP_EXIT_CLEARANCE_TICKS + 2)),
            [Cue::ShutterRoll]
        );
        assert_eq!(
            observed.observe(frame(
                Phase::ShopReturn,
                SHOP_EXIT_CLEARANCE_TICKS + SHOP_SHUTTER_TICKS + 2
            )),
            [Cue::ShutterClack]
        );
        assert!(
            observed
                .observe(frame(
                    Phase::ShopReturn,
                    SHOP_EXIT_CLEARANCE_TICKS + SHOP_SHUTTER_TICKS + 2
                ))
                .is_empty()
        );
    }

    #[test]
    fn synchronized_open_door_does_not_replay_its_roll() {
        let state = frame(Phase::ShopApproach, SHOP_SHUTTER_TICKS);
        let mut audio = ChapterAudio::new(None);
        audio.synchronize_frame(state);
        assert!(audio.observed.observe(state).is_empty());
    }

    fn returning_from_shop() -> Chapter {
        use crate::adventure::chapter::{ChapterInput, Checkpoint, CheckpointStage, World};
        let mut checkpoint = Checkpoint::new(false);
        checkpoint.stage = CheckpointStage::DriverChecked;
        let mut chapter = Chapter::from_checkpoint(World::bundled(), checkpoint).unwrap();
        chapter.combat.player.position = Vec2::new(550.0, 580.0);
        chapter.tick(ChapterInput {
            interact: true,
            ..Default::default()
        });
        for _ in 0..180 {
            chapter.tick(ChapterInput::default());
        }
        assert_eq!(chapter.phase, Phase::ShopDialogue);
        for _ in 0..3 {
            chapter.tick(ChapterInput {
                advance: true,
                ..Default::default()
            });
        }
        assert_eq!(chapter.phase, Phase::ShopReturn);
        chapter
    }

    #[test]
    fn natural_shutter_contact_survives_the_same_tick_phase_transition() {
        use crate::adventure::chapter::ChapterInput;
        let mut chapter = returning_from_shop();
        let mut observed = Observed::at(Frame::from(&chapter));
        let mut shutter = Vec::new();
        for _ in 0..SHOP_EXIT_CLEARANCE_TICKS + SHOP_SHUTTER_TICKS {
            assert_eq!(chapter.phase, Phase::ShopReturn);
            chapter.tick(ChapterInput::default());
            shutter.extend(
                observed
                    .observe(Frame::from(&chapter))
                    .into_iter()
                    .filter(|cue| matches!(cue, Cue::ShutterRoll | Cue::ShutterClack)),
            );
        }
        assert_eq!(chapter.phase, Phase::ExploreNeighbour);
        assert_eq!(chapter.phase_ticks, 0);
        assert_eq!(shutter, [Cue::ShutterRoll, Cue::ShutterClack]);
        assert!(observed.observe(Frame::from(&chapter)).is_empty());
    }

    #[test]
    fn skipped_shutter_transition_does_not_play_an_abandoned_clack() {
        use crate::adventure::chapter::ChapterInput;
        for age in [
            SHOP_EXIT_CLEARANCE_TICKS - 1,
            SHOP_EXIT_CLEARANCE_TICKS + 1,
            SHOP_EXIT_CLEARANCE_TICKS + SHOP_SHUTTER_TICKS - 1,
        ] {
            let mut chapter = returning_from_shop();
            let mut audio = ChapterAudio::new(None);
            for _ in 0..age {
                chapter.tick(ChapterInput::default());
                audio.update(&chapter, false);
            }
            audio.suspended.push(Cue::ShutterRoll);
            chapter.tick(ChapterInput {
                skip: true,
                ..Default::default()
            });
            assert_eq!(chapter.phase, Phase::ExploreNeighbour);
            audio.synchronize(&chapter);
            assert!(audio.suspended.is_empty());
            assert!(audio.observed.observe(Frame::from(&chapter)).is_empty());
        }
    }

    #[test]
    fn footfalls_follow_motion_and_stride_without_stacking_after_a_slow_render() {
        let mut state = frame(Phase::ExploreDriver, 0);
        let mut observed = Observed::at(state);
        state.walk_ticks = Some(1);
        state.position.x = 2.0;
        assert_eq!(observed.observe(state), [Cue::Step]);
        assert!(observed.observe(state).is_empty());
        state.walk_ticks = Some(STEP_TICKS * 5);
        assert!(
            observed.observe(state).is_empty(),
            "walking into a wall stays quiet"
        );
        state.walk_ticks = Some(STEP_TICKS * 9);
        state.position.x = 60.0;
        assert_eq!(observed.observe(state), [Cue::Step]);
        state.walk_ticks = None;
        state.position.y -= 20.0;
        assert!(
            observed.observe(state).is_empty(),
            "airborne movement makes no footfall"
        );
    }

    #[test]
    fn contacts_are_unique_even_when_rendered_more_than_once() {
        let mut state = frame(Phase::PassageCombat, 20);
        state.contact = Some((20, Cue::Hurt));
        let mut observed = Observed::default();
        assert_eq!(observed.observe(state), [Cue::Hurt]);
        assert!(observed.observe(state).is_empty());
        state.contact = None;
        observed.observe(state);
        state.contact = Some((35, Cue::Block));
        assert_eq!(observed.observe(state), [Cue::Block]);
    }
}
