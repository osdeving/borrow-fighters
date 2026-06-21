#!/usr/bin/env python3
"""Extracts a clean Rust fighter atlas from the current reference sheet."""

from __future__ import annotations

from sprite_atlas_extractor import ROOT, AtlasConfig, FrameSpec, write_outputs


CLIPS: dict[str, dict[str, object]] = {
    "idle": {"loop": True},
    "walk": {"loop": True},
    "crouch": {"loop": False},
    "jump": {"loop": False},
    "punch_light": {"loop": False},
    "punch_heavy": {"loop": False},
    "kick": {"loop": False},
    "block": {"loop": True},
    "hit": {"loop": False},
    "taunt": {"loop": False},
    "special": {"loop": False},
    "projectile": {"loop": True},
}

FRAMES: list[FrameSpec] = [
    FrameSpec("idle_0", "idle", (32, 40, 160, 232), (94, 224), 140),
    FrameSpec("idle_1", "idle", (170, 40, 300, 232), (235, 224), 140),
    FrameSpec("idle_2", "idle", (310, 40, 440, 232), (374, 224), 140),
    FrameSpec("idle_3", "idle", (450, 40, 580, 232), (515, 225), 140),
    FrameSpec("idle_4", "idle", (582, 40, 710, 232), (644, 225), 140),
    FrameSpec("walk_0", "walk", (10, 270, 142, 448), (76, 438), 90),
    FrameSpec("walk_1", "walk", (148, 270, 282, 448), (213, 438), 90),
    FrameSpec("walk_2", "walk", (292, 270, 424, 448), (356, 439), 90),
    FrameSpec("walk_3", "walk", (434, 270, 562, 448), (496, 439), 90),
    FrameSpec("walk_4", "walk", (574, 270, 704, 448), (638, 439), 90),
    FrameSpec("crouch_0", "crouch", (736, 312, 858, 448), (800, 439), 120),
    FrameSpec("crouch_1", "crouch", (856, 312, 982, 448), (917, 439), 120),
    FrameSpec("jump_0", "jump", (996, 280, 1142, 456), (1068, 445), 120),
    FrameSpec("jump_1", "jump", (1134, 236, 1280, 400), (1204, 390), 120),
    FrameSpec("jump_2", "jump", (1274, 296, 1420, 452), (1348, 440), 120),
    FrameSpec("punch_0", "punch_light", (0, 492, 142, 668), (70, 657), 80),
    FrameSpec("punch_1", "punch_light", (150, 492, 332, 668), (220, 657), 80),
    FrameSpec("punch_2", "punch_heavy", (356, 492, 486, 668), (412, 657), 110),
    FrameSpec("kick_0", "kick", (520, 492, 656, 668), (586, 658), 95),
    FrameSpec("kick_1", "kick", (670, 492, 835, 668), (748, 657), 95),
    FrameSpec("kick_2", "kick", (858, 492, 970, 668), (900, 657), 95),
    FrameSpec("block_0", "block", (8, 694, 164, 866), (85, 855), 120),
    FrameSpec("block_1", "block", (162, 694, 326, 866), (245, 855), 120),
    FrameSpec("hit_0", "hit", (1018, 498, 1204, 668), (1110, 657), 120),
    FrameSpec("hit_1", "hit", (1208, 512, 1340, 668), (1280, 656), 120),
    FrameSpec("taunt_0", "taunt", (476, 692, 608, 866), (540, 855), 180),
    FrameSpec("taunt_1", "taunt", (616, 692, 782, 866), (685, 853), 180),
    FrameSpec("special_0", "special", (0, 890, 178, 1070), (82, 1058), 100),
    FrameSpec("special_1", "special", (216, 888, 438, 1070), (296, 1059), 100),
    FrameSpec("special_2", "special", (458, 888, 704, 1070), (546, 1058), 100),
    FrameSpec("projectile_0", "projectile", (730, 898, 956, 1032), (842, 1014), 80),
    FrameSpec("projectile_1", "projectile", (972, 894, 1210, 1032), (1092, 1015), 80),
    FrameSpec("projectile_2", "projectile", (1232, 892, 1448, 1032), (1344, 1015), 80),
]

CONFIG = AtlasConfig(
    source_path=ROOT / "assets/references/sprinte-rust.png",
    atlas_path=ROOT / "assets/placeholder/rust-fighter-atlas.png",
    manifest_path=ROOT / "assets/placeholder/rust-fighter.sprite.json",
    projectile_path=ROOT / "assets/placeholder/rust-gear-projectile.png",
    preview_path=ROOT / "tmp/art/rust-fighter-atlas-preview.png",
    frames=FRAMES,
    clips=CLIPS,
)


if __name__ == "__main__":
    write_outputs(CONFIG)
