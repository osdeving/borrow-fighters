//! Draws the motor lane and the car's honk, braking, impact and damaged aftermath.
//!
//! System: Adventure presentation. Traffic samples come from the story clock;
//! dust, wheels, lamps and the bending pole never create gameplay contacts.

use raylib::prelude::*;

use super::{assets::Assets, pieces::PiecePose};
use crate::adventure::{
    ambient::{
        AmbientState, CAR_HORN_TICK, CAR_IMPACT_TICK, CAR_SKID_START_X, CRASH_POLE_X, IncidentPhase,
    },
    combat::Facing,
};

const INK: Color = Color::new(45, 52, 49, 255);
const CONCRETE: Color = Color::new(183, 173, 141, 255);

/// Paints asphalt between the raised sidewalk and the existing protected cycle lane.
pub fn road(d: &mut impl RaylibDraw, offset: f32) {
    d.draw_rectangle_gradient_v(
        0,
        366,
        1728,
        72,
        Color::new(90, 95, 88, 255),
        Color::new(111, 109, 97, 255),
    );
    // Low-contrast aggregate follows the scenery, never the simulation clock.
    for i in 0..420 {
        let x = (i as f32 * 67.31 - offset).rem_euclid(1788.0) - 30.0;
        let y = 371.0 + (i as f32 * 19.73).rem_euclid(63.0);
        d.draw_line_ex(
            Vector2::new(x, y),
            Vector2::new(x + 2.5, y),
            0.7,
            Color::new(216, 197, 161, 28),
        );
    }
    d.draw_rectangle(0, 364, 1728, 5, CONCRETE);
    d.draw_line_ex(
        Vector2::new(0.0, 369.0),
        Vector2::new(1728.0, 369.0),
        2.0,
        Color::new(47, 54, 49, 170),
    );
    for i in -1..16 {
        let x = i as f32 * 106.0 - offset.rem_euclid(106.0);
        d.draw_line_ex(
            Vector2::new(x, 403.0),
            Vector2::new(x + 48.0, 403.0),
            2.0,
            Color::new(236, 210, 140, 205),
        );
        d.draw_line_ex(
            Vector2::new(x, 365.0),
            Vector2::new(x + 2.0, 368.0),
            1.0,
            INK,
        );
    }
    // Raised separator makes the cycle lane inaccessible to the cars.
    d.draw_rectangle(0, 435, 1728, 5, Color::new(201, 189, 155, 255));
    d.draw_line_ex(
        Vector2::new(0.0, 439.0),
        Vector2::new(1728.0, 439.0),
        1.5,
        INK,
    );
}

