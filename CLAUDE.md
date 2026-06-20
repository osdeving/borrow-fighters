# CLAUDE.md

## Project Rules

- Treat this as a docs-first Rust/Raylib 2D fighting game prototype.
- Start with `README.md`; route to the smallest relevant docs instead of reading the whole repo.
- Use `.claude/skills/` for project workflows and invoke a skill when the task matches it.
- Do not create Rust code unless the user asks for implementation.
- New Rust files must begin with module-level `//!` documentation explaining the file responsibility.
- Keep `main.rs` thin and place testable game logic under library modules.
- Keep Raylib at the application/engine boundary where practical.
- Avoid overengineering: no ECS, scripting, plugin architecture, editor, or asset pipeline before a proven prototype need.

## Useful Skills

- `/borrow-fighters-repo-atlas`: orient in the repository.
- `/borrow-fighters-rust-gamedev`: plan or implement Rust/Raylib changes.
- `/borrow-fighters-gameplay-design`: evaluate fighting-game mechanics.
- `/borrow-fighters-art-direction`: evaluate moods, characters, sprites, assets, and visual feedback.

## Verification

- Docs-only work: validate links/YAML when touched.
- Rust work: run `cargo fmt`, `cargo clippy`, and `cargo test` once `Cargo.toml` exists.
- Explain any verification that could not run.
