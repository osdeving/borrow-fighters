//! Draws arenas, fighters, menus and match presentation.
//!
//! System: Raylib presentation. Artwork and smooth embedded typography share
//! screen geometry with pointer input; debug overlays remain optional.

use raylib::core::text::RaylibFont;
use raylib::prelude::*;
use std::{f32::consts::TAU, ffi::CString};

mod authored_actors;
mod authored_supers;
mod cinematic_effects;
mod combat_lab;
mod move_showcase;
mod signature_effects;
mod sprite_viewer;
mod stage_life;

pub use combat_lab::draw_combat_lab;
pub use move_showcase::draw_move_showcase;
pub use sprite_viewer::{draw_sprite_viewer, draw_sprite_viewer_error};

use crate::characters::CharacterId;
use crate::combat::fighter::{AttackPhase, Facing, Fighter, PlayerSlot};
use crate::combat::projectile::Projectile;
use crate::config::{
    ARENA_LEFT, ARENA_RIGHT, FLOOR_Y, WINDOW_HEIGHT, WINDOW_WIDTH, screen_px, world_px,
};
use crate::engine::assets::{GameAssets, SpriteAtlasAsset};
use crate::engine::sprites;
use crate::game::arena::ArenaId;
use crate::game::feature_flags::{FeatureFlag, FeatureFlags, PREFERENCE_FLAGS};
use crate::game::world::{HIT_EFFECT_LIFETIME, MatchOutcome, World};
use crate::lore::{LoreBook, LoreChapter, LoreCharacter};
use crate::math::rect::Rect;
use crate::scenes::preferences::{MenuPage, PreferencesMenu};
use crate::ui::binary_text::{DEFAULT_BINARY_REVEAL_FRAMES, binary_reveal_text_with_seed};
use crate::ui::menu_layout::{MenuBounds as MenuPanel, MenuLayout};

const BACKGROUND: Color = Color::new(18, 20, 26, 255);
const FLOOR: Color = Color::new(72, 76, 88, 255);
const PLAYER_ONE: Color = Color::new(112, 181, 255, 255);
const PLAYER_TWO: Color = Color::new(255, 178, 104, 255);
const PLAYER_GO: Color = Color::new(96, 220, 190, 255);
pub(super) const PLAYER_C: Color = Color::new(126, 194, 255, 255);
pub(super) const PLAYER_PYTHON: Color = Color::new(255, 210, 92, 255);
pub(super) const PLAYER_CPP: Color = Color::new(118, 214, 255, 255);
const BODY_OUTLINE: Color = Color::new(238, 241, 247, 255);
const HURTBOX: Color = Color::new(105, 240, 174, 255);
const HITBOX: Color = Color::new(255, 82, 82, 255);
const HITBOX_FILL: Color = Color::new(255, 82, 82, 82);
const HITSPARK: Color = Color::new(255, 235, 59, 255);
const HIT_FLASH: Color = Color::new(255, 110, 86, 255);
const HIT_FLASH_FILL: Color = Color::new(255, 69, 58, 58);
const BLOCK_FLASH: Color = Color::new(154, 205, 255, 255);
const BLOCK_FLASH_FILL: Color = Color::new(86, 156, 255, 44);
const ACTIVE_SPRITE_TINT: Color = Color::new(255, 244, 177, 255);
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
const RECORDING: Color = Color::new(255, 55, 72, 255);
const MENU_PANEL_STRONG: Color = Color::new(8, 14, 28, 238);
const MENU_ACCENT: Color = Color::new(0, 202, 255, 255);
const MENU_ACCENT_ALT: Color = Color::new(255, 191, 67, 255);
const MENU_HACK_GREEN: Color = Color::new(95, 255, 174, 255);
const MENU_CURSOR: Color = Color::new(255, 218, 92, 255);
const MENU_MAGENTA: Color = Color::new(255, 87, 178, 100);
const MENU_ROW: Color = Color::new(10, 20, 39, 218);
const MENU_ROW_SELECTED: Color = Color::new(19, 52, 85, 240);

/// Draw target accepted by game renderers, either the window or a render texture.
pub trait DrawTarget: RaylibDraw {}

impl<T: RaylibDraw> DrawTarget for T {}

/// Connected gamepad status reported by Raylib for this frame.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct GamepadStatus {
    pub player_one: bool,
    pub player_two: bool,
}

/// Draws the off-screen render target into the visible window.
pub fn draw_render_target_to_window(draw: &mut impl DrawTarget, target: &RenderTexture2D) {
    let texture = target.texture();
    let source = Rectangle::new(0.0, 0.0, texture.width() as f32, -(texture.height() as f32));
    let dest = Rectangle::new(0.0, 0.0, WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32);
    draw.clear_background(BACKGROUND);
    draw.draw_texture_pro(
        texture,
        source,
        dest,
        Vector2::new(0.0, 0.0),
        0.0,
        Color::WHITE,
    );
}

/// Draws the current world state.
pub fn draw_fight(
    draw: &mut impl DrawTarget,
    world: &World,
    arena: ArenaId,
    visual_time_seconds: f32,
    flags: FeatureFlags,
    gamepad_status: GamepadStatus,
    assets: &GameAssets,
) {
    if authored_supers::draw_override(draw, world) {
        return;
    }
    draw.clear_background(BACKGROUND);
    let visual_time_seconds = authored_supers::arena_time(world, visual_time_seconds);
    draw_arena(draw, arena, assets.arenas.get(arena), visual_time_seconds);
    draw_stage_life_layer(draw, arena, visual_time_seconds, flags, Some(world), assets);
    draw_world_cinematic_background(draw, world, assets);
    let show_debug = flags.enabled(FeatureFlag::ShowCombatDebug);
    if show_debug {
        draw_arena_bounds(draw);
    }
    let spawn_intro = world.spawn_intro_active();
    let player_one_visuals = character_visuals(world.player_one_character(), assets);
    let player_two_visuals = character_visuals(world.player_two_character(), assets);
    let (player_one_clip, player_one_time) =
        fighter_match_presentation(world, &world.player_one, spawn_intro);
    let (player_two_clip, player_two_time) =
        fighter_match_presentation(world, &world.player_two, spawn_intro);

    draw_fighter_ground_lights(draw, world);
    draw_projectiles(draw, world, show_debug, assets);
    if !hides_authored_actor(world, world.player_one.slot, assets) {
        draw_fighter(
            draw,
            &world.player_one,
            FighterDrawOptions {
                body_color: player_one_visuals.body_color,
                show_debug,
                sprite_atlas: fighter_atlas_for_intro(
                    spawn_intro,
                    player_one_visuals.start_atlas,
                    player_one_visuals.fight_atlas,
                ),
                spritesheet: assets.fighter_spritesheet.as_ref(),
                world_elapsed_seconds: player_one_time,
                forced_clip: player_one_clip,
            },
        );
    }
    if !hides_authored_actor(world, world.player_two.slot, assets) {
        draw_fighter(
            draw,
            &world.player_two,
            FighterDrawOptions {
                body_color: player_two_visuals.body_color,
                show_debug,
                sprite_atlas: fighter_atlas_for_intro(
                    spawn_intro,
                    player_two_visuals.start_atlas,
                    player_two_visuals.fight_atlas,
                ),
                spritesheet: assets.fighter_spritesheet.as_ref(),
                world_elapsed_seconds: player_two_time,
                forced_clip: player_two_clip,
            },
        );
    }
    draw_authored_actors(draw, world, assets);
    signature_effects::draw_signature_effects(draw, world, show_debug, assets);
    if show_debug {
        draw_body_collision(draw, world);
    }
    draw_hit_effects(draw, world, assets.menu_font.as_ref());
    draw_world_cinematic_foreground(draw, world, assets);

    if flags.enabled(FeatureFlag::ShowHud) {
        draw_hud(draw, world, flags, gamepad_status, show_debug, assets);
    }

    if let Some(label) = world.countdown_label() {
        draw_countdown(draw, label, assets);
    }

    if flags.enabled(FeatureFlag::ShowControlsHelp) {
        draw_help(draw, assets.menu_font.as_ref());
    }
}

/// Draws the main menu and nested prototype setup screens.
pub fn draw_preferences(draw: &mut impl DrawTarget, options: PreferencesDrawOptions<'_>) {
    draw.clear_background(BACKGROUND);
    draw_arena(
        draw,
        options.arena,
        options.assets.arenas.get(options.arena),
        options.visual_time_seconds,
    );
    draw_stage_life_layer(
        draw,
        options.arena,
        options.visual_time_seconds,
        options.flags,
        None,
        options.assets,
    );
    draw.draw_rectangle(0, 0, WINDOW_WIDTH, WINDOW_HEIGHT, Color::new(0, 0, 0, 164));
    draw.draw_rectangle(0, 0, WINDOW_WIDTH, WINDOW_HEIGHT, Color::new(4, 9, 22, 68));

    let font = options.assets.menu_font.as_ref();
    draw_menu_backdrop(draw, font);
    draw_menu_chrome(draw, font, &options);

    match options.menu.page() {
        MenuPage::Main => draw_main_menu(draw, font, &options),
        MenuPage::Versus => draw_versus_menu(draw, font, &options),
        MenuPage::Training => draw_training_menu(draw, font, &options),
        MenuPage::Lore => draw_lore_menu(draw, font, &options),
        MenuPage::Options => draw_options_menu(draw, font, &options),
    }
}

/// Draws the global local-recording status over any scene.
pub fn draw_video_capture_overlay(
    draw: &mut impl DrawTarget,
    recording: bool,
    message: Option<&str>,
) {
    if recording {
        let text = "REC  F10 para";
        let font_size = screen_px(16);
        let width = measure_text_width(text, font_size);
        let box_width = width + screen_px(48);
        let x = (WINDOW_WIDTH - box_width) / 2;
        let y = screen_px(14);

        draw.draw_rectangle(x, y, box_width, screen_px(28), Color::new(8, 10, 14, 218));
        draw.draw_rectangle_lines(x, y, box_width, screen_px(28), RECORDING);
        draw.draw_circle(
            x + screen_px(18),
            y + screen_px(14),
            world_px(6.0),
            RECORDING,
        );
        draw.draw_text(
            text,
            x + screen_px(34),
            y + screen_px(6),
            font_size,
            UI_TEXT,
        );
        return;
    }

    let Some(message) = message else {
        return;
    };
    if !message.starts_with("Falha") && !message.starts_with("Gravacao salva") {
        return;
    }

    let text = truncate_middle(message, 92);
    let font_size = screen_px(13);
    let width = measure_text_width(&text, font_size);
    let x = WINDOW_WIDTH - width - screen_px(20);
    let y = WINDOW_HEIGHT - screen_px(24);
    draw.draw_rectangle(
        x - screen_px(8),
        y - screen_px(4),
        width + screen_px(16),
        screen_px(22),
        Color::new(8, 10, 14, 190),
    );
    draw.draw_text(&text, x, y, font_size, UI_MUTED);
}

/// Draws the WSL fallback pointer while the native cursor remains enabled.
pub fn draw_linker_chip_cursor(
    draw: &mut impl DrawTarget,
    mouse_position: Vector2,
    visual_time_seconds: f32,
    assets: &GameAssets,
) {
    if mouse_position.x < 0.0
        || mouse_position.y < 0.0
        || mouse_position.x >= WINDOW_WIDTH as f32
        || mouse_position.y >= WINDOW_HEIGHT as f32
    {
        return;
    }

    // Scale the complete pointer around its hotspot, including text and glow.
    let mut cursor_draw = draw.begin_mode2D(Camera2D {
        offset: mouse_position,
        target: mouse_position,
        rotation: 0.0,
        zoom: 0.5,
    });
    let draw = &mut cursor_draw;

    let chip_size = screen_px(42);
    let pin_len = screen_px(7);
    let margin = pin_len + screen_px(4);
    let x = (mouse_position.x.round() as i32 - chip_size / 2)
        .clamp(margin, WINDOW_WIDTH - chip_size - margin);
    let y = (mouse_position.y.round() as i32 - chip_size / 2)
        .clamp(margin, WINDOW_HEIGHT - chip_size - margin);
    let pulse_alpha = 28 + ((visual_time_seconds * TAU * 1.4).sin().max(0.0) * 30.0) as u8;
    let center_x = x + chip_size / 2;
    let center_y = y + chip_size / 2;

    draw.draw_circle_gradient(
        center_x,
        center_y,
        screen_px(34) as f32,
        Color::new(0, 202, 255, pulse_alpha),
        Color::new(0, 202, 255, 0),
    );

    draw.draw_rectangle(
        x + screen_px(4),
        y + screen_px(5),
        chip_size,
        chip_size,
        Color::new(0, 0, 0, 138),
    );

    let pin_count = 9;
    let pin_color = Color::new(213, 217, 213, 236);
    let pin_shadow = Color::new(66, 72, 76, 210);
    for index in 0..pin_count {
        let offset = (index + 1) * chip_size / (pin_count + 1);
        let vertical = y + offset;
        let horizontal = x + offset;

        draw.draw_line_ex(
            Vector2::new((x - pin_len + screen_px(1)) as f32, (vertical + 1) as f32),
            Vector2::new((x + screen_px(1)) as f32, (vertical + 1) as f32),
            screen_px(1).max(1) as f32,
            pin_shadow,
        );
        draw.draw_line_ex(
            Vector2::new((x + chip_size - screen_px(1)) as f32, (vertical + 1) as f32),
            Vector2::new(
                (x + chip_size + pin_len - screen_px(1)) as f32,
                (vertical + 1) as f32,
            ),
            screen_px(1).max(1) as f32,
            pin_shadow,
        );
        draw.draw_line_ex(
            Vector2::new((x - pin_len) as f32, vertical as f32),
            Vector2::new(x as f32, vertical as f32),
            screen_px(1).max(1) as f32,
            pin_color,
        );
        draw.draw_line_ex(
            Vector2::new((x + chip_size) as f32, vertical as f32),
            Vector2::new((x + chip_size + pin_len) as f32, vertical as f32),
            screen_px(1).max(1) as f32,
            pin_color,
        );

        draw.draw_line_ex(
            Vector2::new((horizontal + 1) as f32, (y - pin_len + screen_px(1)) as f32),
            Vector2::new((horizontal + 1) as f32, (y + screen_px(1)) as f32),
            screen_px(1).max(1) as f32,
            pin_shadow,
        );
        draw.draw_line_ex(
            Vector2::new(
                (horizontal + 1) as f32,
                (y + chip_size - screen_px(1)) as f32,
            ),
            Vector2::new(
                (horizontal + 1) as f32,
                (y + chip_size + pin_len - screen_px(1)) as f32,
            ),
            screen_px(1).max(1) as f32,
            pin_shadow,
        );
        draw.draw_line_ex(
            Vector2::new(horizontal as f32, (y - pin_len) as f32),
            Vector2::new(horizontal as f32, y as f32),
            screen_px(1).max(1) as f32,
            pin_color,
        );
        draw.draw_line_ex(
            Vector2::new(horizontal as f32, (y + chip_size) as f32),
            Vector2::new(horizontal as f32, (y + chip_size + pin_len) as f32),
            screen_px(1).max(1) as f32,
            pin_color,
        );
    }

    draw.draw_rectangle_gradient_v(
        x,
        y,
        chip_size,
        chip_size,
        Color::new(42, 45, 45, 250),
        Color::new(12, 14, 15, 250),
    );
    draw.draw_rectangle_lines(x, y, chip_size, chip_size, Color::new(185, 190, 188, 190));
    draw.draw_rectangle_lines(
        x + screen_px(3),
        y + screen_px(3),
        chip_size - screen_px(6),
        chip_size - screen_px(6),
        Color::new(255, 255, 255, 24),
    );

    draw.draw_rectangle(
        x + screen_px(5),
        y + screen_px(5),
        chip_size - screen_px(10),
        screen_px(2),
        Color::new(255, 255, 255, 26),
    );
    draw.draw_circle_lines(
        x + screen_px(8),
        y + chip_size - screen_px(8),
        screen_px(3) as f32,
        Color::new(0, 0, 0, 210),
    );
    draw.draw_circle(
        x + screen_px(8),
        y + chip_size - screen_px(8),
        screen_px(1) as f32,
        Color::new(96, 103, 103, 230),
    );
    draw.draw_rectangle(
        x + chip_size - screen_px(8),
        y + screen_px(7),
        screen_px(3),
        screen_px(3),
        Color::new(95, 255, 174, 110),
    );

    let label = "Linker";
    let label_size = 8.5;
    let label_width = menu_text_width(assets.menu_font.as_ref(), label, label_size, 1.0);
    draw_menu_text(
        draw,
        assets.menu_font.as_ref(),
        label,
        center_x - label_width / 2,
        center_y - screen_px(7),
        label_size,
        Color::new(217, 220, 213, 236),
    );

    let sub_label = "LNK-01";
    let sub_label_size = 5.5;
    let sub_label_width =
        menu_text_width(assets.menu_font.as_ref(), sub_label, sub_label_size, 1.0);
    draw_menu_text(
        draw,
        assets.menu_font.as_ref(),
        sub_label,
        center_x - sub_label_width / 2,
        center_y + screen_px(5),
        sub_label_size,
        Color::new(149, 157, 151, 210),
    );
}

