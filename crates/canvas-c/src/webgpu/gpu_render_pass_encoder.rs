use std::borrow::Cow;
use std::sync::Arc;
use std::{ffi::CStr, os::raw::c_char};


use crate::webgpu::error::handle_error;
use crate::webgpu::prelude::label_to_ptr;

use super::{
    enums::CanvasIndexFormat, gpu::CanvasWebGPUInstance, gpu_bind_group::CanvasGPUBindGroup,
    gpu_buffer::CanvasGPUBuffer, gpu_render_bundle::CanvasGPURenderBundle,
    gpu_render_pipeline::CanvasGPURenderPipeline, structs::CanvasColor,
};

pub struct CanvasGPURenderPassEncoder {
    pub(crate) label: Option<Cow<'static, str>>,
    pub(crate) instance: Arc<CanvasWebGPUInstance>,
    pub(crate) pass:
        parking_lot::Mutex<Option<wgpu_core::command::RenderPass>>,
    pub(crate) error_sink: super::gpu_device::ErrorSink,
}

// impl Drop for CanvasGPURenderPassEncoder {
//     fn drop(&mut self) {
//         if !std::thread::panicking() {
//             let mut pass = self.pass.lock().take();
//             if let Some(pass) = pass{
//                 drop(pass);
//             }
//            // drop(unsafe { Box::from_raw(self.pass) });
//         }
//     }
// }
// CanvasGPURenderPassEncoder is thread-unsafe
unsafe impl Send for CanvasGPURenderPassEncoder {}
unsafe impl Sync for CanvasGPURenderPassEncoder {}


