use std::os::raw::c_char;
use std::sync::Arc;

use crate::webgpu::enums::CanvasIndexFormat;
use crate::webgpu::error::handle_error_fatal;
use crate::webgpu::gpu::CanvasWebGPUInstance;
use crate::webgpu::gpu_bind_group::CanvasGPUBindGroup;
use crate::webgpu::gpu_buffer::CanvasGPUBuffer;
use crate::webgpu::gpu_render_bundle::CanvasGPURenderBundle;
use crate::webgpu::gpu_render_pipeline::CanvasGPURenderPipeline;
use crate::webgpu::prelude::ptr_into_label;

pub struct CanvasGPURenderBundleEncoder {
    pub(crate) instance: Arc<CanvasWebGPUInstance>,
    pub(crate) encoder: *mut Option<wgpu_core::command::RenderBundleEncoder>,
}


impl Drop for CanvasGPURenderBundleEncoder {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            if self.encoder.is_null() { return; }
            drop(unsafe { Box::from_raw(self.encoder) });
        }
    }
}
// RenderBundleEncoder is thread-unsafe
unsafe impl Send for CanvasGPURenderBundleEncoder {}
unsafe impl Sync for CanvasGPURenderBundleEncoder {}


#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_render_bundle_encoder_draw(
    render_bundle: *const CanvasGPURenderBundleEncoder,
    vertex_count: u32,
    instance_count: u32,
    first_vertex: u32,
    first_instance: u32,
) {
    if render_bundle.is_null() {
        return;
    }

    let render_bundle = &*render_bundle;

    if render_bundle.encoder.is_null() {
        return;
    }

    if let Some(encoder) = render_bundle.encoder.as_mut() {
        if let Some(encoder) = encoder {
            wgpu_core::command::bundle_ffi::wgpu_render_bundle_draw(encoder, vertex_count,
                                                                    instance_count,
                                                                    first_vertex,
                                                                    first_instance);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_render_bundle_encoder_draw_indexed(
    render_bundle: *const CanvasGPURenderBundleEncoder,
    index_count: u32,
    instance_count: u32,
    first_index: u32,
    base_vertex: i32,
    first_instance: u32,
) {
    if render_bundle.is_null() {
        return;
    }

    let render_bundle = &*render_bundle;

    if render_bundle.encoder.is_null() {
        return;
    }

    if let Some(encoder) = render_bundle.encoder.as_mut() {
        if let Some(encoder) = encoder {
            wgpu_core::command::bundle_ffi::wgpu_render_bundle_draw_indexed(
                encoder,
                index_count,
                instance_count,
                first_index,
                base_vertex,
                first_instance,
            );
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_render_bundle_encoder_draw_indexed_indirect(
    render_bundle: *const CanvasGPURenderBundleEncoder,
    indirect_buffer: *const CanvasGPUBuffer,
    indirect_offset: u64,
) {
    if render_bundle.is_null() || indirect_buffer.is_null() {
        return;
    }

    let render_bundle = &*render_bundle;

    let indirect_buffer = &*indirect_buffer;

    let buffer_id = indirect_buffer.buffer;

    if render_bundle.encoder.is_null() {
        return;
    }

    if let Some(encoder) = render_bundle.encoder.as_mut() {
        if let Some(encoder) = encoder {
            wgpu_core::command::bundle_ffi::wgpu_render_bundle_draw_indexed_indirect(
                encoder,
                buffer_id,
                indirect_offset,
            );
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_render_bundle_encoder_draw_indirect(
    render_bundle: *const CanvasGPURenderBundleEncoder,
    indirect_buffer: *const CanvasGPUBuffer,
    indirect_offset: u64,
) {
    if render_bundle.is_null() || indirect_buffer.is_null() {
        return;
    }

    let render_bundle = &*render_bundle;

    let indirect_buffer = &*indirect_buffer;

    let buffer_id = indirect_buffer.buffer;

    if render_bundle.encoder.is_null() {
        return;
    }

    if let Some(encoder) = render_bundle.encoder.as_mut() {
        if let Some(encoder) = encoder {
            wgpu_core::command::bundle_ffi::wgpu_render_bundle_draw_indirect(
                encoder,
                buffer_id,
                indirect_offset,
            );
        }
    }
}


#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_render_bundle_encoder_insert_debug_marker(
    render_bundle: *const CanvasGPURenderBundleEncoder,
    label: *const c_char,
) {
    if render_bundle.is_null() || label.is_null() {
        return;
    }

    let render_bundle = &*render_bundle;

    if render_bundle.encoder.is_null() {
        return;
    }

    if let Some(encoder) = render_bundle.encoder.as_mut() {
        if let Some(encoder) = encoder {
            wgpu_core::command::bundle_ffi::wgpu_render_bundle_insert_debug_marker(
                encoder,
                label,
            );
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_render_bundle_encoder_pop_debug_group(
    render_bundle: *const CanvasGPURenderBundleEncoder,
) {
    if render_bundle.is_null() {
        return;
    }

    let render_bundle = &*render_bundle;

    if render_bundle.encoder.is_null() {
        return;
    }

    if let Some(encoder) = render_bundle.encoder.as_mut() {
        if let Some(encoder) = encoder {
            wgpu_core::command::bundle_ffi::wgpu_render_bundle_pop_debug_group(encoder);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_render_bundle_encoder_push_debug_group(
    render_bundle: *const CanvasGPURenderBundleEncoder,
    label: *const c_char,
) {
    if render_bundle.is_null() || label.is_null() {
        return;
    }

    let render_bundle = &*render_bundle;

    if render_bundle.encoder.is_null() {
        return;
    }

    if let Some(encoder) = render_bundle.encoder.as_mut() {
        if let Some(encoder) = encoder {
            wgpu_core::command::bundle_ffi::wgpu_render_bundle_push_debug_group(
                encoder, label,
            );
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_render_bundle_encoder_set_bind_group(
    render_bundle: *const CanvasGPURenderBundleEncoder,
    index: u32,
    bind_group: *const CanvasGPUBindGroup,
    dynamic_offsets: *const u32,
    dynamic_offsets_size: usize,
    dynamic_offsets_start: usize,
    dynamic_offsets_length: usize,
) {
    if render_bundle.is_null() || bind_group.is_null() {
        return;
    }

    let render_bundle = &*render_bundle;

    if render_bundle.encoder.is_null() {
        return;
    }


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

        if let Some(encoder) = render_bundle.encoder.as_mut() {
            if let Some(encoder) = encoder {
                wgpu_core::command::bundle_ffi::wgpu_render_bundle_set_bind_group(
                    encoder,
                    index,
                    bind_group_id,
                    dynamic_offsets.as_ptr(),
                    dynamic_offsets.len(),
                );
            }
        }
    } else {
        if let Some(encoder) = render_bundle.encoder.as_mut() {
            if let Some(encoder) = encoder {
                wgpu_core::command::bundle_ffi::wgpu_render_bundle_set_bind_group(
                    encoder,
                    index,
                    bind_group_id,
                    std::ptr::null(),
                    0,
                );
            }
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_render_bundle_encoder_set_index_buffer(
    render_bundle: *const CanvasGPURenderBundleEncoder,
    buffer: *const CanvasGPUBuffer,
    index_format: CanvasIndexFormat,
    offset: i64,
    size: i64,
) {
    if render_bundle.is_null() || buffer.is_null() {
        return;
    }

    let render_bundle = &*render_bundle;

    if render_bundle.encoder.is_null() {
        return;
    }

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
            if let Some(encoder) = render_bundle.encoder.as_mut() {
                if let Some(encoder) = encoder {
                    wgpu_core::command::bundle_ffi::wgpu_render_bundle_set_index_buffer(
                        encoder,
                        buffer_id,
                        index_format.into(),
                        offset,
                        Some(size),
                    );
                }
            }
        } else {
            // todo error ??
        }
    } else {
        if let Some(encoder) = render_bundle.encoder.as_mut() {
            if let Some(encoder) = encoder {
                wgpu_core::command::bundle_ffi::wgpu_render_bundle_set_index_buffer(
                    encoder,
                    buffer_id,
                    index_format.into(),
                    offset,
                    None,
                );
            }
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_render_bundle_encoder_set_pipeline(
    render_bundle: *const CanvasGPURenderBundleEncoder,
    pipeline: *const CanvasGPURenderPipeline,
) {
    if render_bundle.is_null() || pipeline.is_null() {
        return;
    }

    let render_bundle = &*render_bundle;

    let pipeline = &*pipeline;
    let pipeline_id = pipeline.pipeline;

    if render_bundle.encoder.is_null() {
        return;
    }

    if let Some(encoder) = render_bundle.encoder.as_mut() {
        if let Some(encoder) = encoder {
            wgpu_core::command::bundle_ffi::wgpu_render_bundle_set_pipeline(encoder, pipeline_id);
        }
    }
}


#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_render_bundle_encoder_set_vertex_buffer(
    render_bundle: *const CanvasGPURenderBundleEncoder,
    slot: u32,
    buffer: *const CanvasGPUBuffer,
    offset: i64,
    size: i64,
) {
    if render_bundle.is_null() || buffer.is_null() {
        return;
    }

    let render_bundle = &*render_bundle;

    if render_bundle.encoder.is_null() {
        return;
    }


    let buffer = &*buffer;
    let buffer_id = buffer.buffer;

    let size: Option<u64> = size.try_into().ok();

    let mut sizeValue: Option<std::num::NonZero<u64>> = None;

    if let Some(value) = size {
        sizeValue = std::num::NonZero::new(value);
    } else {
        sizeValue = std::num::NonZero::new(buffer.size);
    }

    if size.is_some() {
        if let Some(size) = sizeValue {
            if let Some(encoder) = render_bundle.encoder.as_mut() {
                if let Some(encoder) = encoder {
                    wgpu_core::command::bundle_ffi::wgpu_render_bundle_set_vertex_buffer(
                        encoder,
                        slot,
                        buffer_id,
                        offset.try_into().unwrap_or_default(),
                        Some(size),
                    );
                }
            }
        } else {
            // todo error ??
        }
    } else {
        if let Some(encoder) = render_bundle.encoder.as_mut() {
            if let Some(encoder) = encoder {
                wgpu_core::command::bundle_ffi::wgpu_render_bundle_set_vertex_buffer(
                    encoder,
                    slot,
                    buffer_id,
                    offset.try_into().unwrap_or_default(),
                    None,
                );
            }
        }
    }
}


#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_render_bundle_encoder_finish(
    render_bundle: *const CanvasGPURenderBundleEncoder,
    label: *const c_char,
) -> *const CanvasGPURenderBundle {
    if render_bundle.is_null() {
        return std::ptr::null_mut();
    }

    let render_bundle = &*render_bundle;
    let global = render_bundle.instance.global();

    if render_bundle.encoder.is_null() {
        return std::ptr::null();
    }

    if let Some(encoder) = render_bundle.encoder.as_mut() {
        if let Some(encoder) = encoder.take() {
            let desc = if label.is_null() {
                wgpu_types::RenderBundleDescriptor::default()
            } else {
                wgpu_types::RenderBundleDescriptor { label: ptr_into_label(label) }
            };

            let (render_bundle_id, error) = gfx_select!(bundle =>  global.render_bundle_encoder_finish(encoder, &desc, None));

            if let Some(cause) = error {
                handle_error_fatal(global, cause, "canvas_native_webgpu_render_bundle_encoder_finish");
            }


            return Arc::into_raw(Arc::new(CanvasGPURenderBundle {
                instance: render_bundle.instance.clone(),
                bundle: render_bundle_id,
            }));
        }
    }

    std::ptr::null()
}


#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_render_bundle_encoder_reference(
    render_bundle: *const CanvasGPURenderBundleEncoder,
) {
    if render_bundle.is_null() {
        return;
    }
    Arc::increment_strong_count(render_bundle);
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_render_bundle_encoder_release(
    render_bundle: *const CanvasGPURenderBundleEncoder,
) {
    if render_bundle.is_null() {
        return;
    }
    Arc::decrement_strong_count(render_bundle);
}