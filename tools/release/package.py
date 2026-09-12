#!/usr/bin/env python3
"""Stage runtime assets and build self-contained desktop release packages.

Only the Python standard library is needed for staging. Native packaging uses
ISCC on Windows, or ldd/readelf/apt-get/dpkg-deb/rpmbuild on Ubuntu 22.04.
"""

import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import tarfile
import tempfile
import zipfile


ROOT = Path(__file__).resolve().parents[2]
CHARACTERS = ("rust", "duke", "go", "c", "python", "cpp")
SOURCE_BINARY = "borrow-story"
LAB_BINARY = "borrow-actor-lab"
TARGETS = {
    "windows-x86_64": "x86_64-pc-windows-msvc",
    "linux-x86_64": "x86_64-unknown-linux-gnu",
}
# GLFW and miniaudio load these dynamically: ldd on the game alone misses them.
DLOPEN_LIBRARIES = (
    "libX11.so.6", "libX11-xcb.so.1", "libXcursor.so.1", "libXi.so.6",
    "libXinerama.so.1", "libXrandr.so.2", "libXrender.so.1", "libXext.so.6",
    "libXxf86vm.so.1", "libasound.so.2", "libpulse.so.0",
)
# The loader, glibc and graphics drivers must match the host. Never copy them.
SYSTEM_LIBRARY = re.compile(
    r"^(?:linux-vdso|ld-linux|lib(?:c|m|pthread|dl|rt|resolv|util|anl)\.so"
    r"|libnsl\.so\.1(?:$|\.)"
    r"|lib(?:GL|EGL|GLX|GLdispatch|OpenGL|GLES|vulkan|drm|gbm|nvidia))"
)


def run(*command, cwd=ROOT, env=None):
    return subprocess.check_output(command, cwd=cwd, env=env, text=True,
                                   encoding="utf-8", errors="replace").strip()


def copy(source, destination):
    destination.parent.mkdir(parents=True, exist_ok=True)
    shutil.copy2(source, destination)


def within(path, directory):
    return path == directory or directory in path.parents


def sha256(file):
    digest = hashlib.sha256()
    with file.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def asset_file(path):
    path = path.resolve()
    if not within(path, ROOT / "assets") or not path.is_file():
        raise ValueError(f"Missing runtime asset or path outside assets/: {path}")
    return path


def string_values(value):
    if isinstance(value, str):
        yield value
    elif isinstance(value, dict):
        for child in value.values():
            yield from string_values(child)
    elif isinstance(value, list):
        for child in value:
            yield from string_values(child)


def adventure_piece_assets(catalog):
    """Follow every frame image in one adventure catalog, deduplicating atlases."""
    base = ROOT / "assets/adventure"
    catalog = asset_file(catalog)
    files = {catalog}
    pieces = json.loads(catalog.read_text(encoding="utf-8"))["pieces"]
    for piece in pieces.values():
        for frame in piece["frames"]:
            # Catalogs may reuse another adventure's atlas. Only image fields
            # are dependencies; source rectangles and production notes are not.
            name = frame["image"]
            if (not isinstance(name, str) or not name.endswith(".png")
                    or "\\" in name or ":" in name
                    or any(part in ("", ".", "..") for part in name.split("/"))):
                raise ValueError(f"Adventure piece must use a relative local PNG: {name}")
            image = asset_file(base / name)
            if not within(image, base):
                raise ValueError(f"Adventure piece outside assets/adventure/: {name}")
            files.add(image)
    return files


