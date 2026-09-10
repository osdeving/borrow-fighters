#!/usr/bin/env python3
"""Review prologue skipping and the menu through process-owned X11 windows.

Input is directed only to the child PID. XGetImage reads that same window into
FFmpeg without editing its pixels; no desktop capture or focus changes occur.
Menu screenshots need visual review. Physical gamepad/audio approval is separate.
"""

from __future__ import annotations

import argparse
import ctypes as C
import json
import os
from pathlib import Path
import subprocess
import time

from capture_adventure_x11 import Telemetry
from capture_match_flow_x11 import Display, Window, X11, XEvent, sha256


class XButtonEvent(C.Structure):
    _fields_ = [
        ("type", C.c_int), ("serial", C.c_ulong), ("send_event", C.c_int),
        ("display", Display), ("window", Window), ("root", Window),
        ("subwindow", Window), ("time", C.c_ulong), ("x", C.c_int),
        ("y", C.c_int), ("x_root", C.c_int), ("y_root", C.c_int),
        ("state", C.c_uint), ("button", C.c_uint), ("same_screen", C.c_int),
    ]


class XPointerEvent(C.Union):
    _fields_ = [("button", XButtonEvent), ("pad", C.c_long * 24)]


class XImage(C.Structure):
    _fields_ = [
        ("width", C.c_int), ("height", C.c_int), ("xoffset", C.c_int),
        ("format", C.c_int), ("data", C.c_void_p), ("byte_order", C.c_int),
        ("bitmap_unit", C.c_int), ("bitmap_bit_order", C.c_int),
        ("bitmap_pad", C.c_int), ("depth", C.c_int),
        ("bytes_per_line", C.c_int), ("bits_per_pixel", C.c_int),
        ("red_mask", C.c_ulong), ("green_mask", C.c_ulong), ("blue_mask", C.c_ulong),
    ]


