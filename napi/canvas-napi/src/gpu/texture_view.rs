use std::ffi::CString;

#[napi]
pub struct GPUTextureView {
    pub(crate) texture_view: *const canvas_c::webgpu::gpu_texture_view::CanvasGPUTextureView,
}

#[napi]
impl GPUTextureView {
    #[napi(getter)]
    pub fn get_label(&self) -> String {
        let label = unsafe { canvas_c::webgpu::gpu_texture_view::canvas_native_webgpu_texture_view_get_label(self.texture_view) };
        if label.is_null() {
            return String::new();
        }
        unsafe {
            CString::from_raw(label).into_string().unwrap()
        }
    }
}