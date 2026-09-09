//! Coordinates confirmed captures and frame-authored contacts for five supers.
//!
//! System: Match runtime. Ordinary updates yield to one sequence; renderer and
//! audio consume its clock while only this module changes captured health.

use super::*;
use crate::{
    audio::AudioCue,
    combat::super_sequence::{
        CPP_BARRAGE_CADENCE, CPP_BARRAGE_HITS, CPP_BARRAGE_START, CPP_CHARGE_START, CPP_ENTER_TICK,
        CPP_FINISHER_TICK, CPP_FOOTSHOT_TICK, CPP_REBOOT_TICK, CPP_TYPING_TICK, DUKE_CLONE_CADENCE,
        DUKE_CLONE_COUNT, DUKE_COLLECT_START, PYTHON_PEACE_START, PYTHON_TARGET_RETURN_TICK,
        RUST_ARENA_COMMIT_TICK, SUPER_FREEZE_FRAMES, SUPER_TARGET_LAND_END, SuperContact,
        SuperPhase, SuperSequence, super_spec,
    },
    config::{FIXED_TIMESTEP, FLOOR_Y},
};

impl World {
    /// Live capture and phase clock; None restores the ordinary match update.
    pub fn super_sequence(&self) -> Option<&SuperSequence> {
        self.super_sequence.as_ref()
    }

    /// True while input and normal gameplay are suspended by an authored super.
    pub fn super_sequence_active(&self) -> bool {
        self.super_sequence.is_some()
    }

    pub(super) fn try_start_super(&mut self, one: &mut FighterInput, two: &mut FighterInput) {
        let candidates = [
            one.cinematic_special
                && super_spec(self.player_one_character).is_some()
                && self.player_one.can_start_super(),
            two.cinematic_special
                && super_spec(self.player_two_character).is_some()
                && self.player_two.can_start_super(),
        ];
        // Authored requests never fall through to the old melee cinematic.
        if super_spec(self.player_one_character).is_some() {
            one.cinematic_special = false;
        }
        if super_spec(self.player_two_character).is_some() {
            two.cinematic_special = false;
        }
        if self.player_one.is_defeated()
            || self.player_two.is_defeated()
            || self.throw_sequence.is_some()
        {
            return;
        }
        let attacker = match candidates {
            [true, false] => PlayerSlot::One,
            [false, true] => PlayerSlot::Two,
            // Simultaneous eligible requests clash without choosing a privileged slot.
            _ => return,
        };
        let character = self.character_for_slot(attacker);
        let spec = super_spec(character).expect("candidate has authored spec");
        let (actor, target, target_input) = match attacker {
            PlayerSlot::One => (&mut self.player_one, &mut self.player_two, *two),
            PlayerSlot::Two => (&mut self.player_two, &mut self.player_one, *one),
        };
        actor.face_toward(target);
        let facing = actor.facing;
        let guarded = target_input.block || target.blocking;
        let target_crouching = target_input.crouch || target.crouching;
        let attacker_origin = feet_anchor(actor);
        let target_origin = feet_anchor(target);
        let destination = match facing {
            Facing::Right => target.body_rect().x - actor.body_rect().width - MIN_BODY_GAP,
            Facing::Left => target.body_rect().right() + MIN_BODY_GAP,
        }
        .clamp(ARENA_LEFT, ARENA_RIGHT - actor.body_rect().width);
        let sequence = SuperSequence {
            character,
            move_id: spec.move_id,
            label: spec.label,
            attacker,
            target: target.slot,
            guarded,
            target_crouching,
            tick: 0,
            duration_frames: spec.duration_frames,
            attacker_origin,
            target_origin,
            attacker_anchor: attacker_origin,
            target_anchor: target_origin,
            facing,
            fractional_ticks: 0.0,
            charge_destination_x: destination,
        };
        actor.begin_super_capture(false, false);
        target.begin_super_capture(guarded, target_crouching);
        self.super_sequence = Some(sequence);
        self.record_combat(CombatLogKind::CloseAttackStarted {
            slot: attacker,
            character,
            move_id: spec.move_id,
        });
        self.audio_events.push(
            AudioEvent::new(AudioCue::SuperStart)
                .with_fighter(attacker, character)
                .with_move(spec.move_id),
        );
    }

