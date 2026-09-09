//! Presents the Linker's roster with current character art and animated previews.
//!
//! Portraits crop the same idle frames used in matches so selection never shows
//! a different design. Arena and control choices share pointer geometry.

use super::presentation::{self, CYAN, GOLD, MUTED, centered, label, panel};
use super::{DrawTarget, GameAssets, character_visuals};
use crate::characters::CharacterId;
use crate::engine::sprites::{SpriteFrame, SpriteManifest, SpriteRect, frame_for_clip_at};
use crate::scenes::{
    character_select::{CharacterSelect, PUBLIC_ROSTER, RANDOM_SLOT},
    preferences::PlayMode,
};
use crate::ui::roster_layout::{self as layout, Bounds};
use raylib::prelude::*;

fn rect(b: Bounds) -> Rectangle {
    Rectangle::new(b.x as f32, b.y as f32, b.w as f32, b.h as f32)
}

pub fn draw_character_select(
    draw: &mut impl DrawTarget,
    selection: &CharacterSelect,
    assets: &GameAssets,
) {
    let time = selection.elapsed;
    presentation::circuit_background(draw, time);
    label(draw, assets, "BORROW FIGHTERS", 32, 23, 17.0, Color::WHITE);
    label(
        draw,
        assets,
        "LINKER  /  MATCH ASSEMBLY",
        32,
        46,
        12.0,
        MUTED,
    );
    label(
        draw,
        assets,
        "05 ENTIDADES DISPONÍVEIS",
        1013,
        28,
        13.0,
        CYAN,
    );
    label(
        draw,
        assets,
        "LOCAL SESSION  //  READY TO COMPILE",
        975,
        49,
        10.0,
        MUTED,
    );
    centered(
        draw,
        assets,
        "ESCOLHA SEU PERSONAGEM",
        640,
        78,
        34.0,
        Color::WHITE,
    );
    centered(
        draw,
        assets,
        "DUAS LINGUAGENS. UM CONFLITO.",
        640,
        117,
        12.0,
        MUTED,
    );
    for owner in 0..2 {
        draw_preview(draw, selection, owner, assets);
    }
    centered(draw, assets, "VS", 640, 154, 38.0, Color::WHITE);
    let status = if selection.both_ready() {
        "BUILD CONCLUÍDO  //  PRONTOS PARA LUTAR"
    } else if selection.active() == 0 {
        "P1  //  ESCOLHA E CONFIRME"
    } else {
        "P2  //  ESCOLHA E CONFIRME"
    };
    centered(
        draw,
        assets,
        status,
        640,
        201,
        12.0,
        if selection.active() == 0 { CYAN } else { GOLD },
    );
    for index in 0..8 {
        draw_cell(draw, selection, index, assets);
    }

    panel(draw, rect(layout::MODE), CYAN);
    let mode = match selection.mode {
        PlayMode::AgainstCpu => "VOCÊ × CPU",
        PlayMode::LocalDuel => "DUELO LOCAL",
        PlayMode::WatchDemo => "CPU × CPU",
    };
    label(draw, assets, "MODO", 366, 545, 12.0, MUTED);
    centered(draw, assets, mode, 650, 541, 21.0, Color::WHITE);
    label(draw, assets, "TAB / SELECT  >", 807, 545, 12.0, CYAN);
    panel(draw, rect(layout::ARENA), GOLD);
    if let Some(texture) = assets.arenas.get(selection.arena) {
        draw.draw_texture_pro(
            texture,
            Rectangle::new(0.0, 0.0, texture.width() as f32, texture.height() as f32),
            Rectangle::new(400.0, 585.0, 80.0, 42.0),
            Vector2::zero(),
            0.0,
            Color::new(255, 255, 255, 170),
        );
    }
    label(draw, assets, "<", 370, 592, 22.0, GOLD);
    label(
        draw,
        assets,
        selection.arena.label(),
        500,
        587,
        18.0,
        Color::WHITE,
    );
    label(
        draw,
        assets,
        selection.arena.location(),
        501,
        609,
        11.0,
        MUTED,
    );
    label(draw, assets, ">", 904, 592, 22.0, GOLD);
    label(
        draw,
        assets,
        "Q/E · LB/RB (P1)  arena",
        968,
        657,
        12.0,
        MUTED,
    );
    label(
        draw,
        assets,
        "TAB · SELECT/BACK (P1)  modo",
        968,
        677,
        12.0,
        MUTED,
    );
    panel(
        draw,
        rect(layout::LAUNCH),
        if selection.both_ready() { CYAN } else { MUTED },
    );
    if selection.both_ready() {
        draw.draw_rectangle_gradient_h(
            442,
            648,
            396,
            38,
            Color::new(14, 114, 118, 150),
            Color::new(33, 55, 65, 70),
        );
    }
    centered(
        draw,
        assets,
        if selection.both_ready() {
            "LUTAR   //   ENTER / START"
        } else {
            "CONFIRME OS DOIS PERSONAGENS"
        },
        640,
        656,
        17.0,
        if selection.both_ready() {
            Color::WHITE
        } else {
            MUTED
        },
    );
    label(draw, assets, "<  ESC / B   VOLTAR", 36, 659, 14.0, MUTED);
    centered(
        draw,
        assets,
        if selection.mode == PlayMode::LocalDuel {
            "P1  WASD + F    •    P2  SETAS + ENTER    •    D-PAD + A / MOUSE    •    ESPAÇO  lado ativo"
        } else {
            "WASD / SETAS / D-PAD  navegar    •    ENTER / A  confirmar    •    MOUSE  escolher"
        },
        640,
        702,
        11.0,
        MUTED,
    );
    presentation::transition(draw, time);
}

