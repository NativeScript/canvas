use std::{ffi::CStr, os::raw::c_char};

use super::{
    enums::CanvasTextureAspect,
    gpu::CanvasWebGPUInstance,
    gpu_buffer::CanvasGPUBuffer,
    gpu_command_buffer::CanvasGPUCommandBuffer,
    gpu_compute_pass::CanvasGPUComputePass,
    gpu_query_set::CanvasGPUQuerySet,
    gpu_texture::CanvasGPUTexture,
    structs::{CanvasExtent3d, CanvasOrigin3d},
};

pub struct CanvasGPUCommandEncoder {
    pub(crate) instance: CanvasWebGPUInstance,
    pub(crate) encoder: wgpu_core::id::CommandEncoderId,
}

#[repr(C)]
pub struct CanvasImageCopyBuffer {
    buffer: *const CanvasGPUBuffer,
    offset: u64,
    bytes_per_row: i32,
    rows_per_image: i32,
}

#[repr(C)]
pub struct CanvasImageCopyTexture {
    pub texture: *const CanvasGPUTexture,
    pub mip_level: u32,
    pub origin: CanvasOrigin3d,
    pub aspect: CanvasTextureAspect,
}

#[no_mangle]
pub extern "C" fn canvas_native_webgpu_command_encoder_begin_compute_pass(
    command_encoder: *mut CanvasGPUCommandEncoder,
    query_set: *const CanvasGPUQuerySet,
    label: *const c_char,
    beginning_of_pass_write_index: i32,
    end_of_pass_write_index: i32,
) -> *mut CanvasGPUComputePass {
    if command_encoder.is_null() {
        return std::ptr::null_mut();
    }
    let label = if !label.is_null() {
        Some(unsafe { CStr::from_ptr(label).to_string_lossy() })
    } else {
        None
    };

    let timestamp_writes = if !query_set.is_null() {
        let query_set = unsafe { &*query_set };
        let beginning_of_pass_write_index: Option<u32> =
            beginning_of_pass_write_index.try_into().ok();

        let end_of_pass_write_index: Option<u32> = end_of_pass_write_index.try_into().ok();

        Some(wgpu_core::command::ComputePassTimestampWrites {
            query_set: query_set.query,
            beginning_of_pass_write_index: beginning_of_pass_write_index,
            end_of_pass_write_index: end_of_pass_write_index,
        })
    } else {
        None
    };

    let command_encoder = unsafe { &mut *command_encoder };

    let desc = wgpu_core::command::ComputePassDescriptor {
        label: label,
        timestamp_writes: timestamp_writes.as_ref(),
    };

    let pass = wgpu_core::command::ComputePass::new(command_encoder.encoder, &desc);

    let pass_encoder = CanvasGPUComputePass {
        instance: command_encoder.instance.clone(),
        pass,
    };
    Box::into_raw(Box::new(pass_encoder))
}

