#!/usr/bin/env python3
"""Capture native actor evidence, audit source drift and reopen the exported pack.

Requires a native DISPLAY and FFmpeg/FFprobe. Does not overwrite an existing review.
Run from any directory: python3 docs/evidence/production-pipeline/lab/verify.py
"""

from __future__ import annotations

import argparse
import datetime
import hashlib
import json
from pathlib import Path
import subprocess


HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[3]


def digest(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output", type=Path, default=HERE / "cpp-review")
    parser.add_argument("--actor", type=Path, default=ROOT / "assets/adventure/actors/cpp/character.json")
    parser.add_argument("--binary", type=Path, default=ROOT / "target/debug/borrow-actor-lab")
    parser.add_argument("--skip-build", action="store_true")
    args = parser.parse_args()
    out, actor, binary = args.output.resolve(), args.actor.resolve(), args.binary.resolve()
    if out.exists() and (not out.is_dir() or any(out.iterdir())):
        parser.error(f"review output is not an empty directory: {out}")
    out.parent.mkdir(parents=True, exist_ok=True)
    if not args.skip_build:
        subprocess.run(["cargo", "build", "--no-default-features", "--features", "adventure", "--bin", "borrow-actor-lab"], cwd=ROOT, check=True)

    paths: set[Path] = set()
    for folder in [ROOT / "src/adventure/production", ROOT / "src/adventure/lab_app", actor.parent]:
        paths.update(path for path in folder.rglob("*") if path.is_file())
    paths.update(ROOT / path for path in [
        "Cargo.toml", "src/bin/borrow-actor-lab.rs",
        "src/adventure/engine/production/actors.rs", "src/adventure/engine/production/assets.rs",
        "assets/adventure/production-lab.json", "assets/adventure/fonts/Barlow-Regular.ttf",
    ])
    paths.add(Path(__file__).resolve())

    def hashes() -> dict[str, str]:
        result = {}
        for path in sorted(paths):
            try:
                label = str(path.relative_to(ROOT))
            except ValueError:
                label = str(path)
            result[label] = digest(path)
        return result

    before = hashes()
    command = [str(binary), "--actor", str(actor), "--hidden", "--review", str(out)]
    capture_log = out.parent / f"{out.name}-capture.log"
    started = datetime.datetime.now(datetime.timezone.utc).isoformat()
    with capture_log.open("w") as log:
        capture = subprocess.run(command, cwd=ROOT, stdout=log, stderr=subprocess.STDOUT)
    after = hashes()
    report: dict = {
        "schema_version": 1, "started_utc": started, "command": command,
        "capture_exit": capture.returncode, "source_sha256_before": before,
        "source_sha256_after": after,
        "changed_during_capture": [path for path in before if before[path] != after[path]],
    }
    if capture.returncode == 0:
        video = out / "lab-simulation-60fps.mp4"
        probe = subprocess.run([
            "ffprobe", "-v", "error", "-show_entries", "stream=width,height,avg_frame_rate,nb_frames",
            "-show_entries", "format=duration,size", "-of", "json", str(video),
        ], capture_output=True, text=True)
        report["probe_exit"] = probe.returncode
        report["ffprobe"] = json.loads(probe.stdout) if probe.returncode == 0 else probe.stderr
        decode = subprocess.run(["ffmpeg", "-v", "error", "-i", str(video), "-f", "null", "-"], capture_output=True, text=True)
        report["decode_exit"], report["decode_stderr"] = decode.returncode, decode.stderr
        exported = out / "source/player/character.json"
        validate = subprocess.run([str(binary), "--actor", str(exported), "--validate"], cwd="/tmp", capture_output=True, text=True)
        report["portable_validate_exit"] = validate.returncode
        report["portable_validate"] = json.loads(validate.stdout) if validate.returncode == 0 else validate.stderr
        with (out.parent / f"{out.name}-portable-native.log").open("w") as log:
            reopened = subprocess.run([
                str(binary), "--actor", str(exported), "--clip", "run", "--phase", "0.625", "--hidden", "--frames", "2",
            ], cwd="/tmp", stdout=log, stderr=subprocess.STDOUT)
        report["portable_native_exit"] = reopened.returncode
        trace = [json.loads(line) for line in (out / "telemetry.jsonl").read_text().splitlines()]
        report["frames"] = len(trace)
        report["sequential_ticks"] = all(row["tick"] == index + 1 for index, row in enumerate(trace))
        report["video_contract_passed"] = probe.returncode == 0 and all([
            report["ffprobe"]["streams"][0]["width"] == 1280,
            report["ffprobe"]["streams"][0]["height"] == 720,
            report["ffprobe"]["streams"][0]["avg_frame_rate"] == "60/1",
            int(report["ffprobe"]["streams"][0]["nb_frames"]) == len(trace),
            abs(float(report["ffprobe"]["format"]["duration"]) - len(trace) / 60) < 0.001,
        ])
        report["result"] = json.loads((out / "result.json").read_text())
        report["contact_moves"] = sorted({
            event.split('move_id: "', 1)[1].split('"', 1)[0]
            for row in trace for event in row["events"] if event.startswith("Hit ")
        })
        report["artifacts_sha256"] = {str(path.relative_to(out)): digest(path) for path in sorted(out.rglob("*")) if path.is_file()}
    audit = out.parent / f"{out.name}-audit.json"
    audit.write_text(json.dumps(report, ensure_ascii=False, indent=2) + "\n")
    omitted = {"source_sha256_before", "source_sha256_after", "artifacts_sha256", "portable_validate"}
    print(json.dumps({key: value for key, value in report.items() if key not in omitted}, ensure_ascii=False, indent=2))
    return int(bool(report["changed_during_capture"]) or any(report.get(key, 1) != 0 for key in [
        "capture_exit", "probe_exit", "decode_exit", "portable_validate_exit", "portable_native_exit",
    ]) or not report.get("sequential_ticks", False) or not report.get("video_contract_passed", False))


if __name__ == "__main__":
    raise SystemExit(main())
