use napi::bindgen_prelude::ObjectFinalize;
use napi::*;
use napi_derive::napi;
use std::ffi::CString;

#[napi(custom_finalize)]
pub struct g_p_u_shader_module {
  pub(crate) module: *const canvas_c::webgpu::gpu_shader_module::CanvasGPUShaderModule,
}

impl ObjectFinalize for g_p_u_shader_module {
  fn finalize(self, mut env: Env) -> Result<()> {
    unsafe {
      canvas_c::webgpu::gpu_shader_module::canvas_native_webgpu_shader_module_release(self.module)
    }
    Ok(())
  }
}

#[napi]
impl g_p_u_shader_module {
  #[napi(getter)]
  pub fn get_label(&self) -> String {
    let label = unsafe {
      canvas_c::webgpu::gpu_shader_module::canvas_native_webgpu_shader_module_get_label(
        self.module,
      )
    };
    if label.is_null() {
      return String::new();
    }
    unsafe { CString::from_raw(label).into_string().unwrap() }
  }
}
