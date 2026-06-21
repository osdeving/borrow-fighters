//! Draws the greybox prototype.
//!
//! Rendering intentionally uses primitive shapes and debug overlays so gameplay
//! problems are visible before art production starts.

use raylib::prelude::*;

use crate::combat::fighter::{AttackPhase, Facing, PlayerSlot};
use crate::config::{ARENA_LEFT, ARENA_RIGHT, FLOOR_Y, WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::engine::assets::{GameAssets, SpriteAtlasAsset};
use crate::engine::sprites;
use crate::game::feature_flags::{FeatureFlag, FeatureFlags, PREFERENCE_FLAGS};
use crate::game::world::{MatchOutcome, World};
use crate::math::rect::Rect;
use crate::scenes::preferences::PreferencesMenu;

const BACKGROUND: Color = Color::new(18, 20, 26, 255);
const FLOOR: Color = Color::new(72, 76, 88, 255);
const PLAYER_ONE: Color = Color::new(112, 181, 255, 255);
const PLAYER_TWO: Color = Color::new(255, 178, 104, 255);
const BODY_OUTLINE: Color = Color::new(238, 241, 247, 255);
const HURTBOX: Color = Color::new(105, 240, 174, 255);
const HITBOX: Color = Color::new(255, 82, 82, 255);
const HITBOX_FILL: Color = Color::new(255, 82, 82, 82);
const HITSPARK: Color = Color::new(255, 235, 59, 255);
const BODY_COLLISION: Color = Color::new(218, 112, 214, 255);
const PROJECTILE: Color = Color::new(80, 220, 255, 255);
const PROJECTILE_FILL: Color = Color::new(80, 220, 255, 110);
const GUARD: Color = Color::new(86, 156, 255, 255);
const GUARD_FILL: Color = Color::new(86, 156, 255, 90);
const UI_TEXT: Color = Color::new(238, 241, 247, 255);
const UI_MUTED: Color = Color::new(165, 172, 185, 255);
const HEALTH_BACK: Color = Color::new(60, 62, 70, 255);
const HEALTH_FILL: Color = Color::new(76, 217, 100, 255);
const HEALTH_DANGER: Color = Color::new(255, 82, 82, 255);
const PANEL: Color = Color::new(12, 14, 20, 218);
const PANEL_BORDER: Color = Color::new(122, 132, 150, 255);
const SELECTED_ROW: Color = Color::new(42, 49, 64, 230);

/// Connected gamepad status reported by Raylib for this frame.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct GamepadStatus {
    pub player_one: bool,
    pub player_two: bool,
}

/// Draws the current world state.
pub fn draw_fight(
    draw: &mut RaylibDrawHandle<'_>,
    world: &World,
    flags: FeatureFlags,
    gamepad_status: GamepadStatus,
    assets: &GameAssets,
) {
    draw.clear_background(BACKGROUND);
    draw_arena(draw, assets.arena_background.as_ref());
    let show_debug = flags.enabled(FeatureFlag::ShowCombatDebug);

    draw_projectiles(
        draw,
        world,
        show_debug,
        assets.rust_projectile.as_ref(),
        assets.duke_projectile.as_ref(),
    );
    draw_fighter(
        draw,
        &world.player_one,
        FighterDrawOptions {
            body_color: PLAYER_ONE,
            show_debug,
            sprite_atlas: assets.rust_fighter.as_ref(),
            spritesheet: assets.fighter_spritesheet.as_ref(),
            world_elapsed_seconds: world.elapsed_seconds,
            forced_clip: forced_victory_clip(world, PlayerSlot::One),
        },
    );
    draw_fighter(
        draw,
        &world.player_two,
        FighterDrawOptions {
            body_color: PLAYER_TWO,
            show_debug,
            sprite_atlas: assets.duke_fighter.as_ref(),
            spritesheet: assets.fighter_spritesheet.as_ref(),
            world_elapsed_seconds: world.elapsed_seconds,
            forced_clip: forced_victory_clip(world, PlayerSlot::Two),
        },
    );
    if show_debug {
        draw_body_collision(draw, world);
    }
    draw_hit_effects(draw, world);

    if flags.enabled(FeatureFlag::ShowHud) {
        draw_hud(draw, world, flags, gamepad_status);
    }

    if flags.enabled(FeatureFlag::ShowControlsHelp) {
        draw_help(draw);
    }
}

