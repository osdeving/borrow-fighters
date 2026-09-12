//! Holds the external dialogue and interface copy for C++ on Rua Augusta.
//!
//! System: Augusta narrative content. Speakers and complete dialogue blocks are
//! validated together; this module neither advances scenes nor renders text.

use serde::Deserialize;
use std::{collections::BTreeMap, error::Error, fs, path::Path};

/// A named participant. Julia's adult age is explicit content, not visual inference.
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Speaker {
    pub name: String,
    pub age: Option<u8>,
}

/// One complete line, displayed with its speaker by the shared scene renderer.
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Line {
    pub speaker: String,
    pub text: String,
}

/// Local chapter copy; the three named conversations are finite and independent.
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Texts {
    version: u32,
    pub strings: BTreeMap<String, String>,
    pub speakers: BTreeMap<String, Speaker>,
    pub dialogues: BTreeMap<String, Vec<Line>>,
}

impl Texts {
    pub fn load(path: &Path) -> Result<Self, Box<dyn Error>> {
        Self::from_json(&fs::read_to_string(path)?)
    }

    pub fn from_json(json: &str) -> Result<Self, Box<dyn Error>> {
        let texts: Self = serde_json::from_str(json)?;
        if texts.version != 1
            || ["cpp", "julia", "broker", "security"]
                .iter()
                .any(|key| !texts.speakers.contains_key(*key))
            || texts
                .speakers
                .values()
                .any(|speaker| speaker.name.trim().is_empty())
            || texts.speakers.get("julia").and_then(|s| s.age) != Some(22)
            || ["confrontation", "after_guards", "rescue"]
                .iter()
                .any(|key| texts.dialogues.get(*key).is_none_or(Vec::is_empty))
            || texts.dialogues.values().flatten().any(|line| {
                line.text.trim().is_empty() || !texts.speakers.contains_key(&line.speaker)
            })
            || [
                "chapter.title",
                "chapter.summary",
                "objective.approach",
                "objective.guards",
                "objective.after_guards",
                "objective.erratics",
                "objective.rescue",
                "objective.exit",
                "arrival.guards",
                "arrival.title",
                "arrival.erratics",
                "julia.attempt",
                "cinema.skip",
                "cinema.place",
                "cinema.julia",
                "cinema.panic",
                "broker.escape",
                "chapter.complete",
                "chapter.complete_title",
                "chapter.return",
                "controls",
                "dialogue.next",
                "defeat",
                "defeat.title",
                "pause",
                "resume",
                "retry",
                "menu",
                "editor.reloaded",
                "editor.failed",
            ]
            .iter()
            .any(|key| {
                texts
                    .strings
                    .get(*key)
                    .is_none_or(|value| value.trim().is_empty())
            })
        {
            return Err("invalid Augusta copy, dialogue references or adult Julia identity".into());
        }
        Ok(texts)
    }

    pub fn get<'a>(&'a self, key: &'a str) -> &'a str {
        self.strings.get(key).map(String::as_str).unwrap_or(key)
    }

    pub fn line(&self, dialogue: &str, index: usize) -> Option<&Line> {
        self.dialogues.get(dialogue)?.get(index)
    }
}
