#!/usr/bin/env python3
"""Build and run the native street capture with Cargo's exact dependency artifacts."""
import json
from pathlib import Path
import subprocess
import tempfile

root = Path(__file__).resolve().parents[4]
build = subprocess.run(
    ["cargo", "build", "--lib", "--no-default-features", "--features", "adventure",
     "--message-format=json"],
    cwd=root, check=True, stdout=subprocess.PIPE, text=True,
)
libraries = {}
for line in build.stdout.splitlines():
    row = json.loads(line)
    name = row.get("target", {}).get("name")
    if name in ("borrow_fighters", "raylib"):
        for filename in row.get("filenames", []):
            if filename.endswith(".rlib"):
                libraries[name] = filename
assert set(libraries) == {"borrow_fighters", "raylib"}, libraries
with tempfile.TemporaryDirectory(prefix="borrow-world-review-") as temporary:
    binary = Path(temporary) / "capture"
    command = ["rustc", "--edition=2024", str(Path(__file__).with_name("capture.rs")),
               "-L", "dependency=target/debug/deps", "-o", str(binary)]
    for name, path in libraries.items():
        command += ["--extern", f"{name}={path}"]
    subprocess.run(command, cwd=root, check=True)
    subprocess.run([str(binary)], cwd=root, check=True)
