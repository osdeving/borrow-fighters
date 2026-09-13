"""Build the eight painted Augusta wardrobe identities as dressed 3D adults.

Anatomy, sewn garments, hair and footwear are fitted from documented CC0
MakeHuman sources. The catalog preserves separate adult faces, proportions,
hairstyles and palettes; all actors share the production skeletal contract.
"""
import argparse
import math
from pathlib import Path
import sys

import bpy
import bmesh
from mathutils import Vector

sys.path.insert(0,str(Path(__file__).resolve().parent))
from human_base import create_actor,fit_asset,finalize_fit,add_surface_subdivision,save_export_actor
from human_cast import components,outfit_material,skin_visibility
from humans import material,image_factor,tube,bind,bone_coordinates,segment_coordinate,setup_stage,camera_at,mesh_object

SYSTEM="makehuman_system_assets"
# Ages are MPFB's adult macro coordinates, not years; all references are adults.
CAST={
 "leather":dict(gender=.999,age=.55,weight=.49,muscle=.64,height=1.78,race=(.06,.92,.02),skin="young_african_male",skin_factor=(.72,.51,.34),hair="short01",hair_color=(.025,.018,.013),outfit="jacket",color=(.012,.013,.016),inner=(.015,.014,.016),pants=(.011,.017,.029),shoes="shoes03",targets={"chin/chin-width-incr":.17,"nose/nose-width3-incr":.13}),
 "plum-dress":dict(gender=.001,age=.54,weight=.47,muscle=.36,height=1.67,race=(.30,.66,.04),skin="young_african_female",skin_factor=(.93,.66,.46),hair="short04",hair_color=(.025,.012,.008),outfit="dress",color=(.105,.014,.067),shoes="toigo_ankle_boots_female",targets={"head/head-oval":.14,"mouth/mouth-upperlip-volume-incr":.15}),
 "amber-jacket":dict(gender=.999,age=.5,weight=.36,muscle=.4,height=1.79,race=(.92,.04,.04),skin="young_caucasian_male",skin_factor=(.82,.65,.46),hair="short02",hair_color=(.32,.095,.025),outfit="jacket",color=(.26,.115,.017),inner=(.52,.42,.29),pants=(.011,.012,.015),shoes="shoes02",targets={"head/head-oval":.2,"nose/nose-greek-incr":.1}),
 "denim":dict(gender=.001,age=.5,weight=.38,muscle=.43,height=1.64,race=(.94,.03,.03),skin="young_caucasian_female",skin_factor=(.86,.64,.43),hair="short03",hair_color=(.34,.075,.013),outfit="jacket",color=(.045,.105,.19),inner=(.15,.025,.008),pants=(.018,.018,.020),shoes="shoes05",targets={"head/head-rectangular":.1,"nose/nose-width3-decr":.12}),
 "teal-dress":dict(gender=.001,age=.58,weight=.64,muscle=.44,height=1.71,race=(.03,.95,.02),skin="young_african_female",skin_factor=(.58,.38,.24),hair="afro01",hair_color=(.015,.011,.008),outfit="dress",color=(.009,.078,.076),shoes="toigo_ankle_boots_female",targets={"head/head-round":.18,"nose/nose-width3-incr":.18,"mouth/mouth-lowerlip-volume-incr":.12}),
 "white-shirt":dict(gender=.999,age=.54,weight=.39,muscle=.43,height=1.75,race=(.06,.03,.91),skin="young_asian_male",skin_factor=(.84,.69,.48),hair="short04",hair_color=(.018,.014,.011),outfit="shirt",color=(.60,.58,.54),pants=(.019,.021,.025),shoes="shoes01",targets={"head/head-oval":.12,"chin/chin-width-decr":.08}),
 "red-blouse":dict(gender=.001,age=.52,weight=.43,muscle=.36,height=1.68,race=(.61,.27,.12),skin="young_caucasian_female",skin_factor=(.69,.47,.29),hair="long01",hair_color=(.072,.022,.007),outfit="shirt",color=(.24,.012,.013),pants=(.019,.016,.017),shoes="toigo_ankle_boots_female",targets={"head/head-oval":.15,"nose/nose-greek-incr":.08}),
 "long-coat":dict(gender=.999,age=.82,weight=.47,muscle=.36,height=1.78,race=(.86,.1,.04),skin="old_caucasian_male",skin_factor=(.79,.64,.46),hair="short04",hair_color=(.35,.31,.27),outfit="coat",color=(.020,.021,.024),inner=(.10,.016,.020),pants=(.014,.014,.017),shoes="shoes01",targets={"head/head-rectangular":.16,"chin/chin-width-incr":.1,"nose/nose-greek-incr":.13}),
}


