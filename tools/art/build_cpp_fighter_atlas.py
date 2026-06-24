#!/usr/bin/env python3
"""Builds the high-resolution C++ fighter runtime atlases.

The source sheet is a generated raster pose sheet with chroma-keyed full-body
poses. This builder repacks those painted poses into two runtime atlases and
uses per-frame manifest image overrides so one character can span more than one
spritesheet.
"""

from __future__ import annotations

import importlib.util
import json
import math
from pathlib import Path
from typing import Any

import numpy as np
from PIL import Image, ImageDraw, ImageFilter

ROOT = Path(__file__).resolve().parents[2]
PYTHON_BUILDER_PATH = ROOT / "tools/art/build_python_high_res_fighter_atlas.py"
SOURCE_PATH = ROOT / "assets/references/cpp-fighter-raster-source.png"
SKELETON_MANIFEST_PATH = ROOT / "assets/placeholder/python-fighter.sprite.json"
ATLAS_A_PATH = ROOT / "assets/placeholder/cpp-fighter-atlas-a.png"
ATLAS_B_PATH = ROOT / "assets/placeholder/cpp-fighter-atlas-b.png"
MANIFEST_PATH = ROOT / "assets/placeholder/cpp-fighter.sprite.json"
ROSTER_PATH = ROOT / "assets/placeholder/roster-cpp.png"
PROJECTILE_PATH = ROOT / "assets/placeholder/cpp-plusplus-projectile.png"

SOURCE_COLUMNS = 5
SOURCE_ROWS = 4
TARGET_COLUMNS = 6
FRAMES_PER_ATLAS = 56
CELL_WIDTH = 768
CELL_HEIGHT = 512
DEFAULT_PIVOT = {"x": 224, "y": 472}
PROJECTILE_PIVOT = {"x": 408, "y": 264}
RUNTIME_SCALE = 2.0 / 3.0

GENERATED_NOTE = (
    "High-resolution raster C++ fighter atlas generated from "
    "assets/references/cpp-fighter-raster-source.png. Frames are split across "
    "cpp-fighter-atlas-a.png and cpp-fighter-atlas-b.png using per-frame image "
    "overrides."
)

POSE_GRID = {
    "idle": (0, 0),
    "walk": (1, 0),
    "crouch": (2, 0),
    "jump": (3, 0),
    "block": (4, 0),
    "hit": (0, 1),
    "punch_light": (1, 1),
    "punch_heavy": (2, 1),
    "kick": (3, 1),
    "sweep": (4, 1),
    "overhead": (0, 2),
    "anti_air": (1, 2),
    "air_punch": (2, 2),
    "air_kick": (3, 2),
    "throw": (4, 2),
    "special": (0, 3),
    "point": (1, 3),
    "victory": (2, 3),
    "taunt": (3, 3),
    "arms_crossed": (4, 3),
}

CLIP_POSES = {
    "idle": ["idle", "idle", "idle", "idle", "idle", "idle", "idle"],
    "walk": ["walk", "idle", "walk", "idle", "walk", "idle", "walk"],
    "crouch": ["idle", "crouch", "crouch", "crouch"],
    "jump": ["idle", "jump", "jump", "jump", "jump", "idle"],
    "block": ["idle", "block", "block", "block"],
    "hit": ["idle", "hit", "hit", "idle"],
    "knockdown": ["hit", "hit", "crouch", "crouch", "hit", "idle"],
    "punch_light": ["idle", "punch_light", "punch_light", "punch_light", "idle"],
    "punch_medium": [
        "idle",
        "punch_light",
        "punch_heavy",
        "punch_heavy",
        "punch_light",
        "idle",
    ],
    "punch_heavy": [
        "idle",
        "punch_light",
        "punch_heavy",
        "punch_heavy",
        "punch_heavy",
        "punch_light",
        "idle",
    ],
    "kick_light": ["idle", "kick", "kick", "idle", "idle"],
    "kick": ["idle", "kick", "kick", "kick", "idle", "idle"],
    "kick_heavy": ["idle", "kick", "kick", "kick", "air_kick", "kick", "idle"],
    "sweep": ["crouch", "sweep", "sweep"],
    "overhead": ["idle", "overhead", "overhead"],
    "anti_air": ["crouch", "anti_air", "anti_air"],
    "air_punch": ["jump", "air_punch", "air_punch"],
    "air_kick": ["jump", "air_kick", "air_kick"],
    "throw": ["idle", "throw", "throw"],
    "special": ["idle", "point", "special", "special", "point", "idle"],
    "taunt": ["taunt", "point", "taunt", "arms_crossed", "taunt", "point"],
    "victory": ["idle", "victory", "victory", "point", "victory", "taunt", "victory"],
    "projectile": ["special"],
}


