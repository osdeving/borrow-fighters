# Repo Map

## Root

| Path | Purpose |
|---|---|
| `README.md` | Central documentation index |
| `CONTRIBUTING.md` | Contributor workflow |
| `CHANGELOG.md` | Human-readable change log |
| `AGENTS.md` | Codex project instructions |
| `CLAUDE.md` | Claude Code project instructions |

## Documentation

| Path | Purpose |
|---|---|
| `docs/00-vision.md` | Game vision and pillars |
| `docs/01-mini-gdd.md` | Mini-GDD |
| `docs/02-prototype-scope.md` | Prototype 0.1 scope |
| `docs/03-backlog.md` | Initial backlog |
| `docs/04-team-briefing.md` | Contributor briefing |
| `docs/05-governance.md` | PRs, branches, labels, roles, squads |
| `docs/06-release-process.md` | Releases, tags, milestones |
| `docs/07-art-direction.md` | Art direction and moods |
| `docs/08-code-architecture.md` | Rust/Raylib code architecture plan |
| `docs/09-ai-collaboration.md` | AI guidance and skills |
| `docs/adr/` | Architecture Decision Records |
| `docs/templates/` | Reusable proposal/checklist templates |

## Future Code

| Path | Intended purpose |
|---|---|
| `src/main.rs` | Thin executable entrypoint |
| `src/lib.rs` | Testable internal game modules |
| `src/engine/` | Raylib adapters and platform boundary |
| `src/game/` | Match state and world flow |
| `src/combat/` | Hitbox, hurtbox, damage and fighter rules |
| `src/characters/` | Character-specific data/logic |
| `src/scenes/` | Boot, fight, victory scenes |
| `src/ui/` | HUD and debug UI |
| `src/math/` | Small geometry helpers |
| `examples/` | Future isolated experiments |
| `tests/` | Future integration tests |

## Assets

| Path | Purpose |
|---|---|
| `assets/placeholder/` | Temporary assets for prototype work |
| `assets/references/` | Visual references and mood inputs |