def recolor_hair(hair,color):
    """Tint intact hair maps, retaining alpha cards and their sculpted topology."""
    for mat in hair.data.materials:
        shader=next(n for n in mat.node_tree.nodes if n.type=="BSDF_PRINCIPLED")
        source=shader.inputs["Base Color"].links
        if source:
            output=source[0].from_socket
            mix=mat.node_tree.nodes.new("ShaderNodeMix");mix.data_type="RGBA";mix.blend_type="MULTIPLY"
            mix.inputs[0].default_value=1;mix.inputs[7].default_value=(*color,1)
            mat.node_tree.links.new(output,mix.inputs[6]);mat.node_tree.links.new(mix.outputs[2],shader.inputs["Base Color"])
        else:shader.inputs["Base Color"].default_value=(*color,1)
        shader.inputs["Roughness"].default_value=.77


def casual_outfit(suit,rig,source,spec,inner):
    upper=outfit_material("Wardrobe outer cloth",spec["color"],source,.52 if spec["outfit"]=="jacket" and spec["color"][0]<.02 else .77)
    pants=outfit_material("Wardrobe dark trousers",spec["pants"],source)
    suit.data.materials.clear();suit.data.materials.append(upper);suit.data.materials.append(pants)
    bm=bmesh.new();bm.from_mesh(suit.data)
    legs=set()
    for part in components(bm):
        if min(v.co.z for v in part)<.3:legs.update(part)
    waist=rig.data.bones["pelvis"].head_local.z+.11
    delete=[]
    for face in bm.faces:
        c=face.calc_center_median();is_legs=face.verts[0] in legs;face.material_index=int(is_legs)
        if is_legs:continue
        if spec["outfit"]=="jacket":
            if c.z<waist or(c.y<-.023 and abs(c.x)<.042 and c.z>waist):delete.append(face)
        else:
            for side in ["l","r"]:
                a,b=bone_coordinates(rig,"lowerarm_"+side);t,r=segment_coordinate(c,a,b)
                if t>.23 and r<.08:delete.append(face);break
    bmesh.ops.delete(bm,geom=delete,context="FACES");bm.normal_update()
    for v in bm.verts:
        if v in legs:
            if v.co.z>waist-.25:v.co.z+=.06*min(1,max(0,(v.co.z-waist+.25)/.20))
        else:
            if spec["outfit"]=="jacket" and v.co.z<waist+.02:v.co.z=waist
            v.co+=v.normal*.007
    bm.to_mesh(suit.data);bm.free()
    if inner:
        inner.data.materials.clear();inner.data.materials.append(material("Visible inner top",spec["inner"],.83))
        bm=bmesh.new();bm.from_mesh(inner.data)
        bmesh.ops.delete(bm,geom=[f for f in bm.faces if f.calc_center_median().z<waist-.025],context="FACES")
        bm.normal_update()
        for v in bm.verts:v.co-=v.normal*.014
        bm.to_mesh(inner.data);bm.free()
    else:
        for side in ["l","r"]:
            a,b=bone_coordinates(rig,"lowerarm_"+side);axis=(b-a).normalized();normal=axis.cross(Vector((0,1,0))).normalized();cross=axis.cross(normal).normalized()
            for k in range(2):
                c=a+(b-a)*(.13+k*.044)
                points=[c+.042*(normal*math.cos(j*math.tau/24)+cross*math.sin(j*math.tau/24)) for j in range(25)]
                tube("Rolled cloth cuff",points,.007,upper,rig,"lowerarm_"+side)