class Review:
    def __init__(self, args):
        self.args = args
        self.root = Path(__file__).resolve().parents[2]
        self.binary = Path(args.executable).resolve(strict=True)
        self.directory = Path(args.output_directory).absolute()
        self.directory.mkdir(parents=True, exist_ok=False)
        self.shots = self.directory / "screenshots"
        self.shots.mkdir()
        self.started = time.monotonic()
        self.catalog = self.root / "assets/adventure/texts/pt-BR.json"
        self.catalog_hash = sha256(self.catalog)
        self.binary_hash = sha256(self.binary)
        self.x11 = X11(args.display)
        self.x11.lib.XGetImage.argtypes = [Display, Window, C.c_int, C.c_int, C.c_uint, C.c_uint, C.c_ulong, C.c_int]
        self.x11.lib.XGetImage.restype = C.POINTER(XImage)
        self.x11.lib.XDestroyImage.argtypes = [C.POINTER(XImage)]
        self.x11.lib.XDestroyImage.restype = C.c_int
        self.process = None
        self.window = None
        self.video = None
        self.video_frames = 0
        self.next_frame = 0.0
        self.story_trace = None
        self.checks = []
        self.captures = []

    def observe(self, predicate, label, timeout=10):
        deadline = time.monotonic() + timeout
        while time.monotonic() < deadline:
            sample = self.story_trace.read() if self.story_trace else None
            if sample is not None and predicate(sample):
                return sample
            self.wait(0.03)
        raise TimeoutError(f"Telemetry did not confirm {label}: {sample}")

    def log(self, event, **detail):
        value = {"event": event, "elapsed": round(time.monotonic() - self.started, 3), **detail}
        with (self.directory / "events.jsonl").open("a") as out:
            out.write(json.dumps(value, ensure_ascii=False) + "\n")
        print(json.dumps(value, ensure_ascii=False), flush=True)

    def check(self, name, condition, **details):
        self.checks.append({"name": name, "passed": bool(condition), **details})
        self.log("check", **self.checks[-1])
        if not condition:
            raise AssertionError(name)

    def pixels(self):
        self.x11.assert_owned(self.process, self.window)
        source = self.x11.lib.XGetImage(self.x11.display, self.window, 0, 0, 1280, 720, C.c_ulong(-1).value, 2)
        if not source:
            raise RuntimeError("Owned-window XGetImage failed")
        try:
            info = source.contents
            if (info.bits_per_pixel, info.bytes_per_line, info.byte_order, info.red_mask, info.green_mask, info.blue_mask) != (32, 5120, 0, 0xff0000, 0xff00, 0xff):
                raise RuntimeError("Unexpected native pixel format; no conversion attempted")
            return C.string_at(info.data, info.bytes_per_line * info.height)
        finally:
            self.x11.lib.XDestroyImage(source)

    def wait(self, duration):
        end = time.monotonic() + duration
        while time.monotonic() < end:
            self.x11.assert_owned(self.process, self.window)
            now = time.monotonic()
            if self.video is not None and now >= self.next_frame:
                pixels = self.pixels()
                self.video.stdin.write(pixels)
                sample = self.story_trace.read() if self.story_trace else None
                header = [tuple(pixels[(110 * 1280 + x) * 4:(110 * 1280 + x) * 4 + 3]) for x in (670, 800, 1210)]
                menu_visible = all(abs(b - 47) <= 8 and abs(g - 42) <= 8 and abs(r - 28) <= 8 for b, g, r in header)
                with (self.directory / "window-frames.jsonl").open("a") as out:
                    out.write(json.dumps({"frame": self.video_frames, "elapsed": now - self.started, "stage": sample["stage"] if sample else None, "stage_ticks": sample["stage_ticks"] if sample else None, "waiting_for_continue": sample["waiting_for_continue"] if sample else None, "continue_accepted": sample["continue_accepted"] if sample else None, "menu_header_visible": menu_visible, "header_bgr": header}) + "\n")
                self.video_frames += 1
                self.next_frame = now + 0.1
            time.sleep(0.01)

    def tap(self, key):
        self.log("input", key=key, pid=self.process.pid, window=self.window)
        self.x11.send_key(self.process, self.window, key, True)
        self.wait(0.18)
        self.x11.send_key(self.process, self.window, key, False)
        self.wait(0.4)

    def click(self):
        self.log("input", mouse_button="left", pid=self.process.pid, window=self.window)
        for pressed in (True, False):
            self.x11.assert_owned(self.process, self.window)
            event = XPointerEvent()
            event.button = XButtonEvent(
                4 if pressed else 5, 0, 1, self.x11.display, self.window,
                self.x11.root, 0, int(time.monotonic() * 1000) & 0xffffffff,
                1, 1, 1, 1, 0 if pressed else 256, 1, 1,
            )
            if not self.x11.lib.XSendEvent(
                self.x11.display, self.window, 0, 4 if pressed else 8,
                C.cast(C.byref(event), C.POINTER(XEvent)),
            ):
                raise RuntimeError("XSendEvent failed for owned mouse button")
            self.x11.lib.XFlush(self.x11.display)
            self.wait(0.18 if pressed else 0.4)

    def screenshot(self, name, expected):
        path = self.shots / (name + ".png")
        command = ["ffmpeg", "-v", "error", "-y", "-f", "rawvideo", "-pixel_format", "bgr0", "-video_size", "1280x720", "-i", "pipe:0", "-frames:v", "1", str(path)]
        subprocess.run(command, input=self.pixels(), check=True)
        self.captures.append({"file": str(path.relative_to(self.directory)), "expected": expected, "window": self.window, "sha256": sha256(path), "validation": "visual review pending"})
        self.log("screenshot", **self.captures[-1])

    def start(self, name, arguments):
        case = self.directory / name
        case.mkdir()
        env = os.environ.copy()
        env["DISPLAY"] = self.args.display
        env["BORROW_FIGHTERS_DATA_DIR"] = str(case / "userdata")
        env["XDG_DATA_HOME"] = str(case / "xdg")
        env["BORROW_FIGHTERS_ASSET_DIR"] = str(self.root / "assets")
        # stdbuf execs the binary, preserving the child PID used for ownership.
        # Raylib's last FBO log must be observable even without frame capture.
        command = ["stdbuf", "-oL", "-eL", str(self.binary), *arguments]
        with (case / "game.log").open("wb") as log:
            self.process = subprocess.Popen(command, cwd=case, env=env, stdout=log, stderr=subprocess.STDOUT)
        self.window = None
        self.story_trace = None
        deadline = time.monotonic() + 20
        while self.window is None and time.monotonic() < deadline and self.process.poll() is None:
            self.window = self.x11.find_window(self.process.pid)
            time.sleep(0.03)
        if self.window is None:
            raise RuntimeError("Owned child window not found")
        self.log("started", name=name, command=command, pid=self.process.pid, window=self.window)
        return case

    def start_captured(self, name, arguments):
        capture = self.directory / name / "adventure"
        case = self.start(name, [*arguments, "--capture", str(capture)])
        self.story_trace = Telemetry(capture / "telemetry.jsonl")
        self.wait_for_story(case)
        self.observe(lambda sample: True, "first story frame", 20)
        return case

    def result(self, case):
        path = case / "adventure/result.json"
        if not path.is_file():
            raise AssertionError(f"Missing story result: {path}")
        return json.loads(path.read_text())

    def assert_waiting_at_title(self, case, name):
        sample = self.observe(lambda s: s["stage"] == "Complete", "completion prompt")
        log = (case / "game.log").read_text(errors="replace")
        self.check(name, not (case / "adventure/result.json").exists()
                   and log.count("Framebuffer object created successfully") == 1
                   and self.process.poll() is None and sample["waiting_for_continue"]
                   and not sample["continue_accepted"],
                   stage=sample["stage"], frames=sample["frame"])

    def next_segment(self, stage, first_tick, last_tick, label):
        before = self.story_trace.read()
        self.tap("Return")
        sample = self.observe(lambda s: s["frame"] > before["frame"], label)
        self.check(label, sample["stage"] == stage
                   and first_tick <= sample["stage_ticks"] < last_tick,
                   stage=sample["stage"], stage_ticks=sample["stage_ticks"])
        return sample

    def opening_to_logo(self):
        cuts = [3, 6, 9, 14, 19, 24, 29, 33, 37, 41, 48]
        for index, second in enumerate(cuts[:-1]):
            self.next_segment("Opening", second * 60, cuts[index + 1] * 60,
                              f"opening_segment_{second}s")

    def assert_direct_skip(self, case, stage, player_hp, enemy_hp, name):
        original_window = self.window
        self.tap("BackSpace")
        self.wait_for_menu(case)
        result = self.result(case)
        self.story_trace.read()
        self.check(name, result["final_stage"] == stage
                   and result["exit_reason"] == "Skipped"
                   and result["outcome"] == "Ongoing"
                   and result["player_hp"] == player_hp
                   and result["enemy_hp"] == enemy_hp,
                   result=result)
        self.check(f"{name}_same_window", self.x11.find_window(self.process.pid) == original_window)
        self.check(f"{name}_never_waited_at_completion",
                   all(s["stage"] != "Complete" for s in self.story_trace.samples))
        self.screenshot(name, "Main terminal menu after skipping the whole prologue")

    def stop_video(self):
        if self.video is not None:
            self.video.stdin.close()
            code = self.video.wait(timeout=20)
            self.video = None
            self.check("native_window_video_encoded", code == 0, frames=self.video_frames, fps=10)

    def wait_for_menu(self, case, timeout=35):
        # The story result precedes fighting texture/audio loading. Wait for its
        # second framebuffer, then allow menu fade/input polling to start.
        deadline = time.monotonic() + timeout
        while time.monotonic() < deadline:
            log = (case / "game.log").read_text(errors="replace")
            if log.count("Framebuffer object created successfully") >= 2:
                self.wait(1.5)
                self.log("menu_framebuffer_ready", pid=self.process.pid, window=self.window)
                return
            self.wait(0.1)
        raise TimeoutError("Fighting menu framebuffer did not load")

    def wait_for_story(self, case):
        deadline = time.monotonic() + 35
        while time.monotonic() < deadline:
            log = (case / "game.log").read_text(errors="replace")
            if "Framebuffer object created successfully" in log:
                self.wait(0.6)
                return
            self.wait(0.1)
        raise TimeoutError("Adventure framebuffer did not load")

    def close(self):
        self.stop_video()
        if self.process is not None and self.process.poll() is None:
            self.x11.request_close(self.process, self.window)
            try:
                self.process.wait(timeout=8)
            except subprocess.TimeoutExpired:
                self.process.terminate()
                self.process.wait(timeout=5)

    def run(self):
        success = False
        try:
            case = self.start_captured("transition", [])
            original_window = self.window
            initial = self.story_trace.read()
            self.check("default_entry_starts_with_ada", initial["stage"] == "AdaPrologue")
            cuts = [6, 13, 21, 29, 36, 40]
            for index, second in enumerate(cuts[:-1]):
                self.next_segment("AdaPrologue", second * 60, cuts[index + 1] * 60,
                                  f"ada_segment_{second}s")
            self.next_segment("RustMorning", 0, 150, "ada_last_segment_to_morning")
            cuts = [150, 225, 355, 500, 585, 840]
            for index, tick in enumerate(cuts[:-1]):
                self.next_segment("RustMorning", tick, cuts[index + 1],
                                  f"morning_segment_{tick}ticks")
            encounter = self.next_segment("Encounter", 0, 120,
                                          "morning_last_segment_to_encounter")
            opening = self.next_segment("Opening", 0, 180,
                                        "encounter_skip_to_presentation")
            self.check("encounter_skip_preserves_health_and_outcome",
                       opening["outcome"] == encounter["outcome"] == "Ongoing"
                       and opening["player"]["hp"] == encounter["player"]["hp"]
                       and opening["enemy"]["hp"] == encounter["enemy"]["hp"],
                       before=encounter, after=opening)
            self.check("skipped_encounter_does_not_invent_remorse",
                       all(s["stage"] != "Aftermath" for s in self.story_trace.samples))
            self.opening_to_logo()
            with (case / "window-video.log").open("wb") as log:
                self.video = subprocess.Popen(["ffmpeg", "-v", "error", "-y", "-f", "rawvideo", "-pixel_format", "bgr0", "-video_size", "1280x720", "-framerate", "10", "-i", "pipe:0", "-c:v", "libx264", "-preset", "veryfast", "-crf", "20", "-pix_fmt", "yuv420p", str(self.directory / "title-to-menu-native.mp4")], stdin=subprocess.PIPE, stderr=log)
            self.wait(3)
            self.screenshot("01-final-title", "Five characters behind BORROW FIGHTERS before completion")
            self.observe(lambda s: s["stage"] == "Complete", "natural title completion", 15)
            self.wait(3)
            self.assert_waiting_at_title(case, "natural_completion_waits_for_new_input")
            self.screenshot("02-any-key-prompt", "Final title with Aperte qualquer tecla para continuar")
            self.tap("z")
            self.wait_for_menu(case)
            result = self.result(case)
            self.check("fresh_unassigned_key_continues_to_menu",
                       result["final_stage"] == "Complete" and result["exit_reason"] == "Completed",
                       events=result["events"])
            self.check("same_native_window_after_story", self.x11.find_window(self.process.pid) == original_window, original_window=original_window, current_window=self.x11.find_window(self.process.pid))
            self.check("host_did_not_write_onboarding_marker", not list((case / "userdata").rglob("*onboarding*")))
            self.screenshot("02-main-menu", "Main terminal menu; Modo História selected; no completion menu or onboarding")
            self.tap("Return")
            self.screenshot("03-story-enter-inert", "Same main menu after Enter on Modo História")
            self.tap("space")
            self.screenshot("04-story-space-inert", "Same main menu after Space on Modo História")
            self.tap("Down")
            self.tap("Return")
            self.wait(1)
            self.screenshot("05-versus-roster", "Existing Linker character selection")
            self.tap("Escape")
            self.wait(1)
            self.screenshot("06-roster-back-menu", "Returned to main menu")
            self.tap("Up")
            for row, name in [(2, "training"), (3, "lore"), (4, "options"), (5, "how-to-play")]:
                # Returning from any submenu resets Main to its first row.
                for _ in range(row):
                    self.tap("Down")
                self.tap("Return")
                self.wait(1)
                self.screenshot(f"{6+row:02}-{name}", f"Existing {name} menu content")
                self.tap("Escape")
                self.wait(0.8)
            self.screenshot("12-final-main", "Main menu after visiting preserved submenus")
            self.check("explicit_guide_visit_records_only_test_userdata", (case / "userdata/onboarding-v1.seen").is_file())
            self.close()
            self.check("main_session_clean_exit", self.process.returncode == 0)
            log = (case / "game.log").read_text()
            self.check("audio_devices_handoff_without_crash", log.count("AUDIO: Device initialized successfully") == 2 and log.count("AUDIO: Device closed successfully") == 2)
            case = self.start_captured("held-enter", ["--start", "opening"])
            self.opening_to_logo()
            self.log("input_hold", key="Return", pid=self.process.pid, window=self.window)
            self.x11.send_key(self.process, self.window, "Return", True)
            self.observe(lambda s: s["stage"] == "Complete", "last segment skipped to completion")
            self.wait(3)
            self.assert_waiting_at_title(case, "held_skip_key_cannot_accept_completion")
            self.screenshot("held-enter-completion", "Completion prompt remains while final skip key is held")
            self.x11.send_key(self.process, self.window, "Return", False)
            self.wait(1)
            self.assert_waiting_at_title(case, "releasing_skip_key_cannot_accept_completion")
            self.tap("Return")
            self.wait_for_menu(case)
            self.check("fresh_enter_after_release_continues",
                       self.result(case)["exit_reason"] == "Completed")
            self.close()

            case = self.start_captured("mouse-continue", ["--start", "opening"])
            self.opening_to_logo()
            self.tap("Return")
            self.assert_waiting_at_title(case, "last_segment_skip_waits_for_mouse_confirmation")
            self.click()
            self.wait_for_menu(case)
            self.check("fresh_mouse_click_continues_to_menu",
                       self.result(case)["exit_reason"] == "Completed")
            self.screenshot("mouse-continued-menu", "Main menu after a new left mouse click confirms the final prompt")
            self.close()

            case = self.start_captured("skip-all-ada", [])
            sample = self.story_trace.read()
            self.screenshot("ada-skip-controls", "Ada scene with Tab reveal and segment/global skip controls")
            self.assert_direct_skip(case, "AdaPrologue", sample["player"]["hp"],
                                    sample["enemy"]["hp"], "global_skip_from_ada")
            self.close()

            case = self.start_captured("skip-all-encounter", ["--start", "encounter"])
            sample = self.story_trace.read()
            self.check("global_skip_encounter_starts_unpaused", not sample["paused"])
            self.assert_direct_skip(case, "Encounter", sample["player"]["hp"],
                                    sample["enemy"]["hp"], "global_skip_from_active_encounter")
            self.close()

            case = self.start_captured("skip-all-paused-encounter", ["--start", "encounter"])
            self.tap("Escape")
            paused = self.observe(lambda s: s["paused"], "paused encounter")
            self.screenshot("paused-skip-controls", "Paused encounter with whole-prologue skip control")
            self.assert_direct_skip(case, "Encounter", paused["player"]["hp"],
                                    paused["enemy"]["hp"], "global_skip_from_paused_encounter")
            self.close()

            case = self.start_captured("skip-all-opening", ["--start", "opening"])
            sample = self.story_trace.read()
            self.assert_direct_skip(case, "Opening", sample["player"]["hp"],
                                    sample["enemy"]["hp"], "global_skip_from_presentation")
            self.close()

            capture = self.directory / "hosted-review" / "adventure"
            case = self.start("hosted-review", ["--start", "opening", "--review", str(capture)])
            original_window = self.window
            self.story_trace = Telemetry(capture / "telemetry.jsonl")
            self.wait_for_story(case)
            self.wait_for_menu(case, timeout=120)
            result = self.result(case)
            self.story_trace.read()
            complete = [s for s in self.story_trace.samples if s["stage"] == "Complete"]
            first_accepted = next((i for i, s in enumerate(complete) if s["continue_accepted"]), None)
            self.check("hosted_review_finishes_confirmation_and_full_fade",
                       result["mode"] == "deterministic_review"
                       and result["exit_reason"] == "Completed"
                       and result["final_stage"] == "Complete"
                       and first_accepted is not None and first_accepted >= 180
                       and sum(s["continue_accepted"] for s in complete) >= 24,
                       result=result, completion_frames=len(complete),
                       first_accepted_frame=first_accepted)
            self.check("hosted_review_preserves_native_window",
                       self.x11.find_window(self.process.pid) == original_window)
            self.screenshot("hosted-review-menu", "Visible menu after deterministic review waits at the prompt and finishes its fade")
            self.close()

            case = self.start("close-in-adventure", ["--start", "opening"])
            self.wait_for_story(case)
            self.tap("Escape")
            self.x11.request_close(self.process, self.window)
            self.process.wait(timeout=10)
            log = (case / "game.log").read_text()
            self.check("closing_adventure_never_opens_menu", self.process.returncode == 0 and log.count("AUDIO: Device initialized successfully") == 1, exit_code=self.process.returncode)

            case = self.start("frame-limit", ["--start", "opening", "--frames", "90"])
            self.process.wait(timeout=40)
            log = (case / "game.log").read_text()
            self.check("frame_limit_never_opens_menu", self.process.returncode == 0 and log.count("AUDIO: Device initialized successfully") == 1, exit_code=self.process.returncode)

            case = self.start("hidden-to-visible", ["--hidden", "--start", "opening"])
            self.wait_for_story(case)
            before = subprocess.check_output(["xwininfo", "-id", str(self.window)], text=True)
            self.check("requested_hidden_window_starts_unmapped", "Map State: IsUnMapped" in before)
            self.tap("BackSpace")
            self.wait_for_menu(case)
            after = subprocess.check_output(["xwininfo", "-id", str(self.window)], text=True)
            self.check("menu_maps_previously_hidden_window", "Map State: IsViewable" in after)
            self.screenshot("hidden-now-visible-menu", "Visible main menu after hidden story was globally skipped")
            self.close()
            self.check("user_catalog_unchanged", sha256(self.catalog) == self.catalog_hash)
            self.check("binary_unchanged_during_review", sha256(self.binary) == self.binary_hash)
            self.check("no_x11_errors", not self.x11.errors, errors=self.x11.errors)
            success = True
        finally:
            self.close()
            (self.directory / "native-checks.json").write_text(json.dumps({"success": success, "checks": self.checks, "captures": self.captures, "executable_sha256": self.binary_hash, "catalog_sha256": self.catalog_hash, "physical_gamepad_tested": False, "audio_listening_performed": False, "input": "XSendEvent directed to owned PID/window", "capture": "XGetImage of that window, unchanged source pixels"}, indent=2, ensure_ascii=False) + "\n")
            self.x11.close()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--executable", required=True)
    parser.add_argument("--output-directory", required=True)
    parser.add_argument("--display", default=os.environ.get("DISPLAY", ":0"))
    Review(parser.parse_args()).run()


if __name__ == "__main__":
    main()
