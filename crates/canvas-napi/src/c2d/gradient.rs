use canvas_c::PaintStyle;
use napi::JsString;

use napi::*;
use napi::bindgen_prelude::ObjectFinalize;
use napi_derive::napi;

#[napi(custom_finalize)]
pub struct CanvasGradient {
    pub(crate) style: *mut PaintStyle,
}

impl ObjectFinalize for CanvasGradient {
    fn finalize(self, _: Env) -> Result<()> {
        canvas_c::canvas_native_paint_style_release(self.style);
        Ok(())
    }
}


#[napi]
impl CanvasGradient {
    #[napi]
    pub fn add_color_stop(&self, offset: f64, color: JsString) {
        if let Some(color) = color.into_utf8().ok() {
            if let Ok(color) = color.as_str() {
                let style = unsafe { &mut *self.style };
                style.add_color_stop(offset as f32, color);
            }
        }
    }
}
