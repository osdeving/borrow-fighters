#!/usr/bin/env python3
"""Export explicit sprite crops through ImageMagick and seal their provenance.

Recipe paths and landmarks are relative to the recipe directory and original
source image respectively. No trimming, resizing, painting or anchor inference
occurs. ``--check`` re-exports into temporary storage and compares exact bytes,
without touching the sources, exports or manifest. Requires Python 3.8+ and
ImageMagick 6 (convert) or 7 (magick); no Pillow/NumPy dependency.
"""

import argparse
import hashlib
import json
import math
import os
from pathlib import Path, PurePosixPath
import re
import shutil
import subprocess
import sys
import tempfile


class ImportFailure(ValueError):
    """Invalid recipe, unsafe destination or non-reproducible export."""


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def relative_path(root, value, label):
    if not isinstance(value, str) or not value or "\\" in value or ":" in value:
        raise ImportFailure(f"{label}: expected a safe relative POSIX path")
    path = PurePosixPath(value)
    if path.is_absolute() or ".." in path.parts or any(ord(c) < 32 for c in value):
        raise ImportFailure(f"{label}: path must stay inside the recipe directory")
    resolved = (root / path).resolve()
    try:
        resolved.relative_to(root)
    except ValueError as error:
        raise ImportFailure(f"{label}: symlink escapes the recipe directory") from error
    if resolved == root:
        raise ImportFailure(f"{label}: expected a file path")
    if any(parent.exists() and not parent.is_dir() for parent in resolved.parents):
        raise ImportFailure(f"{label}: a parent path is not a directory")
    return resolved


class ImageMagick:
    def __init__(self):
        executable = shutil.which("magick") or shutil.which("convert")
        if not executable:
            raise ImportFailure("ImageMagick is required (magick or convert on PATH)")
        self.executable = executable
        self.environment = dict(os.environ, MAGICK_THREAD_LIMIT="1", LC_ALL="C")
        self.version = self.run("-version").decode().splitlines()[0]
        if "ImageMagick" not in self.version:
            raise ImportFailure("convert on PATH is not ImageMagick")

    def run(self, *arguments):
        result = subprocess.run(
            [self.executable, *map(str, arguments)], capture_output=True,
            env=self.environment, check=False,
        )
        if result.returncode:
            raise ImportFailure(result.stderr.decode(errors="replace").strip())
        return result.stdout

    def size(self, source):
        fields = self.run(source, "-format", "%w %h %n", "info:").decode().split()
        if len(fields) != 3 or fields[2] != "1":
            raise ImportFailure(f"{source.name}: expected one still image")
        return [int(fields[0]), int(fields[1])]

    def flat_key_domain(self, source, work):
        """Limit unmixing to the keyed field and a two-pixel edge neighborhood.

        ImageMagick expands only a selection mask, never the exported alpha.
        Bounded reach prevents a gap in a dark outline from selecting an entire
        purple garment. The interior RGB and alpha remain untouched.
        """
        seeds, candidates, domain = (work / name for name in
                                      ("seeds.miff", "candidates.miff", "domain.miff"))
        self.run(source, "-alpha", "off", "-fx", "min(r,b)-g>180/255 && g<100/255", seeds)
        self.run(source, "-alpha", "off", "-fx", "min(r,b)-g>12/255", candidates)
        self.run(seeds, "-morphology", "Dilate", "Diamond:1", candidates,
                 "-compose", "Multiply", "-composite", domain)
        self.run(domain, "-morphology", "Dilate", "Diamond:1", candidates,
                 "-compose", "Multiply", "-composite", domain)
        return domain

    def export(self, source, crop, keying, destination, work):
        x, y, width, height = crop
        raw, matte, clean = (work / name for name in ("crop.miff", "matte.miff", "clean.miff"))
        self.run(source, "-crop", f"{width}x{height}+{x}+{y}", "+repage", "-alpha", "on", raw)
        if keying in ("magenta", "magenta-flat"):
            # Estimate magenta coverage from red/blue excess over green. A flat
            # keyed field is removed, while mixed dark contour pixels retain
            # fractional alpha. No morphological erosion is applied.
            coverage = "(min(r,b)-g>180/255 && g<100/255)?0:1-max(0,min(r,b)-g)"
            self.run(raw, "-alpha", "off", "-fx", coverage, matte)
            if keying == "magenta-flat":
                domain = self.flat_key_domain(raw, work)
                self.run(matte, domain, "-fx", "1-(1-u.r)*v.r", matte)
            # Unmix the known FF00FF contribution using the original matte;
            # changing RGB cannot feed back into alpha or shrink the outline.
            self.run(raw, matte, "-channel", "RB", "-fx",
                     "(u-(1-v.r))/max(v.r,0.001)", "+channel", clean)
            self.run(clean, matte, "-channel", "G", "-fx",
                     "u/max(v.r,0.001)", "+channel", clean)
            self.run(clean, matte, "-channel", "A", "-fx",
                     "u.a*v.r", "+channel", raw)
        # Normalize transparent RGB and strip machine-dependent PNG metadata.
        self.run(raw, "-background", "black", "-alpha", "background", "-strip",
                 "-depth", "8", "-define", "png:exclude-chunks=date,time",
                 "-define", "png:compression-level=9", f"PNG32:{destination}")

    def alpha_summary(self, output, keying, size):
        rgba = self.run(output, "-depth", "8", "rgba:-")
        if len(rgba) != size[0] * size[1] * 4:
            raise ImportFailure(f"{output.name}: exported dimensions changed")
        opaque = partial = residual = 0
        bounds = [size[0], size[1], -1, -1]
        for offset in range(0, len(rgba), 4):
            red, green, blue, alpha = rgba[offset:offset + 4]
            opaque += alpha == 255
            partial += 0 < alpha < 255
            if alpha:
                pixel = offset // 4
                x, y = pixel % size[0], pixel // size[0]
                bounds = [min(bounds[0], x), min(bounds[1], y),
                          max(bounds[2], x), max(bounds[3], y)]
            magenta = (min(red, blue) - green > 24 if keying == "magenta"
                       else min(red, blue) - green > 180 and green < 100)
            if alpha >= 16 and magenta:
                residual += 1
        if not opaque and not partial:
            raise ImportFailure(f"{output.name}: empty alpha after extraction")
        if keying in ("magenta", "magenta-flat") and residual:
            raise ImportFailure(f"{output.name}: {residual} visible magenta pixels remain")
        return {"opaque_pixels": opaque, "partial_pixels": partial,
                "transparent_pixels": size[0] * size[1] - opaque - partial,
                "visible_bounds": bounds}


