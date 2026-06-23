//! Draws combat inspection overlays for lab and debug views.
//!
//! System: Combat debug UI. This module draws boxes, labels, pivots, and timing
//! overlays from snapshots owned elsewhere; it does not advance combat state.

use raylib::prelude::*;

use crate::characters::character_spec;
use crate::combat::fighter::{AttackPhase, Fighter};
use crate::config::{WINDOW_HEIGHT, WINDOW_WIDTH, screen_px, world_px};
use crate::math::rect::Rect;
use crate::scenes::combat_lab::{CombatLab, CombatLabMove, CombatLabPose};

const BODY_OUTLINE: Color = Color::new(238, 241, 247, 255);
const HURTBOX: Color = Color::new(105, 240, 174, 255);
const HITBOX: Color = Color::new(255, 82, 82, 255);
const HITBOX_FILL: Color = Color::new(255, 82, 82, 82);
const HITSPARK: Color = Color::new(255, 235, 59, 255);
const PROJECTILE: Color = Color::new(80, 220, 255, 255);
const PANEL: Color = Color::new(12, 14, 20, 218);
const PANEL_BORDER: Color = Color::new(122, 132, 150, 255);
const UI_MUTED: Color = Color::new(165, 172, 185, 255);
const UI_TEXT: Color = Color::new(238, 241, 247, 255);

/// Draws every Combat Lab debug overlay controlled by lab toggles.
pub fn draw_combat_lab_debug(draw: &mut impl RaylibDraw, lab: &CombatLab) {
    draw_lab_boxes(draw, lab);
    if lab.show_dummy() {
        draw_lab_dummy(draw, lab);
    }
    if lab.show_pivot() {
        draw_lab_pivot(draw, lab.fighter());
    }
    draw_lab_overlay(draw, lab);
}

fn draw_lab_boxes(draw: &mut impl RaylibDraw, lab: &CombatLab) {
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

fn draw_lab_dummy(draw: &mut impl RaylibDraw, lab: &CombatLab) {
    let dummy = lab.dummy_body_rect();
    outline_rect(draw, dummy, PANEL_BORDER);
    draw.draw_text(
        "DUMMY",
        dummy.x as i32 + screen_px(6),
        dummy.y as i32 - screen_px(24),
        screen_px(16),
        UI_MUTED,
    );
}

fn draw_lab_pivot(draw: &mut impl RaylibDraw, fighter: &Fighter) {
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
    draw.draw_circle(pivot_x, pivot_y, world_px(5.0), HITSPARK);
    draw.draw_text(
        "PIVOT",
        pivot_x + screen_px(8),
        pivot_y - screen_px(22),
        screen_px(14),
        HITSPARK,
    );
}

fn draw_lab_overlay(draw: &mut impl RaylibDraw, lab: &CombatLab) {
    let panel_x = screen_px(20);
    let panel_y = screen_px(18);
    let panel_width = screen_px(560);
    let panel_height = screen_px(172);

    draw.draw_rectangle(panel_x, panel_y, panel_width, panel_height, PANEL);
    draw.draw_rectangle_lines(panel_x, panel_y, panel_width, panel_height, PANEL_BORDER);

    draw.draw_text(
        "Combat Lab",
        panel_x + screen_px(16),
        panel_y + screen_px(12),
        screen_px(22),
        UI_TEXT,
    );
    draw.draw_text(
        &format!(
            "{} / {} / frame {:03}",
            character_spec(lab.character()).display_name,
            lab_label(lab),
            lab.current_frame().get()
        ),
        panel_x + screen_px(16),
        panel_y + screen_px(42),
        screen_px(16),
        UI_TEXT,
    );
    draw.draw_text(
        &lab_timing_text(lab),
        panel_x + screen_px(16),
        panel_y + screen_px(68),
        screen_px(14),
        UI_MUTED,
    );
    draw.draw_text(
        &lab_toggle_text(lab),
        panel_x + screen_px(16),
        panel_y + screen_px(132),
        screen_px(13),
        UI_MUTED,
    );
    if let Some(advantage) = lab.advantage() {
        draw.draw_text(
            &format!(
                "adv hit {} block {} | rec {}f whiff {}f{}",
                signed_frames(advantage.hit_advantage),
                signed_frames(advantage.block_advantage),
                advantage.attacker_recovery_after_contact.get(),
                advantage.whiff_recovery.get(),
                cooldown_suffix(advantage.projectile_cooldown_after_contact)
            ),
            panel_x + screen_px(16),
            panel_y + screen_px(92),
            screen_px(13),
            UI_MUTED,
        );
        draw.draw_text(
            &format!(
                "push H/B {:.0}/{:.0} | gap H/B {:.0}/{:.0}",
                advantage.hit_pushback,
                advantage.block_pushback,
                advantage.hit_body_gap_after_pushback,
                advantage.block_body_gap_after_pushback
            ),
            panel_x + screen_px(16),
            panel_y + screen_px(112),
            screen_px(13),
            UI_MUTED,
        );
    }
    draw.draw_text(
        if lab.paused() { "PAUSED" } else { "PLAYING" },
        panel_x + panel_width - screen_px(88),
        panel_y + screen_px(16),
        screen_px(14),
        HITSPARK,
    );
}

fn lab_timing_text(lab: &CombatLab) -> String {
    if !matches!(lab.pose(), CombatLabPose::Move) {
        return format!("pose {} | inspect boxes/pivot", lab.pose().label());
    }

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
                fighter.projectile_spec().damage
            )
        }
        _ => {
            let fighter = lab.fighter();
            let phase = match fighter.attack_phase() {
                AttackPhase::Idle => "idle",
                AttackPhase::Startup => "startup",
                AttackPhase::Active => "active",
                AttackPhase::Recovery => "recovery",
                AttackPhase::WhiffRecovery => "whiff",
            };
            if let (Some(spec), Some(elapsed), Some(frame_data)) = (
                fighter.attack_move_spec(),
                fighter.attack_elapsed_frames(),
                fighter.attack_frame_data(),
            ) {
                format!(
                    "{} {:02}/{:02} {} act {:02}-{:02} dmg {}",
                    spec.label,
                    elapsed.get(),
                    frame_data.duration.get(),
                    phase,
                    frame_data.active_start.get(),
                    frame_data.active_end.get(),
                    spec.damage
                )
            } else {
                format!("waiting for {}", lab.selected_move().label())
            }
        }
    }
}

fn lab_label(lab: &CombatLab) -> String {
    match lab.pose() {
        CombatLabPose::Move => format!("move {}", lab.selected_move().label()),
        pose => format!("pose {}", pose.label()),
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

fn signed_frames(frames: i16) -> String {
    if frames >= 0 {
        format!("+{}", frames)
    } else {
        frames.to_string()
    }
}

fn cooldown_suffix(cooldown: crate::combat::frame::FrameCount) -> String {
    if cooldown == crate::combat::frame::FrameCount::ZERO {
        String::new()
    } else {
        format!(" cd {}f", cooldown.get())
    }
}

fn outline_rect(draw: &mut impl RaylibDraw, rect: Rect, color: Color) {
    draw.draw_rectangle_lines(
        rect.x.round() as i32,
        rect.y.round() as i32,
        rect.width.round() as i32,
        rect.height.round() as i32,
        color,
    );
}
