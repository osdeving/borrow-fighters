//! Exercises reusable binary text menu animation helpers.

use borrow_fighters::ui::binary_text::{
    DEFAULT_BINARY_REVEAL_FRAMES, binary_reveal_text, binary_reveal_text_with_seed,
};

#[test]
fn binary_reveal_returns_original_text_when_finished() {
    assert_eq!(binary_reveal_text("QUICK FIGHT", 0), "QUICK FIGHT");
}

#[test]
fn binary_reveal_preserves_spacing_and_punctuation() {
    let text = "VERSUS SETUP // P1";
    let revealed = binary_reveal_text(text, DEFAULT_BINARY_REVEAL_FRAMES);

    assert_eq!(revealed.chars().count(), text.chars().count());
    assert_eq!(revealed.chars().nth(6), Some(' '));
    assert_eq!(revealed.chars().nth(12), Some(' '));
    assert_eq!(revealed.chars().nth(13), Some('/'));
    assert_eq!(revealed.chars().nth(14), Some('/'));
}

#[test]
fn binary_reveal_replaces_alphanumeric_text_at_full_pulse() {
    let revealed = binary_reveal_text("TRAINING", DEFAULT_BINARY_REVEAL_FRAMES);

    assert!(revealed.chars().all(|glyph| glyph == '0' || glyph == '1'));
}

#[test]
fn seeded_binary_reveal_can_vary_repeated_labels() {
    let first = binary_reveal_text_with_seed("OPTIONS", 18, DEFAULT_BINARY_REVEAL_FRAMES, 1);
    let second = binary_reveal_text_with_seed("OPTIONS", 18, DEFAULT_BINARY_REVEAL_FRAMES, 2);

    assert_ne!(first, second);
}
