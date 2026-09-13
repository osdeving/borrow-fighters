"""Build Augusta's anatomical, skinned humans with Blender and CC0 MPFB data.

The external MPFB checkout is a production tool, never a runtime dependency.
Source garments are fitted to the same anatomical surface before customizing
the C++ silhouette; exports retain real mesh depth, fingers and skeletal clips.
"""

from __future__ import annotations

import argparse
import json
import math
import os
from pathlib import Path
import random
import sys

import bpy
import bmesh
from mathutils import Matrix, Quaternion, Vector

ROOT = Path(__file__).resolve().parents[2]
SOURCE = ROOT / "assets/adventure/production-3d/humans/source"
OUT = ROOT / "assets/adventure/production-3d/humans"
MODELS = ROOT / "assets/adventure/models"


def initialize_mpfb(checkout: Path):
    import addon_utils

    extension_root = Path("/tmp/borrow-fighters-blender-extensions")
    extension_root.mkdir(exist_ok=True)
    link = extension_root / "mpfb"
    if not link.exists():
        link.symlink_to(checkout / "src/mpfb", target_is_directory=True)
    bpy.context.preferences.extensions.repos.new(
        name="Borrow Fighters production", module="borrow_fighters",
        custom_directory=str(extension_root),
    )
    addon_utils.enable("bl_ext.borrow_fighters.mpfb", default_set=True, persistent=False)
    from bl_ext.borrow_fighters.mpfb.services.humanservice import HumanService
    from bl_ext.borrow_fighters.mpfb.services.targetservice import TargetService
    return HumanService, TargetService


def active(obj):
    bpy.ops.object.select_all(action="DESELECT")
    obj.select_set(True)
    bpy.context.view_layer.objects.active = obj


def material(name, color, roughness=.5, metallic=0.0):
    mat = bpy.data.materials.new(name)
    mat.use_nodes = True
    mat.diffuse_color = (*color, 1)
    node = mat.node_tree.nodes.get("Principled BSDF")
    node.inputs["Base Color"].default_value = (*color, 1)
    node.inputs["Roughness"].default_value = roughness
    node.inputs["Metallic"].default_value = metallic
    return mat


def image_factor(mat, image_path, factor, normal_path=None):
    nodes,links=mat.node_tree.nodes,mat.node_tree.links
    shader=next(n for n in nodes if n.type=="BSDF_PRINCIPLED")
    image=nodes.new("ShaderNodeTexImage")
    image.image=bpy.data.images.load(str(image_path),check_existing=True)
    mix=nodes.new("ShaderNodeMix");mix.data_type="RGBA";mix.blend_type="MULTIPLY"
    mix.inputs[0].default_value=1
    mix.inputs[7].default_value=(*factor,1)
    links.new(image.outputs["Color"],mix.inputs[6]);links.new(mix.outputs[2],shader.inputs["Base Color"])
    if normal_path:
        tex=nodes.new("ShaderNodeTexImage");tex.image=bpy.data.images.load(str(normal_path),check_existing=True)
        tex.image.colorspace_settings.name="Non-Color"
        normal=nodes.new("ShaderNodeNormalMap");normal.inputs["Strength"].default_value=.65
        links.new(tex.outputs["Color"],normal.inputs["Color"]);links.new(normal.outputs["Normal"],shader.inputs["Normal"])


def mesh_object(name, vertices, faces, mat):
    mesh = bpy.data.meshes.new(name)
    mesh.from_pydata(vertices, [], faces)
    mesh.update()
    obj = bpy.data.objects.new(name, mesh)
    bpy.context.collection.objects.link(obj)
    obj.data.materials.append(mat)
    for face in obj.data.polygons:
        face.use_smooth = True
    return obj


def bind(obj, rig, bone):
    obj.parent = rig
    group = obj.vertex_groups.new(name=bone)
    group.add(list(range(len(obj.data.vertices))), 1, "REPLACE")
    mod = obj.modifiers.new("Skeletal deformation", "ARMATURE")
    mod.object = rig


def tube(name, points, radius, mat, rig=None, bone=None, sides=8):
    points = [Vector(p) for p in points]
    verts, faces = [], []
    radii = radius if isinstance(radius, list) else [radius] * len(points)
    for i, point in enumerate(points):
        tangent = (points[min(i + 1, len(points) - 1)] - points[max(i - 1, 0)]).normalized()
        axis = tangent.cross(Vector((0, 1, 0)))
        if axis.length < .1:
            axis = tangent.cross(Vector((1, 0, 0)))
        axis.normalize()
        other = tangent.cross(axis).normalized()
        for j in range(sides):
            a = j * 2 * math.pi / sides
            verts.append(point + radii[i] * (axis * math.cos(a) + other * math.sin(a)))
        if i:
            for j in range(sides):
                a, b = (i - 1) * sides + j, (i - 1) * sides + (j + 1) % sides
                faces.append((a, b, b + sides, a + sides))
    faces += [tuple(reversed(range(sides))), tuple(range((len(points)-1)*sides, len(points)*sides))]
    obj = mesh_object(name, verts, faces, mat)
    if rig:
        bind(obj, rig, bone)
    return obj


