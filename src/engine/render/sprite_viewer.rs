//! Draws the standalone sprite combat viewer.
//!
//! System: Raylib render boundary. This module visualizes sprite manifests and
//! viewer state without owning sprite metadata or editing pipeline rules.

use raylib::prelude::*;

use crate::{
    config::{FLOOR_Y, WINDOW_HEIGHT, WINDOW_WIDTH},
    engine::sprites::{SpriteFrame, SpriteRect},
    scenes::sprite_viewer::{SpriteViewer, ViewerRect},
};

use super::{BACKGROUND, PANEL, PANEL_BORDER, UI_MUTED, UI_TEXT};

const GRID_MAJOR: i32 = 80;
const GRID_MINOR: i32 = 20;
const GRID_MAJOR_COLOR: Color = Color::new(54, 60, 72, 210);
const GRID_MINOR_COLOR: Color = Color::new(36, 41, 52, 190);
const FRAME_COLOR: Color = Color::new(255, 210, 74, 255);
const TRIM_COLOR: Color = Color::new(105, 240, 174, 255);
const SOURCE_COLOR: Color = Color::new(86, 156, 255, 255);
const PIVOT_COLOR: Color = Color::new(255, 82, 82, 255);
const DUMMY_COLOR: Color = Color::new(255, 178, 104, 174);
const DUMMY_PIVOT_COLOR: Color = Color::new(255, 178, 104, 255);

/// Draws the sprite viewer scene.
pub fn draw_sprite_viewer(
    draw: &mut RaylibDrawHandle<'_>,
    viewer: &SpriteViewer,
    texture: Option<&Texture2D>,
) {
    draw.clear_background(BACKGROUND);
    if viewer.show_grid() {
        draw_viewer_grid(draw);
    }

    draw.draw_line(0, FLOOR_Y as i32, WINDOW_WIDTH, FLOOR_Y as i32, UI_MUTED);

    if let Some(texture) = texture {
        if viewer.show_dummy() {
            draw_sprite_instance(
                draw,
                viewer,
                texture,
                viewer.dummy_screen_rect(),
                true,
                DUMMY_COLOR,
            );
        }
        draw_sprite_instance(
            draw,
            viewer,
            texture,
            viewer.sprite_screen_rect(),
            false,
            Color::WHITE,
        );
    } else {
        draw.draw_text(
            "Atlas texture not loaded",
            360,
            (FLOOR_Y - 120.0) as i32,
            22,
            FRAME_COLOR,
        );
    }

    if viewer.show_bounds() {
        if viewer.show_dummy() {
            draw_frame_guides(draw, viewer, viewer.dummy_screen_rect(), true);
        }
        draw_frame_guides(draw, viewer, viewer.sprite_screen_rect(), false);
    }

    if viewer.show_dummy() {
        draw_dummy_distance(draw, viewer);
    }

    if viewer.show_pivot() {
        if viewer.show_dummy() {
            draw_pivot_at(draw, viewer.dummy_anchor(), DUMMY_PIVOT_COLOR);
        }
        draw_pivot_at(draw, viewer.anchor(), PIVOT_COLOR);
    }

    draw_info_panel(draw, viewer);
}

/// Draws a load error for the sprite viewer startup path.
pub fn draw_sprite_viewer_error(draw: &mut RaylibDrawHandle<'_>, message: &str) {
    draw.clear_background(BACKGROUND);
    draw_viewer_grid(draw);
    draw.draw_rectangle(80, 150, WINDOW_WIDTH - 160, 180, PANEL);
    draw.draw_rectangle_lines(80, 150, WINDOW_WIDTH - 160, 180, PANEL_BORDER);
    draw.draw_text("Sprite Viewer", 112, 184, 28, UI_TEXT);
    draw.draw_text(
        "Nao foi possivel abrir o manifesto.",
        112,
        224,
        18,
        FRAME_COLOR,
    );
    draw_wrapped_text(draw, message, 112, 258, WINDOW_WIDTH - 224, 16, UI_MUTED);
}