def dress_outfit(dress,sleeves,rig,source,spec):
    cloth=outfit_material("Night dress woven cloth",spec["color"],source,.83)
    dress.data.materials.clear();dress.data.materials.append(cloth)
    sleeves.data.materials.clear();sleeves.data.materials.append(cloth)
    knee=rig.data.bones["calf_l"].head_local.z;waist=rig.data.bones["pelvis"].head_local.z+.1
    old=min(v.co.z for v in dress.data.vertices);hem=knee+.055
    for v in dress.data.vertices:
        if v.co.z<waist:v.co.z=hem+(v.co.z-old)*(waist-hem)/(waist-old)
    # Use a complete sewn tee bodice rather than detached sleeve patches.
    # The shift dress supplies only the skirt, overlapping the fitted waist.
    bm=bmesh.new();bm.from_mesh(dress.data)
    bmesh.ops.delete(bm,geom=[f for f in bm.faces if f.calc_center_median().z>waist+.060],context="FACES")
    bm.to_mesh(dress.data);bm.free()
    bm=bmesh.new();bm.from_mesh(sleeves.data)
    bmesh.ops.delete(bm,geom=[f for f in bm.faces if f.calc_center_median().z<waist-.080],context="FACES")
    if spec["hair"]=="afro01":
        deform=bm.verts.layers.deform.active
        for side in ["l","r"]:
            a,b=bone_coordinates(rig,"upperarm_"+side);elbow,wrist=bone_coordinates(rig,"lowerarm_"+side)
            axis=b-a;chain=[]
            for v in bm.verts:
                t,r=segment_coordinate(v.co,a,b)
                if t>.38 and r<.085:chain.append((v,t))
            old_max=max(t for _,t in chain)
            upper=sleeves.vertex_groups["upperarm_"+side].index;lower=sleeves.vertex_groups["lowerarm_"+side].index
            for v,t in chain:
                q=(t-.38)/(old_max-.38)
                radial=v.co-(a+axis*t)
                v.co=(a+axis*.38).lerp(elbow+(wrist-elbow)*.32,q)+radial
                weight=max(0,min(1,(q-.52)/.37));weight=weight*weight*(3-2*weight)
                weights=v[deform];weights.clear();weights[upper]=1-weight;weights[lower]=weight
    bm.normal_update()
    for v in bm.verts:v.co+=v.normal*.004
    bm.to_mesh(sleeves.data);bm.free()
    return hem


def long_coat(suit,rig,source,spec):
    cloth=outfit_material("Charcoal long coat",spec["color"],source,.79)
    trousers=outfit_material("Dark trousers",spec["pants"],source,.81)
    shirt=material("Burgundy shirt under coat",spec["inner"],.8)
    suit.data.materials.clear()
    for mat in [cloth,trousers,shirt]:suit.data.materials.append(mat)
    bm=bmesh.new();bm.from_mesh(suit.data);parts=components(bm)
    coat=set(max(parts,key=lambda p:max(v.co.z for v in p)))
    low=min(v.co.z for v in coat);waist=rig.data.bones["pelvis"].head_local.z+.13
    uv=bm.loops.layers.uv.active
    for face in bm.faces:
        face.material_index=0 if face.verts[0] in coat else 1
        u=sum(l[uv].uv.x for l in face.loops)/len(face.loops);v=sum(l[uv].uv.y for l in face.loops)/len(face.loops)
        if .348<u<.50 and v>.812 and abs(face.calc_center_median().x)<.073 and face.calc_center_median().y<-.015:face.material_index=2
    for v in coat:
        if v.co.z<waist:v.co.z-=.19*min(1,(waist-v.co.z)/(waist-low))
    bm.to_mesh(suit.data);bm.free()



