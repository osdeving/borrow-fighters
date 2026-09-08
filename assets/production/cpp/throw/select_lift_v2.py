"""Select only the generated lift revision; preserve the other five source keys."""
from pathlib import Path
from PIL import Image
import json
folder=Path(__file__).resolve().parent
original=Image.open(folder/'keyed-extraordinary-v1-edge-clean.png').convert('RGBA')
revision=Image.open(folder/'keyed-lift-v2-edge-clean.png').convert('RGBA')
rect=(1050,0,1470,500)
original.paste(revision.crop(rect),(rect[0],rect[1]))
original.save(folder/'selected-lift-v2.png')
