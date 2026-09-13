"""Author Augusta's anatomical delivery rider and review the scooter contact.

The CC0 MakeHuman body and fitted garments remain editable in the Blender source.
The olive jacket, blue trousers and dark helmet translate the existing delivery
rider. The scooter is imported only after exporting the separate human GLB.
"""

import argparse
import hashlib
import json
import math
from pathlib import Path
import sys

import bpy
from mathutils import Vector

sys.path.insert(0, str(Path(__file__).resolve().parent))
from human_base import create_actor, fit_asset, finalize_fit, save_export_actor, add_surface_subdivision
from humans import ROOT, OUT, MODELS, material, image_factor, mesh_object, bind, tube, boot_weights, active
from actor_motion import Performer
from vehicles import srgb, studio, look_at


AUTHORING_HELPERS_SHA256 = {
    str((Path(__file__).resolve().parent / name).relative_to(ROOT)):
    hashlib.sha256((Path(__file__).resolve().parent / name).read_bytes()).hexdigest()
    for name in ("human_base.py", "humans.py", "actor_motion.py")
}


def color(name, rgb, roughness=.6, metallic=0):
    return material(name, tuple(srgb(v) for v in rgb), roughness, metallic)


def components(obj):
    adjacent = [set() for _ in obj.data.vertices]
    for edge in obj.data.edges:
        a, b = edge.vertices
        adjacent[a].add(b)
        adjacent[b].add(a)
    remaining = set(range(len(adjacent)))
    groups = []
    while remaining:
        seed = remaining.pop()
        group, pending = {seed}, [seed]
        while pending:
            for number in adjacent[pending.pop()]:
                if number in remaining:
                    remaining.remove(number)
                    group.add(number)
                    pending.append(number)
        groups.append(group)
    return groups


def paint_outfit(suit, directory, rig):
    olive = color("Delivery olive padded jacket", (120, 115, 60), .84)
    denim = color("Delivery dark blue denim", (39, 53, 70), .9)
    for mat, rgb in ((olive, (120, 115, 60)), (denim, (39, 53, 70))):
        image_factor(mat, directory / "male_casualsuit05_ao.png",
                     tuple(srgb(v) for v in rgb), directory / "male_casualsuit05_normal.png")
    suit.data.materials.clear()
    suit.data.materials.append(olive)
    suit.data.materials.append(denim)
    pants = set()
    hip = rig.data.bones["pelvis"].head_local.z
    for group in components(suit):
        lower = min(suit.data.vertices[v].co.z for v in group)
        upper = max(suit.data.vertices[v].co.z for v in group)
        if lower < .30 and upper < hip + .25:
            pants.update(group)
    assert pants, "the fitted suit must contain a separate trouser component"
    for polygon in suit.data.polygons:
        polygon.material_index = int(polygon.vertices[0] in pants)