def freeze_shapes(obj):
    keys = obj.data.shape_keys
    if not keys:
        return
    basis = keys.key_blocks[0]
    coords = [v.co.copy() for v in basis.data]
    for key in list(keys.key_blocks)[1:]:
        if key.value:
            for i, v in enumerate(key.data):
                coords[i] += (v.co - key.relative_key.data[i].co) * key.value
    obj.shape_key_clear()
    for vertex, co in zip(obj.data.vertices, coords):
        vertex.co = co


def bone_coordinates(rig, name):
    b = rig.data.bones[name]
    return b.head_local.copy(), b.tail_local.copy()


def segment_coordinate(point, a, b):
    axis = b-a
    t = (point-a).dot(axis) / axis.length_squared
    radial = (point-(a+axis*t)).length
    return t, radial


def sleeve_and_cloth(suit, rig, white, black, trim):
    """Keep the original modeled collar/button topology and roll the sleeves."""
    suit.data.materials.clear()
    suit.data.materials.append(white)
    suit.data.materials.append(black)
    pelvis = rig.data.bones["pelvis"].head_local.z
    waist = pelvis + .12
    bm = bmesh.new()
    bm.from_mesh(suit.data)
    # The source contains disconnected shirt and trouser shells. Classifying
    # those shells preserves the overlapping waist instead of painting a
    # horizontal black stripe through the bottom of a long shirt.
    visited, trousers = set(), set()
    for start in bm.verts:
        if start in visited:
            continue
        stack, component = [start], []
        visited.add(start)
        while stack:
            v = stack.pop()
            component.append(v)
            for edge in v.link_edges:
                other = edge.other_vert(v)
                if other not in visited:
                    visited.add(other)
                    stack.append(other)
        if min(v.co.z for v in component) < .35:
            trousers.update(component)
    doomed = []
    for face in bm.faces:
        c = face.calc_center_median()
        is_pants = face.verts[0] in trousers
        face.material_index = int(is_pants)
        if (not is_pants and c.z < waist) or (is_pants and c.z < .205):
            doomed.append(face)
            continue
        for side in ["l", "r"]:
            elbow, wrist = bone_coordinates(rig, "lowerarm_" + side)
            t, radial = segment_coordinate(c, elbow, wrist)
            if .18 < t and radial < .085:
                doomed.append(face)
                break
    bmesh.ops.delete(bm, geom=doomed, context="FACES")
    for v in bm.verts:
        if v not in trousers and v.co.z < waist + .06:
            # Tuck the hem slightly inside the high waist; no loose shirt tail.
            v.co.x *= .93
            v.co.y *= .93
        if v in trousers and v.co.z > pelvis-.12:
            v.co.z += .085 * min(1, max(0, (v.co.z-(pelvis-.12))/.20))
        if v in trousers and v.co.z < pelvis-.06:
            side="l" if v.co.x>0 else "r"
            hip,knee=bone_coordinates(rig,"thigh_"+side)
            _,ankle=bone_coordinates(rig,"calf_"+side)
            a,b=(hip,knee) if v.co.z>knee.z else (knee,ankle)
            t=max(0,min(1,(a.z-v.co.z)/(a.z-b.z)))
            center=a.lerp(b,t)
            tight=.84 if v.co.z>knee.z else .77
            v.co.x=center.x+(v.co.x-center.x)*tight
            v.co.y=center.y+(v.co.y-center.y)*.90
        if v in trousers and v.co.z < .245:
            v.co.z=.235
    bm.to_mesh(suit.data)
    bm.free()
    # Four overlapping rings read as rolled fabric, with a seam and visible rim.
    for side in ["l", "r"]:
        elbow, wrist = bone_coordinates(rig, "lowerarm_" + side)
        axis = (wrist-elbow).normalized()
        normal = axis.cross(Vector((0, 1, 0))).normalized()
        across = axis.cross(normal).normalized()
        for k in range(3):
            center = elbow + (wrist-elbow)*(.095+k*.032)
            ring = [center + .049 * (normal*math.cos(a*2*math.pi/32)+across*math.sin(a*2*math.pi/32)) for a in range(33)]
            tube("Rolled cuff " + side + str(k), ring, .009, trim, rig, "lowerarm_"+side)