/// Data needed by the preferences renderer.
pub struct PreferencesDrawOptions<'a> {
    pub menu: &'a PreferencesMenu,
    pub player_one_character: CharacterId,
    pub player_two_character: CharacterId,
    pub arena: ArenaId,
    pub music_volume_percent: u8,
    pub visual_time_seconds: f32,
    pub flags: FeatureFlags,
    pub gamepad_status: GamepadStatus,
    pub recording: bool,
    pub assets: &'a GameAssets,
}

struct MenuLine<'a> {
    label: &'a str,
    description: &'a str,
    value: Option<&'a str>,
    checked: Option<bool>,
}

struct WrappedText<'a> {
    font: Option<&'a Font>,
    text: &'a str,
    x: i32,
    y: i32,
    max_width: i32,
    font_size: f32,
    line_height: f32,
    color: Color,
}

#[derive(Clone, Copy)]
struct ScrollArea {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

struct ProfileFieldDraw<'a> {
    label_font: Option<&'a Font>,
    body_font: Option<&'a Font>,
    label: &'a str,
    body: &'a str,
    x: i32,
    y: i32,
    content_width: i32,
}

#[derive(Clone, Copy)]
struct MenuTerminal {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    alpha: u8,
    command_offset: usize,
    font_size: f32,
}

const MENU_TERMINAL_COMMANDS: [&str; 36] = [
    "$ git commit -m \"combo-route\"",
    "$ make all",
    "$ cargo test --local",
    "$ rustup update stable",
    "$ go mod init arena/linker",
    "$ go test ./...",
    "$ mvn -q test",
    "$ gradle build",
    "$ npm run build",
    "$ pnpm install --frozen-lockfile",
    "$ yarn dlx patch-package",
    "$ bun test",
    "$ deno task check",
    "$ pip install -r tools.txt",
    "$ poetry lock --no-update",
    "$ composer install",
    "$ dotnet restore",
    "$ nuget locals all -clear",
    "$ cmake --build build/",
    "$ zig build test",
    "$ pkg-config --libs raylib",
    "$ rg hurtbox src/",
    "$ ssh -N linker@localhost",
    "$ gpg --verify release.sig",
    "$ openssl dgst -sha256 atlas.png",
    "$ tcpdump -i lo --snapshot-length 64",
    "$ nmap --top-ports 32 localhost",
    "$ dig arena.local",
    "$ whois borrow.invalid",
    "$ chmod +x ./dash_cancel",
    "$ curl arena://health",
    "TRACE 0101 frame-link ok",
    "BUFFER 0x00F1 guard=true",
    "SCAN hurtbox overlap",
    "ASSERT win_condition",
    "PATCH sprite_manifest",
];

fn draw_menu_backdrop(draw: &mut impl DrawTarget, font: Option<&Font>) {
    draw.draw_rectangle(0, 0, WINDOW_WIDTH, WINDOW_HEIGHT, Color::new(2, 8, 18, 96));

    let vanish_x = WINDOW_WIDTH / 2;
    let horizon_y = screen_px(132);
    for x in (-screen_px(160)..WINDOW_WIDTH + screen_px(160)).step_by(screen_px(80) as usize) {
        draw.draw_line(
            x,
            WINDOW_HEIGHT,
            vanish_x,
            horizon_y,
            Color::new(28, 76, 106, 68),
        );
    }
    for index in 0..11 {
        let y = horizon_y + screen_px(index * index * 4 + index * 10);
        if y >= WINDOW_HEIGHT {
            continue;
        }
        let alpha = (34 + index * 8).min(112) as u8;
        draw.draw_line(0, y, WINDOW_WIDTH, y, Color::new(48, 110, 144, alpha));
    }

    let terminals = [
        MenuTerminal {
            x: screen_px(58),
            y: screen_px(108),
            width: screen_px(168),
            height: screen_px(86),
            alpha: 42,
            command_offset: 0,
            font_size: 8.0,
        },
        MenuTerminal {
            x: screen_px(724),
            y: screen_px(104),
            width: screen_px(178),
            height: screen_px(92),
            alpha: 44,
            command_offset: 5,
            font_size: 8.0,
        },
        MenuTerminal {
            x: screen_px(214),
            y: screen_px(94),
            width: screen_px(184),
            height: screen_px(78),
            alpha: 34,
            command_offset: 10,
            font_size: 7.0,
        },
        MenuTerminal {
            x: screen_px(560),
            y: screen_px(92),
            width: screen_px(182),
            height: screen_px(78),
            alpha: 34,
            command_offset: 15,
            font_size: 7.0,
        },
        MenuTerminal {
            x: screen_px(32),
            y: screen_px(222),
            width: screen_px(238),
            height: screen_px(148),
            alpha: 72,
            command_offset: 18,
            font_size: 9.0,
        },
        MenuTerminal {
            x: screen_px(698),
            y: screen_px(226),
            width: screen_px(232),
            height: screen_px(146),
            alpha: 70,
            command_offset: 23,
            font_size: 9.0,
        },
        MenuTerminal {
            x: screen_px(88),
            y: screen_px(394),
            width: screen_px(248),
            height: screen_px(96),
            alpha: 52,
            command_offset: 28,
            font_size: 8.0,
        },
        MenuTerminal {
            x: screen_px(640),
            y: screen_px(394),
            width: screen_px(248),
            height: screen_px(96),
            alpha: 52,
            command_offset: 32,
            font_size: 8.0,
        },
    ];
    for terminal in terminals {
        draw_terminal_panel(draw, font, terminal);
    }

    for index in 0..18 {
        let x = screen_px(36 + index * 54);
        let y = screen_px(462 + (index % 4) * 13);
        let glyphs = if index % 2 == 0 {
            "01001011"
        } else {
            "10110100"
        };
        draw_menu_text(draw, font, glyphs, x, y, 9.0, Color::new(68, 230, 173, 48));
    }

    draw.draw_rectangle(0, 0, WINDOW_WIDTH, screen_px(132), Color::new(0, 0, 0, 84));
    draw.draw_rectangle(
        0,
        WINDOW_HEIGHT - screen_px(54),
        WINDOW_WIDTH,
        screen_px(54),
        Color::new(0, 0, 0, 122),
    );
}

fn draw_terminal_panel(draw: &mut impl DrawTarget, font: Option<&Font>, terminal: MenuTerminal) {
    draw.draw_rectangle(
        terminal.x + terminal.width / 7,
        terminal.y + terminal.height / 5,
        terminal.width,
        terminal.height,
        Color::new(0, 0, 0, terminal.alpha / 2),
    );
    draw.draw_rectangle(
        terminal.x,
        terminal.y,
        terminal.width,
        terminal.height,
        Color::new(1, 8, 18, terminal.alpha),
    );
    draw.draw_rectangle_lines(
        terminal.x,
        terminal.y,
        terminal.width,
        terminal.height,
        Color::new(0, 202, 255, terminal.alpha.saturating_add(22)),
    );
    draw.draw_line(
        terminal.x + screen_px(8),
        terminal.y + screen_px(16),
        terminal.x + terminal.width - screen_px(8),
        terminal.y + screen_px(16),
        Color::new(95, 255, 174, terminal.alpha.saturating_add(18)),
    );

    let rows = ((terminal.height - screen_px(26)) / screen_px(14)).max(1) as usize;
    for row in 0..rows {
        let command =
            MENU_TERMINAL_COMMANDS[(terminal.command_offset + row) % MENU_TERMINAL_COMMANDS.len()];
        let color = if row % 3 == 0 {
            Color::new(95, 255, 174, terminal.alpha.saturating_add(72))
        } else if row % 3 == 1 {
            Color::new(0, 202, 255, terminal.alpha.saturating_add(48))
        } else {
            Color::new(176, 190, 210, terminal.alpha.saturating_add(28))
        };
        draw_menu_text(
            draw,
            font,
            command,
            terminal.x + screen_px(10),
            terminal.y + screen_px(25) + row as i32 * screen_px(14),
            terminal.font_size,
            color,
        );
    }
}

fn draw_menu_chrome(
    draw: &mut impl DrawTarget,
    font: Option<&Font>,
    options: &PreferencesDrawOptions<'_>,
) {
    if options.menu.page() == MenuPage::Main {
        draw_menu_title_sprite(draw, font, options.assets.menu_title.as_ref());
        draw_centered_menu_text(
            draw,
            font,
            "CÓDIGO NO PUNHO. BRASIL NO CENÁRIO.",
            WINDOW_WIDTH / 2,
            screen_px(128),
            12.0,
            MENU_HACK_GREEN,
        );
        // Slow traveling lights sit outside the interactive rows.
        let travel = (options.visual_time_seconds * 0.18).fract();
        let light_y = screen_px(176) + (travel * world_px(304.0)) as i32;
        for x in [screen_px(284), screen_px(674)] {
            draw.draw_rectangle_gradient_v(
                x,
                light_y,
                screen_px(2),
                screen_px(38),
                Color::new(0, 202, 255, 0),
                Color::new(0, 202, 255, 145),
            );
        }
    } else {
        draw_menu_text(
            draw,
            font,
            "BORROW FIGHTERS",
            screen_px(28),
            screen_px(12),
            16.0,
            UI_TEXT,
        );
    }

    let status = format!(
        "GAMEPADS   P1 {}   /   P2 {}",
        connected_label(options.gamepad_status.player_one),
        connected_label(options.gamepad_status.player_two)
    );
    let width = menu_text_width(font, &status, 11.0, 1.0);
    draw_menu_text(
        draw,
        font,
        &status,
        WINDOW_WIDTH - width - screen_px(28),
        screen_px(14),
        11.0,
        UI_MUTED,
    );
}

fn draw_menu_title_sprite(
    draw: &mut impl DrawTarget,
    font: Option<&Font>,
    title: Option<&Texture2D>,
) {
    if let Some(title) = title {
        let target_width = world_px(646.0);
        let target_height = target_width * title.height() as f32 / title.width() as f32;
        let dest = Rectangle::new(
            (WINDOW_WIDTH as f32 - target_width) / 2.0,
            world_px(-6.0),
            target_width,
            target_height,
        );
        let source = Rectangle::new(0.0, 0.0, title.width() as f32, title.height() as f32);
        draw.draw_rectangle(
            dest.x.round() as i32 + screen_px(56),
            screen_px(54),
            (dest.width - world_px(112.0)).round() as i32,
            screen_px(36),
            Color::new(0, 0, 0, 66),
        );
        draw.draw_texture_pro(
            title,
            source,
            dest,
            Vector2::new(0.0, 0.0),
            0.0,
            Color::WHITE,
        );
        return;
    }

    let title_text = "Borrow Fighters";
    draw_centered_menu_text(
        draw,
        font,
        title_text,
        WINDOW_WIDTH / 2 - 3,
        screen_px(34),
        58.0,
        MENU_MAGENTA,
    );
    draw_centered_menu_text(
        draw,
        font,
        title_text,
        WINDOW_WIDTH / 2 + 5,
        screen_px(41),
        58.0,
        Color::new(0, 202, 255, 118),
    );
    draw_centered_menu_text(
        draw,
        font,
        title_text,
        WINDOW_WIDTH / 2,
        screen_px(30),
        58.0,
        UI_TEXT,
    );
}

fn draw_main_menu(
    draw: &mut impl DrawTarget,
    font: Option<&Font>,
    options: &PreferencesDrawOptions<'_>,
) {
    let geometry = MenuLayout::for_page(MenuPage::Main);
    let panel = geometry.panel;
    draw_menu_panel(draw, panel);
    draw_menu_page_title(draw, font, panel, "ENTRE NA LUTA");

    let rows = [
        MenuLine {
            label: "QUICK FIGHT",
            description: "Seu próximo round começa aqui",
            value: None,
            checked: None,
        },
        MenuLine {
            label: "VERSUS SETUP",
            description: "Escolha lutadores e cenário",
            value: None,
            checked: None,
        },
        MenuLine {
            label: "TRAINING",
            description: "Domine golpes e especiais",
            value: None,
            checked: None,
        },
        MenuLine {
            label: "LORE / ROSTER",
            description: "Conheça quem está no ringue",
            value: None,
            checked: None,
        },
        MenuLine {
            label: "OPTIONS",
            description: "Áudio, controles e preferências",
            value: None,
            checked: None,
        },
        MenuLine {
            label: "EXIT",
            description: "Até o próximo round",
            value: None,
            checked: None,
        },
    ];

    draw_menu_rows(
        draw,
        font,
        &rows,
        options.menu.selected(),
        MenuRowsLayout {
            geometry,
            large_labels: true,
            show_descriptions: true,
            selection_pulse_frames: options.menu.selection_pulse_frames(),
        },
    );
    draw_menu_footer(
        draw,
        font,
        panel,
        "Mouse/Setas navegam  |  Clique/Enter confirma",
    );
}

