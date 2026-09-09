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
                if sequence.character == CharacterId::Cpp {
                    "CPU: C++  //  INHERITED FROM OLD C"
                } else {
                    "CPU: OLD C  //  REAL MODE RECOVERY"
                },
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
        alpha(
            Color::new(2, 5, 15, 255),
            if s.character == CharacterId::Rust {
                0.08 * fade
            } else {
                0.62 * fade
            },
        ),
    );
    match s.character {
        CharacterId::Rust if s.phase() != SuperPhase::Freeze => {
            draw_mutation(draw, s, assets, fade)
        }
        CharacterId::C => draw_crash(draw, s, assets, fade),
        CharacterId::Cpp => {
            draw_crash(draw, s, assets, fade);
            if s.phase() == SuperPhase::CppLaptop {
                draw_cpp_code(draw, s, assets);
            }
            if matches!(s.phase(), SuperPhase::CppFootshot | SuperPhase::CppHop)
                && s.tick >= crate::combat::super_sequence::CPP_FOOTSHOT_TICK
            {
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

fn draw_cpp_code(draw: &mut impl DrawTarget, s: &SuperSequence, assets: &GameAssets) {
    let local = s.tick - s.phase_span().start;
    if local < 25 {
        return;
    }
    let x = if s.attacker_origin.x > 640.0 { 42 } else { 420 };
    let y = 219;
    draw.draw_rectangle(x + 8, y + 8, 718, 273, Color::new(0, 0, 0, 145));
    draw.draw_rectangle(x, y, 718, 273, Color::new(8, 18, 27, 248));
    draw.draw_rectangle_lines(x, y, 718, 273, ICE);
    draw.draw_rectangle(x + 1, y + 1, 716, 31, Color::new(24, 61, 108, 255));
    draw_menu_text(
        draw,
        assets.menu_font.as_ref(),
        "daughter.cpp  /  C++  /  inherited bad ideas",
        x + 14,
        y + 4,
        21.0,
        Color::WHITE,
    );
    let lines = [
        "int main() {",
        "    int* alvo = nullptr;",
        "    *alvo = 42;",
        "}",
    ];
    for (index, line) in lines.iter().enumerate() {
        if local < 30 + index as u32 * 9 {
            break;
        }
        let color = if index == 2 {
            Color::new(255, 124, 115, 255)
        } else {
            Color::new(152, 255, 198, 255)
        };
        draw_menu_text(
            draw,
            assets.menu_font.as_ref(),
            line,
            x + 29,
            y + 45 + index as i32 * 35,
            29.0,
            color,
        );
    }
    if local >= 72 {
        draw_menu_text(
            draw,
            assets.menu_font.as_ref(),
            "WRITE TO 0x00000000  ->  undefined behavior",
            x + 25,
            y + 206,
            22.0,
            Color::new(255, 192, 105, 255),
        );
        draw_menu_text(
            draw,
            assets.menu_font.as_ref(),
            "$ ./footgun    [ENTER]",
            x + 25,
            y + 239,
            20.0,
            Color::WHITE,
        );
    }
}

fn draw_pointer_fault(draw: &mut impl DrawTarget, s: &SuperSequence, assets: &GameAssets) {
    let age = (s.tick - crate::combat::super_sequence::CPP_FOOTSHOT_TICK) as f32;
    let fade = if s.phase() == SuperPhase::CppHop {
        (1.0 - s.phase_progress()).min(0.5) * 2.0
    } else {
        1.0
    };
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
        let hop = (((s.tick - s.phase_span().start) as f32 / 12.0) * PI)
            .sin()
            .abs()
            * 23.0;
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

/// Each cell reveals the same screen-space slice from Sirius, never a prop collage.
fn draw_mutation(draw: &mut impl DrawTarget, s: &SuperSequence, assets: &GameAssets, fade: f32) {
    let progress = ((s.tick.saturating_sub(11)) as f32 / 114.0).clamp(0.0, 1.0);
    if s.tick < crate::combat::super_sequence::RUST_ARENA_COMMIT_TICK
        && let Some(texture) = assets.arenas.get(ArenaId::Sirius)
    {
        for row in 0..9 {
            for column in 0..16 {
                // A diagonal wave with a deterministic stagger is readable in either direction.
                let directed_column = if s.facing == crate::combat::fighter::Facing::Right {
                    column
                } else {
                    15 - column
                };
                let order = (directed_column as f32 / 15.0) * 0.57
                    + row as f32 / 8.0 * 0.26
                    + ((column * 7 + row * 11) % 5) as f32 * 0.012;
                let reveal = ((progress - order) / 0.09).clamp(0.0, 1.0);
                if reveal <= 0.0 {
                    continue;
                }
                let x = column as f32 * 80.0;
                let y = row as f32 * 80.0;
                let inset = (1.0 - reveal) * 40.0;
                let dest = Rectangle::new(x + inset, y + inset, 80.0 * reveal, 80.0 * reveal);
                let source = Rectangle::new(
                    (x + inset) / WINDOW_WIDTH as f32 * texture.width() as f32,
                    (y + inset) / WINDOW_HEIGHT as f32 * texture.height() as f32,
                    dest.width / WINDOW_WIDTH as f32 * texture.width() as f32,
                    dest.height / WINDOW_HEIGHT as f32 * texture.height() as f32,
                );
                draw.draw_texture_pro(texture, source, dest, Vector2::zero(), 0.0, Color::WHITE);
                draw.draw_rectangle_rec(dest, Color::new(0, 0, 0, 50));
                if reveal < 1.0 {
                    draw.draw_rectangle_lines_ex(dest, 2.0, alpha(AMBER, reveal));
                }
            }
        }
    }
    if s.phase() != SuperPhase::Restore {
        let label = if progress < 1.0 {
            "world = Sirius;  // applying &mut world"
        } else {
            "OWNERSHIP TRANSFERRED / SIRIUS"
        };
        draw.draw_rectangle(338, 214, 604, 46, alpha(Color::new(4, 10, 22, 255), 0.86));
        draw_menu_text(
            draw,
            assets.menu_font.as_ref(),
            label,
            360,
            223,
            25.0,
            alpha(AMBER, fade),
        );
        let target = Vector2::new(s.target_anchor.x, s.target_anchor.y - 85.0);
        if s.phase() == SuperPhase::RustCharge {
            let charge = s.phase_progress();
            draw.draw_ring(
                target,
                62.0 + charge * 30.0,
                65.0 + charge * 30.0,
                s.tick as f32 * 4.0,
                s.tick as f32 * 4.0 + 280.0,
                48,
                alpha(AMBER, 0.8),
            );
        }
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
                &format!(
                    "{} / terminal {:02}",
                    if s.character == CharacterId::Cpp {
                        "daughter.cpp"
                    } else {
                        "old-c.exe"
                    },
                    i + 1
                ),
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
            if s.character == CharacterId::Cpp {
                "INHERITED PROTECTION FAULT"
            } else {
                "GENERAL PROTECTION FAULT"
            },
            92,
            224,
            48.0,
            Color::WHITE,
        );
        draw_menu_text(
            draw,
            assets.menu_font.as_ref(),
            if s.character == CharacterId::Cpp {
                "C++ : public OLD_C  /  NULL POINTER"
            } else {
                "#GP  /  STOP 0x0000000D"
            },
            94,
            281,
            29.0,
            Color::WHITE,
        );
        for (i, line) in [
            if s.character == CharacterId::Cpp {
                "A fatal exception has occurred in FOOTGUN.CPP."
            } else {
                "A fatal exception has occurred in OLD_C.SYS."
            },
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
    let banner_y = if s.character == CharacterId::Python {
        WINDOW_HEIGHT - 82
    } else {
        116
    };
    draw.draw_rectangle(
        0,
        banner_y,
        WINDOW_WIDTH,
        76,
        alpha(Color::new(3, 7, 19, 255), 0.93 * fade),
    );
    draw.draw_rectangle(0, banner_y, WINDOW_WIDTH, 2, alpha(accent, fade));
    draw_menu_text(
        draw,
        assets.menu_font.as_ref(),
        s.label,
        30,
        banner_y + 5,
        35.0,
        alpha(Color::WHITE, fade),
    );
    let subtitle = match s.character {
        CharacterId::Rust => "Mutação de arena / Sirius Light Ring",
        CharacterId::Duke => "Chuva de lixo / Clones / Coleta",
        CharacterId::C => "Terminais / Tela azul / BIOS",
        CharacterId::Cpp => "Ponteiro nulo / Tiro no pé / Reboot",
        CharacterId::Python => "Transformação / Devorar / Celebrar",
        CharacterId::Go => "Goroutines / Canais",
    };
    draw_menu_text(
        draw,
        assets.menu_font.as_ref(),
        subtitle,
        32,
        banner_y + 44,
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
        banner_y + 35,
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
