//! Draws the greybox prototype.
//!
//! Rendering intentionally uses primitive shapes and debug overlays so gameplay
//! problems are visible before art production starts.

use raylib::core::text::RaylibFont;
use raylib::prelude::*;
use std::ffi::CString;

mod combat_lab;
mod sprite_viewer;

pub use combat_lab::draw_combat_lab;
pub use sprite_viewer::{draw_sprite_viewer, draw_sprite_viewer_error};

use crate::characters::{CharacterId, character_spec};
use crate::combat::fighter::{AttackPhase, Facing, Fighter, PlayerSlot};
use crate::config::{ARENA_LEFT, ARENA_RIGHT, FLOOR_Y, WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::engine::assets::{GameAssets, SpriteAtlasAsset};
use crate::engine::sprites;
use crate::game::arena::ArenaId;
use crate::game::feature_flags::{FeatureFlag, FeatureFlags, PREFERENCE_FLAGS};
use crate::game::world::{MatchOutcome, World};
use crate::math::rect::Rect;
use crate::scenes::preferences::{MenuPage, PreferencesMenu};

const BACKGROUND: Color = Color::new(18, 20, 26, 255);
const FLOOR: Color = Color::new(72, 76, 88, 255);
const PLAYER_ONE: Color = Color::new(112, 181, 255, 255);
const PLAYER_TWO: Color = Color::new(255, 178, 104, 255);
const PLAYER_GO: Color = Color::new(96, 220, 190, 255);
pub(super) const PLAYER_C: Color = Color::new(126, 194, 255, 255);
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
const RECORDING: Color = Color::new(255, 55, 72, 255);
const MENU_PANEL_STRONG: Color = Color::new(8, 14, 28, 238);
const MENU_ACCENT: Color = Color::new(0, 202, 255, 255);
const MENU_ACCENT_ALT: Color = Color::new(255, 191, 67, 255);
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
    flags: FeatureFlags,
    gamepad_status: GamepadStatus,
    assets: &GameAssets,
) {
    draw.clear_background(BACKGROUND);
    draw_arena(draw, assets.arenas.get(arena));
    let show_debug = flags.enabled(FeatureFlag::ShowCombatDebug);
    let spawn_intro = world.spawn_intro_active();
    let player_one_visuals = character_visuals(world.player_one_character(), assets);
    let player_two_visuals = character_visuals(world.player_two_character(), assets);

    draw_projectiles(draw, world, show_debug, assets);
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
            world_elapsed_seconds: fighter_visual_elapsed_seconds(
                world,
                spawn_intro,
                player_one_visuals.start_atlas.is_some(),
            ),
            forced_clip: forced_fighter_clip(
                world,
                PlayerSlot::One,
                spawn_intro,
                player_one_visuals.start_atlas.is_some(),
            ),
        },
    );
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
            world_elapsed_seconds: fighter_visual_elapsed_seconds(
                world,
                spawn_intro,
                player_two_visuals.start_atlas.is_some(),
            ),
            forced_clip: forced_fighter_clip(
                world,
                PlayerSlot::Two,
                spawn_intro,
                player_two_visuals.start_atlas.is_some(),
            ),
        },
    );
    if show_debug {
        draw_body_collision(draw, world);
    }
    draw_hit_effects(draw, world);

    if flags.enabled(FeatureFlag::ShowHud) {
        draw_hud(draw, world, flags, gamepad_status);
    }

    if let Some(label) = world.countdown_label() {
        draw_countdown(draw, label, assets);
    }

    if flags.enabled(FeatureFlag::ShowControlsHelp) {
        draw_help(draw);
    }
}

