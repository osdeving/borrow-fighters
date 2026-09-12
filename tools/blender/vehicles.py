"""Author Augusta's existing hatch, taxi and delivery scooter as editable 3D assets.

Run with Blender 4.5 LTS in background mode. Geometry and materials are original
code-authored translations of production/nightlife.rs; no outside model is used.
Blender coordinates are metres, Z up and -Y forward. glTF exports Y up/+Z forward.
"""

import argparse
import hashlib
import json
import math
from pathlib import Path
import struct
import sys

import bpy
from mathutils import Vector


ROOT = Path(__file__).resolve().parents[2]
AUTHORED = ROOT / "assets/adventure/production-3d/vehicles"
RUNTIME = ROOT / "assets/adventure/models/vehicles"
PARTS = []
MATERIALS = {}
WHEELS = []
TAU = math.tau


def srgb(value):
    value /= 255.0
    return value / 12.92 if value <= 0.04045 else ((value + 0.055) / 1.055) ** 2.4


def material(name, rgb, metallic=0.0, roughness=0.45, alpha=1.0, emission=0.0):
    mat = bpy.data.materials.new(name)
    mat.use_nodes = True
    color = tuple(srgb(v) for v in rgb) + (alpha,)
    mat.diffuse_color = color
    node = mat.node_tree.nodes.get("Principled BSDF")
    node.inputs["Base Color"].default_value = color
    node.inputs["Metallic"].default_value = metallic
    node.inputs["Roughness"].default_value = roughness
    node.inputs["Alpha"].default_value = alpha
    if emission:
        node.inputs["Emission Color"].default_value = color
        node.inputs["Emission Strength"].default_value = emission
    if alpha < 1:
        mat.surface_render_method = "DITHERED"
    MATERIALS[name] = mat
    return mat


def palette(kind):
    MATERIALS.clear()
    material("body_paint", (188, 183, 160) if kind == "taxi" else
             (134, 75, 59) if kind == "delivery-scooter" else (61, 84, 110),
             metallic=0.32, roughness=0.3)
    material("rubber", (22, 25, 28), roughness=0.85)
    material("tread", (31, 35, 37), roughness=0.84)
    material("trim", (29, 34, 39), roughness=0.5)
    material("alloy", (163, 169, 169), metallic=0.8, roughness=0.27)
    material("steel", (80, 90, 99), metallic=0.72, roughness=0.4)
    material("glass", (65, 90, 105), metallic=0.05, roughness=0.14, alpha=0.48)
    material("headlamp", (246, 221, 161), metallic=0.1, roughness=0.19, emission=0.45)
    material("taillamp", (219, 76, 65), roughness=0.24, emission=0.12)
    material("amber", (232, 199, 115), roughness=0.28, emission=0.12)
    material("upholstery", (45, 50, 59), roughness=0.88)
    material("stitching", (86, 91, 95), roughness=0.7)
    material("taxi_band", (99, 90, 74), metallic=0.12, roughness=0.44)
    material("delivery_box", (130, 63, 47), roughness=0.57)
    material("reflector", (197, 139, 86), roughness=0.31)
    material("plate", (181, 184, 173), roughness=0.5)


def register(obj, mat, bone="root", smooth=True):
    obj.data.materials.append(MATERIALS[mat])
    obj["deform_bone"] = bone
    for face in getattr(obj.data, "polygons", []):
        face.use_smooth = smooth
    PARTS.append(obj)
    return obj


def mesh(name, verts, faces, mat, bone="root", smooth=True):
    data = bpy.data.meshes.new(name)
    data.from_pydata(verts, [], faces)
    data.update()
    obj = bpy.data.objects.new(name, data)
    bpy.context.collection.objects.link(obj)
    return register(obj, mat, bone, smooth)


def bevel(obj, amount=0.025, segments=3):
    mod = obj.modifiers.new("rounded manufactured edges", "BEVEL")
    mod.width = amount
    mod.segments = segments
    mod.affect = "EDGES"
    normal = obj.modifiers.new("weighted corner normals", "WEIGHTED_NORMAL")
    normal.keep_sharp = True
    return obj


def box(name, center, dimensions, mat, radius=0.025, rotation=None, bone="root"):
    bpy.ops.mesh.primitive_cube_add(size=1, location=center)
    obj = bpy.context.object
    obj.name = name
    obj.dimensions = dimensions
    bpy.ops.object.transform_apply(location=False, rotation=False, scale=True)
    if rotation:
        obj.rotation_euler = rotation
    register(obj, mat, bone, smooth=bool(radius))
    return bevel(obj, min(radius, min(dimensions) * 0.45), 3) if radius else obj


def tube(name, points, radius, mat, bone="root", resolution=2):
    curve = bpy.data.curves.new(name, "CURVE")
    curve.dimensions = "3D"
    curve.bevel_depth = radius
    curve.bevel_resolution = resolution
    curve.resolution_u = 2
    spline = curve.splines.new("POLY")
    spline.points.add(len(points) - 1)
    for vertex, point in zip(spline.points, points):
        vertex.co = (*point, 1)
    obj = bpy.data.objects.new(name, curve)
    bpy.context.collection.objects.link(obj)
    return register(obj, mat, bone)


