#![allow(dead_code)]
#![allow(non_upper_case_globals)]

use std::ffi::{c_ulong, CStr, CString};
use std::os::raw::{c_char, c_longlong, c_void};

use canvas_core::image_asset::ImageAsset;

use crate::prelude::*;
use crate::utils;
use crate::webgl::canvas_native_webgl_get_parameter;

pub struct GLSync(gl_bindings::types::GLsync);

pub fn canvas_native_webgl2_begin_query(target: u32, id: u32, state: &mut WebGLState) {
    state.make_current();
    unsafe {
        gl_bindings::BeginQuery(target, id);
    }
}

pub fn canvas_native_webgl2_begin_transform_feedback(primitive_mode: u32, state: &mut WebGLState) {
    state.make_current();
    unsafe {
        gl_bindings::BeginTransformFeedback(primitive_mode);
    }
}

pub fn canvas_native_webgl2_bind_buffer_base(
    target: u32,
    index: u32,
    buffer: u32,
    state: &mut WebGLState,
) {
    state.make_current();
    unsafe {
        gl_bindings::BindBufferBase(target, index, buffer);
    }
}

pub fn canvas_native_webgl2_bind_buffer_range(
    target: u32,
    index: u32,
    buffer: u32,
    offset: isize,
    size: isize,
    state: &mut WebGLState,
) {
    state.make_current();
    unsafe {
        gl_bindings::BindBufferRange(
            target,
            index,
            buffer,
            offset.try_into().unwrap(),
            size.try_into().unwrap(),
        )
    }
}

pub fn canvas_native_webgl2_bind_sampler(unit: u32, sampler: u32, state: &mut WebGLState) {
    state.make_current();
    unsafe {
        gl_bindings::BindSampler(unit, sampler);
    }
}

pub fn canvas_native_webgl2_bind_transform_feedback(
    target: u32,
    transform_feedback: u32,
    state: &mut WebGLState,
) {
    state.make_current();
    unsafe {
        gl_bindings::BindTransformFeedback(target, transform_feedback);
    }
}

pub fn canvas_native_webgl2_bind_vertex_array(vertex_array: u32, state: &mut WebGLState) {
    state.make_current();
    unsafe {
        gl_bindings::BindVertexArray(vertex_array);
    }
}

pub fn canvas_native_webgl2_blit_framebuffer(
    src_x0: i32,
    src_y0: i32,
    src_x1: i32,
    src_y1: i32,
    dst_x0: i32,
    dst_y0: i32,
    dst_x1: i32,
    dst_y1: i32,
    mask: u32,
    filter: u32,
    state: &mut WebGLState,
) {
    state.make_current();
    unsafe {
        gl_bindings::BlitFramebuffer(
            src_x0, src_y0, src_x1, src_y1, dst_x0, dst_y0, dst_x1, dst_y1, mask, filter,
        );
    }
}

pub fn canvas_native_webgl2_clear_bufferfi(
    buffer: u32,
    drawbuffer: i32,
    depth: f32,
    stencil: i32,
    state: &mut WebGLState,
) {
    state.make_current();
    unsafe {
        gl_bindings::ClearBufferfi(buffer, drawbuffer, depth, stencil);
    }
}

pub fn canvas_native_webgl2_clear_bufferfv(
    buffer: u32,
    drawbuffer: i32,
    values: &[f32],
    state: &mut WebGLState,
) {
    state.make_current();
    unsafe {
        gl_bindings::ClearBufferfv(buffer, drawbuffer, values.as_ptr());
    }
}

pub fn canvas_native_webgl2_clear_bufferiv(
    buffer: u32,
    drawbuffer: i32,
    values: &[i32],
    state: &mut WebGLState,
) {
    state.make_current();
    unsafe {
        gl_bindings::ClearBufferiv(buffer, drawbuffer, values.as_ptr());
    }
}

pub fn canvas_native_webgl2_clear_bufferuiv(
    buffer: u32,
    drawbuffer: i32,
    values: &[u32],
    state: &mut WebGLState,
) {
    state.make_current();
    unsafe {
        gl_bindings::ClearBufferuiv(buffer, drawbuffer, values.as_ptr());
    }
}

pub fn canvas_native_webgl2_client_wait_sync(
    sync: &GLSync,
    flags: u32,
    timeout: c_ulong,
    state: &mut WebGLState,
) -> u32 {
    state.make_current();
    let ret = unsafe { gl_bindings::ClientWaitSync(sync.0, flags, timeout.into()) };

    ret
}

pub fn canvas_native_webgl2_compressed_tex_sub_image3d_none(
    target: u32,
    level: i32,
    xoffset: i32,
    yoffset: i32,
    zoffset: i32,
    width: i32,
    height: i32,
    depth: i32,
    format: u32,
    image_size: i32,
    offset: i32,
    state: &mut WebGLState,
) {
    state.make_current();
    unsafe {
        gl_bindings::CompressedTexSubImage3D(
            target,
            level,
            xoffset,
            yoffset,
            zoffset,
            width,
            height,
            depth,
            format,
            image_size,
            offset as *const c_void,
        )
    }
}

pub fn canvas_native_webgl2_compressed_tex_sub_image3d(
    target: u32,
    level: i32,
    xoffset: i32,
    yoffset: i32,
    zoffset: i32,
    width: i32,
    height: i32,
    depth: i32,
    format: u32,
    src: &[u8],
    src_offset: usize,
    src_length_override: usize,
    state: &mut WebGLState,
) {
    state.make_current();

    let mut len = src.len();
    if src_offset > len {
        return;
    }
    if src_length_override == 0 {
        len = len - src_offset;
    } else {
        len = src_length_override;
    }

    let mut buf = src;
    if src_offset > 0 {
        let ptr = unsafe { buf.as_ptr().offset(src_offset as isize) };
        buf = unsafe { std::slice::from_raw_parts(ptr, len) }
    }

    unsafe {
        gl_bindings::CompressedTexSubImage3D(
            target,
            level,
            xoffset,
            yoffset,
            zoffset,
            width,
            height,
            depth,
            format,
            len.try_into().unwrap(),
            buf.as_ptr() as *const c_void,
        )
    }
}

pub fn canvas_native_webgl2_copy_buffer_sub_data(
    read_target: u32,
    write_target: u32,
    read_offset: isize,
    write_offset: isize,
    size: isize,
    state: &mut WebGLState,
) {
    state.make_current();
    unsafe {
        gl_bindings::CopyBufferSubData(
            read_target,
            write_target,
            read_offset.try_into().unwrap(),
            write_offset.try_into().unwrap(),
            size.try_into().unwrap(),
        );
    }
}

pub fn canvas_native_webgl2_copy_tex_sub_image3d(
    target: u32,
    level: i32,
    xoffset: i32,
    yoffset: i32,
    zoffset: i32,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    state: &mut WebGLState,
) {
    state.make_current();
    unsafe {
        gl_bindings::CopyTexSubImage3D(
            target, level, xoffset, yoffset, zoffset, x, y, width, height,
        );
    }
}

pub fn canvas_native_webgl2_create_query(state: &mut WebGLState) -> u32 {
    state.make_current();
    let mut queries = [0u32; 1];
    unsafe { gl_bindings::GenQueries(1, queries.as_mut_ptr()) }

    queries[0]
}

pub fn canvas_native_webgl2_create_sampler(state: &mut WebGLState) -> u32 {
    state.make_current();
    let mut samplers = [0u32; 1];
    unsafe { gl_bindings::GenSamplers(1, samplers.as_mut_ptr()) }

    samplers[0]
}

pub fn canvas_native_webgl2_create_transform_feedback(state: &mut WebGLState) -> u32 {
    state.make_current();
    let mut feedbacks = [0u32; 1];
    unsafe { gl_bindings::GenTransformFeedbacks(1, feedbacks.as_mut_ptr()) }

    feedbacks[0]
}

pub fn canvas_native_webgl2_create_vertex_array(state: &mut WebGLState) -> u32 {
    state.make_current();
    let mut arrays = [0u32; 1];
    unsafe { gl_bindings::GenVertexArrays(1, arrays.as_mut_ptr()) }

    arrays[0]
}

pub fn canvas_native_webgl2_delete_query_with_query(id: u32, state: &mut WebGLState) {
    state.make_current();
    let queries = [id];
    unsafe {
        gl_bindings::DeleteQueries(1, queries.as_ptr());
    }
}

