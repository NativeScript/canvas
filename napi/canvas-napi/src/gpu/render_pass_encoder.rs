use napi_derive::napi;
use std::ffi::CString;

#[napi(js_name = "GPURenderPassEncoder")]
pub struct GPURenderPassEncoder {
    pub(crate) encoder: *const canvas_c::webgpu::gpu_render_pass_encoder::CanvasGPURenderPassEncoder,
}

#[napi]
impl GPURenderPassEncoder {
    #[napi(getter)]
    pub fn get_label(&self) -> String {
        let label = unsafe { canvas_c::webgpu::gpu_render_pass_encoder::canvas_native_webgpu_render_pass_encoder_get_label(self.encoder) };
        if label.is_null() {
            return String::new();
        }
        unsafe {
            CString::from_raw(label).into_string().unwrap()
        }
    }
}