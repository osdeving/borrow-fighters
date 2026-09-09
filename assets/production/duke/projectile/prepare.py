"""Uniformly fit the new generated coffee-bean texture; preserve runtime origin."""

import hashlib
import json
from pathlib import Path

from PIL import Image

root = Path(__file__).resolve().parent
source = root/'source-v1.png'
image = Image.open(root/'keyed-v1.png').convert('RGBA')
bounds = image.getchannel('A').getbbox()
art = image.crop(bounds)
# One uniform factor fits a detached cluster inside the previous150x124 canvas.
factor = min(142/art.width, 116/art.height)
art = art.resize((round(art.width*factor),round(art.height*factor)),Image.Resampling.LANCZOS)
canvas = Image.new('RGBA',(158,132))
offset = ((158-art.width)//2,(132-art.height)//2)
canvas.alpha_composite(art,offset)
output = root/'prepared-v1.png'
canvas.save(output)
canvas.save(root.parents[2]/'candidates/duke/duke-projectile.png')
report = {
    'operation':'New real image_gen texture replacing cropped muzzle flare and white halo; local alpha extraction and uniform fit only.',
    'generator':'built-in image_gen', 'source':source.name,
    'source_sha256':hashlib.sha256(source.read_bytes()).hexdigest(),
    'original_preserved':'source-existing.png', 'source_alpha_bounds':list(bounds),
    'uniform_scale':factor, 'prepared_size':list(canvas.size),
    'prepared_cluster_size':list(art.size),'prepared_offset':list(offset),
    'alpha_bounds':list(canvas.getchannel('A').getbbox()), 'runtime_scale':0.6,
    'runtime_canvas_size':[94.8,79.2],
    'origin':'Renderer still centers the texture at unchanged ProjectileSpec origin. Symmetric4px additional margin around the original150x124 canvas. No collision, origin, timing or physical-dimension change.',
    'difference':'The replacement has nine readable beans and a more compact horizontal spread; the cut muzzle explosion is removed. It is a new texture, not claimed as cleanup/reuse.'
}
(root/'production.json').write_text(json.dumps(report,indent=2)+'\n')
print(json.dumps(report,indent=2))
