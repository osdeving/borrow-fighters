//! Draws Python's authored transformation, devouring serpent and human celebration.
//!
//! System: Raylib presentation. The four atlases supply only visual poses. The
//! captured opponent stays in World; its placement snapshot never changes damage.

use std::f32::consts::{PI, TAU};

use raylib::prelude::*;

use crate::{
    characters::CharacterId,
    combat::{
        fighter::{Facing, PlayerSlot},
        super_sequence::{
            PYTHON_PEACE_START, PYTHON_SWALLOW_TICK, PYTHON_TARGET_RETURN_TICK, SuperPhase,
            SuperSequence,
        },
    },
    config::{FLOOR_Y, WINDOW_WIDTH},
    game::world::World,
    math::vec2::Vec2,
};

use super::DrawTarget;

const HUMAN_SCALE: f32 = 0.54;

/// All sheets must load before the normal caster is suppressed.
#[derive(Clone, Copy, Default)]
pub(super) struct PythonSuperTextures<'a> {
    pub transform: Option<&'a Texture2D>,
    pub serpent: Option<&'a Texture2D>,
    pub revert: Option<&'a Texture2D>,
    pub celebrate: Option<&'a Texture2D>,
}

impl<'a> PythonSuperTextures<'a> {
    pub(super) fn ready(self) -> bool {
        self.transform.is_some()
            && self.serpent.is_some()
            && self.revert.is_some()
            && self.celebrate.is_some()
    }

    fn get(self, sheet: Sheet) -> Option<&'a Texture2D> {
        match sheet {
            Sheet::Transform => self.transform,
            Sheet::Serpent => self.serpent,
            Sheet::Revert => self.revert,
            Sheet::Celebrate => self.celebrate,
        }
    }
}

/// Feet-centered placement for the real defender's current reaction sprite.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct PythonTargetVisual {
    pub anchor: Vec2,
    pub scale: f32,
    pub rotation_degrees: f32,
    pub opacity: f32,
    pub hidden: bool,
}

/// The ordinary caster remains available as fallback when a sheet is missing.
pub(super) fn hides_actor(
    world: &World,
    slot: PlayerSlot,
    textures: PythonSuperTextures<'_>,
) -> bool {
    textures.ready()
        && world.super_sequence().is_some_and(|sequence| {
            sequence.character == CharacterId::Python
                && sequence.attacker == slot
                && sequence.phase() != SuperPhase::Freeze
        })
}

