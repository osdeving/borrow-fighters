#!/usr/bin/env python3
"""Builds the Python fighter atlas candidate from an AI-generated source sheet."""

from __future__ import annotations

import json
from pathlib import Path
from typing import Any

from PIL import Image

ROOT = Path(__file__).resolve().parents[2]
SOURCE_PATH = ROOT / "assets/references/python-fighter-atlas-source.png"
REFERENCE_MANIFEST_PATH = ROOT / "assets/placeholder/c-fighter.sprite.json"
ATLAS_PATH = ROOT / "assets/placeholder/python-fighter-atlas.png"
MANIFEST_PATH = ROOT / "assets/placeholder/python-fighter.sprite.json"

TARGET_COLUMNS = 6
CELL_WIDTH = 384
CELL_HEIGHT = 256
DEFAULT_PIVOT = {"x": 112, "y": 236}
PROJECTILE_PIVOT = {"x": 204, "y": 132}
TARGET_CHARACTER_HEIGHT = 204
MAX_CHARACTER_WIDTH = 340
ALPHA_THRESHOLD = 28
MIN_SOURCE_COMPONENT_AREA = 1000
MIN_SOURCE_COMPONENT_CHROMA = 24.0


def main() -> None:
    source = Image.open(SOURCE_PATH).convert("RGBA")
    source_sprites = extract_source_sprites(source)
    reference = json.loads(REFERENCE_MANIFEST_PATH.read_text())
    reference_frames = reference["frames"][:94]

    atlas = Image.new("RGBA", (TARGET_COLUMNS * CELL_WIDTH, 16 * CELL_HEIGHT), (0, 0, 0, 0))
    frames: list[dict[str, Any]] = []

    for index, reference_frame in enumerate(reference_frames):
        source_sprite = source_sprite_for_frame(source_sprites, index, len(reference_frames))
        packed_cell = repack_cell(source_sprite, reference_frame["name"])
        target_frame = target_frame_rect(index)
        atlas.alpha_composite(packed_cell, (target_frame["x"], target_frame["y"]))

        frame_record = {
            "name": reference_frame["name"],
            "clip": reference_frame["clip"],
            "duration_ms": reference_frame["duration_ms"],
            "pivot": frame_pivot(reference_frame["name"]),
            "frame": target_frame,
        }
        bounds = alpha_bounds(packed_cell)
        if bounds is not None:
            frame_record["source_crop"] = bounds
            frame_record["trimmed_bounds"] = bounds
        combat = combat_metadata(reference_frame["name"])
        if combat is not None:
            frame_record["combat"] = combat
        frames.append(frame_record)

    ATLAS_PATH.parent.mkdir(parents=True, exist_ok=True)
    atlas.save(ATLAS_PATH)

    manifest = {
        "schema": "borrow-fighters.sprite.v1",
        "image": ATLAS_PATH.name,
        "source": "assets/references/python-fighter-atlas-source.png",
        "cell": {"w": CELL_WIDTH, "h": CELL_HEIGHT},
        "default_pivot": DEFAULT_PIVOT,
        "scale": 1.3333333333333333,
        "notes": [
            "AI-generated Python fighter atlas candidate for sprite validation.",
            "Original adult character inspired by Python and data science; not a portrait or exact likeness of any real person.",
            "Source sheet was generated as a chroma-key atlas, cleaned to alpha, segmented into major pose components, then repacked to the C atlas 6x16 runtime layout.",
            "The source provided 77 major poses, so some neighboring runtime frames intentionally reuse a pose until hand animation is produced.",
            "Light punch frames are intended to read as the snake bite; heavy punch frames are the fighter's own punch.",
            "Taunt frames use data-science visual effects such as points, charts, and matrix-like energy without readable text.",
            "This asset is not integrated into the playable roster yet and still needs review in Sprite Studio.",
        ],
        "frames": frames,
        "clips": reference["clips"],
    }
    MANIFEST_PATH.write_text(json.dumps(manifest, indent=2) + "\n")
    print(f"wrote {ATLAS_PATH.relative_to(ROOT)}")
    print(f"wrote {MANIFEST_PATH.relative_to(ROOT)}")


def extract_source_sprites(source: Image.Image) -> list[Image.Image]:
    alpha = source.getchannel("A")
    width, height = source.size
    visited = bytearray(width * height)
    components: list[dict[str, int]] = []

    for y in range(height):
        for x in range(width):
            index = y * width + x
            if visited[index] or alpha.getpixel((x, y)) <= ALPHA_THRESHOLD:
                continue

            stack = [(x, y)]
            visited[index] = 1
            area = 0
            min_x = max_x = x
            min_y = max_y = y

            while stack:
                current_x, current_y = stack.pop()
                area += 1
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
                        next_index = next_y * width + next_x
                        if visited[next_index]:
                            continue
                        if alpha.getpixel((next_x, next_y)) <= ALPHA_THRESHOLD:
                            continue
                        visited[next_index] = 1
                        stack.append((next_x, next_y))

            box_width = max_x - min_x + 1
            box_height = max_y - min_y + 1
            if (
                area >= MIN_SOURCE_COMPONENT_AREA
                and box_width > 25
                and box_height > 25
                and component_average_chroma(
                    source, min_x, min_y, max_x + 1, max_y + 1
                )
                >= MIN_SOURCE_COMPONENT_CHROMA
            ):
                components.append(
                    {
                        "area": area,
                        "x": min_x,
                        "y": min_y,
                        "w": box_width,
                        "h": box_height,
                    }
                )

    components.sort(
        key=lambda component: (
            round((component["y"] + component["h"] / 2) / 80),
            component["x"] + component["w"] / 2,
        )
    )
    sprites = []
    for component in components:
        margin = 4
        left = max(0, component["x"] - margin)
        top = max(0, component["y"] - margin)
        right = min(width, component["x"] + component["w"] + margin)
        bottom = min(height, component["y"] + component["h"] + margin)
        sprites.append(source.crop((left, top, right, bottom)))
    return sprites


