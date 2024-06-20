use std::{
    borrow::Cow,
    ffi::CString,
    os::raw::{c_char, c_void},
};

use crate::buffers::U8Buffer;

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
    pub(crate) instance: CanvasWebGPUInstance,
    pub(crate) label: Cow<'static, str>,
    pub(crate) buffer: wgpu_core::id::BufferId,
    pub(crate) size: u64,
    pub(crate) usage: u32,
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
        let global = &self.instance.0;
        gfx_select!(buffer_id => global.buffer_destroy(buffer_id));
    }

    pub fn unmap(&self) {
        let buffer_id = self.buffer;
        let global = &self.instance.0;
        gfx_select!(buffer_id => global.buffer_unmap(buffer_id));
    }
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
    dst: *mut u8,
    dst_size: usize,
) -> *mut c_char {
    if buffer.is_null() {
        return std::ptr::null_mut();
    }
    let buffer = unsafe { &*buffer };
    let offset: u64 = offset.try_into().ok().unwrap_or_default();
    let size: Option<u64> = size.try_into().ok();

    let buffer_id = buffer.buffer;
    let global = &buffer.instance.0;

    let range = gfx_select!( buffer_id => global.buffer_get_mapped_range(buffer_id, offset, size));

    match range {
        Ok((buf, len)) => {
            let src = std::slice::from_raw_parts(buf, len as usize);
            let dst = std::slice::from_raw_parts_mut(dst, dst_size);
            dst.copy_from_slice(src);
            std::ptr::null_mut()
        }
        Err(err) => {
            let err = err.to_string();
            CString::new(err).unwrap().into_raw()
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
                    std::mem::transmute::<*const i64, fn(*mut c_char, *mut c_void)>(callback as _)
                };
                let callback_data = callback_data as *mut c_void;
                if let Err(error) = result {
                    let error = error.to_string();
                    let error = CString::new(error).unwrap().into_raw();
                    callback(error, callback_data);
                } else {
                    callback(std::ptr::null_mut(), callback_data);
                }
            },
        ))),
    };

    let global = &buffer.instance.0;
    let buffer_id = buffer.buffer;

    gfx_select!(buffer_id => global.buffer_map_async(buffer_id, offset, size, op));
}
