"""Inspect exported human geometry and actions without Blender or the game.

This checks the actual GLB buffers, skin, named clips and animation timestamps.
It does not judge anatomy, identity, interpenetration or convincing motion;
those require the frame review alongside this export check.
"""

import argparse
import hashlib
import json
import math
from pathlib import Path
import struct

from validate_vehicles import accessor


ROOT = Path(__file__).resolve().parents[2]
CATALOG = ROOT / 'assets/adventure/models/humans.json'


def inspect(actor_id, entry):
    path = CATALOG.parent / entry['file']
    raw = path.read_bytes()
    assert struct.unpack_from('<4sII', raw) == (b'glTF', 2, len(raw)), path
    length, tag = struct.unpack_from('<I4s', raw, 12)
    assert tag == b'JSON'
    model = json.loads(raw[20:20 + length])
    binary_length, tag = struct.unpack_from('<I4s', raw, 20 + length)
    assert tag == b'BIN\0'
    binary = raw[28 + length:28 + length + binary_length]
    assert len(binary) == binary_length
    assert all('uri' not in item for item in model['buffers'] + model.get('images', []))
    assert not model.get('cameras')
    assert len(model.get('skins', [])) == len(model.get('meshes', [])) == 1
    skin = model['skins'][0]
    joints = [model['nodes'][number]['name'] for number in skin['joints']]
    assert len(joints) == 53 and len(set(joints)) == 53
    required = {'Root', 'pelvis', 'spine_01', 'spine_02', 'spine_03', 'neck_01', 'head'}
    required.update(f'{part}_{side}' for side in ('l', 'r')
                    for part in ('upperarm', 'lowerarm', 'hand', 'thigh', 'calf', 'foot'))
    assert required <= set(joints)
    cache = {}

    def values(index):
        if index not in cache:
            cache[index] = accessor(model, binary, index)
        return cache[index]

    vertices = triangles = 0
    weights_error = 0.0
    used = set()
    lower, upper = [math.inf] * 3, [-math.inf] * 3
    for primitive in model['meshes'][0]['primitives']:
        fields = primitive['attributes']
        positions = values(fields['POSITION'])
        normals = values(fields['NORMAL'])
        weights = values(fields['WEIGHTS_0'])
        bones = values(fields['JOINTS_0'])
        indices = values(primitive['indices'])
        assert primitive.get('mode', 4) == 4
        assert len(positions) == len(normals) == len(weights) == len(bones)
        assert len(indices) % 3 == 0
        assert max(row[0] for row in indices) < len(positions)
        for position, normal, influence, ids in zip(positions, normals, weights, bones):
            assert all(math.isfinite(v) for v in position + normal + influence)
            assert all(0 <= v <= 1 for v in influence)
            weights_error = max(weights_error, abs(sum(influence) - 1))
            assert all(0 <= number < len(joints) for number in ids)
            used.update(number for number, weight in zip(ids, influence) if weight > 0.001)
            for axis in range(3):
                lower[axis] = min(lower[axis], position[axis])
                upper[axis] = max(upper[axis], position[axis])
        vertices += len(positions)
        triangles += len(indices) // 3
    assert weights_error < 0.00001
    assert {f'{part}_{side}' for side in ('l', 'r')
            for part in ('upperarm', 'lowerarm', 'hand', 'thigh', 'calf', 'foot')} <= {joints[i] for i in used}
    assert 1.0 < entry['height_m'] < 2.4
    assert -.08 < lower[1] < .08, (actor_id, lower)
    assert abs(upper[1] - lower[1] - entry['height_m']) < .15, (actor_id, lower, upper, entry['height_m'])
    animations = {clip['name']: clip for clip in model['animations']}
    assert len(animations) == len(model['animations'])
    assert set(entry['clips'].values()) <= set(animations)
    assert entry['animation_fps'] == 60
    durations = {}
    for logical, name in entry['clips'].items():
        clip = animations[name]
        ends = []
        for channel in clip['channels']:
            sampler = clip['samplers'][channel['sampler']]
            times = [row[0] for row in values(sampler['input'])]
            poses = values(sampler['output'])
            assert len(times) == len(poses) and len(times) > 1
            assert abs(times[0]) < .000001 and all(b > a for a, b in zip(times, times[1:]))
            assert all(abs(t * 60 - round(t * 60)) < .0001 for t in times)
            assert all(math.isfinite(v) for pose in poses for v in pose)
            ends.append(times[-1])
        assert ends and max(ends) - min(ends) < .000001
        durations[logical] = ends[0]
    return {'file': str(path.relative_to(ROOT)), 'sha256': hashlib.sha256(raw).hexdigest(),
            'bytes': len(raw), 'vertices_exported': vertices, 'triangles': triangles,
            'materials': len(model['materials']), 'bones': len(joints),
            'maximum_weight_sum_error': weights_error,
            'bounds_gltf_m': {'min': lower, 'max': upper}, 'duration_seconds': durations,
            'status': 'pass', 'visual_review_required': True}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--actor', help='Inspect one catalog entry only')
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    catalog = json.loads(CATALOG.read_text())
    assert catalog['schema_version'] == 1
    entries = catalog['entries']
    if args.actor:
        entries = {args.actor: entries[args.actor]}
    report = {'status': 'pass', 'scope': 'Exported geometry and animation data; visual review separate',
              'models': {name: inspect(name, entry) for name, entry in entries.items()}}
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(report, indent=2) + '\n')
    print(f'Validated {len(entries)} human GLBs: {args.output}')


if __name__ == '__main__':
    main()
