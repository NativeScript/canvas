use std::os::raw::c_float;

use skia_safe::{
    BlurStyle,
    font_style::{Slant, Weight, Width},
    FontMetrics, FontMgr, FontStyle, FontStyleSet, MaskFilter, Paint, Point, Size, typeface::Typeface,
};
use skia_safe::paint::Style;

use crate::common::context::Device;
use crate::common::context::text_styles::text_align::TextAlign;
use crate::common::context::text_styles::text_baseline::TextBaseLine;
use crate::common::context::text_styles::text_direction::TextDirection;
use crate::common::utils::dimensions::parse_size;

const XX_SMALL: &'static str = "9px";
const X_SMALL: &'static str = "10px";
const SMALL: &'static str = "13px";
const MEDIUM: &'static str = "16px";
const LARGE: &'static str = "18px";
const X_LARGE: &'static str = "24px";
const XX_LARGE: &'static str = "32px";

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

#[derive(Clone)]
pub struct Font {
    pub(crate) font_details: String,
    density: c_float,
}

impl Font {
    pub fn new(font_details: &str, density: c_float) -> Self {
        Self {
            font_details: font_details.to_string(),
            density,
        }
    }

    pub fn get_font_details(&self) -> &str {
        self.font_details.as_str()
    }

    pub fn get_font(&self, device: &Device) -> skia_safe::Font {
        parse_font(self.get_font_details(), device).unwrap().1
    }
}

pub(crate) fn parse_font(font: &str, device: &Device) -> Result<(String, skia_safe::Font), ()> {
    let mut data: Vec<_> = font.split(' ').collect();
    let mut data: Vec<String> = data.into_iter().map(|x| x.trim().to_string()).collect();
    let size = data.len();
    if !data.is_empty() && size >= 2 {
        let font_size_line_height = data.get(size - 2);
        let font_families = data.last();
        let mut font_size = "10px".to_string();
        let mut style = "normal".to_string();
        let mut variant = "normal".to_string();
        let mut weight = "normal".to_string();
        let mut width = "normal".to_string();
        let mut line_height = "normal".to_string();
        let mut family = "sans-serif";

        match (font_size_line_height, font_families) {
            (Some(font_size_line_height), Some(_)) => {
                if let Ok(result) = to_size_line_height(font_size_line_height) {
                    font_size = result.0;
                    line_height = result.1;
                } else {
                    return Err(());
                }
            }
            _ => {
                return Err(());
            }
        }

        for prop in data.into_iter().take(size - 2) {
            match prop.as_str() {
                "italic" | "oblique" => style = prop,
                "small-caps" => variant = prop,
                "bold" | "bolder" | "lighter" | "thin" | "100" | "200" | "300" | "400" | "500"
                | "600" | "700" | "800" | "900" => weight = prop,
                "ultra-condensed" | "extra-condensed" | "condensed" | "semi-condensed"
                | "semi-expanded" | "expanded" | "extra-expanded" | "ultra-expanded" => {
                    width = prop
                }
                _ => {}
            }
        }

        let style = to_font_style(weight.as_str(), width.as_str(), style.as_str());
        let mut families: Vec<_> = family.split("/").collect();
        let mut families: Vec<String> = families
            .into_iter()
            .map(|x| x.trim().replace("\"", ""))
            .collect();
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
        return Ok((
            font.to_string(),
            skia_safe::Font::from_typeface(default_typeface, Some(parse_size(font_size.as_str(), device))),
        ));
    }
    Err(())
}

fn is_line_height_size_length(value: &str) -> bool {
    if is_size_length(value) {
        return true;
    }
    if let Ok(_) = value.parse::<i32>() {
        return true;
    }
    return false;
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

fn to_size_line_height(value: &str) -> Result<(String, String), ()> {
    let mut val: Vec<String> = value
        .split('/')
        .into_iter()
        .map(|x| x.to_string())
        .collect();
    val = val.into_iter().take(2).collect();
    let mut size = None;
    let mut line_height = "normal";
    if let Some(val) = val.first() {
        if is_size_length(val) {
            size = Some(val);
        } else {
            match val.as_str() {
                "medium" | "xx-small" | "x-small" | "small" | "large" | "x-large" | "xx-large"
                | "smaller" | "larger" => size = Some(val),
                _ => {}
            }
        }
    }

    if let Some(val) = val.last() {
        if is_line_height_size_length(value) {
            line_height = val
        }
    }

    if let Some(size) = size {
        return Ok((size.to_string(), line_height.to_string()));
    }

    Err(())
}

fn to_font_style(weight: &str, width: &str, slant: &str) -> FontStyle {
    FontStyle::new(to_weight(weight), to_width(width), to_slant(slant))
}

fn to_weight(value: &str) -> Weight {
    match value {
        "bold" => Weight::BOLD,
        // TODO use parent
        "lighter" => Weight::LIGHT,
        "bolder" => Weight::BOLD,
        "100" | "200" | "300" | "400" | "500" | "600" | "700" | "800" | "900" => {
            Weight::from(value.parse::<i32>().unwrap())
        }
        _ => Weight::NORMAL,
    }
}

fn to_width(value: &str) -> Width {
    match value {
        "ultra-condensed" => Width::ULTRA_CONDENSED,
        "extra-condensed" => Width::EXTRA_CONDENSED,
        "condensed" => Width::CONDENSED,
        "semi-condensed" => Width::SEMI_CONDENSED,
        "semi-expanded" => Width::SEMI_EXPANDED,
        "expanded" => Width::EXPANDED,
        "extra-expanded" => Width::EXTRA_EXPANDED,
        "ultra-expanded" => Width::ULTRA_EXPANDED,
        _ => Width::NORMAL,
    }
}

fn to_slant(value: &str) -> Slant {
    match value {
        "italic" => Slant::Italic,
        "oblique" => Slant::Oblique,
        _ => Slant::Upright,
    }
}

fn is_size(value: &str) -> bool {
    match value {
        _ => false,
    }
}
