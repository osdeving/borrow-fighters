//! Presents the fixed Augusta street as a perspective stage during authored shots.
//!
//! Painted façades and actors keep their map coordinates. Ground and building
//! volumes provide depth, while bounded front-side cameras preserve their identity.

use super::{
    actors::{attachment, draw_actor, frame_stride_ticks},
    assets::ProductionAssets,
    nightlife,
};
use crate::adventure::augusta::{
    Chapter, CheckpointStage, Phase, ambient::Nightlife, cinema::Shot,
};
use raylib::prelude::*;

fn backdrop(d: &mut impl RaylibDraw, a: &ProductionAssets, shot: Shot) {
    let bg = &a.art.pieces[&a.art.background];
    let offset = ((shot.target[0] - 640.) * a.art.parallax).rem_euclid(bg.size[0]);
    for i in -1..=2 {
        attachment(
            d,
            &a.textures[&bg.image],
            bg,
            Vector2::new(i as f32 * bg.size[0] - offset, 0.),
            1.,
            false,
            0.,
            Color::WHITE,
        );
    }
    d.draw_rectangle_gradient_v(
        0,
        0,
        1280,
        720,
        Color::new(6, 12, 27, 70),
        Color::new(14, 19, 29, 220),
    );
}

fn floor(d: &mut impl RaylibDraw, a: &ProductionAssets) {
    let mut m = d.rl_push_matrix();
    m.rl_translatef(0., 1., -150.);
    m.rl_rotatef(90., 1., 0., 0.);
    m.rl_scalef(1., 3.6, 1.);
    let tile = &a.art.pieces[&a.art.ground];
    for i in -2_i32..9 {
        let flip = a.art.mirror_ground_tiles && i.rem_euclid(2) == 1;
        attachment(
            &mut *m,
            &a.textures[&tile.image],
            tile,
            Vector2::new((i as f32 + if flip { 1. } else { 0. }) * tile.size[0], 0.),
            1.,
            flip,
            0.,
            Color::WHITE,
        );
    }
    m.draw_rectangle(-1280, 67, 7200, 150, Color::new(13, 18, 27, 150));
    for x in (-12..48).map(|i| i * 120) {
        m.draw_rectangle(x, 131, 61, 2, Color::new(236, 201, 128, 145));
        m.draw_rectangle(x, 26, 118, 1, Color::new(204, 183, 151, 50));
    }
}

fn facades(d: &mut impl RaylibDraw, a: &ProductionAssets) {
    let mut m = d.rl_push_matrix();
    m.rl_translatef(0., a.art.facade_baseline, -150.);
    m.rl_scalef(1., -1., 1.);
    m.draw_rectangle(-500, 310, 4700, 190, Color::new(24, 26, 33, 255));
    for p in &a.world.pieces {
        let sprite = &a.art.pieces[&p.piece];
        attachment(
            &mut *m,
            &a.textures[&sprite.image],
            sprite,
            Vector2::new(p.position[0], p.position[1]),
            p.scale,
            false,
            0.,
            Color::WHITE,
        );
    }
    for sign in &a.art.signs {
        let width = a.font.measure_text(&sign.text, sign.size, 1.).x;
        // Subtle repeated outlines read as sign light without detaching the lettering.
        for radius in [3., 1.] {
            m.draw_text_ex(
                &a.font,
                &sign.text,
                Vector2::new(sign.position[0] - width * 0.5 - radius, sign.position[1]),
                sign.size,
                1.,
                Color::new(sign.color[0], sign.color[1], sign.color[2], 35),
            );
        }
        m.draw_text_ex(
            &a.font,
            &sign.text,
            Vector2::new(sign.position[0] - width * 0.5, sign.position[1]),
            sign.size,
            1.,
            Color::new(sign.color[0], sign.color[1], sign.color[2], 255),
        );
    }
}

