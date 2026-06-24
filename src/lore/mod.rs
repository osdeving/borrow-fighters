//! Loads editable story and roster profile data.
//!
//! System: Lore data. This module owns narrative text loaded from disk and
//! stays independent from Raylib, menu rendering, and combat rules.

use std::{fs, path::Path};

use serde::Deserialize;

use crate::characters::CharacterId;

/// Runtime path for the editable lore book.
pub const LORE_BOOK_PATH: &str = "assets/lore/story.json";

const DEFAULT_LORE_JSON: &str = include_str!("../../assets/lore/story.json");

/// Editable story book consumed by the menu.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct LoreBook {
    pub version: u32,
    pub title: String,
    pub subtitle: String,
    #[serde(default)]
    pub chapters: Vec<LoreChapter>,
    #[serde(default)]
    pub characters: Vec<LoreCharacter>,
}

/// One chapter in the lore reader.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct LoreChapter {
    pub title: String,
    pub code: String,
    #[serde(default)]
    pub body: Vec<String>,
}

/// One character profile shown by the roster.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct LoreCharacter {
    pub id: String,
    pub name: String,
    pub file_name: String,
    pub cycle: String,
    pub epithet: String,
    pub origin: String,
    pub goal: String,
    pub profile: String,
    pub combat_style: String,
    pub linker_note: String,
}

impl LoreBook {
    /// Loads a lore book from disk.
    pub fn load(path: impl AsRef<Path>) -> Result<Self, LoreBookError> {
        let path = path.as_ref();
        let text = fs::read_to_string(path).map_err(|source| LoreBookError::Read {
            path: path.display().to_string(),
            source,
        })?;
        Self::from_json(&text).map_err(|source| LoreBookError::Parse {
            path: path.display().to_string(),
            source,
        })
    }

    /// Parses a lore book from JSON text.
    pub fn from_json(text: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(text)
    }

    /// Loads the runtime book or falls back to the bundled prototype copy.
    pub fn load_or_default(path: impl AsRef<Path>) -> Self {
        match Self::load(path) {
            Ok(book) => book,
            Err(error) => {
                eprintln!("warning: using bundled lore book: {error}");
                Self::default()
            }
        }
    }

    /// Returns a chapter by wrapping the selected index.
    pub fn chapter(&self, index: usize) -> Option<&LoreChapter> {
        let count = self.chapters.len();
        if count == 0 {
            None
        } else {
            self.chapters.get(index % count)
        }
    }

    /// Returns a character profile by wrapping the selected index.
    pub fn character(&self, index: usize) -> Option<&LoreCharacter> {
        let count = self.characters.len();
        if count == 0 {
            None
        } else {
            self.characters.get(index % count)
        }
    }

    /// Number of selectable chapters, at least one for menu navigation.
    pub fn chapter_count_for_menu(&self) -> usize {
        self.chapters.len().max(1)
    }

    /// Number of selectable character profiles, at least one for menu navigation.
    pub fn character_count_for_menu(&self) -> usize {
        self.characters.len().max(1)
    }
}

impl LoreCharacter {
    /// Parses the profile id into the runtime character id when possible.
    pub fn character_id(&self) -> Option<CharacterId> {
        CharacterId::from_audio_key(&self.id)
    }
}

impl Default for LoreBook {
    fn default() -> Self {
        Self::from_json(DEFAULT_LORE_JSON).expect("bundled lore book must parse")
    }
}

/// Lore loading error.
#[derive(Debug)]
pub enum LoreBookError {
    Read {
        path: String,
        source: std::io::Error,
    },
    Parse {
        path: String,
        source: serde_json::Error,
    },
}

impl std::fmt::Display for LoreBookError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read { path, source } => {
                write!(formatter, "could not read lore book {path}: {source}")
            }
            Self::Parse { path, source } => {
                write!(formatter, "could not parse lore book {path}: {source}")
            }
        }
    }
}

impl std::error::Error for LoreBookError {}
