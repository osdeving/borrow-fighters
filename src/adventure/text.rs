//! Loads the adventure's editable Portuguese copy without embedding it in the binary.
//!
//! System: Adventure content. Reload is transactional: invalid edits preserve the
//! last valid catalog, and narrative clocks/gameplay are never touched here.

use serde::Deserialize;
use std::{
    collections::BTreeMap,
    error::Error,
    fs,
    path::{Path, PathBuf},
};

const REQUIRED_KEYS: &[&str] = &[
    "window.title",
    "street.arrival.controls",
    "street.ep_arrival.controls",
    "street.bar.kind",
    "street.bar.name",
    "street.stop.title",
    "street.stop.route",
    "ada.eyebrow",
    "ada.caption.ordinary",
    "ada.caption.message",
    "ada.caption.signal",
    "ada.caption.assembly",
    "ada.caption.after",
    "ada.sender",
    "ada.message",
    "ada.cursor",
    "ada.years",
    "ada.years_detail",
    "ada.controls",
    "morning.eyebrow",
    "morning.name",
    "morning.caption",
    "morning.controls",
    "morning.poster.rust",
    "morning.poster.slogan",
    "morning.setup.command",
    "morning.setup.status",
    "encounter.name",
    "encounter.objective",
    "encounter.explore",
    "encounter.direction",
    "encounter.alert",
    "encounter.controls",
    "aftermath.caption",
    "ending.title",
    "ending.caption",
    "ending.detail",
    "ending.controls",
    "defeat.title",
    "defeat.caption",
    "defeat.controls",
    "pause.title",
    "pause.resume",
    "pause.exit",
    "pause.controls",
    "editor.reloaded",
    "editor.failed",
    "opening.controls",
    "opening.newspaper",
    "opening.news.0.section",
    "opening.news.0.headline",
    "opening.news.0.deck",
    "opening.news.1.section",
    "opening.news.1.headline",
    "opening.news.1.deck",
    "opening.news.2.section",
    "opening.news.2.headline",
    "opening.news.2.deck",
    "opening.cpp.name",
    "opening.cpp.role",
    "opening.cpp.before",
    "opening.cpp.after",
    "opening.python.name",
    "opening.python.role",
    "opening.python.before",
    "opening.python.after",
    "opening.duke.name",
    "opening.duke.role",
    "opening.duke.before",
    "opening.duke.after",
    "opening.c.name",
    "opening.c.role",
    "opening.c.before",
    "opening.c.after",
    "opening.go.name",
    "opening.go.role",
    "opening.rust.name",
    "opening.rust.role",
    "opening.logo.top",
    "opening.logo.bottom",
    "opening.subtitle",
    "opening.tagline",
    "navigation.controls_menu",
    "navigation.controls",
    "navigation.ada_controls_menu",
    "navigation.ada_controls",
    "navigation.pause_menu",
    "navigation.pause",
    "navigation.continue",
    "navigation.continue_detail",
];

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Document {
    version: u32,
    text: BTreeMap<String, String>,
}

/// Validated external text and the exact source used for subsequent reloads.
pub struct TextCatalog {
    source: PathBuf,
    document: Document,
}

impl TextCatalog {
    /// Reads UTF-8 JSON; missing files/keys and unsupported schemas are explicit errors.
    pub fn load(path: impl AsRef<Path>) -> Result<Self, Box<dyn Error>> {
        let source = path.as_ref().to_owned();
        let read = || -> Result<Document, Box<dyn Error>> {
            let document: Document = serde_json::from_str(&fs::read_to_string(&source)?)?;
            if document.version != 1 {
                return Err("unsupported text version; expected 1".into());
            }
            for key in REQUIRED_KEYS {
                let value = document
                    .text
                    .get(*key)
                    .ok_or_else(|| format!("missing text key: {key}"))?;
                if value.trim().is_empty() {
                    return Err(format!("empty text key: {key}").into());
                }
            }
            for (key, value) in &document.text {
                if value.chars().count() > 2000
                    || value.chars().any(|c| c.is_control() && c != '\n')
                {
                    return Err(format!(
                        "invalid text in {key}: limit 2000 characters; only newline control allowed"
                    )
                    .into());
                }
            }
            Ok(document)
        };
        let document = read().map_err(|e| format!("{}: {e}", source.display()))?;
        Ok(Self { source, document })
    }

    /// Replaces the catalog only after the complete edited file passes validation.
    pub fn reload(&mut self) -> Result<(), Box<dyn Error>> {
        let replacement = Self::load(&self.source)?;
        *self = replacement;
        Ok(())
    }

    /// Checks dynamic scene references against this catalog, including --texts overrides.
    pub fn contains_key(&self, key: &str) -> bool {
        self.document.text.contains_key(key)
    }

    /// Returns editable copy, with a visible diagnostic for a missing key.
    pub fn get(&self, key: &str) -> &str {
        self.document
            .text
            .get(key)
            .map(String::as_str)
            .unwrap_or("[missing text key]")
    }

    /// Location the author edits, also used by the running application for reload.
    pub fn path(&self) -> &Path {
        &self.source
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime_paths::asset_path;
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct Fixture(PathBuf);
    impl Fixture {
        fn new() -> Self {
            static NEXT: AtomicUsize = AtomicUsize::new(0);
            let path = std::env::temp_dir().join(format!(
                "borrow-copy-{}-{}.json",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
            fs::copy(asset_path("assets/adventure/texts/pt-BR.json"), &path).unwrap();
            Self(path)
        }
    }
    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = fs::remove_file(&self.0);
        }
    }

    #[test]
    fn external_edits_reload_utf8_without_changing_the_binary_and_invalid_edits_preserve_copy() {
        let fixture = Fixture::new();
        let mut catalog = TextCatalog::load(&fixture.0).unwrap();
        let mut json: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(&fixture.0).unwrap()).unwrap();
        json["text"]["ada.message"] = "Ada, amanhã haverá São Paulo.\nC++ e Python.".into();
        fs::write(&fixture.0, serde_json::to_string_pretty(&json).unwrap()).unwrap();
        catalog.reload().unwrap();
        assert_eq!(
            catalog.get("ada.message"),
            "Ada, amanhã haverá São Paulo.\nC++ e Python."
        );
        for invalid in ["{broken", "{\"version\":1,\"text\":{}}"] {
            fs::write(&fixture.0, invalid).unwrap();
            assert!(catalog.reload().is_err());
            assert_eq!(
                catalog.get("ada.message"),
                "Ada, amanhã haverá São Paulo.\nC++ e Python."
            );
        }
    }

    #[test]
    fn unsupported_version_and_empty_required_copy_are_reported_with_source_path() {
        let fixture = Fixture::new();
        let mut json: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(&fixture.0).unwrap()).unwrap();
        json["version"] = 99.into();
        fs::write(&fixture.0, serde_json::to_string(&json).unwrap()).unwrap();
        let error = TextCatalog::load(&fixture.0).err().unwrap().to_string();
        assert!(error.contains(fixture.0.to_str().unwrap()));
        assert!(error.contains("version"));
        json["version"] = 1.into();
        json["text"]["opening.subtitle"] = "  ".into();
        fs::write(&fixture.0, serde_json::to_string(&json).unwrap()).unwrap();
        assert!(
            TextCatalog::load(&fixture.0)
                .err()
                .unwrap()
                .to_string()
                .contains("opening.subtitle")
        );
    }
}