def adventure_assets():
    """Expand adventure loaders, opening portraits and declared piece catalogs."""
    files = set()
    base = ROOT / "assets/adventure"
    # These adapters prepend their own directory to literal file names. Keep
    # that mapping explicit; a recursive copy would ship unused art and reviews.
    for module, directory in (("assets.rs", base),
                              ("opening.rs", base / "opening"),
                              ("audio.rs", base / "audio")):
        source = ROOT / "src/adventure/engine" / module
        names = re.findall(r'"([^"/{}\n]+\.(?:png|json|ogg|wav|ttf))"',
                           source.read_text(encoding="utf-8"))
        files.update(asset_file(directory / name) for name in names)
    roster = asset_file(base / "opening/roster.json")
    files.add(roster)
    data = json.loads(roster.read_text(encoding="utf-8"))
    for character in data["characters"]:
        # Mirror the runtime's local-image rule. Provenance `source` points at
        # production artwork and is deliberately not a runtime dependency.
        name = character["image"]
        if (not name or "\\" in name or ":" in name
                or any(part in ("", ".", "..") for part in name.split("/"))):
            raise ValueError(f"Opening portrait must use a relative local image: {name}")
        portrait = asset_file(roster.parent / name)
        if not within(portrait, roster.parent):
            raise ValueError(f"Opening portrait outside adventure opening/: {name}")
        files.add(portrait)
    files.add(asset_file(base / "street/scene.json"))
    catalogs = {base / "street/catalog.json", base / "chapter/catalog.json"}
    # Each declared catalog owns its transitive PNG dependencies. New scene or
    # animation catalogs therefore ship without copying unused production art.
    for source in (ROOT / "src/adventure").rglob("*.rs"):
        for name in re.findall(r'"(assets/adventure/[^"\n{}]+/catalog\.json)"',
                               source.read_text(encoding="utf-8")):
            catalogs.add(ROOT / name)
    for catalog in catalogs:
        if catalog == base / "audio/production/catalog.json":
            # This catalog contains sound samples; the production closure below
            # follows its file fields rather than treating it as a sprite atlas.
            continue
        files.update(adventure_piece_assets(catalog))
    for name in ("fonts/BARLOW-OFL.txt", "fonts/LORA-OFL.txt", "fonts/README.md",
                 "audio/README.md", "texts/README.md", "ART-PROVENANCE.md",
                 "opening/ART-PROVENANCE.md", "street/README.md",
                 "chapter/README.md", "chapter/DRIVER.md", "chapter/audio/README.md"):
        files.add(asset_file(base / name))
    return files


def production_file(directory, name, suffix):
    """Mirror production package containment without collecting authoring data."""
    if (not isinstance(name, str) or not name.endswith(suffix)
            or "\\" in name or ":" in name
            or any(part in ("", ".", "..") for part in name.split("/"))):
        raise ValueError(f"Production resource must use a relative local {suffix}: {name}")
    if {"source", "sources", "review", "reviews", "prompts"} & set(name.split("/")):
        raise ValueError(f"Production runtime reference points to authoring material: {name}")
    file = asset_file(directory / name)
    if not within(file, directory.resolve()):
        raise ValueError(f"Production resource outside its package: {name}")
    return file


def production_assets():
    """Follow campaign/chapter/actor/audio fields, never whole asset directories.

    Rust retains its legacy loader. Independent chapter ids name their package
    directory; the campaign's typed routes remain the runtime authority. The
    external lab defaults to C++, while custom --actor paths are user content.
    """
    base = ROOT / "assets/adventure"
    files = set()

    def document(path):
        files.add(path)
        try:
            data = json.loads(path.read_text(encoding="utf-8"))
        except (OSError, json.JSONDecodeError) as error:
            raise ValueError(f"Invalid production JSON {path}: {error}") from error
        if not isinstance(data, dict):
            raise ValueError(f"Production JSON must be an object: {path}")
        return data

    def attachments(directory, entries):
        for attachment in entries.values():
            files.add(production_file(directory, attachment["image"], ".png"))

    def actor(path):
        if path in files:
            return
        spec = document(path)
        directory = path.parent
        document(production_file(directory, spec["combat"], ".json"))
        document(production_file(directory, spec["clips"], ".json"))
        rig = document(production_file(directory, spec["rig"], ".json"))
        attachments(directory, rig["attachments"])

    try:
        registry = document(production_file(base, "campaign.json", ".json"))
        document(production_file(base, "production-lab.json", ".json"))
        actor(production_file(base, "actors/cpp/character.json", ".json"))
        for entry in registry["chapters"]:
            identifier = entry["id"]
            if identifier == "rust":
                continue
            if not isinstance(identifier, str) or not re.fullmatch(r"[a-z0-9]+(?:-[a-z0-9]+)*", identifier):
                raise ValueError(f"Invalid production chapter id: {identifier}")
            path = production_file(base, f"chapters/{identifier}/chapter.json", ".json")
            spec = document(path)
            directory = path.parent
            document(production_file(directory, spec["world"], ".json"))
            document(production_file(directory, spec["texts"], ".json"))
            art = document(production_file(directory, spec["art"], ".json"))
            attachments(directory, art["pieces"])
            for name in art["actors"].values():
                actor(production_file(base, name, ".json"))
        audio_path = production_file(base, "audio/production/catalog.json", ".json")
        audio = document(audio_path)
        loops = [audio[key] for key in ("ambience", "air") if key in audio]
        for sample in [*loops, *audio["effects"].values()]:
            files.add(production_file(audio_path.parent, sample["file"], ".wav"))
    except (KeyError, TypeError, AttributeError) as error:
        raise ValueError(f"Malformed production dependency descriptor: {error}") from error
    return files


