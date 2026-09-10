#!/usr/bin/env python3
"""Reconstruct adventure review audio from execution states and original WAVs.

Usage: python3 tools/review/mix_adventure_review_audio.py REVIEW_DIRECTORY

Reads telemetry.jsonl and adventure-silent.mp4, then writes adventure-mix.wav,
adventure.mp4 and mix-review.json. Python dependencies are stdlib only; ffprobe
and ffmpeg must be installed. No host/device audio is recorded. This is an
explicitly reconstructed mix from the execution's observed states, not evidence
of the actual Raylib audio-device output or a human listening approval.

The state observer mirrors src/adventure/engine/audio.rs: volume 0.3, looping
ambience, the non-looping opening score, frozen playback cursors during pause,
silence while paused, one active voice
per cue, contact deduplication, transition cues and the street accident's shared
milestones. Explicit skip telemetry discards cue voices and seeks the score.
Legacy telemetry without street clocks never invents traffic effects.
FFmpeg copies the recorded
video unchanged and encodes the reconstructed mono mix as AAC.
"""

from __future__ import annotations

import argparse
from array import array
from collections import Counter
from fractions import Fraction
import hashlib
import json
import math
from pathlib import Path
import subprocess
import sys
import tempfile
import wave

ROOT = Path(__file__).resolve().parents[2]
RATE = 22050
VOLUME = 0.3
TRACKS = ("ada", "morning", "threat", "remorse", "opening")
NON_LOOPING_TRACKS = {"opening"}
TRAFFIC_MILESTONES = ((38, "car_horn"), (78, "car_skid"), (112, "car_crash"))
TRAFFIC_CUES = tuple(cue for _, cue in TRAFFIC_MILESTONES)
CUES = ("strike", "block", "hurt", "transition") + TRAFFIC_CUES
STAGES = {"AdaPrologue", "RustMorning", "Encounter", "Aftermath", "Opening", "Complete"}
LIMITATION = (
    "Audio reconstructed offline from telemetry and the original adventure WAVs; "
    "not captured from the host or Raylib audio device. Timing follows the recorded "
    "state samples, aligned to the first video frame. Device latency, resampling, "
    "native mixer/pan behavior and human audio approval are not verified. "
    "The review video may use simulated controls; reconstruction does not establish "
    "physical-input or real-device validation."
)


def read_telemetry(path: Path) -> list[dict]:
    rows = []
    previous = -math.inf
    with path.open() as source:
        for number, line in enumerate(source, 1):
            if not line.strip():
                continue
            try:
                row = json.loads(line)
                seconds = float(row["seconds"])
                stage = row["stage"]
                ticks = row["ticks"]
                if not math.isfinite(seconds) or seconds < 0 or seconds < previous:
                    raise ValueError("seconds must be finite, nonnegative and monotonic")
                if stage not in STAGES:
                    raise ValueError(f"unknown stage {stage!r}")
                if not isinstance(row["paused"], bool) or not isinstance(row["enemy_awake"], bool):
                    raise ValueError("paused and enemy_awake must be booleans")
                if not isinstance(ticks, int) or ticks < 0:
                    raise ValueError("ticks must be a nonnegative integer")
                if "stage_ticks" in row and (not isinstance(row["stage_ticks"], int) or row["stage_ticks"] < 0):
                    raise ValueError("stage_ticks must be a nonnegative integer")
                if not isinstance(row.get("audio_synced_after_skip", False), bool):
                    raise ValueError("audio_synced_after_skip must be a boolean")
                if "ambience" in row:
                    ambience = row["ambience"]
                    if not isinstance(ambience, dict):
                        raise ValueError("ambience must be an object")
                    for key in ("ticks", "accident_ticks"):
                        value = ambience.get(key)
                        if key == "ticks" and key in ambience and value is None:
                            raise ValueError("ambience.ticks must be a nonnegative integer")
                        if value is not None and (type(value) is not int or value < 0):
                            raise ValueError(f"ambience.{key} must be a nonnegative integer or null")
                hit = row.get("hit")
                if hit is not None:
                    if hit["target"] not in {"Player", "Erratic"}:
                        raise ValueError("unknown hit target")
                    if not isinstance(hit["age"], int) or hit["age"] < 0 or not isinstance(hit["blocked"], bool):
                        raise ValueError("invalid hit age/blocked fields")
            except (ValueError, KeyError, TypeError) as error:
                raise ValueError(f"{path}:{number}: invalid telemetry: {error}") from error
            row["seconds"] = seconds
            rows.append(row)
            previous = seconds
    if not rows:
        raise ValueError(f"{path}: telemetry is empty")
    return rows


