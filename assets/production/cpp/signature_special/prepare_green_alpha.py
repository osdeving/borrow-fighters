"""Extract the reviewed green matte while retaining white clothing and violet energy.

Local alpha preparation is authorized in assets/production/README.md. This only
removes the generated background and unmixes its edge color; it never poses or
paints the character. The original generated RGB sheet stays unchanged.
"""
from pathlib import Path
import numpy as np
from PIL import Image

root = Path(__file__).parent
rgba = np.array(Image.open(root / 'source-green-v3.png').convert('RGBA'))
rgb = rgba[:, :, :3].astype(np.float32)
spill = np.maximum(0, rgb[:, :, 1] - np.maximum(rgb[:, :, 0], rgb[:, :, 2]))
alpha = 1.0 - spill / 255.0
background = (spill > 170) & (rgb[:, :, 1] > 180)
alpha[background] = 0
mixed = (spill > 12) & ~background
for channel, key in enumerate((0.0, 255.0, 0.0)):
    clean = (rgb[:, :, channel] - (1.0 - alpha) * key) / np.maximum(alpha, .01)
    rgb[:, :, channel] = np.where(mixed, np.clip(clean, 0, 255), rgb[:, :, channel])
# The costume and violet arc contain no green pigment; remove the last
# sub-threshold green edge spill without touching gold, white, or violet.
rgb[:, :, 1] = np.minimum(rgb[:, :, 1], np.maximum(rgb[:, :, 0], rgb[:, :, 2]))
rgba[:, :, :3] = rgb.astype(np.uint8)
rgba[:, :, 3] = np.minimum(rgba[:, :, 3], np.round(alpha * 255).astype(np.uint8))
rgba[rgba[:, :, 3] == 0] = 0
Image.fromarray(rgba).save(root / 'keyed-green-v3.png')
print('Prepared green matte; violet VFX retained, transparent pixels:', int((rgba[:, :, 3] == 0).sum()))