/// Draws the initial preferences screen.
pub fn draw_preferences(
    draw: &mut RaylibDrawHandle<'_>,
    menu: &PreferencesMenu,
    flags: FeatureFlags,
    gamepad_status: GamepadStatus,
    assets: &GameAssets,
) {
    draw.clear_background(BACKGROUND);
    draw_arena(draw, assets.arena_background.as_ref());
    draw.draw_rectangle(0, 0, WINDOW_WIDTH, WINDOW_HEIGHT, Color::new(0, 0, 0, 138));

    let panel_x = 88;
    let panel_y = 64;
    let panel_width = WINDOW_WIDTH - panel_x * 2;
    let panel_height = WINDOW_HEIGHT - panel_y * 2;
    draw.draw_rectangle(panel_x, panel_y, panel_width, panel_height, PANEL);
    draw.draw_rectangle_lines(panel_x, panel_y, panel_width, panel_height, PANEL_BORDER);

    draw.draw_text("Borrow Fighters", panel_x + 32, panel_y + 26, 30, UI_TEXT);
    draw.draw_text(
        "Ajustes do prototipo",
        panel_x + 32,
        panel_y + 62,
        18,
        UI_MUTED,
    );

    let status = format!(
        "Joystick Raylib: P1 {} | P2 {}",
        connected_label(gamepad_status.player_one),
        connected_label(gamepad_status.player_two)
    );
    let status_width = draw.measure_text(&status, 16);
    draw.draw_text(
        &status,
        panel_x + panel_width - status_width - 32,
        panel_y + 34,
        16,
        UI_MUTED,
    );

    let row_start_y = panel_y + 90;
    let row_spacing = 36;

    draw_menu_row(
        draw,
        MenuRow {
            x: panel_x + 32,
            y: row_start_y,
            width: panel_width - 64,
            selected: menu.selected() == 0,
            label: "Comecar luta",
            description: "Enter/Menu inicia ou volta para a luta.",
            checked: None,
        },
    );

    for (index, flag) in PREFERENCE_FLAGS.iter().copied().enumerate() {
        let row = index + 1;
        draw_menu_row(
            draw,
            MenuRow {
                x: panel_x + 32,
                y: row_start_y + row as i32 * row_spacing,
                width: panel_width - 64,
                selected: menu.selected() == row,
                label: flag.label(),
                description: flag.description(),
                checked: Some(flags.enabled(flag)),
            },
        );
    }

    draw.draw_text(
        "Setas/W/S navegam | Espaco alterna | Enter comeca | Esc abre ajustes durante a luta",
        panel_x + 32,
        panel_y + panel_height - 34,
        15,
        UI_MUTED,
    );
}

struct MenuRow<'a> {
    x: i32,
    y: i32,
    width: i32,
    selected: bool,
    label: &'a str,
    description: &'a str,
    checked: Option<bool>,
}

fn draw_menu_row(draw: &mut RaylibDrawHandle<'_>, row: MenuRow<'_>) {
    let height = 32;
    if row.selected {
        draw.draw_rectangle(row.x, row.y - 2, row.width, height + 4, SELECTED_ROW);
        draw.draw_rectangle_lines(row.x, row.y - 2, row.width, height + 4, PANEL_BORDER);
    }

    let label_x = if let Some(enabled) = row.checked {
        draw_checkbox(draw, row.x + 14, row.y + 6, enabled);
        row.x + 48
    } else {
        draw.draw_text(">", row.x + 18, row.y + 6, 18, UI_TEXT);
        row.x + 48
    };

    draw.draw_text(row.label, label_x, row.y + 2, 18, UI_TEXT);
    draw.draw_text(row.description, label_x, row.y + 20, 12, UI_MUTED);
}

fn draw_checkbox(draw: &mut RaylibDrawHandle<'_>, x: i32, y: i32, enabled: bool) {
    let size = 18;
    draw.draw_rectangle_lines(x, y, size, size, UI_TEXT);
    if enabled {
        draw.draw_rectangle(x + 4, y + 4, size - 8, size - 8, HEALTH_FILL);
    }
}