fn draw_versus_menu(
    draw: &mut impl DrawTarget,
    font: Option<&Font>,
    options: &PreferencesDrawOptions<'_>,
) {
    let geometry = MenuLayout::for_page(MenuPage::Versus);
    let panel = geometry.panel;
    draw_menu_panel(draw, panel);
    draw_menu_page_title(draw, font, panel, "VERSUS SETUP");

    let player_one = character_select_label(options.player_one_character);
    let player_two = character_select_label(options.player_two_character);
    let arena_value = arena_select_label(options.arena);
    let arena_description = arena_select_description(options.arena);
    let rows = [
        MenuLine {
            label: "START FIGHT",
            description: "Comeca a luta com estes personagens.",
            value: None,
            checked: None,
        },
        MenuLine {
            label: "PLAYER 1",
            description: "Esquerda/direita troca o personagem.",
            value: Some(player_one),
            checked: None,
        },
        MenuLine {
            label: "PLAYER 2",
            description: "Esquerda/direita troca o personagem.",
            value: Some(player_two),
            checked: None,
        },
        MenuLine {
            label: "ARENA",
            description: &arena_description,
            value: Some(&arena_value),
            checked: None,
        },
        MenuLine {
            label: "BACK",
            description: "Volta ao menu principal.",
            value: None,
            checked: None,
        },
    ];

    draw_menu_rows(
        draw,
        font,
        &rows,
        options.menu.selected(),
        MenuRowsLayout {
            geometry,
            large_labels: false,
            show_descriptions: true,
            selection_pulse_frames: options.menu.selection_pulse_frames(),
        },
    );
    draw_menu_footer(
        draw,
        font,
        panel,
        "Clique/A/D ajusta  |  Botao direito: anterior  |  Esc volta",
    );
}

fn character_select_label(character: CharacterId) -> &'static str {
    match character {
        CharacterId::Rust => "rust.rs",
        CharacterId::Duke => "duke.java",
        CharacterId::Go => "gopher.go",
        CharacterId::C => "old.c",
        CharacterId::Python => "python.py",
        CharacterId::Cpp => "cpp.cpp",
    }
}

fn arena_select_label(arena: ArenaId) -> String {
    arena.label().to_owned()
}

fn arena_select_description(arena: ArenaId) -> String {
    format!("{} - {}", arena.location(), arena.concept())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lethal_landing_keeps_loser_on_floor_instead_of_restarting_standing_defeat() {
        let mut world = World::new_greybox();
        world.player_one.health = 0;
        world.player_one.start_knockdown();
        world.outcome = Some(MatchOutcome::Winner(PlayerSlot::Two));
        let (clip, time) = fighter_match_presentation(&world, &world.player_one, false);
        assert_eq!(clip, Some(sprites::FighterSpriteClip::Knockdown));
        let manifest =
            sprites::SpriteManifest::load("assets/candidates/rust/rust-fighter.sprite.json")
                .unwrap();
        let frame = sprites::frame_for_fighter_clip_at(&manifest, clip.unwrap(), time).unwrap();
        assert_eq!(frame.name, "knockdown_01");
        world.elapsed_seconds += 30.0;
        assert_eq!(
            fighter_match_presentation(&world, &world.player_one, false),
            (clip, time)
        );
        assert_eq!(
            fighter_match_presentation(&world, &world.player_two, false).0,
            Some(sprites::FighterSpriteClip::Victory)
        );
    }

    #[test]
    fn character_select_labels_use_source_file_theme() {
        assert_eq!(character_select_label(CharacterId::C), "old.c");
        assert_eq!(character_select_label(CharacterId::Rust), "rust.rs");
        assert_eq!(character_select_label(CharacterId::Duke), "duke.java");
        assert_eq!(character_select_label(CharacterId::Go), "gopher.go");
        assert_eq!(character_select_label(CharacterId::Python), "python.py");
        assert_eq!(character_select_label(CharacterId::Cpp), "cpp.cpp");
    }

    #[test]
    fn main_atlas_spawn_is_preferred_and_legacy_start_is_a_fallback() {
        let manifest = sprites::SpriteManifest::load(sprites::RUST_FIGHTER_MANIFEST_PATH).unwrap();
        let mut fight = SpriteAtlasAsset {
            combat_manifest: manifest.clone(),
            manifest,
            textures: Vec::new(),
        };
        let start_manifest =
            sprites::SpriteManifest::load(sprites::RUST_START_MANIFEST_PATH).unwrap();
        let start = SpriteAtlasAsset {
            combat_manifest: start_manifest.clone(),
            manifest: start_manifest,
            textures: Vec::new(),
        };
        assert!(std::ptr::eq(
            fighter_atlas_for_intro(true, Some(&start), Some(&fight)).unwrap(),
            &start
        ));
        let mut spawn = fight.manifest.clip_named("idle").unwrap().clone();
        spawn.name = "spawn".to_string();
        fight.manifest.clips.push(spawn);
        assert!(std::ptr::eq(
            fighter_atlas_for_intro(true, Some(&start), Some(&fight)).unwrap(),
            &fight
        ));
        assert!(std::ptr::eq(
            fighter_atlas_for_intro(true, None, Some(&fight)).unwrap(),
            &fight
        ));
        assert_eq!(
            sprites::match_fighter_sprite_clip(None, PlayerSlot::One, true),
            Some(sprites::FighterSpriteClip::Spawn)
        );
        assert!(std::ptr::eq(
            fighter_atlas_for_intro(false, Some(&start), Some(&fight)).unwrap(),
            &fight
        ));
    }

    #[test]
    fn outcome_render_time_does_not_skip_to_the_end_after_a_long_match() {
        let mut world = World::new_greybox();
        world.elapsed_seconds = 60.0;
        world.player_two.health = 0;
        let input = crate::combat::fighter::FighterInput::default();
        world.update(1.0 / 60.0, input, input);
        assert_eq!(fighter_visual_elapsed_seconds(&world), 0.0);
        world.update(1.0 / 60.0, input, input);
        assert_eq!(fighter_visual_elapsed_seconds(&world), 1.0 / 60.0);
    }
}

fn draw_training_menu(
    draw: &mut impl DrawTarget,
    font: Option<&Font>,
    options: &PreferencesDrawOptions<'_>,
) {
    let geometry = MenuLayout::for_page(MenuPage::Training);
    let panel = geometry.panel;
    draw_menu_panel(draw, panel);
    draw_menu_page_title(draw, font, panel, "TRAINING");

    let rows = [
        MenuLine {
            label: "COMBAT LAB",
            description: "Teste golpes, frames, hitboxes e hurtboxes.",
            value: None,
            checked: None,
        },
        MenuLine {
            label: "MOVE SHOWCASE",
            description: "Veja golpes e defesa em combate real.",
            value: None,
            checked: None,
        },
        MenuLine {
            label: "SPRITE VIEWER",
            description: "Inspecione atlas, pivot, boxes e projetil.",
            value: None,
            checked: None,
        },
        MenuLine {
            label: "BACK",
            description: "Volta ao menu principal.",
            value: None,
            checked: None,
        },
    ];

    draw_menu_rows(
        draw,
        font,
        &rows,
        options.menu.selected(),
        MenuRowsLayout {
            geometry,
            large_labels: false,
            show_descriptions: true,
            selection_pulse_frames: options.menu.selection_pulse_frames(),
        },
    );
    draw_menu_footer(draw, font, panel, "Esc volta das ferramentas para o menu");
}

fn draw_lore_menu(
    draw: &mut impl DrawTarget,
    font: Option<&Font>,
    options: &PreferencesDrawOptions<'_>,
) {
    let geometry = MenuLayout::for_page(MenuPage::Lore);
    let panel = geometry.panel;
    let book = &options.assets.lore_book;
    let lore_font = options.assets.lore_font.as_ref().or(font);
    let lore_body_font = options
        .assets
        .lore_body_font
        .as_ref()
        .or(options.assets.lore_font.as_ref())
        .or(font);
    let selected_chapter = book.chapter(options.menu.lore_chapter());
    let selected_character = book.character(options.menu.lore_character());

    draw.draw_rectangle(
        panel.x + screen_px(12),
        panel.y + screen_px(14),
        panel.width,
        panel.height,
        Color::new(0, 0, 0, 110),
    );
    draw.draw_rectangle(
        panel.x,
        panel.y,
        panel.width,
        panel.height,
        Color::new(10, 17, 28, 244),
    );
    draw.draw_rectangle_lines(panel.x, panel.y, panel.width, panel.height, MENU_ACCENT);
    draw.draw_line(
        panel.x + panel.width / 2,
        panel.y + screen_px(62),
        panel.x + panel.width / 2,
        panel.y + panel.height - screen_px(42),
        Color::new(138, 161, 184, 112),
    );
    for index in 0..8 {
        let y = panel.y + screen_px(96 + index * 34);
        draw.draw_line(
            panel.x + screen_px(34),
            y,
            panel.x + panel.width / 2 - screen_px(24),
            y,
            Color::new(95, 255, 174, 12),
        );
    }

    draw_centered_menu_text(
        draw,
        lore_font,
        &book.title,
        panel.x + panel.width / 2,
        panel.y + screen_px(16),
        24.0,
        UI_TEXT,
    );
    draw_centered_menu_text(
        draw,
        font,
        &book.subtitle,
        panel.x + panel.width / 2,
        panel.y + screen_px(48),
        10.0,
        UI_MUTED,
    );

    let chapter_label = lore_chapter_label(book, options.menu.lore_chapter());
    let character_label = lore_character_label(book, options.menu.lore_character());
    let controls = [
        MenuLine {
            label: "CHAPTER",
            description: "A/D troca o capitulo do livro.",
            value: Some(&chapter_label),
            checked: None,
        },
        MenuLine {
            label: "CHARACTER",
            description: "A/D troca a ficha do roster.",
            value: Some(&character_label),
            checked: None,
        },
        MenuLine {
            label: "BACK",
            description: "Volta ao menu principal.",
            value: None,
            checked: None,
        },
    ];
    draw_menu_rows(
        draw,
        font,
        &controls,
        options.menu.selected(),
        MenuRowsLayout {
            geometry,
            large_labels: false,
            show_descriptions: true,
            selection_pulse_frames: options.menu.selection_pulse_frames(),
        },
    );

    let story_area = ScrollArea {
        x: panel.x + screen_px(38),
        y: panel.y + screen_px(232),
        width: screen_px(376),
        height: panel.height - screen_px(298),
    };
    if let Some(chapter) = selected_chapter {
        let content_width = story_area.width - screen_px(18);
        let content_height = lore_chapter_content_height(lore_body_font, chapter, content_width);
        let offset = scroll_offset_px(
            options.menu.lore_chapter_scroll(),
            16,
            content_height,
            story_area.height,
        );
        {
            let mut clipped = draw.begin_scissor_mode(
                story_area.x,
                story_area.y,
                story_area.width,
                story_area.height,
            );
            draw_lore_chapter_content(
                &mut clipped,
                lore_font,
                lore_body_font,
                chapter,
                story_area.x,
                story_area.y - offset,
                content_width,
            );
        }
        draw_scrollbar(draw, story_area, content_height, offset);
    } else {
        draw_menu_text(
            draw,
            lore_font,
            "Lore file sem capitulos carregados.",
            story_area.x,
            story_area.y,
            15.0,
            UI_MUTED,
        );
    }

    let roster_x = panel.x + panel.width / 2 + screen_px(34);
    let roster_y = panel.y + screen_px(90);
    if let Some(character) = selected_character {
        draw_character_portrait(draw, character, roster_x, roster_y, options.assets);
        draw_menu_text(
            draw,
            font,
            &character.file_name,
            roster_x + screen_px(174),
            roster_y + screen_px(6),
            16.0,
            MENU_HACK_GREEN,
        );
        draw_menu_text(
            draw,
            lore_font,
            &character.name,
            roster_x + screen_px(174),
            roster_y + screen_px(30),
            24.0,
            UI_TEXT,
        );
        draw_wrapped_menu_text(
            draw,
            WrappedText {
                font: lore_font,
                text: &character.epithet,
                x: roster_x + screen_px(174),
                y: roster_y + screen_px(68),
                max_width: screen_px(214),
                font_size: 12.0,
                line_height: 16.0,
                color: MENU_ACCENT_ALT,
            },
        );

        let profile_area = ScrollArea {
            x: roster_x,
            y: roster_y + screen_px(156),
            width: screen_px(404),
            height: panel.y + panel.height - screen_px(60) - (roster_y + screen_px(156)),
        };
        let profile_width = profile_area.width - screen_px(18);
        let content_height = lore_profile_content_height(lore_body_font, character, profile_width);
        let offset = scroll_offset_px(
            options.menu.lore_character_scroll(),
            14,
            content_height,
            profile_area.height,
        );
        {
            let mut clipped = draw.begin_scissor_mode(
                profile_area.x,
                profile_area.y,
                profile_area.width,
                profile_area.height,
            );
            draw_lore_profile_content(
                &mut clipped,
                font,
                lore_body_font,
                character,
                profile_area.x,
                profile_area.y - offset,
                profile_width,
            );
        }
        draw_scrollbar(draw, profile_area, content_height, offset);
    } else {
        draw_menu_text(
            draw,
            lore_font,
            "Lore file sem personagens carregados.",
            roster_x,
            roster_y,
            15.0,
            UI_MUTED,
        );
    }

    draw_menu_footer(
        draw,
        font,
        panel,
        "Selecione CHAPTER/CHARACTER  |  roda/PageUp/PageDown rola texto  |  Esc volta",
    );
}