def load_wavs(directory: Path) -> tuple[dict[str, array], dict]:
    clips = {}
    metadata = {}
    for name in TRACKS + CUES:
        path = directory / f"{name}.wav"
        with wave.open(str(path), "rb") as sound:
            if (sound.getnchannels(), sound.getsampwidth(), sound.getframerate(), sound.getcomptype()) != (1, 2, RATE, "NONE"):
                raise ValueError(f"{path}: expected mono PCM16 at {RATE} Hz")
            pcm = array("h")
            pcm.frombytes(sound.readframes(sound.getnframes()))
        if sys.byteorder != "little":
            pcm.byteswap()
        if not pcm:
            raise ValueError(f"{path}: WAV is empty")
        clips[name] = pcm
        metadata[name] = {
            "file": str(path.resolve()),
            "sha256": hashlib.sha256(path.read_bytes()).hexdigest(),
            "samples": len(pcm),
            "duration_seconds": len(pcm) / RATE,
            "looping": name in TRACKS and name not in NON_LOOPING_TRACKS,
        }
    return clips, metadata


def probe_video(path: Path) -> dict:
    output = subprocess.check_output([
        "ffprobe", "-v", "error", "-select_streams", "v:0", "-show_entries",
        "stream=duration,avg_frame_rate,nb_frames:format=duration", "-of", "json", str(path),
    ], text=True)
    data = json.loads(output)
    if not data.get("streams"):
        raise ValueError(f"{path}: no video stream")
    stream = data["streams"][0]
    fps = float(Fraction(stream["avg_frame_rate"]))
    duration = float(stream.get("duration", data.get("format", {}).get("duration", 0)))
    if not math.isfinite(duration) or duration <= 0 or abs(fps - 30.0) > 0.001:
        raise ValueError(f"{path}: expected a positive-duration 30 fps review video")
    return {"duration_seconds": duration, "fps": fps, "frames": stream.get("nb_frames")}


def background_for(row: dict) -> str:
    stage = row["stage"]
    if stage == "AdaPrologue":
        return "ada"
    if stage == "RustMorning":
        return "morning"
    if stage == "Encounter":
        hp = row.get("player", {}).get("hp", row.get("player_hp"))
        if row.get("outcome") == "Defeat" or (hp is not None and hp <= 0):
            return "remorse"
        return "threat" if row["enemy_awake"] else "morning"
    if stage in {"Aftermath", "Complete"}:
        return "remorse"
    if stage == "Opening":
        return "opening"
    raise ValueError(f"Unknown stage: {stage}")