fn draw_preview(
    draw: &mut impl DrawTarget,
    selection: &CharacterSelect,
    owner: usize,
    assets: &GameAssets,
) {
    let bounds = layout::preview(owner);
    let accent = if owner == 0 { CYAN } else { GOLD };
    let cx = bounds.x + bounds.w / 2;
    panel(draw, rect(bounds), accent);
    draw.draw_rectangle_gradient_v(
        bounds.x + 1,
        bounds.y + 1,
        bounds.w - 2,
        bounds.h - 2,
        Color::new(accent.r, accent.g, accent.b, 15),
        Color::new(3, 8, 17, 20),
    );
    let id = selection.preview(owner);
    let future = selection.cursor(owner) > RANDOM_SLOT && !selection.ready(owner);
    let (file, role, signature) = if future {
        (
            "?.module",
            "NOVOS PERSONAGENS EM BREVE",
            "AINDA NÃO DISPONÍVEL",
        )
    } else {
        identity(id)
    };
    label(
        draw,
        assets,
        if owner == 0 {
            "01 / PLAYER ONE"
        } else {
            "02 / PLAYER TWO"
        },
        bounds.x + 17,
        bounds.y + 16,
        12.0,
        accent,
    );
    centered(draw, assets, file, cx, bounds.y + 41, 30.0, Color::WHITE);
    centered(draw, assets, role, cx, bounds.y + 79, 12.0, MUTED);
    let pulse = (selection.elapsed * 2.5).sin();
    draw.draw_ellipse(
        cx,
        519,
        105.0,
        15.0,
        Color::new(accent.r, accent.g, accent.b, 18),
    );
    draw.draw_ellipse_lines(
        cx,
        519,
        111.0 + pulse * 3.0,
        18.0,
        Color::new(accent.r, accent.g, accent.b, 80),
    );
    for i in 0..5 {
        let y = 264 + i * 44;
        draw.draw_line(
            bounds.x + 15,
            y,
            bounds.x + 25,
            y,
            Color::new(accent.r, accent.g, accent.b, 65),
        );
        draw.draw_line(
            bounds.x + bounds.w - 25,
            y,
            bounds.x + bounds.w - 15,
            y,
            Color::new(accent.r, accent.g, accent.b, 65),
        );
    }
    if future {
        centered(draw, assets, "?", cx, 323, 100.0, MUTED);
    } else {
        draw_character_art(
            draw,
            assets,
            id,
            Rectangle::new((bounds.x + 26) as f32, 247.0, 246.0, 272.0),
            selection.elapsed,
            CharacterArtView::Standing {
                mirrored: owner == 1,
                focus_time: selection.focus_elapsed[owner],
            },
        );
    }
    if selection.focus_elapsed[owner] < 0.35 {
        let y = 241 + (selection.focus_elapsed[owner] / 0.35 * 280.0) as i32;
        draw.draw_rectangle(
            bounds.x + 12,
            y,
            bounds.w - 24,
            2,
            Color::new(accent.r, accent.g, accent.b, 105),
        );
    }
    centered(
        draw,
        assets,
        if selection.ready(owner) {
            "OK  /  CONFIRMADO"
        } else {
            signature
        },
        cx,
        545,
        12.0,
        if selection.ready(owner) {
            accent
        } else {
            MUTED
        },
    );
    label(
        draw,
        assets,
        if owner == 0 {
            "P1   WASD + F   /   CONTROLE 1"
        } else {
            "P2   SETAS + ENTER   /   CONTROLE 2"
        },
        bounds.x + 6,
        586,
        12.0,
        accent,
    );
    if selection.mode != PlayMode::LocalDuel {
        label(
            draw,
            assets,
            "Escolha os dois lados com Enter / A",
            bounds.x + 6,
            608,
            11.0,
            MUTED,
        );
    }
}

