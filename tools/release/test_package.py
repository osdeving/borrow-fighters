"""Check story release assets and executable staging with standard-library fixtures."""

import argparse
from contextlib import ExitStack
import json
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

import package


class RepositoryAssetsTests(unittest.TestCase):
    def test_story_release_contains_every_scene_audio_pose_font_and_opening_portrait(self):
        assets = {p.relative_to(package.ROOT).as_posix() for p in package.runtime_assets()}
        required = {
            "ada-prologue.png", "prologue-environments.png", "rust-morning.png",
            "street-life.png", "street-life.json",
            "rust-actions.png", "erratic.png", "rust-morning-poses.json",
            "rust-actions-poses.json", "erratic-poses.json", "texts/pt-BR.json",
            "texts/README.md", "opening/roster.json", "opening/cpp-origin.png",
            "opening/python-teacher.png", "fonts/Barlow-Regular.ttf",
            "fonts/Lora-Variable.ttf", "fonts/BarlowCondensed-SemiBold.ttf",
            "fonts/BARLOW-OFL.txt", "fonts/LORA-OFL.txt", "audio/README.md",
        }
        required.update(f"audio/{name}.wav" for name in (
            "ada", "morning", "threat", "remorse", "opening", "strike",
            "block", "hurt", "transition"))
        required.update(f"opening/roster/{name}.png" for name in (
            "rust", "duke", "c", "cpp", "python"))
        self.assertFalse({f"assets/adventure/{name}" for name in required} - assets)
        self.assertNotIn("assets/adventure/opening/roster/go.png", assets)
        self.assertNotIn("assets/adventure/adventure-environments.png", assets)
        self.assertFalse(any("prompts" in Path(name).parts for name in assets))
        self.assertNotIn("assets/adventure/audio/generate_audio.py", assets)


