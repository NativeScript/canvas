use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[derive(Clone)]
pub struct Path(pub(crate) canvas_2d::context::paths::path::Path);

impl Path {
    pub fn with_d(d: &str) -> Self {
        Path(canvas_2d::context::paths::path::Path::from_str(d))
    }
}

impl Default for Path {
    fn default() -> Self {
        Self(Default::default())
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_path_trim(path: *mut Path, start: f32, end: f32) {
    if path.is_null() {
        return;
    }
    let path = unsafe { &mut *path };
    path.0.trim(start, end);
}

#[no_mangle]
pub extern "C" fn canvas_native_path_add_path(path: *mut Path, path_to_add: *const Path) {
    if path.is_null() || path_to_add.is_null() {
        return;
    }
    let path = unsafe { &mut *path };
    let path_to_add = unsafe { &*path_to_add };
    path.0.add_path(&path_to_add.0, None);
}

#[no_mangle]
pub extern "C" fn canvas_native_path_create() -> *mut Path {
    Box::into_raw(Box::new(Path::default()))
}

#[no_mangle]
pub extern "C" fn canvas_native_path_create_with_path(path: *const Path) -> *mut Path {
    if path.is_null() {
        return std::ptr::null_mut();
    }
    let path = unsafe { &*path };
    Box::into_raw(Box::new(path.clone()))
}

#[no_mangle]
pub extern "C" fn canvas_native_path_create_with_string(string: *const c_char) -> *mut Path {
    if string.is_null() {
        return std::ptr::null_mut();
    }
    let string = unsafe { CStr::from_ptr(string) };
    let string = string.to_string_lossy();
    Box::into_raw(Box::new(Path(
        canvas_2d::context::paths::path::Path::from_str(string.as_ref()),
    )))
}

#[no_mangle]
pub extern "C" fn canvas_native_path_close_path(path: *mut Path) {
    if path.is_null() {
        return;
    }
    let path = unsafe { &mut *path };
    path.0.close_path()
}

#[no_mangle]
pub extern "C" fn canvas_native_path_move_to(path: *mut Path, x: f32, y: f32) {
    if path.is_null() {
        return;
    }
    let path = unsafe { &mut *path };
    path.0.move_to(x, y)
}

#[no_mangle]
pub extern "C" fn canvas_native_path_line_to(path: *mut Path, x: f32, y: f32) {
    if path.is_null() {
        return;
    }
    let path = unsafe { &mut *path };
    path.0.line_to(x, y)
}

#[no_mangle]
pub extern "C" fn canvas_native_path_bezier_curve_to(
    path: *mut Path,
    cp1x: f32,
    cp1y: f32,
    cp2x: f32,
    cp2y: f32,
    x: f32,
    y: f32,
) {
    if path.is_null() {
        return;
    }
    let path = unsafe { &mut *path };
    path.0.bezier_curve_to(cp1x, cp1y, cp2x, cp2y, x, y)
}

#[no_mangle]
pub extern "C" fn canvas_native_path_quadratic_curve_to(
    path: *mut Path,
    cpx: f32,
    cpy: f32,
    x: f32,
    y: f32,
) {
    if path.is_null() {
        return;
    }
    let path = unsafe { &mut *path };
    path.0.quadratic_curve_to(cpx, cpy, x, y)
}

#[no_mangle]
pub extern "C" fn canvas_native_path_arc(
    path: *mut Path,
    x: f32,
    y: f32,
    radius: f32,
    start_angle: f32,
    end_angle: f32,
    anti_clockwise: bool,
) {
    if path.is_null() {
        return;
    }
    let path = unsafe { &mut *path };
    path.0
        .arc(x, y, radius, start_angle, end_angle, anti_clockwise)
}

#[no_mangle]
pub extern "C" fn canvas_native_path_arc_to(
    path: *mut Path,
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    radius: f32,
) {
    if path.is_null() {
        return;
    }
    let path = unsafe { &mut *path };
    path.0.arc_to(x1, y1, x2, y2, radius)
}

#[no_mangle]
pub extern "C" fn canvas_native_path_ellipse(
    path: *mut Path,
    x: f32,
    y: f32,
    radius_x: f32,
    radius_y: f32,
    rotation: f32,
    start_angle: f32,
    end_angle: f32,
    anticlockwise: bool,
) {
    if path.is_null() {
        return;
    }
    let path = unsafe { &mut *path };
    path.0.ellipse(
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

#[no_mangle]
pub extern "C" fn canvas_native_path_rect(
    path: *mut Path,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
) {
    if path.is_null() {
        return;
    }
    let path = unsafe { &mut *path };
    path.0.rect(x, y, width, height)
}

#[no_mangle]
pub extern "C" fn canvas_native_path_round_rect(
    path: *mut Path,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    radii: *const f32,
    size: usize,
) {
    assert!(!path.is_null());
    let radii = unsafe { std::slice::from_raw_parts(radii, size) };
    let path = unsafe { &mut *path };

    println!("radii: {:?}", radii);

    let size = radii.len();
    if size == 0 {
        return;
    }
    /*
    [all-corners]
    [top-left-and-bottom-right, top-right-and-bottom-left]
    [top-left, top-right-and-bottom-left, bottom-right]
    [top-left, top-right, bottom-right, bottom-left]
     */
    let mut top_left = 0.;
    let mut top_right = 0.;
    let mut bottom_right = 0.;
    let mut bottom_left = 0.;

    match size {
        1 => {
            top_left = radii[0];
            top_right = top_left;
            bottom_right = top_left;
            bottom_left = top_left;
        }
        2 => {
            top_left = radii[0];
            top_right = radii[1];
            bottom_right = top_left;
            bottom_left = top_right;
        }

        3 => {
            top_left = radii[0];
            top_right = radii[1];
            bottom_right = radii[2];
            bottom_left = top_right
        }
        4 => {
            top_left = radii[0];
            top_right = radii[1];
            bottom_right = radii[2];
            bottom_left = radii[3];
        }
        _ => {}
    }

    if size > 0 && size <= 4 {
        path.0.round_rect(
            x,
            y,
            width,
            height,
            &[
                top_left,
                top_left,
                top_right,
                top_right,
                bottom_right,
                bottom_right,
                bottom_left,
                bottom_left,
            ],
        )
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_path_round_rect_tl_tr_br_bl(
    path: *mut Path,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    top_left: f32,
    top_right: f32,
    bottom_right: f32,
    bottom_left: f32,
) {
    assert!(!path.is_null());
    let path = unsafe { &mut *path };

    path.0.round_rect(
        x,
        y,
        width,
        height,
        &[
            top_left,
            top_left,
            top_right,
            top_right,
            bottom_right,
            bottom_right,
            bottom_left,
            bottom_left,
        ],
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_path_to_string(path: *const Path) -> *const c_char {
    if path.is_null() {
        return std::ptr::null_mut();
    }
    let path = unsafe { &*path };
    let svg = path.0.path().to_svg();
    CString::new(svg).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn canvas_native_path_release(value: *mut Path) {
    if value.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(value) };
}