fn draw_options_menu(
    draw: &mut impl DrawTarget,
    font: Option<&Font>,
    options: &PreferencesDrawOptions<'_>,
) {
    let geometry = MenuLayout::for_page(MenuPage::Options);
    let panel = geometry.panel;
    draw_menu_panel(draw, panel);
    draw_menu_page_title(draw, font, panel, "OPTIONS");

    let first_row = geometry.row_bounds(0);
    let row_x = first_row.x;
    let row_width = first_row.width;
    let row_height = first_row.height;
    let row_y = first_row.y;

    draw_option_row(
        draw,
        font,
        OptionRow {
            x: row_x,
            y: row_y,
            width: row_width,
            height: row_height,
            selected: options.menu.selected() == PreferencesMenu::OPTIONS_RECORDING_ROW,
            label: "LOCAL RECORDING",
            value: if options.recording { "REC" } else { "OFF" },
            checked: Some(options.recording),
        },
    );

    let music_volume = format!("{}%", options.music_volume_percent);
    draw_option_row(
        draw,
        font,
        OptionRow {
            x: row_x,
            y: geometry
                .row_bounds(PreferencesMenu::OPTIONS_MUSIC_VOLUME_ROW)
                .y,
            width: row_width,
            height: row_height,
            selected: options.menu.selected() == PreferencesMenu::OPTIONS_MUSIC_VOLUME_ROW,
            label: "MUSIC VOLUME",
            value: &music_volume,
            checked: None,
        },
    );

    for (index, flag) in PREFERENCE_FLAGS.iter().copied().enumerate() {
        let row = index + PreferencesMenu::OPTIONS_FIRST_FLAG_ROW;
        draw_option_row(
            draw,
            font,
            OptionRow {
                x: row_x,
                y: geometry.row_bounds(row).y,
                width: row_width,
                height: row_height,
                selected: options.menu.selected() == row,
                label: flag.label(),
                value: if options.flags.enabled(flag) {
                    "ON"
                } else {
                    "OFF"
                },
                checked: Some(options.flags.enabled(flag)),
            },
        );
    }

    let back_row = options.menu.row_count() - 1;
    draw_option_row(
        draw,
        font,
        OptionRow {
            x: row_x,
            y: geometry.row_bounds(back_row).y,
            width: row_width,
            height: row_height,
            selected: options.menu.selected() == back_row,
            label: "BACK",
            value: "",
            checked: None,
        },
    );

    let hint = selected_options_hint(options);
    draw_menu_text(
        draw,
        font,
        hint,
        panel.x + screen_px(48),
        panel.y + panel.height - screen_px(64),
        13.0,
        UI_MUTED,
    );
    draw_menu_footer(
        draw,
        font,
        panel,
        "Clique/Enter alterna  |  Botao direito: anterior  |  Esc volta",
    );
}

fn draw_menu_panel(draw: &mut impl DrawTarget, panel: MenuPanel) {
    draw.draw_rectangle(
        panel.x - screen_px(14),
        panel.y - screen_px(14),
        panel.width + screen_px(28),
        panel.height + screen_px(28),
        Color::new(0, 0, 0, 118),
    );
    draw.draw_rectangle(
        panel.x,
        panel.y,
        panel.width,
        panel.height,
        MENU_PANEL_STRONG,
    );
    for y in (panel.y + screen_px(10)..panel.y + panel.height - screen_px(10))
        .step_by(screen_px(18) as usize)
    {
        draw.draw_line(
            panel.x + screen_px(10),
            y,
            panel.x + panel.width - screen_px(10),
            y,
            Color::new(78, 130, 164, 6),
        );
    }
    draw.draw_rectangle_lines(
        panel.x,
        panel.y,
        panel.width,
        panel.height,
        Color::new(87, 193, 255, 150),
    );
    draw.draw_rectangle_lines(
        panel.x + screen_px(5),
        panel.y + screen_px(5),
        panel.width - screen_px(10),
        panel.height - screen_px(10),
        Color::new(255, 191, 67, 44),
    );
    draw.draw_line(
        panel.x + screen_px(22),
        panel.y + screen_px(18),
        panel.x + panel.width - screen_px(22),
        panel.y + screen_px(18),
        Color::new(87, 193, 255, 128),
    );
    draw.draw_line(
        panel.x + screen_px(22),
        panel.y + panel.height - screen_px(44),
        panel.x + panel.width - screen_px(22),
        panel.y + panel.height - screen_px(44),
        Color::new(87, 193, 255, 80),
    );
    draw.draw_rectangle(
        panel.x - screen_px(4),
        panel.y - screen_px(4),
        screen_px(34),
        screen_px(4),
        MENU_ACCENT,
    );
    draw.draw_rectangle(
        panel.x - screen_px(4),
        panel.y - screen_px(4),
        screen_px(4),
        screen_px(34),
        MENU_ACCENT,
    );
    draw.draw_rectangle(
        panel.x + panel.width - screen_px(30),
        panel.y - screen_px(4),
        screen_px(34),
        screen_px(4),
        MENU_ACCENT_ALT,
    );
    draw.draw_rectangle(
        panel.x + panel.width,
        panel.y - screen_px(4),
        screen_px(4),
        screen_px(34),
        MENU_ACCENT_ALT,
    );
    draw.draw_rectangle(
        panel.x - screen_px(4),
        panel.y + panel.height,
        screen_px(34),
        screen_px(4),
        MENU_ACCENT_ALT,
    );
    draw.draw_rectangle(
        panel.x - screen_px(4),
        panel.y + panel.height - screen_px(30),
        screen_px(4),
        screen_px(34),
        MENU_ACCENT_ALT,
    );
    draw.draw_rectangle(
        panel.x + panel.width - screen_px(30),
        panel.y + panel.height,
        screen_px(34),
        screen_px(4),
        MENU_ACCENT,
    );
    draw.draw_rectangle(
        panel.x + panel.width,
        panel.y + panel.height - screen_px(30),
        screen_px(4),
        screen_px(34),
        MENU_ACCENT,
    );
}

fn draw_menu_page_title(
    draw: &mut impl DrawTarget,
    font: Option<&Font>,
    panel: MenuPanel,
    title: &str,
) {
    draw_centered_menu_text(
        draw,
        font,
        title,
        panel.x + panel.width / 2,
        panel.y + screen_px(26),
        21.0,
        UI_TEXT,
    );
}

fn draw_menu_rows(
    draw: &mut impl DrawTarget,
    font: Option<&Font>,
    rows: &[MenuLine<'_>],
    selected: usize,
    layout: MenuRowsLayout,
) {
    let compact_rows = layout.geometry.row_step <= screen_px(42);
    for (index, row) in rows.iter().enumerate() {
        let bounds = layout.geometry.row_bounds(index);
        draw_large_menu_row(
            draw,
            font,
            LargeMenuRow {
                x: bounds.x,
                y: bounds.y,
                width: bounds.width,
                height: bounds.height,
                selected: selected == index,
                label: row.label,
                description: row.description,
                value: row.value,
                checked: row.checked,
                large_label: layout.large_labels,
                show_description: layout.show_descriptions,
                animation_frames: if selected == index && !compact_rows {
                    layout.selection_pulse_frames
                } else {
                    0
                },
                visual_seed: index as u32,
            },
        );
    }
}

#[derive(Clone, Copy)]
struct MenuRowsLayout {
    geometry: MenuLayout,
    large_labels: bool,
    show_descriptions: bool,
    selection_pulse_frames: u16,
}

struct LargeMenuRow<'a> {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    selected: bool,
    label: &'a str,
    description: &'a str,
    value: Option<&'a str>,
    checked: Option<bool>,
    large_label: bool,
    show_description: bool,
    animation_frames: u16,
    visual_seed: u32,
}

fn draw_large_menu_row(draw: &mut impl DrawTarget, font: Option<&Font>, row: LargeMenuRow<'_>) {
    let compact_row = row.height <= screen_px(36);
    let fill = if row.selected {
        MENU_ROW_SELECTED
    } else {
        MENU_ROW
    };
    let border = if row.selected {
        MENU_ACCENT
    } else {
        Color::new(92, 113, 144, 160)
    };
    draw.draw_rectangle(
        row.x + screen_px(6),
        row.y + screen_px(7),
        row.width,
        row.height,
        Color::new(0, 0, 0, 86),
    );
    draw.draw_rectangle(row.x, row.y, row.width, row.height, fill);
    draw.draw_rectangle_lines(row.x, row.y, row.width, row.height, border);
    draw.draw_line(
        row.x + screen_px(12),
        row.y + row.height - screen_px(7),
        row.x + row.width - screen_px(12),
        row.y + row.height - screen_px(7),
        Color::new(255, 255, 255, 24),
    );

    if row.selected {
        draw_selected_menu_cursor(draw, row.x, row.y, row.height, row.animation_frames);
        draw_selected_row_xray(draw, font, &row);
    }

    let label_x = if let Some(checked) = row.checked {
        draw_checkbox(
            draw,
            row.x + screen_px(18),
            row.y + row.height / 2 - screen_px(9),
            checked,
        );
        row.x + screen_px(50)
    } else {
        row.x + screen_px(50)
    };

    let label_size = if row.large_label {
        22.0
    } else if compact_row {
        16.0
    } else {
        20.0
    };
    let label_y = if row.show_description {
        row.y + screen_px(if compact_row { 5 } else { 3 })
    } else {
        row.y + row.height / 2 - screen_px(15)
    };
    let animated_label;
    let label = if row.animation_frames > 0 {
        animated_label = binary_reveal_text_with_seed(
            row.label,
            row.animation_frames,
            DEFAULT_BINARY_REVEAL_FRAMES,
            row.visual_seed,
        );
        animated_label.as_str()
    } else {
        row.label
    };
    let label_color = if row.animation_frames > 0 {
        MENU_HACK_GREEN
    } else {
        UI_TEXT
    };
    draw_menu_text(draw, font, label, label_x, label_y, label_size, label_color);

    if row.show_description && !row.description.is_empty() {
        draw_menu_text(
            draw,
            font,
            row.description,
            label_x,
            row.y + row.height - screen_px(if compact_row { 13 } else { 17 }),
            if compact_row { 10.0 } else { 11.0 },
            UI_MUTED,
        );
    }

    if let Some(value) = row.value {
        let text = format!("< {value} >");
        let label_width = menu_text_width(font, label, label_size, 1.0);
        let min_x = label_x + label_width + screen_px(12);
        let right_x = row.x + row.width - screen_px(12);
        let mut value_size = if compact_row { 10.0 } else { 18.0 };
        let available_width = (right_x - min_x).max(screen_px(38));
        while value_size > 8.0 && menu_text_width(font, &text, value_size, 1.0) > available_width {
            value_size -= 0.5;
        }
        let text_width = menu_text_width(font, &text, value_size, 1.0);
        {
            let mut clipped = draw.begin_scissor_mode(min_x, row.y, available_width, row.height);
            draw_menu_text(
                &mut clipped,
                font,
                &text,
                right_x - text_width,
                row.y + screen_px(if compact_row { 11 } else { 9 }),
                value_size,
                HEALTH_FILL,
            );
        }
    }
}

fn draw_selected_menu_cursor(
    draw: &mut impl DrawTarget,
    row_x: i32,
    row_y: i32,
    row_height: i32,
    animation_frames: u16,
) {
    let center_y = row_y + row_height / 2;
    let flash = if animation_frames > DEFAULT_BINARY_REVEAL_FRAMES / 2 {
        MENU_HACK_GREEN
    } else {
        MENU_CURSOR
    };

    draw.draw_rectangle(
        row_x - screen_px(24),
        center_y - screen_px(4),
        screen_px(22),
        screen_px(8),
        flash,
    );
    draw.draw_line(
        row_x - screen_px(2),
        center_y - screen_px(14),
        row_x + screen_px(12),
        center_y,
        flash,
    );
    draw.draw_line(
        row_x + screen_px(12),
        center_y,
        row_x - screen_px(2),
        center_y + screen_px(14),
        flash,
    );
    draw.draw_line(
        row_x - screen_px(18),
        center_y - screen_px(18),
        row_x - screen_px(4),
        center_y - screen_px(4),
        MENU_ACCENT,
    );
    draw.draw_line(
        row_x - screen_px(18),
        center_y + screen_px(18),
        row_x - screen_px(4),
        center_y + screen_px(4),
        MENU_ACCENT,
    );
    draw.draw_rectangle(row_x, row_y, screen_px(5), row_height, MENU_CURSOR);
}

fn draw_selected_row_xray(draw: &mut impl DrawTarget, font: Option<&Font>, row: &LargeMenuRow<'_>) {
    draw.draw_rectangle(
        row.x + screen_px(6),
        row.y + screen_px(5),
        row.width - screen_px(12),
        row.height - screen_px(10),
        Color::new(0, 202, 255, 22),
    );
    for offset in (screen_px(8)..row.width - screen_px(18)).step_by(screen_px(34) as usize) {
        draw.draw_line(
            row.x + offset,
            row.y + screen_px(6),
            row.x + offset + screen_px(16),
            row.y + row.height - screen_px(8),
            Color::new(95, 255, 174, 12),
        );
    }

    if row.animation_frames == 0 {
        return;
    }

    let elapsed = DEFAULT_BINARY_REVEAL_FRAMES.saturating_sub(row.animation_frames) as i32;
    let scan_width = row.width - screen_px(28);
    let scan_x = row.x + screen_px(14) + scan_width * elapsed / DEFAULT_BINARY_REVEAL_FRAMES as i32;
    draw.draw_rectangle(
        scan_x - screen_px(7),
        row.y + screen_px(3),
        screen_px(14),
        row.height - screen_px(6),
        Color::new(255, 255, 255, 72),
    );
    draw.draw_line(
        scan_x,
        row.y + screen_px(4),
        scan_x,
        row.y + row.height - screen_px(5),
        MENU_CURSOR,
    );

    if row.width < screen_px(320) {
        return;
    }

    let code = if row.visual_seed.is_multiple_of(2) {
        "0101 1100 0110"
    } else {
        "1010 0011 1001"
    };
    let code_width = menu_text_width(font, code, 10.0, 1.0);
    draw_menu_text(
        draw,
        font,
        code,
        row.x + row.width - code_width - screen_px(14),
        row.y + row.height - screen_px(16),
        10.0,
        Color::new(95, 255, 174, 132),
    );
}

struct OptionRow<'a> {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    selected: bool,
    label: &'a str,
    value: &'a str,
    checked: Option<bool>,
}

fn draw_option_row(draw: &mut impl DrawTarget, font: Option<&Font>, row: OptionRow<'_>) {
    let fill = if row.selected {
        SELECTED_ROW
    } else {
        Color::new(9, 15, 28, 190)
    };
    draw.draw_rectangle(row.x, row.y, row.width, row.height, fill);
    if row.selected {
        draw.draw_rectangle_lines(row.x, row.y, row.width, row.height, MENU_ACCENT);
    }

    let label_x = if let Some(checked) = row.checked {
        draw_checkbox(draw, row.x + screen_px(12), row.y + screen_px(5), checked);
        row.x + screen_px(42)
    } else {
        row.x + screen_px(42)
    };

    draw_menu_text(
        draw,
        font,
        row.label,
        label_x,
        row.y + screen_px(5),
        15.0,
        UI_TEXT,
    );

    if !row.value.is_empty() {
        let color = if row.value == "ON" || row.value == "REC" {
            HEALTH_FILL
        } else {
            UI_MUTED
        };
        let width = menu_text_width(font, row.value, 15.0, 1.0);
        draw_menu_text(
            draw,
            font,
            row.value,
            row.x + row.width - width - screen_px(14),
            row.y + screen_px(5),
            15.0,
            color,
        );
    }
}