fn draw_sprite_instance(
    draw: &mut RaylibDrawHandle<'_>,
    viewer: &SpriteViewer,
    texture: &Texture2D,
    screen: ViewerRect,
    mirrored: bool,
    tint: Color,
) {
    let frame = viewer.current_frame();
    let mut source = Rectangle::new(
        frame.frame.x as f32,
        frame.frame.y as f32,
        frame.frame.w as f32,
        frame.frame.h as f32,
    );
    if mirrored {
        source.x += source.width;
        source.width = -source.width;
    }
    let dest = Rectangle::new(screen.x, screen.y, screen.width, screen.height);

    draw.draw_texture_pro(texture, source, dest, Vector2::new(0.0, 0.0), 0.0, tint);
}

fn draw_frame_guides(
    draw: &mut RaylibDrawHandle<'_>,
    viewer: &SpriteViewer,
    screen: ViewerRect,
    muted: bool,
) {
    let frame = viewer.current_frame();
    draw_outline(
        draw,
        screen,
        if muted {
            Color::new(255, 210, 74, 150)
        } else {
            FRAME_COLOR
        },
        2.0,
    );

    if let Some(trimmed_bounds) = frame.trimmed_bounds {
        draw_relative_guide(
            draw,
            screen,
            frame,
            trimmed_bounds,
            if muted {
                Color::new(105, 240, 174, 130)
            } else {
                TRIM_COLOR
            },
        );
    }

    if let Some(source_crop) = frame.source_crop {
        draw_relative_guide(
            draw,
            screen,
            frame,
            source_crop,
            if muted {
                Color::new(86, 156, 255, 130)
            } else {
                SOURCE_COLOR
            },
        );
    }
}

fn draw_relative_guide(
    draw: &mut RaylibDrawHandle<'_>,
    screen: ViewerRect,
    frame: &SpriteFrame,
    guide: SpriteRect,
    color: Color,
) {
    let scale_x = screen.width / frame.frame.w as f32;
    let scale_y = screen.height / frame.frame.h as f32;
    let rect = ViewerRect {
        x: screen.x + guide.x as f32 * scale_x,
        y: screen.y + guide.y as f32 * scale_y,
        width: guide.w as f32 * scale_x,
        height: guide.h as f32 * scale_y,
    };
    draw_outline(draw, rect, color, 1.0);
}

fn draw_dummy_distance(draw: &mut RaylibDrawHandle<'_>, viewer: &SpriteViewer) {
    let anchor = viewer.anchor();
    let dummy = viewer.dummy_anchor();
    let y = (FLOOR_Y + 24.0) as i32;
    draw.draw_line_ex(
        Vector2::new(anchor.x, y as f32),
        Vector2::new(dummy.x, y as f32),
        2.0,
        UI_MUTED,
    );
    draw.draw_text(
        &format!("{:.0}px", viewer.dummy_distance()),
        ((anchor.x + dummy.x) * 0.5).round() as i32 - 24,
        y + 8,
        15,
        UI_MUTED,
    );
}

fn draw_pivot_at(
    draw: &mut RaylibDrawHandle<'_>,
    anchor: crate::scenes::sprite_viewer::ViewerPoint,
    color: Color,
) {
    draw.draw_line_ex(
        Vector2::new(anchor.x - 18.0, anchor.y),
        Vector2::new(anchor.x + 18.0, anchor.y),
        3.0,
        color,
    );
    draw.draw_line_ex(
        Vector2::new(anchor.x, anchor.y - 18.0),
        Vector2::new(anchor.x, anchor.y + 18.0),
        3.0,
        color,
    );
    draw.draw_circle(anchor.x as i32, anchor.y as i32, 4.0, color);
}

fn draw_viewer_grid(draw: &mut RaylibDrawHandle<'_>) {
    for x in (0..=WINDOW_WIDTH).step_by(GRID_MINOR as usize) {
        let color = if x % GRID_MAJOR == 0 {
            GRID_MAJOR_COLOR
        } else {
            GRID_MINOR_COLOR
        };
        draw.draw_line(x, 0, x, WINDOW_HEIGHT, color);
    }
    for y in (0..=WINDOW_HEIGHT).step_by(GRID_MINOR as usize) {
        let color = if y % GRID_MAJOR == 0 {
            GRID_MAJOR_COLOR
        } else {
            GRID_MINOR_COLOR
        };
        draw.draw_line(0, y, WINDOW_WIDTH, y, color);
    }
}

