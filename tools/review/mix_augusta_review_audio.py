#!/usr/bin/env python3
"""Mix Augusta's observed cues into its native review recording.

Usage: python3 tools/review/mix_augusta_review_audio.py REVIEW_DIRECTORY

The source video stays unchanged. Audio is reconstructed from the exact emitted
cue names and pause/reset telemetry; it is not a recording of the Raylib device.
Only original local PCM samples are used. Python dependencies are stdlib only.
"""

from array import array
from collections import Counter
import argparse
import json
import math
from pathlib import Path
import subprocess
import wave

ROOT = Path(__file__).resolve().parents[2]
SAMPLES = ROOT / "assets/adventure/audio/production"
RATE = 22050


def read_sample(path):
    with wave.open(str(path)) as source:
        if source.getframerate() != RATE or source.getsampwidth() != 2:
            raise ValueError(f"unsupported PCM format: {path}")
        channels = source.getnchannels()
        if channels not in (1, 2):
            raise ValueError(f"unsupported channel count: {path}")
        samples = array("h", source.readframes(source.getnframes()))
    return samples, channels


def mix(rows, duration, catalog, sample_root=SAMPLES):
    previous = -1.0
    for row in rows:
        seconds = row["seconds"]
        if not isinstance(seconds, (int, float)) or not math.isfinite(seconds) or seconds < 0 or seconds < previous:
            raise ValueError("telemetry time must be finite, nonnegative and monotonic")
        if type(row["paused"]) is not bool or type(row.get("audio_reset", False)) is not bool:
            raise ValueError("invalid pause/reset flag")
        if not isinstance(row["audio_cues"], list) or any(key not in catalog["effects"] for key in row["audio_cues"]):
            raise ValueError("unknown production cue")
        previous = seconds
    ambience, channels = read_sample(sample_root / catalog["ambience"]["file"])
    loops = len(ambience) // channels
    air_spec = catalog.get("air")
    air, air_channels = read_sample(sample_root / air_spec["file"]) if air_spec else (array("h", [0, 0]), 2)
    air_frames = len(air) // air_channels
    effects = {key: read_sample(sample_root / spec["file"]) for key, spec in catalog["effects"].items()}
    result = array("h")
    voices = {}
    road_cursor = 0
    row_index = 0
    paused = True
    threat_age = None
    counts = Counter()
    for frame in range(round(duration * RATE)):
        while row_index < len(rows) and round(rows[row_index]["seconds"] * RATE) <= frame:
            row = rows[row_index]
            paused = row["paused"]
            # Same persistent clock and six-second traffic fade as runtime.
            # Older recordings have neither this field nor the cinematic cues.
            if "threat_age" in row:
                threat_age = row["threat_age"]
                if threat_age is not None and (type(threat_age) is not int or threat_age < 0):
                    raise ValueError("invalid threat age")
            if row.get("audio_reset", False):
                voices.clear()
            if not paused:
                for key in row["audio_cues"]:
                    voices[key] = 0
                    counts[key] += 1
            row_index += 1
        if paused:
            result.extend((0, 0))
            continue
        position = (road_cursor % loops) * channels
        traffic_gain = 1.0 if threat_age is None else max(0.0, 1.0 - threat_age / 360.0)
        left = ambience[position] * catalog["ambience"]["volume"] * traffic_gain
        right = ambience[position + channels - 1] * catalog["ambience"]["volume"] * traffic_gain
        air_position = (road_cursor % air_frames) * air_channels
        air_volume = air_spec["volume"] if air_spec else 0.0
        left += air[air_position] * air_volume
        right += air[air_position + air_channels - 1] * air_volume
        road_cursor += 1
        completed = []
        for key, cursor in voices.items():
            sample, count = effects[key]
            if cursor * count >= len(sample):
                completed.append(key)
                continue
            volume = catalog["effects"][key]["volume"]
            left += sample[cursor * count] * volume
            right += sample[cursor * count + count - 1] * volume
            voices[key] += 1
        for key in completed:
            del voices[key]
        result.extend(round(max(-32768, min(32767, value))) for value in (left, right))
    return result, counts


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("directory", type=Path)
    args = parser.parse_args()
    directory = args.directory
    video = directory / "adventure-silent.mp4"
    probe = json.loads(subprocess.check_output(["ffprobe", "-v", "error", "-show_entries", "format=duration", "-of", "json", str(video)]))
    duration = float(probe["format"]["duration"])
    rows = [json.loads(line) for line in (directory / "telemetry.jsonl").read_text().splitlines() if line.strip()]
    catalog = json.loads((SAMPLES / "catalog.json").read_text())
    samples, counts = mix(rows, duration, catalog)
    wav_path = directory / "adventure-mix.wav"
    with wave.open(str(wav_path), "wb") as output:
        output.setnchannels(2)
        output.setsampwidth(2)
        output.setframerate(RATE)
        output.writeframes(samples.tobytes())
    subprocess.run(["ffmpeg", "-y", "-loglevel", "error", "-i", str(video), "-i", str(wav_path), "-map", "0:v:0", "-map", "1:a:0", "-c:v", "copy", "-c:a", "aac", "-b:a", "192k", "-shortest", str(directory / "adventure.mp4")], check=True)
    (directory / "mix-review.json").write_text(json.dumps({"duration": duration, "cue_counts": counts,
        "source": "original production samples and observed fixed-update cues",
        "limitation": "Reconstructed audio; not captured from the Raylib device. Device latency and listening approval are not verified."}, indent=2) + "\n")


if __name__ == "__main__":
    main()