def cylinder(name, center, radius, depth, mat, axis="X", bone="root", vertices=32):
    bpy.ops.mesh.primitive_cylinder_add(vertices=vertices, radius=radius, depth=depth,
                                      end_fill_type="NGON", location=center)
    obj = bpy.context.object
    obj.name = name
    if axis == "X":
        obj.rotation_euler[1] = math.pi / 2
    elif axis == "Y":
        obj.rotation_euler[0] = math.pi / 2
    register(obj, mat, bone)
    return bevel(obj, min(0.009, depth * 0.12), 2)


def ellipsoid(name, center, size, mat, bone="root"):
    bpy.ops.mesh.primitive_uv_sphere_add(segments=24, ring_count=12, location=center)
    obj = bpy.context.object
    obj.name = name
    obj.scale = size
    return register(obj, mat, bone)


def lathe_x(name, center, section, mat, bone="root", segments=48):
    verts = [(center[0] + axial,
              center[1] + radius * math.cos(i * TAU / segments),
              center[2] + radius * math.sin(i * TAU / segments))
             for axial, radius in section for i in range(segments)]
    faces = []
    for row in range(len(section)):
        next_row = (row + 1) % len(section)
        for i in range(segments):
            j = (i + 1) % segments
            faces.append((row * segments + i, row * segments + j,
                          next_row * segments + j, next_row * segments + i))
    return mesh(name, verts, faces, mat, bone)


def wheel(name, center, radius=0.31, width=0.21):
    WHEELS.append((name, center, radius))
    half = width * 0.5
    section = [(-half * 0.82, radius * 0.55), (-half, radius * 0.83),
               (-half * 0.75, radius * 0.98), (-half * 0.45, radius),
               (half * 0.45, radius), (half * 0.75, radius * 0.98),
               (half, radius * 0.83), (half * 0.82, radius * 0.55)]
    lathe_x(name + " tire", center, section, "rubber", name)
    for side in (-1, 1):
        face_x = center[0] + side * half * 0.94
        lathe_x(name + " rim lip", (face_x, center[1], center[2]),
                [(-0.012, radius * 0.60), (-0.018, radius * 0.54),
                 (0.018, radius * 0.54), (0.012, radius * 0.60)], "alloy", name, 32)
        cylinder(name + " brake disc", (face_x - side * 0.019, center[1], center[2]),
                 radius * 0.49, 0.016, "steel", bone=name, vertices=24)
        cylinder(name + " hub", (face_x, center[1], center[2]), radius * 0.17,
                 0.033, "alloy", bone=name, vertices=24)
        for spoke in range(5):
            angle = spoke * TAU / 5
            a = (face_x, center[1] + radius * 0.13 * math.cos(angle),
                 center[2] + radius * 0.13 * math.sin(angle))
            b = (face_x, center[1] + radius * 0.54 * math.cos(angle + 0.10),
                 center[2] + radius * 0.54 * math.sin(angle + 0.10))
            tube(name + " five spoke alloy", [a, b], radius * 0.051, "alloy", name)
            cylinder(name + " lug", (face_x + side * 0.022,
                     center[1] + radius * 0.105 * math.cos(angle),
                     center[2] + radius * 0.105 * math.sin(angle)),
                     radius * 0.025, 0.007, "steel", bone=name, vertices=8)
    for i in range(36):
        angle = i * TAU / 36
        points = [(center[0] + offset, center[1] + (radius + 0.001) * math.cos(angle + offset * 0.6),
                   center[2] + (radius + 0.001) * math.sin(angle + offset * 0.6))
                  for offset in (-half * 0.55, 0, half * 0.55)]
        tube(name + " tread sipe", points, 0.002, "tread", name, resolution=0)


def loft_body(length):
    factor = length / 4.0
    sections = [(-2.0, 0.64, 0.40, 0.73), (-1.9, 0.79, 0.31, 0.86),
                (-1.4, 0.85, 0.31, 0.92), (-0.7, 0.86, 0.32, 0.96),
                (0.65, 0.86, 0.32, 0.95), (1.30, 0.85, 0.32, 0.93),
                (1.82, 0.79, 0.33, 0.90), (2.0, 0.66, 0.42, 0.78)]
    verts = []
    for y, width, bottom, top in sections:
        cross = [(-width * .90, bottom), (-width, bottom + .12),
                 (-width * .995, top - .15), (-width * .88, top - .025),
                 (-width * .48, top + .012), (width * .48, top + .012),
                 (width * .88, top - .025), (width * .995, top - .15),
                 (width, bottom + .12), (width * .90, bottom), (0, bottom - .01)]
        verts.extend((x, y * factor, z) for x, z in cross)
    count = 11
    faces = [tuple(reversed(range(count)))]
    for row in range(len(sections) - 1):
        for i in range(count):
            j = (i + 1) % count
            faces.append((row * count + i, row * count + j,
                          (row + 1) * count + j, (row + 1) * count + i))
    faces.append(tuple((len(sections) - 1) * count + i for i in range(count)))
    return mesh("continuous curved body shell", verts, faces, "body_paint")


