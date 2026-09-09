"""Prepare explicitly mapped high-resolution action frames at runtime resolution.

The review plan supplies one uniform scale per action; no pose or animation is
inferred from sheet order. Original sheets and action metadata remain intact.
This writes a separate prepared sheet per action and the exporter's input JSON.
"""

import argparse
import json
import math
from pathlib import Path

from PIL import Image


def prepare(plan_path: Path) -> None:
    root = plan_path.parent
    plan = json.loads(plan_path.read_text())
    production = {"schema": "borrow-fighters.production.v1", "character": plan["character"],
                  "scale": 1.0, "provenance": plan.get("provenance", {}), "actions": {}}
    for action_name, review in plan["actions"].items():
        if review.get("approved") is not True:
            continue
        action = json.loads((root / action_name / "action.json").read_text())
        source_sheet = review.get("sheet", action["sheet"])
        source = Image.open(root / source_sheet).convert("RGBA")
        factor = float(review["scale_to_runtime"])
        if not math.isfinite(factor) or factor <= 0:
            raise ValueError(f"{action_name}: invalid explicit scale")
        crops = []
        for frame in action["frames"]:
            if "combat" in frame:
                raise ValueError("Visual preparation must not transform combat metadata implicitly")
            rect = frame["source_rect"]
            x, y, w, h = (rect[k] for k in ("x", "y", "w", "h"))
            if min(x, y) < 0 or min(w, h) <= 0 or x+w > source.width or y+h > source.height:
                raise ValueError(f"{frame['name']}: source crop outside sheet")
            crop = source.crop((x, y, x+w, y+h))
            if crop.getchannel("A").getextrema()[0] != 0:
                raise ValueError(f"{frame['name']}: no real alpha")
            crop = crop.resize((max(1, round(w*factor)), max(1, round(h*factor))), Image.Resampling.LANCZOS)
            pivot = {k: round(frame["pivot"][k]*factor) for k in ("x", "y")}
            crops.append((frame, crop, pivot))
        width = max(c.width for _, c, _ in crops)
        height = max(c.height for _, c, _ in crops)
        sheet = Image.new("RGBA", (width*len(crops), height))
        prepared_frames = []
        for i, (frame, crop, pivot) in enumerate(crops):
            sheet.paste(crop, (i*width, 0))
            prepared_frames.append({"name": frame["name"], "source_rect": {
                "x": i*width, "y": 0, "w": crop.width, "h": crop.height},
                "pivot": pivot, "duration_ms": frame["duration_ms"],
                "phase": frame.get("phase", action_name)})
        sheet_name = f"{action_name}/prepared.png"
        sheet.save(root / sheet_name)
        production["actions"][action_name] = {"sheet": sheet_name, "reviewed": True,
            "loop": action["loop"], "frames": prepared_frames,
            "provenance": {"source_action": f"{action_name}/action.json",
                           "source_sheet": source_sheet, "scale_to_runtime": factor,
                           "review": review.get("notes", "")}}
    (root / "production.json").write_text(json.dumps(production, indent=2, ensure_ascii=False)+"\n")
    print(f"Prepared {len(production['actions'])} reviewed actions")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("plan", type=Path)
    args = parser.parse_args()
    prepare(args.plan)