def retain_visible_skin(body, rig):
    """Remove covered skin, so no chest or toes can pierce fitted garments."""
    keep_bones = {"head", "neck_01"}
    keep_bones.update(n.name for n in rig.data.bones if any(n.name.startswith(p) for p in ["lowerarm_","hand_","thumb_","index_","middle_","ring_","pinky_"]))
    groups = {g.index for g in body.vertex_groups if g.name in keep_bones}
    neck=rig.data.bones["neck_01"].head_local.z
    keep = {v.index for v in body.data.vertices if sum(g.weight for g in v.groups if g.group in groups) > .35 or (v.co.z>neck-.075 and abs(v.co.x)<.10)}
    bm = bmesh.new();bm.from_mesh(body.data)
    bm.verts.ensure_lookup_table()
    bmesh.ops.delete(bm, geom=[v for v in bm.verts if v.index not in keep], context="VERTS")
    bm.to_mesh(body.data);bm.free()


def scale_character(rig, body, objects, target_height=1.74):
    top = max(v.co.z for v in body.data.vertices if abs(v.co.x) < .1)
    factor = target_height/top
    for obj in objects:
        for v in obj.data.vertices:
            v.co *= factor
    active(rig)
    bpy.ops.object.mode_set(mode="EDIT")
    for bone in rig.data.edit_bones:
        bone.head *= factor
        bone.tail *= factor
    bpy.ops.object.mode_set(mode="OBJECT")
    return factor


def add_hair(body, rig, mats):
    """Continuous scalp and broad overlapping wave locks, with real side volume."""
    random.seed(43)
    # Derive the head envelope from the fitted human, preserving a human skull.
    head_z = rig.data.bones["head"].head_local.z
    head_vertices = [v.co for v in body.data.vertices if v.co.z > head_z]
    top = max(v.z for v in head_vertices)
    center = Vector((0, .007, top-.107))
    rx, ry, rz = .112, .120, .135
    # Scalp ends higher at the forehead and follows the back of the skull.
    verts, faces = [], []
    rings, sides = 18, 64
    for i in range(rings):
        for j in range(sides):
            az = j*2*math.pi/sides
            # Front is -Y. Keep brows/eyes unobscured, lower crown at the nape.
            front = max(0, -math.sin(az))
            end = 2.25-.94*front**3
            polar = .025+(end-.025)*i/(rings-1)
            wav = .003*math.cos(az*12+polar*3)
            co = center + Vector(((rx+wav)*math.sin(polar)*math.cos(az), (ry+wav)*math.sin(polar)*math.sin(az), rz*math.cos(polar)))
            verts.append(co)
            if i:
                a=(i-1)*sides+j;b=(i-1)*sides+(j+1)%sides
                faces.append((a,b,b+sides,a+sides))
    scalp=mesh_object("C++ parted copper crown",verts,faces,mats[0]);bind(scalp,rig,"head")
    # A continuous waved mantle preserves the large uninterrupted hair mass
    # in the painting. Fine swept ridges sit on that volume, so orbiting the
    # camera never exposes disconnected feather-like strips or a bald cap.
    def mantle(theta,t):
        # The crown wraps the temples; long hair settles behind both shoulders.
        blend=max(0,min(1,(t-.20)/.24));blend=blend*blend*(3-2*blend)
        u=(theta+.76)/(math.pi+1.52)
        theta=theta*(1-blend)+(.47+(math.pi-.94)*u)*blend
        z=top+.024-t*(.50+.018*math.sin(theta*5))
        crown=math.sqrt(max(0,1-((z-(top-.104))/.139)**2)) if z>top-.104 else 1
        lower=max(0,min(1,(t-.20)/.42))
        envelope=max(0,math.sin(math.pi*min(t,1)))**.45
        wave=.017*math.sin(t*math.pi*4+math.cos(theta)*2.1)*envelope
        groove=.0045*math.cos(theta*25+t*5)*envelope
        radial_x=.120*crown+.072*lower+wave+groove
        radial_y=.123*crown+.055*lower+wave*.5+groove
        x=radial_x*math.cos(theta)
        y=.008+radial_y*math.sin(theta)
        return Vector((x,y,z))
    mv,mf=[],[]
    columns,rows=97,66
    for i in range(rows):
        t=.012+.988*i/(rows-1)
        for j in range(columns):
            theta=-.76+(math.pi+1.52)*j/(columns-1)
            mv.append(mantle(theta,t))
            if i and j:
                a=(i-1)*columns+j-1
                mf.append((a,a+1,a+1+columns,a+columns))
    mantle_obj=mesh_object("C++ flowing waved hair mantle",mv,mf,mats[1])
    mantle_obj.data.materials.append(mats[2]);mantle_obj.data.materials.append(mats[3])
    for face in mantle_obj.data.polygons:
        column=face.index%(columns-1)
        face.material_index=1 if column%17 in [5,6,7] else 0
    bind(mantle_obj,rig,"head")
    solid=mantle_obj.modifiers.new("Hair volume edge","SOLIDIFY");solid.thickness=.006
    for strand in range(31):
        theta=-.70+(math.pi+1.40)*strand/30
        ridge=[]
        for i in range(64):
            t=.025+.966*i/63
            co=mantle(theta+.009*math.sin(t*11),t)
            co+=Vector((math.cos(theta),math.sin(theta),0))*.003
            ridge.append(co)
        tube("Swept hair strand %02d"%strand,ridge,[.00115*(1-i/75) for i in range(64)],mats[4 if strand%5==0 else 2],rig,"head",sides=5)
    for strand in range(28):
        theta=-.79+(math.pi+1.58)*strand/27
        points=[];radii=[]
        length=1.00+.06*math.sin(strand*1.7)
        for i in range(52):
            t=.012+length*i/51
            p=mantle(theta,t)
            outward=Vector((math.cos(theta),math.sin(theta),0))
            q=i/51
            p+=outward*(.006+.009*math.sin(q*math.pi*5+strand*.47))*min(1,q/.12)
            p.x+=.017*math.sin(q*math.pi*5+strand*.41)*math.sin(q*math.pi)
            points.append(p)
            radii.append(.0062*(.75+.25*math.sin(q*math.pi))*max(.05,1-q**4)*max(.12,min(1,q/.12)))
        tube("Loose copper curl %02d"%strand,points,radii,mats[1+strand%3],rig,"head",sides=8)
    # Forehead sweep and loose front framing waves: their roots cross the
    # parting instead of leaving a smooth uncovered cap above the eyebrows.
    for side in [-1,1]:
        for k in range(8):
            pts=[]
            for i in range(35):
                t=i/34
                if t < .34:
                    q=t/.34
                    p0=Vector((-.03+side*.002*k,-.025+.004*k,top+.022+.001*k))
                    p1=Vector((side*.045,-.11,top+.045))
                    p2=Vector((side*(.122+.005*k),-.135,top-.055))
                    p3=Vector((side*(.116+.005*k),-.102+.004*k,top-.16))
                    p=p0*(1-q)**3+p1*3*q*(1-q)**2+p2*3*q*q*(1-q)+p3*q**3
                    x,y,z=p
                else:
                    q=(t-.34)/.66
                    x=side*(.116+.005*k+.015*math.sin(q*math.pi*3)*(1+q))
                    y=-.102+.003*k-.030*math.sin(q*math.pi/2)+.008*math.sin(q*math.pi*3)
                    z=top-.16-q*(.105+.006*k)
                pts.append(Vector((x,y,z)))
            # Broad sculpted front locks hide seams between cap and long hair.
            radii=[.009*(.8+.4*math.sin(math.pi*i/34))*(1-(i/35)**6) for i in range(35)]
            tube("Front copper sweep %s %s"%(side,k),pts,radii,mats[1+k%3],rig,"head",sides=10)


