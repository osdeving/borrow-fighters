//! Draws the isolated Combat Lab scene.
//!
//! System: Raylib render boundary. This module draws lab snapshots produced by
//! `scenes::combat_lab` and does not own combat rules.
//!
//! The lab renderer stays next to the Raylib boundary while the lab state stays
//! testable under `scenes/combat_lab.rs`.

use raylib::prelude::*;

use crate::characters::CharacterId;
use crate::combat::{fighter::Facing, projectile::Projectile};
use crate::config::{FLOOR_Y, WINDOW_HEIGHT, WINDOW_WIDTH, screen_px};
use crate::engine::{assets::GameAssets, sprites};
use crate::game::arena::ArenaId;
use crate::scenes::combat_lab::{CombatLab, CombatLabPose};
use crate::ui::combat_debug;

use super::{
    BACKGROUND, FighterDrawOptions, PLAYER_C, PLAYER_CPP, PLAYER_GO, PLAYER_ONE, PLAYER_PYTHON,
    PLAYER_TWO, PROJECTILE, PROJECTILE_FILL, UI_MUTED, draw_arena, draw_fighter, outline_rect,
};

/// Draws the isolated Combat Lab scene.
pub fn draw_combat_lab(draw: &mut impl super::DrawTarget, lab: &CombatLab, assets: &GameAssets) {
    if let Some(world) = lab.super_preview_world() {
        use crate::game::feature_flags::{FeatureFlag, FeatureFlags};
        let mut flags = FeatureFlags::default();
        flags.set(FeatureFlag::ShowHud, false);
        flags.set(FeatureFlag::ShowStageLife, false);
        super::draw_fight(
            draw,
            world,
            ArenaId::home_for_character(lab.character()),
            world.elapsed_seconds,
            flags,
            super::GamepadStatus::default(),
            assets,
        );
        if !super::authored_supers::replaces_frame(world) {
            combat_debug::draw_combat_lab_debug(draw, lab, assets.menu_font.as_ref());
        }
        return;
    }
    draw.clear_background(BACKGROUND);
    if lab.show_background() {
        draw_arena(
            draw,
            ArenaId::STARTING_ARENA,
            assets.arenas.get(ArenaId::STARTING_ARENA),
            lab.elapsed_seconds(),
        );
        draw_lab_grid(draw, Color::new(44, 49, 60, 118));
    } else {
        draw_lab_grid(draw, Color::new(44, 49, 60, 255));
    }

    if let Some(state) = lab.fighter().cinematic_special() {
        super::cinematic_effects::draw_background(draw, lab.fighter(), state, assets);
    }

    let (body_color, sprite_atlas, projectile_texture) = match lab.character() {
        CharacterId::Rust => (
            PLAYER_ONE,
            assets.rust_fighter.as_ref(),
            assets.rust_projectile.as_ref(),
        ),
        CharacterId::Duke => (
            PLAYER_TWO,
            assets.duke_fighter.as_ref(),
            assets.duke_projectile.as_ref(),
        ),
        CharacterId::Go => (
            PLAYER_GO,
            assets.go_fighter.as_ref(),
            assets.go_projectile.as_ref(),
        ),
        CharacterId::C => (
            PLAYER_C,
            assets.c_fighter.as_ref(),
            assets.c_projectile.as_ref(),
        ),
        CharacterId::Python => (
            PLAYER_PYTHON,
            assets.python_fighter.as_ref(),
            assets.python_projectile.as_ref(),
        ),
        CharacterId::Cpp => (
            PLAYER_CPP,
            assets.cpp_fighter.as_ref(),
            assets.cpp_projectile.as_ref(),
        ),
    };
    let sprite_atlas = super::fighter_atlas_for_intro(
        lab.pose() == CombatLabPose::Spawn,
        super::character_visuals(lab.character(), assets).start_atlas,
        sprite_atlas,
    );

    draw_fighter(
        draw,
        lab.fighter(),
        FighterDrawOptions {
            body_color,
            show_debug: false,
            sprite_atlas,
            spritesheet: assets.fighter_spritesheet.as_ref(),
            world_elapsed_seconds: lab.elapsed_seconds(),
            forced_clip: forced_clip_for_pose(lab.pose()),
            placement: None,
        },
    );

    draw_lab_projectiles(draw, lab.projectiles(), projectile_texture);
    if let Some(state) = lab.fighter().cinematic_special() {
        super::cinematic_effects::draw_foreground(draw, lab.fighter(), state, assets);
    }
    combat_debug::draw_combat_lab_debug(draw, lab, assets.menu_font.as_ref());
    if lab.is_signature_actor_preview() {
        draw.draw_text(
            "Actor preview. Signature effects/contact: open Move Showcase",
            screen_px(36),
            screen_px(110),
            screen_px(13),
            UI_MUTED,
        );
    }
}

fn draw_lab_grid(draw: &mut impl super::DrawTarget, line_color: Color) {
    for x in (0..=WINDOW_WIDTH).step_by(screen_px(80) as usize) {
        draw.draw_line(x, 0, x, WINDOW_HEIGHT, line_color);
    }
    for y in (0..=WINDOW_HEIGHT).step_by(screen_px(60) as usize) {
        draw.draw_line(0, y, WINDOW_WIDTH, y, line_color);
    }
    draw.draw_line(0, FLOOR_Y as i32, WINDOW_WIDTH, FLOOR_Y as i32, UI_MUTED);
}

fn draw_lab_projectiles(
    draw: &mut impl super::DrawTarget,
    projectiles: &[Projectile],
    texture: Option<&Texture2D>,
) {
    for projectile in projectiles {
        let rect = projectile.rect();
        if let Some(texture) = texture {
            let facing = if projectile.velocity.x < 0.0 {
                Facing::Left
            } else {
                Facing::Right
            };
            let center = rect.center();
            sprites::draw_projectile_texture(
                draw,
                texture,
                Vector2::new(center.x, center.y),
                facing,
                Color::WHITE,
            );
        } else {
            draw.draw_rectangle(
                rect.x.round() as i32,
                rect.y.round() as i32,
                rect.width.round() as i32,
                rect.height.round() as i32,
                PROJECTILE_FILL,
            );
        }
        outline_rect(draw, rect, PROJECTILE);
    }
}

fn forced_clip_for_pose(pose: CombatLabPose) -> Option<sprites::FighterSpriteClip> {
    match pose {
        CombatLabPose::Move => None,
        CombatLabPose::Idle => Some(sprites::FighterSpriteClip::Idle),
        CombatLabPose::Crouch => Some(sprites::FighterSpriteClip::Crouch),
        CombatLabPose::Jump => Some(sprites::FighterSpriteClip::Jump),
        CombatLabPose::Block => Some(sprites::FighterSpriteClip::Block),
        CombatLabPose::Hit => Some(sprites::FighterSpriteClip::Hit),
        CombatLabPose::Victory => Some(sprites::FighterSpriteClip::Victory),
        CombatLabPose::Spawn => Some(sprites::FighterSpriteClip::Spawn),
        CombatLabPose::Defeat => Some(sprites::FighterSpriteClip::Defeat),
        CombatLabPose::CrouchBlock => Some(sprites::FighterSpriteClip::CrouchBlock),
    }
}
