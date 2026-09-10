//! Draws the socketed handset and its readable, generic messenger enlargement.
//!
//! System: Chapter presentation. The panel samples the same phone clock as Rust's
//! gestures; its editable skin contains no network client or application branding.

use crate::adventure::chapter::phone::{PhonePhase, PhoneView};
use crate::adventure::engine::{assets::Assets, typography};
use raylib::prelude::*;
use serde::Deserialize;

/// Editable messenger appearance, independent from the handset and body sprites.
#[derive(Deserialize)]
pub struct PhoneSkin {
    pub contact: String,
    pub online: String,
    pub typing: String,
    pub placeholder: String,
    pub timestamp: String,
    pub header: [u8; 3],
    pub background: [u8; 3],
    pub outgoing: [u8; 3],
    pub incoming: [u8; 3],
}

fn color([r, g, b]: [u8; 3], alpha: u8) -> Color {
    Color::new(r, g, b, alpha)
}

/// Draws the independently replaceable handset at its pose's named hand socket.
pub fn handset(d: &mut impl RaylibDraw, hand: Vector2, scale: f32, angle: f32) {
    let body = Rectangle::new(hand.x, hand.y, 13.0 * scale, 24.0 * scale);
    d.draw_rectangle_pro(
        body,
        Vector2::new(6.5 * scale, 20.6 * scale),
        angle,
        Color::new(27, 32, 34, 255),
    );
    let screen = Rectangle::new(hand.x, hand.y, 9.0 * scale, 18.0 * scale);
    d.draw_rectangle_pro(
        screen,
        Vector2::new(4.5 * scale, 17.6 * scale),
        angle,
        Color::new(128, 181, 160, 255),
    );
}

