#!/usr/bin/env python3
"""Check story-menu routing and checkpoint resumption through owned X11 windows.

Three launches share one isolated userdata directory. The game alone writes its
profile; this harness reads it to confirm real interaction/continuation effects.
Keyboard and pointer events target only the owned child window. Screenshots use
XGetImage of that window, without desktop capture, focus changes or pixel edits.
Character-selection screenshots still require visual inspection.
"""
from __future__ import annotations

import argparse
import ctypes as C
import json
import os
from pathlib import Path
import subprocess
import sys
import time

from capture_match_flow_x11 import Display, Window, XEvent, sha256
from capture_story_menu_x11 import Review, XButtonEvent, XPointerEvent


class XMotionEvent(C.Structure):
    _fields_ = [
        ("type", C.c_int), ("serial", C.c_ulong), ("send_event", C.c_int),
        ("display", Display), ("window", Window), ("root", Window),
        ("subwindow", Window), ("time", C.c_ulong), ("x", C.c_int),
        ("y", C.c_int), ("x_root", C.c_int), ("y_root", C.c_int),
        ("state", C.c_uint), ("is_hint", C.c_char), ("same_screen", C.c_int),
    ]


class MotionEvent(C.Union):
    _fields_ = [("motion", XMotionEvent), ("pad", C.c_long * 24)]


class XFocusEvent(C.Structure):
    _fields_ = [("type", C.c_int), ("serial", C.c_ulong), ("send_event", C.c_int),
                ("display", Display), ("window", Window),
                ("mode", C.c_int), ("detail", C.c_int)]


class XCrossingEvent(C.Structure):
    _fields_ = [
        ("type", C.c_int), ("serial", C.c_ulong), ("send_event", C.c_int),
        ("display", Display), ("window", Window), ("root", Window),
        ("subwindow", Window), ("time", C.c_ulong), ("x", C.c_int),
        ("y", C.c_int), ("x_root", C.c_int), ("y_root", C.c_int),
        ("mode", C.c_int), ("detail", C.c_int), ("same_screen", C.c_int),
        ("focus", C.c_int), ("state", C.c_uint),
    ]


class PointerEntryEvent(C.Union):
    _fields_ = [("focus", XFocusEvent), ("crossing", XCrossingEvent),
                ("pad", C.c_long * 24)]


