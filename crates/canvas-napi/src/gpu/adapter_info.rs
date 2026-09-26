use napi_derive::napi;
use std::sync::Arc;

#[napi(js_name = "GPUAdapterInfo")]
pub struct g_p_u_adapter_info {
  pub(crate) info: Arc<canvas_c::webgpu::gpu_adapter_info::CanvasGPUAdapterInfo>,
}

#[napi]
impl g_p_u_adapter_info {
  #[napi(getter)]
  pub fn get_architecture(&self) -> &str {
    self.info.architecture()
  }

  #[napi(getter)]
  pub fn get_description(&self) -> &str {
    self.info.description()
  }

  #[napi(getter)]
  pub fn get_device(&self) -> &str {
    self.info.device()
  }

  #[napi(getter)]
  pub fn get_vendor(&self) -> &str {
    self.info.vendor()
  }
}
