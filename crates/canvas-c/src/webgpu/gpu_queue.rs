use std::{
    ffi::CString,
    os::raw::{c_char, c_void},
    sync::Arc,
};

use super::{
    gpu::CanvasWebGPUInstance,
    gpu_buffer::CanvasGPUBuffer,
    gpu_command_buffer::CanvasGPUCommandBuffer,
    gpu_command_encoder::CanvasImageCopyTexture,
    structs::{CanvasExtent3d, CanvasImageCopyExternalImage, CanvasImageDataLayout},
};

#[derive(Clone)]
pub struct CanvasGPUQueue {
    pub(crate) instance: CanvasWebGPUInstance,
    pub(crate) queue: wgpu_core::id::QueueId,
}

unsafe impl Send for CanvasGPUQueue {}

fn get_offset_image(
    buffer: &[u8],
    img_width: usize,
    img_height: usize,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
) -> Vec<u8> {
    // todo pass bytes per pixel
    let bytes_per_pixel = buffer.len() / img_width * img_height;
    let mut result = Vec::with_capacity(width * height * bytes_per_pixel);

    for row in 0..height {
        let start = ((y + row) * img_width + x) * bytes_per_pixel;
        let end = start + width * bytes_per_pixel;
        result.extend_from_slice(&buffer[start..end]);
    }

    result
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_queue_copy_external_image_to_texture(
    queue: *mut CanvasGPUQueue,
    source: *const CanvasImageCopyExternalImage,
    destination: *const CanvasImageCopyTexture,
    size: *const CanvasExtent3d,
) {
    if queue.is_null() || source.is_null() || destination.is_null() || size.is_null() {
        return;
    }

    let queue = &mut *queue;
    let queue_id = queue.queue;

    let global = &queue.instance.0;

    let source = &*source;

    if source.source.is_null() || source.source_size == 0 || source.width == 0 || source.height == 0
    {
        return;
    }

    let destination = &*destination;

    if destination.texture.is_null() {
        return;
    }

    let destination_texture = &*destination.texture;

    let destination_texture_id = destination_texture.texture;

    let size = *size;

    let size: wgpu_types::Extent3d = size.into();

    let data = std::slice::from_raw_parts(source.source, source.source_size);

    let data_layout = wgpu_types::ImageDataLayout {
        offset: 0,
        bytes_per_row: None,
        rows_per_image: None,
    };

    let destination = wgpu_types::ImageCopyTexture {
        texture: destination_texture_id,
        mip_level: destination.mip_level,
        origin: destination.origin.into(),
        aspect: destination.aspect.into(),
    };

    let ret = if source.origin.x > 0
        || source.origin.y > 0
        || (size.width > source.width || size.height > source.height)
    {
        let data = get_offset_image(
            data,
            source.width as usize,
            source.height as usize,
            source.origin.x as usize,
            source.origin.y as usize,
            size.width as usize,
            size.height as usize,
        );
        gfx_select!(queue_id =>  global.queue_write_texture(queue_id, &destination, data.as_slice(), &data_layout, &size))
    } else {
        gfx_select!(queue_id =>  global.queue_write_texture(queue_id, &destination, data, &data_layout, &size))
    };

    if let Err(_) = ret {
        // todo handle error
    }
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_queue_on_submitted_work_done(
    queue: *mut CanvasGPUQueue,
    callback: extern "C" fn(*mut c_char, *mut c_void),
    callback_data: *mut c_void,
) {
    if queue.is_null() {
        return;
    }

    let queue = &mut *queue;
    let queue_id = queue.queue;

    let global = &queue.instance.0;

    let func = callback as i64;
    let data = callback_data as i64;
    let done = wgpu_core::device::queue::SubmittedWorkDoneClosure::from_rust(Box::new(move || {
        let callback: fn(*mut c_char, *mut c_void) = std::mem::transmute(func as *mut c_void);
        let callback_data = data as *mut c_void;
        callback(std::ptr::null_mut(), callback_data);
    }));

    if let Err(error) = gfx_select!(queue_id => global.queue_on_submitted_work_done(queue_id, done))
    {
        let error = error.to_string();
        let error = CString::new(error).unwrap().into_raw();
        callback(error, callback_data);
    }
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_queue_submit(
    queue: *mut CanvasGPUQueue,
    command_buffers: *const *const CanvasGPUCommandBuffer,
    command_buffers_size: usize,
) {
    if queue.is_null() || command_buffers.is_null() || command_buffers_size == 0 {
        return;
    }

    let queue = &mut *queue;
    let queue_id = queue.queue;

    let global = &queue.instance.0;

    let command_buffer_ids = std::slice::from_raw_parts(command_buffers, command_buffers_size)
        .iter()
        .map(|buffer| {
            let buffer = &**buffer;
            buffer.command_buffer
        })
        .collect::<Vec<wgpu_core::id::CommandBufferId>>();

    match gfx_select!(queue_id => global.queue_submit(queue_id, command_buffer_ids.as_slice())) {
        Ok(sub) => {
            // noop ??
            println!("canvas_native_webgpu_queue_submit : sub index {:?}", sub);
        }
        Err(err) => {
            println!("canvas_native_webgpu_queue_submit : error {:?}", err.to_string());
            // todo
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_queue_write_buffer(
    queue: *mut CanvasGPUQueue,
    buffer: *mut CanvasGPUBuffer,
    buffer_offset: u64,
    data: *const u8,
    data_size: usize,
    data_offset: usize,
    size: isize,
) {
    if queue.is_null() || buffer.is_null() || data.is_null() || data_size == 0 {
        return;
    }

    let queue = &mut *queue;
    let queue_id = queue.queue;

    let global = &queue.instance.0;

    let buffer = &*buffer;
    let buffer_id = buffer.buffer;

    let data = std::slice::from_raw_parts(data, data_size);

    let size: Option<usize> = size.try_into().ok();

    let data = match size {
        Some(size) => &data[data_offset..(data_offset + size)],
        None => &data[data_offset..],
    };

    if let Err(_) = gfx_select!(queue_id =>  global.queue_write_buffer(queue_id, buffer_id, buffer_offset, data))
    {
        // todo handle error
    }
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_queue_write_texture(
    queue: *mut CanvasGPUQueue,
    destination: *const CanvasImageCopyTexture,
    data_layout: *const CanvasImageDataLayout,
    size: *const CanvasExtent3d,
    buf: *const u8,
    buf_size: usize,
) {
    if queue.is_null() || destination.is_null() || data_layout.is_null() || buf.is_null() {
        return;
    }

    let queue = &mut *queue;
    let queue_id = queue.queue;

    let global = &queue.instance.0;

    let destination = &*destination;
    if destination.texture.is_null() {
        return;
    }
    let destination_texture = &*destination.texture;
    let destination_texture_id = destination_texture.texture;

    let destination = wgpu_types::ImageCopyTexture {
        texture: destination_texture_id,
        mip_level: destination.mip_level,
        origin: destination.origin.into(),
        aspect: destination.aspect.into(),
    };

    let data_layout = *data_layout;

    let data_layout: wgpu_types::ImageDataLayout = data_layout.into();

    let data = std::slice::from_raw_parts(buf, buf_size);

    let size = *size;

    let size: wgpu_types::Extent3d = size.into();

    if let Err(_) = gfx_select!(queue_id =>  global.queue_write_texture(queue_id, &destination, data, &data_layout, &size))
    {
        // todo handle error
    }
}
