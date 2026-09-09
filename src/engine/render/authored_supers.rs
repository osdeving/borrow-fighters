//! Stages the eclipse and system crash around the authoritative super clock.
//!
//! System: Raylib presentation. These layers are scenery only. World owns capture,
//! guard and damage; complete blackouts and BIOS replace the entire game frame.

use std::f32::consts::{PI, TAU};

use raylib::prelude::*;

use crate::{
    characters::CharacterId,
    combat::super_sequence::{SuperPhase, SuperSequence},
    config::{FLOOR_Y, WINDOW_HEIGHT, WINDOW_WIDTH},
    engine::assets::GameAssets,
    game::{arena::ArenaId, world::World},
};

use super::{DrawTarget, draw_menu_text};

const AMBER: Color = Color::new(255, 164, 62, 255);
const ICE: Color = Color::new(120, 224, 255, 255);

fn alpha(mut color: Color, amount: f32) -> Color {
    color.a = (amount.clamp(0.0, 1.0) * 255.0) as u8;
    color
}

pub(super) fn replaces_frame(world: &World) -> bool {
    world
        .super_sequence()
        .is_some_and(|s| matches!(s.phase(), SuperPhase::RustBlackout | SuperPhase::CBios))
}

/// Complete presentation override, before any arena, actor or HUD is drawn.
pub(super) fn draw_override(draw: &mut impl DrawTarget, world: &World) -> bool {
    let Some(sequence) = world.super_sequence() else {
        return false;
    };
    match sequence.phase() {
        SuperPhase::RustBlackout => draw.clear_background(Color::BLACK),
        SuperPhase::CBios => {
            draw.clear_background(Color::BLACK);
            let lines = [
                "BORROW BIOS v0.1  /  LEGACY SYSTEM ROM",
                "Copyright (C) The Linkers. All memory reserved.",
                "",
                "CPU: OLD C  //  REAL MODE RECOVERY",
                "Memory test: 640K OK",
                "Extended memory: 0xDEADBEEF ... FAULT",
                "",
                "Detecting arena controller ........ OK",
                "Restoring interrupt vector table .. OK",
                "Discarding protected process ...... OK",
                "",
                "Booting arena from last known state_",
            ];
            let visible = 4 + (sequence.phase_progress() * 13.0) as usize;
            for (row, line) in lines.iter().take(visible).enumerate() {
                // Deliberate ROM bitmap type, unlike the smooth game UI.
                draw.draw_text(
                    line,
                    54,
                    54 + row as i32 * 35,
                    23,
                    Color::new(195, 200, 197, 255),
                );
            }
            draw.draw_text(
                "DEL: SETUP     F1: CONTINUE",
                54,
                WINDOW_HEIGHT - 62,
                20,
                Color::GRAY,
            );
        }
        _ => return false,
    }
    true
}

/// Holds the original stage animation still while the scripted world takes over.
pub(super) fn arena_time(world: &World, visual_time: f32) -> f32 {
    world.super_sequence().map_or(visual_time, |s| {
        (world.elapsed_seconds - s.tick as f32 / 60.0).max(0.0)
    })
}

pub(super) fn draw_background(draw: &mut impl DrawTarget, world: &World, assets: &GameAssets) {
    let Some(s) = world.super_sequence() else {
        return;
    };
    let fade = if s.phase() == SuperPhase::Restore {
        1.0 - s.phase_progress()
    } else {
        1.0
    };
    draw.draw_rectangle(
        0,
        0,
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        alpha(Color::new(2, 5, 15, 255), 0.62 * fade),
    );
    match s.character {
        CharacterId::Rust if s.phase() != SuperPhase::Freeze => {
            draw_mutation(draw, s, assets, fade)
        }
        CharacterId::C => draw_crash(draw, s, assets, fade),
        CharacterId::Cpp => {
            if (44..110).contains(&s.tick) {
                draw_pointer_fault(draw, s, assets);
            }
            if matches!(
                s.phase(),
                SuperPhase::CppCharge | SuperPhase::CppBarrage | SuperPhase::CppFinisher
            ) {
                for lane in 0..28 {
                    let y = 115.0 + lane as f32 * 18.0;
                    let x = (s.tick as f32 * 46.0 + lane as f32 * 129.0).rem_euclid(1550.0) - 260.0;
                    draw.draw_line_ex(
                        Vector2::new(x, y),
                        Vector2::new(x + 260.0, y - 12.0),
                        2.0,
                        alpha(ICE, 0.3),
                    );
                }
            }
        }
        _ => {}
    }
}