fn draw_cell(
    draw: &mut impl DrawTarget,
    selection: &CharacterSelect,
    index: usize,
    assets: &GameAssets,
) {
    let b = layout::cell(index);
    let owners = [selection.cursor(0) == index, selection.cursor(1) == index];
    let accent = if owners[0] {
        CYAN
    } else if owners[1] {
        GOLD
    } else {
        Color::new(70, 92, 111, 255)
    };
    panel(draw, rect(b), accent);
    if let Some(&character) = PUBLIC_ROSTER.get(index) {
        draw.draw_rectangle_gradient_v(
            b.x + 2,
            b.y + 2,
            b.w - 4,
            111,
            Color::new(36, 58, 80, 150),
            Color::new(9, 21, 33, 255),
        );
        draw_character_art(
            draw,
            assets,
            character,
            Rectangle::new((b.x + 3) as f32, (b.y + 3) as f32, (b.w - 6) as f32, 109.0),
            0.0,
            CharacterArtView::Portrait,
        );
        draw.draw_rectangle(b.x + 1, b.y + 111, b.w - 2, 28, Color::new(7, 15, 25, 245));
        centered(
            draw,
            assets,
            identity(character).0,
            b.x + b.w / 2,
            b.y + 117,
            15.0,
            Color::WHITE,
        );
    } else {
        centered(
            draw,
            assets,
            "?",
            b.x + b.w / 2,
            b.y + 30,
            52.0,
            if index == RANDOM_SLOT {
                Color::WHITE
            } else {
                Color::new(71, 88, 103, 255)
            },
        );
        centered(
            draw,
            assets,
            if index == RANDOM_SLOT {
                "RANDOM"
            } else {
                "EM BREVE"
            },
            b.x + b.w / 2,
            b.y + 105,
            14.0,
            if index == RANDOM_SLOT { CYAN } else { MUTED },
        );
        if index != RANDOM_SLOT {
            centered(
                draw,
                assets,
                "module.pending",
                b.x + b.w / 2,
                b.y + 123,
                9.0,
                MUTED,
            );
        }
    }
    for (owner, selected) in owners.into_iter().enumerate() {
        if selected {
            let color = if owner == 0 { CYAN } else { GOLD };
            let y = if owner == 0 { b.y - 1 } else { b.y + b.h - 3 };
            draw.draw_rectangle(b.x, y, b.w, 3, color);
            let x = b.x + if owner == 0 { 5 } else { b.w - 37 };
            draw.draw_rectangle(x, b.y + 6, 31, 19, color);
            label(
                draw,
                assets,
                if owner == 0 { "P1" } else { "P2" },
                x + 6,
                b.y + 8,
                13.0,
                Color::new(5, 15, 24, 255),
            );
        }
    }
}

