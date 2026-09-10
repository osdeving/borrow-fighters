#!/usr/bin/env python3
"""Synthesize short original engine escape and abandoned-bicycle sounds.

Layered oscillators and seeded filtered noise use only Python's standard library.
The existing traffic synthesizer supplies deterministic PCM normalization and
edge envelopes; this script writes only the two collective evacuation cues.
"""

from array import array
from math import exp, sin, sqrt, tanh
import sys
import wave

from generate_traffic_audio import OUTPUT, RATE, TAU, synthesize


def traffic_escape(t, noise):
    """Three engines rise in pitch while a soft road rush quickly recedes."""
    engines = 0.0
    for delay, fundamental, gain in ((0.0, 73, .68), (.075, 109, .48), (.16, 153, .32)):
        age = t - delay
        if age < 0:
            continue
        phase = TAU * (fundamental * age + 94 * age * age)
        exhaust = sin(phase) + .40 * sin(2 * phase) + .21 * sin(3 * phase)
        throttle = min(1.0, age / .08) * exp(-1.55 * age)
        engines += tanh(exhaust * 1.2) * throttle * gain
    road = noise * .34 * min(1.0, t / .12) * exp(-1.9 * t)
    return engines + road


def bicycle_fall(t, noise):
    """Light tubes rattle and ring against paving, without a heavy crash thump."""
    sound = 0.0
    for at, gain in ((0.0, .68), (.055, .90), (.14, .57), (.26, .42), (.43, .27), (.59, .13)):
        age = t - at
        if age < 0:
            continue
        tubes = (sin(TAU * 647 * age) + .55 * sin(TAU * 1091 * age)
                 + .28 * sin(TAU * 1717 * age))
        contact = noise * exp(-85 * age) * .9
        sound += gain * (tubes * .27 * exp(-16 * age) + contact)
    return sound


def main():
    for name, duration, sampler, peak in (
        ("traffic_escape.wav", 1.4, traffic_escape, .58),
        ("bicycle_fall.wav", .9, bicycle_fall, .50),
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