fn draw_pointer_fault(draw: &mut impl DrawTarget, s: &SuperSequence, assets: &GameAssets) {
    let age = (s.tick - 44) as f32;
    let fade = ((110 - s.tick) as f32 / 12.0).min(1.0);
    let jitter = if age < 9.0 {
        (age * 2.0).sin() * 8.0
    } else {
        0.0
    };
    let x = 352 + jitter as i32;
    let error = Color::new(255, 110, 124, 255);
    draw.draw_rectangle(x + 7, 239, 594, 147, alpha(Color::BLACK, 0.6 * fade));
    draw.draw_rectangle(
        x,
        232,
        594,
        147,
        alpha(Color::new(29, 9, 34, 255), 0.96 * fade),
    );
    draw.draw_rectangle_lines(x, 232, 594, 147, alpha(error, fade));
    draw_menu_text(
        draw,
        assets.menu_font.as_ref(),
        "ASSERT FAILED / target != this",
        x + 20,
        246,
        27.0,
        alpha(error, fade),
    );
    draw_menu_text(
        draw,
        assets.menu_font.as_ref(),
        "weapon.target = &this->foot;",
        x + 20,
        284,
        30.0,
        alpha(Color::WHITE, fade),
    );
    draw_menu_text(
        draw,
        assets.menu_font.as_ref(),
        "EXPECTED: enemy     ACTUAL: meu proprio pe",
        x + 20,
        329,
        20.0,
        alpha(ICE, fade),
    );
    let direction = if s.facing == crate::combat::fighter::Facing::Right {
        1.0
    } else {
        -1.0
    };
    let foot = if s.phase() == SuperPhase::CppHop {
        let hop = (((s.tick - 62) as f32 / 12.0) * PI).sin().abs() * 23.0;
        Vector2::new(
            s.attacker_anchor.x + direction * 10.0,
            FLOOR_Y - 100.0 - hop,
        )
    } else {
        Vector2::new(s.attacker_anchor.x + direction * 59.0, FLOOR_Y - 16.0)
    };
    let joint = Vector2::new(640.0, 404.0);
    draw.draw_line_ex(
        Vector2::new(640.0, 379.0),
        joint,
        2.0,
        alpha(error, 0.7 * fade),
    );
    draw.draw_line_ex(joint, foot, 2.0, alpha(error, 0.5 * fade));
    draw.draw_ring(foot, 32.0, 34.0, 0.0, 360.0, 40, alpha(error, fade));
}

