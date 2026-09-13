//! Draws Augusta's adult night crowd, street furniture and local traffic.
//!
//! System: Adventure production scenery. Both gameplay and cinematic layers use
//! the same world positions, articulated silhouettes, clothing and light accents.

use super::{assets::ProductionAssets, painted_crowd};
use crate::adventure::augusta::ambient::{Nightlife, Pedestrian, Vehicle, VehicleKind};
use raylib::prelude::*;

const INK: Color = Color::new(18, 22, 30, 255);
const METAL: Color = Color::new(51, 59, 66, 255);

/// Physically separate decorations for the cinematic diorama and normal camera.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Layer {
    Sidewalk,
    Traffic,
    Foreground,
}

#[derive(Clone, Copy)]
struct Space {
    x: f32,
    y: f32,
    scale: f32,
    width_ratio: f32,
    facing: f32,
}

impl Space {
    fn point(self, p: [f32; 2]) -> Vector2 {
        Vector2::new(
            self.x + p[0] * self.scale * self.width_ratio * self.facing,
            self.y + p[1] * self.scale,
        )
    }

    fn line(self, d: &mut impl RaylibDraw, a: [f32; 2], b: [f32; 2], width: f32, color: Color) {
        d.draw_line_ex(
            self.point(a),
            self.point(b),
            width * self.scale * self.width_ratio,
            color,
        );
    }

    fn disk(self, d: &mut impl RaylibDraw, p: [f32; 2], radius: f32, color: Color) {
        self.ellipse(d, p, radius, radius, color);
    }

    fn ellipse(self, d: &mut impl RaylibDraw, p: [f32; 2], width: f32, height: f32, color: Color) {
        let point = self.point(p);
        d.draw_ellipse(
            point.x as i32,
            point.y as i32,
            width * self.scale * self.width_ratio,
            height * self.scale,
            color,
        );
    }

    fn polygon(self, d: &mut impl RaylibDraw, points: &[[f32; 2]], color: Color) {
        // Raylib culls clockwise triangles, so retain winding after mirroring.
        let mut projected: Vec<_> = points.iter().map(|p| self.point(*p)).collect();
        let signed_area: f32 = projected
            .iter()
            .zip(projected.iter().cycle().skip(1))
            .map(|(a, b)| a.x * b.y - b.x * a.y)
            .sum();
        if signed_area > 0.0 {
            projected.reverse();
        }
        d.draw_triangle_fan(&projected, color);
    }
}

fn tint(color: Color, factor: f32) -> Color {
    Color::new(
        (color.r as f32 * factor).min(255.0) as u8,
        (color.g as f32 * factor).min(255.0) as u8,
        (color.b as f32 * factor).min(255.0) as u8,
        color.a,
    )
}

fn limb(d: &mut impl RaylibDraw, s: Space, points: [[f32; 2]; 3], widths: [f32; 2], color: Color) {
    for part in 0..2 {
        s.line(d, points[part], points[part + 1], widths[part] + 1.8, INK);
        s.disk(d, points[part + 1], widths[part] * 0.5 + 0.5, INK);
        s.line(d, points[part], points[part + 1], widths[part], color);
        s.disk(d, points[part + 1], widths[part] * 0.5, color);
    }
}

/// Draws a registered painted adult with the existing deterministic pose.
pub fn draw_person(
    d: &mut impl RaylibDraw,
    assets: &ProductionAssets,
    person: &Pedestrian,
    camera_x: f32,
) {
    painted_crowd::draw_person(d, assets, person, camera_x);
}

fn wheel(d: &mut impl RaylibDraw, s: Space, at: [f32; 2], radius: f32, angle: f32) {
    s.disk(d, at, radius + 2.0, INK);
    s.disk(d, at, radius - 2.0, METAL);
    s.disk(d, at, radius * 0.36, Color::new(162, 161, 144, 255));
    for spoke in 0..5 {
        let a = angle + spoke as f32 * std::f32::consts::TAU / 5.0;
        s.line(
            d,
            at,
            [
                at[0] + a.cos() * (radius - 3.0),
                at[1] + a.sin() * (radius - 3.0),
            ],
            1.5,
            Color::new(111, 119, 124, 255),
        );
    }
}

