# Rust/Raylib Rules

## File Header

Every Rust source file must start with a module-level summary:

```rust
//! One-sentence responsibility of this module.
//!
//! Optional short context explaining why this file exists and what belongs here.
```

Use `///` for public items. Keep the first sentence short because rustdoc uses it in summaries.

## Boundaries

- `main.rs`: initialize platform, create app, run loop.
- `lib.rs`: expose modules for tests/examples.
- `engine/*`: Raylib adapters, drawing, input mapping, time, assets, debug draw.
- `combat/*`: pure-ish gameplay rules and data.
- `game/*`: match state, world state, round flow.
- `characters/*`: character definitions and move data.
- `scenes/*`: screen states.
- `ui/*`: HUD and debug UI.
- `math/*`: local geometry helpers only when Raylib types are insufficient.

## Implementation Heuristics

- Start with fewer files than the architecture map if the task is small.
- Add a module when it has a stable responsibility, not just to make the tree symmetrical.
- Keep collision and damage deterministic and easy to unit test.
- Use placeholder rectangles/colors before sprite systems.
- Add debug draw before polished rendering for combat systems.
- Do not introduce global mutable state.

## Verification

When code exists:

```text
cargo fmt
cargo clippy --all-targets --all-features
cargo test --all-targets
```
