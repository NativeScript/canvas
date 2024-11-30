use napi::bindgen_prelude::ObjectFinalize;
use napi::Env;

#[napi(custom_finalize)]
pub struct web_g_l_2_rendering_context {
    state: *mut canvas_c::WebGLState,
}

impl ObjectFinalize for web_g_l_2_rendering_context {
    fn finalize(self, _: Env) -> napi::Result<()> {
        canvas_c::canvas_native_webgl_state_destroy(self.state);
        Ok(())
    }
}