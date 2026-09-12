#!/usr/bin/env python3
"""Compose the original 60-second adventure opening with Python stdlib only.

The score uses newly written melodic/rhythmic patterns and synthesized voices;
there are no recordings, external samples or borrowed melodies. Sections follow
the opening's authored clock: news 0-9, C++ 9-19, Python 19-29, Duke 29-41,
Old C 41-53, logo 53-60. D major resolves the minor build at 53 seconds.
This generator writes opening.wav only; the eight earlier WAVs remain intact.
"""

from array import array
from math import exp, pi, sin, sqrt, tanh
from pathlib import Path
import random
import sys
import wave

RATE = 22050
DURATION = 60.0
TEMPO = 120
TAU = 2 * pi
OUTPUT = Path(__file__).with_name("opening.wav")
SECTIONS = (
    ("news", 0.0, 9.0),
    ("cpp", 9.0, 19.0),
    ("python", 19.0, 29.0),
    ("duke", 29.0, 41.0),
    ("old_c", 41.0, 53.0),
    ("logo", 53.0, 60.0),
)


def frequency(midi):
    return 440 * 2 ** ((midi - 69) / 12)


def envelope(t, duration, attack=.008, release=.12):
    return min(1.0, t / attack, max(0.0, (duration - t) / release))


class Score:
    def __init__(self):
        self.size = round(RATE * DURATION)
        self.music = array("d", [0.0]) * self.size
        self.drums = array("d", [0.0]) * self.size
        self.event_number = 0

    def note(self, at, duration, midi, gain, voice="pluck"):
        start = round(at * RATE)
        count = min(round(duration * RATE), self.size - start)
        pitch = frequency(midi)
        for index in range(max(0, count)):
            age = index / RATE
            phase = TAU * pitch * age
            if voice == "pad":
                body = sin(phase + .018 * sin(TAU * .6 * age)) + .25 * sin(phase * 2) + .10 * sin(phase * 3)
                shape = envelope(age, duration, .11, .55)
            elif voice == "bass":
                body = sin(phase) + .28 * sin(phase * 2) + .10 * sin(phase * 3)
                shape = envelope(age, duration, .006, .10) * (.25 + .75 * exp(-5 * age))
            elif voice == "bell":
                body = sin(phase) + .34 * sin(phase * 2.003) + .12 * sin(phase * 3.997)
                shape = envelope(age, duration, .006, .24) * exp(-2.8 * age)
            elif voice == "lead":
                body = tanh(1.6 * (sin(phase + .025 * sin(TAU * 5 * age)) + .2 * sin(phase * 2)))
                shape = envelope(age, duration, .016, .20) * (.60 + .40 * exp(-4 * age))
            else:
                body = sin(phase + 1.25 * exp(-12 * age) * sin(phase * 2)) + .15 * sin(phase * 3)
                shape = envelope(age, duration, .005, .15) * exp(-4.8 * age)
            self.music[start + index] += gain * body * shape

    def chord(self, at, duration, notes, gain, voice="pad"):
        for note in notes:
            self.note(at, duration, note, gain / sqrt(len(notes)), voice)

    def percussion(self, at, kind, gain, duration=None):
        durations = {"kick": .32, "snare": .28, "hat": .075, "open_hat": .21, "crash": 1.8, "tom": .35, "rise": 1.5}
        duration = duration or durations[kind]
        start = round(at * RATE)
        count = min(round(duration * RATE), self.size - start)
        rng = random.Random(9012041 + self.event_number)
        self.event_number += 1
        previous_noise = 0.0
        for index in range(max(0, count)):
            age = index / RATE
            noise = rng.uniform(-1, 1)
            high_noise = (noise - previous_noise) * .5
            previous_noise = noise
            if kind == "kick":
                phase = TAU * (48 * age + 1.25 * (1 - exp(-age / .025)))
                body = sin(phase) * exp(-13 * age) + .08 * noise * exp(-110 * age)
            elif kind == "snare":
                body = (.72 * high_noise + .28 * sin(TAU * 185 * age)) * exp(-19 * age)
            elif kind in {"hat", "open_hat"}:
                body = high_noise * exp((-55 if kind == "hat" else -16) * age)
            elif kind == "tom":
                body = sin(TAU * (105 * age + .7 * (1 - exp(-age / .045)))) * exp(-12 * age)
            elif kind == "crash":
                metal = .16 * (sin(TAU * 2137 * age) + sin(TAU * 3311 * age))
                body = (high_noise + metal) * exp(-2.5 * age)
            else:
                progress = age / duration
                body = (high_noise * .6 + .16 * sin(TAU * (420 * age + 1200 * age * age / duration))) * progress ** 1.8
            self.drums[start + index] += gain * body * envelope(age, duration, .001, min(.12, duration / 3))

    def finish(self):
        # Short independent echoes give the melodic instruments space without
        # washing out drums or changing the timing of the logo's first attack.
        mixed = array("d", (music + drum for music, drum in zip(self.music, self.drums)))
        for delay, gain in ((.165, .13), (.33, .08), (.495, .045)):
            offset = round(delay * RATE)
            for index in range(offset, self.size):
                mixed[index] += self.music[index - offset] * gain
        for index in range(self.size):
            t = index / RATE
            mixed[index] *= min(1.0, t / .012, max(0.0, (DURATION - t) / .85))
        peak = max(abs(value) for value in mixed)
        pcm = array("h", (round(value / peak * 32767 * .78) for value in mixed))
        # A zero final sample gives the non-looping ending a quiet boundary.
        pcm[-1] = 0
        return pcm


