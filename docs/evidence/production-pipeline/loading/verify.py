#!/usr/bin/env python3
"""Load/render Rust's chapter with every prologue-exclusive image absent.

Requires Linux strace, a native DISPLAY and cached Rust dependencies. It creates
only temporary asset symlinks, never removes or edits the game's own assets.
"""
import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import tempfile

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[3]
EXCLUDED = {"ada-prologue.png", "rust-morning.png", "rust-morning-poses.json", "prologue-environments.png"}


def exclusive(relative):
    return relative.parts[0] == "opening" or relative.name in EXCLUDED


def images(directory):
    return {p.name: hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(directory.glob("*.png"))}


def opened(trace):
    paths = []
    for line in trace.read_text().splitlines():
        if "O_RDONLY" not in line:
            continue
        match = re.search(r'openat\([^,]+, "([^"]+)"', line)
        if match and match[1].endswith(".png") and not line.endswith("= -1 ENOENT (No such file or directory)"):
            paths.append(match[1].split("/assets/")[-1])
    return paths


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    output = args.output.resolve()
    output.mkdir(parents=True, exist_ok=True)
    if any(output.iterdir()):
        parser.error("output directory must be empty")
    if not shutil.which("strace"):
        parser.error("strace is required for the native file-access audit")
    with tempfile.TemporaryDirectory(prefix="borrow-loading-check-") as temporary:
        temp = Path(temporary)
        package = temp / "harness"
        (package / "src").mkdir(parents=True)
        shutil.copy2(HERE / "capture.rs", package / "src/main.rs")
        (package / "Cargo.toml").write_text(
            '[package]\nname="borrow-loading-scope"\nversion="0.1.0"\nedition="2024"\n'
            '[dependencies]\nborrow-fighters={path=' + json.dumps(str(ROOT)) +
            ',default-features=false,features=["adventure"]}\nraylib="6.0.0"\n')
        env = dict(os.environ, CARGO_TARGET_DIR=str(ROOT / "target"))
        subprocess.run(["cargo", "build", "--offline", "--manifest-path", str(package / "Cargo.toml")], cwd=ROOT, env=env, check=True)
        binary = temp / "loading-review"
        shutil.copy2(ROOT / "target/debug/borrow-loading-scope", binary)
        fixture = temp / "fixture/assets"
        omitted = []
        for path in (ROOT / "assets/adventure").rglob("*"):
            if not path.is_file():
                continue
            relative = path.relative_to(ROOT / "assets/adventure")
            if exclusive(relative):
                omitted.append(str(relative))
                continue
            target = fixture / "adventure" / relative
            target.parent.mkdir(parents=True, exist_ok=True)
            target.symlink_to(path)
        results = {}
        for mode in ["chapter", "chapter-without-prologue", "prologue"]:
            run_env = dict(env, BORROW_FIGHTERS_ASSET_DIR=str(fixture if mode == "chapter-without-prologue" else ROOT / "assets"))
            trace = output / (mode + ".trace")
            command = ["strace", "-e", "trace=openat", "-o", str(trace), str(binary), str(output / mode)]
            if mode == "prologue":
                command.append("prologue")
            with (output / (mode + ".log")).open("w") as log:
                subprocess.run(command, cwd=ROOT, env=run_env, stdout=log, stderr=subprocess.STDOUT, check=True)
            paths = opened(trace)
            results[mode] = {"png_open_count": len(paths), "png_unique_count": len(set(paths)), "png_paths": sorted(set(paths)), "screenshots": images(output / mode)}
        assert results["chapter"]["screenshots"] == results["chapter-without-prologue"]["screenshots"], "missing prologue assets changed chapter rendering"
        assert len(results["chapter"]["screenshots"]) == 4
        assert len(results["prologue"]["screenshots"]) == 5
        for path in results["chapter"]["png_paths"]:
            assert not exclusive(Path(path).relative_to("adventure")), "chapter loaded a prologue-only texture: " + path
        report = {"chapter_render_identical_without_prologue": True, "omitted_files": sorted(omitted), "contexts": results}
        (output / "scope-check.json").write_text(json.dumps(report, indent=2) + "\n")
        print(json.dumps({"chapter_render_identical_without_prologue": True, "omitted_file_count": len(omitted), "chapter_png_open_count": results["chapter"]["png_open_count"], "chapter_png_unique_count": results["chapter"]["png_unique_count"]}, indent=2))


if __name__ == "__main__":
    main()
