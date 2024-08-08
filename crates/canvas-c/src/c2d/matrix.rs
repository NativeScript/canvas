pub struct Matrix(pub(crate) canvas_2d::context::matrix::Matrix);

#[no_mangle]
pub extern "C" fn canvas_native_matrix_create() -> *mut Matrix {
    Box::into_raw(Box::new(Matrix(canvas_2d::context::matrix::Matrix::new())))
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_update(matrix: *mut Matrix, slice: *const f32, size: usize) {
    if matrix.is_null() || slice.is_null() {
        return;
    }
    let slice = unsafe { std::slice::from_raw_parts(slice, size) };
    let matrix = unsafe { &mut *matrix };
    let mut affine = [0f32; 6];
    affine.copy_from_slice(slice);
    matrix.0.set_affine(&affine);
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_update_3d(
    matrix: *mut Matrix,
    slice: *const f32,
    size: usize,
) {
    if matrix.is_null() || slice.is_null() {
        return;
    }
    let slice = unsafe { std::slice::from_raw_parts(slice, size) };
    assert_eq!(slice.len(), 16);
    let matrix = unsafe { &mut *matrix };
    let matrix = &mut matrix.0;
    matrix.set_m11(slice[0]);
    matrix.set_m12(slice[1]);
    matrix.set_m13(slice[2]);
    matrix.set_m14(slice[3]);

    matrix.set_m21(slice[4]);
    matrix.set_m22(slice[5]);
    matrix.set_m23(slice[6]);
    matrix.set_m24(slice[7]);

    matrix.set_m31(slice[8]);
    matrix.set_m32(slice[9]);
    matrix.set_m33(slice[10]);
    matrix.set_m34(slice[11]);

    matrix.set_m41(slice[12]);
    matrix.set_m42(slice[13]);
    matrix.set_m43(slice[14]);
    matrix.set_m44(slice[15]);
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_get_a(matrix: *const Matrix) -> f32 {
    let matrix = unsafe { &*matrix };
    let matrix = &matrix.0;
    matrix.a()
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_set_a(matrix: *mut Matrix, a: f32) {
    let matrix = unsafe { &mut *matrix };
    let matrix = &mut matrix.0;
    matrix.set_a(a)
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_get_b(matrix: *const Matrix) -> f32 {
    let matrix = unsafe { &*matrix };
    let matrix = &matrix.0;
    matrix.b()
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_set_b(matrix: *mut Matrix, b: f32) {
    let matrix = unsafe { &mut *matrix };
    let matrix = &mut matrix.0;
    matrix.set_b(b)
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_get_c(matrix: *const Matrix) -> f32 {
    let matrix = unsafe { &*matrix };
    let matrix = &matrix.0;
    matrix.c()
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_set_c(matrix: *mut Matrix, c: f32) {
    let matrix = unsafe { &mut *matrix };
    let matrix = &mut matrix.0;
    matrix.set_c(c)
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_get_d(matrix: *const Matrix) -> f32 {
    let matrix = unsafe { &*matrix };
    let matrix = &matrix.0;
    matrix.d()
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_set_d(matrix: *mut Matrix, d: f32) {
    let matrix = unsafe { &mut *matrix };
    let matrix = &mut matrix.0;
    matrix.set_d(d)
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_get_e(matrix: *const Matrix) -> f32 {
    let matrix = unsafe { &*matrix };
    let matrix = &matrix.0;
    matrix.e()
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_set_e(matrix: *mut Matrix, e: f32) {
    let matrix = unsafe { &mut *matrix };
    let matrix = &mut matrix.0;
    matrix.set_e(e)
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_get_f(matrix: *const Matrix) -> f32 {
    let matrix = unsafe { &*matrix };
    let matrix = &matrix.0;
    matrix.f()
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_set_f(matrix: *mut Matrix, f: f32) {
    let matrix = unsafe { &mut *matrix };
    let matrix = &mut matrix.0;
    matrix.set_f(f)
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_get_m11(matrix: *const Matrix) -> f32 {
    let matrix = unsafe { &*matrix };
    let matrix = &matrix.0;
    matrix.m11()
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_set_m11(matrix: *mut Matrix, m11: f32) {
    let matrix = unsafe { &mut *matrix };
    let matrix = &mut matrix.0;
    matrix.set_m11(m11)
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_get_m12(matrix: *const Matrix) -> f32 {
    let matrix = unsafe { &*matrix };
    let matrix = &matrix.0;
    matrix.m12()
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_set_m12(matrix: *mut Matrix, m12: f32) {
    let matrix = unsafe { &mut *matrix };
    let matrix = &mut matrix.0;
    matrix.set_m12(m12)
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_get_m13(matrix: *const Matrix) -> f32 {
    let matrix = unsafe { &*matrix };
    let matrix = &matrix.0;
    matrix.m13()
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_set_m13(matrix: *mut Matrix, m13: f32) {
    let matrix = unsafe { &mut *matrix };
    let matrix = &mut matrix.0;
    matrix.set_m13(m13)
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_get_m14(matrix: *const Matrix) -> f32 {
    let matrix = unsafe { &*matrix };
    let matrix = &matrix.0;
    matrix.m14()
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_set_m14(matrix: *mut Matrix, m14: f32) {
    let matrix = unsafe { &mut *matrix };
    let matrix = &mut matrix.0;
    matrix.set_m14(m14)
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_get_m21(matrix: *const Matrix) -> f32 {
    let matrix = unsafe { &*matrix };
    let matrix = &matrix.0;
    matrix.m21()
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_set_m21(matrix: *mut Matrix, m21: f32) {
    let matrix = unsafe { &mut *matrix };
    let matrix = &mut matrix.0;
    matrix.set_m21(m21)
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_get_m22(matrix: *mut Matrix) -> f32 {
    let matrix = unsafe { &*matrix };
    let matrix = &matrix.0;
    matrix.m22()
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_set_m22(matrix: *mut Matrix, m22: f32) {
    let matrix = unsafe { &mut *matrix };
    let matrix = &mut matrix.0;
    matrix.set_m22(m22)
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_get_m23(matrix: *const Matrix) -> f32 {
    let matrix = unsafe { &*matrix };
    let matrix = &matrix.0;
    matrix.m23()
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_set_m23(matrix: *mut Matrix, m23: f32) {
    let matrix = unsafe { &mut *matrix };
    let matrix = &mut matrix.0;
    matrix.set_m23(m23)
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_get_m24(matrix: *const Matrix) -> f32 {
    let matrix = unsafe { &*matrix };
    let matrix = &matrix.0;
    matrix.m24()
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_set_m24(matrix: *mut Matrix, m24: f32) {
    let matrix = unsafe { &mut *matrix };
    let matrix = &mut matrix.0;
    matrix.set_m24(m24)
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_get_m31(matrix: *const Matrix) -> f32 {
    let matrix = unsafe { &*matrix };
    let matrix = &matrix.0;
    matrix.m31()
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_set_m31(matrix: *mut Matrix, m31: f32) {
    let matrix = unsafe { &mut *matrix };
    let matrix = &mut matrix.0;
    matrix.set_m31(m31)
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_get_m32(matrix: *const Matrix) -> f32 {
    let matrix = unsafe { &*matrix };
    let matrix = &matrix.0;
    matrix.m32()
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_set_m32(matrix: *mut Matrix, m32: f32) {
    let matrix = unsafe { &mut *matrix };
    let matrix = &mut matrix.0;
    matrix.set_m32(m32)
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_get_m33(matrix: *const Matrix) -> f32 {
    let matrix = unsafe { &*matrix };
    let matrix = &matrix.0;
    matrix.m33()
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_set_m33(matrix: *mut Matrix, m33: f32) {
    let matrix = unsafe { &mut *matrix };
    let matrix = &mut matrix.0;
    matrix.set_m33(m33)
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_get_m34(matrix: *const Matrix) -> f32 {
    let matrix = unsafe { &*matrix };
    let matrix = &matrix.0;
    matrix.m34()
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_set_m34(matrix: *mut Matrix, m34: f32) {
    let matrix = unsafe { &mut *matrix };
    let matrix = &mut matrix.0;
    matrix.set_m34(m34)
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_get_m41(matrix: *const Matrix) -> f32 {
    let matrix = unsafe { &*matrix };
    let matrix = &matrix.0;
    matrix.m41()
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_set_m41(matrix: *mut Matrix, m41: f32) {
    let matrix = unsafe { &mut *matrix };
    let matrix = &mut matrix.0;
    matrix.set_m41(m41)
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_get_m42(matrix: *const Matrix) -> f32 {
    let matrix = unsafe { &*matrix };
    let matrix = &matrix.0;
    matrix.m42()
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_set_m42(matrix: *mut Matrix, m42: f32) {
    let matrix = unsafe { &mut *matrix };
    let matrix = &mut matrix.0;
    matrix.set_m42(m42)
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_get_m43(matrix: *const Matrix) -> f32 {
    let matrix = unsafe { &*matrix };
    let matrix = &matrix.0;
    matrix.m43()
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_set_m43(matrix: *mut Matrix, m43: f32) {
    let matrix = unsafe { &mut *matrix };
    let matrix = &mut matrix.0;
    matrix.set_m43(m43)
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_get_m44(matrix: *const Matrix) -> f32 {
    let matrix = unsafe { &*matrix };
    let matrix = &matrix.0;
    matrix.m44()
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_set_m44(matrix: *mut Matrix, m44: f32) {
    let matrix = unsafe { &mut *matrix };
    let matrix = &mut matrix.0;
    matrix.set_m44(m44)
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_translate(
    x: f32,
    y: f32,
    matrix: *const Matrix,
) -> *mut Matrix {
    assert!(!matrix.is_null());
    let matrix = unsafe { &*matrix };
    let matrix = &matrix.0;
    let ret = canvas_2d::context::matrix::Matrix::translate(x, y, &matrix);

    Box::into_raw(Box::new(Matrix(ret)))
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_translate_self(matrix: *mut Matrix, x: f32, y: f32) {
    assert!(!matrix.is_null());
    let matrix = unsafe { &mut *matrix };
    let matrix = &mut matrix.0;
    matrix.translate_self(x, y);
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_multiply_self(matrix: *mut Matrix, value: *const Matrix) {
    assert!(!matrix.is_null());
    let matrix = unsafe { &mut *matrix };
    let value = unsafe { &*value };
    let value = &value.0;
    let matrix = &mut matrix.0;
    matrix.multiply_self(&value);
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_premultiply_self(matrix: *mut Matrix, value: *const Matrix) {
    assert!(!matrix.is_null());
    let matrix = unsafe { &mut *matrix };
    let value = unsafe { &*value };
    let value = &value.0;
    let matrix = &mut matrix.0;
    matrix.premultiply_self(&value);
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_scale_non_uniform(
    sx: f32,
    sy: f32,
    matrix: *const Matrix,
) -> *mut Matrix {
    assert!(!matrix.is_null());
    let matrix = unsafe { &*matrix };
    let ret = canvas_2d::context::matrix::Matrix::scale_non_uniform(sx, sy, &matrix.0);

    Box::into_raw(Box::new(Matrix(ret)))
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_scale_non_uniform_self(
    matrix: *mut Matrix,
    sx: f32,
    sy: f32,
) {
    assert!(!matrix.is_null());
    let matrix = unsafe { &mut *matrix };
    let matrix = &mut matrix.0;
    matrix.scale_non_uniform_self(sx, sy);
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_rotate(
    angle: f32,
    cx: f32,
    cy: f32,
    matrix: *const Matrix,
) -> *mut Matrix {
    assert!(!matrix.is_null());
    let matrix = unsafe { &*matrix };
    let matrix = &matrix.0;
    let ret = canvas_2d::context::matrix::Matrix::rotate(angle, cx, cy, &matrix);
    Box::into_raw(Box::new(Matrix(ret)))
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_rotate_self(
    matrix: *mut Matrix,
    angle: f32,
    cx: f32,
    cy: f32,
) {
    assert!(!matrix.is_null());
    let matrix = unsafe { &mut *matrix };
    let matrix = &mut matrix.0;
    matrix.rotate_self(angle, cx, cy);
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_skew_x(angle: f32, matrix: *const Matrix) -> *mut Matrix {
    assert!(!matrix.is_null());
    let matrix = unsafe { &*matrix };
    let matrix = &matrix.0;
    let ret = canvas_2d::context::matrix::Matrix::skew_x_matrix(angle, &matrix);
    Box::into_raw(Box::new(Matrix(ret)))
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_skew_x_self(matrix: *mut Matrix, angle: f32) {
    assert!(!matrix.is_null());
    let matrix = unsafe { &mut *matrix };
    let matrix = &mut matrix.0;
    matrix.skew_x_self(angle);
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_skew_y(angle: f32, matrix: *const Matrix) -> *mut Matrix {
    assert!(!matrix.is_null());
    let matrix = unsafe { &*matrix };
    let matrix = &matrix.0;
    let ret = canvas_2d::context::matrix::Matrix::skew_y_matrix(angle, &matrix);
    Box::into_raw(Box::new(Matrix(ret)))
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_skew_y_self(matrix: *mut Matrix, angle: f32) {
    assert!(!matrix.is_null());
    let matrix = unsafe { &mut *matrix };
    let matrix = &mut matrix.0;
    matrix.skew_y_self(angle);
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_clone(matrix: *const Matrix) -> *mut Matrix {
    assert!(!matrix.is_null());
    let matrix = unsafe { &*matrix };
    let matrix = &matrix.0;
    let ret = Matrix(canvas_2d::context::matrix::Matrix::from(
        matrix.clone_inner(),
    ));
    Box::into_raw(Box::new(ret))
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_release(value: *mut Matrix) {
    if value.is_null() {
        return;
    }
    unsafe { drop(Box::from_raw(value)) };
}
