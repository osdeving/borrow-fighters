#!/usr/bin/env python3
"""Extracts spawn animation atlases for the current Rust and Duke references."""

from __future__ import annotations

from PIL import Image

from sprite_atlas_extractor import (
    ROOT,
    AtlasConfig,
    FrameSpec,
    key_checkerboard_to_alpha,
    trim_alpha,
    write_outputs,
)


SPAWN_CLIP = {"spawn": {"loop": False}}

RUST_SPAWN_TIMING_MS = {
    "spawn_00": 130,
    "spawn_01": 120,
    "spawn_02": 120,
    "spawn_03": 110,
    "spawn_04": 110,
    "spawn_05": 110,
    "spawn_06": 120,
    "spawn_07": 120,
    "spawn_08": 120,
    "spawn_09": 125,
    "spawn_10": 130,
    "spawn_11": 150,
    "spawn_12": 140,
    "spawn_13": 180,
    "spawn_14": 150,
    "spawn_15": 130,
    "spawn_16": 140,
    "spawn_17": 160,
    "spawn_18": 180,
}

DUKE_SPAWN_TIMING_MS = {
    "spawn_00": 140,
    "spawn_01": 130,
    "spawn_02": 130,
    "spawn_03": 130,
    "spawn_04": 140,
    "spawn_05": 140,
    "spawn_06": 130,
    "spawn_07": 130,
    "spawn_08": 150,
    "spawn_09": 160,
    "spawn_10": 150,
    "spawn_11": 150,
    "spawn_12": 140,
    "spawn_13": 150,
    "spawn_14": 160,
    "spawn_15": 160,
    "spawn_16": 170,
    "spawn_17": 180,
}


def rust_spawn_frame(
    index: int, crop: tuple[int, int, int, int], pivot: tuple[int, int]
) -> FrameSpec:
    name = f"spawn_{index:02d}"
    return FrameSpec(name, "spawn", crop, pivot, RUST_SPAWN_TIMING_MS[name])


def duke_spawn_frame(
    index: int, crop: tuple[int, int, int, int], pivot: tuple[int, int]
) -> FrameSpec:
    name = f"spawn_{index:02d}"
    return FrameSpec(name, "spawn", crop, pivot, DUKE_SPAWN_TIMING_MS[name])


RUST_FRAMES: list[FrameSpec] = [
    rust_spawn_frame(0, (30, 0, 285, 248), (150, 238)),
    rust_spawn_frame(1, (330, 0, 585, 248), (452, 238)),
    rust_spawn_frame(2, (615, 0, 900, 248), (744, 238)),
    rust_spawn_frame(3, (915, 0, 1195, 248), (1048, 238)),
    rust_spawn_frame(4, (1210, 0, 1568, 248), (1348, 238)),
    rust_spawn_frame(5, (20, 248, 300, 500), (142, 490)),
    rust_spawn_frame(6, (305, 248, 590, 500), (430, 490)),
    rust_spawn_frame(7, (605, 248, 890, 500), (730, 490)),
    rust_spawn_frame(8, (915, 248, 1190, 500), (1040, 490)),
    rust_spawn_frame(9, (1220, 248, 1568, 500), (1345, 490)),
    rust_spawn_frame(10, (0, 500, 285, 746), (134, 744)),
    rust_spawn_frame(11, (300, 500, 585, 746), (444, 744)),
    rust_spawn_frame(12, (600, 500, 850, 746), (728, 744)),
    rust_spawn_frame(13, (900, 500, 1110, 746), (1018, 744)),
    rust_spawn_frame(14, (1110, 500, 1568, 762), (1275, 744)),
    # Source frame 16 is a standalone thrown notebook and is intentionally skipped.
    rust_spawn_frame(15, (300, 748, 595, 1003), (440, 988)),
    rust_spawn_frame(16, (600, 748, 900, 1003), (735, 988)),
    rust_spawn_frame(17, (900, 748, 1200, 1003), (1038, 988)),
    rust_spawn_frame(18, (1200, 748, 1518, 1003), (1340, 988)),
]


DUKE_FRAMES: list[FrameSpec] = [
    duke_spawn_frame(0, (55, 46, 270, 314), (170, 295)),
    duke_spawn_frame(1, (330, 46, 545, 314), (445, 295)),
    duke_spawn_frame(2, (602, 46, 825, 314), (720, 295)),
    duke_spawn_frame(3, (858, 46, 1100, 314), (990, 295)),
    duke_spawn_frame(4, (1122, 46, 1355, 314), (1248, 295)),
    duke_spawn_frame(5, (1380, 46, 1672, 314), (1518, 295)),
    duke_spawn_frame(6, (30, 315, 370, 628), (170, 590)),
    duke_spawn_frame(7, (315, 315, 660, 628), (440, 590)),
    duke_spawn_frame(8, (602, 315, 870, 628), (735, 590)),
    duke_spawn_frame(9, (845, 315, 1165, 628), (1000, 590)),
    duke_spawn_frame(10, (1100, 315, 1405, 628), (1245, 590)),
    duke_spawn_frame(11, (1360, 315, 1672, 628), (1515, 590)),
    duke_spawn_frame(12, (0, 628, 330, 940), (170, 865)),
    duke_spawn_frame(13, (320, 628, 590, 940), (445, 865)),
    duke_spawn_frame(14, (602, 628, 845, 940), (730, 865)),
    duke_spawn_frame(15, (870, 628, 1115, 940), (1000, 865)),
    duke_spawn_frame(16, (1155, 628, 1390, 940), (1270, 865)),
    duke_spawn_frame(17, (1420, 628, 1672, 940), (1530, 865)),
]


