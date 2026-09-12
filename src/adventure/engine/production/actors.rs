//! Draws the exact same authored actor poses in the chapter and external laboratory.
//!
//! System: Adventure production renderer. Painted view attachments, inverse
//! kinematics and continuous cloth retain identity while clips remain editable.

use super::assets::ActorAssets;
use crate::adventure::production::{
    Facing,
    animation::{self as a, Attachment, Chain, Method, Point, Pose},
};
use raylib::prelude::*;

#[derive(Clone, Copy)]
struct Transform {
    at: Point,
    sign: f32,
    scale: f32,
}
impl Transform {
    fn point(self, p: Point) -> Vector2 {
        Vector2::new(
            self.at[0] + p[0] * self.sign * self.scale,
            self.at[1] + p[1] * self.scale,
        )
    }
}

/// Draw a registered attachment; anchors are source pixels and sizes are world pixels.
#[allow(clippy::too_many_arguments)]
pub fn attachment(
    d: &mut impl RaylibDraw,
    texture: &Texture2D,
    sprite: &Attachment,
    position: Vector2,
    scale: f32,
    flip: bool,
    angle: f32,
    tint: Color,
) {
    let [x, y, w, h] = sprite.source;
    let origin = Vector2::new(
        if flip {
            w - sprite.anchor[0]
        } else {
            sprite.anchor[0]
        } * sprite.size[0]
            / w
            * scale,
        sprite.anchor[1] * sprite.size[1] / h * scale,
    );
    d.draw_texture_pro(
        texture,
        Rectangle::new(x, y, if flip { -w } else { w }, h),
        Rectangle::new(
            position.x,
            position.y,
            sprite.size[0] * scale,
            sprite.size[1] * scale,
        ),
        origin,
        angle,
        tint,
    );
}

#[allow(clippy::too_many_arguments)]
fn piece(
    d: &mut impl RaylibDraw,
    assets: &ActorAssets,
    id: &str,
    position: Point,
    angle: f32,
    transform: Transform,
    flip: bool,
    tint: Color,
) {
    let sprite = &assets.rig.attachments[id];
    attachment(
        d,
        &assets.textures[&sprite.image],
        sprite,
        transform.point(position),
        transform.scale,
        (transform.sign < 0.0) ^ flip,
        angle * transform.sign,
        tint,
    );
}

fn cloth(
    d: &mut impl RaylibDraw,
    assets: &ActorAssets,
    id: &str,
    chain: Chain,
    transform: Transform,
    tint: Color,
) {
    let s = assets.rig.skeleton.as_ref().unwrap();
    let triangles = a::leg_mesh(s, chain);
    painted_mesh(d, assets, id, &triangles, transform, tint);
}

fn painted_mesh(
    d: &mut impl RaylibDraw,
    assets: &ActorAssets,
    id: &str,
    triangles: &[[a::Vertex; 3]],
    transform: Transform,
    tint: Color,
) {
    let sprite = &assets.rig.attachments[id];
    let texture = &assets.textures[&sprite.image];
    d.rl_set_texture(texture);
    d.rl_draw(DrawMode::Triangles, |draw| {
        draw.normal3f(0.0, 0.0, 1.0);
        draw.color4ub(tint);
        for triangle in triangles {
            let [v0, v1, v2] = *triangle;
            let p = triangle.map(|v| transform.point(v.position));
            let cross =
                (p[1].x - p[0].x) * (p[2].y - p[0].y) - (p[1].y - p[0].y) * (p[2].x - p[0].x);
            let order = if cross > 0.0 {
                [v0, v2, v1]
            } else {
                [v0, v1, v2]
            };
            for v in order {
                draw.color4ub(Color::new(
                    tint.r,
                    tint.g,
                    tint.b,
                    (tint.a as f32 * v.opacity) as u8,
                ));
                draw.texcoord2f(
                    (sprite.source[0] + v.uv[0] * sprite.source[2]) / texture.width as f32,
                    (sprite.source[1] + v.uv[1] * sprite.source[3]) / texture.height as f32,
                );
                let p = transform.point(v.position);
                draw.vertex2f(p.x, p.y);
            }
        }
    });
    d.rl_disable_texture();
}

fn arm(
    d: &mut impl RaylibDraw,
    assets: &ActorAssets,
    chain: Chain,
    transform: Transform,
    tint: Color,
) {
    piece(
        d,
        assets,
        "upper-arm",
        chain.root,
        a::angle_from_down(a::sub(chain.joint, chain.root)),
        transform,
        false,
        tint,
    );
    piece(
        d,
        assets,
        "forearm",
        chain.joint,
        a::angle_from_down(a::sub(chain.tip, chain.joint)),
        transform,
        false,
        tint,
    );
}

fn debug_chain(d: &mut impl RaylibDraw, c: Chain, t: Transform, color: Color) {
    d.draw_line_ex(t.point(c.root), t.point(c.joint), 1.5, color);
    d.draw_line_ex(t.point(c.joint), t.point(c.tip), 1.5, color);
    for p in [c.root, c.joint, c.tip] {
        d.draw_circle_v(t.point(p), 2.5, color);
    }
}

