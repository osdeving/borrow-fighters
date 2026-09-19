# AGENTS.md

## Working contract

- Start from the task and Git state; preserve unrelated work.
- Read the README index and the smallest relevant section once. Use
  `$borrow-fighters-repo-atlas` as a router, not a request to load every doc.
- Rust/Raylib: use `$borrow-fighters-rust-gamedev` and the relevant section of
  doc 08. Use gameplay/art skills only for those decisions.
- Implement when requested. Keep changes small; use Conventional Commit titles.
- Keep adventure and fighting independent; share only neutral utilities.
- New Rust modules start with a short `//!` responsibility. Keep `main.rs` thin,
  gameplay testable and Raylib near the app/engine boundary.
- Reuse existing code/tools. Avoid speculative frameworks, parallel editors,
  compatibility layers or a workspace migration.

## Context and documentation budget

- Search filenames/symbols with `rg`; read focused sections. Do not reread
  unchanged docs or paste large logs. Reuse findings from the conversation.
- Routine fixes need code/assets and relevant verification. Update docs only
  for changed behavior, commands or contracts; do not create a doc per task.
- ADRs are for durable, costly-to-reverse decisions. Reversible art experiments
  and local refactors do not require ADRs, issues or backlog entries.
- Sustained work uses ONE short journal in `docs/worklogs/`: branch/base,
  checkpoint, next steps and commands/results. Replace stale status rather
  than duplicating conversation/logs. Commit coherent stages when authorized.
- Delegate bounded independent tasks with file ownership and minimal context.
  Root consolidates checks and commits; do not duplicate agents' inspections.

## Verification levels

- Iteration: `./dev check` or `./dev check fast adventure|fighting`, plus only
  directly affected tests. Assets-only edits need asset validation and a short
  visual check, not an automatic Rust rebuild or test matrix.
- Rust handoff: fmt, Clippy and tests of the affected domain via
  `./dev check adventure` or `./dev check fighting`, once after integration.
  Cross-domain changes also need a combined build.
- Full matrix, repository-wide docs/YAML, packaging and broad captures belong
  to CI/merge/release (`./dev check full`), or changes that actually affect them.
- Add regression tests for real behavioral risks, not wording, file presence,
  constants or reversible visual tuning. Keep useful tests; remove tests of
  removed features with their code.
- After checks pass, do not repeat/broaden them without a new change or concern.
  Report concrete results/limitations. A build does not approve an animation.

Commands and asset ownership: [workflow](docs/09-ai-collaboration.md).
