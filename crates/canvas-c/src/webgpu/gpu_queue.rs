use super::{
    gpu::CanvasWebGPUInstance,
    gpu_buffer::CanvasGPUBuffer,
    gpu_command_buffer::CanvasGPUCommandBuffer,
    gpu_command_encoder::CanvasImageCopyTexture,
    structs::{CanvasExtent3d, CanvasImageCopyExternalImage, CanvasImageDataLayout},
};
//use wgpu_core::gfx_select;
use crate::webgpu::error::{handle_error, handle_error_fatal};
use crate::webgpu::prelude::label_to_ptr;
use crate::webgpu::structs::{CanvasImageCopyCanvasRenderingContext2D, CanvasImageCopyImageAsset, CanvasImageCopyWebGL};
use canvas_webgl::utils::gl::bytes_per_pixel;
use std::borrow::Cow;
use std::os::raw::{c_char, c_void};
use std::sync::Arc;

#[derive(Debug)]
pub struct QueueId {
    pub(crate) instance: Arc<CanvasWebGPUInstance>,
    pub(crate) id: wgpu_core::id::QueueId,
}

impl Drop for QueueId {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            let global = self.instance.global();
            global.queue_drop(self.id);
        }
    }
}

#[derive(Clone, Debug)]
pub struct CanvasGPUQueue {
    pub(super) label: Option<Cow<'static, str>>,
    pub(crate) queue: Arc<QueueId>,
    pub(crate) error_sink: super::gpu_device::ErrorSink,
}

unsafe impl Send for CanvasGPUQueue {}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_queue_get_label(queue: *const CanvasGPUQueue) -> *mut c_char {
    if queue.is_null() {
        return std::ptr::null_mut();
    }