fn draw_mutation(draw: &mut impl DrawTarget, s: &SuperSequence, assets: &GameAssets, fade: f32) {
    let tick = s.tick as f32;
    let build = ((tick - 11.0) / 114.0).clamp(0.0, 1.0);
    let center = Vector2::new(640.0, 265.0);
    let radius = 88.0 + 94.0 * build;
    draw.draw_circle_v(center, radius, alpha(Color::new(1, 2, 9, 255), fade));
    for ring in 0..3 {
        let r = radius + ring as f32 * 28.0;
        draw.draw_ring(
            center,
            r,
            r + 3.0,
            0.0,
            360.0,
            96,
            alpha(AMBER, fade * (0.9 - ring as f32 * 0.2)),
        );
        for tooth in 0..24 {
            let a = tooth as f32 * TAU / 24.0 + tick * 0.006;
            draw.draw_line_ex(
                polar(center, r, a),
                polar(center, r + 13.0, a),
                7.0,
                alpha(AMBER, fade * 0.65),
            );
        }
    }
    if let Some(texture) = assets.rust_mutation_props.as_ref() {
        // Reviewed disjoint crops, not nominal quarters: each silhouette stays whole.
        let ring = Rectangle::new(10.0, 20.0, 980.0, 560.0);
        let tower = Rectangle::new(1015.0, 0.0, 433.0, 610.0);
        let plate = Rectangle::new(14.0, 640.0, 884.0, 446.0);
        let cables = Rectangle::new(900.0, 615.0, 548.0, 461.0);
        let rise = build * build * (3.0 - 2.0 * build);
        prop(
            draw,
            texture,
            ring,
            Rectangle::new(
                318.0,
                263.0 + (1.0 - rise) * 180.0,
                648.0,
                332.0 * rise.max(0.02),
            ),
            fade * build,
        );
        for i in 0..6 {
            let local = ((build * 1.8 - i as f32 * 0.12).clamp(0.0, 1.0)) * fade;
            let sway = (tick * 0.035 + i as f32 * 1.4).sin();
            let x = 24.0 + i as f32 * 215.0;
            prop(
                draw,
                texture,
                plate,
                Rectangle::new(
                    x,
                    FLOOR_Y - 53.0 - local * (30.0 + sway * 14.0),
                    245.0,
                    110.0 * local.max(0.01),
                ),
                local,
            );
            for drip in 0..4 {
                let dx = x + 32.0 + drip as f32 * 43.0;
                let melt =
                    ((tick * 1.3 + drip as f32 * 19.0 + i as f32 * 9.0).rem_euclid(54.0)) * local;
                draw.draw_line_ex(
                    Vector2::new(dx, FLOOR_Y - 5.0),
                    Vector2::new(dx + sway * 8.0, FLOOR_Y - 5.0 + melt),
                    3.0,
                    alpha(AMBER, local * 0.75),
                );
            }
        }
        for side in 0..2 {
            let grow = ((build - side as f32 * 0.18) * 1.6).clamp(0.0, 1.0);
            let height = (315.0 + (tick * 0.045).sin() * 18.0) * grow * fade;
            let x = if side == 0 { 62.0 } else { 994.0 };
            prop(
                draw,
                texture,
                tower,
                Rectangle::new(x, FLOOR_Y - height - 25.0, 236.0, height),
                grow * fade,
            );
            prop(
                draw,
                texture,
                cables,
                Rectangle::new(x - 45.0, FLOOR_Y - 200.0 * grow, 294.0, 225.0 * grow),
                grow * fade,
            );
            if s.phase() == SuperPhase::RustCharge || s.phase() == SuperPhase::RustPulse {
                let from = Vector2::new(x + 118.0, FLOOR_Y - height + 60.0);
                let target = Vector2::new(s.target_anchor.x, s.target_anchor.y - 95.0);
                for segment in 0..7 {
                    let a = segment as f32 / 7.0;
                    let b = (segment + 1) as f32 / 7.0;
                    let wobble = (tick * 0.7 + segment as f32 * 2.0).sin() * 23.0;
                    let start = Vector2::new(
                        from.x + (target.x - from.x) * a,
                        from.y + (target.y - from.y) * a + wobble,
                    );
                    let end = Vector2::new(
                        from.x + (target.x - from.x) * b,
                        from.y + (target.y - from.y) * b - wobble,
                    );
                    draw.draw_line_ex(start, end, 3.0, alpha(AMBER, 0.65 * fade));
                }
            }
        }
    }
    for i in 0..50 {
        let x = (i as f32 * 179.0 + tick * 0.8).rem_euclid(1280.0);
        let y = FLOOR_Y - ((tick * 2.0 + i as f32 * 39.0).rem_euclid(460.0)) * build;
        draw.draw_poly_lines_ex(
            Vector2::new(x, y),
            6,
            5.0 + i as f32 % 4.0,
            tick,
            1.0,
            alpha(AMBER, fade * build * 0.5),
        );
    }
    draw_menu_text(
        draw,
        assets.menu_font.as_ref(),
        "&mut world",
        536,
        258,
        36.0,
        alpha(AMBER, fade),
    );
    draw_menu_text(
        draw,
        assets.menu_font.as_ref(),
        "SIRIUS / matter borrowed",
        489,
        307,
        20.0,
        alpha(Color::WHITE, fade * 0.8),
    );
}

