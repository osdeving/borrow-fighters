"""Remove residual RED matte only at transparent contours of reviewed blue/gold art.

Input is the existing RGBA magenta-key output. This never removes blue pixels;
blue serpent detail must survive. Source raster and the first keyed result stay
separate. Inspect light/dark backgrounds before selecting the cleaned output.
"""
from pathlib import Path
import argparse
import numpy as np
from PIL import Image, ImageFilter

def clean(source, destination):
    rgba=np.asarray(Image.open(source).convert('RGBA')).copy()
    removed=0
    for _ in range(8):
        outside=Image.fromarray(np.where(rgba[:,:,3]==0,255,0).astype(np.uint8))
        edge=np.asarray(outside.filter(ImageFilter.MaxFilter(3)))>0
        r,g,b=(rgba[:,:,i].astype(float) for i in range(3))
        residue=edge & (rgba[:,:,3]>0) & (g<80) & (r>40) & (r>2.6*g) & (r>2*b)
        removed+=int(residue.sum())
        if not residue.any(): break
        rgba[residue]=0
    Image.fromarray(rgba).save(destination)
    print(str(destination)+': removed red boundary matte pixels='+str(removed))

if __name__=='__main__':
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('source',type=Path);p.add_argument('destination',type=Path)
    a=p.parse_args();clean(a.source,a.destination)