def ellipsoid(name,center,scale,mat,rig,bone,rotation=None):
    bpy.ops.mesh.primitive_uv_sphere_add(segments=24,ring_count=12,location=center)
    obj=bpy.context.object;obj.name=name;obj.scale=scale
    if rotation is not None:obj.rotation_euler=rotation
    active(obj);bpy.ops.object.transform_apply(location=True,rotation=True,scale=True)
    obj.data.materials.append(mat)
    for face in obj.data.polygons:face.use_smooth=True
    bind(obj,rig,bone)
    return obj


def boot_weights(obj,rig,side=None):
    """One continuous shin/ankle deformation for leather, laces and eyelets."""
    for group in list(obj.vertex_groups):obj.vertex_groups.remove(group)
    groups={name:obj.vertex_groups.new(name=name) for name in ["calf_l","foot_l","calf_r","foot_r"]}
    for v in obj.data.vertices:
        s=side or ("l" if v.co.x>0 else "r")
        ankle=rig.data.bones["foot_"+s].head_local.z
        amount=max(0,min(1,(v.co.z-ankle+.006)/.062))
        amount=amount*amount*(3-2*amount)
        groups["calf_"+s].add([v.index],amount,"REPLACE")
        groups["foot_"+s].add([v.index],1-amount,"REPLACE")