def runtime_assets():
    """Follow runtime fields, excluding provenance/source/review directories."""
    files = set()
    # Concrete paths in Rust cover arenas, super actors, lore, audio and metrics.
    # Formatted candidate paths are expanded explicitly below.
    for source in (ROOT / "src").rglob("*.rs"):
        for name in re.findall(r'"(assets/[^"\n]+\.(?:png|json|ogg|wav|ttf))"',
                               source.read_text(encoding="utf-8")):
            if "{" not in name:
                files.add(asset_file(ROOT / name))
    for character in CHARACTERS:
        base = ROOT / "assets/candidates" / character
        files.add(asset_file(base / f"{character}-fighter.sprite.json"))
        for name in (f"{character}-signature-fx.sprite.json", f"{character}-projectile.png"):
            if (base / name).is_file():
                files.add(asset_file(base / name))
    for manifest in tuple(files):
        if manifest.name.endswith(".sprite.json"):
            data = json.loads(manifest.read_text(encoding="utf-8"))
            images = [data["image"]] + [frame["image"] for frame in data["frames"]
                                        if "image" in frame]
            files.update(asset_file(manifest.parent / name) for name in images)
        elif manifest.name == "audio_manifest.json":
            data = json.loads(manifest.read_text(encoding="utf-8"))
            files.update(asset_file(ROOT / value) for value in string_values(data)
                         if value.startswith("assets/"))
    files.update(adventure_assets())
    files.update(production_assets())
    # Fonts are embedded in the executable, but their notices must travel with it.
    for name in ("BARLOW-OFL.txt", "LORA-OFL.txt", "README.md"):
        files.add(asset_file(ROOT / "assets/fonts" / name))
    files.add(asset_file(ROOT / "assets/audio/ATTRIBUTION.md"))
    return sorted(files)


def rust_notices(stage, target):
    """Archive exact dependency sources, including licenses embedded in C headers."""
    metadata = json.loads(run("cargo", "metadata", "--locked", "--format-version=1",
                              "--filter-platform", TARGETS[target]))
    resolved = {node["id"] for node in metadata["resolve"]["nodes"]}
    packages = sorted((package for package in metadata["packages"]
                       if package["id"] in resolved and package["source"]),
                      key=lambda package: package["name"])
    archive = stage / "THIRD_PARTY_SOURCES/rust-crates.tar.gz"
    archive.parent.mkdir(parents=True)
    notice = ["# Third-party notices", "",
              "This distribution includes the dependencies below. Exact source copies",
              "and their original notices (including raylib's embedded C libraries)",
              "are in THIRD_PARTY_SOURCES/rust-crates.tar.gz. Standalone license files",
              "are also copied into licenses/rust. Some entries are build dependencies.",
              "", "## Rust dependencies", ""]
    with tarfile.open(archive, "w:gz") as tar:
        for package in packages:
            label = f'{package["name"]}-{package["version"]}'
            directory = Path(package["manifest_path"]).parent
            notice.append(f'- {label}: {package["license"] or "see source notices"}; '
                          f'{package["repository"] or package["source"]}')
            tar.add(directory, arcname=label)
            for file in directory.rglob("*"):
                if file.is_file() and re.match(r"(?:license|licence|copying|notice)(?:[.-]|$)",
                                               file.name, re.IGNORECASE):
                    copy(file, stage / "licenses/rust" / label / file.relative_to(directory))
    notice += ["", "## Fonts and audio", "",
               "- Barlow and Lora: assets/fonts/README.md and both OFL license texts.",
               "- Music, sound effects and voices: assets/audio/ATTRIBUTION.md.",
               "- Adventure fonts: assets/adventure/fonts/README.md and local OFL texts.",
               "- Original adventure music and sounds: assets/adventure/audio/README.md.",
               "- Original chapter Foley and phone effects: assets/adventure/chapter/audio/README.md.",
               "", "## Project and prototype artwork", "",
               "Cargo.toml declares MIT OR Apache-2.0 for the project code. The release",
               "does not assign that declaration to third-party audio, fonts or artwork.",
               "Artwork provenance remains documented in the repository's assets/ tree.",
               "Adventure artwork notices are in assets/adventure/ART-PROVENANCE.md",
               "and assets/adventure/opening/ART-PROVENANCE.md. Chapter provenance is in",
               "assets/adventure/chapter/README.md and DRIVER.md; production references",
               "remain available in the source repository at the recorded revision.",
               "The source tag for this build is recorded in BUILD-INFO.json.", ""]
    (stage / "THIRD_PARTY_NOTICES.md").write_text("\n".join(notice), encoding="utf-8")


