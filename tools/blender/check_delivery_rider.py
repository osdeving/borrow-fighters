"""Reimport the delivery rider GLB and measure its seated contact and loop.

The contact test uses the exported skin and anatomical landmarks, independently
of the Blender authoring scene. Boot soles and palm centers are measured rather
than treating ankle/wrist joints as the physical contact surfaces.
"""

import hashlib
import json
from pathlib import Path
import sys

import bpy
from mathutils import Vector

sys.path.insert(0, str(Path(__file__).resolve().parent))
from validate_humans import inspect as inspect_buffers


ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / "assets/adventure/production-3d/humans"
MODEL = ROOT / "assets/adventure/models/delivery-rider.glb"


def main():
    entry = json.loads((OUT / "delivery-rider-model.json").read_text())
    buffers = inspect_buffers("delivery-rider", entry)
    bpy.ops.wm.read_factory_settings(use_empty=True)
    bpy.context.scene.render.fps = 60
    bpy.ops.import_scene.gltf(filepath=str(MODEL))
    # Blender adds an Icosphere bone widget; it is not geometry in the GLB.
    meshes = [obj for obj in bpy.context.scene.objects if obj.type == "MESH"
              and any(mod.type == "ARMATURE" for mod in obj.modifiers)]
    rigs = [obj for obj in bpy.context.scene.objects if obj.type == "ARMATURE"]
    assert len(meshes) == len(rigs) == 1
    mesh, rig = meshes[0], rigs[0]
    rig.animation_data.action = bpy.data.actions["riding"]
    for track in rig.animation_data.nla_tracks:
        track.mute = True
    boot_materials = {i for i, mat in enumerate(mesh.data.materials)
                      if mat.name == "Delivery dark leather boots"}
    boot_indices = {v for poly in mesh.data.polygons
                    if poly.material_index in boot_materials for v in poly.vertices}
    assert len(boot_indices) > 100
    samples, initial = [], None
    loop_error = 0.0
    root_positions = []
    for frame in (0, 30, 60, 90, 120):
        bpy.context.scene.frame_set(frame)
        bpy.context.view_layer.update()
        evaluated = mesh.evaluated_get(bpy.context.evaluated_depsgraph_get())
        surface = evaluated.to_mesh()
        points = [evaluated.matrix_world @ vertex.co for vertex in surface.vertices]
        evaluated.to_mesh_clear()
        if initial is None:
            initial = points
        if frame == 120:
            loop_error = max((start - end).length for start, end in zip(initial, points))
        sole = min(points[i].z for i in boot_indices)
        assert abs(sole - .4215) < .003, (frame, sole)
        root = rig.matrix_world @ rig.pose.bones["Root"].head
        root_positions.append(root)
        contacts = {}
        for side, sign in (("l", 1), ("r", -1)):
            def head(name):
                return rig.matrix_world @ rig.pose.bones[name + "_" + side].head

            wrist = head("hand")
            palm = (wrist + (head("index_01") + head("pinky_01")) * .5) * .5
            grip = Vector((sign * .27, -.44, 1.06))
            knee, ankle = head("calf"), head("foot")
            distance = (palm - grip).length
            assert distance < .035, (frame, side, distance)
            assert abs(knee.x) > .28, (frame, side, knee)
            assert abs(ankle.z - .509) < .00001
            contacts[side] = {"wrist_blender_m": list(wrist), "palm_blender_m": list(palm),
                              "grip_center_blender_m": list(grip), "palm_grip_distance_m": distance,
                              "knee_blender_m": list(knee), "ankle_blender_m": list(ankle)}
        samples.append({"frame": frame, "sole_min_z_m": sole,
                        "floorboard_top_m": .4215, "contacts": contacts})
    root_motion = max((point - root_positions[0]).length for point in root_positions)
    assert root_motion < .000001
    assert loop_error < .00001
    report = {"schema_version": 1, "status": "pass", "blender_version": bpy.app.version_string,
              "method": "Actual GLB buffers plus Blender reimport and evaluated skin",
              "file": str(MODEL.relative_to(ROOT)),
              "sha256": hashlib.sha256(MODEL.read_bytes()).hexdigest(), "buffers": buffers,
              "coordinates": "Blender metres: Z up, -Y forward",
              "animation": "riding", "animation_fps": 60,
              "vertices_reimported": len(initial), "boot_vertices": len(boot_indices),
              "maximum_root_motion_m": root_motion, "maximum_loop_error_m": loop_error,
              "samples": samples,
              "limits": "Contacts and loop verified; anatomy and appearance reviewed in separate renders"}
    output = OUT / "delivery-rider-validation.json"
    output.write_text(json.dumps(report, indent=2) + "\n")
    print("DELIVERY_RIDER_VALIDATION", json.dumps({"status": "pass", "sha256": report["sha256"],
          "loop_error_m": loop_error, "root_motion_m": root_motion, "output": str(output)}), flush=True)


if __name__ == "__main__":
    main()
