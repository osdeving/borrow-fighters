//! Composes Ada's illustrated awakening, Rust's morning and the playable encounter.
//!
//! System: Adventure renderer. It reads domain snapshots, adds visual motion and
//! draws its own interface; drawing never changes contact or story outcomes.

use super::assets::Assets;
use crate::adventure::{
    arrival::ArrivalShot,
    combat::{Action, Actor, FLOOR_Y, Facing, Outcome},
    story::{PrologueBeat, Stage, Story},
};
use raylib::prelude::*;

/// Logical width shared by the adventure's renderer and letterboxed window.
pub const WIDTH: i32 = 1280;
/// Logical height; resizing changes presentation only.
pub const HEIGHT: i32 = 720;
const INK: Color = Color::new(16, 23, 28, 255);
const PAPER: Color = Color::new(247, 235, 213, 255);
const GOLD: Color = Color::new(232, 177, 92, 255);
const MINT: Color = Color::new(125, 218, 190, 255);

/// Brief feedback for an explicit author-initiated text reload.
pub fn reload_notice(d: &mut impl RaylibDraw, a: &Assets, succeeded: bool) {
    d.draw_rectangle(25, 116, 1230, 48, alpha(INK, 0.96));
    text(
        d,
        a,
        a.text.get(if succeeded {
            "editor.reloaded"
        } else {
            "editor.failed"
        }),
        42.0,
        130.0,
        23.0,
        if succeeded { MINT } else { GOLD },
    );
}

/// Draws context-specific skip controls or the hosted title's continue prompt.
pub fn navigation(
    d: &mut impl RaylibDraw,
    a: &Assets,
    story: &Story,
    hosted: bool,
    paused: bool,
    pulse_seconds: f32,
) {
    if paused {
        super::typography::centered(
            d,
            &a.body,
            a.text.get(if hosted {
                "navigation.pause_menu"
            } else {
                "navigation.pause"
            }),
            Vector2::new(640.0, 365.0),
            1100.0,
            23.0,
            PAPER,
        );
        return;
    }
    if story.stage == Stage::Complete {
        if hosted {
            d.draw_rectangle(0, 594, WIDTH, 126, INK);
            let pulse = 0.78 + 0.22 * (pulse_seconds * 2.5).sin().mul_add(0.5, 0.5);
            super::typography::centered(
                d,
                &a.body,
                a.text.get("navigation.continue"),
                Vector2::new(640.0, 620.0),
                1180.0,
                27.0,
                alpha(GOLD, pulse),
            );
            super::typography::centered(
                d,
                &a.body,
                a.text.get("navigation.continue_detail"),
                Vector2::new(640.0, 663.0),
                1180.0,
                19.0,
                alpha(PAPER, 0.85),
            );
        }
        return;
    }
    if story.arrival_active() {
        d.draw_rectangle(0, 670, WIDTH, 50, alpha(INK, 0.92));
        super::typography::centered(
            d,
            &a.body,
            a.text.get("street.arrival.controls"),
            Vector2::new(640.0, 685.0),
            1160.0,
            18.0,
            PAPER,
        );
        return;
    }
    let encounter = story.stage == Stage::Encounter;
    let (top, height, baseline) = if encounter {
        (636, 32, 643.0)
    } else {
        (664, 56, 683.0)
    };
    d.draw_rectangle(0, top, WIDTH, height, INK);
    super::typography::centered(
        d,
        &a.body,
        a.text.get(match (story.stage, hosted) {
            (Stage::AdaPrologue, true) => "navigation.ada_controls_menu",
            (Stage::AdaPrologue, false) => "navigation.ada_controls",
            (_, true) => "navigation.controls_menu",
            (_, false) => "navigation.controls",
        }),
        Vector2::new(640.0, baseline),
        1200.0,
        if encounter { 18.0 } else { 19.0 },
        alpha(PAPER, 0.85),
    );
}

