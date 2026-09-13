"""Build Julia, the broker and Augusta's dressed adult supporting cast.

Human anatomy and garment topology come from documented CC0 sources. Costume
adaptations preserve the chapter's existing palettes, silhouettes and props;
all motion is baked on the same real skeleton used by the C++ pilot.
"""

import argparse
import json
import math
from pathlib import Path
import sys

import bpy
import bmesh
from mathutils import Vector

sys.path.insert(0,str(Path(__file__).resolve().parent))
from human_base import create_actor,fit_asset,finalize_fit,add_surface_subdivision,save_export_actor,OUT
from humans import active,material,image_factor,tube,ellipsoid,bind,bone_coordinates,segment_coordinate,setup_stage,camera_at


def components(bm):
    visited=set();result=[]
    for start in bm.verts:
        if start in visited:continue
        pending=[start];part=[];visited.add(start)
        while pending:
            v=pending.pop();part.append(v)
            for edge in v.link_edges:
                other=edge.other_vert(v)
                if other not in visited:visited.add(other);pending.append(other)
        result.append(part)
    return result


def outfit_material(name,color,source,roughness=.73):
    mat=material(name,color,roughness)
    ao=next(source.glob("*_ao.png"),None)
    normal=next(source.glob("*_normal.png"),None)
    if ao:image_factor(mat,ao,color,normal)
    return mat


def skin_visibility(body,rig,*,upper_arms=False,ankles=False,dress_hem=None,neck_opening=.075):
    """Retain only exposed skin after shortening sleeves or a garment hem."""
    allowed={"head","neck_01"}
    prefixes=["lowerarm_","hand_","thumb_","index_","middle_","ring_","pinky_"]
    if upper_arms:prefixes.append("upperarm_")
    allowed.update(b.name for b in rig.data.bones if any(b.name.startswith(p) for p in prefixes))
    indices={g.index for g in body.vertex_groups if g.name in allowed}
    neck=rig.data.bones["neck_01"].head_local.z
    keep=set()
    for v in body.data.vertices:
        visible=sum(g.weight for g in v.groups if g.group in indices)>.33
        visible|=v.co.z>neck-neck_opening and abs(v.co.x)<.11
        visible|=ankles and .055<v.co.z<.20
        visible|=dress_hem is not None and .05<v.co.z<dress_hem+.025
        if visible:keep.add(v.index)
    bm=bmesh.new();bm.from_mesh(body.data);bm.verts.ensure_lookup_table()
    bmesh.ops.delete(bm,geom=[v for v in bm.verts if v.index not in keep],context="VERTS")
    bm.to_mesh(body.data);bm.free()


def cut_shirt_jacket(suit,rig,jacket,pants,trim,*,open_front=True,hem=.12):
    """Fit rolled casual jacket panels and slim trousers from a real shirt suit."""
    suit.data.materials.clear();suit.data.materials.append(jacket);suit.data.materials.append(pants)
    waist=rig.data.bones["pelvis"].head_local.z+.105
    bm=bmesh.new();bm.from_mesh(suit.data)
    trouser=set()
    for part in components(bm):
        if min(v.co.z for v in part)<.35:trouser.update(part)
    remove=[]
    for face in bm.faces:
        center=face.calc_center_median();is_pants=face.verts[0] in trouser
        face.material_index=int(is_pants)
        if not is_pants and center.z<waist:remove.append(face);continue
        if is_pants and center.z<hem-.025:remove.append(face);continue
        if not is_pants and open_front and center.y<-.028 and abs(center.x)<.050 and center.z>waist:
            remove.append(face);continue
        if not is_pants:
            for side in ["l","r"]:
                a,b=bone_coordinates(rig,"lowerarm_"+side)
                t,r=segment_coordinate(center,a,b)
                if t>.17 and r<.085:remove.append(face);break
    bmesh.ops.delete(bm,geom=remove,context="FACES")
    for v in bm.verts:
        if v in trouser:
            if v.co.z<hem+.025:v.co.z=hem
            if v.co.z>waist-.25:v.co.z+=.04*min(1,max(0,(v.co.z-waist+.25)/.20))
        elif v.co.z<waist+.025:v.co.z=waist
    bm.normal_update()
    for v in bm.verts:
        if v not in trouser:v.co+=v.normal*.005
    bm.to_mesh(suit.data);bm.free()
    for side in ["l","r"]:
        elbow,wrist=bone_coordinates(rig,"lowerarm_"+side)
        axis=(wrist-elbow).normalized();normal=axis.cross(Vector((0,1,0))).normalized();cross=axis.cross(normal).normalized()
        for k in range(3):
            center=elbow+(wrist-elbow)*(.07+k*.036)
            points=[center+.042*(normal*math.cos(a*math.tau/32)+cross*math.sin(a*math.tau/32)) for a in range(33)]
            tube("Rolled casual cuff "+side+str(k),points,.008,trim,rig,"lowerarm_"+side)