def add_wrist_cuffs(body, rig, leather, gold):
    """Fit wrist bands to each anatomical cross-section in the rest pose.

    The forearm controls these bands; hand articulation belongs to the glove.
    A fixed circular radius fails on broader or elliptical wrists, so each
    sample follows the evaluated skin before adding the leather thickness.
    """
    from mathutils.bvhtree import BVHTree

    # Generators add the final skin subdivision after accessories, whereas an
    # existing source already has it. Fit to the same finished skin in both.
    temporary = None
    if not any(mod.type == "SUBSURF" for mod in body.modifiers):
        temporary = body.modifiers.new("Wrist fitting surface", "SUBSURF")
        temporary.levels = temporary.render_levels = 1
    try:
        bpy.context.view_layer.update()
        tree = BVHTree.FromObject(body, bpy.context.evaluated_depsgraph_get())
    finally:
        if temporary is not None:
            body.modifiers.remove(temporary)
    for side in ("l", "r"):
        elbow, wrist = bone_coordinates(rig, "lowerarm_" + side)
        axis = (wrist - elbow).normalized()
        a = axis.cross(Vector((0, 1, 0))).normalized()
        b = axis.cross(a).normalized()

        def section(fraction, clearance):
            center = elbow + (wrist - elbow) * fraction
            points = []
            for index in range(33):
                angle = index * math.tau / 32
                direction = a * math.cos(angle) + b * math.sin(angle)
                hit, normal, _, _ = tree.ray_cast(center + direction * .15, -direction, .15)
                if hit is None:
                    raise ValueError("Missing anatomical wrist surface: " + side)
                points.append(hit + direction * clearance)
            return points

        for index in range(3):
            tube("Glove cuff " + side + str(index), section(.93 + index * .026, .007),
                 .006, leather, rig, "lowerarm_" + side)
        tube("Brass wrist edging " + side, section(.95, .014), .002,
             gold, rig, "lowerarm_" + side)


def add_gloves(body,rig,leather,gold):
    names={"hand_l","hand_r"}
    names.update(p+"_01_"+s for p in ["index","middle","ring","pinky","thumb"] for s in ["l","r"])
    indices={g.index for g in body.vertex_groups if g.name in names}
    glove=body.copy();glove.data=body.data.copy();glove.name="C++ fingerless gloves"
    bpy.context.collection.objects.link(glove)
    for mod in list(glove.modifiers):
        if mod.type!="ARMATURE":glove.modifiers.remove(mod)
    keep=set()
    for v in glove.data.vertices:
        score=sum(g.weight for g in v.groups if g.group in indices)
        if score>.70:keep.add(v.index)
    bm=bmesh.new();bm.from_mesh(glove.data);bm.verts.ensure_lookup_table()
    bmesh.ops.delete(bm,geom=[v for v in bm.verts if v.index not in keep],context="VERTS")
    bm.normal_update()
    for v in bm.verts:v.co+=v.normal*.0023
    bm.to_mesh(glove.data);bm.free()
    glove.data.materials.clear();glove.data.materials.append(leather)
    solid=glove.modifiers.new("Leather edge thickness","SOLIDIFY");solid.thickness=.0013
    add_wrist_cuffs(body, rig, leather, gold)
    for side in ["l","r"]:
        for finger in ["index","middle","ring","pinky"]:
            joint=rig.data.bones[finger+"_01_"+side].head_local
            ellipsoid("Brass glove rivet",joint+Vector((0,.004,.004)),(.0034,.0034,.0034),gold,rig,"hand_"+side)