/// Draws a complete frame at the logical resolution.
pub fn draw(
    d: &mut impl RaylibDraw,
    story: &Story,
    assets: &Assets,
    paused: bool,
    debug: bool,
    reveal_text: bool,
) {
    d.clear_background(INK);
    match story.stage {
        Stage::AdaPrologue => ada(d, story, assets, reveal_text),
        Stage::RustMorning => morning(d, story, assets),
        Stage::Opening | Stage::Complete => super::opening::draw(d, story, assets),
        Stage::Encounter | Stage::Aftermath => encounter(d, story, assets, debug),
    }
    if story.stage == Stage::Complete {
        ending(d, assets);
    }
    if story.combat.outcome == Outcome::Defeat {
        defeat(d, assets);
    }
    if paused {
        pause(d, assets);
    }
}

fn ada(d: &mut impl RaylibDraw, story: &Story, a: &Assets, reveal_text: bool) {
    let beat = story
        .prologue_beat()
        .unwrap_or(PrologueBeat::AdaOrdinaryLife);
    let index = PrologueBeat::ALL
        .iter()
        .position(|b| *b == beat)
        .unwrap_or(0);
    let t = story.beat_ticks() as f32 / 60.0;
    let duration = beat.duration_ticks() as f32 / 60.0;
    let source = cell(&a.ada, 3, 2, index);
    cinematic_panel(d, &a.ada, source, t / duration, Color::WHITE);
    if index > 0 && t < 0.65 {
        cinematic_panel(
            d,
            &a.ada,
            cell(&a.ada, 3, 2, index - 1),
            1.0,
            alpha(Color::WHITE, 1.0 - t / 0.65),
        );
    }
    if (2..=4).contains(&index) {
        for i in 0..26 {
            let f = i as f32;
            let x = 710.0 + (f * 19.13 + t * 0.26).sin() * 290.0;
            let y = 340.0 + (f * 7.7 - t * 0.32).cos() * 230.0;
            let pulse = ((t * 1.7 + f).sin() + 1.0) * 0.35;
            d.draw_circle_v(Vector2::new(x, y), 1.5 + pulse, alpha(MINT, pulse));
        }
    }
    d.draw_rectangle_gradient_v(0, 460, WIDTH, 260, Color::BLANK, alpha(INK, 0.97));
    d.draw_rectangle(0, 0, WIDTH, 44, alpha(INK, 0.9));
    text(d, a, a.text.get("ada.eyebrow"), 42.0, 13.0, 17.0, GOLD);
    let subtitle = match beat {
        PrologueBeat::AdaOrdinaryLife => a.text.get("ada.caption.ordinary"),
        PrologueBeat::StrangeMessage => a.text.get("ada.caption.message"),
        PrologueBeat::FollowingTheSignal => a.text.get("ada.caption.signal"),
        PrologueBeat::AssemblyAwakens => a.text.get("ada.caption.assembly"),
        PrologueBeat::AfterTheContact => a.text.get("ada.caption.after"),
        PrologueBeat::LongYears => "",
    };
    if beat == PrologueBeat::StrangeMessage {
        let opacity = (t / 0.7).min(1.0);
        d.draw_rectangle_rounded(
            Rectangle::new(685.0, 216.0, 545.0, 243.0),
            0.04,
            8,
            alpha(INK, opacity * 0.94),
        );
        d.draw_rectangle(706, 238, 3, 22, alpha(MINT, opacity));
        text(
            d,
            a,
            a.text.get("ada.sender"),
            722.0,
            238.0,
            17.0,
            alpha(GOLD, opacity),
        );
        let message = a.text.get("ada.message");
        type_text(
            d,
            a,
            message,
            if reveal_text {
                usize::MAX
            } else {
                ((t - 0.6).max(0.0) * 23.0) as usize
            },
            Vector2::new(710.0, 286.0),
            23.0,
            MINT,
        );
        if (t * 2.0) as i32 % 2 == 0 {
            text(d, a, a.text.get("ada.cursor"), 712.0, 417.0, 24.0, MINT);
        }
    }
    if beat == PrologueBeat::LongYears {
        d.draw_rectangle(0, 44, WIDTH, HEIGHT - 44, alpha(INK, (t / 1.0).min(0.9)));
        heading(
            d,
            a,
            &a.text
                .get("ada.years")
                .chars()
                .take((t * 16.0) as usize)
                .collect::<String>(),
            365.0,
            288.0,
            46.0,
            PAPER,
        );
        text(
            d,
            a,
            a.text.get("ada.years_detail"),
            492.0,
            353.0,
            25.0,
            GOLD,
        );
    } else {
        super::typography::paragraph(
            d,
            &a.body,
            subtitle,
            Rectangle::new(70.0, 585.0, 1140.0, 74.0),
            28.0,
            PAPER,
        );
    }
    footer(d, a, a.text.get("ada.controls"));
    for i in 0..6 {
        d.draw_rectangle(
            1080 + i * 23,
            26,
            16,
            3,
            if i as usize <= index {
                GOLD
            } else {
                alpha(PAPER, 0.25)
            },
        );
    }
}

