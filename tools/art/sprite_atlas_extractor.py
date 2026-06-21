#!/usr/bin/env python3
"""Shared sprite atlas extraction helpers for prototype character sheets."""

from __future__ import annotations

import json
from collections.abc import Callable
from dataclasses import dataclass
from pathlib import Path
from typing import Any

from PIL import Image, ImageDraw


ROOT = Path(__file__).resolve().parents[2]


@dataclass(frozen=True)
class FrameSpec:
    name: str
    clip: str
    crop: tuple[int, int, int, int]
    pivot: tuple[int, int]
    duration_ms: int = 100


@dataclass(frozen=True)
class AtlasConfig:
    source_path: Path
    atlas_path: Path
    manifest_path: Path
    projectile_path: Path
    preview_path: Path
    frames: list[FrameSpec]
    clips: dict[str, dict[str, object]]
    cell_width: int = 320
    cell_height: int = 256
    columns: int = 6
    target_pivot: tuple[int, int] = (112, 236)
    scale: float = 1.16
    projectile_frame_name: str = "projectile_0"
    clean_white_body: bool = False
    remove_stray_components: bool = True
    source_processor: Callable[[Image.Image], Image.Image] | None = None
    cell_processor: Callable[[Image.Image, FrameSpec], Image.Image] | None = None


def write_outputs(config: AtlasConfig) -> None:
    """Writes atlas, manifest, projectile crop, and local preview."""
    source = Image.open(config.source_path).convert("RGB")
    if config.source_processor is not None:
        source = config.source_processor(source).convert("RGB")
    rows = (len(config.frames) + config.columns - 1) // config.columns
    atlas = Image.new(
        "RGBA",
        (config.cell_width * config.columns, config.cell_height * rows),
        (0, 0, 0, 0),
    )
    frame_records: list[dict[str, Any]] = []

    for index, spec in enumerate(config.frames):
        cell, record = extract_frame(source, spec, config)
        col = index % config.columns
        row = index // config.columns
        atlas.alpha_composite(cell, (col * config.cell_width, row * config.cell_height))
        record["frame"] = {
            "x": col * config.cell_width,
            "y": row * config.cell_height,
            "w": config.cell_width,
            "h": config.cell_height,
        }
        frame_records.append(record)

    config.atlas_path.parent.mkdir(parents=True, exist_ok=True)
    atlas.save(config.atlas_path)
    if config.projectile_frame_name:
        write_projectile_asset(atlas, frame_records, config)
    write_manifest(frame_records, config)
    write_preview(atlas, frame_records, config)

    print(f"wrote {config.atlas_path.relative_to(ROOT)}")
    print(f"wrote {config.manifest_path.relative_to(ROOT)}")
    if config.projectile_frame_name:
        print(f"wrote {config.projectile_path.relative_to(ROOT)}")
    print(f"wrote {config.preview_path.relative_to(ROOT)}")


def extract_frame(
    source: Image.Image, spec: FrameSpec, config: AtlasConfig
) -> tuple[Image.Image, dict[str, Any]]:
    crop_x, crop_y, _, _ = spec.crop
    local_pivot_y = spec.pivot[1] - crop_y
    keyed = key_checkerboard_to_alpha(source.crop(spec.crop))
    if config.remove_stray_components:
        keyed = remove_stray_components(keyed, local_pivot_y)
    trimmed, trim_box = trim_alpha(keyed)
    width = round(trimmed.width * config.scale)
    height = round(trimmed.height * config.scale)
    sprite = trimmed.resize((width, height), Image.Resampling.LANCZOS)

    trim_x, trim_y, _, _ = trim_box
    local_pivot_x = (spec.pivot[0] - crop_x - trim_x) * config.scale
    local_pivot_y = (spec.pivot[1] - crop_y - trim_y) * config.scale
    if spec.clip == "projectile":
        paste_x = round((config.cell_width - width) * 0.5)
        paste_y = round((config.cell_height - height) * 0.5)
        target_pivot_x = round(paste_x + local_pivot_x)
        target_pivot_y = round(paste_y + local_pivot_y)
    else:
        target_pivot_x, target_pivot_y = config.target_pivot
        paste_x = round(target_pivot_x - local_pivot_x)
        paste_y = round(target_pivot_y - local_pivot_y)

    cell = Image.new("RGBA", (config.cell_width, config.cell_height), (0, 0, 0, 0))
    cell.alpha_composite(sprite, (paste_x, paste_y))
    if config.clean_white_body and spec.clip != "projectile":
        cell = clean_white_body(cell, (target_pivot_x, target_pivot_y))
    if config.cell_processor is not None:
        cell = config.cell_processor(cell, spec)

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


def clean_white_body(image: Image.Image, pivot: tuple[int, int]) -> Image.Image:
    """Softens dark neutral flecks inside the white body area."""
    rgba = image.copy()
    pixels = rgba.load()
    width, height = rgba.size
    pivot_x, pivot_y = pivot
    region = (
        max(0, pivot_x - 116),
        max(0, pivot_y - 232),
        min(width, pivot_x + 126),
        min(height, pivot_y + 16),
    )
    replacements: list[tuple[int, int, tuple[int, int, int, int]]] = []

    for y in range(region[1], region[3]):
        for x in range(region[0], region[2]):
            r, g, b, a = pixels[x, y]
            if a < 180 or not is_dark_neutral((r, g, b)):
                continue
            if near_transparent_edge(rgba, x, y):
                continue

            white_neighbors = nearby_white_pixels(rgba, x, y, radius=4)
            if len(white_neighbors) < 18:
                continue

            replacements.append((x, y, average_color(white_neighbors, a)))

    for x, y, color in replacements:
        pixels[x, y] = color

    return rgba


