use std::ptr;

use canvas_c::Matrix;
use napi::bindgen_prelude::{FromNapiValue, ObjectFinalize, TypeName, Unknown};
use napi::*;

use crate::module::{downcast, property};

#[napi(custom_finalize, js_name = "DOMMatrix")]
pub struct DOMMatrix {
    pub(crate) matrix: *mut canvas_c::Matrix,
}

impl ObjectFinalize for DOMMatrix {
    fn finalize(self, _: Env) -> Result<()> {
        canvas_c::canvas_native_matrix_release(self.matrix);
        Ok(())
    }
}

/// A DOMMatrix argument: the native object, or a `packages/canvas` wrapper around one (its
/// `native`). Anything else reads as no matrix (null), which the V8 bindings ignore.
pub struct MatrixArg(pub(crate) *mut Matrix);

impl MatrixArg {
    pub(crate) fn get(&self) -> Option<*mut Matrix> {
        (!self.0.is_null()).then_some(self.0)
    }
}

impl TypeName for MatrixArg {
    fn type_name() -> &'static str {
        "DOMMatrix"
    }

    fn value_type() -> ValueType {
        ValueType::Object
    }
}

impl FromNapiValue for MatrixArg {
    unsafe fn from_napi_value(env: sys::napi_env, value: sys::napi_value) -> Result<Self> {
        let value = unsafe { Unknown::from_raw_unchecked(env, value) };
        if let Some(matrix) = downcast::<DOMMatrix>(&value) {
            return Ok(MatrixArg(matrix.matrix));
        }
        let wrapped = property(&value, c"native");
        Ok(MatrixArg(
            wrapped
                .as_ref()
                .and_then(downcast::<DOMMatrix>)
                .map_or(ptr::null_mut(), |matrix| matrix.matrix),
        ))
    }
}

impl DOMMatrix {
    fn from_raw(matrix: *mut Matrix) -> DOMMatrix {
        DOMMatrix { matrix }
    }

    /// The matrix the non-mutating methods read: the one passed last (as `packages/canvas` calls
    /// them, `m.translate(x, y, m)`), else this one.
    fn source(&self, matrix: Option<MatrixArg>) -> *const Matrix {
        matrix.and_then(|m| m.get()).unwrap_or(self.matrix)
    }

    /// Runs `f` with `matrix`, copied first when it is this matrix (`m.multiplySelf(m)`), since
    /// canvas-c reads the operand while it writes this one.
    fn with_operand(&self, matrix: *mut Matrix, f: impl FnOnce(*const Matrix)) {
        if matrix == self.matrix {
            let copy = canvas_c::canvas_native_matrix_clone(matrix);
            f(copy);
            canvas_c::canvas_native_matrix_release(copy);
        } else {
            f(matrix);
        }
    }
}

#[napi]
impl DOMMatrix {
    /// `new DOMMatrix()`, `new DOMMatrix([a, b, c, d, e, f])` or a 16-value column-major array.
    #[napi(constructor)]
    pub fn new(data: Option<Vec<f64>>) -> DOMMatrix {
        let matrix = canvas_c::canvas_native_matrix_create();
        if let Some(init) = data {
            let init = init.into_iter().map(|v| v as f32).collect::<Vec<f32>>();
            match init.len() {
                6 => {
                    canvas_c::canvas_native_matrix_update(matrix, init.as_ptr(), init.len())
                }
                16 => {
                    canvas_c::canvas_native_matrix_update_3d(matrix, init.as_ptr(), init.len())
                }
                _ => {}
            }
        }
        DOMMatrix { matrix }
    }

    #[napi(getter)]
    pub fn a(&self) -> f64 {
        canvas_c::canvas_native_matrix_get_a(self.matrix) as f64
    }

    #[napi(setter)]
    pub fn set_a(&self, value: f64) {
        canvas_c::canvas_native_matrix_set_a(self.matrix, value as f32)
    }

    #[napi(getter)]
    pub fn b(&self) -> f64 {
        canvas_c::canvas_native_matrix_get_b(self.matrix) as f64
    }

    #[napi(setter)]
    pub fn set_b(&self, value: f64) {
        canvas_c::canvas_native_matrix_set_b(self.matrix, value as f32)
    }

    #[napi(getter)]
    pub fn c(&self) -> f64 {
        canvas_c::canvas_native_matrix_get_c(self.matrix) as f64
    }

    #[napi(setter)]
    pub fn set_c(&self, value: f64) {
        canvas_c::canvas_native_matrix_set_c(self.matrix, value as f32)
    }

    #[napi(getter)]
    pub fn d(&self) -> f64 {
        canvas_c::canvas_native_matrix_get_d(self.matrix) as f64
    }

    #[napi(setter)]
    pub fn set_d(&self, value: f64) {
        canvas_c::canvas_native_matrix_set_d(self.matrix, value as f32)
    }

    #[napi(getter)]
    pub fn e(&self) -> f64 {
        canvas_c::canvas_native_matrix_get_e(self.matrix) as f64
    }

