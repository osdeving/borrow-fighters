//! Authors Augusta's camera cuts in a single, fixed street coordinate system.
//!
//! The stage uses real perspective with illustrated actors. Camera rails stay on
//! the street side of the façades, so cuts never invent a reverse of painted art.

use super::{Chapter, Phase};
use serde::Serialize;

/// A lens and its rail sample, independent from the graphics library.
#[derive(Clone, Copy, Debug, Serialize)]
pub struct Shot {
    pub name: &'static str,
    pub eye: [f32; 3],
    pub target: [f32; 3],
    pub fov: f32,
    pub roll: f32,
    pub fade: f32,
}

fn ease(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

fn mix(a: [f32; 3], b: [f32; 3], t: f32) -> [f32; 3] {
    std::array::from_fn(|i| a[i] + (b[i] - a[i]) * ease(t))
}

fn rail(
    name: &'static str,
    target: [f32; 3],
    end: [f32; 3],
    start_offset: [f32; 3],
    end_offset: [f32; 3],
    t: f32,
) -> Shot {
    let target = mix(target, end, t);
    let offset = mix(start_offset, end_offset, t);
    Shot {
        name,
        eye: std::array::from_fn(|i| target[i] + offset[i]),
        target,
        fov: 38.0,
        roll: 0.0,
        fade: 0.0,
    }
}

/// Film direction owns the frame only during authored acting and conversations.
pub fn shot(chapter: &Chapter) -> Option<Shot> {
    let t = chapter.phase_ticks() as f32;
    let w = &chapter.world;
    let player = chapter.simulation().player().position.x;
    let mut s = match chapter.phase() {
        Phase::Introduction => {
            let p = t / chapter.spec.timing.introduction_ticks as f32;
            if p < 0.23 {
                rail(
                    "augusta-crane",
                    [1050., 230., -100.],
                    [930., 150., -60.],
                    [-450., 610., 1600.],
                    [120., 290., 1290.],
                    p / 0.23,
                )
            } else if p < 0.43 {
                rail(
                    "nightlife-tracking",
                    [650., 105., -40.],
                    [810., 110., -40.],
                    [-240., 65., 940.],
                    [100., 45., 880.],
                    (p - 0.23) / 0.20,
                )
            } else if p < 0.62 {
                rail(
                    "limiar-exterior",
                    [w.bar_door[0], 250., -150.],
                    [w.bar_door[0], 140., -150.],
                    [240., 180., 840.],
                    [80., 60., 630.],
                    (p - 0.43) / 0.19,
                )
            } else if p < 0.79 {
                rail(
                    "julia-held",
                    [w.broker_x + 48., 141., -18.],
                    [w.broker_x + 55., 146., -18.],
                    [120., 18., 460.],
                    [-40., 12., 405.],
                    (p - 0.62) / 0.17,
                )
            } else {
                rail(
                    "cpp-reveal",
                    [player, 44., 0.],
                    [player + 25., 118., 0.],
                    [125., 16., 345.],
                    [-100., 35., 590.],
                    (p - 0.79) / 0.21,
                )
            }
        }
        Phase::JuliaAttempt => {
            let p = t / chapter.spec.timing.julia_attempt_ticks as f32;
            if p < 0.27 {
                rail(
                    "julia-recognizes-cpp",
                    [w.julia_initial_x, 157., -48.],
                    [w.julia_initial_x - 8., 160., -48.],
                    [110., 10., 340.],
                    [50., 5., 310.],
                    p / 0.27,
                )
            } else if p < 0.64 {
                rail(
                    "wrist-restraint",
                    [w.broker_x + 61., 121., -10.],
                    [w.broker_x + 55., 128., -10.],
                    [65., 15., 300.],
                    [-40., 4., 280.],
                    (p - 0.27) / 0.37,
                )
            } else {
                rail(
                    "cpp-intervenes",
                    [(player + w.broker_x) * 0.5, 119., 0.],
                    [(player + w.broker_x) * 0.5, 128., 0.],
                    [-170., 28., 780.],
                    [-40., 15., 700.],
                    (p - 0.64) / 0.36,
                )
            }
        }
        Phase::Confrontation | Phase::AfterGuards | Phase::RescueDialogue => {
            let (key, line) = chapter.dialogue()?;
            let speaker = &chapter.texts.line(key, line)?.speaker;
            let target_x = match speaker.as_str() {
                "cpp" => player,
                "julia" => {
                    chapter
                        .npcs()
                        .iter()
                        .find(|n| n.character == "julia")?
                        .position
                        .x
                }
                _ => w.broker_x,
            };
            let p = (chapter.dialogue_ticks() as f32 / 420.).min(1.);
            let offset = if speaker == "cpp" { 65. } else { -65. };
            rail(
                if speaker == "cpp" {
                    "cpp-dialogue"
                } else if speaker == "julia" {
                    "julia-dialogue"
                } else {
                    "broker-dialogue"
                },
                [target_x + offset * 0.2, 135., 0.],
                [target_x, 140., 0.],
                [offset, 25., 435.],
                [offset * 0.65, 17., 395.],
                p,
            )
        }
        Phase::GuardsArrival => {
            let p = t / chapter.spec.timing.guards_arrival_ticks as f32;
            if p < 0.27 {
                rail(
                    "bar-door-opens",
                    [w.bar_door[0], 90., -130.],
                    [w.bar_door[0], 130., -100.],
                    [-100., -22., 530.],
                    [65., 5., 580.],
                    p / 0.27,
                )
            } else if p < 0.68 {
                rail(
                    "guards-emerge",
                    [w.bar_door[0] + 25., 115., -65.],
                    [w.bar_door[0] + 85., 120., -25.],
                    [180., 20., 810.],
                    [-140., 55., 930.],
                    (p - 0.27) / 0.41,
                )
            } else {
                rail(
                    "guards-surround",
                    [w.confrontation_x + 110., 110., 0.],
                    [w.confrontation_x + 125., 115., 0.],
                    [-200., 115., 1330.],
                    [0., 100., 1460.],
                    (p - 0.68) / 0.32,
                )
            }
        }
        Phase::Regroup => rail(
            "uneasy-silence",
            [player, 135., 0.],
            [player, 147., 0.],
            [-95., 35., 570.],
            [15., 80., 670.],
            t / 180.,
        ),
        Phase::ErraticsArrival => {
            let p = t / chapter.spec.timing.erratics_arrival_ticks as f32;
            if p < 0.13 {
                rail(
                    "sky-rupture",
                    [w.erratic_center_x, 1340., -25.],
                    [w.erratic_center_x, 1280., -25.],
                    [-180., -100., 1480.],
                    [-100., -60., 1500.],
                    p / 0.13,
                )
            } else if p < 0.39 {
                let broker = chapter
                    .npcs()
                    .iter()
                    .find(|n| n.character == "broker")
                    .map_or(w.broker_x, |n| n.position.x);
                rail(
                    "broker-panics",
                    [broker, 127., 0.],
                    [broker, 132., 0.],
                    [-125., 8., 540.],
                    [30., 25., 630.],
                    (p - 0.13) / 0.26,
                )
            } else {
                let descent = (p / 0.75).clamp(0., 1.).powi(3);
                let center_y = (1350. * (1. - descent) * 0.5 + 125.).max(125.);
                rail(
                    "erratics-descend",
                    [w.erratic_center_x, center_y, 0.],
                    [w.erratic_center_x, center_y, 0.],
                    [200., 110., 1960.],
                    [0., 100., 1480.],
                    (p - 0.39) / 0.61,
                )
            }
        }
        _ => return None,
    };
    // A brief shutter at the handoff avoids a perspective pop into the 2D camera.
    let duration = match chapter.phase() {
        Phase::Introduction => Some(chapter.spec.timing.introduction_ticks),
        Phase::GuardsArrival => Some(chapter.spec.timing.guards_arrival_ticks),
        Phase::ErraticsArrival => Some(chapter.spec.timing.erratics_arrival_ticks),
        _ => None,
    };
    if let Some(duration) = duration {
        s.fade = ease((t - duration as f32 + 15.) / 15.);
    }
    if chapter.phase() == Phase::Introduction {
        s.fade = s.fade.max(1. - ease(t / 35.));
    }
    if let Some(age) = chapter.landing_age().filter(|age| *age < 18) {
        let decay = 1. - age as f32 / 18.;
        s.roll = (age as f32 * 1.7).sin() * 0.65 * decay;
        s.eye[0] += (age as f32 * 2.1).sin() * 7. * decay;
    }
    Some(s)
}

/// Gameplay returns under the same shutter, with no input delay after a cut.
pub fn handoff_fade(chapter: &Chapter) -> f32 {
    if matches!(
        chapter.phase(),
        Phase::Approach | Phase::GuardsFight | Phase::ErraticsFight
    ) {
        1. - ease(chapter.phase_ticks() as f32 / 15.)
    } else {
        0.
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rails_do_not_overshoot_their_endpoints() {
        let start = [-450., 610., 1600.];
        let end = [120., 290., 1290.];
        for i in -20..=120 {
            let p = mix(start, end, i as f32 / 100.);
            for k in 0..3 {
                assert!((start[k].min(end[k])..=start[k].max(end[k])).contains(&p[k]));
            }
            assert!(p[2] > 0., "camera must stay on the illustrated front side");
        }
    }
}
