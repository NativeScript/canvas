use std::{ffi::CStr, num::NonZeroU64, os::raw::c_char};

use super::{
    enums::CanvasIndexFormat, gpu::CanvasWebGPUInstance, gpu_bind_group::CanvasGPUBindGroup,
    gpu_buffer::CanvasGPUBuffer, gpu_command_encoder::CanvasGPUCommandEncoder,
    gpu_query_set::CanvasGPUQuerySet, gpu_render_bundle::CanvasGPURenderBundle,
    gpu_render_pipeline::CanvasGPURenderPipeline, structs::CanvasColor,
};

pub struct CanvasGPURenderPass {
    pub(crate) instance: CanvasWebGPUInstance,
    pub(crate) pass: wgpu_core::command::RenderPass,
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_render_pass_begin_occlusion_query(
    render_pass: *mut CanvasGPURenderPass,
    query_index: u32,
) {
    if render_pass.is_null() {
        return;
    }

    let render_pass = &mut *render_pass;
    let render_pass = &mut render_pass.pass;

    wgpu_core::command::render_commands::wgpu_render_pass_begin_occlusion_query(
        render_pass,
        query_index,
    );
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_render_pass_encoder_draw(
    render_pass: *mut CanvasGPURenderPass,
    vertex_count: u32,
    instance_count: u32,
    first_vertex: u32,
    first_instance: u32,
) {
    if render_pass.is_null() {
        return;
    }

    let render_pass = &mut *render_pass;

    let global = &mut render_pass.instance.0;

    let pass = &mut render_pass.pass;

    wgpu_core::command::render_commands::wgpu_render_pass_draw(
        pass,
        vertex_count,
        instance_count,
        first_vertex,
        first_instance,
    );
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_render_pass_draw_indexed(
    render_pass: *mut CanvasGPURenderPass,
    index_count: u32,
    instance_count: u32,
    first_index: u32,
    base_vertex: i32,
    first_instance: u32,
) {
    if render_pass.is_null() {
        return;
    }

    let render_pass = &mut *render_pass;

    let global = &mut render_pass.instance.0;

    let pass = &mut render_pass.pass;

    wgpu_core::command::render_commands::wgpu_render_pass_draw_indexed(
        pass,
        index_count,
        instance_count,
        first_index,
        base_vertex,
        first_instance,
    );
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_render_pass_draw_indexed_indirect(
    render_pass: *mut CanvasGPURenderPass,
    indirect_buffer: *mut CanvasGPUBuffer,
    indirect_offset: u64,
) {
    if render_pass.is_null() || indirect_buffer.is_null() {
        return;
    }

    let render_pass = &mut *render_pass;

    let indirect_buffer = &mut *indirect_buffer;

    let buffer_id = indirect_buffer.buffer;

    let global = &mut render_pass.instance.0;

    let pass = &mut render_pass.pass;

    wgpu_core::command::render_commands::wgpu_render_pass_draw_indexed_indirect(
        pass,
        buffer_id,
        indirect_offset,
    );
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_render_pass_draw_indirect(
    render_pass: *mut CanvasGPURenderPass,
    indirect_buffer: *mut CanvasGPUBuffer,
    indirect_offset: u64,
) {
    if render_pass.is_null() || indirect_buffer.is_null() {
        return;
    }

    let render_pass = &mut *render_pass;

    let indirect_buffer = &mut *indirect_buffer;

    let buffer_id = indirect_buffer.buffer;

    let global = &mut render_pass.instance.0;

    let pass = &mut render_pass.pass;

    wgpu_core::command::render_commands::wgpu_render_pass_draw_indirect(
        pass,
        buffer_id,
        indirect_offset,
    );
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_render_pass_end(
    render_pass: *mut CanvasGPURenderPass,
) {
    if render_pass.is_null() {
        return;
    }

    let render_pass = &mut *render_pass;

    let global = &mut render_pass.instance.0;

    let pass = &mut render_pass.pass;

    let command_encoder = pass.parent_id();

    if let Err(_) = gfx_select!(command_encoder => global.command_encoder_run_render_pass(command_encoder, pass))
    {
        // todo
    }
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_render_pass_end_occlusion_query(
    render_pass: *mut CanvasGPURenderPass,
) {
    if render_pass.is_null() {
        return;
    }

    let render_pass = &mut *render_pass;
    let render_pass = &mut render_pass.pass;

    wgpu_core::command::render_commands::wgpu_render_pass_end_occlusion_query(render_pass);
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_render_pass_execute_bundles(
    render_pass: *mut CanvasGPURenderPass,
    bundles: *const CanvasGPURenderBundle,
    bundles_size: usize,
) {
    if render_pass.is_null() || bundles.is_null() || bundles_size == 0 {
        return;
    }

    let render_pass = &mut *render_pass;
    let render_pass = &mut render_pass.pass;

    let bundles = std::slice::from_raw_parts(bundles, bundles_size)
        .iter()
        .map(|value| value.bundle)
        .collect::<Vec<wgpu_core::id::RenderBundleId>>();

    wgpu_core::command::render_commands::wgpu_render_pass_execute_bundles(
        render_pass,
        bundles.as_slice(),
    );
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_render_pass_insert_debug_marker(
    render_pass: *mut CanvasGPURenderPass,
    label: *const c_char,
) {
    if render_pass.is_null() || label.is_null() {
        return;
    }

    let render_pass = &mut *render_pass;
    let render_pass = &mut render_pass.pass;

    let label = CStr::from_ptr(label);
    let label = label.to_string_lossy();

    wgpu_core::command::render_commands::wgpu_render_pass_insert_debug_marker(
        render_pass,
        label.as_ref(),
        0,
    );
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_render_pass_pop_debug_group(
    render_pass: *mut CanvasGPURenderPass,
) {
    if render_pass.is_null() {
        return;
    }

    let render_pass = &mut *render_pass;
    let render_pass = &mut render_pass.pass;

    wgpu_core::command::render_commands::wgpu_render_pass_pop_debug_group(render_pass);
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_render_pass_push_debug_group(
    render_pass: *mut CanvasGPURenderPass,
    label: *const c_char,
) {
    if render_pass.is_null() || label.is_null() {
        return;
    }

    let render_pass = &mut *render_pass;
    let render_pass = &mut render_pass.pass;

    let label = CStr::from_ptr(label);
    let label = label.to_string_lossy();

    wgpu_core::command::render_commands::wgpu_render_pass_push_debug_group(
        render_pass,
        label.as_ref(),
        0,
    );
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_render_pass_set_bind_group(
    render_pass: *mut CanvasGPURenderPass,
    index: u32,
    bind_group: *const CanvasGPUBindGroup,
    dynamic_offsets: *const u32,
    dynamic_offsets_size: usize,
    dynamic_offsets_start: usize,
    dynamic_offsets_length: usize,
) {
    if render_pass.is_null() || bind_group.is_null() {
        return;
    }

    let render_pass = &mut *render_pass;
    let render_pass = &mut render_pass.pass;

    let bind_group = &*bind_group;
    let bind_group_id = bind_group.group;

    if !dynamic_offsets.is_null() && dynamic_offsets_size > 0 {
        let dynamic_offsets = std::slice::from_raw_parts(dynamic_offsets, dynamic_offsets_size);

        let start = dynamic_offsets_start;
        let len = dynamic_offsets_length;

        // Assert that length and start are both in bounds
        assert!(start <= dynamic_offsets.len());
        assert!(len <= dynamic_offsets.len() - start);

        let dynamic_offsets: &[u32] = &dynamic_offsets[start..start + len];

        wgpu_core::command::render_commands::wgpu_render_pass_set_bind_group(
            render_pass,
            index,
            bind_group_id,
            dynamic_offsets,
        );
    } else {
        wgpu_core::command::render_commands::wgpu_render_pass_set_bind_group(
            render_pass,
            index,
            bind_group_id,
            &[],
        );
    }
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_render_pass_set_blend_constant(
    render_pass: *mut CanvasGPURenderPass,
    color: *const CanvasColor,
) {
    if render_pass.is_null() || color.is_null() {
        return;
    }

    let render_pass = &mut *render_pass;
    let render_pass = &mut render_pass.pass;

    let color: wgpu_types::Color = (*color).into();

    wgpu_core::command::render_commands::wgpu_render_pass_set_blend_constant(
        render_pass,
        &color,
    );
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_render_pass_set_index_buffer(
    render_pass: *mut CanvasGPURenderPass,
    buffer: *const CanvasGPUBuffer,
    index_format: CanvasIndexFormat,
    offset: i64,
    size: i64,
) {
    if render_pass.is_null() || buffer.is_null() {
        return;
    }

    let render_pass = &mut *render_pass;
    let render_pass = &mut render_pass.pass;

    let buffer = &*buffer;
    let buffer_id = buffer.buffer;

    let offset: u64 = offset.try_into().unwrap_or_default();
  
    let size: Option<u64> = size.try_into().ok();

    let mut sizeValue: Option<std::num::NonZero<u64>> = None;

    if let Some(value) = size {
        sizeValue = std::num::NonZero::new(value);
    }

    if size.is_some() {
        if let Some(size) = sizeValue {
            wgpu_core::command::render_commands::wgpu_render_pass_set_index_buffer(
                render_pass,
                buffer_id,
                index_format.into(),
                offset,
                Some(size),
            );
        } else {
            // todo error ??
        }
    } else {
        wgpu_core::command::render_commands::wgpu_render_pass_set_index_buffer(
            render_pass,
            buffer_id,
            index_format.into(),
            offset,
            None,
        );
    }
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_render_pass_set_pipeline(
    render_pass: *mut CanvasGPURenderPass,
    pipeline: *const CanvasGPURenderPipeline,
) {
    if render_pass.is_null() || pipeline.is_null() {
        return;
    }

    let render_pass = &mut *render_pass;
    let render_pass = &mut render_pass.pass;

    let pipeline = &*pipeline;
    let pipeline_id = pipeline.pipeline;

    wgpu_core::command::render_commands::wgpu_render_pass_set_pipeline(render_pass, pipeline_id);
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_render_pass_set_scissor_rect(
    render_pass: *mut CanvasGPURenderPass,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
) {
    if render_pass.is_null() {
        return;
    }

    let render_pass = &mut *render_pass;
    let render_pass = &mut render_pass.pass;

    wgpu_core::command::render_commands::wgpu_render_pass_set_scissor_rect(
        render_pass,
        x,
        y,
        width,
        height,
    );
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_render_pass_set_stencil_reference(
    render_pass: *mut CanvasGPURenderPass,
    reference: u32,
) {
    if render_pass.is_null() {
        return;
    }

    let render_pass = &mut *render_pass;
    let render_pass = &mut render_pass.pass;

    wgpu_core::command::render_commands::wgpu_render_pass_set_stencil_reference(
        render_pass,
        reference,
    );
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_render_pass_set_vertex_buffer(
    render_pass: *mut CanvasGPURenderPass,
    slot: u32,
    buffer: *mut CanvasGPUBuffer,
    offset: i64,
    size: i64,
) {
    if render_pass.is_null() || buffer.is_null() {
        return;
    }

    let render_pass = &mut *render_pass;
    let render_pass = &mut render_pass.pass;

    let buffer = &*buffer;
    let buffer_id = buffer.buffer;

    let size: Option<u64> = size.try_into().ok();

    let mut sizeValue: Option<std::num::NonZero<u64>> = None;

    if let Some(value) = size {
        sizeValue = std::num::NonZero::new(value);
    }else {
        sizeValue = std::num::NonZero::new(buffer.size);
        println!("SIZE {}", buffer.size);
    }

    if size.is_some() {
        if let Some(size) = sizeValue {
            wgpu_core::command::render_commands::wgpu_render_pass_set_vertex_buffer(
                render_pass,
                slot,
                buffer_id,
                offset.try_into().unwrap_or_default(),
                Some(size),
            );
        } else {
            // todo error ??
        }
    } else {
        wgpu_core::command::render_commands::wgpu_render_pass_set_vertex_buffer(
            render_pass,
            slot,
            buffer_id,
            offset.try_into().unwrap_or_default(),
            None,
        );
    }
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_render_pass_set_viewport(
    render_pass: *mut CanvasGPURenderPass,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    depth_min: f32,
    depth_max: f32,
) {
    if render_pass.is_null() {
        return;
    }

    let render_pass = &mut *render_pass;
    let render_pass = &mut render_pass.pass;

    wgpu_core::command::render_commands::wgpu_render_pass_set_viewport(
        render_pass,
        x,
        y,
        width,
        height,
        depth_min,
        depth_max,
    );
}