fn morning(d: &mut impl RaylibDraw, story: &Story, a: &Assets) {
    let t = story.stage_ticks as f32 / 60.0;
    background(d, &a.environments, 0, 0.0, 1.0);
    super::morning::draw_room_details(d, story, a);
    for i in 0..26 {
        let f = i as f32;
        let x = 720.0 + (f * 18.7).sin() * 270.0 + t * 3.0;
        let y = 80.0 + (f * 47.0 + t * 11.0) % 440.0;
        d.draw_circle_v(Vector2::new(x, y), 1.7, alpha(GOLD, 0.3));
    }
    super::morning::draw_morning_character(d, story, a);
    d.draw_rectangle_gradient_v(0, 530, WIDTH, 190, Color::BLANK, alpha(INK, 0.93));
    text(d, a, a.text.get("morning.eyebrow"), 49.0, 47.0, 18.0, INK);
    if t > 9.5 {
        heading(d, a, a.text.get("morning.name"), 65.0, 576.0, 48.0, PAPER);
        text(
            d,
            a,
            a.text.get("morning.caption"),
            205.0,
            597.0,
            26.0,
            PAPER,
        );
    }
    if t < 1.0 {
        d.draw_rectangle(0, 0, WIDTH, HEIGHT, alpha(INK, 1.0 - t));
    }
    if t > 13.25 {
        d.draw_rectangle(0, 0, WIDTH, HEIGHT, alpha(INK, (t - 13.25) / 0.75));
    }
    footer(d, a, a.text.get("morning.controls"));
}

