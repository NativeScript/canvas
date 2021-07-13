use crate::common::context::Device;

const SIZE_CM: f32 = 1.0;
const SIZE_MILLIMETERS: f32 = SIZE_CM * 10.0;
const SIZE_INCH: f32 = SIZE_CM * 2.54;
const SIZE_PX: f32 = 1.0 / 96.0 * SIZE_INCH;
const SIZE_POINT: f32 = 1.0 / 72.0 * SIZE_INCH;
const SIZE_PICAS: f32 = SIZE_POINT * 12.0;

pub(crate) fn parse_size(value: &str, device: Device) -> f32 {
    if value.contains("px") {
        return value.replace("px", "").parse::<f32>().unwrap_or(0.0);
    } else if value.contains("cm") {
        let cm = 96. / 2.54;
        return value.replace("cm", "").parse::<f32>().unwrap_or(0.0) * cm;
    } else if value.contains("in") {
        let inch = 96.;
        return value.replace("in", "").parse::<f32>().unwrap_or(0.0) * inch;
    } else if value.contains("pt") {
        let pt = 96. * (1.0 / 72.0);
        return value.replace("pt", "").parse::<f32>().unwrap_or(0.0) * pt;
    } else if value.contains("pc") {
        let pc = (96. * (1.0 / 72.0)) * 12.0;
        return value.replace("pt", "").parse::<f32>().unwrap_or(0.0) * pc;
    } else if value.contains("vh") {
        let vh = value.replace("vh", "").parse::<f32>().unwrap_or(0.0) / 100.0;
        return device.height * vh;
    } else if value.contains("vw") {
        let vw = value.replace("vw", "").parse::<f32>().unwrap_or(0.0) / 100.0;
        return device.width * vw;
    }
    return 0.0;
}
