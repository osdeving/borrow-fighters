#!/usr/bin/env python3
"""Compile the standalone native gait review against this checkout's library."""
import json
from pathlib import Path
import subprocess
import sys
import tempfile

root = Path(__file__).resolve().parents[4]
subprocess.run(
    ["cargo", "build", "--lib", "--no-default-features", "--features", "adventure"],
    cwd=root, check=True,
)
libraries = []
for manifest in (root / "target/debug/.fingerprint").glob(
    "borrow-fighters-*/lib-borrow_fighters.json"
):
    if json.loads(manifest.read_text())["features"] != '["adventure"]':
        continue
    suffix = manifest.parent.name.rsplit("-", 1)[1]
    library = root / f"target/debug/deps/libborrow_fighters-{suffix}.rlib"
    if library.exists():
        libraries.append(library)
library = max(libraries, key=lambda path: path.stat().st_mtime)
raylibs = sorted(
    (root / "target/debug/deps").glob("libraylib-*.rlib"),
    key=lambda path: path.stat().st_mtime, reverse=True,
)
with tempfile.TemporaryDirectory(prefix="borrow-run-review-") as temporary:
    binary = Path(temporary) / "native"
    for raylib in raylibs:
        result = subprocess.run(
            [
                "rustc", "--edition=2024", str(Path(__file__).with_name("native.rs")),
                "-L", "dependency=target/debug/deps",
                "--extern", f"borrow_fighters={library}",
                "--extern", f"raylib={raylib}", "-o", str(binary),
            ],
            cwd=root, capture_output=True, text=True,
        )
        if result.returncode == 0:
            break
    else:
        raise RuntimeError(result.stderr if raylibs else "No compiled Raylib library")
    subprocess.run([str(binary), *sys.argv[1:]], cwd=root, check=True)
