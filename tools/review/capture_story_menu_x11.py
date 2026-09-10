#!/usr/bin/env python3
"""Review the composed story/menu through one process-owned native X11 window.

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
from capture_match_flow_x11 import Display, Window, X11, sha256


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
                    out.write(json.dumps({"frame": self.video_frames, "elapsed": now - self.started, "stage": sample["stage"] if sample else None, "stage_ticks": sample["stage_ticks"] if sample else None, "menu_header_visible": menu_visible, "header_bgr": header}) + "\n")
                self.video_frames += 1
                self.next_frame = now + 0.1
            time.sleep(0.01)

    def tap(self, key):
        self.log("input", key=key, pid=self.process.pid, window=self.window)
        self.x11.send_key(self.process, self.window, key, True)
        self.wait(0.18)
        self.x11.send_key(self.process, self.window, key, False)
        self.wait(0.4)

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
        deadline = time.monotonic() + 20
        while self.window is None and time.monotonic() < deadline and self.process.poll() is None:
            self.window = self.x11.find_window(self.process.pid)
            time.sleep(0.03)
        if self.window is None:
            raise RuntimeError("Owned child window not found")
        self.log("started", name=name, command=command, pid=self.process.pid, window=self.window)
        return case

    def stop_video(self):
        if self.video is not None:
            self.video.stdin.close()
            code = self.video.wait(timeout=20)
            self.video = None
            self.check("native_window_video_encoded", code == 0, frames=self.video_frames, fps=10)

    def wait_for_menu(self, case):
        # The story result precedes fighting texture/audio loading. Wait for its
        # second framebuffer, then allow menu fade/input polling to start.
        deadline = time.monotonic() + 35
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
            case = self.start("transition", ["--start", "opening", "--capture", str(self.directory / "adventure")])
            original_window = self.window
            trace = Telemetry(self.directory / "adventure/telemetry.jsonl")
            self.story_trace = trace
            deadline = time.monotonic() + 100
            last_log = 0
            while time.monotonic() < deadline:
                sample = trace.read()
                if sample and sample["stage_ticks"] >= 40 * 60:
                    break
                self.wait(0.1)
                if time.monotonic() - last_log > 15:
                    self.log("awaiting_title", stage_ticks=sample["stage_ticks"] if sample else None)
                    last_log = time.monotonic()
            else:
                raise TimeoutError("Opening did not reach title")
            with (case / "window-video.log").open("wb") as log:
                self.video = subprocess.Popen(["ffmpeg", "-v", "error", "-y", "-f", "rawvideo", "-pixel_format", "bgr0", "-video_size", "1280x720", "-framerate", "10", "-i", "pipe:0", "-c:v", "libx264", "-preset", "veryfast", "-crf", "20", "-pix_fmt", "yuv420p", str(self.directory / "title-to-menu-native.mp4")], stdin=subprocess.PIPE, stderr=log)
            self.wait(4)
            self.screenshot("01-final-title", "Five characters behind BORROW FIGHTERS before main menu")
            result_path = self.directory / "adventure/result.json"
            deadline = time.monotonic() + 25
            while not result_path.exists() and time.monotonic() < deadline:
                self.wait(0.1)
            if not result_path.exists():
                raise TimeoutError("Hosted adventure did not return")
            self.wait_for_menu(case)
            result = json.loads(result_path.read_text())
            self.check("opening_completed_before_menu", result["final_stage"] == "Complete", events=result["events"])
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
            case = self.start("close-in-adventure", ["--start", "opening"])
            self.wait_for_story(case)
            self.tap("Escape")
            self.screenshot("13-adventure-paused", "Paused adventure before Backspace exits the whole host")
            self.x11.send_key(self.process, self.window, "BackSpace", True)
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
            self.tap("Return")
            self.wait_for_menu(case)
            after = subprocess.check_output(["xwininfo", "-id", str(self.window)], text=True)
            self.check("menu_maps_previously_hidden_window", "Map State: IsViewable" in after)
            self.screenshot("14-hidden-now-visible-menu", "Visible main menu after hidden story completed")
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
