use canvas_c::PaintStyle;
use napi::bindgen_prelude::ObjectFinalize;
use napi::*;
use napi_derive::napi;

use crate::dom_matrix::MatrixArg;

#[napi(custom_finalize)]
pub struct CanvasPattern {
    pub(crate) style: *mut PaintStyle,
}

impl ObjectFinalize for CanvasPattern {
    fn finalize(self, _: Env) -> Result<()> {
        canvas_c::canvas_native_paint_style_release(self.style);
        Ok(())
    }
}


#[napi]
impl CanvasPattern {
    /// `setTransform(matrix)`: a DOMMatrix; anything else is ignored.
    #[napi(ts_args_type = "matrix: DOMMatrix")]
    pub fn set_transform(&self, matrix: Option<MatrixArg>) {
        if let Some(matrix) = matrix.and_then(|m| m.get()) {
            canvas_c::canvas_native_pattern_set_transform(self.style, matrix);
        }
    }
}