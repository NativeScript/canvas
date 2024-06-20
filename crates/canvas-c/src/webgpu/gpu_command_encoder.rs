use std::{ffi::CStr, os::raw::c_char};

use super::{
    gpu::CanvasWebGPUInstance, gpu_compute_pass::CanvasGPUComputePass,
    gpu_query_set::CanvasGPUQuerySet,
};

pub struct CanvasGPUCommandEncoder {
    pub(crate) instance: CanvasWebGPUInstance,
    pub(crate) encoder: wgpu_core::id::CommandEncoderId,
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
