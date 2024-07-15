use std::borrow::Cow;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::sync::Arc;

use crate::webgpu::error::handle_error;
use crate::webgpu::gpu_bind_group::CanvasGPUBindGroup;
use crate::webgpu::gpu_buffer::CanvasGPUBuffer;
use crate::webgpu::gpu_compute_pipeline::CanvasGPUComputePipeline;

use super::gpu::CanvasWebGPUInstance;

pub struct CanvasGPUComputePassEncoder {
    pub(crate) label: Option<Cow<'static, str>>,
    pub(crate) instance: Arc<CanvasWebGPUInstance>,
    pub(crate) pass: *mut wgpu_core::command::ComputePass,
    pub(crate) error_sink: super::gpu_device::ErrorSink,
}

impl Drop for CanvasGPUComputePassEncoder {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            drop(unsafe { Box::from_raw(self.pass) });
        }
    }
}
// CanvasGPUComputePassEncoder is thread-unsafe
unsafe impl Send for CanvasGPUComputePassEncoder {}
unsafe impl Sync for CanvasGPUComputePassEncoder {}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_compute_pass_reference(
    compute_pass: *const CanvasGPUComputePassEncoder
) {
    if compute_pass.is_null() {
        return;
    }
    Arc::increment_strong_count(compute_pass);
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_compute_pass_release(
    compute_pass: *const CanvasGPUComputePassEncoder
) {
    if compute_pass.is_null() {
        return;
    }
    Arc::decrement_strong_count(compute_pass);
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_compute_pass_dispatch_workgroups(
    compute_pass: *const CanvasGPUComputePassEncoder,
    workgroup_count_x: u32,
    workgroup_count_y: u32,
    workgroup_count_z: u32,
) {
    if compute_pass.is_null() {
        return;
    }

    let compute_pass = &*compute_pass;
    let compute_pass = &mut *compute_pass.pass;

    wgpu_core::command::compute_commands::wgpu_compute_pass_dispatch_workgroups(compute_pass, workgroup_count_x, workgroup_count_y, workgroup_count_z)
}


#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_compute_pass_dispatch_workgroups_indirect(
    compute_pass: *const CanvasGPUComputePassEncoder,
    indirect_buffer: *const CanvasGPUBuffer,
    indirect_offset: usize,
) {
    if compute_pass.is_null() {
        return;
    }

    let compute_pass = &*compute_pass;
    let compute_pass = &mut *compute_pass.pass;

    let indirect_buffer = &*indirect_buffer;
    let indirect_buffer = indirect_buffer.buffer;

    wgpu_core::command::compute_commands::wgpu_compute_pass_dispatch_workgroups_indirect(compute_pass, indirect_buffer, indirect_offset as u64)
}


#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_compute_pass_end(
    compute_pass: *const CanvasGPUComputePassEncoder,
) {
    if compute_pass.is_null() {
        return;
    }

    let compute_pass = &*compute_pass;
    let label = compute_pass.label.clone();
    let error_sink = compute_pass.error_sink.as_ref();

    let global = compute_pass.instance.global();

    let compute_pass = &mut *compute_pass.pass;

    let command_encoder_id = compute_pass.parent_id();


    if let Err(cause) = gfx_select!(command_encoder_id =>   global.command_encoder_run_compute_pass(command_encoder_id, compute_pass)) {
        handle_error(
            global,
            error_sink,
            cause,
            "encoder",
            label,
            "canvas_native_webgpu_compute_pass_end",
        );
    }
}


#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_compute_pass_insert_debug_marker(
    compute_pass: *const CanvasGPUComputePassEncoder,
    label: *const c_char,
) {
    if compute_pass.is_null() || label.is_null() {
        return;
    }

    let compute_pass = &*compute_pass;
    let compute_pass = &mut *compute_pass.pass;

    let label = CStr::from_ptr(label);
    let label = label.to_string_lossy();

    wgpu_core::command::compute_commands::wgpu_compute_pass_insert_debug_marker(
        compute_pass,
        label.as_ref(),
        0,
    );
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_compute_pass_pop_debug_group(
    compute_pass: *const CanvasGPUComputePassEncoder,
) {
    if compute_pass.is_null() {
        return;
    }

    let compute_pass = &*compute_pass;
    let compute_pass = &mut *compute_pass.pass;

    wgpu_core::command::compute_commands::wgpu_compute_pass_pop_debug_group(compute_pass);
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_compute_pass_push_debug_group(
    compute_pass: *const CanvasGPUComputePassEncoder,
    label: *const c_char,
) {
    if compute_pass.is_null() || label.is_null() {
        return;
    }

    let compute_pass = &*compute_pass;
    let compute_pass = &mut *compute_pass.pass;

    let label = CStr::from_ptr(label);
    let label = label.to_string_lossy();

    wgpu_core::command::compute_commands::wgpu_compute_pass_push_debug_group(
        compute_pass,
        label.as_ref(),
        0,
    );
}


#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_compute_pass_set_bind_group(
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
    let compute_pass = &mut *compute_pass.pass;

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

        wgpu_core::command::compute_commands::wgpu_compute_pass_set_bind_group(
            compute_pass,
            index,
            bind_group_id,
            dynamic_offsets,
        );
    } else {
        wgpu_core::command::compute_commands::wgpu_compute_pass_set_bind_group(
            compute_pass,
            index,
            bind_group_id,
            &[],
        );
    }
}


#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_compute_pass_set_pipeline(
    compute_pass: *mut CanvasGPUComputePassEncoder,
    pipeline: *const CanvasGPUComputePipeline,
) {
    if compute_pass.is_null() || pipeline.is_null() {
        return;
    }

    let compute_pass = &*compute_pass;
    let compute_pass = &mut *compute_pass.pass;

    let pipeline = &*pipeline;
    let pipeline_id = pipeline.pipeline;

    wgpu_core::command::compute_commands::wgpu_compute_pass_set_pipeline(compute_pass, pipeline_id);
}