/// Keeps the world visible beside a panel anchored to Rust's on-screen position.
pub fn draw(
    d: &mut impl RaylibDraw,
    a: &Assets,
    skin: &PhoneSkin,
    view: PhoneView,
    messages: [&str; 3],
    actor_screen: Vector2,
) {
    if !view.panel_visible {
        return;
    }
    let fade = ((view.ticks.saturating_sub(90)) as f32 / 18.0)
        .min(1.0)
        .min((810_u32.saturating_sub(view.ticks)) as f32 / 18.0);
    let opacity = (255.0 * fade) as u8;
    let x = if actor_screen.x < 720.0 {
        (actor_screen.x + 125.0).clamp(640.0, 845.0)
    } else {
        (actor_screen.x - 510.0).clamp(35.0, 740.0)
    };
    let y = 105.0 + (1.0 - fade) * 12.0;
    let width = 398.0;
    let height = 472.0;
    let ink = Color::new(34, 48, 43, opacity);
    d.draw_rectangle_rounded(
        Rectangle::new(x + 7.0, y + 9.0, width, height),
        0.065,
        12,
        Color::new(5, 16, 14, (fade * 80.0) as u8),
    );
    d.draw_rectangle_rounded(
        Rectangle::new(x, y, width, height),
        0.065,
        12,
        color(skin.background, opacity),
    );
    d.draw_rectangle_rounded(
        Rectangle::new(x, y, width, 80.0),
        0.12,
        12,
        color(skin.header, opacity),
    );
    d.draw_rectangle(
        (x) as i32,
        (y + 40.0) as i32,
        width as i32,
        40,
        color(skin.header, opacity),
    );
    d.draw_line_ex(
        Vector2::new(x + 20.0, y + 30.0),
        Vector2::new(x + 12.0, y + 39.0),
        2.0,
        Color::new(238, 247, 241, opacity),
    );
    d.draw_line_ex(
        Vector2::new(x + 12.0, y + 39.0),
        Vector2::new(x + 20.0, y + 48.0),
        2.0,
        Color::new(238, 247, 241, opacity),
    );
    d.draw_circle_v(
        Vector2::new(x + 53.0, y + 40.0),
        23.0,
        Color::new(223, 206, 143, opacity),
    );
    typography::centered(
        d,
        &a.signage,
        "Py",
        Vector2::new(x + 53.0, y + 24.0),
        40.0,
        28.0,
        ink,
    );
    typography::paragraph(
        d,
        &a.signage,
        &skin.contact,
        Rectangle::new(x + 89.0, y + 16.0, 258.0, 32.0),
        26.0,
        Color::new(248, 248, 237, opacity),
    );
    typography::paragraph(
        d,
        &a.body,
        if view.python_typing {
            &skin.typing
        } else {
            &skin.online
        },
        Rectangle::new(x + 89.0, y + 48.0, 258.0, 22.0),
        16.0,
        Color::new(208, 231, 215, opacity),
    );
    for i in 0..3 {
        d.draw_circle_v(
            Vector2::new(x + width - 22.0, y + 31.0 + i as f32 * 8.0),
            1.7,
            Color::new(238, 247, 241, opacity),
        );
    }
    for row in 0..12 {
        for col in 0..13 {
            d.draw_circle_v(
                Vector2::new(x + 15.0 + col as f32 * 30.0, y + 96.0 + row as f32 * 26.0),
                0.7,
                Color::new(141, 154, 135, (fade * 40.0) as u8),
            );
        }
    }
    for (index, message) in messages.iter().enumerate().take(view.message_count) {
        let outgoing = index != 1;
        let bx = x + if outgoing { 65.0 } else { 17.0 };
        let by = y + 108.0 + index as f32 * 86.0;
        let bw = 316.0;
        d.draw_rectangle_rounded(
            Rectangle::new(bx + 1.0, by + 2.0, bw, 72.0),
            0.12,
            8,
            Color::new(58, 67, 49, (fade * 30.0) as u8),
        );
        d.draw_rectangle_rounded(
            Rectangle::new(bx, by, bw, 72.0),
            0.12,
            8,
            color(
                if outgoing {
                    skin.outgoing
                } else {
                    skin.incoming
                },
                opacity,
            ),
        );
        typography::paragraph(
            d,
            &a.body,
            message,
            Rectangle::new(bx + 14.0, by + 12.0, bw - 28.0, 35.0),
            24.0,
            ink,
        );
        typography::paragraph(
            d,
            &a.body,
            &skin.timestamp,
            Rectangle::new(bx + bw - 66.0, by + 51.0, 42.0, 17.0),
            12.0,
            Color::new(101, 122, 109, opacity),
        );
        if outgoing {
            let check = if view.first_read || index == 2 {
                Color::new(46, 156, 192, opacity)
            } else {
                Color::new(121, 148, 133, opacity)
            };
            for n in 0..2 {
                let cx = bx + bw - 22.0 + n as f32 * 5.0;
                d.draw_line_ex(
                    Vector2::new(cx, by + 58.0),
                    Vector2::new(cx + 3.0, by + 61.0),
                    1.2,
                    check,
                );
                d.draw_line_ex(
                    Vector2::new(cx + 3.0, by + 61.0),
                    Vector2::new(cx + 9.0, by + 53.0),
                    1.2,
                    check,
                );
            }
        }
    }
    if view.python_typing {
        d.draw_rectangle_rounded(
            Rectangle::new(x + 17.0, y + 194.0, 80.0, 46.0),
            0.22,
            8,
            color(skin.incoming, opacity),
        );
        for i in 0..3 {
            let bob = ((view.phase_ticks as f32 * 0.15 - i as f32 * 0.8).sin() * 2.0).max(0.0);
            d.draw_circle_v(
                Vector2::new(x + 38.0 + i as f32 * 18.0, y + 217.0 - bob),
                3.5,
                Color::new(117, 145, 130, opacity),
            );
        }
    }
    d.draw_rectangle_rounded(
        Rectangle::new(x + 12.0, y + height - 59.0, width - 76.0, 46.0),
        0.45,
        10,
        Color::new(251, 252, 247, opacity),
    );
    let typed = view.active_message.map(|index| {
        let value = messages[index];
        value
            .chars()
            .take((value.chars().count() as f32 * view.typed_fraction).floor() as usize)
            .collect::<String>()
    });
    typography::paragraph(
        d,
        &a.body,
        typed.as_deref().unwrap_or(&skin.placeholder),
        Rectangle::new(x + 30.0, y + height - 48.0, width - 115.0, 32.0),
        21.0,
        if typed.is_some() {
            ink
        } else {
            Color::new(131, 143, 132, opacity)
        },
    );
    d.draw_circle_v(
        Vector2::new(x + width - 34.0, y + height - 36.0),
        23.0,
        color(skin.header, opacity),
    );
    let center = Vector2::new(x + width - 34.0, y + height - 36.0);
    d.draw_triangle(
        Vector2::new(center.x - 7.0, center.y - 8.0),
        Vector2::new(center.x - 7.0, center.y + 8.0),
        Vector2::new(center.x + 10.0, center.y),
        Color::new(244, 248, 236, opacity),
    );
}

/// Selects the bodily clip from the same authored clock as the messenger.
pub(super) fn body_clip(view: PhoneView) -> (&'static str, u32) {
    match view.phase {
        PhonePhase::Drawing => ("rust.phone.draw", view.phase_ticks),
        PhonePhase::TypingFirst | PhonePhase::TypingLast => ("rust.phone.type", view.phase_ticks),
        PhonePhase::Stowing => ("rust.phone.stow", view.phase_ticks),
        _ => ("rust.phone.read", view.phase_ticks),
    }
}