pub fn canvas_native_webgl2_delete_sampler_with_sampler(sampler: u32, state: &mut WebGLState) {
    state.make_current();
    let samplers = [sampler];
    unsafe {
        gl_bindings::DeleteSamplers(1, samplers.as_ptr());
    }
}

pub fn canvas_native_webgl2_delete_sync_with_sync(sync: &GLSync, state: &mut WebGLState) {
    state.make_current();
    unsafe {
        gl_bindings::DeleteSync(sync.0);
    }
}

pub fn canvas_native_webgl2_delete_transform_feedback(
    transform_feedback: u32,
    state: &mut WebGLState,
) {
    state.make_current();
    let feebacks = [transform_feedback];
    unsafe {
        gl_bindings::DeleteTransformFeedbacks(1, feebacks.as_ptr());
    }
}

pub fn canvas_native_webgl2_delete_vertex_array_with_vertex_array(
    vertex_array: u32,
    state: &mut WebGLState,
) {
    state.make_current();
    let arrays = [vertex_array];
    unsafe {
        gl_bindings::DeleteVertexArrays(1, arrays.as_ptr());
    }
}

pub fn canvas_native_webgl2_draw_arrays_instanced(
    mode: u32,
    first: i32,
    count: i32,
    instance_count: i32,
    state: &mut WebGLState,
) {
    state.make_current();
    unsafe {
        gl_bindings::DrawArraysInstanced(mode, first, count, instance_count);
    }
}

pub fn canvas_native_webgl2_draw_buffers(buffers: &[u32], state: &mut WebGLState) {
    state.make_current();
    unsafe {
        gl_bindings::DrawBuffers(buffers.len().try_into().unwrap(), buffers.as_ptr());
    }
}

pub fn canvas_native_webgl2_draw_elements_instanced(
    mode: u32,
    count: i32,
    type_: u32,
    offset: isize,
    instance_count: i32,
    state: &mut WebGLState,
) {
    state.make_current();
    unsafe {
        gl_bindings::DrawElementsInstanced(
            mode,
            count,
            type_,
            offset as *const c_void,
            instance_count,
        );
    }
}

pub fn canvas_native_webgl2_draw_range_elements(
    mode: u32,
    start: u32,
    end: u32,
    count: i32,
    type_: u32,
    offset: isize,
    state: &mut WebGLState,
) {
    state.make_current();
    unsafe {
        gl_bindings::DrawRangeElements(mode, start, end, count, type_, offset as *const c_void);
    }
}

pub fn canvas_native_webgl2_end_query(target: u32, state: &mut WebGLState) {
    state.make_current();
    unsafe {
        gl_bindings::EndQuery(target);
    }
}

pub fn canvas_native_webgl2_end_transform_feedback(state: &mut WebGLState) {
    state.make_current();
    unsafe {
        gl_bindings::EndTransformFeedback();
    }
}

pub fn canvas_native_webgl2_fence_sync(
    condition: u32,
    flags: u32,
    state: &mut WebGLState,
) -> GLSync {
    state.make_current();
    let ret = unsafe { GLSync(gl_bindings::FenceSync(condition, flags)) };

    ret
}

pub fn canvas_native_webgl2_framebuffer_texture_layer(
    target: u32,
    attachment: u32,
    texture: u32,
    level: i32,
    layer: i32,
    state: &mut WebGLState,
) {
    state.make_current();
    unsafe {
        gl_bindings::FramebufferTextureLayer(target, attachment, texture, level, layer);
    }
}

pub fn canvas_native_webgl2_get_active_uniform_block_name(
    program: u32,
    uniform_block_index: u32,
    state: &mut WebGLState,
) -> String {
    state.make_current();
    let mut max_name_length = 0i32;
    unsafe {
        gl_bindings::GetProgramiv(
            program,
            gl_bindings::ACTIVE_UNIFORM_BLOCK_MAX_NAME_LENGTH,
            &mut max_name_length,
        )
    }
    let mut name = vec![0; max_name_length as usize];
    let mut length = 0i32;

    unsafe {
        gl_bindings::GetActiveUniformBlockName(
            program,
            uniform_block_index,
            max_name_length,
            &mut length,
            name.as_mut_ptr() as *mut c_char,
        )
    }

    name.shrink_to(length as usize);
    let c_str = unsafe { CStr::from_ptr(name.as_ptr()) };
    c_str.to_string_lossy().to_string()
}

pub fn canvas_native_webgl2_get_active_uniform_block_parameter(
    program: u32,
    uniform_block_index: u32,
    pname: u32,
    state: &mut WebGLState,
) -> WebGLResult {
    state.make_current();
    let ret = match pname {
        gl_bindings::UNIFORM_BLOCK_BINDING
        | gl_bindings::UNIFORM_BLOCK_DATA_SIZE
        | gl_bindings::UNIFORM_BLOCK_ACTIVE_UNIFORMS => {
            let mut int_value = [0i32];
            unsafe {
                gl_bindings::GetActiveUniformBlockiv(
                    program,
                    uniform_block_index,
                    pname,
                    int_value.as_mut_ptr(),
                )
            }
            return WebGLResult::I32(int_value[0]);
        }
        gl_bindings::UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES => {
            let mut uniform_count = [0i32];
            unsafe {
                gl_bindings::GetActiveUniformBlockiv(
                    program,
                    uniform_block_index,
                    gl_bindings::UNIFORM_BLOCK_ACTIVE_UNIFORMS,
                    uniform_count.as_mut_ptr(),
                )
            }

            let mut indices: Vec<i32> = vec![0i32;uniform_count[0] as usize];
            unsafe {
                gl_bindings::GetActiveUniformBlockiv(
                    program,
                    uniform_block_index,
                    pname,
                    indices.as_mut_ptr(),
                )
            }
            return WebGLResult::I32Array(indices);
        }
        gl_bindings::UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER
        | gl_bindings::UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER => {
            let mut bool_value = [0i32];
            unsafe {
                gl_bindings::GetActiveUniformBlockiv(
                    program,
                    uniform_block_index,
                    pname,
                    bool_value.as_mut_ptr(),
                )
            }
            return WebGLResult::Boolean(bool_value[0] == 1);
        }
        _ => WebGLResult::None,
    };

    ret
}

pub fn canvas_native_webgl2_get_active_uniforms(
    program: u32,
    uniform_indices: &[u32],
    pname: u32,
    state: &mut WebGLState,
) -> WebGLResult {
    state.make_current();
    let mut params: Vec<i32> = vec![0i32;uniform_indices.len()];

    unsafe {
        gl_bindings::GetActiveUniformsiv(
            program,
            uniform_indices.len().try_into().unwrap(),
            uniform_indices.as_ptr(),
            pname,
            params.as_mut_ptr(),
        )
    }

    let ret = match pname {
        gl_bindings::UNIFORM_TYPE | gl_bindings::UNIFORM_SIZE => WebGLResult::U32Array(unsafe {
            std::slice::from_raw_parts(params.as_ptr() as *const u32, params.len()).to_vec()
        }),
        gl_bindings::UNIFORM_BLOCK_INDEX
        | gl_bindings::UNIFORM_OFFSET
        | gl_bindings::UNIFORM_ARRAY_STRIDE
        | gl_bindings::UNIFORM_MATRIX_STRIDE => WebGLResult::I32Array(params),
        gl_bindings::UNIFORM_IS_ROW_MAJOR => WebGLResult::BooleanArray(unsafe {
            std::slice::from_raw_parts(params.as_ptr() as *const bool, params.len()).to_vec()
        }),
        _ => WebGLResult::None,
    };

    ret
}

pub fn canvas_native_webgl2_get_buffer_sub_data(
    target: u32,
    src_byte_offset: isize,
    dst_data: &mut [u8],
    dst_offset: usize,
    length: usize,
    state: &mut WebGLState,
) {
    state.make_current();

    let len = dst_data.len();

    if dst_offset > len {
        // todo log error
        return;
    }

    if length > length {
        // todo log error
        return;
    }

    let mut len = len - dst_offset;

    if length != 0 {
        len = length;
    }

    let mut ptr = dst_data.as_mut_ptr();

    if dst_offset != 0 {
        unsafe { ptr = ptr.offset(dst_offset as isize) }
    }

    let new_buf = unsafe { std::slice::from_raw_parts_mut(ptr, len) };

    unsafe {
        gl_bindings::BufferSubData(
            target,
            src_byte_offset.try_into().unwrap(),
            new_buf.len().try_into().unwrap(),
            new_buf.as_mut_ptr() as *const c_void,
        )
    }
}

