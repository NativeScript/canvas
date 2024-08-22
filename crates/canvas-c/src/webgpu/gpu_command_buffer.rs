use std::borrow::Cow;
use std::os::raw::c_char;
use std::sync::Arc;

use crate::webgpu::prelude::label_to_ptr;

//use wgpu_core::gfx_select;
use super::gpu::CanvasWebGPUInstance;

pub struct CanvasGPUCommandBuffer {
    pub(crate) label: Option<Cow<'static, str>>,
    pub(crate) instance: Arc<CanvasWebGPUInstance>,
    pub(crate) command_buffer: wgpu_core::id::CommandBufferId,
    pub(crate) open: std::sync::atomic::AtomicBool,
}

impl Drop for CanvasGPUCommandBuffer {
    fn drop(&mut self) {
        if self.open.load(std::sync::atomic::Ordering::SeqCst) && !std::thread::panicking() {
            let context = self.instance.global();
            let command_buffer = self.command_buffer;
            gfx_select!(command_buffer => context.command_buffer_drop(command_buffer));
            // let mut lock = self.command_buffer.lock();
            // if let Some(command_buffer) = lock.take() {
            //     gfx_select!(self.id => context.command_buffer_drop(command_buffer));
            // }
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_command_buffer_get_label(
    command_buffer: *const CanvasGPUCommandBuffer,
) -> *mut c_char {
    if command_buffer.is_null() {
        return std::ptr::null_mut();
    }
    let command_buffer = &*command_buffer;
    label_to_ptr(command_buffer.label.clone())
}


#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_command_buffer_reference(
    command_buffer: *const CanvasGPUCommandBuffer,
) {
    if command_buffer.is_null() {
        return;
    }

    Arc::increment_strong_count(command_buffer);
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_command_buffer_release(
    command_buffer: *const CanvasGPUCommandBuffer,
) {
    if command_buffer.is_null() {
        return;
    }

    Arc::decrement_strong_count(command_buffer);
}
