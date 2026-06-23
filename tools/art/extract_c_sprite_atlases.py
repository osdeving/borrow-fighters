#!/usr/bin/env python3
"""Extracts clean C fighter atlases from the current chroma-key references."""

from __future__ import annotations

import json
from pathlib import Path

from PIL import Image, ImageDraw, ImageFont

from sprite_atlas_extractor import ROOT, AtlasConfig, FrameSpec, write_outputs

SECOND_SHEET_Y = 1254
FONT_PATH = Path("/usr/share/fonts/truetype/dejavu/DejaVuSansMono-Bold.ttf")


def load_font(size: int) -> ImageFont.ImageFont:
    """Loads a monospace font when available, falling back to Pillow default."""
    if FONT_PATH.exists():
        return ImageFont.truetype(str(FONT_PATH), size=size)
    return ImageFont.load_default()


def key_magenta_to_checkerboard(source: Image.Image) -> Image.Image:
    """Converts the temporary magenta chroma key to the extractor key color."""
    rgb = source.convert("RGB")
    pixels = rgb.load()
    for y in range(rgb.height):
        for x in range(rgb.width):
            r, g, b = pixels[x, y]
            is_chroma = r >= 170 and g <= 105 and b >= 145 and (r - g) >= 70 and (b - g) >= 65
            if is_chroma:
                pixels[x, y] = (230, 230, 230)
    return rgb


def combined_fight_source(source: Image.Image) -> Image.Image:
    """Builds one logical source image from the two C reference sheets."""
    first = source.convert("RGB")
    second = Image.open(ROOT / "assets/references/langc-04.png").convert("RGB")
    combined = Image.new(
        "RGB",
        (max(first.width, second.width), first.height + second.height),
        (245, 4, 240),
    )
    combined.paste(first, (0, 0))
    combined.paste(second, (0, SECOND_SHEET_Y))
    return key_magenta_to_checkerboard(combined)


def sheet2_box(box: tuple[int, int, int, int]) -> tuple[int, int, int, int]:
    left, top, right, bottom = box
    return (left, top + SECOND_SHEET_Y, right, bottom + SECOND_SHEET_Y)


def frame(
    name: str,
    clip: str,
    box: tuple[int, int, int, int],
    pivot_offset_x: int,
    duration_ms: int,
    pad: int = 8,
) -> FrameSpec:
    left, top, right, bottom = box
    return FrameSpec(
        name,
        clip,
        (
            max(0, left - pad),
            max(0, top - pad),
            right + pad,
            bottom + pad,
        ),
        (left + pivot_offset_x, bottom),
        duration_ms,
    )


def frame2(
    name: str,
    clip: str,
    box: tuple[int, int, int, int],
    pivot_offset_x: int,
    duration_ms: int,
    pad: int = 8,
) -> FrameSpec:
    return frame(name, clip, sheet2_box(box), pivot_offset_x, duration_ms, pad)


