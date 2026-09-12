#!/usr/bin/env python3
"""Native Rust run review; does not change runtime or art.

Run from any directory:
  python3 docs/evidence/run-cycle-rebuild/capture.py --output /tmp/rust-run-review
  python3 docs/evidence/run-cycle-rebuild/capture.py --output docs/evidence/run-cycle-rebuild/after-ankle

The output directory must be empty, so the baseline cannot be overwritten.
Default: 16 normalized phases, both facings, native scale and 2x, idle comparison,
reported phases 0/.125/.375, and a 12-second 60fps start/stop/reverse movie.
Two 1280x720 context frames use the real prologue/chapter renderer and camera.
Dedicated 2x ankle sheets sample .45/.5/.55/.5742512/.6/.625/.65/.7 in both
facings, including the pose reported in after/transition-359.png. Preserve
before/ and after/; each subsequent review uses its own empty output directory.
Static run poses use nominal PLAYER_RUN_SPEED with the selected facing. The
movie retains authoritative velocity, including acceleration, braking and turns.
Use --no-video for fast contact sheets, or --phases 12 for a smaller survey.
No implementation-specific rig API is used. Same harness can review an atlas,
interpolation, or another technique as long as actors::rust_scaled remains public.

Reject visually: squashed thighs/torso, changing boot shape or volume, broken
ankle/knee/hip transitions, inconsistent head/body/limb proportions versus idle,
sliding contact feet, clipping/pop at the wrap, and start/stop/reverse snaps.
A bent limb must keep plausible volume; its pixel height alone is not a failure.
Visual acceptance is manual; successful capture is not approval of the animation.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import tempfile
import time

ROOT = Path(__file__).resolve().parents[3]
HERE = Path(__file__).resolve().parent
CRITERIA = [
    "No torso/thigh/shin squashing or sudden volume loss, especially phases 0, 0.125 and 0.375.",
    "Boot toe, sole, heel and shaft keep their recognizable shape and relative size.",
    "Knee, ankle, hip and shoulder connections look anatomical, without gaps or pasted fragments.",
    "During recovery the boot follows the shin continuously, without a reversed or abruptly bent ankle; inspect phase .5742512 and its neighbours in both facings.",
    "Head/body/limb proportions agree with the idle reference at 1x and 2x.",
    "Grounded foot contacts resist sliding; flight and recovery read as a run, not a shuffle.",
    "Both facings preserve anatomy; loop wrap and start/stop/direct reverse avoid visible popping.",
    "Judge silhouette and volume, not the bounding-box height of a legitimately bent leg.",
    "In the real street and chapter, Rust keeps readable silhouette, floor contact and coherent scale against nearby scenery and the native interface.",
]


def hashes() -> dict[str, str]:
    paths = set((ROOT / "assets/adventure/locomotion").rglob("*.json"))
    paths.update((ROOT / "assets/adventure/locomotion").rglob("*.png"))
    paths.update((ROOT / "src/adventure/locomotion").rglob("*.rs"))
    for directory in ("world", "street", "chapter"):
        for extension in ("*.json", "*.png"):
            paths.update((ROOT / "assets/adventure" / directory).rglob(extension))
    for directory in ("chapter", "engine/chapter"):
        paths.update((ROOT / "src/adventure" / directory).rglob("*.rs"))
    paths.update((ROOT / "assets/adventure/fonts").glob("*.ttf"))
    paths.update(ROOT / name for name in (
        "src/adventure/combat.rs", "src/adventure/engine/actors.rs",
        "src/adventure/engine/locomotion.rs", "src/adventure/locomotion.rs",
        "src/adventure/engine/pieces.rs", "src/adventure/engine/assets.rs",
        "src/adventure/scenery.rs", "src/math/vec2.rs",
        "src/adventure/story.rs", "src/adventure/ambient.rs", "src/adventure/landscape.rs",
        "src/adventure/engine/landscape.rs", "src/adventure/engine/render.rs",
        "src/adventure/engine/street.rs", "src/adventure/engine/typography.rs",
        "assets/adventure/texts/pt-BR.json", "src/adventure/text.rs",
        "assets/adventure/street-life.png", "assets/adventure/street-traffic.png",
        "assets/adventure/rust-actions.png", "assets/adventure/rust-actions-poses.json",
    ))
    return {str(p.relative_to(ROOT)): hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(paths)}


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--phases", type=int, choices=(12, 16), default=16)
    parser.add_argument("--no-video", action="store_true")
    parser.add_argument("--display", default=os.environ.get("DISPLAY", ":0"))
    args = parser.parse_args()
    output = args.output if args.output.is_absolute() else ROOT / args.output
    output = output.resolve()
    if output.exists() and any(output.iterdir()):
        parser.error(f"Refusing to overwrite nonempty evidence directory: {output}")
    output.mkdir(parents=True, exist_ok=True)
    before = hashes()
    report = {
        "created_unix": time.time(), "harness_version": 4, "phase_count": args.phases,
        "phases": [i / args.phases for i in range(args.phases)],
        "reported_phases": [0, 0.125, 0.375], "scales": [1, 2],
        "facings": ["right", "left"], "video": not args.no_video,
        "render_api": "actors::rust_scaled", "actor_source": "Story::default().combat.player",
        "input_api": "Combat::tick_exploration", "criteria": CRITERIA,
        "static_pose_velocity": "PLAYER_RUN_SPEED * facing.sign(); vertical=0; grounded=true",
        "video_pose_velocity": "Unmodified authoritative velocity after tick_exploration",
        "state_event_capture": "Action or facing changes, in addition to fixed input-edge snapshots",
        "world_context_capture": "Two native 1280x720 frames: prologue right at phase .125 and chapter left at phase .375; real scene cameras after ordinary movement, final pose sampled at nominal speed",
        "ankle_recovery_capture": "Both facings at 2x, phases .45/.5/.55/.5742512/.6/.625/.65/.7; reported tick 359 highlighted; exact runtime sampling in capture-settings.json",
        "harness_sha256": {name: hashlib.sha256((HERE / name).read_bytes()).hexdigest() for name in ("capture.py", "capture.rs")},
        "visual_acceptance": "manual review required", "source_sha256_before": before,
        "command": ["python3", str(Path(__file__).resolve().relative_to(ROOT)), "--output", str(args.output), "--phases", str(args.phases)] + (["--no-video"] if args.no_video else []),
    }
    (output / "protocol.json").write_text(json.dumps(report, indent=2) + "\n")
    env = dict(os.environ, DISPLAY=args.display, CARGO_TARGET_DIR=str(ROOT / "target"))
    with tempfile.TemporaryDirectory(prefix="borrow-run-review-") as temporary:
        package = Path(temporary)
        (package / "src").mkdir()
        shutil.copy2(HERE / "capture.rs", package / "src/main.rs")
        (package / "Cargo.toml").write_text(
            '[package]\nname="borrow-run-native-review"\nversion="0.1.0"\nedition="2024"\n'
            '[dependencies]\nborrow-fighters={path=' + json.dumps(str(ROOT)) + ',default-features=false,features=["adventure"]}\nraylib="6.0.0"\n'
        )
        command = ["cargo", "run", "--offline", "--manifest-path", str(package / "Cargo.toml"), "--", str(output), str(args.phases)]
        if args.no_video:
            command.append("--no-video")
        with (output / "capture.log").open("w") as log:
            result = subprocess.run(command, cwd=ROOT, env=env, stdout=log, stderr=subprocess.STDOUT, check=False)
    after = hashes()
    report["source_sha256_after"] = after
    report["changed_during_capture"] = sorted(p for p in before.keys() | after.keys() if before.get(p) != after.get(p))
    report["capture_exit_code"] = result.returncode
    if (output / "capture-settings.json").exists():
        report["sampling"] = json.loads((output / "capture-settings.json").read_text())
    if (output / "world-context.json").exists():
        report["world_context"] = json.loads((output / "world-context.json").read_text())
    report["artifacts"] = sorted(p.name for p in output.iterdir() if p.is_file())
    if result.returncode == 0 and not args.no_video:
        probe = subprocess.run(["ffprobe", "-v", "error", "-show_entries", "stream=width,height,avg_frame_rate,nb_frames", "-show_entries", "format=duration,size", "-of", "json", str(output / "start-stop-reverse-60fps.mp4")], capture_output=True, text=True, check=True)
        report["video_probe"] = json.loads(probe.stdout)
    (output / "protocol.json").write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps({k: report.get(k) for k in ("capture_exit_code", "changed_during_capture", "artifacts", "video_probe")}, indent=2))
    return result.returncode or int(bool(report["changed_during_capture"]))


if __name__ == "__main__":
    raise SystemExit(main())