/// Draws the main menu and nested prototype setup screens.
pub fn draw_preferences(draw: &mut impl DrawTarget, options: PreferencesDrawOptions<'_>) {
    draw.clear_background(BACKGROUND);
    draw_arena(draw, options.assets.arenas.get(options.arena));
    draw.draw_rectangle(0, 0, WINDOW_WIDTH, WINDOW_HEIGHT, Color::new(0, 0, 0, 164));
    draw.draw_rectangle(0, 0, WINDOW_WIDTH, WINDOW_HEIGHT, Color::new(4, 9, 22, 68));

    let font = options.assets.menu_font.as_ref();
    draw_menu_side_fighters(draw, options.assets);
    draw_menu_chrome(draw, font, &options);

    match options.menu.page() {
        MenuPage::Main => draw_main_menu(draw, font, &options),
        MenuPage::Versus => draw_versus_menu(draw, font, &options),
        MenuPage::Training => draw_training_menu(draw, font, &options),
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
        let width = measure_text_width(text, 16);
        let box_width = width + 48;
        let x = (WINDOW_WIDTH - box_width) / 2;
        let y = 14;

        draw.draw_rectangle(x, y, box_width, 28, Color::new(8, 10, 14, 218));
        draw.draw_rectangle_lines(x, y, box_width, 28, RECORDING);
        draw.draw_circle(x + 18, y + 14, 6.0, RECORDING);
        draw.draw_text(text, x + 34, y + 6, 16, UI_TEXT);
        return;
    }

    let Some(message) = message else {
        return;
    };
    if !message.starts_with("Falha") && !message.starts_with("Gravacao salva") {
        return;
    }

    let text = truncate_middle(message, 92);
    let width = measure_text_width(&text, 13);
    let x = WINDOW_WIDTH - width - 20;
    let y = WINDOW_HEIGHT - 24;
    draw.draw_rectangle(x - 8, y - 4, width + 16, 22, Color::new(8, 10, 14, 190));
    draw.draw_text(&text, x, y, 13, UI_MUTED);
}

/// Data needed by the preferences renderer.
pub struct PreferencesDrawOptions<'a> {
    pub menu: &'a PreferencesMenu,
    pub player_one_character: CharacterId,
    pub player_two_character: CharacterId,
    pub arena: ArenaId,
    pub flags: FeatureFlags,
    pub gamepad_status: GamepadStatus,
    pub recording: bool,
    pub assets: &'a GameAssets,
}

#[derive(Clone, Copy)]
struct MenuPanel {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

struct MenuLine<'a> {
    label: &'a str,
    description: &'a str,
    value: Option<&'a str>,
    checked: Option<bool>,
}

fn draw_menu_chrome(
    draw: &mut impl DrawTarget,
    font: Option<&Font>,
    options: &PreferencesDrawOptions<'_>,
) {
    draw_centered_menu_text(
        draw,
        font,
        "BORROW FIGHTERS",
        WINDOW_WIDTH / 2,
        34,
        50.0,
        UI_TEXT,
    );
    draw_centered_menu_text(
        draw,
        font,
        "CODE. COMMIT. COMBO.",
        WINDOW_WIDTH / 2,
        89,
        18.0,
        MENU_ACCENT,
    );

    let status = format!(
        "PADS  P1 {}  P2 {}",
        connected_label(options.gamepad_status.player_one),
        connected_label(options.gamepad_status.player_two)
    );
    let width = menu_text_width(font, &status, 14.0, 1.0);
    draw_menu_text(
        draw,
        font,
        &status,
        WINDOW_WIDTH - width - 28,
        20,
        14.0,
        UI_MUTED,
    );
}

