"""Build Augusta's existing bald, muscular security guard as an editable human.

The navy polo, charcoal cargo trousers, boots, gloves and badge follow the
original security painting. The same model is reused by the three guards.
"""

import argparse
import hashlib
import json
import math
from pathlib import Path
import sys

import bpy
import bmesh
from mathutils import Matrix, Vector
from mathutils.bvhtree import BVHTree
from mathutils.kdtree import KDTree

sys.path.insert(0, str(Path(__file__).resolve().parent))
from human_base import create_actor, fit_asset, finalize_fit, add_surface_subdivision, save_export_actor, OUT, SOURCE
from humans import material, image_factor, mesh_object, bind, tube, add_gloves, boot_weights, setup_stage, camera_at


def patch_surface(obj, label, x_range, z_range, mat, rig, bone):
    """Lay a badge/pocket/belt on the fitted garment surface rather than floating."""
    bpy.context.view_layer.update()
    tree = BVHTree.FromObject(obj, bpy.context.evaluated_depsgraph_get())
    vertices, faces = [], []
    for iz in range(3):
        z = z_range[0] + (z_range[1] - z_range[0]) * iz / 2
        for ix in range(5):
            x = x_range[0] + (x_range[1] - x_range[0]) * ix / 4
            hit = tree.ray_cast(Vector((x, -1, z)), Vector((0, 1, 0)))
            if hit[0] is None:
                raise ValueError(f'{label} is outside garment at {(x, z)}')
            vertices.append(hit[0] + Vector((0, -.0035, 0)))
            if ix and iz:
                n = len(vertices) - 1
                faces.append((n - 6, n - 5, n, n - 1))
    patch = mesh_object(label, vertices, faces, mat)
    bind(patch, rig, bone)
    solid = patch.modifiers.new('Fabric edge', 'SOLIDIFY')
    solid.thickness = .002
    return patch