def arch_cut(body, center):
    bpy.ops.mesh.primitive_cylinder_add(vertices=48, radius=.353, depth=.66, location=center,
                                      rotation=(0, math.pi / 2, 0))
    cutter = bpy.context.object
    modifier = body.modifiers.new("wheel arch cut", "BOOLEAN")
    modifier.operation = "DIFFERENCE"
    modifier.solver = "EXACT"
    modifier.object = cutter
    bpy.context.view_layer.objects.active = body
    bpy.ops.object.modifier_apply(modifier=modifier.name)
    bpy.data.objects.remove(cutter, do_unlink=True)


def passenger_well(body):
    """Keep the body shell hollow where the seats and driver's legs belong."""
    bpy.ops.mesh.primitive_cube_add(size=1, location=(0, .045, 1.02))
    cutter = bpy.context.object
    cutter.dimensions = (1.40, 2.04, 1.20)
    bpy.ops.object.transform_apply(location=False, rotation=False, scale=True)
    modifier = body.modifiers.new("open passenger compartment", "BOOLEAN")
    modifier.operation = "DIFFERENCE"
    modifier.solver = "EXACT"
    modifier.object = cutter
    bpy.context.view_layer.objects.active = body
    bpy.ops.object.modifier_apply(modifier=modifier.name)
    bpy.data.objects.remove(cutter, do_unlink=True)


def arch_lip(center, side):
    points = [(side * .862, center[1] + .357 * math.cos(i * math.pi / 24),
               center[2] + .357 * math.sin(i * math.pi / 24)) for i in range(25)]
    tube("wheel arch rubber reveal", points, .016, "trim")


def body_top_section(y, kind):
    """Interpolate the manufactured body loft for flush window and seam placement."""
    at = y / (1.075 if kind == "taxi" else 1.0)
    stations = [(-2.0, .64, .73), (-1.9, .79, .86), (-1.4, .85, .92),
                (-.7, .86, .96), (.65, .86, .95), (1.30, .85, .93),
                (1.82, .79, .90), (2.0, .66, .78)]
    for (a, wa, za), (b, wb, zb) in zip(stations, stations[1:]):
        if a <= at <= b:
            blend = (at - a) / (b - a)
            return wa + (wb - wa) * blend, za + (zb - za) * blend
    raise ValueError("detail outside body loft")


def body_top_z(x, y, kind):
    width, height = body_top_section(y, kind)
    profile = [(0, height + .012), (width * .48, height + .012),
               (width * .88, height - .025), (width * .995, height - .15)]
    for (a, za), (b, zb) in zip(profile, profile[1:]):
        if a <= abs(x) <= b:
            return za + (zb - za) * (abs(x) - a) / (b - a)
    raise ValueError("detail outside upper body surface")


def cabin(kind):
    rear_roof = .80 if kind == "hatch" else .66
    rear_base = 1.65 if kind == "hatch" else 1.17

    def base(side, y, breadth=.88):
        width, height = body_top_section(y, kind)
        return side * width * breadth, y, height - .013

    def seam(side, y, z):
        width, top = body_top_section(y, kind)
        z = min(z, top - .020)
        if z <= top - .15:
            x = width
        elif z <= top - .025:
            x = width * (.995 - .115 * (z - (top - .15)) / .125)
        else:
            x = width * (.88 - .40 * (z - (top - .025)) / .037)
        return side * (x + .001), y, z

    box("rounded roof skin", (0, (rear_roof - .58) * .5, 1.452),
        (1.408, rear_roof + .70, .075), "body_paint", .065)
    for side in (-1, 1):
        front = [base(side, -1.16), (side * .684, -.57, 1.414),
                 (side * .700, -.025, 1.420), base(side, -.025)]
        rear = [base(side, .055), (side * .700, .055, 1.420),
                (side * .684, rear_roof - .02, 1.409), base(side, rear_base)]
        for label, points in (("front side glass", front), ("rear side glass", rear)):
            mesh(label, points, [(3, 2, 1, 0) if side > 0 else (0, 1, 2, 3)],
                 "glass", smooth=False)
            tube(label + " weather seal", points + points[:1], .018, "trim")
        tube("A pillar", [front[0], front[1]], .039, "body_paint")
        tube("B pillar", [base(side, .012), (side * .705, .012, 1.432)], .030, "trim")
        tube("rear pillar", [rear[2], rear[3]], .047, "body_paint")
        belt = [base(side, y) for y in (-1.16, -.70, .012, .65, rear_base)]
        tube("window belt trim", [(x + side * .006, y, z - .007) for x, y, z in belt], .012, "alloy")
        for y in (-.40, .69):
            box("flush door handle", (side * .866, y, .804), (.027, .143, .025), "alloy", .012)
        for points in ([(-1.12, .925), (-.76, .73), (-.60, .41), (-.03, .39), (-.025, .927)],
                       [(.058, .926), (.06, .39), (.85, .39), (.994, .59), (1.14, .91)]):
            tube("door panel seam", [seam(side, y, z) for y, z in points], .0033, "trim", resolution=1)
        mount = Vector(front[0]).lerp(Vector(front[1]), .17)
        mount.x += side * .01
        tube("mirror stalk", [mount, (side * .978, -.99, 1.028)], .020, "trim")
        box("painted side mirror", (side * .98, -.972, 1.053), (.19, .13, .096), "body_paint", .037)
        box("side mirror glass", (side * .98, -.897, 1.053), (.148, .008, .070), "alloy", .023)
    front_glass = [base(-1, -1.178, .84), base(1, -1.178, .84),
                   (.667, -.575, 1.421), (-.667, -.575, 1.421)]
    rear_glass = [base(-1, rear_base, .84), (-.665, rear_roof, 1.410),
                  (.665, rear_roof, 1.410), base(1, rear_base, .84)]
    for label, points in (("windscreen", front_glass), ("rear glass", rear_glass)):
        mesh(label, points, [(0, 1, 2, 3)], "glass", smooth=False)
        tube(label + " seal", points + points[:1], .018, "trim")
    front_z = front_glass[0][2]
    slope = (1.421 - front_z) / .603
    for side in (-1, 1):
        tube("windscreen wiper", [(side * .18, -1.163, front_z + .015 * slope + .01),
             (side * .59, -1.047, front_z + .131 * slope + .01)], .009, "trim")
    rear_z = rear_glass[0][2]
    rear_slope = (1.410 - rear_z) / (rear_base - rear_roof)
    tube("rear glass wiper", [(0, rear_base - .012, rear_z + .012 * rear_slope + .01),
         (.37, rear_base - .14, rear_z + .14 * rear_slope + .01)], .008, "trim")