pub fn canvas_native_webgl2_get_frag_data_location(
    program: u32,
    name: &str,
    state: &mut WebGLState,
) -> i32 {
    state.make_current();
    let name = CString::new(name).unwrap();
    let ret = unsafe { gl_bindings::GetFragDataLocation(program, name.as_ptr()) };

    ret
}

pub fn canvas_native_webgl2_get_indexed_parameter(
    target: u32,
    index: u32,
    state: &mut WebGLState,
) -> WebGLIndexedParameter {
    state.make_current();
    let mut binding = WebGLIndexedParameter::default();

    let ret = match target {
        gl_bindings::UNIFORM_BUFFER_BINDING | gl_bindings::TRANSFORM_FEEDBACK_BUFFER_BINDING => {
            let mut new_target = [0i32];
            unsafe { gl_bindings::GetIntegerv(target, new_target.as_mut_ptr()) }
            if new_target[0] == 0 {
                return binding;
            }

            let mut buffer = [0i32];
            unsafe {
                gl_bindings::GetIntegeri_v(
                    new_target[0].try_into().unwrap(),
                    index,
                    buffer.as_mut_ptr(),
                )
            }
            binding.buffer_value = buffer[0] as isize;
            binding.is_buffer = true;
            return binding;
        }
        gl_bindings::TRANSFORM_FEEDBACK_BUFFER_SIZE
        | gl_bindings::TRANSFORM_FEEDBACK_BUFFER_START
        | gl_bindings::UNIFORM_BUFFER_SIZE
        | gl_bindings::UNIFORM_BUFFER_START => {
            let mut ptr = [0 as c_longlong];
            unsafe { gl_bindings::GetInteger64i_v(target, index, ptr.as_mut_ptr()) }
            binding.is_buffer = false;
            binding.value = ptr[0] as isize;
            return binding;
        }
        _ => binding, // return null
    };

    ret
}

pub fn canvas_native_webgl2_get_internalformat_parameter(
    target: u32,
    internalformat: u32,
    pname: u32,
    state: &mut WebGLState,
) -> WebGLResult {
    let mut result: Option<WebGLResult> = None;
    match internalformat {
        gl_bindings::RGB
        | gl_bindings::RGBA
        | gl_bindings::R8UI
        | gl_bindings::R8I
        | gl_bindings::R16UI
        | gl_bindings::R16I
        | gl_bindings::R32UI
        | gl_bindings::R32I
        | gl_bindings::RG8UI
        | gl_bindings::RG8I
        | gl_bindings::RG16UI
        | gl_bindings::RG16I
        | gl_bindings::RG32UI
        | gl_bindings::RG32I
        | gl_bindings::RGBA8UI
        | gl_bindings::RGBA8I
        | gl_bindings::RGB10_A2UI
        | gl_bindings::RGBA16UI
        | gl_bindings::RGBA16I
        | gl_bindings::RGBA32UI
        | gl_bindings::RGBA32I => {
            result = Some(WebGLResult::I32Array(Vec::with_capacity(0)));
        }
        gl_bindings::R8
        | gl_bindings::RG8
        | gl_bindings::RGB565
        | gl_bindings::RGBA8
        | gl_bindings::SRGB8_ALPHA8
        | gl_bindings::RGB5_A1
        | gl_bindings::RGBA4
        | gl_bindings::RGB10_A2
        | gl_bindings::DEPTH_COMPONENT16
        | gl_bindings::DEPTH_COMPONENT24
        | gl_bindings::DEPTH_COMPONENT32F
        | gl_bindings::DEPTH24_STENCIL8
        | gl_bindings::DEPTH32F_STENCIL8
        | gl_bindings::STENCIL_INDEX8 => {}
        gl_bindings::R16F
        | gl_bindings::RG16F
        | gl_bindings::R32F
        | gl_bindings::RG32F
        | gl_bindings::RGBA32F
        | gl_bindings::R11F_G11F_B10F => {}
        _ => {
            result = Some(WebGLResult::None);
        }
    }

    if result.is_some() {
        return result.unwrap();
    }
    state.make_current();
    let ret = if pname == gl_bindings::SAMPLES {
        let mut length = [0i32];
        unsafe {
            gl_bindings::GetInternalformativ(
                target,
                internalformat,
                gl_bindings::NUM_SAMPLE_COUNTS,
                1,
                length.as_mut_ptr(),
            )
        }
        if length[0] <= 0 {
            return WebGLResult::I32Array(Vec::with_capacity(0));
        }
        let mut values: Vec<i32> = vec![0i32;length[0] as usize];
        unsafe {
            gl_bindings::GetInternalformativ(
                target,
                internalformat,
                pname,
                length[0],
                values.as_mut_ptr(),
            )
        }
        WebGLResult::I32Array(values)
    } else {
        WebGLResult::None
    };

    state.make_current();

    ret
}

pub fn canvas_native_webgl2_get_parameter(pname: u32, state: &mut WebGLState) -> WebGLResult {
    state.make_current();
    match pname {
        gl_bindings::COPY_READ_BUFFER_BINDING
        | gl_bindings::COPY_WRITE_BUFFER_BINDING
        | gl_bindings::DRAW_FRAMEBUFFER_BINDING => {
            let mut params = [0i32];
            unsafe { gl_bindings::GetIntegerv(pname, params.as_mut_ptr()) }
            if params[0] == 0 {
                return WebGLResult::None;
            }
            return WebGLResult::I32(params[0]);
        }
        _ => canvas_native_webgl_get_parameter(pname, state),
    }
}

pub fn canvas_native_webgl2_get_query_parameter(
    query: u32,
    pname: u32,
    state: &mut WebGLState,
) -> WebGLResult {
    state.make_current();
    let mut params = [0u32];

    unsafe { gl_bindings::GetQueryObjectuiv(query, pname, params.as_mut_ptr()) }

    match pname {
        gl_bindings::QUERY_RESULT => WebGLResult::Boolean(params[0] == 1),
        gl_bindings::QUERY_RESULT_AVAILABLE => WebGLResult::U32(params[0]),
        _ => WebGLResult::None,
    }
}

pub fn canvas_native_webgl2_get_query(
    target: u32,
    pname: u32,
    state: &mut WebGLState,
) -> WebGLResult {
    state.make_current();
    let ret = if pname == gl_bindings::CURRENT_QUERY {
        let mut params = [0i32];
        unsafe { gl_bindings::GetQueryiv(target, pname, params.as_mut_ptr()) }
        WebGLResult::I32(params[0])
    } else {
        WebGLResult::None
    };

    ret
}

pub fn canvas_native_webgl2_get_sampler_parameter(
    sampler: u32,
    pname: u32,
    state: &mut WebGLState,
) -> WebGLResult {
    state.make_current();
    let ret = match pname {
        gl_bindings::TEXTURE_MAX_LOD | gl_bindings::TEXTURE_MIN_LOD => {
            let mut float_value = [0f32];
            unsafe { gl_bindings::GetSamplerParameterfv(sampler, pname, float_value.as_mut_ptr()) }
            return WebGLResult::F32(float_value[0]);
        }
        gl_bindings::TEXTURE_COMPARE_FUNC
        | gl_bindings::TEXTURE_COMPARE_MODE
        | gl_bindings::TEXTURE_MAG_FILTER
        | gl_bindings::TEXTURE_MIN_FILTER
        | gl_bindings::TEXTURE_WRAP_R
        | gl_bindings::TEXTURE_WRAP_S
        | gl_bindings::TEXTURE_WRAP_T => {
            let mut int_value = [0i32];
            unsafe { gl_bindings::GetSamplerParameteriv(sampler, pname, int_value.as_mut_ptr()) };
            return WebGLResult::I32(int_value[0]);
        }
        _ => WebGLResult::None,
    };

    ret
}

pub fn canvas_native_webgl2_get_sync_parameter(
    sync: &GLSync,
    pname: u32,
    state: &mut WebGLState,
) -> WebGLResult {
    state.make_current();
    let ret = match pname {
        gl_bindings::OBJECT_TYPE
        | gl_bindings::SYNC_STATUS
        | gl_bindings::SYNC_CONDITION
        | gl_bindings::SYNC_FLAGS => {
            let mut value = [0i32];
            let mut length = [0i32];
            unsafe {
                gl_bindings::GetSynciv(sync.0, pname, 1, length.as_mut_ptr(), value.as_mut_ptr())
            }
            return WebGLResult::I32(value[0]);
        }
        _ => WebGLResult::None,
    };

    ret
}

