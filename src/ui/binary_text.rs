//! Generates menu text variants for the binary reveal effect.
//!
//! The helper is deterministic and independent from Raylib so any menu can use
//! it later without coupling scene navigation to rendering code.

pub const DEFAULT_BINARY_REVEAL_FRAMES: u16 = 24;

/// Returns `text` with a shrinking set of alphanumeric glyphs replaced by `0`
/// and `1` characters.
pub fn binary_reveal_text(text: &str, remaining_frames: u16) -> String {
    binary_reveal_text_with_seed(text, remaining_frames, DEFAULT_BINARY_REVEAL_FRAMES, 0)
}

/// Same as [`binary_reveal_text`], with a caller-provided seed for repeated
/// labels that should animate with different binary patterns.
pub fn binary_reveal_text_with_seed(
    text: &str,
    remaining_frames: u16,
    total_frames: u16,
    seed: u32,
) -> String {
    let total_frames = total_frames.max(1);
    let remaining_frames = remaining_frames.min(total_frames);
    if remaining_frames == 0 {
        return text.to_owned();
    }

    let revealable_count = text
        .chars()
        .filter(|glyph| glyph.is_ascii_alphanumeric())
        .count();
    if revealable_count == 0 {
        return text.to_owned();
    }

    let binary_count = ((revealable_count as u32 * remaining_frames as u32)
        .div_ceil(total_frames as u32)) as usize;
    let mut revealable_index = 0;

    text.chars()
        .map(|glyph| {
            if !glyph.is_ascii_alphanumeric() {
                return glyph;
            }

            let rank = stable_noise(revealable_index as u32 ^ seed) as usize % revealable_count;
            let replacement = if rank < binary_count {
                binary_glyph(revealable_index as u32, remaining_frames, seed)
            } else {
                glyph
            };
            revealable_index += 1;
            replacement
        })
        .collect()
}

fn binary_glyph(index: u32, remaining_frames: u16, seed: u32) -> char {
    let frame = remaining_frames as u32;
    if stable_noise(index ^ seed ^ frame.wrapping_mul(0x9e37_79b9)) & 1 == 0 {
        '0'
    } else {
        '1'
    }
}

fn stable_noise(mut value: u32) -> u32 {
    value ^= value >> 16;
    value = value.wrapping_mul(0x7feb_352d);
    value ^= value >> 15;
    value = value.wrapping_mul(0x846c_a68b);
    value ^ (value >> 16)
}
