//! Draws the isolated Combat Lab scene.
//!
//! The lab renderer stays next to the Raylib boundary while the lab state stays
//! testable under `scenes/combat_lab.rs`.

use raylib::prelude::*;

use crate::combat::{
    fighter::{AttackPhase, Facing, Fighter},
    projectile::{PROJECTILE_DAMAGE, Projectile},
};
use crate::config::{FLOOR_Y, WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::engine::{assets::GameAssets, sprites};
use crate::math::rect::Rect;
use crate::scenes::combat_lab::{CombatLab, CombatLabCharacter, CombatLabMove};

use super::{
    BACKGROUND, BODY_OUTLINE, FighterDrawOptions, HITBOX, HITBOX_FILL, HITSPARK, HURTBOX, PANEL,
    PANEL_BORDER, PLAYER_ONE, PLAYER_TWO, PROJECTILE, PROJECTILE_FILL, UI_MUTED, UI_TEXT,
    draw_fighter, outline_rect,
};

/// Draws the isolated Combat Lab scene.
pub fn draw_combat_lab(draw: &mut RaylibDrawHandle<'_>, lab: &CombatLab, assets: &GameAssets) {
    draw.clear_background(BACKGROUND);
    draw_lab_grid(draw);

    let (body_color, sprite_atlas, projectile_texture) = match lab.character() {
        CombatLabCharacter::Rust => (
            PLAYER_ONE,
            assets.rust_fighter.as_ref(),
            assets.rust_projectile.as_ref(),
        ),
        CombatLabCharacter::Duke => (
            PLAYER_TWO,
            assets.duke_fighter.as_ref(),
            assets.duke_projectile.as_ref(),
        ),
    };

    draw_fighter(
        draw,
        lab.fighter(),
        FighterDrawOptions {
            body_color,
            show_debug: false,
            sprite_atlas,
            spritesheet: assets.fighter_spritesheet.as_ref(),
            world_elapsed_seconds: lab.elapsed_seconds(),
            forced_clip: None,
        },
    );

    draw_lab_projectiles(draw, lab.projectiles(), projectile_texture);
    draw_lab_boxes(draw, lab);
    if lab.show_dummy() {
        draw_lab_dummy(draw);
    }
    if lab.show_pivot() {
        draw_lab_pivot(draw, lab.fighter());
    }
    draw_lab_overlay(draw, lab);
}

fn draw_lab_grid(draw: &mut RaylibDrawHandle<'_>) {
    for x in (0..=WINDOW_WIDTH).step_by(80) {
        draw.draw_line(x, 0, x, WINDOW_HEIGHT, Color::new(44, 49, 60, 255));
    }
    for y in (0..=WINDOW_HEIGHT).step_by(60) {
        draw.draw_line(0, y, WINDOW_WIDTH, y, Color::new(44, 49, 60, 255));
    }
    draw.draw_line(0, FLOOR_Y as i32, WINDOW_WIDTH, FLOOR_Y as i32, UI_MUTED);
}

fn draw_lab_projectiles(
    draw: &mut RaylibDrawHandle<'_>,
    projectiles: &[Projectile],
    texture: Option<&Texture2D>,
) {
    for projectile in projectiles {
        let rect = projectile.rect();
        if let Some(texture) = texture {
            let facing = if projectile.velocity.x < 0.0 {
                Facing::Left
            } else {
                Facing::Right
            };
            let center = rect.center();
            sprites::draw_projectile_texture(
                draw,
                texture,
                Vector2::new(center.x, center.y),
                facing,
                Color::WHITE,
            );
        } else {
            draw.draw_rectangle(
                rect.x.round() as i32,
                rect.y.round() as i32,
                rect.width.round() as i32,
                rect.height.round() as i32,
                PROJECTILE_FILL,
            );
        }
        outline_rect(draw, rect, PROJECTILE);
    }
}

fn draw_lab_boxes(draw: &mut RaylibDrawHandle<'_>, lab: &CombatLab) {
    let fighter = lab.fighter();
    outline_rect(draw, fighter.body_rect(), BODY_OUTLINE);

    if lab.show_hurtboxes() {
        for hurtbox in fighter.hurtboxes().rects() {
            outline_rect(draw, hurtbox, HURTBOX);
        }
    }

    if !lab.show_hitboxes() {
        return;
    }

    if let Some(attack_box) = fighter.attack_box() {
        let active = fighter.active_hitbox().is_some();
        draw.draw_rectangle(
            attack_box.x.round() as i32,
            attack_box.y.round() as i32,
            attack_box.width.round() as i32,
            attack_box.height.round() as i32,
            if active {
                HITBOX_FILL
            } else {
                Color::new(255, 82, 82, 34)
            },
        );
        outline_rect(draw, attack_box, HITBOX);
    }

    for projectile in lab.projectiles() {
        outline_rect(draw, projectile.rect(), PROJECTILE);
    }
}

fn draw_lab_dummy(draw: &mut RaylibDrawHandle<'_>) {
    let dummy = Rect::new(690.0, FLOOR_Y - 168.0, 76.0, 168.0);
    outline_rect(draw, dummy, PANEL_BORDER);
    draw.draw_text(
        "DUMMY",
        dummy.x as i32 + 6,
        dummy.y as i32 - 24,
        16,
        UI_MUTED,
    );
}

fn draw_lab_pivot(draw: &mut RaylibDrawHandle<'_>, fighter: &Fighter) {
    let body = fighter.body_rect();
    let pivot_x = body.center_x().round() as i32;
    let pivot_y = body.bottom().round() as i32;

    draw.draw_line(
        pivot_x,
        0,
        pivot_x,
        WINDOW_HEIGHT,
        Color::new(255, 235, 59, 120),
    );
    draw.draw_line(
        0,
        pivot_y,
        WINDOW_WIDTH,
        pivot_y,
        Color::new(255, 235, 59, 120),
    );
    draw.draw_circle(pivot_x, pivot_y, 5.0, HITSPARK);
    draw.draw_text("PIVOT", pivot_x + 8, pivot_y - 22, 14, HITSPARK);
}

fn draw_lab_overlay(draw: &mut RaylibDrawHandle<'_>, lab: &CombatLab) {
    let panel_x = 20;
    let panel_y = 18;
    let panel_width = 360;
    let panel_height = 128;

    draw.draw_rectangle(panel_x, panel_y, panel_width, panel_height, PANEL);
    draw.draw_rectangle_lines(panel_x, panel_y, panel_width, panel_height, PANEL_BORDER);

    draw.draw_text("Combat Lab", panel_x + 16, panel_y + 12, 22, UI_TEXT);
    draw.draw_text(
        &format!(
            "{} / {} / frame {:03}",
            lab.character().label(),
            lab.selected_move().label(),
            lab.current_frame().get()
        ),
        panel_x + 16,
        panel_y + 42,
        16,
        UI_TEXT,
    );
    draw.draw_text(
        &lab_timing_text(lab),
        panel_x + 16,
        panel_y + 68,
        14,
        UI_MUTED,
    );
    draw.draw_text(
        &lab_toggle_text(lab),
        panel_x + 16,
        panel_y + 92,
        13,
        UI_MUTED,
    );
    draw.draw_text(
        if lab.paused() { "PAUSED" } else { "PLAYING" },
        panel_x + panel_width - 88,
        panel_y + 16,
        14,
        HITSPARK,
    );
}

fn lab_timing_text(lab: &CombatLab) -> String {
    match lab.selected_move() {
        CombatLabMove::Projectile => {
            let fighter = lab.fighter();
            let frame_data = fighter.projectile_frame_data();
            let frame = fighter.special_elapsed_frames().unwrap_or_default();
            format!(
                "special {:02}/{:02} spawn {:02} cd {:02} dmg {}",
                frame.get(),
                frame_data.visual_duration.get(),
                frame_data.spawn_frame.get(),
                fighter.projectile_cooldown_remaining_frames().get(),
                PROJECTILE_DAMAGE
            )
        }
        _ => {
            let fighter = lab.fighter();
            let phase = match fighter.attack_phase() {
                AttackPhase::Idle => "idle",
                AttackPhase::Startup => "startup",
                AttackPhase::Active => "active",
                AttackPhase::Recovery => "recovery",
            };
            if let (Some(kind), Some(elapsed), Some(frame_data)) = (
                fighter.attack_kind(),
                fighter.attack_elapsed_frames(),
                fighter.attack_frame_data(),
            ) {
                format!(
                    "{} {:02}/{:02} {} act {:02}-{:02} dmg {}",
                    kind.label(),
                    elapsed.get(),
                    frame_data.duration.get(),
                    phase,
                    frame_data.active_start.get(),
                    frame_data.active_end.get(),
                    kind.damage()
                )
            } else {
                format!("waiting for {}", lab.selected_move().label())
            }
        }
    }
}

fn lab_toggle_text(lab: &CombatLab) -> String {
    format!(
        "hurtbox {} | hitbox {} | pivot {} | dummy {}",
        on_off(lab.show_hurtboxes()),
        on_off(lab.show_hitboxes()),
        on_off(lab.show_pivot()),
        on_off(lab.show_dummy())
    )
}

fn on_off(enabled: bool) -> &'static str {
    if enabled { "ON" } else { "OFF" }
}
