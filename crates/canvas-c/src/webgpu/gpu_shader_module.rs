use std::borrow::Cow;
use std::os::raw::c_char;
use std::sync::Arc;

use crate::webgpu::prelude::label_to_ptr;

//use wgpu_core::gfx_select;
use super::gpu::CanvasWebGPUInstance;

pub struct CanvasGPUShaderModule {
    pub(crate) label: Option<Cow<'static, str>>,
    pub(crate) instance: Arc<CanvasWebGPUInstance>,
    pub(crate) module: wgpu_core::id::ShaderModuleId,
}

impl Drop for CanvasGPUShaderModule {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            let global = self.instance.global();
             global.shader_module_drop(self.module);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_shader_module_get_label(
    shader_module: *const CanvasGPUShaderModule,
) -> *mut c_char {
    if shader_module.is_null() {
        return std::ptr::null_mut();
    }

    let shader_module = &*shader_module;

    label_to_ptr(shader_module.label.clone())
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_shader_module_reference(
    shader_module: *const CanvasGPUShaderModule,
) {
    if shader_module.is_null() {
        return;
    }
    Arc::increment_strong_count(shader_module);
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_shader_module_release(
    shader_module: *const CanvasGPUShaderModule,
) {
    if shader_module.is_null() {
        return;
    }
    Arc::decrement_strong_count(shader_module);
}

/*#[no_mangle]
pub extern "C" fn canvas_native_webgpu_device_create_shader_module_get_compilation_info(
    shader_module: *const CanvasGPUShaderModule,
) -> *mut CanvasGPUBuffer {
    if shader_module.is_null() {
        return std::ptr::null_mut();
    }

    let shader_module = unsafe { &*shader_module };

    let global = shader_module.instance.global();



    device.create_buffer(label, size, usage, mapped_at_creation, error)
}
*/
