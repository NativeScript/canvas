use crate::canvas2d::paint::PaintStyle;
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