def load_recipe(recipe_path, images):
    root = recipe_path.parent.resolve()
    try:
        recipe = json.loads(recipe_path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as error:
        raise ImportFailure(f"cannot read recipe: {error}") from error
    if not isinstance(recipe, dict) or recipe.get("version") != 1:
        raise ImportFailure("recipe.version must be 1")
    keying = recipe.get("keying", "none")
    if keying not in ("none", "magenta", "magenta-flat"):
        raise ImportFailure("keying must be none, magenta or magenta-flat")
    sources = recipe.get("sources")
    exports = recipe.get("exports")
    if not isinstance(sources, dict) or not sources:
        raise ImportFailure("sources must be a nonempty object")
    if not isinstance(exports, list) or not exports:
        raise ImportFailure("exports must be a nonempty list")
    metadata = recipe.get("metadata", "import-manifest.json")
    metadata_path = relative_path(root, metadata, "metadata")
    if metadata_path.suffix.lower() != ".json" or (metadata_path.exists() and not metadata_path.is_file()):
        raise ImportFailure("metadata must be a JSON file path")
    source_records = {}
    occupied = {recipe_path.resolve(), metadata_path}
    if len(occupied) != 2:
        raise ImportFailure("metadata cannot overwrite the recipe")
    for name, entry in sources.items():
        if not isinstance(entry, dict):
            raise ImportFailure(f"source {name}: expected an object")
        path = relative_path(root, entry.get("path"), f"source {name}")
        if path in occupied or not path.is_file() or path.suffix.lower() != ".png":
            raise ImportFailure(f"source {name}: expected a distinct existing PNG")
        occupied.add(path)
        sha = digest(path)
        locked = entry.get("sha256")
        if locked is not None and (not isinstance(locked, str)
                                   or not re.fullmatch(r"[0-9a-f]{64}", locked)
                                   or locked != sha):
            raise ImportFailure(f"source {name}: SHA256 lock does not match (source changed)")
        source_records[name] = {"path": entry["path"], "sha256": sha,
                                "size": images.size(path)}
    identifiers = set()
    for entry in exports:
        if not isinstance(entry, dict):
            raise ImportFailure("each export must be an object")
        name = entry.get("id")
        if not isinstance(name, str) or not name or name in identifiers:
            raise ImportFailure("exports require unique nonempty ids")
        identifiers.add(name)
        if entry.get("source") not in source_records:
            raise ImportFailure(f"{name}: unknown source")
        target = relative_path(root, entry.get("output"), f"{name}.output")
        if target in occupied or target.suffix.lower() != ".png":
            raise ImportFailure(f"{name}: duplicate/reserved output or non-PNG path")
        if any(path in target.parents or target in path.parents for path in occupied):
            raise ImportFailure(f"{name}: output conflicts with another file's parent path")
        occupied.add(target)
        if target.exists() and not target.is_file():
            raise ImportFailure(f"{name}: output is not a regular file")
        crop = entry.get("crop")
        if (not isinstance(crop, list) or len(crop) != 4
                or any(type(v) is not int for v in crop)):
            raise ImportFailure(f"{name}: crop must contain four integers [x,y,w,h]")
        x, y, width, height = crop
        source_width, source_height = source_records[entry["source"]]["size"]
        if (min(x, y) < 0 or min(width, height) <= 0
                or x + width > source_width or y + height > source_height):
            raise ImportFailure(f"{name}: crop is outside source bounds")
        landmarks = entry.get("landmarks", {})
        if not isinstance(landmarks, dict):
            raise ImportFailure(f"{name}: landmarks must map names to source [x,y]")
        for landmark, point in landmarks.items():
            if (not isinstance(point, list) or len(point) != 2
                    or any(type(v) not in (int, float) or not math.isfinite(v) for v in point)
                    or not (x <= point[0] <= x + width and y <= point[1] <= y + height)):
                raise ImportFailure(f"{name}.{landmark}: landmark is outside its crop or nonfinite")
    return recipe, source_records, metadata_path


def import_recipe(recipe_path, check=False):
    recipe_path = recipe_path.resolve()
    root = recipe_path.parent
    images = ImageMagick()
    recipe, sources, metadata_path = load_recipe(recipe_path, images)
    manifest = {"version": 1, "algorithm": "explicit-crop-magenta-unmix-v1",
                "imagemagick": images.version, "recipe_sha256": digest(recipe_path),
                "keying": recipe.get("keying", "none"), "sources": sources, "exports": []}
    with tempfile.TemporaryDirectory(prefix="adventure-import-") as temporary:
        work = Path(temporary)
        staged = []
        for index, entry in enumerate(recipe["exports"]):
            output = work / f"{index:04}.png"
            x, y, width, height = entry["crop"]
            source = root / sources[entry["source"]]["path"]
            images.export(source, entry["crop"], manifest["keying"], output, work)
            alpha = images.alpha_summary(output, manifest["keying"], [width, height])
            landmarks = entry.get("landmarks", {})
            manifest["exports"].append({
                "id": entry["id"], "source": entry["source"], "output": entry["output"],
                "crop": entry["crop"], "size": [width, height], "sha256": digest(output),
                "landmarks_source": landmarks,
                "landmarks_local": {name: [point[0] - x, point[1] - y]
                                    for name, point in landmarks.items()},
                "alpha": alpha,
            })
            staged.append((output, relative_path(root, entry["output"], "output")))
        # Detect a retake saved while ImageMagick was exporting. No partial
        # result is promoted if any validation or generation failed.
        if digest(recipe_path) != manifest["recipe_sha256"]:
            raise ImportFailure("recipe changed during import")
        for entry in sources.values():
            if digest(root / entry["path"]) != entry["sha256"]:
                raise ImportFailure("source changed during import")
        manifest_bytes = (json.dumps(manifest, ensure_ascii=False, indent=2) + "\n").encode()
        if check:
            for generated, installed in staged:
                if not installed.is_file() or installed.read_bytes() != generated.read_bytes():
                    raise ImportFailure(f"export differs or is missing: {installed.relative_to(root)}")
            if not metadata_path.is_file() or metadata_path.read_bytes() != manifest_bytes:
                raise ImportFailure("import manifest differs or is missing; re-import intentionally")
        else:
            sealed = work / "manifest.json"
            sealed.write_bytes(manifest_bytes)
            staged.append((sealed, metadata_path))
            for generated, installed in staged:
                installed.parent.mkdir(parents=True, exist_ok=True)
                # Stage beside the destination so promotion works across
                # filesystems, retaining exact exported bytes.
                with tempfile.NamedTemporaryFile(dir=installed.parent, delete=False) as candidate:
                    candidate.write(generated.read_bytes())
                    candidate_path = Path(candidate.name)
                try:
                    os.replace(candidate_path, installed)
                finally:
                    candidate_path.unlink(missing_ok=True)
    return manifest


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--recipe", type=Path, required=True)
    parser.add_argument("--check", action="store_true", help="compare regenerated exports without writing")
    args = parser.parse_args()
    try:
        manifest = import_recipe(args.recipe, args.check)
    except (ImportFailure, OSError, TypeError) as error:
        parser.exit(1, f"import error: {error}\n")
    print(f"{'Verified' if args.check else 'Imported'} {len(manifest['exports'])} exports: {args.recipe}")


if __name__ == "__main__":
    main()