fn draw_arena(draw: &mut RaylibDrawHandle<'_>, background: Option<&Texture2D>) {
    if let Some(texture) = background {
        draw_arena_background(draw, texture);
    } else {
        draw.draw_rectangle(
            0,
            FLOOR_Y as i32,
            WINDOW_WIDTH,
            WINDOW_HEIGHT - FLOOR_Y as i32,
            FLOOR,
        );
    }

    draw.draw_line(
        ARENA_LEFT as i32,
        FLOOR_Y as i32,
        ARENA_RIGHT as i32,
        FLOOR_Y as i32,
        UI_MUTED,
    );
    draw.draw_line(
        ARENA_LEFT as i32,
        96,
        ARENA_LEFT as i32,
        FLOOR_Y as i32,
        UI_MUTED,
    );
    draw.draw_line(
        ARENA_RIGHT as i32,
        96,
        ARENA_RIGHT as i32,
        FLOOR_Y as i32,
        UI_MUTED,
    );
}

fn draw_arena_background(draw: &mut RaylibDrawHandle<'_>, texture: &Texture2D) {
    let source = Rectangle::new(0.0, 0.0, texture.width() as f32, texture.height() as f32);
    let dest = Rectangle::new(0.0, 0.0, WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32);
    draw.draw_texture_pro(
        texture,
        source,
        dest,
        Vector2::new(0.0, 0.0),
        0.0,
        Color::WHITE,
    );

    draw.draw_rectangle(0, 0, WINDOW_WIDTH, WINDOW_HEIGHT, Color::new(0, 0, 0, 50));
    draw.draw_rectangle(
        0,
        FLOOR_Y as i32,
        WINDOW_WIDTH,
        WINDOW_HEIGHT - FLOOR_Y as i32,
        Color::new(0, 0, 0, 34),
    );
}

fn draw_fighter(
    draw: &mut RaylibDrawHandle<'_>,
    fighter: &crate::combat::fighter::Fighter,
    options: FighterDrawOptions<'_>,
) {
    let phase = fighter.attack_phase();
    let body = match phase {
        AttackPhase::Idle => options.body_color,
        AttackPhase::Startup => lighten(options.body_color, 30),
        AttackPhase::Active => Color::new(255, 222, 89, 255),
        AttackPhase::Recovery => dim(options.body_color, 25),
    };

    if let Some(sprite_atlas) = options.sprite_atlas
        && sprites::draw_manifest_fighter_sprite(
            draw,
            &sprite_atlas.texture,
            &sprite_atlas.manifest,
            fighter,
            options.world_elapsed_seconds,
            options.forced_clip,
            Color::WHITE,
        )
    {
    } else if let Some(texture) = options.spritesheet {
        sprites::draw_fighter_sprite(draw, texture, fighter, body);
    } else {
        draw_body_parts(draw, fighter, body);
    }

    if options.show_debug {
        outline_rect(draw, fighter.body_rect(), BODY_OUTLINE);
        for hurtbox in fighter.hurtboxes().rects() {
            outline_rect(draw, hurtbox, HURTBOX);
        }
    }

    if options.show_debug
        && let Some(attack_box) = fighter.attack_box()
    {
        draw.draw_rectangle(
            attack_box.x.round() as i32,
            attack_box.y.round() as i32,
            attack_box.width.round() as i32,
            attack_box.height.round() as i32,
            if phase == AttackPhase::Active {
                HITBOX_FILL
            } else {
                Color::new(255, 82, 82, 34)
            },
        );
        outline_rect(draw, attack_box, HITBOX);
    }

    if fighter.blocking {
        let guard = fighter.guard_box();
        draw.draw_rectangle(
            guard.x.round() as i32,
            guard.y.round() as i32,
            guard.width.round() as i32,
            guard.height.round() as i32,
            GUARD_FILL,
        );
        outline_rect(draw, guard, GUARD);
        if options.show_debug {
            draw.draw_text("BLOCK", guard.x as i32 - 18, guard.y as i32 - 22, 16, GUARD);
        }
    }

    if options.show_debug
        && let Some(hitbox) = fighter.active_hitbox()
    {
        draw.draw_text(
            "ACTIVE",
            hitbox.x as i32,
            (hitbox.y - 22.0) as i32,
            16,
            HITBOX,
        );
    }

    let label_x = fighter.position.x as i32;
    let label_y = (fighter.position.y - 22.0) as i32;
    draw.draw_text(fighter.name, label_x, label_y, 16, UI_TEXT);

    if options.show_debug && phase != AttackPhase::Idle {
        let attack_label = fighter
            .attack_kind()
            .map_or("ATTACK", crate::combat::fighter::AttackKind::label);
        let text = match phase {
            AttackPhase::Startup => attack_label,
            AttackPhase::Active => "HITBOX",
            AttackPhase::Recovery => "RECOVER",
            AttackPhase::Idle => "",
        };
        draw.draw_text(text, label_x, label_y - 22, 18, HITSPARK);
    } else if options.show_debug && fighter.crouching {
        draw.draw_text("CROUCH", label_x, label_y - 22, 18, UI_MUTED);
    }
}