fn prop(
    draw: &mut impl DrawTarget,
    texture: &Texture2D,
    crop: Rectangle,
    dest: Rectangle,
    opacity: f32,
) {
    if dest.height > 1.0 && opacity > 0.01 {
        draw.draw_texture_pro(
            texture,
            crop,
            dest,
            Vector2::zero(),
            0.0,
            alpha(Color::WHITE, opacity),
        );
    }
}

fn draw_crash(draw: &mut impl DrawTarget, s: &SuperSequence, assets: &GameAssets, fade: f32) {
    if s.phase() == SuperPhase::CTerminalStorm {
        const ERRORS: [&str; 6] = [
            "Segmentation fault (core dumped)",
            "NULL pointer dereference",
            "Stack overflow at 0xDEADBEEF",
            "Access violation: read of 0x0000",
            "Illegal instruction: SIGILL",
            "#GP: protection ring violation",
        ];
        let count = 1 + (s.phase_progress() * 27.0) as i32;
        for i in 0..count {
            let x = 26 + (i * 181) % 876;
            let y = 147 + (i * 53) % 364;
            draw.draw_rectangle(x + 7, y + 7, 352, 132, Color::new(0, 0, 0, 150));
            draw.draw_rectangle(x, y, 352, 132, Color::new(7, 14, 16, 255));
            draw.draw_rectangle_lines(x, y, 352, 132, Color::new(166, 175, 180, 255));
            draw.draw_rectangle(x + 2, y + 2, 348, 23, Color::new(15, 53, 133, 255));
            draw_menu_text(
                draw,
                assets.menu_font.as_ref(),
                &format!("old-c.exe / terminal {:02}", i + 1),
                x + 10,
                y + 3,
                16.0,
                Color::WHITE,
            );
            draw_menu_text(
                draw,
                assets.menu_font.as_ref(),
                ERRORS[i as usize % ERRORS.len()],
                x + 12,
                y + 40,
                17.0,
                Color::new(125, 255, 167, 255),
            );
            draw_menu_text(
                draw,
                assets.menu_font.as_ref(),
                "Process cannot continue.",
                x + 12,
                y + 65,
                15.0,
                Color::LIGHTGRAY,
            );
            draw.draw_rectangle(x + 279, y + 96, 58, 25, Color::LIGHTGRAY);
            draw_menu_text(
                draw,
                assets.menu_font.as_ref(),
                "OK",
                x + 296,
                y + 98,
                17.0,
                Color::DARKBLUE,
            );
        }
    } else if s.phase() == SuperPhase::CBlueScreen {
        draw.draw_rectangle(
            0,
            0,
            WINDOW_WIDTH,
            WINDOW_HEIGHT,
            Color::new(0, 20, 168, 244),
        );
        draw_menu_text(
            draw,
            assets.menu_font.as_ref(),
            "GENERAL PROTECTION FAULT",
            92,
            224,
            48.0,
            Color::WHITE,
        );
        draw_menu_text(
            draw,
            assets.menu_font.as_ref(),
            "#GP  /  STOP 0x0000000D",
            94,
            281,
            29.0,
            Color::WHITE,
        );
        for (i, line) in [
            "A fatal exception has occurred in OLD_C.SYS.",
            "The current process will be terminated.",
            "Protection rings could not contain this pointer.",
            "Preparing emergency reboot ...",
        ]
        .iter()
        .enumerate()
        {
            draw_menu_text(
                draw,
                assets.menu_font.as_ref(),
                line,
                360,
                347 + i as i32 * 29,
                22.0,
                Color::WHITE,
            );
        }
    } else if s.phase() == SuperPhase::CReboot {
        let p = s.phase_progress();
        draw.draw_rectangle(
            0,
            0,
            WINDOW_WIDTH,
            WINDOW_HEIGHT,
            alpha(Color::new(0, 26, 176, 255), (1.0 - p) * 0.35),
        );
        for row in 0..18 {
            let y = row * 42;
            draw.draw_rectangle(0, y, WINDOW_WIDTH, 2, alpha(ICE, (1.0 - p) * fade * 0.35));
        }
    }
}

