//! Validates the small external text catalogue used by this chapter.
//!
//! System: Adventure chapter copy. Dialogue and message identifiers come from
//! the model; this catalogue supplies editable Portuguese text without I/O.

use serde::Deserialize;
use std::collections::BTreeMap;

/// External chapter dialogue, objectives and exact authored phone messages.
#[derive(Clone, Debug, Deserialize)]
pub struct ChapterTexts {
    /// Text schema version.
    pub version: u32,
    /// Text indexed by stable semantic keys.
    pub text: BTreeMap<String, String>,
}

impl ChapterTexts {
    /// Parses a replacement catalogue while preserving all required entries.
    pub fn from_json(source: &str) -> Result<Self, String> {
        let catalog: Self = serde_json::from_str(source).map_err(|e| e.to_string())?;
        if catalog.version != 1 {
            return Err("Chapter text requires version 1".into());
        }
        let defaults: Self = serde_json::from_str(Self::SOURCE).map_err(|e| e.to_string())?;
        for key in defaults.text.keys() {
            if catalog
                .text
                .get(key)
                .is_none_or(|value| value.trim().is_empty())
            {
                return Err(format!("Missing chapter text: {key}"));
            }
        }
        Ok(catalog)
    }

    const SOURCE: &str = include_str!("../../../assets/adventure/chapter/chapter-texts.json");

    /// Returns the approved text without accessing the filesystem.
    pub fn bundled() -> Self {
        Self::from_json(Self::SOURCE).expect("bundled chapter text must be valid")
    }

    /// Returns the requested text, with a visible marker for an unknown key.
    pub fn get(&self, key: &str) -> &str {
        self.text
            .get(key)
            .map(String::as_str)
            .unwrap_or("[texto ausente]")
    }
}
