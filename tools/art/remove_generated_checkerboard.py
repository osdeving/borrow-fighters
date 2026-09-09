"""Remove only border-connected pale neutral backdrop from generated sheets.

This is preparation of AI-generated artwork, not an artwork generator. Inspect
the result on both dark and light backgrounds before marking an action reviewed.
The user authorized local background removal for this production run.
"""

import argparse
from pathlib import Path

import numpy as np
from PIL import Image, ImageDraw, ImageFilter


def remove_background(source: Path, destination: Path) -> None:
    image = Image.open(source).convert("RGBA")
    rgba = np.array(image)
    rgb = rgba[:, :, :3].astype(np.int16)
    pale = (rgb.min(axis=2) >= 210) & (rgb.max(axis=2) - rgb.min(axis=2) <= 15)
    # A revised sheet may retain a pale gap beside already extracted alpha.
    # Treat existing transparency as exterior so those connected remnants can
    # be removed without touching enclosed costume highlights or shoe soles.
    pale |= rgba[:, :, 3] == 0
    mask = Image.fromarray(np.where(pale, 255, 0).astype(np.uint8))
    # A one-pixel border connects all background patches touching any edge.
    padded = Image.new("L", (mask.width + 2, mask.height + 2), 255)
    padded.paste(mask, (1, 1))
    ImageDraw.floodfill(padded, (0, 0), 128)
    background = np.asarray(padded)[1:-1, 1:-1] == 128
    rgba[background] = 0
    # Pale antialias pixels mixed with the matte remain immediately outside the
    # drawn dark contour. Remove at most two such border pixels; enclosed eyes,
    # highlights and shoe soles never participate in this edge operation.
    for _ in range(2):
        outside = Image.fromarray(np.where(rgba[:, :, 3] == 0, 255, 0).astype(np.uint8))
        touches_background = np.asarray(outside.filter(ImageFilter.MaxFilter(3))) > 0
        fringe = touches_background & (rgba[:, :, 3] > 0) & (rgb.min(axis=2) > 85) & (rgb.max(axis=2) - rgb.min(axis=2) < 30)
        rgba[fringe] = 0
    destination.parent.mkdir(parents=True, exist_ok=True)
    Image.fromarray(rgba).save(destination)
    print(f"{destination}: removed {int(background.sum())} backdrop pixels")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("source", type=Path)
    parser.add_argument("destination", type=Path)
    args = parser.parse_args()
    remove_background(args.source, args.destination)