def add_accessories(rig,leather,gold,white,boots):
    pelvis=rig.data.bones["pelvis"].head_local.z
    waist=pelvis+.12
    # The belt bridges the tucked blouse and the raised trouser waistband.
    vertices=[];faces=[]
    for row,z in enumerate([waist-.019,waist+.023]):
        for k in range(64):
            a=k*math.tau/64
            vertices.append((.151*math.cos(a),-.035+.074*math.sin(a),z))
            if row:faces.append((k,(k+1)%64,(k+1)%64+64,k+64))
    belt=mesh_object("C++ high waist leather belt",vertices,faces,leather);bind(belt,rig,"pelvis")
    tube("Brass belt buckle",[(-.014,-.098,waist-.012),(.014,-.098,waist-.012),(.014,-.098,waist+.016),(-.014,-.098,waist+.016),(-.014,-.098,waist-.012)],.0024,gold,rig,"pelvis")
    center=Vector((-.24,.010,pelvis+.015))
    ellipsoid("C++ circular inheritance satchel",center,(.105,.047,.116),leather,rig,"pelvis")
    for y,radius in [(center.y-.044,.100),(center.y+.034,.098)]:
        ring=[(center.x+radius*math.cos(a*math.tau/64),y,center.z+radius*1.10*math.sin(a*math.tau/64)) for a in range(65)]
        tube("Satchel brass piping",ring,.0026,gold,rig,"pelvis")
    for arm in range(4):
        angle=arm*math.pi/2
        def p(x,z):return center+Vector((x*math.cos(angle)-z*math.sin(angle),-.049,x*math.sin(angle)+z*math.cos(angle)))
        tube("Inheritance cross arm",[p(0,0),p(0,.065),p(-.008,.054),p(0,.065),p(.008,.054)],.0028,gold,rig,"pelvis")
    # Strap is a flat leather ribbon following the shoulder/chest/back, with
    # smooth spine-to-pelvis weights so it remains attached during acting.
    shoulder=rig.data.bones["clavicle_r"].tail_local
    points=[center+Vector((-.045,-.015,.082)),Vector((-.195,-.151,waist+.08)),Vector((-.157,-.182,shoulder.z-.12)),Vector((-.14,-.065,shoulder.z+.039)),Vector((-.14,.055,shoulder.z+.035)),Vector((-.164,.14,shoulder.z-.12)),center+Vector((.042,.047,.075))]
    sv,sf=[],[]
    for i,p in enumerate(points):
        sv.extend([p+Vector((-.011,0,0)),p+Vector((.011,0,0))])
        if i:sf.append(((i-1)*2,(i-1)*2+1,i*2+1,i*2))
    strap=mesh_object("C++ continuous satchel strap",sv,sf,leather);strap.parent=rig
    gp=strap.vertex_groups.new(name="pelvis");gs=strap.vertex_groups.new(name="spine_03")
    for v in strap.data.vertices:
        w=min(1,max(0,(v.co.z-waist)/max(.1,shoulder.z-waist)))
        gp.add([v.index],1-w,"REPLACE");gs.add([v.index],w,"REPLACE")
    sub=strap.modifiers.new("Supple leather strap","SUBSURF");sub.levels=2
    solid=strap.modifiers.new("Strap thickness","SOLIDIFY");solid.thickness=.003
    mod=strap.modifiers.new("Skeletal deformation","ARMATURE");mod.object=rig
    # Gold laces are projected onto the fitted boot surface, then share its
    # ankle weights. They cannot float away when the foot bends in a run.
    from mathutils.bvhtree import BVHTree
    bpy.context.view_layer.update()
    tree=BVHTree.FromObject(boots,bpy.context.evaluated_depsgraph_get())
    def on_boot(x,z):
        hit=tree.ray_cast(Vector((x,-1,z)),Vector((0,1,0)))
        return Vector((x,hit[0].y-.0025 if hit[0] is not None else -.075,z))
    for side in ["l","r"]:
        ankle=rig.data.bones["foot_"+side].head_local
        for row in range(7):
            z=.083+row*.023
            for direction in [-1,1]:
                points=[on_boot(ankle.x+direction*(-.022+.044*i/8),z+.013*i/8) for i in range(9)]
                lace=tube("Golden boot lace",points,.0018,gold,rig,"foot_"+side);boot_weights(lace,rig,side)
            for sign in [-1,1]:
                eyelet=ellipsoid("Boot brass eyelet",on_boot(ankle.x+sign*.024,z),(.0035,.0025,.0035),gold,rig,"foot_"+side);boot_weights(eyelet,rig,side)
    top=rig.data.bones["head"].tail_local.z+.017
    flower=Vector((-.100,-.152,top-.084))
    for k in range(5):
        angle=k*math.tau/5
        offset=Vector((math.sin(angle)*.012,0,math.cos(angle)*.012))
        ellipsoid("C++ flower petal",flower+offset,(.006,.003,.011),gold,rig,"head",(0,angle,0))
    ellipsoid("C++ flower center",flower,(.005,.004,.005),gold,rig,"head")


def setup_stage():
    scene=bpy.context.scene
    scene.render.engine="CYCLES"
    scene.cycles.samples=24
    scene.cycles.use_denoising=True
    scene.render.resolution_x=900
    scene.render.resolution_y=1100
    scene.render.resolution_percentage=100
    scene.view_settings.view_transform="AgX"
    scene.world.color=(.12,.12,.12)
    floor=material("Review floor",(.048,.055,.065),.8)
    bpy.ops.mesh.primitive_plane_add(size=200,location=(0,0,0))
    bpy.context.object.name="REVIEW floor"
    bpy.context.object.data.materials.append(floor)
    def light(name,loc,color,power,size):
        data=bpy.data.lights.new(name,"AREA");data.energy=power;data.color=color;data.shape="DISK";data.size=size
        obj=bpy.data.objects.new(name,data);bpy.context.collection.objects.link(obj);obj.location=loc
        obj.rotation_euler=(Vector((0,0,1))-obj.location).to_track_quat("-Z","Y").to_euler()
    light("REVIEW soft key",(2,-3,4),(1,.79,.61),430,3)
    light("REVIEW cool fill",(-3,-1,2),(.62,.77,1),250,3)
    light("REVIEW warm rim",(.5,2,3),(1,.66,.38),500,2)
    data=bpy.data.cameras.new("REVIEW camera");cam=bpy.data.objects.new("REVIEW camera",data);bpy.context.collection.objects.link(cam)
    scene.camera=cam;cam.data.type="ORTHO";cam.data.ortho_scale=2.12
    return cam


