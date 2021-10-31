use jni::JNIEnv;
use jni::objects::JClass;
use jni::sys::{jfloat, jfloatArray, jlong};

use crate::common::context::matrix::Matrix;

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSDOMMatrix_nativeInit(
    _: JNIEnv,
    _: JClass,
) -> jlong {
    Box::into_raw(Box::new(Matrix::new())) as jlong
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSDOMMatrix_nativeUpdate(
    env: JNIEnv,
    _: JClass,
    matrix: jlong,
    data: jfloatArray,
) {
    if matrix == 0 {
        return;
    }
    unsafe {
        let matrix: *mut Matrix = matrix as _;
        let matrix = &mut *matrix;
        let mut affine = [0f32; 6];
        if let Ok(_) = env.get_float_array_region(data, 0, &mut affine) {
            matrix.set_affine(&affine);
        }
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSDOMMatrix_nativeA(
    _: JNIEnv,
    _: JClass,
    matrix: jlong,
) -> jfloat {
    if matrix == 0 {
        return 0.0;
    }
    unsafe {
        let matrix: *mut Matrix = matrix as _;
        let matrix = &*matrix;
        matrix.a()
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSDOMMatrix_nativeSetA(
    _: JNIEnv,
    _: JClass,
    matrix: jlong,
    a: jfloat,
) {
    if matrix == 0 {
        return;
    }
    unsafe {
        let matrix: *mut Matrix = matrix as _;
        let matrix = &mut *matrix;
        matrix.set_a(a)
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSDOMMatrix_nativeB(
    _: JNIEnv,
    _: JClass,
    matrix: jlong,
) -> jfloat {
    if matrix == 0 {
        return 0.0;
    }
    unsafe {
        let matrix: *mut Matrix = matrix as _;
        let matrix = &*matrix;
        matrix.b()
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSDOMMatrix_nativeSetB(
    _: JNIEnv,
    _: JClass,
    matrix: jlong,
    b: jfloat,
) {
    if matrix == 0 {
        return;
    }
    unsafe {
        let matrix: *mut Matrix = matrix as _;
        let matrix = &mut *matrix;
        matrix.set_b(b)
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSDOMMatrix_nativeC(
    _: JNIEnv,
    _: JClass,
    matrix: jlong,
) -> jfloat {
    if matrix == 0 {
        return 0.0;
    }
    unsafe {
        let matrix: *mut Matrix = matrix as _;
        let matrix = &*matrix;
        matrix.c()
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSDOMMatrix_nativeSetC(
    _: JNIEnv,
    _: JClass,
    matrix: jlong,
    c: jfloat,
) {
    if matrix == 0 {
        return;
    }
    unsafe {
        let matrix: *mut Matrix = matrix as _;
        let matrix = &mut *matrix;
        matrix.set_c(c)
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSDOMMatrix_nativeD(
    _: JNIEnv,
    _: JClass,
    matrix: jlong,
) -> jfloat {
    if matrix == 0 {
        return 0.0;
    }
    unsafe {
        let matrix: *mut Matrix = matrix as _;
        let matrix = &*matrix;
        matrix.d()
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSDOMMatrix_nativeSetD(
    _: JNIEnv,
    _: JClass,
    matrix: jlong,
    d: jfloat,
) {
    if matrix == 0 {
        return;
    }
    unsafe {
        let matrix: *mut Matrix = matrix as _;
        let matrix = &mut *matrix;
        matrix.set_d(d)
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSDOMMatrix_nativeE(
    _: JNIEnv,
    _: JClass,
    matrix: jlong,
) -> jfloat {
    if matrix == 0 {
        return 0.0;
    }
    unsafe {
        let matrix: *mut Matrix = matrix as _;
        let matrix = &*matrix;
        matrix.e()
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSDOMMatrix_nativeSetE(
    _: JNIEnv,
    _: JClass,
    matrix: jlong,
    e: jfloat,
) {
    if matrix == 0 {
        return;
    }
    unsafe {
        let matrix: *mut Matrix = matrix as _;
        let matrix = &mut *matrix;
        matrix.set_e(e)
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSDOMMatrix_nativeF(
    _: JNIEnv,
    _: JClass,
    matrix: jlong,
) -> jfloat {
    if matrix == 0 {
        return 0.0;
    }
    unsafe {
        let matrix: *mut Matrix = matrix as _;
        let matrix = &*matrix;
        matrix.f()
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSDOMMatrix_nativeSetF(
    _: JNIEnv,
    _: JClass,
    matrix: jlong,
    f: jfloat,
) {
    if matrix == 0 {
        return;
    }
    unsafe {
        let matrix: *mut Matrix = matrix as _;
        let matrix = &mut *matrix;
        matrix.set_f(f)
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSDOMMatrix_nativeM11(
    _: JNIEnv,
    _: JClass,
    matrix: jlong,
) -> jfloat {
    if matrix == 0 {
        return 0.0;
    }
    unsafe {
        let matrix: *mut Matrix = matrix as _;
        let matrix = &*matrix;
        matrix.m11()
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSDOMMatrix_nativeSetM11(
    _: JNIEnv,
    _: JClass,
    matrix: jlong,
    m11: jfloat,
) {
    if matrix == 0 {
        return;
    }
    unsafe {
        let matrix: *mut Matrix = matrix as _;
        let matrix = &mut *matrix;
        matrix.set_m11(m11)
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSDOMMatrix_nativeM12(
    _: JNIEnv,
    _: JClass,
    matrix: jlong,
) -> jfloat {
    if matrix == 0 {
        return 0.0;
    }
    unsafe {
        let matrix: *mut Matrix = matrix as _;
        let matrix = &*matrix;
        matrix.m12()
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSDOMMatrix_nativeSetM12(
    _: JNIEnv,
    _: JClass,
    matrix: jlong,
    m12: jfloat,
) {
    if matrix == 0 {
        return;
    }
    unsafe {
        let matrix: *mut Matrix = matrix as _;
        let matrix = &mut *matrix;
        matrix.set_m12(m12)
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSDOMMatrix_nativeM13(
    _: JNIEnv,
    _: JClass,
    matrix: jlong,
) -> jfloat {
    if matrix == 0 {
        return 0.0;
    }
    unsafe {
        let matrix: *mut Matrix = matrix as _;
        let matrix = &*matrix;
        matrix.m13()
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSDOMMatrix_nativeSetM13(
    _: JNIEnv,
    _: JClass,
    matrix: jlong,
    m13: jfloat,
) {
    if matrix == 0 {
        return;
    }
    unsafe {
        let matrix: *mut Matrix = matrix as _;
        let matrix = &mut *matrix;
        matrix.set_m13(m13)
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSDOMMatrix_nativeM14(
    _: JNIEnv,
    _: JClass,
    matrix: jlong,
) -> jfloat {
    if matrix == 0 {
        return 0.0;
    }
    unsafe {
        let matrix: *mut Matrix = matrix as _;
        let matrix = &*matrix;
        matrix.m14()
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSDOMMatrix_nativeSetM14(
    _: JNIEnv,
    _: JClass,
    matrix: jlong,
    m14: jfloat,
) {
    if matrix == 0 {
        return;
    }
    unsafe {
        let matrix: *mut Matrix = matrix as _;
        let matrix = &mut *matrix;
        matrix.set_m14(m14)
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSDOMMatrix_nativeM21(
    _: JNIEnv,
    _: JClass,
    matrix: jlong,
) -> jfloat {
    if matrix == 0 {
        return 0.0;
    }
    unsafe {
        let matrix: *mut Matrix = matrix as _;
        let matrix = &*matrix;
        matrix.m21()
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSDOMMatrix_nativeSetM21(
    _: JNIEnv,
    _: JClass,
    matrix: jlong,
    m21: jfloat,
) {
    if matrix == 0 {
        return;
    }
    unsafe {
        let matrix: *mut Matrix = matrix as _;
        let matrix = &mut *matrix;
        matrix.set_m21(m21)
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSDOMMatrix_nativeM22(
    _: JNIEnv,
    _: JClass,
    matrix: jlong,
) -> jfloat {
    if matrix == 0 {
        return 0.0;
    }
    unsafe {
        let matrix: *mut Matrix = matrix as _;
        let matrix = &*matrix;
        matrix.m22()
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSDOMMatrix_nativeSetM22(
    _: JNIEnv,
    _: JClass,
    matrix: jlong,
    m22: jfloat,
) {
    if matrix == 0 {
        return;
    }
    unsafe {
        let matrix: *mut Matrix = matrix as _;
        let matrix = &mut *matrix;
        matrix.set_m22(m22)
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSDOMMatrix_nativeM23(
    _: JNIEnv,
    _: JClass,
    matrix: jlong,
) -> jfloat {
    if matrix == 0 {
        return 0.0;
    }
    unsafe {
        let matrix: *mut Matrix = matrix as _;
        let matrix = &*matrix;
        matrix.m23()
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSDOMMatrix_nativeSetM23(
    _: JNIEnv,
    _: JClass,
    matrix: jlong,
    m23: jfloat,
) {
    if matrix == 0 {
        return;
    }
    unsafe {
        let matrix: *mut Matrix = matrix as _;
        let matrix = &mut *matrix;
        matrix.set_m23(m23)
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSDOMMatrix_nativeM24(
    _: JNIEnv,
    _: JClass,
    matrix: jlong,
) -> jfloat {
    if matrix == 0 {
        return 0.0;
    }
    unsafe {
        let matrix: *mut Matrix = matrix as _;
        let matrix = &*matrix;
        matrix.m24()
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSDOMMatrix_nativeSetM24(
    _: JNIEnv,
    _: JClass,
    matrix: jlong,
    m24: jfloat,
) {
    if matrix == 0 {
        return;
    }
    unsafe {
        let matrix: *mut Matrix = matrix as _;
        let matrix = &mut *matrix;
        matrix.set_m24(m24)
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSDOMMatrix_nativeM31(
    _: JNIEnv,
    _: JClass,
    matrix: jlong,
) -> jfloat {
    if matrix == 0 {
        return 0.0;
    }
    unsafe {
        let matrix: *mut Matrix = matrix as _;
        let matrix = &*matrix;
        matrix.m31()
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSDOMMatrix_nativeSetM31(
    _: JNIEnv,
    _: JClass,
    matrix: jlong,
    m31: jfloat,
) {
    if matrix == 0 {
        return;
    }
    unsafe {
        let matrix: *mut Matrix = matrix as _;
        let matrix = &mut *matrix;
        matrix.set_m31(m31)
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSDOMMatrix_nativeM32(
    _: JNIEnv,
    _: JClass,
    matrix: jlong,
) -> jfloat {
    if matrix == 0 {
        return 0.0;
    }
    unsafe {
        let matrix: *mut Matrix = matrix as _;
        let matrix = &*matrix;
        matrix.m32()
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSDOMMatrix_nativeSetM32(
    _: JNIEnv,
    _: JClass,
    matrix: jlong,
    m32: jfloat,
) {
    if matrix == 0 {
        return;
    }
    unsafe {
        let matrix: *mut Matrix = matrix as _;
        let matrix = &mut *matrix;
        matrix.set_m32(m32)
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSDOMMatrix_nativeM33(
    _: JNIEnv,
    _: JClass,
    matrix: jlong,
) -> jfloat {
    if matrix == 0 {
        return 0.0;
    }
    unsafe {
        let matrix: *mut Matrix = matrix as _;
        let matrix = &*matrix;
        matrix.m33()
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSDOMMatrix_nativeSetM33(
    _: JNIEnv,
    _: JClass,
    matrix: jlong,
    m33: jfloat,
) {
    if matrix == 0 {
        return;
    }
    unsafe {
        let matrix: *mut Matrix = matrix as _;
        let matrix = &mut *matrix;
        matrix.set_m33(m33)
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSDOMMatrix_nativeM34(
    _: JNIEnv,
    _: JClass,
    matrix: jlong,
) -> jfloat {
    if matrix == 0 {
        return 0.0;
    }
    unsafe {
        let matrix: *mut Matrix = matrix as _;
        let matrix = &*matrix;
        matrix.m34()
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSDOMMatrix_nativeSetM34(
    _: JNIEnv,
    _: JClass,
    matrix: jlong,
    m34: jfloat,
) {
    if matrix == 0 {
        return;
    }
    unsafe {
        let matrix: *mut Matrix = matrix as _;
        let matrix = &mut *matrix;
        matrix.set_m34(m34)
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSDOMMatrix_nativeM41(
    _: JNIEnv,
    _: JClass,
    matrix: jlong,
) -> jfloat {
    if matrix == 0 {
        return 0.0;
    }
    unsafe {
        let matrix: *mut Matrix = matrix as _;
        let matrix = &*matrix;
        matrix.m41()
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSDOMMatrix_nativeSetM41(
    _: JNIEnv,
    _: JClass,
    matrix: jlong,
    m41: jfloat,
) {
    if matrix == 0 {
        return;
    }
    unsafe {
        let matrix: *mut Matrix = matrix as _;
        let matrix = &mut *matrix;
        matrix.set_m41(m41)
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSDOMMatrix_nativeM42(
    _: JNIEnv,
    _: JClass,
    matrix: jlong,
) -> jfloat {
    if matrix == 0 {
        return 0.0;
    }
    unsafe {
        let matrix: *mut Matrix = matrix as _;
        let matrix = &*matrix;
        matrix.m42()
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSDOMMatrix_nativeSetM42(
    _: JNIEnv,
    _: JClass,
    matrix: jlong,
    m42: jfloat,
) {
    if matrix == 0 {
        return;
    }
    unsafe {
        let matrix: *mut Matrix = matrix as _;
        let matrix = &mut *matrix;
        matrix.set_m42(m42)
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSDOMMatrix_nativeM43(
    _: JNIEnv,
    _: JClass,
    matrix: jlong,
) -> jfloat {
    if matrix == 0 {
        return 0.0;
    }
    unsafe {
        let matrix: *mut Matrix = matrix as _;
        let matrix = &*matrix;
        matrix.m43()
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSDOMMatrix_nativeSetM43(
    _: JNIEnv,
    _: JClass,
    matrix: jlong,
    m43: jfloat,
) {
    if matrix == 0 {
        return;
    }
    unsafe {
        let matrix: *mut Matrix = matrix as _;
        let matrix = &mut *matrix;
        matrix.set_m43(m43)
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSDOMMatrix_nativeM44(
    _: JNIEnv,
    _: JClass,
    matrix: jlong,
) -> jfloat {
    if matrix == 0 {
        return 0.0;
    }
    unsafe {
        let matrix: *mut Matrix = matrix as _;
        let matrix = &*matrix;
        matrix.m44()
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSDOMMatrix_nativeSetM44(
    _: JNIEnv,
    _: JClass,
    matrix: jlong,
    m44: jfloat,
) {
    if matrix == 0 {
        return;
    }
    unsafe {
        let matrix: *mut Matrix = matrix as _;
        let matrix = &mut *matrix;
        matrix.set_m44(m44)
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSDOMMatrix_nativeDestroy(
    _: JNIEnv,
    _: JClass,
    matrix: jlong,
) {
    if matrix == 0 {
        return;
    }
    unsafe {
        let matrix: *mut Matrix = matrix as _;
        let _ = Box::from_raw(matrix);
    }
}