def interior(kind):
    box("cabin floor", (0, .05, .39), (1.45, 2.06, .07), "trim", .035)
    box("dashboard", (0, -.935, .91), (1.405, .255, .17), "upholstery", .055)
    for x in (-.40, .40):
        box("front seat cushion", (x, -.15, .588), (.56, .55, .145), "upholstery", .065)
        box("front seat back", (x, .075, .865), (.55, .16, .61), "upholstery", .055,
            rotation=(-.13, 0, 0))
        box("head restraint", (x, .11, 1.215), (.34, .15, .20), "upholstery", .045)
        for offset in (-.18, .18):
            tube("seat stitching", [(x + offset, -.36, .666), (x + offset, .02, .67),
                 (x + offset, .04, 1.06)], .003, "stitching", resolution=1)
    rear_shift = -.22 if kind == "taxi" else 0.0
    box("rear bench cushion", (0, .76 + rear_shift, .60), (1.34, .44, .15), "upholstery", .055)
    box("rear bench back", (0, .972 + rear_shift, .844), (1.32, .15, .49), "upholstery", .05)
    for x in (-.42, .42):
        box("rear head restraint", (x, .995 + rear_shift, 1.13), (.30, .13, .15), "upholstery", .04)
    box("center console", (0, -.28, .64), (.17, .66, .22), "trim", .035)
    tube("gear selector stalk", [(0, -.36, .745), (0, -.40, .856)], .012, "steel")
    ellipsoid("gear selector grip", (0, -.40, .858), (.035, .032, .041), "trim")
    center = (-.40, -.69, 1.032)
    ring = [(center[0] + .155 * math.cos(i * TAU / 32), center[1],
             center[2] + .155 * math.sin(i * TAU / 32)) for i in range(33)]
    tube("steering wheel rim", ring, .016, "trim")
    for angle in (math.pi * .08, math.pi * .92, math.pi * 1.5):
        tube("steering spoke", [center, (center[0] + .14 * math.cos(angle), center[1],
             center[2] + .14 * math.sin(angle))], .015, "steel")
    box("steering center", center, (.12, .038, .073), "trim", .021)
    tube("steering column", [center, (-.40, -.93, .91)], .025, "trim")
    tube("interior mirror stem", [(0, -.58, 1.410), (0, -.66, 1.34)], .009, "trim")
    box("interior rear-view mirror", (0, -.66, 1.323), (.245, .047, .085), "trim", .018)
    box("rear-view mirror surface", (0, -.633, 1.323), (.217, .005, .060), "alloy", .009)
    for x in (-.33, .33):
        box("sun visor", (x, -.50, 1.403), (.42, .17, .019), "upholstery", .006)