def projectile_frame(
    name: str,
    box: tuple[int, int, int, int],
    duration_ms: int,
    pad: int = 4,
) -> FrameSpec:
    left, top, right, bottom = box
    return FrameSpec(
        name,
        "projectile",
        (max(0, left - pad), max(0, top - pad), right + pad, bottom + pad),
        ((left + right) // 2, (top + bottom) // 2),
        duration_ms,
    )


def projectile_frame2(
    name: str,
    box: tuple[int, int, int, int],
    duration_ms: int,
    pad: int = 4,
) -> FrameSpec:
    return projectile_frame(name, sheet2_box(box), duration_ms, pad)


def clean_low_alpha_cell(cell: Image.Image, _spec: FrameSpec) -> Image.Image:
    """Drops resize residue from the keyed chroma background."""
    if _spec.clip == "projectile":
        return render_bitstream_projectile_cell()

    cleaned = cell.copy()
    pixels = cleaned.load()
    for y in range(cleaned.height):
        for x in range(cleaned.width):
            r, g, b, a = pixels[x, y]
            if a < 24:
                pixels[x, y] = (r, g, b, 0)
    cleaned = remove_small_upper_components(cleaned)
    if _spec.clip == "spawn":
        cleaned = add_spawn_bit_effect(cleaned, _spec.name)
    return cleaned


def draw_glow_circle(
    draw: ImageDraw.ImageDraw,
    center: tuple[int, int],
    radius: int,
    color: tuple[int, int, int],
    alpha: int,
) -> None:
    """Draws a soft circular glow with concentric transparent circles."""
    cx, cy = center
    for step in range(radius, 0, -5):
        current_alpha = round(alpha * (step / radius) * 0.32)
        draw.ellipse(
            (cx - step, cy - step, cx + step, cy + step),
            fill=(*color, current_alpha),
        )


def draw_outlined_text(
    draw: ImageDraw.ImageDraw,
    position: tuple[int, int],
    text: str,
    font: ImageFont.ImageFont,
    fill: tuple[int, int, int, int],
    outline: tuple[int, int, int, int],
    stroke_width: int = 2,
) -> None:
    """Draws readable glyphs with a dark outline for stage contrast."""
    draw.text(
        position,
        text,
        font=font,
        fill=fill,
        stroke_width=stroke_width,
        stroke_fill=outline,
    )


def render_bitstream_projectile_cell() -> Image.Image:
    """Creates a readable stream of 0/1 glyphs for C's projectile."""
    cell = Image.new("RGBA", (384, 256), (0, 0, 0, 0))
    draw = ImageDraw.Draw(cell, "RGBA")
    large = load_font(34)
    medium = load_font(27)
    small = load_font(20)

    for offset in range(0, 6):
        alpha = 120 - offset * 13
        y = 126 + (offset - 2) * 4
        draw.line((36, y, 336, y - 12), fill=(64, 218, 255, alpha), width=3)
        draw.line((52, y + 18, 324, y + 9), fill=(168, 96, 255, alpha), width=2)

    glyphs = [
        ("1", 54, 132, small, (178, 242, 255, 210)),
        ("0", 90, 103, medium, (246, 252, 255, 255)),
        ("1", 128, 132, large, (255, 255, 255, 255)),
        ("0", 174, 94, large, (255, 236, 160, 255)),
        ("1", 222, 126, large, (255, 255, 255, 255)),
        ("0", 276, 104, medium, (185, 246, 255, 245)),
        ("1", 314, 137, small, (178, 242, 255, 210)),
    ]
    for text, x, y, font, color in glyphs:
        draw.ellipse((x - 5, y + 2, x + 28, y + 36), fill=(42, 205, 255, 38))
        draw_outlined_text(
            draw,
            (x, y),
            text,
            font,
            color,
            (24, 18, 56, 230),
            stroke_width=3,
        )

    for x, y in [(118, 92), (204, 164), (256, 88), (292, 156), (72, 160)]:
        draw.rectangle((x - 2, y - 2, x + 2, y + 2), fill=(255, 214, 94, 220))
        draw.line((x - 7, y, x + 7, y), fill=(255, 214, 94, 160), width=1)
        draw.line((x, y - 7, x, y + 7), fill=(255, 214, 94, 160), width=1)

    return cell


def add_spawn_bit_effect(cell: Image.Image, frame_name: str) -> Image.Image:
    """Adds a light binary flourish to C's cinematic intro frames."""
    overlay = cell.copy()
    draw = ImageDraw.Draw(overlay, "RGBA")
    font = load_font(17)
    small = load_font(13)
    frame_index = int(frame_name.rsplit("_", 1)[1])
    effects: dict[int, list[tuple[str, int, int, ImageFont.ImageFont, int]]] = {
        1: [("1", 50, 74, font, 210), ("0", 78, 46, font, 230), ("1", 104, 82, small, 160)],
        2: [("0", 120, 38, font, 235), ("1", 150, 55, font, 220), ("0", 186, 96, small, 175)],
        3: [("1", 62, 58, font, 230), ("0", 94, 36, font, 210), ("1", 123, 74, small, 175)],
        4: [("0", 88, 40, font, 220), ("1", 118, 28, font, 205), ("0", 150, 52, small, 160)],
        5: [("1", 150, 94, font, 235), ("0", 184, 112, font, 220), ("1", 218, 135, small, 175)],
        6: [("0", 190, 118, small, 155), ("1", 222, 106, small, 135)],
    }
    if frame_index not in effects:
        return overlay

    for text, x, y, glyph_font, alpha in effects[frame_index]:
        draw.line((x - 8, y + 12, x + 26, y + 4), fill=(70, 220, 255, 95), width=2)
        draw.line((x - 12, y + 20, x + 18, y + 16), fill=(168, 96, 255, 75), width=1)
        draw_outlined_text(
            draw,
            (x, y),
            text,
            glyph_font,
            (230, 250, 255, alpha),
            (28, 18, 58, min(255, alpha + 30)),
            stroke_width=2,
        )
    return overlay


def remove_small_upper_components(cell: Image.Image) -> Image.Image:
    alpha = cell.getchannel("A")
    width, height = cell.size
    visited = bytearray(width * height)
    source = cell.load()
    result = Image.new("RGBA", cell.size, (0, 0, 0, 0))
    target = result.load()

    for start_y in range(height):
        for start_x in range(width):
            index = start_y * width + start_x
            if visited[index] or alpha.getpixel((start_x, start_y)) == 0:
                continue

            stack = [(start_x, start_y)]
            visited[index] = 1
            points: list[tuple[int, int]] = []
            while stack:
                x, y = stack.pop()
                points.append((x, y))
                for nx in (x - 1, x, x + 1):
                    for ny in (y - 1, y, y + 1):
                        if nx < 0 or ny < 0 or nx >= width or ny >= height:
                            continue
                        neighbor_index = ny * width + nx
                        if visited[neighbor_index] or alpha.getpixel((nx, ny)) == 0:
                            continue
                        visited[neighbor_index] = 1
                        stack.append((nx, ny))

            top = min(y for _, y in points)
            bottom = max(y for _, y in points) + 1
            area = len(points)
            if area < 500 and bottom < 86:
                continue
            if area < 48:
                continue

            for x, y in points:
                target[x, y] = source[x, y]

    return result


def annotate_fighter_manifest() -> None:
    manifest_path = ROOT / "assets/placeholder/c-fighter.sprite.json"
    manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
    manifest["source"] = "assets/references/langc-03.png + assets/references/langc-04.png"
    manifest["notes"] = [
        "Extracted from two RGB chroma-key C reference sheets; not a final authored sprite export.",
        "Source labels and magenta background are intentionally excluded from runtime assets.",
        "The special projectile origin is a first-pass alignment point for the book/bitstream attack.",
        "The projectile frame is replaced by a generated readable 0/1 bitstream placeholder.",
    ]
    atlas = Image.open(ROOT / "assets/placeholder/c-fighter-atlas.png").convert("RGBA")
    for frame_record in manifest["frames"]:
        if frame_record["name"] == "special_0":
            frame_record["combat"] = {
                "projectile_origin": {
                    "x": 208,
                    "y": 145,
                }
            }
        if frame_record["name"] == "projectile_0":
            rect = frame_record["frame"]
            projectile_cell = atlas.crop(
                (
                    rect["x"],
                    rect["y"],
                    rect["x"] + rect["w"],
                    rect["y"] + rect["h"],
                )
            )
            bbox = projectile_cell.getchannel("A").getbbox()
            if bbox is not None:
                left, top, right, bottom = bbox
                frame_record["trimmed_bounds"] = {
                    "x": left,
                    "y": top,
                    "w": right - left,
                    "h": bottom - top,
                }
    manifest_path.write_text(json.dumps(manifest, indent=2) + "\n", encoding="utf-8")


def annotate_start_manifest() -> None:
    manifest_path = ROOT / "assets/placeholder/c-start.sprite.json"
    manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
    manifest["notes"] = [
        "Extracted from the INTRO row of the C reference sheet; not final authored animation.",
        "Small generated 0/1 flourishes were added to make the entrance read as C/linker energy.",
        "This atlas is presentation-only and must not carry combat metadata.",
    ]
    manifest_path.write_text(json.dumps(manifest, indent=2) + "\n", encoding="utf-8")


FIGHT_CLIPS: dict[str, dict[str, object]] = {
    "idle": {"loop": True},
    "walk": {"loop": True},
    "crouch": {"loop": False},
    "jump": {"loop": False},
    "punch_light": {"loop": False},
    "punch_medium": {"loop": False},
    "punch_heavy": {"loop": False},
    "kick_light": {"loop": False},
    "kick": {"loop": False},
    "kick_heavy": {"loop": False},
    "block": {"loop": True},
    "hit": {"loop": False},
    "knockdown": {"loop": False},
    "taunt": {"loop": False},
    "victory": {"loop": False},
    "special": {"loop": False},
    "projectile": {"loop": True},
}

SPAWN_CLIP = {"spawn": {"loop": False}}


C_START_FRAMES: list[FrameSpec] = [
    frame("spawn_00", "spawn", (98, 4, 243, 187), 72, 110),
    frame("spawn_01", "spawn", (261, 6, 383, 188), 61, 135),
    frame("spawn_02", "spawn", (419, 4, 530, 188), 56, 155),
    frame("spawn_03", "spawn", (542, 5, 676, 188), 67, 125),
    frame("spawn_04", "spawn", (693, 3, 804, 188), 56, 130),
    frame("spawn_05", "spawn", (845, 5, 964, 188), 60, 150),
    frame("spawn_06", "spawn", (998, 4, 1117, 188), 60, 210),
]

C_FIGHT_FRAMES: list[FrameSpec] = [
    frame("idle_0", "idle", (127, 189, 219, 367), 46, 120),
    frame("idle_1", "idle", (275, 191, 370, 367), 48, 120),
    frame("idle_2", "idle", (416, 190, 510, 366), 48, 120),
    frame("idle_3", "idle", (559, 193, 656, 366), 49, 120),
    frame("idle_4", "idle", (696, 191, 793, 366), 49, 120),
    frame("idle_5", "idle", (830, 193, 931, 367), 51, 120),
    frame("idle_6", "idle", (953, 192, 1046, 367), 47, 120),
    frame("walk_0", "walk", (97, 368, 217, 519), 62, 85),
    frame("walk_1", "walk", (267, 369, 388, 521), 64, 85),
    frame("walk_2", "walk", (436, 369, 546, 521), 58, 85),
    frame("walk_3", "walk", (595, 372, 714, 522), 61, 85),
    frame("walk_4", "walk", (750, 369, 866, 521), 59, 85),
    frame("walk_5", "walk", (894, 372, 1013, 523), 61, 85),
    frame("walk_6", "walk", (1043, 371, 1160, 523), 60, 85),
    frame("crouch_0", "crouch", (96, 525, 198, 645), 52, 110),
    frame("crouch_1", "crouch", (273, 528, 384, 645), 56, 110),
    frame("crouch_2", "crouch", (473, 528, 573, 645), 51, 110),
    frame("crouch_3", "crouch", (666, 524, 771, 645), 53, 140),
    frame("jump_0", "jump", (95, 689, 189, 794), 48, 105),
    frame("jump_1", "jump", (238, 665, 343, 798), 53, 105),
    frame("jump_2", "jump", (396, 648, 483, 787), 45, 105),
    frame("jump_3", "jump", (539, 645, 645, 785), 54, 105),
    frame("jump_4", "jump", (689, 669, 813, 786), 64, 105),
    frame("jump_5", "jump", (861, 677, 951, 794), 46, 120),
    frame("block_0", "block", (98, 799, 220, 948), 61, 105),
    frame("block_1", "block", (272, 804, 395, 947), 62, 105),
    frame("block_2", "block", (442, 804, 562, 948), 60, 105),
    frame("block_3", "block", (598, 805, 720, 947), 61, 120),
    frame("hit_0", "hit", (98, 950, 252, 1081), 66, 110),
    frame("hit_1", "hit", (288, 948, 416, 1070), 61, 110),
    frame("hit_2", "hit", (444, 962, 569, 1079), 62, 110),
    frame("hit_3", "hit", (587, 962, 702, 1075), 58, 130),
    frame("taunt_0", "taunt", (108, 1080, 214, 1248), 54, 150),
    frame("taunt_1", "taunt", (286, 1080, 404, 1248), 58, 150),
    frame("taunt_2", "taunt", (453, 1079, 575, 1248), 62, 150),
    frame("taunt_3", "taunt", (627, 1080, 739, 1248), 56, 150),
    frame("taunt_4", "taunt", (770, 1079, 902, 1248), 66, 150),
    frame("taunt_5", "taunt", (945, 1078, 1072, 1248), 64, 180),
    frame2("punch_light_0", "punch_light", (117, 21, 224, 167), 54, 75),
    frame2("punch_light_1", "punch_light", (285, 22, 397, 167), 56, 75),
    frame2("punch_light_2", "punch_light", (453, 22, 628, 167), 55, 75),
    frame2("punch_light_3", "punch_light", (655, 22, 830, 167), 55, 75),
    frame2("punch_light_4", "punch_light", (865, 22, 970, 167), 53, 95),
    frame2("punch_medium_0", "punch_medium", (109, 187, 216, 326), 54, 80),
    frame2("punch_medium_1", "punch_medium", (261, 187, 429, 326), 56, 80),
    frame2("punch_medium_2", "punch_medium", (451, 187, 648, 326), 58, 80),
    frame2("punch_medium_3", "punch_medium", (673, 191, 864, 326), 56, 80),
    frame2("punch_medium_4", "punch_medium", (866, 191, 975, 327), 55, 80),
    frame2("punch_medium_5", "punch_medium", (1009, 198, 1122, 327), 57, 100),
    frame2("punch_heavy_0", "punch_heavy", (118, 343, 230, 504), 56, 95),
    frame2("punch_heavy_1", "punch_heavy", (263, 358, 375, 504), 56, 95),
    frame2("punch_heavy_2", "punch_heavy", (410, 358, 531, 504), 56, 95),
    frame2("punch_heavy_3", "punch_heavy", (568, 337, 695, 505), 58, 95),
    frame2("punch_heavy_4", "punch_heavy", (717, 355, 842, 514), 60, 95),
    frame2("punch_heavy_5", "punch_heavy", (853, 355, 962, 505), 55, 95),
    frame2("punch_heavy_6", "punch_heavy", (997, 358, 1101, 504), 53, 110),
    frame2("kick_light_0", "kick_light", (123, 524, 226, 661), 53, 85),
    frame2("kick_light_1", "kick_light", (294, 523, 434, 662), 50, 85),
    frame2("kick_light_2", "kick_light", (494, 526, 628, 661), 51, 85),
    frame2("kick_light_3", "kick_light", (678, 524, 795, 662), 53, 85),
    frame2("kick_light_4", "kick_light", (813, 527, 914, 663), 51, 105),
    frame2("kick_0", "kick", (129, 678, 219, 816), 45, 90),
    frame2("kick_1", "kick", (267, 678, 407, 818), 44, 90),
    frame2("kick_2", "kick", (453, 678, 581, 817), 42, 90),
    frame2("kick_3", "kick", (618, 678, 757, 817), 44, 90),
    frame2("kick_4", "kick", (791, 687, 867, 818), 39, 90),
    frame2("kick_5", "kick", (914, 683, 1002, 818), 44, 110),
    frame2("kick_heavy_0", "kick_heavy", (110, 845, 225, 976), 56, 95),
    frame2("kick_heavy_1", "kick_heavy", (272, 833, 360, 974), 46, 95),
    frame2("kick_heavy_2", "kick_heavy", (419, 835, 560, 974), 47, 95),
    frame2("kick_heavy_3", "kick_heavy", (601, 826, 757, 974), 43, 95),
    frame2("kick_heavy_4", "kick_heavy", (768, 837, 839, 974), 36, 95),
    frame2("kick_heavy_5", "kick_heavy", (886, 853, 981, 976), 48, 95),
    frame2("kick_heavy_6", "kick_heavy", (1013, 853, 1099, 976), 44, 115),
    frame2("special_0", "special", (97, 995, 219, 1137), 58, 95),
    frame2("special_1", "special", (248, 995, 390, 1137), 55, 95),
    frame2("special_2", "special", (390, 995, 522, 1138), 57, 95),
    frame2("special_3", "special", (540, 995, 662, 1137), 58, 95),
    frame2("special_4", "special", (672, 980, 842, 1137), 58, 95),
    frame2("special_5", "special", (837, 1006, 1076, 1137), 57, 120),
    projectile_frame2("projectile_0", (988, 1036, 1122, 1124), 75),
    frame2("knockdown_0", "knockdown", (61, 1185, 203, 1264), 68, 90),
    frame2("knockdown_1", "knockdown", (227, 1171, 355, 1256), 64, 90),
    frame2("knockdown_2", "knockdown", (384, 1185, 549, 1265), 82, 90),
    frame2("knockdown_3", "knockdown", (593, 1206, 740, 1272), 74, 90),
    frame2("knockdown_4", "knockdown", (776, 1219, 938, 1268), 81, 90),
    frame2("knockdown_5", "knockdown", (965, 1230, 1122, 1273), 79, 120),
    frame2("victory_0", "victory", (120, 1292, 212, 1402), 46, 140),
    frame2("victory_1", "victory", (248, 1284, 352, 1402), 52, 140),
    frame2("victory_2", "victory", (381, 1307, 471, 1402), 45, 140),
    frame2("victory_3", "victory", (495, 1307, 622, 1402), 64, 140),
    frame2("victory_4", "victory", (661, 1312, 757, 1402), 49, 140),
    frame2("victory_5", "victory", (778, 1291, 899, 1402), 61, 140),
    frame2("victory_6", "victory", (926, 1313, 1031, 1402), 53, 170),
]


START_CONFIG = AtlasConfig(
    source_path=ROOT / "assets/references/langc-03.png",
    atlas_path=ROOT / "assets/placeholder/c-start-atlas.png",
    manifest_path=ROOT / "assets/placeholder/c-start.sprite.json",
    projectile_path=ROOT / "assets/placeholder/c-bitstream-projectile.png",
    preview_path=ROOT / "tmp/art/c-start-atlas-preview.png",
    frames=C_START_FRAMES,
    clips=SPAWN_CLIP,
    cell_width=384,
    cell_height=256,
    columns=4,
    target_pivot=(112, 236),
    scale=1.16,
    projectile_frame_name="",
    source_processor=key_magenta_to_checkerboard,
    cell_processor=clean_low_alpha_cell,
)

FIGHT_CONFIG = AtlasConfig(
    source_path=ROOT / "assets/references/langc-03.png",
    atlas_path=ROOT / "assets/placeholder/c-fighter-atlas.png",
    manifest_path=ROOT / "assets/placeholder/c-fighter.sprite.json",
    projectile_path=ROOT / "assets/placeholder/c-bitstream-projectile.png",
    preview_path=ROOT / "tmp/art/c-fighter-atlas-preview.png",
    frames=C_FIGHT_FRAMES,
    clips=FIGHT_CLIPS,
    cell_width=384,
    cell_height=256,
    columns=6,
    target_pivot=(112, 236),
    scale=1.16,
    projectile_frame_name="projectile_0",
    source_processor=combined_fight_source,
    cell_processor=clean_low_alpha_cell,
)


if __name__ == "__main__":
    write_outputs(START_CONFIG)
    write_outputs(FIGHT_CONFIG)
    annotate_fighter_manifest()
    annotate_start_manifest()
