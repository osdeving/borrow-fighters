#!/usr/bin/env python3
"""Review the slow native street establishment and its covered cut to Rust.

Preview mode records uninterrupted cadence; functional mode checks input locks,
pause, local skip and restart through the owned window's normal commands.
"""
from __future__ import annotations

import argparse
import json
from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "tools/review"))
from capture_traffic_x11 import TrafficReview


class KiteArrivalReview(TrafficReview):
    def sequence(self):
        track = json.loads((ROOT / "assets/adventure/street/arrival-camera.json").read_text())
        (self.directory / "camera-track.json").write_text(json.dumps(track, indent=2) + "\n")
        duration = track["keys"][-1]["tick"]
        first = self.wait(lambda s: s["stage"] == "Encounter" and s["arrival_active"], "kite")
        self.check("starts_with_the_kite_and_a_calm_occupied_street",
                   first["ticks"] == 0 and first["arrival_camera"]["zoom"] > 3
                   and not first["enemy_awake"] and first["ambience"]["accident_ticks"] is None)
        if self.args.preview_only:
            handoff = self.wait(lambda s: not s["arrival_active"], "natural handoff", duration / 60 + 12)
            elapsed = handoff["seconds"] - first["seconds"]
            simulation = (handoff["stage_ticks"] - first["stage_ticks"]) / 60
            self.check("uninterrupted_establishment_preserves_authored_cadence",
                       abs(elapsed - simulation) < .18 and simulation >= duration / 60 - .1,
                       observed_seconds=elapsed, simulation_seconds=simulation)
        else:
            for key in ("d", "j", "v", "space"):
                self.hold(key, True)
            self.settle(.4)
            self.release_all()
            blocked = self.observe()
            self.check("early_input_cannot_move_attack_or_start_the_EP",
                       blocked["player"] == first["player"] and blocked["ticks"] == 0
                       and not blocked["enemy_awake"])
            self.wait(lambda s: s["stage_ticks"] >= 815, "fade before covered cut", 24)
            self.tap("Escape")
            paused = self.wait(lambda s: s["paused"], "pause during fade")
            self.settle(.6)
            held = self.observe()
            self.check("pause_freezes_camera_fade_and_ordinary_street_life",
                       all(held[key] == paused[key] for key in
                           ("stage_ticks", "ticks", "arrival_camera", "ambience", "neighborhood")))
            self.tap("Return")
            self.wait(lambda s: not s["paused"], "resume fade")
            self.tap("Return")
            handoff = self.wait(lambda s: not s["arrival_active"], "skip only street camera")
            self.check("local_skip_returns_to_exploration_without_panic_or_victory",
                       handoff["stage"] == "Encounter" and handoff["outcome"] == "Ongoing"
                       and not handoff["enemy_awake"] and handoff["player"]["hp"] == 100
                       and handoff["ambience"]["accident_ticks"] is None)

        rows = [s for s in self.telemetry.samples if s["arrival_active"]]
        self.check("ordinary_life_continues_while_combat_remains_suspended",
                   all(s["ticks"] == 0 and s["player"]["x"] == first["player"]["x"]
                       and not s["enemy_awake"] and s["ambience"]["accident_ticks"] is None
                       for s in rows)
                   and rows[-1]["ambience"]["ticks"] > rows[0]["ambience"]["ticks"],
                   observed_samples=len(rows))
        self.check("handoff_matches_the_exact_gameplay_camera",
                   handoff["arrival_camera"] == {"x": 640.0, "y": 360.0, "zoom": 1.0})
        self.hold("d", True)
        self.wait(lambda s: s["player"]["x"] > handoff["player"]["x"] + 90, "control released")
        self.hold("d", False)
        self.settle(.3)
        if not self.args.preview_only:
            self.tap("BackSpace")
            self.wait(lambda s: s["stage"] == "Complete", "skip full story")
            self.tap("Return")
            self.wait(lambda s: s["stage"] == "AdaPrologue", "restart story")
            for _ in range(14):
                if self.observe()["stage"] == "Encounter":
                    break
                self.tap("Return")
            restarted = self.wait(lambda s: s["stage"] == "Encounter", "replay street arrival")
            self.check("restart_restores_the_kite_without_replaying_a_panic",
                       restarted["arrival_active"] and restarted["arrival_camera"]["zoom"] > 3
                       and not restarted["enemy_awake"]
                       and restarted["ambience"]["accident_ticks"] is None)

    def cleanup(self):
        super().cleanup()
        path = self.directory / "native-checks.json"
        report = json.loads(path.read_text())
        report["review"] = "slow kite, child and neighbourhood establishment; covered cut to Rust"
        path.write_text(json.dumps(report, ensure_ascii=False, indent=2) + "\n")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--executable", default=str(ROOT / "target/debug/borrow-adventure"))
    parser.add_argument("--output-directory", required=True)
    parser.add_argument("--display", default=":0")
    parser.add_argument("--timeout", type=float, default=120)
    parser.add_argument("--mute", action="store_true")
    parser.add_argument("--preview-only", action="store_true")
    review = KiteArrivalReview(parser.parse_args())
    try:
        review.run()
    except (Exception, KeyboardInterrupt) as error:
        review.failure = f"{type(error).__name__}: {error}"
        print(review.failure, file=sys.stderr, flush=True)
    finally:
        review.cleanup()
    return int(review.failure is not None)


if __name__ == "__main__":
    raise SystemExit(main())