def checksums(stage):
    lines = []
    for file in sorted(stage.rglob("*")):
        if file.is_file() and file.name != "PACKAGE-SHA256SUMS.txt":
            digest = sha256(file)
            lines.append(f"{digest}  {file.relative_to(stage).as_posix()}")
    (stage / "PACKAGE-SHA256SUMS.txt").write_text("\n".join(lines) + "\n", encoding="utf-8")


def stage_package(args):
    source_name = SOURCE_BINARY + (".exe" if args.target == "windows-x86_64" else "")
    if args.binary.name != source_name:
        raise ValueError(f"Release requires the composed Cargo binary {source_name}: {args.binary}")
    lab_binary = getattr(args, "lab_binary", None)
    lab_executable = LAB_BINARY + ".exe" if args.target == "windows-x86_64" else "bin/" + LAB_BINARY
    if lab_binary is not None:
        expected_lab = LAB_BINARY + (".exe" if args.target == "windows-x86_64" else "")
        if lab_binary.name != expected_lab:
            raise ValueError(f"Production lab requires Cargo binary {expected_lab}: {lab_binary}")
        lab_binary = lab_binary.resolve(strict=True)
    assets = runtime_assets()
    stage = args.output.resolve()
    if stage.exists() and any(stage.iterdir()):
        raise ValueError(f"Stage destination must be empty: {stage}")
    stage.mkdir(parents=True, exist_ok=True)
    executable = "borrow-fighters.exe" if args.target == "windows-x86_64" else "bin/borrow-fighters"
    copy(args.binary.resolve(strict=True), stage / executable)
    for source in assets:
        copy(source, stage / source.relative_to(ROOT))
    if lab_binary is not None:
        copy(lab_binary, stage / lab_executable)
    copy(ROOT / "packaging/JOGUE-PRIMEIRO.md", stage / "JOGUE-PRIMEIRO.md")
    for license_file in ROOT.glob("LICENSE*"):
        if license_file.is_file():
            copy(license_file, stage / license_file.name)
    if args.target == "linux-x86_64":
        copy(ROOT / "packaging/linux/borrow-fighters", stage / "borrow-fighters")
        (stage / "borrow-fighters").chmod(0o755)
        (stage / executable).chmod(0o755)
        if lab_binary is not None:
            copy(ROOT / "packaging/linux/borrow-actor-lab", stage / LAB_BINARY)
            (stage / LAB_BINARY).chmod(0o755)
            (stage / lab_executable).chmod(0o755)
    rust_notices(stage, args.target)
    (stage / "BUILD-INFO.json").write_text(json.dumps({
        "version": args.version, "target": args.target,
        "cargo_binary": SOURCE_BINARY,
        "tools": ([{"cargo_binary": LAB_BINARY, "path": lab_executable}]
                  if lab_binary is not None else []),
        "commit": run("git", "rev-parse", "HEAD"),
        "rustc": run("rustc", "--version"),
        "asset_count": len(assets),
    }, indent=2) + "\n", encoding="utf-8")
    checksums(stage)
    validate_assets(stage)
    print(f"Staged {args.target} in {stage}")


