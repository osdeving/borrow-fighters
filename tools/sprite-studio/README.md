# Borrow Fighters Sprite Studio

Desktop tool for inspecting and editing Borrow Fighters `*.sprite.json` files outside the Raylib game loop.

This app is intentionally isolated from the main game code. It can read and write repository artifacts, but it must not import Rust modules, structs, crates or tests from the game.

This cut uses Tauri 1.8 because the current WSL/Ubuntu 20.04 baseline ships GLib/GIO 2.64.6. Tauri 2 currently pulls the Linux WebKitGTK/libsoup3 path that requires GLib/GIO 2.70.

## Commands

```bash
pnpm install
pnpm build
pnpm tauri dev
pnpm tauri build --debug
```

## Linux Prerequisites

Tauri requires native desktop libraries. On Ubuntu 20.04:

```bash
sudo apt-get install -y \
  libdbus-1-dev \
  libwebkit2gtk-4.0-dev \
  librsvg2-dev \
  libxdo-dev \
  libayatana-appindicator3-dev \
  libssl-dev
```

Newer Ubuntu releases may use `libwebkit2gtk-4.1-dev`; check the official Tauri prerequisite docs for the local OS.

## Current Scope

- Open a sprite manifest by repository-relative path or native file picker.
- Use Rust, Duke, Go and C presets.
- Render the selected atlas frame.
- Navigate clips and frames through the side list and horizontal timeline.
- Use desktop-style File/Edit/View/Help menus and a toolbar.
- Collapse the browser panel, inspector and timeline when the canvas needs space.
- Inspect grid, frame bounds, visual scale target, pivot, hitboxes, hurtboxes and projectile origin.
- Edit `scale`, `pivot`, `duration_ms`, `projectile_origin`, `hitboxes` and `hurtboxes`.
- Drag pivot, projectile origin and boxes directly on the canvas.
- Resize boxes through visual handles, hover cursors and optional snap.
- Zoom with the slider, `+`/`-` or mouse wheel over the canvas.
- Seed initial combat boxes for standing/crouch hurtbox, mid/low strike and projectile origin.
- Undo/redo local edits.
- Open the built-in guide with `F1` or Help.
- Autosave dirty work to `target/sprite-studio-autosave/`.
- Backup the previous manifest to `target/sprite-studio-backups/` before manual saves.
- Edit `assets/tuning/character-body-metrics.json`.
- Run runtime validation through the repository `cargo test` contract.
- Export a PNG review image.
- Export a JSON review package with notes, selected frame data and validation output.
- Save the manifest JSON back to disk.

## Artifact Contract

The Sprite Studio produces files that the game consumes:

- `assets/placeholder/*.sprite.json`
- PNG atlas files referenced by `image`

Run the game test suite after saving production changes:

```bash
cd ../..
cargo test --all-targets
```
