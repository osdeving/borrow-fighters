#!/usr/bin/env python3
"""Play Chapter 01 with keyboard events sent only to an owned native X11 window.

Uses --start chapter --capture, never the application's automated --review
policy. Fresh evidence-local user data isolates campaign saves. Telemetry is
read-only: navigation, conversations, jumping, combat and retry all use keys.
The hidden game records its own render target; desktop focus is untouched.
"""
from __future__ import annotations

import argparse
import json
from pathlib import Path
import shutil
import subprocess
import sys
import time

from capture_traffic_x11 import TrafficReview


class ChapterReview(TrafficReview):
    start_stage = "chapter"

    def __init__(self, args):
        super().__init__(args)
        world = json.loads((self.root / "assets/adventure/chapter/world.json").read_text())
        self.scenes = {scene["id"]: scene for scene in world["scenes"]}
        self.retries = 0
        self.check("fresh_isolated_campaign_directory", not self.userdata.exists(),
                   data_directory=str(self.userdata),
                   environment_variable="BORROW_FIGHTERS_DATA_DIR")

    def phase(self, name, timeout=20.0):
        return self.wait(lambda s: s["phase"] == name, name, timeout)

    def ticks(self, count, label, timeout=12.0):
        initial = self.observe()["ticks"]
        return self.wait(lambda s: s["ticks"] >= initial + count, label, timeout)

    def screenshot(self, name):
        if self.args.mode == "preview":
            sample = self.observe()
            self.log("preview_marker", name=name, capture_seconds=sample["capture_seconds"], phase=sample["phase"])
            return None
        # Chapter F12 files live directly in --capture, alongside automatic
        # phase pictures. Match only the manual prefix and move, not duplicate.
        existing = set(self.directory.glob("chapter-*.png"))
        self.tap("F12")
        deadline = time.monotonic() + 6.0
        while time.monotonic() < deadline:
            sample = self.observe()
            created = set(self.directory.glob("chapter-*.png")) - existing
            if created:
                source = max(created, key=lambda path: path.stat().st_mtime_ns)
                target = self.shots / f"{name}.png"
                shutil.move(str(source), target)
                self.captures.append({"file": str(target.relative_to(self.directory)),
                                      "observed": sample})
                self.log("screenshot", file=str(target), phase=sample["phase"])
                return target
            time.sleep(0.03)
        raise TimeoutError(f"F12 did not create {name}")

    def walk_to(self, x, expected_phase, timeout=15.0):
        initial = self.observe()
        direction = 1 if x >= initial["player"]["x"] else -1
        key = "d" if direction > 0 else "a"
        self.hold(key, True)
        try:
            sample = self.wait(
                lambda s: s["phase"] != expected_phase
                or direction * (s["player"]["x"] - x) >= 0,
                f"walk to x={x} in {expected_phase}", timeout)
            if sample["phase"] != expected_phase:
                raise AssertionError(f"Unexpected phase while walking: {sample}")
        finally:
            self.hold(key, False)
        # Let real acceleration settle before changing direction or pressing E.
        # This is essential when backing away before a rightward jump.
        return self.wait(lambda s: s["phase"] == expected_phase
                         and s["player"]["action"] == "Idle", "walking input released")

    def interact(self, scene_id, point_id, explore_phase, dialogue_phase, next_phase, picture):
        self.phase(explore_phase)
        point = next(p for p in self.scenes[scene_id]["points"] if p["id"] == point_id)
        region = point["region"]
        self.walk_to(region["x"] + region["width"] * 0.5, explore_phase)
        before = self.wait(lambda s: s["interaction"] == point_id,
                           f"nearby interaction {point_id}")
        self.check(f"{point_id}_requires_reaching_its_region", before["controls_active"],
                   interaction=before["interaction"], player=before["player"])
        self.tap("e")
        arrived = self.phase(dialogue_phase)
        self.check(f"{point_id}_approach_releases_no_exploration_input",
                   not arrived["controls_active"] and arrived["line"] == 0,
                   player=arrived["player"], phase=arrived["phase"])
        if point["path"]:
            endpoint = point["path"][-1]
            self.check(f"{point_id}_walks_to_authored_path_endpoint",
                       abs(arrived["player"]["x"] - endpoint["x"]) < 1.0
                       and abs(arrived["player"]["y"] - endpoint["y"]) < 1.0,
                       actual=arrived["player"], endpoint=endpoint)
        for line in range(3):
            sample = self.wait(lambda s: s["phase"] == dialogue_phase
                               and s["line"] == line
                               and s["phase_ticks"] >= self.args.dialogue_ticks,
                               f"{point_id} line {line} readable", 15.0)
            if line == 1:
                self.screenshot(picture)
            self.log("dialogue_read", interaction=point_id, line=line,
                     text_key=sample["dialogue"], phase_ticks=sample["phase_ticks"])
            self.tap("Return")
        after = self.phase(next_phase)
        self.check(f"{point_id}_conversation_completed_in_order",
                   after["phase"] == next_phase,
                   checkpoint=after["checkpoint"], player=after["player"])
        return after

    def phone(self):
        self.wait(lambda s: s["phase"] == "Phone"
                  and s["phone"]["phase"] == "TypingFirst"
                  and s["phone"]["ticks"] >= 110, "first message typing")
        before = self.observe()
        if self.args.mode == "functional":
            self.hold("d", True)
            try:
                self.tap("j")
                self.tap("space")
                after = self.ticks(28, "movement keys ignored while typing")
            finally:
                self.hold("d", False)
            self.check("phone_typing_retains_position_and_ignores_gameplay_keys",
                       after["phase"] == "Phone" and not after["controls_active"]
                       and all(after["player"][k] == before["player"][k] for k in ("x", "y", "hp"))
                       and after["player"]["action"] not in ("LightAttack", "HeavyAttack", "Jump"),
                       before=before["player"], after=after["player"])
            self.tap("Escape")
            paused = self.wait(lambda s: s["paused"] and s["menu"] == "Pause", "phone paused")
            self.settle(0.65)
            still = self.observe()
            fields = ("ticks", "phase", "phase_ticks", "phone", "player", "camera", "shutter")
            self.check("pause_freezes_phone_world_and_camera",
                       all(still[k] == paused[k] for k in fields),
                       phone=still["phone"], ticks=still["ticks"])
            self.screenshot("05-phone-paused")
            self.tap("Return")
            resumed = self.wait(lambda s: not s["paused"], "phone resumed")
            self.check("resume_confirm_does_not_skip_phone",
                       resumed["phase"] == "Phone"
                       and resumed["phone"]["messages"] == paused["phone"]["messages"],
                       paused=paused["phone"], resumed=resumed["phone"])
        for count, phase, minimum in ((1, "FirstSent", 230), (2, "ReadingReply", 540),
                                      (3, "LastSent", 750)):
            sample = self.wait(lambda s: s["phase"] == "Phone"
                               and s["phone"]["phase"] == phase
                               and s["phone"]["ticks"] >= minimum,
                               f"phone message {count}", 15.0)
            self.check(f"phone_message_{count}_visible_in_order",
                       sample["phone"]["messages"] == count and sample["phone"]["panel"]
                       and sample["phone"]["device"], phone=sample["phone"])
            self.screenshot(f"0{5 + count}-phone-message-{count}")
        self.wait(lambda s: s["phase"] == "Phone" and s["phone"]["phase"] == "Stowing",
                  "phone stowed after all messages")
        self.screenshot("09-phone-stowing")
        after = self.phase("LeaveStreet")
        self.check("phone_returns_control_without_queued_movement_or_attack",
                   after["controls_active"] and after["phone"] is None
                   and all(after["player"][k] == before["player"][k] for k in ("x", "y", "hp"))
                   and after["player"]["action"] == "Idle",
                   player=after["player"], checkpoint=after["checkpoint"])

    def leave(self, scene_id, phase, next_phase):
        start = self.phase(phase)
        self.hold("d", True)
        try:
            after = self.phase(next_phase, 20.0)
        finally:
            self.hold("d", False)
        exit_region = self.scenes[scene_id]["exit"]
        preceding = [s for s in self.telemetry.samples
                     if s["frame"] >= start["frame"] and s["phase"] == phase]
        self.check(f"{scene_id}_exit_reached_by_walking",
                   bool(preceding) and max(s["player"]["x"] for s in preceding)
                   >= exit_region["x"] - 15,
                   next_phase=after["phase"], authored_exit=exit_region)
        return after

    def lane_obstacle(self):
        scene = self.scenes["lane"]
        obstacle = scene["obstacles"][0]
        self.walk_to(obstacle["x"] - 80, "LaneExplore")
        if self.args.mode == "functional":
            self.hold("d", True)
            try:
                self.ticks(35, "walking reaches obstacle")
                blocked = self.observe()
                later = self.ticks(15, "solid obstacle retains walking player")
            finally:
                self.hold("d", False)
            self.check("lane_obstacle_blocks_grounded_walking",
                       later["player"]["grounded"]
                       and later["player"]["x"] == blocked["player"]["x"]
                       and later["player"]["x"] < obstacle["x"],
                       obstacle=obstacle, player=later["player"])
            self.screenshot("10-lane-obstacle")
            self.walk_to(obstacle["x"] - 80, "LaneExplore")
        self.hold("d", True)
        try:
            self.tap("space")
            airborne = self.wait(lambda s: not s["player"]["grounded"]
                                and s["player"]["y"] < obstacle["y"] - 10,
                                "jump rises above obstacle")
            self.screenshot("11-lane-jump")
            landed = self.wait(lambda s: s["player"]["x"] > obstacle["x"] + obstacle["width"] + 30
                              and s["player"]["grounded"]
                              and abs(s["player"]["y"] - scene["floor_y"]) < 1,
                              "jump clears obstacle and lands", 12.0)
        finally:
            self.hold("d", False)
        self.check("lane_jump_clears_solid_and_lands_on_floor",
                   landed["player"]["x"] > obstacle["x"] + obstacle["width"],
                   airborne=airborne["player"], landed=landed["player"])

    def retry_combat(self, defeated):
        self.release_all()
        self.retries += 1
        self.screenshot(f"13-passage-defeat-{self.retries}")
        self.tap("r")
        retried = self.wait(lambda s: s["phase"] == "PassageCombat"
                            and s["outcome"] == "Ongoing", "local combat retry")
        self.check(f"defeat_retry_{self.retries}_restores_only_passage",
                   retried["scene"] == "Passage" and retried["player"]["hp"] > 0
                   and retried["enemy"]["hp"] > 0
                   and retried["checkpoint"] == defeated["checkpoint"]
                   and retried["checkpoint"]["stage"] == "passage_fight"
                   and retried["phone"] is None and retried["dialogue"] is None,
                   observed=retried)
        resumed_records = [s for s in self.telemetry.samples if s["frame"] > defeated["frame"]]
        self.check(f"defeat_retry_{self.retries}_does_not_replay_earlier_scenes",
                   all(s["scene"] == "Passage" and s["phase"] == "PassageCombat"
                       for s in resumed_records), records=len(resumed_records))
        return retried

    def real_defeat(self):
        first = self.phase("PassageCombat")
        self.hold("d", True)
        try:
            self.wait(lambda s: abs(s["enemy"]["x"] - s["player"]["x"]) < 110,
                      "walk into enemy attack range", 12.0)
        finally:
            self.release_all()
        defeated = self.wait(lambda s: s["outcome"] == "Defeat", "real unguarded defeat", 50.0)
        records = [s for s in self.telemetry.samples if s["frame"] >= first["frame"]]
        health = sorted({s["player"]["hp"] for s in records})
        self.check("unguarded_enemy_contacts_cause_real_defeat",
                   defeated["player"]["hp"] == 0 and len(health) > 2
                   and defeated["checkpoint"]["stage"] == "passage_fight",
                   observed_health=health, enemy_hp=defeated["enemy"]["hp"])
        self.retry_combat(defeated)

    def fight(self):
        if self.args.mode == "functional":
            self.real_defeat()
        first = self.phase("PassageCombat")
        self.hold("q", True)
        try:
            self.wait(lambda s: s["player"]["action"] == "Block", "Q activates native guard")
            self.ticks(12, "guard held through fixed updates")
        finally:
            self.hold("q", False)
        deadline = time.monotonic() + 90.0
        attacks = 0
        last_attack = 0.0
        picture = False
        try:
            while time.monotonic() < deadline:
                sample = self.observe()
                if sample["phase"] == "PassageClear":
                    break
                if sample["phase"] != "PassageCombat":
                    raise AssertionError(f"Unexpected phase during combat: {sample}")
                if sample["outcome"] == "Defeat":
                    forced_retries = int(self.args.mode == "functional")
                    if self.retries - forced_retries >= self.args.max_retries:
                        self.release_all()
                        self.screenshot("13-passage-retries-exhausted")
                        raise AssertionError("Real combat exhausted its retry allowance")
                    self.retry_combat(sample)
                    continue
                delta = sample["enemy"]["x"] - sample["player"]["x"]
                warning = sample["enemy"]["action"] in ("Telegraph", "Lunge")
                facing = "Right" if delta >= 0 else "Left"
                approach = not warning and (abs(delta) > 90 or sample["player"]["facing"] != facing)
                self.hold("d", approach and delta >= 0)
                self.hold("a", approach and delta < 0)
                self.hold("q", warning and abs(delta) < 190)
                if sample["enemy"]["hp"] < first["enemy"]["hp"] and not picture:
                    self.screenshot("13-passage-combat")
                    picture = True
                    continue
                ready = sample["player"]["action"] in ("Idle", "Walk", "Block")
                if ready and not warning and abs(delta) <= 104 and time.monotonic() - last_attack >= 0.22:
                    self.tap("j" if attacks % 2 == 0 else "k")
                    attacks += 1
                    last_attack = time.monotonic()
                else:
                    time.sleep(0.02)
            else:
                raise TimeoutError(f"Combat did not finish: {self.observe()}")
        finally:
            self.release_all()
        after = self.phase("PassageClear")
        records = [s for s in self.telemetry.samples if s["frame"] >= first["frame"]]
        actions = sorted({s["player"]["action"] for s in records})
        self.check("passage_cleared_through_real_light_heavy_guard_and_health",
                   after["outcome"] == "Victory" and after["enemy"]["hp"] == 0
                   and after["player"]["hp"] > 0
                   and {"LightAttack", "HeavyAttack", "Block"}.issubset(actions),
                   actions=actions, attacks_sent=attacks, retries=self.retries,
                   player_hp=after["player"]["hp"], enemy_hp=after["enemy"]["hp"])
        self.screenshot("14-passage-clear")

    def sequence(self):
        first = self.wait(lambda s: s.get("stage") == "Chapter", "chapter telemetry", 20.0)
        self.check("fresh_chapter_starts_in_intro", first["phase"] == "Intro", observed=first)
        self.wait(lambda s: s["phase"] == "Intro" and s["phase_ticks"] >= 300,
                  "intro midpoint", 20.0)
        self.screenshot("01-street-aftermath")
        ready = self.phase("ExploreDriver", 20.0)
        self.check("intro_plays_600_ticks_without_advance", ready["ticks"] >= 600,
                   ticks=ready["ticks"], controls_active=ready["controls_active"])
        self.interact("street", "driver", "ExploreDriver", "DriverDialogue", "ExploreShop", "02-driver")
        self.interact("street", "shop", "ExploreShop", "ShopDialogue", "ExploreNeighbour", "03-shop")
        self.interact("street", "neighbour", "ExploreNeighbour", "NeighbourDialogue", "Phone", "04-neighbour")
        self.phone()
        self.leave("street", "LeaveStreet", "LaneExplore")
        self.lane_obstacle()
        self.interact("lane", "lane_resident", "LaneExplore", "LaneDialogue", "LaneExit", "12-lane-resident")
        self.leave("lane", "LaneExit", "PassageExplore")
        self.hold("d", True)
        try:
            self.phase("PassageCombat", 15.0)
        finally:
            self.hold("d", False)
        self.fight()
        self.leave("passage", "PassageClear", "Departure")
        completed = self.phase("Complete", 12.0)
        self.screenshot("15-chapter-complete")
        save_path = self.userdata / "adventure/campaign-v1.json"
        progress = json.loads(save_path.read_text(encoding="utf-8"))
        self.check("chapter_completes_and_persists_inside_isolated_data",
                   completed["checkpoint"]["stage"] == "complete"
                   and progress["checkpoint"]["stage"] == "complete"
                   and not progress["prologue_played_victory"],
                   save=str(save_path.relative_to(self.directory)), progress=progress)

    def verify_video(self):
        # The shared chapter recorder keeps the adventure filename, while its
        # trace clock is capture_seconds. It does not produce result.json.
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
            expected = self.telemetry.samples[-1]["capture_seconds"]
            frames = int(stream.get("nb_frames", "0"))
            result.update(codec=stream.get("codec_name"), width=stream["width"], height=stream["height"],
                          frame_rate=stream.get("r_frame_rate"), frames=frames,
                          duration_seconds=duration, telemetry_seconds=expected,
                          absolute_duration_error=abs(duration - expected))
            result["passed"] = (stream["width"] == 1280 and stream["height"] == 720
                                and stream.get("r_frame_rate") == "30/1" and frames > 0
                                and duration > 1.0 and abs(duration - expected) <= 1.0)
        except (OSError, subprocess.SubprocessError, ValueError, KeyError, IndexError, StopIteration) as error:
            result["error"] = f"{type(error).__name__}: {error}"
        return result

    def cleanup(self):
        # Reuse owned-window key release, WM_DELETE_WINDOW, encoder shutdown,
        # executable/catalog integrity checks and failure-safe evidence output.
        super().cleanup()
        path = self.directory / "native-checks.json"
        report = json.loads(path.read_text(encoding="utf-8"))
        report["review"] = "chapter 01 through real native keyboard input"
        report["data_directory"] = "userdata"
        report["state_injection"] = False
        report["combat_retries"] = self.retries
        report["mode"] = self.args.mode
        report["dialogue_ticks_per_line"] = self.args.dialogue_ticks
        report.pop("external_text_catalog", None)
        path.write_text(json.dumps(report, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    root = Path(__file__).resolve().parents[2]
    parser.add_argument("--executable", default=str(root / "target/debug/borrow-adventure"))
    parser.add_argument("--output-directory", help="New evidence directory; must not already exist")
    parser.add_argument("--display", default=":0")
    parser.add_argument("--timeout", type=float, default=300.0)
    parser.add_argument("--dialogue-ticks", type=int, default=150,
                        help="Minimum readable time per line at 60 fixed updates/second")
    parser.add_argument("--max-retries", type=int, default=2,
                        help="Unexpected defeats allowed in addition to the functional defeat check")
    mode = parser.add_mutually_exclusive_group()
    mode.add_argument("--functional", dest="mode", action="store_const", const="functional",
                      help="Check paused input and force one real defeat/retry (default)")
    mode.add_argument("--preview", dest="mode", action="store_const", const="preview",
                      help="Continuous chapter recording without pause or deliberate defeat")
    parser.set_defaults(mode="functional")
    parser.add_argument("--mute", action="store_true", help="Disable game audio; recorded MP4 is always silent")
    args = parser.parse_args()
    if not 60 <= args.timeout <= 900:
        parser.error("--timeout must be between 60 and 900 seconds")
    if not 90 <= args.dialogue_ticks <= 360:
        parser.error("--dialogue-ticks must be between 90 and 360")
    if not 0 <= args.max_retries <= 5:
        parser.error("--max-retries must be between 0 and 5")
    review = ChapterReview(args)
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
