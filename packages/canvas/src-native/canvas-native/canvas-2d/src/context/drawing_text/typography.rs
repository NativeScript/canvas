use std::collections::VecDeque;

use skia_safe::{
    font_style::{Slant, Weight, Width},
    FontMetrics,
    FontMgr, FontStyle, typeface::Typeface,
};

use crate::context::{
    Device, text_styles::text_align::TextAlign,
    text_styles::text_baseline::TextBaseLine, text_styles::text_direction::TextDirection,
};
use crate::utils::dimensions::parse_size;

const XX_SMALL: &str = "9px";
const X_SMALL: &str = "10px";
const SMALL: &str = "13px";
const MEDIUM: &str = "16px";
const LARGE: &str = "18px";
const X_LARGE: &str = "24px";
const XX_LARGE: &str = "32px";

pub(crate) fn get_font_baseline(metrics: FontMetrics, baseline: TextBaseLine) -> f32 {
    match baseline {
        TextBaseLine::TOP => metrics.ascent,
        TextBaseLine::HANGING => {
            // According to http://wiki.apache.org/xmlgraphics-fop/LineLayout/AlignmentHandling
            // "FOP (Formatting Objects Processor) puts the hanging baseline at 80% of the ascender height"
            (metrics.ascent * 4.0) / 5.0
        }
        TextBaseLine::MIDDLE => -metrics.descent + metrics.cap_height / 2.0,
        TextBaseLine::IDEOGRAPHIC | TextBaseLine::BOTTOM => -metrics.descent,
        TextBaseLine::ALPHABETIC => 0.0,
    }
}

pub(crate) fn to_text_align(align: &str) -> Option<TextAlign> {
    match align {
        "start" => Some(TextAlign::START),
        "left" => Some(TextAlign::LEFT),
        "center" => Some(TextAlign::CENTER),
        "end" => Some(TextAlign::END),
        "right" => Some(TextAlign::RIGHT),
        _ => None,
    }
}

pub(crate) fn to_real_text_align(align: TextAlign, direction: TextDirection) -> TextAlign {
    match align {
        TextAlign::START => {
            if direction == TextDirection::RTL {
                TextAlign::RIGHT
            } else {
                TextAlign::LEFT
            }
        }
        TextAlign::END => {
            if direction == TextDirection::RTL {
                TextAlign::LEFT
            } else {
                TextAlign::RIGHT
            }
        }
        TextAlign::RIGHT | TextAlign::CENTER | TextAlign::LEFT => align,
    }
}

#[derive(Debug, Clone)]
pub struct Font {
    pub(crate) font_details: String,
    pub(crate) font: ParsedFont,
    pub(crate) device: Device,
}

impl Font {
    pub fn new(font_details: &str, device: Device) -> Self {
        Self {
            font_details: font_details.to_string(),
            font: parse_font(font_details),
            device,
        }
    }

    pub fn get_font_details(&self) -> &str {
        self.font_details.as_ref()
    }

    pub fn get_font(&self) -> &ParsedFont {
        &self.font
    }

    pub fn set_font(&mut self, font_details: &str) {
        if font_details.is_empty() {
            return;
        }

        self.font_details = font_details.to_string();
        self.font = parse_font(font_details);
    }

    fn to_font(&self) -> skia_safe::Font {
        let style = to_font_style(self.font.font_weight(), self.font.font_style());
        let families: Vec<String> = parse_font_family(self.font.font_family());
        let mut default_typeface =
            Typeface::from_name("sans-serif", style).unwrap_or(Typeface::default());
        let mgr = FontMgr::default();
        let families_count = mgr.count_families();
        for i in 0..families_count {
            let name = mgr.family_name(i);
            if families.contains(&name) {
                if let Some(typeface) = mgr.match_family_style(&name, style) {
                    default_typeface = typeface;
                    break;
                }
            }
        }

        skia_safe::Font::from_typeface(
            default_typeface,
            Some(parse_size(self.font.font_size(), self.device)),
        )
    }

    pub fn to_skia(&self) -> skia_safe::Font {
        self.to_font()
    }
}

#[derive(Clone, Debug)]
pub struct ParsedFont {
    font_style: ParsedFontStyle,
    font_variant: String,
    font_weight: ParsedFontWeight,
    line_height: Option<String>,
    font_size: Option<String>,
    font_family: Option<String>,
}

impl ParsedFont {
    pub fn font_style(&self) -> ParsedFontStyle {
        self.font_style
    }
    pub fn font_variant(&self) -> &str {
        &self.font_variant
    }
    pub fn font_weight(&self) -> ParsedFontWeight {
        self.font_weight
    }
    pub fn line_height(&self) -> &str {
        self.line_height.as_ref().map_or("1.4px", |v| v)
    }
    pub fn font_size(&self) -> &str {
        self.font_size.as_ref().map_or("10px", |v| v)
    }
    pub fn font_family(&self) -> &str {
        self.font_family.as_ref().map_or("sans-serif", |v| v)
    }
}

impl Default for ParsedFont {
    fn default() -> Self {
        Self {
            font_style: ParsedFontStyle::Normal,
            font_variant: "normal".to_string(),
            font_weight: ParsedFontWeight::Normal,
            line_height: None,
            font_size: None,
            font_family: None,
        }
    }
}