fn draw_info_panel(draw: &mut RaylibDrawHandle<'_>, viewer: &SpriteViewer) {
    let panel_x = 16;
    let panel_y = 16;
    let panel_width = WINDOW_WIDTH - 32;
    let panel_height = 126;
    draw.draw_rectangle(panel_x, panel_y, panel_width, panel_height, PANEL);
    draw.draw_rectangle_lines(panel_x, panel_y, panel_width, panel_height, PANEL_BORDER);

    let (clip_index, clip_count) = viewer.clip_position();
    let (frame_index, frame_count) = viewer.frame_position();
    let frame = viewer.current_frame();
    let anchor = viewer.anchor();
    let playback = if viewer.playing() {
        "playing"
    } else {
        "paused"
    };
    let manifest = viewer.manifest_path().display().to_string();
    let image = viewer.image_path().display().to_string();

    draw.draw_text(
        "Sprite Combat Viewer",
        panel_x + 16,
        panel_y + 14,
        22,
        UI_TEXT,
    );
    draw.draw_text(
        &format!(
            "clip {}/{}: {} | frame {}/{}: {} | {}",
            clip_index + 1,
            clip_count,
            viewer.current_clip_name(),
            frame_index + 1,
            frame_count,
            frame.name,
            playback,
        ),
        panel_x + 16,
        panel_y + 44,
        16,
        UI_TEXT,
    );
    draw.draw_text(
        &format!(
            "anchor {:.1},{:.1} | dummy {:.1}px | pivot {},{} | scale {:.2} | zoom {:.2}",
            anchor.x,
            anchor.y,
            viewer.dummy_distance(),
            frame.pivot.x,
            frame.pivot.y,
            viewer.scale(),
            viewer.zoom(),
        ),
        panel_x + 16,
        panel_y + 66,
        16,
        UI_MUTED,
    );
    draw.draw_text(
        "mouse arrasta | Tab clip | ./, frame | Wheel zoom | 0 zoom | O dummy | F5 reload | F12 shot",
        panel_x + 16,
        panel_y + 92,
        15,
        UI_MUTED,
    );

    draw.draw_text(
        &truncate_middle(&manifest, 74),
        panel_x + 430,
        panel_y + 16,
        14,
        UI_MUTED,
    );
    draw.draw_text(
        &truncate_middle(&image, 74),
        panel_x + 430,
        panel_y + 36,
        14,
        UI_MUTED,
    );

    if let Some(error) = viewer.texture_error() {
        draw.draw_text(
            &truncate_middle(error, 100),
            panel_x + 430,
            panel_y + 66,
            14,
            FRAME_COLOR,
        );
    } else if let Some(message) = viewer.status_message() {
        draw.draw_text(
            &truncate_middle(message, 100),
            panel_x + 430,
            panel_y + 66,
            14,
            TRIM_COLOR,
        );
    }
}

fn draw_outline(draw: &mut RaylibDrawHandle<'_>, rect: ViewerRect, color: Color, thickness: f32) {
    draw.draw_rectangle_lines_ex(
        Rectangle::new(rect.x, rect.y, rect.width, rect.height),
        thickness,
        color,
    );
}

fn draw_wrapped_text(
    draw: &mut RaylibDrawHandle<'_>,
    text: &str,
    x: i32,
    y: i32,
    max_width: i32,
    font_size: i32,
    color: Color,
) {
    let mut line = String::new();
    let mut offset_y = 0;
    for word in text.split_whitespace() {
        let candidate = if line.is_empty() {
            word.to_string()
        } else {
            format!("{line} {word}")
        };
        if draw.measure_text(&candidate, font_size) > max_width && !line.is_empty() {
            draw.draw_text(&line, x, y + offset_y, font_size, color);
            line.clear();
            line.push_str(word);
            offset_y += font_size + 6;
        } else {
            line = candidate;
        }
    }

    if !line.is_empty() {
        draw.draw_text(&line, x, y + offset_y, font_size, color);
    }
}

fn truncate_middle(text: &str, max_chars: usize) -> String {
    if text.chars().count() <= max_chars {
        return text.to_string();
    }
    let keep = max_chars.saturating_sub(3) / 2;
    let start = text.chars().take(keep).collect::<String>();
    let end = text
        .chars()
        .rev()
        .take(keep)
        .collect::<String>()
        .chars()
        .rev()
        .collect::<String>();
    format!("{start}...{end}")
}