pub fn canvas_native_webgl2_get_transform_feedback_varying(
    program: u32,
    index: u32,
    state: &mut WebGLState,
) -> WebGLActiveInfo {
    state.make_current();
    let mut max_index = [0i32];
    unsafe {
        gl_bindings::GetProgramiv(
            program,
            gl_bindings::TRANSFORM_FEEDBACK_VARYINGS,
            max_index.as_mut_ptr(),
        )
    }
    if index >= max_index[0].try_into().unwrap_or_default() {
        return WebGLActiveInfo::empty();
    }
    let mut max_name_length = [0i32];
    unsafe {
        gl_bindings::GetProgramiv(
            program,
            gl_bindings::TRANSFORM_FEEDBACK_VARYING_MAX_LENGTH,
            max_name_length.as_mut_ptr(),
        )
    }
    if max_name_length[0] <= 0 {
        return WebGLActiveInfo::empty();
    }
    let mut name = vec![0; max_name_length[0] as usize];
    let mut length = [0i32];
    let mut size = [0i32];
    let mut type_ = [0u32];

    unsafe {
        gl_bindings::GetTransformFeedbackVarying(
            program,
            index,
            max_name_length[0],
            length.as_mut_ptr(),
            size.as_mut_ptr(),
            type_.as_mut_ptr(),
            name.as_mut_ptr() as *mut c_char,
        )
    }

    if length[0] == 0 || size[0] == 0 || type_[0] == 0 {
        return WebGLActiveInfo::empty();
    }
    name.shrink_to(length[0] as usize);

    let c_str = unsafe { CStr::from_ptr(name.as_ptr()) };
    WebGLActiveInfo::new(c_str.to_string_lossy().to_string(), size[0], type_[0])
}

pub fn canvas_native_webgl2_get_uniform_block_index(
    program: u32,
    uniform_block_name: &str,
    state: &mut WebGLState,
) -> u32 {
    state.make_current();
    let name = CString::new(uniform_block_name).unwrap();
    let ret = unsafe { gl_bindings::GetUniformBlockIndex(program, name.as_ptr()) };

    ret
}

pub fn canvas_native_webgl2_get_uniform_indices(
    program: u32,
    uniform_names: &[String],
    state: &mut WebGLState,
) -> Vec<u32> {
    state.make_current();
    // TODO improve performance
    let mut count: Vec<u32> = vec![0;uniform_names.len()];
    let mut buffer: Vec<CString> = Vec::with_capacity(uniform_names.len());
    for name in uniform_names.into_iter() {
        let name = CString::new(name.as_str()).unwrap();
        buffer.push(name);
    }

    let buf: Vec<*const c_char> = buffer.iter().map(|f| f.as_ptr()).collect();
    unsafe {
        gl_bindings::GetUniformIndices(
            program,
            uniform_names.len().try_into().unwrap(),
            buf.as_ptr(),
            count.as_mut_ptr(),
        )
    }

    count
}

pub fn canvas_native_webgl2_invalidate_framebuffer(
    target: u32,
    attachments: &[u32],
    state: &mut WebGLState,
) {
    state.make_current();
    unsafe {
        gl_bindings::InvalidateFramebuffer(
            target,
            attachments.len().try_into().unwrap(),
            attachments.as_ptr(),
        );
    }
}

pub fn canvas_native_webgl2_invalidate_sub_framebuffer(
    target: u32,
    attachments: &[u32],
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    state: &mut WebGLState,
) {
    state.make_current();
    unsafe {
        gl_bindings::InvalidateSubFramebuffer(
            target,
            attachments.len().try_into().unwrap(),
            attachments.as_ptr(),
            x,
            y,
            width,
            height,
        );
    }
}

pub fn canvas_native_webgl2_is_query(query: u32, state: &mut WebGLState) -> bool {
    state.make_current();
    let ret = unsafe { gl_bindings::IsQuery(query) == 1 };

    ret
}

pub fn canvas_native_webgl2_is_sampler(sampler: u32, state: &mut WebGLState) -> bool {
    state.make_current();
    let ret = unsafe { gl_bindings::IsSampler(sampler) == 1 };

    ret
}

pub fn canvas_native_webgl2_is_sync(sync: &GLSync, state: &mut WebGLState) -> bool {
    state.make_current();
    let ret = unsafe { gl_bindings::IsSync(sync.0) == 1 };

    ret
}

pub fn canvas_native_webgl2_is_transform_feedback(
    transform_feedback: u32,
    state: &mut WebGLState,
) -> bool {
    state.make_current();
    let ret = unsafe { gl_bindings::IsTransformFeedback(transform_feedback) == 1 };

    ret
}

pub fn canvas_native_webgl2_is_vertex_array(vertex_array: u32, state: &mut WebGLState) -> bool {
    state.make_current();
    let ret = unsafe { gl_bindings::IsVertexArray(vertex_array) == 1 };

    ret
}

pub fn canvas_native_webgl2_pause_transform_feedback(state: &mut WebGLState) {
    state.make_current();
    unsafe {
        gl_bindings::PauseTransformFeedback();
    }
}

pub fn canvas_native_webgl2_read_buffer(src: u32, state: &mut WebGLState) {
    state.make_current();
    unsafe {
        gl_bindings::ReadBuffer(src);
    }
}

pub fn canvas_native_webgl2_renderbuffer_storage_multisample(
    target: u32,
    samples: i32,
    internal_format: u32,
    width: i32,
    height: i32,
    state: &mut WebGLState,
) {
    state.make_current();
    unsafe {
        gl_bindings::RenderbufferStorageMultisample(
            target,
            samples,
            internal_format,
            width,
            height,
        );
    }
}

pub fn canvas_native_webgl2_resume_transform_feedback(state: &mut WebGLState) {
    state.make_current();
    unsafe {
        gl_bindings::ResumeTransformFeedback();
    }
}

pub fn canvas_native_webgl2_sampler_parameterf(
    sampler: u32,
    pname: u32,
    param: f32,
    state: &mut WebGLState,
) {
    state.make_current();
    unsafe {
        gl_bindings::SamplerParameterf(sampler, pname, param);
    }
}

pub fn canvas_native_webgl2_sampler_parameteri(
    sampler: u32,
    pname: u32,
    param: i32,
    state: &mut WebGLState,
) {
    state.make_current();
    unsafe {
        gl_bindings::SamplerParameteri(sampler, pname, param);
    }
}

pub fn canvas_native_webgl2_tex_image3d_none(
    target: u32,
    level: i32,
    internalformat: i32,
    width: i32,
    height: i32,
    depth: i32,
    border: i32,
    format: u32,
    type_: u32,
    offset: usize,
    state: &mut WebGLState,
) {
    state.make_current();
    unsafe {
        gl_bindings::TexImage3D(
            target,
            level,
            internalformat,
            width,
            height,
            depth,
            border,
            format,
            type_,
            offset as *const c_void,
        )
    }
}

pub fn canvas_native_webgl2_tex_image3d_asset(
    target: u32,
    level: i32,
    _internalformat: i32,
    _width: i32,
    _height: i32,
    depth: i32,
    border: i32,
    format: u32,
    type_: u32,
    asset: &ImageAsset,
    state: &mut WebGLState,
) {
    if let Some(bytes) = asset.get_bytes() {
        let width = asset.width() as i32;
        let height = asset.height() as i32;
        state.make_current();
        unsafe {
            if state.get_flip_y() {
                let mut buffer = bytes.to_vec();
                utils::gl::flip_in_place_3d(
                    buffer.as_mut_ptr(),
                    buffer.len(),
                    (utils::gl::bytes_per_pixel(type_ as u32, format as u32) as i32 * width)
                        as usize,
                    height as usize,
                    depth as usize,
                );

                gl_bindings::TexImage3D(
                    target,
                    level,
                    //internalformat,
                    gl_bindings::RGBA as _,
                    width,
                    height,
                    depth,
                    border,
                   // format,
                    gl_bindings::RGBA as _,
                   // type_,
                    gl_bindings::UNSIGNED_BYTE as _,
                    buffer.as_ptr() as *const c_void,
                );

                return;
            }

            gl_bindings::TexImage3D(
                target,
                level,
               // internalformat,
                gl_bindings::RGBA as _,
                width,
                height,
                depth,
                border,
                //format,
                gl_bindings::RGBA as _,
                //type_,
                gl_bindings::UNSIGNED_BYTE as _,
                bytes.as_ptr() as *const c_void,
            );
        }
    }
}

