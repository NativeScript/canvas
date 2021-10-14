use std::ffi::CStr;
use std::os::raw::{c_char, c_float, c_longlong};

use crate::common::context::matrix::Matrix;
use crate::common::context::paths::path::Path;

#[no_mangle]
pub extern "C" fn path_create() -> c_longlong {
    Box::into_raw(Box::new(Path::new())) as c_longlong
}

#[no_mangle]
pub extern "C" fn path_create_with_path(path: c_longlong) -> c_longlong {
    if path == 0 {
        return Box::into_raw(Box::new(Path::new())) as c_longlong;
    }
    unsafe {
        let path: *mut Path = path as _;
        let path = &mut *path;
        Box::into_raw(Box::new(Path::from_path(&path.path))) as c_longlong
    }
}

#[no_mangle]
pub extern "C" fn path_create_with_string(string: *const c_char) -> c_longlong {
    if string.is_null() {
        return Box::into_raw(Box::new(Path::new())) as c_longlong;
    }
    unsafe {
        let string = CStr::from_ptr(string).to_string_lossy();
        Box::into_raw(Box::new(Path::from_str(string.as_ref()))) as c_longlong
    }
}

#[no_mangle]
pub extern "C" fn path_add_path(path: c_longlong, path_to_add: c_longlong) {
    if path == 0 || path_to_add == 0 {
        return;
    }
    unsafe {
        let path: *mut Path = path as _;
        let path = &mut *path;
        let path_to_add: *mut Path = path_to_add as _;
        let path_to_add = &mut *path_to_add;
        path.add_path(path_to_add, None);
    }
}

#[no_mangle]
pub extern "C" fn path_add_path_with_matrix(
    path: c_longlong,
    path_to_add: c_longlong,
    matrix: c_longlong,
) {
    if path == 0 || path_to_add == 0 || matrix == 0 {
        return;
    }
    unsafe {
        let path: *mut Path = path as _;
        let path = &mut *path;
        let matrix: *mut Matrix = matrix as _;
        let matrix = &mut *matrix;
        let path_to_add: *mut Path = path_to_add as _;
        let path_to_add = &mut *path_to_add;
        path.add_path(path_to_add, Some(matrix))
    }
}

#[no_mangle]
pub extern "C" fn path_close_path(path: c_longlong) {
    if path == 0 {
        return;
    }
    unsafe {
        let path: *mut Path = path as _;
        let path = &mut *path;
        path.close_path()
    }
}

#[no_mangle]
pub extern "C" fn path_move_to(path: c_longlong, x: c_float, y: c_float) {
    if path == 0 {
        return;
    }
    unsafe {
        let path: *mut Path = path as _;
        let path = &mut *path;
        path.move_to(x, y)
    }
}

#[no_mangle]
pub extern "C" fn path_line_to(path: c_longlong, x: c_float, y: c_float) {
    if path == 0 {
        return;
    }
    unsafe {
        let path: *mut Path = path as _;
        let path = &mut *path;
        path.line_to(x, y)
    }
}

#[no_mangle]
pub extern "C" fn path_bezier_curve_to(
    path: c_longlong,
    cp1x: c_float,
    cp1y: c_float,
    cp2x: c_float,
    cp2y: c_float,
    x: c_float,
    y: c_float,
) {
    unsafe {
        if path == 0 {
            return;
        }
        let path: *mut Path = path as _;
        let path = &mut *path;
        path.bezier_curve_to(cp1x, cp1y, cp2x, cp2y, x, y)
    }
}

#[no_mangle]
pub extern "C" fn path_quadratic_curve_to(
    path: c_longlong,
    cpx: c_float,
    cpy: c_float,
    x: c_float,
    y: c_float,
) {
    unsafe {
        if path == 0 {
            return;
        }
        let path: *mut Path = path as _;
        let path = &mut *path;
        path.quadratic_curve_to(cpx, cpy, x, y)
    }
}

#[no_mangle]
pub extern "C" fn path_arc(
    path: c_longlong,
    x: c_float,
    y: c_float,
    radius: c_float,
    start_angle: c_float,
    end_angle: c_float,
    anti_clockwise: bool,
) {
    unsafe {
        if path == 0 {
            return;
        }
        let path: *mut Path = path as _;
        let path = &mut *path;
        path.arc(x, y, radius, start_angle, end_angle, anti_clockwise)
    }
}

#[no_mangle]
pub extern "C" fn path_arc_to(
    path: c_longlong,
    x1: c_float,
    y1: c_float,
    x2: c_float,
    y2: c_float,
    radius: c_float,
) {
    unsafe {
        if path == 0 {
            return;
        }
        let path: *mut Path = path as _;
        let path = &mut *path;
        path.arc_to(x1, y1, x2, y2, radius)
    }
}

#[no_mangle]
pub extern "C" fn path_ellipse(
    path: c_longlong,
    x: c_float,
    y: c_float,
    radius_x: c_float,
    radius_y: c_float,
    rotation: c_float,
    start_angle: c_float,
    end_angle: c_float,
    anticlockwise: bool,
) {
    unsafe {
        if path == 0 {
            return;
        }
        let path: *mut Path = path as _;
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
pub extern "C" fn path_rect(
    path: c_longlong,
    x: c_float,
    y: c_float,
    width: c_float,
    height: c_float,
) {
    unsafe {
        if path == 0 {
            return;
        }
        let path: *mut Path = path as _;
        let path = &mut *path;
        path.rect(x, y, width, height)
    }
}

#[no_mangle]
pub extern "C" fn destroy_path(path: c_longlong) {
    unsafe {
        if path == 0 {
            return;
        }
        let path: *mut Path = path as _;
        let _ = Box::from_raw(path);
    }
}
