"""Pack imagegen reaction drawings without repainting or replacing existing clips.

Each authored four-drawing strip has one scale. Source cutouts retain the generated
RGBA values; compound rectangles separate the interleaved C airborne silhouettes.
Run with --preview while reviewing art, or without it to write runtime manifests.
"""

import argparse
import hashlib
import json
from pathlib import Path

from PIL import Image, ImageDraw

ROOT = Path(__file__).resolve().parents[3]
PRODUCTION = Path(__file__).resolve().parent
CELL = (448, 352)
PIVOT = (224, 310)
PROFILES = ["head", "body", "low", "guard_high", "guard_low", "launch", "fall", "rise"]
CONFIG = {
    "rust": {
        "upper_y": [0, 334, 625, 940, 1254],
        "lower_y": [0, 365, 718, 913, 1254],
        "upper_x": [[0, 335, 640, 945, 1254]] * 4,
        "lower_x": [[0, 335, 650, 940, 1254], [0, 335, 650, 940, 1254],
                    [0, 336, 623, 928, 1254], [0, 344, 640, 943, 1254]],
        "scale": [0.97] * 4 + [0.83, 0.907, 0.907, 0.907],
        "fall_end": [913, 913, 913, 907],
    },
    "duke": {
        "upper_y": [0, 310, 620, 927, 1254],
        "lower_y": [0, 348, 695, 913, 1254],
        "upper_x": [[0, 335, 640, 950, 1254]] * 4,
        "lower_x": [[0, 335, 640, 940, 1254]] * 4,
        "scale": [0.977] * 4 + [0.89, 0.921, 0.921, 0.921],
        "fall_end": [913] * 4,
    },
    "c": {
        "upper_y": [0, 357, 662, 950, 1254],
        "lower_y": [0, 377, 714, 913, 1254],
        "upper_x": [[0, 350, 650, 940, 1254], [0, 335, 640, 940, 1254],
                    [0, 362, 640, 940, 1254], [0, 335, 640, 940, 1254]],
        "lower_x": [[0, 335, 640, 930, 1254], [0, 330, 640, 930, 1254],
                    [0, 352, 633, 938, 1254], [0, 365, 640, 938, 1254]],
        "scale": [0.846] * 4 + [0.899] * 4,
        "fall_end": [913, 913, 914, 902],
    },
    "go": {
        "upper_y": [0, 344, 650, 921, 1254],
        "lower_y": [0, 340, 690, 900, 1254],
        "upper_x": [[0, 335, 640, 943, 1254]] * 4,
        "lower_x": [[0, 335, 657, 950, 1254], [0, 335, 678, 935, 1254],
                    [0, 345, 629, 927, 1254], [0, 351, 660, 945, 1254]],
        "scale": [0.903] * 4 + [0.68, 0.862, 0.862, 0.862],
        "fall_end": [900, 900, 900, 885],
    },
}


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def cutout(source, rectangles):
    """Compose rectangular crops in source coordinates, retaining original alpha."""
    box = [min(r[0] for r in rectangles), min(r[1] for r in rectangles),
           max(r[2] for r in rectangles), max(r[3] for r in rectangles)]
    image = Image.new("RGBA", (box[2] - box[0], box[3] - box[1]))
    for rectangle in rectangles:
        image.paste(source.crop(rectangle), (rectangle[0] - box[0], rectangle[1] - box[1]))
    # Tight rectangular extent with a four-pixel antialias margin. Extremely faint
    # background speckles outside this extent are not part of the selected pose.
    bounds = image.getchannel("A").point(lambda a: 255 if a > 32 else 0).getbbox()
    assert bounds
    bounds = (max(0, bounds[0] - 4), max(0, bounds[1] - 4),
              min(image.width, bounds[2] + 4), min(image.height, bounds[3] + 4))
    box = [box[0] + bounds[0], box[1] + bounds[1],
           box[0] + bounds[2], box[1] + bounds[3]]
    return image.crop(bounds), box


