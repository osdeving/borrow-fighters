//! Selects sprite frames from clips over time.
//!
//! Animation timing is independent of Raylib so clips can be validated and
//! tested without opening a window.

use crate::engine::sprites::manifest::{SpriteFrame, SpriteManifest};
use crate::engine::sprites::selection::FighterSpriteClip;

/// Resolves presentation clips with explicit compatibility fallbacks.
///
/// Fallback poses are visual only: combat metadata must use the requested clip
/// directly so an older atlas cannot change attack reach through an alias.
pub fn frame_for_fighter_clip_at(
    manifest: &SpriteManifest,
    clip: FighterSpriteClip,
    elapsed_seconds: f32,
) -> Option<&SpriteFrame> {
    let fallback = match clip {
        FighterSpriteClip::Victory => &["victory", "taunt", "idle"][..],
        FighterSpriteClip::Defeat => &["defeat", "hit", "idle"],
        FighterSpriteClip::Spawn => &["spawn", "idle"],
        FighterSpriteClip::CrouchBlock => &["crouch_block", "block", "idle"],
        FighterSpriteClip::Sweep | FighterSpriteClip::AirKick => &[clip.as_str(), "kick", "idle"],
        FighterSpriteClip::Overhead | FighterSpriteClip::AntiAir => {
            &[clip.as_str(), "punch_heavy", "idle"]
        }
        FighterSpriteClip::AirPunch | FighterSpriteClip::Throw => {
            &[clip.as_str(), "punch_light", "idle"]
        }
        _ => &[clip.as_str(), "idle"],
    };
    fallback
        .iter()
        .find_map(|name| frame_for_clip_at(manifest, name, elapsed_seconds))
}

/// Returns the frame for a clip at the given elapsed time.
pub fn frame_for_clip_at<'a>(
    manifest: &'a SpriteManifest,
    clip_name: &str,
    elapsed_seconds: f32,
) -> Option<&'a SpriteFrame> {
    let clip = manifest.clip_named(clip_name)?;
    let total_ms = clip
        .frames
        .iter()
        .filter_map(|name| manifest.frame_named(name))
        .map(|frame| frame.duration_ms as f32)
        .sum::<f32>();

    if total_ms <= 0.0 {
        return None;
    }

    let mut remaining_ms = elapsed_seconds.max(0.0) * 1000.0;
    if clip.r#loop {
        remaining_ms %= total_ms;
    } else if remaining_ms >= total_ms {
        return clip
            .frames
            .last()
            .and_then(|frame_name| manifest.frame_named(frame_name));
    }

    for frame_name in &clip.frames {
        let frame = manifest.frame_named(frame_name)?;
        let duration = frame.duration_ms as f32;
        if remaining_ms < duration {
            return Some(frame);
        }
        remaining_ms -= duration;
    }

    clip.frames
        .last()
        .and_then(|frame_name| manifest.frame_named(frame_name))
}
