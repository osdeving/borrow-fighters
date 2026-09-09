"""Pack explicitly selected imagegen cutouts; preserve generated alpha and old clips."""

import hashlib
import json
from pathlib import Path

from PIL import Image


ROOT = Path(__file__).resolve().parents[4]
PRODUCTION = Path(__file__).resolve().parent
CANDIDATE = ROOT / "assets/candidates/python"
MANIFEST = CANDIDATE / "python-fighter.sprite.json"
CELL = (448, 352)
DEST_PIVOT = (224, 310)

# Source boxes are x0,y0,x1,y1. Pivots are absolute source coordinates.
# One uniform scale per sheet preserves the height differences of folded poses.
SELECTIONS = [
    ("reaction_head", "hits", 0.68, [
        ([44, 0, 362, 410], [165, 404]),
        ([408, 0, 712, 410], [545, 404]),
        ([719, 8, 1090, 409], [935, 404]),
        ([1118, 12, 1440, 410], [1285, 404]),
    ]),
    ("reaction_body", "hits", 0.68, [
        ([45, 409, 330, 756], [151, 749]),
        ([420, 447, 718, 750], [539, 749]),
        ([775, 407, 1110, 754], [920, 749]),
        ([1150, 412, 1440, 755], [1305, 749]),
    ]),
    ("reaction_low", "hits", 0.68, [
        ([20, 755, 330, 1086], [194, 1075]),
        ([424, 750, 712, 1082], [568, 1075]),
        ([728, 792, 1092, 1080], [890, 1075]),
        ([1146, 760, 1440, 1080], [1301, 1075]),
    ]),
    ("reaction_guard_high", "guards", 0.55, [
        ([70, 0, 465, 520], [270, 508]),
        ([498, 36, 889, 521], [715, 508]),
        ([980, 20, 1319, 521], [1155, 508]),
        ([1398, 26, 1748, 521], [1570, 508]),
    ]),
    ("reaction_guard_low", "guards", 0.55, [
        ([32, 531, 456, 880], [247, 863]),
        ([537, 562, 854, 880], [719, 863]),
        ([951, 530, 1312, 880], [1127, 863]),
        ([1411, 531, 1753, 880], [1573, 863]),
    ]),
    ("reaction_launch", "launch", 0.40, [
        ([22, 0, 600, 705], [526, 704]),
        ([656, 10, 1072, 645], [891, 704]),
        ([1180, 0, 1512, 600], [1380, 704]),
        ([1730, 17, 2150, 715], [2088, 704]),
    ]),
    ("reaction_fall", "ground-rise", 0.57, [
        ([0, 251, 382, 458], [220, 446]),
        ([383, 200, 755, 458], [585, 446]),
        ([1155, 231, 1536, 459], [1340, 446]),
        ([755, 290, 1155, 452], [963, 446]),
    ]),
    ("reaction_rise", "ground-rise", 0.57, [
        ([0, 751, 456, 964], [246, 956]),
        ([459, 664, 829, 965], [650, 956]),
        ([839, 591, 1156, 967], [985, 956]),
        ([1231, 477, 1530, 971], [1400, 956]),
    ]),
]


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    manifest = json.loads(MANIFEST.read_text())
    previous_frames = [frame for frame in manifest["frames"]
                       if not frame["clip"].startswith("reaction_")]
    previous_clips = [clip for clip in manifest["clips"]
                      if not clip["name"].startswith("reaction_")]
    baseline = {"frames": previous_frames, "clips": previous_clips,
                "scale": manifest["scale"], "image": manifest["image"]}
    baseline_path = PRODUCTION / "preserved-baseline.json"
    if baseline_path.exists():
        assert json.loads(baseline_path.read_text()) == baseline
    else:
        baseline_path.write_text(json.dumps(baseline, indent=2) + "\n")

    atlas = Image.new("RGBA", (CELL[0] * 4, CELL[1] * len(SELECTIONS)))
    manifest["frames"] = previous_frames[:]
    manifest["clips"] = previous_clips[:]
    records = []
    for row, (clip, sheet, scale, poses) in enumerate(SELECTIONS):
        source_path = PRODUCTION / f"{sheet}-alpha.png"
        source = Image.open(source_path)
        assert source.mode == "RGBA"
        names = []
        for index, (box, pivot) in enumerate(poses):
            name = f"{clip}_{index:02}"
            names.append(name)
            cutout = source.crop(box)
            size = tuple(round(axis * scale) for axis in cutout.size)
            cutout = cutout.resize(size, Image.Resampling.LANCZOS)
            offset = (round(DEST_PIVOT[0] - (pivot[0] - box[0]) * scale),
                      round(DEST_PIVOT[1] - (pivot[1] - box[1]) * scale))
            assert min(offset) >= 0, (name, offset)
            assert offset[0] + size[0] <= CELL[0], (name, offset, size)
            assert offset[1] + size[1] <= CELL[1], (name, offset, size)
            position = (index * CELL[0] + offset[0], row * CELL[1] + offset[1])
            atlas.paste(cutout, position)
            # Bounds describe the visible body; the RGBA export retains all alpha.
            bounds = cutout.getchannel("A").point(lambda value: 255 if value > 32 else 0).getbbox()
            assert bounds is not None
            manifest["frames"].append({
                "name": name, "clip": clip, "duration_ms": 70,
                "image": "python-reactions-atlas.png",
                "pivot": {"x": DEST_PIVOT[0], "y": DEST_PIVOT[1]},
                "frame": {"x": index * CELL[0], "y": row * CELL[1],
                          "w": CELL[0], "h": CELL[1]},
                "trimmed_bounds": {"x": offset[0] + bounds[0],
                                   "y": offset[1] + bounds[1],
                                   "w": bounds[2] - bounds[0],
                                   "h": bounds[3] - bounds[1]},
            })
            records.append({"name": name, "sheet": source_path.name,
                            "source_box_xyxy": box, "source_pivot": pivot,
                            "uniform_sheet_scale": scale,
                            "destination_offset": offset, "resized_size": size})
        manifest["clips"].append({"name": clip, "loop": False, "frames": names})

    note = "Dedicated impact/guard/air/ground reactions: ../../production/python/reactions-2026-09-09/README.md"
    if note not in manifest["notes"]:
        manifest["notes"].append(note)
    atlas_path = CANDIDATE / "python-reactions-atlas.png"
    atlas.save(atlas_path)
    MANIFEST.write_text(json.dumps(manifest, indent=2, ensure_ascii=False) + "\n")
    audit = {
        "method": "Imagegen artwork and alpha; crop, uniform sheet resize, RGBA packing only. No local matte removal or repaint.",
        "baseline_preserved": baseline,
        "cell": CELL, "pivot": DEST_PIVOT,
        "atlas_sha256": sha(atlas_path), "manifest_sha256": sha(MANIFEST),
        "source_sha256": {f"{sheet}-alpha.png": sha(PRODUCTION / f"{sheet}-alpha.png")
                          for sheet in {item[1] for item in SELECTIONS}},
        "frames": records,
    }
    (PRODUCTION / "export-audit.json").write_text(json.dumps(audit, indent=2) + "\n")
    print(f"Added {len(records)} drawings in {len(SELECTIONS)} clips; preserved "
          f"{len(previous_frames)} previous frames and {len(previous_clips)} clips")


if __name__ == "__main__":
    main()
