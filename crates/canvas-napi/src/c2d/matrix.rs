//! `DOMMatrix`, mirroring `canvas2d/MatrixImpl.cpp` in the V8 bindings.

use canvas_c::Matrix;
use napi::sys;

use crate::constructor;
use crate::util::class::ClassDef;
use crate::util::native::{Native, NativeType};

pub struct DOMMatrix {
    pub(crate) matrix: *mut Matrix,
}

impl Native for DOMMatrix {
    const KIND: NativeType = NativeType::Matrix;
}

impl Drop for DOMMatrix {
    fn drop(&mut self) {
        canvas_c::canvas_native_matrix_release(self.matrix);
    }
}

constructor!(ctor, DOMMatrix, 1, |cx| {
    let matrix = canvas_c::canvas_native_matrix_create();
    (!matrix.is_null()).then_some(DOMMatrix { matrix })
});

pub unsafe fn init(env: sys::napi_env, exports: sys::napi_value) {
    ClassDef::new(c"DOMMatrix", ctor).define(env, exports, NativeType::Matrix);
}