def camera_at(cam,azimuth,height=1.15,target=1.0):
    cam.location=(3.8*math.sin(azimuth),-3.8*math.cos(azimuth),height)
    cam.rotation_euler=(Vector((0,0,target))-cam.location).to_track_quat("-Z","Y").to_euler()


def build_cpp(args,HumanService,TargetService):
    bpy.ops.object.select_all(action="SELECT");bpy.ops.object.delete(use_global=False)
    macros=TargetService.get_default_macro_info_dict()
    macros.update(gender=.001,age=.5,muscle=.58,weight=.44,proportions=.55,height=.53,cupsize=.42,firmness=.6,race=dict(caucasian=.82,african=.1,asian=.08))
    body=HumanService.create_human(macro_detail_dict=macros)
    body.name="C++ anatomical skin"
    morphs={"head/head-diamond":.31,"head/head-scale-vert-decr":.12,"chin/chin-prominent-incr":.14,"chin/chin-width-decr":.10,"nose/nose-greek-incr":.18,"nose/nose-width3-decr":.12,"mouth/mouth-upperlip-volume-incr":.13,"mouth/mouth-lowerlip-volume-incr":.08,"eyes/l-eye-scale-incr":.18,"eyes/r-eye-scale-incr":.18,"cheek/l-cheek-bones-incr":.18,"cheek/r-cheek-bones-incr":.18,"eyebrows/eyebrows-angle-down":.12}
    for name,value in morphs.items():
        path=args.mpfb/"src/mpfb/data/targets"/(name+".target.gz")
        if path.exists():TargetService.load_target(body,str(path),weight=value)
    rig=HumanService.add_builtin_rig(body,"game_engine")
    rig.name="cpp_skeleton"
    base=SOURCE/"makehuman_system_assets"
    HumanService.set_character_skin(str(base/"skins/young_caucasian_female/young_caucasian_female.mhmat"),body,skin_type="MAKESKIN")
    for mat in body.data.materials:
        factor=(.74,.49,.30)
        if mat.name.endswith(".lips"):factor=(.60,.29,.22)
        image_factor(mat,base/"skins/young_caucasian_female/young_lightskinned_female_diffuse.png",factor)
    parts=[]
    for kind,source in [("Eyes","eyes/high-poly/high-poly.mhclo"),("Eyebrows","eyebrows/eyebrow003/eyebrow003.mhclo"),("Eyelashes","eyelashes/eyelashes01/eyelashes01.mhclo")]:
        parts.append(HumanService.add_mhclo_asset(str(base/source),body,asset_type=kind,subdiv_levels=0))
    suit=HumanService.add_mhclo_asset(str(base/"clothes/male_casualsuit03/male_casualsuit03.mhclo"),body,subdiv_levels=0)
    boots=HumanService.add_mhclo_asset(str(base/"clothes/shoes03/shoes03.mhclo"),body,subdiv_levels=0)
    for obj in [body,suit,boots]+parts:freeze_shapes(obj)
    # Garment delete masks include the unrolled lower sleeves, so the only
    # surviving body mask here is the core helper removal mask.
    for mod in list(body.modifiers):
        if mod.type=="MASK" and mod.vertex_group!="body":body.modifiers.remove(mod)
    for mod in list(body.modifiers):
        if mod.type=="MASK":
            active(body);bpy.ops.object.modifier_apply(modifier=mod.name)
    objs=[obj for obj in bpy.context.scene.objects if obj.type=="MESH"]
    scale_character(rig,body,objs)
    brows=parts[1]
    for side in [-1,1]:
        vs=[v for v in brows.data.vertices if v.co.x*side>0]
        center=sum((v.co for v in vs),Vector())/len(vs)
        for v in vs:
            v.co.z=center.z+(v.co.z-center.z)*1.75+.001
            v.co.x=center.x+(v.co.x-center.x)*1.10
            v.co.y-=.001
    for mat in brows.data.materials:
        image_factor(mat,base/"eyebrows/eyebrow003/eyebrow003.png",(.21,.09,.027))
    white=material("C++ ivory cotton",(.76,.73,.665),.77)
    trim=material("C++ rolled cotton and stitching",(.79,.76,.70),.77)
    black=material("C++ charcoal denim",(.020,.023,.026),.78)
    leather=material("C++ soft black leather",(.012,.009,.006),.36)
    gold=material("C++ warm brass",(.59,.30,.055),.27,.76)
    clothbase=base/"clothes/male_casualsuit03"
    image_factor(white,clothbase/"male_casualsuit03_ao.png",(.79,.76,.70),clothbase/"male_casualsuit03_normal.png")
    image_factor(black,clothbase/"male_casualsuit03_ao.png",(.019,.021,.024),clothbase/"male_casualsuit03_normal.png")
    sleeve_and_cloth(suit,rig,white,black,trim)
    retain_visible_skin(body,rig)
    boot_leather=material("C++ sturdy combat boot leather",(.018,.015,.012),.42)
    image_factor(boot_leather,base/"clothes/shoes03/shoes03_diffuse.png",(.45,.40,.33))
    boots.data.materials.clear();boots.data.materials.append(boot_leather)
    for v in boots.data.vertices:
        side="l" if v.co.x>0 else "r"
        center=rig.data.bones["foot_"+side].head_local
        if v.co.z>.15:
            v.co.x=center.x+(v.co.x-center.x)*1.24
            v.co.y=center.y+(v.co.y-center.y)*1.16
    for side in ["l","r"]:
        vs=[v for v in boots.data.vertices if (v.co.x>0)==(side=="l")]
        old_top=max(v.co.z for v in vs)
        for v in vs:
            if v.co.z>.08:v.co.z=.08+(v.co.z-.08)*(.275-.08)/(old_top-.08)
    boot_weights(boots,rig)
    hair=[material("Copper hair "+str(i),c,.53) for i,c in enumerate([(.016,.004,.0015),(.043,.011,.003),(.069,.020,.005),(.10,.030,.008),(.18,.069,.018)])]
    add_hair(body,rig,hair)
    add_gloves(body,rig,leather,gold)
    add_accessories(rig,leather,gold,white,boots)
    for obj in [body,suit,boots]+parts:
        for face in obj.data.polygons:face.use_smooth=True
        if obj in [body,suit,boots]:
            sub=obj.modifiers.new("Sculpted smooth surface","SUBSURF");sub.levels=1;sub.render_levels=1
    bpy.context.view_layer.update()
    OUT.mkdir(parents=True,exist_ok=True)
    sys.path.insert(0,str(ROOT/"tools/blender"))
    from actor_motion import build_actions,Performer
    metadata=build_actions(rig,actor_id="cpp",fps=60)
    (OUT/"cpp-actions.json").write_text(json.dumps(metadata,indent=2)+"\n")
    # Save editable named meshes; export a joined render surface to avoid a
    # draw call for every strand, stitch, rivet or buckle.
    bpy.ops.file.pack_all()
    bpy.ops.wm.save_as_mainfile(filepath=str(OUT/"cpp-pilot.blend"),compress=True)
    rig.animation_data.action=None
    for bone in rig.pose.bones:bone.matrix_basis=Matrix.Identity(4)
    for obj in [o for o in bpy.context.scene.objects if o.type=="MESH"]:
        active(obj)
        for mod in list(obj.modifiers):
            if mod.type!="ARMATURE":bpy.ops.object.modifier_apply(modifier=mod.name)
    bpy.ops.object.select_all(action="DESELECT")
    for obj in [o for o in bpy.context.scene.objects if o.type=="MESH"]:obj.select_set(True)
    bpy.context.view_layer.objects.active=body
    bpy.ops.object.join()
    body.name="cpp_render_surface"
    # Export the actual skinned geometry for independent runtime inspection.
    model_objects=[obj for obj in bpy.context.scene.objects if obj.type in {"MESH","ARMATURE"}]
    bpy.ops.object.select_all(action="DESELECT")
    for obj in model_objects:obj.select_set(True)
    bpy.context.view_layer.objects.active=rig
    MODELS.mkdir(parents=True,exist_ok=True)
    if not args.skip_export:
        bpy.ops.export_scene.gltf(filepath=str(MODELS/"cpp.glb"),export_format="GLB",use_selection=True,export_yup=True,export_apply=True,export_animations=True,export_morph=False)
    cam=setup_stage()
    Performer(rig,"cpp").apply("idle",0)
    args.review.mkdir(parents=True,exist_ok=True)
    for label,azimuth in [("front",0),("quarter",.60),("profile",math.pi/2),("back",math.pi)]:
        camera_at(cam,azimuth)
        bpy.context.scene.render.filepath=str(args.review/(label+".png"))
        bpy.ops.render.render(write_still=True)
    print("CPP_PILOT_COMPLETE",str(args.review))


def main():
    parser=argparse.ArgumentParser()
    parser.add_argument("--mpfb",type=Path,default=Path("/tmp/borrow-fighters-mpfb2"))
    parser.add_argument("--review",type=Path,default=Path("/tmp/cpp-3d-pilot"))
    parser.add_argument("--skip-export",action="store_true",help="Leave the runtime GLB untouched during a native capture")
    args=parser.parse_args(sys.argv[sys.argv.index("--")+1:] if "--" in sys.argv else [])
    HumanService,TargetService=initialize_mpfb(args.mpfb)
    build_cpp(args,HumanService,TargetService)


if __name__=="__main__":
    main()
