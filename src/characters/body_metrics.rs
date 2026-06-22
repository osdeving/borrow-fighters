//! Loads character body metrics from a small JSON manifest.
//!
//! System: Character data. This module owns data-driven physical body sizes
//! without changing move data, rendering, or match flow.

use std::{
    error::Error,
    fmt::{Display, Formatter},
    fs,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::combat::fighter::FighterBodyMetrics;

use super::{CharacterId, character_spec};

pub const CHARACTER_BODY_METRICS_PATH: &str = "assets/tuning/character-body-metrics.json";
const CHARACTER_BODY_METRICS_SCHEMA: &str = "borrow-fighters.character-body-metrics.v1";

/// Body metrics loaded from data files, with static character defaults as fallback.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CharacterBodyMetricsCatalog {
    entries: Vec<CharacterBodyMetricsEntry>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct CharacterBodyMetricsEntry {
    character: CharacterId,
    body: FighterBodyMetrics,
}

#[derive(Debug, Deserialize, Serialize)]
struct CharacterBodyMetricsManifest {
    schema: String,
    characters: Vec<CharacterBodyMetricsRecord>,
}

#[derive(Debug, Deserialize, Serialize)]
struct CharacterBodyMetricsRecord {
    id: String,
    body: FighterBodyMetricsRecord,
}

#[derive(Debug, Deserialize, Serialize)]
struct FighterBodyMetricsRecord {
    width: f32,
    standing_height: f32,
    crouch_height: f32,
}

/// Error returned while loading character body metrics.
#[derive(Debug)]
pub enum CharacterBodyMetricsError {
    Io {
        path: PathBuf,
        source: std::io::Error,
    },
    Json {
        path: PathBuf,
        source: serde_json::Error,
    },
    JsonWrite {
        path: PathBuf,
        source: serde_json::Error,
    },
    Invalid(String),
}

impl CharacterBodyMetricsCatalog {
    /// Loads character body metrics from disk.
    pub fn load(path: impl AsRef<Path>) -> Result<Self, CharacterBodyMetricsError> {
        let path = path.as_ref().to_path_buf();
        let content =
            fs::read_to_string(&path).map_err(|source| CharacterBodyMetricsError::Io {
                path: path.clone(),
                source,
            })?;
        Self::from_json_str(&content).map_err(|error| error.with_path(path))
    }

    /// Saves character body metrics to disk as stable pretty JSON.
    pub fn save(&self, path: impl AsRef<Path>) -> Result<(), CharacterBodyMetricsError> {
        let path = path.as_ref().to_path_buf();
        let manifest = CharacterBodyMetricsManifest {
            schema: CHARACTER_BODY_METRICS_SCHEMA.to_string(),
            characters: self
                .entries
                .iter()
                .map(|entry| CharacterBodyMetricsRecord {
                    id: entry.character.audio_key().to_string(),
                    body: FighterBodyMetricsRecord {
                        width: entry.body.width,
                        standing_height: entry.body.standing_height,
                        crouch_height: entry.body.crouch_height,
                    },
                })
                .collect(),
        };
        let content = serde_json::to_string_pretty(&manifest).map_err(|source| {
            CharacterBodyMetricsError::JsonWrite {
                path: path.clone(),
                source,
            }
        })?;
        fs::write(&path, format!("{content}\n"))
            .map_err(|source| CharacterBodyMetricsError::Io { path, source })
    }

    /// Loads character body metrics from a JSON string.
    pub fn from_json_str(content: &str) -> Result<Self, CharacterBodyMetricsError> {
        let manifest: CharacterBodyMetricsManifest =
            serde_json::from_str(content).map_err(|source| CharacterBodyMetricsError::Json {
                path: PathBuf::new(),
                source,
            })?;
        manifest.try_into()
    }

    /// Returns body metrics for a character, falling back to static defaults.
    pub fn body_metrics_for(&self, character: CharacterId) -> FighterBodyMetrics {
        self.entries
            .iter()
            .find(|entry| entry.character == character)
            .map(|entry| entry.body)
            .unwrap_or_else(|| character_spec(character).body_metrics)
    }

    /// Overrides body metrics for a character in memory.
    pub fn set_body_metrics_for(&mut self, character: CharacterId, body: FighterBodyMetrics) {
        let body = body.sanitized();
        if let Some(entry) = self
            .entries
            .iter_mut()
            .find(|entry| entry.character == character)
        {
            entry.body = body;
            return;
        }

        self.entries
            .push(CharacterBodyMetricsEntry { character, body });
    }
}

impl TryFrom<CharacterBodyMetricsManifest> for CharacterBodyMetricsCatalog {
    type Error = CharacterBodyMetricsError;

    fn try_from(manifest: CharacterBodyMetricsManifest) -> Result<Self, Self::Error> {
        if manifest.schema != CHARACTER_BODY_METRICS_SCHEMA {
            return Err(CharacterBodyMetricsError::Invalid(format!(
                "unsupported character body metrics schema '{}'",
                manifest.schema
            )));
        }

        let mut entries = Vec::with_capacity(manifest.characters.len());
        for record in manifest.characters {
            let character = CharacterId::from_audio_key(&record.id).ok_or_else(|| {
                CharacterBodyMetricsError::Invalid(format!(
                    "unknown character body metrics id '{}'",
                    record.id
                ))
            })?;
            if entries
                .iter()
                .any(|entry: &CharacterBodyMetricsEntry| entry.character == character)
            {
                return Err(CharacterBodyMetricsError::Invalid(format!(
                    "duplicate character body metrics id '{}'",
                    record.id
                )));
            }

            let body = FighterBodyMetrics {
                width: record.body.width,
                standing_height: record.body.standing_height,
                crouch_height: record.body.crouch_height,
            }
            .sanitized();
            entries.push(CharacterBodyMetricsEntry { character, body });
        }

        Ok(Self { entries })
    }
}

impl CharacterBodyMetricsError {
    fn with_path(self, path: PathBuf) -> Self {
        match self {
            Self::Json { source, .. } => Self::Json { path, source },
            error => error,
        }
    }
}

impl Display for CharacterBodyMetricsError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io { path, source } => {
                write!(
                    formatter,
                    "could not read character body metrics {}: {source}",
                    path.display()
                )
            }
            Self::Json { path, source } if path.as_os_str().is_empty() => {
                write!(
                    formatter,
                    "could not parse character body metrics: {source}"
                )
            }
            Self::Json { path, source } => {
                write!(
                    formatter,
                    "could not parse character body metrics {}: {source}",
                    path.display()
                )
            }
            Self::JsonWrite { path, source } => {
                write!(
                    formatter,
                    "could not serialize character body metrics {}: {source}",
                    path.display()
                )
            }
            Self::Invalid(message) => formatter.write_str(message),
        }
    }
}

impl Error for CharacterBodyMetricsError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Io { source, .. } => Some(source),
            Self::Json { source, .. } => Some(source),
            Self::JsonWrite { source, .. } => Some(source),
            Self::Invalid(_) => None,
        }
    }
}
