//! Describes contact-specific reaction drawings for the Python/C++ animation pilot.
//!
//! System: Fighter presentation state. Profiles and visual recovery clocks do not
//! change damage, stun, physical movement, hitboxes or hurtboxes.

use super::{AttackKind, Fighter, FrameCount, GuardRule, MoveId};

/// The anatomical response or recovery phase shown by an authored reaction clip.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContactReactionProfile {
    Head,
    Body,
    Low,
    GuardHigh,
    GuardLow,
    Launch,
    Fall,
    Rise,
}

/// A contact-local visual clock, independent of the remaining gameplay stun.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ContactReactionState {
    pub profile: ContactReactionProfile,
    pub elapsed_seconds: f32,
    pub duration_seconds: f32,
    pub strength: f32,
    pub cinematic: bool,
}

impl ContactReactionState {
    /// Starts at the impact drawing and reaches recovery before the next contact.
    pub fn progress(self) -> f32 {
        (self.elapsed_seconds / self.duration_seconds.max(1.0 / 60.0)).clamp(0.0, 1.0)
    }
}

impl Fighter {
    /// Whether this fighter participates in the Python/C++ authored reaction pilot.
    pub fn uses_contact_reactions(&self) -> bool {
        self.move_ids.contains(&MoveId::PythonEventHorizon)
            || self.move_ids.contains(&MoveId::CppTemplateSingularity)
    }

    /// Returns the precise impact, flight, fall or rise phase for the current contact.
    /// Other fighters continue using the existing `ReactionVisualState` contract.
    pub fn contact_reaction_state(&self) -> Option<ContactReactionState> {
        if !self.uses_contact_reactions() || !self.is_reacting() {
            return None;
        }
        let mut state = ContactReactionState {
            profile: self.contact_reaction_profile,
            elapsed_seconds: self.reaction_visual_elapsed,
            duration_seconds: self.contact_reaction_duration,
            strength: self.reaction_strength,
            cinematic: self.super_reaction,
        };
        if self.in_air_reaction() || self.in_capture() {
            state.profile = ContactReactionProfile::Launch;
            state.duration_seconds = self.reaction_visual_duration;
        } else if self.in_knockdown() {
            let duration = self.reaction_visual_duration;
            let low_impact = if !self.reaction_landed
                && self.contact_reaction_profile == ContactReactionProfile::Low
            {
                FrameCount::new(6).as_seconds()
            } else {
                0.0
            };
            let fall_end = duration * 0.58;
            if self.is_defeated() {
                state.profile = ContactReactionProfile::Fall;
                state.elapsed_seconds = duration;
                state.duration_seconds = duration;
            } else if state.elapsed_seconds < low_impact {
                state.profile = ContactReactionProfile::Low;
                state.duration_seconds = low_impact;
            } else if state.elapsed_seconds < fall_end {
                state.profile = ContactReactionProfile::Fall;
                state.elapsed_seconds -= low_impact;
                state.duration_seconds = fall_end - low_impact;
            } else {
                state.profile = ContactReactionProfile::Rise;
                state.elapsed_seconds -= fall_end;
                state.duration_seconds = duration - fall_end;
            }
        }
        Some(state)
    }

    pub(super) fn record_contact_reaction(&mut self, rule: GuardRule, blocked: bool) {
        self.contact_reaction_profile = if blocked {
            if self.reaction_was_crouching {
                ContactReactionProfile::GuardLow
            } else {
                ContactReactionProfile::GuardHigh
            }
        } else if self.in_air_reaction() || self.in_capture() {
            ContactReactionProfile::Launch
        } else if self.reaction_was_crouching || rule == GuardRule::Low {
            ContactReactionProfile::Low
        } else if rule == GuardRule::High {
            ContactReactionProfile::Head
        } else {
            ContactReactionProfile::Body
        };
        self.contact_reaction_duration = self.reaction_visual_duration;
    }

    pub(crate) fn record_close_contact_profile(&mut self, kind: AttackKind) {
        if !self.is_reacting() || self.in_blockstun() || self.in_air_reaction() {
            return;
        }
        self.contact_reaction_profile = match kind {
            AttackKind::Overhead | AttackKind::AirPunch | AttackKind::AirKick => {
                ContactReactionProfile::Head
            }
            AttackKind::Sweep => ContactReactionProfile::Low,
            AttackKind::AntiAir | AttackKind::Throw => ContactReactionProfile::Launch,
            _ => ContactReactionProfile::Body,
        };
    }

    /// Shortens only authored presentation so repeated contacts can complete recoil.
    pub(crate) fn record_super_contact_profile(
        &mut self,
        profile: ContactReactionProfile,
        visual_frames: u32,
    ) {
        if !self.uses_contact_reactions() {
            return;
        }
        if !self.in_blockstun() && !self.in_air_reaction() && !self.in_knockdown() {
            self.contact_reaction_profile = profile;
        }
        self.contact_reaction_duration = visual_frames as f32 * crate::config::FIXED_TIMESTEP;
    }
}