def helmet(body, rig):
    head_group = body.vertex_groups["head"].index
    points = [v.co for v in body.data.vertices
              if any(g.group == head_group and g.weight > .35 for g in v.groups)]
    lower = Vector(tuple(min(p[i] for p in points) for i in range(3)))
    upper = Vector(tuple(max(p[i] for p in points) for i in range(3)))
    center = (lower + upper) * .5
    center.z += .006
    radii = Vector(((upper.x - lower.x) * .5 + .017,
                    (upper.y - lower.y) * .5 + .024,
                    (upper.z - lower.z) * .55 + .008))
    shell_mat = color("Delivery charcoal helmet shell", (27, 35, 42), .26, .15)
    trim = color("Delivery helmet rubber and strap", (16, 20, 24), .75)
    hinge = color("Delivery visor hinge", (74, 88, 96), .30, .65)
    visor = color("Delivery blue visor", (84, 124, 151), .18, .12)
    visor.node_tree.nodes["Principled BSDF"].inputs["Alpha"].default_value = .48
    visor.diffuse_color = (*visor.diffuse_color[:3], .48)
    visor.surface_render_method = "DITHERED"

    def point(theta, phi, margin=0):
        return center + Vector(((radii.x + margin) * math.sin(theta) * math.sin(phi),
                                -(radii.y + margin) * math.sin(theta) * math.cos(phi),
                                (radii.z + margin) * math.cos(theta)))

    def limit(phi):
        return 1.12 + .80 * (1 - math.cos(phi)) * .5

    around, rows = 48, 18
    vertices = [point(.012 + (limit(phi) - .012) * row / rows, phi)
                for row in range(rows + 1) for phi in (col * math.tau / around for col in range(around))]
    faces = [(row * around + col, (row + 1) * around + col,
              (row + 1) * around + (col + 1) % around, row * around + (col + 1) % around)
             for row in range(rows) for col in range(around)]
    faces.append(tuple(range(around)))
    shell = mesh_object("Sculpted open-face helmet shell", vertices, faces, shell_mat)
    thick = shell.modifiers.new("Helmet shell thickness", "SOLIDIFY")
    thick.thickness = .008
    bind(shell, rig, "head")
    border = [point(limit(phi), phi) for phi in (i * math.tau / 48 for i in range(49))]
    tube("Helmet padded edge", border, .005, trim, rig, "head", sides=8)
    columns, rows = 32, 8
    vertices = [point(1.08 + .64 * row / rows, -1.20 + 2.40 * col / columns, .014)
                for row in range(rows + 1) for col in range(columns + 1)]
    faces = [(row * (columns + 1) + col, (row + 1) * (columns + 1) + col,
              (row + 1) * (columns + 1) + col + 1, row * (columns + 1) + col + 1)
             for row in range(rows) for col in range(columns)]
    shield = mesh_object("Curved transparent blue visor", vertices, faces, visor)
    solid = shield.modifiers.new("Visor thickness", "SOLIDIFY")
    solid.thickness = .002
    bind(shield, rig, "head")
    for theta in (1.08, 1.72):
        tube("Visor edge", [point(theta, -1.20 + 2.40 * i / 32, .014) for i in range(33)],
             .0025, trim, rig, "head", sides=6)
    for sign in (-1, 1):
        at = center + Vector((sign * (radii.x + .014), -.015, .025))
        bpy.ops.mesh.primitive_cylinder_add(vertices=24, radius=.020, depth=.010, location=at,
                                            rotation=(0, math.pi / 2, 0))
        cap = bpy.context.object
        cap.name = "Visor hinge screw"
        active(cap)
        bpy.ops.object.transform_apply(location=True, rotation=True, scale=True)
        cap.data.materials.append(hinge)
        bind(cap, rig, "head")
        tube("Helmet chin strap", [center + Vector((sign * radii.x * .90, .006, -.038)),
             center + Vector((sign * .030, -.034, -radii.z * .95))], .005, trim, rig, "head", sides=6)
        tube("Helmet brow vent", [point(.68, sign * .30 + offset, .002) for offset in (-.12, 0, .12)],
             .004, trim, rig, "head", sides=6)
    return {"head_bounds_blender_m": {"min": list(lower), "max": list(upper)},
            "helmet_center_blender_m": list(center), "helmet_radii_m": list(radii)}


def build():
    rig, body, parts, service = create_actor("delivery-rider", gender=.999, age=.42,
        weight=.48, muscle=.48, race={"caucasian": .45, "african": .4, "asian": .15},
        skin_factor=(.69, .49, .34))
    suit, suit_source = fit_asset(service, body, "makehuman_system_assets", "clothes/male_casualsuit05")
    boots, boots_source = fit_asset(service, body, "shoes01", "clothes/toigo_ankle_boots_male")
    finalize_fit(rig, body, 1.74)
    paint_outfit(suit, suit_source, rig)
    leather = color("Delivery dark leather boots", (25, 29, 34), .53)
    boots.data.materials.clear()
    boots.data.materials.append(leather)
    boot_weights(boots, rig)
    anatomy = helmet(body, rig)
    add_surface_subdivision([body, suit, boots], 1)
    return rig, body, anatomy, [suit_source, boots_source]


def contacts(rig):
    """Measure shoe soles and palm centers; these differ from ankle/wrist joints."""
    boots = next(obj for obj in bpy.context.scene.objects if "ankle_boots_male" in obj.name)
    performer = Performer(rig, "delivery-rider")
    samples = []
    for phase in (0, .25, .5, .75, 1):
        performer.apply("riding", phase)
        bpy.context.view_layer.update()
        evaluated = boots.evaluated_get(bpy.context.evaluated_depsgraph_get())
        sole = min((evaluated.matrix_world @ vertex.co).z for vertex in evaluated.data.vertices)
        hands = {}
        for side, sign in (("l", 1), ("r", -1)):
            wrist = rig.pose.bones["hand_" + side].head
            knuckles = (rig.pose.bones["index_01_" + side].head +
                        rig.pose.bones["pinky_01_" + side].head) * .5
            palm = (wrist + knuckles) * .5
            grip = Vector((sign * .27, -.44, 1.06))
            hands[side] = {"wrist": list(wrist), "palm": list(palm),
                           "grip_center": list(grip), "palm_grip_distance_m": (palm - grip).length,
                           "knee": list(rig.pose.bones["calf_" + side].head),
                           "ankle": list(rig.pose.bones["foot_" + side].head)}
        samples.append({"phase": phase, "sole_min_z_m": sole,
                        "floorboard_top_m": .4215, "hands_and_legs": hands})
        assert abs(sole - .4215) < .003, "shoe soles must rest on the traction surface"
        for hand in hands.values():
            assert hand["palm_grip_distance_m"] < .035, "palm must reach the handlebar grip"
            assert abs(hand["knee"][0]) > .28, "knees must remain outside the leg shield"
    performer.apply("riding", 0)
    bpy.context.view_layer.update()
    return {"schema_version": 1, "status": "pass",
            "method": "Evaluated fitted boot mesh and anatomical palm landmarks",
            "coordinates": "Blender metres: Z up, -Y forward",
            "vehicle_foot_socket": "Sole contact at .42m; traction grooves reach .4215m",
            "ankle_target_z_m": .509, "samples": samples}


