---
name: borrow-fighters-gameplay-design
description: Propose, scope, or review Borrow Fighters gameplay mechanics for the 2D fighting prototype, especially movement, attack states, hitbox/hurtbox behavior, damage, knockback, character identity, prototype cuts, and avoiding premature fighting-game complexity.
---

# Borrow Fighters Gameplay Design

## Workflow

1. Read `docs/00-vision.md` for pillars.
2. Read `docs/01-mini-gdd.md` and `docs/02-prototype-scope.md` for current scope.
3. Use `references/prototype-combat.md` when judging a mechanic or character idea.
4. Prefer the smallest playable test over a complete system.
5. Call out whether the idea belongs in Prototype 0.1, later, or parked.

## Evaluation Checklist

- Does this make hit, damage, defense, or movement clearer?
- Can it be tested with rectangles/placeholders?
- Does it create a distinct character identity?
- Does it require animation, audio, AI, online, or tooling before core combat exists?
- Can it be reduced to one move, one state, or one debug visualization?

## Output Pattern

For proposals or reviews, respond with:

- verdict: `0.1`, `later`, or `parked`;
- smallest playable version;
- risks;
- docs/issues that should change.