def curly_bob(body,rig,mat):
    top=max(v.co.z for v in body.data.vertices)
    skull=[v.co for v in body.data.vertices if v.co.z>top-.105]
    center=Vector((0,(min(v.y for v in skull)+max(v.y for v in skull))*.5,top-.106))
    verts=[];faces=[];rows=22;columns=64
    for i in range(rows):
        for j in range(columns):
            a=j*math.tau/columns;front=max(0,-math.sin(a))
            polar=.025+(2.25-1.05*front**1.5)*i/(rows-1)
            wav=.007*math.sin(a*14+polar*7)
            verts.append(center+Vector(((.107+wav)*math.sin(polar)*math.cos(a),(.111+wav)*math.sin(polar)*math.sin(a),.133*math.cos(polar))))
            if i:
                p=(i-1)*columns+j;q=(i-1)*columns+(j+1)%columns
                faces.append((p,q,q+columns,p+columns))
    from humans import mesh_object
    cap=mesh_object("Julia softly waved bob crown",verts,faces,mat);bind(cap,rig,"head")
    for strand in range(46):
        a=-.50+(math.pi+1)*strand/45
        points=[];radii=[]
        for i in range(27):
            t=i/26
            z=top+.012-t*(.263+.018*math.sin(strand*1.8))
            crown=math.sqrt(max(.01,1-((z-center.z)/.136)**2)) if z>center.z else 1
            radius=.109*crown+.012*max(0,(t-.55)/.45)
            x=math.cos(a)*radius+.007*math.sin(t*math.pi*5+strand*.42)*math.sin(t*math.pi)
            y=center.y+math.sin(a)*(radius+.004)+.006*math.cos(t*math.pi*5+strand*.42)*math.sin(t*math.pi)
            points.append((x,y,z));radii.append(.0055*(.92-.72*t**5)*max(.15,min(1,t/.12)))
        tube("Julia loose bob curl %02d"%strand,points,radii,mat,rig,"head",sides=8)
    for side in [-1,1]:
        for k in range(8):
            points=[]
            for i in range(26):
                t=i/25
                p0=Vector((-.024+.003*k,center.y+.006+.002*k,top+.017))
                p1=Vector((side*.05,center.y-.081,top+.019))
                p2=Vector((side*(.110+.003*k),center.y-.104,top-.075))
                p3=Vector((side*(.103+.003*k),center.y-.040,top-.238))
                p=p0*(1-t)**3+p1*3*t*(1-t)**2+p2*3*t*t*(1-t)+p3*t**3
                p+=Vector((.003*math.sin(t*math.pi*7+k),.004*math.cos(t*math.pi*7+k),0))*math.sin(t*math.pi)
                points.append(p)
            tube("Julia front swept curl",points,[.0057*(1-(i/27)**5) for i in range(26)],mat,rig,"head",sides=8)


