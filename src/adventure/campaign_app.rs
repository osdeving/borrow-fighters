//! Presents the protagonist registry and routes to isolated chapter applications.
//!
//! System: Adventure application boundary. This selector loads only menu copy and
//! a font; chapter and fighting textures are acquired by their own selected host.

use super::{
    app::Options,
    campaign::{ChapterId, REGISTRY_PATH, Registry},
    chapter_app::CampaignExit,
    engine::typography,
};
use crate::runtime_paths;
use raylib::prelude::*;
use std::error::Error;

pub fn run_in_window(
    rl: &mut RaylibHandle,
    thread: &RaylibThread,
    options: Options,
) -> Result<CampaignExit, Box<dyn Error>> {
    match options.start.as_deref() {
        Some("chapter") => return super::chapter_app::run_in_window(rl, thread, options),
        Some("augusta") => return super::augusta_app::run_in_window(rl, thread, options),
        _ => {}
    }
    let registry = Registry::load(&runtime_paths::asset_path(REGISTRY_PATH))?;
    let font = load_font(rl, thread)?;
    let mut target = rl.load_render_texture(thread, 1280, 720)?;
    let mut selected = 0;
    let mut frames = 0;
    rl.set_target_fps(60);
    loop {
        if rl.window_should_close() {
            return Ok(CampaignExit::Closed);
        }
        let controls = menu_input(rl);
        if frames > 1 {
            navigate(&mut selected, registry.chapters.len() + 2, &controls);
            if controls.back {
                return Ok(CampaignExit::Menu);
            }
            if controls.confirm_for(registry.chapters.len() + 2) {
                if selected == registry.chapters.len() {
                    return Ok(CampaignExit::ReplayPrologue);
                }
                if selected > registry.chapters.len() {
                    return Ok(CampaignExit::Menu);
                }
                let mut chapter_options = options.clone();
                chapter_options.start = None;
                let result = match registry.chapters[selected].id {
                    ChapterId::Rust => {
                        super::chapter_app::run_in_window(rl, thread, chapter_options)
                    }
                    ChapterId::CppAugusta => {
                        super::augusta_app::run_in_window(rl, thread, chapter_options)
                    }
                }?;
                if result != CampaignExit::Menu {
                    return Ok(result);
                }
                frames = 0;
                rl.set_target_fps(60);
            }
        }
        let mut rows: Vec<String> = registry
            .chapters
            .iter()
            .map(|entry| format!("{} · {}", entry.protagonist, entry.title))
            .collect();
        rows.push(registry.prologue_label.clone());
        rows.push(registry.back_label.clone());
        let detail = registry
            .chapters
            .get(selected)
            .map_or(registry.prompt.as_str(), |entry| entry.summary.as_str());
        {
            let mut canvas = rl.begin_texture_mode(thread, &mut target);
            canvas.clear_background(Color::new(18, 22, 32, 255));
            draw_menu(
                &mut canvas,
                &font,
                &registry.title,
                &rows.iter().map(String::as_str).collect::<Vec<_>>(),
                selected,
                detail,
            );
        }
        present(rl, thread, &target);
        frames += 1;
        if options.max_frames.is_some_and(|limit| frames >= limit) {
            return Ok(CampaignExit::FrameLimit);
        }
    }
}

pub(super) fn load_font(
    rl: &mut RaylibHandle,
    thread: &RaylibThread,
) -> Result<Font, Box<dyn Error>> {
    let mut glyphs: String = (32..=591).filter_map(char::from_u32).collect();
    glyphs.push_str("—–“”‘’…←→↑↓");
    Ok(rl.load_font_ex(
        thread,
        &runtime_paths::asset_path("assets/adventure/fonts/Barlow-Regular.ttf").to_string_lossy(),
        48,
        Some(&glyphs),
    )?)
}

#[derive(Default)]
pub(super) struct MenuInput {
    pub confirm: bool,
    pub back: bool,
    pub up: bool,
    pub down: bool,
    pub hovered: Option<usize>,
    pub mouse_moved: bool,
    pub click: bool,
}

impl MenuInput {
    pub(super) fn confirm_for(&self, count: usize) -> bool {
        self.confirm || self.click && self.hovered.is_some_and(|index| index < count)
    }
}

