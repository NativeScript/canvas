use std::ffi::CStr;
use std::os::raw::{c_char, c_float, c_longlong};

use canvas_core::context::matrix::Matrix;
pub use canvas_core::context::paths::path::*;
use canvas_core::context::paths::path::Path;

#[no_mangle]
pub extern "C" fn canvas_native_path_create() -> *mut Path {
    Box::into_raw(Box::new(Path::new()))
}

#[no_mangle]
pub extern "C" fn canvas_native_path_create_with_path(path: *mut Path) -> *mut Path {
    if path.is_null() {
        return Box::into_raw(Box::new(Path::new()));
    }
    unsafe {
        let path = &mut *path;
        Box::into_raw(Box::new(Path::from_path(path.path())))
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_path_create_with_string(string: *const c_char) -> *mut Path {
    if string.is_null() {
        return Box::into_raw(Box::new(Path::new()));
    }
    unsafe {
        let string = CStr::from_ptr(string).to_string_lossy();
        Box::into_raw(Box::new(Path::from_str(string.as_ref())))
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_path_add_path(path: *mut Path, path_to_add: *mut Path) {
    if path.is_null() || path_to_add.is_null() {
        return;
    }
    unsafe {
        let path = &mut *path;
        let path_to_add = &mut *path_to_add;
        path.add_path(path_to_add, None);
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_path_add_path_with_matrix(
    path: *mut Path,
    path_to_add: *mut Path,
    matrix: *mut Matrix,
) {
    assert!(!path.is_null());
    assert!(!path_to_add.is_null());
    assert!(!matrix.is_null());

    unsafe {
        let path = &mut *path;
        let matrix = &mut *matrix;
        let path_to_add = &mut *path_to_add;
        path.add_path(path_to_add, Some(matrix))
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_path_close_path(path: *mut Path) {
    assert!(!path.is_null());
    unsafe {
        let path = &mut *path;
        path.close_path()
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_path_move_to(path: *mut Path, x: c_float, y: c_float) {
    assert!(!path.is_null());
    unsafe {
        let path = &mut *path;
        path.move_to(x, y)
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_path_line_to(path: *mut Path, x: c_float, y: c_float) {
    assert!(!path.is_null());
    unsafe {
        let path = &mut *path;
        path.line_to(x, y)
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_path_bezier_curve_to(
    path: *mut Path,
    cp1x: c_float,
    cp1y: c_float,
    cp2x: c_float,
    cp2y: c_float,
    x: c_float,
    y: c_float,
) {
    assert!(!path.is_null());
    unsafe {
        let path = &mut *path;
        path.bezier_curve_to(cp1x, cp1y, cp2x, cp2y, x, y)
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_path_quadratic_curve_to(
    path: *mut Path,
    cpx: c_float,
    cpy: c_float,
    x: c_float,
    y: c_float,
) {
    assert!(!path.is_null());
    unsafe {
        let path = &mut *path;
        path.quadratic_curve_to(cpx, cpy, x, y)
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_path_arc(
    path: *mut Path,
    x: c_float,
    y: c_float,
    radius: c_float,
    start_angle: c_float,
    end_angle: c_float,
    anti_clockwise: bool,
) {
    assert!(!path.is_null());
    unsafe {
        let path = &mut *path;
        path.arc(x, y, radius, start_angle, end_angle, anti_clockwise)
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_path_arc_to(
    path: *mut Path,
    x1: c_float,
    y1: c_float,
    x2: c_float,
    y2: c_float,
    radius: c_float,
) {
    assert!(!path.is_null());
    unsafe {
        let path = &mut *path;
        path.arc_to(x1, y1, x2, y2, radius)
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_path_ellipse(
    path: *mut Path,
    x: c_float,
    y: c_float,
    radius_x: c_float,
    radius_y: c_float,
    rotation: c_float,
    start_angle: c_float,
    end_angle: c_float,
    anticlockwise: bool,
) {
    assert!(!path.is_null());
    unsafe {
        let path = &mut *path;
        path.ellipse(
            x,
            y,
            radius_x,
            radius_y,
            rotation,
            start_angle,
            end_angle,
            anticlockwise,
        )
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_path_rect(
    path: *mut Path,
    x: c_float,
    y: c_float,
    width: c_float,
    height: c_float,
) {
    assert!(!path.is_null());
    unsafe {
        let path = &mut *path;
        path.rect(x, y, width, height)
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_destroy_path(path: *mut Path) {
    unsafe {
        if path.is_null() {
            return;
        }
        let _ = Box::from_raw(path);
    }
}
