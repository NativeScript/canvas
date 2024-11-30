use napi::bindgen_prelude::ObjectFinalize;
use napi::*;
use napi_derive::napi;

#[napi(custom_finalize)]
pub struct g_p_u_canvas_context {
    context: *const canvas_c::webgpu::gpu_canvas_context::CanvasGPUCanvasContext,
}

impl ObjectFinalize for g_p_u_canvas_context {
    fn finalize(self, _: Env) -> Result<()> {
        unsafe { canvas_c::webgpu::gpu_canvas_context::canvas_native_webgpu_context_release(self.context); }
        Ok(())
    }
}

impl  g_p_u_canvas_context {}