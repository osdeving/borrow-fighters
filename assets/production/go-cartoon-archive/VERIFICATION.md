# Go candidate review

The candidate contains 20 explicit clips and 59 selected frames generated through the built-in image tool. Existing narrow Go identity, pale muzzle/feet, blue belly, dark gloves and belt were the fixed references. Source sheets, prompts and rejected revisions remain alongside each action.

## Preparation and automated validation

- `prepare_reviewed_actions.py assets/production/go/review-plan.json`: 20 actions prepared with one uniform scale per action. Idle uses scale 0.5; its selected source poses are 526, 537 and 521 px tall before resampling (about 263, 269 and 261 runtime pixels). Deep crouch art retains head size instead of shrinking the whole character.
- `build_reviewed_sprite_atlas.py assets/production/go/production.json`: complete candidate exported to `assets/candidates/go/go-fighter.sprite.json`.
- `render_sprite_review.py assets/candidates/go/go-fighter.sprite.json`: all 20 GIFs, frame strips and overview produced with dark/right-facing and light/mirrored views.
- `cargo test --test sprite_candidates`: both tests passed. Actual candidate frames match startup/active/recovery at every combat tick, including active start and inclusive active end. Candidate clips contain no combat metadata.

Magenta extraction used `remove_generated_magenta.py` without `--rust-contour`. Isolated one-pixel matte remnants at otherwise blank sheet corners were removed in hit, throw, air_punch and punch_light; no blue character contour was removed. The originals remain intact.

## Historical native GPU evidence

Command: `env -u WAYLAND_DISPLAY BORROW_FIGHTERS_SPRITE_CANDIDATES=1 cargo run --example capture_sprite_review -- go target/art/go-combat-lab-final`.

The native Combat Lab finished successfully using the NVIDIA D3D12/Mesa OpenGL device, producing 270 PNGs and `target/art/go-combat-lab-final/capture-report.json`. Its loaded visual manifest matched the Go candidate at capture time. All 19 contexts supported by the Lab were captured, including the nine close attacks, emission, hit, both guards, jump, entrance and outcomes. First-active screenshots were visually inspected for all nine close attacks; emission was inspected at tick 1. The character remains visible and all grounded poses meet the floor.

This capture predates the Lab baseline wiring fix. All nine close attacks use MoveSpec, so their contact evidence remains applicable. The old Lab created projectiles through `Projectile::from_fighter` without reading the baseline origin, however; matching the visual manifest did not verify that combat path. The emission screenshot does not prove the World projectile origin.

The [corrected special capture](../../candidates/go/review/runtime-baseline-special/README.md) contains 19 samples with both manifests checked. It confirms the baseline origin and the channel effect at the emitting palm, closing this technical gap.

## Remaining art and review limits

- Light and heavy punch gloves overlap the existing hitboxes, while the forward fingertips extend slightly beyond their far edges. Sweep and overhead meet the upper portion of their boxes; some upper glove/toe pixels exceed that boundary. AirKick's extended foot overlaps the upper portion, with the toe tip above it. AntiAir and Throw have clear contact overlap.
- Special v2 moves the palm toward the baseline emission point estimated from the source. The original separate projectile is reused. The apparent palm alignment in the historical Lab screenshot is not evidence of the World origin; that origin is now confirmed by the corrected capture linked above.
- Walk selects two genuinely different poses from v2 as a short martial advancing cycle. Duplicate pairs in v1/v2 and the exaggerated bob in v3 were rejected. The two-frame cycle has limited fluidity.
- Jump uses takeoff/rise/fall at 60/340/400 ms. Rise/fall pivots register the belt and torso rather than the tucked feet, avoiding a second embedded upward translation. The static Lab does not verify a complete physical jump trajectory.
- The Lab has no walk pose and uses native right-facing Go. Walking, full jumps, outcome transitions and both orientations in an actual Go match are not claimed by this capture. Mirrored appearance was reviewed in the generated previews.
- This is a reviewed visual candidate, not final-art approval. All baseline boxes, move timing, physical dimensions and projectile origins remain authoritative.

## Full World motion capture

The later controlled World scenario completed 122 GPU captures and 585 simulation ticks with Go on both facings simultaneously: real intro/countdown, advance/retreat, return to idle, one full jump and grounded return. Video and summary are in [runtime-motion](../../candidates/go/review/runtime-motion/README.md); complete evidence is local at `target/sprite-production/go-motion-final`. Loaded candidate matched disk throughout. The reviewer inspected walk and the consecutive apex/rise-to-fall frames: head/torso remain aligned and both silhouettes stay visible; tucked legs unfold during descent. Health remains 92 for both fighters, and landing selects idle at tick 543. This is deterministic simulation input, not a manual keyboard or combat outcome playtest.