def validate_assets(stage):
    for source in runtime_assets():
        staged = stage / source.relative_to(ROOT)
        if not staged.is_file() or staged.read_bytes() != source.read_bytes():
            raise ValueError(f"Staged asset missing or modified: {staged}")


def validate_build_info(stage, target, version):
    build = json.loads((stage / "BUILD-INFO.json").read_text(encoding="utf-8"))
    if build["target"] != target or build["version"] != version:
        raise ValueError(f"Staging target/version differs from requested package: {build}")
    if build.get("cargo_binary") != SOURCE_BINARY:
        raise ValueError(f"Release staging must contain the {SOURCE_BINARY} composition: {build}")
    for tool in build.get("tools", []):
        expected = LAB_BINARY + ".exe" if target == "windows-x86_64" else "bin/" + LAB_BINARY
        if tool != {"cargo_binary": LAB_BINARY, "path": expected}:
            raise ValueError(f"Invalid staged production tool: {tool}")
        if not (stage / expected).is_file():
            raise ValueError(f"Missing production lab executable: {expected}")


def ldd_libraries(binary, env=None):
    output = run("ldd", str(binary), env=env)
    if "not found" in output:
        raise ValueError(f"Unresolved libraries for {binary}:\n{output}")
    return {match[0]: Path(match[1]) for match in
            re.findall(r"^\s*(\S+) => (/\S+)", output, re.MULTILINE)}


def native_package(path):
    # dpkg ownership may use the legacy /lib path on merged-/usr systems.
    candidates = [str(path), str(path.resolve())]
    if str(path).startswith("/usr/lib/"):
        candidates.append(str(path)[4:])
    for candidate in candidates:
        result = subprocess.run(["dpkg-query", "-S", candidate], text=True,
                                capture_output=True, check=False)
        if result.returncode == 0:
            return result.stdout.split(": ", 1)[0]
    raise ValueError(f"Cannot identify distribution package for {path}")


def native_data_packages(paths):
    """Identify owners of copied data as well as dereferenced symlink targets."""
    files = sorted({str(file) for path in paths for file in (path, path.resolve())})
    packages = set()
    for offset in range(0, len(files), 100):
        ownership = run("dpkg-query", "-S", *files[offset:offset + 100])
        for line in ownership.splitlines():
            packages.update(line.split(": ", 1)[0].split(", "))
    return packages


def native_package_owners(libraries, data_files):
    packages = {native_package(path) for path in libraries.values()}
    packages.update(native_data_packages(data_files))
    return sorted(packages)


def native_notices(stage, libraries, data_files=()):
    """Carry licenses and exact sources for both native libraries and their data."""
    entries = []
    sources = set()
    for package in native_package_owners(libraries, data_files):
        fields = run("dpkg-query", "-W", "-f=${source:Package}\t${source:Version}\t${Version}", package)
        source, source_version, binary_version = fields.split("\t")
        copyright_path = Path("/usr/share/doc") / package.split(":")[0] / "copyright"
        if not copyright_path.is_file():
            raise ValueError(f"Missing native copyright: {copyright_path}")
        copy(copyright_path, stage / "licenses/native" / f"{package.replace(':', '-')}.copyright")
        # GCC's runtime exception permits this linked binary distribution. Retain
        # the full copyright/exceptions, without copying the huge compiler source.
        if not source.startswith("gcc-"):
            sources.add((source, source_version))
        entries.append({"binary_package": package, "binary_version": binary_version,
                        "source_package": source, "source_version": source_version})
    for name in ("GPL-2", "GPL-3", "LGPL-2", "LGPL-2.1", "LGPL-3"):
        path = Path("/usr/share/common-licenses") / name
        if path.is_file():
            copy(path, stage / "licenses/native" / name)
    source_dir = stage / "THIRD_PARTY_SOURCES/native"
    source_dir.mkdir(parents=True, exist_ok=True)
    for source, version in sorted(sources):
        subprocess.run(["apt-get", "source", "--download-only", f"{source}={version}"],
                       cwd=source_dir, check=True)
    (stage / "NATIVE-LIBRARIES.json").write_text(json.dumps({
        "libraries": sorted(libraries), "packages": entries,
        "data_files": [str(path) for path in data_files],
        "system_requirements": ["glibc >= 2.35", "X11 or XWayland", "OpenGL >= 3.3", "audio server/device"],
    }, indent=2) + "\n", encoding="utf-8")
    with (stage / "THIRD_PARTY_NOTICES.md").open("a", encoding="utf-8") as file:
        file.write("\n## Bundled Linux libraries\n\nExact package/source versions are in NATIVE-LIBRARIES.json.\n"
                   "Copyright and license texts are in licenses/native; corresponding\n"
                   "unmodified source archives and distribution patches are in\n"
                   "THIRD_PARTY_SOURCES/native. GCC runtime carries its runtime exception.\n"
                   "Libraries are dynamically linked and may be replaced in lib/.\n")


