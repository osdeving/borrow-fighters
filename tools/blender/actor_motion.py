"""Author grounded actions on the MakeHuman game_engine rig in Blender.

The source rig stays editable. Two-bone IK is solved in armature space and baked
to ordinary bone transforms, so exported GLBs require no Blender constraints at
runtime. C++ reuses the existing chapter's authored action targets and timing.
Other adults share anatomically scaled walk/run, conversation and seated poses.
"""

import json
import math
from pathlib import Path

import bpy
from mathutils import Matrix, Quaternion, Vector

ROOT = Path(__file__).resolve().parents[2]
TAU = math.tau


def smooth(t):
    t = min(1.0, max(0.0, t))
    return t * t * (3.0 - 2.0 * t)


def rotate_at(matrix, axis, angle):
    """Rotate around a bone's current head without moving the joint."""
    pivot = Matrix.Translation(matrix.translation)
    return pivot @ Quaternion(axis, math.radians(angle)).to_matrix().to_4x4() @ pivot.inverted() @ matrix


def aimed(bone, head, end):
    direction = end - head
    if direction.length < 1e-7:
        direction = bone.tail_local - bone.head_local
    turn = (bone.tail_local - bone.head_local).rotation_difference(direction)
    return Matrix.Translation(head) @ (turn @ bone.matrix_local.to_quaternion()).to_matrix().to_4x4()


def solve_chain(start, target, length_a, length_b, pole):
    """Use the pole to select a stable elbow/knee plane, without limb stretching."""
    delta = target - start
    distance = min(length_a + length_b - 0.0001, max(abs(length_a - length_b) + 0.0001, delta.length))
    axis = delta.normalized() if delta.length > 1e-7 else Vector((0, 0, -1))
    plane = pole - axis * pole.dot(axis)
    if plane.length < 1e-5:
        plane = axis.cross(Vector((1, 0, 0)))
    plane.normalize()
    along = (length_a * length_a - length_b * length_b + distance * distance) / (2 * distance)
    middle = start + axis * along + plane * math.sqrt(max(0, length_a * length_a - along * along))
    return middle, start + axis * distance


def source_pose(clip, phase):
    keys = clip["keys"]
    phase = min(1.0, max(0.0, phase))
    for first, second in zip(keys, keys[1:]):
        if phase <= second["time"]:
            amount = (phase - first["time"]) / max(1e-8, second["time"] - first["time"])
            amount = smooth(amount) if first.get("curve") == "smooth" else amount
            def mix(a, b):
                if isinstance(a, list):
                    return [mix(x, y) for x, y in zip(a, b)]
                return a + (b - a) * amount
            return {key: mix(value, second["pose"][key]) for key, value in first["pose"].items()}
    return keys[-1]["pose"]


