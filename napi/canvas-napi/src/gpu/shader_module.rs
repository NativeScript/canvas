use napi_derive::napi;
use std::ffi::CString;
use std::sync::Arc;

#[napi]
pub struct g_p_u_shader_module {
  pub(crate) module: Arc<canvas_c::webgpu::gpu_shader_module::CanvasGPUShaderModule>,
}

#[napi]
impl g_p_u_shader_module {
  #[napi(getter)]
  pub fn get_label(&self) -> String {
    let label = unsafe {
      canvas_c::webgpu::gpu_shader_module::canvas_native_webgpu_shader_module_get_label(
        Arc::as_ptr(&self.module),
      )
    };
    if label.is_null() {
      return String::new();
    }
    unsafe { CString::from_raw(label).into_string().unwrap() }
  }
}