fn draw_main_menu(
    draw: &mut impl DrawTarget,
    font: Option<&Font>,
    options: &PreferencesDrawOptions<'_>,
) {
    let panel = MenuPanel {
        x: 300,
        y: 128,
        width: 424,
        height: 380,
    };
    draw_menu_panel(draw, panel);

    let rows = [
        MenuLine {
            label: "QUICK FIGHT",
            description: "Inicia a luta com a configuracao atual.",
            value: None,
            checked: None,
        },
        MenuLine {
            label: "VERSUS SETUP",
            description: "Escolha personagens antes da luta.",
            value: None,
            checked: None,
        },
        MenuLine {
            label: "TRAINING",
            description: "Combat Lab e Sprite Viewer.",
            value: None,
            checked: None,
        },
        MenuLine {
            label: "OPTIONS",
            description: "Flags de prototipo, HUD, CPU e gravacao.",
            value: None,
            checked: None,
        },
        MenuLine {
            label: "EXIT",
            description: "Fecha o prototipo.",
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
            panel,
            row_height: 52,
            start_offset_y: 86,
            large_labels: true,
            show_descriptions: false,
        },
    );
    draw_menu_footer(draw, font, panel, "Setas/W/S navegam  |  Enter confirma");
}

fn draw_versus_menu(
    draw: &mut impl DrawTarget,
    font: Option<&Font>,
    options: &PreferencesDrawOptions<'_>,
) {
    let panel = MenuPanel {
        x: 270,
        y: 138,
        width: 484,
        height: 382,
    };
    draw_menu_panel(draw, panel);
    draw_menu_page_title(draw, font, panel, "VERSUS SETUP");

    let player_one = character_spec(options.player_one_character).display_name;
    let player_two = character_spec(options.player_two_character).display_name;
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
            panel,
            row_height: 58,
            start_offset_y: 108,
            large_labels: false,
            show_descriptions: true,
        },
    );
    draw_menu_footer(
        draw,
        font,
        panel,
        "A/D ou setas ajustam personagem  |  Esc volta",
    );
}

fn draw_training_menu(
    draw: &mut impl DrawTarget,
    font: Option<&Font>,
    options: &PreferencesDrawOptions<'_>,
) {
    let panel = MenuPanel {
        x: 286,
        y: 150,
        width: 452,
        height: 330,
    };
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
            panel,
            row_height: 62,
            start_offset_y: 120,
            large_labels: false,
            show_descriptions: true,
        },
    );
    draw_menu_footer(draw, font, panel, "Esc volta das ferramentas para o menu");
}

