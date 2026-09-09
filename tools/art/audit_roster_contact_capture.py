#!/usr/bin/env python3
"""Check exported World contact clocks and retain a compact evidence summary."""

import argparse
import hashlib
import json
import struct
from pathlib import Path


DEFENDERS = {"rust", "duke", "c", "go"}
PROFILES = {"Head", "Body", "Low", "GuardHigh", "GuardLow", "Launch", "Fall", "Rise"}
CASES = {"body", "head", "low", "guard-high", "guard-low", "throw", "cinematic"}
CLIPS = {"Head": "reaction_head", "Body": "reaction_body", "Low": "reaction_low",
         "GuardHigh": "reaction_guard_high", "GuardLow": "reaction_guard_low",
         "Launch": "reaction_launch", "Fall": "reaction_fall", "Rise": "reaction_rise"}
ROOT = Path(__file__).resolve().parents[2]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--input", type=Path, action="append", required=True, help="Directory or exported roster-contact-review.json")
    parser.add_argument("--report", type=Path, required=True)
    args = parser.parse_args()
    coverage = {}
    drawing_coverage = {}
    complete_cases = set()
    manifests = {}
    for defender in DEFENDERS:
        path = ROOT / f"assets/candidates/{defender}/{defender}-fighter.sprite.json"
        manifest = json.loads(path.read_text())
        manifests[defender] = {"sha256": hashlib.sha256(path.read_bytes()).hexdigest(),
                              "clips": {clip["name"]: clip["frames"] for clip in manifest["clips"]}}
    scenarios = []
    windows = []
    screenshots = 0
    for source in args.input:
        path = source / "roster-contact-review.json" if source.is_dir() else source
        report = json.loads(path.read_text())
        assert report["fps"] == 60 and report["render_size"] == [1280, 720]
        assert report["source"] == "World::update_with_flags + render::draw_fight"
        for scenario in report["scenarios"]:
            defender = scenario["defender"]
            assert defender in DEFENDERS
            reverse = scenario["reverse"]
            protected = scenario["protected_health"]
            observations = scenario["observations"]
            assert observations, (path, defender, scenario["case"])
            if protected:
                assert scenario["initial_health"] == scenario["final_health"] == 1
                assert all(row["health"] == 1 and row["damage"] == 0 for row in observations)
            else:
                assert scenario["final_health"] < scenario["initial_health"]
            profiles = {row["profile"] for row in observations}
            key = (defender, reverse, protected)
            coverage.setdefault(key, set()).update(profiles)
            if not report["barrage_only"]:
                case_key = (*key, scenario["case"])
                assert case_key not in complete_cases, ("duplicate complete scenario", case_key)
                complete_cases.add(case_key)
            assert profiles <= PROFILES
            for row in observations:
                assert row["clip"] == CLIPS[row["profile"]], row
                assert row["frame"] in manifests[defender]["clips"][row["clip"]], row
                assert row["duration_frames"] > 0
                drawing_coverage.setdefault((*key, row["profile"]), set()).add(row["frame"])
                # Throw capture lifts the body before begin_launch changes the
                # grounded flag. Floor support applies once that capture ends.
                if row["grounded"] and not row["captured"]:
                    assert abs(row["feet_y"] - report["floor_y"]) < 0.01, row
            for item in scenario["screenshots"]:
                image = path.parent / item["image"]
                with image.open("rb") as stream:
                    header = stream.read(24)
                assert header[:8] == b"\x89PNG\r\n\x1a\n", image
                assert struct.unpack(">II", header[16:24]) == (1280, 720), image
                screenshots += 1
            if scenario["case"] == "cinematic":
                ticks = {row["sequence_tick"]: row for row in observations if row["sequence_tick"] is not None}
                for impact in range(344, 424, 10):
                    rows = [ticks[tick] for tick in range(impact, impact + 10)]
                    assert all(row["profile"] == "Head" and row["clip"] == "reaction_head" for row in rows)
                    assert abs(rows[0]["age_frames"]) < 0.001
                    assert all(abs(row["duration_frames"] - 9) < 0.001 for row in rows)
                    frames = list(dict.fromkeys(row["frame"] for row in rows))
                    assert len(frames) == 4, (defender, reverse, protected, impact, frames)
                    assert rows[-1]["frame"] == frames[-1]
                    assert rows[0]["damage"] == (0 if protected else 3)
                    windows.append({"defender": defender, "reverse": reverse, "protected_health": protected,
                                    "impact_tick": impact, "frames": frames, "duration_frames": 9})
                captured_barrage = [item for item in scenario["screenshots"]
                                    if item["sequence_tick"] is not None and 344 <= item["sequence_tick"] < 424]
                assert [item["sequence_tick"] for item in captured_barrage] == list(range(344, 424))
                for item in captured_barrage:
                    assert item["frame"] == ticks[item["sequence_tick"]]["frame"], item
            if scenario["case"] == "knockout":
                assert not protected and scenario["final_health"] == 0
                # A KO is only demonstrated if the last pose remains on the
                # physical floor for a full second, after the sequence ends.
                final_rows = observations[-60:]
                assert len(final_rows) == 60
                assert [row["tick"] for row in final_rows] == list(range(final_rows[0]["tick"], final_rows[0]["tick"] + 60))
                for row in final_rows:
                    assert row["sequence_tick"] is None and row["grounded"], row
                    assert row["profile"] == "Fall" and row["frame"] == manifests[defender]["clips"]["reaction_fall"][-1], row
            scenarios.append({"defender": defender, "case": scenario["case"], "reverse": reverse,
                              "protected_health": protected, "profiles": sorted(profiles),
                              "first_contact_tick": scenario["first_contact_tick"],
                              "initial_health": scenario["initial_health"], "final_health": scenario["final_health"],
                              "screenshots": len(scenario["screenshots"])})
    for defender in DEFENDERS:
        for reverse in (False, True):
            for protected in (False, True):
                key = (defender, reverse, protected)
                assert coverage.get(key) == PROFILES, (key, coverage.get(key))
                for case in CASES | ({"knockout"} if not protected else set()):
                    assert (*key, case) in complete_cases, ("missing complete scenario", *key, case)
                for profile in PROFILES:
                    expected = set(manifests[defender]["clips"][CLIPS[profile]])
                    assert drawing_coverage.get((*key, profile)) == expected, (key, profile, drawing_coverage.get((*key, profile)), expected)
    summary = {
        "method": "Real World contact exports and unmodified PNG headers; visual articulation is reviewed separately",
        "inputs": [str(path) for path in args.input], "scenarios": scenarios,
        "scenarios_checked": len(scenarios), "screenshots_checked": screenshots,
        "barrage_windows_checked": len(windows), "barrage_windows": windows,
        "manifest_sha256": {defender: manifest["sha256"] for defender, manifest in sorted(manifests.items())},
        "profile_coverage": [{"defender": defender, "reverse": reverse, "protected_health": protected, "profiles": sorted(profiles)}
                             for (defender, reverse, protected), profiles in sorted(coverage.items())],
    }
    args.report.parent.mkdir(parents=True, exist_ok=True)
    args.report.write_text(json.dumps(summary, ensure_ascii=False, indent=2) + "\n")
    print(f"PASS: {len(scenarios)} scenarios, {screenshots} PNGs, {len(windows)} complete barrage windows; all 32 drawings for 4 defenders in both directions with damage on/off; sustained floor KO")


if __name__ == "__main__":
    main()