def component_average_chroma(
    source: Image.Image, left: int, top: int, right: int, bottom: int
) -> float:
    crop = source.crop((left, top, right, bottom))
    total = 0
    count = 0
    for red, green, blue, alpha in crop.getdata():
        if alpha <= ALPHA_THRESHOLD:
            continue
        total += max(red, green, blue) - min(red, green, blue)
        count += 1
    if count == 0:
        return 0.0
    return total / count


def source_sprite_for_frame(
    source_sprites: list[Image.Image], frame_index: int, total_frames: int
) -> Image.Image:
    if not source_sprites:
        return Image.new("RGBA", (1, 1), (0, 0, 0, 0))
    if total_frames <= 1:
        return source_sprites[0]
    source_index = round(frame_index * (len(source_sprites) - 1) / (total_frames - 1))
    return source_sprites[source_index]


def repack_cell(source_cell: Image.Image, frame_name: str) -> Image.Image:
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

    target_height = target_height_for(frame_name)
    scale = min(target_height / sprite.height, MAX_CHARACTER_WIDTH / sprite.width)
    width = max(1, round(sprite.width * scale))
    height = max(1, round(sprite.height * scale))
    sprite = sprite.resize((width, height), Image.Resampling.LANCZOS)

    if frame_name == "projectile_0":
        left = (CELL_WIDTH - width) // 2
        top = (CELL_HEIGHT - height) // 2
    else:
        pivot = DEFAULT_PIVOT
        left = round(pivot["x"] - width * 0.45)
        top = pivot["y"] - height

    left = max(0, min(CELL_WIDTH - width, left))
    top = max(0, min(CELL_HEIGHT - height, top))
    output.alpha_composite(sprite, (left, top))
    return output


def remove_soft_alpha_noise(image: Image.Image) -> Image.Image:
    pixels = image.load()
    for y in range(image.height):
        for x in range(image.width):
            red, green, blue, alpha = pixels[x, y]
            if alpha < 14:
                pixels[x, y] = (red, green, blue, 0)
    return image


def target_height_for(frame_name: str) -> int:
    if frame_name.startswith(("knockdown", "projectile")):
        return 156
    if frame_name.startswith("crouch"):
        return 176
    if frame_name.startswith(("kick_heavy", "special")):
        return 196
    return TARGET_CHARACTER_HEIGHT


def target_frame_rect(index: int) -> dict[str, int]:
    column = index % TARGET_COLUMNS
    row = index // TARGET_COLUMNS
    return {
        "x": column * CELL_WIDTH,
        "y": row * CELL_HEIGHT,
        "w": CELL_WIDTH,
        "h": CELL_HEIGHT,
    }


def frame_pivot(frame_name: str) -> dict[str, int]:
    if frame_name == "projectile_0":
        return PROJECTILE_PIVOT
    return DEFAULT_PIVOT


def alpha_bounds(image: Image.Image) -> dict[str, int] | None:
    bbox = image.getchannel("A").getbbox()
    if bbox is None:
        return None
    x0, y0, x1, y1 = bbox
    return {"x": x0, "y": y0, "w": x1 - x0, "h": y1 - y0}


def combat_metadata(frame_name: str) -> dict[str, Any] | None:
    light_bite_frames = {
        "punch_light_2": {"x": 186, "y": 72, "w": 112, "h": 46},
        "punch_light_3": {"x": 198, "y": 70, "w": 122, "h": 48},
        "punch_light_4": {"x": 188, "y": 76, "w": 104, "h": 44},
    }
    heavy_punch_frames = {
        "punch_heavy_3": {"x": 172, "y": 84, "w": 92, "h": 48},
        "punch_heavy_4": {"x": 184, "y": 82, "w": 104, "h": 50},
        "punch_heavy_5": {"x": 176, "y": 86, "w": 94, "h": 48},
    }
    special_origins = {
        "special_0": {"x": 196, "y": 112},
        "special_1": {"x": 206, "y": 112},
        "special_2": {"x": 214, "y": 112},
        "special_3": {"x": 222, "y": 114},
        "special_4": {"x": 210, "y": 116},
        "special_5": {"x": 200, "y": 116},
    }

    if frame_name in light_bite_frames:
        hitbox = {**light_bite_frames[frame_name], "label": "snake_bite"}
        return {"hitboxes": [hitbox]}
    if frame_name in heavy_punch_frames:
        hitbox = {**heavy_punch_frames[frame_name], "label": "punch"}
        return {"hitboxes": [hitbox]}
    if frame_name in special_origins:
        return {"projectile_origin": special_origins[frame_name]}
    return None


if __name__ == "__main__":
    main()