def linux_executables(stage):
    executables = [stage / "bin/borrow-fighters"]
    lab = stage / "bin" / LAB_BINARY
    if lab.is_file():
        executables.append(lab)
    return executables


def linux_runtime_files(stage):
    """Resolve host runtime dependencies without copying or changing anything."""
    cache = run("ldconfig", "-p")
    libraries = {}
    for executable in linux_executables(stage):
        libraries.update(ldd_libraries(executable))
    for soname in DLOPEN_LIBRARIES:
        found = re.search(rf"^\s*{re.escape(soname)} \(libc6,x86-64[^)]*\) => (\S+)$",
                          cache, re.MULTILINE)
        if not found:
            raise ValueError(f"Install the Ubuntu runtime package providing {soname}")
        libraries[soname] = Path(found[1])
    pending = list(libraries.values())
    visited = set()
    while pending:
        file = pending.pop()
        if file in visited:
            continue
        visited.add(file)
        for soname, dependency in ldd_libraries(file).items():
            if soname not in libraries:
                libraries[soname] = dependency
                pending.append(dependency)
    libraries = {name: path for name, path in libraries.items() if not SYSTEM_LIBRARY.match(name)}
    alsa_config = Path("/usr/share/alsa")
    if not alsa_config.is_dir():
        raise ValueError("Install libasound2-data for ALSA configuration")
    alsa_files = sorted(path for path in alsa_config.rglob("*") if path.is_file())
    return libraries, alsa_files


def print_native_packages(args):
    """Print only installed package names, suitable for a targeted apt upgrade."""
    stage = args.stage.resolve()
    build = json.loads((stage / "BUILD-INFO.json").read_text(encoding="utf-8"))
    if build["target"] != "linux-x86_64":
        raise ValueError("native-packages requires Linux staging")
    libraries, data_files = linux_runtime_files(stage)
    print("\n".join(native_package_owners(libraries, data_files)))


def bundle_linux(stage):
    libraries, alsa_files = linux_runtime_files(stage)
    for soname, source in libraries.items():
        copy(source, stage / "lib" / soname)
    # miniaudio tries the development name first. Keep that lookup in the bundle
    # even on a host with libpulse-dev installed; its private libpulsecommon is
    # already included by the transitive dependency collection above.
    pulse_alias = stage / "lib/libpulse.so"
    pulse_alias.unlink(missing_ok=True)
    pulse_alias.symlink_to("libpulse.so.0")
    # ALSA's data files are required for libasound configuration lookup.
    shutil.copytree("/usr/share/alsa", stage / "share/alsa", dirs_exist_ok=True)
    native_notices(stage, libraries, alsa_files)
    validate_linux_libraries(stage)


def validate_linux_libraries(stage):
    env = dict(os.environ, LD_LIBRARY_PATH=str(stage / "lib"))
    for file in [*linux_executables(stage), *sorted((stage / "lib").glob("*.so*"))]:
        for soname, resolved in ldd_libraries(file, env).items():
            if not SYSTEM_LIBRARY.match(soname) and not within(resolved, stage / "lib"):
                raise ValueError(f"Unbundled dependency {soname} for {file}: {resolved}")
        versions = re.findall(r"GLIBC_(\d+)\.(\d+)", run("readelf", "--version-info", str(file)))
        if versions and max(tuple(map(int, version)) for version in versions) > (2, 35):
            raise ValueError(f"{file} requires glibc newer than Ubuntu 22.04's 2.35")


