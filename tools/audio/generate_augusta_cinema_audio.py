#!/usr/bin/env python3
"""Regenerate Augusta's original combat samples and cinematic street Foley.

Extends generate_production_audio.py without third-party samples, recordings or
synthetic speech. Night air stays separate so traffic can leave with the crowd.
"""

import json
import math

import generate_production_audio as base


def transient(t, center, decay):
    age = t - center
    return math.exp(-age * decay) * min(1.0, age * 500) if age >= 0 else 0.0


def door(t, noise, low):
    latch = transient(t, 0.01, 48) * (noise * 0.31 + math.sin(math.tau * 240 * t) * 0.16)
    hinge = math.sin(math.pi * min(1, t / 0.78)) ** 2
    friction = (low * 0.7 + math.sin(math.tau * (420 * t - 90 * t * t)) * 0.047) * hinge
    stop = transient(t, 0.69, 31) * (low * 1.1 + math.sin(math.tau * 92 * t) * 0.21)
    return latch + friction + stop


def guard_step(t, noise, low):
    heel = transient(t, 0.003, 35) * (math.sin(math.tau * 105 * t) * 0.28 + noise * 0.13)
    sole = transient(t, 0.06, 42) * (low * 1.3 + noise * 0.07)
    return heel + sole


def rupture(t, noise, low):
    envelope = min(1.0, t * 8.0) * math.exp(-t * 2.8)
    sweep = math.sin(math.tau * (112 * t - 24 * t * t)) * 0.22
    crack = transient(t, 0.16, 45) * noise * 0.32
    return (sweep + low * 1.2 + noise * 0.035) * envelope + crack


def panic(t, noise, low):
    # Shoe scuffles, chair scrape and one rolling bottle convey the crowd's
    # movement without replacing the authored dialogue with generated voices.
    steps = sum(transient(t, center, 27) for center in [0.02, 0.16, 0.31, 0.44, 0.62, 0.77, 0.93])
    scramble = steps * (low * 0.9 + noise * 0.08)
    scrape = math.exp(-((t - 0.22) / 0.16) ** 2) * (low * 0.4 + noise * 0.04)
    glass = sum(transient(t, center, 35) for center in [0.43, 0.66, 0.98]) * math.sin(math.tau * 1703 * t) * 0.028
    return scramble + scrape + glass


def night_air(t, noise, low):
    edge = min(1, t / 0.08, (24 - t) / 0.08)
    air = (low * 0.14 + noise * 0.002) * (0.85 + 0.15 * math.sin(math.tau * t / 24))
    return [air * edge, air * (0.97 + 0.03 * math.sin(math.tau * t / 8)) * edge]


def main():
    base.main()
    base.write("night-air.wav", 24, night_air, 2)
    base.write("bar-door.wav", 0.94, door)
    base.write("guard-step.wav", 0.24, guard_step)
    base.write("ep-rupture.wav", 1.5, rupture)
    base.write("panic.wav", 1.4, panic)
    path = base.ROOT / "catalog.json"
    catalog = json.loads(path.read_text())
    catalog["provenance"] = "Original deterministic synthesis; tools/audio/generate_augusta_cinema_audio.py"
    catalog["air"] = {"file": "night-air.wav", "volume": 0.42}
    for key, volume in [("bar-door", 0.52), ("guard-step", 0.36), ("ep-rupture", 0.65), ("panic", 0.43)]:
        catalog["effects"][key] = {"file": key + ".wav", "volume": volume}
    path.write_text(json.dumps(catalog, indent=2) + "\n")


if __name__ == "__main__":
    main()