const NORMAL: &str = "normal";
const ITALIC: &str = "italic";
const OBLIQUE: &str = "oblique";

#[derive(Copy, Clone, Debug)]
pub enum ParsedFontStyle {
    Normal,
    Italic,
    Oblique,
}

impl ParsedFontStyle {
    pub fn value(&self) -> &str {
        match self {
            ParsedFontStyle::Normal => NORMAL,
            ParsedFontStyle::Italic => ITALIC,
            ParsedFontStyle::Oblique => OBLIQUE,
        }
    }

    pub fn is_supported(value: &str) -> bool {
        NORMAL == value || ITALIC == value || OBLIQUE == value
    }

    pub fn into_skia(&self) -> Slant {
        match self {
            ParsedFontStyle::Italic => Slant::Italic,
            ParsedFontStyle::Oblique => Slant::Oblique,
            _ => Slant::Upright,
        }
    }
}

#[repr(u32)]
#[derive(Copy, Clone, Debug)]
pub enum ParsedFontWeight {
    Thin = 100,
    ExtraLight = 200,
    Light = 300,
    Normal = 400,
    Medium = 500,
    SemiBold = 600,
    Bold = 700,
    ExtraBold = 800,
    Black = 900,
}

impl ParsedFontWeight {
    pub fn is_supported(value: &str) -> bool {
        match value {
            "100" | "200" | "300" | "400" | "500" | "600" | "700" | "800" | "900" | "lighter"
            | "bold" | "bolder" | "normal" => true,
            _ => false,
        }
    }

    // todo parse relative values
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "100" => Option::from(ParsedFontWeight::Thin),
            "200" => Option::from(ParsedFontWeight::ExtraLight),
            "300" => Option::from(ParsedFontWeight::Light),
            "400" | "normal" => Option::from(ParsedFontWeight::Normal),
            "500" => Option::from(ParsedFontWeight::Medium),
            "600" => Option::from(ParsedFontWeight::SemiBold),
            "700" | "bold" => Option::from(ParsedFontWeight::Bold),
            "800" => Option::from(ParsedFontWeight::ExtraBold),
            "900" => Option::from(ParsedFontWeight::Black),
            _ => None,
        }
    }

    pub fn into_skia(&self) -> Weight {
        (*self).into()
    }
}

impl Into<Weight> for ParsedFontWeight {
    fn into(self) -> Weight {
        Weight::from(*&self as i32)
    }
}

pub fn parse_font(font: &str) -> ParsedFont {
    let pat = regex::Regex::new(r"\s+").unwrap();
    let res = pat.split(font);

    let mut parsed_font = ParsedFont::default();

    let mut res: VecDeque<_> = res.collect();

    for _ in 0..res.len() {
        if let Some(part) = res.pop_front() {
            if part.eq("normal") {} else if part.eq("small-caps") {
                parsed_font.font_variant = part.into();
            } else if ParsedFontStyle::is_supported(part) {
                match part {
                    NORMAL => {
                        parsed_font.font_style = ParsedFontStyle::Normal;
                    }
                    ITALIC => {
                        parsed_font.font_style = ParsedFontStyle::Italic;
                    }
                    OBLIQUE => {
                        parsed_font.font_style = ParsedFontStyle::Oblique;
                    }
                    _ => {}
                }
            } else if ParsedFontWeight::is_supported(part) {
                parsed_font.font_weight =
                    ParsedFontWeight::parse(part).unwrap_or(ParsedFontWeight::Normal);
            } else if parsed_font.font_size.is_none() {
                let sizes = part.split('/');
                for (j, size) in sizes.enumerate() {
                    if j == 0 {
                        parsed_font.font_size = Some(size.into());
                    }

                    if j == 1 {
                        parsed_font.line_height = Some(size.into());
                    }
                }
            } else {
                parsed_font.font_family = Some(part.into());
                if !res.is_empty() {
                    let mut current = parsed_font.font_family.unwrap_or_default();
                    for item in res.iter() {
                        current.push_str(&format!(" {}", *item));
                    }
                    parsed_font.font_family = Some(current);
                }
                break;
            }
        }
    }

    parsed_font
}

fn parse_font_family(value: &str) -> Vec<String> {
    let mut result = Vec::new();
    if value.is_empty() {
        return result;
    }

    let regex = regex::Regex::new(r#"/['"]+/g"#).unwrap();

    let split = value.split(',');
    for item in split {
        let s = regex.replace(item.trim(), "");
        if !s.is_empty() {
            result.push(s.to_string());
        }
    }

    return result;
}

fn is_size_length(value: &str) -> bool {
    return value.contains("pc")
        || value.contains("pt")
        || value.contains("vh")
        || value.contains("vw")
        || value.contains("px")
        || value.contains("em")
        || value.contains("cm")
        || value.contains("%");
}

fn to_font_style(weight: ParsedFontWeight, style: ParsedFontStyle) -> FontStyle {
    FontStyle::new(weight.into_skia(), Width::NORMAL, style.into_skia())
}
