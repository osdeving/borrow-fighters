"""Refine only generated hair in an existing editable actor source.

The anatomical mesh, clothes, accessories and baked actions remain in place.
Use export_actions.py afterwards when the geometry review is complete.
"""
import argparse
import math
from pathlib import Path
import sys

import bpy
from mathutils import Matrix

sys.path.insert(0,str(Path(__file__).resolve().parent))
from humans import add_hair,setup_stage,camera_at
from human_cast import curly_bob


def main():
    p=argparse.ArgumentParser();p.add_argument('--actor',choices=['cpp','julia'],required=True);p.add_argument('--review',type=Path,required=True)
    args=p.parse_args(sys.argv[sys.argv.index('--')+1:])
    rig=next(o for o in bpy.data.objects if o.type=='ARMATURE')
    body=next(o for o in bpy.data.objects if o.type=='MESH' and o.name.endswith('anatomical skin'))
    old_action=rig.animation_data.action
    rig.animation_data.action=None
    for bone in rig.pose.bones:bone.matrix_basis=Matrix.Identity(4)
    bpy.context.view_layer.update()
    if args.actor=='cpp':
        prefixes=('C++ parted copper crown','C++ flowing waved hair mantle','Swept hair strand','Loose copper curl','Front copper sweep')
        mats=[bpy.data.materials['Copper hair '+str(i)] for i in range(5)]
    else:
        prefixes=('Julia softly waved bob crown','Julia loose bob curl','Julia front swept curl')
        mats=[bpy.data.materials['Julia dark brown curls']]
    for obj in list(bpy.data.objects):
        if obj.type=='MESH' and obj.name.startswith(prefixes):bpy.data.objects.remove(obj,do_unlink=True)
    if args.actor=='cpp':add_hair(body,rig,mats)
    else:curly_bob(body,rig,mats[0])
    rig.animation_data.action=old_action
    bpy.context.scene.frame_set(0)
    bpy.ops.wm.save_as_mainfile(filepath=bpy.data.filepath,compress=True)
    camera=setup_stage();args.review.mkdir(parents=True,exist_ok=True)
    for name,angle in [('quarter',.60),('profile',math.pi/2),('back',math.pi)]:
        camera_at(camera,angle);bpy.context.scene.render.filepath=str(args.review/(name+'.png'));bpy.ops.render.render(write_still=True)
    print('HAIR_REFINED',args.actor,flush=True)


if __name__=='__main__':main()
