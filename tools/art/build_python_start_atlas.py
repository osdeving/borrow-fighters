#!/usr/bin/env python3
"""Builds the Python spawn intro atlas from an AI-generated source sheet."""

from __future__ import annotations

import json
from pathlib import Path
from typing import Any

from PIL import Image, ImageDraw


ROOT = Path(__file__).resolve().parents[2]
SOURCE_PATH = ROOT / "assets/references/python-start-atlas-source.png"
ATLAS_PATH = ROOT / "assets/placeholder/python-start-atlas.png"
MANIFEST_PATH = ROOT / "assets/placeholder/python-start.sprite.json"
PREVIEW_PATH = ROOT / "tmp/art/python-start-atlas-preview.png"

SOURCE_COLUMNS = 4
SOURCE_ROWS = 3
FRAME_COUNT = 12
TARGET_COLUMNS = 4
CELL_WIDTH = 512
CELL_HEIGHT = 320
TARGET_PIVOT = {"x": 160, "y": 292}
TARGET_CONTENT_HEIGHT = 220
MAX_CONTENT_WIDTH = 464
TARGET_BOTTOM_Y = 296
RUNTIME_SCALE = 1.2266666666666666
ALPHA_THRESHOLD = 16

FRAME_DURATIONS_MS = [
    150,
    140,
    150,
    170,
    160,
    170,
    160,
    220,
    160,
    190,
    220,
    260,
]


def main() -> None:
    source = Image.open(SOURCE_PATH).convert("RGBA")
    atlas = Image.new("RGBA", atlas_size(), (0, 0, 0, 0))
    frames: list[dict[str, Any]] = []

    for index in range(FRAME_COUNT):
        source_cell = source_frame_cell(source, index)
        source_cell = remove_number_label(source_cell)
        packed_cell = repack_cell(source_cell)
        target_frame = target_frame_rect(index)
        atlas.alpha_composite(packed_cell, (target_frame["x"], target_frame["y"]))

        frame_record = {
            "name": frame_name(index),
            "clip": "spawn",
            "duration_ms": FRAME_DURATIONS_MS[index],
            "pivot": TARGET_PIVOT,
            "frame": target_frame,
        }
        bounds = alpha_bounds(packed_cell)
        if bounds is not None:
            frame_record["source_crop"] = bounds
            frame_record["trimmed_bounds"] = bounds
        frames.append(frame_record)

    ATLAS_PATH.parent.mkdir(parents=True, exist_ok=True)
    atlas.save(ATLAS_PATH)
    write_manifest(frames)
    write_preview(atlas, frames)

    print(f"wrote {ATLAS_PATH.relative_to(ROOT)}")
    print(f"wrote {MANIFEST_PATH.relative_to(ROOT)}")
    print(f"wrote {PREVIEW_PATH.relative_to(ROOT)}")


def atlas_size() -> tuple[int, int]:
    rows = (FRAME_COUNT + TARGET_COLUMNS - 1) // TARGET_COLUMNS
    return (TARGET_COLUMNS * CELL_WIDTH, rows * CELL_HEIGHT)


def source_frame_cell(source: Image.Image, index: int) -> Image.Image:
    column = index % SOURCE_COLUMNS
    row = index // SOURCE_COLUMNS
    left = round(source.width * column / SOURCE_COLUMNS)
    top = round(source.height * row / SOURCE_ROWS)
    right = round(source.width * (column + 1) / SOURCE_COLUMNS)
    bottom = round(source.height * (row + 1) / SOURCE_ROWS)
    return source.crop((left, top, right, bottom))


def remove_number_label(image: Image.Image) -> Image.Image:
    """Removes generated white frame numbers in the top-left of each cell."""
    alpha = image.getchannel("A")
    width, height = image.size
    visited = bytearray(width * height)
    source = image.load()
    cleaned = image.copy()
    target = cleaned.load()

    for start_y in range(min(height, 90)):
        for start_x in range(min(width, 100)):
            start_index = start_y * width + start_x
            if visited[start_index] or alpha.getpixel((start_x, start_y)) <= ALPHA_THRESHOLD:
                continue

            stack = [(start_x, start_y)]
            visited[start_index] = 1
            points: list[tuple[int, int]] = []

            while stack:
                x, y = stack.pop()
                points.append((x, y))
                for ny in range(y - 1, y + 2):
                    if ny < 0 or ny >= height:
                        continue
                    for nx in range(x - 1, x + 2):
                        if nx < 0 or nx >= width:
                            continue
                        index = ny * width + nx
                        if visited[index] or alpha.getpixel((nx, ny)) <= ALPHA_THRESHOLD:
                            continue
                        visited[index] = 1
                        stack.append((nx, ny))

            if should_remove_label_component(image, points):
                for x, y in points:
                    target[x, y] = (0, 0, 0, 0)

    return cleaned


def should_remove_label_component(
    image: Image.Image, points: list[tuple[int, int]]
) -> bool:
    xs = [point[0] for point in points]
    ys = [point[1] for point in points]
    left, top, right, bottom = min(xs), min(ys), max(xs) + 1, max(ys) + 1
    component_width = right - left
    component_height = bottom - top
    if left > 82 or top > 58 or component_width > 76 or component_height > 58:
        return False
    if len(points) < 12:
        return False

    bright_pixels = 0
    for x, y in points:
        red, green, blue, alpha = image.getpixel((x, y))
        if alpha > ALPHA_THRESHOLD and red > 145 and green > 145 and blue > 135:
            bright_pixels += 1
    return bright_pixels / len(points) >= 0.65


