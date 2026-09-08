"""Pack this action's six explicitly mapped imagegen effects, without drawing art.

Reads fx-plan.json beside this file. The authored pivot may lie below a crop;
transparent padding registers that image against the physical ground anchor.
"""
from pathlib import Path
import hashlib
import json
from PIL import Image

folder = Path(__file__).resolve().parent
root = folder.parents[3]
plan = json.loads((folder / 'fx-plan.json').read_text())
character = plan['character']
output = root / 'assets' / 'candidates' / character
frames = []
for index, entry in enumerate(plan['frames']):
    image = Image.open(folder / entry['sheet']).convert('RGBA')
    rect = entry['source_rect']
    x, y, width, height = (rect[key] for key in ('x', 'y', 'w', 'h'))
    crop = image.crop((x, y, x + width, y + height))
    px = entry['pivot_source']['x'] - x
    py = entry['pivot_source']['y'] - y
    left, top = max(0, -px), max(0, -py)
    canvas = Image.new('RGBA', (max(width, px + 1) + left, max(height, py + 1) + top))
    canvas.paste(crop, (left, top))
    scale = entry['scale']
    canvas = canvas.resize((round(canvas.width * scale), round(canvas.height * scale)), Image.Resampling.LANCZOS)
    frames.append((entry, canvas, {'x': round((px + left) * scale), 'y': round((py + top) * scale)}))
cell_w = max(image.width for _, image, _ in frames)
cell_h = max(image.height for _, image, _ in frames)
atlas = Image.new('RGBA', (cell_w * 3, cell_h * 2))
runtime = []
clips = []
for clip, loop in [('projectile', True), ('impact', False)]:
    clips.append({'name': clip, 'loop': loop, 'frames': []})
for index, (entry, image, pivot) in enumerate(frames):
    name = f"{entry['clip']}_{index:02d}"
    x, y = index % 3 * cell_w, index // 3 * cell_h
    atlas.paste(image, (x, y))
    box = image.getchannel('A').getbbox()
    assert box and image.getchannel('A').getextrema()[0] == 0
    runtime.append({'name': name, 'clip': entry['clip'], 'duration_ms': entry['duration_ms'], 'pivot': pivot,
                    'frame': {'x': x, 'y': y, 'w': image.width, 'h': image.height},
                    'trimmed_bounds': dict(zip(('x', 'y', 'w', 'h'), (box[0], box[1], box[2]-box[0], box[3]-box[1])))})
    next(clip for clip in clips if clip['name'] == entry['clip'])['frames'].append(name)
    (folder / 'frames').mkdir(exist_ok=True)
    image.save(folder / 'frames' / f'{name}.png')
output.mkdir(exist_ok=True)
atlas_path = output / f'{character}-signature-fx-atlas.png'
manifest_path = output / f'{character}-signature-fx.sprite.json'
atlas.save(atlas_path)
manifest = {'schema': 'borrow-fighters.sprite.v1', 'image': atlas_path.name,
            'source': f'../../production/{character}/signature_fx/fx-plan.json',
            'cell': {'w': cell_w, 'h': cell_h}, 'default_pivot': frames[0][2], 'scale': 1.0,
            'frames': runtime, 'clips': clips, 'notes': [plan['notes']]}
manifest_path.write_text(json.dumps(manifest, indent=2) + '\n')
sha = lambda path: hashlib.sha256(path.read_bytes()).hexdigest()
(folder / 'export-audit.json').write_text(json.dumps({'frames': len(runtime), 'clips': clips,
    'atlas_sha256': sha(atlas_path), 'manifest_sha256': sha(manifest_path),
    'plan_sha256': sha(folder / 'fx-plan.json'), 'sources': {entry['sheet']: sha(folder / entry['sheet']) for entry in plan['frames']}}, indent=2) + '\n')
print(manifest_path)