struct FighterDrawOptions<'a> {
    body_color: Color,
    show_debug: bool,
    sprite_atlas: Option<&'a SpriteAtlasAsset>,
    spritesheet: Option<&'a Texture2D>,
    world_elapsed_seconds: f32,
    forced_clip: Option<sprites::FighterSpriteClip>,
}

fn draw_hud(
    draw: &mut RaylibDrawHandle<'_>,
    world: &World,
    flags: FeatureFlags,
    gamepad_status: GamepadStatus,
) {
    draw.draw_text(
        "Borrow Fighters / Prototype 0.1 Greybox",
        24,
        12,
        20,
        UI_TEXT,
    );

    let status = format!(
        "P2 CPU {} | Pad P1 {} | P2 {}",
        connected_label(flags.enabled(FeatureFlag::PlayerTwoCpu)),
        connected_label(gamepad_status.player_one),
        connected_label(gamepad_status.player_two)
    );
    let width = draw.measure_text(&status, 14);
    draw.draw_text(&status, WINDOW_WIDTH - width - 24, 16, 14, UI_MUTED);

    draw_health_bar(draw, 24, 72, world.player_one.health, "Rust");
    draw_health_bar(
        draw,
        WINDOW_WIDTH - 324,
        72,
        world.player_two.health,
        "Java",
    );

    if let Some(outcome) = world.outcome {
        let message = match outcome {
            MatchOutcome::Winner(PlayerSlot::One) => "Rust wins - press R/Menu",
            MatchOutcome::Winner(PlayerSlot::Two) => "Java wins - press R/Menu",
            MatchOutcome::Draw => "Draw - press R/Menu",
        };
        let width = draw.measure_text(message, 32);
        draw.draw_text(message, (WINDOW_WIDTH - width) / 2, 124, 32, UI_TEXT);
    }
}

fn draw_help(draw: &mut RaylibDrawHandle<'_>) {
    draw.draw_text(
        "P1: A/D/W/S/Q or Pad LS/DPad, A jump, LB/LT block",
        24,
        WINDOW_HEIGHT - 100,
        16,
        UI_TEXT,
    );
    draw.draw_text(
        "P1 attacks: F/H/V/G or Pad X/Y/B/RB",
        24,
        WINDOW_HEIGHT - 76,
        16,
        UI_TEXT,
    );
    draw.draw_text(
        "P2: CPU default; C or View toggles manual",
        24,
        WINDOW_HEIGHT - 52,
        16,
        UI_TEXT,
    );
    draw.draw_text(
        "P2 manual: keyboard or second Pad same layout; Start/R restarts",
        24,
        WINDOW_HEIGHT - 28,
        16,
        UI_MUTED,
    );
}

fn connected_label(connected: bool) -> &'static str {
    if connected { "ON" } else { "OFF" }
}

fn draw_health_bar(draw: &mut RaylibDrawHandle<'_>, x: i32, y: i32, health: i32, label: &str) {
    let width = 300;
    let height = 18;
    let fill_width = (width as f32 * (health.max(0) as f32 / 100.0)).round() as i32;
    let fill = if health <= 24 {
        HEALTH_DANGER
    } else {
        HEALTH_FILL
    };

    draw.draw_rectangle(x, y, width, height, HEALTH_BACK);
    draw.draw_rectangle(x, y, fill_width, height, fill);
    draw.draw_rectangle_lines(x, y, width, height, UI_TEXT);

    let text = format!("{label} HP {health:03}");
    draw.draw_text(&text, x, y - 24, 20, UI_TEXT);
}

