---
name: borrow-fighters-rust-gamedev
description: Plan, implement, review, or refactor Borrow Fighters Rust/Raylib code. Use for source layout, game loop, module boundaries, hitbox/hurtbox code, tests, Rust documentation comments, and keeping gameplay logic testable without overengineering.
---

# Borrow Fighters Rust Gamedev

## Workflow

1. Read `docs/08-code-architecture.md`.
2. Read `docs/adr/0001-stack-rust-raylib.md` and `docs/adr/0003-code-architecture-rust-raylib.md` for structural decisions.
3. For implementation, create only modules required by the current task.
4. Keep `src/main.rs` thin and move testable logic into `src/lib.rs` modules.
5. Keep Raylib types near `engine/*` or app boundaries unless using them directly reduces complexity.
6. Add module-level `//!` documentation to every new Rust file.
7. Verify with `cargo fmt`, `cargo clippy`, and `cargo test` when `Cargo.toml` exists.

## Architecture Bias

- Prefer one Cargo package until the prototype proves a need for workspace crates.
- Prefer explicit state structs and simple systems over ECS.
- Prefer plain data and functions for combat rules.
- Prefer polling/input translation over event buses.
- Prefer fixed timestep for gameplay update once code exists.
- Avoid asset pipeline, scripting, plugins, and editor tooling in Prototype 0.1.

## References

- Read `references/rust-raylib-rules.md` for code boundaries and file header rules.
