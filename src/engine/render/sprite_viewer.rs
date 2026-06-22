//! Draws the standalone sprite combat viewer.
//!
//! System: Raylib render boundary. This module visualizes sprite manifests and
//! viewer state without owning sprite metadata or editing pipeline rules.

use raylib::prelude::*;

use crate::{
    config::{FLOOR_Y, WINDOW_HEIGHT, WINDOW_WIDTH},
    engine::sprites::{SpriteFrame, SpriteRect},
    math::rect::Rect,
    scenes::sprite_viewer::{
        SpriteCombatOverlay, SpriteFrameCombatBoxOverlay, SpriteFrameCombatOverlay,
        SpriteFrameCursor, SpriteTimelinePhase, SpriteViewer, ViewerPoint, ViewerRect,
    },
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
const CURSOR_COLOR: Color = Color::new(255, 255, 255, 230);
const DUMMY_COLOR: Color = Color::new(255, 178, 104, 174);
const DUMMY_PIVOT_COLOR: Color = Color::new(255, 178, 104, 255);
const COMBAT_BODY: Color = Color::new(218, 112, 214, 210);
const COMBAT_HURTBOX: Color = Color::new(105, 240, 174, 230);
const COMBAT_HITBOX: Color = Color::new(255, 82, 82, 235);
const COMBAT_HITBOX_FILL: Color = Color::new(255, 82, 82, 70);
const COMBAT_PROJECTILE: Color = Color::new(80, 220, 255, 235);
const COMBAT_PROJECTILE_FILL: Color = Color::new(80, 220, 255, 70);
const PROJECTILE_TRAJECTORY: Color = Color::new(80, 220, 255, 150);
const PROJECTILE_TRAJECTORY_FILL: Color = Color::new(80, 220, 255, 28);
const FRAME_DATA_HURTBOX: Color = Color::new(84, 255, 170, 255);
const FRAME_DATA_HURTBOX_FILL: Color = Color::new(84, 255, 170, 38);
const FRAME_DATA_HITBOX: Color = Color::new(255, 96, 96, 255);
const FRAME_DATA_HITBOX_FILL: Color = Color::new(255, 96, 96, 82);
const TIMELINE_STARTUP: Color = Color::new(255, 210, 74, 230);
const TIMELINE_ACTIVE: Color = Color::new(255, 82, 82, 235);
const TIMELINE_RECOVERY: Color = Color::new(116, 151, 255, 220);
const TIMELINE_INACTIVE: Color = Color::new(68, 76, 92, 220);

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
    if let Some(cursor) = viewer.frame_cursor() {
        draw_frame_cursor(draw, cursor);
    }

    if viewer.show_dummy() {
        draw_dummy_distance(draw, viewer);
    }

    if let Some(trajectory) = viewer.projectile_trajectory() {
        draw_projectile_trajectory(draw, &trajectory);
    }
    if let Some(overlay) = viewer.combat_overlay() {
        draw_combat_overlay(draw, overlay);
    }
    if let Some(overlay) = viewer.frame_combat_overlay() {
        draw_frame_combat_overlay(draw, &overlay);
    }

    if viewer.show_pivot() {
        if viewer.show_dummy() {
            draw_pivot_at(draw, viewer.dummy_anchor(), DUMMY_PIVOT_COLOR);
        }
        draw_pivot_at(draw, viewer.anchor(), PIVOT_COLOR);
    }

    draw_timeline(draw, viewer);
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

fn draw_combat_overlay(draw: &mut RaylibDrawHandle<'_>, overlay: SpriteCombatOverlay) {
    draw_combat_rect(draw, overlay.body, COMBAT_BODY, None);
    draw_combat_rect(draw, overlay.hurtboxes.head, COMBAT_HURTBOX, None);
    draw_combat_rect(draw, overlay.hurtboxes.torso, COMBAT_HURTBOX, None);
    draw_combat_rect(draw, overlay.hurtboxes.legs, COMBAT_HURTBOX, None);

    if let Some(hitbox) = overlay.hitbox {
        draw_combat_rect(draw, hitbox, COMBAT_HITBOX, Some(COMBAT_HITBOX_FILL));
    }

    if let Some(projectile) = overlay.projectile {
        draw_combat_rect(
            draw,
            projectile,
            COMBAT_PROJECTILE,
            Some(COMBAT_PROJECTILE_FILL),
        );
    }

    if let Some(origin) = overlay.projectile_origin {
        draw_projectile_origin(draw, origin, COMBAT_PROJECTILE);
    }
}

fn draw_combat_rect(
    draw: &mut RaylibDrawHandle<'_>,
    rect: Rect,
    outline: Color,
    fill: Option<Color>,
) {
    if let Some(fill) = fill {
        draw.draw_rectangle(
            rect.x.round() as i32,
            rect.y.round() as i32,
            rect.width.round() as i32,
            rect.height.round() as i32,
            fill,
        );
    }
    draw.draw_rectangle_lines_ex(
        Rectangle::new(rect.x, rect.y, rect.width, rect.height),
        2.0,
        outline,
    );
}

fn draw_frame_combat_overlay(draw: &mut RaylibDrawHandle<'_>, overlay: &SpriteFrameCombatOverlay) {
    for hurtbox in &overlay.hurtboxes {
        draw_frame_data_box(
            draw,
            hurtbox,
            FRAME_DATA_HURTBOX,
            Some(FRAME_DATA_HURTBOX_FILL),
        );
    }
    for hitbox in &overlay.hitboxes {
        draw_frame_data_box(
            draw,
            hitbox,
            FRAME_DATA_HITBOX,
            Some(FRAME_DATA_HITBOX_FILL),
        );
    }
    if let Some(origin) = overlay.projectile_origin {
        draw_projectile_origin(draw, origin, COMBAT_PROJECTILE);
    }
}

fn draw_frame_data_box(
    draw: &mut RaylibDrawHandle<'_>,
    overlay_box: &SpriteFrameCombatBoxOverlay,
    outline: Color,
    fill: Option<Color>,
) {
    if let Some(fill) = fill {
        draw.draw_rectangle(
            overlay_box.rect.x.round() as i32,
            overlay_box.rect.y.round() as i32,
            overlay_box.rect.width.round() as i32,
            overlay_box.rect.height.round() as i32,
            fill,
        );
    }
    draw_outline(draw, overlay_box.rect, outline, 3.0);
    if let Some(label) = overlay_box.label.as_deref() {
        draw.draw_text(
            label,
            overlay_box.rect.x.round() as i32 + 4,
            overlay_box.rect.y.round() as i32 + 4,
            13,
            outline,
        );
    }
}

fn draw_projectile_origin(draw: &mut RaylibDrawHandle<'_>, origin: ViewerPoint, color: Color) {
    draw.draw_circle(origin.x.round() as i32, origin.y.round() as i32, 5.0, color);
    draw.draw_line(
        origin.x.round() as i32 - 10,
        origin.y.round() as i32,
        origin.x.round() as i32 + 10,
        origin.y.round() as i32,
        color,
    );
    draw.draw_line(
        origin.x.round() as i32,
        origin.y.round() as i32 - 10,
        origin.x.round() as i32,
        origin.y.round() as i32 + 10,
        color,
    );
}

fn draw_projectile_trajectory(
    draw: &mut RaylibDrawHandle<'_>,
    trajectory: &crate::scenes::sprite_viewer::SpriteProjectileTrajectory,
) {
    draw.draw_line_ex(
        Vector2::new(trajectory.origin.x, trajectory.origin.y),
        Vector2::new(trajectory.end.x, trajectory.end.y),
        2.0,
        PROJECTILE_TRAJECTORY,
    );
    for sample in &trajectory.samples {
        draw_combat_rect(
            draw,
            *sample,
            PROJECTILE_TRAJECTORY,
            Some(PROJECTILE_TRAJECTORY_FILL),
        );
    }
    draw.draw_text(
        &format!("{:.0}px travel", trajectory.travel_distance),
        trajectory.origin.x.round() as i32 + 12,
        trajectory.origin.y.round() as i32 + 12,
        13,
        PROJECTILE_TRAJECTORY,
    );
}

fn draw_frame_cursor(draw: &mut RaylibDrawHandle<'_>, cursor: SpriteFrameCursor) {
    let x = cursor.screen_position.x.round() as i32;
    let y = cursor.screen_position.y.round() as i32;
    draw.draw_line(x - 10, y, x + 10, y, CURSOR_COLOR);
    draw.draw_line(x, y - 10, x, y + 10, CURSOR_COLOR);
    draw.draw_text(
        &format!(
            "local {},{} | atlas {},{}",
            cursor.local_x, cursor.local_y, cursor.atlas_x, cursor.atlas_y
        ),
        x + 12,
        y + 12,
        13,
        CURSOR_COLOR,
    );
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

fn draw_timeline(draw: &mut RaylibDrawHandle<'_>, viewer: &SpriteViewer) {
    let frame_names = viewer.current_clip_frame_names();
    if frame_names.is_empty() {
        return;
    }

    let timeline_x = 16;
    let timeline_y = WINDOW_HEIGHT - 42;
    let timeline_width = WINDOW_WIDTH - 32;
    let timeline_height = 22;
    let segment_width = timeline_width as f32 / frame_names.len() as f32;
    let (current_frame, _) = viewer.frame_position();

    draw.draw_rectangle(
        timeline_x,
        timeline_y - 18,
        timeline_width,
        timeline_height + 28,
        PANEL,
    );
    draw.draw_rectangle_lines(
        timeline_x,
        timeline_y - 18,
        timeline_width,
        timeline_height + 28,
        PANEL_BORDER,
    );
    draw.draw_text("timeline", timeline_x + 8, timeline_y - 15, 13, UI_MUTED);

    for (index, frame_name) in frame_names.iter().enumerate() {
        let x = timeline_x as f32 + index as f32 * segment_width;
        let width = segment_width.max(1.0).ceil();
        let color = timeline_color(viewer.timeline_phase_for_frame_index(index));
        draw.draw_rectangle(
            x.round() as i32,
            timeline_y,
            width.round() as i32,
            timeline_height,
            color,
        );
        draw.draw_rectangle_lines(
            x.round() as i32,
            timeline_y,
            width.round() as i32,
            timeline_height,
            PANEL_BORDER,
        );

        if index == current_frame {
            draw.draw_rectangle_lines_ex(
                Rectangle::new(
                    x,
                    timeline_y as f32 - 2.0,
                    width,
                    timeline_height as f32 + 4.0,
                ),
                3.0,
                UI_TEXT,
            );
        }

        if segment_width >= 34.0 {
            draw.draw_text(
                &(index + 1).to_string(),
                x.round() as i32 + 4,
                timeline_y + 4,
                12,
                UI_TEXT,
            );
        }

        if index == current_frame {
            draw.draw_text(
                &truncate_middle(frame_name, 36),
                timeline_x + 86,
                timeline_y - 15,
                13,
                UI_TEXT,
            );
        }
    }
}

fn timeline_color(phase: Option<SpriteTimelinePhase>) -> Color {
    match phase {
        Some(SpriteTimelinePhase::Startup) => TIMELINE_STARTUP,
        Some(SpriteTimelinePhase::Active) => TIMELINE_ACTIVE,
        Some(SpriteTimelinePhase::Recovery) => TIMELINE_RECOVERY,
        None => TIMELINE_INACTIVE,
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
    let combat = viewer.combat_overlay();
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
        "mouse inspect/drag | Tab clip | Enter sync | [] golpe | C char | ./, frame | T traj",
        panel_x + 16,
        panel_y + 92,
        15,
        UI_MUTED,
    );
    if let Some(overlay) = combat {
        draw.draw_text(
            &format!(
                "combat: {:?} / {} ({}) | M overlay",
                overlay.character,
                overlay.selected_move.label(),
                overlay.move_label,
            ),
            panel_x + 16,
            panel_y + 112,
            14,
            COMBAT_HURTBOX,
        );
    } else if let Some(character) = viewer.selected_character() {
        draw.draw_text(
            &format!(
                "combat: {:?} / {} | M overlay",
                character,
                viewer.selected_move().label(),
            ),
            panel_x + 16,
            panel_y + 112,
            14,
            COMBAT_HURTBOX,
        );
    }

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

    if let Some(frame_combat) = &frame.combat {
        let origin = if frame_combat.projectile_origin.is_some() {
            "origin yes"
        } else {
            "origin no"
        };
        draw.draw_text(
            &format!(
                "frame data: {} hurt / {} hit / {}",
                frame_combat.hurtboxes.len(),
                frame_combat.hitboxes.len(),
                origin,
            ),
            panel_x + 430,
            panel_y + 88,
            14,
            FRAME_DATA_HURTBOX,
        );
    }

    if let Some(cursor) = viewer.frame_cursor() {
        draw.draw_text(
            &format!(
                "cursor local {},{} | atlas {},{}",
                cursor.local_x, cursor.local_y, cursor.atlas_x, cursor.atlas_y
            ),
            panel_x + 430,
            panel_y + 108,
            14,
            CURSOR_COLOR,
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
