#!/usr/bin/env python3
"""Verify the adventure through keyboard events sent to its own X11 window.

Only the generic X11 boundary is reused from the existing capture tool. The
owned native window stays hidden while its render target is recorded. This
script does not import arena rules, change focus, capture the desktop or control
an existing process. Observed telemetry proves domain effects; XSendEvent is
synthetic window input, not a physical keyboard/gamepad or subjective review.
"""

from __future__ import annotations

import argparse
from datetime import datetime, timezone
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import time

from capture_match_flow_x11 import X11


class Telemetry:
    """Tail complete JSON lines without rereading or accepting a partial record."""

    def __init__(self, path):
        self.path = path
        self.cursor = 0
        self.pending = ""
        self.samples = []

    def read(self):
        if not self.path.exists():
            return None
        with self.path.open(encoding="utf-8") as source:
            source.seek(self.cursor)
            self.pending += source.read()
            self.cursor = source.tell()
        while "\n" in self.pending:
            line, self.pending = self.pending.split("\n", 1)
            if line.strip():
                self.samples.append(json.loads(line))
        return self.samples[-1] if self.samples else None


class AdventureReview:
    def __init__(self, args):
        self.args = args
        self.root = Path(__file__).resolve().parents[2]
        self.binary = Path(args.executable).resolve(strict=True)
        if not os.access(self.binary, os.X_OK):
            raise RuntimeError(f"Not executable: {self.binary}")
        self.directory = (
            Path(args.output_directory).absolute()
            if args.output_directory else Path(tempfile.mkdtemp(prefix="borrow-adventure-x11-"))
        )
        if args.output_directory:
            self.directory.mkdir(parents=True, exist_ok=False)
        self.shots = self.directory / "screenshots"
        self.shots.mkdir()
        self.userdata = self.directory / "userdata"
        self.telemetry = Telemetry(self.directory / "telemetry.jsonl")
        self.x11 = None
        self.process = None
        self.window = None
        self.held = set()
        self.started = time.monotonic()
        self.checks = []
        self.captures = []
        self.failure = None

    def log(self, event, **details):
        record = {
            "event": event,
            "elapsed_seconds": round(time.monotonic() - self.started, 3),
            **details,
        }
        with (self.directory / "native-events.jsonl").open("a", encoding="utf-8") as target:
            target.write(json.dumps(record, ensure_ascii=False) + "\n")
        print(json.dumps(record, ensure_ascii=False), flush=True)

    def alive(self):
        if time.monotonic() - self.started > self.args.timeout:
            raise TimeoutError(f"Review exceeded its {self.args.timeout:.0f}s process limit")
        if self.process is not None and self.process.poll() is not None:
            raise RuntimeError(f"Adventure exited unexpectedly: {self.process.returncode}")

    def observe(self):
        self.alive()
        return self.telemetry.read()

    def wait(self, predicate, label, timeout=8.0):
        deadline = time.monotonic() + timeout
        while time.monotonic() < deadline:
            sample = self.observe()
            if sample is not None and predicate(sample):
                return sample
            time.sleep(0.02)
        raise TimeoutError(f"Telemetry did not confirm {label}; last={self.telemetry.read()}")

    def settle(self, duration):
        end = time.monotonic() + duration
        while time.monotonic() < end:
            self.observe()
            time.sleep(min(0.02, max(0.0, end - time.monotonic())))

    def hold(self, key, pressed):
        if pressed == (key in self.held):
            return
        self.x11.send_key(self.process, self.window, key, pressed)
        if pressed:
            self.held.add(key)
        else:
            self.held.discard(key)

    def tap(self, key):
        # The renderer may be slow while encoding. Keep each edge across actual
        # event polls instead of sending press/release between two game frames.
        initial_frame = self.observe()["frame"]
        self.hold(key, True)
        self.wait(lambda sample: sample["frame"] >= initial_frame + 2, f"{key} press polled", 8.0)
        release_frame = self.observe()["frame"]
        self.hold(key, False)
        self.wait(lambda sample: sample["frame"] >= release_frame + 2, f"{key} release polled", 8.0)

    def release_all(self):
        for key in list(self.held):
            self.hold(key, False)

    def check(self, name, passed, **measurements):
        item = {"name": name, "passed": bool(passed), **measurements}
        self.checks.append(item)
        self.log("check", **item)
        if not passed:
            raise AssertionError(f"Native check failed: {name}: {measurements}")

    def screenshot(self, name):
        source_dir = self.userdata / "captures" / "adventure"
        existing = set(source_dir.glob("*.png"))
        self.tap("F12")
        deadline = time.monotonic() + 4.0
        while time.monotonic() < deadline:
            self.observe()
            created = set(source_dir.glob("*.png")) - existing
            if created:
                source = max(created, key=lambda path: path.stat().st_mtime)
                target = self.shots / f"{name}.png"
                shutil.copy2(source, target)
                snapshot = self.telemetry.read()
                self.captures.append({"file": str(target.relative_to(self.directory)), "observed": snapshot})
                self.log("screenshot", file=str(target), stage=snapshot["stage"])
                return
            time.sleep(0.03)
        raise TimeoutError(f"F12 did not create the requested {name} screenshot")

    def run(self):
        self.x11 = X11(self.args.display)
        env = os.environ.copy()
        env["DISPLAY"] = self.args.display
        env["BORROW_FIGHTERS_DATA_DIR"] = str(self.userdata)
        env["BORROW_FIGHTERS_ASSET_DIR"] = str(self.root / "assets")
        command = [str(self.binary), "--capture", str(self.directory), "--start", "morning", "--mute", "--hidden"]
        with (self.directory / "game.log").open("w", encoding="utf-8") as game_log:
            self.process = subprocess.Popen(command, cwd=self.directory, env=env, stdout=game_log, stderr=subprocess.STDOUT)
            self.log("started", pid=self.process.pid, command=command, display=self.args.display)
            deadline = time.monotonic() + 12.0
            while self.window is None and time.monotonic() < deadline:
                self.alive()
                self.window = self.x11.find_window(self.process.pid)
                time.sleep(0.04)
            if self.window is None:
                raise RuntimeError("No X11 window advertised the owned subprocess PID")
            self.sequence()

    def sequence(self):
        morning = self.wait(lambda s: s["stage"] == "RustMorning" and s["stage_ticks"] >= 45, "running morning", 15.0)
        self.check("morning_runs_before_control", morning["ticks"] == 0 and morning["player"]["x"] == 340.0,
                   stage_ticks=morning["stage_ticks"], combat_ticks=morning["ticks"])
        self.screenshot("01-morning")
        self.tap("Escape")
        paused = self.wait(lambda s: s["paused"], "pause")
        self.settle(0.6)
        held = self.observe()
        unchanged = (held["stage_ticks"], held["ticks"], held["player"]["hp"], held["enemy"]["hp"]) == (
            paused["stage_ticks"], paused["ticks"], paused["player"]["hp"], paused["enemy"]["hp"])
        self.check("pause_freezes_story_combat_and_health", held["paused"] and unchanged,
                   stage_tick_delta=held["stage_ticks"] - paused["stage_ticks"],
                   combat_tick_delta=held["ticks"] - paused["ticks"], observed_seconds=0.6)
        self.screenshot("02-morning-paused")
        self.tap("Return")
        resumed = self.wait(lambda s: not s["paused"] and s["stage_ticks"] > held["stage_ticks"], "resumed morning")
        self.check("enter_resumes_without_skipping_morning", resumed["stage"] == "RustMorning",
                   stage=resumed["stage"], stage_ticks=resumed["stage_ticks"])
        self.settle(0.2)
        self.tap("Return")
        encounter = self.wait(lambda s: s["stage"] == "Encounter", "separate Enter skips morning")
        self.check("separate_enter_enters_playable_scene", encounter["outcome"] == "Ongoing" and not encounter["enemy_awake"],
                   stage=encounter["stage"], x=encounter["player"]["x"])
        start_x = encounter["player"]["x"]
        self.hold("d", True)
        moving = self.wait(lambda s: s["player"]["x"] >= start_x + 80.0, "D moves Rust")
        self.check("keyboard_moves_rust", moving["player"]["x"] - start_x >= 80.0,
                   displacement=moving["player"]["x"] - start_x)
        self.tap("space")
        airborne = self.wait(lambda s: s["player"]["y"] <= 550.0, "Space jumps")
        self.check("keyboard_jump_leaves_floor", airborne["player"]["y"] < 550.0,
                   height_above_floor=580.0 - airborne["player"]["y"])
        self.screenshot("03-native-jump")
        self.wait(lambda s: s["enemy_awake"], "walking triggers enemy aggro", 8.0)
        self.hold("d", False)
        self.release_all()
        self.log("waiting_for_unprotected_defeat")
        dead = self.wait(lambda s: s["outcome"] == "Defeat", "unprotected Rust loses health and dies", 35.0)
        self.check("enemy_can_defeat_unprotected_player", dead["player"]["hp"] == 0 and dead["stage"] == "Encounter",
                   player_hp=dead["player"]["hp"], enemy_hp=dead["enemy"]["hp"])
        self.settle(0.9)
        self.screenshot("04-native-defeat")
        self.tap("r")
        retry = self.wait(lambda s: s["outcome"] == "Ongoing" and s["player"]["hp"] == 100, "R retries without prologue")
        self.check("retry_resets_health_without_prologue", retry["stage"] == "Encounter" and retry["enemy"]["hp"] == 96,
                   stage=retry["stage"], player_hp=retry["player"]["hp"], enemy_hp=retry["enemy"]["hp"])
        self.screenshot("05-native-retry")
        self.fight()
        remorse = self.wait(lambda s: s["stage"] == "Aftermath" and s["player"]["action"] == "Remorse", "compassionate gesture", 10.0)
        self.check("victory_reaches_compassionate_gesture", remorse["enemy"]["hp"] == 0 and remorse["player"]["hp"] > 0,
                   player_hp=remorse["player"]["hp"], enemy_hp=remorse["enemy"]["hp"], action=remorse["player"]["action"])
        self.settle(1.7)
        self.screenshot("07-native-remorse")
        complete = self.wait(lambda s: s["stage"] == "Complete", "ending after remorse", 10.0)
        self.check("story_completes_only_after_real_victory", complete["outcome"] == "Victory" and complete["enemy"]["hp"] == 0,
                   stage=complete["stage"], outcome=complete["outcome"])
        self.screenshot("08-native-complete")

    def fight(self):
        self.log("keyboard_combat_started")
        start_frame = self.observe()["frame"]
        deadline = time.monotonic() + 45.0
        last_attack = 0.0
        attack_count = 0
        while time.monotonic() < deadline:
            sample = self.observe()
            if sample["outcome"] != "Ongoing":
                break
            player, enemy = sample["player"], sample["enemy"]
            dx = enemy["x"] - player["x"]
            direction = "Right" if dx >= 0 else "Left"
            move = abs(dx) > 96.0 or player["facing"] != direction
            self.hold("d", move and dx >= 0)
            self.hold("a", move and dx < 0)
            warning = enemy["action"] in ("Telegraph", "Lunge")
            self.hold("q", warning and abs(dx) < 175.0)
            can_attack = player["action"] in ("Idle", "Walk", "Block")
            if not warning and abs(dx) <= 112.0 and can_attack and time.monotonic() - last_attack >= 0.22:
                self.tap("j" if attack_count % 2 == 0 else "k")
                attack_count += 1
                last_attack = time.monotonic()
            else:
                time.sleep(0.02)
        self.release_all()
        final = self.observe()
        samples = [s for s in self.telemetry.samples if s["frame"] >= start_frame]
        player_hits = {(s["ticks"] - s["hit"]["age"]) for s in samples
                       if s["hit"] and s["hit"]["target"] == "Erratic" and not s["hit"]["blocked"]}
        blocks = {(s["ticks"] - s["hit"]["age"]) for s in samples if s["hit"] and s["hit"]["blocked"]}
        actions = {s["player"]["action"] for s in samples}
        self.check("light_and_heavy_keys_reach_the_simulation", {"LightAttack", "HeavyAttack"}.issubset(actions),
                   observed_attack_actions=sorted(actions & {"LightAttack", "HeavyAttack"}))
        self.check("native_keyboard_wins_through_melee_contact", final["outcome"] == "Victory" and len(player_hits) >= 4,
                   outcome=final["outcome"], enemy_hp=final["enemy"]["hp"], player_hp=final["player"]["hp"],
                   distinct_player_contacts=len(player_hits), distinct_guard_contacts=len(blocks))
        self.check("keyboard_guard_blocks_real_enemy_contacts", len(blocks) >= 1, distinct_guard_contacts=len(blocks))
        self.screenshot("06-native-victory")

    def cleanup(self):
        if self.process is not None and self.process.poll() is None:
            try:
                if self.window is not None:
                    self.release_all()
                    self.x11.request_close(self.process, self.window)
                else:
                    self.process.terminate()
                self.process.wait(timeout=8.0)
            except (subprocess.TimeoutExpired, RuntimeError):
                self.process.kill()
                self.process.wait(timeout=5.0)
        self.telemetry.read()
        if self.x11 is not None:
            self.x11.close()
        if self.process is not None and self.process.returncode != 0 and self.failure is None:
            self.failure = f"Adventure did not exit cleanly: status {self.process.returncode}"
        video_check = self.verify_video()
        self.checks.append(video_check)
        self.log("check", **video_check)
        if not video_check["passed"] and self.failure is None:
            self.failure = "The native input checks completed, but the recorded MP4 did not validate"
        result = {
            "created_utc": datetime.now(timezone.utc).isoformat(),
            "success": self.failure is None and bool(self.checks) and all(c["passed"] for c in self.checks),
            "failure": self.failure,
            "elapsed_seconds": round(time.monotonic() - self.started, 3),
            "owned_pid": self.process.pid if self.process else None,
            "owned_window": self.window,
            "process_exit_code": self.process.returncode if self.process else None,
            "display": self.args.display,
            "input_method": "XSendEvent addressed to _NET_WM_PID-owned window",
            "limitations": "Synthetic window keyboard input; no physical gamepad, focus changes, desktop capture, audio listening or human visual approval.",
            "checks": self.checks,
            "screenshots": self.captures,
            "telemetry_records": len(self.telemetry.samples),
            "video": "adventure-silent.mp4",
            "x11_errors": self.x11.errors if self.x11 else [],
        }
        (self.directory / "native-checks.json").write_text(json.dumps(result, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
        self.log("finished", success=result["success"], evidence=str(self.directory), failure=self.failure)

    def verify_video(self):
        """Require a decoded video stream and its actual elapsed capture duration."""
        result = {"name": "recorded_mp4_has_frames_and_preserves_elapsed_time", "passed": False}
        try:
            probe = subprocess.run([
                "ffprobe", "-v", "error", "-show_entries",
                "format=duration:stream=codec_name,width,height,r_frame_rate,nb_frames",
                "-of", "json", str(self.directory / "adventure-silent.mp4"),
            ], capture_output=True, text=True, check=True, timeout=10.0)
            data = json.loads(probe.stdout)
            stream = next(s for s in data.get("streams", []) if s.get("width"))
            duration = float(data["format"]["duration"])
            expected = self.telemetry.samples[-1]["seconds"]
            frames = int(stream.get("nb_frames", "0"))
            result.update({
                "codec": stream.get("codec_name"), "width": stream["width"], "height": stream["height"],
                "frame_rate": stream.get("r_frame_rate"), "frames": frames,
                "duration_seconds": duration, "telemetry_seconds": expected,
                "absolute_duration_error": abs(duration - expected),
            })
            result["passed"] = (
                stream["width"] == 1280 and stream["height"] == 720
                and stream.get("r_frame_rate") == "30/1" and frames > 0
                and duration > 1.0 and abs(duration - expected) <= 1.0
                and (self.directory / "result.json").is_file()
            )
        except (OSError, subprocess.SubprocessError, ValueError, KeyError, IndexError, StopIteration) as error:
            result["error"] = f"{type(error).__name__}: {error}"
        return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--executable", default=str(Path(__file__).resolve().parents[2] / "target/debug/borrow-adventure"))
    parser.add_argument("--output-directory")
    parser.add_argument("--display", default=":0")
    parser.add_argument("--timeout", type=float, default=120.0)
    args = parser.parse_args()
    if not 1.0 <= args.timeout <= 120.0:
        parser.error("--timeout must be between 1 and 120 seconds")
    review = AdventureReview(args)
    try:
        review.run()
    except (Exception, KeyboardInterrupt) as error:
        review.failure = f"{type(error).__name__}: {error}"
        print(review.failure, file=sys.stderr, flush=True)
    finally:
        review.cleanup()
    return 1 if review.failure else 0


if __name__ == "__main__":
    raise SystemExit(main())
