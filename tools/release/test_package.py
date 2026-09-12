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
            "street-life.png", "street-traffic.png", "street/catalog.json",
            "street/scene.json", "street/vehicles.png", "street/props.png",
            "street/cyclist-escape.png", "street/README.md",
            "street/neighbours.png", "street/shopkeeper.png", "street/shutter.png", "street/caramelo.png",
            "rust-actions.png", "erratic.png", "rust-morning-poses.json",
            "rust-actions-poses.json", "erratic-poses.json", "texts/pt-BR.json",
            "texts/README.md", "opening/roster.json", "opening/cpp-origin.png",
            "opening/python-teacher.png", "fonts/Barlow-Regular.ttf",
            "fonts/Lora-Variable.ttf", "fonts/BarlowCondensed-SemiBold.ttf",
            "fonts/BARLOW-OFL.txt", "fonts/LORA-OFL.txt", "audio/README.md",
            "chapter/world.json", "chapter/chapter-texts.json", "chapter/phone-style.json",
            "chapter/catalog.json", "chapter/lane.png", "chapter/rust-narrative.png",
            "chapter/driver.png", "chapter/README.md", "chapter/DRIVER.md",
            "chapter/audio/README.md",
        }
        required.update(f"audio/{name}.wav" for name in (
            "ada", "morning_ambience", "street_air", "street_traffic", "remorse", "opening", "strike",
            "block", "hurt", "transition", "car_horn", "car_skid", "car_crash",
            "traffic_escape", "bicycle_fall", "dog_alert", "shutter_roll", "shutter_clack"))
        required.update(f"opening/roster/{name}.png" for name in (
            "rust", "duke", "c", "cpp", "python"))
        required.update(f"chapter/audio/{name}.wav" for name in (
            "phone_pocket", "phone_tap", "phone_send", "phone_receive", "footstep"))
        self.assertFalse({f"assets/adventure/{name}" for name in required} - assets)
        self.assertNotIn("assets/adventure/opening/roster/go.png", assets)
        self.assertNotIn("assets/adventure/adventure-environments.png", assets)
        self.assertFalse(any("prompts" in Path(name).parts for name in assets))
        self.assertNotIn("assets/adventure/audio/generate_audio.py", assets)
        self.assertNotIn("assets/adventure/audio/generate_traffic_audio.py", assets)
        self.assertNotIn("assets/adventure/audio/generate_evacuation_audio.py", assets)
        self.assertNotIn("assets/adventure/audio/generate_neighbourhood_audio.py", assets)
        self.assertNotIn("assets/adventure/audio/morning.wav", assets)
        self.assertNotIn("assets/adventure/audio/threat.wav", assets)
        for name in ("lane.json", "driver.json", "rust-narrative.json", "audio/generate_audio.py"):
            self.assertNotIn(f"assets/adventure/chapter/{name}", assets)


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
                     "texts/README.md", "ART-PROVENANCE.md", "opening/ART-PROVENANCE.md",
                     "street/car.png", "street/poses.png", "street/prop.png",
                     "street/README.md", "chapter/README.md", "chapter/DRIVER.md",
                     "chapter/audio/README.md", "chapter/rust.png", "chapter/driver.png"):
            self.write(f"assets/adventure/{name}", "fixture")
        self.roster("roster/portrait.png")
        self.street_catalog("street/car.png", "street/poses.png")
        self.chapter_catalog("chapter/rust.png", "chapter/driver.png")
        self.write("assets/adventure/street/scene.json", json.dumps({
            "version": 1, "props": [],
        }))

    def write(self, name, content):
        path = self.root / name
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(content, encoding="utf-8")

    def roster(self, image):
        self.write("assets/adventure/opening/roster.json", json.dumps({
            "characters": [{"image": image,
                            "source": "assets/production/unused-missing.png"}],
        }))

    def street_catalog(self, *images):
        self.write("assets/adventure/street/catalog.json", json.dumps({
            "version": 1,
            "pieces": {
                "vehicle.test": {
                    "width": 160, "frame_ticks": 6,
                    "frames": [{"image": image, "source": [0, 0, 400, 200],
                                "anchor": [200, 200]} for image in images],
                },
                "prop.test": {
                    "width": 80, "frame_ticks": 1,
                    "frames": [{"image": "street/prop.png", "source": [0, 0, 80, 100],
                                "anchor": [40, 100]}],
                },
            },
        }))

    def chapter_catalog(self, *images):
        self.write("assets/adventure/chapter/catalog.json", json.dumps({
            "version": 1,
            "pieces": {"rust.phone": {
                "width": 90, "frame_ticks": 7,
                "frames": [{"image": image, "source": [0, 0, 90, 170],
                            "anchor": [45, 170]} for image in images],
            }},
            "source": "chapter/production-unused.png",
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

    def test_follows_all_street_pieces_frames_and_shared_images_only_once(self):
        self.street_catalog("street/car.png", "street/poses.png", "street/car.png")
        self.write("assets/adventure/street/unused.png", "unused source art")
        assets = package.adventure_assets()
        expected = {self.base / name for name in (
            "street/catalog.json", "street/scene.json", "street/car.png",
            "street/poses.png", "street/prop.png", "street/README.md")}
        self.assertTrue(expected <= assets)
        self.assertEqual(sum(path == self.base / "street/car.png" for path in assets), 1)
        self.assertNotIn(self.base / "street/unused.png", assets)

    def test_missing_later_street_frame_stops_packaging(self):
        (self.base / "street/poses.png").unlink()
        with self.assertRaisesRegex(ValueError, "Missing runtime asset"):
            package.adventure_assets()

    def test_chapter_catalog_follows_every_frame_and_reuses_street_atlas_once(self):
        self.chapter_catalog("chapter/rust.png", "street/poses.png", "chapter/driver.png",
                             "chapter/rust.png")
        self.write("assets/adventure/chapter/production-unused.png", "production art")
        assets = package.adventure_assets()
        expected = {self.base / name for name in (
            "chapter/catalog.json", "chapter/rust.png", "chapter/driver.png",
            "chapter/README.md", "chapter/DRIVER.md", "chapter/audio/README.md")}
        self.assertTrue(expected <= assets)
        self.assertEqual(sum(path == self.base / "street/poses.png" for path in assets), 1)
        self.assertEqual(sum(path == self.base / "chapter/rust.png" for path in assets), 1)
        self.assertNotIn(self.base / "chapter/production-unused.png", assets)

    def test_new_declared_catalogs_ship_images_without_copying_unused_art(self):
        self.write("src/adventure/engine/locomotion.rs",
                   'load_catalog("assets/adventure/locomotion/catalog.json")')
        self.write("assets/adventure/locomotion/catalog.json", json.dumps({
            "version": 1, "pieces": {"rust.kick": {"frames": [
                {"image": "locomotion/kick.png"}, {"image": "chapter/rust.png"}]}},
        }))
        self.write("assets/adventure/locomotion/kick.png", "runtime art")
        self.write("assets/adventure/locomotion/unused.png", "unused source")
        assets = package.adventure_assets()
        self.assertIn(self.base / "locomotion/catalog.json", assets)
        self.assertIn(self.base / "locomotion/kick.png", assets)
        self.assertNotIn(self.base / "locomotion/unused.png", assets)
        (self.base / "locomotion/kick.png").unlink()
        with self.assertRaisesRegex(ValueError, "Missing runtime asset"):
            package.adventure_assets()

    def test_missing_later_chapter_frame_stops_packaging(self):
        (self.base / "chapter/driver.png").unlink()
        with self.assertRaisesRegex(ValueError, "Missing runtime asset"):
            package.adventure_assets()

    def test_street_frame_paths_are_local_pngs_on_either_platform(self):
        for image in ("../outside.png", "/scene.png", "C:/scene.png",
                      "..\\scene.png", "street//car.png", "street/./car.png",
                      "street/car.jpg", "https://example.com/car.png", "", None):
            with self.subTest(image=image):
                self.street_catalog(image)
                with self.assertRaisesRegex(ValueError, "relative local PNG"):
                    package.adventure_assets()

    def test_chapter_frame_paths_are_local_pngs_on_either_platform(self):
        for image in ("../outside.png", "/scene.png", "C:/scene.png",
                      "..\\scene.png", "chapter//rust.png", "chapter/./rust.png",
                      "chapter/rust.jpg", "https://example.com/rust.png", "", None):
            with self.subTest(image=image):
                self.chapter_catalog(image)
                with self.assertRaisesRegex(ValueError, "relative local PNG"):
                    package.adventure_assets()

    def test_chapter_image_symlink_cannot_point_into_fighting_assets(self):
        self.write("assets/candidates/rust.png", "other domain")
        link = self.base / "chapter/link.png"
        try:
            link.symlink_to(self.root / "assets/candidates/rust.png")
        except (OSError, NotImplementedError) as error:
            self.skipTest(f"Symlinks unavailable: {error}")
        self.chapter_catalog("chapter/link.png")
        with self.assertRaisesRegex(ValueError, "outside assets/adventure"):
            package.adventure_assets()

    def test_street_image_symlink_cannot_point_into_fighting_assets(self):
        self.write("assets/candidates/car.png", "other domain")
        link = self.base / "street/link.png"
        try:
            link.symlink_to(self.root / "assets/candidates/car.png")
        except (OSError, NotImplementedError) as error:
            self.skipTest(f"Symlinks unavailable: {error}")
        self.street_catalog("street/link.png")
        with self.assertRaisesRegex(ValueError, "outside assets/adventure"):
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

    def test_stage_preserves_chapter_catalog_closure_and_verify_detects_missing_pose(self):
        base = self.root / "assets/adventure"
        (base / "chapter").mkdir()
        catalog = base / "chapter/catalog.json"
        catalog.write_text(json.dumps({"pieces": {"rust.phone": {
            "frames": [{"image": f"chapter/{name}.png"} for name in ("draw", "stow")],
        }}}), encoding="utf-8")
        for name in ("draw", "stow", "unused"):
            (base / f"chapter/{name}.png").write_bytes(name.encode())
        closure = package.adventure_piece_assets(catalog)
        with patch.object(package, "runtime_assets", return_value=sorted(closure | {self.asset})):
            stage = self.stage("linux-x86_64")
            package.verify_package(argparse.Namespace(stage=stage))
            self.assertFalse((stage / "assets/adventure/chapter/unused.png").exists())
            (stage / "assets/adventure/chapter/stow.png").unlink()
            with self.assertRaisesRegex(ValueError, "Staged asset missing or modified"):
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
