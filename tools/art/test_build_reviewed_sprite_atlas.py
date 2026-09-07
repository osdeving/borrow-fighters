"""Exercise packing and rejection paths with tiny synthetic test fixtures only."""

from __future__ import annotations

import hashlib
import json
from pathlib import Path
import tempfile
import unittest

from PIL import Image

from build_reviewed_sprite_atlas import build


class ReviewedAtlasTests(unittest.TestCase):
    def setUp(self) -> None:
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        root = Path(self.temp.name)
        self.work = root / "production/rust"
        (self.work / "idle").mkdir(parents=True)
        self.output = root / "candidates/rust"
        self.source = self.work / "idle/source.png"
        self.production = self.work / "production.json"
        sheet = Image.new("RGBA", (12, 6))
        # Distinct opaque and soft-edge pixels expose alpha being applied twice.
        sheet.putpixel((1, 1), (200, 100, 50, 255))
        sheet.putpixel((2, 1), (80, 90, 100, 64))
        sheet.putpixel((8, 3), (40, 50, 60, 255))
        sheet.save(self.source)
        self.spec = {
            "schema": "borrow-fighters.production.v1", "character": "rust", "scale": 0.75,
            "provenance": {"review": "synthetic exporter test fixture, not character art"},
            "actions": {"idle": {
                "sheet": "idle/source.png", "reviewed": True, "loop": True,
                "frames": [
                    {"name": "idle_recover", "source_rect": {"x": 6, "y": 0, "w": 6, "h": 6},
                     "pivot": {"x": 2, "y": 6}, "duration_ms": 120, "phase": "recovery"},
                    {"name": "idle_prepare", "source_rect": {"x": 0, "y": 0, "w": 6, "h": 4},
                     "pivot": {"x": 3, "y": 4}, "duration_ms": 80, "phase": "startup"},
                ],
            }},
        }

    def write_spec(self) -> None:
        self.production.write_text(json.dumps(self.spec))

    def export(self, columns: int | None = None) -> dict:
        self.write_spec()
        path = build(self.production, self.output, columns)
        return json.loads(path.read_text())

    def test_preserves_pixels_explicit_sequence_pivots_and_source(self) -> None:
        source_hash = hashlib.sha256(self.source.read_bytes()).hexdigest()
        manifest = self.export(columns=1)
        self.assertEqual(manifest["scale"], 0.75)
        self.assertEqual(manifest["cell"], {"w": 6, "h": 6})
        self.assertEqual(manifest["clips"], [
            {"name": "idle", "loop": True, "frames": ["idle_recover", "idle_prepare"]},
        ])
        with Image.open(self.source) as sheet, Image.open(self.output / manifest["image"]) as atlas:
            self.assertEqual(atlas.size, (6, 12))
            for supplied, frame in zip(self.spec["actions"]["idle"]["frames"], manifest["frames"]):
                rect = supplied["source_rect"]
                original = sheet.crop((rect["x"], rect["y"], rect["x"] + rect["w"], rect["y"] + rect["h"]))
                packed = frame["frame"]
                result = atlas.crop((packed["x"], packed["y"], packed["x"] + packed["w"], packed["y"] + packed["h"]))
                self.assertEqual(result.tobytes(), original.tobytes())
                with Image.open(self.work / "idle/frames" / f"{frame['name']}.png") as crop:
                    self.assertEqual(crop.tobytes(), original.tobytes())
                self.assertEqual(frame["pivot"], supplied["pivot"])
                self.assertEqual(frame["duration_ms"], supplied["duration_ms"])
                self.assertNotIn("combat", frame)
                self.assertNotIn("phase", frame)
        self.assertEqual(hashlib.sha256(self.source.read_bytes()).hexdigest(), source_hash)
        provenance = json.loads((self.output / "rust-fighter.provenance.json").read_text())
        self.assertEqual(provenance["production_snapshot"], self.spec)
        self.assertEqual(provenance["sources"][0]["sha256"], source_hash)
        self.assertEqual(provenance["frames"][0]["phase"], "recovery")
        self.assertEqual(provenance["frames"][0]["source_rect"], {"x": 6, "y": 0, "w": 6, "h": 6})

    def test_allocates_enough_rows_for_all_frames_and_actions(self) -> None:
        action = self.spec["actions"]["idle"]
        template = action["frames"][0]
        action["frames"] = [dict(template, name=f"idle_{i}") for i in range(12)]
        self.spec["actions"]["kick"] = {
            "sheet": "idle/source.png", "loop": False, "reviewed": True,
            "frames": [dict(template, name="kick_active")],
        }
        manifest = self.export(columns=3)
        self.assertEqual(len(manifest["frames"]), 13)
        self.assertEqual(manifest["clips"][1]["frames"], ["kick_active"])
        last = manifest["frames"][-1]["frame"]
        with Image.open(self.output / manifest["image"]) as atlas:
            self.assertEqual(atlas.size, (18, 30))
            self.assertEqual(atlas.getpixel((last["x"] + 2, last["y"] + 3)), (40, 50, 60, 255))

    def test_rejects_invalid_input_before_writing_outputs(self) -> None:
        original = json.loads(json.dumps(self.spec))
        cases = [
            ("unreviewed", lambda p: p["actions"]["idle"].update(reviewed=False), "reviewed=true"),
            ("zero duration", lambda p: p["actions"]["idle"]["frames"][0].update(duration_ms=0), "duration_ms"),
            ("boolean duration", lambda p: p["actions"]["idle"]["frames"][0].update(duration_ms=True), "duration_ms"),
            ("negative scale", lambda p: p.update(scale=-1), "scale"),
            ("missing loop", lambda p: p["actions"]["idle"].pop("loop"), "loop"),
            ("duplicate name", lambda p: p["actions"]["idle"]["frames"][1].update(name="idle_recover"), "duplicate"),
            ("out of PNG", lambda p: p["actions"]["idle"]["frames"][0]["source_rect"].update(w=7), "PNG bounds"),
            ("missing pivot", lambda p: p["actions"]["idle"]["frames"][0].pop("pivot"), "pivot"),
            ("out of pivot", lambda p: p["actions"]["idle"]["frames"][0]["pivot"].update(y=7), "pivot.y"),
            ("empty crop", lambda p: p["actions"]["idle"]["frames"][0].update(
                source_rect={"x": 0, "y": 4, "w": 6, "h": 2}, pivot={"x": 3, "y": 2}), "visible art"),
            ("unsafe name", lambda p: p.update(character="../rust"), "character"),
        ]
        for label, mutate, expected in cases:
            with self.subTest(label=label):
                self.spec = json.loads(json.dumps(original))
                mutate(self.spec)
                self.write_spec()
                with self.assertRaisesRegex(ValueError, expected):
                    build(self.production, self.output)
                self.assertFalse(self.output.exists())
                self.assertFalse((self.work / "idle/frames").exists())

    def test_rejects_opaque_png_even_with_alpha_channel(self) -> None:
        for mode in ("RGB", "RGBA"):
            with self.subTest(mode=mode):
                Image.new(mode, (12, 6), "white").save(self.source)
                self.write_spec()
                with self.assertRaisesRegex(ValueError, "real transparent"):
                    build(self.production, self.output)
                self.assertFalse(self.output.exists())

    def test_rejects_opaque_crop_despite_transparent_sheet(self) -> None:
        image = Image.new("RGBA", (12, 6))
        image.paste((50, 50, 50, 255), (6, 0, 12, 6))
        image.save(self.source)
        self.write_spec()
        with self.assertRaisesRegex(ValueError, "each crop needs"):
            build(self.production, self.output)

    def test_explicit_combat_is_preserved_and_validated(self) -> None:
        frame = self.spec["actions"]["idle"]["frames"][0]
        frame["combat"] = {"hurtboxes": [{"x": 1, "y": 1, "w": 3, "h": 5}],
                           "projectile_origin": {"x": 6, "y": 0}}
        manifest = self.export()
        self.assertEqual(manifest["frames"][0]["combat"], frame["combat"])
        frame["combat"]["hurtboxes"][0]["w"] = 6
        self.write_spec()
        with self.assertRaisesRegex(ValueError, "inside the source rectangle"):
            build(self.production, self.output)

    def test_rejects_output_inside_work_directory(self) -> None:
        self.write_spec()
        with self.assertRaisesRegex(ValueError, "separate"):
            build(self.production, self.work / "export")


if __name__ == "__main__":
    unittest.main()
