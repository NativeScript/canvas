use skia_safe::Color;

pub fn parse_color(value: &str) -> Option<Color> {
    match value.parse::<csscolorparser::Color>() {
        Ok(color) => {
            let color = color.rgba_u8();
            Some(Color::from_argb(
            color.3,
            color.0,
            color.1,
            color.2,
        ))},
        _ => None,
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