def rhythmic_section(score, start, end, roots, chords, intensity, voice="pluck"):
    bars = round((end - start) / 2)
    for bar in range(bars):
        onset = start + bar * 2
        root = roots[bar % len(roots)]
        chord = chords[bar % len(chords)]
        score.chord(onset, min(2.5, end - onset + .35), chord, .095 * intensity)
        for step, length, velocity in ((0, .38, 1), (.5, .30, .78), (1.25, .20, .65), (1.5, .39, .87)):
            if onset + step < end:
                score.note(onset + step, length, root, .24 * intensity * velocity, "bass")
        for step in range(8):
            t = onset + step * .25
            if t >= end:
                continue
            notes = (chord[0] + 12, chord[1] + 12, chord[2] + 12, chord[1] + 24)
            score.note(t, .40 if voice == "pluck" else .70, notes[step % 4], .048 * intensity * (1 if step % 2 == 0 else .75), voice)
        for beat in range(4):
            t = onset + beat * .5
            if t >= end:
                continue
            score.percussion(t, "kick", .38 * intensity if beat % 2 == 0 else .21 * intensity)
            if beat % 2:
                score.percussion(t, "snare", .26 * intensity)
            score.percussion(t + .25, "open_hat" if beat == 3 else "hat", .12 * intensity)
            if intensity > 1:
                score.percussion(t + .125, "hat", .045 * intensity)
                score.percussion(t + .375, "hat", .060 * intensity)


