//! Presents one painted, anatomically connected Julia/broker performance in both cameras.
//!
//! The source painting uses a green screen removed only by its draw shader. A
//! small cloth-like mesh follows the actors and pins the shared wrist to the story socket.

use super::assets::ProductionAssets;
use crate::adventure::augusta::Chapter;
use raylib::prelude::*;
use std::cell::RefCell;

const CHROMA_FRAGMENT: &str = r#"#version 330
in vec2 fragTexCoord;
in vec4 fragColor;
uniform sampler2D texture0;
uniform vec4 colDiffuse;
uniform float chromaCutoff;
out vec4 finalColor;
void main() {
    vec4 paint = texture(texture0, fragTexCoord);
    float excess = paint.g - max(paint.r, paint.b);
    float key = smoothstep(chromaCutoff, chromaCutoff + 0.16, excess);
    float alpha = paint.a * (1.0 - key);
    if (alpha < 0.015) discard;
    // Remove the green tint from filtered silhouette pixels before compositing.
    if (excess > 0.025) paint.g = min(paint.g, max(paint.r, paint.b) + 0.015);
    finalColor = vec4(paint.rgb, alpha) * fragColor * colDiffuse;
}
"#;

/// Context-owned shader; it allocates no texture beyond the world-art catalog.
pub struct RestraintArt {
    shader: RefCell<Shader>,
}

impl RestraintArt {
    pub fn load(rl: &mut RaylibHandle, thread: &RaylibThread) -> Result<Self, String> {
        let mut shader = rl.load_shader_from_memory(thread, None, Some(CHROMA_FRAGMENT));
        let cutoff = shader.get_shader_location("chromaCutoff");
        if !shader.is_shader_valid() || cutoff < 0 {
            return Err("paired restraint chroma shader could not be loaded".into());
        }
        shader.set_shader_value(cutoff, 0.04_f32);
        Ok(Self {
            shader: RefCell::new(shader),
        })
    }
}

fn smooth(value: f32) -> f32 {
    let value = value.clamp(0., 1.);
    value * value * (3. - 2. * value)
}

/// Source registrations measured on the complete 1254px paired painting.
/// There is one wrist patch, so deformation never separates his grip from her arm.
struct Performance {
    broker: Vector2,
    julia: Vector2,
    wrist: Vector2,
    tension: f32,
    breath: f32,
}

impl Performance {
    fn actor_points(&self, source: Vector2) -> [Vector2; 2] {
        let broker_scale = 194. / 1149.;
        let julia_scale = broker_scale;
        let broker = self.broker
            + Vector2::new(
                (source.x - 418.) * broker_scale,
                (source.y - 1194.) * broker_scale,
            );
        let high = ((1178. - source.y) / 1031.).clamp(0., 1.);
        let julia = self.julia
            + Vector2::new(
                (source.x - 951.) * julia_scale - self.tension * 5. * high,
                (source.y - 1178.) * julia_scale - self.breath * high,
            );
        [broker, julia]
    }

    fn base(&self, source: Vector2) -> Vector2 {
        let [broker, julia] = self.actor_points(source);
        let arm_weight = smooth((source.x - 590.) / 270.);
        let separate = smooth((source.y - 550.) / 50.);
        let body_weight = if source.x >= 660. { 1. } else { 0. };
        let weight = arm_weight + (body_weight - arm_weight) * separate;
        broker + (julia - broker) * weight
    }

    fn point(&self, source: Vector2) -> Vector2 {
        let grip = Vector2::new(700., 505.);
        let delta = self.wrist - self.base(grip);
        let distance = Vector2::new((source.x - grip.x) / 360., (source.y - grip.y) / 170.);
        let weight = (-3. * (distance.x * distance.x + distance.y * distance.y)).exp()
            * (1. - smooth((source.y - 560.) / 40.));
        self.base(source) + delta * weight
    }
}

