//! Draws small Brazilian background cameos and occasional caramelo crossings.
//!
//! System: Engine presentation. These decorative actors use a visual clock and
//! never create fighters, hitboxes, collisions, audio events, or gameplay state.

use raylib::prelude::*;

use super::{DrawTarget, draw_menu_text};
use crate::config::{WINDOW_WIDTH, screen_px, world_px};
use crate::game::arena::ArenaId;

const DOG_FRAMES: usize = 4;
const JESSICA_FRAMES: usize = 3;
const DOG_CYCLE_SECONDS: f32 = 26.0;
const DOG_RUN_SECONDS: f32 = 6.8;
const DOG_WIDTH: f32 = 98.0;

/// Authored crops are irregular; a shared scale preserves body size across poses.
#[derive(Clone, Copy, Debug)]
struct StageFrame {
    source: [u32; 4],
    pivot: [f32; 2],
}

// Alpha component bounds plus a four-pixel sampling margin. The dog's common
// source floor is y=540; the first airborne pose keeps its intentional lift.
const DOG_CROPS: [StageFrame; DOG_FRAMES] = [
    StageFrame {
        source: [12, 190, 607, 312],
        pivot: [320.0, 350.0],
    },
    StageFrame {
        source: [622, 197, 502, 345],
        pivot: [246.0, 343.0],
    },
    StageFrame {
        source: [1180, 206, 460, 330],
        pivot: [212.0, 334.0],
    },
    StageFrame {
        source: [1659, 204, 490, 337],
        pivot: [225.0, 336.0],
    },
];
const JESSICA_CROPS: [StageFrame; JESSICA_FRAMES] = [
    StageFrame {
        source: [58, 62, 396, 899],
        pivot: [198.0, 896.0],
    },
    StageFrame {
        source: [540, 65, 455, 891],
        pivot: [229.0, 893.0],
    },
    StageFrame {
        source: [1010, 67, 526, 890],
        pivot: [310.0, 891.0],
    },
];

/// Resources and presentation state for the optional layer behind the fighters.
pub(super) struct StageLifeDrawOptions<'a> {
    pub arena: ArenaId,
    pub time: f32,
    pub dog_atlas: Option<&'a Texture2D>,
    pub jessica_atlas: Option<&'a Texture2D>,
    pub font: Option<&'a Font>,
    pub cinematic_active: bool,
}

/// Draws scenery details before the fighters, keeping cinematic moments quiet.
pub(super) fn draw_stage_life(draw: &mut impl DrawTarget, options: StageLifeDrawOptions<'_>) {
    draw_local_details(draw, options.arena, options.font, options.time);
    if options.cinematic_active {
        return;
    }

    if let Some(texture) = options.dog_atlas
        && let Some(pose) = dog_pose(options.arena, options.time)
    {
        draw_dog(draw, texture, pose);
    }
    if options.arena == ArenaId::JavaStreet
        && let Some(texture) = options.jessica_atlas
    {
        draw_jessica(draw, texture, options.time);
    }
}

#[derive(Clone, Copy, Debug)]
struct DogPose {
    x: f32,
    feet_y: f32,
    frame: usize,
    facing_right: bool,
}

fn dog_pose(arena: ArenaId, time: f32) -> Option<DogPose> {
    // Time comes from presentation, so a menu or a replay never consumes match
    // RNG or changes combat outcomes. Most of each cycle has no crossing.
    let (delay, feet_y) = match arena {
        ArenaId::Sirius => (5.0, 423.0),
        ArenaId::Fortaleza => (8.0, 414.0),
        ArenaId::JavaStreet => (6.0, 434.0),
        ArenaId::BioTic => (10.0, 409.0),
        ArenaId::PortoDigital => (7.0, 399.0),
        ArenaId::ValeDoPinhao => (9.0, 385.0),
    };
    if !time.is_finite() || time < delay {
        return None;
    }

    let elapsed = time - delay;
    let local_time = elapsed.rem_euclid(DOG_CYCLE_SECONDS);
    if local_time >= DOG_RUN_SECONDS {
        return None;
    }
    let progress = local_time / DOG_RUN_SECONDS;
    let facing_right = ((elapsed / DOG_CYCLE_SECONDS).floor() as u32).is_multiple_of(2);
    let width = world_px(DOG_WIDTH);
    let left = -width;
    let right = WINDOW_WIDTH as f32 + width;
    let x = if facing_right {
        left + (right - left) * progress
    } else {
        right - (right - left) * progress
    };
    Some(DogPose {
        x,
        feet_y: world_px(feet_y),
        frame: (local_time * 12.0) as usize % DOG_FRAMES,
        facing_right,
    })
}

