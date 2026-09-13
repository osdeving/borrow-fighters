"""Patch anatomical wrist bands in an existing actor, preserving baked actions.

This changes only the named cuffs and, for security, moves the left cargo flap
2 mm outward. It writes a separate candidate source/export and review report;
the reviewed candidates can then replace the published source and runtime GLB.
"""

import argparse
import hashlib
import json
import math
from pathlib import Path
import struct
import sys

import bpy
from mathutils import Matrix
from mathutils.bvhtree import BVHTree

sys.path.insert(0, str(Path(__file__).resolve().parent))
from humans import add_wrist_cuffs, setup_stage, camera_at


def geometry_digest(obj):
    digest = hashlib.sha256()
    for vertex in obj.data.vertices:
        digest.update(struct.pack('<3f', *vertex.co))
        for group in vertex.groups:
            digest.update(struct.pack('<If', group.group, group.weight))
    for face in obj.data.polygons:
        digest.update(str((tuple(face.vertices), face.material_index)).encode())
    return digest.hexdigest()


def actions_digest():
    digest = hashlib.sha256()
    for action in sorted(bpy.data.actions, key=lambda item: item.name):
        digest.update(action.name.encode())
        for curve in action.fcurves:
            digest.update(str((curve.data_path, curve.array_index)).encode())
            for point in curve.keyframe_points:
                digest.update(str((tuple(point.co), tuple(point.handle_left),
                                   tuple(point.handle_right), point.interpolation)).encode())
    return digest.hexdigest()


def measure(body, rig):
    samples = []
    for name, phase in (('guard', .2), ('run', 0), ('run', .25), ('run', .5), ('run', .75)):
        action = bpy.data.actions[name]
        rig.animation_data.action = action
        frame = float(action.frame_range[1]) * phase
        bpy.context.scene.frame_set(math.floor(frame), subframe=frame % 1)
        bpy.context.view_layer.update()
        graph = bpy.context.evaluated_depsgraph_get()
        tree = BVHTree.FromObject(body, graph)
        row = {'action': name, 'phase': phase, 'bands': {}}
        for obj in bpy.context.scene.objects:
            if not obj.name.startswith(('Glove cuff ', 'Brass wrist edging ')):
                continue
            distances = []
            evaluated = obj.evaluated_get(graph)
            for vertex in evaluated.data.vertices:
                nearest, normal, _, _ = tree.find_nearest(vertex.co)
                distances.append((vertex.co - nearest).dot(normal))
            row['bands'][obj.name] = {'minimum_skin_distance_m': min(distances),
                                     'maximum_skin_distance_m': max(distances),
                                     'fraction_inside_skin': sum(v < -.001 for v in distances) / len(distances)}
            assert min(distances) > -.001, (name, phase, obj.name, min(distances))
        pocket = bpy.data.objects.get('Security cargo pocket l')
        if pocket is not None:
            pants = next(obj for obj in bpy.context.scene.objects
                         if obj.type == 'MESH' and 'cortu_cargo_pants' in obj.name)
            pants_tree = BVHTree.FromObject(pants, graph)
            distances = []
            for vertex in pocket.evaluated_get(graph).data.vertices:
                nearest, normal, _, _ = pants_tree.find_nearest(vertex.co)
                distances.append((vertex.co - nearest).dot(normal))
            row['left_cargo_flap_minimum_clearance_m'] = min(distances)
            assert min(distances) > -.001, (name, phase, min(distances))
        samples.append(row)
    return samples