    #[napi(setter)]
    pub fn set_e(&self, value: f64) {
        canvas_c::canvas_native_matrix_set_e(self.matrix, value as f32)
    }

    #[napi(getter)]
    pub fn f(&self) -> f64 {
        canvas_c::canvas_native_matrix_get_f(self.matrix) as f64
    }

    #[napi(setter)]
    pub fn set_f(&self, value: f64) {
        canvas_c::canvas_native_matrix_set_f(self.matrix, value as f32)
    }

    #[napi(getter)]
    pub fn m11(&self) -> f64 {
        canvas_c::canvas_native_matrix_get_m11(self.matrix) as f64
    }

    #[napi(setter)]
    pub fn set_m11(&self, value: f64) {
        canvas_c::canvas_native_matrix_set_m11(self.matrix, value as f32)
    }

    #[napi(getter)]
    pub fn m12(&self) -> f64 {
        canvas_c::canvas_native_matrix_get_m12(self.matrix) as f64
    }

    #[napi(setter)]
    pub fn set_m12(&self, value: f64) {
        canvas_c::canvas_native_matrix_set_m12(self.matrix, value as f32)
    }

    #[napi(getter)]
    pub fn m13(&self) -> f64 {
        canvas_c::canvas_native_matrix_get_m13(self.matrix) as f64
    }

    #[napi(setter)]
    pub fn set_m13(&self, value: f64) {
        canvas_c::canvas_native_matrix_set_m13(self.matrix, value as f32)
    }

    #[napi(getter)]
    pub fn m14(&self) -> f64 {
        canvas_c::canvas_native_matrix_get_m14(self.matrix) as f64
    }

    #[napi(setter)]
    pub fn set_m14(&self, value: f64) {
        canvas_c::canvas_native_matrix_set_m14(self.matrix, value as f32)
    }

    #[napi(getter)]
    pub fn m21(&self) -> f64 {
        canvas_c::canvas_native_matrix_get_m21(self.matrix) as f64
    }

    #[napi(setter)]
    pub fn set_m21(&self, value: f64) {
        canvas_c::canvas_native_matrix_set_m21(self.matrix, value as f32)
    }

    #[napi(getter)]
    pub fn m22(&self) -> f64 {
        canvas_c::canvas_native_matrix_get_m22(self.matrix) as f64
    }

    #[napi(setter)]
    pub fn set_m22(&self, value: f64) {
        canvas_c::canvas_native_matrix_set_m22(self.matrix, value as f32)
    }

    #[napi(getter)]
    pub fn m23(&self) -> f64 {
        canvas_c::canvas_native_matrix_get_m23(self.matrix) as f64
    }

    #[napi(setter)]
    pub fn set_m23(&self, value: f64) {
        canvas_c::canvas_native_matrix_set_m23(self.matrix, value as f32)
    }

    #[napi(getter)]
    pub fn m24(&self) -> f64 {
        canvas_c::canvas_native_matrix_get_m24(self.matrix) as f64
    }

    #[napi(setter)]
    pub fn set_m24(&self, value: f64) {
        canvas_c::canvas_native_matrix_set_m24(self.matrix, value as f32)
    }

    #[napi(getter)]
    pub fn m31(&self) -> f64 {
        canvas_c::canvas_native_matrix_get_m31(self.matrix) as f64
    }

    #[napi(setter)]
    pub fn set_m31(&self, value: f64) {
        canvas_c::canvas_native_matrix_set_m31(self.matrix, value as f32)
    }

    #[napi(getter)]
    pub fn m32(&self) -> f64 {
        canvas_c::canvas_native_matrix_get_m32(self.matrix) as f64
    }

    #[napi(setter)]
    pub fn set_m32(&self, value: f64) {
        canvas_c::canvas_native_matrix_set_m32(self.matrix, value as f32)
    }

    #[napi(getter)]
    pub fn m33(&self) -> f64 {
        canvas_c::canvas_native_matrix_get_m33(self.matrix) as f64
    }

    #[napi(setter)]
    pub fn set_m33(&self, value: f64) {
        canvas_c::canvas_native_matrix_set_m33(self.matrix, value as f32)
    }

    #[napi(getter)]
    pub fn m34(&self) -> f64 {
        canvas_c::canvas_native_matrix_get_m34(self.matrix) as f64
    }

    #[napi(setter)]
    pub fn set_m34(&self, value: f64) {
        canvas_c::canvas_native_matrix_set_m34(self.matrix, value as f32)
    }

    #[napi(getter)]
    pub fn m41(&self) -> f64 {
        canvas_c::canvas_native_matrix_get_m41(self.matrix) as f64
    }

    #[napi(setter)]
    pub fn set_m41(&self, value: f64) {
        canvas_c::canvas_native_matrix_set_m41(self.matrix, value as f32)
    }

    #[napi(getter)]
    pub fn m42(&self) -> f64 {
        canvas_c::canvas_native_matrix_get_m42(self.matrix) as f64
    }

    #[napi(setter)]
    pub fn set_m42(&self, value: f64) {
        canvas_c::canvas_native_matrix_set_m42(self.matrix, value as f32)
    }

