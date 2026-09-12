//! Defines safe Augusta milestones without storing transient combat actors.
//!
//! System: C++ chapter saves. A retry reconstructs only the current encounter;
//! Rust's checkpoint format and prologue provenance stay in their original file.

use serde::{Deserialize, Serialize};

/// Reconstructible progress boundaries, advanced only by actual narrative/combat results.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckpointStage {
    #[default]
    Arrival,
    GuardsFight,
    ErraticsFight,
    Rescue,
    Complete,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Checkpoint {
    pub version: u32,
    pub stage: CheckpointStage,
}

impl Default for Checkpoint {
    fn default() -> Self {
        Self {
            version: 1,
            stage: CheckpointStage::Arrival,
        }
    }
}

impl Checkpoint {
    pub fn validate(self) -> Result<(), String> {
        if self.version != 1 {
            return Err("unsupported Augusta checkpoint version".into());
        }
        Ok(())
    }
}