def car(kind):
    length = 4.0 if kind == "hatch" else 4.3
    body = loft_body(length)
    passenger_well(body)
    for side in (-1, 1):
        for axle, y in (("front", -1.27 if kind == "hatch" else -1.38),
                        ("rear", 1.23 if kind == "hatch" else 1.31)):
            center = (side * .79, y, .313)
            arch_cut(body, (side * .87, y, .313))
            arch_lip(center, side)
            wheel(f"wheel_{axle}_{'left' if side < 0 else 'right'}", center, .31, .22)
    bevel(body, .027, 3)
    cabin(kind)
    interior(kind)
    for side in (-1, 1):
        box("side rocker trim", (side * .834, 0, .338), (.08, 1.71, .09), "trim", .024)
        box("body side lower crease", (side * .861, 0, .487), (.016, 1.62, .028), "body_paint", .012)
        front_y = -length * .5
        box("headlamp housing", (side * .51, front_y + .040, .73), (.39, .11, .145), "trim", .04)
        box("warm headlamp lens", (side * .51, front_y - .025, .735), (.34, .028, .109), "headlamp", .031)
        box("front indicator", (side * .68, front_y - .025, .731), (.056, .031, .094), "amber", .012)
        box("rear lamp housing", (side * .49, length * .5 - .013, .734), (.31, .093, .144), "trim", .03)
        box("rear lamp red lens", (side * .49, length * .5 + .040, .746), (.27, .025, .105), "taillamp", .019)
        box("rear reverse inset", (side * .42, length * .5 + .056, .729), (.07, .009, .038), "plate", .005)
        box("reflective rear lower lens", (side * .66, length * .5 - .038, .43), (.18, .028, .042), "taillamp", .01)
    for end in (-1, 1):
        y = end * length * .5
        box("wraparound bumper", (0, y - end * .045, .457), (1.43, .154, .137), "trim", .052)
        box("blank license plate surround", (0, y + end * .047, .53), (.48, .022, .125), "trim", .007)
        box("blank registration plate", (0, y + end * .063, .535), (.435, .008, .096), "plate", .004)
    box("front intake", (0, -length * .5 - .012, .66), (.56, .04, .115), "trim", .016)
    for z in (.632, .674, .709):
        box("intake grille bar", (0, -length * .5 - .039, z), (.54, .012, .01), "steel", .003)
    hood = [(-.69, -length * .5 + .29), (-.70, -1.23),
            (.70, -1.23), (.69, -length * .5 + .29)]
    tube("hood joint", [(x, y, body_top_z(x, y, kind) + .002) for x, y in hood],
         .003, "trim", resolution=1)
    cylinder("exhaust tip", (.56, length * .5 - .05, .301), .035, .22, "steel", axis="Y", vertices=20)
    if kind == "taxi":
        box("taxi amber roof sign", (0, .09, 1.575), (.44, .20, .17), "amber", .031)
        box("taxi roof sign base", (0, .09, 1.493), (.48, .22, .025), "trim", .008)
        for side in (-1, 1):
            for y in (-.37, .23):
                box("original taxi door stripe", (side * .862, y, .642),
                    (.009, .047, .455), "taxi_band", .004)
    return {"driver_hip": [-.40, -.14, .70], "driver_hand_left": [-.54, -.69, 1.04],
            "driver_hand_right": [-.26, -.69, 1.04], "driver_foot_left": [-.52, -.84, .40],
            "driver_foot_right": [-.27, -.88, .40]}


def mudguard(name, center, radius, width):
    points = [(x, center[1] + radius * math.cos(i * math.pi / 20),
               center[2] + radius * math.sin(i * math.pi / 20))
              for x in (-width * .5, width * .5) for i in range(21)]
    obj = mesh(name, points, [(i, i + 1, 22 + i, 21 + i) for i in range(20)], "body_paint")
    solid = obj.modifiers.new("formed fender thickness", "SOLIDIFY")
    solid.thickness = .016


