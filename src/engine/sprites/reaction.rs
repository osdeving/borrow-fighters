//! Samples contact animations and deforms their silhouettes around the fighter.
//!
//! System: Sprite presentation. Stun, gravity and recovery remain authoritative
//! in Fighter; this module fits reviewed artwork to those clocks without moving boxes.

use std::f32::consts::PI;

use crate::{
    combat::fighter::{ContactReactionProfile, Facing, Fighter, HitReactionKind},
    config::world_px,
    math::vec2::Vec2,
};

use super::{
    FighterSpriteClip, SpriteFrame, SpriteManifest, fighter_clip_elapsed_seconds,
    frame_for_clip_at, frame_for_fighter_clip_at,
};

/// Visual-only displacement, squash/stretch and rotation, shared by all actors.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FighterVisualTransform {
    pub offset: Vec2,
    pub scale: Vec2,
    pub rotation_degrees: f32,
}

impl Default for FighterVisualTransform {
    fn default() -> Self {
        Self {
            offset: Vec2::ZERO,
            scale: Vec2::new(1.0, 1.0),
            rotation_degrees: 0.0,
        }
    }
}

/// Optional scene-authored placement of the actual fighter sprite around its feet.
/// Used by transformations such as Python's swallow without replacing the target art.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FighterVisualPlacement {
    pub anchor: Vec2,
    pub scale: f32,
    pub rotation_degrees: f32,
    pub opacity: f32,
}

/// Presentation arguments for drawing a fighter with an optional cinematic placement.
#[derive(Clone, Copy, Debug)]
pub struct FighterSpritePresentation {
    pub elapsed_seconds: f32,
    pub forced_clip: Option<FighterSpriteClip>,
    pub placement: Option<FighterVisualPlacement>,
}

/// Optional authored response selected from the confirmed contact, not the attack clock.
pub const fn contact_reaction_clip_name(profile: ContactReactionProfile) -> &'static str {
    match profile {
        ContactReactionProfile::Head => "reaction_head",
        ContactReactionProfile::Body => "reaction_body",
        ContactReactionProfile::Low => "reaction_low",
        ContactReactionProfile::GuardHigh => "reaction_guard_high",
        ContactReactionProfile::GuardLow => "reaction_guard_low",
        ContactReactionProfile::Launch => "reaction_launch",
        ContactReactionProfile::Fall => "reaction_fall",
        ContactReactionProfile::Rise => "reaction_rise",
    }
}

/// Fits the complete response to one contact window; a new contact starts on impact.
pub fn frame_for_contact_reaction<'a>(
    manifest: &'a SpriteManifest,
    fighter: &Fighter,
) -> Option<&'a SpriteFrame> {
    let contact = fighter.contact_reaction_state()?;
    let name = contact_reaction_clip_name(contact.profile);
    let source = manifest.clip_named(name)?;
    let duration = source
        .frames
        .iter()
        .filter_map(|name| manifest.frame_named(name))
        .map(|frame| frame.duration_ms as f32 / 1000.0)
        .sum::<f32>();
    // A throw holds the impact/lift key until World actually releases the target.
    let progress = if fighter.in_capture() {
        0.0
    } else {
        contact.progress()
    };
    frame_for_clip_at(manifest, name, progress.min(0.9999) * duration)
}

/// Samples the full reaction within actual stun, preserving authored pose durations.
/// Air poses follow ascent/apex/descent; knockdowns expose the complete get-up.
pub fn frame_for_fighter_state<'a>(
    manifest: &'a SpriteManifest,
    fighter: &Fighter,
    clip: FighterSpriteClip,
    world_elapsed_seconds: f32,
) -> Option<&'a SpriteFrame> {
    if let Some(frame) = frame_for_contact_reaction(manifest, fighter) {
        return Some(frame);
    }
    let elapsed = fighter_clip_elapsed_seconds(fighter, world_elapsed_seconds);
    let Some(reaction) = fighter.reaction_visual_state() else {
        return frame_for_fighter_clip_at(manifest, clip, elapsed);
    };
    let first = frame_for_fighter_clip_at(manifest, clip, 0.0)?;
    let source = manifest.clip_named(&first.clip)?;
    let total_seconds = source
        .frames
        .iter()
        .filter_map(|name| manifest.frame_named(name))
        .map(|frame| frame.duration_ms as f32 / 1000.0)
        .sum::<f32>();
    if reaction.kind == HitReactionKind::Knockdown && fighter.is_defeated() && !reaction.guarded {
        // Reviewed knockdown sheets hold the prone drawing longest. Some have
        // an extra seated impact key, so a fixed 100 ms offset is not portable.
        return source
            .frames
            .iter()
            .skip(1)
            .take(source.frames.len().saturating_sub(2))
            .filter_map(|name| manifest.frame_named(name))
            .max_by_key(|frame| frame.duration_ms)
            .or(Some(first));
    }
    let progress = reaction.progress();
    let mapped = if fighter.in_capture() {
        // Do not show an inverted flight pose before the throw releases.
        progress * 0.20
    } else if fighter.in_air_reaction() {
        let first_key = if reaction.kind == HitReactionKind::Thrown {
            0.23
        } else {
            0.0
        };
        first_key + progress * (0.96 - first_key)
    } else if reaction.kind == HitReactionKind::Knockdown && !reaction.guarded {
        if reaction.landed {
            0.18 + progress * 0.82
        } else {
            progress
        }
    } else {
        progress
    };
    frame_for_fighter_clip_at(manifest, clip, mapped.min(0.9999) * total_seconds)
}