fn shadow(d: &mut impl RaylibDraw, x: f32, depth: f32, radius: f32) {
    let mut m = d.rl_push_matrix();
    m.rl_translatef(x, 0.5, depth);
    m.rl_rotatef(90., 1., 0., 0.);
    m.draw_ellipse(0, 0, radius, radius * 0.36, Color::new(3, 6, 11, 125));
}

fn door_angle(c: &Chapter) -> f32 {
    if c.phase() == Phase::GuardsArrival {
        let p = (c.phase_ticks() as f32 / (c.spec.timing.guards_arrival_ticks as f32 * 0.17))
            .clamp(0., 1.);
        90. * p * p * (3. - 2. * p)
    } else if c.checkpoint().stage == CheckpointStage::Arrival {
        0.
    } else {
        90.
    }
}

/// The hinged leaf keeps its opening state when perspective hands back to gameplay.
pub(super) fn draw_door_flat(d: &mut impl RaylibDraw, c: &Chapter, camera: f32) {
    let angle = door_angle(c).to_radians();
    let x = c.world.bar_door[0] - 48. - camera + 640.;
    let y = c.world.bar_door[1];
    let dx = 92. * angle.cos();
    let dy = 92. * angle.sin() / 3.;
    let points = [
        Vector2::new(x, y - 167.),
        Vector2::new(x + dx, y - 167. + dy),
        Vector2::new(x + dx, y + dy),
        Vector2::new(x, y),
    ];
    d.draw_triangle(points[0], points[2], points[1], Color::new(40, 26, 25, 242));
    d.draw_triangle(points[0], points[3], points[2], Color::new(40, 26, 25, 242));
    d.draw_line_ex(points[0], points[3], 3., Color::new(151, 101, 59, 255));
}

fn door(d: &mut impl RaylibDraw, c: &Chapter) {
    let angle = door_angle(c);
    let mut m = d.rl_push_matrix();
    m.rl_translatef(c.world.bar_door[0] - 48., 0., -148.);
    m.rl_rotatef(-angle, 0., 1., 0.);
    m.rl_scalef(1., -1., 1.);
    m.draw_rectangle(0, -167, 92, 167, Color::new(40, 26, 25, 242));
    m.draw_rectangle(7, -155, 77, 120, Color::new(76, 49, 35, 175));
    m.draw_rectangle_lines_ex(
        Rectangle::new(5., -160., 81., 126.),
        3.,
        Color::new(151, 101, 59, 255),
    );
    m.draw_line_ex(
        Vector2::new(75., -68.),
        Vector2::new(75., -46.),
        4.,
        Color::new(219, 168, 88, 255),
    );
}