def repack_cell(source_cell: Image.Image) -> Image.Image:
    output = Image.new("RGBA", (CELL_WIDTH, CELL_HEIGHT), (0, 0, 0, 0))
    bounds = alpha_bounds(source_cell)
    if bounds is None:
        return output

    sprite = source_cell.crop(
        (
            bounds["x"],
            bounds["y"],
            bounds["x"] + bounds["w"],
            bounds["y"] + bounds["h"],
        )
    )
    sprite = remove_soft_alpha_noise(sprite)
    scale = min(TARGET_CONTENT_HEIGHT / sprite.height, MAX_CONTENT_WIDTH / sprite.width)
    width = max(1, round(sprite.width * scale))
    height = max(1, round(sprite.height * scale))
    sprite = sprite.resize((width, height), Image.Resampling.LANCZOS)

    left = round(TARGET_PIVOT["x"] - width * 0.34)
    if width > 390:
        left = 24
    left = max(16, min(CELL_WIDTH - width - 8, left))
    top = max(0, min(CELL_HEIGHT - height, TARGET_BOTTOM_Y - height))
    output.alpha_composite(sprite, (left, top))
    return output


def remove_soft_alpha_noise(image: Image.Image) -> Image.Image:
    result = image.copy()
    pixels = result.load()
    for y in range(result.height):
        for x in range(result.width):
            red, green, blue, alpha = pixels[x, y]
            if alpha < ALPHA_THRESHOLD:
                pixels[x, y] = (red, green, blue, 0)
    return result


def target_frame_rect(index: int) -> dict[str, int]:
    column = index % TARGET_COLUMNS
    row = index // TARGET_COLUMNS
    return {
        "x": column * CELL_WIDTH,
        "y": row * CELL_HEIGHT,
        "w": CELL_WIDTH,
        "h": CELL_HEIGHT,
    }


def frame_name(index: int) -> str:
    return f"spawn_{index:02d}"


def alpha_bounds(image: Image.Image) -> dict[str, int] | None:
    bbox = image.getchannel("A").getbbox()
    if bbox is None:
        return None
    x0, y0, x1, y1 = bbox
    return {"x": x0, "y": y0, "w": x1 - x0, "h": y1 - y0}


def write_manifest(frames: list[dict[str, Any]]) -> None:
    manifest = {
        "schema": "borrow-fighters.sprite.v1",
        "image": ATLAS_PATH.name,
        "source": "assets/references/python-start-atlas-source.png",
        "cell": {"w": CELL_WIDTH, "h": CELL_HEIGHT},
        "default_pivot": TARGET_PIVOT,
        "scale": RUNTIME_SCALE,
        "notes": [
            "AI-generated Python spawn intro candidate for runtime sprite validation.",
            "Original adult fictional character inspired by Python, data science, and arcade fighting games; not a portrait or exact likeness of any real person.",
            "Source was generated on a chroma-key background, converted to alpha locally, cleaned of generated frame numbers, and repacked into 512x320 runtime cells.",
            "The spawn animation shows the fighter bringing in an easel, revealing a colorful bar chart, and pointing at the strongest variable relation before returning toward fighting stance.",
            "The clip is cinematic only and should not carry hitbox or hurtbox gameplay metadata.",
        ],
        "frames": frames,
        "clips": [
            {
                "name": "spawn",
                "loop": False,
                "frames": [frame["name"] for frame in frames],
            }
        ],
    }
    MANIFEST_PATH.write_text(json.dumps(manifest, indent=2) + "\n", encoding="utf-8")


def write_preview(atlas: Image.Image, frames: list[dict[str, Any]]) -> None:
    PREVIEW_PATH.parent.mkdir(parents=True, exist_ok=True)
    preview = make_checkerboard(atlas.width, atlas.height)
    preview.alpha_composite(atlas)
    draw = ImageDraw.Draw(preview)
    for frame in frames:
        rect = frame["frame"]
        draw.rectangle(
            (
                rect["x"],
                rect["y"],
                rect["x"] + rect["w"] - 1,
                rect["y"] + rect["h"] - 1,
            ),
            outline=(255, 78, 78, 255),
            width=1,
        )
        draw.text((rect["x"] + 8, rect["y"] + 6), frame["name"], fill=(255, 255, 255, 255))
        pivot = frame["pivot"]
        px = rect["x"] + int(pivot["x"])
        py = rect["y"] + int(pivot["y"])
        draw.line((px - 7, py, px + 7, py), fill=(80, 220, 255, 255), width=1)
        draw.line((px, py - 7, px, py + 7), fill=(80, 220, 255, 255), width=1)
    preview.save(PREVIEW_PATH)


def make_checkerboard(width: int, height: int, tile: int = 16) -> Image.Image:
    image = Image.new("RGBA", (width, height), (0, 0, 0, 255))
    pixels = image.load()
    for y in range(height):
        for x in range(width):
            value = 64 if ((x // tile + y // tile) % 2) else 42
            pixels[x, y] = (value, value, value, 255)
    return image


if __name__ == "__main__":
    main()
