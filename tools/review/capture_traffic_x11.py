#!/usr/bin/env python3
"""Compatibility CLI and shared X11 runner for the current street evacuation review.

Running this filename delegates to capture_street_evacuation_x11.py, including
its five vehicle types, cyclist evacuation and permanently empty street checks.
TrafficReview retains only shared launch, clock, camera and cleanup behavior.
Native audio stays enabled unless --mute is supplied; game video is silent.
"""
from __future__ import annotations

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
                   "--start", getattr(self, "start_stage", "encounter"),
                   "--hidden", "--texts", str(self.catalog_path)]
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
    # Import lazily: the current evacuation review subclasses this shared runner.
    from capture_street_evacuation_x11 import main as evacuation_main
    return evacuation_main()


if __name__ == "__main__":
    raise SystemExit(main())