/// Draws one vehicle, preserving model position, orientation and wheel rotation.
pub fn draw_vehicle(d: &mut impl RaylibDraw, vehicle: &Vehicle, camera_x: f32) {
    let s = Space {
        x: vehicle.x - camera_x + 640.0,
        y: vehicle.ground_y,
        scale: vehicle.scale,
        width_ratio: 1.0,
        facing: vehicle.facing,
    };
    s.ellipse(d, [0.0, 1.0], 94.0, 6.0, Color::new(4, 8, 16, 135));
    if vehicle.kind == VehicleKind::DeliveryScooter {
        for x in [-25.0, 30.0] {
            wheel(d, s, [x, -9.0], 9.0, vehicle.wheel_angle);
        }
        s.line(
            d,
            [-25.0, -9.0],
            [-9.0, -28.0],
            5.0,
            Color::new(134, 75, 59, 255),
        );
        s.line(
            d,
            [-9.0, -28.0],
            [11.0, -11.0],
            7.0,
            Color::new(143, 80, 60, 255),
        );
        s.line(d, [11.0, -11.0], [30.0, -9.0], 5.0, METAL);
        s.line(d, [30.0, -9.0], [21.0, -42.0], 5.0, METAL);
        s.line(d, [21.0, -42.0], [13.0, -44.0], 4.0, INK);
        s.line(d, [-30.0, -30.0], [0.0, -30.0], 7.0, INK);
        s.polygon(
            d,
            &[
                [-38.0, -52.0],
                [-15.0, -52.0],
                [-15.0, -33.0],
                [-38.0, -33.0],
            ],
            Color::new(130, 63, 47, 255),
        );
        s.line(
            d,
            [-26.0, -51.0],
            [-26.0, -35.0],
            2.0,
            Color::new(197, 139, 86, 255),
        );
        limb(
            d,
            s,
            [[-4.0, -33.0], [14.0, -23.0], [8.0, -13.0]],
            [8.0, 7.0],
            Color::new(39, 53, 70, 255),
        );
        s.line(
            d,
            [-4.0, -34.0],
            [-6.0, -58.0],
            16.0,
            Color::new(120, 115, 60, 255),
        );
        limb(
            d,
            s,
            [[-3.0, -56.0], [10.0, -47.0], [19.0, -44.0]],
            [6.0, 4.0],
            Color::new(125, 119, 63, 255),
        );
        s.disk(d, [-4.0, -69.0], 10.0, INK);
        s.ellipse(d, [-1.0, -69.0], 8.0, 4.5, Color::new(124, 156, 162, 255));
        s.disk(d, [28.0, -30.0], 4.0, Color::new(246, 220, 160, 255));
        return;
    }
    let taxi = vehicle.kind == VehicleKind::Taxi;
    let body = if taxi {
        Color::new(188, 183, 160, 255)
    } else if vehicle.id == 0 {
        Color::new(61, 84, 110, 255)
    } else {
        Color::new(131, 60, 57, 255)
    };
    let rear = if taxi { -94.0 } else { -79.0 };
    s.polygon(
        d,
        &[
            [rear, -14.0],
            [rear, -33.0],
            [-55.0, -39.0],
            [-35.0, -63.0],
            [25.0, -63.0],
            [52.0, -39.0],
            [87.0, -32.0],
            [93.0, -14.0],
        ],
        INK,
    );
    s.polygon(
        d,
        &[
            [rear + 2.0, -16.0],
            [rear + 3.0, -31.0],
            [-53.0, -37.0],
            [-33.0, -60.0],
            [24.0, -60.0],
            [51.0, -37.0],
            [85.0, -30.0],
            [90.0, -16.0],
        ],
        body,
    );
    s.polygon(
        d,
        &[[-49.0, -37.0], [-31.0, -57.0], [-8.0, -57.0], [-8.0, -37.0]],
        Color::new(30, 46, 60, 255),
    );
    s.polygon(
        d,
        &[[-4.0, -37.0], [-4.0, -57.0], [22.0, -57.0], [44.0, -37.0]],
        Color::new(36, 54, 67, 255),
    );
    s.line(
        d,
        [-27.0, -55.0],
        [-42.0, -39.0],
        2.0,
        Color::new(105, 134, 143, 190),
    );
    s.line(
        d,
        [1.0, -55.0],
        [16.0, -55.0],
        1.8,
        Color::new(124, 145, 143, 255),
    );
    s.disk(d, [20.0, -44.0], 5.0, Color::new(91, 78, 69, 255));
    s.line(
        d,
        [16.0, -38.0],
        [29.0, -37.0],
        5.0,
        Color::new(45, 50, 59, 255),
    );
    s.line(d, [rear + 7.0, -33.0], [82.0, -30.0], 2.0, tint(body, 1.28));
    s.line(d, [rear + 4.0, -20.0], [88.0, -20.0], 5.0, tint(body, 0.78));
    s.line(d, [-5.0, -35.0], [-4.0, -19.0], 1.0, tint(body, 0.57));
    s.line(d, [40.0, -35.0], [40.0, -19.0], 1.0, tint(body, 0.57));
    s.line(
        d,
        [4.0, -30.0],
        [13.0, -30.0],
        2.0,
        Color::new(174, 170, 145, 255),
    );
    s.line(
        d,
        [rear + 2.0, -28.0],
        [rear + 8.0, -28.0],
        5.0,
        Color::new(219, 76, 65, 255),
    );
    s.line(
        d,
        [81.0, -27.0],
        [89.0, -27.0],
        5.0,
        Color::new(246, 221, 161, 255),
    );
    s.ellipse(d, [112.0, -4.0], 40.0, 3.0, Color::new(247, 209, 139, 25));
    if taxi {
        s.polygon(
            d,
            &[[-13.0, -64.0], [-10.0, -72.0], [11.0, -72.0], [14.0, -64.0]],
            Color::new(232, 199, 115, 255),
        );
        s.line(
            d,
            [-10.0, -37.0],
            [-10.0, -19.0],
            4.0,
            Color::new(99, 90, 74, 255),
        );
        s.line(
            d,
            [17.0, -37.0],
            [17.0, -19.0],
            4.0,
            Color::new(99, 90, 74, 255),
        );
    }
    for x in [-51.0, 56.0] {
        wheel(d, s, [x, -10.0], 11.0, vehicle.wheel_angle);
    }
}