    pub(super) fn update_super_sequence(&mut self, dt: f32, flags: FeatureFlags) {
        let Some(mut sequence) = self.super_sequence.take() else {
            return;
        };
        sequence.fractional_ticks += dt.max(0.0) / FIXED_TIMESTEP;
        let steps = (sequence.fractional_ticks + 0.0001).floor() as u32;
        sequence.fractional_ticks = (sequence.fractional_ticks - steps as f32).max(0.0);
        let spec = super_spec(sequence.character).expect("running super has spec");
        for _ in 0..steps {
            sequence.tick += 1;
            if sequence.tick >= sequence.duration_frames {
                self.player_one.finish_super_capture();
                self.player_two.finish_super_capture();
                self.audio_events
                    .push(super_event(&sequence, AudioCue::SuperEnd));
                self.resolve_outcome();
                return;
            }
            if (SUPER_FREEZE_FRAMES..=SUPER_TARGET_LAND_END).contains(&sequence.tick)
                && sequence.target_origin.y < FLOOR_Y
            {
                // Capture brings airborne victims to the authored floor anchor
                // after the entry freeze; no one-frame teleport or floating rush.
                let progress = (sequence.tick - SUPER_FREEZE_FRAMES) as f32
                    / (SUPER_TARGET_LAND_END - SUPER_FREEZE_FRAMES) as f32;
                let feet_y =
                    sequence.target_origin.y + (FLOOR_Y - sequence.target_origin.y) * progress;
                let target = self.super_fighter_mut(sequence.target);
                target.position.y = feet_y - target.body_metrics().standing_height;
                target.grounded = sequence.tick == SUPER_TARGET_LAND_END;
            }
            self.player_one.advance_super_visuals(FIXED_TIMESTEP);
            self.player_two.advance_super_visuals(FIXED_TIMESTEP);
            if sequence.character == CharacterId::Cpp && sequence.tick >= CPP_CHARGE_START {
                let progress = (sequence.tick.saturating_sub(CPP_CHARGE_START) as f32
                    / (CPP_BARRAGE_START - CPP_CHARGE_START) as f32)
                    .clamp(0.0, 1.0);
                let actor = self.super_fighter_mut(sequence.attacker);
                let from = sequence.attacker_origin.x - actor.body_rect().width * 0.5;
                actor.position.x = from + (sequence.charge_destination_x - from) * progress;
                actor.clamp_to_arena();
            }
            if sequence.character == CharacterId::Rust && sequence.tick == RUST_ARENA_COMMIT_TICK {
                self.arena_override = Some(crate::game::arena::ArenaId::Sirius);
            }
            if sequence.character == CharacterId::Python
                && sequence.tick == PYTHON_TARGET_RETURN_TICK
            {
                self.super_fighter_mut(sequence.target)
                    .resume_super_target_reaction();
            }
            if let Some(cue) = phase_cue(&sequence) {
                self.audio_events.push(super_event(&sequence, cue));
            }
            for contact in spec
                .contacts
                .iter()
                .copied()
                .filter(|contact| contact.tick == sequence.tick)
            {
                self.apply_super_contact(&sequence, contact, flags);
            }
            sequence.attacker_anchor = feet_anchor(self.super_fighter(sequence.attacker));
            sequence.target_anchor = feet_anchor(self.super_fighter(sequence.target));
        }
        self.super_sequence = Some(sequence);
    }

    fn apply_super_contact(
        &mut self,
        sequence: &SuperSequence,
        contact: SuperContact,
        flags: FeatureFlags,
    ) {
        let takes_damage = flags.enabled(match sequence.target {
            PlayerSlot::One => FeatureFlag::PlayerOneTakesDamage,
            PlayerSlot::Two => FeatureFlag::PlayerTwoTakesDamage,
        });
        let target_character = self.character_for_slot(sequence.target);
        let target = self.super_fighter_mut(sequence.target);
        let before = target.health;
        let damage = if !takes_damage {
            0
        } else if sequence.guarded {
            (contact.damage / 4).max(1).min((before - 1).max(0))
        } else {
            contact.damage.min(before.max(0))
        };
        if takes_damage {
            target.receive_super_contact(damage, sequence.guarded, contact.knockdown);
            if sequence.character == CharacterId::Cpp
                && let Some(profile) =
                    crate::combat::super_sequence::cpp_barrage_reaction_profile(contact.tick)
            {
                target.record_super_contact_profile(profile, CPP_BARRAGE_CADENCE - 1);
            }
        }
        let impact_height = target.contact_reaction_state().map_or(0.45, |reaction| {
            use crate::combat::fighter::ContactReactionProfile;
            match reaction.profile {
                // Authored heads extend above the physical body; keep the spark
                // at the visible fist/foot contact without moving collision boxes.
                ContactReactionProfile::Head | ContactReactionProfile::GuardHigh => 0.06,
                ContactReactionProfile::Low | ContactReactionProfile::GuardLow => 0.78,
                _ => 0.45,
            }
        });
        let position = Vec2::new(
            target.body_rect().center_x(),
            target.body_rect().y + target.body_rect().height * impact_height,
        );
        let facing = target.facing;
        self.hit_effects
            .push(HitEffect::new(position, damage, sequence.guarded, facing));
        self.record_combat(CombatLogKind::CloseAttackResolved {
            attacker: sequence.attacker,
            defender: sequence.target,
            attacker_character: sequence.character,
            defender_character: target_character,
            move_id: sequence.move_id,
            damage,
            blocked: sequence.guarded,
        });
        if damage > 0 {
            self.audio_events.push(if sequence.guarded {
                AudioEvent::combat_block(sequence.attacker, sequence.character, sequence.move_id)
            } else {
                AudioEvent::combat_hit(sequence.attacker, sequence.character, sequence.move_id)
            });
            self.audio_events.push(if sequence.guarded {
                AudioEvent::fighter_block(sequence.target, target_character)
            } else {
                AudioEvent::fighter_hurt(sequence.target, target_character)
            });
        }
    }

