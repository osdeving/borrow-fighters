#!/usr/bin/env python3
"""Captures the improved morning and street, reusing the repository's owned-window X11 harness.

Captures are driven by observed simulation ticks; native frame timing is real,
not deterministic. Only the subprocess window receives synthetic key events.
"""
from __future__ import annotations

import argparse
from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "tools/review"))
from capture_adventure_x11 import AdventureReview


class SceneReview(AdventureReview):
    def pause_check(self, label):
        self.tap("Escape")
        paused = self.wait(lambda s: s["paused"], label + " pause")
        self.settle(0.7)
        held = self.observe()
        keys = ["stage_ticks", "ticks", "player", "enemy", "ambience"]
        self.check(label + "_pause_freezes_simulation",
                   all(held.get(k) == paused.get(k) for k in keys),
                   before={k: paused.get(k) for k in keys},
                   after={k: held.get(k) for k in keys})
        self.screenshot(label + "-paused")
        self.tap("Return")
        self.wait(lambda s: not s["paused"], label + " resumed")

    def sequence(self):
        self.wait(lambda s: s["stage"] == "RustMorning", "morning", 15.0)
        for ticks in (90, 180, 250, 320, 390, 470, 545, 650):
            self.wait(lambda s: s["stage_ticks"] >= ticks, "morning pose", 10.0)
            self.screenshot(f"morning-{ticks:03}")
        self.pause_check("morning")
        self.tap("Return")
        entered = self.wait(lambda s: s["stage"] == "Encounter", "street after segment skip")
        self.check("morning_skip_enters_exploration", not entered["enemy_awake"])
        self.wait(lambda s: s["ticks"] >= 90, "street settles")
        self.screenshot("street-01-calm")
        first = self.observe()
        self.wait(lambda s: s["ticks"] >= first["ticks"] + 150, "background moves")
        self.screenshot("street-02-calm-later")
        self.pause_check("street-calm")
        self.hold("a", True)
        self.wait(lambda s: s["player"]["x"] <= 100.0, "left camera extreme")
        self.hold("a", False)
        self.screenshot("street-03-left-edge")
        self.hold("d", True)
        self.wait(lambda s: s["player"]["x"] >= 950.0, "approaching threat", 12.0)
        self.hold("d", False)
        quiet = self.observe()
        self.check("approach_still_calm", not quiet["enemy_awake"], observed=quiet)
        self.screenshot("street-04-before-ep")
        self.hold("d", True)
        awake = self.wait(lambda s: s["enemy_awake"], "EP triggers flight")
        self.hold("d", False)
        self.check("EP_awakens_during_approach", awake["enemy_awake"], observed=awake)
        self.screenshot("street-05-boy-startle")
        trigger = awake["ticks"]
        self.wait(lambda s: s["ambience"]["kid_phase"] == "Releasing", "boy releases line")
        self.tap("Escape")
        releasing = self.wait(lambda s: s["paused"], "freeze release pose")
        self.check("release_pose_is_visible_before_running", releasing["ambience"]["kid_phase"] == "Releasing",
                   observed=releasing)
        self.screenshot("street-06-release-paused")
        self.tap("Return")
        self.wait(lambda s: not s["paused"], "resume release")
        for delta, name in ((100, "run"), (210, "run-later"), (420, "empty")):
            self.wait(lambda s: s["ticks"] >= trigger + delta, "boy " + name, 12.0)
            self.screenshot("street-06-" + name)
            if name == "run":
                self.pause_check("street-flight")
        dead = self.wait(lambda s: s["outcome"] == "Defeat", "unprotected defeat", 40.0)
        self.check("real_defeat_before_retry", dead["player"]["hp"] == 0)
        self.tap("r")
        retry = self.wait(lambda s: s["outcome"] == "Ongoing" and s["player"]["hp"] == 100,
                          "retry checkpoint")
        self.check("retry_resets_combat_at_awake_checkpoint", retry["enemy_awake"]
                   and retry["enemy"]["hp"] == 96 and retry["stage"] == "Encounter", observed=retry)
        self.screenshot("street-08-retry")
        self.hold("d", True)
        self.wait(lambda s: s["enemy"]["x"] - s["player"]["x"] <= 245,
                  "jump approach after retry", 6.0)
        self.tap("space")
        right = self.wait(lambda s: s["player"]["x"] >= 1430, "right camera clamp", 10.0)
        self.hold("d", False)
        self.check("right_camera_clamp_reached", right["player"]["x"] >= 1370, observed=right)
        self.screenshot("street-09-right-camera-clamp")
        self.tap("Return")
        skipped = self.wait(lambda s: s["stage"] == "Opening", "encounter segment skip")
        self.check("encounter_skip_preserves_outcome", skipped["outcome"] == "Ongoing")
        self.tap("Escape")
        self.wait(lambda s: s["paused"], "opening paused")
        self.tap("BackSpace")
        complete = self.wait(lambda s: s["stage"] == "Complete", "full skip during pause")
        self.check("full_skip_from_pause_completes_without_victory", not complete["paused"]
                   and complete["outcome"] == "Ongoing", observed=complete)
        self.screenshot("skip-complete")
        self.tap("Return")
        restart = self.wait(lambda s: s["stage"] == "AdaPrologue", "full restart")
        self.check("restart_restores_calm_world", not restart["enemy_awake"]
                   and restart["ticks"] == 0, observed=restart)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--executable", default=str(ROOT / "target/debug/borrow-adventure"))
    parser.add_argument("--output-directory", required=True)
    parser.add_argument("--display", default=":0")
    parser.add_argument("--timeout", type=float, default=180.0)
    review = SceneReview(parser.parse_args())
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
