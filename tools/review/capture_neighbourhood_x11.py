#!/usr/bin/env python3
"""Review the cinematic street arrival, neighbours taking shelter and shop closure.

Synthetic input targets only the newly launched game window. Camera, control,
resident and dog checks use its telemetry; screenshots come from its own F12
capture. Native audio is enabled unless --mute is supplied, while the game's
video recorder remains silent and may be paired with an external isolated sink.
"""
from __future__ import annotations

import argparse
import json
from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "tools/review"))
from capture_street_evacuation_x11 import EvacuationReview

# Public scene dimensions/timings mirrored from adventure::neighborhood. The
# wider portal makes the short movement behind its jamb faster than the run.
DOOR_CENTER_X = 603.0
DOOR_WIDTH = 97.0
ENTERING_TICKS = 24
RESIDENT_MAX_STEP = max(260.0 / 60.0, (DOOR_WIDTH / 2.0 + 60.0) / ENTERING_TICKS)


class NeighbourhoodReview(EvacuationReview):
    def pause_scene(self, label):
        self.tap("Escape")
        paused = self.wait(lambda s: s["paused"], label + " paused")
        self.settle(.65)
        held = self.observe()
        keys = ("stage_ticks", "ticks", "player", "enemy", "ambience", "neighborhood",
                "arrival_active", "arrival_camera")
        self.check(label + "_pause_freezes_camera_world_and_neighbours",
                   held["paused"] and all(paused[key] == held[key] for key in keys),
                   before={key: paused[key] for key in ("stage_ticks", "neighborhood", "arrival_camera")},
                   after={key: held[key] for key in ("stage_ticks", "neighborhood", "arrival_camera")})
        self.screenshot(label + "-paused")
        self.tap("Return")
        resumed = self.wait(lambda s: not s["paused"], label + " resumed")
        self.check(label + "_resume_keeps_the_same_scene", resumed["stage"] == "Encounter"
                   and resumed["arrival_active"] == paused["arrival_active"], observed=resumed)
        return resumed

    def arrival(self):
        initial = self.wait(lambda s: s["stage"] == "Encounter" and s["arrival_active"],
                            "cinematic street arrival", 15.0)
        first_frame = initial["frame"]
        self.check("arrival_starts_on_the_kite_before_control",
                   initial["ticks"] == 0 and initial["arrival_camera"]["zoom"] > 1.0
                   and initial["player"]["x"] == 340.0 and not initial["enemy_awake"], observed=initial)
        self.screenshot("arrival-01-kite")
        before_input = self.observe()
        for key in ("d", "j", "space"):
            self.hold(key, True)
        blocked = self.wait(lambda s: s["stage_ticks"] >= before_input["stage_ticks"] + 55,
                            "early movement and attack input")
        self.release_all()
        self.check("early_movement_jump_and_attack_are_blocked_while_the_street_stays_alive",
                   blocked["arrival_active"] and blocked["ticks"] == 0
                   and blocked["player"] == initial["player"] and blocked["enemy"]["hp"] == 96
                   and blocked["ambience"]["ticks"] > initial["ambience"]["ticks"]
                   and blocked["ambience"]["traffic_cars"] != initial["ambience"]["traffic_cars"], observed=blocked)
        self.wait(lambda s: s["stage_ticks"] >= 180, "descending camera")
        self.screenshot("arrival-02-descent")
        self.pause_scene("arrival-03-descent")
        settled = self.wait(lambda s: s["stage"] == "Encounter" and not s["arrival_active"],
                            "automatic delivery of control", 12.0)
        self.check("automatic_arrival_ends_at_exact_playable_camera",
                   settled["arrival_camera"] == {"x": 640.0, "y": 360.0, "zoom": 1.0}
                   and not settled["enemy_awake"] and settled["player"]["x"] == 340.0, observed=settled)
        samples = [row for row in self.telemetry.samples if first_frame <= row["frame"] <= settled["frame"]]
        continuous = True
        for before, after in zip(samples, samples[1:]):
            elapsed = after["stage_ticks"] - before["stage_ticks"]
            for key, limit in (("x", 5.0), ("y", 5.0), ("zoom", .03)):
                continuous &= abs(after["arrival_camera"][key] - before["arrival_camera"][key]) <= elapsed * limit + .001
        self.check("arrival_camera_moves_continuously_through_pause_and_handoff", continuous,
                   examined_samples=len(samples))
        self.hold("d", True)
        moved = self.wait(lambda s: s["player"]["x"] >= 400.0, "control moves Rust after arrival")
        self.hold("d", False)
        self.check("movement_is_released_after_the_camera_settles", moved["ticks"] > 0)
        self.screenshot("arrival-04-playable-neighbourhood")

    def check_sheltering_continuity(self, first_frame, last_frame):
        rows = [row for row in self.telemetry.samples
                if first_frame <= row["frame"] <= last_frame and row["stage"] == "Encounter"]
        failures = []
        entered = set()
        for before, after in zip(rows, rows[1:]):
            age = after["ambience"]["accident_ticks"]
            if age is None:
                continue
            elapsed = after["ambience"]["ticks"] - before["ambience"]["ticks"]
            residents = after["neighborhood"]["residents"]
            for old, resident in zip(before["neighborhood"]["residents"], residents):
                distance = old["x"] - resident["x"]
                if not -.02 <= distance <= RESIDENT_MAX_STEP * elapsed + .02:
                    failures.append({"kind": "resident_jump", "id": resident["id"], "frame": after["frame"]})
                if abs(old["y"] - resident["y"]) > 7.0 / ENTERING_TICKS * elapsed + .02:
                    failures.append({"kind": "resident_depth_jump", "id": resident["id"], "frame": after["frame"]})
                if resident["phase"] == "Entering":
                    entered.add(resident["id"])
                    if resident["x"] > DOOR_CENTER_X + .01:
                        failures.append({"kind": "entry_before_door", "id": resident["id"]})
                if resident["id"] != 3 and not resident["visible"] and resident["id"] not in entered:
                    failures.append({"kind": "resident_disappeared_before_entry", "id": resident["id"]})
                if not old["visible"] and resident["visible"]:
                    failures.append({"kind": "resident_respawn", "id": resident["id"]})
            if after["neighborhood"]["shutter"]["phase"] != "Open":
                if any(resident["visible"] for resident in residents if resident["id"] != 3):
                    failures.append({"kind": "door_closed_before_last_visitor", "frame": after["frame"]})
            old_dog, dog = before["neighborhood"]["dog"], after["neighborhood"]["dog"]
            maximum = .23 if before["ambience"]["accident_ticks"] is None else 6.0
            if abs(old_dog["x"] - dog["x"]) > maximum * elapsed + .02:
                failures.append({"kind": "dog_discontinuity", "frame": after["frame"]})
            if not old_dog["visible"] and dog["visible"]:
                failures.append({"kind": "dog_respawn", "frame": after["frame"]})
        self.check("late_EP_keeps_continuous_paths_and_the_door_waits_for_every_visitor",
                   not failures and entered == {0, 1, 2}, examined_samples=len(rows),
                   entered_residents=sorted(entered), failures=failures[:8])

    def sequence(self):
        self.arrival()
        calm = self.observe()
        self.check("calm_street_has_four_residents_caramelo_and_five_vehicle_types",
                   len(calm["neighborhood"]["residents"]) == 4
                   and all(person["visible"] and person["phase"] == "Idle" for person in calm["neighborhood"]["residents"])
                   and calm["neighborhood"]["dog"]["visible"]
                   and calm["neighborhood"]["shutter"]["phase"] == "Open"
                   and sorted(car["style"] for car in calm["ambience"]["traffic_cars"]) == [0, 1, 2, 3, 4])
        self.hold("d", True)
        self.wait(lambda s: s["player"]["x"] >= 1030.0, "calm shop and bus stop", 12.0)
        self.hold("d", False)
        self.screenshot("neighbours-01-shop-bus-stop-caramelo")
        before_alarm = self.observe()
        self.check("the_EP_is_delayed_until_after_the_calm_neighbourhood_view",
                   not before_alarm["enemy_awake"] and before_alarm["ambience"]["ticks"] > 360)
        first_frame = before_alarm["frame"]
        self.hold("d", True)
        alarm = self.wait(lambda s: s["enemy_awake"], "residents react to the EP")
        self.hold("d", False)
        self.hold("q", True)
        self.check("residents_and_dog_notice_the_same_EP",
                   alarm["neighborhood"]["dog"]["phase"] == "Startled"
                   and all(person["phase"] in ("Startled", "Waiting") for person in alarm["neighborhood"]["residents"]),
                   observed=alarm)
        self.accident_age(32, "caramelo runs safely away")
        self.screenshot("neighbours-02-caramelo-escape")
        self.accident_age(160, "bus-stop residents reach the shop")
        self.screenshot("neighbours-03-running-to-shelter")
        entering = self.accident_age(234, "last visitor enters through the doorway")
        self.check("last_visitor_reaches_the_door_while_the_shopkeeper_waits",
                   entering["neighborhood"]["residents"][1]["phase"] == "Entering"
                   and entering["neighborhood"]["residents"][3]["phase"] == "Waiting"
                   and entering["neighborhood"]["shutter"]["phase"] == "Open", observed=entering)
        self.screenshot("neighbours-04-last-visitor-entering")
        closing = self.accident_age(292, "shopkeeper lowers the shutter")
        self.check("shopkeeper_pulls_only_after_every_visitor_is_inside",
                   closing["neighborhood"]["shutter"]["phase"] == "Closing"
                   and all(not person["visible"] for person in closing["neighborhood"]["residents"][:3])
                   and closing["neighborhood"]["residents"][3]["phase"] == "Closing"
                   and closing["neighborhood"]["residents"][3]["visible"], observed=closing)
        self.pause_scene("neighbours-05-shutter")
        self.screenshot("neighbours-06-shutter-lowering")
        closed = self.accident_age(330, "shutter contacts the threshold")
        self.check("closed_shutter_covers_the_shopkeeper_and_everyone_is_safe",
                   closed["neighborhood"]["shutter"]["phase"] == "Closed"
                   and closed["neighborhood"]["shutter"]["progress"] == 1.0
                   and all(not person["visible"] for person in closed["neighborhood"]["residents"])
                   and not closed["neighborhood"]["dog"]["visible"], observed=closed)
        self.screenshot("neighbours-07-closed-shop")
        self.accident_age(360, "street traffic finishes evacuating")
        right = self.guarded_right_camera()
        self.hold("q", True)
        self.assert_empty(right, "right_camera_shows_the_empty_street")
        self.screenshot("neighbours-08-right-camera")
        persistent = self.wait(lambda s: s["ambience"]["accident_ticks"] >= 1200,
                               "shop stays closed in a long encounter", 24.0)
        self.check("residents_dog_and_traffic_never_return_and_the_shop_stays_closed",
                   persistent["neighborhood"]["shutter"]["phase"] == "Closed"
                   and persistent["neighborhood"]["shutter"]["progress"] == 1.0
                   and all(not person["visible"] for person in persistent["neighborhood"]["residents"])
                   and not persistent["neighborhood"]["dog"]["visible"]
                   and all(not car["visible"] for car in persistent["ambience"]["traffic_cars"]), observed=persistent)
        self.check_sheltering_continuity(first_frame, persistent["frame"])
        self.hold("q", False)
        self.wait(lambda s: s["outcome"] == "Defeat", "real defeat before retry", 40.0)
        self.tap("r")
        retry = self.wait(lambda s: s["outcome"] == "Ongoing" and s["player"]["hp"] == 100, "awake retry")
        self.hold("q", True)
        self.check("awake_retry_skips_arrival_and_rearms_the_open_shop",
                   not retry["arrival_active"] and retry["enemy_awake"]
                   and retry["arrival_camera"] == {"x": 640.0, "y": 360.0, "zoom": 1.0}
                   and retry["neighborhood"]["shutter"]["phase"] == "Open"
                   and all(person["visible"] for person in retry["neighborhood"]["residents"]), observed=retry)
        self.screenshot("neighbours-09-awake-retry")
        self.accident_age(100, "retry sheltering begins again")
        self.tap("Return")
        skipped = self.wait(lambda s: s["stage"] == "Opening", "skip active encounter")
        self.hold("q", False)
        self.settle(.5)
        self.check("encounter_skip_freezes_sheltering_without_fabricating_victory",
                   skipped["outcome"] == "Ongoing" and self.observe()["neighborhood"] == skipped["neighborhood"])
        self.tap("Escape")
        self.wait(lambda s: s["paused"], "pause opening")
        self.tap("BackSpace")
        self.wait(lambda s: s["stage"] == "Complete", "full skip")
        self.tap("Return")
        self.wait(lambda s: s["stage"] == "AdaPrologue", "restart story")
        for _ in range(14):
            if self.observe()["stage"] == "Encounter":
                break
            self.tap("Return")
        restarted = self.wait(lambda s: s["stage"] == "Encounter", "arrival after full restart")
        self.check("full_restart_restores_the_arrival_caramelo_and_open_shop",
                   restarted["arrival_active"] and not restarted["enemy_awake"]
                   and restarted["neighborhood"]["dog"]["visible"]
                   and restarted["neighborhood"]["shutter"]["phase"] == "Open", observed=restarted)
        self.tap("Return")
        advanced = self.wait(lambda s: not s["arrival_active"], "explicit advance ends the arrival")
        self.check("Return_during_arrival_releases_exploration_without_skipping_the_encounter",
                   advanced["stage"] == "Encounter" and advanced["outcome"] == "Ongoing"
                   and not advanced["enemy_awake"]
                   and advanced["arrival_camera"] == {"x": 640.0, "y": 360.0, "zoom": 1.0}
                   and advanced["ambience"]["ticks"] < 360, observed=advanced)
        self.hold("d", True)
        self.wait(lambda s: s["player"]["x"] >= 400, "movement after explicitly advancing arrival")
        self.hold("d", False)
        self.screenshot("neighbours-10-restarted-exploration")

    def cleanup(self):
        super().cleanup()
        path = self.directory / "native-checks.json"
        report = json.loads(path.read_text(encoding="utf-8"))
        report["review"] = "cinematic arrival, sheltering neighbours, caramelo and rolling shop shutter"
        path.write_text(json.dumps(report, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--executable", default=str(ROOT / "target/debug/borrow-adventure"))
    parser.add_argument("--output-directory", required=True)
    parser.add_argument("--display", default=":0")
    parser.add_argument("--timeout", type=float, default=240.0)
    parser.add_argument("--mute", action="store_true")
    review = NeighbourhoodReview(parser.parse_args())
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