fn draw_options_menu(
    draw: &mut impl DrawTarget,
    font: Option<&Font>,
    options: &PreferencesDrawOptions<'_>,
) {
    let panel = MenuPanel {
        x: 176,
        y: 74,
        width: 672,
        height: 448,
    };
    draw_menu_panel(draw, panel);
    draw_menu_page_title(draw, font, panel, "OPTIONS");

    let row_x = panel.x + 48;
    let row_width = panel.width - 96;
    let row_height = 28;
    let row_y = panel.y + 86;

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

    for (index, flag) in PREFERENCE_FLAGS.iter().copied().enumerate() {
        let row = index + PreferencesMenu::OPTIONS_FIRST_FLAG_ROW;
        draw_option_row(
            draw,
            font,
            OptionRow {
                x: row_x,
                y: row_y + row as i32 * row_height,
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
            y: row_y + back_row as i32 * row_height,
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
        panel.x + 48,
        panel.y + panel.height - 52,
        13.0,
        UI_MUTED,
    );
    draw_menu_footer(
        draw,
        font,
        panel,
        "Enter/Espaco alterna  |  F9/F10 gravacao  |  Esc volta",
    );
}

fn draw_menu_panel(draw: &mut impl DrawTarget, panel: MenuPanel) {
    draw.draw_rectangle(
        panel.x - 8,
        panel.y - 8,
        panel.width + 16,
        panel.height + 16,
        Color::new(0, 0, 0, 94),
    );
    draw.draw_rectangle(
        panel.x,
        panel.y,
        panel.width,
        panel.height,
        MENU_PANEL_STRONG,
    );
    draw.draw_rectangle_lines(
        panel.x,
        panel.y,
        panel.width,
        panel.height,
        Color::new(87, 193, 255, 150),
    );
    draw.draw_line(
        panel.x + 22,
        panel.y + 18,
        panel.x + panel.width - 22,
        panel.y + 18,
        Color::new(87, 193, 255, 128),
    );
    draw.draw_line(
        panel.x + 22,
        panel.y + panel.height - 44,
        panel.x + panel.width - 22,
        panel.y + panel.height - 44,
        Color::new(87, 193, 255, 80),
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
        panel.y + 44,
        26.0,
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
    let row_x = layout.panel.x + 42;
    let row_width = layout.panel.width - 84;
    for (index, row) in rows.iter().enumerate() {
        draw_large_menu_row(
            draw,
            font,
            LargeMenuRow {
                x: row_x,
                y: layout.panel.y + layout.start_offset_y + index as i32 * layout.row_height,
                width: row_width,
                height: layout.row_height - 8,
                selected: selected == index,
                label: row.label,
                description: row.description,
                value: row.value,
                checked: row.checked,
                large_label: layout.large_labels,
                show_description: layout.show_descriptions,
            },
        );
    }
}

#[derive(Clone, Copy)]
struct MenuRowsLayout {
    panel: MenuPanel,
    row_height: i32,
    start_offset_y: i32,
    large_labels: bool,
    show_descriptions: bool,
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
}

fn draw_large_menu_row(draw: &mut impl DrawTarget, font: Option<&Font>, row: LargeMenuRow<'_>) {
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
    draw.draw_rectangle(row.x, row.y, row.width, row.height, fill);
    draw.draw_rectangle_lines(row.x, row.y, row.width, row.height, border);

    if row.selected {
        draw.draw_rectangle(row.x, row.y, 5, row.height, MENU_ACCENT_ALT);
    }

    let label_x = if let Some(checked) = row.checked {
        draw_checkbox(draw, row.x + 18, row.y + row.height / 2 - 9, checked);
        row.x + 50
    } else {
        draw_menu_text(
            draw,
            font,
            if row.selected { ">" } else { "" },
            row.x + 18,
            row.y + row.height / 2 - 13,
            22.0,
            MENU_ACCENT,
        );
        row.x + 50
    };

    let label_size = if row.large_label { 27.0 } else { 20.0 };
    let label_y = if row.show_description {
        row.y + 5
    } else {
        row.y + row.height / 2 - 15
    };
    draw_menu_text(draw, font, row.label, label_x, label_y, label_size, UI_TEXT);

    if row.show_description && !row.description.is_empty() {
        draw_menu_text(
            draw,
            font,
            row.description,
            label_x,
            row.y + row.height - 17,
            12.0,
            UI_MUTED,
        );
    }

    if let Some(value) = row.value {
        let text = format!("< {value} >");
        let text_width = menu_text_width(font, &text, 18.0, 1.0);
        draw_menu_text(
            draw,
            font,
            &text,
            row.x + row.width - text_width - 18,
            row.y + 9,
            18.0,
            HEALTH_FILL,
        );
    }
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
    draw.draw_rectangle(row.x, row.y, row.width, row.height - 2, fill);
    if row.selected {
        draw.draw_rectangle_lines(row.x, row.y, row.width, row.height - 2, MENU_ACCENT);
    }

    let label_x = if let Some(checked) = row.checked {
        draw_checkbox(draw, row.x + 12, row.y + 5, checked);
        row.x + 42
    } else {
        row.x + 42
    };

    draw_menu_text(draw, font, row.label, label_x, row.y + 5, 15.0, UI_TEXT);

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
            row.x + row.width - width - 14,
            row.y + 5,
            15.0,
            color,
        );
    }
}

fn draw_checkbox(draw: &mut impl DrawTarget, x: i32, y: i32, enabled: bool) {
    let size = 18;
    draw.draw_rectangle_lines(x, y, size, size, UI_TEXT);
    if enabled {
        draw.draw_rectangle(x + 4, y + 4, size - 8, size - 8, HEALTH_FILL);
    }
}

fn draw_menu_footer(draw: &mut impl DrawTarget, font: Option<&Font>, panel: MenuPanel, text: &str) {
    draw_centered_menu_text(
        draw,
        font,
        text,
        panel.x + panel.width / 2,
        panel.y + panel.height - 30,
        12.0,
        UI_MUTED,
    );
}

fn selected_options_hint(options: &PreferencesDrawOptions<'_>) -> &'static str {
    if options.menu.selected() == PreferencesMenu::OPTIONS_RECORDING_ROW {
        return "Gravacao local salva videos em captures/; F9 inicia e F10 para.";
    }
    if options.menu.selected() == options.menu.row_count() - 1 {
        return "Volta para o menu principal.";
    }

    PREFERENCE_FLAGS[options.menu.selected() - PreferencesMenu::OPTIONS_FIRST_FLAG_ROW]
        .description()
}

fn draw_menu_side_fighters(draw: &mut impl DrawTarget, assets: &GameAssets) {
    let mut c_fighter = Fighter::new(
        PlayerSlot::One,
        character_spec(CharacterId::C).display_name,
        182.0,
    );
    c_fighter.facing = Facing::Right;
    let c_visuals = character_visuals(CharacterId::C, assets);
    draw_fighter(
        draw,
        &c_fighter,
        FighterDrawOptions {
            body_color: c_visuals.body_color,
            show_debug: false,
            sprite_atlas: c_visuals.fight_atlas,
            spritesheet: assets.fighter_spritesheet.as_ref(),
            world_elapsed_seconds: 0.0,
            forced_clip: Some(sprites::FighterSpriteClip::Idle),
        },
    );

    let mut rust_fighter = Fighter::new(
        PlayerSlot::Two,
        character_spec(CharacterId::Rust).display_name,
        796.0,
    );
    rust_fighter.facing = Facing::Left;
    let rust_visuals = character_visuals(CharacterId::Rust, assets);
    draw_fighter(
        draw,
        &rust_fighter,
        FighterDrawOptions {
            body_color: rust_visuals.body_color,
            show_debug: false,
            sprite_atlas: rust_visuals.fight_atlas,
            spritesheet: assets.fighter_spritesheet.as_ref(),
            world_elapsed_seconds: 0.0,
            forced_clip: Some(sprites::FighterSpriteClip::Idle),
        },
    );

    draw.draw_rectangle(246, 120, 532, 440, Color::new(0, 0, 0, 60));
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
    if let Some(font) = font {
        font.measure_text(text, font_size, spacing).x.round() as i32
    } else {
        measure_text_width(text, font_size.round() as i32)
    }
}

fn draw_arena(draw: &mut impl DrawTarget, background: Option<&Texture2D>) {
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

fn draw_fighter(
    draw: &mut impl DrawTarget,
    fighter: &crate::combat::fighter::Fighter,
    options: FighterDrawOptions<'_>,
) {
    let phase = fighter.attack_phase();
    let body = match phase {
        AttackPhase::Idle => options.body_color,
        AttackPhase::Startup => lighten(options.body_color, 30),
        AttackPhase::Active => Color::new(255, 222, 89, 255),
        AttackPhase::Recovery => dim(options.body_color, 25),
        AttackPhase::WhiffRecovery => dim(options.body_color, 45),
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

    let sprite_combat = options.sprite_atlas.and_then(|sprite_atlas| {
        sprites::projected_fighter_combat(
            &sprite_atlas.manifest,
            fighter,
            options.world_elapsed_seconds,
        )
    });

    if options.show_debug {
        outline_rect(draw, fighter.body_rect(), BODY_OUTLINE);
        if let Some(sprite_combat) = sprite_combat
            .as_ref()
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

    if options.show_debug {
        let sprite_hitboxes = sprite_combat
            .as_ref()
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

    let active_label_hitbox = if phase == AttackPhase::Active {
        sprite_combat
            .as_ref()
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
            (hitbox.y - 22.0) as i32,
            16,
            HITBOX,
        );
    }

    let label_x = fighter.position.x as i32;
    let label_y = (fighter.position.y - 22.0) as i32;
    draw.draw_text(fighter.name, label_x, label_y, 16, UI_TEXT);

    if options.show_debug && fighter.in_hitstun() {
        let stun_text = format!("HITSTUN {:02}", fighter.hitstun_remaining_frames().get());
        draw.draw_text(&stun_text, label_x, label_y - 24, 14, HITSPARK);
    } else if options.show_debug && fighter.in_blockstun() {
        let stun_text = format!(
            "BLOCKSTUN {:02}",
            fighter.blockstun_remaining_frames().get()
        );
        draw.draw_text(&stun_text, label_x, label_y - 24, 14, GUARD);
    } else if options.show_debug && fighter.in_whiff_recovery() {
        let recovery_text = format!(
            "WHIFF {:02}",
            fighter.whiff_recovery_remaining_frames().get()
        );
        draw.draw_text(&recovery_text, label_x, label_y - 40, 14, UI_MUTED);
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
        draw.draw_text(&frame_text, label_x, label_y - 24, 14, PROJECTILE);
        draw.draw_text(&timing_text, label_x, label_y - 40, 12, UI_MUTED);
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
        draw.draw_text(&frame_text, label_x, label_y - 24, 14, HITSPARK);

        if let Some(frame_data) = fighter.attack_frame_data() {
            let active_text = format!(
                "ACT {:02}-{:02}",
                frame_data.active_start.get(),
                frame_data.active_end.get()
            );
            draw.draw_text(&active_text, label_x, label_y - 40, 12, UI_MUTED);
        }
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
    }
}

fn draw_hud(
    draw: &mut impl DrawTarget,
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
        "P1 CPU {} | P2 CPU {} | Pad P1 {} | P2 {}",
        connected_label(flags.enabled(FeatureFlag::PlayerOneCpu)),
        connected_label(flags.enabled(FeatureFlag::PlayerTwoCpu)),
        connected_label(gamepad_status.player_one),
        connected_label(gamepad_status.player_two)
    );
    let width = measure_text_width(&status, 14);
    draw.draw_text(&status, WINDOW_WIDTH - width - 24, 16, 14, UI_MUTED);

    draw_health_bar(
        draw,
        24,
        72,
        world.player_one.health,
        world.player_one.max_health,
        world.player_one.name,
    );
    draw_health_bar(
        draw,
        WINDOW_WIDTH - 324,
        72,
        world.player_two.health,
        world.player_two.max_health,
        world.player_two.name,
    );

    if let Some(outcome) = world.outcome {
        let message = match outcome {
            MatchOutcome::Winner(PlayerSlot::One) => {
                format!("{} wins - press R/Menu", world.player_one.name)
            }
            MatchOutcome::Winner(PlayerSlot::Two) => {
                format!("{} wins - press R/Menu", world.player_two.name)
            }
            MatchOutcome::Draw => "Draw - press R/Menu".to_owned(),
        };
        let width = measure_text_width(&message, 32);
        draw.draw_text(&message, (WINDOW_WIDTH - width) / 2, 124, 32, UI_TEXT);
    }
}

fn draw_countdown_sprite(draw: &mut impl DrawTarget, texture: &Texture2D) {
    let source = Rectangle::new(0.0, 0.0, texture.width() as f32, texture.height() as f32);

    let target_height = 120.0;
    let scale = target_height / source.height;

    let dest_width = source.width * scale;
    let dest_height = source.height * scale;

    let dest = Rectangle::new(
        (WINDOW_WIDTH as f32 - dest_width) * 0.5,
        WINDOW_HEIGHT as f32 * 0.5 - dest_height * 0.5 - 18.0,
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
    let font_size = if label == "Fight!" { 54 } else { 78 };
    let width = measure_text_width(label, font_size);
    let x = (WINDOW_WIDTH - width) / 2;
    let y = WINDOW_HEIGHT / 2 - font_size / 2 - 18;
    let padding_x = 34;
    let padding_y = 18;

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
    draw.draw_text(label, x + 4, y + 4, font_size, Color::new(0, 0, 0, 190));
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

fn draw_help(draw: &mut impl DrawTarget) {
    draw.draw_text(
        "P1: A/D/W/S/Q or Pad LS/DPad, A jump, LB/LT block",
        24,
        WINDOW_HEIGHT - 124,
        15,
        UI_TEXT,
    );
    draw.draw_text(
        "P1 attacks: F LP, H HP, V kick, G special or Pad X/Y/B/RB",
        24,
        WINDOW_HEIGHT - 100,
        15,
        UI_TEXT,
    );
    draw.draw_text(
        "P1 mods: S+V sweep, S+H anti-air, forward+H overhead, Q+F throw, air F/V",
        24,
        WINDOW_HEIGHT - 76,
        15,
        UI_TEXT,
    );
    draw.draw_text(
        "P2: CPU default; C or View toggles P2 manual",
        24,
        WINDOW_HEIGHT - 52,
        15,
        UI_TEXT,
    );
    draw.draw_text(
        "P2 manual: keyboard or second Pad same layout; Start/R restarts; F9/F10 records",
        24,
        WINDOW_HEIGHT - 28,
        15,
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
    x: i32,
    y: i32,
    health: i32,
    max_health: i32,
    label: &str,
) {
    let width = 300;
    let height = 18;
    let max_health = max_health.max(1);
    let ratio = health.max(0) as f32 / max_health as f32;
    let fill_width = (width as f32 * ratio.clamp(0.0, 1.0)).round() as i32;
    let fill = if health * 4 <= max_health {
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
    draw: &mut impl DrawTarget,
    world: &World,
    show_debug: bool,
    assets: &GameAssets,
) {
    for projectile in &world.projectiles {
        let rect = projectile.rect();
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

fn fighter_atlas_for_intro<'a>(
    spawn_intro: bool,
    start_atlas: Option<&'a SpriteAtlasAsset>,
    fight_atlas: Option<&'a SpriteAtlasAsset>,
) -> Option<&'a SpriteAtlasAsset> {
    if spawn_intro {
        start_atlas.or(fight_atlas)
    } else {
        fight_atlas
    }
}

fn fighter_visual_elapsed_seconds(world: &World, spawn_intro: bool, has_start_atlas: bool) -> f32 {
    if spawn_intro && has_start_atlas {
        world.spawn_intro_elapsed_seconds()
    } else {
        world.elapsed_seconds
    }
}

fn forced_fighter_clip(
    world: &World,
    slot: PlayerSlot,
    spawn_intro: bool,
    has_start_atlas: bool,
) -> Option<sprites::FighterSpriteClip> {
    if spawn_intro && has_start_atlas {
        return Some(sprites::FighterSpriteClip::Spawn);
    }

    matches!(world.outcome, Some(MatchOutcome::Winner(winner)) if winner == slot)
        .then_some(sprites::FighterSpriteClip::Taunt)
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
        6.0,
        BODY_COLLISION,
    );
    draw.draw_text("BODY COLLISION", x - 76, top - 24, 18, BODY_COLLISION);
}

fn draw_hit_effects(draw: &mut impl DrawTarget, world: &World) {
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
    draw: &mut impl DrawTarget,
    fighter: &crate::combat::fighter::Fighter,
    color: Color,
) {
    let parts = fighter.body_parts();
    fill_rect(draw, parts.head, lighten(color, 28));
    fill_rect(draw, parts.torso, color);
    fill_rect(draw, parts.legs, dim(color, 22));
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