fn draw_checkbox(draw: &mut impl DrawTarget, x: i32, y: i32, enabled: bool) {
    let size = screen_px(18);
    draw.draw_rectangle_lines(x, y, size, size, UI_TEXT);
    if enabled {
        draw.draw_rectangle(
            x + screen_px(4),
            y + screen_px(4),
            size - screen_px(8),
            size - screen_px(8),
            HEALTH_FILL,
        );
    }
}

fn draw_menu_footer(draw: &mut impl DrawTarget, font: Option<&Font>, panel: MenuPanel, text: &str) {
    draw_centered_menu_text(
        draw,
        font,
        text,
        panel.x + panel.width / 2,
        panel.y + panel.height - screen_px(30),
        12.0,
        UI_MUTED,
    );
}

fn selected_options_hint(options: &PreferencesDrawOptions<'_>) -> &'static str {
    if options.menu.selected() == PreferencesMenu::OPTIONS_RECORDING_ROW {
        return "Grave seus melhores rounds. F9 inicia; F10 encerra e salva.";
    }
    if options.menu.selected() == PreferencesMenu::OPTIONS_MUSIC_VOLUME_ROW {
        return "A/D ou setas esquerda/direita ajustam o volume da música.";
    }
    if options.menu.selected() == options.menu.row_count() - 1 {
        return "Volta para o menu principal.";
    }

    PREFERENCE_FLAGS[options.menu.selected() - PreferencesMenu::OPTIONS_FIRST_FLAG_ROW]
        .description()
}

fn lore_chapter_label(book: &LoreBook, selected: usize) -> String {
    book.chapter(selected)
        .map(|chapter| chapter.code.clone())
        .unwrap_or_else(|| "sem capitulos".to_owned())
}

fn lore_character_label(book: &LoreBook, selected: usize) -> String {
    book.character(selected)
        .map(|character| character.file_name.clone())
        .unwrap_or_else(|| "sem roster".to_owned())
}

fn draw_character_portrait(
    draw: &mut impl DrawTarget,
    character: &LoreCharacter,
    x: i32,
    y: i32,
    assets: &GameAssets,
) {
    let size = screen_px(148);
    draw.draw_rectangle(x, y, size, size, Color::new(4, 9, 16, 228));
    draw.draw_rectangle_lines(x, y, size, size, Color::new(138, 161, 184, 188));

    let texture = character
        .character_id()
        .and_then(|id| assets.roster_portraits.get(id));
    let Some(texture) = texture else {
        draw_centered_menu_text(
            draw,
            assets.menu_font.as_ref(),
            &character.file_name,
            x + size / 2,
            y + size / 2 - screen_px(8),
            16.0,
            UI_MUTED,
        );
        return;
    };

    let padding = screen_px(10);
    let source = Rectangle::new(0.0, 0.0, texture.width() as f32, texture.height() as f32);
    let dest = Rectangle::new(
        (x + padding) as f32,
        (y + padding) as f32,
        (size - padding * 2) as f32,
        (size - padding * 2) as f32,
    );
    draw.draw_texture_pro(
        texture,
        source,
        dest,
        Vector2::new(0.0, 0.0),
        0.0,
        Color::WHITE,
    );
}

fn lore_chapter_content_height(
    font: Option<&Font>,
    chapter: &LoreChapter,
    content_width: i32,
) -> i32 {
    let mut height = screen_px(54);
    for paragraph in &chapter.body {
        height +=
            wrapped_menu_text_height(font, paragraph, content_width, 11.5, 16.0) + screen_px(10);
    }
    height
}

fn draw_lore_chapter_content(
    draw: &mut impl DrawTarget,
    title_font: Option<&Font>,
    body_font: Option<&Font>,
    chapter: &LoreChapter,
    x: i32,
    y: i32,
    content_width: i32,
) -> i32 {
    draw_menu_text(draw, title_font, &chapter.code, x, y, 10.0, MENU_HACK_GREEN);
    draw_menu_text(
        draw,
        title_font,
        &chapter.title,
        x,
        y + screen_px(21),
        16.0,
        UI_TEXT,
    );

    let mut next_y = y + screen_px(54);
    for paragraph in &chapter.body {
        next_y = draw_wrapped_menu_text(
            draw,
            WrappedText {
                font: body_font,
                text: paragraph,
                x,
                y: next_y,
                max_width: content_width,
                font_size: 11.5,
                line_height: 16.0,
                color: Color::new(226, 233, 242, 255),
            },
        ) + screen_px(10);
    }
    next_y
}

fn lore_profile_content_height(
    body_font: Option<&Font>,
    character: &LoreCharacter,
    content_width: i32,
) -> i32 {
    lore_profile_fields(character)
        .iter()
        .map(|(_, body)| profile_field_height(body_font, body, content_width))
        .sum()
}

fn draw_lore_profile_content(
    draw: &mut impl DrawTarget,
    label_font: Option<&Font>,
    body_font: Option<&Font>,
    character: &LoreCharacter,
    x: i32,
    y: i32,
    content_width: i32,
) -> i32 {
    let mut next_y = y;
    for (label, body) in lore_profile_fields(character) {
        next_y = draw_profile_field(
            draw,
            ProfileFieldDraw {
                label_font,
                body_font,
                label,
                body,
                x,
                y: next_y,
                content_width,
            },
        );
    }
    next_y
}

fn lore_profile_fields(character: &LoreCharacter) -> [(&'static str, &str); 6] {
    [
        ("CYCLE", character.cycle.as_str()),
        ("ORIGIN", character.origin.as_str()),
        ("PROFILE", character.profile.as_str()),
        ("GOAL", character.goal.as_str()),
        ("COMBAT", character.combat_style.as_str()),
        ("LINKER", character.linker_note.as_str()),
    ]
}

fn profile_field_height(body_font: Option<&Font>, body: &str, content_width: i32) -> i32 {
    screen_px(16)
        + wrapped_menu_text_height(body_font, body, content_width, 10.0, 14.0)
        + screen_px(12)
}

fn draw_profile_field(draw: &mut impl DrawTarget, field: ProfileFieldDraw<'_>) -> i32 {
    draw_menu_text(
        draw,
        field.label_font,
        field.label,
        field.x,
        field.y,
        10.0,
        MENU_HACK_GREEN,
    );
    let next_y = draw_wrapped_menu_text(
        draw,
        WrappedText {
            font: field.body_font,
            text: field.body,
            x: field.x,
            y: field.y + screen_px(16),
            max_width: field.content_width,
            font_size: 10.0,
            line_height: 14.0,
            color: Color::new(226, 233, 242, 255),
        },
    );
    next_y + screen_px(12)
}

fn draw_wrapped_menu_text(draw: &mut impl DrawTarget, text: WrappedText<'_>) -> i32 {
    let mut y = text.y;

    for line in wrap_menu_text_lines(text.font, text.text, text.max_width, text.font_size) {
        draw_menu_text(
            draw,
            text.font,
            &line,
            text.x,
            y,
            text.font_size,
            text.color,
        );
        y += screen_px(text.line_height.round() as i32);
    }

    y
}

fn wrapped_menu_text_height(
    font: Option<&Font>,
    text: &str,
    max_width: i32,
    font_size: f32,
    line_height: f32,
) -> i32 {
    wrap_menu_text_lines(font, text, max_width, font_size).len() as i32
        * screen_px(line_height.round() as i32)
}

fn wrap_menu_text_lines(
    font: Option<&Font>,
    text: &str,
    max_width: i32,
    font_size: f32,
) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current = String::new();

    for word in text.split_whitespace() {
        let candidate = if current.is_empty() {
            word.to_owned()
        } else {
            format!("{current} {word}")
        };

        if menu_text_width(font, &candidate, font_size, 1.0) <= max_width {
            current = candidate;
            continue;
        }

        if !current.is_empty() {
            lines.push(current);
        }
        current = word.to_owned();
    }

    if !current.is_empty() {
        lines.push(current);
    }

    lines
}

fn scroll_offset_px(
    scroll_lines: usize,
    line_height: i32,
    content_height: i32,
    viewport_height: i32,
) -> i32 {
    let max_offset = (content_height - viewport_height).max(0);
    let line_height_px = screen_px(line_height).max(1);
    let safe_lines = scroll_lines.min((i32::MAX / line_height_px) as usize) as i32;

    (safe_lines * line_height_px).min(max_offset)
}

fn draw_scrollbar(draw: &mut impl DrawTarget, area: ScrollArea, content_height: i32, offset: i32) {
    if content_height <= area.height {
        return;
    }

    let track_width = screen_px(4).max(2);
    let track_x = area.x + area.width - track_width;
    let thumb_min_height = screen_px(28);
    let thumb_height =
        ((area.height as f32 / content_height as f32) * area.height as f32).round() as i32;
    let thumb_height = thumb_height.clamp(thumb_min_height, area.height);
    let max_offset = (content_height - area.height).max(1);
    let travel = area.height - thumb_height;
    let thumb_y = area.y + ((offset as f32 / max_offset as f32) * travel as f32).round() as i32;

    draw.draw_rectangle(
        track_x,
        area.y,
        track_width,
        area.height,
        Color::new(138, 161, 184, 42),
    );
    draw.draw_rectangle(
        track_x,
        thumb_y,
        track_width,
        thumb_height,
        Color::new(0, 202, 255, 210),
    );
}

fn draw_menu_text(
    draw: &mut impl DrawTarget,
    font: Option<&Font>,
    text: &str,
    x: i32,
    y: i32,
    font_size: f32,
    color: Color,
) {
    let font_size = world_px(font_size);
    if let Some(font) = font {
        draw.draw_text_ex(
            font,
            text,
            Vector2::new(x as f32, y as f32),
            font_size,
            1.0,
            color,
        );
    } else {
        draw.draw_text(text, x, y, font_size.round() as i32, color);
    }
}

fn draw_centered_menu_text(
    draw: &mut impl DrawTarget,
    font: Option<&Font>,
    text: &str,
    center_x: i32,
    y: i32,
    font_size: f32,
    color: Color,
) {
    let width = menu_text_width(font, text, font_size, 1.0);
    draw_menu_text(draw, font, text, center_x - width / 2, y, font_size, color);
}

fn menu_text_width(font: Option<&Font>, text: &str, font_size: f32, spacing: f32) -> i32 {
    let font_size = world_px(font_size);
    if let Some(font) = font {
        font.measure_text(text, font_size, spacing).x.round() as i32
    } else {
        measure_text_width(text, font_size.round() as i32)
    }
}

fn world_cinematic(
    world: &World,
) -> Option<(&Fighter, crate::combat::cinematic::CinematicSpecialState)> {
    if world.outcome.is_some() {
        return None;
    }
    // One composition at a time on simultaneous starts. Combat still resolves
    // both moves independently; selecting the newest presentation never hits.
    [&world.player_one, &world.player_two]
        .into_iter()
        .filter_map(|fighter| fighter.cinematic_special().map(|state| (fighter, state)))
        .min_by_key(|(_, state)| state.elapsed_frames)
}

fn draw_world_cinematic_background(draw: &mut impl DrawTarget, world: &World, assets: &GameAssets) {
    authored_supers::draw_background(draw, world, assets);
    if let Some((fighter, state)) = world_cinematic(world) {
        cinematic_effects::draw_background(draw, fighter, state, assets);
    }
}

fn authored_actor_textures(assets: &GameAssets) -> authored_actors::AuthoredActorTextures<'_> {
    authored_actors::AuthoredActorTextures {
        duke: assets.duke_collector_poses.as_ref(),
        cpp_comedy: assets.cpp_footgun_comedy.as_ref(),
        cpp_barrage: assets.cpp_footgun_barrage.as_ref(),
        trash: assets.garbage_items.as_ref(),
        font: assets.menu_font.as_ref(),
    }
}

fn hides_authored_actor(world: &World, slot: PlayerSlot, assets: &GameAssets) -> bool {
    authored_actors::hides_actor(world, slot, authored_actor_textures(assets))
}

fn draw_authored_actors(draw: &mut impl DrawTarget, world: &World, assets: &GameAssets) {
    authored_actors::draw_authored_actors(draw, world, authored_actor_textures(assets));
}

fn draw_world_cinematic_foreground(draw: &mut impl DrawTarget, world: &World, assets: &GameAssets) {
    authored_supers::draw_foreground(draw, world, assets);
    if let Some((fighter, state)) = world_cinematic(world) {
        cinematic_effects::draw_foreground(draw, fighter, state, assets);
    }
}

fn draw_stage_life_layer(
    draw: &mut impl DrawTarget,
    arena: ArenaId,
    time: f32,
    flags: FeatureFlags,
    world: Option<&World>,
    assets: &GameAssets,
) {
    if flags.enabled(FeatureFlag::ShowStageLife) {
        stage_life::draw_stage_life(
            draw,
            stage_life::StageLifeDrawOptions {
                arena,
                time,
                dog_atlas: assets.caramelo_run.as_ref(),
                jessica_atlas: assets.jessica_gesture.as_ref(),
                font: assets.menu_font.as_ref(),
                cinematic_active: world.is_some_and(|world| {
                    world_cinematic(world).is_some() || world.super_sequence_active()
                }),
            },
        );
    }
}

fn draw_arena(
    draw: &mut impl DrawTarget,
    arena: ArenaId,
    background: Option<&Texture2D>,
    visual_time_seconds: f32,
) {
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

    draw_arena_background_animation(draw, arena, visual_time_seconds);
    draw_arena_screen_treatment(draw, visual_time_seconds);
}

fn draw_arena_bounds(draw: &mut impl DrawTarget) {
    draw.draw_line(
        ARENA_LEFT as i32,
        FLOOR_Y as i32,
        ARENA_RIGHT as i32,
        FLOOR_Y as i32,
        UI_MUTED,
    );
    draw.draw_line(
        ARENA_LEFT as i32,
        screen_px(96),
        ARENA_LEFT as i32,
        FLOOR_Y as i32,
        UI_MUTED,
    );
    draw.draw_line(
        ARENA_RIGHT as i32,
        screen_px(96),
        ARENA_RIGHT as i32,
        FLOOR_Y as i32,
        UI_MUTED,
    );
}

