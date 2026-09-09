#!/usr/bin/env python3
"""Collect native X11 Borrow Fighters frames through a process-owned window.

Uses only Python stdlib and the host's libX11. XSendEvent is directed to the
window whose _NET_WM_PID matches our subprocess; there is no global input,
focus manipulation, desktop capture, or access to an existing game process.
Asset lookup preserves an adjacent bundle, or explicitly uses this repository's
assets when reviewing a target/ build. Python 3.8+ is supported.

Named expected states require visual review; successful delivery is not proof
that the expected UI state appeared. No physical input or audio listening is
claimed by this harness.
"""
from __future__ import annotations

import argparse
import ctypes as C
import ctypes.util
import hashlib
import json
import os
from pathlib import Path
import shutil
import sys
import subprocess
import tempfile
import time
from datetime import datetime, timezone


Window = C.c_ulong
Display = C.c_void_p


class XKeyEvent(C.Structure):
    _fields_ = [
        ("type", C.c_int), ("serial", C.c_ulong), ("send_event", C.c_int),
        ("display", Display), ("window", Window), ("root", Window),
        ("subwindow", Window), ("time", C.c_ulong), ("x", C.c_int),
        ("y", C.c_int), ("x_root", C.c_int), ("y_root", C.c_int),
        ("state", C.c_uint), ("keycode", C.c_uint), ("same_screen", C.c_int),
    ]


class XClientData(C.Union):
    _fields_ = [("b", C.c_char * 20), ("s", C.c_short * 10), ("l", C.c_long * 5)]


class XClientMessageEvent(C.Structure):
    _fields_ = [
        ("type", C.c_int), ("serial", C.c_ulong), ("send_event", C.c_int),
        ("display", Display), ("window", Window), ("message_type", C.c_ulong),
        ("format", C.c_int), ("data", XClientData),
    ]


class XEvent(C.Union):
    _fields_ = [("key", XKeyEvent), ("client", XClientMessageEvent), ("pad", C.c_long * 24)]


class XErrorEvent(C.Structure):
    _fields_ = [
        ("type", C.c_int), ("display", Display), ("resourceid", C.c_ulong),
        ("serial", C.c_ulong), ("error_code", C.c_ubyte),
        ("request_code", C.c_ubyte), ("minor_code", C.c_ubyte),
    ]


