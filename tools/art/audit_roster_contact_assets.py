#!/usr/bin/env python3
"""Audit the four added contact atlases without changing any artwork.

Compare legacy manifest entries with a Git revision, then inspect the 128 new
RGBA crops, pivots, bounds and relative upright height. Pixel hashes establish
distinct sources; articulated movement still requires viewing runtime frames.
"""

import argparse
import hashlib
import json
import subprocess
from pathlib import Path

from PIL import Image


ROOT = Path(__file__).resolve().parents[2]
CHARACTERS = ("rust", "duke", "c", "go")
PROFILES = ("head", "body", "low", "guard_high", "guard_low", "launch", "fall", "rise")


def audit(character, revision):
    relative = f"assets/candidates/{character}/{character}-fighter.sprite.json"
    path = ROOT / relative
    before = json.loads(subprocess.check_output(["git", "show", f"{revision}:{relative}"], cwd=ROOT))
    current = json.loads(path.read_text())
    for key in ("schema", "image", "cell", "default_pivot", "scale"):
        assert current.get(key) == before.get(key), f"{character}: base {key} changed"
    frames = {frame["name"]: frame for frame in current["frames"]}
    clips = {clip["name"]: clip for clip in current["clips"]}
    for frame in before["frames"]:
        assert frames[frame["name"]] == frame, f"{character}: legacy frame changed: {frame['name']}"
    for clip in before["clips"]:
        assert clips[clip["name"]] == clip, f"{character}: legacy clip changed: {clip['name']}"
    legacy_images = {before["image"]}
    legacy_images.update(frame.get("image", before["image"]) for frame in before["frames"])
    preserved_images = {}
    for source in sorted(legacy_images):
        image_path = (path.parent / source).resolve()
        relative_image = str(image_path.relative_to(ROOT))
        baseline_blob = subprocess.check_output(
            ["git", "rev-parse", f"{revision}:{relative_image}"], cwd=ROOT, text=True
        ).strip()
        current_blob = subprocess.check_output(
            ["git", "hash-object", str(image_path)], cwd=ROOT, text=True
        ).strip()
        assert baseline_blob == current_blob, f"{character}: legacy image changed: {relative_image}"
        preserved_images[relative_image] = current_blob
    assert len(current["frames"]) == len(before["frames"]) + 32, character
    assert len(current["clips"]) == len(before["clips"]) + 8, character
    idle = frames[clips["idle"]["frames"][0]]
    idle_height = idle.get("trimmed_bounds", idle["frame"])["h"]
    observations = []
    all_hashes = set()
    source_paths = set()
    for profile in PROFILES:
        clip = clips[f"reaction_{profile}"]
        assert not clip["loop"] and len(clip["frames"]) == 4, (character, profile)
        for name in clip["frames"]:
            frame = frames[name]
            assert not frame.get("combat"), f"{name}: optional art introduces collision data"
            assert frame["clip"] == clip["name"]
            assert frame["duration_ms"] > 0
            source = path.parent / frame.get("image", current["image"])
            source_paths.add(source)
            with Image.open(source) as image:
                assert image.mode == "RGBA", f"{source}: expected RGBA"
                rect = frame["frame"]
                box = (rect["x"], rect["y"], rect["x"] + rect["w"], rect["y"] + rect["h"])
                assert box[0] >= 0 and box[1] >= 0
                assert box[2] <= image.width and box[3] <= image.height
                crop = image.crop(box)
            alpha = crop.getchannel("A")
            bounds = alpha.getbbox()
            assert bounds, f"{name}: empty drawing"
            assert alpha.getextrema()[0] == 0, f"{name}: no transparency"
            assert 0 < bounds[0] < bounds[2] < crop.width, f"{name}: horizontal clipping"
            assert 0 < bounds[1] < bounds[3] < crop.height, f"{name}: vertical clipping"
            pivot = frame["pivot"]
            assert 0 <= pivot["x"] <= crop.width and 0 <= pivot["y"] <= crop.height
            digest = hashlib.sha256(crop.tobytes()).hexdigest()
            assert digest not in all_hashes, f"{name}: reused pixels from another reaction"
            all_hashes.add(digest)
            opaque = alpha.point(lambda value: 255 if value >= 32 else 0).getbbox()
            assert opaque
            observations.append({
                "name": name, "profile": profile,
                "source": str(source.relative_to(ROOT)), "frame": rect,
                "pivot": pivot, "alpha_bounds": bounds, "opaque_bounds": opaque,
                "visible_height_relative_to_idle": round((opaque[3] - opaque[1]) / idle_height, 4),
                "lowest_pixel_relative_to_pivot": bounds[3] - pivot["y"],
                "rgba_sha256": digest,
            })
    return {
        "character": character, "manifest": relative,
        "manifest_sha256": hashlib.sha256(path.read_bytes()).hexdigest(),
        "legacy_frames_preserved": len(before["frames"]),
        "legacy_clips_preserved": len(before["clips"]),
        "legacy_image_blobs_preserved": preserved_images,
        "scale_before_and_after": current.get("scale"), "idle_trimmed_height": idle_height,
        "distinct_reaction_drawings": len(all_hashes), "frames": observations,
        "atlas_sha256": {str(source.relative_to(ROOT)): hashlib.sha256(source.read_bytes()).hexdigest() for source in sorted(source_paths)},
    }


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--baseline", default="HEAD", help="Git revision preceding the new reaction assets")
    parser.add_argument("--report", type=Path, required=True)
    args = parser.parse_args()
    revision = subprocess.check_output(["git", "rev-parse", "--verify", args.baseline], cwd=ROOT, text=True).strip()
    characters = [audit(character, revision) for character in CHARACTERS]
    report = {
        "baseline_revision": revision,
        "method": "Unchanged legacy manifest entries; original RGBA crops, pivots, alpha bounds and distinct pixel hashes",
        "limitation": "Distinct pixels and bounding boxes do not establish articulated motion; inspect real contact captures separately.",
        "new_drawings": sum(character["distinct_reaction_drawings"] for character in characters),
        "characters": characters,
    }
    args.report.parent.mkdir(parents=True, exist_ok=True)
    args.report.write_text(json.dumps(report, ensure_ascii=False, indent=2) + "\n")
    print(f"PASS: {report['new_drawings']} distinct reaction drawings; legacy frames/clips and manifest scale preserved")


if __name__ == "__main__":
    main()
