"""Check story release assets and executable staging with standard-library fixtures."""

import argparse
from contextlib import ExitStack
import json
import struct
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

import package


class RepositoryAssetsTests(unittest.TestCase):
    def test_production_closure_ships_runtime_art_and_audio_without_sources_or_unused_pngs(self):
        assets = {p.relative_to(package.ROOT).as_posix() for p in package.runtime_assets()}
        base = "assets/adventure/"
        required = {"campaign.json", "production-lab.json",
                    "chapters/cpp-augusta/chapter.json", "chapters/cpp-augusta/world.json",
                    "chapters/cpp-augusta/texts.json", "chapters/cpp-augusta/world-art.json",
                    "chapters/cpp-augusta/nightlife-cast.json",
                    "audio/production/catalog.json"}
        for actor in ("cpp", "julia", "broker", "security", "erratic"):
            required.update(f"actors/{actor}/{name}.json" for name in ("character", "rig", "combat", "clips"))
        required.update(f"chapters/cpp-augusta/sprites/{name}.png" for name in (
            "facade-residential", "facade-bar", "facade-mural", "skyline", "ground",
            "restrained-pair-chroma", "nightlife-cast-a", "nightlife-cast-b",
            "nightlife-profile-a", "nightlife-profile-b"))
        required.update(f"audio/production/{name}.wav" for name in (
            "street-loop", "night-air", "bar-door", "guard-step", "ep-rupture", "panic",
            "swish", "impact", "parry", "projectile", "landing"))
        self.assertFalse({base + name for name in required} - assets)
        production = {name for name in assets if name.startswith((base + "actors/", base + "chapters/"))}
        self.assertFalse(any({"source", "sources", "reviews", "prompts"} & set(Path(name).parts)
                             for name in production))
        self.assertFalse(any("import" in Path(name).name for name in production))
        for name in ("actors/cpp/sprites/body-profile.png", "actors/julia/sprites/talk.png",
                     "actors/erratic/sprites/poses.png"):
            self.assertIn(base + name, production)

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
            "chapter/catalog.json", "chapter/rust-narrative.png",
            "chapter/driver.png", "chapter/README.md", "chapter/DRIVER.md",
            "chapter/audio/README.md",
            "world/map.json", "world/catalog.json", "world/distance.png",
            "world/pavement.png", "world/planter.png", "world/boundary-pillar.png",
            "street/arrival-camera.json",
            "locomotion/catalog.json", "locomotion/run-rig.json",
            "locomotion/run-mesh.json", "locomotion/run-mesh/leg.png",
        }
        required.update(f"audio/{name}.wav" for name in (
            "ada", "morning_ambience", "street_air", "street_traffic", "remorse", "opening", "strike",
            "block", "hurt", "transition", "car_horn", "car_skid", "car_crash",
            "traffic_escape", "bicycle_fall", "dog_alert", "shutter_roll", "shutter_clack"))
        required.update(f"opening/roster/{name}.png" for name in (
            "rust", "duke", "c", "cpp", "python"))
        required.update(f"chapter/audio/{name}.wav" for name in (
            "phone_pocket", "phone_tap", "phone_send", "phone_receive", "footstep"))
        required.update(f"world/{name}.png" for name in (
            "house-cream", "house-balcony", "house-teal", "shop-restaurant",
            "shop-cafe", "wall-gate"))
        required.update(f"locomotion/run/{name}.png" for name in (
            "body", "near-arm", "far-arm", "near-boot", "far-boot"))
        required.update(f"locomotion/crossing/{view}-{frame:02}.png"
                        for view in ("back", "front") for frame in range(8))
        required.update(f"opening/scenes/{name}-painted.png" for name in (
            "duke-paulista", "duke-boardroom", "old-c-workshop", "old-c-foundations"))
        self.assertFalse({f"assets/adventure/{name}" for name in required} - assets)
        self.assertFalse({f"assets/adventure/locomotion/run/{side}-{limb}.png"
                          for side in ("near", "far") for limb in ("thigh", "shin")} & assets)
        self.assertNotIn("assets/adventure/opening/roster/go.png", assets)
        self.assertNotIn("assets/adventure/adventure-environments.png", assets)
        self.assertFalse(any("prompts" in Path(name).parts for name in assets))
        self.assertNotIn("assets/adventure/audio/generate_audio.py", assets)
        self.assertNotIn("assets/adventure/audio/generate_traffic_audio.py", assets)
        self.assertNotIn("assets/adventure/audio/generate_evacuation_audio.py", assets)
        self.assertNotIn("assets/adventure/audio/generate_neighbourhood_audio.py", assets)
        self.assertNotIn("assets/adventure/audio/morning.wav", assets)
        self.assertNotIn("assets/adventure/audio/threat.wav", assets)
        for name in ("lane.png", "lane.json", "driver.json", "rust-narrative.json", "audio/generate_audio.py"):
            self.assertNotIn(f"assets/adventure/chapter/{name}", assets)
        for name in ("world/sources/facades-keyed.png", "world/prompts.md",
                     "locomotion/source/run-rig.png", "locomotion/source/rust-walk-kick.png"):
            self.assertNotIn(f"assets/adventure/{name}", assets)


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


