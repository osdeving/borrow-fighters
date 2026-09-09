//! Draws the welcome guide using the shared menu typography and pointer layout.
//!
//! System: Raylib presentation. The guide explains current inputs and offers
//! existing control assignments without owning combat or persistence.

use super::*;

pub(super) fn draw_guide(
    draw: &mut impl DrawTarget,
    font: Option<&Font>,
    options: &PreferencesDrawOptions<'_>,
) {
    let geometry = MenuLayout::for_page(MenuPage::HowToPlay);
    let panel = geometry.panel;
    draw_menu_panel(draw, panel);
    draw_menu_page_title(draw, font, panel, "COMO JOGAR · BEM-VINDO AO PLAYTEST");
    draw_centered_menu_text(
        draw,
        font,
        "Escolha o modo, confirme P1 e P2 e selecione LUTAR. Espere o Fight! e zere a vida rival.",
        WINDOW_WIDTH / 2,
        screen_px(91),
        13.0,
        UI_TEXT,
    );

    let cards = [
        (
            "P1 · TECLADO",
            MENU_HACK_GREEN,
            [
                "A / D  mover   ·   W  pular   ·   S  abaixar",
                "Q  defender (segure)",
                "F  soco fraco   ·   H  soco forte",
                "V  chute   ·   G  projétil",
                "T  especial de assinatura",
                "Y  cinematográfico (100 de energia)",
                "S + V  rasteira   ·   S + H  anti-air",
                "Frente + H  overhead   ·   Q + F  agarrão",
                "No ar: F / V para atacar",
            ],
        ),
        (
            "P2 · TECLADO LOCAL",
            MENU_ACCENT,
            [
                "Setas  mover, pular e abaixar",
                "U  defender (segure)",
                "O  soco fraco   ·   P  soco forte",
                "; ou /  chute   ·   Ctrl direito  projétil",
                "\\  especial de assinatura",
                "]  cinematográfico (100 de energia)",
                "Baixo + chute  rasteira",
                "Frente + P  overhead   ·   U + O  agarrão",
                "Baixo + P  anti-air   ·   No ar: O / chute",
            ],
        ),
        (
            "CONTROLE · BOTÕES XBOX",
            PLAYER_TWO,
            [
                "Direcional / analógico  mover e abaixar",
                "A  pular   ·   LB / LT  defender",
                "X  soco fraco   ·   Y  soco forte",
                "B  chute   ·   RB  projétil",
                "RT  especial   ·   LB + RT  cinematográfico",
                "Baixo + B  rasteira   ·   Baixo + Y  anti-air",
                "Frente + Y  overhead   ·   LB + X  agarrão",
                "1º controle: P1   ·   2º controle: P2",
                "Menu / Start  pausa   ·   View  CPU P2",
            ],
        ),
    ];
    for (index, (title, accent, lines)) in cards.iter().enumerate() {
        let x = screen_px(53 + index as i32 * 286);
        draw.draw_rectangle(
            x,
            screen_px(118),
            screen_px(280),
            screen_px(181),
            MENU_PANEL_STRONG,
        );
        draw.draw_rectangle(x, screen_px(118), screen_px(280), screen_px(2), *accent);
        draw_menu_text(
            draw,
            font,
            title,
            x + screen_px(10),
            screen_px(132),
            14.0,
            *accent,
        );
        for (row, line) in lines.iter().enumerate() {
            draw_menu_text(
                draw,
                font,
                line,
                x + screen_px(10),
                screen_px(159 + row as i32 * 15),
                10.5,
                UI_TEXT,
            );
        }
    }
    for (text, y) in [
        (
            "Seleção: P1 WASD / F · P2 setas / Enter · mouse / controle · Tab muda modo · Q / E muda arena.",
            309,
        ),
        (
            "Energia começa em 50/100. Acerte, receba golpes ou defenda para encher; cinematográfico custa 100.",
            323,
        ),
        (
            "Defenda em pé contra overheads; baixo + defesa contra rasteiras. Em Training, os especiais são livres.",
            337,
        ),
        (
            "Esc / Start pausa · R reinicia · fim da luta: Revanche, Trocar personagens ou Menu.",
            351,
        ),
    ] {
        draw_centered_menu_text(
            draw,
            font,
            text,
            WINDOW_WIDTH / 2,
            screen_px(y),
            10.5,
            UI_TEXT,
        );
    }

    let rows = [
        MenuLine {
            label: "JOGAR CONTRA CPU · VOCÊ É P1",
            description: "",
            value: None,
            checked: None,
        },
        MenuLine {
            label: "DUELO LOCAL · DOIS JOGADORES",
            description: "",
            value: None,
            checked: None,
        },
        MenuLine {
            label: "ASSISTIR DEMO · CPU CONTRA CPU",
            description: "",
            value: None,
            checked: None,
        },
        MenuLine {
            label: "IR AO MENU PRINCIPAL",
            description: "",
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
            show_descriptions: false,
            selection_pulse_frames: options.menu.selection_pulse_frames(),
        },
    );
    draw_menu_footer(
        draw,
        font,
        panel,
        "Setas / W S / D-pad: navegar · Enter / Espaço / A: confirmar · Mouse: clicar",
    );
}
