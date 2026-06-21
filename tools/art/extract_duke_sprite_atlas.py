#!/usr/bin/env python3
"""Extracts a clean Duke fighter atlas from the current reference sheet."""

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
    FrameSpec("idle_0", "idle", (38, 38, 160, 208), (100, 205), 140),
    FrameSpec("idle_1", "idle", (202, 34, 318, 208), (260, 205), 140),
    FrameSpec("idle_2", "idle", (370, 34, 486, 208), (426, 205), 140),
    FrameSpec("idle_3", "idle", (518, 40, 636, 208), (576, 205), 140),
    FrameSpec("idle_4", "idle", (654, 38, 772, 208), (710, 205), 140),
    FrameSpec("walk_0", "walk", (20, 272, 152, 434), (90, 431), 90),
    FrameSpec("walk_1", "walk", (164, 270, 290, 432), (230, 431), 90),
    FrameSpec("walk_2", "walk", (300, 270, 426, 434), (366, 431), 90),
    FrameSpec("walk_3", "walk", (438, 270, 564, 434), (505, 431), 90),
    FrameSpec("walk_4", "walk", (578, 272, 704, 435), (648, 432), 90),
    FrameSpec("crouch_0", "crouch", (748, 312, 866, 435), (812, 433), 120),
    FrameSpec("crouch_1", "crouch", (876, 314, 990, 435), (942, 433), 120),
    FrameSpec("jump_0", "jump", (1018, 230, 1160, 430), (1090, 426), 120),
    FrameSpec("jump_1", "jump", (1160, 218, 1300, 392), (1235, 386), 120),
    FrameSpec("jump_2", "jump", (1282, 300, 1445, 435), (1360, 431), 120),
    FrameSpec("punch_0", "punch_light", (10, 490, 142, 650), (78, 647), 80),
    FrameSpec("punch_1", "punch_light", (150, 490, 366, 652), (225, 648), 80),
    FrameSpec("punch_2", "punch_heavy", (358, 492, 492, 653), (423, 650), 110),
    FrameSpec("kick_0", "kick", (538, 490, 666, 656), (610, 653), 95),
    FrameSpec("kick_1", "kick", (688, 492, 878, 656), (762, 653), 95),
    FrameSpec("kick_2", "kick", (878, 498, 992, 654), (930, 652), 95),
    FrameSpec("block_0", "block", (16, 700, 175, 850), (92, 847), 120),
    FrameSpec("block_1", "block", (176, 696, 326, 850), (255, 847), 120),
    FrameSpec("hit_0", "hit", (1018, 492, 1190, 657), (1115, 655), 120),
    FrameSpec("hit_1", "hit", (1208, 494, 1360, 656), (1290, 654), 120),
    FrameSpec("taunt_0", "taunt", (488, 700, 622, 854), (560, 852), 180),
    FrameSpec("taunt_1", "taunt", (632, 680, 786, 854), (705, 852), 180),
    FrameSpec("special_0", "special", (0, 890, 205, 1064), (88, 1060), 100),
    FrameSpec("special_1", "special", (210, 884, 446, 1062), (310, 1060), 100),
    FrameSpec("special_2", "special", (450, 884, 708, 1064), (555, 1060), 100),
    FrameSpec("projectile_0", "projectile", (815, 930, 950, 1046), (882, 990), 80),
    FrameSpec("projectile_1", "projectile", (960, 932, 1218, 1046), (1090, 990), 80),
    FrameSpec("projectile_2", "projectile", (1220, 1002, 1448, 1038), (1334, 1020), 80),
]

CONFIG = AtlasConfig(
    source_path=ROOT / "assets/references/duke-sprite.png",
    atlas_path=ROOT / "assets/placeholder/duke-fighter-atlas.png",
    manifest_path=ROOT / "assets/placeholder/duke-fighter.sprite.json",
    projectile_path=ROOT / "assets/placeholder/duke-bean-projectile.png",
    preview_path=ROOT / "tmp/art/duke-fighter-atlas-preview.png",
    frames=FRAMES,
    clips=CLIPS,
    cell_width=384,
    projectile_frame_name="projectile_0",
)


if __name__ == "__main__":
    write_outputs(CONFIG)