/// Render the perspective camera; gameplay uses the same actors in `world`.
pub fn draw_stage(
    d: &mut impl RaylibDraw,
    a: &ProductionAssets,
    c: &Chapter,
    shot: Shot,
    debug: bool,
) {
    backdrop(d, a, shot);
    let camera = Camera3D::perspective(
        Vector3::from(shot.eye),
        Vector3::from(shot.target),
        Vector3::new(
            shot.roll.to_radians().sin(),
            shot.roll.to_radians().cos(),
            0.,
        ),
        shot.fov,
    );
    let mut scene = d.begin_mode3D(camera);
    scene.rl_disable_backface_culling();
    floor(&mut scene, a);
    scene.draw_cube(
        Vector3::new(1800., -10., -57.),
        4700.,
        12.,
        195.,
        Color::new(52, 47, 44, 255),
    );
    scene.draw_cube(
        Vector3::new(1800., -5., 42.),
        4700.,
        10.,
        8.,
        Color::new(140, 124, 98, 255),
    );
    // Each shallow volume follows its own façade footprint and roofline.
    for piece in &a.world.pieces {
        if !piece.piece.starts_with("facade-") {
            continue;
        }
        let art = &a.art.pieces[&piece.piece];
        let height = art.anchor[1] * art.size[1] / art.source[3] * piece.scale - 36.;
        scene.draw_cube(
            Vector3::new(piece.position[0], height * 0.5, -235.),
            art.size[0] * piece.scale * 0.92,
            height,
            165.,
            Color::new(32, 31, 36, 255),
        );
    }
    // Flush the opaque batch before changing depth state; rlgl buffers draws.
    unsafe {
        raylib::ffi::rlDrawRenderBatchActive();
    }
    scene.rl_disable_depth_test();
    // Transparent illustrations are submitted back-to-front, including all limbs.
    facades(&mut scene, a);
    door(&mut scene, c);
    let mut night = Nightlife::sample(c.ticks(), super::world::threat_age(c));
    let half_width = (shot.eye[2] - shot.target[2]) * (shot.fov.to_radians() * 0.5).tan() * 1.78;
    night
        .people
        .retain(|p| (p.x - shot.target[0]).abs() < half_width + 210.);
    night
        .vehicles
        .retain(|v| shot.eye[2] > 660. && (v.x - shot.target[0]).abs() < half_width + 340.);
    if let Some(models) = &a.models {
        {
            let mut m = scene.rl_push_matrix();
            m.rl_translatef(0., 500., -120.);
            m.rl_scalef(1., -1., 1.);
            nightlife::furniture(&mut *m, 640., false, night.ticks);
        }
        for person in &night.people {
            let mut sample = super::models3d::person_sample(person, night.ticks);
            sample.at[1] = 500. - person.ground_y;
            if !models.draw(&mut scene, sample) {
                let mut m = scene.rl_push_matrix();
                m.rl_translatef(0., 500., -120.);
                m.rl_scalef(1., -1., 1.);
                nightlife::draw_person(&mut *m, a, person, 640.);
            }
        }
        let mut m = scene.rl_push_matrix();
        m.rl_translatef(0., 500., -120.);
        m.rl_scalef(1., -1., 1.);
        nightlife::furniture(&mut *m, 640., true, night.ticks);
    } else {
        let mut m = scene.rl_push_matrix();
        m.rl_translatef(0., 500., -120.);
        m.rl_scalef(1., -1., 1.);
        nightlife::draw(&mut *m, a, &night, 640., nightlife::Layer::Sidewalk);
    }
    for npc in c.npcs() {
        if !npc.visible || c.restraint_contact().is_some() {
            continue;
        }
        let assets = &a.actors[npc.character];
        shadow(&mut scene, npc.position.x, -npc.depth * 3., 27.);
        let ticks = frame_stride_ticks(assets, npc.clip, npc.position.x * npc.facing.sign())
            .unwrap_or(npc.ticks as f32);
        let pose = assets.clips.sample(npc.clip, ticks, ticks * 3.).pose;
        let drawn = a.models.as_ref().is_some_and(|models| {
            models.draw(
                &mut scene,
                super::models3d::actor_sample(
                    assets,
                    npc.clip,
                    ticks,
                    [
                        npc.position.x,
                        c.world.ground_y - npc.depth - npc.position.y,
                        -npc.depth * 3.,
                    ],
                    npc.facing.sign(),
                    pose.yaw,
                    1.,
                )
                .traveled(npc.position.x * npc.facing.sign()),
            )
        });
        if !drawn {
            let mut m = scene.rl_push_matrix();
            m.rl_translatef(0., c.world.ground_y - npc.depth, -npc.depth * 3.);
            m.rl_scalef(1., -1., 1.);
            draw_actor(
                &mut *m,
                assets,
                pose,
                npc.clip,
                ticks,
                [npc.position.x, npc.position.y],
                npc.facing,
                1.,
                debug,
            );
        }
    }
    if !super::models3d_pair::draw_stage(&mut scene, a, c) {
        let mut m = scene.rl_push_matrix();
        m.rl_translatef(0., c.world.ground_y - 8., -24.);
        m.rl_scalef(1., -1., 1.);
        super::restraint::draw(&mut *m, a, c, 640.);
    }
    let states = a.animations.borrow();
    for actor in c.simulation().actors() {
        let arrival = c.arrival_pose(actor.id);
        if arrival.is_some_and(|p| !p.visible) {
            continue;
        }
        let (p, depth, scale, clip, ticks, facing) = arrival.map_or(
            (
                actor.position,
                0.,
                1.,
                actor.clip_id(),
                actor.action_ticks as f32,
                actor.facing,
            ),
            |p| {
                (
                    p.position,
                    p.depth,
                    p.scale,
                    p.clip,
                    p.ticks as f32,
                    p.facing,
                )
            },
        );
        let assets = &a.actors[&actor.character];
        let ticks = frame_stride_ticks(assets, clip, p.x * facing.sign()).unwrap_or(ticks);
        let pose = if arrival.is_some() {
            assets.clips.sample(clip, ticks, 0.).pose
        } else {
            states.get(&actor.id.0).map_or_else(
                || assets.clips.sample(clip, ticks, actor.stride_distance).pose,
                |s| s.pose,
            )
        };
        shadow(&mut scene, p.x, -depth * 3., 30. * scale);
        let drawn = a.models.as_ref().is_some_and(|models| {
            models.draw(
                &mut scene,
                super::models3d::actor_sample(
                    assets,
                    clip,
                    ticks,
                    [p.x, c.world.ground_y - depth - p.y, -depth * 3.],
                    facing.sign(),
                    pose.yaw,
                    scale,
                )
                .traveled(p.x * facing.sign()),
            )
        });
        let mut m = scene.rl_push_matrix();
        m.rl_translatef(0., c.world.ground_y - depth, -depth * 3.);
        m.rl_scalef(1., -1., 1.);
        if arrival.is_some() && clip == "arrival" {
            let pulse = (ticks * 0.14).sin();
            let center = Vector2::new(p.x, p.y - 90. * scale);
            m.draw_circle_lines_v(
                center,
                70. * scale + 10. + pulse * 4.,
                Color::new(102, 218, 235, 180),
            );
            for i in 0..6 {
                let dx = (i as f32 - 2.5) * 19.;
                let end = Vector2::new(center.x + dx, center.y - 35.);
                let mid = Vector2::new(end.x + 12. * pulse, end.y - 60. - i as f32 * 7.);
                let top = Vector2::new(mid.x - 18., mid.y - 74.);
                m.draw_line_ex(end, mid, 2.5, Color::new(139, 236, 247, 210));
                m.draw_line_ex(mid, top, 1.5, Color::new(91, 181, 230, 130));
            }
        }
        if !drawn {
            draw_actor(
                &mut *m,
                assets,
                pose,
                clip,
                ticks,
                [p.x, p.y],
                facing,
                scale,
                debug,
            );
        }
        if let Some(age) = arrival.and_then(|p| p.impact_age) {
            super::world::dust(&mut *m, p.x, c.world.ground_y, age);
        }
    }
    // Each traffic lane has its own ground contact and fixed depth. Scaling a
    // vehicle must enlarge its silhouette, never sink its wheels below the road.
    night
        .vehicles
        .sort_by(|a, b| a.ground_y.total_cmp(&b.ground_y));
    for vehicle in &night.vehicles {
        let depth = if vehicle.facing > 0.0 { 600. } else { 500. };
        if let Some(models) = &a.models {
            let mut sample = super::models3d::vehicle_sample(vehicle);
            sample.at[1] = 0.;
            if models.draw(&mut scene, sample) {
                continue;
            }
        }
        let mut m = scene.rl_push_matrix();
        m.rl_translatef(0., vehicle.ground_y, depth);
        m.rl_scalef(1., -1., 1.);
        nightlife::draw_vehicle(&mut *m, vehicle, 640.);
    }
    {
        let mut m = scene.rl_push_matrix();
        m.rl_translatef(0., 676., 720.);
        m.rl_scalef(1., -1., 1.);
        nightlife::draw(&mut *m, a, &night, 640., nightlife::Layer::Foreground);
    }
    // Commit transparent primitives before restoring the state for the next pass.
    unsafe {
        raylib::ffi::rlDrawRenderBatchActive();
    }
    scene.rl_enable_depth_test();
    scene.rl_enable_backface_culling();
}