pub fn canvas_native_webgl2_tex_image3d(
    target: u32,
    level: i32,
    internalformat: i32,
    width: i32,
    height: i32,
    depth: i32,
    border: i32,
    format: u32,
    type_: u32,
    buf: &[u8],
    state: &mut WebGLState,
) {
    state.make_current();
    unsafe {
        if state.get_flip_y() {
            let mut buffer = buf.to_vec();
            utils::gl::flip_in_place_3d(
                buffer.as_mut_ptr(),
                buffer.len(),
                (utils::gl::bytes_per_pixel(type_ as u32, format as u32) as i32 * width) as usize,
                height as usize,
                depth as usize,
            );

            gl_bindings::TexImage3D(
                target,
                level,
                internalformat,
                width,
                height,
                depth,
                border,
                format,
                type_,
                buffer.as_ptr() as *const c_void,
            );

            return;
        }

        gl_bindings::TexImage3D(
            target,
            level,
            internalformat,
            width,
            height,
            depth,
            border,
            format,
            type_,
            buf.as_ptr() as *const c_void,
        );
    }
}

pub fn canvas_native_webgl2_tex_image3d_offset(
    target: u32,
    level: i32,
    internalformat: i32,
    width: i32,
    height: i32,
    depth: i32,
    border: i32,
    format: u32,
    type_: u32,
    buf: &[u8],
    offset: usize,
    state: &mut WebGLState,
) {
    if offset > buf.len() {
        return;
    }
    let new_buf = unsafe {
        std::slice::from_raw_parts(buf.as_ptr().offset(offset as isize), buf.len() - offset)
    };
    canvas_native_webgl2_tex_image3d(
        target,
        level,
        internalformat,
        width,
        height,
        depth,
        border,
        format,
        type_,
        new_buf,
        state,
    )
}

pub fn canvas_native_webgl2_tex_storage2d(
    target: u32,
    levels: i32,
    internalformat: u32,
    width: i32,
    height: i32,
    state: &mut WebGLState,
) {
    state.make_current();
    unsafe {
        gl_bindings::TexStorage2D(target, levels, internalformat, width, height);
    }
}

pub fn canvas_native_webgl2_tex_storage3d(
    target: u32,
    levels: i32,
    internalformat: u32,
    width: i32,
    height: i32,
    depth: i32,
    state: &mut WebGLState,
) {
    state.make_current();
    unsafe {
        gl_bindings::TexStorage3D(target, levels, internalformat, width, height, depth);
    }
}

pub fn canvas_native_webgl2_tex_sub_image3d_none(
    target: u32,
    level: i32,
    xoffset: i32,
    yoffset: i32,
    zoffset: i32,
    width: i32,
    height: i32,
    depth: i32,
    format: u32,
    type_: u32,
    offset: usize,
    state: &mut WebGLState,
) {
    state.make_current();
    unsafe {
        gl_bindings::TexSubImage3D(
            target,
            level,
            xoffset,
            yoffset,
            zoffset,
            width,
            height,
            depth,
            format,
            type_,
            offset as *const c_void,
        );
    }
}

pub fn canvas_native_webgl2_tex_sub_image3d(
    target: u32,
    level: i32,
    xoffset: i32,
    yoffset: i32,
    zoffset: i32,
    width: i32,
    height: i32,
    depth: i32,
    format: u32,
    type_: u32,
    buf: &[u8],
    state: &mut WebGLState,
) {
    state.make_current();

    if state.get_flip_y() {
        let mut buffer = buf.to_vec();
        utils::gl::flip_in_place_3d(
            buffer.as_mut_ptr(),
            buffer.len(),
            (utils::gl::bytes_per_pixel(type_ as u32, format as u32) as i32 * width) as usize,
            height as usize,
            depth as usize,
        );

        unsafe {
            gl_bindings::TexSubImage3D(
                target,
                level,
                xoffset,
                yoffset,
                zoffset,
                width,
                height,
                depth,
                format,
                type_,
                buffer.as_ptr() as *const c_void,
            );
        }

        return;
    }

    unsafe {
        gl_bindings::TexSubImage3D(
            target,
            level,
            xoffset,
            yoffset,
            zoffset,
            width,
            height,
            depth,
            format,
            type_,
            buf.as_ptr() as *const c_void,
        );
    }
}

pub fn canvas_native_webgl2_tex_sub_image3d_asset(
    target: u32,
    level: i32,
    xoffset: i32,
    yoffset: i32,
    zoffset: i32,
    width: i32,
    height: i32,
    depth: i32,
    format: u32,
    type_: u32,
    asset: &ImageAsset,
    state: &mut WebGLState,
) {
    if let Some(bytes) = asset.get_bytes() {
        state.make_current();
        unsafe {
            if state.get_flip_y() {
                let mut buffer = bytes.to_vec();
                utils::gl::flip_in_place_3d(
                    buffer.as_mut_ptr(),
                    buffer.len(),
                    (utils::gl::bytes_per_pixel(type_ as u32, format as u32) as i32
                        * asset.width() as i32) as usize,
                    asset.height() as usize,
                    depth as usize,
                );


                gl_bindings::TexSubImage3D(
                    target,
                    level,
                    xoffset,
                    yoffset,
                    zoffset,
                    width,
                    height,
                    depth,
                   // format,
                    gl_bindings::RGBA as _,
                    //type_,
                    gl_bindings::UNSIGNED_BYTE as _,
                    buffer.as_ptr() as *const c_void,
                );

                return;
            }

            gl_bindings::TexSubImage3D(
                target,
                level,
                xoffset,
                yoffset,
                zoffset,
                width,
                height,
                depth,
               // format,
                gl_bindings::RGBA as _,
               // type_,
                gl_bindings::UNSIGNED_BYTE as _,
                bytes.as_ptr() as *const c_void,
            );
        }
    }
}

pub fn canvas_native_webgl2_tex_sub_image3d_offset(
    target: u32,
    level: i32,
    xoffset: i32,
    yoffset: i32,
    zoffset: i32,
    width: i32,
    height: i32,
    depth: i32,
    format: u32,
    type_: u32,
    buf: &[u8],
    offset: usize,
    state: &mut WebGLState,
) {
    if offset > buf.len() {
        return;
    }
    let new_buf = unsafe {
        std::slice::from_raw_parts(buf.as_ptr().offset(offset as isize), buf.len() - offset)
    };

    canvas_native_webgl2_tex_sub_image3d(
        target, level, xoffset, yoffset, zoffset, width, height, depth, format, type_, new_buf,
        state,
    );
}

pub fn canvas_native_webgl2_transform_feedback_varyings(
    program: u32,
    varyings: &[String],
    buffer_mode: u32,
    state: &mut WebGLState,
) {
    state.make_current();

    // todo improve performance ... no allocation :D
    let mut buf: Vec<CString> = Vec::with_capacity(varyings.len());
    for vary in varyings.into_iter() {
        buf.push(CString::new(vary.as_str()).unwrap())
    }

    let buffer: Vec<*const c_char> = buf.iter().map(|f| f.as_ptr()).collect();

    unsafe {
        gl_bindings::TransformFeedbackVaryings(
            program,
            buffer.len().try_into().unwrap(),
            buffer.as_ptr(),
            buffer_mode,
        );
    }
}

pub fn canvas_native_webgl2_uniform1ui(location: i32, v0: u32, state: &mut WebGLState) {
    state.make_current();
    unsafe {
        gl_bindings::Uniform1ui(location, v0);
    }
}

pub fn canvas_native_webgl2_uniform1uiv(location: i32, data: &[u32], state: &mut WebGLState) {
    state.make_current();
    unsafe {
        gl_bindings::Uniform1uiv(location, data.len().try_into().unwrap(), data.as_ptr());
    }
}

pub fn canvas_native_webgl2_uniform2ui(location: i32, v0: u32, v1: u32, state: &mut WebGLState) {
    state.make_current();
    unsafe {
        gl_bindings::Uniform2ui(location, v0, v1);
    }
}

pub fn canvas_native_webgl2_uniform2uiv(location: i32, data: &[u32], state: &mut WebGLState) {
    state.make_current();
    let count = data.len() / 2;
    unsafe {
        gl_bindings::Uniform2uiv(location, count.try_into().unwrap(), data.as_ptr());
    }
}