class Observer:
    """Mirror the unpaused ObservedAudio state from the adventure runtime."""

    def __init__(self):
        self.stage = None
        self.stage_ticks = 0
        self.combat_ticks = 0
        self.ambient_ticks = 0
        self.accident_ticks = None
        self.enemy_awake = False
        self.hit_tick = None
        self.epoch = 0

    def timeline_restarted(self, row: dict) -> bool:
        return (row["ticks"] < self.combat_ticks
                or row.get("ambience", {}).get("ticks", 0) < self.ambient_ticks
                or (self.stage == row["stage"]
                    and row.get("stage_ticks", 0) < self.stage_ticks))

    def align(self, row: dict):
        """Snapshot skipped state without emitting its abandoned effects."""
        self.stage = row["stage"]
        self.stage_ticks = row.get("stage_ticks", 0)
        self.combat_ticks = row["ticks"]
        self.ambient_ticks = row.get("ambience", {}).get("ticks", 0)
        self.accident_ticks = row.get("ambience", {}).get("accident_ticks")
        self.enemy_awake = row["enemy_awake"]
        hit = row.get("hit")
        self.hit_tick = max(0, row["ticks"] - hit["age"]) if hit is not None else None

    def observe(self, row: dict) -> list[dict]:
        reset = self.timeline_restarted(row)
        accident_ticks = row.get("ambience", {}).get("accident_ticks")
        if reset:
            self.hit_tick = None
            self.accident_ticks = None
            self.epoch += 1
        reasons = []
        if self.stage is not None and self.stage != row["stage"]:
            reasons.append("stage_changed")
        if row["enemy_awake"] and not self.enemy_awake:
            reasons.append("enemy_noticed_rust")
        if reset:
            reasons.append("execution_reset")
        events = []
        if reasons:
            events.append({"cue": "transition", "reasons": reasons, "epoch": self.epoch})
        hit = row.get("hit")
        if hit is not None:
            tick = max(0, row["ticks"] - hit["age"])
            if hit["age"] <= 1 and self.hit_tick != tick:
                cue = "block" if hit["blocked"] else "hurt" if hit["target"] == "Player" else "strike"
                events.append({
                    "cue": cue, "contact_tick": tick, "observed_age": hit["age"],
                    "target": hit["target"], "blocked": hit["blocked"], "epoch": self.epoch,
                })
                self.hit_tick = tick
        if row["stage"] in {"Encounter", "Aftermath"} and accident_ticks is not None:
            for at, cue in TRAFFIC_MILESTONES:
                if accident_ticks >= at and (self.accident_ticks is None or self.accident_ticks < at):
                    events.append({"cue": cue, "milestone_tick": at,
                                   "observed_accident_tick": accident_ticks, "epoch": self.epoch})
        self.stage = row["stage"]
        self.stage_ticks = row.get("stage_ticks", 0)
        self.combat_ticks = row["ticks"]
        self.ambient_ticks = row.get("ambience", {}).get("ticks", 0)
        self.accident_ticks = accident_ticks
        self.enemy_awake = row["enemy_awake"]
        return events