def scooter():
    for axle, y in (("front", -.69), ("rear", .66)):
        wheel("wheel_" + axle, (0, y, .244), .24, .105)
        mudguard(axle + " painted fender", (0, y, .244), .289, .18)
    tube("step-through chassis", [(0, .67, .31), (0, .32, .46), (0, -.22, .38),
         (0, -.43, .83), (0, -.45, 1.01)], .047, "steel")
    box("step-through floorboard", (0, -.02, .375), (.40, .73, .075), "trim", .035)
    for x in (-.125, -.062, 0, .062, .125):
        box("floorboard traction groove", (x, -.035, .418), (.012, .57, .007), "tread", .002)
    box("rounded rear bodywork", (0, .34, .559), (.47, .65, .30), "body_paint", .12)
    box("front leg shield", (0, -.378, .716), (.412, .255, .595), "body_paint", .095,
        rotation=(-.14, 0, 0))
    box("inner leg shield", (0, -.221, .753), (.33, .048, .37), "trim", .035,
        rotation=(-.14, 0, 0))
    for side in (-1, 1):
        tube("front telescopic fork", [(side * .09, -.69, .245), (side * .09, -.48, .854)], .025, "alloy")
        tube("fork lower casing", [(side * .09, -.69, .245), (side * .09, -.595, .503)], .038, "steel")
        tube("rear shock", [(side * .16, .66, .28), (side * .16, .44, .622)], .023, "alloy")
        for i in range(10):
            z = .31 + i * .026
            y = .638 - i * .0165
            tube("rear suspension coil", [(side * .16 + .032 * math.cos(j * TAU / 12),
                 y + .012 * math.sin(j * TAU / 12), z + .024 * math.sin(j * TAU / 12))
                 for j in range(13)], .004, "steel", resolution=1)
    box("saddle", (0, .195, .792), (.475, .775, .13), "upholstery", .06)
    for x in (-.16, .16):
        tube("saddle seam", [(x, -.14, .848), (x, .51, .848)], .003, "stitching", resolution=1)
    box("engine transmission case", (-.13, .385, .327), (.20, .57, .21), "steel", .087)
    cylinder("clutch cover", (-.245, .43, .323), .09, .035, "alloy", vertices=28)
    tube("exhaust pipe", [(.14, .25, .31), (.22, .36, .255), (.235, .68, .30)], .026, "steel")
    cylinder("exhaust silencer", (.23, .63, .32), .063, .38, "trim", axis="Y", vertices=28)
    box("silencer heat shield", (.278, .60, .344), (.014, .295, .083), "alloy", .013)
    tube("folded center stand", [(-.15, .31, .37), (-.15, .53, .155), (.15, .53, .155),
         (.15, .31, .37)], .015, "trim")
    tube("handlebar", [(-.31, -.44, 1.06), (-.16, -.47, 1.066), (0, -.445, 1.025),
         (.16, -.47, 1.066), (.31, -.44, 1.06)], .018, "steel")
    for side in (-1, 1):
        cylinder("rubber grip", (side * .281, -.44, 1.06), .024, .115, "rubber", vertices=20)
        tube("brake lever", [(side * .20, -.472, 1.06), (side * .34, -.483, 1.032)], .006, "alloy")
        tube("mirror stalk", [(side * .19, -.45, 1.08), (side * .35, -.40, 1.263)], .008, "steel")
        ellipsoid("mirror housing", (side * .35, -.40, 1.277), (.077, .028, .046), "trim")
        ellipsoid("mirror silver", (side * .35, -.369, 1.277), (.066, .006, .035), "alloy")
        box("front amber indicator", (side * .184, -.601, .929), (.071, .049, .061), "amber", .018)
    box("headlamp housing", (0, -.534, .996), (.326, .203, .167), "body_paint", .06)
    box("warm scooter headlamp", (0, -.65, .996), (.276, .025, .12), "headlamp", .038)
    box("instrument binnacle", (0, -.425, 1.059), (.195, .115, .038), "trim", .018)
    box("instrument glass", (0, -.417, 1.083), (.152, .076, .01), "glass", .007)
    tube("rear luggage rack", [(-.22, .36, .85), (-.25, .91, .845), (.25, .91, .845),
         (.22, .36, .85)], .013, "steel")
    box("original burgundy delivery box", (0, .668, 1.05), (.55, .50, .402), "delivery_box", .035)
    box("delivery lid lip", (0, .668, 1.263), (.567, .516, .039), "trim", .016)
    box("delivery lid", (0, .668, 1.282), (.558, .509, .027), "delivery_box", .012)
    box("original amber cargo stripe", (0, .927, 1.051), (.037, .008, .334), "reflector", .003)
    for x in (-.175, .175):
        box("cargo lid hinge", (x, .935, 1.238), (.083, .017, .026), "steel", .004)
        box("cargo clasp", (x, .41, 1.218), (.033, .018, .071), "steel", .005)
    tube("rear taillight support", [(0, .56, .67), (0, .80, .655)], .018, "steel")
    box("rear taillight housing", (0, .825, .655), (.213, .091, .096), "trim", .016)
    box("rear taillight", (0, .86, .655), (.19, .041, .075), "taillamp", .015)
    box("scooter rear mud flap", (0, .91, .472), (.17, .025, .208), "trim", .013,
        rotation=(.11, 0, 0))
    box("scooter blank plate", (0, .927, .485), (.14, .009, .106), "plate", .006)
    return {"rider_hip": [0, .10, .95], "rider_hand_left": [-.27, -.44, 1.06],
            "rider_hand_right": [.27, -.44, 1.06], "rider_foot_left": [-.15, -.05, .42],
            "rider_foot_right": [.15, -.05, .42], "rider_helmet_top": [0, -.08, 1.70]}


