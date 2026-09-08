"""Prepare the existing Rust projectile by removing connected neutral matte.

This is authorized local alpha cleanup of a reused asset, not image generation.
Original RGB values of every retained pixel are preserved. The orange gear,
brown/black contours, R mark and colored speed streaks are not repainted.
"""

import argparse
import hashlib
import json
from pathlib import Path

import numpy as np
from PIL import Image, ImageDraw


def clean(source: Path, destination: Path, padding: int = 4) -> dict:
    image = Image.open(source).convert("RGBA")
    original = np.asarray(image).copy()
    rgba = original.copy()
    rgb = rgba[:, :, :3].astype(np.int16)
    # Only the light neutral contaminant can join outside transparency. Colored
    # orange/cream highlights and the low-value dark contour are excluded.
    neutral = (rgb.min(axis=2) >= 100) & (rgb.max(axis=2) - rgb.min(axis=2) <= 46)
    traversable = (rgba[:, :, 3] == 0) | neutral
    padded_mask = Image.new("L", (image.width + 2, image.height + 2), 255)
    padded_mask.paste(Image.fromarray(np.where(traversable, 255, 0).astype(np.uint8)), (1, 1))
    ImageDraw.floodfill(padded_mask, (0, 0), 128)
    outside = np.asarray(padded_mask)[1:-1, 1:-1] == 128
    removed = outside & (rgba[:, :, 3] > 0)
    rgba[outside] = 0

    cleaned = Image.fromarray(rgba)
    canvas = Image.new("RGBA", (image.width + 2 * padding, image.height + 2 * padding))
    canvas.paste(cleaned, (padding, padding))
    canvas.save(destination)
    kept = rgba[:, :, 3] > 0
    report = {
        "operation": "local alpha cleanup of reused Rust projectile; no image generation",
        "source": source.name,
        "source_sha256": hashlib.sha256(source.read_bytes()).hexdigest(),
        "output": destination.name,
        "source_size": list(image.size),
        "output_size": list(canvas.size),
        "symmetric_padding_px": padding,
        "neutral_threshold_min_rgb": 100,
        "neutral_threshold_rgb_range_max": 46,
        "removed_nontransparent_pixels": int(removed.sum()),
        "retained_nontransparent_pixels": int(kept.sum()),
        "retained_rgb_unchanged": bool(np.array_equal(rgba[:, :, :3][kept], original[:, :, :3][kept])),
        "alpha_bounds": list(canvas.getchannel("A").getbbox()),
        "runtime_scale": 0.6,
        "source_canvas_runtime_px": [round(v * 0.6, 3) for v in image.size],
        "output_canvas_runtime_px": [round(v * 0.6, 3) for v in canvas.size],
        "visual_transform": "For every retained source point, (p+padding)-(size+2*padding)/2 = p-size/2. Center and geometry are unchanged at renderer scale0.6.",
        "combat": "No ProjectileSpec, origin, collision or fighter metadata change.",
    }
    destination.with_suffix(".json").write_text(json.dumps(report, indent=2) + "\n")
    return report


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("source", type=Path)
    parser.add_argument("destination", type=Path)
    parser.add_argument("--padding", type=int, default=4)
    args = parser.parse_args()
    print(json.dumps(clean(args.source, args.destination, args.padding), indent=2))