def high_braid(body,rig,hair):
    top=max(v.co.z for v in body.data.vertices)
    root_y=max(v.co.y for v in hair.data.vertices if abs(v.co.z-(top-.085))<.028)
    dark=material("Plum long black braid",(.012,.008,.006),.75)
    sheen=material("Plum braid strand highlight",(.030,.017,.010),.65)
    for strand in range(3):
        points=[]
        for i in range(65):
            t=i/64;angle=t*math.tau*7+strand*math.tau/3
            center=Vector((.026*math.sin(t*2.1),root_y+.004+.080*(1-math.exp(-t*6)),top-.085-t*.51))
            center+=Vector((.019*math.cos(angle),.012*math.sin(angle),0))
            points.append(center)
        tube("Plum interlaced braid",points,[.0105*(1-.52*(i/64)**4) for i in range(65)],dark,rig,"head",sides=7)
        tube("Plum fine braid strand",[p+Vector((0,.006,0)) for p in points],.0012,sheen,rig,"head",sides=4)


def square_neck(sleeves,rig):
    neck=rig.data.bones["neck_01"].head_local.z
    bm=bmesh.new();bm.from_mesh(sleeves.data)
    for origin,normal in [((-.072,0,0),(1,0,0)),((.072,0,0),(1,0,0)),((0,0,neck-.132),(0,0,1)),((0,.005,0),(0,1,0))]:
        bmesh.ops.bisect_plane(bm,geom=list(bm.verts)+list(bm.edges)+list(bm.faces),dist=.00001,plane_co=origin,plane_no=normal,clear_inner=False,clear_outer=False)
    remove=[f for f in bm.faces if abs(f.calc_center_median().x)<.072 and f.calc_center_median().y<.005 and f.calc_center_median().z>neck-.132]
    bmesh.ops.delete(bm,geom=remove,context="FACES")
    bm.to_mesh(sleeves.data);bm.free()


def join_waist(dress,bodice,rig):
    from mathutils.bvhtree import BVHTree
    bpy.context.view_layer.update();tree=BVHTree.FromObject(bodice,bpy.context.evaluated_depsgraph_get())
    waist=rig.data.bones["pelvis"].head_local.z+.1
    for v in dress.data.vertices:
        if v.co.z>waist-.11:
            origin=Vector((0,-.025,v.co.z));direction=v.co-origin;direction.z=0;direction.normalize()
            hit=tree.ray_cast(origin,direction)
            if hit[0] is not None:
                weight=min(1,max(0,(v.co.z-waist+.11)/.065))
                target=hit[0]-direction*.002
                v.co=v.co.lerp(target,weight)


