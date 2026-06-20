# AGENTS.md

## Repository Contract

- Work docs-first: read `README.md` and the relevant docs before changing behavior, process, art direction, or architecture.
- Use `docs/08-code-architecture.md` before creating Rust code.
- Use ADRs for structural decisions that affect more than one module, workflow, or contributor.
- Keep Prototype 0.1 small: two placeholders, local combat, hitbox/hurtbox, health, win condition, restart.
- Prefer small PRs with Conventional Commit titles.

## Rust/Raylib Rules

- Do not create code until the requested task explicitly asks for implementation.
- When creating Rust files, start each module with `//!` explaining the file responsibility in one or two short paragraphs.
- Keep `src/main.rs` thin; put testable gameplay logic under `src/lib.rs` modules.
- Keep Raylib details near `engine/*` and the app boundary. Combat rules should stay as Raylib-independent as practical.
- Avoid ECS, plugin systems, scripting, editor tooling, and asset pipelines until a real need appears.

## AI Workflow

- Use `$borrow-fighters-repo-atlas` before broad repo navigation.
- Use `$borrow-fighters-rust-gamedev` for Rust/Raylib architecture or implementation.
- Use `$borrow-fighters-gameplay-design` for combat mechanics and prototype scope.
- Use `$borrow-fighters-art-direction` for moods, characters, sprites, placeholders, and visual feedback.
- Read only the references needed for the task; do not load every doc by default.

## Verification

- For docs-only changes, validate Markdown links and GitHub YAML when relevant.
- For future Rust changes, run `cargo fmt`, `cargo clippy`, and `cargo test` when the project has a `Cargo.toml`.
- If verification cannot run, state why in the final response.