    #[napi(getter)]
    pub fn m43(&self) -> f64 {
        canvas_c::canvas_native_matrix_get_m43(self.matrix) as f64
    }

    #[napi(setter)]
    pub fn set_m43(&self, value: f64) {
        canvas_c::canvas_native_matrix_set_m43(self.matrix, value as f32)
    }

    #[napi(getter)]
    pub fn m44(&self) -> f64 {
        canvas_c::canvas_native_matrix_get_m44(self.matrix) as f64
    }

    #[napi(setter)]
    pub fn set_m44(&self, value: f64) {
        canvas_c::canvas_native_matrix_set_m44(self.matrix, value as f32)
    }

    /// A new matrix: `matrix` (default: this one) translated.
    #[napi(ts_args_type = "x: number, y: number, matrix?: DOMMatrix")]
    pub fn translate(&self, x: f64, y: f64, matrix: Option<MatrixArg>) -> DOMMatrix {
        DOMMatrix::from_raw(canvas_c::canvas_native_matrix_translate(
            x as f32,
            y as f32,
            self.source(matrix),
        ))
    }

    #[napi]
    pub fn translate_self(&self, x: f64, y: f64) {
        canvas_c::canvas_native_matrix_translate_self(self.matrix, x as f32, y as f32)
    }

    /// `this = this × matrix`; a non-matrix is ignored.
    #[napi(ts_args_type = "matrix: DOMMatrix")]
    pub fn multiply_self(&self, matrix: Option<MatrixArg>) {
        if let Some(matrix) = matrix.and_then(|m| m.get()) {
            self.with_operand(matrix, |m| canvas_c::canvas_native_matrix_multiply_self(self.matrix, m))
        }
    }

    /// `this = matrix × this`; a non-matrix is ignored.
    #[napi(ts_args_type = "matrix: DOMMatrix")]
    pub fn premultiply_self(&self, matrix: Option<MatrixArg>) {
        if let Some(matrix) = matrix.and_then(|m| m.get()) {
            self.with_operand(matrix, |m| canvas_c::canvas_native_matrix_premultiply_self(self.matrix, m))
        }
    }

    /// A new matrix: `matrix` (default: this one) scaled.
    #[napi(ts_args_type = "sx: number, sy: number, matrix?: DOMMatrix")]
    pub fn scale_non_uniform(&self, sx: f64, sy: f64, matrix: Option<MatrixArg>) -> DOMMatrix {
        DOMMatrix::from_raw(canvas_c::canvas_native_matrix_scale_non_uniform(
            sx as f32,
            sy as f32,
            self.source(matrix),
        ))
    }

    #[napi]
    pub fn scale_non_uniform_self(&self, sx: f64, sy: f64) {
        canvas_c::canvas_native_matrix_scale_non_uniform_self(self.matrix, sx as f32, sy as f32)
    }

    /// A new matrix: `matrix` (default: this one) rotated by `angle` degrees about `(cx, cy)`.
    #[napi(ts_args_type = "angle: number, cx?: number, cy?: number, matrix?: DOMMatrix")]
    pub fn rotate(
        &self,
        angle: f64,
        cx: Option<f64>,
        cy: Option<f64>,
        matrix: Option<MatrixArg>,
    ) -> DOMMatrix {
        DOMMatrix::from_raw(canvas_c::canvas_native_matrix_rotate(
            angle as f32,
            cx.unwrap_or(0.) as f32,
            cy.unwrap_or(0.) as f32,
            self.source(matrix),
        ))
    }

    #[napi]
    pub fn rotate_self(&self, angle: f64, cx: Option<f64>, cy: Option<f64>) {
        canvas_c::canvas_native_matrix_rotate_self(
            self.matrix,
            angle as f32,
            cx.unwrap_or(0.) as f32,
            cy.unwrap_or(0.) as f32,
        )
    }

    /// A new matrix: `matrix` (default: this one) skewed along x.
    #[napi(js_name = "skewX", ts_args_type = "angle: number, matrix?: DOMMatrix")]
    pub fn skew_x(&self, angle: f64, matrix: Option<MatrixArg>) -> DOMMatrix {
        DOMMatrix::from_raw(canvas_c::canvas_native_matrix_skew_x(
            angle as f32,
            self.source(matrix),
        ))
    }

    #[napi(js_name = "skewXSelf")]
    pub fn skew_x_self(&self, angle: f64) {
        canvas_c::canvas_native_matrix_skew_x_self(self.matrix, angle as f32)
    }

    /// A new matrix: `matrix` (default: this one) skewed along y.
    #[napi(js_name = "skewY", ts_args_type = "angle: number, matrix?: DOMMatrix")]
    pub fn skew_y(&self, angle: f64, matrix: Option<MatrixArg>) -> DOMMatrix {
        DOMMatrix::from_raw(canvas_c::canvas_native_matrix_skew_y(
            angle as f32,
            self.source(matrix),
        ))
    }

    #[napi(js_name = "skewYSelf")]
    pub fn skew_y_self(&self, angle: f64) {
        canvas_c::canvas_native_matrix_skew_y_self(self.matrix, angle as f32)
    }
}
