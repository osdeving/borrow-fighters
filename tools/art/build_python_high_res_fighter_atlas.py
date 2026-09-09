#!/usr/bin/env python3
"""Builds the high-resolution Python fighter atlas from a raster pose sheet.

The source sheet is a prototype-only generated bitmap with one full-body pose per
cell. This script removes the chroma key, repacks those painted poses into the
runtime atlas grid, and preserves the existing sprite manifest contract.
"""

from __future__ import annotations

import json
import math
from pathlib import Path
from typing import Any, Tuple

import numpy as np
from PIL import Image

ROOT = Path(__file__).resolve().parents[2]
SOURCE_PATH = ROOT / "assets/references/python-fighter-raster-source.png"
BACKUP_MANIFEST_PATH = ROOT / "assets/placeholder/python-fighter-backup.sprite.json"
ATLAS_PATH = ROOT / "assets/placeholder/python-fighter-atlas.png"
MANIFEST_PATH = ROOT / "assets/placeholder/python-fighter.sprite.json"
ROSTER_PATH = ROOT / "assets/placeholder/roster-python.png"

SOURCE_COLUMNS = 5
SOURCE_ROWS = 4
TARGET_COLUMNS = 6
CELL_WIDTH = 768
CELL_HEIGHT = 512
DEFAULT_PIVOT = {"x": 224, "y": 472}
PROJECTILE_PIVOT = {"x": 408, "y": 264}
RUNTIME_SCALE = 2.0 / 3.0
TARGET_CHARACTER_HEIGHT = 420
MAX_CHARACTER_WIDTH = 680
GROUND_Y = 472

GENERATED_NOTE = (
    "High-resolution raster Python fighter atlas generated from "
    "assets/references/python-fighter-raster-source.png. The source is a "
    "prototype-only generated pose sheet with semi-realistic painted sprites, "
    "a white blouse, black skirt, and real body poses for the nine close-range "
    "attack clips."
)

Point = Tuple[float, float]


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
    "punch_medium": ["idle", "punch_light", "punch_heavy", "punch_heavy", "punch_light", "idle"],
    "punch_heavy": ["idle", "punch_light", "punch_heavy", "punch_heavy", "punch_heavy", "punch_light", "idle"],
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
    skeleton = json.loads(BACKUP_MANIFEST_PATH.read_text())
    source = Image.open(SOURCE_PATH).convert("RGBA")
    poses = extract_source_poses(source)
    frames = skeleton["frames"]
    clips = skeleton["clips"]
    frame_to_clip_index = clip_indices(clips)
    rows = math.ceil(len(frames) / TARGET_COLUMNS)
    atlas = Image.new("RGBA", (TARGET_COLUMNS * CELL_WIDTH, rows * CELL_HEIGHT), (0, 0, 0, 0))
    output_frames: list[dict[str, Any]] = []

    for frame_index, frame in enumerate(frames):
        frame_name = frame["name"]
        clip_name = frame["clip"]
        index_in_clip, clip_len = frame_to_clip_index[frame_name]
        pose_name = pose_name_for_frame(clip_name, index_in_clip)
        cell = pack_pose_cell(poses[pose_name], pose_name, index_in_clip, clip_len)
        rect = frame_rect(frame_index)
        atlas.alpha_composite(cell, (rect["x"], rect["y"]))

        record: dict[str, Any] = {
            "name": frame_name,
            "clip": clip_name,
            "duration_ms": frame["duration_ms"],
            "pivot": PROJECTILE_PIVOT if frame_name == "projectile_0" else DEFAULT_PIVOT,
            "frame": rect,
        }
        bounds = alpha_bounds(cell)
        if bounds is not None:
            record["source_crop"] = bounds
            record["trimmed_bounds"] = bounds
        combat = combat_metadata(frame_name)
        if combat is not None:
            record["combat"] = combat
        output_frames.append(record)

    manifest = {
        "schema": "borrow-fighters.sprite.v1",
        "image": ATLAS_PATH.name,
        "source": str(SOURCE_PATH.relative_to(ROOT)),
        "cell": {"w": CELL_WIDTH, "h": CELL_HEIGHT},
        "default_pivot": DEFAULT_PIVOT,
        "scale": RUNTIME_SCALE,
        "notes": [
            GENERATED_NOTE,
            "Previous generated atlas is kept at assets/placeholder/python-fighter-atlas-backup.png with a matching backup manifest.",
            "Close-range attack hitboxes intentionally fall back to MoveSpec until the raster frames are calibrated in Sprite Studio.",
        ],
        "frames": output_frames,
        "clips": clips,
    }

    ATLAS_PATH.parent.mkdir(parents=True, exist_ok=True)
    atlas.save(ATLAS_PATH)
    MANIFEST_PATH.write_text(json.dumps(manifest, indent=2) + "\n")
    write_roster_portrait(poses["idle"])
    print(f"wrote {ATLAS_PATH.relative_to(ROOT)}")
    print(f"wrote {MANIFEST_PATH.relative_to(ROOT)}")
    print(f"wrote {ROSTER_PATH.relative_to(ROOT)}")