fn draw_arena_screen_treatment(draw: &mut impl DrawTarget, visual_time_seconds: f32) {
    let scanline_step = screen_px(7).max(1) as usize;
    let scanline_start =
        screen_px(88) + (visual_time_seconds * 18.0).round() as i32 % scanline_step as i32;
    let scanline_end = FLOOR_Y as i32;
    for y in (scanline_start..scanline_end).step_by(scanline_step) {
        draw.draw_line(0, y, WINDOW_WIDTH, y, Color::new(80, 190, 230, 10));
    }

    let pulse = pulse01(visual_time_seconds, 0.75, 0.0);
    let floor_y = FLOOR_Y;
    draw.draw_line_ex(
        Vector2::new(ARENA_LEFT, floor_y - world_px(2.0)),
        Vector2::new(ARENA_RIGHT, floor_y - world_px(2.0)),
        world_px(3.0),
        Color::new(80, 220, 255, (24.0 + pulse * 18.0).round() as u8),
    );
    draw.draw_line_ex(
        Vector2::new(ARENA_LEFT, floor_y - world_px(7.0)),
        Vector2::new(ARENA_RIGHT, floor_y - world_px(7.0)),
        world_px(1.0),
        Color::new(255, 218, 92, 20),
    );
}

fn draw_arena_background_animation(
    draw: &mut impl DrawTarget,
    arena: ArenaId,
    visual_time_seconds: f32,
) {
    match arena {
        ArenaId::Sirius => draw_sirius_motion(draw, visual_time_seconds),
        ArenaId::Fortaleza => draw_fortaleza_motion(draw, visual_time_seconds),
        ArenaId::JavaStreet => draw_java_street_motion(draw, visual_time_seconds),
        ArenaId::BioTic => draw_biotic_motion(draw, visual_time_seconds),
        ArenaId::PortoDigital => draw_porto_digital_motion(draw, visual_time_seconds),
        ArenaId::ValeDoPinhao => draw_vale_pinhao_motion(draw, visual_time_seconds),
    }
}

fn draw_arena_background(draw: &mut impl DrawTarget, texture: &Texture2D) {
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

fn draw_sirius_motion(draw: &mut impl DrawTarget, time: f32) {
    let origin = Vector2::new(WINDOW_WIDTH as f32 * 0.52, FLOOR_Y - world_px(168.0));
    for index in 0..6 {
        let phase = fract01(time * 0.09 + index as f32 * 0.17);
        let x = WINDOW_WIDTH as f32 * (0.50 + phase * 0.42);
        let y = world_px(24.0 + index as f32 * 12.0);
        let alpha = (72.0 * (1.0 - (phase - 0.48).abs())).clamp(12.0, 72.0) as u8;
        draw.draw_line_ex(
            origin,
            Vector2::new(x, y),
            world_px(1.5),
            Color::new(82, 220, 255, alpha),
        );
    }

    let ring = world_px(42.0 + pulse01(time, 0.8, 0.0) * 18.0);
    draw.draw_circle_lines(
        (WINDOW_WIDTH as f32 * 0.50).round() as i32,
        (FLOOR_Y - world_px(38.0)).round() as i32,
        ring,
        Color::new(82, 220, 255, 34),
    );
}

fn draw_fortaleza_motion(draw: &mut impl DrawTarget, time: f32) {
    for index in 0..8 {
        let phase = fract01(time * 0.12 + index as f32 * 0.13);
        let x = lerp(world_px(40.0), WINDOW_WIDTH as f32 - world_px(90.0), phase);
        let y = world_px(135.0 + (index % 4) as f32 * 27.0) + (time * 2.5).sin() * world_px(5.0);
        let alpha = (18.0 + 50.0 * (1.0 - (phase - 0.5).abs() * 2.0).max(0.0)) as u8;
        draw.draw_circle_lines(
            x.round() as i32,
            y.round() as i32,
            world_px(8.0),
            Color::new(80, 220, 255, alpha),
        );
        draw.draw_line_ex(
            Vector2::new(x - world_px(16.0), y),
            Vector2::new(x - world_px(42.0), y + world_px(7.0)),
            world_px(1.0),
            Color::new(95, 255, 174, alpha / 2),
        );
    }
}

fn draw_java_street_motion(draw: &mut impl DrawTarget, time: f32) {
    for index in 0..9 {
        let phase = fract01(time * 0.22 + index as f32 * 0.19);
        let x = screen_px(78 + index * 91);
        let y = lerp(screen_px(110) as f32, FLOOR_Y - world_px(96.0), phase);
        let alpha = (58.0 * (1.0 - phase)).max(10.0) as u8;
        let glyph = if index % 2 == 0 { "try" } else { "gc" };
        draw.draw_text(
            glyph,
            x,
            y.round() as i32,
            screen_px(12),
            Color::new(95, 255, 174, alpha),
        );
    }

    for index in 0..4 {
        let x = screen_px(126 + index * 214);
        let base_y = FLOOR_Y - world_px(26.0);
        let height = world_px(26.0 + pulse01(time, 0.55, index as f32 * 0.21) * 18.0);
        draw.draw_line_ex(
            Vector2::new(x as f32, base_y),
            Vector2::new(x as f32 + world_px(18.0), base_y - height),
            world_px(2.0),
            Color::new(176, 190, 210, 28),
        );
    }
}

fn draw_biotic_motion(draw: &mut impl DrawTarget, time: f32) {
    for index in 0..5 {
        let phase = time * 0.45 + index as f32 * 1.27;
        let center_x = WINDOW_WIDTH as f32 * (0.22 + index as f32 * 0.14);
        let center_y = world_px(118.0 + (index % 2) as f32 * 32.0);
        let x = center_x + phase.cos() * world_px(20.0);
        let y = center_y + phase.sin() * world_px(8.0);
        draw.draw_circle_gradient(
            x.round() as i32,
            y.round() as i32,
            world_px(12.0),
            Color::new(80, 220, 255, 54),
            Color::new(80, 220, 255, 0),
        );
        draw.draw_circle(
            x.round() as i32,
            y.round() as i32,
            world_px(3.0),
            Color::new(238, 241, 247, 96),
        );
    }

    for index in 0..4 {
        let x = screen_px(228 + index * 154);
        let y = screen_px(182 + (index % 2) * 34);
        let radius = world_px(12.0 + pulse01(time, 0.7, index as f32 * 0.3) * 12.0);
        draw.draw_circle_lines(x, y, radius, Color::new(95, 255, 174, 12));
    }
}

fn draw_porto_digital_motion(draw: &mut impl DrawTarget, time: f32) {
    for index in 0..9 {
        let y = FLOOR_Y - world_px(155.0 - index as f32 * 8.0);
        let wave = (time * 1.8 + index as f32 * 0.65).sin() * world_px(18.0);
        draw.draw_line_ex(
            Vector2::new(world_px(190.0) + wave, y),
            Vector2::new(WINDOW_WIDTH as f32 - world_px(210.0) + wave * 0.35, y),
            world_px(1.0),
            Color::new(80, 220, 255, 24),
        );
    }

    for index in 0..5 {
        let phase = fract01(time * 0.08 + index as f32 * 0.2);
        let x = lerp(
            world_px(160.0),
            WINDOW_WIDTH as f32 - world_px(180.0),
            phase,
        );
        let y = FLOOR_Y - world_px(225.0 + (index % 2) as f32 * 28.0);
        draw.draw_rectangle(
            x.round() as i32,
            y.round() as i32,
            screen_px(10),
            screen_px(3),
            Color::new(255, 191, 67, 48),
        );
    }
}

fn draw_vale_pinhao_motion(draw: &mut impl DrawTarget, time: f32) {
    for index in 0..16 {
        let phase = fract01(time * 0.34 + index as f32 * 0.061);
        let x = screen_px(30 + index * 61);
        let y = lerp(screen_px(70) as f32, FLOOR_Y - world_px(30.0), phase);
        draw.draw_line_ex(
            Vector2::new(x as f32, y),
            Vector2::new(x as f32 - world_px(7.0), y + world_px(24.0)),
            world_px(1.0),
            Color::new(176, 210, 235, 30),
        );
    }

    for index in 0..5 {
        let x = screen_px(148 + index * 168);
        let y = screen_px(198 + (index % 2) * 46);
        let alpha = (28.0 + pulse01(time, 0.5, index as f32 * 0.18) * 42.0) as u8;
        draw.draw_circle_gradient(
            x,
            y,
            world_px(16.0),
            Color::new(95, 255, 174, alpha),
            Color::new(95, 255, 174, 0),
        );
    }
}

fn fract01(value: f32) -> f32 {
    value - value.floor()
}

fn pulse01(time: f32, speed: f32, offset: f32) -> f32 {
    ((time * speed + offset).sin() + 1.0) * 0.5
}

fn lerp(start: f32, end: f32, t: f32) -> f32 {
    start + (end - start) * t
}

fn draw_fighter(
    draw: &mut impl DrawTarget,
    fighter: &crate::combat::fighter::Fighter,
    options: FighterDrawOptions<'_>,
) {
    let phase = fighter.attack_phase();
    let outcome_pose = matches!(
        options.forced_clip,
        Some(sprites::FighterSpriteClip::Victory | sprites::FighterSpriteClip::Defeat)
    );
    let phase_body = match phase {
        AttackPhase::Idle => options.body_color,
        AttackPhase::Startup => lighten(options.body_color, 30),
        AttackPhase::Active => Color::new(255, 222, 89, 255),
        AttackPhase::Recovery => dim(options.body_color, 25),
        AttackPhase::WhiffRecovery => dim(options.body_color, 45),
    };
    let body = if outcome_pose {
        options.body_color
    } else {
        fighter_body_color(fighter, phase_body)
    };
    // Combat freezes at KO; its final attack/reaction must not tint the entire
    // outcome animation. Only presentation changes here, never the fighter.
    let sprite_tint = if outcome_pose {
        Color::WHITE
    } else {
        fighter_sprite_tint(fighter, phase)
    };

    if let Some(sprite_atlas) = options.sprite_atlas
        && sprites::draw_manifest_fighter_sprite(
            draw,
            &sprite_atlas.manifest,
            fighter,
            options.world_elapsed_seconds,
            options.forced_clip,
            sprite_tint,
            |frame| sprite_atlas.texture_for_frame(frame),
        )
    {
    } else if let Some(texture) = options.spritesheet {
        sprites::draw_fighter_sprite(draw, texture, fighter, body);
    } else {
        draw_body_parts(draw, fighter, body);
    }

    // Rectangular body/guard feedback belongs to collision debug. Normal play
    // already communicates contact through sprite tint, ground lights and sparks.
    if options.show_debug && !outcome_pose {
        draw_fighter_state_flash(draw, fighter);
    }

    let sprite_combat = options.sprite_atlas.and_then(|sprite_atlas| {
        sprites::projected_fighter_combat(
            &sprite_atlas.combat_manifest,
            fighter,
            options.world_elapsed_seconds,
        )
    });

    if options.show_debug {
        outline_rect(draw, fighter.body_rect(), BODY_OUTLINE);
        if fighter.has_protected_reaction() {
            // Capture, flight and floor recovery have no vulnerable shape.
        } else if let Some(sprite_combat) = sprite_combat
            .as_ref()
            .filter(|_| !fighter.uses_low_attack_hurtboxes())
            .filter(|combat| !combat.hurtboxes.is_empty())
        {
            for hurtbox in &sprite_combat.hurtboxes {
                outline_rect(draw, *hurtbox, HURTBOX);
            }
        } else {
            for hurtbox in fighter.hurtboxes().rects() {
                outline_rect(draw, hurtbox, HURTBOX);
            }
        }
    }

    if options.show_debug
        && fighter.attack_kind() != Some(crate::combat::fighter::AttackKind::SignatureSpecial)
    {
        let sprite_hitboxes = sprite_combat
            .as_ref()
            .filter(|_| !fighter.uses_move_spec_hitbox())
            .map(|combat| combat.hitboxes.as_slice())
            .filter(|hitboxes| !hitboxes.is_empty());
        if let Some(hitboxes) = sprite_hitboxes {
            for hitbox in hitboxes {
                draw_hitbox_debug(draw, *hitbox, phase);
            }
        } else if let Some(attack_box) = fighter.attack_box() {
            draw_hitbox_debug(draw, attack_box, phase);
        }
    }

    if options.show_debug && fighter.blocking && !outcome_pose {
        let guard = fighter.guard_box();
        draw.draw_rectangle(
            guard.x.round() as i32,
            guard.y.round() as i32,
            guard.width.round() as i32,
            guard.height.round() as i32,
            GUARD_FILL,
        );
        outline_rect(draw, guard, GUARD);
        draw.draw_text(
            "BLOCK",
            (guard.x - world_px(18.0)) as i32,
            (guard.y - world_px(22.0)) as i32,
            screen_px(16),
            GUARD,
        );
    }

    let active_label_hitbox = if phase == AttackPhase::Active {
        sprite_combat
            .as_ref()
            .filter(|_| !fighter.uses_move_spec_hitbox())
            .and_then(|combat| combat.hitboxes.first().copied())
            .or_else(|| fighter.active_hitbox())
    } else {
        None
    };
    if options.show_debug
        && let Some(hitbox) = active_label_hitbox
    {
        draw.draw_text(
            "ACTIVE",
            hitbox.x as i32,
            (hitbox.y - world_px(22.0)) as i32,
            screen_px(16),
            HITBOX,
        );
    }

    let label_x = fighter.position.x as i32;
    let label_y = (fighter.position.y - world_px(22.0)) as i32;
    if options.show_debug {
        draw.draw_text(fighter.name, label_x, label_y, screen_px(16), UI_TEXT);
    }

    if options.show_debug && fighter.in_hitstun() {
        let stun_text = format!("HITSTUN {:02}", fighter.hitstun_remaining_frames().get());
        draw.draw_text(
            &stun_text,
            label_x,
            label_y - screen_px(24),
            screen_px(14),
            HITSPARK,
        );
    } else if options.show_debug && fighter.in_blockstun() {
        let stun_text = format!(
            "BLOCKSTUN {:02}",
            fighter.blockstun_remaining_frames().get()
        );
        draw.draw_text(
            &stun_text,
            label_x,
            label_y - screen_px(24),
            screen_px(14),
            GUARD,
        );
    } else if options.show_debug && fighter.in_whiff_recovery() {
        let recovery_text = format!(
            "WHIFF {:02}",
            fighter.whiff_recovery_remaining_frames().get()
        );
        draw.draw_text(
            &recovery_text,
            label_x,
            label_y - screen_px(40),
            screen_px(14),
            UI_MUTED,
        );
    }

    if options.show_debug
        && let Some(elapsed) = fighter.special_elapsed_frames()
    {
        let frame_data = fighter.projectile_frame_data();
        let frame_text = format!(
            "SPECIAL F{:02}/{:02}",
            elapsed.get(),
            frame_data.visual_duration.get()
        );
        let timing_text = format!(
            "SPAWN {:02} CD {:02}",
            frame_data.spawn_frame.get(),
            fighter.projectile_cooldown_remaining_frames().get()
        );
        draw.draw_text(
            &frame_text,
            label_x,
            label_y - screen_px(24),
            screen_px(14),
            PROJECTILE,
        );
        draw.draw_text(
            &timing_text,
            label_x,
            label_y - screen_px(40),
            screen_px(12),
            UI_MUTED,
        );
    }

    if options.show_debug && phase != AttackPhase::Idle {
        let attack_label = fighter
            .attack_move_spec()
            .map_or("ATTACK", |spec| spec.label);
        let phase_label = match phase {
            AttackPhase::Startup => "STARTUP",
            AttackPhase::Active => "ACTIVE",
            AttackPhase::Recovery => "RECOVER",
            AttackPhase::WhiffRecovery => "WHIFF",
            AttackPhase::Idle => "",
        };
        let frame_text = if let (Some(elapsed), Some(frame_data)) =
            (fighter.attack_elapsed_frames(), fighter.attack_frame_data())
        {
            format!(
                "{} F{:02}/{:02} {}",
                attack_label,
                elapsed.get(),
                frame_data.duration.get(),
                phase_label
            )
        } else {
            format!("{attack_label} {phase_label}")
        };
        draw.draw_text(
            &frame_text,
            label_x,
            label_y - screen_px(24),
            screen_px(14),
            HITSPARK,
        );

        if let Some(frame_data) = fighter.attack_frame_data() {
            let active_text = format!(
                "ACT {:02}-{:02}",
                frame_data.active_start.get(),
                frame_data.active_end.get()
            );
            draw.draw_text(
                &active_text,
                label_x,
                label_y - screen_px(40),
                screen_px(12),
                UI_MUTED,
            );
        }
    } else if options.show_debug && fighter.crouching {
        draw.draw_text(
            "CROUCH",
            label_x,
            label_y - screen_px(22),
            screen_px(18),
            UI_MUTED,
        );
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

struct CharacterVisuals<'a> {
    body_color: Color,
    fight_atlas: Option<&'a SpriteAtlasAsset>,
    start_atlas: Option<&'a SpriteAtlasAsset>,
    projectile_texture: Option<&'a Texture2D>,
}

fn character_visuals<'a>(character: CharacterId, assets: &'a GameAssets) -> CharacterVisuals<'a> {
    match character {
        CharacterId::Rust => CharacterVisuals {
            body_color: PLAYER_ONE,
            fight_atlas: assets.rust_fighter.as_ref(),
            start_atlas: assets.rust_start.as_ref(),
            projectile_texture: assets.rust_projectile.as_ref(),
        },
        CharacterId::Duke => CharacterVisuals {
            body_color: PLAYER_TWO,
            fight_atlas: assets.duke_fighter.as_ref(),
            start_atlas: assets.duke_start.as_ref(),
            projectile_texture: assets.duke_projectile.as_ref(),
        },
        CharacterId::Go => CharacterVisuals {
            body_color: PLAYER_GO,
            fight_atlas: assets.go_fighter.as_ref(),
            start_atlas: assets.go_start.as_ref(),
            projectile_texture: assets.go_projectile.as_ref(),
        },
        CharacterId::C => CharacterVisuals {
            body_color: PLAYER_C,
            fight_atlas: assets.c_fighter.as_ref(),
            start_atlas: assets.c_start.as_ref(),
            projectile_texture: assets.c_projectile.as_ref(),
        },
        CharacterId::Python => CharacterVisuals {
            body_color: PLAYER_PYTHON,
            fight_atlas: assets.python_fighter.as_ref(),
            start_atlas: assets.python_start.as_ref(),
            projectile_texture: assets.python_projectile.as_ref(),
        },
        CharacterId::Cpp => CharacterVisuals {
            body_color: PLAYER_CPP,
            fight_atlas: assets.cpp_fighter.as_ref(),
            start_atlas: None,
            projectile_texture: assets.cpp_projectile.as_ref(),
        },
    }
}