fn draw_projectiles(
    draw: &mut RaylibDrawHandle<'_>,
    world: &World,
    show_debug: bool,
    rust_projectile: Option<&Texture2D>,
    duke_projectile: Option<&Texture2D>,
) {
    for projectile in &world.projectiles {
        let rect = projectile.rect();
        let projectile_texture = match projectile.owner {
            PlayerSlot::One => rust_projectile,
            PlayerSlot::Two => duke_projectile,
        };
        if let Some(texture) = projectile_texture {
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
            draw.draw_circle(
                rect.center().x.round() as i32,
                rect.center().y.round() as i32,
                8.0,
                PROJECTILE,
            );
        }

        if show_debug {
            outline_rect(draw, rect, PROJECTILE);
            draw.draw_text(
                "FIREBALL",
                rect.x as i32 - 12,
                rect.y as i32 - 20,
                14,
                PROJECTILE,
            );
        }
    }
}

fn forced_victory_clip(world: &World, slot: PlayerSlot) -> Option<sprites::FighterSpriteClip> {
    matches!(world.outcome, Some(MatchOutcome::Winner(winner)) if winner == slot)
        .then_some(sprites::FighterSpriteClip::Taunt)
}

fn draw_body_collision(draw: &mut RaylibDrawHandle<'_>, world: &World) {
    if world.body_collision_timer <= 0.0 {
        return;
    }

    let p1 = world.player_one.body_rect();
    let p2 = world.player_two.body_rect();
    let x = if p1.center_x() <= p2.center_x() {
        ((p1.right() + p2.x) * 0.5).round() as i32
    } else {
        ((p2.right() + p1.x) * 0.5).round() as i32
    };
    let top = p1.y.max(p2.y).round() as i32;
    let bottom = p1.bottom().min(p2.bottom()).round() as i32;

    draw.draw_line_ex(
        Vector2::new(x as f32, top as f32),
        Vector2::new(x as f32, bottom as f32),
        6.0,
        BODY_COLLISION,
    );
    draw.draw_text("BODY COLLISION", x - 76, top - 24, 18, BODY_COLLISION);
}

fn draw_hit_effects(draw: &mut RaylibDrawHandle<'_>, world: &World) {
    for effect in &world.hit_effects {
        let color = if effect.blocked { GUARD } else { HITSPARK };
        let x = effect.position.x.round() as i32;
        let y = effect.position.y.round() as i32;
        let radius = (10.0 + effect.timer * 32.0).round() as i32;
        draw.draw_circle_lines(x, y, radius as f32, color);
        draw.draw_line(x - radius, y, x + radius, y, color);
        draw.draw_line(x, y - radius, x, y + radius, color);

        let damage = format!("-{}", effect.damage);
        draw.draw_text(&damage, x + 14, y - 18, 24, color);
        let label = if effect.blocked { "BLOCK" } else { "HIT" };
        draw.draw_text(label, x - 18, y - 42, 20, color);
    }
}

fn draw_body_parts(
    draw: &mut RaylibDrawHandle<'_>,
    fighter: &crate::combat::fighter::Fighter,
    color: Color,
) {
    let parts = fighter.body_parts();
    fill_rect(draw, parts.head, lighten(color, 28));
    fill_rect(draw, parts.torso, color);
    fill_rect(draw, parts.legs, dim(color, 22));
}

fn fill_rect(draw: &mut RaylibDrawHandle<'_>, rect: Rect, color: Color) {
    draw.draw_rectangle(
        rect.x.round() as i32,
        rect.y.round() as i32,
        rect.width.round() as i32,
        rect.height.round() as i32,
        color,
    );
}

fn outline_rect(draw: &mut RaylibDrawHandle<'_>, rect: Rect, color: Color) {
    draw.draw_rectangle_lines(
        rect.x.round() as i32,
        rect.y.round() as i32,
        rect.width.round() as i32,
        rect.height.round() as i32,
        color,
    );
}

fn lighten(color: Color, amount: u8) -> Color {
    Color::new(
        color.r.saturating_add(amount),
        color.g.saturating_add(amount),
        color.b.saturating_add(amount),
        color.a,
    )
}

fn dim(color: Color, amount: u8) -> Color {
    Color::new(
        color.r.saturating_sub(amount),
        color.g.saturating_sub(amount),
        color.b.saturating_sub(amount),
        color.a,
    )
}