def biker_details(suit,rig):
    from mathutils.bvhtree import BVHTree
    bpy.context.view_layer.update();tree=BVHTree.FromObject(suit,bpy.context.evaluated_depsgraph_get())
    neck=rig.data.bones["neck_01"].head_local.z;waist=rig.data.bones["pelvis"].head_local.z+.11
    leather=material("Biker folded leather lapels",(.015,.016,.018),.42)
    silver=material("Biker nickel zippers",(.25,.27,.29),.29,.80)
    def point(x,z):
        ray_x=math.copysign(max(abs(x),.087),x)
        hit=tree.ray_cast(Vector((ray_x,-1,z)),Vector((0,1,0)))
        return Vector((x,min(hit[0].y,-.065)-.010 if hit[0] is not None else -.135,z))
    for side in [-1,1]:
        coords=[(.035,neck+.005),(.15,neck-.12),(.072,neck-.19),(.034,neck-.32)]
        obj=mesh_object("Folded biker lapel",[point(side*x,z) for x,z in coords],[(0,1,2,3)],leather)
        bm=bmesh.new();bm.from_mesh(obj.data)
        bmesh.ops.triangulate(bm,faces=list(bm.faces))
        bmesh.ops.subdivide_edges(bm,edges=list(bm.edges),cuts=4,use_grid_fill=True)
        for v in bm.verts:v.co=point(v.co.x,v.co.z)
        bm.to_mesh(obj.data);bm.free();bind(obj,rig,"spine_03")
        solid=obj.modifiers.new("Folded leather thickness","SOLIDIFY");solid.thickness=.004
        for x in [.064]:
            tube("Open jacket zipper track",[point(side*x,waist+.018+i*(neck-.31-waist)/40) for i in range(41)],.0018,silver,rig,"spine_03",sides=5)
        tube("Diagonal biker pocket zipper",[point(side*(.084+.060*i/14),neck-.26+.018*i/14) for i in range(15)],.0019,silver,rig,"spine_03",sides=5)


def mask_long_sleeves(body,rig):
    """Hide skin covered by the coat while retaining wrists and full hands."""
    bm=bmesh.new();bm.from_mesh(body.data);hidden=[]
    for v in bm.verts:
        for side in ["l","r"]:
            a,b=bone_coordinates(rig,"lowerarm_"+side);t,r=segment_coordinate(v.co,a,b)
            if -.16<t<.88 and r<.095:hidden.append(v);break
    bmesh.ops.delete(bm,geom=hidden,context="VERTS");bm.to_mesh(body.data);bm.free()