class AdventureReferencesTests(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory()
        self.addCleanup(temporary.cleanup)
        self.root = Path(temporary.name).resolve()
        self.base = self.root / "assets/adventure"
        self.contexts = ExitStack()
        self.addCleanup(self.contexts.close)
        self.contexts.enter_context(patch.object(package, "ROOT", self.root))
        self.write("src/adventure/engine/assets.rs", 'texture(rl, thread, "scene.png")')
        self.write("src/adventure/engine/opening.rs", 'texture(rl, thread, "intro.png")')
        self.write("src/adventure/engine/audio.rs", 'Self::Intro => "music.wav"')
        for name in ("scene.png", "opening/intro.png", "audio/music.wav",
                     "opening/roster/portrait.png", "fonts/BARLOW-OFL.txt",
                     "fonts/LORA-OFL.txt", "fonts/README.md", "audio/README.md",
                     "texts/README.md", "ART-PROVENANCE.md", "opening/ART-PROVENANCE.md"):
            self.write(f"assets/adventure/{name}", "fixture")
        self.roster("roster/portrait.png")

    def write(self, name, content):
        path = self.root / name
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(content, encoding="utf-8")

    def roster(self, image):
        self.write("assets/adventure/opening/roster.json", json.dumps({
            "characters": [{"image": image,
                            "source": "assets/production/unused-missing.png"}],
        }))

    def test_follows_dynamic_loader_names_and_image_without_provenance(self):
        assets = package.adventure_assets()
        for name in ("scene.png", "opening/intro.png", "audio/music.wav",
                     "opening/roster/portrait.png"):
            self.assertIn(self.base / name, assets)
        self.assertTrue(all(package.within(path, self.base) for path in assets))

    def test_missing_dynamic_scene_stops_packaging(self):
        (self.base / "scene.png").unlink()
        with self.assertRaisesRegex(ValueError, "Missing runtime asset"):
            package.adventure_assets()

    def test_missing_transitive_portrait_stops_packaging(self):
        (self.base / "opening/roster/portrait.png").unlink()
        with self.assertRaisesRegex(ValueError, "Missing runtime asset"):
            package.adventure_assets()

    def test_roster_cannot_escape_its_asset_directory_on_either_platform(self):
        for image in ("../scene.png", "/scene.png", "C:/scene.png",
                      "..\\scene.png", "roster//portrait.png"):
            with self.subTest(image=image):
                self.roster(image)
                with self.assertRaisesRegex(ValueError, "relative local image"):
                    package.adventure_assets()


class StagingTests(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory(prefix="release ação ")
        self.addCleanup(temporary.cleanup)
        self.root = Path(temporary.name).resolve()
        self.contexts = ExitStack()
        self.addCleanup(self.contexts.close)
        self.contexts.enter_context(patch.object(package, "ROOT", self.root))
        self.contexts.enter_context(patch.object(package, "rust_notices"))
        self.contexts.enter_context(patch.object(package, "run", return_value="fixture"))
        self.asset = self.root / "assets/adventure/texts/pt-BR.json"
        self.asset.parent.mkdir(parents=True)
        self.asset.write_text('{"title": "Ação — aventura"}', encoding="utf-8")
        self.contexts.enter_context(patch.object(package, "runtime_assets", return_value=[self.asset]))
        packaging = self.root / "packaging"
        (packaging / "linux").mkdir(parents=True)
        (packaging / "linux/borrow-fighters").write_text("#!/bin/sh\n", encoding="utf-8")
        (packaging / "JOGUE-PRIMEIRO.md").write_text("Jogue", encoding="utf-8")
        (self.root / "LICENSE-MIT").write_text("license fixture", encoding="utf-8")

    def stage(self, target):
        suffix = ".exe" if target == "windows-x86_64" else ""
        binary = self.root / f"borrow-story{suffix}"
        binary.write_bytes(b"composed executable fixture")
        args = argparse.Namespace(target=target, binary=binary,
                                  output=self.root / target, version="0.1.0-prototype.4")
        package.stage_package(args)
        return args.output

    def test_composition_keeps_public_name_and_licenses_in_unicode_paths(self):
        for target, executable in (("windows-x86_64", "borrow-fighters.exe"),
                                   ("linux-x86_64", "bin/borrow-fighters")):
            with self.subTest(target=target):
                stage = self.stage(target)
                self.assertEqual((stage / executable).read_bytes(), b"composed executable fixture")
                self.assertTrue((stage / "LICENSE-MIT").is_file())
                self.assertTrue((stage / "JOGUE-PRIMEIRO.md").is_file())
                build = json.loads((stage / "BUILD-INFO.json").read_text(encoding="utf-8"))
                self.assertEqual(build["cargo_binary"], "borrow-story")
                package.verify_package(argparse.Namespace(stage=stage))

    def test_verify_rejects_missing_or_changed_external_story_text(self):
        stage = self.stage("linux-x86_64")
        text = stage / self.asset.relative_to(self.root)
        text.write_text("altered", encoding="utf-8")
        with self.assertRaisesRegex(ValueError, "Staged asset missing or modified"):
            package.verify_package(argparse.Namespace(stage=stage))
        text.unlink()
        with self.assertRaisesRegex(ValueError, "Staged asset missing or modified"):
            package.verify_package(argparse.Namespace(stage=stage))

    def test_verify_rejects_changed_executable(self):
        stage = self.stage("windows-x86_64")
        (stage / "borrow-fighters.exe").write_bytes(b"changed")
        with self.assertRaisesRegex(ValueError, "Checksum mismatch: borrow-fighters.exe"):
            package.verify_package(argparse.Namespace(stage=stage))

    def test_rejects_fighting_only_binary_before_creating_stage(self):
        stage = self.root / "rejected"
        args = argparse.Namespace(target="linux-x86_64", binary=self.root / "borrow-fighters",
                                  output=stage, version="0.1.0-prototype.4")
        with self.assertRaisesRegex(ValueError, "requires the composed Cargo binary"):
            package.stage_package(args)
        self.assertFalse(stage.exists())

    def test_older_staging_cannot_be_packaged_as_the_story_release(self):
        stage = self.stage("linux-x86_64")
        path = stage / "BUILD-INFO.json"
        build = json.loads(path.read_text(encoding="utf-8"))
        del build["cargo_binary"]
        path.write_text(json.dumps(build), encoding="utf-8")
        with self.assertRaisesRegex(ValueError, "must contain the borrow-story composition"):
            package.validate_build_info(stage, "linux-x86_64", "0.1.0-prototype.4")


if __name__ == "__main__":
    unittest.main()
