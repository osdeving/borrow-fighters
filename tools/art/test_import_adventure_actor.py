"""Exercise the import CLI with tiny deterministic ImageMagick fixtures."""

import copy
import importlib.util
import json
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import unittest


TOOL = Path(__file__).with_name("import_adventure_actor.py")
SPEC = importlib.util.spec_from_file_location("adventure_import", TOOL)
IMPORT = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(IMPORT)


@unittest.skipUnless(shutil.which("magick") or shutil.which("convert"), "ImageMagick required")
class ImportContract(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(prefix="test-adventure-import-")
        self.addCleanup(self.temporary.cleanup)
        self.root = Path(self.temporary.name)
        self.images = IMPORT.ImageMagick()
        self.source = self.root / "source.png"
        self.images.run("-size", "12x8", "xc:magenta", "-fill", "black", "-draw",
                        "rectangle 3,2 8,5", "-fill", "#800080", "-draw", "point 9,3",
                        "-fill", "white", "-draw", "point 4,3", self.source)
        self.recipe_path = self.root / "import.json"
        self.recipe = {
            "version": 1, "keying": "magenta",
            "sources": {"sheet": {"path": "source.png"}},
            "exports": [{"id": "arm", "source": "sheet", "crop": [2, 1, 8, 6],
                         "output": "sprites/arm.png", "landmarks": {"elbow": [5, 3]}}],
        }
        self.save()

    def save(self):
        self.recipe_path.write_text(json.dumps(self.recipe), encoding="utf-8")

    def cli(self, *extra, success=True):
        result = subprocess.run([sys.executable, str(TOOL), "--recipe", str(self.recipe_path), *extra],
                                capture_output=True, text=True, check=False)
        if success:
            self.assertEqual(result.returncode, 0, result.stderr)
        else:
            self.assertNotEqual(result.returncode, 0)
        return result

    def snapshot(self):
        return {str(p.relative_to(self.root)): (p.read_bytes(), p.stat().st_mtime_ns)
                for p in self.root.rglob("*") if p.is_file()}

    def test_roundtrip_check_keeps_bytes_mtimes_and_source_landmarks(self):
        original = self.source.read_bytes()
        self.cli()
        snapshot = self.snapshot()
        self.cli("--check")
        self.assertEqual(snapshot, self.snapshot())
        self.assertEqual(original, self.source.read_bytes())
        manifest = json.loads((self.root / "import-manifest.json").read_text())
        export = manifest["exports"][0]
        self.assertEqual(export["landmarks_source"], {"elbow": [5, 3]})
        self.assertEqual(export["landmarks_local"], {"elbow": [3, 2]})
        self.assertEqual(export["size"], [8, 6])
        self.assertEqual(export["sha256"], IMPORT.digest(self.root / "sprites/arm.png"))
        rgba = self.images.run(self.root / "sprites/arm.png", "-depth", "8", "rgba:-")
        # Source (9,3) is a black outline blended 50% over magenta. Keep its
        # soft alpha and remove the matte color, without eroding the contour.
        self.assertEqual(rgba[(2 * 8 + 7) * 4:(2 * 8 + 7) * 4 + 4], bytes([0, 0, 0, 127]))
        self.assertEqual(rgba[:4], bytes([0, 0, 0, 0]))

    def test_invalid_later_crop_cannot_partially_replace_exports(self):
        self.cli()
        original = (self.root / "sprites/arm.png").read_bytes()
        bad = copy.deepcopy(self.recipe["exports"][0])
        bad.update(id="bad", output="sprites/bad.png", crop=[11, 0, 2, 1], landmarks={})
        self.recipe["exports"].append(bad)
        self.save()
        self.assertIn("outside source bounds", self.cli(success=False).stderr)
        self.assertEqual(original, (self.root / "sprites/arm.png").read_bytes())
        self.assertFalse((self.root / "sprites/bad.png").exists())

    def test_empty_crop_aborts_all_exports_before_promotion(self):
        bad = copy.deepcopy(self.recipe["exports"][0])
        bad.update(id="empty", output="sprites/empty.png", crop=[0, 0, 2, 2], landmarks={})
        self.recipe["exports"].append(bad)
        self.save()
        self.assertIn("empty alpha", self.cli(success=False).stderr)
        self.assertFalse((self.root / "sprites").exists())

    def test_source_hash_lock_detects_a_retake(self):
        self.recipe["sources"]["sheet"]["sha256"] = IMPORT.digest(self.source)
        self.save()
        self.cli()
        self.images.run(self.source, "-fill", "white", "-draw", "point 3,3", self.source)
        snapshot = self.snapshot()
        self.assertIn("source changed", self.cli(success=False).stderr)
        self.assertEqual(snapshot, self.snapshot())

    def test_unlocked_retake_requires_intentional_import_before_check_passes(self):
        self.cli()
        self.images.run(self.source, "-fill", "white", "-draw", "point 3,3", self.source)
        snapshot = self.snapshot()
        self.cli("--check", success=False)
        self.assertEqual(snapshot, self.snapshot())
        self.cli()
        self.cli("--check")

    def test_output_and_manifest_tampering_are_detected_without_writes(self):
        self.cli()
        for target in [self.root / "sprites/arm.png", self.root / "import-manifest.json"]:
            original = target.read_bytes()
            target.write_bytes(original + b"tampered")
            snapshot = self.snapshot()
            self.cli("--check", success=False)
            self.assertEqual(snapshot, self.snapshot())
            target.write_bytes(original)

    def test_path_escape_absolute_and_source_alias_are_rejected(self):
        for output in ["../outside.png", "/tmp/outside.png", "source.png", "C:\\outside.png"]:
            with self.subTest(output=output):
                self.recipe["exports"][0]["output"] = output
                self.save()
                self.cli(success=False)
        self.assertFalse((self.root / "sprites").exists())

    def test_symlink_escape_is_rejected_for_sources_and_outputs(self):
        with tempfile.TemporaryDirectory() as outside:
            (self.root / "escape").symlink_to(outside, target_is_directory=True)
            self.recipe["exports"][0]["output"] = "escape/arm.png"
            self.save()
            self.assertIn("symlink escapes", self.cli(success=False).stderr)
            self.recipe["exports"][0]["output"] = "sprites/arm.png"
            self.recipe["sources"]["sheet"]["path"] = "escape/source.png"
            self.save()
            self.assertIn("symlink escapes", self.cli(success=False).stderr)

    def test_duplicate_outputs_and_ids_are_rejected(self):
        duplicate = copy.deepcopy(self.recipe["exports"][0])
        duplicate["id"] = "second"
        self.recipe["exports"].append(duplicate)
        self.save()
        self.assertIn("duplicate/reserved", self.cli(success=False).stderr)
        duplicate["output"] = "sprites/second.png"
        duplicate["id"] = "arm"
        self.save()
        self.assertIn("unique", self.cli(success=False).stderr)

    def test_landmarks_outside_crop_or_nonfinite_are_rejected(self):
        for point in [[1, 2], [4, float("nan")], [4, 90]]:
            self.recipe["exports"][0]["landmarks"]["elbow"] = point
            self.save()
            self.assertIn("landmark", self.cli(success=False).stderr)

    def test_alpha_validator_rejects_visible_magenta_residue(self):
        with self.assertRaisesRegex(IMPORT.ImportFailure, "magenta pixels remain"):
            self.images.alpha_summary(self.source, "magenta", [12, 8])

    def test_non_keyed_rgba_preserves_legitimate_purple_and_partial_alpha(self):
        self.images.run("-size", "12x8", "xc:rgba(255,0,255,0.5)", self.source)
        self.recipe["keying"] = "none"
        self.save()
        self.cli()
        rgba = self.images.run(self.root / "sprites/arm.png", "-depth", "8", "rgba:-")
        self.assertEqual(rgba[:3], bytes([255, 0, 255]))
        self.assertIn(rgba[3], (127, 128))
        self.cli("--check")

    def test_flat_key_preserves_jacket_purple_and_keeps_dark_outline_alpha(self):
        self.images.run(self.source, "-fill", "rgb(134,93,170)", "-draw", "point 4,3", self.source)
        self.recipe["keying"] = "magenta-flat"
        self.save()
        self.cli()
        rgba = self.images.run(self.root / "sprites/arm.png", "-depth", "8", "rgba:-")
        self.assertEqual(rgba[(2 * 8 + 2) * 4:(2 * 8 + 2) * 4 + 4], bytes([134, 93, 170, 255]))
        self.assertEqual(rgba[(2 * 8 + 7) * 4:(2 * 8 + 7) * 4 + 4], bytes([0, 0, 0, 127]))
        self.assertEqual(rgba[:4], bytes([0, 0, 0, 0]))
        self.cli("--check")

    def test_nested_output_file_conflicts_fail_before_promotion(self):
        duplicate = copy.deepcopy(self.recipe["exports"][0])
        duplicate.update(id="nested", output="sprites/arm.png/nested.png")
        self.recipe["exports"].append(duplicate)
        self.save()
        self.assertIn("parent path", self.cli(success=False).stderr)
        self.assertFalse((self.root / "sprites").exists())

    def test_flat_key_cannot_flood_purple_interior_through_a_gap_in_outline(self):
        self.images.run("-size", "12x8", "xc:magenta", "-fill", "black", "-draw",
                        "rectangle 2,1 9,6", "-fill", "rgb(134,93,170)", "-draw",
                        "rectangle 3,2 8,5", "-draw", "point 2,3", self.source)
        self.recipe["keying"] = "magenta-flat"
        self.save()
        self.cli()
        rgba = self.images.run(self.root / "sprites/arm.png", "-depth", "8", "rgba:-")
        self.assertEqual(rgba[(2 * 8 + 4) * 4:(2 * 8 + 4) * 4 + 4], bytes([134, 93, 170, 255]))


if __name__ == "__main__":
    unittest.main()