pub fn canvas_native_webgl2_uniform3ui(
    location: i32,
    v0: u32,
    v1: u32,
    v2: u32,
    state: &mut WebGLState,
) {
    state.make_current();
    unsafe {
        gl_bindings::Uniform3ui(location, v0, v1, v2);
    }
}

pub fn canvas_native_webgl2_uniform3uiv(location: i32, data: &[u32], state: &mut WebGLState) {
    state.make_current();
    let count = data.len() / 3;
    unsafe {
        gl_bindings::Uniform3uiv(location, count.try_into().unwrap(), data.as_ptr());
    }
}

pub fn canvas_native_webgl2_uniform4ui(
    location: i32,
    v0: u32,
    v1: u32,
    v2: u32,
    v3: u32,
    state: &mut WebGLState,
) {
    state.make_current();
    unsafe {
        gl_bindings::Uniform4ui(location, v0, v1, v2, v3);
    }
}

pub fn canvas_native_webgl2_uniform4uiv(location: i32, data: &[u32], state: &mut WebGLState) {
    state.make_current();
    let count = data.len() / 4;
    unsafe {
        gl_bindings::Uniform4uiv(location, count.try_into().unwrap(), data.as_ptr());
    }
}

pub fn canvas_native_webgl2_uniform_block_binding(
    program: u32,
    uniform_block_index: u32,
    uniform_block_binding: u32,
    state: &mut WebGLState,
) {
    state.make_current();
    unsafe {
        gl_bindings::UniformBlockBinding(program, uniform_block_index, uniform_block_binding);
    }
}

pub fn canvas_native_webgl2_uniform_matrix2x3fv(
    location: i32,
    transpose: bool,
    data: &[f32],
    state: &mut WebGLState,
) {
    state.make_current();
    let count = data.len() / 6;
    unsafe {
        gl_bindings::UniformMatrix2x3fv(
            location,
            count.try_into().unwrap(),
            transpose as u8,
            data.as_ptr(),
        );
    }
}

pub fn canvas_native_webgl2_uniform_matrix2x4fv(
    location: i32,
    transpose: bool,
    data: &[f32],
    state: &mut WebGLState,
) {
    state.make_current();
    let count = data.len() / 8;
    unsafe {
        gl_bindings::UniformMatrix2x4fv(
            location,
            count.try_into().unwrap(),
            transpose as u8,
            data.as_ptr(),
        );
    }
}

pub fn canvas_native_webgl2_uniform_matrix3x2fv(
    location: i32,
    transpose: bool,
    data: &[f32],
    state: &mut WebGLState,
) {
    state.make_current();
    let count = data.len() / 6;
    unsafe {
        gl_bindings::UniformMatrix3x2fv(
            location,
            count.try_into().unwrap(),
            transpose as u8,
            data.as_ptr(),
        );
    }
}

pub fn canvas_native_webgl2_uniform_matrix3x4fv(
    location: i32,
    transpose: bool,
    data: &[f32],
    state: &mut WebGLState,
) {
    state.make_current();
    let count = data.len() / 12;
    unsafe {
        gl_bindings::UniformMatrix3x4fv(
            location,
            count.try_into().unwrap(),
            transpose as u8,
            data.as_ptr(),
        );
    }
}

pub fn canvas_native_webgl2_uniform_matrix4x2fv(
    location: i32,
    transpose: bool,
    data: &[f32],
    state: &mut WebGLState,
) {
    state.make_current();
    let count = data.len() / 8;
    unsafe {
        gl_bindings::UniformMatrix4x2fv(
            location,
            count.try_into().unwrap(),
            transpose as u8,
            data.as_ptr(),
        );
    }
}

pub fn canvas_native_webgl2_uniform_matrix4x3fv(
    location: i32,
    transpose: bool,
    data: &[f32],
    state: &mut WebGLState,
) {
    state.make_current();
    let count = data.len() / 12;
    unsafe {
        gl_bindings::UniformMatrix4x3fv(
            location,
            count.try_into().unwrap(),
            transpose as u8,
            data.as_ptr(),
        );
    }
}

pub fn canvas_native_webgl2_vertex_attrib_divisor(
    index: u32,
    divisor: u32,
    state: &mut WebGLState,
) {
    state.make_current();
    unsafe {
        gl_bindings::VertexAttribDivisor(index, divisor);
    }
}

pub fn canvas_native_webgl2_vertex_attrib_i4i(
    index: u32,
    x: i32,
    y: i32,
    z: i32,
    w: i32,
    state: &mut WebGLState,
) {
    state.make_current();
    unsafe {
        gl_bindings::VertexAttribI4i(index, x, y, z, w);
    }
}

pub fn canvas_native_webgl2_vertex_attrib_i4iv(index: u32, value: &[i32], state: &mut WebGLState) {
    state.make_current();
    unsafe {
        gl_bindings::VertexAttribI4iv(index, value.as_ptr());
    }
}

pub fn canvas_native_webgl2_vertex_attrib_i4ui(
    index: u32,
    x: u32,
    y: u32,
    z: u32,
    w: u32,
    state: &mut WebGLState,
) {
    state.make_current();
    unsafe {
        gl_bindings::VertexAttribI4ui(index, x, y, z, w);
    }
}

pub fn canvas_native_webgl2_vertex_attrib_i4uiv(index: u32, value: &[u32], state: &mut WebGLState) {
    state.make_current();
    unsafe {
        gl_bindings::VertexAttribI4uiv(index, value.as_ptr());
    }
}

pub const READ_BUFFER: u32 = 0x0C02;

pub const UNPACK_ROW_LENGTH: u32 = 0x0CF2;

pub const UNPACK_SKIP_ROWS: u32 = 0x0CF3;

pub const UNPACK_SKIP_PIXELS: u32 = 0x0CF4;

pub const PACK_ROW_LENGTH: u32 = 0x0D02;

pub const PACK_SKIP_ROWS: u32 = 0x0D03;

pub const PACK_SKIP_PIXELS: u32 = 0x0D04;

pub const TEXTURE_BINDING_3D: u32 = 0x806A;

pub const UNPACK_SKIP_IMAGES: u32 = 0x806D;

pub const UNPACK_IMAGE_HEIGHT: u32 = 0x806E;

pub const MAX_3D_TEXTURE_SIZE: u32 = 0x8073;

pub const MAX_ELEMENTS_VERTICES: u32 = 0x80E8;

pub const MAX_ELEMENTS_INDICES: u32 = 0x80E9;

pub const MAX_TEXTURE_LOD_BIAS: u32 = 0x84FD;

pub const MAX_FRAGMENT_UNIFORM_COMPONENTS: u32 = 0x8B49;

pub const MAX_VERTEX_UNIFORM_COMPONENTS: u32 = 0x8B4A;

pub const MAX_ARRAY_TEXTURE_LAYERS: u32 = 0x88FF;

pub const MIN_PROGRAM_TEXEL_OFFSET: u32 = 0x8904;

pub const MAX_PROGRAM_TEXEL_OFFSET: u32 = 0x8905;

pub const MAX_VARYING_COMPONENTS: u32 = 0x8B4B;

pub const FRAGMENT_SHADER_DERIVATIVE_HINT: u32 = 0x8B8B;

pub const RASTERIZER_DISCARD: u32 = 0x8C89;

pub const VERTEX_ARRAY_BINDING: u32 = 0x85B5;

pub const MAX_VERTEX_OUTPUT_COMPONENTS: u32 = 0x9122;

pub const MAX_FRAGMENT_INPUT_COMPONENTS: u32 = 0x9125;

pub const MAX_SERVER_WAIT_TIMEOUT: u32 = 0x9111;

pub const MAX_ELEMENT_INDEX: u32 = 0x8D6B;

pub const RED: u32 = 0x1903;

pub const RGB8: u32 = 0x8051;

pub const RGBA8: u32 = 0x8058;

pub const RGB10_A2: u32 = 0x8059;

pub const TEXTURE_3D: u32 = 0x806F;

pub const TEXTURE_WRAP_R: u32 = 0x8072;

pub const TEXTURE_MIN_LOD: u32 = 0x813A;

pub const TEXTURE_MAX_LOD: u32 = 0x813B;

pub const TEXTURE_BASE_LEVEL: u32 = 0x813C;

pub const TEXTURE_MAX_LEVEL: u32 = 0x813D;