/// Draws far traffic, then the incident in the near lane and its persistent pole.
pub fn draw(d: &mut impl RaylibDraw, ambient: &AmbientState, a: &Assets, offset: f32) {
    for car in ambient.traffic_cars().iter().filter(|car| car.visible) {
        let id = [
            "vehicle.hatch",
            "vehicle.sedan",
            "vehicle.pickup",
            "vehicle.suv",
            "vehicle.bus",
        ][car.style];
        let mut pose = PiecePose::at(Vector2::new(car.position.x - offset, car.position.y));
        pose.flip = car.facing == Facing::Left;
        pose.tint = Color::new(229, 232, 220, 255);
        let width = a.street.size(id, 0).x;
        d.draw_ellipse(
            pose.position.x as i32,
            pose.position.y as i32,
            width * 0.46,
            3.0,
            Color::new(35, 40, 37, 70),
        );
        a.street.draw(d, id, car.animation_ticks, &pose);
        a.street.wheels(d, id, car.animation_ticks, &pose);
        if car.fleeing {
            for n in 0..2 {
                let x = pose.position.x + width * 0.5 + 6.0;
                let y = pose.position.y - 12.0 - n as f32 * 10.0;
                d.draw_line_ex(
                    Vector2::new(x, y),
                    Vector2::new(x + 22.0, y),
                    1.0,
                    Color::new(217, 208, 176, 130),
                );
            }
        }
    }

    let age = ambient
        .accident_ticks()
        .and_then(|ticks| ticks.checked_sub(CAR_IMPACT_TICK));
    if let Some(car) = ambient.incident_car() {
        let feet = Vector2::new(car.position.x - offset, car.position.y);
        if car.phase != IncidentPhase::Approaching {
            skid_marks(d, feet, offset);
        }
        match car.phase {
            IncidentPhase::Crashed => {
                let impact_age = car.phase_ticks;
                let width = a.street.size("incident.crashed", 0).x;
                // Retain the original uniform scale, pressing the shortened nose
                // against the pole while the rear rolls forward during crumpling.
                let shake = (impact_age as f32 * 2.8).sin()
                    * (1.0 - impact_age as f32 / 20.0).max(0.0)
                    * 3.0;
                let center =
                    Vector2::new(CRASH_POLE_X - offset - width * 0.5 + shake, feet.y + 1.5);
                let mut pose = PiecePose::at(center);
                pose.position.y = feet.y + 1.5;
                a.street.draw(d, "incident.crashed", 0, &pose);
                aftermath(d, impact_age, offset);
            }
            IncidentPhase::Approaching | IncidentPhase::Braking => {
                let ticks = ambient.accident_ticks().unwrap_or(0);
                let pose = PiecePose::at(feet);
                a.street.draw(d, "incident.intact", ticks * 3, &pose);
                a.street.wheels(d, "incident.intact", ticks * 3, &pose);
                if car.phase == IncidentPhase::Braking {
                    // Tail lamps and tire haze precede the metal impact.
                    d.draw_circle_v(
                        Vector2::new(feet.x - 72.0, feet.y - 20.0),
                        3.0,
                        Color::new(255, 89, 45, 230),
                    );
                    for i in 0..5 {
                        let age = (car.phase_ticks as f32 + i as f32 * 7.0).rem_euclid(27.0);
                        d.draw_circle_v(
                            Vector2::new(feet.x - 49.0 - age * 2.0, feet.y - 3.0 - age * 0.25),
                            2.0 + age * 0.35,
                            Color::new(210, 205, 180, (85.0 * (1.0 - age / 27.0)) as u8),
                        );
                    }
                } else if (CAR_HORN_TICK..CAR_HORN_TICK + 28).contains(&ticks) {
                    for i in 0..3 {
                        let x = feet.x + 55.0 + i as f32 * 8.0;
                        d.draw_line_ex(
                            Vector2::new(x, feet.y - 47.0),
                            Vector2::new(x + 4.0, feet.y - 57.0 - i as f32 * 3.0),
                            1.8,
                            Color::new(255, 229, 160, 220),
                        );
                    }
                }
            }
        }
    }
    pole(d, age, offset);
}

fn skid_marks(d: &mut impl RaylibDraw, feet: Vector2, offset: f32) {
    let start = CAR_SKID_START_X - offset - 49.0;
    let end = feet.x - 49.0;
    for y in [426.0, 430.0] {
        d.draw_line_ex(
            Vector2::new(start, y),
            Vector2::new(end, y),
            2.0,
            Color::new(36, 44, 43, 145),
        );
    }
}

