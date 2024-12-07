use napi::bindgen_prelude::ObjectFinalize;
use napi::*;
use napi_derive::napi;

#[napi(js_name = "WebGLShaderPrecisionFormat", custom_finalize)]
pub struct WebGLShaderPrecisionFormat(pub(crate) *const canvas_c::WebGLShaderPrecisionFormat);

impl ObjectFinalize for WebGLShaderPrecisionFormat {
    fn finalize(self, _: Env) -> Result<()> {
        canvas_c::canvas_native_webgl_shader_precision_format_destroy(self.0 as _);
        Ok(())
    }
}


#[napi]
impl WebGLShaderPrecisionFormat {
    #[napi(getter)]
    pub fn get_precision(&self) -> i32 {
        canvas_c::canvas_native_webgl_shader_precision_format_get_precision(self.0)
    }

    #[napi(getter)]
    pub fn get_range_min(&self) -> i32 {
        canvas_c::canvas_native_webgl_shader_precision_format_get_range_min(self.0)
    }

    #[napi(getter)]
    pub fn get_range_max(&self) -> i32 {
        canvas_c::canvas_native_webgl_shader_precision_format_get_range_max(self.0)
    }
}