#!/usr/bin/env python3
"""Audit a complete native Augusta capture against the requested story blocking.

The audit consumes renderer telemetry without altering the game, video or save.
It complements visual inspection; it cannot grade anatomy or cinematic quality.
"""

import argparse
from collections import Counter
import json
from pathlib import Path


def audit(directory):
    rows = [json.loads(line) for line in (directory / "telemetry.jsonl").read_text().splitlines()]
    assert rows and rows[-1]["phase"] == "Complete", "capture must finish the chapter"
    phases = list(dict.fromkeys(row["phase"] for row in rows))
    assert "BrokerEscape" not in phases, "broker escape must happen during arrival"
    assert phases.index("JuliaAttempt") < phases.index("Confrontation") < phases.index("GuardsArrival")
    assert phases.index("GuardsFight") < phases.index("ErraticsArrival") < phases.index("ErraticsFight")
    firsts = {phase: next(row for row in rows if row["phase"] == phase) for phase in phases}
    guards = [row for row in rows if row["phase"] == "GuardsArrival"]
    for row in guards:
        assert all(not a["active"] for a in row["actors"] if a["character"] == "security")
    doors = {}
    for row in guards:
        for pose in row["arrivals"]:
            if pose["visible"]:
                doors.setdefault(pose["id"], [pose["x"], pose["y"]])
    assert len(doors) == 3, "all three guards must emerge"
    origin = next(iter(doors.values()))
    assert all(abs(p[0]-origin[0]) < 1. and abs(p[1]-origin[1]) < 1. for p in doors.values())
    last_poses = {p["id"]: p for p in guards[-1]["arrivals"]}
    for actor in firsts["GuardsFight"]["actors"]:
        if actor["character"] == "security":
            pose = last_poses[actor["id"]]
            assert abs(pose["x"]-actor["x"]) < 1. and abs(pose["y"]-actor["y"]) < 1.
    fleeing = [row for row in rows if row["phase"] == "ErraticsArrival"
               and any(p["character"] == "broker" and p["clip"] == "run" for p in row["npcs"])]
    assert fleeing, "broker must visibly run during EP arrival"
    assert any(a["character"] == "erratic" and a["hp"] > 0 for a in fleeing[0]["actors"])
    fight_frame = firsts["ErraticsFight"]["frame"]
    assert all(not any(p["character"] == "broker" for p in row["npcs"])
               for row in rows if row["frame"] >= fight_frame)
    for row in rows:
        if row["phase"] in ("Introduction", "JuliaAttempt", "Confrontation"):
            cast = {p["character"]: p for p in row["npcs"]}
            assert cast["julia"]["x"] > cast["broker"]["x"] > row["actors"][0]["x"]
    cues = Counter(cue for row in rows for cue in row["audio_cues"])
    for cue in ("bar-door", "ep-rupture", "panic"):
        assert cues[cue] == 1, f"{cue} must happen once"
    assert cues["guard-step"] > 2
    report = {
        "success": True,
        "frames": len(rows),
        "seconds": rows[-1]["seconds"] + 1/60,
        "final_phase": rows[-1]["phase"],
        "final_health": rows[-1]["actors"][0]["hp"],
        "phases": phases,
        "shots": list(dict.fromkeys(row["shot"]["name"] for row in rows if row["shot"])),
        "guard_door_origins": doors,
        "broker_first_running_frame": fleeing[0]["frame"],
        "erratics_combat_frame": fight_frame,
        "audio_cues": dict(cues),
        "scope": "Runtime state and timing; anatomy, lighting and cuts require separate visual inspection.",
    }
    (directory / "story-audit.json").write_text(json.dumps(report, ensure_ascii=False, indent=2) + "\n")
    return report


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("directory", type=Path)
    print(json.dumps(audit(parser.parse_args().directory), ensure_ascii=False, indent=2))
