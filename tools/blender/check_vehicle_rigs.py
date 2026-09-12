"""Reimport exported GLBs and evaluate rigid wheel deformation in Blender.

Run through Blender after vehicles.py. This verifies the exported skin, inverse
bind matrices and sampled animation together, independently of source objects.
"""

import hashlib
import json
from pathlib import Path

import bpy


ROOT = Path(__file__).resolve().parents[2]


def evaluated_vertices(obj, frame):
    bpy.context.scene.frame_set(frame)
    evaluated = obj.evaluated_get(bpy.context.evaluated_depsgraph_get())
    mesh = evaluated.to_mesh()
    points = [obj.matrix_world @ vertex.co for vertex in mesh.vertices]
    evaluated.to_mesh_clear()
    return points


def inspect(entry):
    bpy.ops.wm.read_factory_settings(use_empty=True)
    bpy.context.scene.render.fps = 60
    path = ROOT / "assets/adventure/models" / entry["file"]
    bpy.ops.import_scene.gltf(filepath=str(path))
    # The importer also creates an Icosphere as a bone display widget. Only the
    # skinned mesh comes from the GLB; its mesh count is checked separately.
    meshes = [obj for obj in bpy.context.scene.objects if obj.type == "MESH"
              and any(modifier.type == "ARMATURE" for modifier in obj.modifiers)]
    rigs = [obj for obj in bpy.context.scene.objects if obj.type == "ARMATURE"]
    assert len(meshes) == len(rigs) == 1
    mesh, rig = meshes[0], rigs[0]
    initial = evaluated_vertices(mesh, 0)
    quarter = evaluated_vertices(mesh, 15)
    final = evaluated_vertices(mesh, 60)
    body_motion = wheel_motion = axial_motion = closing_error = 0.0
    for vertex, start, turned, end in zip(mesh.data.vertices, initial, quarter, final):
        names = [mesh.vertex_groups[group.group].name for group in vertex.groups if group.weight > .5]
        assert len(names) == 1
        movement = (turned - start).length
        closing_error = max(closing_error, (end - start).length)
        if names[0] == "root":
            body_motion = max(body_motion, movement)
        else:
            assert names[0].startswith("wheel_")
            wheel_motion = max(wheel_motion, movement)
            axial_motion = max(axial_motion, abs(turned.x - start.x))
    assert body_motion < .000001
    assert axial_motion < .000001
    assert closing_error < .000001
    assert wheel_motion > .25
    return {"file": str(path.relative_to(ROOT)), "sha256": hashlib.sha256(path.read_bytes()).hexdigest(),
            "blender_version": bpy.app.version_string, "evaluated_frames": [0, 15, 60],
            "vertices": len(initial), "bones": len(rig.data.bones),
            "body_motion_max_m": body_motion, "wheel_motion_max_m": wheel_motion,
            "wheel_axial_motion_max_m": axial_motion, "loop_error_max_m": closing_error,
            "status": "pass"}


def main():
    catalog = json.loads((ROOT / "assets/adventure/models/vehicles.json").read_text())
    report = {"schema_version": 1, "status": "pass", "method": "Blender GLB reimport and evaluated skin",
              "models": {key: inspect(value) for key, value in catalog["entries"].items()}}
    output = ROOT / "assets/adventure/production-3d/vehicles/rig-validation.json"
    output.write_text(json.dumps(report, indent=2) + "\n")
    print("RIG_VALIDATION", json.dumps(report), flush=True)


if __name__ == "__main__":
    main()