pub const TEXTURE_COMPARE_MODE: u32 = 0x884C;

pub const TEXTURE_COMPARE_FUNC: u32 = 0x884D;

pub const SRGB: u32 = 0x8C40;

pub const SRGB8: u32 = 0x8C41;

pub const SRGB8_ALPHA8: u32 = 0x8C43;

pub const COMPARE_REF_TO_TEXTURE: u32 = 0x884E;

pub const RGBA32F: u32 = 0x8814;

pub const RGB32F: u32 = 0x8815;

pub const RGBA16F: u32 = 0x881A;

pub const RGB16F: u32 = 0x881B;

pub const TEXTURE_2D_ARRAY: u32 = 0x8C1A;

pub const TEXTURE_BINDING_2D_ARRAY: u32 = 0x8C1D;

pub const R11F_G11F_B10F: u32 = 0x8C3A;

pub const RGB9_E5: u32 = 0x8C3D;

pub const RGBA32UI: u32 = 0x8D70;

pub const RGB32UI: u32 = 0x8D71;

pub const RGBA16UI: u32 = 0x8D76;

pub const RGB16UI: u32 = 0x8D77;

pub const RGBA8UI: u32 = 0x8D7C;

pub const RGB8UI: u32 = 0x8D7D;

pub const RGBA32I: u32 = 0x8D82;

pub const RGB32I: u32 = 0x8D83;

pub const RGBA16I: u32 = 0x8D88;

pub const RGB16I: u32 = 0x8D89;

pub const RGBA8I: u32 = 0x8D8E;

pub const RGB8I: u32 = 0x8D8F;

pub const RED_INTEGER: u32 = 0x8D94;

pub const RGB_INTEGER: u32 = 0x8D98;

pub const RGBA_INTEGER: u32 = 0x8D99;

pub const R8: u32 = 0x8229;

pub const RG8: u32 = 0x822B;

pub const R16F: u32 = 0x822D;

pub const R32F: u32 = 0x822E;

pub const RG16F: u32 = 0x822F;

pub const RG32F: u32 = 0x8230;

pub const R8I: u32 = 0x8231;

pub const R8UI: u32 = 0x8232;

pub const R16I: u32 = 0x8233;

pub const R16UI: u32 = 0x8234;

pub const R32I: u32 = 0x8235;

pub const R32UI: u32 = 0x8236;

pub const RG8I: u32 = 0x8237;

pub const RG8UI: u32 = 0x8238;

pub const RG16I: u32 = 0x8239;

pub const RG16UI: u32 = 0x823A;

pub const RG32I: u32 = 0x823B;

pub const RG32UI: u32 = 0x823C;

pub const R8_SNORM: u32 = 0x8F94;

pub const RG8_SNORM: u32 = 0x8F95;

pub const RGB8_SNORM: u32 = 0x8F96;

pub const RGBA8_SNORM: u32 = 0x8F97;

pub const RGB10_A2UI: u32 = 0x906F;

pub const TEXTURE_IMMUTABLE_FORMAT: u32 = 0x912F;

pub const TEXTURE_IMMUTABLE_LEVELS: u32 = 0x82DF;

pub const UNSIGNED_INT_2_10_10_10_REV: u32 = 0x8368;

pub const UNSIGNED_INT_10F_11F_11F_REV: u32 = 0x8C3B;

pub const UNSIGNED_INT_5_9_9_9_REV: u32 = 0x8C3E;

pub const FLOAT_32_UNSIGNED_INT_24_8_REV: u32 = 0x8DAD;

pub const UNSIGNED_INT_24_8: u32 = 0x84FA;

pub const HALF_FLOAT: u32 = 0x140B;

pub const RG: u32 = 0x8227;

pub const RG_INTEGER: u32 = 0x8228;

pub const INT_2_10_10_10_REV: u32 = 0x8D9F;

pub const QUERY_RESULT_AVAILABLE: u32 = 0x8865;

pub const QUERY_RESULT: u32 = 0x8866;

pub const CURRENT_QUERY: u32 = 0x8867;

pub const ANY_SAMPLES_PASSED: u32 = 0x8C2F;

pub const ANY_SAMPLES_PASSED_CONSERVATIVE: u32 = 0x8D6A;

pub const MAX_DRAW_BUFFERS: u32 = 0x8824;

pub const DRAW_BUFFER0: u32 = 0x8825;

pub const DRAW_BUFFER1: u32 = 0x8826;

pub const DRAW_BUFFER2: u32 = 0x8827;

pub const DRAW_BUFFER3: u32 = 0x8828;

pub const DRAW_BUFFER4: u32 = 0x8829;

pub const DRAW_BUFFER5: u32 = 0x882A;

pub const DRAW_BUFFER6: u32 = 0x882B;

pub const DRAW_BUFFER7: u32 = 0x882C;

pub const DRAW_BUFFER8: u32 = 0x882D;

pub const DRAW_BUFFER9: u32 = 0x882E;

pub const DRAW_BUFFER10: u32 = 0x882F;

/* Getting GL parameter information */

/* Textures */

pub const DRAW_BUFFER11: u32 = 0x8830;

pub const DRAW_BUFFER12: u32 = 0x8831;

pub const DRAW_BUFFER13: u32 = 0x8832;

pub const DRAW_BUFFER14: u32 = 0x8833;

pub const DRAW_BUFFER15: u32 = 0x8834;

pub const MAX_COLOR_ATTACHMENTS: u32 = 0x8CDF;

pub const COLOR_ATTACHMENT1: u32 = 0x8CE1;

pub const COLOR_ATTACHMENT2: u32 = 0x8CE2;

pub const COLOR_ATTACHMENT3: u32 = 0x8CE3;

pub const COLOR_ATTACHMENT4: u32 = 0x8CE4;

pub const COLOR_ATTACHMENT5: u32 = 0x8CE5;

pub const COLOR_ATTACHMENT6: u32 = 0x8CE6;

pub const COLOR_ATTACHMENT7: u32 = 0x8CE7;

pub const COLOR_ATTACHMENT8: u32 = 0x8CE8;

pub const COLOR_ATTACHMENT9: u32 = 0x8CE9;

pub const COLOR_ATTACHMENT10: u32 = 0x8CEA;

pub const COLOR_ATTACHMENT11: u32 = 0x8CEB;

pub const COLOR_ATTACHMENT12: u32 = 0x8CEC;

pub const COLOR_ATTACHMENT13: u32 = 0x8CED;

pub const COLOR_ATTACHMENT14: u32 = 0x8CEE;

pub const COLOR_ATTACHMENT15: u32 = 0x8CEF;

pub const SAMPLER_3D: u32 = 0x8B5F;

pub const SAMPLER_2D_SHADOW: u32 = 0x8B62;

pub const SAMPLER_2D_ARRAY: u32 = 0x8DC1;

pub const SAMPLER_2D_ARRAY_SHADOW: u32 = 0x8DC4;

pub const SAMPLER_CUBE_SHADOW: u32 = 0x8DC5;

pub const INT_SAMPLER_2D: u32 = 0x8DCA;

pub const INT_SAMPLER_3D: u32 = 0x8DCB;

pub const INT_SAMPLER_CUBE: u32 = 0x8DCC;

pub const INT_SAMPLER_2D_ARRAY: u32 = 0x8DCF;

pub const UNSIGNED_INT_SAMPLER_2D: u32 = 0x8DD2;

pub const UNSIGNED_INT_SAMPLER_3D: u32 = 0x8DD3;

pub const UNSIGNED_INT_SAMPLER_CUBE: u32 = 0x8DD4;

pub const UNSIGNED_INT_SAMPLER_2D_ARRAY: u32 = 0x8DD7;

pub const MAX_SAMPLES: u32 = 0x8D57;

pub const SAMPLER_BINDING: u32 = 0x8919;

pub const PIXEL_PACK_BUFFER: u32 = 0x88EB;

pub const PIXEL_UNPACK_BUFFER: u32 = 0x88EC;

pub const PIXEL_PACK_BUFFER_BINDING: u32 = 0x88ED;

pub const PIXEL_UNPACK_BUFFER_BINDING: u32 = 0x88EF;

pub const COPY_READ_BUFFER: u32 = 0x8F36;

pub const COPY_WRITE_BUFFER: u32 = 0x8F37;

pub const COPY_READ_BUFFER_BINDING: u32 = 0x8F36;

pub const COPY_WRITE_BUFFER_BINDING: u32 = 0x8F37;

