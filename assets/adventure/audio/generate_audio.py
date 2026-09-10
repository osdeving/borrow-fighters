"""Generate the original, deterministic adventure pilot sounds using stdlib only."""

from array import array
from math import cos, exp, pi, sin, tanh
from pathlib import Path
import random
import sys
import wave


RATE = 22050
TAU = 2 * pi
OUTPUT = Path(__file__).resolve().parent


def envelope(t, duration, attack=0.02, release=0.12):
    return min(1.0, t / attack, max(0.0, (duration - t) / release))


def bell(t, frequency, decay=2.0):
    if t < 0:
        return 0.0
    return exp(-decay * t) * (
        sin(TAU * frequency * t)
        + 0.28 * sin(TAU * frequency * 2.01 * t)
        + 0.09 * sin(TAU * frequency * 3.98 * t)
    )


def write_sound(name, duration, sample, peak=0.66):
    generator = random.Random(230910 + sum(map(ord, name)))
    samples = [sample(i / RATE, generator) for i in range(round(duration * RATE))]
    maximum = max(max(abs(value) for value in samples), 0.01)
    pcm = array('h', (
        round(32767 * peak * value / maximum * envelope(i / RATE, duration))
        for i, value in enumerate(samples)
    ))
    if sys.byteorder != 'little':
        pcm.byteswap()
    with wave.open(str(OUTPUT / name), 'wb') as audio:
        audio.setnchannels(1)
        audio.setsampwidth(2)
        audio.setframerate(RATE)
        audio.writeframes(pcm.tobytes())


def ada(t, rng):
    pad = sum(sin(TAU * f * t + 0.12 * sin(TAU * 0.2 * t))
              for f in (110, 164.81, 220, 261.63)) * 0.065
    pulse = t % 0.75
    mechanism = exp(-90 * pulse) * (rng.uniform(-1, 1) * 0.035 + sin(TAU * 780 * pulse) * 0.025)
    notes = [(0.0, 440), (3.0, 329.63), (6.0, 392), (9.0, 523.25)]
    memory = sum(0.13 * bell(t - onset, note, 1.1) for onset, note in notes)
    return pad * (0.85 + 0.15 * cos(TAU * t / 12)) + mechanism + memory


def morning(t, rng):
    bed = sum(sin(TAU * f * t) for f in (130.81, 196, 261.63)) * 0.045
    wind = rng.uniform(-1, 1) * 0.006
    birds = 0.0
    for start in (1.2, 1.47, 4.8, 5.09, 8.2, 10.4):
        age = t - start
        if 0 <= age <= 0.18:
            birds += 0.065 * sin(pi * age / 0.18) ** 2 * sin(TAU * (1800 * age + 2500 * age * age))
    return bed + wind + birds


def threat(t, rng):
    pulse = t % 0.6
    low = exp(-8 * pulse) * sin(TAU * (66 * pulse - 15 * pulse * pulse)) * 0.20
    tension = (sin(TAU * 110 * t) + sin(TAU * 116.54 * t)) * 0.035
    click = rng.uniform(-1, 1) * exp(-100 * pulse) * 0.032
    return low + tension + click


def remorse(t, rng):
    chord = sum(sin(TAU * f * t) for f in (130.81, 196, 293.66, 329.63)) * 0.035
    notes = sum(0.10 * bell(t - start, frequency, 0.9)
                for start, frequency in ((0, 392), (3, 329.63), (6, 293.66), (9, 261.63)))
    return chord + notes


def strike(t, rng):
    return (rng.uniform(-1, 1) * exp(-38 * t) * 0.65
            + sin(TAU * (150 * t - 160 * t * t)) * exp(-19 * t) * 0.45)


def block(t, rng):
    return 0.4 * bell(t, 510, 17) + 0.3 * bell(t, 780, 22) + rng.uniform(-1, 1) * exp(-80 * t) * 0.18


def hurt(t, rng):
    return tanh(1.3 * sin(TAU * (125 * t - 65 * t * t))) * exp(-13 * t) * 0.3 + rng.uniform(-1, 1) * exp(-28 * t) * 0.35


def transition(t, rng):
    swell = sin(pi * min(t / 0.8, 1)) ** 2
    return (sin(TAU * (220 * t + 75 * t * t)) * 0.2 + sin(TAU * 330 * t) * 0.08) * swell


def main():
    for name, duration, sampler, peak in (
        ('ada.wav', 12.0, ada, 0.48),
        ('morning.wav', 12.0, morning, 0.42),
        ('threat.wav', 12.0, threat, 0.48),
        ('remorse.wav', 12.0, remorse, 0.40),
        ('strike.wav', 0.22, strike, 0.74),
        ('block.wav', 0.28, block, 0.66),
        ('hurt.wav', 0.36, hurt, 0.68),
        ('transition.wav', 0.80, transition, 0.46),
    ):
        write_sound(name, duration, sampler, peak)
        print(f'{name}: {duration:.2f}s, mono PCM 16-bit/{RATE} Hz')


if __name__ == '__main__':
    main()
