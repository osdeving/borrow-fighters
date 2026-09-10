#!/usr/bin/env python3
"""Create original natural ambience, a small dog's alert and a rolling shutter.

All sources are oscillators and deterministically filtered noise; no recordings,
third-party libraries or external service are used. Ambience has 20-second loops
and restrained high frequencies, independent of the vehicle evacuation layer.
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
LOOP_SECONDS = 20.0


def envelope(t, duration, attack=.01, release=.06):
    return min(1.0, max(0.0, t / attack), max(0.0, (duration - t) / release))


def air(t, low, mid):
    """Soft broadband movement, without tonal drones or sharp white hiss."""
    return (.9 * low + .10 * mid) * (.8 + .12 * sin(TAU * t / 20))


def morning(t, low, mid):
    sound = air(t, low, mid) * .23
    # A few distant chirps with quiet space, no repeated tune or constant birds.
    for at, duration, pitch, bend, gain in (
        (2.3, .22, 1640, 320, .035), (2.68, .13, 1850, -180, .022),
        (8.7, .24, 1420, 230, .029), (13.4, .18, 1740, -270, .032),
        (13.76, .20, 1610, 210, .025), (17.2, .26, 1500, 180, .020),
    ):
        age = t - at
        if 0 < age < duration:
            phase = TAU * (pitch * age + bend * age * age / (2 * duration))
            sound += gain * sin(phase) * sin(pi * age / duration) ** 2
    return sound


def passing_traffic(t, low, mid):
    """Distant engines and filtered tire wash pass at varied, leisurely intervals."""
    sound = 0.0
    for center, width, pitch, strength in (
        (2.1, 1.45, 79, .80), (6.8, 1.8, 63, .62),
        (12.1, 1.5, 94, .52), (17.0, 1.9, 71, .70),
    ):
        age = t - center
        distance = exp(-(age / width) ** 2)
        # A smooth pitch decrease follows the pass. No aggressive revving/horns.
        phase = TAU * (pitch * t - 2.4 * age * age)
        motor = (sin(phase) + .34 * sin(2 * phase + .3)
                 + .13 * sin(3 * phase)) * .12
        tire = .65 * low + .45 * mid
        sound += strength * distance * (motor + tire)
    return sound


def dog_alert(t, low, mid):
    """One short, low woof: a voiced burst with a breathy tail, not a howl."""
    phase = TAU * (195 * t - 125 * t * t)
    voice = tanh(1.6 * sin(phase)) + .3 * sin(2.9 * phase)
    body = sin(pi * min(1.0, t / .25)) ** 1.4
    return body * (voice * .50 + mid * .7) + low * exp(-14 * t) * .2


def shutter_roll(t, low, mid):
    """One second of hand-pulled corrugated slats rattling down their tracks."""
    sound = (.45 * mid + .20 * low) * (.65 + .35 * sin(TAU * 29 * t) ** 2)
    for at in (0, .044, .096, .154, .222, .298, .381, .473, .572, .679, .794, .916):
        age = t - at
        if age >= 0:
            metal = (sin(TAU * 473 * age) + .45 * sin(TAU * 827 * age)
                     + .20 * sin(TAU * 1411 * age))
            sound += .22 * metal * exp(-62 * age)
    return sound


def shutter_clack(t, low, mid):
    """The bottom rail contacts the pavement with one dull metallic clack."""
    impact = (mid * 1.6 + low * 1.2) * exp(-75 * t)
    ring = (sin(TAU * 311 * t) + .36 * sin(TAU * 739 * t)
            + .13 * sin(TAU * 1193 * t)) * exp(-25 * t)
    return impact + .38 * ring


SOUNDS = (
    ("morning_ambience.wav", LOOP_SECONDS, morning, .11, True),
    ("street_air.wav", LOOP_SECONDS, air, .07, True),
    ("street_traffic.wav", LOOP_SECONDS, passing_traffic, .29, True),
    ("dog_alert.wav", .30, dog_alert, .29, False),
    ("shutter_roll.wav", 1.0, shutter_roll, .39, False),
    ("shutter_clack.wav", .36, shutter_clack, .48, False),
)


def synthesize(name, duration, sampler, peak, looping):
    rng = random.Random(20260910 + sum(map(ord, name)))
    low = mid = 0.0
    samples = []
    for index in range(round(duration * RATE)):
        t = index / RATE
        noise = rng.uniform(-1, 1)
        low += .025 * (noise - low)
        mid += .18 * (noise - mid)
        edge = envelope(t, duration, .35 if looping else .004, .35 if looping else .025)
        samples.append(sampler(t, low, mid) * edge)
    gain = peak * 32767 / max(abs(value) for value in samples)
    pcm = array("h", (round(value * gain) for value in samples))
    pcm[0] = pcm[-1] = 0
    return pcm


def main():
    for name, duration, sampler, peak, looping in SOUNDS:
        pcm = synthesize(name, duration, sampler, peak, looping)
        rms = sqrt(sum(value * value for value in pcm) / len(pcm)) / 32767
        actual_peak = max(abs(value) for value in pcm)
        if sys.byteorder != "little":
            pcm.byteswap()
        with wave.open(str(OUTPUT / name), "wb") as sound:
            sound.setnchannels(1)
            sound.setsampwidth(2)
            sound.setframerate(RATE)
            sound.writeframes(pcm.tobytes())
        print(f"{name}: {duration:.3f}s, mono PCM16/{RATE}Hz, "
              f"peak={actual_peak / 32767:.4f}, rms={rms:.4f}")


if __name__ == "__main__":
    main()