def export(character, config, preview):
    own = ROOT / "assets/production" / character / "reactions-own-2026-09-09"
    candidate = ROOT / "assets/candidates" / character
    manifest_path = candidate / f"{character}-fighter.sprite.json"
    manifest = json.loads(manifest_path.read_text())
    frames = [f for f in manifest["frames"] if not f["clip"].startswith("reaction_")]
    clips = [c for c in manifest["clips"] if not c["name"].startswith("reaction_")]
    baseline = {"frames": frames, "clips": clips,
                "scale": manifest["scale"], "image": manifest["image"]}
    baseline_path = own / "preserved-baseline.json"
    if baseline_path.exists():
        assert json.loads(baseline_path.read_text()) == baseline
    else:
        baseline_path.write_text(json.dumps(baseline, indent=2) + "\n")
    manifest["frames"] = frames[:]
    manifest["clips"] = clips[:]
    atlas = Image.new("RGBA", (CELL[0] * 4, CELL[1] * 8))
    audit_frames = []
    atlas_name = f"{character}-contact-reactions-atlas.png"
    for row, profile in enumerate(PROFILES):
        sheet = "upper" if row < 4 else "lower"
        source = Image.open(own / f"{sheet}-alpha.png")
        assert source.mode == "RGBA" and source.getchannel("A").getextrema() == (0, 255)
        source_row = row % 4
        xs, ys = config[f"{sheet}_x"][source_row], config[f"{sheet}_y"]
        scale = config["scale"][row]
        names = []
        for column in range(4):
            y0, y1 = ys[source_row:source_row + 2]
            if row == 6:
                y1 = config["fall_end"][column]
            elif row == 7:
                y0 = config["fall_end"][column]
            rectangles = [[xs[column], y0, xs[column + 1], y1]]
            if character == "c" and row == 5 and column < 2:
                rectangles = ([[0, 377, 330, 714], [330, 580, 379, 714]]
                              if column == 0 else
                              [[330, 377, 640, 580], [379, 580, 640, 714]])
            piece, box = cutout(source, rectangles)
            visible = piece.getchannel("A").point(lambda a: 255 if a > 64 else 0)
            bounds = visible.getbbox()
            assert bounds
            if row == 5:
                # The airborne body's articulation floats above one common baseline.
                pivot = [(box[0] + box[2]) / 2, ys[source_row + 1] - 12]
            elif row == 6 or (row == 7 and column < 2):
                pivot = [(box[0] + box[2]) / 2, box[1] + bounds[3] - 1]
            else:
                feet = visible.crop((0, max(0, bounds[3] - 8), piece.width, bounds[3])).getbbox()
                pivot = [box[0] + (feet[0] + feet[2]) / 2, box[1] + bounds[3] - 1]
            name = f"reaction_{profile}_{column:02}"
            names.append(name)
            size = tuple(round(axis * scale) for axis in piece.size)
            resized = piece.resize(size, Image.Resampling.LANCZOS)
            offset = [round(PIVOT[0] - (pivot[0] - box[0]) * scale),
                      round(PIVOT[1] - (pivot[1] - box[1]) * scale)]
            assert min(offset) >= 0, (character, name, offset)
            assert offset[0] + size[0] <= CELL[0] and offset[1] + size[1] <= CELL[1], (name, offset, size)
            atlas.paste(resized, (column * CELL[0] + offset[0], row * CELL[1] + offset[1]))
            trimmed = resized.getchannel("A").point(lambda a: 255 if a > 32 else 0).getbbox()
            manifest["frames"].append({
                "name": name, "clip": f"reaction_{profile}", "duration_ms": 70,
                "image": atlas_name, "pivot": {"x": PIVOT[0], "y": PIVOT[1]},
                "frame": {"x": column * CELL[0], "y": row * CELL[1], "w": CELL[0], "h": CELL[1]},
                "trimmed_bounds": {"x": offset[0] + trimmed[0], "y": offset[1] + trimmed[1],
                                   "w": trimmed[2] - trimmed[0], "h": trimmed[3] - trimmed[1]},
            })
            audit_frames.append({"name": name, "source": f"{sheet}-alpha.png",
                                 "source_rectangles": rectangles, "source_crop": box,
                                 "source_pivot": pivot, "uniform_strip_scale": scale,
                                 "offset": offset, "size": size,
                                 "pixels_sha256": hashlib.sha256(resized.tobytes()).hexdigest()})
        manifest["clips"].append({"name": f"reaction_{profile}", "loop": False, "frames": names})
    atlas_path = own / "preview-atlas.png" if preview else candidate / atlas_name
    atlas.save(atlas_path)
    note = f"Authored contact reactions: ../../production/{character}/reactions-own-2026-09-09/README.md"
    if note not in manifest["notes"]:
        manifest["notes"].append(note)
    if not preview:
        manifest_path.write_text(json.dumps(manifest, indent=2, ensure_ascii=False) + "\n")
    audit = {"method": "Imagegen art and alpha; rectangular crops, uniform strip resize and RGBA packing only.",
             "cell": CELL, "pivot": PIVOT, "baseline_sha256": sha(baseline_path),
             "atlas_sha256": sha(atlas_path), "frames": audit_frames,
             "source_sha256": {f"{s}-alpha.png": sha(own / f"{s}-alpha.png") for s in ["upper", "lower"]}}
    (own / "export-audit.json").write_text(json.dumps(audit, indent=2) + "\n")
    # Review at exact game pixel scale, alternating dark/light strips.
    review = Image.new("RGB", atlas.size)
    draw = ImageDraw.Draw(review)
    for row, profile in enumerate(PROFILES):
        color = (26, 30, 39) if row % 2 == 0 else (226, 221, 209)
        draw.rectangle((0, row * CELL[1], atlas.width, (row + 1) * CELL[1]), fill=color)
        draw.text((8, row * CELL[1] + 10), profile, fill=(145, 155, 170))
        draw.line((0, row * CELL[1] + PIVOT[1], atlas.width, row * CELL[1] + PIVOT[1]), fill=(105, 125, 140))
    review.paste(atlas, (0, 0), atlas)
    review.save(own / "review-game-scale.png")
    print(f"{character}: 32 authored drawings; {len(frames)} frames/{len(clips)} old clips preserved; preview={preview}")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--preview", action="store_true", help="Write production previews without mutating runtime assets")
    options = parser.parse_args()
    for character, config in CONFIG.items():
        export(character, config, options.preview)
