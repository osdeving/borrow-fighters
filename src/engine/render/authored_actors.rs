//! Draws Duke's collectors and C++'s footgun story from the authoritative super clock.
//!
//! System: Raylib presentation. New action atlases replace the relevant caster
//! poses; their decorations never move the captured fighters or apply damage.

use std::f32::consts::PI;

use raylib::prelude::*;

use crate::{
    characters::CharacterId,
    combat::{
        fighter::{Facing, PlayerSlot},
        super_sequence::{
            CPP_BARRAGE_CADENCE, CPP_BARRAGE_START, CPP_FOOTSHOT_TICK, DUKE_CLONE_CADENCE,
            DUKE_CLONE_COUNT, DUKE_COLLECT_START, SuperPhase, SuperSequence,
        },
    },
    config::{FLOOR_Y, WINDOW_WIDTH},
    game::world::World,
};

use super::{DrawTarget, draw_menu_text};

const CLONE_SCALE: f32 = 0.38;
const CPP_SCALE: f32 = 0.54;

/// Optional resources for the two authored actor stories.
#[derive(Clone, Copy, Default)]
pub(super) struct AuthoredActorTextures<'a> {
    pub duke: Option<&'a Texture2D>,
    pub cpp_comedy: Option<&'a Texture2D>,
    pub cpp_barrage: Option<&'a Texture2D>,
    pub trash: Option<&'a Texture2D>,
    pub font: Option<&'a Font>,
}

#[derive(Clone, Copy)]
struct Frame {
    source: [f32; 4],
    pivot: [f32; 2],
    pieces: &'static [[f32; 4]],
}

const fn frame(source: [f32; 4], pivot: [f32; 2]) -> Frame {
    Frame {
        source,
        pivot,
        pieces: &[],
    }
}

// Explicit alpha crops include detached steam. Every action uses a shared
// character scale; crouches and jumps retain their intended height differences.
const DUKE: [Frame; 8] = [
    frame([17., 27., 370., 453.], [207., 453.]),
    frame([435., 126., 354., 367.], [144., 364.]),
    frame([831., 38., 289., 451.], [144., 452.]),
    frame([1228., 66., 269., 423.], [133., 424.]),
    frame([94., 546., 259., 445.], [129., 445.]),
    frame([421., 520., 319., 389.], [162., 471.]),
    frame([806., 532., 344., 441.], [175., 446.]),
    frame([1171., 621., 348., 356.], [177., 359.]),
];

const CPP_COMEDY: [Frame; 6] = [
    frame([152., 8., 334., 513.], [149., 511.]),
    Frame {
        source: [612., 0., 363., 521.],
        pivot: [173., 519.],
        // The authored toe and next row's hair share three scanlines. Draw
        // separate source pieces instead of altering the image or clipping feet.
        pieces: &[[612., 0., 363., 514.], [661., 514., 50., 7.]],
    },
    frame([1148., 3., 255., 518.], [144., 516.]),
    frame([115., 521., 296., 493.], [158., 492.]),
    Frame {
        source: [640., 514., 312., 505.],
        pivot: [160., 502.],
        pieces: &[[766., 514., 75., 7.], [640., 521., 312., 498.]],
    },
    frame([1038., 546., 444., 463.], [222., 467.]),
];

const CPP_BARRAGE: [Frame; 6] = [
    frame([11., 33., 455., 460.], [271., 472.]),
    frame([530., 18., 519., 486.], [229., 487.]),
    frame([1099., 15., 409., 495.], [195., 494.]),
    frame([15., 495., 483., 513.], [164., 512.]),
    frame([512., 504., 550., 506.], [155., 506.]),
    frame([1060., 513., 449., 497.], [205., 497.]),
];

const TRASH: [Frame; 3] = [
    frame([85., 101., 754., 538.], [377., 538.]),
    frame([898., 135., 624., 453.], [312., 453.]),
    frame([1637., 181., 463., 434.], [231.5, 434.]),
];

/// Suppresses only the caster pose actually replaced by a loaded authored atlas.
pub(super) fn hides_actor(
    world: &World,
    slot: PlayerSlot,
    textures: AuthoredActorTextures<'_>,
) -> bool {
    let Some(sequence) = world.super_sequence() else {
        return false;
    };
    if sequence.attacker != slot {
        return false;
    }
    match sequence.character {
        CharacterId::Duke => textures.duke.is_some() && sequence.phase() != SuperPhase::Freeze,
        CharacterId::Cpp => {
            textures.cpp_comedy.is_some()
                && textures.cpp_barrage.is_some()
                && sequence.phase() != SuperPhase::Freeze
        }
        _ => false,
    }
}

