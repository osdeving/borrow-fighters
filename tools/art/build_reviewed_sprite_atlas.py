#!/usr/bin/env python3
"""Compatibility entrypoint; canonical crop/pack/validation logic lives in art.py.

Older production commands keep their output and crop locations. New work uses
`python3 tools/art/art.py build <recipe>` and source → generated → runtime.
"""

from art import build, legacy_main


if __name__ == "__main__":
    legacy_main()
