use skia_safe::{Color, Color4f};

pub fn parse_color(value: &str) -> Option<Color> {
    match value.parse::<csscolorparser::Color>() {
        Ok(color) => {
            let color = color.to_rgba8();
            Some(Color::from_argb(color[3], color[0], color[1], color[2]))
        }
        _ => None,
    }
}

pub fn parse_color_rgba(value: &str, r: &mut u8, g: &mut u8, b: &mut u8, a: &mut u8) -> bool {
    match value.parse::<csscolorparser::Color>() {
        Ok(color) => {
            let color = color.to_rgba8();
            *r = color[0];
            *g = color[1];
            *b = color[2];
            *a = color[3];
            true
        }
        _ => false,
    }
}

fn to_hex(value: u8) -> String {
    let mut hex = format!("{:x}", value);
    if hex.len() == 1 {
        hex = format!("0{}", hex);
    }
    hex
}

pub fn to_parsed_color(color: Color) -> String {
    if color.a() == 255 {
        format!(
            "#{}{}{}",
            to_hex(color.r()),
            to_hex(color.g()),
            to_hex(color.b())
        )
    } else {
        let alpha = color.a() as f32 / 255.0;
        let alpha = (alpha * 100.0).round() / 100.0;
        let alpha = format!("{:.15}", alpha);
        let alpha = alpha.trim_end_matches('0');
        format!(
            "rgba({}, {}, {}, {})",
            color.r(),
            color.g(),
            color.b(),
            if alpha == "0." { "0" } else { alpha }
        )
    }
}

pub fn to_parsed_color_4f(color: Color4f) -> String {
    let r = (color.r * 255.0).clamp(0.0, 255.0) as u8;
    let g = (color.g * 255.0).clamp(0.0, 255.0) as u8;
    let b = (color.b * 255.0).clamp(0.0, 255.0) as u8;
    if color.is_opaque() {
        format!("#{}{}{}", to_hex(r), to_hex(g), to_hex(b))
    } else {
        let alpha = format!("{:.15}", color.a);
        let alpha = alpha.trim_end_matches('0');
        format!(
            "rgba({}, {}, {}, {})",
            r,
            g,
            b,
            if alpha == "0." { "0" } else { alpha }
        )
    }
}