/// Draws collectors, settled litter, giant Duke, and C++'s authored action poses.
pub(super) fn draw_authored_actors(
    draw: &mut impl DrawTarget,
    world: &World,
    textures: AuthoredActorTextures<'_>,
) {
    let Some(sequence) = world.super_sequence() else {
        return;
    };
    match sequence.character {
        CharacterId::Duke => {
            if let Some(duke) = textures.duke {
                draw_duke_story(draw, sequence, duke, textures.trash);
            }
        }
        CharacterId::Cpp => {
            if let (Some(comedy), Some(barrage)) = (textures.cpp_comedy, textures.cpp_barrage) {
                draw_cpp_story(draw, sequence, comedy, barrage, textures.font);
            }
        }
        _ => {}
    }
}

fn draw_duke_story(
    draw: &mut impl DrawTarget,
    sequence: &SuperSequence,
    duke: &Texture2D,
    trash: Option<&Texture2D>,
) {
    let phase = sequence.phase();
    if matches!(
        phase,
        SuperPhase::DukeTrashFall | SuperPhase::DukeTrashSettle | SuperPhase::DukeCollect
    ) {
        let mut anchor = Vector2::new(sequence.attacker_anchor.x, sequence.attacker_anchor.y);
        anchor.y -= (sequence.tick as f32 * 0.16).sin().abs() * 2.;
        let key = if (sequence.tick / 12).is_multiple_of(2) {
            4
        } else {
            3
        };
        draw_frame(
            draw,
            duke,
            DUKE[key],
            anchor,
            0.56,
            sequence.facing == Facing::Right,
            1.,
        );
    }
    if matches!(
        phase,
        SuperPhase::DukeTrashFall | SuperPhase::DukeTrashSettle | SuperPhase::DukeCollect
    ) || phase == SuperPhase::DukeGiantDrop && sequence.tick < 234
    {
        for index in 0..DUKE_CLONE_COUNT {
            let point = trash_anchor(index);
            let collect_tick = DUKE_COLLECT_START + index * DUKE_CLONE_CADENCE;
            if trash_is_visible(index, sequence.tick) {
                let delay = 8 + index * 2;
                let progress = (sequence.tick.saturating_sub(delay) as f32 / 28.).clamp(0., 1.);
                let feet = Vector2::new(point.x, -60. + (point.y + 60.) * progress * progress);
                shadow(draw, point, 17., progress * 0.32);
                draw_litter(draw, trash, index, feet, 35., 1.);
            }
            let local = sequence.tick as i32 - collect_tick as i32;
            if (-12..18).contains(&local) && phase != SuperPhase::DukeTrashFall {
                let right = index.is_multiple_of(2);
                let sign = if right { 1. } else { -1. };
                // The pickup glove reaches 168 authored pixels past its pivot.
                let mut anchor = Vector2::new(point.x - sign * 168. * CLONE_SCALE, point.y);
                let (key, opacity) = match local {
                    -12..=-5 => {
                        let p = (local + 12) as f32 / 8.;
                        anchor.y = -25. + (point.y + 25.) * p * p;
                        (0, 1.)
                    }
                    -4..=-1 => (1, 1.),
                    0..=3 => (2, 1.),
                    4..=9 => (if local < 7 { 3 } else { 4 }, 1.),
                    _ => {
                        let p = (local - 10) as f32 / 8.;
                        anchor.y -= p * 55.;
                        (5, 1. - p)
                    }
                };
                shadow(draw, Vector2::new(anchor.x, point.y), 29., opacity * 0.32);
                draw_frame(draw, duke, DUKE[key], anchor, CLONE_SCALE, right, opacity);
                if (0..=2).contains(&local) {
                    let bite = local as f32 / 3.;
                    let mouth = Vector2::new(anchor.x + sign * (24. - bite * 12.), anchor.y - 95.);
                    draw_litter(
                        draw,
                        trash,
                        index,
                        mouth,
                        24. * (1. - bite * 0.6),
                        1. - bite,
                    );
                }
            }
        }
    }

    let right = sequence.facing == Facing::Right;
    let target = Vector2::new(sequence.target_origin.x, FLOOR_Y);
    match phase {
        SuperPhase::DukeGiantDrop => {
            let p = sequence.phase_progress();
            let anchor = Vector2::new(target.x, -80. + (FLOOR_Y + 80.) * p * p);
            shadow(draw, target, 190. * p, p * 0.56);
            draw_frame(draw, duke, DUKE[6], anchor, 1.42, right, 1.);
        }
        SuperPhase::DukeImpact => {
            let p = sequence.phase_progress();
            shadow(draw, target, 170., 0.6);
            draw_frame(draw, duke, DUKE[7], target, 1.42, right, 1.);
            for side in [-1., 1.] {
                for index in 0..4 {
                    let x = target.x + side * (70. + index as f32 * 37. + p * 140.);
                    draw.draw_circle_gradient(
                        x as i32,
                        (FLOOR_Y - 10.) as i32,
                        16. + p * 25.,
                        Color::new(219, 196, 145, ((1. - p) * 135.) as u8),
                        Color::BLANK,
                    );
                }
            }
        }
        SuperPhase::Restore => {
            let p = smooth(sequence.phase_progress());
            let anchor = Vector2::new(
                target.x + (sequence.attacker_anchor.x - target.x) * p,
                FLOOR_Y - (p * PI).sin() * 80.,
            );
            draw_frame(
                draw,
                duke,
                DUKE[4],
                anchor,
                1.42 + (0.56 - 1.42) * p,
                right,
                1.,
            );
        }
        _ => {}
    }
}