/// Authored poses supply articulation; only a small contact-local translation is added.
pub fn contact_reaction_transform(fighter: &Fighter) -> FighterVisualTransform {
    let Some(contact) = fighter.contact_reaction_state() else {
        return FighterVisualTransform::default();
    };
    if matches!(
        contact.profile,
        ContactReactionProfile::Launch
            | ContactReactionProfile::Fall
            | ContactReactionProfile::Rise
    ) {
        return FighterVisualTransform::default();
    }
    let away = if fighter.facing == Facing::Right {
        -1.0
    } else {
        1.0
    };
    let recoil = (contact.progress() * PI).sin();
    FighterVisualTransform {
        offset: Vec2::new(away * 6.0 * contact.strength.min(1.6) * recoil, 0.0),
        ..FighterVisualTransform::default()
    }
}

/// Produces a recoil envelope that settles before control returns to the player.
/// Missing legacy airborne/ground poses get a rotating silhouette rather than idle.
pub fn fighter_reaction_transform(
    fighter: &Fighter,
    has_authored_clip: bool,
) -> FighterVisualTransform {
    let Some(reaction) = fighter.reaction_visual_state() else {
        return FighterVisualTransform::default();
    };
    let p = reaction.progress();
    let away = if fighter.facing == Facing::Right {
        -1.0
    } else {
        1.0
    };
    let strength = reaction.strength;
    let mut pose = FighterVisualTransform::default();
    if fighter.in_air_reaction() || fighter.in_capture() {
        let arc = (p * PI).sin();
        pose.scale = Vec2::new(1.0 - 0.055 * arc, 1.0 + 0.075 * arc);
        pose.rotation_degrees = away
            * if has_authored_clip {
                9.0 * (p * PI * 1.5).sin()
            } else {
                22.0 + 95.0 * p
            };
        return pose;
    }
    if reaction.kind == HitReactionKind::Knockdown && !reaction.guarded {
        let landing = (1.0 - p / 0.24).clamp(0.0, 1.0);
        pose.scale = Vec2::new(1.0 + 0.14 * landing, 1.0 - 0.15 * landing);
        if !has_authored_clip {
            let fall = (p / 0.22).clamp(0.0, 1.0);
            let recover = if fighter.is_defeated() {
                0.0
            } else {
                ((p - 0.58) / 0.42).clamp(0.0, 1.0)
            };
            pose.rotation_degrees = away
                * 88.0
                * (if reaction.landed || fighter.is_defeated() {
                    1.0
                } else {
                    fall
                })
                * (1.0 - recover);
        }
        return pose;
    }
    // Fast contact compression, broad recoil, then a smaller counter-recoil.
    // Even a 10-frame jab changes silhouette on its first two display frames.
    let compression = (1.0 - p / 0.28).clamp(0.0, 1.0);
    let recoil = (p * PI).sin() * (1.0 - p).sqrt();
    let settle = (p * PI * 2.0).sin() * (1.0 - p);
    let guarded = if reaction.guarded { 0.45 } else { 1.0 };
    pose.offset.x = away * world_px(11.0) * strength * guarded * recoil;
    pose.offset.y = if reaction.crouched && !reaction.guarded {
        world_px(12.0) * recoil
    } else {
        -world_px(3.0) * recoil * strength
    };
    pose.scale.x = 1.0 + 0.09 * compression * strength * guarded;
    pose.scale.y = 1.0 - 0.075 * compression * strength * guarded;
    pose.rotation_degrees = away * (7.0 * recoil + 2.0 * settle) * strength * guarded;
    pose
}
