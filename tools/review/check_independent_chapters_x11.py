#!/usr/bin/env python3
"""Verify protagonist selection and independent saves through an owned native window.

The application creates both checkpoints in an isolated userdata directory.
This harness never writes a profile, warps the pointer or changes server focus.
Screenshots use the existing XGetImage path without changing their pixels.
"""

import argparse
import json
import os
from pathlib import Path
import sys

from check_chapter_host_x11 import ChapterHostReview


class IndependentChaptersReview(ChapterHostReview):
    def cpp_profile(self):
        path = self.userdata / "adventure/cpp-augusta-v1.json"
        return json.loads(path.read_text()) if path.exists() else None

    def return_to_selector(self):
        self.tap("Escape")
        self.tap("Down")
        self.tap("Down")
        self.tap("Return")
        self.wait(1.0)

    def open_rust(self):
        before = self.framebuffers()
        self.click_row(640, 275)
        # The selector retains the previous child title. A new framebuffer,
        # not that stale title, proves Rust finished reacquiring its assets.
        self.await_condition(lambda: self.framebuffers() > before, "fresh Rust framebuffer", 35)
        self.await_title("Borrow Fighters — Depois do silêncio")
        self.wait(.5)

    def sequence(self):
        self.start("native-menu", ["--menu"])
        self.main_menu()
        self.check("fresh_isolated_profiles_are_absent", self.profile() is None and self.cpp_profile() is None)
        self.click_row(940, 198)
        self.wait(1.0)
        self.screenshot("08-protagonists", "Only Rust and C++ have playable chapter entries")

        self.click_row(640, 335)
        self.await_title("Borrow Fighters — Rua Augusta")
        self.click_row(640, 275)
        self.await_condition(lambda: self.cpp_profile() is not None, "game-created C++ checkpoint")
        cpp_initial = self.cpp_profile()
        self.check("cpp_started_without_creating_a_rust_profile", self.profile() is None and cpp_initial["checkpoint"]["stage"] == "arrival")
        self.wait(3.4)
        self.return_to_selector()

        self.open_rust()
        self.click_row(640, 270)
        self.await_condition(lambda: self.checkpoint_is("street_start"), "game-created Rust checkpoint")
        rust_initial = self.profile()
        self.check("rust_start_preserves_cpp_checkpoint", self.cpp_profile() == cpp_initial)
        self.return_to_selector()

        self.open_rust()
        self.wait(.4)
        self.screenshot("10-rust-continue", "Rust offers Continue after creating its own checkpoint")
        self.tap("Escape")
        self.wait(.7)
        self.click_row(640, 335)
        self.await_title("Borrow Fighters — Rua Augusta")
        self.wait(.4)
        self.screenshot("09-cpp-continue", "C++ separately offers Continue after visiting Rust")
        self.check("returning_to_cpp_preserves_both_profiles", self.profile() == rust_initial and self.cpp_profile() == cpp_initial)
        self.check("profiles_have_distinct_checkpoint_schemas", rust_initial["checkpoint"]["stage"] == "street_start" and cpp_initial["checkpoint"]["stage"] == "arrival")

    def run(self):
        result = super().run()
        path = self.directory / "native-checks.json"
        report = json.loads(path.read_text())
        report["cpp_profile"] = self.cpp_profile()
        report["scope"] = "Native pointer and keyboard navigation; both saves created by the game in isolated userdata."
        path.write_text(json.dumps(report, ensure_ascii=False, indent=2) + "\n")
        return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--executable", default=str(Path(__file__).resolve().parents[2] / "target/debug/borrow-story"))
    parser.add_argument("--output-directory", required=True)
    parser.add_argument("--display", default=os.environ.get("DISPLAY", ":0"))
    parser.add_argument("--timeout", type=float, default=240)
    return IndependentChaptersReview(parser.parse_args()).run()


if __name__ == "__main__":
    sys.exit(main())