fn draw_hud(
    draw: &mut impl DrawTarget,
    world: &World,
    flags: FeatureFlags,
    gamepad_status: GamepadStatus,
    show_debug: bool,
    assets: &GameAssets,
) {
    let font = assets.menu_font.as_ref();
    draw.draw_rectangle_gradient_v(
        0,
        0,
        WINDOW_WIDTH,
        screen_px(108),
        Color::new(3, 9, 20, 236),
        Color::new(3, 9, 20, 0),
    );
    draw_menu_text(
        draw,
        font,
        "BORROW FIGHTERS",
        screen_px(24),
        screen_px(10),
        11.0,
        UI_MUTED,
    );
    draw_centered_menu_text(
        draw,
        font,
        "LOCAL VERSUS",
        WINDOW_WIDTH / 2,
        screen_px(13),
        10.0,
        UI_MUTED,
    );

    draw_health_bar(draw, font, &world.player_one, false);
    draw_health_bar(draw, font, &world.player_two, true);
    let center_x = WINDOW_WIDTH / 2;
    draw.draw_rectangle(
        center_x - screen_px(29),
        screen_px(37),
        screen_px(58),
        screen_px(45),
        Color::new(8, 20, 34, 232),
    );
    draw.draw_rectangle_lines(
        center_x - screen_px(29),
        screen_px(37),
        screen_px(58),
        screen_px(45),
        Color::new(112, 161, 185, 140),
    );
    draw_centered_menu_text(draw, font, "VS", center_x, screen_px(43), 28.0, UI_TEXT);

    if show_debug {
        draw_hud_debug_status(draw, flags, gamepad_status);
    }

    if let Some(outcome) = world.outcome {
        let (title, message, accent) = match outcome {
            MatchOutcome::Winner(PlayerSlot::One) => {
                ("VITÓRIA", world.player_one.name, MENU_ACCENT)
            }
            MatchOutcome::Winner(PlayerSlot::Two) => {
                ("VITÓRIA", world.player_two.name, MENU_ACCENT_ALT)
            }
            MatchOutcome::Draw => ("EMPATE", "Um round à altura dos dois.", UI_TEXT),
        };
        let banner = Rectangle::new(
            world_px(245.0),
            world_px(114.0),
            world_px(470.0),
            world_px(112.0),
        );
        draw.draw_rectangle_rec(banner, Color::new(4, 11, 22, 234));
        draw.draw_rectangle(
            banner.x as i32,
            banner.y as i32,
            banner.width as i32,
            screen_px(3),
            accent,
        );
        draw_centered_menu_text(draw, font, title, center_x, screen_px(121), 37.0, accent);
        draw_centered_menu_text(draw, font, message, center_x, screen_px(162), 21.0, UI_TEXT);
        draw_centered_menu_text(
            draw,
            font,
            "R / START  revanche     •     ESC  menu",
            center_x,
            screen_px(198),
            12.0,
            UI_MUTED,
        );
    }
}

fn draw_hud_debug_status(
    draw: &mut impl DrawTarget,
    flags: FeatureFlags,
    gamepad_status: GamepadStatus,
) {
    let status = format!(
        "P1 CPU {} | P2 CPU {} | Pad P1 {} | P2 {}",
        connected_label(flags.enabled(FeatureFlag::PlayerOneCpu)),
        connected_label(flags.enabled(FeatureFlag::PlayerTwoCpu)),
        connected_label(gamepad_status.player_one),
        connected_label(gamepad_status.player_two)
    );
    let status_font_size = screen_px(14);
    let width = measure_text_width(&status, status_font_size);
    draw.draw_text(
        &status,
        WINDOW_WIDTH - width - screen_px(24),
        screen_px(16),
        status_font_size,
        UI_MUTED,
    );
}

fn draw_countdown_sprite(draw: &mut impl DrawTarget, texture: &Texture2D) {
    let source = Rectangle::new(0.0, 0.0, texture.width() as f32, texture.height() as f32);

    let target_height = world_px(120.0);
    let scale = target_height / source.height;

    let dest_width = source.width * scale;
    let dest_height = source.height * scale;

    let dest = Rectangle::new(
        (WINDOW_WIDTH as f32 - dest_width) * 0.5,
        WINDOW_HEIGHT as f32 * 0.5 - dest_height * 0.5 - world_px(18.0),
        dest_width,
        dest_height,
    );

    draw.draw_texture_pro(
        texture,
        source,
        dest,
        Vector2::new(0.0, 0.0),
        0.0,
        Color::WHITE,
    );
}

fn draw_countdown_text(draw: &mut impl DrawTarget, label: &str) {
    let font_size = if label == "Fight!" {
        screen_px(54)
    } else {
        screen_px(78)
    };
    let width = measure_text_width(label, font_size);
    let x = (WINDOW_WIDTH - width) / 2;
    let y = WINDOW_HEIGHT / 2 - font_size / 2 - screen_px(18);
    let padding_x = screen_px(34);
    let padding_y = screen_px(18);

    draw.draw_rectangle(
        x - padding_x,
        y - padding_y,
        width + padding_x * 2,
        font_size + padding_y * 2,
        Color::new(0, 0, 0, 142),
    );
    draw.draw_rectangle_lines(
        x - padding_x,
        y - padding_y,
        width + padding_x * 2,
        font_size + padding_y * 2,
        Color::new(238, 241, 247, 180),
    );
    draw.draw_text(
        label,
        x + screen_px(4),
        y + screen_px(4),
        font_size,
        Color::new(0, 0, 0, 190),
    );
    draw.draw_text(label, x, y, font_size, UI_TEXT);
}

fn draw_countdown(draw: &mut impl DrawTarget, label: &str, assets: &GameAssets) {
    let texture = match label {
        "11" => assets.countdown_11.as_ref(),
        "10" => assets.countdown_10.as_ref(),
        "01" => assets.countdown_01.as_ref(),
        "Fight!" => assets.countdown_fight.as_ref(),
        _ => None,
    };

    if let Some(texture) = texture {
        draw_countdown_sprite(draw, texture);
    } else {
        draw_countdown_text(draw, label);
    }
}

fn draw_help(draw: &mut impl DrawTarget, font: Option<&Font>) {
    draw.draw_rectangle_gradient_v(
        0,
        WINDOW_HEIGHT - screen_px(142),
        WINDOW_WIDTH,
        screen_px(142),
        Color::new(3, 9, 20, 0),
        Color::new(3, 9, 20, 225),
    );
    draw_menu_text(
        draw,
        font,
        "P1: A/D/W/S movimento, Q defesa  |  Controle: LS/DPad, A pula, LB/LT defende",
        screen_px(24),
        WINDOW_HEIGHT - screen_px(124),
        15.0,
        UI_TEXT,
    );
    draw_menu_text(
        draw,
        font,
        "P1: F soco/X, H forte/Y, V chute/B, G projétil/RB, T assinatura/RT",
        screen_px(24),
        WINDOW_HEIGHT - screen_px(100),
        15.0,
        UI_TEXT,
    );
    draw_menu_text(
        draw,
        font,
        "P1: S+V rasteira, S+H gancho, frente+H overhead, Q+F agarrão, F/V no ar",
        screen_px(24),
        WINDOW_HEIGHT - screen_px(76),
        15.0,
        UI_TEXT,
    );
    draw_menu_text(
        draw,
        font,
        "CINEMÁTICO: P1 Y, P2 ] ou LB+RT  |  CPU: Options muda P1; C/View muda P2",
        screen_px(24),
        WINDOW_HEIGHT - screen_px(52),
        15.0,
        UI_TEXT,
    );
    draw_menu_text(
        draw,
        font,
        "P2: teclado/controle 2; assinatura \\ ou RT  |  R/Start revanche; F9/F10 grava; Esc menu",
        screen_px(24),
        WINDOW_HEIGHT - screen_px(28),
        15.0,
        UI_MUTED,
    );
}

fn connected_label(connected: bool) -> &'static str {
    if connected { "ON" } else { "OFF" }
}

fn measure_text_width(text: &str, font_size: i32) -> i32 {
    let c_text = CString::new(text).unwrap();
    unsafe { raylib::ffi::MeasureText(c_text.as_ptr(), font_size) }
}

fn truncate_middle(text: &str, max_chars: usize) -> String {
    if text.chars().count() <= max_chars {
        return text.to_string();
    }
    let keep = max_chars.saturating_sub(3) / 2;
    let start = text.chars().take(keep).collect::<String>();
    let end = text
        .chars()
        .rev()
        .take(keep)
        .collect::<String>()
        .chars()
        .rev()
        .collect::<String>();
    format!("{start}...{end}")
}

fn draw_health_bar(
    draw: &mut impl DrawTarget,
    font: Option<&Font>,
    fighter: &Fighter,
    mirrored: bool,
) {
    let width = screen_px(382);
    let x = if mirrored {
        WINDOW_WIDTH - screen_px(24) - width
    } else {
        screen_px(24)
    };
    let y = screen_px(66);
    let height = screen_px(15);
    let max_health = fighter.max_health.max(1);
    let health = fighter.health.clamp(0, max_health);
    let ratio = health as f32 / max_health as f32;
    let fill_width = (width as f32 * ratio).round() as i32;
    let accent = if mirrored {
        MENU_ACCENT_ALT
    } else {
        MENU_ACCENT
    };
    let fill = if health * 4 <= max_health {
        HEALTH_DANGER
    } else {
        accent
    };
    let fill_x = if mirrored { x + width - fill_width } else { x };

    draw.draw_rectangle(
        x - screen_px(3),
        y - screen_px(3),
        width + screen_px(6),
        height + screen_px(6),
        Color::new(3, 8, 15, 230),
    );
    draw.draw_rectangle(x, y, width, height, HEALTH_BACK);
    if fill_width > 0 {
        draw.draw_rectangle_gradient_v(
            fill_x,
            y,
            fill_width,
            height,
            fill,
            Color::new(fill.r / 2, fill.g / 2, fill.b / 2, 255),
        );
        draw.draw_rectangle(
            fill_x,
            y,
            fill_width,
            screen_px(2),
            Color::new(255, 255, 255, 164),
        );
    }
    draw.draw_rectangle_lines(
        x - screen_px(3),
        y - screen_px(3),
        width + screen_px(6),
        height + screen_px(6),
        Color::new(176, 194, 209, 144),
    );
    for index in 1..4 {
        let tick_x = x + width * index / 4;
        draw.draw_line(
            tick_x,
            y + screen_px(10),
            tick_x,
            y + height,
            Color::new(3, 8, 15, 150),
        );
    }

    let name_width = menu_text_width(font, fighter.name, 23.0, 1.0);
    let name_x = if mirrored { x + width - name_width } else { x };
    draw_menu_text(
        draw,
        font,
        fighter.name,
        name_x,
        screen_px(34),
        23.0,
        UI_TEXT,
    );
    let life = format!("{health} / {max_health}");
    let life_width = menu_text_width(font, &life, 11.0, 1.0);
    let life_x = if mirrored { x } else { x + width - life_width };
    draw_menu_text(draw, font, &life, life_x, screen_px(45), 11.0, UI_MUTED);
    let slot = if mirrored { "PLAYER 02" } else { "PLAYER 01" };
    let slot_width = menu_text_width(font, slot, 9.0, 1.0);
    let slot_x = if mirrored { x + width - slot_width } else { x };
    draw_menu_text(draw, font, slot, slot_x, screen_px(87), 9.0, accent);
    let special = if mirrored {
        "] / LB+RT   CINEMÁTICO"
    } else {
        "Y / LB+RT   CINEMÁTICO"
    };
    let special_width = menu_text_width(font, special, 10.0, 1.0);
    let special_x = if mirrored {
        x
    } else {
        x + width - special_width
    };
    draw.draw_rectangle(
        special_x - screen_px(4),
        screen_px(85),
        special_width + screen_px(8),
        screen_px(16),
        Color::new(3, 9, 20, 176),
    );
    draw_menu_text(draw, font, special, special_x, screen_px(87), 10.0, UI_TEXT);
}

