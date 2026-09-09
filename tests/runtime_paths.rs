//! Verifies bundled data loading after relocating the executable away from the repository.
//!
//! System: Distribution regression checks. A subprocess provides independent
//! executable, working-directory, and environment paths without global mutations.

use std::{fs, path::PathBuf, process::Command};

use borrow_fighters::{
    audio::AudioBank,
    characters::{CHARACTER_BODY_METRICS_PATH, CharacterBodyMetricsCatalog},
    engine::{assets::ARENA_SIRIUS_PATH, audio::AUDIO_MANIFEST_PATH, sprites::SpriteManifest},
    lore::{LORE_BOOK_PATH, LoreBook},
    runtime_paths::{asset_path, capture_dir, data_dir},
};

const PROBE_ROOT_ENV: &str = "BORROW_FIGHTERS_PATH_TEST_ROOT";

#[test]
fn bundled_resources_load_after_relocation_and_launch_from_another_directory() {
    let temporary = std::env::temp_dir().join(format!(
        "borrow-fighters-paths-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    let bundle = temporary.join("Jogo ação çãõ 日本語");
    let bin = bundle.join("bin");
    let launch_directory = temporary.join("unrelated launch folder");
    fs::create_dir_all(&bin).unwrap();
    fs::create_dir_all(&launch_directory).unwrap();
    for relative in [
        AUDIO_MANIFEST_PATH,
        LORE_BOOK_PATH,
        CHARACTER_BODY_METRICS_PATH,
        "assets/placeholder/rust-fighter.sprite.json",
        ARENA_SIRIUS_PATH,
        "assets/placeholder/fighter-greybox-spritesheet.png",
    ] {
        let target = bundle.join(relative);
        fs::create_dir_all(target.parent().unwrap()).unwrap();
        fs::copy(
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(relative),
            target,
        )
        .unwrap();
    }
    fs::copy(
        bundle.join("assets/placeholder/rust-fighter.sprite.json"),
        launch_directory.join("custom.sprite.json"),
    )
    .unwrap();
    let test_executable = std::env::current_exe().unwrap();
    let relocated = bin.join(test_executable.file_name().unwrap());
    fs::hard_link(&test_executable, &relocated)
        .or_else(|_| fs::copy(&test_executable, &relocated).map(|_| ()))
        .unwrap();

    let result = Command::new(&relocated)
        .args(["--exact", "relocated_resource_probe", "--nocapture"])
        .current_dir(&launch_directory)
        .env(PROBE_ROOT_ENV, &bundle)
        .env("BORROW_FIGHTERS_DATA_DIR", temporary.join("user data"))
        .env_remove("BORROW_FIGHTERS_ASSET_DIR")
        .output()
        .unwrap();
    let _ = fs::remove_dir_all(&temporary);
    assert!(
        result.status.success(),
        "Relocated executable failed:\n{}\n{}",
        String::from_utf8_lossy(&result.stdout),
        String::from_utf8_lossy(&result.stderr)
    );
}

#[test]
fn relocated_resource_probe() {
    let Some(root) = std::env::var_os(PROBE_ROOT_ENV).map(PathBuf::from) else {
        return;
    };
    assert_eq!(asset_path(LORE_BOOK_PATH), root.join(LORE_BOOK_PATH));
    let book = LoreBook::load(asset_path(LORE_BOOK_PATH)).unwrap();
    assert!(!book.characters.is_empty());
    let audio = AudioBank::load(asset_path(AUDIO_MANIFEST_PATH)).unwrap();
    assert!(!audio.clips().is_empty());
    CharacterBodyMetricsCatalog::load(asset_path(CHARACTER_BODY_METRICS_PATH)).unwrap();
    let sprite_path = asset_path("assets/placeholder/rust-fighter.sprite.json");
    let sprite = SpriteManifest::load(&sprite_path).unwrap();
    assert!(sprite.image_path(&sprite_path).starts_with(&root));
    SpriteManifest::load("custom.sprite.json").unwrap();

    // Rust's JSON/file APIs already handle Unicode on Windows. Decode through
    // Raylib too: its C file loader used to reject the exact same resolved paths.
    // Loading into CPU memory requires neither an OpenGL window nor a display.
    for relative in [
        ARENA_SIRIUS_PATH,
        "assets/placeholder/fighter-greybox-spritesheet.png",
    ] {
        let path = asset_path(relative);
        let image = raylib::prelude::Image::load_image(&path.to_string_lossy())
            .unwrap_or_else(|error| panic!("Raylib could not decode {}: {error}", path.display()));
        assert!(image.width > 0 && image.height > 0);
    }

    assert_eq!(data_dir(), root.parent().unwrap().join("user data"));
    assert!(
        !data_dir().exists(),
        "path resolution must not create files"
    );
    fs::create_dir_all(capture_dir()).unwrap();
    fs::write(capture_dir().join("probe.txt"), "writable").unwrap();
    assert!(!root.join("captures").exists());
    assert!(!std::env::current_dir().unwrap().join("captures").exists());
}