class ProductionReferencesTests(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory(prefix="production closure ")
        self.addCleanup(temporary.cleanup)
        self.root = Path(temporary.name).resolve()
        self.base = self.root / "assets/adventure"
        self.contexts = ExitStack()
        self.addCleanup(self.contexts.close)
        self.contexts.enter_context(patch.object(package, "ROOT", self.root))
        self.write("campaign.json", {"chapters": [{"id": "rust"}, {"id": "cpp-test"}]})
        self.write("production-lab.json", {"title": "Independent production tool"})
        self.make_actor("cpp")
        self.make_actor("npc")
        self.chapter = "chapters/cpp-test/"
        self.write(self.chapter + "chapter.json", {"world": "world.json", "texts": "texts.json", "art": "art.json"})
        self.write(self.chapter + "world.json", {"pieces": [{"piece": "wall"}]})
        self.write(self.chapter + "texts.json", {"line": "original text"})
        self.art = {"actors": {"cpp": "actors/cpp/character.json", "npc": "actors/npc/character.json"},
                    "pieces": {"wall": {"image": "sprites/wall.png"}, "wall-again": {"image": "sprites/wall.png"}},
                    "source": "source/missing-generation.png"}
        self.write(self.chapter + "art.json", self.art)
        self.write(self.chapter + "sprites/wall.png", "art")
        self.audio = {"ambience": {"file": "street.wav"}, "effects": {"hit": {"file": "hit.wav"}},
                      "provenance": "source/unshipped.wav"}
        self.write("audio/production/catalog.json", self.audio)
        self.write("audio/production/street.wav", "ambience")
        self.write("audio/production/hit.wav", "hit")

    def write(self, name, content):
        path = self.base / name
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(json.dumps(content) if isinstance(content, dict) else content, encoding="utf-8")

    def make_actor(self, identifier):
        directory = f"actors/{identifier}/"
        self.write(directory + "character.json", {"combat": "moves.json", "rig": "rig.json", "clips": "clips.json"})
        self.write(directory + "moves.json", {"moves": []})
        self.write(directory + "clips.json", {"clips": {}})
        self.write(directory + "rig.json", {"attachments": {
            "first": {"image": "sprites/body.png"}, "reuse": {"image": "sprites/body.png"},
            "last": {"image": "sprites/late.png"}}, "source": "source/missing-retake.png"})
        for name in ("body", "late", "unused"):
            self.write(directory + f"sprites/{name}.png", name)
        self.write(directory + "source/generation.png", "unshipped source")
        self.write(directory + "import-manifest.json", {"source": "source/generation.png"})

    def test_closure_includes_all_actor_attachments_once_and_ignores_authoring_fields(self):
        assets = package.production_assets()
        self.assertIn(self.base / "actors/npc/sprites/late.png", assets)
        self.assertIn(self.base / "actors/cpp/moves.json", assets)
        self.assertEqual(sum(p == self.base / "actors/cpp/sprites/body.png" for p in assets), 1)
        self.assertEqual(sum(p == self.base / self.chapter / "sprites/wall.png" for p in assets), 1)
        self.assertFalse(any("source" in p.parts or p.name.startswith("import") or p.name == "unused.png" for p in assets))

    def test_unregistered_actor_or_chapter_is_not_collected(self):
        self.make_actor("unused")
        self.write("chapters/unregistered/source.png", "not playable")
        assets = package.production_assets()
        self.assertFalse(any("unused" in p.parts or "unregistered" in p.parts for p in assets))

    def test_crowd_registration_is_shipped_and_missing_registration_aborts(self):
        self.art["nightlife_cast"] = "nightlife-cast.json"
        self.write(self.chapter + "art.json", self.art)
        name = self.chapter + "nightlife-cast.json"
        self.write(name, {"schema_version": 1, "entries": {}})
        self.assertIn(self.base / name, package.production_assets())
        (self.base / name).unlink()
        with self.assertRaisesRegex(ValueError, "Missing runtime asset"):
            package.production_assets()

    def test_crowd_registration_is_contained_and_excludes_authoring_material(self):
        for name in ("../cast.json", "source/cast.json"):
            self.art["nightlife_cast"] = name
            self.write(self.chapter + "art.json", self.art)
            with self.subTest(name=name), self.assertRaisesRegex(ValueError, "Production"):
                package.production_assets()

    def make_models(self, gltf=None):
        self.art["models_3d"] = {"humans": "models/humans.json"}
        self.write(self.chapter + "art.json", self.art)
        self.write("models/humans.json", {"schema_version": 1, "entries": {
            "cpp": {"file": "cpp.glb", "source": "source/cpp.blend"}}})
        self.write("models/source/cpp.blend", "editable production source")
        body = json.dumps(gltf or {"asset": {"version": "2.0"}}).encode()
        body += b" " * (-len(body) % 4)
        model = self.base / "models/cpp.glb"
        model.write_bytes(struct.pack("<4sII", b"glTF", 2, 20 + len(body))
                          + struct.pack("<I4s", len(body), b"JSON") + body)
        return model

    def test_model_closure_ships_declared_glbs_and_requires_them_without_shipping_blends(self):
        model = self.make_models()
        assets = package.production_assets()
        self.assertIn(model, assets)
        self.assertIn(self.base / "models/humans.json", assets)
        self.assertNotIn(self.base / "models/source/cpp.blend", assets)
        model.unlink()
        with self.assertRaisesRegex(ValueError, "Missing runtime asset"):
            package.production_assets()

    def test_glb_external_images_and_buffers_cannot_be_lost_during_packaging(self):
        for kind, uri in (("images", "skin.png"), ("buffers", "body.bin")):
            with self.subTest(kind=kind):
                self.make_models({"asset": {"version": "2.0"}, kind: [{"uri": uri}]})
                with self.assertRaisesRegex(ValueError, "embed all buffers and textures"):
                    package.production_assets()

    def test_missing_dependency_at_every_level_aborts_collection(self):
        for name in ("production-lab.json", "actors/npc/moves.json", "actors/npc/clips.json",
                     "actors/npc/rig.json", "actors/npc/sprites/late.png", self.chapter + "chapter.json",
                     self.chapter + "world.json", self.chapter + "texts.json", self.chapter + "art.json",
                     self.chapter + "sprites/wall.png", "audio/production/hit.wav"):
            path = self.base / name
            content = path.read_bytes()
            path.unlink()
            try:
                with self.subTest(name=name), self.assertRaisesRegex(ValueError, "Missing runtime asset"):
                    package.production_assets()
            finally:
                path.write_bytes(content)

    def test_actor_images_cannot_escape_package_or_use_authoring_material(self):
        for name in ("../body.png", "/body.png", "C:/body.png", "..\\body.png",
                     "sprites//body.png", "sprites/./body.png", "sprites/body.jpg", "source/generation.png"):
            self.write("actors/cpp/rig.json", {"attachments": {"body": {"image": name}}})
            with self.subTest(name=name), self.assertRaisesRegex(ValueError, "Production"):
                package.production_assets()

    def test_world_and_audio_references_are_contained(self):
        self.art["pieces"]["wall"]["image"] = "../outside.png"
        self.write(self.chapter + "art.json", self.art)
        with self.assertRaisesRegex(ValueError, "relative local"):
            package.production_assets()
        self.art["pieces"]["wall"]["image"] = "sprites/wall.png"
        self.write(self.chapter + "art.json", self.art)
        self.audio["effects"]["hit"]["file"] = "../../outside.wav"
        self.write("audio/production/catalog.json", self.audio)
        with self.assertRaisesRegex(ValueError, "relative local"):
            package.production_assets()

    def test_actor_image_symlink_cannot_cross_into_another_package(self):
        link = self.base / "actors/cpp/sprites/body.png"
        link.unlink()
        try:
            link.symlink_to(self.base / "actors/npc/sprites/body.png")
        except (OSError, NotImplementedError) as error:
            self.skipTest(f"Symlinks unavailable: {error}")
        with self.assertRaisesRegex(ValueError, "outside its package"):
            package.production_assets()

    def test_registry_ids_cannot_be_paths(self):
        for identifier in ("../cpp", "cpp/test", "/cpp", "C:/cpp", None):
            self.write("campaign.json", {"chapters": [{"id": identifier}]})
            with self.subTest(identifier=identifier), self.assertRaisesRegex(ValueError, "chapter id"):
                package.production_assets()

    def test_standalone_lab_default_stays_available_without_a_production_chapter(self):
        self.write("campaign.json", {"chapters": [{"id": "rust"}]})
        assets = package.production_assets()
        self.assertIn(self.base / "actors/cpp/sprites/late.png", assets)
        self.assertNotIn(self.base / "actors/npc/character.json", assets)


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
        (packaging / "linux/borrow-actor-lab").write_text("#!/bin/sh\n", encoding="utf-8")
        (packaging / "JOGUE-PRIMEIRO.md").write_text("Jogue", encoding="utf-8")
        (self.root / "LICENSE-MIT").write_text("license fixture", encoding="utf-8")

    def stage(self, target, include_lab=False):
        suffix = ".exe" if target == "windows-x86_64" else ""
        binary = self.root / f"borrow-story{suffix}"
        binary.write_bytes(b"composed executable fixture")
        args = argparse.Namespace(target=target, binary=binary,
                                  output=self.root / target, version="0.1.0-prototype.4")
        if include_lab:
            args.lab_binary = self.root / f"borrow-actor-lab{suffix}"
            args.lab_binary.write_bytes(b"production lab executable fixture")
        package.stage_package(args)
        return args.output

    def test_optional_lab_is_staged_and_checksummed_on_both_platforms(self):
        for target, executable in (("windows-x86_64", "borrow-actor-lab.exe"),
                                   ("linux-x86_64", "bin/borrow-actor-lab")):
            with self.subTest(target=target):
                stage = self.stage(target, include_lab=True)
                self.assertEqual((stage / executable).read_bytes(), b"production lab executable fixture")
                info = json.loads((stage / "BUILD-INFO.json").read_text())
                self.assertEqual(info["tools"], [{"cargo_binary": "borrow-actor-lab", "path": executable}])
                package.verify_package(argparse.Namespace(stage=stage))
                (stage / executable).write_bytes(b"changed tool")
                with self.assertRaisesRegex(ValueError, "Checksum mismatch"):
                    package.verify_package(argparse.Namespace(stage=stage))
                (stage / executable).unlink()
                with self.assertRaisesRegex(ValueError, "Missing production lab executable"):
                    package.verify_package(argparse.Namespace(stage=stage))

    def test_player_package_does_not_implicitly_include_a_sibling_lab(self):
        (self.root / "borrow-actor-lab").write_bytes(b"should not be copied")
        stage = self.stage("linux-x86_64")
        self.assertFalse((stage / "bin/borrow-actor-lab").exists())
        self.assertFalse((stage / "borrow-actor-lab").exists())
        self.assertEqual(json.loads((stage / "BUILD-INFO.json").read_text())["tools"], [])

    def test_wrong_optional_lab_binary_is_rejected_before_staging(self):
        args = argparse.Namespace(target="linux-x86_64", binary=self.root / "borrow-story",
                                  lab_binary=self.root / "borrow-fighters", output=self.root / "rejected",
                                  version="0.1.0-prototype.4")
        with self.assertRaisesRegex(ValueError, "Production lab requires Cargo binary"):
            package.stage_package(args)
        self.assertFalse(args.output.exists())

    def test_optional_lab_participates_in_native_dependency_and_glibc_validation(self):
        stage = self.stage("linux-x86_64", include_lab=True)
        with patch.object(package, "ldd_libraries", return_value={}) as ldd:
            with patch.object(package, "run", return_value="GLIBC_2.35"):
                package.validate_linux_libraries(stage)
            self.assertEqual({call.args[0] for call in ldd.call_args_list},
                             {stage / "bin/borrow-fighters", stage / "bin/borrow-actor-lab"})

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