def shoulder_bag(rig,suit):
    canvas=material("Julia warm cream canvas",(.46,.36,.245),.83)
    seam=material("Julia bag stitching",(.22,.13,.058),.77)
    pelvis=rig.data.bones["pelvis"].head_local.z
    center=Vector((-.215,.025,pelvis+.010))
    bpy.ops.mesh.primitive_cube_add(size=1,location=center)
    bag=bpy.context.object;bag.name="Julia cream messenger bag";bag.scale=(.235,.080,.185)
    active(bag);bpy.ops.object.transform_apply(location=True,rotation=True,scale=True)
    bevel=bag.modifiers.new("Soft canvas corners","BEVEL");bevel.width=.024;bevel.segments=4
    bag.data.materials.append(canvas);bind(bag,rig,"pelvis")
    for direction in [-1,1]:
        pts=[center+Vector((x,direction*.042,z)) for x,z in [(-.085,.065),(.085,.065),(.098,-.034),(0,-.064),(-.098,-.034),(-.085,.065)]]
        tube("Julia satchel flap stitching",pts,.0014,seam,rig,"pelvis")
    shoulder=rig.data.bones["clavicle_r"].tail_local
    points=[center+Vector((-.066,0,.07)),Vector((-.177,-.133,pelvis+.25)),Vector((-.13,-.061,shoulder.z+.031)),Vector((-.13,.062,shoulder.z+.027)),Vector((-.17,.055,pelvis+.25)),center+Vector((.066,.023,.072))]
    from mathutils.bvhtree import BVHTree
    bpy.context.view_layer.update()
    tree=BVHTree.FromObject(suit,bpy.context.evaluated_depsgraph_get())
    for i,p in enumerate(points[1:-1],start=1):
        front=i<3
        hit=tree.ray_cast(Vector((p.x,-1 if front else 1,p.z)),Vector((0,1 if front else -1,0)))
        if hit[0] is not None:p.y=hit[0].y+(-.004 if front else .004)
    from humans import mesh_object
    vs=[];fs=[]
    for i,p in enumerate(points):
        vs.extend([p+Vector((-.013,0,0)),p+Vector((.013,0,0))])
        if i:fs.append(((i-1)*2,(i-1)*2+1,i*2+1,i*2))
    strap=mesh_object("Julia continuous bag strap",vs,fs,canvas);strap.parent=rig
    low=strap.vertex_groups.new(name="pelvis");high=strap.vertex_groups.new(name="spine_03")
    for v in strap.data.vertices:
        w=max(0,min(1,(v.co.z-pelvis-.08)/(shoulder.z-pelvis-.08)))
        low.add([v.index],1-w,"REPLACE");high.add([v.index],w,"REPLACE")
    sub=strap.modifiers.new("Supple strap","SUBSURF");sub.levels=2
    solid=strap.modifiers.new("Canvas thickness","SOLIDIFY");solid.thickness=.003
    arm=strap.modifiers.new("Skeletal deformation","ARMATURE");arm.object=rig


def build_julia():
    rig,body,parts,HS=create_actor("julia",gender=.001,age=.5,weight=.43,muscle=.38,
        race=dict(caucasian=.58,african=.35,asian=.07),skin="young_caucasian_female",skin_factor=(.69,.46,.28),
        targets={"head/head-oval":.12,"nose/nose-width3-incr":.10,"mouth/mouth-upperlip-volume-incr":.16})
    suit,source=fit_asset(HS,body,"makehuman_system_assets","clothes/male_casualsuit03")
    inner,_=fit_asset(HS,body,"shirts01","clothes/toigo_camisole_top")
    shoes,_=fit_asset(HS,body,"makehuman_system_assets","clothes/shoes05")
    finalize_fit(rig,body,1.65,discard_clothes_masks=True)
    violet=outfit_material("Julia muted lavender jacket",(.15,.095,.215),source)
    pants=outfit_material("Julia charcoal jeans",(.018,.021,.027),source)
    trim=material("Julia rolled jacket lining",(.26,.21,.29),.83)
    cut_shirt_jacket(suit,rig,violet,pants,trim,hem=.125)
    inner.data.materials.clear();inner.data.materials.append(material("Julia black camisole",(.014,.012,.014),.82))
    bm=bmesh.new();bm.from_mesh(inner.data)
    bmesh.ops.delete(bm,geom=[f for f in bm.faces if abs(f.calc_center_median().x)>.078 or f.calc_center_median().y>-.022],context="FACES")
    bm.normal_update()
    for v in bm.verts:v.co-=v.normal*.014
    bm.to_mesh(inner.data);bm.free()
    bm=bmesh.new();bm.from_mesh(shoes.data)
    socks=[]
    for part in components(bm):
        if max(v.co.z for v in part)>.16:socks.extend(part)
    bmesh.ops.delete(bm,geom=socks,context="VERTS")
    bm.to_mesh(shoes.data);bm.free()
    skin_visibility(body,rig,ankles=True)
    hair=material("Julia dark brown curls",(.018,.006,.002),.77)
    curly_bob(body,rig,hair)
    shoulder_bag(rig,suit)
    add_surface_subdivision([body,suit,inner,shoes])
    return rig,body