    let queue = &*queue;
    label_to_ptr(queue.label.clone())
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_queue_reference(queue: *const CanvasGPUQueue) {
    if queue.is_null() {
        return;
    }

    Arc::increment_strong_count(queue);
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_queue_release(queue: *const CanvasGPUQueue) {
    if queue.is_null() {
        return;
    }

    Arc::decrement_strong_count(queue);
}

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
pub unsafe extern "C" fn canvas_native_webgpu_queue_copy_webgl_to_texture(
    queue: *const CanvasGPUQueue,
    source: *const CanvasImageCopyWebGL,
    destination: *const CanvasImageCopyTexture,
    size: *const CanvasExtent3d,
) {
    if source.is_null() {
        return;
    }

    let source = &*source;
    if source.source.is_null() {
        return;
    }
    let webgl = &*source.source;


    webgl.0.make_current();
    let width = webgl.0.get_drawing_buffer_width();
    let height = webgl.0.get_drawing_buffer_height();


    let row_size = bytes_per_pixel(gl_bindings::RGBA as u32, gl_bindings::RGBA as u32) as i32;

    let mut bytes = vec![0u8; (width * height * row_size) as usize];
    unsafe {
        gl_bindings::Flush();
        gl_bindings::ReadPixels(
            0,
            0,
            width,
            height,
            gl_bindings::RGBA as u32,
            gl_bindings::RGBA as u32,
            bytes.as_mut_ptr() as *mut c_void,
        );
    }


    let ext_source = CanvasImageCopyExternalImage {
        source: bytes.as_ptr(),
        source_size: bytes.len(),
        origin: source.origin,
        flip_y: source.flip_y,
        width: width as u32,
        height: height as u32,
    };

    canvas_native_webgpu_queue_copy_external_image_to_texture(queue, &ext_source, destination, size);
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_queue_copy_context_to_texture(
    queue: *const CanvasGPUQueue,
    source: *const CanvasImageCopyCanvasRenderingContext2D,
    destination: *const CanvasImageCopyTexture,
    size: *const CanvasExtent3d,
) {
    if source.is_null() {
        return;
    }

    let source = &*source;
    if source.source.is_null() {
        return;
    }
    let context = &mut *(source.source as *mut crate::c2d::CanvasRenderingContext2D);

    let (width, height) = context.context.dimensions();

    let mut bytes = vec![0u8; (width * height * 4.) as usize];

    context.context.get_pixels(bytes.as_mut_slice(), (0, 0), (width as i32, height as i32));

    let ext_source = CanvasImageCopyExternalImage {
        source: bytes.as_ptr(),
        source_size: bytes.len(),
        origin: source.origin,
        flip_y: source.flip_y,
        width: width as u32,
        height: height as u32,
    };

    canvas_native_webgpu_queue_copy_external_image_to_texture(queue, &ext_source, destination, size);
}


#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_queue_copy_image_asset_to_texture(
    queue: *const CanvasGPUQueue,
    source: *const CanvasImageCopyImageAsset,
    destination: *const CanvasImageCopyTexture,
    size: *const CanvasExtent3d,
) {
    if source.is_null() {
        return;
    }

    let source = &*source;
    if source.source.is_null() {
        return;
    }
    let image_asset = &*source.source;
    image_asset.with_bytes_dimension(|bytes, dimension| {
        let ext_source = CanvasImageCopyExternalImage {
            source: bytes.as_ptr(),
            source_size: bytes.len(),
            origin: source.origin,
            flip_y: source.flip_y,
            width: dimension.0,
            height: dimension.1,
        };

        canvas_native_webgpu_queue_copy_external_image_to_texture(queue, &ext_source, destination, size);
    });
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_queue_copy_external_image_to_texture(
    queue: *const CanvasGPUQueue,
    source: *const CanvasImageCopyExternalImage,
    destination: *const CanvasImageCopyTexture,
    size: *const CanvasExtent3d,
) {
    if queue.is_null() || source.is_null() || destination.is_null() || size.is_null() {
        return;
    }

    let queue = &*queue;
    let queue_id = queue.queue.id;

    let global = queue.queue.instance.global();

    let source = &*source;

    if source.source.is_null() || source.source_size == 0 || source.width == 0 || source.height == 0
    {
        return;
    }

    let destination = &*destination;

    let destination_texture = &*destination.texture;

    let destination_texture_id = destination_texture.texture;

    let size = *size;

    let size: wgt::Extent3d = size.into();

    let data = std::slice::from_raw_parts(source.source, source.source_size);

    let bytesPerRow = source.source_size / (source.width as usize * source.height as usize);

    let data_layout = wgt::ImageDataLayout {
        offset: 0,
        bytes_per_row: Some(source.width * bytesPerRow as u32),
        rows_per_image: Some(source.height),
    };

    let destination = wgt::ImageCopyTexture {
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
        global.queue_write_texture(queue_id, &destination, data.as_slice(), &data_layout, &size)
    } else {
        global.queue_write_texture(queue_id, &destination, data, &data_layout, &size)
    };

    if let Err(cause) = ret {
        handle_error(
            global,
            queue.error_sink.as_ref(),
            cause,
            "",
            None,
            "canvas_native_webgpu_queue_copy_external_image_to_texture",
        );
    }
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_queue_on_submitted_work_done(
    queue: *const CanvasGPUQueue,
    callback: extern "C" fn(*mut c_char, *mut c_void),
    callback_data: *mut c_void,
) {
    if queue.is_null() {
        return;
    }

    let queue = &*queue;
    let queue_id = queue.queue.id;

    let global = queue.queue.instance.global();

    let func = callback as i64;
    let data = callback_data as i64;
    let done = wgpu_core::device::queue::SubmittedWorkDoneClosure::from_rust(Box::new(move || {
        let callback: fn(*mut c_char, *mut c_void) = std::mem::transmute(func as *mut c_void);
        let callback_data = data as *mut c_void;
        callback(std::ptr::null_mut(), callback_data);
    }));

    global.queue_on_submitted_work_done(queue_id, done);
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_queue_submit(
    queue: *const CanvasGPUQueue,
    command_buffers: *const *const CanvasGPUCommandBuffer,
    command_buffers_size: usize,
) {
    if queue.is_null() || command_buffers.is_null() || command_buffers_size == 0 {
        return;
    }

    let queue = &*queue;
    let queue_id = queue.queue.id;
    let global = queue.queue.instance.global();

    let command_buffer_ids = std::slice::from_raw_parts(command_buffers, command_buffers_size)
        .iter()
        .map(|buffer| {
            let buffer = &**buffer;
            buffer
                .open
                .store(false, std::sync::atomic::Ordering::SeqCst);
            // let mut id = buffer.command_buffer.lock();
            // id.take()
            buffer.command_buffer
        })
        .collect::<Vec<_>>();

    if let Err((_, cause)) = global.queue_submit(queue_id, &command_buffer_ids)
    {
        handle_error_fatal(global, cause, "canvas_native_webgpu_queue_submit");
    }

    for id in command_buffer_ids.into_iter() {
        global.command_buffer_drop(id);
    }
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_queue_write_buffer(
    queue: *const CanvasGPUQueue,
    buffer: *const CanvasGPUBuffer,
    buffer_offset: u64,
    data: *const u8,
    data_size: usize,
    data_offset: usize,
    size: isize,
) {
    if queue.is_null() || buffer.is_null() || data.is_null() || data_size == 0 {
        return;
    }

    let queue = &*queue;
    let queue_id = queue.queue.id;

    let global = queue.queue.instance.global();

    let buffer = &*buffer;
    let buffer_id = buffer.buffer;

    let data = std::slice::from_raw_parts(data, data_size);

    let size: Option<usize> = size.try_into().ok();

    let data = match size {
        Some(size) => &data[data_offset..(data_offset + size)],
        None => &data[data_offset..],
    };

    if let Err(cause) = global.queue_write_buffer(queue_id, buffer_id, buffer_offset, data)
    {
        handle_error(
            global,
            queue.error_sink.as_ref(),
            cause,
            "",
            None,
            "canvas_native_webgpu_queue_write_buffer",
        );
    }
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_queue_write_texture(
    queue: *const CanvasGPUQueue,
    destination: *const CanvasImageCopyTexture,
    data_layout: *const CanvasImageDataLayout,
    size: *const CanvasExtent3d,
    buf: *const u8,
    buf_size: usize,
) {
    if queue.is_null() || destination.is_null() || data_layout.is_null() || buf.is_null() {
        return;
    }

    let queue = &*queue;
    let queue_id = queue.queue.id;

    let global = queue.queue.instance.global();

    let destination = &*destination;

    let destination_texture = &*destination.texture;
    let destination_texture_id = destination_texture.texture;

    let destination = wgt::ImageCopyTexture {
        texture: destination_texture_id,
        mip_level: destination.mip_level,
        origin: destination.origin.into(),
        aspect: destination.aspect.into(),
    };

    let data_layout = *data_layout;

    let data_layout: wgt::ImageDataLayout = data_layout.into();

    let data = std::slice::from_raw_parts(buf, buf_size);

    let size = *size;

    let size: wgt::Extent3d = size.into();

    if let Err(cause) = global.queue_write_texture(queue_id, &destination, data, &data_layout, &size)
    {
        handle_error(
            global,
            queue.error_sink.as_ref(),
            cause,
            "",
            None,
            "canvas_native_webgpu_queue_write_texture",
        );
    }
}
