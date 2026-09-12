"""Validate Augusta's self-contained vehicle GLBs without Blender or the game.

This checks the exported buffers and wheel animation, rather than relying on a
successful exporter exit. Run after vehicles.py; the report records exact hashes.
"""

import hashlib
import json
import math
from pathlib import Path
import struct


ROOT = Path(__file__).resolve().parents[2]
FORMATS = {5120: "b", 5121: "B", 5122: "h", 5123: "H", 5125: "I", 5126: "f"}
WIDTHS = {"SCALAR": 1, "VEC2": 2, "VEC3": 3, "VEC4": 4, "MAT4": 16}


def read_glb(path):
    raw = path.read_bytes()
    assert struct.unpack_from("<4sII", raw) == (b"glTF", 2, len(raw))
    length, tag = struct.unpack_from("<I4s", raw, 12)
    assert tag == b"JSON"
    model = json.loads(raw[20:20 + length])
    binary_length, tag = struct.unpack_from("<I4s", raw, 20 + length)
    assert tag == b"BIN\0"
    binary = raw[28 + length:28 + length + binary_length]
    assert not any("uri" in buffer for buffer in model["buffers"])
    assert not model.get("images") and not model.get("cameras")
    assert len(binary) == binary_length
    return model, binary, hashlib.sha256(raw).hexdigest()


def accessor(model, binary, number):
    field = model["accessors"][number]
    view = model["bufferViews"][field["bufferView"]]
    layout = "<" + FORMATS[field["componentType"]] * WIDTHS[field["type"]]
    size = struct.calcsize(layout)
    stride = view.get("byteStride", size)
    offset = view.get("byteOffset", 0) + field.get("byteOffset", 0)
    return [struct.unpack_from(layout, binary, offset + i * stride)
            for i in range(field["count"])]


def inspect(entry):
    path = ROOT / "assets/adventure/models" / entry["file"]
    model, binary, digest = read_glb(path)
    assert len(model.get("skins", [])) == 1
    assert len(model.get("animations", [])) == 1
    assert len(model.get("meshes", [])) == 1
    skin = model["skins"][0]
    joint_names = [model["nodes"][index]["name"] for index in skin["joints"]]
    wheels = [name for name in joint_names if name.startswith("wheel_")]
    assert len(wheels) == (2 if "scooter" in entry["file"] else 4)
    assert set(joint_names) == {"root", *wheels}
    assert model["materials"][entry["paint_material"] - 1]["name"] == "body_paint"
    vertices = triangles = 0
    all_positions = []
    used_joints = set()
    for mesh in model["meshes"]:
        for primitive in mesh["primitives"]:
            assert primitive.get("mode", 4) == 4
            attributes = primitive["attributes"]
            positions = accessor(model, binary, attributes["POSITION"])
            normals = accessor(model, binary, attributes["NORMAL"])
            weights = accessor(model, binary, attributes["WEIGHTS_0"])
            joints = accessor(model, binary, attributes["JOINTS_0"])
            indices = accessor(model, binary, primitive["indices"])
            assert len(positions) == len(normals) == len(weights) == len(joints)
            assert len(indices) % 3 == 0
            assert max(index[0] for index in indices) < len(positions)
            assert all(math.isfinite(v) for row in positions + normals for v in row)
            assert all(abs(sum(v * v for v in normal) - 1) < .001 for normal in normals)
            for row, bones in zip(weights, joints):
                assert abs(sum(row) - 1) < .00001
                assert sum(weight > .00001 for weight in row) == 1
                used_joints.update(bone for bone, weight in zip(bones, row) if weight > .00001)
            vertices += len(positions)
            triangles += len(indices) // 3
            all_positions.extend(positions)
    assert used_joints == set(range(len(joint_names)))
    lower = [min(position[i] for position in all_positions) for i in range(3)]
    upper = [max(position[i] for position in all_positions) for i in range(3)]
    assert -.01 <= lower[1] <= .01, lower
    assert abs(entry["length_m"] - (upper[2] - lower[2])) < .005
    clip = model["animations"][0]
    assert clip["name"] == entry["clips"]["drive"] == "drive"
    rotating = set()
    for channel in clip["channels"]:
        sampler = clip["samplers"][channel["sampler"]]
        times = [row[0] for row in accessor(model, binary, sampler["input"])]
        values = accessor(model, binary, sampler["output"])
        assert abs(times[0]) < .000001 and abs(times[-1] - 1) < .000001
        assert all(right > left for left, right in zip(times, times[1:]))
        assert len(times) == len(values)
        name = model["nodes"][channel["target"]["node"]]["name"]
        target = channel["target"]["path"]
        if target == "rotation" and name in wheels:
            assert len(times) == 61
            # Quaternion signs may invert after one revolution, but orientation
            # must close and the half-cycle must differ by 180 degrees.
            dot_end = abs(sum(a * b for a, b in zip(values[0], values[-1])))
            dot_half = abs(sum(a * b for a, b in zip(values[0], values[30])))
            assert abs(dot_end - 1) < .00001 and dot_half < .00001
            rotating.add(name)
        else:
            assert all(max(abs(a - b) for a, b in zip(values[0], value)) < .00001
                       for value in values), (name, target)
    assert rotating == set(wheels)
    return {"file": str(path.relative_to(ROOT)), "sha256": digest,
            "bytes": path.stat().st_size, "vertices_exported": vertices,
            "triangles": triangles, "materials": len(model["materials"]),
            "joints": joint_names, "bounds_gltf_m": {"min": lower, "max": upper},
            "drive_seconds": 1, "drive_samples": 61,
            "checks": {"self_contained": True, "finite_vertices_and_unit_normals": True,
                       "one_rigid_joint_per_vertex": True, "ground_contact": True,
                       "wheel_loop_closes": True, "no_root_motion": True,
                       "paint_material_index": True}}


def main():
    catalog = json.loads((ROOT / "assets/adventure/models/vehicles.json").read_text())
    assert catalog["schema_version"] == 1
    assert set(catalog["entries"]) == {"hatchback", "taxi", "scooter"}
    report = {"schema_version": 1, "status": "pass",
              "models": {key: inspect(value) for key, value in catalog["entries"].items()}}
    output = ROOT / "assets/adventure/production-3d/vehicles/validation.json"
    output.write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps(report, indent=2))


if __name__ == "__main__":
    main()