def bind_vehicle(kind, sockets):
    bpy.ops.object.select_all(action="DESELECT")
    bpy.ops.object.armature_add(location=(0, 0, 0))
    rig = bpy.context.object
    rig.name = kind + "_rig"
    bpy.ops.object.mode_set(mode="EDIT")
    root = rig.data.edit_bones[0]
    root.name = "root"
    root.head, root.tail = (0, 0, 0), (0, 0, .30)
    for name, center, _ in WHEELS:
        bone = rig.data.edit_bones.new(name)
        bone.head = center
        bone.tail = (center[0] + .18, center[1], center[2])
        bone.parent = root
    bpy.ops.object.mode_set(mode="OBJECT")
    # Convert all procedural parts to explicit, editable geometry and join by
    # material. Every wheel vertex has exactly one rigid bone, avoiding shear.
    for obj in PARTS:
        bpy.ops.object.select_all(action="DESELECT")
        obj.select_set(True)
        bpy.context.view_layer.objects.active = obj
        bpy.ops.object.convert(target="MESH")
        bpy.ops.object.transform_apply(location=True, rotation=True, scale=True)
        group = obj.vertex_groups.new(name=obj["deform_bone"])
        group.add(range(len(obj.data.vertices)), 1, "REPLACE")
    bpy.ops.object.select_all(action="DESELECT")
    for obj in PARTS:
        obj.select_set(True)
    bpy.context.view_layer.objects.active = PARTS[0]
    bpy.ops.object.join()
    model = bpy.context.object
    model.name = kind + "_body_and_wheels"
    modifier = model.modifiers.new("rigid wheel articulation", "ARMATURE")
    modifier.object = rig
    model.parent = rig
    rig.animation_data_create()
    action = bpy.data.actions.new("drive")
    rig.animation_data.action = action
    for name, _, _ in WHEELS:
        bone = rig.pose.bones[name]
        bone.rotation_mode = "XYZ"
        for frame, angle in ((0, 0), (15, TAU * .25), (30, TAU * .5),
                             (45, TAU * .75), (60, TAU)):
            bone.rotation_euler[1] = angle
            bone.keyframe_insert("rotation_euler", frame=frame, group=name)
    for curve in action.fcurves:
        for key in curve.keyframe_points:
            key.interpolation = "LINEAR"
    bpy.context.scene.frame_start = 0
    bpy.context.scene.frame_end = 60
    bpy.context.scene.render.fps = 60
    bpy.context.scene.frame_set(0)
    root_extra = {"units": "metres", "forward": "-Y Blender / +Z glTF",
                  "wheel_turn_seconds": 1.0, "root_motion": False}
    for key, value in root_extra.items():
        rig[key] = value
    socket_objects = []
    for name, at in sockets.items():
        obj = bpy.data.objects.new(name, None)
        bpy.context.collection.objects.link(obj)
        obj.parent = rig
        obj.location = at
        obj.empty_display_size = .04
        socket_objects.append(obj)
    return rig, model, socket_objects


def look_at(obj, target):
    obj.rotation_euler = (Vector(target) - obj.location).to_track_quat("-Z", "Y").to_euler()


def studio(kind):
    scene = bpy.context.scene
    scene.render.engine = "CYCLES"
    scene.cycles.samples = 48
    scene.cycles.use_denoising = True
    scene.render.resolution_x, scene.render.resolution_y = 1100, 760
    scene.render.resolution_percentage = 100
    if scene.world is None:
        scene.world = bpy.data.worlds.new("studio world")
    scene.world.color = (.12, .12, .12)
    scene.world.use_nodes = True
    scene.world.node_tree.nodes["Background"].inputs[0].default_value = (.065, .082, .11, 1)
    scene.world.node_tree.nodes["Background"].inputs[1].default_value = .45
    floor = bpy.data.materials.new("preview ground only")
    floor.diffuse_color = (.065, .076, .089, 1)
    floor.use_nodes = True
    floor.node_tree.nodes["Principled BSDF"].inputs["Base Color"].default_value = (.065, .076, .089, 1)
    floor.node_tree.nodes["Principled BSDF"].inputs["Roughness"].default_value = .82
    bpy.ops.mesh.primitive_plane_add(size=200, location=(0, 0, -.011))
    ground = bpy.context.object
    ground.name = "STUDIO ground - not exported"
    ground.data.materials.append(floor)
    for name, at, power, color, size in (
        ("key warm", (-3, -4, 6), 950, (1.0, .82, .66), 5.0),
        ("fill cool", (4, -1, 3), 700, (.57, .73, 1.0), 4.0),
        ("rear soft rim", (0, 5, 5), 1100, (1.0, .91, .81), 3.5),
    ):
        data = bpy.data.lights.new(name, "AREA")
        data.energy, data.color, data.shape, data.size = power, color, "DISK", size
        obj = bpy.data.objects.new(name, data)
        bpy.context.collection.objects.link(obj)
        obj.location = at
        look_at(obj, (0, 0, .75))
    bpy.ops.object.camera_add(location=(5.6, -6.8, 3.7))
    camera = bpy.context.object
    camera.name = "STUDIO camera - not exported"
    camera.data.type = "ORTHO"
    camera.data.ortho_scale = 5.5 if kind != "delivery-scooter" else 2.85
    look_at(camera, (0, .05, .70 if kind != "delivery-scooter" else .66))
    scene.camera = camera
    return camera


def parse_glb(path):
    data = path.read_bytes()
    magic, version, length = struct.unpack_from("<4sII", data)
    assert magic == b"glTF" and version == 2 and length == len(data)
    json_length, tag = struct.unpack_from("<I4s", data, 12)
    assert tag == b"JSON"
    return json.loads(data[20:20 + json_length])


