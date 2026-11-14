use std::sync::{mpsc, Arc, OnceLock};
use std::{
    borrow::Cow,
    ffi::CString,
    os::raw::{c_char, c_void},
};

use crate::webgpu::error::{handle_error_fatal, CanvasGPUError, CanvasGPUErrorType};
use crate::webgpu::prelude::label_to_ptr;

use super::gpu::CanvasWebGPUInstance;

// wgpu requires polling to dispatch completed mappings (and work-done callbacks),
// including compute-only workloads with no canvas/frame loop. One worker services
// requests off the UI thread and retains each instance until its pending callbacks
// have run.
pub(crate) fn poll_mappings(instance: Arc<CanvasWebGPUInstance>) {
    static POLLER: OnceLock<mpsc::Sender<Arc<CanvasWebGPUInstance>>> = OnceLock::new();
    let sender = POLLER.get_or_init(|| {
        let (sender, receiver) = mpsc::channel::<Arc<CanvasWebGPUInstance>>();
        std::thread::spawn(move || {
            for instance in receiver {
                if let Err(error) = instance.instance().poll_all_devices(true) {
                    log::error!("WebGPU mapping poll failed: {error}");
                }
            }
        });
        sender
    });
    let _ = sender.send(instance);
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GPUMapMode {
    Read,
    Write,
}

impl Into<wgt::MapMode> for GPUMapMode {
    fn into(self) -> wgt::MapMode {
        match self {
            Self::Read => wgt::MapMode::Read,
            Self::Write => wgt::MapMode::Write,
        }
    }
}

impl From<wgt::MapMode> for GPUMapMode {
    fn from(value: wgt::MapMode) -> Self {
        match value {
            wgt::MapMode::Read => Self::Read,
            wgt::MapMode::Write => Self::Write,
        }
    }
}

#[derive(Debug)]
pub struct CanvasGPUBuffer {
    pub(crate) instance: Arc<CanvasWebGPUInstance>,
    pub(crate) label: Option<Cow<'static, str>>,
    pub(crate) buffer: Arc<wgpu_core::resource::Buffer>,
    pub(crate) size: u64,
    pub(crate) usage: u32,
    pub(crate) error_sink: super::gpu_device::ErrorSink,
}

impl CanvasGPUBuffer {
    pub fn label(&self) -> Option<Cow<'static, str>> {
        return self.label.clone();
    }
    pub fn size(&self) -> u64 {
        self.size
    }

    pub fn usage(&self) -> u32 {
        self.usage
    }

    pub fn destroy(&self) {
        // After the queued work that may still use it.
        super::gpu_device::timeline_of(&self.error_sink).drain();
        self.buffer.destroy();
    }

    pub fn unmap(&self) {
        self.buffer.unmap();
    }
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_buffer_get_label(
    buffer: *const CanvasGPUBuffer,
) -> *mut c_char {
    if buffer.is_null() {
        return std::ptr::null_mut();
    }
    let buffer = &*buffer;
    label_to_ptr(buffer.label.clone())
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_buffer_reference(buffer: *const CanvasGPUBuffer) {
    if buffer.is_null() {
        return;
    }

    Arc::increment_strong_count(buffer);
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_buffer_release(buffer: *const CanvasGPUBuffer) {
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

    let range = buffer.buffer.get_mapped_range(offset, size);

    match range {
        Ok(mapping) => mapping.ptr().as_ptr() as *mut c_void,
        Err(err) => {
            handle_error_fatal(err, "canvas_native_webgpu_buffer_get_mapped_range");
            std::ptr::null_mut()
        }
    }
}

pub unsafe fn canvas_native_webgpu_buffer_get_mapped_range_size(
    buffer: *const CanvasGPUBuffer,
    offset: i64,
    size: i64,
) -> (*mut c_void, u64) {
    if buffer.is_null() {
        return (std::ptr::null_mut(), 0);
    }
    let buffer = unsafe { &*buffer };
    let offset: u64 = offset.try_into().ok().unwrap_or_default();
    let size: Option<u64> = size.try_into().ok();

    let range = buffer.buffer.get_mapped_range(offset, size);

    match range {
        Ok(mapping) => (mapping.ptr().as_ptr() as *mut c_void, mapping.len()),
        Err(err) => {
            handle_error_fatal(err, "canvas_native_webgpu_buffer_get_mapped_range");
            (std::ptr::null_mut(), 0)
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
    callback: extern "C" fn(CanvasGPUErrorType, *mut c_char, *mut c_void),
    callback_data: *mut c_void,
) {
    if buffer.is_null() {
        return;
    }
    let buffer = unsafe { &*buffer };
    // Mapping must follow the queued submits that use the buffer.
    super::gpu_device::timeline_of(&buffer.error_sink).drain();
    let offset: u64 = offset.try_into().ok().unwrap_or_default();
    let size: Option<u64> = size.try_into().ok();

    let callback = callback as i64;
    let callback_data = callback_data as i64;

    let mode: wgt::MapMode = mode.into();
    let callback: wgpu_core::resource::BufferMapCallback = Box::new(move |result| {
        let callback = unsafe {
            std::mem::transmute::<*const i64, fn(CanvasGPUErrorType, *mut c_char, *mut c_void)>(
                callback as _,
            )
        };
        let callback_data = callback_data as *mut c_void;
        if let Err(error) = result {
            let error: CanvasGPUError = error.into();
            let (typ, msg) = match error {
                CanvasGPUError::None => (CanvasGPUErrorType::None, None),
                CanvasGPUError::Lost => (CanvasGPUErrorType::Lost, None),
                CanvasGPUError::OutOfMemory => (CanvasGPUErrorType::OutOfMemory, None),
                CanvasGPUError::Validation(value) => (
                    CanvasGPUErrorType::Validation,
                    Some(CString::new(value).unwrap()),
                ),
                CanvasGPUError::Internal => (CanvasGPUErrorType::Internal, None),
            };

            let msg = msg
                .map(|value| value.into_raw())
                .unwrap_or(std::ptr::null_mut());

            callback(typ, msg, callback_data);
        } else {
            callback(
                CanvasGPUErrorType::None,
                std::ptr::null_mut(),
                callback_data,
            );
        }
    });

    if buffer
        .buffer
        .map_async(mode, offset, size, Some(callback))
        .is_some()
    {
        poll_mappings(Arc::clone(&buffer.instance));
    }
}