def compose():
    score = Score()
    # News: sparse low pulses and a questioning minor motif build into the cast.
    score.chord(0, 4.5, (50, 57, 62), .13)
    score.chord(4, 5, (46, 53, 60, 65), .13)
    for beat in range(18):
        at = beat * .5
        score.note(at, .36, 38 if beat < 8 else 34, .13 + beat * .005, "bass")
        if at >= 2:
            score.percussion(at, "kick", .20 + beat * .009)
        if at >= 4 and beat % 2:
            score.percussion(at, "snare", .17)
        if at >= 3:
            score.percussion(at + .25, "hat", .09)
    for at, note in ((.5, 74), (2, 77), (3.5, 76), (5, 69), (6.5, 72), (7.5, 73)):
        score.note(at, .85, note, .12, "bell")
    score.percussion(7.5, "rise", .21, 1.5)
    for at in (8.25, 8.5, 8.75):
        score.percussion(at, "tom", .24)

    # C++: energetic plucks and a decisive, newly written ascending response.
    rhythmic_section(score, 9, 19, (38, 34, 41, 36, 33),
                     ((50, 53, 57), (46, 50, 53), (53, 57, 60), (48, 52, 55), (45, 49, 52)), 1.0)
    score.percussion(9, "crash", .25)
    cpp_melody = ((9, 74, .65), (10, 77, .38), (10.5, 81, .70), (12, 79, .65), (13, 77, .40),
                  (13.5, 81, .65), (15, 84, .8), (16, 79, .65), (17, 76, .5), (18, 73, .75))
    for at, note, duration in cpp_melody:
        score.note(at, duration, note, .105, "lead")

    # Python: glassy arpeggios and an answering phrase over the same rhythmic pulse.
    rhythmic_section(score, 19, 29, (34, 41, 36, 38, 33),
                     ((46, 50, 53), (53, 57, 60), (48, 52, 55), (50, 53, 57), (45, 49, 52)), .9, "bell")
    score.percussion(19, "crash", .18)
    python_melody = ((19, 77), (20, 81), (21, 84), (22, 81), (23, 79), (24, 76), (25, 77), (26, 74), (27, 76), (28, 73))
    for at, note in python_melody:
        score.note(at, .8, note, .14, "bell")

    # Duke: measured low pulse, restrained plucks and a confident lower motif.
    # The same instruments/palette bridge the limousine and the meeting room.
    rhythmic_section(score, 29, 41, (38, 34, 41, 36, 31, 33),
                     ((50, 53, 57), (46, 50, 53), (53, 57, 60), (48, 52, 55), (43, 46, 50), (45, 49, 52)), .72)
    score.percussion(29, "crash", .15)
    for at, note, length in ((29.5, 62, .9), (31, 65, .9), (32.5, 69, 1.15),
                             (35, 67, 1.1), (37, 65, .9), (39, 61, 1.3)):
        score.note(at, length, note, .105, "lead")

    # Old C: a quieter, deliberate arpeggio and low bell answers over familiar
    # harmony. This leaves room for the intellectual desk/setup composition.
    rhythmic_section(score, 41, 53, (38, 34, 36, 38, 31, 33),
                     ((50, 53, 57), (46, 50, 53), (48, 52, 55), (50, 53, 57), (43, 46, 50), (45, 49, 52)), .54)
    for at, note in ((41.5, 62), (43, 69), (45, 65), (47.5, 67), (49, 64), (51, 61)):
        score.note(at, 1.2, note, .10, "bell")

    # The biographies now lead directly to the title, without Rust's cast card.
    logo = SECTIONS[-1][1]
    score.percussion(logo - 1, "rise", .18, 1)
    for offset in (.5, .25):
        score.percussion(logo - offset, "tom", .17)

    # Original seven-second title resolution, anchored to the current logo start.
    score.percussion(logo, "kick", .68)
    score.percussion(logo, "crash", .40, 2.8)
    score.note(logo, 2.8, 26, .24, "bass")
    score.chord(logo, 6.8, (50, 54, 57, 62, 69), .33)
    for offset, note, duration in ((0, 74, 1.0), (.5, 78, 1.0), (1, 81, 1.5), (2, 86, 2.8), (3, 74, 3.5)):
        score.note(logo + offset, duration, note, .14, "lead")
    for offset in (1, 2):
        score.percussion(logo + offset, "kick", .32)
        score.percussion(logo + offset + .5, "snare", .19)
    score.percussion(logo + 3, "kick", .35)
    score.chord(logo + 3, 4, (62, 66, 69, 74), .15, "bell")
    return score.finish()


def main():
    pcm = compose()
    peak = max(abs(value) for value in pcm)
    rms = sqrt(sum(value * value for value in pcm) / len(pcm)) / 32767
    output = array("h", pcm)
    if sys.byteorder != "little":
        output.byteswap()
    with wave.open(str(OUTPUT), "wb") as sound:
        sound.setnchannels(1)
        sound.setsampwidth(2)
        sound.setframerate(RATE)
        sound.writeframes(output.tobytes())
    print(f"{OUTPUT.name}: {len(pcm) / RATE:.3f}s, PCM16 mono/{RATE}Hz, {TEMPO} BPM, peak={peak}, rms={rms:.4f}")
    print("Original score; title arrival at 53.000s; non-looping resolution through 60.000s.")


if __name__ == "__main__":
    main()