fn encounter(d: &mut impl RaylibDraw, story: &Story, a: &Assets, debug: bool) {
    let c = &story.combat;
    let camera = (c.player.position.x - 450.0).clamp(0.0, 920.0);
    let arriving = story.arrival_active();
    let shot = if arriving {
        ArrivalShot::at(story.stage_ticks)
    } else {
        ArrivalShot::settled()
    };
    {
        let mut world = d.begin_mode2D(Camera2D {
            offset: Vector2::new(640.0, 360.0),
            target: Vector2::new(shot.target.x, shot.target.y),
            rotation: 0.0,
            zoom: shot.zoom,
        });
        let d = &mut world;
        super::street::background(d, a, camera);
        d.draw_rectangle_gradient_v(
            0,
            565,
            WIDTH,
            155,
            Color::BLANK,
            Color::new(44, 42, 39, 255),
        );
        d.draw_line(
            0,
            FLOOR_Y as i32 + 3,
            WIDTH,
            FLOOR_Y as i32 + 3,
            alpha(PAPER, 0.2),
        );
        let scene_time = story.ambient.ticks() as f32 / 60.0;
        super::street::draw(d, &story.ambient, a, camera);
        for i in 0..12 {
            let f = i as f32;
            let x = (f * 197.0 + scene_time * 13.0 - camera * 0.5).rem_euclid(1400.0) - 60.0;
            let y = 80.0 + (scene_time * 0.35 + f).sin() * 24.0 + f * 27.0;
            d.draw_circle_v(Vector2::new(x, y), 1.3, alpha(GOLD, 0.35));
        }
        actor_shadow(d, &c.player, camera);
        // The same awakening signal reveals the threat and startles the child.
        if c.enemy_awake {
            actor_shadow(d, &c.enemy, camera);
            creature(d, a, &c.enemy, camera);
        }
        rust(d, a, &c.player, camera);
        if let Some(hit) = c.last_hit {
            let p = Vector2::new(hit.position.x - camera, hit.position.y);
            let age = hit.age_ticks as f32;
            let opacity = (1.0 - age / 16.0).max(0.0);
            let color = if hit.blocked { MINT } else { GOLD };
            for i in 0..9 {
                let angle = i as f32 * std::f32::consts::TAU / 9.0;
                let r = 7.0 + age * 2.5;
                d.draw_line_ex(
                    Vector2::new(p.x + angle.cos() * r, p.y + angle.sin() * r),
                    Vector2::new(
                        p.x + angle.cos() * (r + 12.0),
                        p.y + angle.sin() * (r + 12.0),
                    ),
                    2.0,
                    alpha(color, opacity),
                );
            }
            d.draw_circle_v(p, 6.0 + age, alpha(color, opacity * 0.45));
        }
        if debug {
            for actor in [&c.player, &c.enemy] {
                let b = actor.hurtbox();
                d.draw_rectangle_lines_ex(
                    Rectangle::new(b.x - camera, b.y, b.width, b.height),
                    2.0,
                    MINT,
                );
                if let Some(b) = actor.attack_hitbox() {
                    d.draw_rectangle_lines_ex(
                        Rectangle::new(b.x - camera, b.y, b.width, b.height),
                        2.0,
                        Color::RED,
                    );
                }
            }
        }
    }
    if arriving {
        let border = (42.0 * shot.matte) as i32;
        d.draw_rectangle(0, 0, WIDTH, border, alpha(INK, 0.94));
        d.draw_rectangle(0, HEIGHT - border, WIDTH, border, alpha(INK, 0.94));
    }
    if story.stage == Stage::Encounter && c.outcome == Outcome::Ongoing && !arriving {
        d.draw_rectangle_rounded(
            Rectangle::new(30.0, 26.0, 292.0, 85.0),
            0.08,
            8,
            alpha(INK, 0.87),
        );
        text(d, a, a.text.get("encounter.name"), 50.0, 38.0, 21.0, PAPER);
        d.draw_rectangle(50, 76, 247, 7, alpha(PAPER, 0.2));
        d.draw_rectangle(
            50,
            76,
            (247.0 * c.player.hp as f32 / c.player.max_hp as f32) as i32,
            7,
            GOLD,
        );
        let objective = if c.enemy_awake {
            a.text.get("encounter.objective")
        } else {
            a.text.get("encounter.explore")
        };
        d.draw_rectangle_rounded(
            Rectangle::new(927.0, 26.0, 323.0, 72.0),
            0.08,
            8,
            alpha(INK, 0.85),
        );
        text(d, a, objective, 950.0, 40.0, 19.0, PAPER);
        if c.enemy_awake {
            let x = c.enemy.position.x - camera;
            d.draw_rectangle((x - 44.0) as i32, 374, 88, 4, alpha(INK, 0.55));
            d.draw_rectangle(
                (x - 44.0) as i32,
                374,
                (88.0 * c.enemy.hp as f32 / c.enemy.max_hp as f32) as i32,
                4,
                Color::new(183, 80, 81, 255),
            );
            if c.enemy.action == Action::Telegraph {
                text(
                    d,
                    a,
                    a.text.get("encounter.alert"),
                    x - 6.0,
                    336.0,
                    31.0,
                    Color::new(191, 66, 52, 255),
                );
            }
        } else if story.stage_ticks > 80 {
            text(
                d,
                a,
                a.text.get("encounter.direction"),
                950.0,
                68.0,
                21.0,
                GOLD,
            );
        }
        footer(d, a, a.text.get("encounter.controls"));
    }
    if story.stage == Stage::Aftermath {
        d.draw_rectangle_gradient_v(0, 500, WIDTH, 220, Color::BLANK, alpha(INK, 0.9));
        if c.player.action == Action::Remorse {
            heading(
                d,
                a,
                a.text.get("aftermath.caption"),
                69.0,
                606.0,
                31.0,
                PAPER,
            );
        }
    }
}

fn actor_shadow(d: &mut impl RaylibDraw, actor: &Actor, camera: f32) {
    d.draw_ellipse(
        (actor.position.x - camera) as i32,
        FLOOR_Y as i32 + 2,
        37.0,
        7.0,
        alpha(INK, 0.22),
    );
}

