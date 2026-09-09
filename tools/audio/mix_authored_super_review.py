#!/usr/bin/env python3
"""Mux deterministic review frames with an explicitly offline event-driven mix.

Requires NumPy, FFmpeg and FFprobe. Replays exported AudioEvents with manifest
binding specificity, shared round-robin cursors, one voice per Sound, and music
pause/resume. This is audiovisual evidence tooling, not runtime audio playback.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import subprocess
import tempfile
from pathlib import Path

import numpy as np

ROOT = Path(__file__).resolve().parents[2]
RATE = 48000


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def decode(path: Path, pitch: float, limit_seconds: float | None = None) -> np.ndarray:
    probe = json.loads(subprocess.check_output([
        "ffprobe", "-v", "error", "-select_streams", "a:0", "-show_entries",
        "stream=channels", "-of", "json", str(path),
    ]))
    channels = probe["streams"][0]["channels"]
    if channels not in (1, 2):
        raise ValueError(f"Unsupported channel layout in {path}")
    command = ["ffmpeg", "-v", "error", "-i", str(path)]
    if limit_seconds is not None:
        command += ["-t", str(limit_seconds)]
    command += [
        "-af", f"aresample={RATE},asetrate={RATE * pitch:.8f},aresample={RATE}",
        "-f", "f32le", "pipe:1",
    ]
    samples = np.frombuffer(subprocess.check_output(command), dtype="<f4").reshape(-1, channels)
    # The native mixer duplicates mono into its stereo buffer before pan law.
    return np.repeat(samples, 2, axis=1) if channels == 1 else samples


def native_pan_gains(volume: float, native_pan: float) -> np.ndarray:
    """Mirror the bundled raylib 6 raudio.c fast-sine stereo pan law."""
    right = (native_pan + 1.0) / 2.0
    sides = np.array([1.0 - right, right])
    return volume * 0.5 * sides * (3.0 - sides * sides)


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--video", type=Path, required=True, help="Completed silent capture")
    parser.add_argument("--events", type=Path, required=True, help="Harness JSON with cue_key metadata")
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--report", type=Path, required=True)
    parser.add_argument("--music-volume", type=float, default=0.5, help="App default is 0.5")
    args = parser.parse_args()
    if args.output.exists():
        parser.error("Output exists; select a new path or explicitly remove the old evidence first")
    manifest_path = ROOT / "assets/audio/audio_manifest.json"
    manifest = json.loads(manifest_path.read_text())
    schedule = json.loads(args.events.read_text())
    fps = schedule["fps"]
    duration = schedule["video_frame_count"] / fps
    samples = round(duration * RATE)
    mix = np.zeros((samples, 2), dtype=np.float64)
    clips = {clip["id"]: clip for clip in manifest["clips"]}
    tracks = {track["id"]: track for track in manifest["music"]}
    cursors = [0] * len(manifest["bindings"])
    cache: dict[str, np.ndarray] = {}
    active: dict[str, tuple[int, int]] = {}  # clip -> sample offset, trace entry
    trace: list[dict] = []
    music_trace: list[dict] = []
    track_cache: dict[str, np.ndarray] = {}
    current_track: str | None = None
    music_cursor = 0
    paused = False
    position = 0
    timeline = []
    for scenario in schedule["scenarios"]:
        timeline.append((scenario["video_frames"][0], {"music_track": scenario["music_track"]}))
        timeline.extend((event["video_frame"], event) for event in scenario["audio_events"])
    timeline.sort(key=lambda item: item[0])  # Stable: preserve same-frame event order.

    def clip_samples(clip_id: str) -> np.ndarray:
        if clip_id not in cache:
            clip = clips[clip_id]
            volume = np.clip(clip.get("volume", 1.0), 0.0, 1.0)
            pitch = max(0.01, clip.get("pitch", 1.0))
            # AudioPlayer passes this existing manifest value directly to raylib.
            pan = float(np.clip(clip.get("pan", 0.5), 0.0, 1.0))
            cache[clip_id] = decode(ROOT / clip["file"], pitch) * native_pan_gains(volume, pan)
        return cache[clip_id]

    def advance(target: int) -> None:
        nonlocal position, music_cursor
        target = min(target, samples)
        count = target - position
        if count <= 0:
            return
        if current_track and not paused:
            music = track_cache[current_track]
            indices = (np.arange(count) + music_cursor) % len(music)
            mix[position:target] += music[indices]
            music_trace.append({
                "track": current_track, "start_frame": position * fps / RATE,
                "end_frame": target * fps / RATE,
                "source_cursor_start_seconds": music_cursor / RATE,
                "source_cursor_end_seconds": (music_cursor + count) / RATE,
            })
            music_cursor = (music_cursor + count) % len(music)
        for clip_id, (offset, entry) in list(active.items()):
            audio = clip_samples(clip_id)
            length = min(count, len(audio) - offset)
            mix[position:position + length] += audio[offset:offset + length]
            if offset + length >= len(audio):
                trace[entry]["actual_end_frame"] = (position + length) * fps / RATE
                del active[clip_id]
            else:
                active[clip_id] = (offset + length, entry)
        position = target

    for frame, event in timeline:
        advance(round(frame * RATE / fps))
        if "music_track" in event:
            track_id = event["music_track"]
            paused = False
            if track_id != current_track:
                current_track, music_cursor = track_id, 0
            if track_id not in track_cache:
                track = tracks[track_id]
                pitch = max(0.01, track.get("pitch", 1.0))
                volume = np.clip(track.get("volume", 0.45), 0.0, 1.0) * np.clip(args.music_volume, 0, 1)
                track_cache[track_id] = decode(ROOT / track["file"], pitch, duration + 1) * native_pan_gains(volume, 0.0)
            continue
        cue = event["cue_key"]
        if cue == "super.start":
            for _, entry in active.values():
                trace[entry].update(actual_end_frame=frame, interrupted_by="super.start")
            active.clear()
            paused = True
        elif cue == "super.end":
            paused = False
        matches = []
        for index, binding in enumerate(manifest["bindings"]):
            context = {"character": event.get("character"), "move": event.get("move_id"), "environment": event.get("environment")}
            if binding["cue"] == cue and all(binding.get(key) is None or binding[key] == value for key, value in context.items()):
                matches.append((sum(binding.get(key) is not None for key in context), index))
        if not matches:
            raise ValueError(f"Unresolved event: {event}")
        _, binding_index = max(matches)  # Rust max_by_key selects the last tied binding.
        candidates = manifest["bindings"][binding_index]["clips"]
        cursor = cursors[binding_index] % len(candidates)
        clip_id = candidates[cursor]
        cursors[binding_index] = (cursor + 1) % len(candidates)
        if clip_id in active:
            trace[active[clip_id][1]].update(actual_end_frame=frame, interrupted_by="same_sound_restarted")
        clip = clips[clip_id]
        entry = len(trace)
        trace.append({
            "cue": cue, "character": event.get("character"), "move_id": event.get("move_id"),
            "video_frame": frame, "start_seconds": frame / fps,
            "binding_index": binding_index, "rotation_index": cursor, "clip_id": clip_id,
            "file": clip["file"], "source_sha256": sha256(ROOT / clip["file"]),
            "volume": clip.get("volume", 1.0), "pitch": clip.get("pitch", 1.0),
            "manifest_pan_passed_to_raylib": clip.get("pan", 0.5),
        })
        active[clip_id] = (0, entry)
        clip_samples(clip_id)
    advance(samples)
    for _, entry in active.values():
        trace[entry]["actual_end_frame"] = schedule["video_frame_count"]
    peak = float(np.max(np.abs(mix)))
    if not np.isfinite(mix).all():
        raise ValueError("Mix contains non-finite samples")
    # Simultaneous offline starts can sum more sharply than device scheduling.
    # One constant gain preserves the entire manifest's relative dynamics.
    master_gain = min(1.0, 10 ** (-2.0 / 20.0) / max(peak, 1e-12))
    mix *= master_gain
    args.output.parent.mkdir(parents=True, exist_ok=True)
    with tempfile.TemporaryDirectory(prefix="borrow-super-mix-") as temp:
        pcm = Path(temp) / "mix.f32"
        mix.astype("<f4").tofile(pcm)
        subprocess.run([
            "ffmpeg", "-v", "error", "-n", "-i", str(args.video),
            "-f", "f32le", "-ar", str(RATE), "-ac", "2", "-i", str(pcm),
            "-map", "0:v:0", "-map", "1:a:0", "-c:v", "copy", "-c:a", "aac",
            "-b:a", "192k", "-t", str(duration), "-movflags", "+faststart",
            str(args.output),
        ], check=True)
    report = {
        "method": "Offline reconstruction of exported real gameplay events; this video does not claim live Pulse capture. Runtime AudioPlayer pause/resume is verified separately.",
        "video": str(args.output), "video_sha256": sha256(args.output),
        "silent_video_sha256": sha256(args.video), "event_json_sha256": sha256(args.events),
        "manifest_sha256": sha256(manifest_path), "fps": fps, "duration_seconds": duration,
        "sample_rate": RATE, "channels": 2, "music_volume_multiplier": args.music_volume,
        "music_default_note": "0.5 matches App's DEFAULT_MUSIC_VOLUME_PERCENT=50; the earlier live review harness used 1.0.",
        "pan_note": "Mirrors current AudioPlayer direct pan call and the bundled raylib 6 native fast-sine law. Native music pan is 0; Sound manifest defaults to0.5. No production pan values changed.",
        "unattenuated_mix_peak_dbfs": float(20 * np.log10(max(peak, 1e-12))),
        "mix_peak_dbfs": float(20 * np.log10(max(peak * master_gain, 1e-12))),
        "master_gain": master_gain,
        "master_gain_db": float(20 * np.log10(master_gain)),
        "master_normalization_or_limiter": "Constant attenuation only when needed for -2dBFS peak headroom; no compressor or limiter, preserve relative manifest dynamics",
        "sound_events": trace, "music_playback_intervals": music_trace,
        "music_sources": {key: {"file": tracks[key]["file"], "sha256": sha256(ROOT / tracks[key]["file"])} for key in track_cache},
    }
    args.report.parent.mkdir(parents=True, exist_ok=True)
    args.report.write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps({"output": str(args.output), "events": len(trace), "seconds": duration, "peak_dbfs": report["mix_peak_dbfs"]}))


if __name__ == "__main__":
    main()