fn draw_dog(draw: &mut impl DrawTarget, texture: &Texture2D, pose: DogPose) {
    let width = world_px(DOG_WIDTH);
    draw.draw_ellipse(
        (pose.x + width * 0.5).round() as i32,
        pose.feet_y.round() as i32,
        width * 0.32,
        world_px(3.0),
        Color::new(15, 24, 26, 42),
    );
    draw_stage_frame(
        draw,
        texture,
        DOG_CROPS[pose.frame],
        Vector2::new(pose.x + width * 0.5, pose.feet_y),
        width / 600.0,
        pose.facing_right,
        Color::new(211, 208, 198, 245),
    );
}

fn jessica_frame(time: f32) -> usize {
    // Neutral between short, readable gestures instead of a constant arm loop.
    let gesture = (time.max(0.0) + 3.0).rem_euclid(12.0);
    if gesture < 1.68 {
        [0, 1, 2, 2, 1, 0][(gesture / 0.28) as usize]
    } else {
        0
    }
}

fn draw_jessica(draw: &mut impl DrawTarget, texture: &Texture2D, time: f32) {
    let feet_y = world_px(408.0);
    let center_x = world_px(794.0);
    draw.draw_ellipse(
        center_x.round() as i32,
        feet_y.round() as i32,
        world_px(12.0),
        world_px(2.0),
        Color::new(12, 20, 27, 42),
    );
    draw_stage_frame(
        draw,
        texture,
        JESSICA_CROPS[jessica_frame(time)],
        Vector2::new(center_x, feet_y),
        world_px(70.0) / 891.0,
        true,
        Color::new(194, 197, 202, 245),
    );
}

fn draw_stage_frame(
    draw: &mut impl DrawTarget,
    texture: &Texture2D,
    frame: StageFrame,
    anchor: Vector2,
    scale: f32,
    facing_right: bool,
    tint: Color,
) {
    let [x, y, width, height] = frame.source;
    if x + width > texture.width() as u32 || y + height > texture.height() as u32 {
        return;
    }
    let pivot_x = if facing_right {
        frame.pivot[0]
    } else {
        width as f32 - frame.pivot[0]
    };
    draw.draw_texture_pro(
        texture,
        Rectangle::new(
            x as f32,
            y as f32,
            if facing_right {
                width as f32
            } else {
                -(width as f32)
            },
            height as f32,
        ),
        Rectangle::new(
            anchor.x - pivot_x * scale,
            anchor.y - frame.pivot[1] * scale,
            width as f32 * scale,
            height as f32 * scale,
        ),
        Vector2::zero(),
        0.0,
        tint,
    );
}

