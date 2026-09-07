"""Render review GIFs and a contact sheet from exported runtime data.

These are previews of the generated sprites, not newly generated artwork.
Two facings share a fixed ground anchor; duration, scale and pivot come from
the exact JSON consumed by the game. Backgrounds are for review only.
"""

import argparse
import json
from pathlib import Path

from PIL import Image, ImageDraw


def render(manifest_path: Path) -> None:
    manifest = json.loads(manifest_path.read_text())
    output = manifest_path.parent / "review"
    output.mkdir(exist_ok=True)
    by_name = {f["name"]: f for f in manifest["frames"]}
    images = {}
    cards = []
    for clip in manifest["clips"]:
        playback = []
        durations = []
        for name in clip["frames"]:
            frame = by_name[name]
            path = frame.get("image", manifest["image"])
            if path not in images:
                images[path] = Image.open(manifest_path.parent / path).convert("RGBA")
            r = frame["frame"]
            sprite = images[path].crop((r["x"], r["y"], r["x"]+r["w"], r["y"]+r["h"]))
            scale = manifest.get("scale", 1.0)
            sprite = sprite.resize((round(sprite.width*scale), round(sprite.height*scale)), Image.Resampling.LANCZOS)
            px, py = (round(frame["pivot"][k]*scale) for k in ("x", "y"))
            card = Image.new("RGB", (960, 410), (23, 28, 37))
            draw = ImageDraw.Draw(card)
            draw.rectangle((480, 0, 960, 410), fill=(203, 207, 203))
            for center, flip in ((205, False), (755, True)):
                art = sprite.transpose(Image.Transpose.FLIP_LEFT_RIGHT) if flip else sprite
                x = center-(sprite.width-px) if flip else center-px
                card.paste(art, (x, 365-py), art)
            draw.line((0, 365, 960, 365), fill=(90, 125, 125))
            draw.text((12, 12), f"{clip['name']} | {name} | {frame['duration_ms']} ms | runtime size", fill="white")
            draw.text((492, 12), "mirrored / light background", fill=(30, 40, 45))
            playback.append(card)
            durations.append(frame["duration_ms"])
        # Retain final pose briefly for inspection; this hold is not runtime data.
        if not clip["loop"]:
            durations[-1] += 400
        playback[0].save(output / f"{clip['name']}.gif", save_all=True,
                         append_images=playback[1:], duration=durations, loop=0)
        montage = Image.new("RGB", (480*len(playback), 410), (23, 28, 37))
        for i, card in enumerate(playback):
            montage.paste(card.crop((0, 0, 480, 410)), (i*480, 0))
        montage.save(output / f"{clip['name']}-frames.png")
        cards.append(playback[min(1, len(playback)-1)].resize((480,205)))
    overview = Image.new("RGB", (960, 205*((len(cards)+1)//2)), (23,28,37))
    for i, card in enumerate(cards):
        overview.paste(card, ((i%2)*480, (i//2)*205))
    overview.save(output / "overview.png")
    print(f"Rendered {len(cards)} clips: {output}")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("manifest", type=Path)
    args = parser.parse_args()
    render(args.manifest)