class X11:
    """Small Xlib boundary; mutations always require an owned process/window."""

    def __init__(self, display_name: str | None):
        library = ctypes.util.find_library("X11")
        if not library:
            raise RuntimeError("Host libX11 was not found; no dependencies were installed")
        self.lib = C.CDLL(library)
        signatures = {
            "XOpenDisplay": ([C.c_char_p], Display),
            "XCloseDisplay": ([Display], C.c_int),
            "XDefaultRootWindow": ([Display], Window),
            "XInternAtom": ([Display, C.c_char_p, C.c_int], C.c_ulong),
            "XQueryTree": ([Display, Window, C.POINTER(Window), C.POINTER(Window), C.POINTER(C.POINTER(Window)), C.POINTER(C.c_uint)], C.c_int),
            "XGetWindowProperty": ([Display, Window, C.c_ulong, C.c_long, C.c_long, C.c_int, C.c_ulong, C.POINTER(C.c_ulong), C.POINTER(C.c_int), C.POINTER(C.c_ulong), C.POINTER(C.c_ulong), C.POINTER(C.POINTER(C.c_ubyte))], C.c_int),
            "XFree": ([C.c_void_p], C.c_int),
            "XStringToKeysym": ([C.c_char_p], C.c_ulong),
            "XKeysymToKeycode": ([Display, C.c_ulong], C.c_ubyte),
            "XSendEvent": ([Display, Window, C.c_int, C.c_long, C.POINTER(XEvent)], C.c_int),
            "XFlush": ([Display], C.c_int),
            "XSync": ([Display, C.c_int], C.c_int),
        }
        for name, (args, result) in signatures.items():
            fn = getattr(self.lib, name)
            fn.argtypes = args
            fn.restype = result
        self.errors: list[dict] = []
        error_type = C.CFUNCTYPE(C.c_int, Display, C.POINTER(XErrorEvent))
        def on_error(_display, event):
            error = event.contents
            self.errors.append({"resource": int(error.resourceid), "code": int(error.error_code), "request": int(error.request_code)})
            return 0
        self.error_handler = error_type(on_error)
        self.lib.XSetErrorHandler.argtypes = [error_type]
        self.lib.XSetErrorHandler.restype = C.c_void_p
        self.lib.XSetErrorHandler(self.error_handler)
        self.display = self.lib.XOpenDisplay(display_name.encode() if display_name else None)
        if not self.display:
            raise RuntimeError(f"Could not open X11 display {display_name or os.environ.get('DISPLAY')!r}")
        self.root = int(self.lib.XDefaultRootWindow(self.display))
        self.pid_atom = self.atom("_NET_WM_PID")
        self.protocols_atom = self.atom("WM_PROTOCOLS")
        self.delete_atom = self.atom("WM_DELETE_WINDOW")

    def atom(self, name: str) -> int:
        return int(self.lib.XInternAtom(self.display, name.encode(), 0))

    def children(self, window: int) -> list[int]:
        root, parent = Window(), Window()
        children = C.POINTER(Window)()
        count = C.c_uint()
        ok = self.lib.XQueryTree(self.display, window, C.byref(root), C.byref(parent), C.byref(children), C.byref(count))
        try:
            return [int(children[i]) for i in range(count.value)] if ok else []
        finally:
            if children:
                self.lib.XFree(children)

    def window_pid(self, window: int) -> int | None:
        actual_type, count, after = C.c_ulong(), C.c_ulong(), C.c_ulong()
        actual_format = C.c_int()
        data = C.POINTER(C.c_ubyte)()
        status = self.lib.XGetWindowProperty(self.display, window, self.pid_atom, 0, 1, 0, 6, C.byref(actual_type), C.byref(actual_format), C.byref(count), C.byref(after), C.byref(data))
        try:
            if status != 0 or actual_format.value != 32 or count.value != 1 or not data:
                return None
            return int(C.cast(data, C.POINTER(C.c_ulong))[0])
        finally:
            if data:
                self.lib.XFree(data)

    def find_window(self, pid: int) -> int | None:
        queue = self.children(self.root)
        examined = 0
        while queue and examined < 10000:
            window = queue.pop(0)
            examined += 1
            if self.window_pid(window) == pid:
                return window
            queue.extend(self.children(window))
        return None

    def assert_owned(self, process: subprocess.Popen, window: int) -> None:
        if process.poll() is not None:
            raise RuntimeError(f"Owned game exited with status {process.returncode}")
        owner = self.window_pid(window)
        if owner != process.pid:
            raise RuntimeError(f"Window ownership changed: {owner} != {process.pid}")

    def send_key(self, process: subprocess.Popen, window: int, key: str, pressed: bool) -> None:
        self.assert_owned(process, window)
        symbol = self.lib.XStringToKeysym(key.encode("ascii"))
        code = int(self.lib.XKeysymToKeycode(self.display, symbol))
        if not code:
            raise RuntimeError(f"No X11 keycode for {key}")
        event = XEvent()
        event.key = XKeyEvent(
            2 if pressed else 3, 0, 1, self.display, window, self.root, 0,
            int(time.monotonic() * 1000) & 0xffffffff, 1, 1, 1, 1, 0, code, 1,
        )
        if not self.lib.XSendEvent(self.display, window, 0, 1 if pressed else 2, C.byref(event)):
            raise RuntimeError(f"XSendEvent failed for owned {key}")
        self.lib.XFlush(self.display)

    def request_close(self, process: subprocess.Popen, window: int) -> None:
        self.assert_owned(process, window)
        event = XEvent()
        event.client.type = 33
        event.client.send_event = 1
        event.client.display = self.display
        event.client.window = window
        event.client.message_type = self.protocols_atom
        event.client.format = 32
        event.client.data.l[0] = self.delete_atom
        event.client.data.l[1] = 0
        self.lib.XSendEvent(self.display, window, 0, 0, C.byref(event))
        self.lib.XFlush(self.display)

    def close(self) -> None:
        if self.display:
            self.lib.XCloseDisplay(self.display)
            self.display = None


def sha256(path: Path) -> str:
    result = hashlib.sha256()
    with path.open("rb") as source:
        for block in iter(lambda: source.read(1024 * 1024), b""):
            result.update(block)
    return result.hexdigest()