def reconstruct(rows: list[dict], clips: dict[str, array], duration: float, rate: int = RATE) -> tuple[array, dict]:
    """Render telemetry intervals; freeze both music and cue cursors while paused."""
    if not rows or not math.isfinite(duration) or duration <= 0:
        raise ValueError("A nonempty timeline and positive finite duration are required")
    count = round(duration * rate)
    if count <= 0 or any(not clips.get(name) for name in TRACKS + CUES):
        raise ValueError("All original tracks/cues and at least one output sample are required")
    mix = array("i", [0]) * count
    origin = rows[0]["seconds"]
    position = 0
    music_cursor = 0
    track = None
    paused = False
    pause_start = None
    voices = {}
    events = []
    changes = []
    spans = []
    pauses = []
    seeks = []
    observer = Observer()

    def advance(target):
        nonlocal position, music_cursor
        target = min(count, max(position, target))
        length = target - position
        if length <= 0:
            return
        if paused:
            position = target
            return
        if track:
            pcm = clips[track]
            audible = min(length, max(0, len(pcm) - music_cursor)) if track in NON_LOOPING_TRACKS else length
            for offset in range(audible):
                mix[position + offset] += pcm[(music_cursor + offset) % len(pcm)]
            if audible and spans and spans[-1]["track"] == track and spans[-1]["end_sample"] == position and spans[-1]["source_end_sample"] == music_cursor:
                spans[-1]["end_sample"] = position + audible
                spans[-1]["source_end_sample"] = music_cursor + audible
            elif audible:
                spans.append({
                    "track": track, "start_sample": position, "end_sample": position + audible,
                    "source_start_sample": music_cursor, "source_end_sample": music_cursor + audible,
                })
            music_cursor += audible
        for cue, (cursor, event_index) in list(voices.items()):
            pcm = clips[cue]
            audible = min(length, len(pcm) - cursor)
            for offset in range(audible):
                mix[position + offset] += pcm[cursor + offset]
            if cursor + audible == len(pcm):
                events[event_index]["end_seconds"] = (position + audible) / rate
                del voices[cue]
            else:
                voices[cue] = (cursor + audible, event_index)
        position = target

    def stop_voices(cues, reason):
        for cue in cues:
            if cue in voices:
                _, event_index = voices.pop(cue)
                events[event_index]["end_seconds"] = position / rate
                events[event_index]["interrupted_by"] = reason

    for row in rows:
        event_sample = round((row["seconds"] - origin) * rate)
        if event_sample >= count:
            break
        advance(event_sample)
        synced_after_skip = row.get("audio_synced_after_skip", False)
        if synced_after_skip:
            stop_voices(CUES, "scene_skip")
            observer.align(row)
        elif observer.timeline_restarted(row):
            stop_voices(TRAFFIC_CUES, "execution_reset")
        elif row["stage"] not in {"Encounter", "Aftermath"}:
            stop_voices(TRAFFIC_CUES, "street_left")
        next_track = background_for(row)
        if next_track != track:
            changes.append({
                "seconds": position / rate, "previous_track": track, "track": next_track,
                "stage": row["stage"], "enemy_awake": row["enemy_awake"],
                "paused": row["paused"], "source_cursor_seconds": 0.0,
                "looping": next_track not in NON_LOOPING_TRACKS,
            })
            track = next_track
            music_cursor = 0
        if synced_after_skip:
            elapsed = round(row.get("stage_ticks", 0) / 60 * rate)
            music_cursor = (min(elapsed, len(clips[track])) if track in NON_LOOPING_TRACKS
                            else elapsed % len(clips[track]))
            seeks.append({"seconds": position / rate, "track": track,
                          "source_cursor_seconds": music_cursor / rate})
            if changes[-1]["seconds"] == position / rate:
                changes[-1]["source_cursor_seconds"] = music_cursor / rate
        if row["paused"] != paused:
            if row["paused"]:
                pause_start = position
            else:
                pauses.append({"start_seconds": pause_start / rate, "end_seconds": position / rate})
                pause_start = None
            paused = row["paused"]
        if paused:
            continue
        for event in observer.observe(row):
            cue = event["cue"]
            if cue in voices:
                previous_event = events[voices[cue][1]]
                previous_event["end_seconds"] = position / rate
                previous_event["interrupted_by_same_cue"] = True
            event.update({
                "seconds": position / rate, "stage": row["stage"], "ticks": row["ticks"],
                "file": f"{cue}.wav", "source_duration_seconds": len(clips[cue]) / rate,
            })
            events.append(event)
            voices[cue] = (0, len(events) - 1)
    advance(count)
    if pause_start is not None:
        pauses.append({"start_seconds": pause_start / rate, "end_seconds": count / rate})
    for _, event_index in voices.values():
        events[event_index]["end_seconds"] = count / rate
        events[event_index]["truncated_by_video_end"] = True

    raw_peak = max(abs(value) for value in mix) * VOLUME
    master_gain = min(1.0, 32760 / raw_peak) if raw_peak else 1.0
    pcm = array("h", (round(value * VOLUME * master_gain) for value in mix))
    for span in spans:
        for field in ("start", "end", "source_start", "source_end"):
            span[f"{field}_seconds"] = span.pop(f"{field}_sample") / rate
    report = {
        "mode": "offline_mix_reconstructed_from_execution_telemetry",
        "limitation": LIMITATION,
        "sample_rate": rate, "channels": 1, "sample_width_bits": 16,
        "duration_seconds": count / rate, "samples": count,
        "per_track_and_cue_volume": VOLUME,
        "master_gain_to_avoid_clipping": master_gain,
        "pre_master_peak_pcm": raw_peak,
        "output_peak_pcm": max(abs(value) for value in pcm),
        "clipped_samples": 0,
        "time_alignment": {
            "first_telemetry_seconds": origin,
            "mapping": "video_seconds = telemetry.seconds - first_telemetry_seconds",
            "telemetry_end_seconds": rows[-1]["seconds"] - origin,
            "last_state_hold_seconds": max(0, count / rate - (rows[-1]["seconds"] - origin)),
            "stage_ticks_present_in_all_samples": all("stage_ticks" in row for row in rows),
            "traffic_clock_present_in_all_samples": all("accident_ticks" in row.get("ambience", {}) for row in rows),
            "explicit_skip_marker_present_in_all_samples": all("audio_synced_after_skip" in row for row in rows),
        },
        "tracks": changes, "audible_music_spans": spans,
        "pause_intervals": pauses, "music_seeks": seeks,
        "events": events, "event_counts": dict(Counter(event["cue"] for event in events)),
    }
    return pcm, report


