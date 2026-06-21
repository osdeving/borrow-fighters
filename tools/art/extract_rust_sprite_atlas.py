#!/usr/bin/env python3
"""Extracts a clean Rust fighter atlas from the current reference sheet."""

from __future__ import annotations

import json
from dataclasses import dataclass
from pathlib import Path

from PIL import Image, ImageDraw


ROOT = Path(__file__).resolve().parents[2]
SOURCE_PATH = ROOT / "assets/references/sprinte-rust.png"
ATLAS_PATH = ROOT / "assets/placeholder/rust-fighter-atlas.png"
MANIFEST_PATH = ROOT / "assets/placeholder/rust-fighter.sprite.json"
PROJECTILE_PATH = ROOT / "assets/placeholder/rust-gear-projectile.png"
PREVIEW_PATH = ROOT / "tmp/art/rust-fighter-atlas-preview.png"

CELL_WIDTH = 320
CELL_HEIGHT = 256
COLUMNS = 6
TARGET_PIVOT_X = 112
TARGET_PIVOT_Y = 236
SCALE = 1.16


@dataclass(frozen=True)
class FrameSpec:
    name: str
    clip: str
    crop: tuple[int, int, int, int]
    pivot: tuple[int, int]
    duration_ms: int = 100


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


def is_checkerboard_pixel(pixel: tuple[int, int, int]) -> bool:
    r, g, b = pixel
    neutral = abs(r - g) <= 5 and abs(r - b) <= 5
    return neutral and r >= 205


def key_checkerboard_to_alpha(image: Image.Image) -> Image.Image:
    rgba = image.convert("RGBA")
    pixels = rgba.load()
    for y in range(rgba.height):
        for x in range(rgba.width):
            r, g, b, a = pixels[x, y]
            if is_checkerboard_pixel((r, g, b)):
                pixels[x, y] = (r, g, b, 0)
    return rgba


def remove_stray_components(image: Image.Image, pivot_y: int) -> Image.Image:
    alpha = image.getchannel("A")
    width, height = image.size
    visited = bytearray(width * height)
    source = image.load()
    cleaned = Image.new("RGBA", image.size, (0, 0, 0, 0))
    target = cleaned.load()

    for start_y in range(height):
        for start_x in range(width):
            start_index = start_y * width + start_x
            if visited[start_index] or alpha.getpixel((start_x, start_y)) == 0:
                continue

            stack = [(start_x, start_y)]
            visited[start_index] = 1
            component: list[tuple[int, int]] = []
            while stack:
                x, y = stack.pop()
                component.append((x, y))
                for nx in (x - 1, x, x + 1):
                    for ny in (y - 1, y, y + 1):
                        if nx < 0 or ny < 0 or nx >= width or ny >= height:
                            continue
                        index = ny * width + nx
                        if visited[index] or alpha.getpixel((nx, ny)) == 0:
                            continue
                        visited[index] = 1
                        stack.append((nx, ny))

            xs = [point[0] for point in component]
            ys = [point[1] for point in component]
            left, top, right, bottom = min(xs), min(ys), max(xs) + 1, max(ys) + 1
            area = len(component)
            height_px = bottom - top
            width_px = right - left
            looks_like_label = bottom < pivot_y - 70 and width_px > 45 and height_px < 45
            looks_like_dust = area < 48
            if looks_like_label or looks_like_dust:
                continue

            for x, y in component:
                target[x, y] = source[x, y]

    return cleaned


def trim_alpha(image: Image.Image) -> tuple[Image.Image, tuple[int, int, int, int]]:
    bbox = image.getchannel("A").getbbox()
    if bbox is None:
        return image, (0, 0, image.width, image.height)
    return image.crop(bbox), bbox


def extract_frame(source: Image.Image, spec: FrameSpec) -> tuple[Image.Image, dict[str, object]]:
    crop_x, crop_y, _, _ = spec.crop
    local_pivot_y = spec.pivot[1] - crop_y
    keyed = remove_stray_components(key_checkerboard_to_alpha(source.crop(spec.crop)), local_pivot_y)
    trimmed, trim_box = trim_alpha(keyed)
    width = round(trimmed.width * SCALE)
    height = round(trimmed.height * SCALE)
    sprite = trimmed.resize((width, height), Image.Resampling.LANCZOS)

    trim_x, trim_y, _, _ = trim_box
    local_pivot_x = (spec.pivot[0] - crop_x - trim_x) * SCALE
    local_pivot_y = (spec.pivot[1] - crop_y - trim_y) * SCALE
    if spec.clip == "projectile":
        paste_x = round((CELL_WIDTH - width) * 0.5)
        paste_y = round((CELL_HEIGHT - height) * 0.5)
        target_pivot_x = round(paste_x + local_pivot_x)
        target_pivot_y = round(paste_y + local_pivot_y)
    else:
        target_pivot_x = TARGET_PIVOT_X
        target_pivot_y = TARGET_PIVOT_Y
        paste_x = round(target_pivot_x - local_pivot_x)
        paste_y = round(target_pivot_y - local_pivot_y)

    cell = Image.new("RGBA", (CELL_WIDTH, CELL_HEIGHT), (0, 0, 0, 0))
    cell.alpha_composite(sprite, (paste_x, paste_y))

    frame_data = {
        "name": spec.name,
        "clip": spec.clip,
        "duration_ms": spec.duration_ms,
        "pivot": {"x": target_pivot_x, "y": target_pivot_y},
        "source_crop": {
            "x": spec.crop[0],
            "y": spec.crop[1],
            "w": spec.crop[2] - spec.crop[0],
            "h": spec.crop[3] - spec.crop[1],
        },
        "trimmed_bounds": {
            "x": paste_x,
            "y": paste_y,
            "w": width,
            "h": height,
        },
    }
    return cell, frame_data


