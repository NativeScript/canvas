use std::borrow::Cow;
use std::os::raw::c_char;
use std::sync::Arc;

use crate::webgpu::prelude::label_to_ptr;

//use wgpu_core::gfx_select;
use super::gpu::CanvasWebGPUInstance;

/// An encoder whose `finish` runs when the command buffer is first needed.
pub(crate) struct PendingFinish {
    pub(crate) encoder: Arc<wgpu_core::command::CommandEncoder>,
    pub(crate) label: Option<Cow<'static, str>>,
    pub(crate) scopes: super::queue_thread::ScopeSnapshot,
}

pub struct CanvasGPUCommandBuffer {
    pub(crate) label: Option<Cow<'static, str>>,
    pub(crate) instance: Arc<CanvasWebGPUInstance>,
    pub(crate) pending: parking_lot::Mutex<Option<PendingFinish>>,
    pub(crate) finished: std::sync::OnceLock<Arc<wgpu_core::command::CommandBuffer>>,
    pub(crate) open: std::sync::atomic::AtomicBool,
}

impl CanvasGPUCommandBuffer {
    /// Encoding the commands is the expensive part of `finish`; it runs here, on the
    /// queue thread for a submit, reporting into the scopes open at `finish()`.
    pub(crate) fn command_buffer(&self) -> Arc<wgpu_core::command::CommandBuffer> {
        Arc::clone(self.finished.get_or_init(|| {
            let pending = self
                .pending
                .lock()
                .take()
                .expect("a command buffer is finished once");
            super::queue_thread::with_scopes(&pending.scopes, || {
                pending.encoder.finish(&wgt::CommandBufferDescriptor {
                    label: pending.label.clone(),
                })
            })
        }))
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