def resolve_assets(executable: Path, requested: str | None):
    """Preserve bundled lookup; use a source checkout only for unbundled builds."""
    if requested:
        directory = Path(requested).resolve(strict=True)
        source = "explicit --asset-directory override"
        override = True
    else:
        candidates = [(executable.parent / "assets", "adjacent executable bundle", False)]
        if executable.parent.name == "bin":
            candidates.append((executable.parent.parent / "assets", "installed bundle", False))
        repository = Path(__file__).resolve().parents[2]
        candidates.append((repository / "assets", "review harness repository", True))
        candidates.append((Path.cwd() / "assets", "caller source directory", True))
        found = next((item for item in candidates if item[0].is_dir()), None)
        if found is None:
            raise RuntimeError("No bundled/source assets found; provide --asset-directory")
        directory, source, override = found
        directory = directory.resolve()
    if not directory.is_dir():
        raise RuntimeError("Asset directory is not a directory: {}".format(directory))
    marker = directory / "candidates" / "rust" / "rust-fighter.sprite.json"
    if not marker.is_file():
        raise RuntimeError("Asset root is missing the Rust runtime manifest: {}".format(marker))
    return directory, source, override


class Review:
    def __init__(self, args, x11: X11):
        self.args, self.x11 = args, x11
        self.exe = Path(args.executable).resolve(strict=True)
        if not self.exe.is_file() or not os.access(self.exe, os.X_OK):
            raise RuntimeError("Provided executable is not an executable file")
        if args.output_directory:
            self.evidence = Path(args.output_directory).absolute()
            self.evidence.mkdir(parents=True, exist_ok=False)
        else:
            self.evidence = Path(tempfile.mkdtemp(prefix="bf-linux-roster-flow-"))
        self.cwd = self.evidence / "cwd"
        self.shots = self.evidence / "screenshots"
        self.data = self.evidence / "userdata"
        self.cwd.mkdir()
        self.shots.mkdir()
        self.asset_root, self.asset_source, self.asset_override = resolve_assets(self.exe, args.asset_directory)
        self.started = time.monotonic()
        self.started_utc = datetime.now(timezone.utc).isoformat()
        self.process = None
        self.window = None
        self.captures = []
        self.held_keys = set()
        self.success = False

    def log(self, kind: str, **detail) -> None:
        event = {"at_utc": datetime.now(timezone.utc).isoformat(), "elapsed_seconds": round(time.monotonic() - self.started, 3), "kind": kind, **detail}
        encoded = json.dumps(event, ensure_ascii=False)
        with (self.evidence / "events.jsonl").open("a", encoding="utf-8") as target:
            target.write(encoded + "\n")
        print(encoded, flush=True)

    def assert_owned(self):
        self.x11.assert_owned(self.process, self.window)

    def wait(self, seconds: float):
        end = time.monotonic() + seconds
        next_log = time.monotonic() + 15
        while time.monotonic() < end:
            time.sleep(min(0.25, max(0, end - time.monotonic())))
            self.assert_owned()
            if time.monotonic() >= next_log:
                self.log("wait", remaining_seconds=round(max(0, end-time.monotonic()), 1), process_alive=True)
                next_log += 15

    def tap(self, key: str, label: str, settle: float = 0.35):
        self.log("input", key=key, label=label, mechanism="XSendEvent to owned _NET_WM_PID window")
        self.x11.send_key(self.process, self.window, key, True)
        self.held_keys.add(key)
        self.wait(0.13)
        self.x11.send_key(self.process, self.window, key, False)
        self.held_keys.discard(key)
        self.wait(settle)

    def capture(self, name: str, expected: str):
        old = set(self.cwd.glob("*.png"))
        self.tap("F12", "Capture own game framebuffer", 0.45)
        deadline = time.monotonic() + 5
        fresh = []
        while time.monotonic() < deadline:
            fresh = sorted(set(self.cwd.glob("*.png")) - old)
            if fresh:
                break
            self.wait(0.15)
        if len(fresh) != 1:
            raise RuntimeError(f"Expected one new framebuffer PNG for {name}, found {len(fresh)}")
        self.wait(0.15)
        destination = self.shots / f"{name}.png"
        shutil.move(str(fresh[0]), destination)
        with destination.open("rb") as image:
            header = image.read(24)
        if len(header) < 24 or header[:8] != b"\x89PNG\r\n\x1a\n":
            raise RuntimeError(f"Capture is not a complete PNG header: {destination}")
        capture = {
            "name": name, "path": str(destination), "expected_state": expected,
            "validation": "manual visual review required", "sha256": sha256(destination),
            "width": int.from_bytes(header[16:20], "big"), "height": int.from_bytes(header[20:24], "big"),
        }
        self.captures.append(capture)
        self.log("capture", **capture)

    def run(self):
        env = os.environ.copy()
        env["BORROW_FIGHTERS_DATA_DIR"] = str(self.data)
        for name in ["BORROW_FIGHTERS_ASSET_DIR", "BORROW_FIGHTERS_SPRITE_CANDIDATES"]:
            env.pop(name, None)
        if self.asset_override:
            env["BORROW_FIGHTERS_ASSET_DIR"] = str(self.asset_root)
        if self.args.display:
            env["DISPLAY"] = self.args.display
        with (self.evidence / "stdout.log").open("wb") as stdout, (self.evidence / "stderr.log").open("wb") as stderr:
            self.process = subprocess.Popen([str(self.exe), "--fight"], cwd=self.cwd, env=env, stdout=stdout, stderr=stderr)
        self.log("process_started", pid=self.process.pid, executable=str(self.exe), executable_sha256=sha256(self.exe), evidence=str(self.evidence), data_directory=str(self.data), working_directory=str(self.cwd), arguments=["--fight"], display=env.get("DISPLAY"), asset_root=str(self.asset_root), asset_source=self.asset_source, asset_environment_override=self.asset_override, python_version=sys.version)
        try:
            deadline = time.monotonic() + 45
            while time.monotonic() < deadline:
                if self.process.poll() is not None:
                    raise RuntimeError(f"Own game exited while loading: {self.process.returncode}")
                self.window = self.x11.find_window(self.process.pid)
                log = (self.evidence / "stdout.log").read_text(errors="replace")
                if self.window and "Framebuffer object created successfully" in log:
                    break
                time.sleep(0.25)
            else:
                raise RuntimeError("Owned X11 window/framebuffer did not become ready within 45 seconds")
            self.assert_owned()
            self.log("window_ready", pid=self.process.pid, window=self.window)
            self.sequence()
            self.success = True
            self.log("sequence_complete", completed_match_review_candidates=self.args.matches, claim="Inputs delivered and captures collected; actual states require visual review", physical_gamepad_tested=False, physical_mouse_tested=False, audio_listening_performed=False)
        except BaseException as error:
            self.log("error", error_type=type(error).__name__, message=str(error))
            raise
        finally:
            self.cleanup()

    def sequence(self):
        tap, capture = self.tap, self.capture
        self.wait(self.args.warmup_seconds)
        capture("01-initial-fight", "Initial Rust vs Duke fight or entry, with energy meters")
        tap("Escape", "Pause")
        capture("02-pause", "Pause overlay: Continue, Restart, Change characters, Menu")
        self.wait(2)
        capture("03-pause-held", "Same fighter poses and health as 02; UI may animate")
        tap("Return", "Continue")
        capture("04-resumed", "Pause closed and initial fight resumes")
        tap("Escape", "Pause again")
        tap("Down", "Select Restart")
        capture("05-restart-highlight", "Pause with Restart selected")
        tap("Return", "Restart")
        capture("06-restarted-entry", "New entry with full health and energy reset")
        tap("Escape", "Pause for character selection")
        tap("Down", "Move to Restart")
        tap("Down", "Move to Change characters")
        tap("Return", "Open character selection", 0.8)
        capture("07-roster", "Linker roster with animated Rust and Duke previews")
        tap("Down", "P1 cursor to slot 4")
        tap("Right", "P1 cursor to Random")
        tap("Right", "P1 cursor to future slot")
        capture("08-future-slot", "P1 future slot with question mark highlighted")
        tap("Return", "Reject future slot")
        capture("09-future-rejected", "Future slot remains unconfirmed and cannot start a match")
        tap("Left", "P1 to Random")
        capture("10-random-preview", "P1 Random preview before confirmation")
        tap("Return", "Confirm P1 Random")
        capture("11-random-confirmed", "P1 resolves one stable character and P2 becomes active")
        self.wait(1)
        capture("12-random-stable", "Same P1 identity as 11 while idle animation continues")
        tap("Right", "P2 cursor to Old C")
        tap("Return", "Confirm P2")
        capture("13-both-confirmed", "Both sides confirmed and launch enabled")
        tap("Tab", "Switch to Local Duel")
        capture("14-local-mode", "Separate P1/P2 controls and confirmations reset")
        tap("f", "Confirm local P1")
        tap("Return", "Confirm local P2")
        capture("15-local-both-confirmed", "Local F and Enter each confirmed its own player")
        tap("Tab", "Switch to Watch Demo")
        tap("e", "Change arena")
        tap("Return", "Confirm demo P1")
        tap("Return", "Confirm demo P2")
        capture("16-demo-ready", "Watch Demo mode, new arena and both players confirmed")
        tap("Return", "Launch demo", 0.1)
        capture("17-demo-entry", "Real match entry from the roster")
        for number in range(1, self.args.matches + 1):
            self.wait(4)
            tap("Escape", f"Pause match {number}")
            capture(f"18-match-{number}-pause", "Pause overlay within this match; combat stays frozen")
            self.wait(1)
            tap("Return", f"Resume match {number}")
            capture(f"18-match-{number}-resumed", "Same match resumed with pause closed and no restart")
            self.wait(max(1, self.args.match_seconds / 2 - 4))
            capture(f"18-match-{number}-mid", "CPU match or completed result; inspect energy and rendering")
            self.wait(self.args.match_seconds / 2)
            capture(f"19-match-{number}-result-candidate", "Expected result with Rematch / Change characters / Menu; verify visually")
            if number < self.args.matches:
                tap("Return", f"Rematch {number}", 0.1)
                capture(f"20-rematch-{number}-entry", "Expected entry with SAME fighters and arena; verify visually")

    def cleanup(self):
        if self.process and self.process.poll() is None:
            try:
                if self.window:
                    self.assert_owned()
                    for key in list(self.held_keys):
                        self.x11.send_key(self.process, self.window, key, False)
                    self.held_keys.clear()
                    self.x11.request_close(self.process, self.window)
                    self.process.wait(timeout=3)
            except (RuntimeError, subprocess.TimeoutExpired) as error:
                self.log("cleanup_note", message=str(error))
            if self.process.poll() is None:
                self.process.terminate()
                try:
                    self.process.wait(timeout=2)
                except subprocess.TimeoutExpired:
                    self.process.kill()
                    self.process.wait()
        summary = {
            "success": self.success, "evidence": str(self.evidence),
            "executable": str(self.exe), "executable_sha256": sha256(self.exe),
            "pid": self.process.pid if self.process else None,
            "exit_code": self.process.returncode if self.process else None,
            "started_at_utc": self.started_utc, "finished_at_utc": datetime.now(timezone.utc).isoformat(),
            "input": "Simulated keyboard via XSendEvent to owned _NET_WM_PID window",
            "physical_gamepad_tested": False, "physical_mouse_tested": False,
            "audio_listening_performed": False, "expected_states_are_assertions": False,
            "asset_root": str(self.asset_root), "asset_source": self.asset_source,
            "asset_environment_override": self.asset_override, "python_version": sys.version,
            "x11_errors": self.x11.errors, "captures": self.captures,
        }
        (self.evidence / "summary.json").write_text(json.dumps(summary, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
        print(json.dumps(summary, ensure_ascii=False), flush=True)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--executable")
    parser.add_argument("--asset-directory", help="Explicit absolute/source asset root; adjacent bundles are preferred by default")
    parser.add_argument("--display", default=os.environ.get("DISPLAY"))
    parser.add_argument("--output-directory")
    parser.add_argument("--warmup-seconds", type=float, default=7)
    parser.add_argument("--matches", type=int, choices=range(4), default=3)
    parser.add_argument("--match-seconds", type=float, default=75)
    parser.add_argument("--check-x11", action="store_true", help="Read-only library/display check; starts no process and sends no input")
    args = parser.parse_args()
    if not args.check_x11 and not args.executable:
        parser.error("--executable is required unless using --check-x11")
    if not 1 <= args.warmup_seconds <= 30 or not 15 <= args.match_seconds <= 180:
        parser.error("warmup-seconds must be 1..30 and match-seconds must be 15..180")
    x11 = X11(args.display)
    try:
        if args.check_x11:
            print(json.dumps({"display": args.display, "root_window": x11.root, "top_level_window_count": len(x11.children(x11.root)), "xevent_bytes": C.sizeof(XEvent), "key_event_bytes": C.sizeof(XKeyEvent), "input_sent": False, "process_started": False, "errors": x11.errors}))
        else:
            Review(args, x11).run()
    finally:
        x11.close()


if __name__ == "__main__":
    main()
