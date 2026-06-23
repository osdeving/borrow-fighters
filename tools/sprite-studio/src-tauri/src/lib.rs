//! Provides local file commands for the Borrow Fighters Sprite Studio.
//!
//! System: Sprite Studio tooling. This backend is intentionally isolated from
//! the game crate and only reads or writes artifact files on disk.

use std::{
    env, fs, io,
    path::{Path, PathBuf},
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

use base64::{engine::general_purpose, Engine as _};
use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Serialize)]
struct SpriteDocument {
    repo_root: String,
    manifest_path: String,
    atlas_path: String,
    manifest_json: String,
    atlas_data_url: String,
}

#[derive(Debug, Serialize)]
struct SaveResult {
    path: String,
    bytes_written: usize,
    backup_path: Option<String>,
}

#[derive(Debug, Serialize)]
struct TextDocument {
    path: String,
    contents: String,
}

#[derive(Debug, Serialize)]
struct RuntimeValidationResult {
    success: bool,
    command: String,
    stdout: String,
    stderr: String,
}

#[derive(Debug, Serialize)]
struct StudioError {
    message: String,
}

impl From<io::Error> for StudioError {
    fn from(error: io::Error) -> Self {
        Self {
            message: error.to_string(),
        }
    }
}

impl From<serde_json::Error> for StudioError {
    fn from(error: serde_json::Error) -> Self {
        Self {
            message: error.to_string(),
        }
    }
}

fn repo_root() -> Result<PathBuf, StudioError> {
    let mut current = env::current_dir()?;
    loop {
        if current.join("Cargo.toml").is_file()
            && current.join("assets").is_dir()
            && current.join("docs").is_dir()
        {
            return Ok(current);
        }
        if !current.pop() {
            return Err(StudioError {
                message: "Could not find Borrow Fighters repository root".to_string(),
            });
        }
    }
}

fn normalize_repo_path(path: &str) -> Result<PathBuf, StudioError> {
    let root = repo_root()?;
    let raw = Path::new(path);
    let full = if raw.is_absolute() {
        raw.to_path_buf()
    } else {
        root.join(raw)
    };
    let normalized = full.canonicalize().map_err(|error| StudioError {
        message: format!("Could not resolve path `{path}`: {error}"),
    })?;
    if !normalized.starts_with(&root) {
        return Err(StudioError {
            message: format!("Path `{path}` is outside the repository"),
        });
    }
    Ok(normalized)
}

fn normalize_repo_write_path(path: &str) -> Result<PathBuf, StudioError> {
    let root = repo_root()?;
    let raw = Path::new(path);
    let full = if raw.is_absolute() {
        raw.to_path_buf()
    } else {
        root.join(raw)
    };
    let parent = full
        .parent()
        .ok_or_else(|| StudioError {
            message: format!("Path `{path}` has no parent directory"),
        })?
        .canonicalize()
        .map_err(|error| StudioError {
            message: format!("Could not resolve parent for `{path}`: {error}"),
        })?;
    let file_name = full.file_name().ok_or_else(|| StudioError {
        message: format!("Path `{path}` has no file name"),
    })?;
    let normalized = parent.join(file_name);
    if !normalized.starts_with(&root) {
        return Err(StudioError {
            message: format!("Path `{path}` is outside the repository"),
        });
    }
    Ok(normalized)
}

fn display_repo_path(path: &Path) -> Result<String, StudioError> {
    let root = repo_root()?;
    let relative = path.strip_prefix(&root).unwrap_or(path);
    Ok(relative.to_string_lossy().replace('\\', "/"))
}

fn image_data_url(path: &Path) -> Result<String, StudioError> {
    let bytes = fs::read(path)?;
    let mime = match path.extension().and_then(|extension| extension.to_str()) {
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("webp") => "image/webp",
        _ => "image/png",
    };
    Ok(format!(
        "data:{mime};base64,{}",
        general_purpose::STANDARD.encode(bytes)
    ))
}

fn safe_artifact_name(path: &str) -> String {
    path.chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() || character == '-' || character == '_' {
                character
            } else {
                '_'
            }
        })
        .collect()
}

fn unix_timestamp() -> Result<u64, StudioError> {
    Ok(SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| StudioError {
            message: format!("System clock is before Unix epoch: {error}"),
        })?
        .as_secs())
}

fn tool_output_dir(name: &str) -> Result<PathBuf, StudioError> {
    let dir = repo_root()?.join("target").join(name);
    fs::create_dir_all(&dir)?;
    Ok(dir)
}

fn backup_existing_file(path: &Path) -> Result<Option<PathBuf>, StudioError> {
    if !path.is_file() {
        return Ok(None);
    }
    let display = display_repo_path(path)?;
    let backup_name = format!(
        "{}.{}.bak.json",
        safe_artifact_name(&display),
        unix_timestamp()?
    );
    let backup_path = tool_output_dir("sprite-studio-backups")?.join(backup_name);
    fs::copy(path, &backup_path)?;
    Ok(Some(backup_path))
}

