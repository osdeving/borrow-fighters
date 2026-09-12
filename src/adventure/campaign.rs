//! Registers the playable protagonists without merging their story or save state.
//!
//! System: Adventure campaign selection. Entries describe navigation only;
//! each chapter validates and persists its own checkpoint schema.

use serde::Deserialize;
use std::{error::Error, fs, path::Path};

/// Implemented chapter routes; future protagonists are added when playable.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ChapterId {
    Rust,
    CppAugusta,
}

/// User-facing labels and the concrete chapter route they describe.
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Entry {
    pub id: ChapterId,
    pub protagonist: String,
    pub title: String,
    pub summary: String,
}

/// Small editable menu copy alongside typed routes into existing chapter apps.
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Registry {
    version: u32,
    pub title: String,
    pub prompt: String,
    pub start_label: String,
    pub continue_label: String,
    pub restart_label: String,
    pub completed_label: String,
    pub back_label: String,
    pub prologue_label: String,
    pub chapters: Vec<Entry>,
}

impl Registry {
    pub fn load(path: &Path) -> Result<Self, Box<dyn Error>> {
        Self::from_json(&fs::read_to_string(path)?)
    }

    pub fn from_json(json: &str) -> Result<Self, Box<dyn Error>> {
        let registry: Self = serde_json::from_str(json)?;
        let unique: std::collections::BTreeSet<_> =
            registry.chapters.iter().map(|entry| entry.id).collect();
        if registry.version != 1
            || registry.chapters.is_empty()
            || unique.len() != registry.chapters.len()
            || [
                &registry.title,
                &registry.prompt,
                &registry.start_label,
                &registry.continue_label,
                &registry.restart_label,
                &registry.completed_label,
                &registry.back_label,
                &registry.prologue_label,
            ]
            .iter()
            .any(|s| s.trim().is_empty())
            || registry.chapters.iter().any(|entry| {
                entry.protagonist.trim().is_empty()
                    || entry.title.trim().is_empty()
                    || entry.summary.trim().is_empty()
            })
        {
            return Err("invalid playable chapter registry".into());
        }
        Ok(registry)
    }
}

/// Registry is presentation content; saves remain behind each chapter boundary.
pub const REGISTRY_PATH: &str = "assets/adventure/campaign.json";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shipped_selector_only_offers_the_two_implemented_protagonists() {
        let registry =
            Registry::load(&Path::new(env!("CARGO_MANIFEST_DIR")).join(REGISTRY_PATH)).unwrap();
        assert_eq!(
            registry
                .chapters
                .iter()
                .map(|entry| entry.id)
                .collect::<Vec<_>>(),
            [ChapterId::Rust, ChapterId::CppAugusta]
        );
        assert_eq!(registry.chapters[0].protagonist, "Rust");
        assert_eq!(registry.chapters[1].protagonist, "C++");
    }

    #[test]
    fn standalone_cpp_registry_is_valid_but_duplicate_or_unknown_routes_are_not() {
        let mut content: serde_json::Value =
            serde_json::from_str(include_str!("../../assets/adventure/campaign.json")).unwrap();
        content["chapters"] = serde_json::json!([content["chapters"][1].clone()]);
        let standalone = Registry::from_json(&content.to_string()).unwrap();
        assert_eq!(standalone.chapters.len(), 1);
        assert_eq!(standalone.chapters[0].id, ChapterId::CppAugusta);
        let duplicate = content["chapters"][0].clone();
        content["chapters"].as_array_mut().unwrap().push(duplicate);
        assert!(Registry::from_json(&content.to_string()).is_err());
        content["chapters"] = serde_json::json!([]);
        assert!(Registry::from_json(&content.to_string()).is_err());
        content["chapters"] = serde_json::json!([{"id":"python","protagonist":"Python","title":"Unavailable","summary":"Not implemented"}]);
        assert!(Registry::from_json(&content.to_string()).is_err());
    }
}
