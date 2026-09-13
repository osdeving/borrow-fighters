"""Render baked human actions at their authored rate for anatomical review.

This reads the editable production blend and never rewrites a runtime GLB.
Frames preserve real clip duration at 24 fps, with grounded profile views for
walking/running and a face-visible three-quarter view for quiet acting.
"""

import argparse
import json
import math
from pathlib import Path
import sys

import bpy

sys.path.insert(0,str(Path(__file__).resolve().parent))
from humans import setup_stage, camera_at


def main():
    parser=argparse.ArgumentParser()
    parser.add_argument("--blend",type=Path,required=True)
    parser.add_argument("--out",type=Path,required=True)
    parser.add_argument("--actions",default="idle,walk,run")
    parser.add_argument("--width",type=int,default=480)
    parser.add_argument("--samples",type=int,default=12)
    args=parser.parse_args(sys.argv[sys.argv.index("--")+1:])
    bpy.ops.wm.open_mainfile(filepath=str(args.blend.resolve()))
    rig=next(obj for obj in bpy.context.scene.objects if obj.type=="ARMATURE")
    camera=setup_stage()
    scene=bpy.context.scene
    scene.render.resolution_x=args.width
    scene.render.resolution_y=int(args.width*1.25)
    scene.cycles.samples=args.samples
    scene.render.film_transparent=False
    args.out.mkdir(parents=True,exist_ok=True)
    manifest={"source_blend":str(args.blend),"fps":24,"actions":{}}
    for name in args.actions.split(","):
        action=bpy.data.actions.get(name)
        if action is None:raise ValueError("Missing baked action "+name)
        rig.animation_data.action=action
        duration=float(action.frame_range[1]-action.frame_range[0])/60
        count=max(2,math.ceil(duration*24))
        output=args.out/name
        output.mkdir(exist_ok=True)
        camera_at(camera,math.pi/2 if name in ["walk","run"] else .60)
        for index in range(count):
            tick=action.frame_range[0]+index*60/24
            scene.frame_set(math.floor(tick),subframe=tick%1)
            scene.render.filepath=str(output/("frame%04d.png"%index))
            bpy.ops.render.render(write_still=True)
        manifest["actions"][name]={"duration_seconds":duration,"frames":count,"first_frame":0,"view":"profile" if name in ["walk","run"] else "quarter"}
        (args.out/"review.json").write_text(json.dumps(manifest,indent=2)+"\n")
    print("HUMAN_REVIEW_COMPLETE",str(args.out),flush=True)


if __name__=="__main__":main()
