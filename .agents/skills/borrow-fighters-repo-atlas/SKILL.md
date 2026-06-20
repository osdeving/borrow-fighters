---
name: borrow-fighters-repo-atlas
description: Orient in the Borrow Fighters repository before broad edits, documentation routing, architecture work, issue triage, PR planning, or when an agent needs to find the smallest relevant context instead of reading the whole repo.
---

# Borrow Fighters Repo Atlas

## Workflow

1. Start with `README.md` to understand the current index.
2. Read `references/repo-map.md` for the shortest route to the right files.
3. Load only the docs needed for the task.
4. If the task changes process, architecture, art direction, release, or contributor rules, check whether an ADR or governance doc update is required.
5. Report the files you used when handing off a plan or summary.

## Routing Rules

- Product/vision: `docs/00-vision.md`, `docs/01-mini-gdd.md`.
- Prototype scope: `docs/02-prototype-scope.md`, `docs/03-backlog.md`.
- Contribution/process: `CONTRIBUTING.md`, `docs/05-governance.md`.
- Release: `docs/06-release-process.md`, `CHANGELOG.md`.
- Art/mood/characters: `docs/07-art-direction.md`.
- Code architecture: `docs/08-code-architecture.md`, `docs/adr/0003-code-architecture-rust-raylib.md`.
- AI collaboration: `AGENTS.md`, `CLAUDE.md`, `docs/09-ai-collaboration.md`.

## Guardrails

- Do not read every file by default.
- Do not create code when the user asks only for architecture or planning.
- Prefer updating the index when adding a new durable doc.
- Prefer issues/ADR for open decisions instead of hiding them in prose.
