use std::os::raw::{c_float, c_longlong};

use crate::common::context::matrix::Matrix;

#[no_mangle]
pub extern "C" fn matrix_create() -> c_longlong {
    Box::into_raw(
        Box::new(
            Matrix::new()
        )
    ) as c_longlong
}


#[no_mangle]
pub extern "C" fn matrix_update(matrix: c_longlong, data: *const c_float, data_len: usize) {
    if matrix == 0 {
        return;
    }
    unsafe {
        let matrix: *mut Matrix = matrix as _;
        let matrix = &mut *matrix;
        let slice = std::slice::from_raw_parts(data, data_len);
        let mut affine = [0f32; 6];
        affine.copy_from_slice(slice);
        matrix.set_affine(&affine);
    }
}


#[no_mangle]
pub extern "C" fn matrix_a(matrix: c_longlong) -> c_float {
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
pub extern "C" fn matrix_set_a(matrix: c_longlong, a: c_float) {
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
pub extern "C" fn matrix_b(matrix: c_longlong) -> c_float {
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
pub extern "C" fn matrix_set_b(matrix: c_longlong, b: c_float) {
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
pub extern "C" fn matrix_c(matrix: c_longlong) -> c_float {
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
pub extern "C" fn matrix_set_c(matrix: c_longlong, c: c_float) {
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
pub extern "C" fn matrix_d(matrix: c_longlong) -> c_float {
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
pub extern "C" fn matrix_set_d(matrix: c_longlong, d: c_float) {
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
pub extern "C" fn matrix_e(matrix: c_longlong) -> c_float {
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
pub extern "C" fn matrix_set_e(matrix: c_longlong, e: c_float) {
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
pub extern "C" fn matrix_f(matrix: c_longlong) -> c_float {
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
pub extern "C" fn matrix_set_f(matrix: c_longlong, f: c_float) {
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
pub extern "C" fn matrix_m11(matrix: c_longlong) -> c_float {
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
pub extern "C" fn matrix_set_m11(matrix: c_longlong, m11: c_float) {
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
pub extern "C" fn matrix_m12(matrix: c_longlong) -> c_float {
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
pub extern "C" fn matrix_set_m12(matrix: c_longlong, m12: c_float) {
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
pub extern "C" fn matrix_m13(matrix: c_longlong) -> c_float {
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
pub extern "C" fn matrix_set_m13(matrix: c_longlong, m13: c_float) {
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
pub extern "C" fn matrix_m14(matrix: c_longlong) -> c_float {
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
pub extern "C" fn matrix_set_m14(matrix: c_longlong, m14: c_float) {
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
pub extern "C" fn matrix_m21(matrix: c_longlong) -> c_float {
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
pub extern "C" fn matrix_set_m21(matrix: c_longlong, m21: c_float) {
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
pub extern "C" fn matrix_m22(matrix: c_longlong) -> c_float {
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
pub extern "C" fn matrix_set_m22(matrix: c_longlong, m22: c_float) {
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
pub extern "C" fn matrix_m23(matrix: c_longlong) -> c_float {
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
pub extern "C" fn matrix_set_m23(matrix: c_longlong, m23: c_float) {
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
pub extern "C" fn matrix_m24(matrix: c_longlong) -> c_float {
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
pub extern "C" fn matrix_set_m24(matrix: c_longlong, m24: c_float) {
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
pub extern "C" fn matrix_m31(matrix: c_longlong) -> c_float {
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
pub extern "C" fn matrix_set_m31(matrix: c_longlong, m31: c_float) {
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
pub extern "C" fn matrix_m32(matrix: c_longlong) -> c_float {
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
pub extern "C" fn matrix_set_m32(matrix: c_longlong, m32: c_float) {
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
pub extern "C" fn matrix_m33(matrix: c_longlong) -> c_float {
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
pub extern "C" fn matrix_set_m33(matrix: c_longlong, m33: c_float) {
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
pub extern "C" fn matrix_m34(matrix: c_longlong) -> c_float {
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
pub extern "C" fn matrix_set_m34(matrix: c_longlong, m34: c_float) {
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
pub extern "C" fn matrix_m41(matrix: c_longlong) -> c_float {
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
pub extern "C" fn matrix_set_m41(matrix: c_longlong, m41: c_float) {
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
pub extern "C" fn matrix_m42(matrix: c_longlong) -> c_float {
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
pub extern "C" fn matrix_set_m42(matrix: c_longlong, m42: c_float) {
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
pub extern "C" fn matrix_m43(matrix: c_longlong) -> c_float {
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
pub extern "C" fn matrix_set_m43(matrix: c_longlong, m43: c_float) {
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
pub extern "C" fn matrix_m44(matrix: c_longlong) -> c_float {
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
pub extern "C" fn matrix_set_m44(matrix: c_longlong, m44: c_float) {
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
pub extern "C" fn destroy_matrix(matrix: c_longlong) {
    if matrix == 0 {
        return;
    }
    unsafe {
        let matrix: *mut Matrix = matrix as _;
        let _ = Box::from_raw(matrix);
    }
}