fn trash_anchor(index: u32) -> Vector2 {
    Vector2::new(
        90. + (WINDOW_WIDTH as f32 - 180.) * (index as f32 + 0.5) / DUKE_CLONE_COUNT as f32,
        FLOOR_Y - (index % 3) as f32 * 5.,
    )
}

fn trash_is_visible(index: u32, tick: u32) -> bool {
    tick >= 8 + index * 2 && tick < DUKE_COLLECT_START + index * DUKE_CLONE_CADENCE - 2
}

fn draw_litter(
    draw: &mut impl DrawTarget,
    texture: Option<&Texture2D>,
    index: u32,
    anchor: Vector2,
    width: f32,
    opacity: f32,
) {
    if let Some(texture) = texture {
        let frame = TRASH[index as usize % TRASH.len()];
        draw_frame(
            draw,
            texture,
            frame,
            anchor,
            width / frame.source[2],
            true,
            opacity,
        );
    }
}

fn draw_cpp_story(
    draw: &mut impl DrawTarget,
    sequence: &SuperSequence,
    comedy: &Texture2D,
    barrage: &Texture2D,
    font: Option<&Font>,
) {
    let mut anchor = Vector2::new(sequence.attacker_anchor.x, sequence.attacker_anchor.y);
    let right = sequence.facing == Facing::Right;
    let sign = if right { 1. } else { -1. };
    let (texture, frame) = match sequence.phase() {
        SuperPhase::Freeze => return,
        SuperPhase::CppFootshot => {
            if sequence.tick >= CPP_FOOTSHOT_TICK {
                let local = (sequence.tick - CPP_FOOTSHOT_TICK) as f32;
                anchor.y -= (local / 18. * PI).sin().max(0.) * 12.;
                draw_footshot(
                    draw,
                    font,
                    Vector2::new(anchor.x + sign * 59., FLOOR_Y - 10.),
                    local,
                );
                (comedy, CPP_COMEDY[1])
            } else {
                (comedy, CPP_COMEDY[0])
            }
        }
        SuperPhase::CppHop => {
            let local = sequence.tick - sequence.phase_span().start;
            anchor.y -= ((local as f32 / 12.) * PI).sin().abs() * 23.;
            (
                comedy,
                CPP_COMEDY[if (local / 6).is_multiple_of(2) { 2 } else { 3 }],
            )
        }
        SuperPhase::CppRage => {
            anchor.x += (sequence.tick as f32 * 1.9).sin() * 2.;
            (comedy, CPP_COMEDY[4])
        }
        SuperPhase::CppCharge => {
            let local = sequence.tick - sequence.phase_span().start;
            anchor.y -= (local as f32 * 0.6).sin().abs() * 7.;
            for index in 0..5 {
                let y = FLOOR_Y - 40. - index as f32 * 31.;
                draw.draw_line_ex(
                    Vector2::new(anchor.x - sign * 60., y),
                    Vector2::new(anchor.x - sign * (140. + index as f32 * 11.), y + 3.),
                    2.,
                    Color::new(255, 213, 100, 75),
                );
            }
            if (local / 5).is_multiple_of(2) {
                (barrage, CPP_BARRAGE[0])
            } else {
                (comedy, CPP_COMEDY[5])
            }
        }
        SuperPhase::CppBarrage => {
            let local = sequence.tick - CPP_BARRAGE_START;
            let key = [1, 3, 2, 4][(local / CPP_BARRAGE_CADENCE) as usize % 4];
            if local % CPP_BARRAGE_CADENCE < 7 {
                (barrage, CPP_BARRAGE[key])
            } else {
                (comedy, CPP_COMEDY[4])
            }
        }
        SuperPhase::CppFinisher => (barrage, CPP_BARRAGE[5]),
        SuperPhase::Restore => (comedy, CPP_COMEDY[4]),
        _ => return,
    };
    shadow(draw, Vector2::new(anchor.x, FLOOR_Y), 46., 0.35);
    draw_frame(draw, texture, frame, anchor, CPP_SCALE, right, 1.);
}

