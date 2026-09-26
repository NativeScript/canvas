use std::ffi::CStr;
use std::sync::Arc;

use canvas_c::webgpu::gpu_shader_module::{
  CanvasGPUCompilationInfo, CanvasGPUCompilationMessage, CanvasGPUCompilationMessageType,
};
use napi::bindgen_prelude::ToNapiValue;
use napi::{Env, Result};
use napi_derive::napi;

use crate::gpu::callback::resolved;
use crate::gpu::parse::take_string;
use crate::module::JsRaw;

#[napi(js_name = "GPUShaderModule")]
pub struct g_p_u_shader_module {
  pub(crate) module: Arc<canvas_c::webgpu::gpu_shader_module::CanvasGPUShaderModule>,
}

#[napi]
impl g_p_u_shader_module {
  #[napi(getter)]
  pub fn get_label(&self) -> String {
    unsafe {
      take_string(
        canvas_c::webgpu::gpu_shader_module::canvas_native_webgpu_shader_module_get_label(
          Arc::as_ptr(&self.module),
        ),
      )
    }
    .unwrap_or_default()
  }

  /// A promise of the module's `GPUCompilationInfo` (known once the module is created).
  #[napi(ts_return_type = "Promise<GPUCompilationInfo>")]
  pub fn get_compilation_info(&self, env: Env) -> Result<JsRaw> {
    let info = canvas_c::webgpu::gpu_shader_module::canvas_native_webgpu_device_create_shader_module_get_compilation_info(
      Arc::as_ptr(&self.module),
    );
    let info = g_p_u_compilation_info { info };
    let value = unsafe { g_p_u_compilation_info::to_napi_value(env.raw(), info) }?;
    Ok(JsRaw(resolved(&env, value)?))
  }
}

#[napi(js_name = "GPUCompilationInfo")]
pub struct g_p_u_compilation_info {
  info: *mut CanvasGPUCompilationInfo,
}

impl Drop for g_p_u_compilation_info {
  fn drop(&mut self) {
    canvas_c::webgpu::gpu_shader_module::canvas_native_webgpu_compilation_info_release(self.info);
  }
}

#[napi]
impl g_p_u_compilation_info {
  #[napi(getter)]
  pub fn get_messages(&self) -> Vec<g_p_u_compilation_message> {
    let count =
      canvas_c::webgpu::gpu_shader_module::canvas_native_webgpu_compilation_info_get_messages_count(
        self.info,
      );
    (0..count)
      .filter_map(|i| {
        let message =
          canvas_c::webgpu::gpu_shader_module::canvas_native_webgpu_compilation_info_get_message_at(
            self.info, i,
          );
        (!message.is_null()).then_some(g_p_u_compilation_message { message })
      })
      .collect()
  }
}

#[napi(js_name = "GPUCompilationMessage")]
pub struct g_p_u_compilation_message {
  message: *mut CanvasGPUCompilationMessage,
}

impl Drop for g_p_u_compilation_message {
  fn drop(&mut self) {
    canvas_c::webgpu::gpu_shader_module::canvas_native_webgpu_compilation_message_release(
      self.message,
    );
  }
}

#[napi]
impl g_p_u_compilation_message {
  #[napi(getter)]
  pub fn get_message(&self) -> String {
    let message = unsafe {
      canvas_c::webgpu::gpu_shader_module::canvas_native_webgpu_compilation_message_get_message(
        self.message,
      )
    };
    if message.is_null() {
      return String::new();
    }
    // Owned by the message.
    unsafe { CStr::from_ptr(message) }
      .to_string_lossy()
      .into_owned()
  }

  #[napi(getter, js_name = "type")]
  pub fn get_type(&self) -> &'static str {
    match canvas_c::webgpu::gpu_shader_module::canvas_native_webgpu_compilation_message_get_type(
      self.message,
    ) {
      CanvasGPUCompilationMessageType::Info => "info",
      CanvasGPUCompilationMessageType::Warning => "warning",
      CanvasGPUCompilationMessageType::Error => "error",
    }
  }

  #[napi(getter)]
  pub fn get_line_num(&self) -> f64 {
    canvas_c::webgpu::gpu_shader_module::canvas_native_webgpu_compilation_message_get_line_num(
      self.message,
    ) as f64
  }

  #[napi(getter)]
  pub fn get_line_pos(&self) -> f64 {
    canvas_c::webgpu::gpu_shader_module::canvas_native_webgpu_compilation_message_get_line_pos(
      self.message,
    ) as f64
  }

  #[napi(getter)]
  pub fn get_offset(&self) -> f64 {
    canvas_c::webgpu::gpu_shader_module::canvas_native_webgpu_compilation_message_get_offset(
      self.message,
    ) as f64
  }

  #[napi(getter)]
  pub fn get_length(&self) -> f64 {
    canvas_c::webgpu::gpu_shader_module::canvas_native_webgpu_compilation_message_get_length(
      self.message,
    ) as f64
  }
}
