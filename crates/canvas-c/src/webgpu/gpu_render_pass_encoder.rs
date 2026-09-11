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
    pub(crate) pass: parking_lot::Mutex<Option<wgpu_core::command::RenderPass>>,
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
    render_pass: *const CanvasGPURenderPassEncoder,
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
    let label = render_pass.label.clone();
    let error_sink = render_pass.error_sink.as_ref();

    let mut pass = render_pass.pass.lock();

    if let Some(pass) = pass.as_mut() {
        pass.begin_occlusion_query(query_index);}
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
    let label = render_pass.label.clone();
    let error_sink = render_pass.error_sink.as_ref();

    let mut pass = render_pass.pass.lock();

    if let Some(pass) = pass.as_mut() {
        pass.draw(vertex_count,
            instance_count,
            first_vertex,
            first_instance);
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
    let label = render_pass.label.clone();
    let error_sink = render_pass.error_sink.as_ref();

    let mut pass = render_pass.pass.lock();

    if let Some(pass) = pass.as_mut() {
        pass.draw_indexed(index_count,
            instance_count,
            first_index,
            base_vertex,
            first_instance);
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
    let label = render_pass.label.clone();
    let error_sink = render_pass.error_sink.as_ref();

    let indirect_buffer = &*indirect_buffer;

    let buffer_id = Arc::clone(&indirect_buffer.buffer);

    let mut pass = render_pass.pass.lock();

    if let Some(pass) = pass.as_mut() {
        pass.draw_indexed_indirect(buffer_id, indirect_offset);}
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
    let label = render_pass.label.clone();
    let error_sink = render_pass.error_sink.as_ref();

    let indirect_buffer = &*indirect_buffer;

    let buffer_id = Arc::clone(&indirect_buffer.buffer);

    let mut pass = render_pass.pass.lock();

    if let Some(pass) = pass.as_mut() {
        pass.multi_draw_indexed_indirect(buffer_id, indirect_offset, count);}
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
    let label = render_pass.label.clone();
    let error_sink = render_pass.error_sink.as_ref();

    let indirect_buffer = &*indirect_buffer;

    let buffer_id = Arc::clone(&indirect_buffer.buffer);
    let mut pass = render_pass.pass.lock();

    if let Some(pass) = pass.as_mut() {
        pass.draw_indirect(buffer_id, indirect_offset);}
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
    let label = render_pass.label.clone();
    let error_sink = render_pass.error_sink.as_ref();

    let indirect_buffer = &*indirect_buffer;

    let buffer_id = Arc::clone(&indirect_buffer.buffer);
    let mut pass = render_pass.pass.lock();

    if let Some(pass) = pass.as_mut() {
        pass.multi_draw_indirect(buffer_id, indirect_offset, count);}
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_render_pass_encoder_end(
    render_pass: *const CanvasGPURenderPassEncoder,
) {
    if render_pass.is_null() {
        return;
    }

    let render_pass = &*render_pass;
    let label = render_pass.label.clone();
    let error_sink = render_pass.error_sink.as_ref();

    let mut lock = render_pass.pass.lock();

    if let Some(pass) = lock.as_mut() {
        pass.end();

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
    let label = render_pass.label.clone();
    let error_sink = render_pass.error_sink.as_ref();
    let mut pass = render_pass.pass.lock();

    if let Some(pass) = pass.as_mut() {
        pass.end_occlusion_query();
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
    let label = render_pass.label.clone();
    let error_sink = render_pass.error_sink.as_ref();
    let mut pass = render_pass.pass.lock();

    if let Some(pass) = pass.as_mut() {
        let bundles = std::slice::from_raw_parts(bundles, bundles_size)
            .iter()
            .map(|value| Arc::clone(&(&**value).bundle))
            .collect::<Vec<_>>();

        pass.execute_bundles(bundles.as_slice());}
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
    let label = render_pass.label.clone();
    let error_sink = render_pass.error_sink.as_ref();
    let mut pass = render_pass.pass.lock();

    if let Some(pass) = pass.as_mut() {
        let marker_label = CStr::from_ptr(marker_label);
        let marker_label = marker_label.to_str().unwrap();

        pass.insert_debug_marker(marker_label, 0);}
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_render_pass_encoder_pop_debug_group(
    render_pass: *const CanvasGPURenderPassEncoder,
) {
    if render_pass.is_null() {
        return;
    }

    let render_pass = &*render_pass;
    let label = render_pass.label.clone();
    let error_sink = render_pass.error_sink.as_ref();
    let mut pass = render_pass.pass.lock();

    if let Some(pass) = pass.as_mut() {
        pass.pop_debug_group();}
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
    let label = render_pass.label.clone();
    let error_sink = render_pass.error_sink.as_ref();
    let mut pass = render_pass.pass.lock();

    if let Some(pass) = pass.as_mut() {
        let group_label = CStr::from_ptr(group_label);
        let group_label = group_label.to_str().unwrap();

        pass.push_debug_group(group_label, 0);}
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
    let label = render_pass.label.clone();
    let error_sink = render_pass.error_sink.as_ref();
    let mut pass = render_pass.pass.lock();

    if let Some(pass) = pass.as_mut() {
        let bind_group_id = if bind_group.is_null() {
            None
        } else {
            let bind_group = &*bind_group;
            let bind_group_id = Arc::clone(&bind_group.group);
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

            pass.set_bind_group(index, bind_group_id, dynamic_offsets);} else {
            pass.set_bind_group(index, bind_group_id, &[]);}
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
    let label = render_pass.label.clone();
    let error_sink = render_pass.error_sink.as_ref();
    let mut pass = render_pass.pass.lock();

    if let Some(pass) = pass.as_mut() {
        let color: wgt::Color = (*color).into();

        pass.set_blend_constant(color);}
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
    let label = render_pass.label.clone();
    let error_sink = render_pass.error_sink.as_ref();
    let mut pass = render_pass.pass.lock();

    if let Some(pass) = pass.as_mut() {
        let buffer = &*buffer;
        let buffer_id = buffer.buffer;

        let offset: u64 = offset.try_into().unwrap_or_default();

        let size: Option<u64> = size.try_into().ok();

        // wgpu takes Option<BufferAddress> now; zero means "no explicit size",
        // which is what NonZero used to encode.
        let sizeValue = size.filter(|value| *value > 0);

        pass.set_index_buffer(
            Arc::clone(&buffer_id),
            index_format.into(),
            offset,
            sizeValue,
        );
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
    let label = render_pass.label.clone();
    let error_sink = render_pass.error_sink.as_ref();
    let mut pass = render_pass.pass.lock();

    if let Some(pass) = pass.as_mut() {
        let pipeline = &*pipeline;
        let pipeline_id = Arc::clone(&pipeline.pipeline);

        pass.set_pipeline(pipeline_id);}
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
    let label = render_pass.label.clone();
    let error_sink = render_pass.error_sink.as_ref();
    let mut pass = render_pass.pass.lock();

    if let Some(pass) = pass.as_mut() {
        pass.set_scissor_rect(x, y, width, height);}
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
    let label = render_pass.label.clone();
    let error_sink = render_pass.error_sink.as_ref();
    let mut pass = render_pass.pass.lock();

    if let Some(pass) = pass.as_mut() {
        pass.set_stencil_reference(reference);}
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
    let label = render_pass.label.clone();
    let error_sink = render_pass.error_sink.as_ref();
    let mut pass = render_pass.pass.lock();

    if let Some(pass) = pass.as_mut() {
        let buffer_id = {
            if buffer.is_null() {
                None
            } else {
                let buffer = &*buffer;
                Some(Arc::clone(&buffer.buffer))
            }
        };

        let size: Option<u64> = size.try_into().ok();

        let sizeValue = if let Some(value) = size {
            Some(value).filter(|v| *v > 0)
        } else if !buffer.is_null() {
            let buffer = &*buffer;
            Some(buffer.size).filter(|v| *v > 0)
        } else {
            None
        };

        pass.set_vertex_buffer(slot,
            buffer_id,
            offset.try_into().unwrap_or_default(),
            sizeValue);}
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
    let label = render_pass.label.clone();
    let error_sink = render_pass.error_sink.as_ref();
    let mut pass = render_pass.pass.lock();

    if let Some(pass) = pass.as_mut() {
        pass.set_viewport(x, y, width, height, depth_min, depth_max);}
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
