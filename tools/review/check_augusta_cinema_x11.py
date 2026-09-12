#!/usr/bin/env python3
"""Exercise Augusta cinematic pause, skip and handoff through its owned X11 window.

Only the game writes the isolated profile. State assertions use captured runtime
telemetry, and keyboard events target the owned process without changing focus.
"""

import argparse
import json
import os
from pathlib import Path
import sys

from check_chapter_host_x11 import ChapterHostReview


class AugustaCinemaReview(ChapterHostReview):
    def state(self):
        path = self.directory / "capture" / "telemetry.jsonl"
        if not path.exists():
            return None
        lines = path.read_text().splitlines()
        for line in reversed(lines[-3:]):
            try:
                return json.loads(line)
            except json.JSONDecodeError:
                pass
        return None

    def phase(self, phase):
        row = self.state()
        return row is not None and row["phase"] == phase

    def pause_check(self, name):
        self.tap("Escape")
        self.await_condition(lambda: self.state()["paused"], "cinematic pause")
        before = self.state()
        self.wait(0.8)
        after = self.state()
        self.check(name, all(before[key] == after[key] for key in
                            ("ticks", "phase_ticks", "shot", "npcs", "actors", "arrivals")))
        self.tap("Escape")
        self.await_condition(lambda: not self.state()["paused"], "resume")
        self.check(name + "_resumes", self.state()["ticks"] > before["ticks"])

    def sequence(self):
        self.start("native", ["--start", "augusta", "--mute", "--capture",
                              str(self.directory / "capture")])
        self.await_condition(lambda: self.phase("Introduction"), "Augusta opening")
        self.pause_check("opening_camera_and_cast_freeze")
        self.tap("BackSpace")
        self.await_condition(lambda: self.phase("Approach"), "skip opening to exploration")
        self.x11.send_key(self.process, self.window, "d", True)
        try:
            self.await_condition(lambda: self.phase("JuliaAttempt"), "Julia attempt", 15)
        finally:
            self.x11.send_key(self.process, self.window, "d", False)
        self.check("attempt_precedes_dialogue", self.state()["dialogue"] is None)
        self.screenshot("01-julia-attempt", "Julia attempts to leave before conversation")
        self.tap("BackSpace")
        self.await_condition(lambda: self.phase("Confrontation"), "attempt skip")
        for _ in range(9):
            if not self.phase("Confrontation"):
                break
            self.tap("Return")
        self.await_condition(lambda: self.phase("GuardsArrival"), "guards exit the bar")
        self.wait(1.4)
        self.pause_check("door_and_guards_freeze")
        self.check("guards_cannot_attack_during_film",
                   all(not actor["active"] for actor in self.state()["actors"]
                       if actor["character"] == "security"))
        self.screenshot("02-guards-emerge", "Inactive guards emerge through the registered bar door")
        self.tap("BackSpace")
        self.await_condition(lambda: self.phase("GuardsFight"), "combat handoff")
        self.check("skip_hands_over_live_enemies",
                   sum(a["active"] and a["hp"] > 0 for a in self.state()["actors"]
                       if a["character"] == "security") == 3)
        self.tap("BackSpace")
        self.check("skip_does_not_win_combat", self.phase("GuardsFight"))
        self.tap("F5")
        self.await_condition(lambda: self.phase("GuardsFight"), "F5 checkpoint reconstruction")
        self.check("reload_retains_encounter", self.state()["checkpoint"]["stage"] == "guards_fight")
        self.screenshot("03-combat-handoff", "Gameplay remains live after skip and reload")
        self.check("rust_profile_was_not_created", not self.profile_path.exists())


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--executable", default=str(Path(__file__).resolve().parents[2] / "target/debug/borrow-story"))
    parser.add_argument("--output-directory", required=True)
    parser.add_argument("--display", default=os.environ.get("DISPLAY", ":0"))
    parser.add_argument("--timeout", type=float, default=180)
    return AugustaCinemaReview(parser.parse_args()).run()


if __name__ == "__main__":
    sys.exit(main())