enum CharacterArtView {
    Portrait,
    Standing { mirrored: bool, focus_time: f32 },
}

fn draw_character_art(
    draw: &mut impl DrawTarget,
    assets: &GameAssets,
    id: CharacterId,
    dest: Rectangle,
    time: f32,
    view: CharacterArtView,
) {
    let (portrait, mirrored, focus_time) = match view {
        CharacterArtView::Portrait => (true, false, 1.0),
        CharacterArtView::Standing {
            mirrored,
            focus_time,
        } => (false, mirrored, focus_time),
    };
    let Some(atlas) = character_visuals(id, assets).fight_atlas else {
        return;
    };
    let Some(frame) = frame_for_clip_at(&atlas.manifest, "idle", time) else {
        return;
    };
    let Some(texture) = atlas.texture_for_frame(frame) else {
        return;
    };
    let bounds = frame_bounds(frame);
    let mut source = Rectangle::new(
        (frame.frame.x + bounds.x) as f32,
        (frame.frame.y + bounds.y) as f32,
        bounds.w as f32,
        bounds.h as f32,
    );
    let mut target = dest;
    if portrait {
        // Duke's face sits below his steam plume; frame the nose, not the plume.
        if id == CharacterId::Duke {
            source.y += source.height * 0.22;
        }
        source.height *= 0.47;
        let aspect = dest.width / dest.height;
        let width = source.height * aspect;
        source.x += (source.width - width) * 0.5;
        source.width = width;
    } else {
        let Some(placement) = standing_preview_target(&atlas.manifest, frame, dest, mirrored)
        else {
            return;
        };
        target = placement;
        let slide = (1.0 - (focus_time / 0.22).clamp(0.0, 1.0)).powi(3) * 25.0;
        target.x += if mirrored { slide } else { -slide };
    }
    if mirrored {
        source.width = -source.width;
    }
    draw.draw_texture_pro(texture, source, target, Vector2::zero(), 0.0, Color::WHITE);
}

fn frame_bounds(frame: &SpriteFrame) -> SpriteRect {
    frame.trimmed_bounds.unwrap_or(SpriteRect {
        x: 0,
        y: 0,
        w: frame.frame.w,
        h: frame.frame.h,
    })
}

fn bounds_from_pivot(frame: &SpriteFrame) -> Rectangle {
    let bounds = frame_bounds(frame);
    Rectangle::new(
        (bounds.x - frame.pivot.x) as f32,
        (bounds.y - frame.pivot.y) as f32,
        bounds.w as f32,
        bounds.h as f32,
    )
}

// Fit a fixed envelope of the idle clip in pivot space. Poses may breathe or
// lean without adding a per-frame zoom or moving their authored support point.
fn standing_preview_target(
    manifest: &SpriteManifest,
    frame: &SpriteFrame,
    dest: Rectangle,
    mirrored: bool,
) -> Option<Rectangle> {
    let mut poses = manifest
        .clip_named("idle")?
        .frames
        .iter()
        .filter_map(|name| manifest.frame_named(name))
        .map(bounds_from_pivot);
    let first = poses.next()?;
    let (mut left, mut top) = (first.x, first.y);
    let (mut right, mut bottom) = (left + first.width, top + first.height);
    for pose in poses {
        left = left.min(pose.x);
        top = top.min(pose.y);
        right = right.max(pose.x + pose.width);
        bottom = bottom.max(pose.y + pose.height);
    }
    let scale = (dest.width / (right - left)).min(dest.height / (bottom - top));
    let center = (left + right) * 0.5;
    let anchor_x = dest.x + dest.width * 0.5 + if mirrored { center } else { -center } * scale;
    let anchor_y = dest.y + dest.height - bottom * scale;
    let pose = bounds_from_pivot(frame);
    let offset_x = if mirrored {
        -pose.x - pose.width
    } else {
        pose.x
    };
    Some(Rectangle::new(
        anchor_x + offset_x * scale,
        anchor_y + pose.y * scale,
        pose.width * scale,
        pose.height * scale,
    ))
}