fn chair(d: &mut impl RaylibDraw, s: Space, x: f32) {
    for side in [-1.0, 1.0] {
        s.line(
            d,
            [x + side * 11.0, -26.0],
            [x + side * 13.0, 0.0],
            2.0,
            METAL,
        );
    }
    s.line(
        d,
        [x - 13.0, -28.0],
        [x + 13.0, -28.0],
        4.0,
        Color::new(119, 95, 65, 255),
    );
    s.line(d, [x - 12.0, -28.0], [x - 12.0, -55.0], 2.5, METAL);
    s.line(d, [x + 12.0, -28.0], [x + 12.0, -55.0], 2.5, METAL);
    s.line(
        d,
        [x - 12.0, -51.0],
        [x + 12.0, -51.0],
        5.0,
        Color::new(107, 85, 61, 255),
    );
}

pub(super) fn furniture(d: &mut impl RaylibDraw, camera_x: f32, front: bool, ticks: u64) {
    for world_x in [615.0, 2791.0] {
        let s = Space {
            x: world_x - camera_x + 640.0,
            y: 504.0,
            scale: 1.8,
            width_ratio: 0.72,
            facing: 1.0,
        };
        if !front {
            // Chair centers stay under the original seated patrons while their
            // seat height and backrest grow to the same adult body scale.
            chair(d, s, -29.0 / (s.scale * s.width_ratio));
            chair(d, s, 29.0 / (s.scale * s.width_ratio));
        } else {
            s.ellipse(d, [0.0, 0.0], 28.0, 5.0, Color::new(4, 8, 16, 92));
            s.line(d, [0.0, -42.0], [0.0, -2.0], 4.0, METAL);
            s.line(d, [0.0, -6.0], [-19.0, 0.0], 3.0, METAL);
            s.line(d, [0.0, -6.0], [19.0, 0.0], 3.0, METAL);
            s.ellipse(d, [0.0, -42.0], 32.0, 8.0, INK);
            s.ellipse(d, [0.0, -44.0], 32.0, 7.0, Color::new(116, 82, 55, 255));
            s.ellipse(d, [0.0, -45.0], 28.0, 5.0, Color::new(141, 102, 64, 255));
            for x in [-17.0, 16.0] {
                s.line(
                    d,
                    [x, -46.0],
                    [x, -56.0],
                    5.0,
                    Color::new(129, 102, 57, 255),
                );
                s.line(
                    d,
                    [x - 1.5, -54.0],
                    [x - 1.5, -46.0],
                    1.0,
                    Color::new(215, 181, 110, 255),
                );
                s.ellipse(d, [x, -56.0], 3.0, 1.4, Color::new(227, 211, 169, 255));
            }
            s.line(
                d,
                [2.0, -44.0],
                [2.0, -54.0],
                5.0,
                Color::new(76, 88, 61, 255),
            );
            s.line(
                d,
                [2.0, -53.0],
                [2.0, -60.0],
                2.5,
                Color::new(82, 99, 66, 255),
            );
            s.ellipse(d, [-4.0, -46.0], 7.0, 2.0, Color::new(204, 194, 163, 255));
        }
    }
    if !front {
        for world_x in [453.0, 873.0, 1305.0, 2204.0, 2640.0, 3080.0] {
            let s = Space {
                x: world_x - camera_x + 640.0,
                y: 501.0,
                scale: 1.6,
                width_ratio: 0.85,
                facing: 1.0,
            };
            s.polygon(
                d,
                &[[-14.0, -24.0], [14.0, -24.0], [10.0, 0.0], [-10.0, 0.0]],
                Color::new(74, 62, 52, 255),
            );
            s.line(
                d,
                [-15.0, -24.0],
                [15.0, -24.0],
                4.0,
                Color::new(110, 93, 68, 255),
            );
            for leaf in 0..7 {
                let sway = (ticks as f32 * 0.018 + leaf as f32).sin() * 1.3;
                let x = (leaf as f32 - 3.0) * 5.5 + sway;
                let y = -35.0 - (leaf % 3) as f32 * 6.0;
                s.line(d, [0.0, -23.0], [x, y], 1.5, Color::new(53, 77, 65, 255));
                s.ellipse(
                    d,
                    [x, y],
                    4.5,
                    7.0,
                    Color::new(48 + leaf * 3, 70 + leaf * 3, 63, 255),
                );
            }
        }
    }
}

