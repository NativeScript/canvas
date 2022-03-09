use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::{jfloat, jlong};

use crate::common::context::matrix::Matrix;
use crate::common::context::paths::path::Path;

#[no_mangle]
pub extern "system" fn Java_org_nativescript_canvas_TNSPath2D_nativeInit(_: JNIEnv, _: JClass) -> jlong {
    Box::into_raw(Box::new(Path::new())) as jlong
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_canvas_TNSPath2D_nativeCreateWithPath(
    _: JNIEnv,
    _: JClass,
    path: jlong,
) -> jlong {
    if path == 0 {
        return Box::into_raw(Box::new(Path::new())) as jlong;
    }
    unsafe {
        let path: *mut Path = path as _;
        let path = &mut *path;
        Box::into_raw(Box::new(Path::from_path(&path.path))) as jlong
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_canvas_TNSPath2D_nativeCreateWithString(
    env: JNIEnv,
    _: JClass,
    string: JString,
) -> jlong {
    if let Ok(string) = env.get_string(string) {
        let string = string.to_string_lossy();
        Box::into_raw(Box::new(Path::from_str(string.as_ref()))) as jlong
    } else {
        return Box::into_raw(Box::new(Path::new())) as jlong;
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_canvas_TNSPath2D_nativeAddPath(
    _: JNIEnv,
    _: JClass,
    path: jlong,
    path_to_add: jlong,
) {
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
pub extern "system" fn Java_org_nativescript_canvas_TNSPath2D_nativeAddPathWithMatrix(
    _: JNIEnv,
    _: JClass,
    path: jlong,
    path_to_add: jlong,
    matrix: jlong,
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
pub extern "system" fn Java_org_nativescript_canvas_TNSPath2D_nativeClosePath(
    _: JNIEnv,
    _: JClass,
    path: jlong,
) {
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
pub extern "system" fn Java_org_nativescript_canvas_TNSPath2D_nativeMoveTo(
    _: JNIEnv,
    _: JClass,
    path: jlong,
    x: jfloat,
    y: jfloat,
) {
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
pub extern "system" fn Java_org_nativescript_canvas_TNSPath2D_nativeLineTo(
    _: JNIEnv,
    _: JClass,
    path: jlong,
    x: jfloat,
    y: jfloat,
) {
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
pub extern "system" fn Java_org_nativescript_canvas_TNSPath2D_nativeBezierCurveTo(
    _: JNIEnv,
    _: JClass,
    path: jlong,
    cp1x: jfloat,
    cp1y: jfloat,
    cp2x: jfloat,
    cp2y: jfloat,
    x: jfloat,
    y: jfloat,
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
pub extern "system" fn Java_org_nativescript_canvas_TNSPath2D_nativeQuadraticCurveTo(
    _: JNIEnv,
    _: JClass,
    path: jlong,
    cpx: jfloat,
    cpy: jfloat,
    x: jfloat,
    y: jfloat,
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
pub extern "system" fn Java_org_nativescript_canvas_TNSPath2D_nativeArc(
    _: JNIEnv,
    _: JClass,
    path: jlong,
    x: jfloat,
    y: jfloat,
    radius: jfloat,
    start_angle: jfloat,
    end_angle: jfloat,
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
pub extern "system" fn Java_org_nativescript_canvas_TNSPath2D_nativeArcTo(
    _: JNIEnv,
    _: JClass,
    path: jlong,
    x1: jfloat,
    y1: jfloat,
    x2: jfloat,
    y2: jfloat,
    radius: jfloat,
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
pub extern "system" fn Java_org_nativescript_canvas_TNSPath2D_nativeEllipse(
    _: JNIEnv,
    _: JClass,
    path: jlong,
    x: jfloat,
    y: jfloat,
    radius_x: jfloat,
    radius_y: jfloat,
    rotation: jfloat,
    start_angle: jfloat,
    end_angle: jfloat,
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
pub extern "system" fn Java_org_nativescript_canvas_TNSPath2D_nativeRect(
    _: JNIEnv,
    _: JClass,
    path: jlong,
    x: jfloat,
    y: jfloat,
    width: jfloat,
    height: jfloat,
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
pub extern "system" fn Java_org_nativescript_canvas_TNSPath2D_nativeDestroy(
    _: JNIEnv,
    _: JClass,
    path: jlong,
) {
    unsafe {
        if path == 0 {
            return;
        }
        let path: *mut Path = path as _;
        let _ = Box::from_raw(path);
    }
}