pub const FLOAT_MAT2x3: u32 = 0x8B65;

pub const FLOAT_MAT2x4: u32 = 0x8B66;

pub const FLOAT_MAT3x2: u32 = 0x8B67;

pub const FLOAT_MAT3x4: u32 = 0x8B68;

pub const FLOAT_MAT4x2: u32 = 0x8B69;

pub const FLOAT_MAT4x3: u32 = 0x8B6A;

pub const UNSIGNED_INT_VEC2: u32 = 0x8DC6;

pub const UNSIGNED_INT_VEC3: u32 = 0x8DC7;

pub const UNSIGNED_INT_VEC4: u32 = 0x8DC8;

pub const UNSIGNED_NORMALIZED: u32 = 0x8C17;

pub const SIGNED_NORMALIZED: u32 = 0x8F9C;

/* Vertex attributes */
pub const VERTEX_ATTRIB_ARRAY_INTEGER: u32 = 0x88FD;

pub const VERTEX_ATTRIB_ARRAY_DIVISOR: u32 = 0x88FE;

pub const TRANSFORM_FEEDBACK_BUFFER_MODE: u32 = 0x8C7F;

pub const MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS: u32 = 0x8C80;

pub const TRANSFORM_FEEDBACK_VARYINGS: u32 = 0x8C83;

pub const TRANSFORM_FEEDBACK_BUFFER_START: u32 = 0x8C84;

pub const TRANSFORM_FEEDBACK_BUFFER_SIZE: u32 = 0x8C85;

pub const TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN: u32 = 0x8C88;

pub const MAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS: u32 = 0x8C8A;

/* Textures */

/* Pixel types */

pub const MAX_TRANSFORM_FEEDBACK_SEPARATE_ATTRIBS: u32 = 0x8C8B;

pub const INTERLEAVED_ATTRIBS: u32 = 0x8C8C;

pub const SEPARATE_ATTRIBS: u32 = 0x8C8D;

pub const TRANSFORM_FEEDBACK_BUFFER: u32 = 0x8C8E;

pub const TRANSFORM_FEEDBACK_BUFFER_BINDING: u32 = 0x8C8F;

pub const TRANSFORM_FEEDBACK: u32 = 0x8E22;

pub const TRANSFORM_FEEDBACK_PAUSED: u32 = 0x8E23;

pub const TRANSFORM_FEEDBACK_ACTIVE: u32 = 0x8E24;

pub const TRANSFORM_FEEDBACK_BINDING: u32 = 0x8E25;

/* Pixel types */

/* Queries */

pub const FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING: u32 = 0x8210;

pub const FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE: u32 = 0x8211;

pub const FRAMEBUFFER_ATTACHMENT_RED_SIZE: u32 = 0x8212;

pub const FRAMEBUFFER_ATTACHMENT_GREEN_SIZE: u32 = 0x8213;

pub const FRAMEBUFFER_ATTACHMENT_BLUE_SIZE: u32 = 0x8214;

/* Queries */

/* Draw buffers */

pub const FRAMEBUFFER_ATTACHMENT_ALPHA_SIZE: u32 = 0x8215;

pub const FRAMEBUFFER_ATTACHMENT_DEPTH_SIZE: u32 = 0x8216;

pub const FRAMEBUFFER_ATTACHMENT_STENCIL_SIZE: u32 = 0x8217;

pub const FRAMEBUFFER_DEFAULT: u32 = 0x8218;

pub const DEPTH_STENCIL_ATTACHMENT: u32 = 0x821A;

pub const DEPTH_STENCIL: u32 = 0x84F9;

pub const DEPTH24_STENCIL8: u32 = 0x88F0;

pub const DRAW_FRAMEBUFFER_BINDING: u32 = 0x8CA6;

pub const READ_FRAMEBUFFER: u32 = 0x8CA8;

pub const DRAW_FRAMEBUFFER: u32 = 0x8CA9;

pub const READ_FRAMEBUFFER_BINDING: u32 = 0x8CAA;

pub const RENDERBUFFER_SAMPLES: u32 = 0x8CAB;

pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER: u32 = 0x8CD4;

pub const FRAMEBUFFER_INCOMPLETE_MULTISAMPLE: u32 = 0x8D56;

pub const UNIFORM_BUFFER: u32 = 0x8A11;

pub const UNIFORM_BUFFER_BINDING: u32 = 0x8A28;

pub const UNIFORM_BUFFER_START: u32 = 0x8A29;

pub const UNIFORM_BUFFER_SIZE: u32 = 0x8A2A;

pub const MAX_VERTEX_UNIFORM_BLOCKS: u32 = 0x8A2B;

pub const MAX_FRAGMENT_UNIFORM_BLOCKS: u32 = 0x8A2D;

pub const MAX_COMBINED_UNIFORM_BLOCKS: u32 = 0x8A2E;

pub const MAX_UNIFORM_BUFFER_BINDINGS: u32 = 0x8A2F;

pub const MAX_UNIFORM_BLOCK_SIZE: u32 = 0x8A30;

pub const MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS: u32 = 0x8A31;

pub const MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS: u32 = 0x8A33;

pub const UNIFORM_BUFFER_OFFSET_ALIGNMENT: u32 = 0x8A34;

pub const ACTIVE_UNIFORM_BLOCKS: u32 = 0x8A36;

pub const UNIFORM_TYPE: u32 = 0x8A37;

pub const UNIFORM_SIZE: u32 = 0x8A38;

pub const UNIFORM_BLOCK_INDEX: u32 = 0x8A3A;

pub const UNIFORM_OFFSET: u32 = 0x8A3B;

pub const UNIFORM_ARRAY_STRIDE: u32 = 0x8A3C;

pub const UNIFORM_MATRIX_STRIDE: u32 = 0x8A3D;

/* Draw buffers */

/* Samplers */

pub const UNIFORM_IS_ROW_MAJOR: u32 = 0x8A3E;

pub const UNIFORM_BLOCK_BINDING: u32 = 0x8A3F;

pub const UNIFORM_BLOCK_DATA_SIZE: u32 = 0x8A40;

pub const UNIFORM_BLOCK_ACTIVE_UNIFORMS: u32 = 0x8A42;

pub const UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES: u32 = 0x8A43;

pub const UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER: u32 = 0x8A44;

pub const UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER: u32 = 0x8A46;

pub const OBJECT_TYPE: u32 = 0x9112;

pub const SYNC_CONDITION: u32 = 0x9113;

pub const SYNC_STATUS: u32 = 0x9114;

pub const SYNC_FLAGS: u32 = 0x9115;

pub const SYNC_FENCE: u32 = 0x9116;

pub const SYNC_GPU_COMMANDS_COMPLETE: u32 = 0x9117;

pub const UNSIGNALED: u32 = 0x9118;

pub const SIGNALED: u32 = 0x9119;

/* Samplers */

/* Buffers */

pub const ALREADY_SIGNALED: u32 = 0x911A;

pub const TIMEOUT_EXPIRED: u32 = 0x911B;

pub const CONDITION_SATISFIED: u32 = 0x911C;

pub const WAIT_FAILED: u32 = 0x911D;

pub const SYNC_FLUSH_COMMANDS_BIT: u32 = 0x00000001;

pub const COLOR: u32 = 0x1800;

pub const DEPTH: u32 = 0x1801;

pub const STENCIL: u32 = 0x1802;

/* Buffers */

/* Data types */

pub const MIN: u32 = 0x8007;

pub const MAX: u32 = 0x8008;

pub const DEPTH_COMPONENT24: u32 = 0x81A6;

pub const STREAM_READ: u32 = 0x88E1;

pub const STREAM_COPY: u32 = 0x88E2;

pub const STATIC_READ: u32 = 0x88E5;

pub const STATIC_COPY: u32 = 0x88E6;

pub const DYNAMIC_READ: u32 = 0x88E9;

pub const DYNAMIC_COPY: u32 = 0x88EA;

pub const DEPTH_COMPONENT32F: u32 = 0x8CAC;

pub const DEPTH32F_STENCIL8: u32 = 0x8CAD;

/* Data types */

pub const INVALID_INDEX: u32 = 0xFFFFFFFF;

pub const TIMEOUT_IGNORED: i32 = -1;

/* Vertex attributes */

/* Transform feedback */

pub const MAX_CLIENT_WAIT_TIMEOUT_WEBGL: u32 = 0x9247;
