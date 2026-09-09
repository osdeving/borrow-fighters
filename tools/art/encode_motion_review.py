"""Encode real World captures at their recorded simulation times.

PNG files come from capture_motion_review's GPU readback. This only assembles
review footage; it does not draw, interpolate or change character artwork.
Use --fps 60 with --every-tick captures for one video frame per simulation tick.
"""

import argparse
import json
import math
from pathlib import Path
import subprocess


def encode(root: Path, fps: int = 60) -> dict:
    root = root.resolve()
    report = json.loads((root / "capture-report.json").read_text())
    captures = report["captures"]
    states = report["states"]
    if not captures or not states:
        raise ValueError("capture report needs nonempty captures and states")
    simulation_fps = round(1 / report["fixed_timestep"])
    if simulation_fps < 1 or not math.isclose(
        report["fixed_timestep"], 1 / simulation_fps, rel_tol=1e-5
    ):
        raise ValueError("expected an integer simulation tick rate")
    ticks = [capture["tick"] for capture in captures]
    if any(b <= a for a, b in zip(ticks, ticks[1:])):
        raise ValueError("capture ticks must be strictly increasing")
    first_tick, last_tick = states[0]["tick"], states[-1]["tick"]
    if ticks[0] != first_tick or ticks[-1] != last_tick:
        raise ValueError("capture images must include the first and last state")

    sequence = []
    for index, capture in enumerate(captures):
        path = (root / capture["image"]).resolve()
        if root not in path.parents or not path.is_file():
            raise ValueError("capture image must be an existing file inside the report directory")
        next_tick = ticks[index + 1] if index + 1 < len(ticks) else last_tick + 1
        sequence.append((path, next_tick - capture["tick"]))
    duration = (last_tick - first_tick + 1) / simulation_fps
    expected_frames = math.ceil((last_tick - first_tick + 1) * fps / simulation_fps)
    video_path = root / "runtime-motion.mp4"
    # One unchanged PNG per simulation tick gives image2pipe an exact timebase.
    # Older ffconcat demuxers default to 25 Hz for PNG input and do not support
    # per-file framerate options, which rounds 60 Hz timings to 40 ms intervals.
    command = [
        "ffmpeg", "-y", "-hide_banner", "-loglevel", "error", "-f", "image2pipe",
        "-framerate", str(simulation_fps), "-vcodec", "png", "-i", "pipe:0",
        "-vf", f"fps={fps}:round=near",
        "-frames:v", str(expected_frames), "-c:v", "libx264", "-pix_fmt", "yuv420p",
        str(video_path),
    ]
    with subprocess.Popen(command, stdin=subprocess.PIPE) as process:
        try:
            for path, held_ticks in sequence:
                png_bytes = path.read_bytes()
                for _ in range(held_ticks):
                    process.stdin.write(png_bytes)
        finally:
            process.stdin.close()
        if process.wait() != 0:
            raise subprocess.CalledProcessError(process.returncode, command)
    probe = json.loads(subprocess.check_output([
        "ffprobe", "-v", "error", "-select_streams", "v:0", "-show_entries",
        "stream=nb_frames,duration,r_frame_rate", "-of", "json", str(video_path),
    ], text=True))["streams"][0]
    if int(probe["nb_frames"]) != expected_frames:
        raise ValueError("encoded video frame count does not match the simulation timeline")

    # Limit the jump summary to the dedicated motion phase: attack mode also
    # jumps before both aerial moves, and those are separate observations.
    airborne = [state for state in states if state["phase"] == "jump_arc" and not state["p1"]["grounded"]]
    frame_map = {frame["name"]: frame for frame in report["loaded_manifest"]["frames"]}
    scale = report["loaded_manifest"].get("scale", 1.0)

    def mark(state: dict) -> dict:
        fighter = state["p1"]
        frame = frame_map[fighter["frame"]]
        trim = frame["trimmed_bounds"]
        body_bottom = fighter["body"][1] + fighter["body"][3]
        alpha_bottom = body_bottom + (trim["y"] + trim["h"] - frame["pivot"]["y"]) * scale
        return {
            "tick": state["tick"], "live_tick": state["live_tick"], "frame": fighter["frame"],
            "velocity_y": fighter["velocity"][1], "grounded": fighter["grounded"],
            "body_bottom_y": body_bottom, "alpha_bottom_y": alpha_bottom,
            "alpha_bottom_offset_from_pivot": alpha_bottom - body_bottom,
        }

    summary = {
        "captured_images": len(captures), "simulation_states": len(states),
        "video": video_path.name, "video_fps": fps, "simulation_fps": simulation_fps,
        "simulation_duration_seconds": duration, "encoded_video": probe,
        "every_tick_captured": ticks == list(range(first_tick, last_tick + 1)),
        "notes": [
            "Video timing follows recorded simulation ticks, with no extra final pause.",
            "Sparse captures hold the previous GPU image until the next sampled tick; use --every-tick when capturing to inspect every simulation frame.",
            "Alpha bounds include every opaque detail and do not identify which pixel is the foot.",
        ],
    }
    if airborne:
        first_air, last_air = airborne[0], airborne[-1]
        apex = min(airborne, key=lambda state: state["p1"]["position"][1])
        landing = next(state for state in states if state["tick"] > last_air["tick"])
        for player in ["p1", "p2"]:
            if first_air[player]["clip"] != "jump" or landing[player]["clip"] != "idle":
                raise ValueError("dedicated jump scenario failed to return to idle")
        summary.update({
            "first_airborne": mark(first_air), "apex": mark(apex),
            "last_airborne": mark(last_air), "landing": mark(landing),
        })
    (root / "motion-summary.json").write_text(json.dumps(summary, indent=2) + "\n")
    return summary


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("capture_directory", type=Path)
    parser.add_argument("--fps", type=int, choices=(20, 30, 60), default=60)
    args = parser.parse_args()
    print(json.dumps(encode(args.capture_directory, args.fps), indent=2))