def extract_source_poses(source: Image.Image) -> dict[str, Image.Image]:
    cell_width = source.width / SOURCE_COLUMNS
    cell_height = source.height / SOURCE_ROWS
    poses: dict[str, Image.Image] = {}
    for pose_name, (column, row) in POSE_GRID.items():
        left = round(column * cell_width)
        top = round(row * cell_height)
        right = round((column + 1) * cell_width)
        bottom = round((row + 1) * cell_height)
        cell = source.crop((left, top, right, bottom)).convert("RGBA")
        cell = remove_chroma_key(cell)
        if pose_name == "throw":
            cell = remove_throw_training_dummy(cell)
        poses[pose_name] = crop_to_subject(remove_small_alpha_components(cell))
    return poses


def remove_chroma_key(image: Image.Image) -> Image.Image:
    pixels = np.array(image).astype(np.int32)
    red = pixels[..., 0]
    green = pixels[..., 1]
    blue = pixels[..., 2]
    bright_green = (
        (green > 120)
        & (green > red + 42)
        & (green > blue + 42)
        & ((green - np.maximum(red, blue)) > 55)
    )
    distance_to_key = np.sqrt(red**2 + (green - 255) ** 2 + blue**2)
    background = bright_green | ((distance_to_key < 118) & (green > 135))

    alpha = np.where(background, 0, 255).astype(np.uint8)
    rgba = pixels.astype(np.uint8)
    rgba[..., 3] = alpha

    foreground = alpha > 0
    excess_green = (rgba[..., 1].astype(np.int16) - np.maximum(rgba[..., 0], rgba[..., 2]).astype(np.int16)) > 24
    despill = foreground & excess_green
    rgba[..., 1] = np.where(despill, np.maximum(rgba[..., 0], rgba[..., 2]), rgba[..., 1]).astype(np.uint8)
    rgba[alpha == 0, :3] = 0
    return Image.fromarray(rgba, "RGBA")


def remove_throw_training_dummy(image: Image.Image) -> Image.Image:
    pixels = np.array(image)
    red = pixels[..., 0].astype(np.int16)
    green = pixels[..., 1].astype(np.int16)
    blue = pixels[..., 2].astype(np.int16)
    alpha = pixels[..., 3]
    max_channel = np.maximum.reduce([red, green, blue])
    min_channel = np.minimum.reduce([red, green, blue])
    saturation = max_channel - min_channel
    _, x_coords = np.indices(alpha.shape)
    x_start = round(image.width * 0.345)

    skin = (
        (red > 120)
        & (green > 55)
        & (green < 170)
        & (blue < 125)
        & ((red - green) > 18)
        & ((green - blue) > 8)
    )
    white_cloth = (max_channel > 168) & (saturation < 58)
    python_blue = (blue > 90) & (green > 70) & (red < 95) & (saturation > 45)
    python_yellow = (red > 135) & (green > 105) & (blue < 90) & (saturation > 45)
    keep_right_side = skin | white_cloth | python_blue | python_yellow
    training_dummy = (x_coords >= x_start) & (alpha > 0) & ~keep_right_side

    pixels[training_dummy, 3] = 0
    pixels[pixels[..., 3] == 0, :3] = 0
    return keep_largest_alpha_component(Image.fromarray(pixels, "RGBA"))


def keep_largest_alpha_component(image: Image.Image) -> Image.Image:
    alpha = np.array(image.getchannel("A"))
    height, width = alpha.shape
    visited = np.zeros((height, width), dtype=bool)
    best_component: list[tuple[int, int]] = []

    for y in range(height):
        for x in range(width):
            if visited[y, x] or alpha[y, x] == 0:
                continue
            stack = [(x, y)]
            visited[y, x] = True
            component: list[tuple[int, int]] = []
            while stack:
                current_x, current_y = stack.pop()
                component.append((current_x, current_y))
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
            if len(component) > len(best_component):
                best_component = component

    keep = np.zeros((height, width), dtype=bool)
    for point_x, point_y in best_component:
        keep[point_y, point_x] = True

    pixels = np.array(image)
    pixels[~keep, 3] = 0
    pixels[pixels[..., 3] == 0, :3] = 0
    return Image.fromarray(pixels, "RGBA")


def remove_small_alpha_components(image: Image.Image, min_area: int = 600) -> Image.Image:
    alpha = np.array(image.getchannel("A"))
    height, width = alpha.shape
    visited = np.zeros((height, width), dtype=bool)
    keep = np.zeros((height, width), dtype=bool)

    for y in range(height):
        for x in range(width):
            if visited[y, x] or alpha[y, x] == 0:
                continue
            stack = [(x, y)]
            visited[y, x] = True
            component: list[tuple[int, int]] = []
            while stack:
                current_x, current_y = stack.pop()
                component.append((current_x, current_y))
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
            if len(component) >= min_area:
                for point_x, point_y in component:
                    keep[point_y, point_x] = True

    pixels = np.array(image)
    pixels[~keep, 3] = 0
    pixels[pixels[..., 3] == 0, :3] = 0
    return Image.fromarray(pixels, "RGBA")