class Performer:
    def __init__(self, rig, actor_id):
        self.rig = rig
        self.actor_id = actor_id
        self.bones = rig.data.bones
        self.order = sorted(self.bones, key=lambda b: len(b.parent_recursive))
        self.rest = {bone.name: bone.matrix_local.copy() for bone in self.bones}
        self.relative = {bone.name: self.rest[bone.parent.name].inverted() @ self.rest[bone.name]
                         if bone.parent else self.rest[bone.name] for bone in self.bones}
        # Unit scale follows the real rig rather than assuming a particular adult height.
        self.unit = (self.bones['head'].tail_local.z - self.bones['Root'].head_local.z) / 1.65
        self.unit = max(0.5, self.unit)
        self.strides = {"walk": 1.12 * self.unit, "run": 2.10 * self.unit}
        self.authored = {}
        source = ROOT / f"assets/adventure/actors/{actor_id}/clips.json"
        if source.is_file():
            self.authored = json.loads(source.read_text())["clips"]
        self.pixels_per_m = 205.0 / (1.74 * self.unit)
        if actor_id == 'cpp':
            self.strides['run'] = 200.0 / self.pixels_per_m

    def children(self, overrides):
        matrices = {}
        for bone in self.order:
            matrices[bone.name] = overrides.get(bone.name)
            if matrices[bone.name] is None:
                matrices[bone.name] = matrices[bone.parent.name] @ self.relative[bone.name] if bone.parent else self.rest[bone.name].copy()
        return matrices

    def foot_target(self, action, phase, index, authored):
        side, sign = ('l', 1) if index == 0 else ('r', -1)
        u = self.unit
        foot, thigh = self.bones[f'foot_{side}'], self.bones[f'thigh_{side}']
        target = foot.head_local.copy()
        target.x = sign * max(0.055 * u, abs(thigh.head_local.x) * 0.92)
        pitch = 0.0
        if action in ('walk', 'run', 'start', 'stop'):
            running = action == 'run'
            leg_phase = (phase + index * 0.5) % 1.0
            stance = 0.36 if running else 0.62
            travel = self.strides['run' if running else 'walk'] * stance
            if leg_phase < stance:
                amount = leg_phase / stance
                target.y += travel * (amount - 0.5)
                toe_off = smooth((amount - 0.82) / 0.18)
                target.z += 0.025 * u * toe_off
                pitch = 12.0 * toe_off
            else:
                amount = (leg_phase - stance) / (1.0 - stance)
                target.y += travel * (0.5 - smooth(amount))
                target.z += (0.23 if running else 0.075) * u * math.sin(math.pi * amount)
                pitch = -10.0 * math.sin(math.pi * amount)
        if action in ('seated', 'scooter'):
            target.y -= (0.34 if action == 'seated' else 0.22) * u
        if action == 'riding':
            target = Vector((sign * 0.15, -0.05, 0.42))
        if action in ('jump', 'fall', 'arrival'):
            target.y += sign * 0.07 * u
            target.z += 0.13 * u * math.sin(math.pi * phase)
        if authored:
            point = authored['feet'][index]
            target.y = foot.head_local.y - point[0] / self.pixels_per_m
            target.z = foot.head_local.z - point[1] / self.pixels_per_m
            pitch = authored.get('foot_angles', [0, 0])[index]
        return target, pitch

    def compose(self, action, phase):
        u = self.unit
        cycle = TAU * phase
        walking = action in ('walk', 'run', 'start', 'stop')
        running = action == 'run'
        bob = 0.003 * u * math.sin(cycle)
        sway = 0.003 * u * math.sin(cycle)
        pitch, twist = 0.0, 0.8 * math.sin(cycle)
        drop = 0.0
        if walking:
            bob = u * ((-0.045 + 0.030 * math.cos(2 * cycle - 1.1)) if running
                       else (-0.030 + 0.010 * math.cos(2 * cycle)))
            sway = 0.009 * u * math.sin(cycle)
            pitch = 8.0 if running else 2.0
            twist = (4.0 if running else 2.4) * math.sin(cycle)
        if action in ('seated', 'scooter'):
            drop = (0.39 if action == 'seated' else 0.31) * u
            pitch = 3.0 if action == 'seated' else 13.0
        if action in ('guard', 'parry'):
            drop = 0.045 * u
            pitch = 4.0
        recoil = math.sin(math.pi * phase) ** 2
        if action == 'hurt':
            pitch = -10.0 * recoil
            drop = 0.055 * u * recoil
        authored = None
        if self.actor_id == 'cpp' and action in self.authored and action not in ('knockout',):
            authored = source_pose(self.authored[action], phase)
            bob = (-authored['pelvis'][1] - 110.0) / self.pixels_per_m
            pitch = authored.get('body_angle', 0.0)
            sway = 0.006 * u * math.sin(cycle) if action == 'run' else 0.0
        overrides = {'Root': Matrix.Translation((sway, 0, bob - drop)) @ self.rest['Root']}
        if action == 'riding':
            offset = Vector((0.0, 0.10, 0.95)) - self.bones['pelvis'].head_local
            overrides['Root'] = Matrix.Translation(offset) @ self.rest['Root']
            pitch = 6.0
        for bone in self.order:
            if bone.name == 'Root':
                continue
            base = overrides[bone.parent.name] @ self.relative[bone.name]
            if bone.name in ('spine_01', 'spine_02', 'spine_03'):
                base = rotate_at(base, (1, 0, 0), pitch / 3)
                if bone.name == 'spine_03':
                    base = rotate_at(base, (0, 0, 1), twist)
            elif bone.name == 'pelvis' and walking:
                base = rotate_at(base, (0, 0, 1), -twist * 0.6)
            elif bone.name == 'pelvis' and action == 'riding':
                base = rotate_at(base, (1, 0, 0), 28.0)
            elif bone.name == 'head':
                base = rotate_at(base, (1, 0, 0), -pitch * 0.35)
                if action in ('conversation', 'talk', 'phone'):
                    base = rotate_at(base, (0, 0, 1), 3.0 * math.sin(cycle))
            overrides[bone.name] = base
        foot_targets = [self.foot_target(action, phase, index, authored) for index in (0, 1)]
        # Lower the pelvis just enough to keep authored support points reachable.
        # This makes the knees absorb a long step instead of lifting planted feet.
        lower_hips = 0.0
        for index, side in enumerate(('l', 'r')):
            hip = overrides[f'thigh_{side}'].translation
            target = foot_targets[index][0]
            reach = self.bones[f'thigh_{side}'].length + self.bones[f'calf_{side}'].length - 0.006 * u
            lateral = (hip.x - target.x) ** 2 + (hip.y - target.y) ** 2
            if lateral < reach * reach:
                allowed_height = target.z + math.sqrt(reach * reach - lateral)
                lower_hips = max(lower_hips, hip.z - allowed_height)
        if lower_hips > 0:
            adjust = Matrix.Translation((0, 0, -lower_hips))
            overrides = {name: adjust @ matrix for name, matrix in overrides.items()}
        body = overrides.copy()
        # Only body controls are fixed; limb descendants inherit the new solved chains.
        overrides = {name: value for name, value in overrides.items()
                     if name in ('Root', 'pelvis', 'spine_01', 'spine_02', 'spine_03', 'neck_01', 'head', 'clavicle_l', 'clavicle_r')}
        for index, (side, sign) in enumerate((('l', 1), ('r', -1))):
            thigh, calf, foot = (self.bones[f'{name}_{side}'] for name in ('thigh', 'calf', 'foot'))
            hip = body[thigh.name].translation
            target, foot_pitch = foot_targets[index]
            knee, ankle = solve_chain(hip, target, thigh.length, calf.length, Vector((sign * 0.04, -1.0, 0.0)))
            overrides[thigh.name] = aimed(thigh, hip, knee)
            overrides[calf.name] = aimed(calf, knee, ankle)
            overrides[foot.name] = Matrix.Translation(ankle) @ (Quaternion((1, 0, 0), math.radians(foot_pitch)) @ foot.matrix_local.to_quaternion()).to_matrix().to_4x4()

            upper, lower, hand = (self.bones[f'{name}_{side}'] for name in ('upperarm', 'lowerarm', 'hand'))
            shoulder = body[upper.name].translation
            reach = upper.length + lower.length
            wrist = Vector((shoulder.x + sign * 0.065 * u, shoulder.y - 0.055 * u,
                            shoulder.z - reach * 0.96))
            relaxed_wrist = wrist.copy()
            if walking:
                # Arm and leg on the same side counter-swing: a forward foot
                # places its hand behind the torso, including C++'s authored run.
                half_travel = self.strides['run' if running else 'walk'] * (0.36 if running else 0.62) * 0.5
                swing = max(-1.0, min(1.0, -(target.y - foot.head_local.y) / half_travel))
                wrist.y += ((0.20 * swing - 0.075) if running else 0.12 * swing) * u
                wrist.z += (0.18 if running else 0.025) * u
                wrist.x += sign * 0.015 * u
            if action in ('conversation', 'talk', 'interact') and side == 'l':
                wrist.y = shoulder.y - (0.23 + 0.035 * math.sin(cycle)) * u
                wrist.z = shoulder.z - (0.22 + 0.04 * math.cos(cycle)) * u
            if action == 'phone' and side == 'r':
                wrist = body['head'].translation + Vector((-0.10 * u, -0.045 * u, 0.015 * u))
            if action == 'seated':
                wrist = Vector((sign * 0.13 * u, shoulder.y - 0.24 * u, hip.z + 0.12 * u))
            if action == 'scooter':
                wrist = Vector((sign * 0.27 * u, shoulder.y - 0.46 * u, hip.z + 0.23 * u))
            if action == 'riding':
                wrist = Vector((sign * 0.27, -0.44, 1.06))
            if action in ('guard', 'parry'):
                wrist = Vector((shoulder.x * 0.90, shoulder.y - 0.22 * u, shoulder.z + 0.06 * u))
            if action in ('restrain', 'restrained', 'pull'):
                if (action == 'restrain' and side == 'l') or (action != 'restrain' and side == 'r'):
                    wrist = Vector((sign * 0.18 * u, (0.39 if action == 'restrain' else -0.39) * u,
                                    shoulder.z - 0.23 * u))
            if action == 'light-1' and not authored and side == 'l':
                hit = smooth(phase / 0.28) * (1.0 - smooth((phase - 0.48) / 0.52))
                wrist = Vector((shoulder.x, shoulder.y - reach * (0.45 + 0.51 * hit), shoulder.z - 0.04 * u))
            if authored and action not in ('idle', 'run', 'guard', 'interact', 'start', 'stop', 'turn', 'arrival'):
                point = authored['hands'][index]
                wrist = Vector((shoulder.x + sign * 0.035 * u, -point[0] / self.pixels_per_m,
                                -point[1] / self.pixels_per_m))
                if action not in ('run', 'guard'):
                    weight = smooth(phase / 0.16) * (1.0 - smooth((phase - 0.65) / 0.35))
                    wrist = relaxed_wrist.lerp(wrist, weight)
            pole = Vector((sign * 0.25, 0.80, -0.80 if running else -0.30))
            if action in ('guard', 'parry'):
                pole = Vector((sign * 0.18, 0.20, -1.0))
            elbow, end = solve_chain(shoulder, wrist, upper.length, lower.length, pole)
            overrides[upper.name] = aimed(upper, shoulder, elbow)
            overrides[lower.name] = aimed(lower, elbow, end)
            # The wrist follows the forearm instead of imposing a down-facing
            # hand on a horizontal arm, which creates the old claw-like pose.
            direction = (end - elbow).normalized().lerp(Vector((0, 0, -1)), 0.12)
            if action == 'phone' and side == 'r':
                direction = Vector((0, 0, 1))
            overrides[hand.name] = aimed(hand, end, end + direction)
        result = self.children(overrides)
        self.curl_fingers(result, action, phase)
        if action == 'knockout':
            fall = smooth((phase - 0.10) / 0.65)
            anchor = Vector((0, 0, self.bones['Root'].head_local.z))
            transform = Matrix.Translation(anchor + Vector((0, 0, 0.075 * u * fall))) @ Quaternion((1, 0, 0), -math.pi * 0.49 * fall).to_matrix().to_4x4() @ Matrix.Translation(-anchor)
            result = {name: transform @ matrix for name, matrix in result.items()}
        return result

    def curl_fingers(self, matrices, action, phase):
        """Close fingers in their anatomical palm plane, independent of bone roll."""
        grip = 0.18
        if action == 'run':
            grip = 0.48
        elif action in ('guard', 'parry', 'riding', 'scooter'):
            grip = 0.88
        elif action.startswith('light-') or action in ('kick', 'spin', 'linker'):
            grip = 0.18 + 0.82 * smooth(phase / 0.12) * (1 - smooth((phase - 0.85) / 0.15))
        elif action in ('restrain', 'restrained', 'pull'):
            grip = 0.45
        for side, sign in (('l', 1), ('r', -1)):
            across = self.bones[f'index_01_{side}'].head_local - self.bones[f'pinky_01_{side}'].head_local
            forward = self.bones[f'middle_03_{side}'].tail_local - self.bones[f'hand_{side}'].head_local
            palm = across.cross(forward).normalized()
            if palm.x * sign > 0:
                palm.negate()
            for finger in ('index', 'middle', 'ring', 'pinky', 'thumb'):
                for joint, angle in enumerate((65, 85, 60), 1):
                    bone = self.bones[f'{finger}_{joint:02}_{side}']
                    parent = bone.parent.name
                    base = matrices[parent] @ self.relative[bone.name]
                    axis = (bone.tail_local - bone.head_local).normalized().cross(palm).normalized()
                    turn = matrices[parent].to_quaternion() @ self.rest[parent].to_quaternion().inverted()
                    axis = turn @ axis
                    amount = grip * (0.35 if finger == 'thumb' else 1.0)
                    if finger == 'thumb' and joint == 1:
                        toward = self.bones[f'middle_02_{side}'].head_local - bone.head_local
                        direction = base.to_quaternion() @ Vector((0, 1, 0))
                        oppose = direction.rotation_difference(turn @ toward)
                        oppose = Quaternion().slerp(oppose, grip * 0.85)
                        pivot = Matrix.Translation(base.translation)
                        base = pivot @ oppose.to_matrix().to_4x4() @ pivot.inverted() @ base
                    matrices[bone.name] = rotate_at(base, axis, angle * amount)

    def apply(self, action, phase, frame=None):
        matrices = self.compose(action, phase)
        for bone in self.order:
            pose = self.rig.pose.bones[bone.name]
            args = dict(matrix=matrices[bone.name], matrix_local=bone.matrix_local, invert=True)
            if bone.parent:
                args.update(parent_matrix=matrices[bone.parent.name], parent_matrix_local=bone.parent.matrix_local)
            basis = bone.convert_local_to_pose(**args)
            location, rotation, scale = basis.decompose()
            pose.rotation_mode = 'QUATERNION'
            # Keep quaternion signs continuous across frames for exported interpolation.
            if pose.rotation_quaternion.dot(rotation) < 0:
                rotation.negate()
            pose.location = location
            pose.rotation_quaternion = rotation
            pose.scale = scale
            if frame is not None:
                for channel in ('location', 'rotation_quaternion', 'scale'):
                    pose.keyframe_insert(channel, frame=frame, group=bone.name)


