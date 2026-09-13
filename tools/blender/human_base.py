"""Shared CC0 anatomy, fitting and export helpers for Augusta's human cast.

Call create_actor, fit real MHCLO garments, then finalize_fit and customize the
outfit. Export keeps a named editable Blender source and emits an independently
loadable GLB with embedded maps, one surface and a game_engine skeleton.
"""

import json
from pathlib import Path
import shutil
import sys

import bpy
from mathutils import Matrix

sys.path.insert(0,str(Path(__file__).resolve().parent))
from humans import ROOT,SOURCE,OUT,MODELS,initialize_mpfb,active,freeze_shapes,scale_character,image_factor


def source_asset(pack,relative,pack_root=Path("/tmp/borrow-fighters-mh-assets")):
    """Preserve the selected official source directory, without repainted maps."""
    destination=SOURCE/pack/relative
    if not destination.exists():
        origin=pack_root/pack/relative
        if not origin.exists():raise FileNotFoundError(origin)
        shutil.copytree(origin,destination)
    return destination


def create_actor(actor_id,*,gender=.999,age=.5,weight=.5,muscle=.5,
                 race=None,skin="young_caucasian_male",skin_factor=(.85,.72,.57),
                 targets=None,mpfb=Path("/tmp/borrow-fighters-mpfb2")):
    HumanService,TargetService=initialize_mpfb(mpfb)
    bpy.ops.object.select_all(action="SELECT");bpy.ops.object.delete(use_global=False)
    macros=TargetService.get_default_macro_info_dict()
    macros.update(gender=gender,age=age,weight=weight,muscle=muscle,
                  race=race or dict(caucasian=.8,african=.1,asian=.1))
    body=HumanService.create_human(macro_detail_dict=macros)
    body.name=actor_id+" anatomical skin"
    for target,value in (targets or {}).items():
        path=mpfb/"src/mpfb/data/targets"/(target+".target.gz")
        if not path.exists():raise FileNotFoundError(path)
        TargetService.load_target(body,str(path),weight=value)
    rig=HumanService.add_builtin_rig(body,"game_engine");rig.name=actor_id+"_skeleton"
    skin_dir=source_asset("makehuman_system_assets","skins/"+skin)
    mhmat=skin_dir/(skin+".mhmat")
    HumanService.set_character_skin(str(mhmat),body,skin_type="MAKESKIN")
    diffuse=next(line.split(maxsplit=1)[1] for line in mhmat.read_text().splitlines() if line.startswith("diffuseTexture "))
    for mat in body.data.materials:
        factor=skin_factor
        if mat.name.endswith(".lips"):factor=tuple(c*s for c,s in zip(skin_factor,(.92,.83,.80)))
        image_factor(mat,skin_dir/diffuse,factor)
    source_asset("makehuman_system_assets","eyes/materials")
    parts=[]
    for kind,path in [("Eyes","eyes/high-poly"),("Eyebrows","eyebrows/eyebrow003"),("Eyelashes","eyelashes/eyelashes01")]:
        directory=source_asset("makehuman_system_assets",path)
        parts.append(HumanService.add_mhclo_asset(str(next(directory.glob("*.mhclo"))),body,asset_type=kind,subdiv_levels=0))
    return rig,body,parts,HumanService


def fit_asset(HumanService,body,pack,relative,*,kind="Clothes"):
    directory=source_asset(pack,relative)
    obj=HumanService.add_mhclo_asset(str(next(directory.glob("*.mhclo"))),body,asset_type=kind,subdiv_levels=0)
    return obj,directory


def finalize_fit(rig,body,height_m,*,discard_clothes_masks=False):
    objects=[o for o in bpy.context.scene.objects if o.type=="MESH"]
    for obj in objects:freeze_shapes(obj)
    if discard_clothes_masks:
        for mod in list(body.modifiers):
            if mod.type=="MASK" and mod.vertex_group!="body":body.modifiers.remove(mod)
    active(body)
    for mod in list(body.modifiers):
        if mod.type=="MASK":bpy.ops.object.modifier_apply(modifier=mod.name)
    factor=scale_character(rig,body,objects,height_m)
    for obj in objects:
        for face in obj.data.polygons:face.use_smooth=True
    return factor


def add_surface_subdivision(objects,levels=1):
    for obj in objects:
        sub=obj.modifiers.new("Anatomical surface finish","SUBSURF")
        sub.levels=levels;sub.render_levels=levels


def save_export_actor(rig,body,actor_id,*,height_m=None):
    from actor_motion import build_actions
    metadata=build_actions(rig,actor_id=actor_id,fps=60)
    OUT.mkdir(parents=True,exist_ok=True);MODELS.mkdir(parents=True,exist_ok=True)
    # Restore bind pose to measure model height, independent of breathing.
    rig.animation_data.action=None
    for bone in rig.pose.bones:bone.matrix_basis=Matrix.Identity(4)
    bpy.context.view_layer.update()
    if height_m is None:
        coordinates=[]
        depsgraph=bpy.context.evaluated_depsgraph_get()
        for obj in bpy.context.scene.objects:
            if obj.type=="MESH":
                evaluated=obj.evaluated_get(depsgraph)
                coordinates.extend((evaluated.matrix_world@v.co).z for v in evaluated.data.vertices)
        height_m=max(coordinates)-min(coordinates)
    rig.animation_data.action=bpy.data.actions[metadata["clips"]["idle"]]
    bpy.context.scene.frame_set(0)
    bpy.ops.file.pack_all()
    bpy.ops.wm.save_as_mainfile(filepath=str(OUT/(actor_id+".blend")),compress=True)
    rig.animation_data.action=None
    for bone in rig.pose.bones:bone.matrix_basis=Matrix.Identity(4)
    bpy.context.view_layer.update()
    objects=[o for o in bpy.context.scene.objects if o.type=="MESH"]
    for obj in objects:
        active(obj)
        for mod in list(obj.modifiers):
            if mod.type!="ARMATURE":bpy.ops.object.modifier_apply(modifier=mod.name)
    bpy.ops.object.select_all(action="DESELECT")
    for obj in objects:obj.select_set(True)
    bpy.context.view_layer.objects.active=body
    bpy.ops.object.join();body.name=actor_id+"_render_surface"
    rig.select_set(True)
    bpy.context.view_layer.objects.active=rig
    bpy.ops.export_scene.gltf(filepath=str(MODELS/(actor_id+".glb")),export_format="GLB",use_selection=True,export_yup=True,export_apply=True,export_animations=True,export_morph=False)
    record={"file":actor_id+".glb","height_m":height_m,"animation_fps":60,"clips":metadata["clips"],"strides_m":metadata["strides_m"],"durations":metadata["durations"]}
    (OUT/(actor_id+"-model.json")).write_text(json.dumps(record,indent=2)+"\n")
    return record
