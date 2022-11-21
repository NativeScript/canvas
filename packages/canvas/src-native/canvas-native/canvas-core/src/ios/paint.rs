use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_longlong};
use std::str::FromStr;

use css_color_parser::{Color};

use crate::common::context::Context;
use crate::common::context::fill_and_stroke_styles::paint::PaintStyle;
use crate::common::utils::color::to_parsed_color;

pub(crate) fn paint_style_set_color_with_string(
    context: c_longlong,
    is_fill: bool,
    color: *const c_char,
) {
    if context == 0 || color.is_null() {
        return;
    }
    unsafe {
        let context: *mut Context = context as _;
        let context = &mut *context;
        let color = CStr::from_ptr(color).to_string_lossy();
        if let Ok(color) = Color::from_str(color.as_ref()) {
            let style = PaintStyle::Color(skia_safe::Color::from_argb(
                (color.a * 255.0) as u8,
                color.r,
                color.g,
                color.b,
            ));
            if is_fill {
                context.set_fill_style(style);
            } else {
                context.set_stroke_style(style);
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn paint_style_set_fill_color_with_string(
    context: c_longlong,
    color: *const c_char,
) {
    paint_style_set_color_with_string(context, true, color);
}

#[no_mangle]
pub extern "C" fn paint_style_set_stroke_color_with_string(
    context: c_longlong,
    color: *const c_char,
) {
    paint_style_set_color_with_string(context, false, color);
}

#[no_mangle]
pub extern "C" fn paint_style_get_color_string(color: c_longlong) -> *const c_char {
    if color == 0 {
        return std::ptr::null();
    }
    unsafe {
        let color: *const PaintStyle = color as _;
        let color = &*color;
        match color {
            PaintStyle::Color(color) => {
                let string = to_parsed_color(*color);
                CString::new(string).unwrap().into_raw()
            }
            _ => std::ptr::null(),
        }
    }
}