def main() -> None:
    python_builder = load_python_builder()
    skeleton = json.loads(SKELETON_MANIFEST_PATH.read_text())
    source = Image.open(SOURCE_PATH).convert("RGBA")
    poses = extract_source_poses(source, python_builder)
    frames = skeleton["frames"]
    clips = skeleton["clips"]
    frame_to_clip_index = python_builder.clip_indices(clips)
    atlas_rows = math.ceil(FRAMES_PER_ATLAS / TARGET_COLUMNS)
    atlas_a = Image.new("RGBA", (TARGET_COLUMNS * CELL_WIDTH, atlas_rows * CELL_HEIGHT), (0, 0, 0, 0))
    atlas_b = Image.new("RGBA", atlas_a.size, (0, 0, 0, 0))
    output_frames: list[dict[str, Any]] = []

    for frame_index, frame in enumerate(frames):
        frame_name = frame["name"]
        clip_name = frame["clip"]
        index_in_clip, clip_len = frame_to_clip_index[frame_name]
        pose_name = pose_name_for_frame(clip_name, index_in_clip)
        cell = python_builder.pack_pose_cell(poses[pose_name], pose_name, index_in_clip, clip_len)
        atlas, local_index, image_name = atlas_for_frame(frame_index, atlas_a, atlas_b)
        rect = frame_rect(local_index)
        atlas.alpha_composite(cell, (rect["x"], rect["y"]))

        record: dict[str, Any] = {
            "name": frame_name,
            "clip": clip_name,
            "duration_ms": frame["duration_ms"],
            "pivot": PROJECTILE_PIVOT if frame_name == "projectile_0" else DEFAULT_PIVOT,
            "frame": rect,
        }
        if image_name != ATLAS_A_PATH.name:
            record["image"] = image_name
        bounds = python_builder.alpha_bounds(cell)
        if bounds is not None:
            record["source_crop"] = bounds
            record["trimmed_bounds"] = bounds
        combat = combat_metadata(frame_name)
        if combat is not None:
            record["combat"] = combat
        output_frames.append(record)

    manifest = {
        "schema": "borrow-fighters.sprite.v1",
        "image": ATLAS_A_PATH.name,
        "source": str(SOURCE_PATH.relative_to(ROOT)),
        "cell": {"w": CELL_WIDTH, "h": CELL_HEIGHT},
        "default_pivot": DEFAULT_PIVOT,
        "scale": RUNTIME_SCALE,
        "notes": [
            GENERATED_NOTE,
            "Close-range attack hitboxes intentionally fall back to MoveSpec until the raster frames are calibrated in Sprite Studio.",
            "The source pose sheet depicts C++ as a daughter of C through operator/projectile motifs and a compact high-level stance.",
        ],
        "frames": output_frames,
        "clips": clips,
    }

    ATLAS_A_PATH.parent.mkdir(parents=True, exist_ok=True)
    atlas_a.save(ATLAS_A_PATH)
    atlas_b.save(ATLAS_B_PATH)
    MANIFEST_PATH.write_text(json.dumps(manifest, indent=2) + "\n")
    write_roster_portrait(poses["idle"])
    write_projectile_texture()
    print(f"wrote {ATLAS_A_PATH.relative_to(ROOT)}")
    print(f"wrote {ATLAS_B_PATH.relative_to(ROOT)}")
    print(f"wrote {MANIFEST_PATH.relative_to(ROOT)}")
    print(f"wrote {ROSTER_PATH.relative_to(ROOT)}")
    print(f"wrote {PROJECTILE_PATH.relative_to(ROOT)}")


def load_python_builder() -> Any:
    spec = importlib.util.spec_from_file_location("python_atlas_builder", PYTHON_BUILDER_PATH)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"could not load {PYTHON_BUILDER_PATH}")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def extract_source_poses(source: Image.Image, python_builder: Any) -> dict[str, Image.Image]:
    cell_width = source.width / SOURCE_COLUMNS
    cell_height = source.height / SOURCE_ROWS
    poses: dict[str, Image.Image] = {}
    for pose_name, (column, row) in POSE_GRID.items():
        left = round(column * cell_width)
        top = round(row * cell_height)
        right = round((column + 1) * cell_width)
        bottom = round((row + 1) * cell_height)
        cell = source.crop((left, top, right, bottom)).convert("RGBA")
        cell = python_builder.remove_chroma_key(cell)
        cell = erase_border(cell)
        cell = remove_grid_line_components(cell)
        cell = python_builder.remove_small_alpha_components(cell)
        poses[pose_name] = python_builder.crop_to_subject(cell)
    return poses


def erase_border(image: Image.Image, width: int = 5) -> Image.Image:
    pixels = np.array(image)
    pixels[:width, :, 3] = 0
    pixels[-width:, :, 3] = 0
    pixels[:, :width, 3] = 0
    pixels[:, -width:, 3] = 0
    pixels[pixels[..., 3] == 0, :3] = 0
    return Image.fromarray(pixels, "RGBA")