def rust_cell_processor(cell: Image.Image, spec: FrameSpec) -> Image.Image:
    """Adds the missing notebook to extracted Rust spawn frame 14."""
    result = cell.copy()

    if spec.name == "spawn_13":
        result = keep_large_components(result, min_area=8000)
        source = Image.open(ROOT / "assets/references/rust-start-anim.png").convert("RGB")
        paste_trimmed_patch(result, source, (1410, 548, 1548, 638), (258, 82), scale=0.48)

    if spec.name == "spawn_17":
        result = remove_components_matching(
            result,
            lambda component: component.left > 280 and component.area < 200,
        )

    return result


def duke_cell_processor(cell: Image.Image, spec: FrameSpec) -> Image.Image:
    """Restores missing table and cup continuity inside extracted Duke cells."""
    result = cell.copy()
    source = Image.open(ROOT / "assets/references/duke-start-anim.png").convert("RGB")

    if spec.name in {
        "spawn_00",
        "spawn_01",
        "spawn_02",
        "spawn_03",
        "spawn_04",
        "spawn_05",
        "spawn_08",
    }:
        paste_trimmed_patch(result, source, (190, 466, 360, 606), (206, 166), scale=0.92)

    if spec.name == "spawn_04":
        paste_trimmed_patch(result, source, (500, 130, 560, 232), (246, 82), scale=0.84)

    if spec.name in {
        "spawn_00",
        "spawn_01",
        "spawn_02",
        "spawn_03",
        "spawn_04",
        "spawn_05",
        "spawn_06",
        "spawn_07",
        "spawn_08",
    }:
        result = remove_components_matching(
            result,
            lambda component: component.left >= 320 and component.area < 6000,
        )

    if spec.name in {"spawn_06", "spawn_07"}:
        result = remove_components_matching(
            result,
            lambda component: component.left >= 300 and component.area < 6000,
        )

    return result


def paste_trimmed_patch(
    target: Image.Image,
    source: Image.Image,
    source_box: tuple[int, int, int, int],
    target_xy: tuple[int, int],
    scale: float,
) -> None:
    patch = key_checkerboard_to_alpha(source.crop(source_box))
    patch, _ = trim_alpha(patch)
    if scale != 1.0:
        patch = patch.resize(
            (round(patch.width * scale), round(patch.height * scale)),
            Image.Resampling.LANCZOS,
        )
    target.alpha_composite(patch, target_xy)


class Component:
    def __init__(self, points: list[tuple[int, int]]):
        self.points = points
        xs = [point[0] for point in points]
        ys = [point[1] for point in points]
        self.left = min(xs)
        self.top = min(ys)
        self.right = max(xs) + 1
        self.bottom = max(ys) + 1
        self.area = len(points)


def keep_large_components(image: Image.Image, min_area: int) -> Image.Image:
    return remove_components_matching(image, lambda component: component.area < min_area)


def remove_components_matching(image: Image.Image, should_remove) -> Image.Image:
    alpha = image.getchannel("A")
    width, height = image.size
    visited = bytearray(width * height)
    source = image.load()
    cleaned = Image.new("RGBA", image.size, (0, 0, 0, 0))
    target = cleaned.load()

    for start_y in range(height):
        for start_x in range(width):
            start_index = start_y * width + start_x
            if visited[start_index] or alpha.getpixel((start_x, start_y)) == 0:
                continue

            stack = [(start_x, start_y)]
            visited[start_index] = 1
            points: list[tuple[int, int]] = []
            while stack:
                x, y = stack.pop()
                points.append((x, y))
                for ny in range(y - 1, y + 2):
                    for nx in range(x - 1, x + 2):
                        if nx < 0 or ny < 0 or nx >= width or ny >= height:
                            continue
                        index = ny * width + nx
                        if visited[index] or alpha.getpixel((nx, ny)) == 0:
                            continue
                        visited[index] = 1
                        stack.append((nx, ny))

            component = Component(points)
            if should_remove(component):
                continue

            for x, y in points:
                target[x, y] = source[x, y]

    return cleaned


RUST_CONFIG = AtlasConfig(
    source_path=ROOT / "assets/references/rust-start-anim.png",
    atlas_path=ROOT / "assets/placeholder/rust-start-atlas.png",
    manifest_path=ROOT / "assets/placeholder/rust-start.sprite.json",
    projectile_path=ROOT / "tmp/art/rust-start-unused-projectile.png",
    preview_path=ROOT / "tmp/art/rust-start-atlas-preview.png",
    frames=RUST_FRAMES,
    clips=SPAWN_CLIP,
    cell_width=512,
    cell_height=320,
    columns=5,
    target_pivot=(160, 292),
    scale=0.92,
    projectile_frame_name="",
    remove_stray_components=False,
    cell_processor=rust_cell_processor,
)


DUKE_CONFIG = AtlasConfig(
    source_path=ROOT / "assets/references/duke-start-anim.png",
    atlas_path=ROOT / "assets/placeholder/duke-start-atlas.png",
    manifest_path=ROOT / "assets/placeholder/duke-start.sprite.json",
    projectile_path=ROOT / "tmp/art/duke-start-unused-projectile.png",
    preview_path=ROOT / "tmp/art/duke-start-atlas-preview.png",
    frames=DUKE_FRAMES,
    clips=SPAWN_CLIP,
    cell_width=512,
    cell_height=320,
    columns=6,
    target_pivot=(160, 292),
    scale=0.92,
    projectile_frame_name="",
    remove_stray_components=False,
    clean_white_body=True,
    cell_processor=duke_cell_processor,
)


if __name__ == "__main__":
    write_outputs(RUST_CONFIG)
    write_outputs(DUKE_CONFIG)