fn rust(d: &mut impl RaylibDraw, a: &Assets, actor: &Actor, camera: f32) {
    let pos = Vector2::new(actor.position.x - camera, actor.position.y);
    if actor.action == Action::Remorse {
        let frame = match actor.action_ticks {
            0..=29 => 8,
            30..=59 => 9,
            60..=89 => 10,
            90..=119 => 9,
            _ => 11,
        };
        let tallest = a
            .morning_bounds
            .iter()
            .skip(6)
            .map(|r| r.height)
            .fold(1.0, f32::max);
        pose(
            d,
            &a.morning,
            a.morning_bounds[frame],
            pos,
            174.0 / tallest,
            actor.facing == Facing::Left,
            Color::WHITE,
        );
        return;
    }
    let frame = match actor.action {
        Action::Walk => 2 + (actor.action_ticks / 7 % 4) as usize,
        Action::Jump if actor.velocity.y < 0.0 => 6,
        Action::Jump => 7,
        Action::LightAttack if actor.action_ticks < 5 => 8,
        Action::LightAttack if actor.action_ticks < 10 => 9,
        Action::LightAttack => 10,
        Action::HeavyAttack if actor.action_ticks < 13 => 11,
        Action::HeavyAttack if actor.action_ticks < 20 => 12,
        Action::Block => 13,
        Action::Hurt => 14,
        Action::Defeated => 15,
        _ => (actor.action_ticks / 30 % 2) as usize,
    };
    let tallest = a
        .action_bounds
        .iter()
        .take(15)
        .map(|r| r.height)
        .fold(1.0, f32::max);
    pose(
        d,
        &a.actions,
        a.action_bounds[frame],
        pos,
        174.0 / tallest,
        actor.facing == Facing::Left,
        Color::WHITE,
    );
    if actor.action == Action::HeavyAttack && (13..20).contains(&actor.action_ticks) {
        let x = pos.x + actor.facing.sign() * 65.0;
        let y = pos.y - 115.0;
        let t = (actor.action_ticks - 13) as f32;
        d.draw_circle_lines(
            x as i32,
            y as i32,
            14.0 + t * 2.0,
            alpha(GOLD, 1.0 - t / 8.0),
        );
        for i in 0..4 {
            let angle = i as f32 * std::f32::consts::FRAC_PI_2 + t * 0.13;
            let p = Vector2::new(x + angle.cos() * 23.0, y + angle.sin() * 23.0);
            d.draw_rectangle(p.x as i32 - 2, p.y as i32 - 2, 4, 4, GOLD);
        }
    }
}

fn creature(d: &mut impl RaylibDraw, a: &Assets, actor: &Actor, camera: f32) {
    let frame = match actor.action {
        Action::Walk => 1 + (actor.action_ticks / 10 % 2) as usize,
        Action::Telegraph => 3,
        Action::Lunge => 4,
        Action::Hurt => 5,
        Action::Defeated if actor.action_ticks < 24 => 6,
        Action::Defeated => 7,
        _ => 0,
    };
    let height = a
        .erratic_bounds
        .iter()
        .take(6)
        .map(|r| r.height)
        .fold(1.0, f32::max);
    let tremor = if actor.hp > 0 {
        (actor.action_ticks as f32 * 0.7).sin() * 1.2
    } else {
        0.0
    };
    pose(
        d,
        &a.erratic,
        a.erratic_bounds[frame],
        Vector2::new(actor.position.x - camera + tremor, actor.position.y),
        187.0 / height,
        actor.facing == Facing::Left,
        Color::WHITE,
    );
}

fn pose(
    d: &mut impl RaylibDraw,
    texture: &Texture2D,
    source: Rectangle,
    feet: Vector2,
    scale: f32,
    flip: bool,
    tint: Color,
) {
    let width = source.width * scale;
    let height = source.height * scale;
    let src = Rectangle::new(
        source.x,
        source.y,
        if flip { -source.width } else { source.width },
        source.height,
    );
    d.draw_texture_pro(
        texture,
        src,
        Rectangle::new(feet.x, feet.y, width, height),
        Vector2::new(width * 0.5, height),
        0.0,
        tint,
    );
}

fn background(d: &mut impl RaylibDraw, texture: &Texture2D, index: usize, x: f32, scale: f32) {
    d.draw_texture_pro(
        texture,
        cell(texture, 1, 2, index),
        Rectangle::new(x, 0.0, WIDTH as f32 * scale, HEIGHT as f32),
        Vector2::zero(),
        0.0,
        Color::WHITE,
    );
    // The street's wide crop ends beyond the furthest camera position.
}

