use napi::bindgen_prelude::ObjectFinalize;
use napi::*;
use napi_derive::napi;

#[napi(js_name = "WebGLSync", custom_finalize)]
pub struct WebGLSync(pub(crate) *const canvas_c::WebGLSync);

impl ObjectFinalize for WebGLSync {
    fn finalize(self, _: Env) -> Result<()> {
        canvas_c::canvas_native_webgl2_sync_destroy(self.0);
        Ok(())
    }
}

