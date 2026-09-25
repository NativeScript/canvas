//! Colour parsing for interpolation; Skia parses colours internally but exposes no values.

/// Straight (non-premultiplied) RGBA, 0-255 per channel.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Rgba {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

impl Rgba {
    fn new(r: f64, g: f64, b: f64, a: f64) -> Self {
        Self { r, g, b, a }
    }

    pub fn lerp(self, other: Self, t: f64) -> Self {
        let mix = |a: f64, b: f64| a + (b - a) * t;
        Self::new(
            mix(self.r, other.r),
            mix(self.g, other.g),
            mix(self.b, other.b),
            mix(self.a, other.a),
        )
    }

    pub fn to_css(self) -> String {
        let clamp = |v: f64| v.clamp(0.0, 255.0).round() as u8;
        if self.a >= 1.0 {
            format!("rgb({},{},{})", clamp(self.r), clamp(self.g), clamp(self.b))
        } else {
            format!(
                "rgba({},{},{},{})",
                clamp(self.r),
                clamp(self.g),
                clamp(self.b),
                (self.a.clamp(0.0, 1.0) * 1000.0).round() / 1000.0
            )
        }
    }
}

/// `None` for `none`, `currentColor`, paint servers etc., which then animate discretely.
pub fn parse(input: &str) -> Option<Rgba> {
    let value = input.trim();
    if value.is_empty() {
        return None;
    }

    if let Some(hex) = value.strip_prefix('#') {
        return parse_hex(hex);
    }

    let lower = value.to_ascii_lowercase();
    if let Some(args) = lower
        .strip_prefix("rgba(")
        .or_else(|| lower.strip_prefix("rgb("))
        .and_then(|rest| rest.strip_suffix(')'))
    {
        return parse_rgb_args(args);
    }

    named(&lower)
}

fn parse_hex(hex: &str) -> Option<Rgba> {
    let digits: Vec<u32> = hex.chars().map(|c| c.to_digit(16)).collect::<Option<_>>()?;
    let (r, g, b, a) = match digits.len() {
        3 | 4 => {
            let expand = |d: u32| (d * 16 + d) as f64;
            (
                expand(digits[0]),
                expand(digits[1]),
                expand(digits[2]),
                digits.get(3).map_or(255.0, |d| expand(*d)),
            )
        }
        6 | 8 => {
            let byte = |i: usize| (digits[i] * 16 + digits[i + 1]) as f64;
            (
                byte(0),
                byte(2),
                byte(4),
                if digits.len() == 8 { byte(6) } else { 255.0 },
            )
        }
        _ => return None,
    };
    Some(Rgba::new(r, g, b, a / 255.0))
}

fn parse_rgb_args(args: &str) -> Option<Rgba> {
    let parts: Vec<&str> = args
        .split(|c| c == ',' || c == '/' || c == ' ')
        .map(str::trim)
        .filter(|p| !p.is_empty())
        .collect();
    if parts.len() < 3 {
        return None;
    }

    let channel = |part: &str| -> Option<f64> {
        match part.strip_suffix('%') {
            Some(pct) => pct.parse::<f64>().ok().map(|v| v * 255.0 / 100.0),
            None => part.parse::<f64>().ok(),
        }
    };
    let alpha = |part: &str| -> Option<f64> {
        match part.strip_suffix('%') {
            Some(pct) => pct.parse::<f64>().ok().map(|v| v / 100.0),
            None => part.parse::<f64>().ok(),
        }
    };

    Some(Rgba::new(
        channel(parts[0])?,
        channel(parts[1])?,
        channel(parts[2])?,
        parts.get(3).and_then(|p| alpha(p)).unwrap_or(1.0),
    ))
}