class ChapterHostReview(Review):
    def __init__(self, args):
        super().__init__(args)
        self.userdata = self.directory / "userdata"
        self.profile_path = self.userdata / "adventure/campaign-v1.json"
        self.sessions = []
        self.failure = None
        self.x11.lib.XGetInputFocus.argtypes = [Display, C.POINTER(Window), C.POINTER(C.c_int)]

    def wait(self, duration):
        if time.monotonic() - self.started > self.args.timeout:
            raise TimeoutError("Chapter host review exceeded its process deadline")
        super().wait(duration)

    def start(self, name, arguments):
        case = self.directory / name
        case.mkdir()
        self.case = case
        env = os.environ.copy()
        env.update(DISPLAY=self.args.display, BORROW_FIGHTERS_DATA_DIR=str(self.userdata),
                   BORROW_FIGHTERS_ASSET_DIR=str(self.root / "assets"),
                   XDG_DATA_HOME=str(self.directory / "xdg"))
        command = ["stdbuf", "-oL", "-eL", str(self.binary), *arguments]
        with (case / "game.log").open("wb") as log:
            self.process = subprocess.Popen(command, cwd=case, env=env,
                                            stdout=log, stderr=subprocess.STDOUT)
        self.window = None
        self.story_trace = None
        deadline = time.monotonic() + 25
        while self.window is None and time.monotonic() < deadline and self.process.poll() is None:
            self.window = self.x11.find_window(self.process.pid)
            time.sleep(.03)
        if self.window is None:
            raise RuntimeError("Owned host window was not created")
        self.sessions.append({"case": name, "pid": self.process.pid, "window": self.window})
        self.log("started", name=name, command=command, pid=self.process.pid, window=self.window)
        # _NET_WM_PID appears before the fresh window is mapped. Its first
        # framebuffer also confirms asset preparation before any XGetImage.
        self.await_condition(lambda: self.framebuffers() > 0,
                             "first native framebuffer", 35)
        self.wait(.5)
        return case

    def title(self):
        self.x11.assert_owned(self.process, self.window)
        actual_type, count, after = C.c_ulong(), C.c_ulong(), C.c_ulong()
        actual_format = C.c_int()
        data = C.POINTER(C.c_ubyte)()
        status = self.x11.lib.XGetWindowProperty(
            self.x11.display, self.window, self.x11.atom("_NET_WM_NAME"), 0, 512,
            0, 0, C.byref(actual_type), C.byref(actual_format), C.byref(count),
            C.byref(after), C.byref(data))
        try:
            if status or actual_format.value != 8 or not data:
                return ""
            return C.string_at(data, count.value).decode("utf-8", errors="replace")
        finally:
            if data:
                self.x11.lib.XFree(data)

    def await_condition(self, predicate, label, timeout=15):
        deadline = time.monotonic() + timeout
        while time.monotonic() < deadline:
            if predicate():
                return
            self.wait(.05)
        raise TimeoutError(f"Did not observe {label}; title={self.title()}; profile={self.profile()}")

    def await_title(self, expected):
        self.await_condition(lambda: self.title() == expected, expected, 25)
        self.wait(.5)

    def framebuffers(self):
        return (self.case / "game.log").read_text(errors="replace").count(
            "Framebuffer object created successfully")

    def first_ada_beat(self, name):
        # The native progress strip paints all beats up to the current one
        # gold. One gold mark proves the first beat without OCR or exact frames.
        pixels = self.pixels()
        marks = [tuple(pixels[(27 * 1280 + 1087 + 23 * i) * 4:
                              (27 * 1280 + 1087 + 23 * i) * 4 + 3])
                 for i in range(6)]
        gold = [max(abs(a-b) for a, b in zip(mark, (92, 177, 232))) <= 8
                for mark in marks]
        self.check(name, gold == [True, False, False, False, False, False],
                   progress_marks_bgr=marks)

    def await_prologue(self, previous_framebuffers=0):
        # The host sets Ada's title before loading its textures. Wait for the
        # new render target and visible content before sending the first input.
        self.await_title("Borrow Fighters — Ada Lovelace")
        self.await_condition(lambda: self.framebuffers() > previous_framebuffers,
                             "new adventure framebuffer", 35)

        def drawn():
            pixels = self.pixels()
            offsets = [(y * 1280 + x) * 4
                       for y in range(60, 660, 32) for x in range(40, 1240, 32)]
            return sum(max(pixels[o:o+3]) > 35 for o in offsets) > len(offsets) / 10

        self.await_condition(drawn, "visible Ada content", 15)
        self.wait(.6)

    def profile(self):
        if not self.profile_path.exists():
            return None
        return json.loads(self.profile_path.read_text(encoding="utf-8"))

    def checkpoint_is(self, stage):
        profile = self.profile()
        return profile is not None and (profile.get("checkpoint") or {}).get("stage") == stage

    def main_menu(self):
        def visible():
            if self.title() != "Borrow Fighters":
                return False
            pixels = self.pixels()
            header = [tuple(pixels[(110 * 1280 + x) * 4:(110 * 1280 + x) * 4 + 3])
                      for x in (670, 800, 1210)]
            return all(abs(b - 47) <= 8 and abs(g - 42) <= 8 and abs(r - 28) <= 8
                       for b, g, r in header)
        self.await_condition(visible, "main terminal menu", 30)
        self.wait(.6)

    def click_row(self, x, y):
        # MotionNotify updates the game's own cursor callback. No pointer warp
        # or global cursor movement is used, even when clicking an unselected row.
        self.x11.assert_owned(self.process, self.window)
        focus_before = self.server_focus()
        # The menu accepts pointer input only after the native callbacks report
        # focus and pointer entry. Deliver those notifications to this child;
        # XSendEvent does not change the server's input focus or physical cursor.
        entry = PointerEntryEvent()
        entry.focus = XFocusEvent(9, 0, 1, self.x11.display, self.window, 0, 3)
        self.x11.lib.XSendEvent(self.x11.display, self.window, 0, 1 << 21,
                               C.cast(C.byref(entry), C.POINTER(XEvent)))
        entry = PointerEntryEvent()
        entry.crossing = XCrossingEvent(7, 0, 1, self.x11.display, self.window,
                                       self.x11.root, 0, 0, x, y, x, y, 0, 3, 1, 1, 0)
        self.x11.lib.XSendEvent(self.x11.display, self.window, 0, 16,
                               C.cast(C.byref(entry), C.POINTER(XEvent)))
        event = MotionEvent()
        event.motion = XMotionEvent(6, 0, 1, self.x11.display, self.window,
                                   self.x11.root, 0, int(time.monotonic() * 1000) & 0xffffffff,
                                   x, y, x, y, 0, b"\0", 1)
        if not self.x11.lib.XSendEvent(self.x11.display, self.window, 0, 64,
                                      C.cast(C.byref(event), C.POINTER(XEvent))):
            raise RuntimeError("Owned pointer motion was rejected")
        self.x11.lib.XFlush(self.x11.display)
        self.wait(.12)
        for pressed in (True, False):
            event = XPointerEvent()
            event.button = XButtonEvent(4 if pressed else 5, 0, 1, self.x11.display,
                                       self.window, self.x11.root, 0,
                                       int(time.monotonic() * 1000) & 0xffffffff,
                                       x, y, x, y, 0 if pressed else 256, 1, 1)
            self.x11.assert_owned(self.process, self.window)
            if not self.x11.lib.XSendEvent(self.x11.display, self.window, 0, 4 if pressed else 8,
                                          C.cast(C.byref(event), C.POINTER(XEvent))):
                raise RuntimeError("Owned click was rejected")
            self.x11.lib.XFlush(self.x11.display)
            self.wait(.18 if pressed else .4)
        self.log("pointer_click", x=x, y=y, pid=self.process.pid, window=self.window)
        self.check("owned_pointer_notifications_preserve_server_focus",
                   self.server_focus() == focus_before, server_focus=focus_before)

    def server_focus(self):
        focus, revert = Window(), C.c_int()
        self.x11.lib.XGetInputFocus(self.x11.display, C.byref(focus), C.byref(revert))
        return focus.value

    def hold_for(self, key, seconds):
        self.x11.send_key(self.process, self.window, key, True)
        try:
            self.wait(seconds)
        finally:
            self.x11.send_key(self.process, self.window, key, False)
        self.wait(.12)

    def leave_campaign(self):
        self.tap("Escape")
        self.tap("Down")
        self.tap("Down")
        self.tap("Return")
        self.main_menu()

    def advance_conversation(self, checkpoint):
        self.tap("e")
        self.wait(1.7)
        for _ in range(3):
            self.tap("Return")
        self.await_condition(lambda: self.checkpoint_is(checkpoint), checkpoint, 12)

    def sequence(self):
        self.start("01-first-launch", [])
        self.await_prologue()
        self.check("first_launch_shows_ada_without_a_fabricated_profile", self.profile() is None)
        self.first_ada_beat("first_launch_starts_with_adas_research")
        self.screenshot("01-first-launch-ada", "Ada on first launch")
        self.tap("BackSpace")
        self.main_menu()
        skipped = self.profile()
        self.check("skipping_records_seen_without_a_played_victory",
                   skipped is not None and skipped["prologue_seen"]
                   and not skipped["prologue_played_victory"] and skipped["checkpoint"] is None,
                   profile=skipped)
        self.close()
        self.check("first_host_closes_normally", self.process.returncode == 0)

        self.start("02-returning-launch", [])
        self.main_menu()
        self.check("returning_launch_reaches_the_menu_without_replaying_ada", self.profile() == skipped)
        self.screenshot("02-returning-main-menu", "Saved startup goes directly to the terminal menu")
        self.close()
        self.check("returning_host_closes_normally", self.process.returncode == 0)
        self.start("03-explicit-menu", ["--menu"])
        self.main_menu()
        owned_window = self.window
        self.check("explicit_menu_entry_preserves_the_existing_profile", self.profile() == skipped)
        # Main row0 uses the same 1280×720 layout as its native hit test.
        self.click_row(940, 198)
        self.await_title("Borrow Fighters — Depois do silêncio")
        self.check("pointer_click_opens_the_campaign_submenu", self.profile()["checkpoint"] is None)
        self.screenshot("03-campaign-submenu", "Modo História offers Iniciar capítulo, Rever prólogo and Voltar")
        self.click_row(640, 255)
        self.await_condition(lambda: self.checkpoint_is("street_start"), "new chapter checkpoint")
        self.check("pointer_click_starts_the_new_chapter", self.checkpoint_is("street_start"))
        self.screenshot("04-new-chapter-arrival", "Canonical aftermath starts with its own introduction")
        self.tap("Return")  # Explicitly settle the chapter's introductory shot.
        self.hold_for("d", .60)
        self.advance_conversation("driver_checked")
        driver_profile = self.profile()
        self.check("driver_interaction_creates_a_real_safe_checkpoint",
                   driver_profile["checkpoint"]["stage"] == "driver_checked"
                   and not driver_profile["prologue_played_victory"], profile=driver_profile)
        self.screenshot("05-driver-checked", "Driver conversation completed; objective points to the shop")

        self.leave_campaign()
        self.check("pause_returns_to_the_same_host_and_preserves_the_checkpoint",
                   self.window == owned_window and self.x11.find_window(self.process.pid) == owned_window
                   and self.profile() == driver_profile)
        self.tap("Return")
        self.await_title("Borrow Fighters — Depois do silêncio")
        self.screenshot("06-continue-submenu", "Continuar capítulo appears for the saved driver checkpoint")
        self.click_row(640, 255)
        # A resumed DriverChecked spawns at x1265. Reaching the shop and saving
        # ShopChecked without advancing an introduction proves control resumed.
        self.hold_for("a", 2.35)
        self.advance_conversation("shop_checked")
        resumed_profile = self.profile()
        self.check("continue_resumes_control_and_progress_without_a_new_intro",
                   resumed_profile["checkpoint"]["stage"] == "shop_checked"
                   and not resumed_profile["prologue_played_victory"], profile=resumed_profile)
        self.screenshot("07-resumed-shop-complete", "Shop conversation completed after Continue")

        self.leave_campaign()
        self.tap("Return")
        self.await_title("Borrow Fighters — Depois do silêncio")
        self.tap("Down")
        self.tap("Down")
        previous_framebuffers = self.framebuffers()
        self.tap("Return")
        self.await_prologue(previous_framebuffers)
        self.first_ada_beat("replay_starts_with_adas_research_without_reusing_confirm")
        self.screenshot("08-explicit-prologue-replay", "Rever prólogo explicitly returns to Ada")
        self.tap("BackSpace")
        self.main_menu()
        self.check("explicit_replay_and_skip_preserve_the_existing_campaign",
                   self.profile() == resumed_profile and self.x11.find_window(self.process.pid) == owned_window)

        original = self.pixels()
        self.tap("Down")
        self.tap("Return")
        self.wait(1.0)
        selection = self.pixels()
        offsets = [(y * 1280 + x) * 4 for y in range(20, 650, 8) for x in range(20, 1260, 8)]
        changed = sum(max(abs(selection[o+c]-original[o+c]) for c in range(3)) > 40 for o in offsets) / len(offsets)
        # Both screens have large dark backgrounds, so total changed area is
        # not a reliable oracle. The roster has distinct P1/P2 selection edges.
        marks = [((x, 220), (228, 231, 76)) for x in (350, 420, 488)]
        marks += [((x, 359), (96, 190, 255)) for x in (500, 560, 620)]
        edges = [tuple(selection[(y * 1280 + x) * 4:(y * 1280 + x) * 4 + 3])
                 for (x, y), _ in marks]
        self.check("versus_roster_has_both_native_selection_borders",
                   all(max(abs(a-b) for a, b in zip(actual, expected)) <= 8
                       for actual, (_, expected) in zip(edges, marks)),
                   border_bgr=edges, changed_sample_fraction=changed,
                   exact_screen_validation="inspect screenshot")
        self.screenshot("09-versus-selection", "Existing Versus character selection remains reachable")
        self.tap("Escape")
        self.main_menu()
        self.screenshot("10-final-main-menu", "Main menu after returning from Versus")
        self.check("versus_visit_preserves_the_campaign_save", self.profile() == resumed_profile)

    def run(self):
        try:
            self.sequence()
            self.check("binary_and_text_catalog_unchanged", sha256(self.binary) == self.binary_hash
                       and sha256(self.catalog) == self.catalog_hash)
            self.check("no_x11_errors", not self.x11.errors, errors=self.x11.errors)
        except (Exception, KeyboardInterrupt) as error:
            self.failure = f"{type(error).__name__}: {error}"
            self.log("failure", detail=self.failure)
            if self.process is not None and self.process.poll() is None:
                try:
                    self.screenshot("failure", "Native screen at the failed check")
                except Exception as capture_error:
                    self.log("failure_capture_unavailable", detail=str(capture_error))
        finally:
            try:
                self.close()
            except Exception as error:
                self.failure = self.failure or f"Cleanup failed: {type(error).__name__}: {error}"
            report = {"success": self.failure is None, "failure": self.failure,
                      "checks": self.checks, "captures": self.captures, "sessions": self.sessions,
                      "profile": self.profile(), "elapsed_seconds": round(time.monotonic()-self.started, 3),
                      "executable_sha256": self.binary_hash,
                      "process_exit_code": self.process.returncode if self.process else None,
                      "input": "XSendEvent addressed to the owned PID/window; no global pointer movement",
                      "capture": "XGetImage of the owned window; unchanged source pixels",
                      "save_edits_by_harness": False, "physical_gamepad_tested": False,
                      "audio_listening_performed": False}
            (self.directory / "native-checks.json").write_text(json.dumps(report, ensure_ascii=False, indent=2)+"\n", encoding="utf-8")
            self.x11.close()
        return int(self.failure is not None)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--executable", default=str(Path(__file__).resolve().parents[2] / "target/debug/borrow-story"))
    parser.add_argument("--output-directory", required=True)
    parser.add_argument("--display", default=os.environ.get("DISPLAY", ":0"))
    parser.add_argument("--timeout", type=float, default=180)
    return ChapterHostReview(parser.parse_args()).run()


if __name__ == "__main__":
    sys.exit(main())
