use std::{
    borrow::Cow,
    ffi::CString,
    os::raw::{c_char, c_void},
};
use std::sync::Arc;

use crate::webgpu::error::{CanvasGPUError, CanvasGPUErrorType, handle_error_fatal};

use super::gpu::CanvasWebGPUInstance;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GPUMapMode {
    Read,
    Write,
}

impl Into<wgpu_core::device::HostMap> for GPUMapMode {
    fn into(self) -> wgpu_core::device::HostMap {
        match self {
            Self::Read => wgpu_core::device::HostMap::Read,
            Self::Write => wgpu_core::device::HostMap::Write,
        }
    }
}

impl From<wgpu_core::device::HostMap> for GPUMapMode {
    fn from(value: wgpu_core::device::HostMap) -> Self {
        match value {
            wgpu_core::device::HostMap::Read => Self::Read,
            wgpu_core::device::HostMap::Write => Self::Write,
        }
    }
}

pub struct CanvasGPUBuffer {
    pub(crate) instance: Arc<CanvasWebGPUInstance>,
    pub(crate) label: Cow<'static, str>,
    pub(crate) buffer: wgpu_core::id::BufferId,
    pub(crate) size: u64,
    pub(crate) usage: u32,
    pub(crate) error_sink: super::gpu_device::ErrorSink,
}

impl Drop for CanvasGPUBuffer {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            let global = self.instance.global();
            gfx_select!(self.id => global.buffer_drop(self.buffer, false));
        }
    }
}


impl CanvasGPUBuffer {
    pub fn size(&self) -> u64 {
        self.size
    }

    pub fn usage(&self) -> u32 {
        self.usage
    }

    pub fn destroy(&self) {
        let buffer_id = self.buffer;
        let global = self.instance.global();
        let _ = gfx_select!(buffer_id => global.buffer_destroy(buffer_id));
    }

    pub fn unmap(&self) {
        let buffer_id = self.buffer;
        let global = self.instance.global();
        gfx_select!(buffer_id => global.buffer_unmap(buffer_id));
    }
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_buffer_reference(
    buffer: *const CanvasGPUBuffer
) {
    if buffer.is_null() {
        return;
    }

    Arc::increment_strong_count(buffer);
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_buffer_release(
    buffer: *const CanvasGPUBuffer
) {
    if buffer.is_null() {
        return;
    }

    Arc::decrement_strong_count(buffer);
}

#[no_mangle]
pub extern "C" fn canvas_native_webgpu_buffer_usage(buffer: *const CanvasGPUBuffer) -> u32 {
    if buffer.is_null() {
        return 0_u32;
    }
    let buffer = unsafe { &*buffer };
    buffer.usage()
}

#[no_mangle]
pub extern "C" fn canvas_native_webgpu_buffer_size(buffer: *const CanvasGPUBuffer) -> u64 {
    if buffer.is_null() {
        return 0_u64;
    }
    let buffer = unsafe { &*buffer };
    buffer.size()
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_buffer_get_mapped_range(
    buffer: *const CanvasGPUBuffer,
    offset: i64,
    size: i64,
) -> *mut c_void {
    if buffer.is_null() {
        return std::ptr::null_mut();
    }
    let buffer = unsafe { &*buffer };
    let offset: u64 = offset.try_into().ok().unwrap_or_default();
    let size: Option<u64> = size.try_into().ok();

    let buffer_id = buffer.buffer;
    let global = buffer.instance.global();

    let range = gfx_select!( buffer_id => global.buffer_get_mapped_range(buffer_id, offset, size));


    match range {
        Ok((buf, _)) => {
            buf as *mut c_void
        }
        Err(err) => {
            handle_error_fatal(global, err, "canvas_native_webgpu_buffer_get_mapped_range");
            std::ptr::null_mut()
        }
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_webgpu_buffer_destroy(buffer: *const CanvasGPUBuffer) {
    if buffer.is_null() {
        return;
    }
    let buffer = unsafe { &*buffer };
    buffer.destroy()
}

#[no_mangle]
pub extern "C" fn canvas_native_webgpu_buffer_unmap(buffer: *const CanvasGPUBuffer) {
    if buffer.is_null() {
        return;
    }
    let buffer = unsafe { &*buffer };
    buffer.unmap();
}

#[no_mangle]
pub extern "C" fn canvas_native_webgpu_buffer_map_async(
    buffer: *const CanvasGPUBuffer,
    mode: GPUMapMode,
    offset: i64,
    size: i64,
    callback: extern "C" fn(*mut c_char, *mut c_void),
    callback_data: *mut c_void,
) {
    if buffer.is_null() {
        return;
    }
    let buffer = unsafe { &*buffer };
    let offset: u64 = offset.try_into().ok().unwrap_or_default();
    let size: Option<u64> = size.try_into().ok();

    let callback = callback as i64;
    let callback_data = callback_data as i64;

    let op = wgpu_core::resource::BufferMapOperation {
        host: mode.into(),
        callback: Some(wgpu_core::resource::BufferMapCallback::from_rust(Box::new(
            move |result| {
                let callback = unsafe {
                    std::mem::transmute::<*const i64, fn(CanvasGPUErrorType, *mut c_char, *mut c_void)>(callback as _)
                };
                let callback_data = callback_data as *mut c_void;
                if let Err(error) = result {
                    let error: CanvasGPUError = error.into();
                    let (typ, msg) = match error {
                        CanvasGPUError::None => (
                            CanvasGPUErrorType::None,
                            None
                        ),
                        CanvasGPUError::Lost => {
                            (
                                CanvasGPUErrorType::Lost,
                                None
                            )
                        }
                        CanvasGPUError::OutOfMemory => (
                            CanvasGPUErrorType::OutOfMemory,
                            None
                        ),
                        CanvasGPUError::Validation(value) => (
                            CanvasGPUErrorType::Validation,
                            Some(CString::new(value).unwrap())
                        ),
                        CanvasGPUError::Internal => (
                            CanvasGPUErrorType::Internal,
                            None
                        ),
                    };

                    let msg = msg
                        .map(|value| value.into_raw())
                        .unwrap_or(std::ptr::null_mut());

                    callback(typ, msg, callback_data);
                } else {
                    callback(CanvasGPUErrorType::None, std::ptr::null_mut(), callback_data);
                }
            },
        ))),
    };

    let global = buffer.instance.global();
    let buffer_id = buffer.buffer;

    gfx_select!(buffer_id => global.buffer_map_async(buffer_id, offset, size, op));
}
