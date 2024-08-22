#![allow(dead_code)]

const SIZE_CM: f32 = 1.0;
const SIZE_MILLIMETERS: f32 = SIZE_CM * 10.0;
const SIZE_INCH: f32 = SIZE_CM * 2.54;
const SIZE_PX: f32 = 1.0 / 96.0 * SIZE_INCH;
const SIZE_POINT: f32 = 1.0 / 72.0 * SIZE_INCH;
const SIZE_PICAS: f32 = SIZE_POINT * 12.0;

/// The default font size.
pub const FONT_MEDIUM_PX: f32 = 16.0;

pub(crate) fn parse_size(value: &str, width: f32, height: f32, ppi: f32) -> f32 {
    if value.contains("%") {
        return value.replace("%", "").parse::<f32>().unwrap_or(0.0);
    }
    if value.contains("px") {
        return value.replace("px", "").parse::<f32>().unwrap_or(0.0);
    } else if value.contains("cm") {
        let cm = ppi / 2.54;
        return value.replace("cm", "").parse::<f32>().unwrap_or(0.0) * cm;
    } else if value.contains("in") {
        let inch = ppi;
        return value.replace("in", "").parse::<f32>().unwrap_or(0.0) * inch;
    } else if value.contains("pt") {
        let pt = ppi * (1.0 / 72.0);
        return value.replace("pt", "").parse::<f32>().unwrap_or(0.0) * pt;
    } else if value.contains("pc") {
        let pc = (ppi * (1.0 / 72.0)) * 12.0;
        return value.replace("pt", "").parse::<f32>().unwrap_or(0.0) * pc;
    } else if value.contains("vh") {
        let vh = value.replace("vh", "").parse::<f32>().unwrap_or(0.0) / 100.0;
        return height * vh;
    } else if value.contains("vw") {
        let vw = value.replace("vw", "").parse::<f32>().unwrap_or(0.0) / 100.0;
        return width * vw;
    }
    return 0.0;
}
