#!/usr/bin/env python3
"""Capture 900 actual-renderer crowd frames, five motions and both facings.

Uses the repository Rust build and local FFmpeg. The gallery alters only the
display positions/wardrobe/facing of continuously sampled decorative people;
it never edits assets, profiles, combat state, source PNGs or frame pixels.
"""

import argparse
import hashlib
import json
from pathlib import Path
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[2]


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("output", type=Path)
    args = parser.parse_args()
    output = args.output.resolve()
    if output.exists() and any(output.iterdir()):
        parser.error("output must be empty to avoid mixing revisions")
    output.mkdir(parents=True, exist_ok=True)
    chapter = ROOT / "assets/adventure/chapters/cpp-augusta"
    sources = [
        chapter / "nightlife-cast.json", chapter / "world-art.json",
        chapter / "sprites/nightlife-cast-a.png", chapter / "sprites/nightlife-cast-b.png",
        chapter / "sprites/nightlife-profile-a.png", chapter / "sprites/nightlife-profile-b.png",
        ROOT / "src/adventure/engine/production/painted_crowd.rs",
        ROOT / "src/adventure/engine/production/nightlife.rs",
        ROOT / "src/adventure/engine/production/assets.rs",
        ROOT / "src/adventure/augusta/ambient.rs",
        Path(__file__).resolve(), Path(__file__).resolve().with_name("review_augusta_crowd.rs"),
    ]
    hashes = {str(path.relative_to(ROOT)): digest(path) for path in sources}
    build = subprocess.run(
        ["cargo", "build", "--lib", "--message-format=json"],
        cwd=ROOT, check=True, text=True, stdout=subprocess.PIPE,
    )
    libraries = {}
    for line in build.stdout.splitlines():
        message = json.loads(line)
        if message.get("reason") != "compiler-artifact":
            continue
        for name in message["filenames"]:
            if name.endswith(".rlib"):
                libraries[message["target"]["name"]] = Path(name)
    required = ("borrow_fighters", "raylib", "serde_json")
    for name in required:
        if name not in libraries:
            raise RuntimeError(f"Cargo did not report {name} rlib")
    with tempfile.TemporaryDirectory(prefix="augusta-crowd-harness-") as temporary:
        executable = Path(temporary) / "crowd-review"
        command = [
            "rustc", "--edition", "2024", str(Path(__file__).with_name("review_augusta_crowd.rs")),
            "-L", f"dependency={libraries['raylib'].parent}",
        ]
        for name in required:
            command.extend(["--extern", f"{name}={libraries[name]}"])
        command.extend(["-o", str(executable)])
        subprocess.run(command, cwd=ROOT, check=True)
        subprocess.run([str(executable), str(output)], cwd=ROOT, check=True)
    frames = json.loads((output / "frames.json").read_text())
    assert len(frames) == 900
    lossless = output / "crowd-lossless.mkv"
    subprocess.run([
        "ffmpeg", "-y", "-loglevel", "error", "-i", str(lossless),
        "-map", "0:v:0", "-compression_level", "3", "-start_number", "0",
        str(output / "frame%04d.png"), "-map", "0:v:0", "-c:v", "libx264",
        "-preset", "veryfast", "-crf", "19", "-pix_fmt", "yuv420p",
        str(output / "crowd-motion.mp4"),
    ], check=True)
    assert len(list(output.glob("frame[0-9][0-9][0-9][0-9].png"))) == 900
    lossless.unlink()
    if hashes != {str(path.relative_to(ROOT)): digest(path) for path in sources}:
        raise RuntimeError("source changed during capture; repeat with a stable revision")
    (output / "review.json").write_text(json.dumps({
        "frames": 900, "fps": 30, "seconds": 30,
        "motions": list(dict.fromkeys(frame["motion"] for frame in frames)),
        "wardrobes": 8, "facings": [1, -1],
        "sources_sha256": hashes,
        "scope": "Native articulation gallery; visual inspection remains required. World positions are replaced by fixed gallery slots, with uninterrupted story poses.",
    }, indent=2) + "\n")
    print(f"Captured 900 frames and crowd-motion.mp4 in {output}")


if __name__ == "__main__":
    main()