def is_dark_neutral(pixel: tuple[int, int, int]) -> bool:
    r, g, b = pixel
    luminance = 0.2126 * r + 0.7152 * g + 0.0722 * b
    spread = max(r, g, b) - min(r, g, b)
    return luminance < 142 and spread < 42


def is_white_body_pixel(pixel: tuple[int, int, int, int]) -> bool:
    r, g, b, a = pixel
    if a < 180:
        return False
    luminance = 0.2126 * r + 0.7152 * g + 0.0722 * b
    spread = max(r, g, b) - min(r, g, b)
    return luminance >= 150 and spread < 64 and r >= 130 and g >= 125 and b >= 105


def near_transparent_edge(image: Image.Image, x: int, y: int) -> bool:
    alpha = image.getchannel("A")
    width, height = image.size
    for ny in range(max(0, y - 2), min(height, y + 3)):
        for nx in range(max(0, x - 2), min(width, x + 3)):
            if alpha.getpixel((nx, ny)) < 64:
                return True
    return False


def nearby_white_pixels(
    image: Image.Image, x: int, y: int, radius: int
) -> list[tuple[int, int, int]]:
    pixels = image.load()
    width, height = image.size
    white_pixels: list[tuple[int, int, int]] = []
    for ny in range(max(0, y - radius), min(height, y + radius + 1)):
        for nx in range(max(0, x - radius), min(width, x + radius + 1)):
            if nx == x and ny == y:
                continue
            r, g, b, a = pixels[nx, ny]
            if is_white_body_pixel((r, g, b, a)):
                white_pixels.append((r, g, b))
    return white_pixels


def average_color(
    colors: list[tuple[int, int, int]], alpha: int
) -> tuple[int, int, int, int]:
    count = len(colors)
    r = round(sum(color[0] for color in colors) / count)
    g = round(sum(color[1] for color in colors) / count)
    b = round(sum(color[2] for color in colors) / count)
    return (r, g, b, alpha)


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


def write_projectile_asset(
    atlas: Image.Image, frame_records: list[dict[str, Any]], config: AtlasConfig
) -> None:
    projectile_rect = next(
        frame["frame"]
        for frame in frame_records
        if frame["name"] == config.projectile_frame_name
    )
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
        projectile_frame.crop(projectile_bbox).save(config.projectile_path)


def write_manifest(frame_records: list[dict[str, Any]], config: AtlasConfig) -> None:
    clip_records = []
    for clip_name, clip_config in config.clips.items():
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
        "image": config.atlas_path.name,
        "source": str(config.source_path.relative_to(ROOT)),
        "cell": {"w": config.cell_width, "h": config.cell_height},
        "default_pivot": {"x": config.target_pivot[0], "y": config.target_pivot[1]},
        "scale": config.scale,
        "frames": frame_records,
        "clips": clip_records,
        "notes": [
            "Extracted from an RGB concept/reference sheet; not a final authored sprite export.",
            "Labels and checkerboard background are intentionally excluded from runtime assets.",
            "Future artist-authored files should prefer native alpha and Aseprite or TexturePacker metadata.",
        ],
    }
    config.manifest_path.write_text(json.dumps(manifest, indent=2) + "\n", encoding="utf-8")


def write_preview(
    atlas: Image.Image, frame_records: list[dict[str, Any]], config: AtlasConfig
) -> None:
    config.preview_path.parent.mkdir(parents=True, exist_ok=True)
    preview = make_checkerboard(atlas.width, atlas.height)
    preview.alpha_composite(atlas)
    draw = ImageDraw.Draw(preview)
    for index, frame in enumerate(frame_records):
        col = index % config.columns
        row = index // config.columns
        x = col * config.cell_width
        y = row * config.cell_height
        draw.rectangle(
            (x, y, x + config.cell_width - 1, y + config.cell_height - 1),
            outline=(255, 78, 78, 255),
            width=1,
        )
        draw.text((x + 6, y + 5), frame["name"], fill=(255, 255, 255, 255))
        pivot = frame["pivot"]
        px = x + int(pivot["x"])
        py = y + int(pivot["y"])
        draw.line((px - 7, py, px + 7, py), fill=(80, 220, 255, 255), width=1)
        draw.line((px, py - 7, px, py + 7), fill=(80, 220, 255, 255), width=1)
    preview.save(config.preview_path)


def make_checkerboard(width: int, height: int, tile: int = 16) -> Image.Image:
    image = Image.new("RGBA", (width, height), (0, 0, 0, 255))
    pixels = image.load()
    for y in range(height):
        for x in range(width):
            value = 64 if ((x // tile + y // tile) % 2) else 42
            pixels[x, y] = (value, value, value, 255)
    return image