def preview(rig, directory, record=None):
    if record:
        rig.animation_data.action = bpy.data.actions[record["clips"]["riding"]]
        bpy.context.scene.frame_set(0)
    else:
        Performer(rig, "delivery-rider").apply("riding", 0)
        bpy.context.view_layer.update()
    bpy.ops.import_scene.gltf(filepath=str(MODELS / "vehicles/delivery-scooter.glb"))
    for obj in bpy.context.scene.objects:
        if obj.type == "MESH" and obj.name.startswith("Icosphere"):
            obj.hide_render = True
    camera = studio("delivery-scooter")
    camera.data.ortho_scale = 3.0
    bpy.context.scene.render.resolution_y = 900
    directory.mkdir(parents=True, exist_ok=True)
    for label, at in (("front", (4, -5, 2.7)), ("side", (6, 0, 2.0)),
                       ("rear", (-4, 5, 2.7))):
        camera.location = at
        look_at(camera, (0, .05, .88))
        bpy.context.scene.render.filepath = str(directory / ("rider-scooter-" + label + ".png"))
        bpy.ops.render.render(write_still=True)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--preview-only", action="store_true")
    parser.add_argument("--no-render", action="store_true")
    parser.add_argument("--preview-directory", type=Path,
                        default=OUT / "previews/delivery-rider")
    args = parser.parse_args(sys.argv[sys.argv.index("--") + 1:] if "--" in sys.argv else [])
    rig, body, anatomy, sources = build()
    contact = contacts(rig)
    args.preview_directory.mkdir(parents=True, exist_ok=True)
    (args.preview_directory / "contact-measurements.json").write_text(json.dumps(contact, indent=2) + "\n")
    record = None if args.preview_only else save_export_actor(rig, body, "delivery-rider")
    if record:
        package_names = ("makehuman_system_assets", "shoes01")
        packages = []
        for name in package_names:
            source = json.loads((Path("/tmp/borrow-fighters-mh-assets") / (name + ".download.json")).read_text())
            packages.append({key: source[key] for key in ("name", "url", "license", "sha256", "bytes")})
        provenance = {"schema_version": 1, "identity": "Augusta delivery rider: olive jacket, blue trousers, dark helmet and blue visor",
                      "human_source": "CC0 MakeHuman/MPFB anatomy and fitted garments; authored helmet and visor",
                      "sources": [str(path.relative_to(ROOT)) for path in sources],
                      "source_packages": packages, "authoring_helpers_sha256": AUTHORING_HELPERS_SHA256,
                      "generator": "tools/blender/delivery_rider.py",
                      "generator_sha256": hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
                      "model": record, "anatomy": anatomy,
                      "source_blend": str((OUT / "delivery-rider.blend").relative_to(ROOT)),
                      "runtime_glb": str((MODELS / "delivery-rider.glb").relative_to(ROOT)),
                      "contact_contract": {"sole_target_z_m": .4215, "ankle_target_z_m": .509,
                                           "grip_center_blender_m": ["±0.27", -.44, 1.06],
                                           "wrist_target_blender_m": ["±0.27", -.375, 1.09]},
                      "source_blend_sha256": hashlib.sha256((OUT / "delivery-rider.blend").read_bytes()).hexdigest(),
                      "runtime_glb_sha256": hashlib.sha256((MODELS / "delivery-rider.glb").read_bytes()).hexdigest()}
        (OUT / "delivery-rider.provenance.json").write_text(json.dumps(provenance, indent=2) + "\n")
    if not args.no_render:
        preview(rig, args.preview_directory, record)
    print("DELIVERY_RIDER_COMPLETE", json.dumps(record), flush=True)


if __name__ == "__main__":
    main()