def crop_to_subject(image: Image.Image) -> Image.Image:
    bbox = image.getchannel("A").getbbox()
    if bbox is None:
        return Image.new("RGBA", (1, 1), (0, 0, 0, 0))
    pad = 8
    left = max(0, bbox[0] - pad)
    top = max(0, bbox[1] - pad)
    right = min(image.width, bbox[2] + pad)
    bottom = min(image.height, bbox[3] + pad)
    return image.crop((left, top, right, bottom))


def clip_indices(clips: list[dict[str, Any]]) -> dict[str, tuple[int, int]]:
    indices: dict[str, tuple[int, int]] = {}
    for clip in clips:
        names = clip["frames"]
        for index, frame_name in enumerate(names):
            indices[frame_name] = (index, len(names))
    return indices


def frame_rect(index: int) -> dict[str, int]:
    column = index % TARGET_COLUMNS
    row = index // TARGET_COLUMNS
    return {
        "x": column * CELL_WIDTH,
        "y": row * CELL_HEIGHT,
        "w": CELL_WIDTH,
        "h": CELL_HEIGHT,
    }


def pose_name_for_frame(clip: str, index: int) -> str:
    sequence = CLIP_POSES.get(clip, ["idle"])
    return sequence[min(index, len(sequence) - 1)]


def pack_pose_cell(pose: Image.Image, pose_name: str, index: int, total: int) -> Image.Image:
    output = Image.new("RGBA", (CELL_WIDTH, CELL_HEIGHT), (0, 0, 0, 0))
    target_height = target_height_for_pose(pose_name)
    target_width = max_width_for_pose(pose_name)
    scale = min(target_height / pose.height, target_width / pose.width)
    width = max(1, round(pose.width * scale))
    height = max(1, round(pose.height * scale))
    sprite = pose.resize((width, height), Image.Resampling.LANCZOS)

    if pose_name == "projectile":
        left = (CELL_WIDTH - width) // 2
        top = (CELL_HEIGHT - height) // 2
    else:
        phase = 0 if total <= 1 else index / (total - 1)
        left = round(DEFAULT_PIVOT["x"] - width * anchor_fraction_for_pose(pose_name))
        top = round(ground_y_for_pose(pose_name, phase) - height)

    left = max(0, min(CELL_WIDTH - width, left))
    top = max(0, min(CELL_HEIGHT - height, top))
    output.alpha_composite(sprite, (left, top))
    return output


def target_height_for_pose(pose_name: str) -> int:
    if pose_name in {"crouch", "sweep"}:
        return 310
    if pose_name in {"jump", "air_punch", "air_kick", "special"}:
        return 410
    if pose_name == "hit":
        return 420
    return TARGET_CHARACTER_HEIGHT


def max_width_for_pose(pose_name: str) -> int:
    if pose_name in {"kick", "sweep", "air_kick", "special"}:
        return 720
    if pose_name in {"punch_light", "punch_heavy", "air_punch", "throw"}:
        return 700
    return MAX_CHARACTER_WIDTH


def anchor_fraction_for_pose(pose_name: str) -> float:
    if pose_name in {"kick", "sweep", "air_kick", "special"}:
        return 0.32
    if pose_name in {"punch_light", "punch_heavy", "air_punch", "throw"}:
        return 0.34
    if pose_name == "walk":
        return 0.44
    return 0.45


def ground_y_for_pose(pose_name: str, phase: float) -> int:
    if pose_name in {"jump", "air_punch", "air_kick"}:
        return GROUND_Y - 72 - round(math.sin(phase * math.pi) * 22)
    if pose_name == "anti_air":
        return GROUND_Y - round(phase * 48)
    return GROUND_Y


def combat_metadata(frame_name: str) -> dict[str, Any] | None:
    if frame_name == "special_0":
        return {"projectile_origin": {"x": 532, "y": 236}}
    return None


def alpha_bounds(image: Image.Image) -> dict[str, int] | None:
    bbox = image.getchannel("A").getbbox()
    if bbox is None:
        return None
    x0, y0, x1, y1 = bbox
    return {"x": x0, "y": y0, "w": x1 - x0, "h": y1 - y0}


def write_roster_portrait(idle_pose: Image.Image) -> None:
    card = Image.new("RGBA", (240, 240), (78, 37, 158, 255))
    for y in range(card.height):
        blend = y / max(1, card.height - 1)
        color = (50 + round(45 * blend), 25 + round(20 * blend), 118 + round(94 * blend), 255)
        card.paste(color, (0, y, card.width, y + 1))

    sprite = idle_pose.copy()
    sprite.thumbnail((158, 208), Image.Resampling.LANCZOS)
    card.alpha_composite(sprite, ((card.width - sprite.width) // 2, 22 + (190 - sprite.height) // 2))

    from PIL import ImageDraw

    draw = ImageDraw.Draw(card, "RGBA")
    draw.ellipse((42, 194, 198, 225), fill=(24, 20, 40, 92))
    draw.rounded_rectangle((7, 7, 233, 233), radius=10, outline=(255, 238, 115, 255), width=4)
    card.save(ROSTER_PATH)


if __name__ == "__main__":
    main()