def build(kind, preview_directory, render):
    bpy.ops.wm.read_factory_settings(use_empty=True)
    bpy.context.scene.unit_settings.system = "METRIC"
    bpy.context.scene.unit_settings.scale_length = 1
    PARTS.clear()
    WHEELS.clear()
    palette(kind)
    sockets = scooter() if kind == "delivery-scooter" else car(kind)
    rig, model, socket_objects = bind_vehicle(kind, sockets)
    points = [model.matrix_world @ v.co for v in model.data.vertices]
    lower = [min(p[i] for p in points) for i in range(3)]
    upper = [max(p[i] for p in points) for i in range(3)]
    bpy.ops.object.select_all(action="DESELECT")
    for obj in (rig, model, *socket_objects):
        obj.select_set(True)
    bpy.context.view_layer.objects.active = rig
    output = RUNTIME / (kind + ".glb")
    bpy.ops.export_scene.gltf(filepath=str(output), export_format="GLB", use_selection=True,
                             export_animations=True, export_animation_mode="ACTIONS",
                             export_frame_range=True, export_frame_step=1,
                             export_force_sampling=True, export_skins=True,
                             export_extras=True, export_yup=True, export_cameras=False,
                             export_lights=False, export_apply=False)
    gltf = parse_glb(output)
    assert len(gltf.get("skins", [])) == 1
    assert any(animation.get("name") == "drive" for animation in gltf.get("animations", []))
    paint_index = next(i for i, mat in enumerate(gltf["materials"]) if mat["name"] == "body_paint")
    camera = studio(kind)
    source = AUTHORED / (kind + ".blend")
    bpy.ops.wm.save_as_mainfile(filepath=str(source))
    if render:
        poses = [("front", (5.6, -6.8, 3.7)), ("rear", (-5.6, 6.8, 3.4)),
                 ("side", (7, 0, 2.2))]
        for label, at in poses:
            camera.location = at
            look_at(camera, (0, .05, .70 if kind != "delivery-scooter" else .66))
            bpy.context.scene.render.filepath = str(preview_directory / (kind + "-" + label + ".png"))
            bpy.ops.render.render(write_still=True)
    data = {"file": "vehicles/" + kind + ".glb", "height_m": upper[2] - min(lower[2], 0),
            "length_m": upper[1] - lower[1], "animation_fps": 60,
            "clips": {"drive": "drive"}, "paint_material": paint_index + 1}
    if kind == "delivery-scooter":
        # Integration adds the separate anatomical human at the documented sockets.
        data["height_m"] = 1.70
    authored = {"source_blend": str(source.relative_to(ROOT)),
                "runtime_glb": str(output.relative_to(ROOT)), "mesh_vertices": len(model.data.vertices),
                "mesh_triangles": sum(len(p.vertices) - 2 for p in model.data.polygons),
                "bounds_blender_m": {"min": lower, "max": upper}, "sockets_blender_m": sockets,
                "sockets_gltf_m": {name: [p[0], p[2], -p[1]] for name, p in sockets.items()},
                "wheel_bones": [{"name": name, "center_blender_m": center, "radius_m": radius}
                                for name, center, radius in WHEELS],
                "drive_animation": "One positive axle revolution in 1 second, 60Hz; no root motion.",
                "paint_gltf_material": paint_index, "paint_raylib_material": paint_index + 1}
    print("VEHICLE_COMPLETE", kind, json.dumps(data), flush=True)
    return data, authored


def main():
    # These sources are versioned by Git; avoid stale .blend1 files on regeneration.
    bpy.context.preferences.filepaths.save_version = 0
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--only", nargs="+", choices=("hatch", "taxi", "delivery-scooter"))
    parser.add_argument("--no-render", action="store_true")
    parser.add_argument("--preview-directory", type=Path, default=AUTHORED / "previews")
    arguments = parser.parse_args(sys.argv[sys.argv.index("--") + 1:] if "--" in sys.argv else [])
    AUTHORED.mkdir(parents=True, exist_ok=True)
    RUNTIME.mkdir(parents=True, exist_ok=True)
    arguments.preview_directory.mkdir(parents=True, exist_ok=True)
    catalog_path = RUNTIME.parent / "vehicles.json"
    catalog = json.loads(catalog_path.read_text()) if catalog_path.exists() else {"schema_version": 1, "entries": {}}
    provenance_path = AUTHORED / "vehicles.provenance.json"
    provenance = json.loads(provenance_path.read_text()) if provenance_path.exists() else {
        "schema_version": 1, "authorship": "Original procedural modeling by Codex in Blender; no external models or textures.",
        "identity_source": "src/adventure/engine/production/nightlife.rs: existing Hatch, Taxi and DeliveryScooter",
        "coordinates": "Metres; Blender Z up/-Y forward; glTF Y up/+Z forward; wheel contact at ground origin.",
        "paint_material_contract": "Runtime catalog paint_material uses Raylib GLTF material index = glTF index + 1.",
        "models": {}}
    for kind in arguments.only or ("hatch", "taxi", "delivery-scooter"):
        runtime, authored = build(kind, arguments.preview_directory, not arguments.no_render)
        catalog["entries"][{"hatch": "hatchback", "taxi": "taxi", "delivery-scooter": "scooter"}[kind]] = runtime
        provenance["models"][kind] = authored
        catalog_path.write_text(json.dumps(catalog, indent=2) + "\n")
        provenance["blender_version"] = bpy.app.version_string
        provenance["generator"] = str(Path(__file__).resolve().relative_to(ROOT))
        provenance["generator_sha256"] = hashlib.sha256(Path(__file__).read_bytes()).hexdigest()
        for model in provenance["models"].values():
            for key in ("source_blend", "runtime_glb"):
                path = ROOT / model[key]
                model[key + "_sha256"] = hashlib.sha256(path.read_bytes()).hexdigest()
        provenance_path.write_text(json.dumps(provenance, indent=2) + "\n")


if __name__ == "__main__":
    main()
