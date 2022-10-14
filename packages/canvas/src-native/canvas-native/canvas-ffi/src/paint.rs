use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_longlong};
use std::str::FromStr;

use css_color_parser::{Color, ColorParseError};

use canvas_core::context::{Context, ContextWrapper};
use canvas_core::context::fill_and_stroke_styles::paint::PaintStyle;
use canvas_core::ffi::paint_style_value::PaintStyleValue;
use canvas_core::utils::color::to_parsed_color;

pub(crate) fn paint_style_set_color_with_string(
    context: *mut ContextWrapper,
    is_fill: bool,
    color: *const c_char,
) {
    assert!(!context.is_null());
    assert!(!color.is_null());
    unsafe {
        let context = &mut *context;
        let mut context = context.get_context();
        let color = CStr::from_ptr(color).to_string_lossy();
        if let Ok(color) = css_color_parser::Color::from_str(color.as_ref()) {
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
pub extern "C" fn canvas_native_paint_style_set_fill_color_with_string(
    context: *mut ContextWrapper,
    color: *const c_char,
) {
    paint_style_set_color_with_string(context, true, color);
}

#[no_mangle]
pub extern "C" fn canvas_native_paint_style_set_stroke_color_with_string(
    context: *mut ContextWrapper,
    color: *const c_char,
) {
    paint_style_set_color_with_string(context, false, color);
}

#[no_mangle]
pub extern "C" fn canvas_native_paint_style_get_color_string(
    color: *mut PaintStyle,
) -> *const c_char {
    assert!(!color.is_null());
    unsafe {
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

#[no_mangle]
pub extern "C" fn canvas_native_destroy_paint_style_value(value: *mut PaintStyleValue) {
    if value.is_null() {
        return;
    }
    unsafe {
        Box::from_raw(value);
    }
}
