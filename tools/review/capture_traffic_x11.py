#!/usr/bin/env python3
"""Review native traffic and the pole accident through the owned-window X11 harness.

The game emits its own audio unless --mute is supplied. Its built-in recorder
still writes silent video; an outer PulseAudio recorder may capture an isolated
sink for synchronization. Screenshots and checks use observed simulation clocks,
not a scripted replacement for game state or a capture of the desktop.
"""
from __future__ import annotations

import argparse
import json
import os
from pathlib import Path
import subprocess
import sys
import time

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "tools/review"))
from capture_adventure_x11 import AdventureReview, X11


class TrafficReview(AdventureReview):
    def run(self):
        self.x11 = X11(self.args.display)
        env = os.environ.copy()
        env["DISPLAY"] = self.args.display
        env["BORROW_FIGHTERS_DATA_DIR"] = str(self.userdata)
        env["BORROW_FIGHTERS_ASSET_DIR"] = str(self.root / "assets")
        command = [str(self.binary), "--capture", str(self.directory),
                   "--start", "encounter", "--hidden", "--texts", str(self.catalog_path)]
        if self.args.mute:
            command.append("--mute")
        with (self.directory / "game.log").open("w", encoding="utf-8") as game_log:
            self.process = subprocess.Popen(command, cwd=self.directory, env=env,
                                            stdout=game_log, stderr=subprocess.STDOUT)
            self.log("started", pid=self.process.pid, command=command, display=self.args.display)
            deadline = time.monotonic() + 12.0
            while self.window is None and time.monotonic() < deadline:
                self.alive()
                self.window = self.x11.find_window(self.process.pid)
                time.sleep(0.04)
            if self.window is None:
                raise RuntimeError("No X11 window advertised the owned subprocess PID")
            self.sequence()

    def accident_age(self, age, label):
        return self.wait(lambda s: s["ambience"]["accident_ticks"] is not None
                         and s["ambience"]["accident_ticks"] >= age, label)

    def braking_pause_check(self):
        self.tap("Escape")
        paused = self.wait(lambda s: s["paused"], "braking pause")
        self.check("pause_catches_braking_before_impact",
                   paused["ambience"]["incident_car"]["phase"] == "Braking", observed=paused)
        self.settle(0.7)
        held = self.observe()
        keys = ["stage_ticks", "ticks", "player", "enemy", "ambience"]
        self.check("braking_pause_freezes_simulation_and_every_ambient_actor",
                   held["paused"] and all(held.get(k) == paused.get(k) for k in keys),
                   before={k: paused.get(k) for k in keys},
                   after={k: held.get(k) for k in keys})
        self.screenshot("traffic-06-braking-paused")
        self.tap("Return")
        resumed = self.wait(lambda s: not s["paused"], "braking resume")
        self.check("resume_continues_the_same_accident", resumed["stage"] == "Encounter"
                   and resumed["ambience"]["accident_ticks"] >= held["ambience"]["accident_ticks"],
                   observed=resumed)
        self.screenshot("traffic-07-braking-resumed")

    def guarded_right_camera(self):
        # The enemy's tall hurtbox prevents a jump from clearing its body.
        # Advance during recovery and guard each warning/lunge, keeping this
        # camera inspection independent of the player's remaining health.
        deadline = time.monotonic() + 30.0
        self.hold("d", True)
        try:
            while time.monotonic() < deadline:
                sample = self.observe()
                if sample["outcome"] != "Ongoing":
                    raise AssertionError(f"Combat ended before the camera inspection: {sample}")
                if sample["player"]["x"] >= 1430.0:
                    return sample
                self.hold("q", sample["enemy"]["action"] in ("Telegraph", "Lunge"))
                time.sleep(0.02)
            raise TimeoutError(f"Guarded walking did not reach the right camera clamp: {self.observe()}")
        finally:
            self.hold("d", False)
            self.hold("q", False)

    def sequence(self):
        calm = self.wait(lambda s: s["stage"] == "Encounter" and s["ticks"] >= 60,
                         "calm street", 15.0)
        self.check("traffic_begins_before_the_EP", not calm["enemy_awake"]
                   and calm["ambience"]["incident_car"] is None
                   and len(calm["ambience"]["traffic_cars"]) == 3, observed=calm)
        self.screenshot("traffic-01-calm")
        later = self.wait(lambda s: s["ticks"] >= calm["ticks"] + 120, "cars drive through the street")
        initial_positions = [car["x"] for car in calm["ambience"]["traffic_cars"]]
        later_positions = [car["x"] for car in later["ambience"]["traffic_cars"]]
        self.check("ordinary_cars_move_without_triggering_a_crash",
                   initial_positions != later_positions
                   and later["ambience"]["accident_ticks"] is None, before=calm["ambience"],
                   after=later["ambience"])
        self.screenshot("traffic-02-calm-later")
        self.hold("a", True)
        self.wait(lambda s: s["player"]["x"] <= 100.0, "left camera edge")
        self.hold("a", False)
        self.screenshot("traffic-03-left-camera")
        self.hold("d", True)
        quiet = self.wait(lambda s: s["player"]["x"] >= 950.0, "approaching EP", 12.0)
        self.hold("d", False)
        self.check("street_remains_calm_on_approach", not quiet["enemy_awake"]
                   and quiet["ambience"]["incident_car"] is None, observed=quiet)
        self.screenshot("traffic-04-before-EP")
        self.hold("d", True)
        awake = self.wait(lambda s: s["enemy_awake"], "EP triggers incident")
        self.hold("d", False)
        self.hold("q", True)
        self.check("EP_starts_child_reaction_and_incident",
                   awake["ambience"]["accident_ticks"] is not None
                   and awake["ambience"]["kid_phase"] != "Playing", observed=awake)
        self.accident_age(40, "horn during approach")
        self.screenshot("traffic-05-horn")
        self.accident_age(84, "braking before collision")
        self.braking_pause_check()
        self.accident_age(114, "front contact with pole")
        self.screenshot("traffic-08-impact")
        wreck = self.accident_age(165, "deformed car after impact")
        self.check("collision_finishes_at_the_fixed_pole",
                   wreck["ambience"]["incident_car"]["phase"] == "Crashed"
                   and abs(wreck["ambience"]["incident_car"]["x"] + 80.0 - 1330.0) < 0.01,
                   observed=wreck)
        self.screenshot("traffic-09-wreck")
        persistent = self.accident_age(320, "persistent accident aftermath")
        first_car = wreck["ambience"]["incident_car"]
        later_car = persistent["ambience"]["incident_car"]
        self.check("wreck_persists_while_child_leaves_and_other_cars_continue",
                   later_car["phase"] == "Crashed"
                   and (later_car["x"], later_car["y"]) == (first_car["x"], first_car["y"])
                   and persistent["ambience"]["kid_phase"] == "Gone"
                   and [car["x"] for car in persistent["ambience"]["traffic_cars"]]
                   != [car["x"] for car in wreck["ambience"]["traffic_cars"]],
                   before=wreck["ambience"], after=persistent["ambience"])
        self.screenshot("traffic-10-persistent-aftermath")

        right = self.guarded_right_camera()
        self.check("wreck_survives_camera_travel", right["ambience"]["incident_car"]["phase"] == "Crashed",
                   observed=right)
        self.screenshot("traffic-11-right-camera")

        self.log("waiting_for_unprotected_defeat")
        dead = self.wait(lambda s: s["outcome"] == "Defeat", "unprotected defeat", 40.0)
        self.check("real_defeat_before_retry", dead["player"]["hp"] == 0)
        self.tap("r")
        retry = self.wait(lambda s: s["outcome"] == "Ongoing" and s["player"]["hp"] == 100,
                          "retry checkpoint")
        self.check("retry_rearms_the_incident_at_the_awake_checkpoint", retry["enemy_awake"]
                   and retry["stage"] == "Encounter"
                   and retry["ambience"]["accident_ticks"] < 38
                   and retry["ambience"]["incident_car"]["phase"] == "Approaching", observed=retry)
        self.screenshot("traffic-12-retry")
        braking = self.accident_age(82, "braking after retry")
        self.check("retry_replays_the_braking_phase",
                   braking["ambience"]["incident_car"]["phase"] == "Braking", observed=braking)
        self.tap("Return")
        skipped = self.wait(lambda s: s["stage"] == "Opening", "skip accident during braking")
        self.check("skip_discards_the_incident_without_inventing_a_victory",
                   skipped["outcome"] == "Ongoing"
                   and skipped["ambience"]["incident_car"]["phase"] == "Braking", observed=skipped)
        self.settle(0.5)
        held = self.observe()
        self.check("opening_does_not_advance_the_discarded_accident",
                   held["ambience"] == skipped["ambience"], before=skipped["ambience"], after=held["ambience"])
        self.tap("Escape")
        self.wait(lambda s: s["paused"], "opening paused")
        self.tap("BackSpace")
        complete = self.wait(lambda s: s["stage"] == "Complete", "full skip during pause")
        self.check("full_skip_from_pause_finishes_without_victory", not complete["paused"]
                   and complete["outcome"] == "Ongoing", observed=complete)
        self.screenshot("traffic-13-skip-complete")
        self.tap("Return")
        restart = self.wait(lambda s: s["stage"] == "AdaPrologue", "full restart")
        self.check("restart_restores_a_calm_world", not restart["enemy_awake"]
                   and restart["ticks"] == 0 and restart["ambience"]["accident_ticks"] is None
                   and restart["ambience"]["incident_car"] is None, observed=restart)
        for _ in range(14):
            if self.observe()["stage"] == "Encounter":
                break
            self.tap("Return")
        quiet_again = self.wait(lambda s: s["stage"] == "Encounter", "street after full restart")
        self.check("restarted_street_has_traffic_and_no_wreck", not quiet_again["enemy_awake"]
                   and quiet_again["ambience"]["incident_car"] is None, observed=quiet_again)
        self.screenshot("traffic-14-restarted-calm")

    def cleanup(self):
        super().cleanup()
        report_path = self.directory / "native-checks.json"
        report = json.loads(report_path.read_text(encoding="utf-8"))
        report.pop("edited_valid_catalog_evidence", None)
        report.pop("expected_edited_text", None)
        report["review"] = "traffic and scripted pole accident"
        report["native_audio_enabled"] = not self.args.mute
        report["audio_capture"] = "External recorder required; the built-in MP4 contains video only."
        report_path.write_text(json.dumps(report, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--executable", default=str(ROOT / "target/debug/borrow-adventure"))
    parser.add_argument("--output-directory", required=True)
    parser.add_argument("--display", default=":0")
    parser.add_argument("--timeout", type=float, default=180.0)
    parser.add_argument("--mute", action="store_true", help="Disable native audio for a visual-only review.")
    review = TrafficReview(parser.parse_args())
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
