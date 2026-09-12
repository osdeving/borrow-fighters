#!/usr/bin/env python3
"""Compare the 3D actor revision with Augusta's preserved scene and story.

Scenery, camera direction and trajectories must remain byte-identical to the
painted baseline. Optional capture comparison checks every original telemetry
field at the same frame, independently of the replacement actor renderer.
Visual identity, anatomy and shading still require inspection of actual frames.
"""

import argparse
import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
BASELINE = ROOT / "docs/evidence/augusta-3d-pilot/baseline.json"
REFERENCE = ROOT / "docs/evidence/augusta-cinematic-revision/film/telemetry.jsonl"


def check(telemetry=None, prefix=False):
    baseline = json.loads(BASELINE.read_text())
    differences = []
    for name, expected in baseline["sha256"].items():
        path = ROOT / name
        if not path.is_file() or hashlib.sha256(path.read_bytes()).hexdigest() != expected:
            differences.append(name)
    if differences:
        raise ValueError("Fixed scene changed: " + ", ".join(differences))
    report = {"success": True, "fixed_files": len(baseline["sha256"]),
              "base_commit": baseline["base_commit"], "matched_frames": 0,
              "scope": "Scenery and authored state continuity; visual review remains separate."}
    if telemetry is not None:
        expected = [json.loads(line) for line in REFERENCE.read_text().splitlines()]
        actual = [json.loads(line) for line in telemetry.read_text().splitlines()]
        if not actual or len(actual) > len(expected) or (not prefix and len(actual) != len(expected)):
            raise ValueError(f"Capture length differs: {len(actual)} vs {len(expected)} reference frames")
        for index, (before, after) in enumerate(zip(expected, actual)):
            for key, value in before.items():
                if key not in after or after[key] != value:
                    raise ValueError(f"Frame {index}, {key}: {value!r} -> {after.get(key)!r}")
        report.update(matched_frames=len(actual), full_capture=len(actual) == len(expected))
    return report


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--telemetry", type=Path)
    parser.add_argument("--prefix", action="store_true", help="Allow an explicitly partial pilot capture")
    parser.add_argument("--output", type=Path)
    args = parser.parse_args()
    report = json.dumps(check(args.telemetry, args.prefix), indent=2) + "\n"
    if args.output:
        args.output.write_text(report)
    print(report, end="")


if __name__ == "__main__":
    main()