def write_wav(path: Path, pcm: array, rate: int = RATE):
    output = array("h", pcm)
    if sys.byteorder != "little":
        output.byteswap()
    with wave.open(str(path), "wb") as sound:
        sound.setnchannels(1)
        sound.setsampwidth(2)
        sound.setframerate(rate)
        sound.writeframes(output.tobytes())


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("directory", type=Path, help="Directory containing telemetry.jsonl and adventure-silent.mp4")
    parser.add_argument("--assets", type=Path, default=ROOT / "assets/adventure/audio")
    parser.add_argument("--force", action="store_true", help="Replace this directory's previous reconstructed outputs")
    args = parser.parse_args()
    directory = args.directory.resolve()
    telemetry = directory / "telemetry.jsonl"
    video = directory / "adventure-silent.mp4"
    destinations = [directory / name for name in ("adventure-mix.wav", "adventure.mp4", "mix-review.json")]
    try:
        if not args.force and any(path.exists() for path in destinations):
            raise ValueError("Reconstructed output already exists; use --force to replace it")
        if not video.is_file():
            raise ValueError(f"Missing silent video: {video}")
        rows = read_telemetry(telemetry)
        probe = probe_video(video)
        clips, sources = load_wavs(args.assets)
        pcm, report = reconstruct(rows, clips, probe["duration_seconds"])
        report.update({
            "video": {"source": str(video), **probe, "copied_without_reencoding": True},
            "telemetry": {"file": str(telemetry), "rows": len(rows), "sha256": hashlib.sha256(telemetry.read_bytes()).hexdigest()},
            "sources": sources,
            "outputs": {"wav": str(destinations[0]), "video": str(destinations[1])},
        })
        with tempfile.TemporaryDirectory(prefix="adventure-audio-", dir=directory) as temporary:
            staging = Path(temporary)
            wav = staging / "adventure-mix.wav"
            muxed = staging / "adventure.mp4"
            write_wav(wav, pcm)
            subprocess.run([
                "ffmpeg", "-v", "error", "-nostdin", "-y", "-i", str(video), "-i", str(wav),
                "-map", "0:v:0", "-map", "1:a:0", "-c:v", "copy", "-c:a", "aac", "-b:a", "128k",
                "-ac", "1", "-ar", str(RATE), "-t", f"{probe['duration_seconds']:.9f}",
                "-movflags", "+faststart", str(muxed),
            ], check=True)
            metadata = staging / "mix-review.json"
            metadata.write_text(json.dumps(report, ensure_ascii=False, indent=2) + "\n")
            for source, destination in zip((wav, muxed, metadata), destinations):
                source.replace(destination)
        print(f"Reconstructed review audio: {destinations[1]}")
        print(f"{report['duration_seconds']:.3f}s, {len(report['events'])} cues; report: {destinations[2]}")
        print(LIMITATION)
    except (OSError, ValueError, wave.Error, subprocess.CalledProcessError) as error:
        print(f"Cannot reconstruct adventure review audio: {error}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