    fn super_fighter(&self, slot: PlayerSlot) -> &Fighter {
        match slot {
            PlayerSlot::One => &self.player_one,
            PlayerSlot::Two => &self.player_two,
        }
    }
    fn super_fighter_mut(&mut self, slot: PlayerSlot) -> &mut Fighter {
        match slot {
            PlayerSlot::One => &mut self.player_one,
            PlayerSlot::Two => &mut self.player_two,
        }
    }
}

fn feet_anchor(fighter: &Fighter) -> Vec2 {
    Vec2::new(
        fighter.body_rect().center_x(),
        fighter.body_rect().bottom().min(FLOOR_Y),
    )
}
fn super_event(sequence: &SuperSequence, cue: AudioCue) -> AudioEvent {
    AudioEvent::new(cue)
        .with_fighter(sequence.attacker, sequence.character)
        .with_move(sequence.move_id)
}

fn phase_cue(sequence: &SuperSequence) -> Option<AudioCue> {
    use AudioCue::*;
    let tick = sequence.tick;
    match sequence.character {
        CharacterId::Rust if tick == 11 || tick == RUST_ARENA_COMMIT_TICK => Some(SuperMutation),
        CharacterId::Duke if tick == 8 => Some(SuperTrashRain),
        CharacterId::Duke
            if (DUKE_COLLECT_START..DUKE_COLLECT_START + DUKE_CLONE_COUNT * DUKE_CLONE_CADENCE)
                .contains(&tick)
                && (tick - DUKE_COLLECT_START).is_multiple_of(DUKE_CLONE_CADENCE) =>
        {
            Some(SuperCollect)
        }
        CharacterId::Duke
            if sequence.phase() == SuperPhase::DukeImpact
                && tick == sequence.phase_span().start =>
        {
            Some(SuperGiantDrop)
        }
        CharacterId::C if matches!(tick, 8 | 105) => Some(SuperError),
        CharacterId::C if tick == 180 => Some(SuperBoot),
        CharacterId::Cpp if tick == CPP_TYPING_TICK => Some(SuperTyping),
        CharacterId::Cpp if tick == CPP_ENTER_TICK => Some(SuperEnter),
        CharacterId::Cpp if tick == CPP_FOOTSHOT_TICK => Some(SuperFootshot),
        CharacterId::Cpp
            if matches!(
                sequence.phase(),
                SuperPhase::CTerminalStorm | SuperPhase::CBlueScreen
            ) && tick == sequence.phase_span().start =>
        {
            Some(SuperError)
        }
        CharacterId::Cpp
            if sequence.phase() == SuperPhase::CBios && tick == sequence.phase_span().start =>
        {
            Some(SuperBoot)
        }
        CharacterId::Cpp if tick == CPP_REBOOT_TICK => Some(SuperBarrageHit),
        CharacterId::Python
            if matches!(
                sequence.phase(),
                SuperPhase::PythonMorph | SuperPhase::PythonGrow
            ) && tick == sequence.phase_span().start =>
        {
            Some(SuperMutation)
        }
        CharacterId::Python if tick == PYTHON_PEACE_START => Some(SuperPeace),
        CharacterId::Python if tick == sequence.phase_span().start => match sequence.phase() {
            SuperPhase::PythonLunge => Some(SuperLunge),
            SuperPhase::PythonSwallow => Some(SuperSwallow),
            SuperPhase::PythonRevert => Some(SuperRevert),
            SuperPhase::PythonCelebrate => Some(SuperCelebrate),
            _ => None,
        },
        CharacterId::Cpp
            if ((CPP_BARRAGE_START
                ..CPP_BARRAGE_START + CPP_BARRAGE_HITS * CPP_BARRAGE_CADENCE)
                .contains(&tick)
                && (tick - CPP_BARRAGE_START).is_multiple_of(CPP_BARRAGE_CADENCE))
                || tick == CPP_FINISHER_TICK =>
        {
            Some(SuperBarrageHit)
        }
        _ => None,
    }
}
