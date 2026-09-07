#!/usr/bin/env python3
"""Pack explicitly reviewed action sheets into the existing sprite JSON contract.

This utility only crops named rectangles and packs their unchanged pixels. It
does not generate art, infer poses, retouch backgrounds, rescale frames, or
invent combat metadata. Source files and a reproducible provenance record stay
separate from the candidate runtime atlas. Requires Pillow.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import math
import os
from pathlib import Path
import re
from typing import Any

from PIL import Image

ROOT = Path(__file__).resolve().parents[2]
PRODUCTION_SCHEMA = "borrow-fighters.production.v1"


def require(condition: bool, message: str) -> None:
    if not condition:
        raise ValueError(message)


def identifier(value: Any, context: str) -> str:
    require(isinstance(value, str) and re.fullmatch(r"[a-z0-9][a-z0-9_-]*", value) is not None,
            f"{context} must contain lowercase letters, digits, underscores or hyphens")
    return value


def integer(value: Any, context: str, minimum: int = 0, maximum: int = 2**31 - 1) -> int:
    require(type(value) is int and minimum <= value <= maximum,
            f"{context} must be an integer in [{minimum}, {maximum}]")
    return value


def file_hash(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def relative(path: Path, parent: Path) -> str:
    return Path(os.path.relpath(path, parent)).as_posix()


def write_json(path: Path, value: Any) -> None:
    path.write_text(json.dumps(value, ensure_ascii=False, indent=2, allow_nan=False) + "\n")


def validate_combat(combat: Any, width: int, height: int, context: str) -> None:
    """Accept only explicit frame-local metadata that the runtime can validate."""
    require(isinstance(combat, dict), f"{context}.combat must be an object")
    require(not set(combat) - {"hurtboxes", "hitboxes", "projectile_origin"},
            f"{context}.combat contains unsupported fields")
    for kind in ("hurtboxes", "hitboxes"):
        boxes = combat.get(kind, [])
        require(isinstance(boxes, list), f"{context}.{kind} must be a list")
        for box in boxes:
            require(isinstance(box, dict), f"{context}.{kind} must contain objects")
            x = integer(box.get("x"), f"{context}.{kind}.x")
            y = integer(box.get("y"), f"{context}.{kind}.y")
            w = integer(box.get("w"), f"{context}.{kind}.w", 1)
            h = integer(box.get("h"), f"{context}.{kind}.h", 1)
            require(x + w <= width and y + h <= height,
                    f"{context}.{kind} must be inside the source rectangle")
            if "label" in box:
                require(isinstance(box["label"], str) and bool(box["label"].strip()),
                        f"{context}.{kind}.label must be nonempty")
    if "projectile_origin" in combat:
        point = combat["projectile_origin"]
        require(isinstance(point, dict), f"{context}.projectile_origin must be an object")
        integer(point.get("x"), f"{context}.projectile_origin.x", maximum=width)
        integer(point.get("y"), f"{context}.projectile_origin.y", maximum=height)


def build(production_path: Path, output_dir: Path | None = None,
          columns: int | None = None) -> Path:
    """Validate all input before writing crops, one atlas, manifest and provenance."""
    production_path = production_path.resolve()
    production = json.loads(production_path.read_text())
    require(isinstance(production, dict), "production manifest must be an object")
    require(production.get("schema") == PRODUCTION_SCHEMA, "unsupported production schema")
    character = identifier(production.get("character"), "character")
    scale = production.get("scale")
    require(type(scale) in (int, float) and math.isfinite(scale) and scale > 0,
            "scale must be finite and positive")
    actions = production.get("actions")
    require(isinstance(actions, dict) and bool(actions), "actions must be a nonempty object")
    output_dir = (output_dir or ROOT / "assets/candidates" / character).resolve()
    work_dir = production_path.parent
    require(output_dir != work_dir and work_dir not in output_dir.parents,
            "candidate output must be separate from the production directory")

    sheets: dict[Path, Image.Image] = {}
    prepared: list[tuple[str, dict[str, Any], Image.Image, Path]] = []
    names: set[str] = set()
    clips: list[dict[str, Any]] = []
    for action_name, action in actions.items():
        identifier(action_name, "action name")
        require(isinstance(action, dict), f"{action_name} must be an object")
        require(action.get("reviewed") is True,
                f"{action_name}: set reviewed=true only after reviewing the source frames")
        require(type(action.get("loop")) is bool, f"{action_name}.loop must be explicit")
        sheet_name = action.get("sheet")
        require(isinstance(sheet_name, str) and bool(sheet_name.strip()),
                f"{action_name}.sheet is required")
        sheet_path = (work_dir / sheet_name).resolve()
        if sheet_path not in sheets:
            with Image.open(sheet_path) as image:
                require(image.format == "PNG", f"{sheet_path}: source must be PNG")
                rgba = image.convert("RGBA")
                require(rgba.getchannel("A").getextrema()[0] == 0,
                        f"{sheet_path}: PNG must have real transparent pixels")
                sheets[sheet_path] = rgba
        sheet = sheets[sheet_path]
        frames = action.get("frames")
        require(isinstance(frames, list) and bool(frames),
                f"{action_name}.frames must be a nonempty explicit sequence")
        clip_names = []
        for frame in frames:
            require(isinstance(frame, dict), f"{action_name}: frame must be an object")
            name = identifier(frame.get("name"), f"{action_name} frame name")
            require(name not in names, f"duplicate frame name: {name}")
            names.add(name)
            clip_names.append(name)
            rect = frame.get("source_rect")
            require(isinstance(rect, dict), f"{name}.source_rect is required")
            x = integer(rect.get("x"), f"{name}.source_rect.x")
            y = integer(rect.get("y"), f"{name}.source_rect.y")
            w = integer(rect.get("w"), f"{name}.source_rect.w", 1)
            h = integer(rect.get("h"), f"{name}.source_rect.h", 1)
            require(x + w <= sheet.width and y + h <= sheet.height,
                    f"{name}: source rectangle exceeds the PNG bounds")
            pivot = frame.get("pivot")
            require(isinstance(pivot, dict), f"{name}.pivot is required")
            integer(pivot.get("x"), f"{name}.pivot.x", maximum=w)
            integer(pivot.get("y"), f"{name}.pivot.y", maximum=h)
            integer(frame.get("duration_ms"), f"{name}.duration_ms", 1, 2**32 - 1)
            if "phase" in frame:
                require(isinstance(frame["phase"], str) and bool(frame["phase"].strip()),
                        f"{name}.phase must be nonempty")
            if "combat" in frame:
                validate_combat(frame["combat"], w, h, name)
            crop = sheet.crop((x, y, x + w, y + h))
            low, high = crop.getchannel("A").getextrema()
            require(low == 0 and high > 0,
                    f"{name}: each crop needs visible art and real transparent pixels")
            prepared.append((action_name, frame, crop, sheet_path))
        clips.append({"name": action_name, "loop": action["loop"], "frames": clip_names})

    cell_w = max(crop.width for _, _, crop, _ in prepared)
    cell_h = max(crop.height for _, _, crop, _ in prepared)
    if columns is None:
        columns = min(len(prepared), max(1, math.ceil(math.sqrt(len(prepared) * cell_h / cell_w))))
    integer(columns, "columns", 1)
    columns = min(columns, len(prepared))
    rows = math.ceil(len(prepared) / columns)
    atlas = Image.new("RGBA", (columns * cell_w, rows * cell_h))
    runtime_frames = []
    source_frames = []
    crops_to_write = []
    for index, (action_name, frame, crop, sheet_path) in enumerate(prepared):
        x, y = (index % columns) * cell_w, (index // columns) * cell_h
        # Paste without a mask preserves RGB and alpha exactly, including soft edges.
        atlas.paste(crop, (x, y))
        bounds = crop.getchannel("A").getbbox()
        assert bounds is not None
        left, top, right, bottom = bounds
        runtime_frame = {
            "name": frame["name"], "clip": action_name,
            "duration_ms": frame["duration_ms"], "pivot": frame["pivot"],
            "frame": {"x": x, "y": y, "w": crop.width, "h": crop.height},
            "trimmed_bounds": {"x": left, "y": top, "w": right - left, "h": bottom - top},
        }
        if "combat" in frame:
            runtime_frame["combat"] = frame["combat"]
        runtime_frames.append(runtime_frame)
        crop_path = work_dir / action_name / "frames" / f"{frame['name']}.png"
        crops_to_write.append((crop_path, crop))
        source_frame = {
            "name": frame["name"], "action": action_name,
            "sheet": relative(sheet_path, output_dir),
            "source_rect": frame["source_rect"],
            "exported_frame": relative(crop_path, output_dir),
        }
        if "phase" in frame:
            source_frame["phase"] = frame["phase"]
        source_frames.append(source_frame)

    atlas_path = output_dir / f"{character}-fighter-atlas.png"
    manifest_path = output_dir / f"{character}-fighter.sprite.json"
    provenance_path = output_dir / f"{character}-fighter.provenance.json"
    protected = {production_path, *sheets}
    destinations = {path.resolve() for path in
                    [atlas_path, manifest_path, provenance_path, *(p for p, _ in crops_to_write)]}
    require(not protected & destinations, "export would overwrite an original source file")
    output_dir.mkdir(parents=True, exist_ok=True)
    for crop_path, crop in crops_to_write:
        crop_path.parent.mkdir(parents=True, exist_ok=True)
        crop.save(crop_path)
    atlas.save(atlas_path)
    write_json(manifest_path, {
        "schema": "borrow-fighters.sprite.v1", "image": atlas_path.name,
        "source": relative(production_path, output_dir),
        "cell": {"w": cell_w, "h": cell_h}, "default_pivot": prepared[0][1]["pivot"],
        "scale": scale, "frames": runtime_frames, "clips": clips,
        "notes": ["Candidate assembled from explicitly reviewed action frames.",
                  "Combat metadata is included only when supplied explicitly in production.json.",
                  f"Source rectangles and production provenance: {provenance_path.name}"],
    })
    write_json(provenance_path, {
        "schema": "borrow-fighters.sprite-provenance.v1", "character": character,
        "production": relative(production_path, output_dir),
        "production_sha256": file_hash(production_path),
        "production_snapshot": production,
        "sources": [{"path": relative(path, output_dir), "sha256": file_hash(path)}
                    for path in sheets],
        "frames": source_frames,
        "atlas_sha256": file_hash(atlas_path), "manifest_sha256": file_hash(manifest_path),
    })
    return manifest_path


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("production", type=Path, help="assets/production/<character>/production.json")
    parser.add_argument("--output-dir", type=Path, help="defaults to assets/candidates/<character>")
    parser.add_argument("--columns", type=int, help="packing columns; default balances atlas dimensions")
    args = parser.parse_args()
    try:
        print(build(args.production, args.output_dir, args.columns))
    except (OSError, ValueError) as error:
        parser.exit(1, f"error: {error}\n")


if __name__ == "__main__":
    main()
