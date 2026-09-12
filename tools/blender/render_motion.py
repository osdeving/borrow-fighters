"""Render selected phases from an editable actor without exporting or running the game.

Usage: blender --background actor.blend --python tools/blender/render_motion.py --
       --actor cpp --output /tmp/cpp-motion --samples run:0,run:0.25,light-1:0.35
"""

import argparse
from pathlib import Path
import sys

import bpy
from mathutils import Vector

sys.path.insert(0, str(Path(__file__).resolve().parent))
from actor_motion import Performer


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--actor', default='cpp')
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--samples', default='idle:0,run:0,run:0.25,run:0.5,run:0.75,light-1:0.35')
    parser.add_argument('--angle', choices=('quarter', 'profile', 'front', 'hands'), default='quarter')
    args = parser.parse_args(sys.argv[sys.argv.index('--') + 1:])
    args.output.mkdir(parents=True, exist_ok=True)
    rig = next(obj for obj in bpy.data.objects if obj.type == 'ARMATURE')
    rig.animation_data_clear()
    performer = Performer(rig, args.actor)
    scene = bpy.context.scene
    for obj in list(bpy.data.objects):
        if obj.type in ('LIGHT', 'CAMERA'):
            bpy.data.objects.remove(obj, do_unlink=True)
    views = {'quarter': (2.8, -5.5, 2.3), 'profile': (5.5, 0, 1.8),
             'front': (0, -5.5, 1.6), 'hands': (2.8, -5.5, 2.6)}
    aim = Vector((0, -0.1, 1.32 if args.angle == 'hands' else 0.9))
    bpy.ops.object.camera_add(location=views[args.angle])
    camera = bpy.context.object
    camera.rotation_euler = (aim - camera.location).to_track_quat('-Z', 'Y').to_euler()
    camera.data.type = 'ORTHO'
    camera.data.ortho_scale = 0.82 if args.angle == 'hands' else 2.25
    scene.camera = camera
    for label, at, power, size in [('Key', (3, -4, 5), 650, 4),
                                    ('Fill', (-3, -1, 2), 180, 3),
                                    ('Rim', (1, 3, 3), 450, 3)]:
        bpy.ops.object.light_add(type='AREA', location=at)
        light = bpy.context.object
        light.name = label
        light.data.energy = power
        light.data.shape = 'DISK'
        light.data.size = size
        light.rotation_euler = (Vector((0, 0, 1)) - light.location).to_track_quat('-Z', 'Y').to_euler()
    scene.world.color = (0.12, 0.12, 0.12)
    scene.render.engine = 'CYCLES'
    scene.cycles.samples = 12
    scene.render.resolution_x = 640
    scene.render.resolution_y = 800
    scene.render.resolution_percentage = 100
    for item in args.samples.split(','):
        action, phase = item.split(':')
        performer.apply(action, float(phase))
        bpy.context.view_layer.update()
        scene.render.filepath = str(args.output / f'{action}-{phase}-{args.angle}.png')
        bpy.ops.render.render(write_still=True)


if __name__ == '__main__':
    main()