def install_tree(stage, directory):
    shutil.copytree(stage, directory / "opt/borrow-fighters")
    command = directory / "usr/bin/borrow-fighters"
    command.parent.mkdir(parents=True)
    command.write_text('#!/bin/sh\nexec /opt/borrow-fighters/borrow-fighters "$@"\n', encoding="utf-8")
    command.chmod(0o755)
    if (stage / LAB_BINARY).is_file():
        lab_command = directory / "usr/bin" / LAB_BINARY
        lab_command.write_text('#!/bin/sh\nexec /opt/borrow-fighters/borrow-actor-lab "$@"\n', encoding="utf-8")
        lab_command.chmod(0o755)
    copy(ROOT / "packaging/linux/borrow-fighters.desktop",
         directory / "usr/share/applications/borrow-fighters.desktop")
    copy(ROOT / "packaging/linux/borrow-fighters.svg",
         directory / "usr/share/icons/hicolor/scalable/apps/borrow-fighters.svg")


def linux_packages(args):
    stage, output = args.stage.resolve(), args.output.resolve()
    validate_build_info(stage, "linux-x86_64", args.version)
    validate_assets(stage)
    bundle_linux(stage)
    checksums(stage)
    output.mkdir(parents=True, exist_ok=True)
    stem = f"borrow-fighters-{args.version}-linux-x86_64"
    with tarfile.open(output / f"{stem}.tar.gz", "w:gz") as tar:
        tar.add(stage, arcname=stem)
    with tempfile.TemporaryDirectory(prefix="borrow-fighters-package-") as temporary:
        temporary = Path(temporary)
        root = temporary / "root"
        install_tree(stage, root)
        debian = root / "DEBIAN"
        debian.mkdir()
        deb_version = args.version.replace("-", "~", 1)
        installed_size = sum(path.stat().st_size for path in root.rglob("*") if path.is_file()) // 1024
        (debian / "control").write_text(
            f"Package: borrow-fighters\nVersion: {deb_version}\nArchitecture: amd64\n"
            "Maintainer: Borrow Fighters contributors\nSection: games\nPriority: optional\n"
            f"Installed-Size: {installed_size}\nDepends: libc6 (>= 2.35), libgl1\n"
            "Recommends: xwayland\nDescription: Local fighting game prototype with programming humor\n"
            " Includes runtime assets and portable native libraries. Requires an X11\n"
            " or XWayland desktop and an OpenGL 3.3 graphics driver.\n", encoding="utf-8")
        subprocess.run(["dpkg-deb", "--build", "--root-owner-group", str(root),
                        str(output / f"borrow-fighters_{deb_version}_amd64.deb")], check=True)
        shutil.rmtree(debian)
        rpm_root = temporary / "rpmbuild"
        for directory in ("BUILD", "BUILDROOT", "RPMS", "SOURCES", "SPECS", "SRPMS"):
            (rpm_root / directory).mkdir(parents=True)
        rpm_version = args.version.replace("-", "~", 1)
        spec = rpm_root / "SPECS/borrow-fighters.spec"
        spec.write_text(
            "%global debug_package %{nil}\n%global __os_install_post %{nil}\n"
            "Name: borrow-fighters\n"
            f"Version: {rpm_version}\nRelease: 1\nSummary: Local fighting game prototype\n"
            "License: MIT OR Apache-2.0\nBuildArch: x86_64\nAutoReqProv: no\n"
            "Requires: glibc >= 2.35\nRequires: libGL.so.1()(64bit)\nRequires: /bin/sh\n"
            "%description\nA local fighting game with programming humor.\n"
            "Includes assets and native libraries. Requires X11 or XWayland and OpenGL 3.3.\n"
            "%install\nmkdir -p %{buildroot}\n"
            f"cp -a '{root}/.' %{{buildroot}}/\n"
            "%files\n/opt/borrow-fighters\n/usr/bin/borrow-fighters\n"
            "/usr/share/applications/borrow-fighters.desktop\n"
            "/usr/share/icons/hicolor/scalable/apps/borrow-fighters.svg\n", encoding="utf-8")
        subprocess.run(["rpmbuild", "--define", f"_topdir {rpm_root}", "-bb", str(spec)], check=True)
        for file in (rpm_root / "RPMS").rglob("*.rpm"):
            copy(file, output / file.name)
    print(f"Linux packages created in {output}")


