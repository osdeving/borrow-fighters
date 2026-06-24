//! Validates editable lore data loaded by the menu.

use borrow_fighters::characters::CharacterId;
use borrow_fighters::engine::assets::{
    ROSTER_C_PATH, ROSTER_DUKE_PATH, ROSTER_PYTHON_PATH, ROSTER_RUST_PATH,
};
use borrow_fighters::lore::{LORE_BOOK_PATH, LoreBook};

#[test]
fn repository_lore_book_loads_story_and_roster() {
    let book = LoreBook::load(LORE_BOOK_PATH).expect("lore book loads");

    assert_eq!(book.version, 1);
    assert!(book.title.contains("Linker"));
    assert!(book.chapters.len() >= 4);
    assert_eq!(book.characters.len(), 4);

    for character in &book.characters {
        assert!(
            character.character_id().is_some(),
            "unknown roster character id {}",
            character.id
        );
        assert!(!character.profile.is_empty());
        assert!(!character.goal.is_empty());
        assert!(!character.cycle.contains("197"));
        assert!(!character.cycle.contains("199"));
        assert!(!character.cycle.contains("201"));
    }
}

#[test]
fn lore_book_wraps_selected_indices_for_menu() {
    let book = LoreBook::load(LORE_BOOK_PATH).expect("lore book loads");

    assert_eq!(book.chapter(0), book.chapter(book.chapters.len()));
    assert_eq!(book.character(0), book.character(book.characters.len()));
    assert_eq!(
        book.characters
            .iter()
            .find(|profile| profile.character_id() == Some(CharacterId::Rust))
            .map(|profile| profile.file_name.as_str()),
        Some("rust.rs")
    );
}

#[test]
fn roster_portrait_assets_exist() {
    for path in [
        ROSTER_RUST_PATH,
        ROSTER_DUKE_PATH,
        ROSTER_C_PATH,
        ROSTER_PYTHON_PATH,
    ] {
        assert!(std::path::Path::new(path).exists(), "{path} missing");
    }
}