#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_render_pass_encoder_get_label(
    render_pass: *const CanvasGPURenderPassEncoder
) -> *mut c_char {
    if render_pass.is_null() {
        return std::ptr::null_mut();
    }

    let render_pass = &*render_pass;
    let render_pass = &*render_pass;
    label_to_ptr(render_pass.label.clone())
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_render_pass_encoder_begin_occlusion_query(
    render_pass: *const CanvasGPURenderPassEncoder,
    query_index: u32,
) {
    if render_pass.is_null() {
        return;
    }

    let render_pass = &*render_pass;
    let global = render_pass.instance.global();
    let label = render_pass.label.clone();
    let error_sink = render_pass.error_sink.as_ref();

    let mut pass = render_pass.pass.lock();

    if let Some(pass) = pass.as_mut() {
        if let Err(cause) = global.render_pass_begin_occlusion_query(pass, query_index) {
            handle_error(
                global,
                error_sink,
                cause,
                "encoder",
                label,
                "canvas_native_webgpu_render_pass_encoder_begin_occlusion_query",
            );
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_render_pass_encoder_draw(
    render_pass: *const CanvasGPURenderPassEncoder,
    vertex_count: u32,
    instance_count: u32,
    first_vertex: u32,
    first_instance: u32,
) {
    if render_pass.is_null() {
        return;
    }

    let render_pass = &*render_pass;
    let global = render_pass.instance.global();
    let label = render_pass.label.clone();
    let error_sink = render_pass.error_sink.as_ref();

    let mut pass = render_pass.pass.lock();

    if let Some(pass) = pass.as_mut() {
        if let Err(cause) = global.render_pass_draw(
            pass,
            vertex_count,
            instance_count,
            first_vertex,
            first_instance,
        ) {
            handle_error(
                global,
                error_sink,
                cause,
                "encoder",
                label,
                "canvas_native_webgpu_render_pass_encoder_draw",
            );
        };
    }
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_render_pass_encoder_draw_indexed(
    render_pass: *const CanvasGPURenderPassEncoder,
    index_count: u32,
    instance_count: u32,
    first_index: u32,
    base_vertex: i32,
    first_instance: u32,
) {
    if render_pass.is_null() {
        return;
    }

    let render_pass = &*render_pass;
    let global = render_pass.instance.global();
    let label = render_pass.label.clone();
    let error_sink = render_pass.error_sink.as_ref();

    let mut pass = render_pass.pass.lock();

    if let Some(pass) = pass.as_mut() {
        if let Err(cause) = global.render_pass_draw_indexed(
            pass,
            index_count,
            instance_count,
            first_index,
            base_vertex,
            first_instance,
        ) {
            handle_error(
                global,
                error_sink,
                cause,
                "encoder",
                label,
                "canvas_native_webgpu_render_pass_encoder_draw_indexed",
            );
        };
    }
}


#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_render_pass_encoder_draw_indexed_indirect(
    render_pass: *const CanvasGPURenderPassEncoder,
    indirect_buffer: *const CanvasGPUBuffer,
    indirect_offset: u64,
) {
    if render_pass.is_null() || indirect_buffer.is_null() {
        return;
    }

    let render_pass = &*render_pass;
    let global = render_pass.instance.global();
    let label = render_pass.label.clone();
    let error_sink = render_pass.error_sink.as_ref();

    let indirect_buffer = &*indirect_buffer;

    let buffer_id = indirect_buffer.buffer;

    let mut pass = render_pass.pass.lock();

    if let Some(pass) = pass.as_mut() {
        if let Err(cause) = global.render_pass_draw_indexed_indirect(pass, buffer_id, indirect_offset) {
            handle_error(
                global,
                error_sink,
                cause,
                "encoder",
                label,
                "canvas_native_webgpu_render_pass_encoder_draw_indexed_indirect",
            );
        }
    }
}


#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_render_pass_encoder_multi_draw_indexed_indirect(
    render_pass: *const CanvasGPURenderPassEncoder,
    indirect_buffer: *const CanvasGPUBuffer,
    indirect_offset: u64,
    count: u32,
) {
    if render_pass.is_null() || indirect_buffer.is_null() {
        return;
    }

    let render_pass = &*render_pass;
    let global = render_pass.instance.global();
    let label = render_pass.label.clone();
    let error_sink = render_pass.error_sink.as_ref();

    let indirect_buffer = &*indirect_buffer;

    let buffer_id = indirect_buffer.buffer;

    let mut pass = render_pass.pass.lock();

    if let Some(pass) = pass.as_mut() {
        if let Err(cause) = global.render_pass_multi_draw_indexed_indirect(pass, buffer_id, indirect_offset, count) {
            handle_error(
                global,
                error_sink,
                cause,
                "encoder",
                label,
                "canvas_native_webgpu_render_pass_encoder_draw_indexed_indirect",
            );
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_render_pass_encoder_draw_indirect(
    render_pass: *const CanvasGPURenderPassEncoder,
    indirect_buffer: *const CanvasGPUBuffer,
    indirect_offset: u64,
) {
    if render_pass.is_null() || indirect_buffer.is_null() {
        return;
    }

    let render_pass = &*render_pass;
    let global = render_pass.instance.global();
    let label = render_pass.label.clone();
    let error_sink = render_pass.error_sink.as_ref();

    let indirect_buffer = &*indirect_buffer;

    let buffer_id = indirect_buffer.buffer;
    let mut pass = render_pass.pass.lock();

    if let Some(pass) = pass.as_mut() {
        if let Err(cause) = global.render_pass_draw_indirect(pass, buffer_id, indirect_offset) {
            handle_error(
                global,
                error_sink,
                cause,
                "encoder",
                label,
                "canvas_native_webgpu_render_pass_encoder_draw_indirect",
            );
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_render_pass_encoder_multi_draw_indirect(
    render_pass: *const CanvasGPURenderPassEncoder,
    indirect_buffer: *const CanvasGPUBuffer,
    indirect_offset: u64,
    count: u32,
) {
    if render_pass.is_null() || indirect_buffer.is_null() {
        return;
    }

    let render_pass = &*render_pass;
    let global = render_pass.instance.global();
    let label = render_pass.label.clone();
    let error_sink = render_pass.error_sink.as_ref();

    let indirect_buffer = &*indirect_buffer;

    let buffer_id = indirect_buffer.buffer;
    let mut pass = render_pass.pass.lock();

    if let Some(pass) = pass.as_mut() {
        if let Err(cause) = global.render_pass_multi_draw_indirect(pass, buffer_id, indirect_offset, count) {
            handle_error(
                global,
                error_sink,
                cause,
                "encoder",
                label,
                "canvas_native_webgpu_render_pass_encoder_draw_indirect",
            );
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_render_pass_encoder_end(
    render_pass: *const CanvasGPURenderPassEncoder,
) {
    if render_pass.is_null() {
        return;
    }

    let render_pass = &*render_pass;
    let global = render_pass.instance.global();
    let label = render_pass.label.clone();
    let error_sink = render_pass.error_sink.as_ref();

    let mut lock = render_pass.pass.lock();

    if let Some(pass) = lock.as_mut() {
        if let Err(cause) = global.render_pass_end(pass) {
            println!("canvas_native_webgpu_render_pass_encoder_end: {:?}", cause);
            handle_error(
                global,
                error_sink,
                cause,
                "encoder",
                label,
                "canvas_native_webgpu_render_pass_encoder_end",
            );
        }

        if let Some(pass) = lock.take() {
            drop(pass);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_render_pass_encoder_end_occlusion_query(
    render_pass: *const CanvasGPURenderPassEncoder,
) {
    if render_pass.is_null() {
        return;
    }

    let render_pass = &*render_pass;
    let global = render_pass.instance.global();
    let label = render_pass.label.clone();
    let error_sink = render_pass.error_sink.as_ref();
    let mut pass = render_pass.pass.lock();

    if let Some(pass) = pass.as_mut() {
        if let Err(cause) = global.render_pass_end_occlusion_query(pass) {
            handle_error(
                global,
                error_sink,
                cause,
                "encoder",
                label,
                "canvas_native_webgpu_render_pass_encoder_end_occlusion_query",
            );
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_render_pass_encoder_execute_bundles(
    render_pass: *const CanvasGPURenderPassEncoder,
    bundles: *const *const CanvasGPURenderBundle,
    bundles_size: usize,
) {
    if render_pass.is_null() || bundles.is_null() || bundles_size == 0 {
        return;
    }

    let render_pass = &*render_pass;
    let global = render_pass.instance.global();
    let label = render_pass.label.clone();
    let error_sink = render_pass.error_sink.as_ref();
    let mut pass = render_pass.pass.lock();

    if let Some(pass) = pass.as_mut() {
        let bundles = std::slice::from_raw_parts(bundles, bundles_size)
            .iter()
            .map(|value| (&**value).bundle)
            .collect::<Vec<_>>();

        if let Err(cause) = global.render_pass_execute_bundles(pass, bundles.as_slice()) {
            handle_error(
                global,
                error_sink,
                cause,
                "encoder",
                label,
                "canvas_native_webgpu_render_pass_encoder_execute_bundles",
            );
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_render_pass_encoder_insert_debug_marker(
    render_pass: *const CanvasGPURenderPassEncoder,
    marker_label: *const c_char,
) {
    if render_pass.is_null() || marker_label.is_null() {
        return;
    }

    let render_pass = &*render_pass;
    let global = render_pass.instance.global();
    let label = render_pass.label.clone();
    let error_sink = render_pass.error_sink.as_ref();
    let mut pass = render_pass.pass.lock();

    if let Some(pass) = pass.as_mut() {
        let marker_label = CStr::from_ptr(marker_label);
        let marker_label = marker_label.to_str().unwrap();

        if let Err(cause) = global.render_pass_insert_debug_marker(pass, marker_label, 0) {
            handle_error(
                global,
                error_sink,
                cause,
                "encoder",
                label,
                "canvas_native_webgpu_render_pass_encoder_insert_debug_marker",
            );
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_render_pass_encoder_pop_debug_group(
    render_pass: *const CanvasGPURenderPassEncoder,
) {
    if render_pass.is_null() {
        return;
    }

    let render_pass = &*render_pass;
    let global = render_pass.instance.global();
    let label = render_pass.label.clone();
    let error_sink = render_pass.error_sink.as_ref();
    let mut pass = render_pass.pass.lock();

    if let Some(pass) = pass.as_mut() {
        if let Err(cause) = global.render_pass_pop_debug_group(pass) {
            handle_error(
                global,
                error_sink,
                cause,
                "encoder",
                label,
                "canvas_native_webgpu_render_pass_encoder_pop_debug_group",
            );
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_render_pass_encoder_push_debug_group(
    render_pass: *const CanvasGPURenderPassEncoder,
    group_label: *const c_char,
) {
    if render_pass.is_null() || group_label.is_null() {
        return;
    }

    let render_pass = &*render_pass;
    let global = render_pass.instance.global();
    let label = render_pass.label.clone();
    let error_sink = render_pass.error_sink.as_ref();
    let mut pass = render_pass.pass.lock();

    if let Some(pass) = pass.as_mut() {
        let group_label = CStr::from_ptr(group_label);
        let group_label = group_label.to_str().unwrap();

        if let Err(cause) = global.render_pass_push_debug_group(pass, group_label, 0) {
            handle_error(
                global,
                error_sink,
                cause,
                "encoder",
                label,
                "canvas_native_webgpu_render_pass_encoder_push_debug_group",
            );
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_render_pass_encoder_set_bind_group(
    render_pass: *const CanvasGPURenderPassEncoder,
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

    let render_pass = &*render_pass;
    let global = render_pass.instance.global();
    let label = render_pass.label.clone();
    let error_sink = render_pass.error_sink.as_ref();
    let mut pass = render_pass.pass.lock();

    if let Some(pass) = pass.as_mut() {
        let bind_group_id = if bind_group.is_null() {
            None
        } else {
            let bind_group = &*bind_group;
            let bind_group_id = bind_group.group;
            Some(bind_group_id)
        };


        if !dynamic_offsets.is_null() && dynamic_offsets_size > 0 {
            let dynamic_offsets = std::slice::from_raw_parts(dynamic_offsets, dynamic_offsets_size);

            let start = dynamic_offsets_start;
            let len = dynamic_offsets_length;

            // Assert that length and start are both in bounds
            assert!(start <= dynamic_offsets.len());
            assert!(len <= dynamic_offsets.len() - start);

            let dynamic_offsets: &[u32] = &dynamic_offsets[start..start + len];

            if let Err(cause) = global.render_pass_set_bind_group(pass, index, bind_group_id, dynamic_offsets) {
                handle_error(
                    global,
                    error_sink,
                    cause,
                    "encoder",
                    label,
                    "canvas_native_webgpu_render_pass_encoder_set_bind_group",
                );
            }
        } else {
            if let Err(cause) = global.render_pass_set_bind_group(pass, index, bind_group_id, &[]) {
                handle_error(
                    global,
                    error_sink,
                    cause,
                    "encoder",
                    label,
                    "canvas_native_webgpu_render_pass_encoder_set_bind_group",
                );
            }
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_render_pass_encoder_set_blend_constant(
    render_pass: *const CanvasGPURenderPassEncoder,
    color: *const CanvasColor,
) {
    if render_pass.is_null() || color.is_null() {
        return;
    }

    let render_pass = &*render_pass;
    let global = render_pass.instance.global();
    let label = render_pass.label.clone();
    let error_sink = render_pass.error_sink.as_ref();
    let mut pass = render_pass.pass.lock();

    if let Some(pass) = pass.as_mut() {
        let color: wgt::Color = (*color).into();

        if let Err(cause) = global.render_pass_set_blend_constant(pass, color) {
            handle_error(
                global,
                error_sink,
                cause,
                "encoder",
                label,
                "canvas_native_webgpu_render_pass_encoder_set_blend_constant",
            );
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_render_pass_encoder_set_index_buffer(
    render_pass: *const CanvasGPURenderPassEncoder,
    buffer: *const CanvasGPUBuffer,
    index_format: CanvasIndexFormat,
    offset: i64,
    size: i64,
) {
    if render_pass.is_null() || buffer.is_null() {
        return;
    }

    let render_pass = &*render_pass;
    let global = render_pass.instance.global();
    let label = render_pass.label.clone();
    let error_sink = render_pass.error_sink.as_ref();
    let mut pass = render_pass.pass.lock();

    if let Some(pass) = pass.as_mut() {
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
                if let Err(cause) = global.render_pass_set_index_buffer(
                    pass,
                    buffer_id,
                    index_format.into(),
                    offset,
                    Some(size),
                ) {
                    handle_error(
                        global,
                        error_sink,
                        cause,
                        "encoder",
                        label,
                        "canvas_native_webgpu_render_pass_encoder_set_index_buffer",
                    );
                }
            } else {
                // todo error ??
            }
        } else {
            if let Err(cause) =
                global.render_pass_set_index_buffer(pass, buffer_id, index_format.into(), offset, None)
            {
                handle_error(
                    global,
                    error_sink,
                    cause,
                    "encoder",
                    label,
                    "canvas_native_webgpu_render_pass_encoder_set_index_buffer",
                );
            }
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_render_pass_encoder_set_pipeline(
    render_pass: *const CanvasGPURenderPassEncoder,
    pipeline: *const CanvasGPURenderPipeline,
) {
    if render_pass.is_null() || pipeline.is_null() {
        return;
    }

    let render_pass = &*render_pass;
    let global = render_pass.instance.global();
    let label = render_pass.label.clone();
    let error_sink = render_pass.error_sink.as_ref();
    let mut pass = render_pass.pass.lock();

    if let Some(pass) = pass.as_mut() {
        let pipeline = &*pipeline;
        let pipeline_id = pipeline.pipeline;

        if let Err(cause) = global.render_pass_set_pipeline(pass, pipeline_id) {
            handle_error(
                global,
                error_sink,
                cause,
                "encoder",
                label,
                "canvas_native_webgpu_render_pass_encoder_set_pipeline",
            );
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_render_pass_encoder_set_scissor_rect(
    render_pass: *const CanvasGPURenderPassEncoder,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
) {
    if render_pass.is_null() {
        return;
    }

    let render_pass = &*render_pass;
    let global = render_pass.instance.global();
    let label = render_pass.label.clone();
    let error_sink = render_pass.error_sink.as_ref();
    let mut pass = render_pass.pass.lock();

    if let Some(pass) = pass.as_mut() {
        if let Err(cause) = global.render_pass_set_scissor_rect(pass, x, y, width, height) {
            handle_error(
                global,
                error_sink,
                cause,
                "encoder",
                label,
                "canvas_native_webgpu_render_pass_encoder_set_scissor_rect",
            );
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_render_pass_encoder_set_stencil_reference(
    render_pass: *const CanvasGPURenderPassEncoder,
    reference: u32,
) {
    if render_pass.is_null() {
        return;
    }

    let render_pass = &*render_pass;
    let global = render_pass.instance.global();
    let label = render_pass.label.clone();
    let error_sink = render_pass.error_sink.as_ref();
    let mut pass = render_pass.pass.lock();

    if let Some(pass) = pass.as_mut() {
        if let Err(cause) = global.render_pass_set_stencil_reference(pass, reference) {
            handle_error(
                global,
                error_sink,
                cause,
                "encoder",
                label,
                "canvas_native_webgpu_render_pass_encoder_set_stencil_reference",
            );
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_render_pass_encoder_set_vertex_buffer(
    render_pass: *const CanvasGPURenderPassEncoder,
    slot: u32,
    buffer: *const CanvasGPUBuffer,
    offset: i64,
    size: i64,
) {
    if render_pass.is_null() || buffer.is_null() {
        return;
    }

    let render_pass = &*render_pass;
    let global = render_pass.instance.global();
    let label = render_pass.label.clone();
    let error_sink = render_pass.error_sink.as_ref();
    let mut pass = render_pass.pass.lock();

    if let Some(pass) = pass.as_mut() {
        let buffer = &*buffer;
        let buffer_id = buffer.buffer;

        let size: Option<u64> = size.try_into().ok();

        let sizeValue: Option<std::num::NonZero<u64>>;

        if let Some(value) = size {
            sizeValue = std::num::NonZero::new(value);
        } else {
            sizeValue = std::num::NonZero::new(buffer.size);
        }

        if size.is_some() {
            if let Some(size) = sizeValue {
                if let Err(cause) = global.render_pass_set_vertex_buffer(
                    pass,
                    slot,
                    buffer_id,
                    offset.try_into().unwrap_or_default(),
                    Some(size),
                ) {
                    handle_error(
                        global,
                        error_sink,
                        cause,
                        "encoder",
                        label,
                        "canvas_native_webgpu_render_pass_encoder_set_vertex_buffer",
                    );
                }
            } else {
                // todo error ??
            }
        } else {
            if let Err(cause) = global.render_pass_set_vertex_buffer(
                pass,
                slot,
                buffer_id,
                offset.try_into().unwrap_or_default(),
                None,
            ) {
                handle_error(
                    global,
                    error_sink,
                    cause,
                    "encoder",
                    label,
                    "canvas_native_webgpu_render_pass_encoder_set_vertex_buffer",
                );
            }
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_render_pass_encoder_set_viewport(
    pass: *const CanvasGPURenderPassEncoder,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    depth_min: f32,
    depth_max: f32,
) {
    if pass.is_null() {
        return;
    }

    let render_pass = &*pass;
    let global = render_pass.instance.global();
    let label = render_pass.label.clone();
    let error_sink = render_pass.error_sink.as_ref();
    let mut pass = render_pass.pass.lock();

    if let Some(pass) = pass.as_mut() {
        if let Err(cause) = global.render_pass_set_viewport(pass, x, y, width, height, depth_min, depth_max) {
            handle_error(
                global,
                error_sink,
                cause,
                "encoder",
                label,
                "canvas_native_webgpu_render_pass_encoder_set_viewport",
            );
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_render_pass_encoder_reference(
    render_pass: *const CanvasGPURenderPassEncoder,
) {
    if render_pass.is_null() {
        return;
    }
    Arc::increment_strong_count(render_pass);
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_render_pass_encoder_release(
    render_pass: *const CanvasGPURenderPassEncoder,
) {
    if render_pass.is_null() {
        return;
    }
    Arc::decrement_strong_count(render_pass);
}