fn identity(id: CharacterId) -> (&'static str, &'static str, &'static str) {
    match id {
        CharacterId::Rust => ("Rust.rs", "OWNERSHIP / CONTROLE", "EMPRÉSTIMO COM JUROS"),
        CharacterId::Duke => (
            "Duke.java",
            "ENTERPRISE / PRESSÃO",
            "WRITE ONCE. FIGHT ANYWHERE.",
        ),
        CharacterId::C => ("Old.c", "LEGADO / PRECISÃO", "O KERNEL NUNCA ESQUECE"),
        CharacterId::Python => ("Python.py", "DADOS / AGILIDADE", "IMPORT VICTORY"),
        CharacterId::Cpp => ("C++.cpp", "TEMPLATES / PUNIÇÃO", "UNDEFINED BEHAVIOR"),
        CharacterId::Go => ("Go.go", "GOROUTINES / VELOCIDADE", "GO FIGHT()"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn public_idle_previews_keep_scale_and_pivot_and_fit_in_both_orientations() {
        let manifests = [
            include_str!("../../../assets/candidates/rust/rust-fighter.sprite.json"),
            include_str!("../../../assets/candidates/duke/duke-fighter.sprite.json"),
            include_str!("../../../assets/candidates/c/c-fighter.sprite.json"),
            include_str!("../../../assets/candidates/python/python-fighter.sprite.json"),
            include_str!("../../../assets/candidates/cpp/cpp-fighter.sprite.json"),
        ];
        let dest = Rectangle::new(56.0, 247.0, 246.0, 272.0);
        for json in manifests {
            let manifest: SpriteManifest = serde_json::from_str(json).unwrap();
            for mirrored in [false, true] {
                let mut reference = None;
                for name in &manifest.clip_named("idle").unwrap().frames {
                    let frame = manifest.frame_named(name).unwrap();
                    let target = standing_preview_target(&manifest, frame, dest, mirrored).unwrap();
                    let bounds = frame_bounds(frame);
                    let scale = target.width / bounds.w as f32;
                    let pivot_x = if mirrored {
                        bounds.x + bounds.w - frame.pivot.x
                    } else {
                        frame.pivot.x - bounds.x
                    } as f32;
                    let pivot = (
                        target.x + pivot_x * scale,
                        target.y + (frame.pivot.y - bounds.y) as f32 * scale,
                    );
                    let (expected_scale, expected_pivot) = *reference.get_or_insert((scale, pivot));
                    assert!((scale - expected_scale).abs() < 0.0001);
                    assert!((pivot.0 - expected_pivot.0).abs() < 0.001);
                    assert!((pivot.1 - expected_pivot.1).abs() < 0.001);
                    assert!(target.x >= dest.x - 0.001 && target.y >= dest.y - 0.001);
                    assert!(target.x + target.width <= dest.x + dest.width + 0.001);
                    assert!(target.y + target.height <= dest.y + dest.height + 0.001);
                    let opposite =
                        standing_preview_target(&manifest, frame, dest, !mirrored).unwrap();
                    assert!(
                        (target.x + target.width + opposite.x - 2.0 * dest.x - dest.width).abs()
                            < 0.001
                    );
                    assert!((target.y - opposite.y).abs() < 0.001);
                }
            }
        }
    }
}