def make_checkerboard(width: int, height: int, tile: int = 16) -> Image.Image:
    image = Image.new("RGBA", (width, height), (0, 0, 0, 255))
    pixels = image.load()
    for y in range(height):
        for x in range(width):
            value = 64 if ((x // tile + y // tile) % 2) else 42
            pixels[x, y] = (value, value, value, 255)
    return image


def write_outputs() -> None:
    source = Image.open(SOURCE_PATH).convert("RGB")
    rows = (len(FRAMES) + COLUMNS - 1) // COLUMNS
    atlas = Image.new("RGBA", (CELL_WIDTH * COLUMNS, CELL_HEIGHT * rows), (0, 0, 0, 0))
    frame_records: list[dict[str, object]] = []

    for index, spec in enumerate(FRAMES):
        cell, record = extract_frame(source, spec)
        col = index % COLUMNS
        row = index // COLUMNS
        atlas.alpha_composite(cell, (col * CELL_WIDTH, row * CELL_HEIGHT))
        record["frame"] = {
            "x": col * CELL_WIDTH,
            "y": row * CELL_HEIGHT,
            "w": CELL_WIDTH,
            "h": CELL_HEIGHT,
        }
        frame_records.append(record)

    ATLAS_PATH.parent.mkdir(parents=True, exist_ok=True)
    atlas.save(ATLAS_PATH)

    projectile_rect = frame_records[-3]["frame"]
    projectile_frame = atlas.crop(
        (
            projectile_rect["x"],
            projectile_rect["y"],
            projectile_rect["x"] + projectile_rect["w"],
            projectile_rect["y"] + projectile_rect["h"],
        )
    )
    projectile_bbox = projectile_frame.getchannel("A").getbbox()
    if projectile_bbox:
        projectile_frame.crop(projectile_bbox).save(PROJECTILE_PATH)

    clip_records = []
    for clip_name, clip_config in CLIPS.items():
        names = [frame["name"] for frame in frame_records if frame["clip"] == clip_name]
        clip_records.append(
            {
                "name": clip_name,
                "loop": clip_config["loop"],
                "frames": names,
            }
        )

    manifest = {
        "schema": "borrow-fighters.sprite.v1",
        "image": ATLAS_PATH.name,
        "source": str(SOURCE_PATH.relative_to(ROOT)),
        "cell": {"w": CELL_WIDTH, "h": CELL_HEIGHT},
        "default_pivot": {"x": TARGET_PIVOT_X, "y": TARGET_PIVOT_Y},
        "scale": SCALE,
        "frames": frame_records,
        "clips": clip_records,
        "notes": [
            "Extracted from an RGB concept/reference sheet; not a final authored sprite export.",
            "Labels and checkerboard background are intentionally excluded from runtime assets.",
            "Future artist-authored files should prefer native alpha and Aseprite or TexturePacker metadata.",
        ],
    }
    MANIFEST_PATH.write_text(json.dumps(manifest, indent=2) + "\n", encoding="utf-8")

    PREVIEW_PATH.parent.mkdir(parents=True, exist_ok=True)
    preview = make_checkerboard(atlas.width, atlas.height)
    preview.alpha_composite(atlas)
    draw = ImageDraw.Draw(preview)
    for index, frame in enumerate(frame_records):
        col = index % COLUMNS
        row = index // COLUMNS
        x = col * CELL_WIDTH
        y = row * CELL_HEIGHT
        draw.rectangle((x, y, x + CELL_WIDTH - 1, y + CELL_HEIGHT - 1), outline=(255, 78, 78, 255), width=1)
        draw.text((x + 6, y + 5), frame["name"], fill=(255, 255, 255, 255))
        pivot = frame["pivot"]
        px = x + int(pivot["x"])
        py = y + int(pivot["y"])
        draw.line((px - 7, py, px + 7, py), fill=(80, 220, 255, 255), width=1)
        draw.line((px, py - 7, px, py + 7), fill=(80, 220, 255, 255), width=1)
    preview.save(PREVIEW_PATH)

    print(f"wrote {ATLAS_PATH.relative_to(ROOT)}")
    print(f"wrote {MANIFEST_PATH.relative_to(ROOT)}")
    print(f"wrote {PROJECTILE_PATH.relative_to(ROOT)}")
    print(f"wrote {PREVIEW_PATH.relative_to(ROOT)}")


if __name__ == "__main__":
    write_outputs()
