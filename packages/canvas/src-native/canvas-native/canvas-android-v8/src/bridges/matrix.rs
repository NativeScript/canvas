#[derive(Clone)]
pub struct Matrix(canvas_core::context::matrix::Matrix);

impl Matrix {
    pub fn inner(&self) -> &canvas_core::context::matrix::Matrix {
        &self.0
    }

    pub fn inner_mut(&mut self) -> &mut canvas_core::context::matrix::Matrix {
        &mut self.0
    }
}

#[cxx::bridge]
mod ffi {
    extern "Rust" {
        type Matrix;

        fn canvas_native_matrix_create() -> Box<Matrix>;

        fn canvas_native_matrix_update(matrix: &mut Matrix, slice: &[f32]);

        fn canvas_native_matrix_get_a(matrix: &Matrix) -> f32;

        fn canvas_native_matrix_set_a(matrix: &mut Matrix, a: f32);

        fn canvas_native_matrix_get_b(matrix: &Matrix) -> f32;

        fn canvas_native_matrix_set_b(matrix: &mut Matrix, b: f32);

        fn canvas_native_matrix_get_c(matrix: &Matrix) -> f32;

        fn canvas_native_matrix_set_c(matrix: &mut Matrix, c: f32);

        fn canvas_native_matrix_get_d(matrix: &Matrix) -> f32;

        fn canvas_native_matrix_set_d(matrix: &mut Matrix, d: f32);

        fn canvas_native_matrix_get_e(matrix: &Matrix) -> f32;

        fn canvas_native_matrix_set_e(matrix: &mut Matrix, e: f32);

        fn canvas_native_matrix_get_f(matrix: &Matrix) -> f32;

        fn canvas_native_matrix_set_f(matrix: &mut Matrix, f: f32);

        fn canvas_native_matrix_get_m11(matrix: &Matrix) -> f32;

        fn canvas_native_matrix_set_m11(matrix: &mut Matrix, m11: f32);

        fn canvas_native_matrix_get_m12(matrix: &Matrix) -> f32;

        fn canvas_native_matrix_set_m12(matrix: &mut Matrix, m12: f32);

        fn canvas_native_matrix_get_m13(matrix: &Matrix) -> f32;

        fn canvas_native_matrix_set_m13(matrix: &mut Matrix, m13: f32);

        fn canvas_native_matrix_get_m14(matrix: &Matrix) -> f32;

        fn canvas_native_matrix_set_m14(matrix: &mut Matrix, m14: f32);

        fn canvas_native_matrix_get_m21(matrix: &Matrix) -> f32;

        fn canvas_native_matrix_set_m21(matrix: &mut Matrix, m21: f32);

        fn canvas_native_matrix_get_m22(matrix: &Matrix) -> f32;

        fn canvas_native_matrix_set_m22(matrix: &mut Matrix, m22: f32);

        fn canvas_native_matrix_get_m23(matrix: &Matrix) -> f32;

        fn canvas_native_matrix_set_m23(matrix: &mut Matrix, m23: f32);

        fn canvas_native_matrix_get_m24(matrix: &Matrix) -> f32;

        fn canvas_native_matrix_set_m24(matrix: &mut Matrix, m24: f32);

        fn canvas_native_matrix_get_m31(matrix: &Matrix) -> f32;

        fn canvas_native_matrix_set_m31(matrix: &mut Matrix, m31: f32);

        fn canvas_native_matrix_get_m32(matrix: &Matrix) -> f32;

        fn canvas_native_matrix_set_m32(matrix: &mut Matrix, m32: f32);

        fn canvas_native_matrix_get_m33(matrix: &Matrix) -> f32;

        fn canvas_native_matrix_set_m33(matrix: &mut Matrix, m33: f32);

        fn canvas_native_matrix_get_m34(matrix: &Matrix) -> f32;

        fn canvas_native_matrix_set_m34(matrix: &mut Matrix, m34: f32);

        fn canvas_native_matrix_get_m41(matrix: &Matrix) -> f32;

        fn canvas_native_matrix_set_m41(matrix: &mut Matrix, m41: f32);

        fn canvas_native_matrix_get_m42(matrix: &Matrix) -> f32;

        fn canvas_native_matrix_set_m42(matrix: &mut Matrix, m42: f32);

        fn canvas_native_matrix_get_m43(matrix: &Matrix) -> f32;

        fn canvas_native_matrix_set_m43(matrix: &mut Matrix, m43: f32);

        fn canvas_native_matrix_get_m44(matrix: &Matrix) -> f32;

        fn canvas_native_matrix_set_m44(matrix: &mut Matrix, m44: f32);
    }
}

pub fn canvas_native_matrix_create() -> Box<Matrix> {
    Box::new(Matrix::new())
}

pub fn canvas_native_matrix_update(matrix: &mut Matrix, slice: &[f32]) {
    let mut affine = [0f32; 6];
    affine.copy_from_slice(slice);
    matrix.inner_mut().set_affine(&affine);
}

pub fn canvas_native_matrix_get_a(matrix: &Matrix) -> f32 {
    matrix.inner().a()
}

pub fn canvas_native_matrix_set_a(matrix: &mut Matrix, a: f32) {
    matrix.inner_mut().set_a(a)
}

pub fn canvas_native_matrix_get_b(matrix: &Matrix) -> f32 {
    matrix.inner().b()
}

pub fn canvas_native_matrix_set_b(matrix: &mut Matrix, b: f32) {
    matrix.inner_mut().set_b(b)
}

