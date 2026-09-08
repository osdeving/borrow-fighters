//! Draws authored signature animation sheets at their physical effect anchors.
//!
//! System: Raylib presentation. World owns emission, movement and collision;
//! this layer adds atlas animation and readable language jokes to those entities.

use raylib::prelude::*;

use crate::{
    combat::{
        fighter::PlayerSlot,
        signature::{SignatureEffect, SignatureEffectKind},
    },
    engine::{assets::GameAssets, sprites},
    game::world::World,
};

pub(super) fn draw_signature_effects(
    draw: &mut impl super::DrawTarget,
    world: &World,
    show_debug: bool,
    assets: &GameAssets,
) {
    for (index, effect) in world.signature_effects.iter().enumerate() {
        if show_debug
            && !effect.has_connected()
            && let Some(hitbox) = effect.hitbox()
        {
            super::outline_rect(draw, hitbox, super::HITBOX);
        }
        let character = match effect.owner {
            PlayerSlot::One => world.player_one_character(),
            PlayerSlot::Two => world.player_two_character(),
        };
        let Some(atlas) = assets.signature_atlas(character) else {
            continue;
        };
        let clip = effect_clip(effect);
        // Map the authored sequence to the lifetime of the actual entity. A
        // rocket uses its looping travel keys until World changes it to impact.
        let authored_ms = atlas
            .manifest
            .clip_named(clip)
            .map(|clip| {
                clip.frames
                    .iter()
                    .filter_map(|name| atlas.manifest.frame_named(name))
                    .map(|frame| frame.duration_ms)
                    .sum::<u32>()
            })
            .unwrap_or(1);
        let elapsed = if clip == "impact" {
            (effect.elapsed_seconds / effect.lifetime_seconds.max(0.001)) * authored_ms as f32
                / 1000.0
        } else {
            effect.elapsed_seconds
        };
        if let Some(frame) = sprites::frame_for_clip_at(&atlas.manifest, clip, elapsed)
            && let Some(texture) = atlas.texture_for_frame(frame)
        {
            let scale = atlas.manifest.scale.unwrap_or(1.0);
            let rect = frame.frame;
            let mirrored = effect.direction < 0.0;
            let source = sprites::mirrored_source_rect(
                Rectangle::new(rect.x as f32, rect.y as f32, rect.w as f32, rect.h as f32),
                mirrored,
            );
            let pivot_x = if mirrored {
                rect.w - frame.pivot.x
            } else {
                frame.pivot.x
            };
            let dest = Rectangle::new(
                effect.position.x - pivot_x as f32 * scale,
                effect.position.y - frame.pivot.y as f32 * scale,
                rect.w as f32 * scale,
                rect.h as f32 * scale,
            );
            if effect.kind == SignatureEffectKind::CppRocket {
                // Both authored rocket keys already point about 24 degrees down.
                let angle = (effect
                    .velocity
                    .y
                    .atan2(effect.velocity.x.abs())
                    .to_degrees()
                    - 24.0)
                    * effect.direction;
                draw.draw_texture_pro(
                    texture,
                    source,
                    Rectangle::new(
                        effect.position.x,
                        effect.position.y,
                        dest.width,
                        dest.height,
                    ),
                    Vector2::new(pivot_x as f32 * scale, frame.pivot.y as f32 * scale),
                    angle,
                    Color::WHITE,
                );
            } else {
                draw.draw_texture_pro(texture, source, dest, Vector2::zero(), 0.0, Color::WHITE);
            }
        }
        draw_language_caption(draw, effect, index, assets);
    }
}

fn effect_clip(effect: &SignatureEffect) -> &'static str {
    if effect.has_connected() {
        return "impact";
    }
    match effect.kind {
        SignatureEffectKind::CppExplosion | SignatureEffectKind::CMemoryRupture => "impact",
        SignatureEffectKind::RustFortress
        | SignatureEffectKind::PythonVortex
        | SignatureEffectKind::DukeCodeSheet
        | SignatureEffectKind::CppRocket => "projectile",
    }
}

fn draw_language_caption(
    draw: &mut impl super::DrawTarget,
    effect: &SignatureEffect,
    index: usize,
    assets: &GameAssets,
) {
    let (text, color, y) = match effect.kind {
        SignatureEffectKind::DukeCodeSheet => (
            "System.out.println(\"Hello, World!\");",
            Color::new(255, 250, 190, 240),
            effect.position.y - 195.0 - (index % 3) as f32 * 26.0,
        ),
        SignatureEffectKind::CMemoryRupture => (
            "SEGMENTATION FAULT",
            Color::new(255, 125, 135, 240),
            effect.position.y - 258.0,
        ),
        SignatureEffectKind::PythonVortex => (
            "import antigravity",
            Color::new(255, 225, 95, 245),
            effect.position.y - 440.0,
        ),
        SignatureEffectKind::RustFortress => (
            "MEMORY SAFE",
            Color::new(255, 205, 110, 240),
            effect.position.y - 235.0,
        ),
        SignatureEffectKind::CppExplosion => (
            "UNDEFINED BEHAVIOR!",
            Color::new(255, 214, 115, 240),
            effect.position.y - 190.0,
        ),
        SignatureEffectKind::CppRocket => return,
    };
    // Text remains upright in either facing and stays below the HUD.
    super::draw_menu_text(
        draw,
        assets.menu_font.as_ref(),
        text,
        (effect.position.x - 90.0).clamp(16.0, 910.0) as i32,
        y.max(218.0) as i32,
        13.0,
        color,
    );
}
