use std::ffi::CStr;
use std::os::raw::{c_char, c_longlong};

use canvas_core::context::{Context, ContextWrapper};

#[no_mangle]
pub extern "C" fn canvas_native_svg_draw_from_string(
    context: *mut ContextWrapper,
    svg: *const c_char,
) {
    assert!(!context.is_null());
    assert!(!svg.is_null());
    unsafe {
        let context = &mut *context;
        let mut context = context.get_context();
        let svg = CStr::from_ptr(svg);
        let svg = svg.to_string_lossy();
        canvas_core::svg::draw_svg(&mut context, svg.as_ref());
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_svg_draw_from_path(
    context: *mut ContextWrapper,
    path: *const c_char,
) {
    assert!(!context.is_null());
    assert!(!path.is_null());
    unsafe {
        let context = &mut *context;
        let mut context = context.get_context();
        let path = CStr::from_ptr(path);
        let path = path.to_string_lossy();
        canvas_core::svg::draw_svg_from_path(&mut context, path.as_ref());
    }
}
