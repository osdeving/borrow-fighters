#!/usr/bin/env python3
"""Capture and verify the EP's sky shot, continuous landing and impact-led panic.

Only the launched game window receives input. Screenshots/video are the native
game renderer; telemetry verifies input, clocks, camera, ground contact and retry.
"""
from __future__ import annotations

import argparse
import json
from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "tools/review"))
from capture_traffic_x11 import TrafficReview


class EpArrivalReview(TrafficReview):
    def ep_age(self, age):
        return self.wait(lambda s: s["ep_arrival"]["ticks"] is not None
                         and s["ep_arrival"]["ticks"] >= age, f"EP age {age}")

    def sequence(self):
        if self.args.preview_only:
            self.preview()
            return
        self.wait(lambda s: s["stage"] == "Encounter" and s["arrival_active"], "kite shot")
        self.tap("Return")
        self.wait(lambda s: not s["arrival_active"], "kite hands off")
        self.hold("d", True)
        first = self.wait(lambda s: s["ep_arrival"]["active"], "EP enters from above", 24.0)
        self.hold("d", False)
        first_frame = first["frame"]
        self.check("EP_descent_starts_before_panic_or_combat",
                   not first["enemy_awake"] and first["ambience"]["accident_ticks"] is None
                   and first["arrival_camera"]["zoom"] > 2.0, observed=first)
        for key in ("d", "j", "v", "space"):
            self.hold(key, True)
        sky = self.ep_age(12)
        self.release_all()
        self.check("cinematic_input_cannot_move_jump_or_damage",
                   sky["player"]["x"] == first["player"]["x"]
                   and sky["player"]["y"] == first["player"]["y"]
                   and sky["ticks"] == first["ticks"]
                   and sky["player"]["hp"] == 100 and sky["enemy"]["hp"] == 96, observed=sky)
        self.screenshot("ep-01-sky-body")
        self.tap("Escape")
        paused = self.wait(lambda s: s["paused"], "pause aerial shot")
        self.settle(.6)
        held = self.observe()
        self.check("pause_freezes_the_EP_camera_street_and_body",
                   all(held[key] == paused[key] for key in
                       ("ep_arrival", "arrival_camera", "ticks", "stage_ticks", "ambience")))
        self.screenshot("ep-02-paused-in-flight")
        self.tap("Return")
        self.wait(lambda s: not s["paused"], "resume EP descent")
        self.ep_age(30)
        self.screenshot("ep-03-opening-camera")
        wide = self.ep_age(first["ep_arrival"]["gameplay_tick"])
        self.check("gameplay_view_returns_while_the_same_EP_is_still_airborne",
                   wide["arrival_camera"] == {"x": 640.0, "y": 360.0, "zoom": 1.0}
                   and wide["ep_arrival"]["feet_y"] < 580.0
                   and not wide["enemy_awake"] and wide["ambience"]["accident_ticks"] is None,
                   observed=wide)
        self.screenshot("ep-04-gameplay-fall")
        impact_tick = first["ep_arrival"]["impact_tick"]
        impact = self.ep_age(impact_tick + 3)
        self.check("physical_ground_contact_starts_the_panic_and_kneeling_pose",
                   impact["ep_arrival"]["feet_y"] == 580.0 and impact["ep_arrival"]["pose"] == 6
                   and impact["enemy_awake"]
                   and impact["ambience"]["accident_ticks"] == impact["ep_arrival"]["impact_age"], observed=impact)
        self.screenshot("ep-05-impact-and-dust")
        self.ep_age(impact_tick + 28)
        self.screenshot("ep-06-grounded-recovery")
        playable = self.wait(lambda s: s["enemy_awake"] and not s["ep_arrival"]["active"], "landing recovery completed")
        self.check("landing_hands_back_a_healthy_encounter", playable["player"]["hp"] == 100
                   and playable["enemy"]["hp"] == 96 and playable["outcome"] == "Ongoing")
        rows = [row for row in self.telemetry.samples if first_frame <= row["frame"] <= playable["frame"]]
        failures = []
        for before, after in zip(rows, rows[1:]):
            dt = after["ep_arrival"]["ticks"] - before["ep_arrival"]["ticks"]
            dy = after["ep_arrival"]["feet_y"] - before["ep_arrival"]["feet_y"]
            if not -.001 <= dy <= dt * 14.0 + .001:
                failures.append(after["frame"])
            if after["ep_arrival"]["ticks"] < impact_tick and after["enemy_awake"]:
                failures.append(after["frame"])
        self.check("one_continuous_body_path_crosses_the_camera_handoff", not failures,
                   examined_samples=len(rows), failures=failures)
        self.hold("d", True)
        self.wait(lambda s: s["player"]["x"] > playable["player"]["x"] + 80.0, "movement restored")
        self.hold("d", False)
        self.wait(lambda s: s["outcome"] == "Defeat", "real defeat for checkpoint retry", 45.0)
        self.tap("r")
        retry = self.wait(lambda s: s["outcome"] == "Ongoing", "retry combat")
        self.check("retry_omits_both_cinematics_and_restores_health",
                   not retry["arrival_active"] and not retry["ep_arrival"]["active"]
                   and retry["enemy_awake"] and retry["player"]["hp"] == 100, observed=retry)
        self.screenshot("ep-07-awake-retry")
        self.tap("BackSpace")
        self.wait(lambda s: s["stage"] == "Complete", "full skip")
        self.tap("Return")
        self.wait(lambda s: s["stage"] == "AdaPrologue", "story restart")
        for _ in range(14):
            if self.observe()["stage"] == "Encounter":
                break
            self.tap("Return")
        self.wait(lambda s: s["stage"] == "Encounter", "street restarted")
        self.tap("Return")
        self.hold("d", True)
        self.wait(lambda s: s["ep_arrival"]["active"], "EP replay restored after restart", 24.0)
        self.hold("d", False)
        self.tap("Return")
        skipped = self.wait(lambda s: not s["ep_arrival"]["active"], "skip only EP landing")
        self.check("advance_during_EP_arrival_lands_safely_without_skipping_combat",
                   skipped["stage"] == "Encounter" and skipped["outcome"] == "Ongoing"
                   and skipped["enemy_awake"] and skipped["enemy"]["hp"] == 96
                   and skipped["player"]["hp"] == 100, observed=skipped)
        self.tap("Return")
        self.wait(lambda s: s["stage"] == "Opening", "subsequent advance reaches opening")

    def preview(self):
        """Keep the fast fall's real cadence free of screenshot or pause stalls."""
        self.wait(lambda s: s["stage"] == "Encounter" and s["arrival_active"], "kite shot")
        self.tap("Return")
        self.wait(lambda s: not s["arrival_active"], "kite hands off")
        self.hold("d", True)
        first = self.wait(lambda s: s["ep_arrival"]["active"], "fast EP approach", 24.0)
        self.hold("d", False)
        contact = self.wait(lambda s: s["enemy_awake"], "fast EP ground contact")
        elapsed = contact["seconds"] - first["seconds"]
        expected = (contact["ep_arrival"]["ticks"] - first["ep_arrival"]["ticks"]) / 60.0
        self.check("uninterrupted_fall_keeps_its_actual_fast_cadence",
                   abs(elapsed - expected) < .16 and expected < 1.1,
                   elapsed_seconds=elapsed, simulation_seconds=expected)
        self.wait(lambda s: s["ambience"]["accident_ticks"] is not None
                  and s["ambience"]["accident_ticks"] >= 165, "impact settles into combat")

    def cleanup(self):
        super().cleanup()
        path = self.directory / "native-checks.json"
        report = json.loads(path.read_text(encoding="utf-8"))
        report["review"] = "EP aerial arrival, continuous game-view fall, impact and checkpoint recovery"
        path.write_text(json.dumps(report, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--executable", default=str(ROOT / "target/debug/borrow-adventure"))
    parser.add_argument("--output-directory", required=True)
    parser.add_argument("--display", default=":0")
    parser.add_argument("--timeout", type=float, default=180.0)
    parser.add_argument("--mute", action="store_true")
    parser.add_argument("--preview-only", action="store_true", help="record an uninterrupted fall without pause/screenshot checks")
    review = EpArrivalReview(parser.parse_args())
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