pub(super) fn draw_foreground(draw: &mut impl DrawTarget, world: &World, assets: &GameAssets) {
    let Some(s) = world.super_sequence() else {
        return;
    };
    let fade = if s.phase() == SuperPhase::Restore {
        1.0 - s.phase_progress()
    } else {
        1.0
    };
    let accent = if s.character == CharacterId::Rust || s.character == CharacterId::Duke {
        AMBER
    } else {
        ICE
    };
    if s.phase() == SuperPhase::Freeze {
        let c = Vector2::new(s.attacker_anchor.x, s.attacker_anchor.y - 100.0);
        for i in 0..20 {
            let a = i as f32 * TAU / 20.0;
            draw.draw_line_ex(
                polar(c, 75.0, a),
                polar(c, 750.0, a),
                2.0,
                alpha(accent, 0.4),
            );
        }
        draw.draw_ring(c, 84.0, 91.0, 0.0, 360.0, 64, accent);
    }
    if s.phase() == SuperPhase::RustPulse {
        let p = s.phase_progress();
        let center = Vector2::new(s.target_anchor.x, s.target_anchor.y - 90.0);
        let radius = 35.0 + p * 870.0;
        draw.draw_ring(
            center,
            radius,
            radius + 12.0 * (1.0 - p),
            0.0,
            360.0,
            100,
            alpha(AMBER, 1.0 - p),
        );
        draw.draw_rectangle(
            0,
            0,
            WINDOW_WIDTH,
            WINDOW_HEIGHT,
            alpha(AMBER, (p * PI).sin() * 0.11),
        );
    }
    draw.draw_rectangle(
        0,
        116,
        WINDOW_WIDTH,
        76,
        alpha(Color::new(3, 7, 19, 255), 0.93 * fade),
    );
    draw.draw_rectangle(0, 116, WINDOW_WIDTH, 2, alpha(accent, fade));
    draw_menu_text(
        draw,
        assets.menu_font.as_ref(),
        s.label,
        30,
        121,
        35.0,
        alpha(Color::WHITE, fade),
    );
    let home = ArenaId::home_for_character(s.character);
    draw_menu_text(
        draw,
        assets.menu_font.as_ref(),
        &format!("{}  /  {}", home.label(), home.location()),
        32,
        160,
        18.0,
        alpha(accent, fade),
    );
    let guard = if s.guarded {
        "GUARD / CHIP"
    } else {
        "CAPTURED"
    };
    draw_menu_text(
        draw,
        assets.menu_font.as_ref(),
        guard,
        1060,
        151,
        19.0,
        alpha(accent, fade),
    );
}

fn polar(center: Vector2, radius: f32, angle: f32) -> Vector2 {
    Vector2::new(
        center.x + angle.cos() * radius,
        center.y + angle.sin() * radius,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{combat::fighter::FighterInput, config::FIXED_TIMESTEP};

    #[test]
    fn paused_super_scenery_ignores_the_app_wall_clock() {
        let mut world = World::new_with_characters(CharacterId::Rust, CharacterId::Duke);
        world.update(
            FIXED_TIMESTEP,
            FighterInput {
                cinematic_special: true,
                ..FighterInput::default()
            },
            FighterInput::default(),
        );
        assert!(world.super_sequence_active());
        let time = arena_time(&world, 30.0);
        assert_eq!(arena_time(&world, 90.0), time);
        for _ in 0..30 {
            world.update(
                FIXED_TIMESTEP,
                FighterInput::default(),
                FighterInput::default(),
            );
        }
        assert!((arena_time(&world, 120.0) - time).abs() < 0.0001);
    }
}