def remove_grid_line_components(image: Image.Image) -> Image.Image:
    alpha = np.array(image.getchannel("A"))
    height, width = alpha.shape
    visited = np.zeros((height, width), dtype=bool)
    keep = np.ones((height, width), dtype=bool)

    for y in range(height):
        for x in range(width):
            if visited[y, x] or alpha[y, x] == 0:
                continue
            stack = [(x, y)]
            visited[y, x] = True
            component: list[tuple[int, int]] = []
            min_x = max_x = x
            min_y = max_y = y
            while stack:
                current_x, current_y = stack.pop()
                component.append((current_x, current_y))
                min_x = min(min_x, current_x)
                max_x = max(max_x, current_x)
                min_y = min(min_y, current_y)
                max_y = max(max_y, current_y)
                for next_y in range(current_y - 1, current_y + 2):
                    if next_y < 0 or next_y >= height:
                        continue
                    for next_x in range(current_x - 1, current_x + 2):
                        if next_x < 0 or next_x >= width:
                            continue
                        if visited[next_y, next_x] or alpha[next_y, next_x] == 0:
                            continue
                        visited[next_y, next_x] = True
                        stack.append((next_x, next_y))

            box_width = max_x - min_x + 1
            box_height = max_y - min_y + 1
            thin_vertical = box_width <= 24 and box_height >= 72
            thin_horizontal = box_height <= 14 and box_width >= 96
            if thin_vertical or thin_horizontal:
                for point_x, point_y in component:
                    keep[point_y, point_x] = False

    pixels = np.array(image)
    pixels[~keep, 3] = 0
    pixels[pixels[..., 3] == 0, :3] = 0
    return Image.fromarray(pixels, "RGBA")


def pose_name_for_frame(clip: str, index: int) -> str:
    sequence = CLIP_POSES.get(clip, ["idle"])
    return sequence[min(index, len(sequence) - 1)]


def atlas_for_frame(
    frame_index: int,
    atlas_a: Image.Image,
    atlas_b: Image.Image,
) -> tuple[Image.Image, int, str]:
    if frame_index < FRAMES_PER_ATLAS:
        return atlas_a, frame_index, ATLAS_A_PATH.name
    return atlas_b, frame_index - FRAMES_PER_ATLAS, ATLAS_B_PATH.name


def frame_rect(index: int) -> dict[str, int]:
    column = index % TARGET_COLUMNS
    row = index // TARGET_COLUMNS
    return {
        "x": column * CELL_WIDTH,
        "y": row * CELL_HEIGHT,
        "w": CELL_WIDTH,
        "h": CELL_HEIGHT,
    }


def combat_metadata(frame_name: str) -> dict[str, Any] | None:
    if frame_name == "special_0":
        return {"projectile_origin": {"x": 532, "y": 236}}
    return None


def write_roster_portrait(idle_pose: Image.Image) -> None:
    card = Image.new("RGBA", (240, 240), (20, 36, 54, 255))
    for y in range(card.height):
        blend = y / max(1, card.height - 1)
        color = (18 + round(20 * blend), 30 + round(28 * blend), 45 + round(50 * blend), 255)
        card.paste(color, (0, y, card.width, y + 1))

    sprite = idle_pose.copy()
    sprite.thumbnail((158, 208), Image.Resampling.LANCZOS)
    card.alpha_composite(sprite, ((card.width - sprite.width) // 2, 18 + (196 - sprite.height) // 2))

    draw = ImageDraw.Draw(card, "RGBA")
    draw.ellipse((42, 194, 198, 225), fill=(0, 0, 0, 92))
    draw.rounded_rectangle((7, 7, 233, 233), radius=10, outline=(118, 214, 255, 255), width=4)
    draw.line((42, 34, 196, 34), fill=(255, 213, 82, 190), width=2)
    card.save(ROSTER_PATH)


def write_projectile_texture() -> None:
    width, height = 256, 128
    image = Image.new("RGBA", (width, height), (0, 0, 0, 0))
    glow = Image.new("RGBA", image.size, (0, 0, 0, 0))
    glow_draw = ImageDraw.Draw(glow, "RGBA")
    draw_cpp_burst(glow_draw, 0, 0, glow=True)
    glow = glow.filter(ImageFilter.GaussianBlur(7))
    image.alpha_composite(glow)

    draw = ImageDraw.Draw(image, "RGBA")
    draw_cpp_burst(draw, 0, 0, glow=False)
    image.save(PROJECTILE_PATH)


def draw_cpp_burst(draw: ImageDraw.ImageDraw, offset_x: int, offset_y: int, glow: bool) -> None:
    cyan = (103, 224, 255, 96 if glow else 245)
    gold = (255, 213, 82, 74 if glow else 238)
    white = (245, 252, 255, 68 if glow else 245)
    line_width = 13 if glow else 5

    for base_x, base_y, size, color in [
        (58, 62, 34, cyan),
        (110, 48, 27, gold),
        (150, 70, 30, cyan),
    ]:
        x = offset_x + base_x
        y = offset_y + base_y
        draw.line((x - size, y, x + size, y), fill=color, width=line_width)
        draw.line((x, y - size, x, y + size), fill=color, width=line_width)

    draw.line((24, 39, 8, 64, 24, 89), fill=white, width=line_width)
    draw.line((214, 39, 236, 64, 214, 89), fill=white, width=line_width)
    draw.line((44, 92, 198, 38), fill=(255, 255, 255, 38 if glow else 130), width=3 if glow else 2)
    draw.line((51, 101, 206, 51), fill=(103, 224, 255, 44 if glow else 155), width=4 if glow else 2)


if __name__ == "__main__":
    main()