def build_broker():
    rig,body,parts,HS=create_actor("broker",gender=.999,age=.62,weight=.48,muscle=.52,
        race=dict(caucasian=.82,african=.13,asian=.05),skin="middleage_caucasian_male",skin_factor=(.78,.62,.44),
        targets={"head/head-rectangular":.25,"chin/chin-width-incr":.18,"nose/nose-greek-incr":.20,"eyebrows/eyebrows-angle-down":.15})
    suit,source=fit_asset(HS,body,"makehuman_system_assets","clothes/male_elegantsuit01")
    hair,_=fit_asset(HS,body,"makehuman_system_assets","hair/short04",kind="Hair")
    shoes,_=fit_asset(HS,body,"makehuman_system_assets","clothes/shoes04")
    finalize_fit(rig,body,1.83)
    coal=outfit_material("Broker charcoal tailored suit",(.021,.022,.026),source,.72)
    red=material("Broker dark burgundy open shirt",(.13,.020,.023),.78)
    suit.data.materials.clear();suit.data.materials.append(coal);suit.data.materials.append(red)
    bm=bmesh.new();bm.from_mesh(suit.data)
    neck=rig.data.bones["neck_01"].head_local.z
    components_info=[]
    for part in components(bm):
        xs=[v.co.x for v in part];ys=[v.co.y for v in part];zs=[v.co.z for v in part]
        components_info.append({"vertices":len(part),"min":[min(xs),min(ys),min(zs)],"max":[max(xs),max(ys),max(zs)]})
    uv=bm.loops.layers.uv.active
    for face in bm.faces:
        u=sum(loop[uv].uv.x for loop in face.loops)/len(face.loops)
        v=sum(loop[uv].uv.y for loop in face.loops)/len(face.loops)
        if .348<u<.50 and v>.812 and abs(face.calc_center_median().x)<.073 and face.calc_center_median().y<-.015:face.material_index=1
    bm.to_mesh(suit.data);bm.free()
    (OUT/"broker-components.json").write_text(json.dumps(components_info,indent=2)+"\n")
    add_surface_subdivision([body,suit,shoes])
    return rig,body


def main():
    parser=argparse.ArgumentParser()
    parser.add_argument("--actor",choices=["julia","broker"],required=True)
    parser.add_argument("--review",type=Path,default=Path("/tmp/augusta-human-cast"))
    args=parser.parse_args(sys.argv[sys.argv.index("--")+1:])
    rig,body={"julia":build_julia,"broker":build_broker}[args.actor]()
    record=save_export_actor(rig,body,args.actor)
    camera=setup_stage();camera_at(camera,.60)
    rig.animation_data.action=bpy.data.actions[record["clips"]["idle"]]
    bpy.context.scene.frame_set(0)
    output=args.review/args.actor;output.mkdir(parents=True,exist_ok=True)
    for name,angle in [("quarter",.60),("profile",math.pi/2),("back",math.pi)]:
        camera_at(camera,angle)
        bpy.context.scene.render.filepath=str(output/(name+".png"))
        bpy.ops.render.render(write_still=True)
    print("CAST_ACTOR_COMPLETE",args.actor,str(output),flush=True)


if __name__=="__main__":main()
