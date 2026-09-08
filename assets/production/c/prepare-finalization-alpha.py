"""Prepare preserved imagegen sheets with local alpha cleanup only.
No Rust color contour filter. Magenta is absent from both fixed costume palettes.
The second matte mask removes uneven dark magenta rejected by the general keyer;
it does not redraw or change opaque character pixels. Inspect both backgrounds.
"""
from pathlib import Path
import importlib.util
import sys
import numpy as np
from PIL import Image
root=Path(__file__).resolve().parents[3]
spec=importlib.util.spec_from_file_location("magenta",root/"tools/art/remove_generated_magenta.py")
module=importlib.util.module_from_spec(spec);spec.loader.exec_module(module)
source,dest=map(Path,sys.argv[1:3])
module.remove_background(source,dest,rust_contour=False)
src=np.asarray(Image.open(source).convert("RGBA")).astype(np.int16)
out=np.array(Image.open(dest).convert("RGBA"))
r,g,b=(src[:,:,i] for i in range(3))
matte=(np.minimum(r,b)>100)&(g<80)&(np.minimum(r,b)-g>80)
out[matte]=0
Image.fromarray(out).save(dest)
print(f"{dest}: uneven magenta mask removed {int(matte.sum())} source matte pixels")
