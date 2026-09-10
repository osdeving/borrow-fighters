#!/usr/bin/env python3
"""Original chapter Foley and phone cues, synthesized with Python's stdlib.

Seeded filtered noise and brief oscillators produce small mono PCM16 WAVs.
No samples, recordings, service, external package or musical loop is used.
"""
from array import array
from math import exp, pi, sin, sqrt
from pathlib import Path
import random
import sys
import wave

RATE = 22050
TAU = 2 * pi
OUTPUT = Path(__file__).resolve().parent


def pocket(t, noise, low):
    return (noise * .22 + low * .85) * (sin(pi * min(1.0, t / .30)) ** 2)


def tap(t, noise, low):
    return (noise * .45 + .19 * sin(TAU * 1060 * t)) * exp(-100 * t)


def send(t, noise, low):
    envelope = sin(pi * min(1.0, t / .16)) ** 2
    return envelope * (.24 * noise + .09 * sin(TAU * (720 * t - 900 * t * t)))


def receive(t, noise, low):
    envelope = min(1.0, t / .025) * exp(-16 * t)
    return envelope * (sin(TAU * 740 * t) + .18 * sin(TAU * 1480 * t))


def step(t, noise, low):
    heel = (low * 1.2 + .1 * sin(TAU * 91 * t)) * exp(-43 * t)
    sole = noise * .32 * exp(-60 * abs(t - .032))
    return heel + sole


SOUNDS = (
    ('phone_pocket.wav', .32, pocket, .16),
    ('phone_tap.wav', .07, tap, .10),
    ('phone_send.wav', .18, send, .15),
    ('phone_receive.wav', .30, receive, .18),
    ('footstep.wav', .14, step, .15),
)


def synthesize(name, duration, sampler, peak):
    rng = random.Random(20260910 + sum(map(ord, name)))
    noise = low = 0.0
    samples = []
    for index in range(round(duration * RATE)):
        t = index / RATE
        white = rng.uniform(-1, 1)
        noise += .3 * (white - noise)
        low += .045 * (white - low)
        edge = min(1.0, t / .003, max(0.0, (duration - t) / .020))
        samples.append(sampler(t, noise, low) * edge)
    gain = peak * 32767 / max(abs(value) for value in samples)
    pcm = array('h', (round(value * gain) for value in samples))
    pcm[0] = pcm[-1] = 0
    return pcm


def main():
    for name, duration, sampler, peak in SOUNDS:
        pcm = synthesize(name, duration, sampler, peak)
        rms = sqrt(sum(value * value for value in pcm) / len(pcm)) / 32767
        if sys.byteorder != 'little':
            pcm.byteswap()
        with wave.open(str(OUTPUT / name), 'wb') as sound:
            sound.setnchannels(1)
            sound.setsampwidth(2)
            sound.setframerate(RATE)
            sound.writeframes(pcm.tobytes())
        print(f'{name}: {len(pcm) / RATE:.4f}s, peak={peak:.2f}, rms={rms:.4f}')


if __name__ == '__main__':
    main()