pub fn canvas_native_matrix_get_c(matrix: &Matrix) -> f32 {
    matrix.inner().c()
}

pub fn canvas_native_matrix_set_c(matrix: &mut Matrix, c: f32) {
    matrix.inner_mut().set_c(c)
}

pub fn canvas_native_matrix_get_d(matrix: &Matrix) -> f32 {
    matrix.inner().d()
}

pub fn canvas_native_matrix_set_d(matrix: &mut Matrix, d: f32) {
    matrix.inner_mut().set_d(d)
}

pub fn canvas_native_matrix_get_e(matrix: &Matrix) -> f32 {
    matrix.inner().e()
}

pub fn canvas_native_matrix_set_e(matrix: &mut Matrix, e: f32) {
    matrix.inner_mut().set_e(e)
}

pub fn canvas_native_matrix_get_f(matrix: &Matrix) -> f32 {
    matrix.inner().f()
}

pub fn canvas_native_matrix_set_f(matrix: &mut Matrix, f: f32) {
    matrix.inner_mut().set_f(f)
}

pub fn canvas_native_matrix_get_m11(matrix: &Matrix) -> f32 {
    matrix.inner().m11()
}

pub fn canvas_native_matrix_set_m11(matrix: &mut Matrix, m11: f32) {
    matrix.inner_mut().set_m11(m11)
}

pub fn canvas_native_matrix_get_m12(matrix: &Matrix) -> f32 {
    matrix.inner().m12()
}

pub fn canvas_native_matrix_set_m12(matrix: &mut Matrix, m12: f32) {
    matrix.inner_mut().set_m12(m12)
}

pub fn canvas_native_matrix_get_m13(matrix: &Matrix) -> f32 {
    matrix.inner().m13()
}

pub fn canvas_native_matrix_set_m13(matrix: &mut Matrix, m13: f32) {
    matrix.inner_mut().set_m13(m13)
}

pub fn canvas_native_matrix_get_m14(matrix: &Matrix) -> f32 {
    matrix.inner().m14()
}

pub fn canvas_native_matrix_set_m14(matrix: &mut Matrix, m14: f32) {
    matrix.inner_mut().set_m14(m14)
}

pub fn canvas_native_matrix_get_m21(matrix: &Matrix) -> f32 {
    matrix.inner().m21()
}

pub fn canvas_native_matrix_set_m21(matrix: &mut Matrix, m21: f32) {
    matrix.inner_mut().set_m21(m21)
}

pub fn canvas_native_matrix_get_m22(matrix: &Matrix) -> f32 {
    matrix.inner().m22()
}

pub fn canvas_native_matrix_set_m22(matrix: &mut Matrix, m22: f32) {
    matrix.inner_mut().set_m22(m22)
}

pub fn canvas_native_matrix_get_m23(matrix: &Matrix) -> f32 {
    matrix.inner().m23()
}

pub fn canvas_native_matrix_set_m23(matrix: &mut Matrix, m23: f32) {
    matrix.inner_mut().set_m23(m23)
}

pub fn canvas_native_matrix_get_m24(matrix: &Matrix) -> f32 {
    matrix.inner().m24()
}

pub fn canvas_native_matrix_set_m24(matrix: &mut Matrix, m24: f32) {
    matrix.inner_mut().set_m24(m24)
}

pub fn canvas_native_matrix_get_m31(matrix: &Matrix) -> f32 {
    matrix.inner().m31()
}

pub fn canvas_native_matrix_set_m31(matrix: &mut Matrix, m31: f32) {
    matrix.inner_mut().set_m31(m31)
}

pub fn canvas_native_matrix_get_m32(matrix: &Matrix) -> f32 {
    matrix.inner().m32()
}

pub fn canvas_native_matrix_set_m32(matrix: &mut Matrix, m32: f32) {
    matrix.inner_mut().set_m32(m32)
}

pub fn canvas_native_matrix_get_m33(matrix: &Matrix) -> f32 {
    matrix.inner().m33()
}

pub fn canvas_native_matrix_set_m33(matrix: &mut Matrix, m33: f32) {
    matrix.inner_mut().set_m33(m33)
}

pub fn canvas_native_matrix_get_m34(matrix: &Matrix) -> f32 {
    matrix.inner().m34()
}

pub fn canvas_native_matrix_set_m34(matrix: &mut Matrix, m34: f32) {
    matrix.inner_mut().set_m34(m34)
}

pub fn canvas_native_matrix_get_m41(matrix: &Matrix) -> f32 {
    matrix.inner().m41()
}

pub fn canvas_native_matrix_set_m41(matrix: &mut Matrix, m41: f32) {
    matrix.inner_mut().set_m41(m41)
}

pub fn canvas_native_matrix_get_m42(matrix: &Matrix) -> f32 {
    matrix.m42()
}

pub fn canvas_native_matrix_set_m42(matrix: &mut Matrix, m42: f32) {
    matrix.inner_mut().set_m42(m42)
}

pub fn canvas_native_matrix_get_m43(matrix: &Matrix) -> f32 {
    matrix.inner().m43()
}

pub fn canvas_native_matrix_set_m43(matrix: &mut Matrix, m43: f32) {
    matrix.inner_mut().set_m43(m43)
}

pub fn canvas_native_matrix_get_m44(matrix: &Matrix) -> f32 {
    matrix.inner().m44()
}

pub fn canvas_native_matrix_set_m44(matrix: &mut Matrix, m44: f32) {
    matrix.inner_mut().set_m44(m44)
}
