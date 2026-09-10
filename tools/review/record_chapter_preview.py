#!/usr/bin/env python3
"""Record Chapter 01 in an isolated Pulse sink, then align its actual native audio.

Run with Python containing NumPy (analysis only), FFmpeg, pactl and the native
X11 capture prerequisites. No build occurs. The capture helper owns its window
and evidence-local saves; this wrapper owns only its child processes and sink.
Audio is never reconstructed, normalized, stretched or shifted by correlation.

Example, after building the desired executable:
  python3.13 tools/review/record_chapter_preview.py \
    --output-directory /absolute/scratch/chapter-preview \
    --video-output /absolute/evidence/chapter-01.mp4

Use --finalize-only to retry analysis of a completed recording without launching
the game. --remove-raw-after-verification removes only this tool's PCM MKA after
the mux, stream preservation, levels and all three phone cues have passed.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import math
import os
from pathlib import Path
import shutil
import signal
import subprocess
import sys
import time
import uuid


ROOT = Path(__file__).resolve().parents[2]
RATE = 48000
ANALYSIS_RATE = 22050
RAW_LIMIT = 40 * 1024 * 1024
PHONE_EVENTS = ((210, "phone_send"), (510, "phone_receive"), (720, "phone_send"))


def write_json(path, value):
    path.write_text(json.dumps(value, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")


def sha256(path):
    digest = hashlib.sha256()
    with path.open("rb") as source:
        for chunk in iter(lambda: source.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


class Commands:
    """Keep exact commands and stderr, without copying decoded PCM to disk."""

    def __init__(self, directory):
        self.directory = directory

    def note(self, command):
        with (self.directory / "commands.jsonl").open("a", encoding="utf-8") as file:
            file.write(json.dumps({"wall_seconds": time.time(), "argv": command}) + "\n")

    def run(self, command, timeout=60):
        self.note(command)
        with (self.directory / "analysis.log").open("ab") as log:
            return subprocess.check_output(command, stderr=log, timeout=timeout, cwd=ROOT)

    def probe(self, path):
        return json.loads(self.run([
            "ffprobe", "-v", "error", "-show_streams", "-show_format", "-of", "json", str(path),
        ]))

    def decode(self, path, filters=None, mono=False):
        import numpy as np
        command = ["ffmpeg", "-v", "error", "-nostdin", "-i", str(path), "-map", "0:a:0"]
        if filters:
            command += ["-af", filters]
        if mono:
            command += ["-ar", str(ANALYSIS_RATE), "-ac", "1"]
        command += ["-f", "f32le", "-"]
        return np.frombuffer(self.run(command), dtype="<f4")

    def video_hash(self, path):
        return self.run([
            "ffmpeg", "-v", "error", "-nostdin", "-i", str(path), "-map", "0:v:0",
            "-c:v", "copy", "-f", "hash", "-hash", "sha256", "-",
        ]).decode().strip().split("=", 1)[1]


def stop_owned(process, label, events):
    if process is None or process.poll() is not None:
        return
    # First let the helper release its keys, close its owned game and flush MP4.
    # FFmpeg handles SIGINT by completing its MKA trailer.
    try:
        process.send_signal(signal.SIGINT)
    except ProcessLookupError:
        process.wait(timeout=5)
        return
    events.append({"process": label, "pid": process.pid, "signal": "SIGINT"})
    try:
        process.wait(timeout=20)
    except subprocess.TimeoutExpired:
        if label == "capture_helper":
            # This helper and its descendants are in our newly created session.
            os.killpg(process.pid, signal.SIGTERM)
        else:
            process.terminate()
        events.append({"process": label, "pid": process.pid, "signal": "SIGTERM"})
        try:
            process.wait(timeout=5)
        except subprocess.TimeoutExpired:
            if label == "capture_helper":
                os.killpg(process.pid, signal.SIGKILL)
            else:
                process.kill()
            events.append({"process": label, "pid": process.pid, "signal": "SIGKILL"})
            process.wait(timeout=5)


def record(args, commands):
    directory = args.output_directory
    raw = directory / "native-audio.mka"
    sink = "borrow_chapter_" + uuid.uuid4().hex[:16]
    state = {
        "mode": "native_isolated_pulseaudio_monitor", "completed": False,
        "started_wall_seconds": time.time(), "sink": sink, "module": None,
        "executable": str(args.executable), "executable_sha256": sha256(args.executable),
        "helper": str(ROOT / "tools/review/capture_chapter_x11.py"),
        "helper_sha256": sha256(ROOT / "tools/review/capture_chapter_x11.py"),
        "raw_audio": str(raw), "raw_byte_limit": RAW_LIMIT, "cleanup": [],
    }
    recorder = helper = None
    module = None
    failure = None
    try:
        module = commands.run([
            "pactl", "load-module", "module-null-sink", f"sink_name={sink}",
            "rate=48000", "channels=2",
        ]).decode().strip()
        if not module.isdecimal():
            raise RuntimeError(f"Unexpected module identifier: {module!r}")
        state["module"] = int(module)
        command = [
            "ffmpeg", "-nostdin", "-n", "-loglevel", "info", "-copyts",
            "-f", "pulse", "-sample_rate", str(RATE), "-channels", "2",
            "-fragment_size", "4096", "-i", sink + ".monitor", "-c:a", "pcm_s16le",
            "-fs", str(RAW_LIMIT), str(raw),
        ]
        commands.note(command)
        with (directory / "pulse.log").open("wb") as pulse_log:
            recorder = subprocess.Popen(command, stdout=pulse_log, stderr=subprocess.STDOUT, cwd=ROOT)
            state["recorder_pid"] = recorder.pid
            deadline = time.monotonic() + 8
            while not raw.exists() or raw.stat().st_size == 0:
                if recorder.poll() is not None:
                    raise RuntimeError("Pulse recorder exited before initialization; see pulse.log")
                if time.monotonic() >= deadline:
                    raise TimeoutError("Pulse recorder did not produce its MKA header")
                time.sleep(0.05)
            environment = os.environ.copy()
            environment["PULSE_SINK"] = sink
            command = [
                sys.executable, state["helper"], "--preview", "--output-directory",
                str(directory / "capture"), "--executable", str(args.executable),
                "--display", args.display, "--timeout", str(args.timeout),
                "--dialogue-ticks", str(args.dialogue_ticks), "--max-retries", "0",
            ]
            commands.note(command)
            with (directory / "capture.log").open("wb") as capture_log:
                helper = subprocess.Popen(command, stdout=capture_log, stderr=subprocess.STDOUT,
                                          env=environment, cwd=ROOT, start_new_session=True)
                state["capture_helper_pid"] = helper.pid
                write_json(directory / "recording.json", state)
                deadline = time.monotonic() + args.timeout + 30
                while helper.poll() is None:
                    if recorder.poll() is not None:
                        raise RuntimeError("Pulse recorder exited while the game was running")
                    if raw.stat().st_size >= RAW_LIMIT:
                        raise RuntimeError("PCM recording reached its 40 MiB limit; capture stopped")
                    if time.monotonic() >= deadline:
                        raise TimeoutError("Capture helper exceeded its deadline")
                    time.sleep(0.1)
                state["capture_helper_exit"] = helper.returncode
                if helper.returncode:
                    raise RuntimeError(f"Native chapter checks failed ({helper.returncode}); see capture.log")
                state["completed"] = True
    except BaseException as error:
        failure = error
        state["error"] = f"{type(error).__name__}: {error}"
    finally:
        # Only owned processes and the exact module ID created above are touched.
        # On failure, stop PCM growth before waiting for the window's cleanup.
        for process, label in ((recorder, "pulse_recorder"), (helper, "capture_helper")):
            try:
                stop_owned(process, label, state["cleanup"])
            except (OSError, subprocess.SubprocessError) as error:
                state["cleanup"].append({"process": label, "error": str(error)})
                failure = failure or error
        if recorder is not None:
            state["recorder_exit"] = recorder.returncode
        if module is not None and module.isdecimal():
            try:
                commands.run(["pactl", "unload-module", module])
                state["cleanup"].append({"unloaded_own_module": int(module)})
            except (OSError, subprocess.SubprocessError) as error:
                state["cleanup"].append({"module": module, "error": str(error)})
                failure = failure or error
        state["stopped_wall_seconds"] = time.time()
        state["completed"] = state["completed"] and failure is None
        write_json(directory / "recording.json", state)
    if failure is not None:
        raise failure


def alignment(rows, packets):
    first = rows[0]
    origin = float(first["wall_seconds"]) - float(first["capture_seconds"])
    audio_start = float(packets[0]["pts_time"])
    trim = origin - audio_start
    if not math.isfinite(trim) or not 0 <= trim <= 60:
        raise ValueError(f"Invalid measured audio preroll {trim}; refusing to guess an offset")
    offsets = [float(r["wall_seconds"]) - float(r["capture_seconds"]) for r in rows]
    if not all(math.isfinite(value) for value in offsets):
        raise ValueError("Non-finite telemetry clock")
    elapsed = 0.0
    errors = []
    for packet in packets:
        errors.append(float(packet["pts_time"]) - audio_start - elapsed)
        elapsed += int(packet["size"]) / (RATE * 2 * 2)
    return {
        "video_origin_wall_seconds": origin, "audio_first_packet_wall_seconds": audio_start,
        "audio_preroll_trim_seconds": trim,
        "telemetry_origin_spread_ms": (max(offsets) - min(offsets)) * 1000,
        "packet_wallclock_minus_pcm_clock_range_ms": [min(errors) * 1000, max(errors) * 1000],
        "packet_pcm_seconds": elapsed, "native_latency_preserved": True,
        "timestamp_precision_note": "MKA PTS are millisecond precision; decoded PCM advances on its sample clock.",
        "video_sampling_uncertainty_ms": 1000 / 30,
    }


def waveform_match(recorded, reference, expected, radius=0.4):
    """Locate a reference only inside its telemetry window; do not shift output."""
    import numpy as np
    template = np.asarray(reference[:round(0.18 * ANALYSIS_RATE)], dtype=np.float64)
    low = max(0, round((expected - radius) * ANALYSIS_RATE))
    high = min(len(recorded) - len(template), round((expected + radius) * ANALYSIS_RATE))
    if high <= low or len(template) < 2:
        raise ValueError("Phone event outside the recorded PCM interval")
    search = np.asarray(recorded[low:high + len(template)], dtype=np.float64)
    count = 1 << (len(search) + len(template) - 2).bit_length()
    cross = np.fft.irfft(np.fft.rfft(search, count) * np.fft.rfft(template[::-1], count), count)
    cross = cross[len(template) - 1:len(search)]
    sums = np.r_[0.0, np.cumsum(search ** 2)]
    energy = np.maximum(sums[len(template):] - sums[:-len(template)], 0)
    scores = cross / (np.sqrt(energy * np.dot(template, template)) + 1e-20)
    # FFT roundoff in a silent window must not become a huge normalized score.
    scores[energy <= max(float(np.max(energy)) * 1e-12, 1e-20)] = 0
    scores = np.clip(scores, -1.0, 1.0)
    index = int(np.argmax(scores))
    onset = (low + index) / ANALYSIS_RATE
    return {
        "pcm_onset_seconds": onset, "expected_pcm_seconds": expected,
        "latency_ms": (onset - expected) * 1000, "correlation": float(scores[index]),
        "template_seconds": len(template) / ANALYSIS_RATE, "search_radius_seconds": radius,
        "analysis_band_hz": None,
    }


def levels(samples):
    import numpy as np
    if not len(samples) or not np.all(np.isfinite(samples)):
        raise ValueError("Empty or non-finite audio")
    return {
        "peak_normalized": float(np.max(np.abs(samples))),
        "rms_normalized": float(np.sqrt(np.mean(samples.astype(np.float64) ** 2))),
        "samples_at_or_beyond_full_scale": int(np.sum(np.abs(samples) >= 1)),
    }


def finalize(args, commands):
    directory = args.output_directory
    state = json.loads((directory / "recording.json").read_text())
    if not state.get("completed") or "stopped_wall_seconds" not in state:
        raise ValueError("Recording did not finish successfully; inspect recording.json first")
    raw = directory / "native-audio.mka"
    video = directory / "capture/adventure-silent.mp4"
    telemetry = directory / "capture/telemetry.jsonl"
    checks_path = directory / "capture/native-checks.json"
    checks = json.loads(checks_path.read_text())
    if not checks.get("success") or checks.get("mode") != "preview":
        raise ValueError("Capture helper did not report successful preview checks")
    rows = [json.loads(line) for line in telemetry.read_text().splitlines() if line]
    if not rows or any(r["paused"] or r.get("menu") is not None for r in rows):
        raise ValueError("Continuous preview requires nonempty, unpaused telemetry")
    if rows[0]["phase"] != "Intro" or rows[-1]["phase"] != "Complete":
        raise ValueError("Preview must preserve the complete chapter from Intro to Complete")
    if checks.get("combat_retries", 0):
        raise ValueError("Preview contains a retry; retain it as functional evidence instead")
    video_probe, audio_probe = commands.probe(video), commands.probe(raw)
    write_json(directory / "source-video-probe.json", video_probe)
    write_json(directory / "source-audio-probe.json", audio_probe)
    source = next(s for s in video_probe["streams"] if s["codec_type"] == "video")
    native = next(s for s in audio_probe["streams"] if s["codec_type"] == "audio")
    if (native["codec_name"], int(native["sample_rate"]), native["channels"]) != ("pcm_s16le", RATE, 2):
        raise ValueError("Expected native stereo PCM16 at 48000 Hz")
    duration = float(source["duration"])
    frames = int(source["nb_frames"])
    if source["r_frame_rate"] != "30/1" or frames <= 0 or not math.isfinite(duration):
        raise ValueError("Unexpected video timebase or empty video")
    packets = json.loads(commands.run([
        "ffprobe", "-v", "error", "-select_streams", "a:0", "-show_packets", "-show_entries",
        "packet=pts_time,duration_time,size", "-of", "json", str(raw),
    ]))["packets"]
    write_json(directory / "audio-packets.json", packets)
    timing = alignment(rows, packets)
    trim = timing["audio_preroll_trim_seconds"]
    if timing["packet_pcm_seconds"] + 1 / RATE < trim + duration:
        raise ValueError("Native recording does not cover the entire video; silence will not be fabricated")
    filters = f"asetpts=N/SR/TB,atrim=start={trim:.9f}:duration={duration:.9f},asetpts=PTS-STARTPTS"
    recorded = commands.decode(raw, mono=True)
    captured = []
    for tick, name in PHONE_EVENTS:
        row = next((r for r in rows if r.get("phone") and r["phone"]["ticks"] >= tick), None)
        if row is None:
            raise ValueError(f"Missing phone milestone {tick}")
        reference = ROOT / "assets/adventure/chapter/audio" / f"{name}.wav"
        expected = float(row["capture_seconds"]) + trim
        match = waveform_match(recorded, commands.decode(reference, mono=True), expected)
        match["muxed_onset_seconds"] = match["pcm_onset_seconds"] - trim
        captured.append({"cue": name, "phone_tick": tick, "observed_phone_tick": row["phone"]["ticks"],
                         "frame": row["frame"], "video_seconds": row["capture_seconds"],
                         "reference": str(reference), "reference_sha256": sha256(reference), "match": match})
    write_json(directory / "phone-correlation.json", captured)
    for cue in captured:
        match = cue["match"]
        if match["correlation"] < 0.65 or not -50 <= match["latency_ms"] <= 300:
            raise ValueError(f"Native cue {cue['phone_tick']} did not validate; inspect phone-correlation.json")
    source_levels = levels(commands.decode(raw, filters))
    if source_levels["rms_normalized"] <= 1e-6 or source_levels["samples_at_or_beyond_full_scale"]:
        raise ValueError(f"Native recording is silent or clipped: {source_levels}")
    output = args.video_output
    output.parent.mkdir(parents=True, exist_ok=True)
    if output.exists() or output.with_name("native-audio.json").exists():
        raise FileExistsError("Final video or native-audio.json already exists; choose a fresh destination")
    temporary = output.with_name(output.stem + ".partial-" + uuid.uuid4().hex[:8] + ".mp4")
    # No -shortest or global -t: every original video packet must survive.
    commands.run([
        "ffmpeg", "-v", "error", "-nostdin", "-n", "-i", str(video), "-i", str(raw),
        "-map", "0:v:0", "-map", "1:a:0", "-c:v", "copy", "-af", filters,
        "-c:a", "aac", "-ar", str(RATE), "-b:a", "192k", "-movflags", "+faststart", str(temporary),
    ])
    original_hash, exported_hash = commands.video_hash(video), commands.video_hash(temporary)
    exported = commands.probe(temporary)
    out_video = next(s for s in exported["streams"] if s["codec_type"] == "video")
    out_audio = next(s for s in exported["streams"] if s["codec_type"] == "audio")
    encoded_levels = levels(commands.decode(temporary))
    if original_hash != exported_hash or int(out_video["nb_frames"]) != frames:
        raise ValueError("Compressed video stream or frame count changed")
    if (out_audio["codec_name"], int(out_audio["sample_rate"]), out_audio["channels"]) != ("aac", RATE, 2):
        raise ValueError("Unexpected output audio format")
    if encoded_levels["samples_at_or_beyond_full_scale"] or encoded_levels["rms_normalized"] <= 1e-6:
        raise ValueError("Decoded AAC is silent or clipped")
    if output.exists():
        raise FileExistsError(f"Final destination appeared during verification: {output}")
    temporary.rename(output)
    report = {
        "mode": state["mode"], "verified": True, "recording": str(directory / "recording.json"),
        "output": {"path": str(output), "sha256": sha256(output), "video_duration_seconds": duration,
                   "frames": frames, "fps": 30, "audio_codec": "aac", "audio_sample_rate": RATE,
                   "audio_channels": 2},
        "inputs": {name: {"path": str(path), "sha256": sha256(path)} for name, path in
                   (("video", video), ("native_pcm", raw), ("telemetry", telemetry), ("native_checks", checks_path))},
        "processing": {"audio_filter": filters, "video_copied_without_reencoding": True,
                       "source_compressed_video_sha256": original_hash,
                       "output_compressed_video_sha256": exported_hash, "cuts": 0,
                       "synthesized_replacement": False, "mix_reconstruction": False,
                       "gain_adjustment": False, "time_stretching": False,
                       "correlation_used_to_shift_audio": False, "analysis_filters_applied_to_output": False},
        "alignment": timing, "captured_phone_cues": captured,
        "levels": {"aligned_native_pcm": source_levels, "decoded_aac": encoded_levels},
        "telemetry": {"rows": len(rows), "first_phase": rows[0]["phase"],
                      "last_phase": rows[-1]["phase"], "paused_rows": 0, "combat_retries": 0},
        "raw_audio_retained": True,
        "limitations": [
            "Cue matching identifies local source signals in actual native output; it does not assess timbre or speakers.",
            "Only timestamp-derived preroll is removed. Native buffering latency remains audible and is measured, not corrected.",
            "Frame sampling, millisecond packet timestamps and the recorded telemetry clock spread limit sync precision.",
            "Source assets are original synthesized effects; the exported soundtrack is the actual game recording.",
        ],
    }
    destination = output.with_name("native-audio.json")
    write_json(destination, report)
    if args.remove_raw_after_verification:
        raw.unlink()
        report["raw_audio_retained"] = False
        write_json(destination, report)
    print(json.dumps({"video": str(output), "measurements": str(destination), "frames": frames,
                      "duration_seconds": duration, "phone_cues": len(captured)}, indent=2))


def main():
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--output-directory", type=Path, required=True,
                        help="Fresh absolute scratch/evidence directory, or completed input with --finalize-only")
    parser.add_argument("--video-output", type=Path, help="Absolute final MP4 path; defaults inside output-directory")
    parser.add_argument("--executable", type=Path, default=ROOT / "target/debug/borrow-adventure")
    parser.add_argument("--display", default=":0")
    parser.add_argument("--timeout", type=float, default=210, help="Helper timeout; PCM still has a 40 MiB cap")
    parser.add_argument("--dialogue-ticks", type=int, default=150)
    parser.add_argument("--finalize-only", action="store_true")
    parser.add_argument("--remove-raw-after-verification", action="store_true")
    args = parser.parse_args()
    if not args.output_directory.is_absolute() or args.video_output and not args.video_output.is_absolute():
        parser.error("Evidence directory and final video paths must be absolute")
    args.output_directory = args.output_directory.resolve()
    args.executable = args.executable.resolve(strict=True)
    args.video_output = (args.video_output or args.output_directory / "chapter-preview.mp4").resolve()
    if not 60 <= args.timeout <= 900 or not 90 <= args.dialogue_ticks <= 360:
        parser.error("Timeout must be 60–900 seconds and dialogue ticks 90–360")
    if args.video_output.suffix.lower() != ".mp4":
        parser.error("--video-output must end in .mp4")
    required = ["ffmpeg", "ffprobe"] + ([] if args.finalize_only else ["pactl"])
    if missing := [name for name in required if shutil.which(name) is None]:
        parser.error(f"Missing executables: {', '.join(missing)}")
    try:
        import numpy  # noqa: F401 — analysis prerequisite checked before recording
    except ImportError:
        parser.error("NumPy is required for measurement; select the existing python3.13 environment")
    if not args.finalize_only:
        args.output_directory.mkdir(parents=True, exist_ok=False)
    commands = Commands(args.output_directory)
    try:
        if not args.finalize_only:
            record(args, commands)
        finalize(args, commands)
    except (Exception, KeyboardInterrupt) as error:
        write_json(args.output_directory / "native-audio-failure.json",
                   {"error": f"{type(error).__name__}: {error}", "wall_seconds": time.time()})
        print(f"{type(error).__name__}: {error}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
