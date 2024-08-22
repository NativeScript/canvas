use std::ffi::CStr;
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn canvas_native_svg_draw_from_string(
    data: *mut u8,
    size: usize,
    width: f32,
    height: f32,
    scale: f32,
    svg: *const c_char,
) {
    if data.is_null() {
        return;
    }

    let slice = unsafe { std::slice::from_raw_parts_mut(data, size) };

    let svg = unsafe { CStr::from_ptr(svg) };

    let info = skia_safe::ImageInfo::new_n32_premul(
        skia_safe::ISize::new(width as i32, height as i32),
        None,
    );

    let svg = svg.to_string_lossy();

    if let Some(mut surface) = skia_safe::surface::surfaces::wrap_pixels(&info, slice, None, None) {
        // surface.canvas().scale((scale, scale));
        crate::draw_svg(&mut surface, svg.as_ref())
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_svg_draw_from_path(
    data: *mut u8,
    size: usize,
    width: f32,
    height: f32,
    scale: f32,
    path: *const c_char,
) {
    if data.is_null() {
        return;
    }

    let slice = unsafe { std::slice::from_raw_parts_mut(data, size) };

    let path = unsafe { CStr::from_ptr(path) };

    let info = skia_safe::ImageInfo::new_n32_premul(
        skia_safe::ISize::new(width as i32, height as i32),
        None,
    );

    let path = path.to_string_lossy();

    if let Some(mut surface) = skia_safe::surface::surfaces::wrap_pixels(&info, slice, None, None) {
        // surface.canvas().scale((scale, scale));
        crate::draw_svg_from_path(&mut surface, path.as_ref())
    }
}
