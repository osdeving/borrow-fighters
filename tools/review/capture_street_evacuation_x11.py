#!/usr/bin/env python3
"""Verify evacuation, abandoned bicycles and a permanently empty street natively.

Reuses the owned-window traffic harness. Only the newly launched game receives
synthetic input; telemetry supplies simulation evidence and F12 supplies game
frames. Native sound stays enabled unless --mute is requested. The built-in
video is silent; an outer recorder may capture an isolated game audio sink.
"""
from __future__ import annotations

import argparse
import json
from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "tools/review"))
from capture_traffic_x11 import TrafficReview


class EvacuationReview(TrafficReview):
    def pause_dismount(self):
        self.tap("Escape")
        paused = self.wait(lambda s: s["paused"], "pause during dismount")
        self.check("both_cyclists_dismount_before_running",
                   all(actor["phase"] == "Dismounting" for actor in paused["ambience"]["cyclists"]),
                   observed=paused)
        self.settle(.7)
        held = self.observe()
        keys = ("stage_ticks", "ticks", "player", "enemy", "ambience")
        self.check("pause_freezes_every_evacuation_actor_and_clock",
                   held["paused"] and all(held[key] == paused[key] for key in keys),
                   before=paused["ambience"], after=held["ambience"])
        self.screenshot("evacuation-04-dismount-paused")
        self.tap("Return")
        resumed = self.wait(lambda s: not s["paused"], "resume dismount")
        self.check("resume_retains_the_original_evacuation",
                   resumed["stage"] == "Encounter"
                   and resumed["ambience"]["accident_ticks"] >= held["ambience"]["accident_ticks"],
                   observed=resumed)
        self.screenshot("evacuation-05-dismount-resumed")

    def assert_empty(self, sample, name):
        ambience = sample["ambience"]
        self.check(name, all(not actor["visible"] for actor in ambience["traffic_cars"])
                   and all(not actor["visible"] and actor["phase"] == "Gone" for actor in ambience["cyclists"])
                   and ambience["kid_phase"] == "Gone"
                   and ambience["incident_car"]["phase"] == "Crashed"
                   and all(bike is not None for bike in ambience["abandoned_bicycles"]), observed=sample)

    def check_continuity(self, first_frame, last_frame):
        samples = [row for row in self.telemetry.samples
                   if first_frame <= row["frame"] <= last_frame and row["stage"] == "Encounter"]
        failures = []
        phases = {0: set(), 1: set()}
        for previous, current in zip(samples, samples[1:]):
            before, after = previous["ambience"], current["ambience"]
            age = after["accident_ticks"]
            if age is None:
                continue
            elapsed = after["ticks"] - before["ticks"]
            for old, car in zip(before["traffic_cars"], after["traffic_cars"]):
                distance = old["x"] - car["x"]
                if before["accident_ticks"] is None:
                    # An ordinary wrap may have happened before the first alarm
                    # within this telemetry interval; it is forbidden afterward.
                    distance %= 2720.0
                if not -.02 <= distance <= 11.0 * elapsed + .02:
                    failures.append({"kind": "car_discontinuity", "frame": current["frame"],
                                     "style": car["style"], "distance": distance, "elapsed": elapsed})
                if not old["visible"] and car["visible"]:
                    failures.append({"kind": "car_respawn", "frame": current["frame"], "style": car["style"]})
            for old, cyclist in zip(before["cyclists"], after["cyclists"]):
                phases[cyclist["id"]].add(cyclist["phase"])
                distance = abs(cyclist["x"] - old["x"])
                if before["accident_ticks"] is None:
                    distance = min(distance, abs(2520.0 - distance))
                if distance > 8.0 * elapsed + .03:
                    failures.append({"kind": "cyclist_discontinuity", "frame": current["frame"],
                                     "id": cyclist["id"], "distance": distance, "elapsed": elapsed})
                if not old["visible"] and cyclist["visible"]:
                    failures.append({"kind": "cyclist_respawn", "frame": current["frame"], "id": cyclist["id"]})
            for old, bike in zip(before["abandoned_bicycles"], after["abandoned_bicycles"]):
                if old is not None and (bike is None or (old["x"], old["y"]) != (bike["x"], bike["y"])):
                    failures.append({"kind": "abandoned_bicycle_moved", "frame": current["frame"]})
        self.check("evacuation_positions_are_continuous_and_never_wrap_or_respawn", not failures,
                   examined_samples=len(samples), failures=failures[:8])
        expected = {"Braking", "Dismounting", "Running", "Gone"}
        self.check("both_cyclists_complete_every_reaction_phase",
                   all(expected.issubset(observed) for observed in phases.values()),
                   phases={str(actor): sorted(observed) for actor, observed in phases.items()})

    def sequence(self):
        calm = self.wait(lambda s: s["stage"] == "Encounter" and s["ticks"] >= 60, "calm street", 15.0)
        self.check("calm_street_contains_five_vehicle_types_and_two_riders",
                   not calm["enemy_awake"]
                   and sorted(car["style"] for car in calm["ambience"]["traffic_cars"]) == [0, 1, 2, 3, 4]
                   and all(not car["fleeing"] for car in calm["ambience"]["traffic_cars"])
                   and len(calm["ambience"]["cyclists"]) == 2
                   and all(actor["phase"] == "Riding" for actor in calm["ambience"]["cyclists"]), observed=calm)
        self.screenshot("evacuation-01-calm")
        later = self.wait(lambda s: s["ticks"] >= calm["ticks"] + 120, "ordinary traffic moves")
        self.check("traffic_moves_before_the_threat",
                   [car["x"] for car in calm["ambience"]["traffic_cars"]]
                   != [car["x"] for car in later["ambience"]["traffic_cars"]]
                   and later["ambience"]["accident_ticks"] is None)
        self.hold("a", True)
        self.wait(lambda s: s["player"]["x"] <= 100, "left camera edge")
        self.hold("a", False)
        self.screenshot("evacuation-02-left-camera")
        self.hold("d", True)
        quiet = self.wait(lambda s: s["player"]["x"] >= 950, "approach EP", 12.0)
        self.hold("d", False)
        self.check("all_actors_remain_calm_before_the_EP", quiet["ambience"]["accident_ticks"] is None)
        self.screenshot("evacuation-03-before-EP")
        first_frame = self.observe()["frame"]
        self.hold("d", True)
        awake = self.wait(lambda s: s["enemy_awake"], "EP begins collective evacuation")
        self.hold("d", False)
        self.hold("q", True)
        self.check("EP_reacts_in_all_vehicle_and_cyclist_states",
                   all(car["fleeing"] for car in awake["ambience"]["traffic_cars"])
                   and all(actor["phase"] == "Braking" for actor in awake["ambience"]["cyclists"])
                   and awake["ambience"]["kid_phase"] != "Playing", observed=awake)
        self.accident_age(25, "cyclists leave their bicycles")
        self.pause_dismount()
        running = self.accident_age(60, "former cyclists run away")
        self.check("both_cyclists_run_and_leave_two_bicycles",
                   all(actor["phase"] == "Running" for actor in running["ambience"]["cyclists"])
                   and all(bike is not None for bike in running["ambience"]["abandoned_bicycles"]), observed=running)
        self.screenshot("evacuation-06-running-and-falling-bikes")
        self.accident_age(114, "preserved pole impact")
        self.screenshot("evacuation-07-car-impact")
        empty = self.accident_age(360, "street empties")
        self.assert_empty(empty, "all_moving_background_actors_leave_by_six_seconds")
        self.screenshot("evacuation-08-empty-street")
        right = self.guarded_right_camera()
        self.hold("q", True)
        self.assert_empty(right, "right_camera_contains_debris_without_new_traffic")
        self.screenshot("evacuation-09-empty-right-camera")
        persistent = self.wait(lambda s: s["ambience"]["accident_ticks"] >= 1200,
                               "long encounter remains empty", 24.0)
        self.assert_empty(persistent, "street_stays_empty_after_twenty_seconds")
        for field in ("traffic_cars", "cyclists", "abandoned_bicycles"):
            self.check(field + "_positions_remain_fixed_after_evacuation",
                       [(actor["x"], actor["y"]) for actor in empty["ambience"][field]]
                       == [(actor["x"], actor["y"]) for actor in persistent["ambience"][field]])
        self.screenshot("evacuation-10-persistent-empty-street")
        self.check_continuity(first_frame, persistent["frame"])

        self.hold("q", False)
        self.log("waiting_for_unprotected_defeat")
        self.wait(lambda s: s["outcome"] == "Defeat", "real defeat before retry", 40.0)
        self.tap("r")
        retry = self.wait(lambda s: s["outcome"] == "Ongoing" and s["player"]["hp"] == 100,
                          "retry checkpoint")
        self.hold("q", True)
        self.check("retry_rearms_all_actors_and_removes_abandoned_bikes",
                   retry["enemy_awake"] and retry["ambience"]["accident_ticks"] < 24
                   and all(actor["phase"] == "Braking" for actor in retry["ambience"]["cyclists"])
                   and all(bike is None for bike in retry["ambience"]["abandoned_bicycles"]), observed=retry)
        self.screenshot("evacuation-11-retry")
        self.accident_age(60, "retry dismounts again")
        self.tap("Return")
        skipped = self.wait(lambda s: s["stage"] == "Opening", "skip ongoing evacuation")
        self.hold("q", False)
        self.settle(.5)
        frozen = self.observe()
        self.check("skip_freezes_the_abandoned_evacuation_without_victory",
                   skipped["outcome"] == "Ongoing" and frozen["ambience"] == skipped["ambience"])
        self.tap("Escape")
        self.wait(lambda s: s["paused"], "opening pauses")
        self.tap("BackSpace")
        complete = self.wait(lambda s: s["stage"] == "Complete", "full skip from pause")
        self.check("full_skip_finishes_without_a_combat_victory",
                   not complete["paused"] and complete["outcome"] == "Ongoing")
        self.tap("Return")
        self.wait(lambda s: s["stage"] == "AdaPrologue", "restart entire story")
        for _ in range(14):
            if self.observe()["stage"] == "Encounter":
                break
            self.tap("Return")
        restarted = self.wait(lambda s: s["stage"] == "Encounter", "restarted calm street")
        self.check("restart_restores_everyday_traffic_and_clears_all_debris",
                   not restarted["enemy_awake"] and restarted["ambience"]["incident_car"] is None
                   and all(actor["phase"] == "Riding" for actor in restarted["ambience"]["cyclists"])
                   and all(not car["fleeing"] for car in restarted["ambience"]["traffic_cars"])
                   and all(bike is None for bike in restarted["ambience"]["abandoned_bicycles"]), observed=restarted)
        self.screenshot("evacuation-12-restarted-calm")

    def cleanup(self):
        super().cleanup()
        report_path = self.directory / "native-checks.json"
        report = json.loads(report_path.read_text(encoding="utf-8"))
        report["review"] = "collective street evacuation and persistent abandoned objects"
        report_path.write_text(json.dumps(report, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--executable", default=str(ROOT / "target/debug/borrow-adventure"))
    parser.add_argument("--output-directory", required=True)
    parser.add_argument("--display", default=":0")
    parser.add_argument("--timeout", type=float, default=240.0)
    parser.add_argument("--mute", action="store_true")
    review = EvacuationReview(parser.parse_args())
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
