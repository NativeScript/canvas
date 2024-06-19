use std::os::raw::c_float;

use canvas_core::context::matrix::Matrix;

#[no_mangle]
pub extern "C" fn canvas_native_matrix_create() -> *mut Matrix {
    Box::into_raw(Box::new(Matrix::new()))
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_update(
    matrix: *mut Matrix,
    data: *const c_float,
    data_len: usize,
) {
    assert!(!matrix.is_null());
    unsafe {
        let matrix = &mut *matrix;
        let slice = std::slice::from_raw_parts(data, data_len);
        let mut affine = [0f32; 6];
        affine.copy_from_slice(slice);
        matrix.set_affine(&affine);
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_a(matrix: *mut Matrix) -> c_float {
    assert!(!matrix.is_null());
    unsafe {
        let matrix = &*matrix;
        matrix.a()
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_set_a(matrix: *mut Matrix, a: c_float) {
    assert!(!matrix.is_null());
    unsafe {
        let matrix = &mut *matrix;
        matrix.set_a(a)
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_b(matrix: *mut Matrix) -> c_float {
    assert!(!matrix.is_null());
    unsafe {
        let matrix = &*matrix;
        matrix.b()
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_set_b(matrix: *mut Matrix, b: c_float) {
    assert!(!matrix.is_null());
    unsafe {
        let matrix = &mut *matrix;
        matrix.set_b(b)
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_c(matrix: *mut Matrix) -> c_float {
    assert!(!matrix.is_null());
    unsafe {
        let matrix = &*matrix;
        matrix.c()
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_set_c(matrix: *mut Matrix, c: c_float) {
    assert!(!matrix.is_null());
    unsafe {
        let matrix = &mut *matrix;
        matrix.set_c(c)
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_d(matrix: *mut Matrix) -> c_float {
    assert!(!matrix.is_null());
    unsafe {
        let matrix = &*matrix;
        matrix.d()
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_set_d(matrix: *mut Matrix, d: c_float) {
    assert!(!matrix.is_null());
    unsafe {
        let matrix = &mut *matrix;
        matrix.set_d(d)
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_e(matrix: *mut Matrix) -> c_float {
    assert!(!matrix.is_null());
    unsafe {
        let matrix = &*matrix;
        matrix.e()
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_set_e(matrix: *mut Matrix, e: c_float) {
    assert!(!matrix.is_null());
    unsafe {
        let matrix = &mut *matrix;
        matrix.set_e(e)
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_f(matrix: *mut Matrix) -> c_float {
    assert!(!matrix.is_null());
    unsafe {
        let matrix = &*matrix;
        matrix.f()
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_set_f(matrix: *mut Matrix, f: c_float) {
    assert!(!matrix.is_null());
    unsafe {
        let matrix = &mut *matrix;
        matrix.set_f(f)
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_m11(matrix: *mut Matrix) -> c_float {
    assert!(!matrix.is_null());
    unsafe {
        let matrix = &*matrix;
        matrix.m11()
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_set_m11(matrix: *mut Matrix, m11: c_float) {
    assert!(!matrix.is_null());
    unsafe {
        let matrix = &mut *matrix;
        matrix.set_m11(m11)
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_m12(matrix: *mut Matrix) -> c_float {
    assert!(!matrix.is_null());
    unsafe {
        let matrix = &*matrix;
        matrix.m12()
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_set_m12(matrix: *mut Matrix, m12: c_float) {
    assert!(!matrix.is_null());
    unsafe {
        let matrix = &mut *matrix;
        matrix.set_m12(m12)
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_m13(matrix: *mut Matrix) -> c_float {
    assert!(!matrix.is_null());
    unsafe {
        let matrix = &*matrix;
        matrix.m13()
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_set_m13(matrix: *mut Matrix, m13: c_float) {
    assert!(!matrix.is_null());
    unsafe {
        let matrix = &mut *matrix;
        matrix.set_m13(m13)
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_m14(matrix: *mut Matrix) -> c_float {
    assert!(!matrix.is_null());
    unsafe {
        let matrix = &*matrix;
        matrix.m14()
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_set_m14(matrix: *mut Matrix, m14: c_float) {
    assert!(!matrix.is_null());
    unsafe {
        let matrix = &mut *matrix;
        matrix.set_m14(m14)
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_m21(matrix: *mut Matrix) -> c_float {
    assert!(!matrix.is_null());
    unsafe {
        let matrix = &*matrix;
        matrix.m21()
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_set_m21(matrix: *mut Matrix, m21: c_float) {
    assert!(!matrix.is_null());
    unsafe {
        let matrix = &mut *matrix;
        matrix.set_m21(m21)
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_m22(matrix: *mut Matrix) -> c_float {
    assert!(!matrix.is_null());
    unsafe {
        let matrix = &*matrix;
        matrix.m22()
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_set_m22(matrix: *mut Matrix, m22: c_float) {
    assert!(!matrix.is_null());
    unsafe {
        let matrix = &mut *matrix;
        matrix.set_m22(m22)
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_m23(matrix: *mut Matrix) -> c_float {
    assert!(!matrix.is_null());
    unsafe {
        let matrix = &*matrix;
        matrix.m23()
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_set_m23(matrix: *mut Matrix, m23: c_float) {
    assert!(!matrix.is_null());
    unsafe {
        let matrix = &mut *matrix;
        matrix.set_m23(m23)
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_m24(matrix: *mut Matrix) -> c_float {
    assert!(!matrix.is_null());
    unsafe {
        let matrix = &*matrix;
        matrix.m24()
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_set_m24(matrix: *mut Matrix, m24: c_float) {
    assert!(!matrix.is_null());
    unsafe {
        let matrix = &mut *matrix;
        matrix.set_m24(m24)
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_m31(matrix: *mut Matrix) -> c_float {
    assert!(!matrix.is_null());
    unsafe {
        let matrix = &*matrix;
        matrix.m31()
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_set_m31(matrix: *mut Matrix, m31: c_float) {
    assert!(!matrix.is_null());
    unsafe {
        let matrix = &mut *matrix;
        matrix.set_m31(m31)
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_m32(matrix: *mut Matrix) -> c_float {
    assert!(!matrix.is_null());
    unsafe {
        let matrix = &*matrix;
        matrix.m32()
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_set_m32(matrix: *mut Matrix, m32: c_float) {
    assert!(!matrix.is_null());
    unsafe {
        let matrix = &mut *matrix;
        matrix.set_m32(m32)
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_m33(matrix: *mut Matrix) -> c_float {
    assert!(!matrix.is_null());
    unsafe {
        let matrix = &*matrix;
        matrix.m33()
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_set_m33(matrix: *mut Matrix, m33: c_float) {
    assert!(!matrix.is_null());
    unsafe {
        let matrix = &mut *matrix;
        matrix.set_m33(m33)
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_m34(matrix: *mut Matrix) -> c_float {
    assert!(!matrix.is_null());
    unsafe {
        let matrix = &*matrix;
        matrix.m34()
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_set_m34(matrix: *mut Matrix, m34: c_float) {
    assert!(!matrix.is_null());
    unsafe {
        let matrix = &mut *matrix;
        matrix.set_m34(m34)
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_m41(matrix: *mut Matrix) -> c_float {
    assert!(!matrix.is_null());
    unsafe {
        let matrix = &*matrix;
        matrix.m41()
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_set_m41(matrix: *mut Matrix, m41: c_float) {
    assert!(!matrix.is_null());
    unsafe {
        let matrix = &mut *matrix;
        matrix.set_m41(m41)
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_m42(matrix: *mut Matrix) -> c_float {
    assert!(!matrix.is_null());
    unsafe {
        let matrix = &*matrix;
        matrix.m42()
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_set_m42(matrix: *mut Matrix, m42: c_float) {
    assert!(!matrix.is_null());
    unsafe {
        let matrix = &mut *matrix;
        matrix.set_m42(m42)
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_m43(matrix: *mut Matrix) -> c_float {
    assert!(!matrix.is_null());
    unsafe {
        let matrix = &*matrix;
        matrix.m43()
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_set_m43(matrix: *mut Matrix, m43: c_float) {
    assert!(!matrix.is_null());
    unsafe {
        let matrix = &mut *matrix;
        matrix.set_m43(m43)
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_m44(matrix: *mut Matrix) -> c_float {
    assert!(!matrix.is_null());
    unsafe {
        let matrix = &*matrix;
        matrix.m44()
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_set_m44(matrix: *mut Matrix, m44: c_float) {
    assert!(!matrix.is_null());
    unsafe {
        let matrix = &mut *matrix;
        matrix.set_m44(m44)
    }
}

#[no_mangle]
pub extern "C" fn destroy_matrix(matrix: *mut Matrix) {
    if matrix.is_null() {
        return;
    }
    unsafe {
        let _ = Box::from_raw(matrix);
    }
}