#[tauri::command]
fn load_sprite_manifest(path: String) -> Result<SpriteDocument, StudioError> {
    let manifest_path = normalize_repo_path(&path)?;
    let manifest_json = fs::read_to_string(&manifest_path)?;
    let manifest: Value = serde_json::from_str(&manifest_json)?;
    let image_name = manifest
        .get("image")
        .and_then(Value::as_str)
        .ok_or_else(|| StudioError {
            message: "Manifest is missing string field `image`".to_string(),
        })?;
    let atlas_path = manifest_path
        .parent()
        .ok_or_else(|| StudioError {
            message: "Manifest path has no parent directory".to_string(),
        })?
        .join(image_name)
        .canonicalize()
        .map_err(|error| StudioError {
            message: format!("Could not resolve atlas `{image_name}`: {error}"),
        })?;
    let root = repo_root()?;
    if !atlas_path.starts_with(&root) {
        return Err(StudioError {
            message: format!("Atlas `{image_name}` is outside the repository"),
        });
    }

    Ok(SpriteDocument {
        repo_root: root.to_string_lossy().to_string(),
        manifest_path: display_repo_path(&manifest_path)?,
        atlas_path: display_repo_path(&atlas_path)?,
        manifest_json,
        atlas_data_url: image_data_url(&atlas_path)?,
    })
}

#[tauri::command]
fn save_sprite_manifest(path: String, manifest_json: String) -> Result<SaveResult, StudioError> {
    let manifest_path = normalize_repo_write_path(&path)?;
    let parsed: Value = serde_json::from_str(&manifest_json)?;
    let pretty = format!("{}\n", serde_json::to_string_pretty(&parsed)?);
    let backup_path = backup_existing_file(&manifest_path)?;
    fs::write(&manifest_path, pretty.as_bytes())?;

    Ok(SaveResult {
        path: display_repo_path(&manifest_path)?,
        bytes_written: pretty.len(),
        backup_path: backup_path.as_deref().map(display_repo_path).transpose()?,
    })
}

#[tauri::command]
fn autosave_sprite_manifest(
    path: String,
    manifest_json: String,
) -> Result<SaveResult, StudioError> {
    let parsed: Value = serde_json::from_str(&manifest_json)?;
    let pretty = format!("{}\n", serde_json::to_string_pretty(&parsed)?);
    let target_name = format!("{}.autosave.json", safe_artifact_name(&path));
    let autosave_path = tool_output_dir("sprite-studio-autosave")?.join(target_name);
    fs::write(&autosave_path, pretty.as_bytes())?;

    Ok(SaveResult {
        path: display_repo_path(&autosave_path)?,
        bytes_written: pretty.len(),
        backup_path: None,
    })
}

#[tauri::command]
fn load_repo_text_file(path: String) -> Result<TextDocument, StudioError> {
    let file_path = normalize_repo_path(&path)?;
    Ok(TextDocument {
        path: display_repo_path(&file_path)?,
        contents: fs::read_to_string(&file_path)?,
    })
}

#[tauri::command]
fn save_repo_text_file(path: String, contents: String) -> Result<SaveResult, StudioError> {
    let file_path = normalize_repo_write_path(&path)?;
    fs::write(&file_path, contents.as_bytes())?;
    Ok(SaveResult {
        path: display_repo_path(&file_path)?,
        bytes_written: contents.len(),
        backup_path: None,
    })
}

#[tauri::command]
fn studio_repo_root() -> Result<String, StudioError> {
    Ok(repo_root()?.to_string_lossy().to_string())
}

#[tauri::command]
fn validate_runtime_assets() -> Result<RuntimeValidationResult, StudioError> {
    let root = repo_root()?;
    let args = [
        "test",
        "--test",
        "sprite_manifest",
        "--test",
        "sprite_frames",
        "--test",
        "sprite_selection",
        "--test",
        "characters",
    ];
    let output = Command::new("cargo")
        .args(args)
        .current_dir(&root)
        .output()
        .map_err(|error| StudioError {
            message: format!("Could not run cargo validation: {error}"),
        })?;

    Ok(RuntimeValidationResult {
        success: output.status.success(),
        command: format!("cargo {}", args.join(" ")),
        stdout: String::from_utf8_lossy(&output.stdout).to_string(),
        stderr: String::from_utf8_lossy(&output.stderr).to_string(),
    })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            load_sprite_manifest,
            load_repo_text_file,
            save_repo_text_file,
            save_sprite_manifest,
            autosave_sprite_manifest,
            studio_repo_root,
            validate_runtime_assets
        ])
        .run(tauri::generate_context!())
        .expect("error while running Borrow Fighters Sprite Studio");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_path_allows_new_files_inside_repo() {
        let path = normalize_repo_write_path("target/sprite-studio-review-test.json")
            .expect("new file under repo target should be writable");

        assert!(path.ends_with("target/sprite-studio-review-test.json"));
    }

    #[test]
    fn write_path_rejects_files_outside_repo() {
        let error = normalize_repo_write_path("/tmp/sprite-studio-review-test.json")
            .expect_err("absolute path outside repo should be rejected");

        assert!(error.message.contains("outside the repository"));
    }
}
