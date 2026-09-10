#!/usr/bin/env python3
"""Synthesize the adventure street's original horn, tires and crumpling metal.

All samples are written from deterministic oscillators and filtered seeded noise
using Python's standard library. No external recordings or sampled music are
used. The skid lasts 34/60 seconds, matching the shared simulation interval
between CAR_SKID_TICK=78 and CAR_IMPACT_TICK=112 in adventure/ambient.rs.
"""

from array import array
from math import exp, pi, sin, sqrt, tanh
from pathlib import Path
import random
import sys
import wave

RATE = 22050
TAU = 2 * pi
OUTPUT = Path(__file__).resolve().parent


def edge(t, duration, attack=.004, release=.035):
    return max(0.0, min(1.0, t / attack, (duration - t) / release))


def horn(t, _noise):
    """An urgent dual-tone car horn, with rich brass-like harmonics."""
    voice = 0.0
    wobble = .017 * sin(TAU * 7 * t)
    for fundamental in (392, 493.88):
        phase = TAU * fundamental * t + wobble
        voice += (sin(phase) + .43 * sin(2 * phase)
                  + .23 * sin(3 * phase) + .09 * sin(5 * phase))
    # A brief dip sounds like the driver's breathless repeated pressure.
    pressure = .45 if .19 < t < .225 else 1.0
    return tanh(voice * .85) * pressure


def skid(t, noise):
    """Falling tire squeal over rough, filtered road friction."""
    phase = TAU * (1390 * t - 340 * t * t) + .6 * sin(TAU * 33 * t)
    bite = .64 + .36 * sin(TAU * 21 * t) ** 2
    rubber = sin(phase) + .27 * sin(phase * 1.497) + .13 * sin(phase * 2.01)
    return .61 * tanh(rubber * 1.7) * bite + .57 * noise


def crash(t, noise):
    """A low collision, successive crunches and an inharmonic metal tail."""
    impact = sin(TAU * (75 * t + 2.4 * (1 - exp(-t / .025))))
    body = impact * exp(-9 * t) * .92 + noise * exp(-15 * t) * 1.4
    for at, gain in ((.025, .80), (.077, .66), (.135, .54), (.225, .37), (.36, .22)):
        age = t - at
        if age < 0:
            continue
        bending = (sin(TAU * 317 * age + 1.5 * exp(-20 * age))
                   + .56 * sin(TAU * 533 * age)
                   + .33 * sin(TAU * 859 * age))
        body += gain * (noise * 1.3 + bending * .30) * exp(-age * 28)
    ringing = sum(sin(TAU * frequency * t) * gain
                  for frequency, gain in ((181, .18), (347, .12), (593, .07)))
    return tanh(body * 1.15) + ringing * exp(-3.8 * t)


def synthesize(name, duration, sampler, peak):
    rng = random.Random(20260910 + sum(map(ord, name)))
    samples = []
    low_noise = 0.0
    for index in range(round(duration * RATE)):
        t = index / RATE
        # Filter abrasive hiss while retaining the crunch and road texture.
        low_noise += .44 * (rng.uniform(-1, 1) - low_noise)
        samples.append(sampler(t, low_noise) * edge(t, duration))
    maximum = max(abs(value) for value in samples)
    pcm = array("h", (round(32767 * peak * value / maximum) for value in samples))
    pcm[0] = pcm[-1] = 0
    return pcm


def main():
    for name, duration, sampler, peak in (
        ("car_horn.wav", .60, horn, .82),
        ("car_skid.wav", 34 / 60, skid, .73),
        ("car_crash.wav", 1.35, crash, .94),
    ):
        pcm = synthesize(name, duration, sampler, peak)
        rms = sqrt(sum(value * value for value in pcm) / len(pcm)) / 32767
        encoded = array("h", pcm)
        if sys.byteorder != "little":
            encoded.byteswap()
        with wave.open(str(OUTPUT / name), "wb") as sound:
            sound.setnchannels(1)
            sound.setsampwidth(2)
            sound.setframerate(RATE)
            sound.writeframes(encoded.tobytes())
        print(f"{name}: {len(pcm) / RATE:.3f}s, mono PCM16/{RATE}Hz, "
              f"peak={max(abs(value) for value in pcm)}, rms={rms:.4f}")


if __name__ == "__main__":
    main()