def build(wardrobe):
    spec=CAST[wardrobe];actor="crowd-"+wardrobe
    macros={k:spec[k] for k in ["gender","age","weight","muscle","skin","skin_factor","targets"]}
    macros["race"]=dict(zip(["caucasian","african","asian"],spec["race"]))
    rig,body,parts,HS=create_actor(actor,**macros)
    hair,_=fit_asset(HS,body,SYSTEM,"hair/"+spec["hair"],kind="Hair")
    beard=None
    if wardrobe in ["leather","long-coat"]:beard,_=fit_asset(HS,body,"bodyparts05","clothes/culturalibre_faun_beard")
    inner=None;sleeves=None
    if spec["outfit"]=="dress":
        suit,source=fit_asset(HS,body,"dress01","clothes/toigo_shift_dress")
        sleeves,_=fit_asset(HS,body,"shirts01","clothes/toigo_basic_tucked_t-shirt")
    else:
        asset="male_elegantsuit01" if spec["outfit"]=="coat" else "male_casualsuit03"
        suit,source=fit_asset(HS,body,SYSTEM,"clothes/"+asset)
        if spec["outfit"]=="jacket":inner,_=fit_asset(HS,body,"shirts01","clothes/toigo_basic_tucked_t-shirt")
    shoe_pack="shoes01" if spec["shoes"].startswith("toigo") else SYSTEM
    shoes,_=fit_asset(HS,body,shoe_pack,"clothes/"+spec["shoes"])
    finalize_fit(rig,body,spec["height"],discard_clothes_masks=True)
    recolor_hair(hair,spec["hair_color"])
    if wardrobe=="teal-dress":
        top=max(v.co.z for v in body.data.vertices)
        center=Vector((0,(min(v.co.y for v in hair.data.vertices)+max(v.co.y for v in hair.data.vertices))*.5,top-.10))
        for v in hair.data.vertices:
            delta=v.co-center;w=max(0,min(1,(v.co.z-top+.115)/.10));w=w*w*(3-2*w)
            v.co=center+Vector((delta.x*(1+.45*w),delta.y*(1+.36*w),delta.z+max(0,delta.z)*.30))
    if beard:
        lip_low=min(p.center.z for p in body.data.polygons if body.data.materials[p.material_index].name.endswith(".lips"))
        for v in beard.data.vertices:
            if v.co.z<lip_low-.018:v.co.z=lip_low-.018+(v.co.z-lip_low+.018)*.35
        recolor_hair(beard,(.11,.09,.07))
    if wardrobe=="long-coat":
        for obj in [hair,beard]:
            for mat in obj.data.materials:
                shader=next(n for n in mat.node_tree.nodes if n.type=="BSDF_PRINCIPLED")
                for link in list(shader.inputs["Base Color"].links):mat.node_tree.links.remove(link)
                shader.inputs["Base Color"].default_value=(.18,.17,.15,1)

    if spec["shoes"].startswith("toigo"):
        shoes.data.materials.clear();shoes.data.materials.append(material("Dark leather ankle boots",(.018,.013,.011),.43))
        bm=bmesh.new();bm.from_mesh(shoes.data)
        bmesh.ops.holes_fill(bm,edges=[e for e in bm.edges if e.is_boundary],sides=0)
        bm.to_mesh(shoes.data);bm.free()
    if spec["outfit"]=="dress":
        hem=dress_outfit(suit,sleeves,rig,source,spec)
        if wardrobe=="plum-dress":square_neck(sleeves,rig);high_braid(body,rig,hair)
        join_waist(suit,sleeves,rig)
        skin_visibility(body,rig,upper_arms=wardrobe=="plum-dress",dress_hem=hem,neck_opening=.18 if wardrobe=="plum-dress" else .075)
        if wardrobe=="teal-dress":
            bm=bmesh.new();bm.from_mesh(body.data);hidden=[]
            for v in bm.verts:
                for side in ["l","r"]:
                    a,b=bone_coordinates(rig,"lowerarm_"+side);t,r=segment_coordinate(v.co,a,b)
                    if -.15<t<.23 and r<.095:hidden.append(v);break
            bmesh.ops.delete(bm,geom=hidden,context="VERTS");bm.to_mesh(body.data);bm.free()
        bm=bmesh.new();bm.from_mesh(body.data)
        bmesh.ops.delete(bm,geom=[v for v in bm.verts if v.co.z<.105],context="VERTS")
        bm.to_mesh(body.data);bm.free()
    elif spec["outfit"]=="coat":
        long_coat(suit,rig,source,spec);skin_visibility(body,rig);mask_long_sleeves(body,rig)
    else:
        casual_outfit(suit,rig,source,spec,inner);skin_visibility(body,rig)
        if wardrobe=="leather":biker_details(suit,rig)
    # Fingers, faces and garment edges remain anatomical; no extra hair subdivision.
    add_surface_subdivision([suit,shoes]+([inner] if inner else [])+([sleeves] if sleeves else []))
    return rig,body


def main():
    p=argparse.ArgumentParser();p.add_argument("--wardrobe",choices=list(CAST),required=True);p.add_argument("--review",type=Path,default=Path("/tmp/augusta-crowd-3d"));p.add_argument("--preview",action="store_true")
    args=p.parse_args(sys.argv[sys.argv.index("--")+1:]);rig,body=build(args.wardrobe)
    if args.preview:
        from actor_motion import Performer
        Performer(rig,"crowd-"+args.wardrobe).apply("idle",0)
    else:
        record=save_export_actor(rig,body,"crowd-"+args.wardrobe)
        rig.animation_data.action=bpy.data.actions[record["clips"]["idle"]];bpy.context.scene.frame_set(0)
    camera=setup_stage();output=args.review/args.wardrobe;output.mkdir(parents=True,exist_ok=True)
    for name,angle in [("quarter",.60),("profile",math.pi/2),("back",math.pi)]:
        camera_at(camera,angle);bpy.context.scene.render.filepath=str(output/(name+".png"));bpy.ops.render.render(write_still=True)
    print("CROWD_ACTOR_COMPLETE",args.wardrobe,str(output),flush=True)


if __name__=="__main__":main()