def build():
    rig, body, parts, hs = create_actor('security', gender=.999, age=.59, weight=.80, muscle=.98,
        race=dict(caucasian=.55, african=.38, asian=.07),
        skin='middleage_caucasian_male', skin_factor=(.48, .27, .13),
        targets={'head/head-square': .32, 'chin/chin-width-incr': .27,
                 'nose/nose-width3-incr': .16, 'eyebrows/eyebrows-angle-down': .20})
    shirt, shirt_source = fit_asset(hs, body, 'shirts01', 'clothes/namuhekam_male_polo_shirt')
    pants, pants_source = fit_asset(hs, body, 'pants01', 'clothes/cortu_cargo_pants')
    boots, boot_source = fit_asset(hs, body, 'makehuman_system_assets', 'clothes/shoes03')
    beard, beard_source = fit_asset(hs, body, 'bodyparts05', 'clothes/culturalibre_faun_beard')
    finalize_fit(rig, body, 1.88)
    broad = Matrix.Diagonal(Vector((1.10, 1, 1, 1)))
    rig.data.transform(broad)
    for obj in bpy.context.scene.objects:
        if obj.type == 'MESH':
            obj.data.transform(broad)
    navy = material('Security navy woven polo', (.024, .034, .057), .82)
    image_factor(navy, shirt_source / 'Polo_Base_Color.png', (.019, .028, .049),
                 shirt_source / 'Polo_Normal_OpenGL.png')
    shirt.data.materials.clear()
    shirt.data.materials.append(navy)
    charcoal = material('Security charcoal cargo cloth', (.045, .044, .042), .84)
    image_factor(charcoal, pants_source / 'cargo_pants_diff.png', (.10, .10, .095),
                 pants_source / 'cargo_pants_norm.png')
    pants.data.materials.clear()
    pants.data.materials.append(charcoal)
    leather = material('Security black leather', (.011, .012, .014), .55)
    steel = material('Security brushed buckle and badge', (.29, .31, .32), .48, .35)
    boots.data.materials.clear()
    boot_leather = material('Security boot leather with sewn panels', (.019, .020, .023), .61)
    image_factor(boot_leather, boot_source / 'shoes03_diffuse.png', (.55, .55, .55))
    boots.data.materials.append(boot_leather)
    floor = min(v.co.z for v in boots.data.vertices)
    for vertex in boots.data.vertices:
        vertex.co.z -= floor
    bm = bmesh.new()
    bm.from_mesh(pants.data)
    bmesh.ops.delete(bm, geom=[f for f in bm.faces if f.calc_center_median().z < .222], context='FACES')
    for vertex in bm.verts:
        if vertex.co.z < .24:
            vertex.co.z = .24
    bm.to_mesh(pants.data)
    bm.free()
    waist = max(v.co.z for v in pants.data.vertices) - .035
    # Tuck the polo into the waistband instead of layering its hanging hem over it.
    for vertex in shirt.data.vertices:
        if vertex.co.z < waist + .025:
            vertex.co.z = waist + .025 + (vertex.co.z - waist - .025) * .18
    add_gloves(body, rig, leather, leather)
    # Garment masks leave thigh fragments exposed after a deep fighting bend.
    # Only the anatomical skin actually exposed by this outfit remains visible.
    allowed = {g.index for g in body.vertex_groups if g.name in ('head', 'neck_01')
               or g.name.startswith(('upperarm_', 'lowerarm_', 'hand_', 'thumb_',
                                     'index_', 'middle_', 'ring_', 'pinky_'))}
    keep = {v.index for v in body.data.vertices if sum(g.weight for g in v.groups if g.group in allowed) > .18}
    bm = bmesh.new()
    bm.from_mesh(body.data)
    bm.verts.ensure_lookup_table()
    bmesh.ops.delete(bm, geom=[v for v in bm.verts if v.index not in keep], context='VERTS')
    bm.to_mesh(body.data)
    bm.free()
    for obj in bpy.context.scene.objects:
        if obj.name.startswith('C++'):
            obj.name = obj.name.replace('C++', 'Security')
    chest = rig.data.bones['neck_01'].head_local.z - .16
    patch_surface(shirt, 'Security plain badge', (.065, .107), (chest, chest + .020), steel, rig, 'spine_03')
    patch_surface(pants, 'Security dark belt front', (-.11, .11), (waist -.018, waist + .014), leather, rig, 'pelvis')
    patch_surface(pants, 'Security rectangular belt buckle', (-.020, .020), (waist -.016, waist + .012), steel, rig, 'pelvis')
    # Shorten the fitted CC0 beard to the painted guard's close chin hair.
    lips = [p.center.z for p in body.data.polygons if body.data.materials[p.material_index].name.endswith('.lips')]
    lip_low, lip_high = min(lips), max(lips)
    for vertex in beard.data.vertices:
        if vertex.co.z < lip_low - .018:
            vertex.co.z = lip_low - .018 + (vertex.co.z - lip_low + .018) * .40
    beard_mat = material('Security short dark beard', (.025, .015, .009), .92)
    image_factor(beard_mat, beard_source / 'brown.png', (.32, .24, .18))
    beard.data.materials.clear()
    beard.data.materials.append(beard_mat)
    add_surface_subdivision([body, shirt, pants, boots])
    detail_clothes(pants, boots, rig, waist, charcoal, leather)
    print('SECURITY_LIPS', lip_low, lip_high, 'WAIST', waist, flush=True)
    return rig, body