def windows_packages(args):
    stage, output = args.stage.resolve(), args.output.resolve()
    validate_build_info(stage, "windows-x86_64", args.version)
    validate_assets(stage)
    output.mkdir(parents=True, exist_ok=True)
    stem = f"borrow-fighters-{args.version}-windows-x86_64"
    # Cargo source archives can use reproducible timestamps older than 1980.
    # ZIP clamps only the date; the original license bytes remain unchanged.
    with zipfile.ZipFile(output / f"{stem}.zip", "w", zipfile.ZIP_DEFLATED,
                         strict_timestamps=False) as archive:
        for file in sorted(stage.rglob("*")):
            if file.is_file():
                archive.write(file, f"{stem}/{file.relative_to(stage).as_posix()}")
    compiler = shutil.which("ISCC.exe") or shutil.which("iscc")
    if not compiler:
        compiler = str(Path(os.environ.get("ProgramFiles(x86)", "C:/Program Files (x86)"))
                       / "Inno Setup 6/ISCC.exe")
    numeric = args.version.split("-", 1)[0]
    subprocess.run([compiler, f"/DStageDir={stage}", f"/DOutputDir={output}",
                    f"/DAppVersion={args.version}", f"/DNumericVersion={numeric}",
                    str(ROOT / "packaging/windows/installer.iss")], check=True)
    print(f"Windows packages created in {output}")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    commands = parser.add_subparsers(dest="command", required=True)
    stage = commands.add_parser("stage", help="Copy the executable, runtime assets and notices")
    stage.add_argument("--target", choices=TARGETS, required=True)
    stage.add_argument("--binary", type=Path, required=True,
                       help="Cargo output borrow-story[.exe]; shipped as borrow-fighters[.exe]")
    stage.add_argument("--lab-binary", type=Path,
                       help="Optionally include the separate borrow-actor-lab[.exe] production tool")
    stage.add_argument("--output", type=Path, required=True)
    stage.set_defaults(function=stage_package)
    for name, function in (("linux", linux_packages), ("windows", windows_packages)):
        command = commands.add_parser(name, help=f"Create {name} packages from a staged tree")
        command.add_argument("--stage", type=Path, required=True)
        command.add_argument("--output", type=Path, required=True)
        command.set_defaults(function=function)
    for command in (stage, *[commands.choices[name] for name in ("linux", "windows")]):
        command.add_argument("--version", required=True, help="Release version without leading v")
    verify = commands.add_parser("verify", help="Verify staged assets, hashes and Linux dependencies")
    verify.add_argument("--stage", type=Path, required=True)
    verify.set_defaults(function=verify_package)
    native_packages = commands.add_parser(
        "native-packages", help="Print installed Linux packages whose files will be bundled")
    native_packages.add_argument("--stage", type=Path, required=True)
    native_packages.set_defaults(function=print_native_packages)
    args = parser.parse_args()
    if hasattr(args, "version") and not re.fullmatch(r"\d+\.\d+\.\d+(?:-[a-zA-Z0-9.]+)?", args.version):
        parser.error("--version must be a semantic version without a leading v")
    args.function(args)


def verify_package(args):
    stage = args.stage.resolve()
    build = json.loads((stage / "BUILD-INFO.json").read_text(encoding="utf-8"))
    validate_build_info(stage, build["target"], build["version"])
    validate_assets(stage)
    for line in (stage / "PACKAGE-SHA256SUMS.txt").read_text(encoding="utf-8").splitlines():
        expected, name = line.split("  ", 1)
        file = stage / name
        actual = sha256(file)
        if expected != actual:
            raise ValueError(f"Checksum mismatch: {name}")
    if (stage / "NATIVE-LIBRARIES.json").is_file():
        validate_linux_libraries(stage)
    print(f"Verified staged assets and checksums in {stage}")


if __name__ == "__main__":
    main()
