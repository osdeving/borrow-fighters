//! Selects sprite frames from clips over time.
//!
//! Animation timing is independent of Raylib so clips can be validated and
//! tested without opening a window.

use crate::engine::sprites::manifest::{SpriteFrame, SpriteManifest};

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
