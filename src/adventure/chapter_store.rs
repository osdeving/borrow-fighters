//! Persists safe campaign checkpoints separately from captures and fighting preferences.
//!
//! System: Adventure persistence boundary. Writes replace a flushed temporary file
//! atomically; malformed or newer saves are reported and never silently overwritten.

use super::chapter::Checkpoint;
use crate::runtime_paths;
use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fs,
    io::{ErrorKind, Write},
    path::{Path, PathBuf},
};

/// Minimal progress shared by the prologue handoff and the chapter selection screen.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CampaignProgress {
    pub version: u32,
    pub prologue_seen: bool,
    pub prologue_played_victory: bool,
    #[serde(default)]
    pub street_wake_tick: Option<u32>,
    pub checkpoint: Option<Checkpoint>,
}

impl Default for CampaignProgress {
    fn default() -> Self {
        Self {
            version: 1,
            prologue_seen: false,
            prologue_played_victory: false,
            street_wake_tick: None,
            checkpoint: None,
        }
    }
}

/// Story save location; the standard data-dir override isolates automated playtests.
pub fn path() -> PathBuf {
    runtime_paths::data_dir().join("adventure/campaign-v1.json")
}

/// Reads progress, distinguishing a first run from an unreadable or invalid save.
pub fn load(path: &Path) -> Result<CampaignProgress, Box<dyn Error>> {
    let bytes = match fs::read(path) {
        Ok(bytes) => bytes,
        Err(error) if error.kind() == ErrorKind::NotFound => return Ok(CampaignProgress::default()),
        Err(error) => return Err(error.into()),
    };
    let profile: CampaignProgress = serde_json::from_slice(&bytes)?;
    validate(&profile)?;
    Ok(profile)
}

fn validate(profile: &CampaignProgress) -> Result<(), Box<dyn Error>> {
    if profile.version != 1 {
        return Err("unsupported campaign save version".into());
    }
    if let Some(checkpoint) = profile.checkpoint {
        checkpoint.validate()?;
    }
    Ok(())
}

/// Commits a complete checkpoint without deleting the previous file first.
pub fn save(path: &Path, profile: &CampaignProgress) -> Result<(), Box<dyn Error>> {
    validate(profile)?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let temporary = path.with_extension(format!("tmp-{}", std::process::id()));
    let result = (|| -> Result<(), Box<dyn Error>> {
        let mut output = fs::File::create(&temporary)?;
        serde_json::to_writer_pretty(&mut output, profile)?;
        output.write_all(b"\n")?;
        output.sync_all()?;
        drop(output);
        fs::rename(&temporary, path)?;
        Ok(())
    })();
    if result.is_err() {
        let _ = fs::remove_file(&temporary);
    }
    result
}

/// Records actual prologue play separately from the chapter's canonical starting point.
pub fn remember_prologue(
    played_victory: bool,
    street_wake_tick: Option<u32>,
) -> Result<(), Box<dyn Error>> {
    let destination = path();
    let mut progress = load(&destination)?;
    progress.prologue_seen = true;
    progress.prologue_played_victory |= played_victory;
    if played_victory {
        progress.street_wake_tick = street_wake_tick;
    }
    save(&destination, &progress)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adventure::chapter::CheckpointStage;

    #[test]
    fn checkpoint_replacement_preserves_prologue_provenance_and_rejects_bad_schema() {
        let dir =
            std::env::temp_dir().join(format!("borrow-chapter-save-test-{}", std::process::id()));
        let path = dir.join("campaign.json");
        let mut profile = load(&path).unwrap();
        profile.prologue_seen = true;
        profile.checkpoint = Some(Checkpoint::new(false));
        save(&path, &profile).unwrap();
        profile.checkpoint.as_mut().unwrap().stage = CheckpointStage::PhoneDone;
        save(&path, &profile).unwrap();
        assert_eq!(load(&path).unwrap(), profile);
        assert!(!profile.prologue_played_victory);
        let good = fs::read(&path).unwrap();
        profile.version = 99;
        assert!(save(&path, &profile).is_err());
        assert_eq!(fs::read(&path).unwrap(), good);
        fs::write(&path, b"{broken").unwrap();
        assert!(load(&path).is_err());
        assert_eq!(fs::read(&path).unwrap(), b"{broken");
        fs::remove_dir_all(dir).unwrap();
    }
}