/// The lab and chapter resolve projectile identity through the owner's art pack.
pub fn draw_projectile(
    d: &mut impl RaylibDraw,
    assets: &ActorAssets,
    visual_id: &str,
    position: Point,
    facing: Facing,
    ticks: f32,
) {
    let fx = &assets.rig.effects[visual_id];
    let color = Color::new(fx.color[0], fx.color[1], fx.color[2], fx.color[3]);
    let core = Color::new(fx.core[0], fx.core[1], fx.core[2], fx.core[3]);
    let p = Vector2::new(position[0], position[1]);
    let tail = Vector2::new(p.x - facing.sign() * fx.trail_length, p.y);
    d.draw_line_ex(
        tail,
        p,
        fx.trail_width * 3.6,
        Color::new(color.r, color.g, color.b, 100),
    );
    d.draw_line_ex(tail, p, fx.trail_width, color);
    if let Some(id) = &fx.sprite {
        let sprite = &assets.rig.attachments[id];
        attachment(
            d,
            &assets.textures[&sprite.image],
            sprite,
            p,
            1.0,
            facing == Facing::Left,
            ticks * fx.spin,
            Color::WHITE,
        );
    } else {
        d.draw_circle_v(p, fx.radius * 0.44, core);
        d.draw_poly_lines_ex(p, fx.sides as i32, fx.radius, ticks * fx.spin, 2.0, color);
    }
}

/// Presentation-only draw. Tick/distance and the pose are supplied by the shared model.
#[allow(clippy::too_many_arguments)]
pub fn draw_actor(
    d: &mut impl RaylibDraw,
    assets: &ActorAssets,
    pose: Pose,
    clip: &str,
    ticks: f32,
    position: Point,
    facing: Facing,
    scale: f32,
    debug: bool,
) {
    let transform = Transform {
        at: position,
        sign: facing.sign(),
        scale,
    };
    if assets.rig.method == Method::Frames {
        let sample = assets.clips.sample(clip, ticks, ticks * 3.0);
        if let Some(frame) = sample.frame {
            let offset = a::add(pose.pelvis, [0.0, 96.0]);
            piece(
                d,
                assets,
                frame,
                offset,
                pose.body_angle,
                transform,
                false,
                Color::WHITE,
            );
        }
        return;
    }
    let s = assets.rig.skeleton.as_ref().unwrap();
    let view = assets.rig.view(pose.yaw).unwrap();
    let shoulders = view.shoulders.unwrap_or(s.shoulders);
    let hips = view.hips.unwrap_or(s.hips);
    let view_sign = if view.flip { -1.0 } else { 1.0 };
    let place = |p: Point| {
        a::add(
            pose.pelvis,
            a::rotate([p[0] * view_sign, p[1]], pose.body_angle),
        )
    };
    let legs = std::array::from_fn::<_, 2, _>(|i| {
        let root = place(hips[i]);
        let ankle = a::add(pose.feet[i], [0.0, -s.ankle_height]);
        a::solve_chain(root, ankle, s.thigh, s.shin, -1.0)
    });
    let arms = std::array::from_fn::<_, 2, _>(|i| {
        a::solve_arm(place(shoulders[i]), pose.hands[i], s.upper_arm, s.forearm)
    });
    let back_tint = Color::new(182, 188, 201, 255);
    arm(d, assets, arms[1], transform, back_tint);
    for (i, tint) in [(1, back_tint), (0, Color::WHITE)] {
        cloth(d, assets, &view.leg, legs[i], transform, tint);
        // Airborne boots follow the shin; planted soles retain authored pitch.
        let airborne = (-pose.feet[i][1] / 12.0).clamp(0.0, 1.0);
        let shin_angle = a::angle_from_down(a::sub(legs[i].tip, legs[i].joint));
        let foot_angle =
            pose.foot_angles[i] + a::angular_delta(pose.foot_angles[i], shin_angle) * airborne;
        piece(
            d,
            assets,
            &view.boot,
            legs[i].tip,
            foot_angle,
            transform,
            view.flip,
            tint,
        );
    }
    if view.body_leg_blend.is_some() {
        let mesh = a::body_mesh(
            &assets.rig.attachments[&view.body],
            pose,
            view,
            hips,
            legs,
            s.leg_width,
        );
        painted_mesh(d, assets, &view.body, &mesh, transform, Color::WHITE);
    } else {
        piece(
            d,
            assets,
            &view.body,
            pose.pelvis,
            pose.body_angle,
            transform,
            view.flip,
            Color::WHITE,
        );
    }
    let bag = place(view.bag.unwrap_or([-18.0, 0.0]));
    piece(
        d,
        assets,
        "bag",
        bag,
        pose.body_angle + pose.bag_angle,
        transform,
        view.flip,
        Color::WHITE,
    );
    arm(d, assets, arms[0], transform, Color::WHITE);
    if debug {
        for c in legs {
            debug_chain(d, c, transform, Color::LIME);
        }
        for c in arms {
            debug_chain(d, c, transform, Color::SKYBLUE);
        }
        d.draw_circle_v(transform.point(pose.pelvis), 3.0, Color::ORANGE);
        for f in pose.feet {
            d.draw_circle_lines_v(transform.point(f), 4.0, Color::YELLOW);
        }
    }
}
