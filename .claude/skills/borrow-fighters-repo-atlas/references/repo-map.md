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
| `docs/10-greybox-playtest.md` | Current playtest script |
| `docs/11-sprite-pipeline.md` | Sprite manifest and atlas pipeline |
| `docs/13-combat-design-roadmap.md` | Combat design, frame data and Combat Lab roadmap |
| `docs/adr/` | Architecture Decision Records |
| `docs/templates/` | Reusable proposal/checklist templates |

## Code

| Path | Purpose |
|---|---|
| `src/main.rs` | Thin executable entrypoint |
| `src/lib.rs` | Testable internal game modules |
| `src/cli.rs` | Startup argument parser, including Combat Lab mode |
| `src/engine/` | Raylib adapters and platform boundary |
| `src/engine/sprites/` | Sprite manifest loading, animation and drawing |
| `src/game/` | Match state and world flow |
| `src/game/ai.rs` | Simple CPU behavior for playtest |
| `src/game/feature_flags.rs` | Runtime feature flags and preference menu data |
| `src/combat/` | Hitbox, hurtbox, damage and fighter rules |
| `src/characters/` | Reserved for future character-specific data/logic |
| `src/scenes/` | Preference screen and isolated Combat Lab scene state |
| `src/ui/` | Reserved for future UI components |
| `src/math/` | Small geometry helpers |
| `tests/` | Integration tests for combat, flags, CLI, Combat Lab and sprites |
| `tools/art/` | Local asset extraction utilities |

## Assets

| Path | Purpose |
|---|---|
| `assets/placeholder/` | Temporary assets for prototype work |
| `assets/references/` | Visual references and mood inputs |