fn named(name: &str) -> Option<Rgba> {
    let hex = match name {
        "transparent" => return Some(Rgba::new(0.0, 0.0, 0.0, 0.0)),
        "aliceblue" => 0xf0f8ff,
        "antiquewhite" => 0xfaebd7,
        "aqua" => 0x00ffff,
        "aquamarine" => 0x7fffd4,
        "azure" => 0xf0ffff,
        "beige" => 0xf5f5dc,
        "bisque" => 0xffe4c4,
        "black" => 0x000000,
        "blanchedalmond" => 0xffebcd,
        "blue" => 0x0000ff,
        "blueviolet" => 0x8a2be2,
        "brown" => 0xa52a2a,
        "burlywood" => 0xdeb887,
        "cadetblue" => 0x5f9ea0,
        "chartreuse" => 0x7fff00,
        "chocolate" => 0xd2691e,
        "coral" => 0xff7f50,
        "cornflowerblue" => 0x6495ed,
        "cornsilk" => 0xfff8dc,
        "crimson" => 0xdc143c,
        "cyan" => 0x00ffff,
        "darkblue" => 0x00008b,
        "darkcyan" => 0x008b8b,
        "darkgoldenrod" => 0xb8860b,
        "darkgray" | "darkgrey" => 0xa9a9a9,
        "darkgreen" => 0x006400,
        "darkkhaki" => 0xbdb76b,
        "darkmagenta" => 0x8b008b,
        "darkolivegreen" => 0x556b2f,
        "darkorange" => 0xff8c00,
        "darkorchid" => 0x9932cc,
        "darkred" => 0x8b0000,
        "darksalmon" => 0xe9967a,
        "darkseagreen" => 0x8fbc8f,
        "darkslateblue" => 0x483d8b,
        "darkslategray" | "darkslategrey" => 0x2f4f4f,
        "darkturquoise" => 0x00ced1,
        "darkviolet" => 0x9400d3,
        "deeppink" => 0xff1493,
        "deepskyblue" => 0x00bfff,
        "dimgray" | "dimgrey" => 0x696969,
        "dodgerblue" => 0x1e90ff,
        "firebrick" => 0xb22222,
        "floralwhite" => 0xfffaf0,
        "forestgreen" => 0x228b22,
        "fuchsia" => 0xff00ff,
        "gainsboro" => 0xdcdcdc,
        "ghostwhite" => 0xf8f8ff,
        "gold" => 0xffd700,
        "goldenrod" => 0xdaa520,
        "gray" | "grey" => 0x808080,
        "green" => 0x008000,
        "greenyellow" => 0xadff2f,
        "honeydew" => 0xf0fff0,
        "hotpink" => 0xff69b4,
        "indianred" => 0xcd5c5c,
        "indigo" => 0x4b0082,
        "ivory" => 0xfffff0,
        "khaki" => 0xf0e68c,
        "lavender" => 0xe6e6fa,
        "lavenderblush" => 0xfff0f5,
        "lawngreen" => 0x7cfc00,
        "lemonchiffon" => 0xfffacd,
        "lightblue" => 0xadd8e6,
        "lightcoral" => 0xf08080,
        "lightcyan" => 0xe0ffff,
        "lightgoldenrodyellow" => 0xfafad2,
        "lightgray" | "lightgrey" => 0xd3d3d3,
        "lightgreen" => 0x90ee90,
        "lightpink" => 0xffb6c1,
        "lightsalmon" => 0xffa07a,
        "lightseagreen" => 0x20b2aa,
        "lightskyblue" => 0x87cefa,
        "lightslategray" | "lightslategrey" => 0x778899,
        "lightsteelblue" => 0xb0c4de,
        "lightyellow" => 0xffffe0,
        "lime" => 0x00ff00,
        "limegreen" => 0x32cd32,
        "linen" => 0xfaf0e6,
        "magenta" => 0xff00ff,
        "maroon" => 0x800000,
        "mediumaquamarine" => 0x66cdaa,
        "mediumblue" => 0x0000cd,
        "mediumorchid" => 0xba55d3,
        "mediumpurple" => 0x9370db,
        "mediumseagreen" => 0x3cb371,
        "mediumslateblue" => 0x7b68ee,
        "mediumspringgreen" => 0x00fa9a,
        "mediumturquoise" => 0x48d1cc,
        "mediumvioletred" => 0xc71585,
        "midnightblue" => 0x191970,
        "mintcream" => 0xf5fffa,
        "mistyrose" => 0xffe4e1,
        "moccasin" => 0xffe4b5,
        "navajowhite" => 0xffdead,
        "navy" => 0x000080,
        "oldlace" => 0xfdf5e6,
        "olive" => 0x808000,
        "olivedrab" => 0x6b8e23,
        "orange" => 0xffa500,
        "orangered" => 0xff4500,
        "orchid" => 0xda70d6,
        "palegoldenrod" => 0xeee8aa,
        "palegreen" => 0x98fb98,
        "paleturquoise" => 0xafeeee,
        "palevioletred" => 0xdb7093,
        "papayawhip" => 0xffefd5,
        "peachpuff" => 0xffdab9,
        "peru" => 0xcd853f,
        "pink" => 0xffc0cb,
        "plum" => 0xdda0dd,
        "powderblue" => 0xb0e0e6,
        "purple" => 0x800080,
        "rebeccapurple" => 0x663399,
        "red" => 0xff0000,
        "rosybrown" => 0xbc8f8f,
        "royalblue" => 0x4169e1,
        "saddlebrown" => 0x8b4513,
        "salmon" => 0xfa8072,
        "sandybrown" => 0xf4a460,
        "seagreen" => 0x2e8b57,
        "seashell" => 0xfff5ee,
        "sienna" => 0xa0522d,
        "silver" => 0xc0c0c0,
        "skyblue" => 0x87ceeb,
        "slateblue" => 0x6a5acd,
        "slategray" | "slategrey" => 0x708090,
        "snow" => 0xfffafa,
        "springgreen" => 0x00ff7f,
        "steelblue" => 0x4682b4,
        "tan" => 0xd2b48c,
        "teal" => 0x008080,
        "thistle" => 0xd8bfd8,
        "tomato" => 0xff6347,
        "turquoise" => 0x40e0d0,
        "violet" => 0xee82ee,
        "wheat" => 0xf5deb3,
        "white" => 0xffffff,
        "whitesmoke" => 0xf5f5f5,
        "yellow" => 0xffff00,
        "yellowgreen" => 0x9acd32,
        _ => return None,
    };
    Some(Rgba::new(
        ((hex >> 16) & 0xff) as f64,
        ((hex >> 8) & 0xff) as f64,
        (hex & 0xff) as f64,
        1.0,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_every_hex_form() {
        assert_eq!(parse("#f00"), Some(Rgba::new(255.0, 0.0, 0.0, 1.0)));
        assert_eq!(parse("#ff0000"), Some(Rgba::new(255.0, 0.0, 0.0, 1.0)));
        assert_eq!(parse("#0f08"), parse("#00ff0088"));
    }

    #[test]
    fn parses_functional_and_named() {
        assert_eq!(parse("rgb(1, 2, 3)"), Some(Rgba::new(1.0, 2.0, 3.0, 1.0)));
        assert_eq!(parse("rgb(100%,0%,0%)"), parse("red"));
        assert_eq!(parse("REBECCAPURPLE"), parse("#663399"));
        assert_eq!(parse("none"), None);
        assert_eq!(parse("url(#grad)"), None);
    }

    #[test]
    fn midpoint_is_halfway_on_every_channel() {
        let black = parse("black").unwrap();
        let white = parse("white").unwrap();
        assert_eq!(black.lerp(white, 0.5).to_css(), "rgb(128,128,128)");
    }
}