/// Draws the connected pair in world XY (camera 640 preserves world X).
/// Callers omit their individual NPC sprites while restraint_contact is present.
pub fn draw(d: &mut impl RaylibDraw, assets: &ProductionAssets, chapter: &Chapter, camera_x: f32) {
    let Some(contact) = chapter.restraint_contact() else {
        return;
    };
    let Some(sprite) = assets.art.pieces.get("restrained-pair") else {
        return;
    };
    let npcs = chapter.npcs();
    let Some(julia) = npcs.iter().find(|npc| npc.character == "julia") else {
        return;
    };
    let offset = -camera_x + 640.;
    let performance = Performance {
        broker: Vector2::new(
            chapter.world.broker_x + offset - contact.tension * 12.,
            chapter.world.ground_y,
        ),
        // The pair shares one depth plane. Preserve the painting's common
        // pavement baseline instead of shearing separate NPC depth offsets.
        julia: Vector2::new(
            julia.position.x + offset,
            chapter.world.ground_y - 16. * 194. / 1149.,
        ),
        wrist: Vector2::new(contact.julia_wrist.x + offset, contact.julia_wrist.y),
        tension: contact.tension,
        breath: (chapter.ticks() as f32 * 0.043).sin() * 0.55,
    };
    for feet in [performance.julia, performance.broker] {
        d.draw_ellipse(
            feet.x as i32,
            feet.y as i32,
            26.,
            6.,
            Color::new(4, 8, 13, 115),
        );
    }
    let texture = &assets.textures[&sprite.image];
    let mut shader = assets.restraint.shader.borrow_mut();
    let mut keyed = d.begin_shader_mode(&mut shader);
    keyed.rl_set_texture(texture);
    // Below the connected arms, each silhouette owns a separate patch. This
    // avoids blending two moving shoes across the transparent gap between them.
    for part in [0, 2, 1] {
        keyed.rl_set_texture(texture);
        keyed.rl_draw(DrawMode::Triangles, |draw| {
            draw.normal3f(0., 0., 1.);
            draw.color4ub(Color::WHITE);
            let grid = if part == 0 { 24 } else { 16 };
            let sample = |col: u32, row: u32| {
                let u = col as f32 / grid as f32;
                let v = row as f32 / grid as f32;
                let y = if part == 0 { v * 600. } else { 600. + v * 654. };
                // This seam stays inside the painted gap, including both shoes.
                let split = if y < 1050. {
                    660.
                } else if y < 1174. {
                    660. + (y - 1050.) * 0.315
                } else {
                    699. + (y - 1174.) * 0.7
                };
                let x = match part {
                    0 => u * 1254.,
                    1 => u * split,
                    _ => split + u * (1254. - split),
                };
                Vector2::new(x, y)
            };
            for row in 0..grid {
                for col in 0..grid {
                    let source = [
                        sample(col, row),
                        sample(col + 1, row),
                        sample(col, row + 1),
                        sample(col + 1, row + 1),
                    ];
                    for i in [0, 2, 1, 1, 2, 3] {
                        let p = if part == 0 {
                            performance.point(source[i])
                        } else {
                            performance.actor_points(source[i])[(part - 1) as usize]
                        };
                        draw.texcoord2f(
                            (sprite.source[0] + source[i].x / 1254. * sprite.source[2])
                                / texture.width as f32,
                            (sprite.source[1] + source[i].y / 1254. * sprite.source[3])
                                / texture.height as f32,
                        );
                        draw.vertex2f(p.x, p.y);
                    }
                }
            }
        });
    }
    keyed.rl_disable_texture();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shared_grip_stays_pinned_without_folding_the_painted_forearms() {
        for step in 0..=32 {
            let tension = step as f32 / 32.;
            let performance = Performance {
                broker: Vector2::new(-12. * tension, 0.),
                julia: Vector2::new(90. - 22. * tension, -16. * 194. / 1149.),
                wrist: Vector2::new(50. - 22. * tension, -122.),
                tension,
                breath: 0.55,
            };
            let grip = performance.point(Vector2::new(700., 505.));
            assert!((grip - performance.wrist).length() < 0.0001);
            for x in (540..=840).step_by(10) {
                for y in (360..=590).step_by(10) {
                    let source = Vector2::new(x as f32, y as f32);
                    let origin = performance.point(source);
                    let dx = performance.point(source + Vector2::new(1., 0.)) - origin;
                    let dy = performance.point(source + Vector2::new(0., 1.)) - origin;
                    assert!(
                        dx.x * dy.y - dx.y * dy.x > 0.004,
                        "folded wrist patch at {tension}, {source:?}"
                    );
                }
            }
        }
    }
}
