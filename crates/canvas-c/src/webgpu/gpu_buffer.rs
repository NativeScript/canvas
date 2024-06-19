use std::borrow::Cow;

pub struct CanvasGPUBuffer {
    pub(crate) label: Cow<'static, str>,
    pub(crate) buffer: wgpu::Buffer,
}

impl CanvasGPUBuffer {
    pub fn size(&self) -> u64 {
        self.buffer.size()
    }

    pub fn usage(&self) -> u32 {
        self.buffer.usage().bits()
    }

    // pub fn get_mapped_range(&self){
    //     self.buffer.
    // }
    pub fn destroy(&self) {
        self.buffer.destroy();
    }
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
pub extern "C" fn canvas_native_webgpu_buffer_usage(buffer: *const CanvasGPUBuffer) -> u32 {
    if buffer.is_null() {
        return 0_u32;
    }
    let buffer = unsafe { &*buffer };
    buffer.usage()
}


#[no_mangle]
pub extern "C" fn canvas_native_webgpu_buffer_destroy(buffer: *const CanvasGPUBuffer) {
    if buffer.is_null() {
        return;
    }
    let buffer = unsafe { &*buffer };
    buffer.destroy()
}