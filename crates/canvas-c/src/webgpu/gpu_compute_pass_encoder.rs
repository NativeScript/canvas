use std::borrow::Cow;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::sync::Arc;

use wgpu_core::command::DynComputePass;

use crate::webgpu::error::handle_error;
use crate::webgpu::gpu_bind_group::CanvasGPUBindGroup;
use crate::webgpu::gpu_buffer::CanvasGPUBuffer;
use crate::webgpu::gpu_compute_pipeline::CanvasGPUComputePipeline;

use super::gpu::CanvasWebGPUInstance;

pub struct CanvasGPUComputePassEncoder {
    pub(crate) label: Option<Cow<'static, str>>,
    pub(crate) instance: Arc<CanvasWebGPUInstance>,
    #[cfg(any(target_os = "android"))]
    pub(crate) pass:
        parking_lot::Mutex<Option<wgpu_core::command::ComputePass<wgpu_core::api::Vulkan>>>,
    #[cfg(any(target_os = "ios", target_os = "macos"))]
    pub(crate) pass:
        parking_lot::Mutex<Option<wgpu_core::command::ComputePass<wgpu_core::api::Metal>>>,
    pub(crate) error_sink: super::gpu_device::ErrorSink,
}

// impl Drop for CanvasGPUComputePassEncoder {
//     fn drop(&mut self) {
//         if !std::thread::panicking() {
//             drop(unsafe { Box::from_raw(self.pass) });
//         }
//     }
// }
// CanvasGPUComputePassEncoder is thread-unsafe
unsafe impl Send for CanvasGPUComputePassEncoder {}
unsafe impl Sync for CanvasGPUComputePassEncoder {}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_compute_pass_encoder_reference(
    compute_pass: *const CanvasGPUComputePassEncoder,
) {
    if compute_pass.is_null() {
        return;
    }
    Arc::increment_strong_count(compute_pass);
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_compute_pass_encoder_release(
    compute_pass: *const CanvasGPUComputePassEncoder,
) {
    if compute_pass.is_null() {
        return;
    }
    Arc::decrement_strong_count(compute_pass);
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_compute_pass_encoder_dispatch_workgroups(
    compute_pass: *const CanvasGPUComputePassEncoder,
    workgroup_count_x: u32,
    workgroup_count_y: u32,
    workgroup_count_z: u32,
) {
    if compute_pass.is_null() {
        return;
    }

    let compute_pass = &*compute_pass;
    let global = compute_pass.instance.global();
    let error_sink = compute_pass.error_sink.as_ref();

    let mut lock = compute_pass.pass.lock();
    if let Some(compute_pass) = lock.as_mut() {
        if let Err(cause) = compute_pass.dispatch_workgroups(
            global,
            workgroup_count_x,
            workgroup_count_y,
            workgroup_count_z,
        ) {
            handle_error(
                global,
                error_sink,
                cause,
                "encoder",
                None,
                "canvas_native_webgpu_compute_pass_encoder_dispatch_workgroups",
            );
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_compute_pass_encoder_dispatch_workgroups_indirect(
    compute_pass: *const CanvasGPUComputePassEncoder,
    indirect_buffer: *const CanvasGPUBuffer,
    indirect_offset: usize,
) {
    if compute_pass.is_null() {
        return;
    }

    let compute_pass = &*compute_pass;
    let global = compute_pass.instance.global();
    let error_sink = compute_pass.error_sink.as_ref();
    let mut lock = compute_pass.pass.lock();
    if let Some(compute_pass) = lock.as_mut() {
        let indirect_buffer = &*indirect_buffer;
        let indirect_buffer = indirect_buffer.buffer;

        if let Err(cause) = compute_pass.dispatch_workgroups_indirect(
            global,
            indirect_buffer,
            indirect_offset as u64,
        ) {
            handle_error(
                global,
                error_sink,
                cause,
                "encoder",
                None,
                "canvas_native_webgpu_compute_pass_encoder_dispatch_workgroups_indirect",
            );
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_compute_pass_encoder_end(
    compute_pass: *const CanvasGPUComputePassEncoder,
) {
    if compute_pass.is_null() {
        return;
    }

    let compute_pass = &*compute_pass;
    let label = compute_pass.label.clone();
    let error_sink = compute_pass.error_sink.as_ref();

    let global = compute_pass.instance.global();

    let mut lock = compute_pass.pass.lock();

    if let Some(compute_pass) = lock.as_mut() {
        if let Err(cause) = compute_pass.end(global) {
            handle_error(
                global,
                error_sink,
                cause,
                "encoder",
                label,
                "canvas_native_webgpu_compute_pass_end",
            );
        }
        if let Some(pass) = lock.take() {
            drop(pass);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_compute_pass_encoder_insert_debug_marker(
    compute_pass: *const CanvasGPUComputePassEncoder,
    label: *const c_char,
) {
    if compute_pass.is_null() || label.is_null() {
        return;
    }

    let compute_pass = &*compute_pass;
    let global = compute_pass.instance.global();
    let error_sink = compute_pass.error_sink.as_ref();

    let label = CStr::from_ptr(label);
    let label = label.to_string_lossy();

    let mut lock = compute_pass.pass.lock();
    if let Some(compute_pass) = lock.as_mut() {
        if let Err(cause) = compute_pass.insert_debug_marker(global, label.as_ref(), 0) {
            handle_error(
                global,
                error_sink,
                cause,
                "encoder",
                None,
                "canvas_native_webgpu_compute_pass_encoder_insert_debug_marker",
            );
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_compute_pass_encoder_pop_debug_group(
    compute_pass: *const CanvasGPUComputePassEncoder,
) {
    if compute_pass.is_null() {
        return;
    }

    let compute_pass = &*compute_pass;
    let global = compute_pass.instance.global();
    let error_sink = compute_pass.error_sink.as_ref();

    let mut lock = compute_pass.pass.lock();
    if let Some(compute_pass) = lock.as_mut() {
        if let Err(cause) = compute_pass.pop_debug_group(global) {
            handle_error(
                global,
                error_sink,
                cause,
                "encoder",
                None,
                "canvas_native_webgpu_compute_pass_encoder_pop_debug_group",
            );
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_compute_pass_encoder_push_debug_group(
    compute_pass: *const CanvasGPUComputePassEncoder,
    label: *const c_char,
) {
    if compute_pass.is_null() || label.is_null() {
        return;
    }

    let compute_pass = &*compute_pass;
    let global = compute_pass.instance.global();
    let error_sink = compute_pass.error_sink.as_ref();

    let mut lock = compute_pass.pass.lock();
    if let Some(compute_pass) = lock.as_mut() {
        let label = CStr::from_ptr(label);
        let label = label.to_string_lossy();

        if let Err(cause) = compute_pass.push_debug_group(global, label.as_ref(), 0) {
            handle_error(
                global,
                error_sink,
                cause,
                "encoder",
                None,
                "canvas_native_webgpu_compute_pass_encoder_push_debug_group",
            );
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_compute_pass_encoder_set_bind_group(
    compute_pass: *const CanvasGPUComputePassEncoder,
    index: u32,
    bind_group: *const CanvasGPUBindGroup,
    dynamic_offsets: *const u32,
    dynamic_offsets_size: usize,
    dynamic_offsets_start: usize,
    dynamic_offsets_length: usize,
) {
    if compute_pass.is_null() || bind_group.is_null() {
        return;
    }

    let compute_pass = &*compute_pass;
    let global = compute_pass.instance.global();
    let error_sink = compute_pass.error_sink.as_ref();

    let mut lock = compute_pass.pass.lock();
    if let Some(compute_pass) = lock.as_mut() {
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

            if let Err(cause) =
                compute_pass.set_bind_group(global, index, bind_group_id, dynamic_offsets)
            {
                handle_error(
                    global,
                    error_sink,
                    cause,
                    "encoder",
                    None,
                    "canvas_native_webgpu_compute_pass_encoder_set_bind_group",
                );
            }
        } else {
            if let Err(cause) = compute_pass.set_bind_group(global, index, bind_group_id, &[]) {
                handle_error(
                    global,
                    error_sink,
                    cause,
                    "encoder",
                    None,
                    "canvas_native_webgpu_compute_pass_encoder_set_bind_group",
                );
            }
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_compute_pass_encoder_set_pipeline(
    compute_pass: *const CanvasGPUComputePassEncoder,
    pipeline: *const CanvasGPUComputePipeline,
) {
    if compute_pass.is_null() || pipeline.is_null() {
        return;
    }

    let compute_pass = &*compute_pass;
    let global = compute_pass.instance.global();
    let error_sink = compute_pass.error_sink.as_ref();

    let mut lock = compute_pass.pass.lock();
    if let Some(compute_pass) = lock.as_mut() {
        let pipeline = &*pipeline;
        let pipeline_id = pipeline.pipeline;

        if let Err(cause) = compute_pass.set_pipeline(global, pipeline_id) {
            handle_error(
                global,
                error_sink,
                cause,
                "encoder",
                None,
                "canvas_native_webgpu_compute_pass_encoder_set_pipeline",
            );
        }
    }
}