def build_actions(rig, actor_id='cpp', fps=60):
    """Create named baked actions and return runtime clip/stride metadata.

    Call after rig transforms have been applied. Export these ordinary named
    actions with Blender's glTF ACTIONS mode. The active action is restored to
    idle at frame zero, which also keeps exported timestamps starting at zero.
    """
    if fps != 60:
        raise ValueError('Augusta and the Raylib GLB loader use 60 ticks per second')
    performer = Performer(rig, actor_id)
    rig.animation_data_create()
    rig.animation_data.action = None
    for track in list(rig.animation_data.nla_tracks):
        rig.animation_data.nla_tracks.remove(track)
    for action in list(bpy.data.actions):
        if action.get('borrow_actor') == actor_id:
            bpy.data.actions.remove(action, do_unlink=True)
    durations = {name: clip['duration_ticks'] for name, clip in performer.authored.items()}
    durations.update({'walk': 72, 'conversation': 150, 'talk': 150, 'seated': 150,
                      'phone': 150, 'scooter': 120, 'riding': 120,
                      'restrain': 120, 'restrained': 120, 'pull': 120})
    defaults = {'idle': 120, 'run': 42, 'start': 12, 'stop': 12, 'turn': 16,
                'jump': 24, 'fall': 24, 'land': 12, 'guard': 90, 'parry': 18,
                'hurt': 24, 'knockout': 50, 'arrival': 80, 'interact': 120, 'light-1': 52}
    for name, count in defaults.items():
        durations.setdefault(name, count)
    clips = {}
    for name, duration in durations.items():
        old = bpy.data.actions.get(name)
        if old and old.users == 0:
            bpy.data.actions.remove(old)
        action = bpy.data.actions.new(name)
        action.use_fake_user = True
        action['borrow_actor'] = actor_id
        action['borrow_clip'] = name
        action['duration_ticks'] = duration
        rig.animation_data.action = action
        for tick in range(duration + 1):
            performer.apply(name, tick / duration, frame=tick)
        # Keyframes are baked at the runtime rate; linear interpolation preserves poses.
        for fcurve in action.fcurves:
            for point in fcurve.keyframe_points:
                point.interpolation = 'LINEAR'
        clips[name] = action.name
        print(f'AUTHORED {actor_id}/{name}: {duration + 1} keys', flush=True)
    rig.animation_data.action = bpy.data.actions[clips['idle']]
    bpy.context.scene.render.fps = fps
    bpy.context.scene.frame_set(0)
    performer.apply('idle', 0)
    bpy.context.view_layer.update()
    return {'clips': clips, 'strides_m': performer.strides, 'durations': durations}