/// Draws a complete physical layer with a normal 1280-wide gameplay camera.
///
/// Pass `camera_x = 640` when an outer 2D/3D transform already projects world
/// coordinates. Individual people and vehicles are also exposed for depth sort.
pub fn draw(
    d: &mut impl RaylibDraw,
    assets: &ProductionAssets,
    frame: &Nightlife,
    camera_x: f32,
    layer: Layer,
) {
    match layer {
        Layer::Sidewalk => {
            furniture(d, camera_x, false, frame.ticks);
            for person in &frame.people {
                draw_person(d, assets, person, camera_x);
            }
            furniture(d, camera_x, true, frame.ticks);
        }
        Layer::Traffic => {
            let mut vehicles: Vec<_> = frame.vehicles.iter().collect();
            vehicles.sort_by(|a, b| a.ground_y.total_cmp(&b.ground_y));
            for vehicle in vehicles {
                draw_vehicle(d, vehicle, camera_x);
            }
        }
        Layer::Foreground => {
            // Low kerb markers reinforce depth without hiding combat silhouettes.
            for world_x in [125.0, 440.0, 875.0, 1310.0, 2200.0, 2640.0, 3080.0, 3490.0] {
                let s = Space {
                    x: world_x - camera_x + 640.0,
                    y: 676.0,
                    scale: 1.5,
                    width_ratio: 0.85,
                    facing: 1.0,
                };
                s.ellipse(d, [0.0, 1.0], 14.0, 4.0, Color::new(3, 8, 15, 110));
                s.line(
                    d,
                    [0.0, -26.0],
                    [0.0, 0.0],
                    8.0,
                    Color::new(38, 43, 49, 255),
                );
                s.disk(d, [0.0, -26.0], 4.0, Color::new(88, 89, 84, 255));
                s.line(
                    d,
                    [-2.0, -21.0],
                    [2.0, -21.0],
                    3.0,
                    Color::new(167, 150, 97, 255),
                );
            }
        }
    }
}