fn draw_local_details(draw: &mut impl DrawTarget, arena: ArenaId, font: Option<&Font>, time: f32) {
    // Coordinates follow the 960x540 arena source images. Small labels attach
    // to railings, noticeboards and kiosks; the central combat lane stays clear.
    let (bounds, lines, ink, paper) = match arena {
        ArenaId::Sirius => (
            [136, 287, 100, 25],
            ["NAZARE.exe", "calculando..."],
            Color::new(139, 197, 198, 205),
            Color::new(8, 30, 39, 218),
        ),
        ArenaId::Fortaleza => (
            [586, 323, 103, 25],
            ["BORA, BILL!", "sinal de chamada"],
            Color::new(197, 204, 187, 204),
            Color::new(17, 43, 58, 223),
        ),
        ArenaId::JavaStreet => (
            [26, 378, 53, 27],
            ["JA ACABOU,", "JESSICA?"],
            Color::new(39, 39, 37, 207),
            Color::new(195, 183, 155, 226),
        ),
        ArenaId::BioTic => (
            [174, 319, 70, 25],
            ["E VERDADE", "ESSE BILETE"],
            Color::new(41, 61, 49, 205),
            Color::new(196, 197, 158, 224),
        ),
        ArenaId::PortoDigital => (
            [83, 324, 100, 25],
            ["AMOSTRADINHO", "desde o primeiro byte"],
            Color::new(190, 178, 140, 205),
            Color::new(23, 43, 48, 206),
        ),
        ArenaId::ValeDoPinhao => (
            [229, 267, 47, 31],
            ["HOJE NAO,", "FARO"],
            Color::new(163, 195, 183, 203),
            Color::new(12, 34, 43, 226),
        ),
    };
    let [x, y, width, height] = bounds;
    draw.draw_rectangle(
        screen_px(x),
        screen_px(y),
        screen_px(width),
        screen_px(height),
        paper,
    );
    draw.draw_rectangle_lines(
        screen_px(x),
        screen_px(y),
        screen_px(width),
        screen_px(height),
        Color::new(134, 152, 141, 68),
    );
    let size = if width < 70 { 7.5 } else { 8.0 };
    for (index, line) in lines.into_iter().enumerate() {
        draw_menu_text(
            draw,
            font,
            line,
            screen_px(x + 4),
            screen_px(y + 4 + index as i32 * 11),
            if index == 0 { size } else { size - 1.0 },
            ink,
        );
    }
    if arena == ArenaId::Sirius {
        // The small loading cursor turns the Nazaré reference into lab signage.
        let alpha = if time.rem_euclid(1.4) < 0.7 { 165 } else { 48 };
        draw.draw_rectangle(
            screen_px(x + width - 10),
            screen_px(y + height - 9),
            screen_px(3),
            screen_px(4),
            Color::new(134, 192, 188, alpha),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn authored_crops_fit_packaged_atlases_and_keep_their_shared_floor() {
        for (path, crops, expected_size, floor) in [
            (
                crate::engine::assets::CARAMELO_RUN_PATH,
                DOG_CROPS.as_slice(),
                [2172, 724],
                540.0,
            ),
            (
                crate::engine::assets::JESSICA_GESTURE_PATH,
                JESSICA_CROPS.as_slice(),
                [1536, 1024],
                958.0,
            ),
        ] {
            let png = std::fs::read(path).unwrap();
            assert_eq!(&png[..8], b"\x89PNG\r\n\x1a\n");
            assert_eq!(png[25], 6, "stage actors must retain their RGBA channel");
            let width = u32::from_be_bytes(png[16..20].try_into().unwrap());
            let height = u32::from_be_bytes(png[20..24].try_into().unwrap());
            assert_eq!([width, height], expected_size);
            for frame in crops {
                let [x, y, w, h] = frame.source;
                assert!(x + w <= width && y + h <= height);
                assert!((y as f32 + frame.pivot[1] - floor).abs() < f32::EPSILON);
            }
        }
    }

    #[test]
    fn dog_waits_between_complete_crossings_and_alternates_direction() {
        assert!(dog_pose(ArenaId::Sirius, 4.9).is_none());
        let entrance = dog_pose(ArenaId::Sirius, 5.0).unwrap();
        let exit = dog_pose(ArenaId::Sirius, 11.7).unwrap();
        assert!(entrance.x + world_px(DOG_WIDTH) <= 0.0);
        assert!(exit.x > WINDOW_WIDTH as f32);
        assert!(entrance.facing_right);
        assert!(dog_pose(ArenaId::Sirius, 12.0).is_none());
        assert!(dog_pose(ArenaId::Sirius, 30.9).is_none());
        let returning = dog_pose(ArenaId::Sirius, 31.0).unwrap();
        assert!(!returning.facing_right);
        assert!(returning.x > WINDOW_WIDTH as f32);
    }

    #[test]
    fn decorative_actors_stay_in_the_background_and_use_valid_atlas_cells() {
        for arena in ArenaId::ROTATION {
            for tick in 0..6000 {
                let time = tick as f32 / 60.0;
                if let Some(pose) = dog_pose(arena, time) {
                    assert!(pose.feet_y < crate::config::FLOOR_Y - world_px(20.0));
                    assert!(pose.frame < DOG_FRAMES);
                }
                assert!(jessica_frame(time) < JESSICA_FRAMES);
            }
        }
        assert!(dog_pose(ArenaId::Sirius, f32::NAN).is_none());
    }
}
