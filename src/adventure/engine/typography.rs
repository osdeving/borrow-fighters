//! Fits editable narrative text into explicit presentation boxes.
//!
//! System: Adventure typography. UTF-8 words wrap using actual font metrics;
//! unusually long edits are contained without covering adjacent scene content.

use raylib::prelude::*;

/// Draws wrapped copy, reducing size to fit and containing excessive author edits.
pub fn paragraph(
    d: &mut impl RaylibDraw,
    font: &Font,
    value: &str,
    area: Rectangle,
    maximum: f32,
    color: Color,
) {
    let mut size = maximum;
    let mut lines;
    loop {
        lines = wrap(value, area.width, |s| font.measure_text(s, size, 0.2).x);
        if lines.len() as f32 * size * 1.22 <= area.height || size <= 15.0 {
            break;
        }
        size -= 1.0;
    }
    let visible = (area.height / (size * 1.22)).floor().max(1.0) as usize;
    if lines.len() > visible {
        lines.truncate(visible);
        let last = lines.last_mut().expect("at least one visible line");
        while !last.is_empty() && font.measure_text(&format!("{last}…"), size, 0.2).x > area.width
        {
            last.pop();
        }
        last.push('…');
    }
    for (i, line) in lines.iter().enumerate() {
        d.draw_text_ex(
            font,
            line,
            Vector2::new(area.x, area.y + i as f32 * size * 1.22),
            size,
            0.2,
            color,
        );
    }
}

/// Draws one centered line, fitting its actual glyph width to the available space.
pub fn centered(
    d: &mut impl RaylibDraw,
    font: &Font,
    value: &str,
    center: Vector2,
    width: f32,
    maximum: f32,
    color: Color,
) {
    let measured = font.measure_text(value, maximum, 0.2).x.max(1.0);
    let size = maximum * (width / measured).min(1.0);
    let measured = font.measure_text(value, size, 0.2);
    d.draw_text_ex(
        font,
        value,
        Vector2::new(center.x - measured.x * 0.5, center.y),
        size,
        0.2,
        color,
    );
}

fn wrap(value: &str, width: f32, measure: impl Fn(&str) -> f32) -> Vec<String> {
    let mut output = Vec::new();
    for paragraph in value.split('\n') {
        let mut line = String::new();
        for word in paragraph.split_whitespace() {
            let candidate = if line.is_empty() {
                word.to_string()
            } else {
                format!("{line} {word}")
            };
            if measure(&candidate) <= width {
                line = candidate;
                continue;
            }
            if !line.is_empty() {
                output.push(std::mem::take(&mut line));
            }
            for character in word.chars() {
                let candidate = format!("{line}{character}");
                if !line.is_empty() && measure(&candidate) > width {
                    output.push(std::mem::take(&mut line));
                }
                line.push(character);
            }
        }
        output.push(line);
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn edited_utf8_wraps_without_splitting_bytes_and_preserves_paragraphs() {
        let lines = wrap("São Paulo\n\né amanhã", 6.0, |v| {
            v.chars().count() as f32
        });
        assert_eq!(lines, ["São", "Paulo", "", "é", "amanhã"]);
        assert_eq!(
            wrap("áéíóúç", 3.0, |v| v.chars().count() as f32),
            ["áéí", "óúç"]
        );
    }
}
