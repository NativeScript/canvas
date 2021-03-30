use std::ffi::CStr;
use std::os::raw::c_longlong;

use crate::common::context::Context;

#[no_mangle]
pub extern "C" fn svg_draw_from_string(context: c_longlong, svg: *const i8) {
    unsafe {
        if !svg.is_null() && context != 0 {
            let context: *mut Context = context as _;
            let context = &mut *context;
            let svg = CStr::from_ptr(svg);
            let svg = svg.to_string_lossy();
            crate::common::svg::draw_svg(context, svg.as_ref());
        }
    }
}

#[no_mangle]
pub extern "C" fn svg_draw_from_path(context: c_longlong, path: *const i8) {
    unsafe {
        if !path.is_null() && context != 0 {
            let context: *mut Context = context as _;
            let context = &mut *context;
            let path = CStr::from_ptr(path);
            let path = path.to_string_lossy();
            crate::common::svg::draw_svg_from_path(context, path.as_ref());
        }
    }
}