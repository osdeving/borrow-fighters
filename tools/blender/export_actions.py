"""Rebake one actor's motion from its .blend without rebuilding its appearance.

The input scene must contain one actor, its mesh pieces and one armature.
Use Blender's --background actor.blend before --python to load the source.
"""

import argparse
import json
from pathlib import Path
import sys

import bpy
from mathutils import Matrix

sys.path.insert(0, str(Path(__file__).resolve().parent))
from actor_motion import build_actions


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--actor', required=True)
    parser.add_argument('--model', type=Path, required=True)
    parser.add_argument('--metadata', type=Path, required=True)
    parser.add_argument('--save-source', action='store_true',
                        help='Save the new actions into the loaded editable .blend')
    args = parser.parse_args(sys.argv[sys.argv.index('--') + 1:])
    rigs = [obj for obj in bpy.context.scene.objects if obj.type == 'ARMATURE']
    if len(rigs) != 1 or not bpy.data.filepath:
        raise ValueError('Load an actor .blend containing exactly one armature')
    if args.model.suffix != '.glb':
        raise ValueError('--model must end in .glb')
    rig = rigs[0]
    metadata = build_actions(rig, args.actor)
    bpy.ops.file.pack_all()
    if args.save_source:
        bpy.ops.wm.save_as_mainfile(filepath=bpy.data.filepath, compress=True)
    rig.animation_data.action = None
    for bone in rig.pose.bones:
        bone.matrix_basis = Matrix.Identity(4)
    meshes = [obj for obj in bpy.context.scene.objects if obj.type == 'MESH']
    if not meshes:
        raise ValueError('Actor has no mesh')
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
    surface = bpy.context.object
    surface.name = args.actor + '_render_surface'
    rig.select_set(True)
    bpy.context.view_layer.objects.active = rig
    args.model.parent.mkdir(parents=True, exist_ok=True)
    temporary = args.model.with_suffix('.candidate.glb')
    bpy.ops.export_scene.gltf(filepath=str(temporary), export_format='GLB',
                             use_selection=True, export_yup=True, export_apply=True,
                             export_animations=True, export_animation_mode='ACTIONS',
                             export_morph=False)
    temporary.replace(args.model)
    args.metadata.parent.mkdir(parents=True, exist_ok=True)
    args.metadata.write_text(json.dumps(metadata, indent=2) + '\n')
    print(f'EXPORTED_ACTIONS {args.actor}: {len(metadata["clips"])} actions -> {args.model}')


if __name__ == '__main__':
    main()