def export_existing(rig, actor_id, output):
    """Export the existing actions without regenerating any animation curve."""
    rig.animation_data.action = None
    for bone in rig.pose.bones:
        bone.matrix_basis = Matrix.Identity(4)
    bpy.context.view_layer.update()
    meshes = [obj for obj in bpy.context.scene.objects if obj.type == 'MESH']
    for obj in meshes:
        bpy.ops.object.select_all(action='DESELECT')
        obj.select_set(True)
        bpy.context.view_layer.objects.active = obj
        for modifier in list(obj.modifiers):
            if modifier.type != 'ARMATURE':
                bpy.ops.object.modifier_apply(modifier=modifier.name)
    bpy.ops.object.select_all(action='DESELECT')
    for obj in meshes:
        obj.select_set(True)
    bpy.context.view_layer.objects.active = meshes[0]
    bpy.ops.object.join()
    bpy.context.object.name = actor_id + '_render_surface'
    rig.select_set(True)
    bpy.context.view_layer.objects.active = rig
    bpy.ops.export_scene.gltf(filepath=str(output), export_format='GLB', use_selection=True,
                             export_yup=True, export_apply=True, export_animations=True,
                             export_animation_mode='ACTIONS', export_morph=False)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--actor', choices=('cpp', 'security'), required=True)
    parser.add_argument('--source', type=Path, required=True)
    parser.add_argument('--out', type=Path, required=True)
    parser.add_argument('--no-render', action='store_true')
    args = parser.parse_args(sys.argv[sys.argv.index('--') + 1:])
    args.out.mkdir(parents=True, exist_ok=True)
    bpy.ops.wm.open_mainfile(filepath=str(args.source.resolve()))
    rig = next(obj for obj in bpy.context.scene.objects if obj.type == 'ARMATURE')
    body = bpy.data.objects['C++ anatomical skin' if args.actor == 'cpp' else 'security anatomical skin']
    before_actions = actions_digest()
    excluded = ('Glove cuff ', 'Brass wrist edging ', 'Security cargo pocket l')
    before_geometry = {obj.name: geometry_digest(obj) for obj in bpy.context.scene.objects
                       if obj.type == 'MESH' and not obj.name.startswith(excluded)}
    rig.animation_data.action = None
    for bone in rig.pose.bones:
        bone.matrix_basis = Matrix.Identity(4)
    bpy.context.view_layer.update()
    leather = bpy.data.objects['Glove cuff l0'].data.materials[0]
    metal = bpy.data.objects['Brass wrist edging l'].data.materials[0]
    for obj in list(bpy.context.scene.objects):
        if obj.name.startswith(('Glove cuff ', 'Brass wrist edging ')):
            bpy.data.objects.remove(obj, do_unlink=True)
    add_wrist_cuffs(body, rig, leather, metal)
    if args.actor == 'security':
        pocket = bpy.data.objects['Security cargo pocket l']
        for vertex in pocket.data.vertices:
            vertex.co.x += .002
    assert before_actions == actions_digest()
    assert before_geometry == {name: geometry_digest(bpy.data.objects[name]) for name in before_geometry}
    samples = measure(body, rig)
    rig.animation_data.action = bpy.data.actions['idle']
    bpy.context.scene.frame_set(0)
    source_output = args.out / args.source.name
    bpy.context.preferences.filepaths.save_version = 0
    bpy.ops.file.pack_all()
    bpy.ops.wm.save_as_mainfile(filepath=str(source_output.resolve()), compress=True)
    model_output = args.out / (args.actor + '.glb')
    export_existing(rig, args.actor, model_output)
    assert before_actions == actions_digest()
    report = {'actor': args.actor, 'source_input': str(args.source),
              'source_input_sha256': hashlib.sha256(args.source.read_bytes()).hexdigest(),
              'source_output_sha256': hashlib.sha256(source_output.read_bytes()).hexdigest(),
              'model_output_sha256': hashlib.sha256(model_output.read_bytes()).hexdigest(),
              'actions_unchanged_sha256': before_actions,
              'other_geometry_unchanged': before_geometry, 'samples': samples}
    (args.out / 'wrist-fit.json').write_text(json.dumps(report, indent=2) + '\n')
    if not args.no_render:
        camera = setup_stage()
        scene = bpy.context.scene
        scene.render.resolution_x = scene.render.resolution_y = 900
        scene.cycles.samples = 16
        camera.data.ortho_scale = 1.28
        for name, phase, label, angle in (('guard', .2, 'guard-quarter', .6),
                                         ('guard', .2, 'guard-profile', math.pi / 2),
                                         ('run', 0, 'run-0-profile', math.pi / 2),
                                         ('run', .5, 'run-half-profile', math.pi / 2)):
            action = bpy.data.actions[name]
            rig.animation_data.action = action
            frame = float(action.frame_range[1]) * phase
            scene.frame_set(math.floor(frame), subframe=frame % 1)
            camera_at(camera, angle, height=1.38, target=1.28)
            scene.render.filepath = str(args.out / (label + '.png'))
            bpy.ops.render.render(write_still=True)
    print('WRIST_FIT_COMPLETE', args.actor, str(args.out), flush=True)


if __name__ == '__main__':
    main()