/// Sucks the opponent toward the drawn mouth, hides it, then follows real recoil.
/// Call only when the Python sheets are ready; World owns the hidden interval.
pub(super) fn target_visual(sequence: &SuperSequence) -> Option<PythonTargetVisual> {
    if sequence.character != CharacterId::Python {
        return None;
    }
    let mut visual = PythonTargetVisual {
        anchor: sequence.target_anchor,
        scale: 1.0,
        rotation_degrees: 0.0,
        opacity: 1.0,
        hidden: sequence.target_hidden(),
    };
    if sequence.phase() == SuperPhase::PythonLunge {
        let progress = smooth(sequence.phase_progress());
        let mouth = mouth_anchor(sequence);
        visual.scale = 1.0 - progress * 0.88;
        visual.anchor = lerp_point(
            sequence.target_anchor,
            Vec2::new(mouth.x, mouth.y + 135.0 * visual.scale),
            progress,
        );
        visual.rotation_degrees = direction(sequence) * progress * -68.0;
        visual.opacity = (1.0 - (progress - 0.88).max(0.0) * 3.0).max(0.25);
    } else if (PYTHON_TARGET_RETURN_TICK..PYTHON_TARGET_RETURN_TICK + 6).contains(&sequence.tick) {
        // Core resumes the stored reaction at 388. Follow that physical anchor,
        // avoiding a second visual flight arc on top of World-owned recoil.
        let progress = (sequence.tick - PYTHON_TARGET_RETURN_TICK) as f32 / 6.0;
        visual.scale = 0.65 + 0.35 * progress;
        visual.opacity = progress;
    }
    Some(visual)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Sheet {
    Transform,
    Serpent,
    Revert,
    Celebrate,
}

#[derive(Clone, Copy)]
struct Frame {
    source: [f32; 4],
    /// Absolute coordinates within the source atlas, including virtual jump floor.
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

const TRANSFORM: [Frame; 8] = [
    frame([43., 15., 263., 510.], [174., 520.]),
    frame([378., 45., 322., 476.], [545., 516.]),
    frame([743., 47., 312., 461.], [915., 505.]),
    frame([1115., 74., 309., 440.], [1270., 511.]),
    frame([25., 586., 320., 483.], [185., 1065.]),
    frame([383., 607., 331., 452.], [550., 1056.]),
    frame([733., 633., 340., 426.], [904., 1056.]),
    frame([1099., 540., 336., 529.], [1267., 1065.]),
];

// Sources are explicit because the long lunge does not occupy a nominal grid cell.
const SERPENT: [Frame; 8] = [
    frame([2., 184., 360., 330.], [185., 510.]),
    frame([373., 46., 342., 475.], [546., 518.]),
    frame([738., 132., 350., 390.], [914., 517.]),
    frame([1106., 76., 342., 451.], [1276., 523.]),
    Frame {
        source: [0., 662., 552., 344.],
        pivot: [163., 1000.],
        pieces: &[[0., 662., 552., 172.], [0., 834., 416., 172.]],
    },
    Frame {
        source: [414., 738., 447., 282.],
        pivot: [605., 1013.],
        pieces: &[
            [553., 738., 308., 96.],
            [416., 834., 445., 34.],
            [416., 868., 407., 152.],
        ],
    },
    Frame {
        source: [820., 597., 311., 439.],
        pivot: [977., 1032.],
        pieces: &[[880., 597., 248., 270.], [820., 867., 308., 169.]],
    },
    frame([1129., 632., 319., 407.], [1292., 1035.]),
];

const REVERT: [Frame; 8] = [
    frame([18., 59., 376., 475.], [205., 530.]),
    frame([408., 134., 340., 399.], [576., 529.]),
    frame([756., 120., 291., 408.], [902., 523.]),
    frame([1081., 93., 354., 439.], [1260., 530.]),
    frame([42., 558., 345., 501.], [204., 1055.]),
    frame([436., 648., 310., 410.], [590., 1053.]),
    frame([775., 712., 335., 345.], [939., 1053.]),
    frame([1140., 533., 294., 525.], [1289., 1053.]),
];

const CELEBRATE: [Frame; 8] = [
    frame([54., 131., 270., 401.], [185., 529.]),
    frame([426., 31., 263., 386.], [554., 518.]),
    frame([774., 3., 294., 397.], [918., 518.]),
    frame([1070., 34., 378., 492.], [1264., 523.]),
    frame([47., 619., 330., 438.], [206., 1054.]),
    frame([438., 541., 270., 525.], [576., 1062.]),
    frame([796., 542., 241., 524.], [920., 1062.]),
    frame([1136., 540., 289., 525.], [1284., 1061.]),
];

fn frames(sheet: Sheet) -> &'static [Frame; 8] {
    match sheet {
        Sheet::Transform => &TRANSFORM,
        Sheet::Serpent => &SERPENT,
        Sheet::Revert => &REVERT,
        Sheet::Celebrate => &CELEBRATE,
    }
}

#[derive(Clone, Copy, Debug)]
struct Pose {
    sheet: Sheet,
    index: usize,
    anchor: Vec2,
    scale: f32,
}

fn direction(sequence: &SuperSequence) -> f32 {
    if sequence.facing == Facing::Right {
        1.0
    } else {
        -1.0
    }
}

fn serpent_stage_anchor(sequence: &SuperSequence) -> Vec2 {
    Vec2::new(
        WINDOW_WIDTH as f32
            * if sequence.facing == Facing::Right {
                0.26
            } else {
                0.74
            },
        FLOOR_Y,
    )
}

fn pose_at(sequence: &SuperSequence) -> Option<Pose> {
    if sequence.character != CharacterId::Python || sequence.phase() == SuperPhase::Freeze {
        return None;
    }
    let phase = sequence.phase();
    let progress = sequence.phase_progress();
    let local = sequence.tick - sequence.phase_span().start;
    let story = serpent_stage_anchor(sequence);
    let mut pose = Pose {
        sheet: Sheet::Transform,
        index: 0,
        anchor: sequence.attacker_anchor,
        scale: HUMAN_SCALE,
    };
    match phase {
        SuperPhase::PythonPrepare => {
            pose.index = usize::from(local >= 30);
            pose.anchor.y -= (local as f32 * 0.12).sin().abs() * 2.0;
        }
        SuperPhase::PythonMorph => {
            pose.index = (2 + local as usize / 16).min(7);
            pose.anchor = lerp_point(sequence.attacker_anchor, story, smooth(progress));
            pose.scale += progress * 0.08;
        }
        SuperPhase::PythonGrow => {
            pose.sheet = Sheet::Serpent;
            pose.index = usize::from(local >= 24);
            pose.anchor = story;
            let height = 326.0 + smooth(progress) * 192.0;
            pose.scale = height / SERPENT[pose.index].source[3];
        }
        SuperPhase::PythonMouth => {
            pose.sheet = Sheet::Serpent;
            pose.index = if local < 12 { 2 } else { 3 };
            pose.anchor = story;
            pose.scale = (500.0 + progress * 18.0) / SERPENT[pose.index].source[3];
            pose.anchor.x -= direction(sequence) * smooth(progress) * 28.0;
        }
        SuperPhase::PythonLunge => {
            pose.sheet = Sheet::Serpent;
            pose.index = if local < 24 { 4 } else { 5 };
            pose.anchor = story;
            pose.anchor.x += direction(sequence) * (smooth(progress) * 98.0 - 28.0);
            pose.scale = 1.1;
        }
        SuperPhase::PythonSwallow => {
            pose.sheet = Sheet::Serpent;
            pose.index = if local < 18 { 6 } else { 7 };
            pose.anchor = story;
            pose.scale = 1.1;
            pose.anchor.y -= (progress * PI).sin() * 8.0;
        }
        SuperPhase::PythonRevert => {
            pose.sheet = Sheet::Revert;
            pose.index = (local as usize / 7).min(7);
            pose.anchor = lerp_point(story, sequence.attacker_anchor, smooth(progress));
            pose.scale = 1.1 - 0.56 * smooth(progress);
        }
        SuperPhase::PythonCelebrate => {
            pose.sheet = Sheet::Celebrate;
            pose.index = match local {
                0..=5 => 0,
                6..=14 => 1,
                15..=26 => 2,
                27..=35 => 3,
                36..=43 => 4,
                44..=61 => 5,
                _ => 6,
            };
            if (6..36).contains(&local) {
                pose.anchor.y -= ((local - 6) as f32 / 30.0 * PI).sin() * 106.0;
            }
        }
        SuperPhase::Restore => {
            pose.sheet = Sheet::Celebrate;
            pose.index = 7;
            pose.anchor.y -= (local as f32 * 0.12).sin().abs() * 1.5;
        }
        _ => return None,
    }
    Some(pose)
}

fn mouth_anchor(sequence: &SuperSequence) -> Vec2 {
    let pose = pose_at(sequence).expect("Python lunge has an authored pose");
    // Absolute mouth centers of the two lunge drawings, relative to their floor pivot.
    let mouth = if pose.index == 4 {
        [478.0, 751.0]
    } else {
        [812.0, 793.0]
    };
    let pivot = SERPENT[pose.index].pivot;
    Vec2::new(
        pose.anchor.x + direction(sequence) * (mouth[0] - pivot[0]) * pose.scale,
        pose.anchor.y + (mouth[1] - pivot[1]) * pose.scale,
    )
}

/// Draw after the ordinary actors; the serpent mouth covers the shrinking target.
pub(super) fn draw_python_super(
    draw: &mut impl DrawTarget,
    world: &World,
    textures: PythonSuperTextures<'_>,
) {
    if !textures.ready() {
        return;
    }
    let Some(sequence) = world.super_sequence() else {
        return;
    };
    let Some(pose) = pose_at(sequence) else {
        return;
    };
    let Some(texture) = textures.get(pose.sheet) else {
        return;
    };
    draw_energy(draw, sequence, pose);
    draw.draw_ellipse(
        pose.anchor.x as i32,
        FLOOR_Y as i32,
        if pose.sheet == Sheet::Serpent {
            130.0
        } else {
            43.0
        },
        10.0,
        Color::new(3, 9, 20, 100),
    );
    draw_frame(
        draw,
        texture,
        frames(pose.sheet)[pose.index],
        pose,
        sequence.facing,
    );
}

fn draw_energy(draw: &mut impl DrawTarget, sequence: &SuperSequence, pose: Pose) {
    let phase = sequence.phase();
    if matches!(
        phase,
        SuperPhase::PythonMorph | SuperPhase::PythonGrow | SuperPhase::PythonRevert
    ) {
        let strength = (sequence.phase_progress() * PI).sin().abs();
        for band in 0..14 {
            let angle = sequence.tick as f32 * 0.035 + band as f32 * TAU / 14.0;
            let center = Vector2::new(
                pose.anchor.x + angle.cos() * (84.0 + strength * 105.0),
                FLOOR_Y - 35.0 - band as f32 * 30.0,
            );
            let tint = if band % 2 == 0 {
                Color::new(65, 147, 239, 140)
            } else {
                Color::new(255, 204, 68, 140)
            };
            draw.draw_poly_lines_ex(center, 6, 5.0 + strength * 5.0, angle * 20.0, 1.5, tint);
        }
    }
    if phase == SuperPhase::PythonLunge {
        let mouth = mouth_anchor(sequence);
        for lane in 0..18 {
            let travel = (sequence.tick as f32 * 0.09 + lane as f32 * 0.173).fract();
            let from = Vec2::new(sequence.target_anchor.x, 200.0 + lane as f32 * 23.0);
            let a = lerp_point(from, mouth, travel);
            let b = lerp_point(from, mouth, (travel + 0.1).min(1.0));
            draw.draw_line_ex(
                Vector2::new(a.x, a.y),
                Vector2::new(b.x, b.y),
                1.5,
                Color::new(95, 188, 255, 95),
            );
        }
    }
    let accent_age = [PYTHON_SWALLOW_TICK, PYTHON_PEACE_START]
        .into_iter()
        .find_map(|start| {
            (start..start + 10)
                .contains(&sequence.tick)
                .then(|| sequence.tick - start)
        });
    if let Some(age) = accent_age {
        let progress = age as f32 / 10.0;
        let tint = Color::new(255, 216, 81, ((1.0 - progress) * 180.0) as u8);
        let radius = 75.0 + progress * 80.0;
        draw.draw_ring(
            Vector2::new(pose.anchor.x, FLOOR_Y - 140.0),
            radius,
            radius + 4.0 * (1.0 - progress),
            0.0,
            360.0,
            60,
            tint,
        );
    }
}

fn draw_frame(
    draw: &mut impl DrawTarget,
    texture: &Texture2D,
    frame: Frame,
    pose: Pose,
    facing: Facing,
) {
    let pieces = if frame.pieces.is_empty() {
        std::slice::from_ref(&frame.source)
    } else {
        frame.pieces
    };
    for &[x, y, width, height] in pieces {
        let right = facing == Facing::Right;
        let left = if right {
            x - frame.pivot[0]
        } else {
            frame.pivot[0] - x - width
        };
        draw.draw_texture_pro(
            texture,
            Rectangle::new(x, y, if right { width } else { -width }, height),
            Rectangle::new(
                pose.anchor.x + left * pose.scale,
                pose.anchor.y + (y - frame.pivot[1]) * pose.scale,
                width * pose.scale,
                height * pose.scale,
            ),
            Vector2::zero(),
            0.0,
            Color::WHITE,
        );
    }
}

fn smooth(value: f32) -> f32 {
    value * value * (3.0 - 2.0 * value)
}

fn lerp_point(from: Vec2, to: Vec2, progress: f32) -> Vec2 {
    Vec2::new(
        from.x + (to.x - from.x) * progress,
        from.y + (to.y - from.y) * progress,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::combat::super_sequence::super_spec;

    fn snapshot(tick: u32, facing: Facing) -> SuperSequence {
        let spec = super_spec(CharacterId::Python).unwrap();
        SuperSequence {
            character: CharacterId::Python,
            move_id: spec.move_id,
            label: spec.label,
            attacker: PlayerSlot::One,
            target: PlayerSlot::Two,
            guarded: false,
            target_crouching: false,
            tick,
            duration_frames: spec.duration_frames,
            attacker_origin: Vec2::new(170.0, FLOOR_Y),
            target_origin: Vec2::new(1100.0, FLOOR_Y),
            attacker_anchor: Vec2::new(170.0, FLOOR_Y),
            target_anchor: Vec2::new(1100.0, FLOOR_Y),
            facing,
            fractional_ticks: 0.0,
            charge_destination_x: 170.0,
        }
    }

    #[test]
    fn every_authored_drawing_is_played_and_fits_its_rgba_sheet() {
        let mut seen = [[false; 8]; 4];
        for tick in 8..528 {
            let pose = pose_at(&snapshot(tick, Facing::Right)).unwrap();
            let sheet = match pose.sheet {
                Sheet::Transform => 0,
                Sheet::Serpent => 1,
                Sheet::Revert => 2,
                Sheet::Celebrate => 3,
            };
            seen[sheet][pose.index] = true;
            assert!(pose.scale.is_finite() && pose.scale > 0.0);
        }
        assert!(seen.into_iter().flatten().all(|seen| seen));
        for (name, sheet) in [
            ("transform", Sheet::Transform),
            ("serpent", Sheet::Serpent),
            ("revert", Sheet::Revert),
            ("celebrate", Sheet::Celebrate),
        ] {
            let data = std::fs::read(format!(
                "assets/production/super-sequences/python/{name}.png"
            ))
            .unwrap();
            assert_eq!(&data[..8], b"\x89PNG\r\n\x1a\n");
            assert_eq!(data[25], 6, "{name} requires real RGBA");
            let width = u32::from_be_bytes(data[16..20].try_into().unwrap()) as f32;
            let height = u32::from_be_bytes(data[20..24].try_into().unwrap()) as f32;
            for frame in frames(sheet) {
                for &[x, y, w, h] in std::iter::once(&frame.source).chain(frame.pieces.iter()) {
                    assert!(x >= 0.0 && y >= 0.0 && w > 0.0 && h > 0.0);
                    assert!(x + w <= width && y + h <= height, "{name} crop clips sheet");
                }
            }
        }
    }

    #[test]
    fn swallow_visibility_and_return_follow_the_authoritative_clock() {
        for facing in [Facing::Right, Facing::Left] {
            let start = target_visual(&snapshot(270, facing)).unwrap();
            let last = target_visual(&snapshot(303, facing)).unwrap();
            assert_eq!(start.anchor, Vec2::new(1100.0, FLOOR_Y));
            assert_eq!(start.scale, 1.0);
            assert!(!last.hidden && last.scale < 0.15);
            for tick in 304..388 {
                assert!(target_visual(&snapshot(tick, facing)).unwrap().hidden);
            }
            let mut returning = snapshot(388, facing);
            returning.target_anchor = Vec2::new(1020.0, FLOOR_Y - 95.0);
            let visual = target_visual(&returning).unwrap();
            assert!(!visual.hidden);
            assert_eq!(visual.anchor, returning.target_anchor);
            assert_eq!(visual.opacity, 0.0);
            returning.tick = 394;
            let visual = target_visual(&returning).unwrap();
            assert_eq!(visual.anchor, returning.target_anchor);
            assert_eq!(visual.scale, 1.0);
            assert_eq!(visual.opacity, 1.0);
        }
    }
}
