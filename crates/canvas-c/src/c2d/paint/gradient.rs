use crate::c2d::paint::PaintStyle;
use std::ffi::CStr;
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn canvas_native_gradient_add_color_stop(
    style: *mut PaintStyle,
    stop: f32,
    color: *const c_char,
) {
    if style.is_null() || color.is_null() {
        return;
    }
    let style = unsafe { &mut *style };
    let style = &mut style.0;
    let color = unsafe { CStr::from_ptr(color) };
    let color = color.to_string_lossy();
    match style {
        canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Gradient(gradient) => {
            gradient.add_color_stop_str(stop, color.as_ref())
        }
        _ => {}
    }
}

/// Add a color stop using pre-parsed RGBA values (0-255 each).
/// Skips CSS color string parsing entirely.
#[no_mangle]
pub extern "C" fn canvas_native_gradient_add_color_stop_rgba(
    style: *mut PaintStyle,
    stop: f32,
    r: u8,
    g: u8,
    b: u8,
    a: u8,
) {
    if style.is_null() {
        return;
    }
    let style = unsafe { &mut *style };
    let style = &mut style.0;
    match style {
        canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Gradient(gradient) => {
            let color = ((a as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
            gradient.add_color_stop_with_argb(stop, color)
        }
        _ => {}
    }
}