pub(super) fn menu_input(rl: &RaylibHandle) -> MenuInput {
    use GamepadButton::*;
    use KeyboardKey::*;
    let pad = |b| rl.is_gamepad_available(0) && rl.is_gamepad_button_pressed(0, b);
    let scale = (rl.get_screen_width() as f32 / 1280.0)
        .min(rl.get_screen_height() as f32 / 720.0)
        .max(0.01);
    let mouse = rl.get_mouse_position();
    let point = Vector2::new(
        (mouse.x - (rl.get_screen_width() as f32 - 1280.0 * scale) * 0.5) / scale,
        (mouse.y - (rl.get_screen_height() as f32 - 720.0 * scale) * 0.5) / scale,
    );
    let hovered = (0..5).find(|i| {
        Rectangle::new(330.0, 250.0 + *i as f32 * 60.0, 620.0, 50.0)
            .check_collision_point_rec(point)
    });
    let click = rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT);
    MenuInput {
        confirm: rl.is_key_pressed(KEY_ENTER) || pad(GAMEPAD_BUTTON_RIGHT_FACE_DOWN),
        back: rl.is_key_pressed(KEY_ESCAPE) || pad(GAMEPAD_BUTTON_RIGHT_FACE_RIGHT),
        up: rl.is_key_pressed(KEY_UP)
            || rl.is_key_pressed(KEY_W)
            || pad(GAMEPAD_BUTTON_LEFT_FACE_UP),
        down: rl.is_key_pressed(KEY_DOWN)
            || rl.is_key_pressed(KEY_S)
            || pad(GAMEPAD_BUTTON_LEFT_FACE_DOWN),
        hovered,
        click,
        mouse_moved: click || rl.get_mouse_delta().length_sqr() > 0.0,
    }
}

pub(super) fn navigate(selected: &mut usize, count: usize, controls: &MenuInput) {
    if controls.mouse_moved
        && let Some(index) = controls.hovered.filter(|index| *index < count)
    {
        *selected = index;
    }
    if controls.up {
        *selected = (*selected + count - 1) % count;
    }
    if controls.down {
        *selected = (*selected + 1) % count;
    }
}

pub(super) fn draw_menu(
    d: &mut impl RaylibDraw,
    font: &Font,
    title: &str,
    rows: &[&str],
    selected: usize,
    detail: &str,
) {
    d.draw_rectangle(0, 0, 1280, 720, Color::new(9, 15, 23, 235));
    typography::paragraph(
        d,
        font,
        title,
        Rectangle::new(170.0, 95.0, 940.0, 85.0),
        46.0,
        Color::new(237, 192, 107, 255),
    );
    for (index, row) in rows.iter().enumerate() {
        let y = 250 + index as i32 * 60;
        d.draw_rectangle(
            330,
            y,
            620,
            50,
            if index == selected {
                Color::new(62, 80, 100, 255)
            } else {
                Color::new(29, 38, 52, 255)
            },
        );
        typography::paragraph(
            d,
            font,
            row,
            Rectangle::new(351.0, y as f32 + 10.0, 578.0, 33.0),
            26.0,
            Color::RAYWHITE,
        );
    }
    typography::paragraph(
        d,
        font,
        detail,
        Rectangle::new(200.0, 590.0, 880.0, 88.0),
        24.0,
        Color::new(201, 213, 223, 255),
    );
}

pub(super) fn present(rl: &mut RaylibHandle, thread: &RaylibThread, target: &RenderTexture2D) {
    let width = rl.get_screen_width() as f32;
    let height = rl.get_screen_height() as f32;
    let scale = (width / 1280.0).min(height / 720.0);
    let mut screen = rl.begin_drawing(thread);
    screen.clear_background(Color::BLACK);
    screen.draw_texture_pro(
        target.texture(),
        Rectangle::new(0.0, 0.0, 1280.0, -720.0),
        Rectangle::new(
            (width - 1280.0 * scale) * 0.5,
            (height - 720.0 * scale) * 0.5,
            1280.0 * scale,
            720.0 * scale,
        ),
        Vector2::zero(),
        0.0,
        Color::WHITE,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_chapter_menu_cycles_its_real_rows_and_ignores_empty_mouse_rows() {
        let count = 1 + 2; // One implemented chapter, prologue and return.
        let mut selected = 0;
        navigate(
            &mut selected,
            count,
            &MenuInput {
                up: true,
                ..MenuInput::default()
            },
        );
        assert_eq!(selected, 2);
        navigate(
            &mut selected,
            count,
            &MenuInput {
                down: true,
                ..MenuInput::default()
            },
        );
        assert_eq!(selected, 0);
        let outside = MenuInput {
            click: true,
            mouse_moved: true,
            hovered: Some(3),
            ..MenuInput::default()
        };
        navigate(&mut selected, count, &outside);
        assert_eq!(selected, 0);
        assert!(!outside.confirm_for(count));
        assert!(
            MenuInput {
                click: true,
                hovered: Some(2),
                ..MenuInput::default()
            }
            .confirm_for(count)
        );
    }
}
