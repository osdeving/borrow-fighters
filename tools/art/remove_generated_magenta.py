"""Extract opaque cartoon art from a flat magenta generation matte.

Remove magenta in gaps as well as outside the body, and unmix the matte from
antialiased contours. Do not use for characters containing actual purple art.
The original sheet remains unchanged; inspect the result before exporting.
"""

import argparse
from pathlib import Path

import numpy as np
from PIL import Image, ImageFilter


def remove_background(source: Path, destination: Path, rust_contour: bool = False) -> None:
    rgba = np.array(Image.open(source).convert("RGBA"))
    rgb = rgba[:, :, :3].astype(np.float32)
    # In a black outline blended over FF00FF, red/blue excess estimates matte
    # coverage. Orange costume pixels have low blue and are therefore retained.
    spill = np.maximum(0, np.minimum(rgb[:, :, 0], rgb[:, :, 2]) - rgb[:, :, 1])
    alpha = 1.0 - spill / 255.0
    background = (spill > 180) & (rgb[:, :, 1] < 100)
    alpha[background] = 0
    mixed = (spill > 12) & ~background
    # Unmix against the known magenta matte, avoiding dark-purple edge residue.
    for channel, key in enumerate((255.0, 0.0, 255.0)):
        clean = (rgb[:, :, channel] - (1.0 - alpha) * key) / np.maximum(alpha, .01)
        rgb[:, :, channel] = np.where(mixed, np.clip(clean, 0, 255), rgb[:, :, channel])
    rgba[:, :, :3] = rgb.astype(np.uint8)
    rgba[:, :, 3] = np.minimum(rgba[:, :, 3], np.round(alpha * 255).astype(np.uint8))
    # The generated matte can be uneven in hue. Reject residual saturated red
    # or blue immediately outside the dark contour; Rust's orange art retains
    # substantial green. This is deliberately unsuitable for red/blue fighters.
    if rust_contour:
        for _ in range(12):
            outside = Image.fromarray(np.where(rgba[:, :, 3] == 0, 255, 0).astype(np.uint8))
            edge = np.asarray(outside.filter(ImageFilter.MaxFilter(3))) > 0
            r, g, b = (rgba[:, :, i].astype(np.float32) for i in range(3))
            residue = edge & (g < 30) & (np.maximum(r,b) > 25) & (np.maximum(r,b) > 3*g) & (np.minimum(r,b) < np.maximum(r,b)/2)
            if not residue.any():
                break
            rgba[residue] = 0
    rgba[rgba[:, :, 3] == 0] = 0
    destination.parent.mkdir(parents=True, exist_ok=True)
    Image.fromarray(rgba).save(destination)
    print(f"{destination}: transparent={int((rgba[:, :, 3] == 0).sum())}")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("source", type=Path)
    parser.add_argument("destination", type=Path)
    parser.add_argument("--rust-contour", action="store_true", help="Remove red/blue matte residue absent from Rust's orange palette; never use on red/blue fighters")
    args = parser.parse_args()
    remove_background(args.source, args.destination, args.rust_contour)
