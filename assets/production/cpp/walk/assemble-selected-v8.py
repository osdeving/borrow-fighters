"""Pack four complete generated walk keys, preserving the keyed source pixels.

V6 supplies the two alternating contact keys and the first passing key. V7
supplies the corrected opposite passing key; its third key regressed and is
not selected. No body parts are assembled, painted, or geometrically warped.
"""

from pathlib import Path

from PIL import Image


root = Path(__file__).resolve().parent
sheet = Image.new("RGBA", (1920, 760))
selections = [
    ("keyed-v6.png", (0, 0, 480, 760)),
    ("keyed-v6.png", (480, 0, 960, 760)),
    ("keyed-v6.png", (960, 0, 1440, 760)),
    ("keyed-v7.png", (1439, 0, 1919, 760)),
]
for index, (name, rect) in enumerate(selections):
    sheet.paste(Image.open(root / name).convert("RGBA").crop(rect), (480 * index, 0))
sheet.save(root / "assembled-selected-v8.png")
