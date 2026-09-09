"""Prepare the reviewed v6 matte without changing the generated character art."""

import sys
from pathlib import Path

import numpy as np
from PIL import Image

ROOT = Path(__file__).resolve().parent
sys.path.insert(0, str(ROOT.parents[3] / "tools" / "art"))
from remove_generated_checkerboard import remove_background as remove_pale
from remove_generated_magenta import remove_background as remove_magenta

remove_magenta(ROOT / "source-v6.png", ROOT / "cleaned-v6.png", rust_contour=True)
remove_pale(ROOT / "cleaned-v6.png", ROOT / "keyed-v6.png")

# The edit retained a small pink-white matte pocket between the raised arm and
# thigh. This reviewed background region contains no eyes, sole or skin.
# Remove the 27 residual pale pixels; the source and dark outline are preserved.
rgba = np.array(Image.open(ROOT / "keyed-v6.png"))
region = rgba[427:481, 768:867]
rgb = region[:, :, :3].astype(int)
matte = (rgb.min(axis=2) > 85) & (rgb.max(axis=2) - rgb.min(axis=2) < 100)
matte &= region[:, :, 3] > 0
print(f"Reviewed matte pocket: {int(matte.sum())} pixels")
region[matte] = 0
Image.fromarray(rgba).save(ROOT / "keyed-v6.png")
