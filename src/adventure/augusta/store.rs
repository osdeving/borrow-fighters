//! Persists C++ progress separately from the legacy Rust campaign profile.
//!
//! System: Augusta persistence boundary. Invalid/newer data is reported intact;
//! complete validated profiles replace a flushed temporary file atomically.

use super::checkpoint::Checkpoint;
use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fs,
    io::{ErrorKind, Write},
    path::{Path, PathBuf},
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Progress {
    pub version: u32,
    pub chapter: String,
    pub protagonist: String,
    pub checkpoint: Option<Checkpoint>,
}

impl Default for Progress {
    fn default() -> Self {
        Self {
            version: 1,
            chapter: "cpp-augusta".into(),
            protagonist: "cpp".into(),
            checkpoint: None,
        }
    }
}

pub fn path() -> PathBuf {
    crate::runtime_paths::data_dir().join("adventure/cpp-augusta-v1.json")
}

pub fn load(path: &Path) -> Result<Progress, Box<dyn Error>> {
    let bytes = match fs::read(path) {
        Ok(bytes) => bytes,
        Err(error) if error.kind() == ErrorKind::NotFound => return Ok(Progress::default()),
        Err(error) => return Err(error.into()),
    };
    let progress: Progress = serde_json::from_slice(&bytes)?;
    validate(&progress)?;
    Ok(progress)
}

fn validate(progress: &Progress) -> Result<(), Box<dyn Error>> {
    if progress.version != 1 || progress.chapter != "cpp-augusta" || progress.protagonist != "cpp" {
        return Err("unsupported C++ chapter profile".into());
    }
    if let Some(checkpoint) = progress.checkpoint {
        checkpoint.validate()?;
    }
    Ok(())
}

pub fn save(path: &Path, progress: &Progress) -> Result<(), Box<dyn Error>> {
    validate(progress)?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let temporary = path.with_extension(format!("tmp-{}", std::process::id()));
    let result = (|| -> Result<(), Box<dyn Error>> {
        let mut output = fs::File::create(&temporary)?;
        serde_json::to_writer_pretty(&mut output, progress)?;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn corrupt_or_future_profiles_are_reported_without_repairing_or_erasing_them() {
        let directory =
            std::env::temp_dir().join(format!("borrow-augusta-invalid-{}", std::process::id()));
        fs::create_dir_all(&directory).unwrap();
        let path = directory.join("cpp-augusta-v1.json");
        for bytes in [b"not valid JSON".as_slice(), br#"{"version":2,"chapter":"cpp-augusta","protagonist":"cpp","checkpoint":null}"#.as_slice(),
            br#"{"version":1,"chapter":"cpp-augusta","protagonist":"cpp","checkpoint":{"version":9,"stage":"rescue"}}"#.as_slice()] {
            fs::write(&path, bytes).unwrap();
            assert!(load(&path).is_err());
            assert_eq!(fs::read(&path).unwrap(), bytes);
        }
        fs::remove_file(&path).unwrap();
        assert_eq!(load(&path).unwrap(), Progress::default());
        fs::remove_dir_all(directory).unwrap();
    }
}