fn draw_projectiles(
    draw: &mut impl DrawTarget,
    world: &World,
    show_debug: bool,
    assets: &GameAssets,
) {
    for projectile in &world.projectiles {
        let rect = projectile.rect();
        draw_projectile_trail(draw, projectile, rect);
        let projectile_texture =
            character_visuals(world.character_for_slot(projectile.owner), assets)
                .projectile_texture;
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
                world_px(8.0),
                PROJECTILE,
            );
        }

        if show_debug {
            outline_rect(draw, rect, PROJECTILE);
            draw.draw_text(
                "FIREBALL",
                (rect.x - world_px(12.0)) as i32,
                (rect.y - world_px(20.0)) as i32,
                screen_px(14),
                PROJECTILE,
            );
        }
    }
}

fn draw_projectile_trail(draw: &mut impl DrawTarget, projectile: &Projectile, rect: Rect) {
    let center = rect.center();
    let trail_direction = if projectile.velocity.x >= 0.0 {
        -1.0
    } else {
        1.0
    };
    let owner_color = match projectile.owner {
        PlayerSlot::One => Color::new(80, 220, 255, 255),
        PlayerSlot::Two => Color::new(255, 190, 92, 255),
    };

    draw.draw_line_ex(
        Vector2::new(center.x + trail_direction * world_px(10.0), center.y),
        Vector2::new(center.x + trail_direction * world_px(58.0), center.y),
        world_px(4.0),
        color_with_alpha(owner_color, 30),
    );

    for index in 0..4 {
        let distance = world_px(13.0 + index as f32 * 12.0);
        let radius = world_px(13.0 - index as f32 * 2.2).max(world_px(4.0));
        let alpha = [66, 48, 30, 16][index];
        let x = (center.x + trail_direction * distance).round() as i32;
        let y = center.y.round() as i32;
        draw.draw_circle_gradient(
            x,
            y,
            radius,
            color_with_alpha(owner_color, alpha),
            color_with_alpha(owner_color, 0),
        );
    }
}

fn fighter_atlas_for_intro<'a>(
    spawn_intro: bool,
    start_atlas: Option<&'a SpriteAtlasAsset>,
    fight_atlas: Option<&'a SpriteAtlasAsset>,
) -> Option<&'a SpriteAtlasAsset> {
    if spawn_intro {
        fight_atlas
            .filter(|atlas| atlas.manifest.clip_named("spawn").is_some())
            .or_else(|| start_atlas.filter(|atlas| atlas.manifest.clip_named("spawn").is_some()))
            .or(fight_atlas)
    } else {
        fight_atlas
    }
}

fn fighter_match_presentation(
    world: &World,
    fighter: &Fighter,
    spawn_intro: bool,
) -> (Option<sprites::FighterSpriteClip>, f32) {
    if let Some(sequence) = world.super_sequence() {
        use crate::combat::super_sequence::SuperPhase;
        if fighter.in_knockdown() {
            return (Some(sprites::FighterSpriteClip::Knockdown), 0.1);
        }
        if fighter.slot == sequence.attacker && sequence.phase() != SuperPhase::Freeze {
            let time = match sequence.character {
                CharacterId::Rust => match sequence.phase() {
                    SuperPhase::RustBuild => 0.08 + sequence.phase_progress() * 0.22,
                    SuperPhase::RustCharge => 0.3 + sequence.phase_progress() * 0.22,
                    SuperPhase::RustPulse => 0.52 + sequence.phase_progress() * 0.2,
                    SuperPhase::Restore => 0.72 + sequence.phase_progress() * 0.4,
                    _ => 0.0,
                },
                CharacterId::C => (sequence.tick as f32 / sequence.duration_frames as f32) * 1.1,
                _ => 0.0,
            };
            return (Some(sprites::FighterSpriteClip::SignatureSpecial), time);
        }
        return (
            None,
            authored_supers::arena_time(world, world.elapsed_seconds),
        );
    }
    if world.outcome.is_some() && fighter.health <= 0 && fighter.in_knockdown() {
        // A lethal throw/launch ends on the floor. Never restart a standing
        // defeat animation or play the recovery that would make the loser rise.
        return (Some(sprites::FighterSpriteClip::Knockdown), 0.1);
    }
    (
        sprites::match_fighter_sprite_clip(world.outcome, fighter.slot, spawn_intro),
        fighter_visual_elapsed_seconds(world),
    )
}

fn fighter_visual_elapsed_seconds(world: &World) -> f32 {
    if world.outcome.is_some() {
        world.outcome_elapsed_seconds()
    } else if world.spawn_intro_active() {
        world.spawn_intro_elapsed_seconds()
    } else {
        world.elapsed_seconds
    }
}

fn draw_body_collision(draw: &mut impl DrawTarget, world: &World) {
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
        world_px(6.0),
        BODY_COLLISION,
    );
    draw.draw_text(
        "BODY COLLISION",
        x - screen_px(76),
        top - screen_px(24),
        screen_px(18),
        BODY_COLLISION,
    );
}

fn draw_fighter_ground_lights(draw: &mut impl DrawTarget, world: &World) {
    if world.outcome.is_some() {
        return;
    }
    draw_fighter_ground_light(draw, &world.player_one);
    draw_fighter_ground_light(draw, &world.player_two);
}

fn draw_fighter_ground_light(draw: &mut impl DrawTarget, fighter: &Fighter) {
    let (fill, outline, frames) = if fighter.in_hitstun() {
        (
            Color::new(255, 90, 70, 58),
            Color::new(255, 218, 92, 120),
            fighter.hitstun_remaining_frames().get(),
        )
    } else if fighter.in_blockstun() {
        (
            Color::new(86, 156, 255, 52),
            Color::new(154, 205, 255, 110),
            fighter.blockstun_remaining_frames().get(),
        )
    } else {
        return;
    };

    let body = fighter.body_rect();
    let pulse = if frames % 4 < 2 { 1.0 } else { 0.84 };
    let center_x = body.center_x().round() as i32;
    let center_y = (FLOOR_Y + world_px(4.0)).round() as i32;
    let radius_h = (body.width * 0.72 + world_px(34.0)) * pulse;
    let radius_v = world_px(9.0) * pulse;

    draw.draw_ellipse(center_x, center_y, radius_h, radius_v, fill);
    draw.draw_ellipse_lines(center_x, center_y, radius_h, radius_v, outline);
    draw.draw_ellipse_lines(
        center_x,
        center_y,
        radius_h * 0.64,
        radius_v * 0.58,
        color_with_alpha(outline, 72),
    );
}

fn draw_hit_effects(draw: &mut impl DrawTarget, world: &World, font: Option<&Font>) {
    for effect in &world.hit_effects {
        let progress = hit_effect_progress(effect.timer);
        let fade_alpha = ((1.0 - progress) * 255.0).clamp(0.0, 255.0).round() as u8;
        let color = if effect.blocked {
            color_with_alpha(GUARD, fade_alpha)
        } else {
            color_with_alpha(HITSPARK, fade_alpha)
        };
        let x = effect.position.x.round() as i32;
        let y = effect.position.y.round() as i32;
        if effect.blocked {
            draw_block_pulse(
                draw,
                effect.position,
                effect.front_direction,
                progress,
                fade_alpha,
            );
        } else {
            draw_hitspark(draw, effect.position, progress, fade_alpha);
        }

        let damage = format!("-{}", effect.damage);
        draw_menu_text(
            draw,
            font,
            &damage,
            x + screen_px(14),
            y - screen_px(18),
            24.0,
            color,
        );
        let label = if effect.blocked { "BLOCK" } else { "HIT" };
        draw_menu_text(
            draw,
            font,
            label,
            x - screen_px(18),
            y - screen_px(42),
            20.0,
            color,
        );
    }
}

fn hit_effect_progress(timer: f32) -> f32 {
    1.0 - (timer / HIT_EFFECT_LIFETIME).clamp(0.0, 1.0)
}

fn draw_hitspark(
    draw: &mut impl DrawTarget,
    position: crate::math::vec2::Vec2,
    progress: f32,
    alpha: u8,
) {
    let x = position.x.round() as i32;
    let y = position.y.round() as i32;
    let radius = world_px(11.0 + progress * 35.0);
    let core = Color::new(255, 244, 150, alpha);
    let edge = Color::new(255, 110, 86, alpha.saturating_sub(28));

    draw.draw_circle_gradient(x, y, radius * 0.48, core, color_with_alpha(edge, 0));
    draw.draw_circle_lines(
        x,
        y,
        radius,
        color_with_alpha(core, alpha.saturating_sub(18)),
    );
    draw.draw_circle_lines(x, y, radius * 0.48, edge);

    for index in 0..8 {
        let angle = index as f32 * TAU / 8.0 + progress * 0.7;
        let inner = radius * 0.16;
        let outer = radius * (0.62 + (index % 3) as f32 * 0.13);
        let start = Vector2::new(
            position.x + angle.cos() * inner,
            position.y + angle.sin() * inner,
        );
        let end = Vector2::new(
            position.x + angle.cos() * outer,
            position.y + angle.sin() * outer,
        );
        let thickness = if index % 2 == 0 {
            world_px(3.0)
        } else {
            world_px(2.0)
        };
        draw.draw_line_ex(start, end, thickness, color_with_alpha(core, alpha));
    }
}

fn draw_block_pulse(
    draw: &mut impl DrawTarget,
    position: crate::math::vec2::Vec2,
    front_direction: f32,
    progress: f32,
    alpha: u8,
) {
    let x = position.x.round() as i32;
    let y = position.y.round() as i32;
    let radius = world_px(17.0 + progress * 39.0);
    let shield = Color::new(154, 205, 255, alpha);
    let glow = Color::new(86, 156, 255, alpha.saturating_sub(56));

    draw.draw_circle_gradient(
        x,
        y,
        radius * 0.6,
        color_with_alpha(glow, 52),
        color_with_alpha(glow, 0),
    );
    draw.draw_circle_lines(x, y, radius, shield);
    draw.draw_circle_lines(
        x,
        y,
        radius * 0.66,
        color_with_alpha(shield, alpha.saturating_sub(42)),
    );

    let side_x = x as f32 + radius * 0.46 * front_direction;
    let back_x = x as f32 - radius * 0.42 * front_direction;
    draw.draw_line_ex(
        Vector2::new(side_x, y as f32 - radius * 0.74),
        Vector2::new(side_x, y as f32 + radius * 0.74),
        world_px(3.0),
        shield,
    );
    draw.draw_line_ex(
        Vector2::new(side_x, y as f32 - radius * 0.74),
        Vector2::new(back_x, y as f32),
        world_px(2.0),
        color_with_alpha(shield, alpha.saturating_sub(36)),
    );
    draw.draw_line_ex(
        Vector2::new(side_x, y as f32 + radius * 0.74),
        Vector2::new(back_x, y as f32),
        world_px(2.0),
        color_with_alpha(shield, alpha.saturating_sub(36)),
    );
}

fn draw_body_parts(
    draw: &mut impl DrawTarget,
    fighter: &crate::combat::fighter::Fighter,
    color: Color,
) {
    let parts = fighter.body_parts();
    fill_rect(draw, parts.head, lighten(color, 28));
    fill_rect(draw, parts.torso, color);
    fill_rect(draw, parts.legs, dim(color, 22));
}

fn fighter_body_color(fighter: &crate::combat::fighter::Fighter, phase_color: Color) -> Color {
    if fighter.in_hitstun() {
        HIT_FLASH
    } else if fighter.in_blockstun() {
        BLOCK_FLASH
    } else {
        phase_color
    }
}

fn fighter_sprite_tint(fighter: &crate::combat::fighter::Fighter, phase: AttackPhase) -> Color {
    let impact_flash = fighter.reaction_visual_elapsed_seconds() < 4.0 / 60.0;
    if fighter.in_hitstun() && impact_flash {
        HIT_FLASH
    } else if fighter.in_blockstun() && impact_flash {
        BLOCK_FLASH
    } else if phase == AttackPhase::Active {
        ACTIVE_SPRITE_TINT
    } else {
        Color::WHITE
    }
}

fn draw_fighter_state_flash(draw: &mut impl DrawTarget, fighter: &crate::combat::fighter::Fighter) {
    let (fill, outline) = if fighter.in_hitstun() {
        (HIT_FLASH_FILL, HIT_FLASH)
    } else if fighter.in_blockstun() {
        (BLOCK_FLASH_FILL, BLOCK_FLASH)
    } else {
        return;
    };

    let body = fighter.body_rect();
    let rect = Rect::new(
        body.x - world_px(8.0),
        body.y - world_px(6.0),
        body.width + world_px(16.0),
        body.height + world_px(12.0),
    );
    fill_rect(draw, rect, fill);
    outline_rect(draw, rect, outline);
}

fn fill_rect(draw: &mut impl DrawTarget, rect: Rect, color: Color) {
    draw.draw_rectangle(
        rect.x.round() as i32,
        rect.y.round() as i32,
        rect.width.round() as i32,
        rect.height.round() as i32,
        color,
    );
}

fn draw_hitbox_debug(draw: &mut impl DrawTarget, hitbox: Rect, phase: AttackPhase) {
    draw.draw_rectangle(
        hitbox.x.round() as i32,
        hitbox.y.round() as i32,
        hitbox.width.round() as i32,
        hitbox.height.round() as i32,
        if phase == AttackPhase::Active {
            HITBOX_FILL
        } else {
            Color::new(255, 82, 82, 34)
        },
    );
    outline_rect(draw, hitbox, HITBOX);
}

fn outline_rect(draw: &mut impl DrawTarget, rect: Rect, color: Color) {
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

fn color_with_alpha(color: Color, alpha: u8) -> Color {
    Color::new(color.r, color.g, color.b, alpha)
}
