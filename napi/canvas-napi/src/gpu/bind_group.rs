use std::ffi::CString;
use std::sync::Arc;

#[napi]
pub struct g_p_u_bind_group {
  pub(crate) group: Arc<canvas_c::webgpu::gpu_bind_group::CanvasGPUBindGroup>,
}

#[napi]
impl g_p_u_bind_group {
  #[napi(getter)]
  pub fn get_label(&self) -> String {
    let label = unsafe {
      canvas_c::webgpu::gpu_bind_group::canvas_native_webgpu_bind_group_get_label(Arc::as_ptr(
        &self.group,
      ))
    };
    if label.is_null() {
      return String::new();
    }
    unsafe { CString::from_raw(label).into_string().unwrap() }
  }
}
