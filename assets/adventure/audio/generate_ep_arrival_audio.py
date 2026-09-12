#!/usr/bin/env python3
"""Synthesize the original EP's falling air and heavy ground impact, without samples."""
from __future__ import annotations

from array import array
import json
import math
from pathlib import Path
import random
import wave

RATE = 22050
ROOT = Path(__file__).resolve().parent


def write(name, samples, peak):
    gain = peak / max(abs(value) for value in samples)
    pcm = array("h", (int(max(-1.0, min(1.0, value * gain)) * 32767) for value in samples))
    with wave.open(str(ROOT / name), "wb") as stream:
        stream.setparams((1, 2, RATE, 0, "NONE", "not compressed"))
        stream.writeframes(pcm.tobytes())


def main():
    rng = random.Random(20260912)
    samples = []
    noise = 0.0
    phase = 0.0
    trajectory = json.loads((ROOT.parent / "street/ep-arrival.json").read_text())
    duration = trajectory["descent"][-1]["tick"] / 60.0
    for i in range(round(duration * RATE)):
        t = i / RATE
        progress = t / duration
        noise += (rng.uniform(-1.0, 1.0) - noise) * 0.09
        phase += math.tau * (53.0 + 30.0 * progress) / RATE
        edge = min(1.0, t / 0.25, (duration - t) / 0.15)
        envelope = edge * (0.2 + 0.8 * progress ** 2)
        samples.append(envelope * (noise * 0.75 + math.sin(phase) * 0.065))
    write("ep_descent.wav", samples, 0.39)
    # Independent noise keeps trajectory edits from changing the impact timbre.
    rng = random.Random(20260913)
    samples = []
    low_noise = 0.0
    phase = 0.0
    duration = 1.75
    for i in range(round(duration * RATE)):
        t = i / RATE
        noise = rng.uniform(-1.0, 1.0)
        low_noise += (noise - low_noise) * 0.035
        phase += math.tau * (43.0 + 40.0 * math.exp(-t * 16.0)) / RATE
        thud = math.sin(phase) * math.exp(-t * 6.0) * 0.8
        crack = noise * math.exp(-t * 42.0) * 0.55
        rubble = low_noise * math.exp(-t * 2.4) * 1.2
        scattering = noise * math.exp(-t * 5.0) * abs(math.sin(t * 91.0)) * 0.09
        edge = min(1.0, t / 0.002, (duration - t) / 0.08)
        samples.append((thud + crack + rubble + scattering) * edge)
    write("ep_impact.wav", samples, 0.9)


if __name__ == "__main__":
    main()