/*
#[no_mangle]
pub extern "C" fn canvas_native_webgpu_command_encoder_begin_render_pass(
    command_encoder: *mut CanvasGPUCommandEncoder,
    query_set: *const CanvasGPUQuerySet,
    label: *const c_char,
    beginning_of_pass_write_index: i32,
    end_of_pass_write_index: i32,
) -> *mut CanvasGPURenderPassEncoder {
    if command_encoder.is_null() {
        return std::ptr::null_mut();
    }
    let label = if !label.is_null() {
        Some(unsafe { CStr::from_ptr(label).to_string_lossy() })
    } else {
        None
    };

    let timestamp_writes = if !query_set.is_null() {
        let query_set = unsafe { &*query_set };
        let beginning_of_pass_write_index = if beginning_of_pass_write_index < 0 {
            None
        } else {
            Some(beginning_of_pass_write_index as u32)
        };

        let end_of_pass_write_index = if end_of_pass_write_index < 0 {
            None
        } else {
            Some(end_of_pass_write_index as u32)
        };

        Some(wgpu::ComputePassTimestampWrites {
            query_set: &query_set.0,
            beginning_of_pass_write_index: beginning_of_pass_write_index,
            end_of_pass_write_index: end_of_pass_write_index,
        })
    } else {
        None
    };

    let command_encoder = unsafe { &mut *command_encoder };

    // wgpu::Operations { load: todo!(), store: todo!() }

    wgpu::RenderPassColorAttachment { view: todo!(), resolve_target: todo!(), ops: todo!() }
    let desc = wgpu::RenderPassDescriptor {
        label: label,
        color_attachments: todo!(),
        depth_stencil_attachment: todo!(),
        occlusion_query_set: todo!(),
    };
    let pass = command_encoder.0.begin_render_pass(&desc);
    let pass_encoder = CanvasGPUComputePassEncoder { pass };
    Box::into_raw(Box::new(pass_encoder))
}

*/

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_command_encoder_clear_buffer(
    command_encoder: *mut CanvasGPUCommandEncoder,
    buffer: *const CanvasGPUBuffer,
    offset: i64,
    size: i64,
) {
    if command_encoder.is_null() || buffer.is_null() {
        return;
    }

    let command_encoder = &mut *command_encoder;
    let command_encoder_id = command_encoder.encoder;
    let buffer = &*buffer;
    let buffer_id = buffer.buffer;
    let offset: u64 = offset.try_into().unwrap_or_default();
    let size = size.try_into().ok();

    let global = &command_encoder.instance.0;

    if let Err(_) = gfx_select!(buffer_id => global.command_encoder_clear_buffer(command_encoder_id, buffer_id, offset, size))
    {
        // todo handle error
    }
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_command_encoder_copy_buffer_to_buffer(
    command_encoder: *mut CanvasGPUCommandEncoder,
    src: *const CanvasGPUBuffer,
    src_offset: i64,
    dst: *const CanvasGPUBuffer,
    dst_offset: i64,
    size: u64,
) {
    if command_encoder.is_null() || src.is_null() || dst.is_null() {
        return;
    }

    let command_encoder = &mut *command_encoder;
    let command_encoder_id = command_encoder.encoder;
    let src = &*src;
    let src_id = src.buffer;
    let src_offset: u64 = src_offset.try_into().unwrap_or_default();

    let dst = &*dst;
    let dst_id = dst.buffer;
    let dst_offset: u64 = dst_offset.try_into().unwrap_or_default();

    let global = &command_encoder.instance.0;

    if let Err(_) = gfx_select!(command_encoder_id => global.command_encoder_copy_buffer_to_buffer(command_encoder_id, src_id, src_offset, dst_id, dst_offset, size))
    {
        // todo handle error
    }
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_command_encoder_copy_buffer_to_texture(
    command_encoder: *mut CanvasGPUCommandEncoder,
    src: *const CanvasImageCopyBuffer,
    dst: *const CanvasImageCopyTexture,
    copy_size: *const CanvasExtent3d,
) {
    if command_encoder.is_null() || src.is_null() || dst.is_null() {
        return;
    }

    let command_encoder = &mut *command_encoder;
    let command_encoder_id = command_encoder.encoder;
    let src = &*src;
    let src_buffer = &*src.buffer;

    if (src.buffer.is_null()) {
        return;
    }

    let src_buffer_id = src_buffer.buffer;
    let dst = &*dst;
    let dst_texture = &*dst.texture;
    let dst_texture_id = dst_texture.texture;

    let global = &command_encoder.instance.0;

    let layout = wgpu_types::ImageDataLayout {
        offset: src.offset,
        bytes_per_row: src.bytes_per_row.try_into().ok(),
        rows_per_image: src.rows_per_image.try_into().ok(),
    };
    let image_copy_buffer = wgpu_types::ImageCopyBuffer {
        buffer: src_buffer_id,
        layout: layout,
    };

    let image_copy_texture = wgpu_types::ImageCopyTexture {
        texture: dst_texture_id,
        mip_level: dst.mip_level,
        origin: dst.origin.into(),
        aspect: dst.aspect.into(),
    };

    let copy_size = *copy_size;
    let copy_size: wgpu_types::Extent3d = copy_size.into();

    if let Err(_) = gfx_select!(command_encoder_id => global.command_encoder_copy_buffer_to_texture(command_encoder_id, &image_copy_buffer, &image_copy_texture, &copy_size))
    {
        // todo handle error
    }
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_command_encoder_copy_texture_to_buffer(
    command_encoder: *mut CanvasGPUCommandEncoder,
    src: *const CanvasImageCopyTexture,
    dst: *const CanvasImageCopyBuffer,
    copy_size: *const CanvasExtent3d,
) {
    if command_encoder.is_null() || src.is_null() || dst.is_null() {
        return;
    }

    let command_encoder = &mut *command_encoder;
    let command_encoder_id = command_encoder.encoder;
    let src = &*src;

    if (src.texture.is_null()) {
        return;
    }
    let src_texture = &*src.texture;

    let src_texture_id = src_texture.texture;
    let dst = &*dst;

    if dst.buffer.is_null() {
        return;
    }

    let dst_buffer = &*dst.buffer;

    let dst_buffer_id = dst_buffer.buffer;

    let global = &command_encoder.instance.0;

    let layout = wgpu_types::ImageDataLayout {
        offset: dst.offset,
        bytes_per_row: dst.bytes_per_row.try_into().ok(),
        rows_per_image: dst.rows_per_image.try_into().ok(),
    };
    let image_copy_buffer = wgpu_types::ImageCopyBuffer {
        buffer: dst_buffer_id,
        layout: layout,
    };

    let image_copy_texture = wgpu_types::ImageCopyTexture {
        texture: src_texture_id,
        mip_level: src.mip_level,
        origin: src.origin.into(),
        aspect: src.aspect.into(),
    };

    let copy_size = *copy_size;
    let copy_size: wgpu_types::Extent3d = copy_size.into();

    if let Err(_) = gfx_select!(command_encoder_id => global.command_encoder_copy_texture_to_buffer(command_encoder_id, &image_copy_texture, &image_copy_buffer, &copy_size))
    {
        // todo handle error
    }
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_command_encoder_copy_texture_to_texture(
    command_encoder: *mut CanvasGPUCommandEncoder,
    src: *const CanvasImageCopyTexture,
    dst: *const CanvasImageCopyTexture,
    copy_size: *const CanvasExtent3d,
) {
    if command_encoder.is_null() || src.is_null() || dst.is_null() {
        return;
    }

    let command_encoder = &mut *command_encoder;
    let command_encoder_id = command_encoder.encoder;
    let src = &*src;

    if (src.texture.is_null()) {
        return;
    }

    let src_texture = &*src.texture;

    let src_texture_id = src_texture.texture;
    let dst = &*dst;

    if dst.texture.is_null() {
        return;
    }

    let dst_texture = &*dst.texture;

    let dst_texture_id = dst_texture.texture;

    let global = &command_encoder.instance.0;

    let image_copy_texture_src = wgpu_types::ImageCopyTexture {
        texture: src_texture_id,
        mip_level: src.mip_level,
        origin: src.origin.into(),
        aspect: src.aspect.into(),
    };

    let image_copy_texture_dst = wgpu_types::ImageCopyTexture {
        texture: dst_texture_id,
        mip_level: dst.mip_level,
        origin: dst.origin.into(),
        aspect: dst.aspect.into(),
    };

    let copy_size = *copy_size;
    let copy_size: wgpu_types::Extent3d = copy_size.into();

    if let Err(_) = gfx_select!(command_encoder_id => global.command_encoder_copy_texture_to_texture(command_encoder_id, &image_copy_texture_src, &image_copy_texture_dst, &copy_size))
    {
        // todo handle error
    }
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_command_encoder_finish(
    command_encoder: *mut CanvasGPUCommandEncoder,
    label: *const c_char,
) -> *mut CanvasGPUCommandBuffer {
    if command_encoder.is_null() {
        return std::ptr::null_mut();
    }

    let command_encoder = &mut *command_encoder;
    let command_encoder_id = command_encoder.encoder;
    let global = &command_encoder.instance.0;

    let label = if !label.is_null() {
        Some(unsafe { CStr::from_ptr(label).to_string_lossy() })
    } else {
        None
    };

    let desc = wgpu_types::CommandBufferDescriptor { label: label };

    let (id, err) =
        gfx_select!(command_encoder_id => global.command_encoder_finish(command_encoder_id, &desc));

    if let Some(_) = err {
        // todo handle error
        return std::ptr::null_mut();
    }

    Box::into_raw(Box::new(CanvasGPUCommandBuffer {
        instance: command_encoder.instance.clone(),
        command_buffer: id,
    }))
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_command_encoder_pop_debug_group(
    command_encoder: *mut CanvasGPUCommandEncoder,
) {
    if command_encoder.is_null() {
        return;
    }

    let command_encoder = &mut *command_encoder;
    let command_encoder_id = command_encoder.encoder;
    let global = &command_encoder.instance.0;

    if let Err(_) = gfx_select!(command_encoder_id => global.command_encoder_pop_debug_group(command_encoder_id))
    {
        // todo handle error
    }
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_command_encoder_push_debug_group(
    command_encoder: *mut CanvasGPUCommandEncoder,
    label: *const c_char,
) {
    if command_encoder.is_null() || label.is_null() {
        return;
    }

    let command_encoder = &mut *command_encoder;
    let command_encoder_id = command_encoder.encoder;
    let global = &command_encoder.instance.0;

    let label = CStr::from_ptr(label).to_string_lossy();
    if let Err(_) = gfx_select!(command_encoder_id => global.command_encoder_push_debug_group(command_encoder_id, label.as_ref()))
    {
        // todo handle error
    }
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_command_encoder_resolve_query_set(
    command_encoder: *mut CanvasGPUCommandEncoder,
    query_set: *const CanvasGPUQuerySet,
    first_query: u32,
    query_count: u32,
    dst: *mut CanvasGPUBuffer,
    dst_offset: u64,
) {
    if command_encoder.is_null() || query_set.is_null() || dst.is_null() {
        return;
    }

    let command_encoder = &mut *command_encoder;
    let command_encoder_id = command_encoder.encoder;

    let query_set = &*query_set;
    let query_set_id = query_set.query;

    let dst = &*dst;
    let dst_id = dst.buffer;

    let global = &command_encoder.instance.0;

    if let Err(_) = gfx_select!(command_encoder_id => global.command_encoder_resolve_query_set(command_encoder_id, query_set_id, first_query, query_count, dst_id, dst_offset))
    {
    }
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_command_encoder_write_timestamp(
    command_encoder: *mut CanvasGPUCommandEncoder,
    query_set: *const CanvasGPUQuerySet,
    query_index: u32,
) {
    if command_encoder.is_null() || query_set.is_null() {
        return;
    }

    let command_encoder = &mut *command_encoder;
    let command_encoder_id = command_encoder.encoder;

    let query_set = &*query_set;
    let query_set_id = query_set.query;

    let global = &command_encoder.instance.0;

    if let Err(_) = gfx_select!(command_encoder_id => global.command_encoder_write_timestamp(command_encoder_id, query_set_id, query_index))
    {
    }
}
