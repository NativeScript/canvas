use std::sync::Arc;

use super::gpu::CanvasWebGPUInstance;

pub struct CanvasGPUShaderModule {
    pub(crate) instance: Arc<CanvasWebGPUInstance>,
    pub(crate) module: wgpu_core::id::ShaderModuleId,
}


// #[no_mangle]
// pub extern "C" fn canvas_native_webgpu_device_create_shader_module_get_compilation_info(
//     shader_module: *const CanvasGPUShaderModule,
// ) -> *mut CanvasGPUBuffer {
//     if shader_module.is_null() {
//         return std::ptr::null_mut();
//     }

//     let shader_module = unsafe { &*shader_module };


//     device.create_buffer(label, size, usage, mapped_at_creation, error)
// }
