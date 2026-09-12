#!/usr/bin/env python3
"""Reproduce original Augusta street ambience and shared production combat Foley.

No recorded voices or third-party samples are used. Every oscillator and noise
source is seeded; wave files and catalogue can be regenerated independently.
"""
import array
import json
import math
from pathlib import Path
import random
import wave

RATE = 22050
ROOT = Path(__file__).resolve().parents[2] / "assets/adventure/audio/production"


def write(name, seconds, sample, channels=1):
    frames = array.array("h")
    rng = random.Random(20260912)
    state = [0.0]
    for i in range(round(seconds * RATE)):
        t = i / RATE
        noise = rng.uniform(-1, 1)
        state[0] = state[0] * 0.96 + noise * 0.04
        values = sample(t, noise, state[0])
        if channels == 1:
            values = [values]
        frames.extend(round(max(-1, min(1, v)) * 30000) for v in values)
    with wave.open(str(ROOT / name), "wb") as output:
        output.setnchannels(channels)
        output.setsampwidth(2)
        output.setframerate(RATE)
        output.writeframes(frames.tobytes())


def ambience(t, noise, low):
    # Periodic road hum under staggered, panned vehicle passes. The tiny end fade
    # contains noise at the loop join while the low ambience remains continuous.
    edge = min(1, t / 0.08, (24 - t) / 0.08)
    road = 0.027 * math.sin(math.tau * 55 * t) + low * 0.13 + noise * 0.008
    left = right = road
    for center, direction in [(4.0, 1), (12.0, -1), (20.0, 1)]:
        travel = (t - center) / 2.5
        amplitude = math.exp(-travel * travel) * 0.105
        engine = math.sin(math.tau * (77 * t + 2 * math.sin(t))) * 0.32 + low * 2.8 + noise * 0.08
        pan = max(-0.8, min(0.8, travel * direction * 0.7))
        left += amplitude * engine * (1 - pan)
        right += amplitude * engine * (1 + pan)
    horn = math.exp(-((t - 8.0) / 0.21) ** 4) * 0.016
    signal = horn * (math.sin(math.tau * 349 * t) + math.sin(math.tau * 440 * t))
    return [(left + signal) * edge, (right + signal * 0.75) * edge]


def main():
    ROOT.mkdir(parents=True, exist_ok=True)
    write("street-loop.wav", 24, ambience, 2)
    write("swish.wav", 0.25, lambda t,n,l: n * math.sin(math.pi * min(t / 0.25, 1)) ** 2 * 0.18)
    write("impact.wav", 0.27, lambda t,n,l: (math.sin(math.tau * (100 * t - 75 * t*t)) * 0.48 + n * 0.28) * math.exp(-t * 24))
    write("parry.wav", 0.42, lambda t,n,l: (math.sin(math.tau * 1450 * t) * 0.19 + math.sin(math.tau * 2071 * t) * 0.12 + n * 0.2 * math.exp(-t * 90)) * math.exp(-t * 12))
    write("projectile.wav", 0.48, lambda t,n,l: (math.sin(math.tau * (500 * t - 350 * t*t)) * 0.15 + n * 0.07) * math.exp(-t * 7) * min(1, t * 150))
    write("landing.wav", 0.65, lambda t,n,l: (math.sin(math.tau * (53 * t - 17 * t*t)) * 0.46 + l * 1.4 + n * 0.08) * math.exp(-t * 8))
    catalog = {"version": 1, "provenance": "Original deterministic synthesis; tools/audio/generate_production_audio.py", "ambience": {"file": "street-loop.wav", "volume": 0.45}, "effects": {key: {"file": key + ".wav", "volume": volume} for key,volume in [("swish",0.35),("impact",0.58),("parry",0.45),("projectile",0.5),("landing",0.7)]}}
    (ROOT / "catalog.json").write_text(json.dumps(catalog, indent=2) + "\n")

if __name__ == "__main__":
    main()