fn cinematic_panel(
    d: &mut impl RaylibDraw,
    texture: &Texture2D,
    source: Rectangle,
    progress: f32,
    tint: Color,
) {
    let zoom = 1.0 + progress.clamp(0.0, 1.0) * 0.035;
    let width = source.width / zoom;
    let height = source.height / zoom;
    let crop = Rectangle::new(
        source.x + (source.width - width) * 0.5,
        source.y + (source.height - height) * 0.4,
        width,
        height,
    );
    d.draw_texture_pro(
        texture,
        crop,
        Rectangle::new(0.0, 0.0, WIDTH as f32, HEIGHT as f32),
        Vector2::zero(),
        0.0,
        tint,
    );
}

fn cell(texture: &Texture2D, columns: usize, rows: usize, index: usize) -> Rectangle {
    let w = texture.width() as f32 / columns as f32;
    let h = texture.height() as f32 / rows as f32;
    Rectangle::new(
        (index % columns) as f32 * w,
        (index / columns) as f32 * h,
        w,
        h,
    )
}

fn ending(d: &mut impl RaylibDraw, a: &Assets) {
    super::typography::centered(
        d,
        &a.body,
        a.text.get("ending.title"),
        Vector2::new(640.0, 610.0),
        1180.0,
        24.0,
        PAPER,
    );
    footer(d, a, a.text.get("ending.controls"));
}

fn defeat(d: &mut impl RaylibDraw, a: &Assets) {
    d.draw_rectangle(0, 0, WIDTH, HEIGHT, alpha(INK, 0.7));
    heading(d, a, a.text.get("defeat.title"), 324.0, 256.0, 41.0, PAPER);
    text(
        d,
        a,
        a.text.get("defeat.caption"),
        327.0,
        330.0,
        24.0,
        PAPER,
    );
    text(
        d,
        a,
        a.text.get("defeat.controls"),
        396.0,
        414.0,
        24.0,
        GOLD,
    );
}

fn pause(d: &mut impl RaylibDraw, a: &Assets) {
    d.draw_rectangle(0, 0, WIDTH, HEIGHT, alpha(INK, 0.87));
    heading(d, a, a.text.get("pause.title"), 470.0, 213.0, 47.0, PAPER);
    text(d, a, a.text.get("pause.resume"), 446.0, 316.0, 25.0, GOLD);
    text(
        d,
        a,
        a.text.get("pause.controls"),
        275.0,
        455.0,
        21.0,
        PAPER,
    );
}

fn footer(d: &mut impl RaylibDraw, a: &Assets, value: &str) {
    d.draw_rectangle(0, 668, WIDTH, 52, alpha(INK, 0.93));
    text(d, a, value, 36.0, 684.0, 19.0, alpha(PAPER, 0.85));
}

fn text(d: &mut impl RaylibDraw, a: &Assets, value: &str, x: f32, y: f32, size: f32, color: Color) {
    let measured = a.body.measure_text(value, size, 0.2).x.max(1.0);
    let fitted = size * ((WIDTH as f32 - x - 30.0) / measured).min(1.0);
    d.draw_text_ex(&a.body, value, Vector2::new(x, y), fitted, 0.2, color);
}

fn heading(
    d: &mut impl RaylibDraw,
    a: &Assets,
    value: &str,
    x: f32,
    y: f32,
    size: f32,
    color: Color,
) {
    let measured = a.title.measure_text(value, size, 0.2).x.max(1.0);
    let fitted = size * ((WIDTH as f32 - x - 30.0) / measured).min(1.0);
    d.draw_text_ex(&a.title, value, Vector2::new(x, y), fitted, 0.2, color);
}

fn type_text(
    d: &mut impl RaylibDraw,
    a: &Assets,
    value: &str,
    characters: usize,
    position: Vector2,
    size: f32,
    color: Color,
) {
    let visible: String = value.chars().take(characters).collect();
    super::typography::paragraph(
        d,
        &a.body,
        &visible,
        Rectangle::new(position.x, position.y, 492.0, 157.0),
        size,
        color,
    );
}

fn alpha(color: Color, value: f32) -> Color {
    Color::new(
        color.r,
        color.g,
        color.b,
        (255.0 * value.clamp(0.0, 1.0)) as u8,
    )
}