def detail_clothes(pants, boots, rig, waist, cloth, leather):
    """Add cargo flaps and laces on the fitted surfaces, with the same limb weights."""
    bpy.context.view_layer.update()
    graph = bpy.context.evaluated_depsgraph_get()
    pants_tree = BVHTree.FromObject(pants, graph)
    boot_tree = BVHTree.FromObject(boots, graph)
    nearby = KDTree(len(pants.data.vertices))
    for vertex in pants.data.vertices:
        nearby.insert(vertex.co, vertex.index)
    nearby.balance()
    lace_mat = material('Security charcoal laces', (.075, .080, .087), .86)
    for side, sign in (('l', 1), ('r', -1)):
        vertices, faces = [], []
        for iz in range(3):
            z = waist - .31 + iz * .065
            for iy in range(3):
                y = -.012 + iy * .032
                hit = pants_tree.ray_cast(Vector((sign, y, z)), Vector((-sign, 0, 0)))
                if hit[0] is None:
                    raise ValueError('Cargo flap outside fitted trousers')
                # The left flap needs another 2 mm to remain outside the
                # subdivided trousers while the thigh bends in guard/run.
                vertices.append(hit[0] + Vector((sign * (.004 if side == 'l' else .002), 0, 0)))
                if iz and iy:
                    n = len(vertices) - 1
                    faces.append((n - 4, n - 3, n, n - 1))
        pocket = mesh_object('Security cargo pocket ' + side, vertices, faces, cloth)
        bind(pocket, rig, 'thigh_' + side)
        pocket.vertex_groups.clear()
        groups = {group.index: pocket.vertex_groups.new(name=group.name) for group in pants.vertex_groups}
        for vertex in pocket.data.vertices:
            nearest = nearby.find_n(vertex.co, 4)
            total = sum(1 / max(distance, .0001) for _, _, distance in nearest)
            weights = {}
            for _, index, distance in nearest:
                influence = 1 / max(distance, .0001) / total
                for group in pants.data.vertices[index].groups:
                    weights[group.group] = weights.get(group.group, 0) + group.weight * influence
            for index, weight in weights.items():
                if weight > .00001:
                    groups[index].add([vertex.index], weight, 'REPLACE')
        solid = pocket.modifiers.new('Cargo flap thickness', 'SOLIDIFY')
        solid.thickness = .002
        center_x = rig.data.bones['foot_' + side].head_local.x
        for row in range(7):
            points = []
            for dx, dz in ((-.019, 0), (.019, .014)):
                z = .082 + row * .021 + dz
                hit = boot_tree.ray_cast(Vector((center_x + dx, -1, z)), Vector((0, 1, 0)))
                if hit[0] is None:
                    raise ValueError('Lace outside fitted boot')
                points.append(hit[0] + Vector((0, -.003, 0)))
            lace = tube('Security boot lace ' + side + str(row), points, .0018, lace_mat, rig, 'foot_' + side)
            boot_weights(lace, rig)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--review', type=Path, default=Path('/tmp/security-3d-review'))
    args = parser.parse_args(sys.argv[sys.argv.index('--') + 1:] if '--' in sys.argv else [])
    rig, body = build()
    record = save_export_actor(rig, body, 'security')
    source_paths = [('shirts01', 'clothes/namuhekam_male_polo_shirt'),
                    ('pants01', 'clothes/cortu_cargo_pants'),
                    ('makehuman_system_assets', 'clothes/shoes03'),
                    ('bodyparts05', 'clothes/culturalibre_faun_beard')]
    provenance_path = OUT / 'security-provenance.json'
    previous = json.loads(provenance_path.read_text()) if provenance_path.exists() else {}
    package_records = {entry['name']: entry for entry in previous.get('packages', [])}
    for pack in sorted({pack for pack, _ in source_paths}):
        if pack not in package_records:
            package_records[pack] = json.loads((Path('/tmp/borrow-fighters-mh-assets') / f'{pack}.download.json').read_text())
    provenance = {'reference': '../../actors/security/sprites/idle.png',
                  'license': 'CC0-1.0 source garments; original adaptation in security.py',
                  'authors': ['Namuhekam (polo)', 'Cortu Johnstone (cargo trousers)', 'culturalibre (beard)', 'MakeHuman community (shoe/sock base, anatomy, skin)'],
                  'blender': bpy.app.version_string,
                  'generator_sha256': hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
                  'model_sha256': hashlib.sha256((OUT.parent.parent / 'models/security.glb').read_bytes()).hexdigest(),
                  'packages': [package_records[pack] for pack in sorted(package_records)],
                  'source_files': []}
    for pack, relative in source_paths:
        for path in sorted((SOURCE / pack / relative).rglob('*')):
            if path.is_file():
                provenance['source_files'].append({'file': str(path.relative_to(OUT)),
                                                   'sha256': hashlib.sha256(path.read_bytes()).hexdigest()})
    provenance_path.write_text(json.dumps(provenance, indent=2) + '\n')
    from actor_motion import Performer
    camera = setup_stage()
    args.review.mkdir(parents=True, exist_ok=True)
    for label, angle in [('quarter', .6), ('profile', math.pi / 2), ('back', math.pi)]:
        Performer(rig, 'security').apply('guard', .2)
        bpy.context.view_layer.update()
        camera_at(camera, angle)
        bpy.context.scene.render.filepath = str(args.review / f'{label}.png')
        bpy.ops.render.render(write_still=True)
    print('SECURITY_COMPLETE', json.dumps(record), flush=True)


if __name__ == '__main__':
    main()
