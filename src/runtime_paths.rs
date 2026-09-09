//! Locates bundled assets and per-user files independently from the launch directory.
//!
//! System: Runtime platform paths. Installed and portable builds share the same
//! asset layout; generated files live outside their potentially read-only bundle.

use std::path::{Path, PathBuf};

/// Resolves a built-in `assets/...` path without changing the working directory.
///
/// Explicit absolute paths and paths outside `assets` are returned unchanged,
/// so developer tools can still accept files relative to the caller's directory.
pub fn asset_path(path: impl AsRef<Path>) -> PathBuf {
    let path = path.as_ref();
    let Ok(relative) = path.strip_prefix("assets") else {
        return path.to_path_buf();
    };
    asset_dir().join(relative)
}

fn asset_dir() -> PathBuf {
    if let Some(directory) = absolute_env_path("BORROW_FIGHTERS_ASSET_DIR") {
        return directory;
    }
    let executable = std::env::current_exe().ok();
    let working_directory = std::env::current_dir().ok();
    let source_directory = Path::new(env!("CARGO_MANIFEST_DIR"));
    asset_candidates(
        executable.as_deref(),
        working_directory.as_deref(),
        source_directory,
    )
    .into_iter()
    .find(|directory| directory.is_dir())
    .unwrap_or_else(|| PathBuf::from("assets"))
}

fn asset_candidates(
    executable: Option<&Path>,
    working_directory: Option<&Path>,
    source_directory: &Path,
) -> Vec<PathBuf> {
    let mut candidates = Vec::new();
    if let Some(directory) = executable.and_then(Path::parent) {
        candidates.push(directory.join("assets"));
        if directory.file_name().is_some_and(|name| name == "bin")
            && let Some(parent) = directory.parent()
        {
            candidates.push(parent.join("assets"));
        }
    }
    if let Some(directory) = working_directory {
        candidates.push(directory.join("assets"));
    }
    candidates.push(source_directory.join("assets"));
    candidates
}

/// Returns the per-user directory for first-run state and generated files.
///
/// Callers create it when writing. `BORROW_FIGHTERS_DATA_DIR` can select another
/// absolute directory, for example to keep automated playtests isolated.
pub fn data_dir() -> PathBuf {
    if let Some(directory) = absolute_env_path("BORROW_FIGHTERS_DATA_DIR") {
        return directory;
    }
    let home = absolute_env_path(if cfg!(windows) { "USERPROFILE" } else { "HOME" });
    platform_data_dir(
        cfg!(windows),
        absolute_env_path("LOCALAPPDATA").as_deref(),
        absolute_env_path("XDG_DATA_HOME").as_deref(),
        home.as_deref(),
    )
    .unwrap_or_else(|| std::env::temp_dir().join("borrow-fighters"))
}

/// Returns the writable directory for local videos and screenshots.
pub fn capture_dir() -> PathBuf {
    data_dir().join("captures")
}

fn absolute_env_path(name: &str) -> Option<PathBuf> {
    std::env::var_os(name)
        .map(PathBuf::from)
        .filter(|path| path.is_absolute())
}

fn platform_data_dir(
    windows: bool,
    local_app_data: Option<&Path>,
    xdg_data_home: Option<&Path>,
    home: Option<&Path>,
) -> Option<PathBuf> {
    if windows {
        local_app_data
            .map(|directory| directory.join("BorrowFighters"))
            .or_else(|| home.map(|directory| directory.join("AppData/Local/BorrowFighters")))
    } else {
        xdg_data_home
            .map(|directory| directory.join("borrow-fighters"))
            .or_else(|| home.map(|directory| directory.join(".local/share/borrow-fighters")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bundled_assets_precede_unrelated_launch_directory_and_build_machine() {
        assert_eq!(
            asset_candidates(
                Some(Path::new("/opt/borrow-fighters/bin/borrow-fighters")),
                Some(Path::new("/tmp")),
                Path::new("/build/repository"),
            ),
            [
                "/opt/borrow-fighters/bin/assets",
                "/opt/borrow-fighters/assets",
                "/tmp/assets",
                "/build/repository/assets",
            ]
            .map(PathBuf::from)
        );
    }

    #[test]
    fn portable_folder_and_cargo_run_both_have_asset_candidates() {
        let candidates = asset_candidates(
            Some(Path::new("/games/Borrow Fighters/borrow-fighters")),
            None,
            Path::new("/source"),
        );
        assert_eq!(candidates[0], Path::new("/games/Borrow Fighters/assets"));
        assert_eq!(candidates[1], Path::new("/source/assets"));
        assert_eq!(
            asset_path("custom/fighter.json"),
            Path::new("custom/fighter.json")
        );
        assert_eq!(
            asset_path("/custom/fighter.json"),
            Path::new("/custom/fighter.json")
        );
    }

    #[test]
    fn generated_files_use_per_user_platform_directories() {
        assert_eq!(
            platform_data_dir(
                true,
                Some(Path::new("/local")),
                None,
                Some(Path::new("/home"))
            ),
            Some(PathBuf::from("/local/BorrowFighters"))
        );
        assert_eq!(
            platform_data_dir(
                false,
                None,
                Some(Path::new("/xdg")),
                Some(Path::new("/home"))
            ),
            Some(PathBuf::from("/xdg/borrow-fighters"))
        );
        assert_eq!(
            platform_data_dir(false, None, None, Some(Path::new("/home"))),
            Some(PathBuf::from("/home/.local/share/borrow-fighters"))
        );
        assert_eq!(platform_data_dir(false, None, None, None), None);
    }
}