fn pole(d: &mut impl RaylibDraw, age: Option<u32>, offset: f32) {
    let x = CRASH_POLE_X - offset + 3.0;
    let bend = age.map_or(0.0, |ticks| {
        let t = ticks as f32 / 60.0;
        9.0 * (1.0 - (-t * 12.0).exp()) + (t * 28.0).sin() * (-t * 3.0).exp() * 8.0
    });
    let base = Vector2::new(x, 431.0);
    let top = Vector2::new(x + bend, 232.0);
    d.draw_ellipse(x as i32 + 5, 432, 16.0, 2.5, Color::new(36, 42, 34, 95));
    d.draw_line_ex(base, top, 10.0, INK);
    d.draw_line_ex(
        Vector2::new(base.x - 1.0, base.y - 2.0),
        Vector2::new(top.x - 1.0, top.y),
        7.0,
        CONCRETE,
    );
    d.draw_line_ex(
        Vector2::new(base.x - 3.0, base.y - 5.0),
        Vector2::new(top.x - 3.0, top.y),
        1.4,
        Color::new(238, 212, 161, 240),
    );
    for y in [282.0, 339.0, 398.0] {
        let px = x + bend * (431.0 - y) / 199.0;
        d.draw_line_ex(
            Vector2::new(px - 4.0, y),
            Vector2::new(px + 4.0, y),
            2.0,
            Color::new(103, 105, 89, 180),
        );
    }
    let lamp = Vector2::new(top.x + 43.0, top.y - 9.0);
    d.draw_line_ex(Vector2::new(top.x, top.y + 20.0), lamp, 3.0, INK);
    d.draw_ellipse(lamp.x as i32 + 8, lamp.y as i32, 17.0, 4.0, INK);
    d.draw_line_ex(
        Vector2::new(lamp.x - 3.0, lamp.y + 1.0),
        Vector2::new(lamp.x + 17.0, lamp.y + 1.0),
        2.0,
        CONCRETE,
    );
    if age.is_some() {
        d.draw_line_ex(
            Vector2::new(x - 4.0, 405.0),
            Vector2::new(x + 2.0, 398.0),
            1.5,
            INK,
        );
        d.draw_line_ex(
            Vector2::new(x + 2.0, 398.0),
            Vector2::new(x - 2.0, 394.0),
            1.5,
            INK,
        );
    }
}

fn aftermath(d: &mut impl RaylibDraw, ticks: u32, offset: f32) {
    let x = CRASH_POLE_X - offset - 9.0;
    let t = ticks as f32 / 60.0;
    if ticks < 42 {
        let fade = 1.0 - ticks as f32 / 42.0;
        for i in 0..9 {
            let angle = 2.0 + i as f32 * 0.37;
            let radius = 5.0 + t * 65.0;
            d.draw_circle_v(
                Vector2::new(
                    x + angle.cos() * radius,
                    411.0 + angle.sin() * radius * 0.34,
                ),
                3.0 + t * 15.0,
                Color::new(225, 213, 183, (fade * 130.0) as u8),
            );
        }
    }
    if ticks < 16 {
        let fade = 1.0 - ticks as f32 / 16.0;
        for i in 0..7 {
            let angle = 2.2 + i as f32 * 0.61;
            let radius = 9.0 + t * 84.0;
            let center = Vector2::new(x + angle.cos() * radius, 401.0 + angle.sin() * radius);
            d.draw_line_ex(
                center,
                Vector2::new(center.x + angle.cos() * 8.0, center.y + angle.sin() * 8.0),
                1.8,
                Color::new(255, 231, 143, (fade * 255.0) as u8),
            );
        }
    }
    // Small fragments fall once and remain near the damaged nose.
    for i in 0..5 {
        let f = i as f32;
        let travel = t.min(1.0);
        let px = x - 7.0 - travel * (24.0 + f * 14.0);
        let py = (400.0 - travel * (32.0 + f * 5.0) + travel * travel * 138.0).min(430.0 - f % 2.0);
        d.draw_rectangle_pro(
            Rectangle::new(px, py, 3.0 + f % 2.0, 2.0),
            Vector2::zero(),
            f * 33.0 + travel * 200.0,
            Color::new(121, 151, 149, 230),
        );
    }
    // Restrained continuous steam leaves the crumpled hood readable.
    for i in 0..6 {
        let age = (ticks as f32 + i as f32 * 19.0).rem_euclid(118.0);
        if age > ticks as f32 {
            continue;
        }
        let fraction = age / 118.0;
        d.draw_circle_v(
            Vector2::new(
                x - 15.0 - age * 0.17 + (age * 0.08).sin() * 3.0,
                389.0 - age * 0.53,
            ),
            2.0 + fraction * 9.0,
            Color::new(181, 184, 164, ((1.0 - fraction) * 90.0) as u8),
        );
    }
}
