use canvas_c::PaintStyle;
use napi::bindgen_prelude::ObjectFinalize;
use napi::*;
use napi_derive::napi;

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
impl CanvasPattern {}