fn draw_footshot(draw: &mut impl DrawTarget, font: Option<&Font>, point: Vector2, local: f32) {
    let strength = (1. - local / 18.).clamp(0., 1.);
    let tint = Color::new(255, 186, 55, (strength * 220.) as u8);
    for index in 0..7 {
        let angle = index as f32 * PI / 6. + PI;
        let end = Vector2::new(
            point.x + angle.cos() * (22. + local * 2.),
            point.y + angle.sin() * (24. + local * 2.),
        );
        draw.draw_line_ex(point, end, 3. * strength + 1., tint);
    }
    draw_menu_text(
        draw,
        font,
        "NULL -> PE!",
        (point.x - 65.) as i32,
        (point.y - 315. - local) as i32,
        13.,
        Color::new(255, 116, 77, (strength * 230.) as u8),
    );
}

fn shadow(draw: &mut impl DrawTarget, anchor: Vector2, radius: f32, opacity: f32) {
    draw.draw_ellipse(
        anchor.x as i32,
        anchor.y as i32,
        radius,
        radius * 0.15,
        Color::new(9, 13, 22, (opacity.clamp(0., 1.) * 255.) as u8),
    );
}

fn smooth(value: f32) -> f32 {
    value * value * (3. - 2. * value)
}

fn draw_frame(
    draw: &mut impl DrawTarget,
    texture: &Texture2D,
    frame: Frame,
    anchor: Vector2,
    scale: f32,
    right: bool,
    opacity: f32,
) {
    let pieces = if frame.pieces.is_empty() {
        std::slice::from_ref(&frame.source)
    } else {
        frame.pieces
    };
    for &[x, y, width, height] in pieces {
        if x + width > texture.width() as f32 || y + height > texture.height() as f32 {
            continue;
        }
        let local_x = x - frame.source[0];
        let local_y = y - frame.source[1];
        let dest_x = if right {
            local_x - frame.pivot[0]
        } else {
            frame.pivot[0] - local_x - width
        };
        draw.draw_texture_pro(
            texture,
            Rectangle::new(x, y, if right { width } else { -width }, height),
            Rectangle::new(
                anchor.x + dest_x * scale,
                anchor.y + (local_y - frame.pivot[1]) * scale,
                width * scale,
                height * scale,
            ),
            Vector2::zero(),
            0.,
            Color::new(255, 255, 255, (opacity.clamp(0., 1.) * 255.) as u8),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn litter_settles_before_collectors_and_remains_until_the_pickup() {
        for index in 0..DUKE_CLONE_COUNT {
            let cue = DUKE_COLLECT_START + index * DUKE_CLONE_CADENCE;
            assert!(trash_is_visible(index, 60));
            assert!(trash_is_visible(index, cue - 3));
            assert!(!trash_is_visible(index, cue - 2));
            assert!(!trash_is_visible(index, cue));
            let anchor = trash_anchor(index);
            assert!(anchor.x > 90. && anchor.x < WINDOW_WIDTH as f32 - 90.);
            assert!(anchor.y <= FLOOR_Y);
        }
    }

    #[test]
    fn authored_source_regions_fit_rgba_assets_without_repacking() {
        for (path, frames) in [
            ("duke/collector-poses.png", DUKE.as_slice()),
            ("cpp/footgun-comedy.png", CPP_COMEDY.as_slice()),
            ("cpp/footgun-barrage.png", CPP_BARRAGE.as_slice()),
            ("trash/garbage-items.png", TRASH.as_slice()),
        ] {
            let data = std::fs::read(format!("assets/production/super-sequences/{path}")).unwrap();
            assert_eq!(&data[..8], b"\x89PNG\r\n\x1a\n");
            assert_eq!(data[25], 6, "real RGBA required: {path}");
            let width = u32::from_be_bytes(data[16..20].try_into().unwrap()) as f32;
            let height = u32::from_be_bytes(data[20..24].try_into().unwrap()) as f32;
            for frame in frames {
                let [x, y, w, h] = frame.source;
                assert!(x >= 0. && y >= 0. && x + w <= width && y + h <= height);
                for &[px, py, pw, ph] in frame.pieces {
                    assert!(px >= x && py >= y && px + pw <= x + w && py + ph <= y + h);
                }
            }
        }
    }
}
