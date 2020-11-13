use skia_safe::Color;

pub(crate) fn parse_color(value: &str) -> Option<Color> {
    match value.parse::<css_color_parser::Color>() {
        Ok(color) => Some(Color::from_argb(
            (color.a * 255.0) as u8,
            color.r,
            color.g,
            color.b,
        )),
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

pub(crate) fn to_parsed_color(color: Color) -> String {
    if color.a() == 255 {
        format!(
            "#{}{}{}",
            to_hex(color.r()),
            to_hex(color.g()),
            to_hex(color.b())
        )
    } else {
        format!(
            "rgba({},{},{},{})",
            color.r(),
            color.g(),
            color.b(),
            (color.a() as f32 / 255.0)
        )
    }
}
