use std::ffi::{c_ulong, CStr, CString};
use std::os::raw::{c_char, c_long, c_longlong, c_uchar, c_void};

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
    unsafe { gl_bindings::ClientWaitSync(sync.0, flags, timeout.into()) }
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
    unsafe { GLSync(gl_bindings::FenceSync(condition, flags)) }
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
    match pname {
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

            let mut indices: Vec<i32> = Vec::with_capacity(uniform_count[0] as usize);
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
    }
}

pub fn canvas_native_webgl2_get_active_uniforms(
    program: u32,
    uniform_indices: &[u32],
    pname: u32,
    state: &mut WebGLState,
) -> WebGLResult {
    state.make_current();
    let mut params: Vec<i32> = Vec::with_capacity(uniform_indices.len());

    unsafe {
        gl_bindings::GetActiveUniformsiv(
            program,
            uniform_indices.len().try_into().unwrap(),
            uniform_indices.as_ptr(),
            pname,
            params.as_mut_ptr(),
        )
    }

    match pname {
        gl_bindings::UNIFORM_TYPE | gl_bindings::UNIFORM_SIZE => {
            WebGLResult::U32Array(unsafe {
                std::slice::from_raw_parts(params.as_ptr() as *const u32, params.len()).to_vec()
            })
        }
        gl_bindings::UNIFORM_BLOCK_INDEX
        | gl_bindings::UNIFORM_OFFSET
        | gl_bindings::UNIFORM_ARRAY_STRIDE
        | gl_bindings::UNIFORM_MATRIX_STRIDE => WebGLResult::I32Array(params),
        gl_bindings::UNIFORM_IS_ROW_MAJOR => WebGLResult::BooleanArray(unsafe {
            std::slice::from_raw_parts(params.as_ptr() as *const bool, params.len()).to_vec()
        }),
        _ => WebGLResult::None,
    }
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

    let mut len = dst_data.len();

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

    let mut new_buf = unsafe { std::slice::from_raw_parts_mut(ptr, len) };

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
    unsafe { gl_bindings::GetFragDataLocation(program, name.as_ptr()) }
}

pub fn canvas_native_webgl2_get_indexed_parameter(
    target: u32,
    index: u32,
    state: &mut WebGLState,
) -> WebGLIndexedParameter {
    state.make_current();
    let mut binding = WebGLIndexedParameter::default();

    match target {
        gl_bindings::UNIFORM_BUFFER_BINDING
        | gl_bindings::TRANSFORM_FEEDBACK_BUFFER_BINDING => {
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
    }
}

pub fn canvas_native_webgl2_get_internalformat_parameter(
    target: u32,
    internalformat: u32,
    pname: u32,
    state: &mut WebGLState,
) -> WebGLResult {
    state.make_current();
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
    return if pname == gl_bindings::SAMPLES {
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
        let mut values: Vec<i32> = Vec::with_capacity(length[0] as usize);
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
    let mut params = 0u32;

    unsafe { gl_bindings::GetQueryObjectuiv(query, pname, &mut params) }

    match pname {
        gl_bindings::QUERY_RESULT => WebGLResult::Boolean(params == 1),
        gl_bindings::QUERY_RESULT_AVAILABLE => WebGLResult::U32(params),
        _ => WebGLResult::None,
    }
}

pub fn canvas_native_webgl2_get_query(
    target: u32,
    pname: u32,
    state: &mut WebGLState,
) -> WebGLResult {
    state.make_current();
    if pname == gl_bindings::CURRENT_QUERY {
        let mut params = [0i32];
        unsafe { gl_bindings::GetQueryiv(target, pname, params.as_mut_ptr()) }
        WebGLResult::I32(params[0])
    } else {
        WebGLResult::None
    }
}

pub fn canvas_native_webgl2_get_sampler_parameter(
    sampler: u32,
    pname: u32,
    state: &mut WebGLState,
) -> WebGLResult {
    state.make_current();
    match pname {
        gl_bindings::TEXTURE_MAX_LOD | gl_bindings::TEXTURE_MIN_LOD => {
            let mut float_value = [0f32];
            unsafe {
                gl_bindings::GetSamplerParameterfv(sampler, pname, float_value.as_mut_ptr())
            }
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
    }
}

pub fn canvas_native_webgl2_get_sync_parameter(
    sync: &GLSync,
    pname: u32,
    state: &mut WebGLState,
) -> WebGLResult {
    state.make_current();
    match pname {
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
    }
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
    unsafe { gl_bindings::GetUniformBlockIndex(program, name.as_ptr()) }
}

pub fn canvas_native_webgl2_get_uniform_indices(
    program: u32,
    uniform_names: &[String],
    state: &mut WebGLState,
) -> Vec<u32> {
    state.make_current();
    // TODO improve performance
    let mut count: Vec<u32> = Vec::with_capacity(uniform_names.len());
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
    unsafe { gl_bindings::IsQuery(query) == 1 }
}

pub fn canvas_native_webgl2_is_sampler(sampler: u32, state: &mut WebGLState) -> bool {
    state.make_current();
    unsafe { gl_bindings::IsSampler(sampler) == 1 }
}

pub fn canvas_native_webgl2_is_sync(sync: &GLSync, state: &mut WebGLState) -> bool {
    state.make_current();
    unsafe { gl_bindings::IsSync(sync.0) == 1 }
}

pub fn canvas_native_webgl2_is_transform_feedback(
    transform_feedback: u32,
    state: &mut WebGLState,
) -> bool {
    state.make_current();
    unsafe { gl_bindings::IsTransformFeedback(transform_feedback) == 1 }
}

pub fn canvas_native_webgl2_is_vertex_array(vertex_array: u32, state: &mut WebGLState) -> bool {
    state.make_current();
    unsafe { gl_bindings::IsVertexArray(vertex_array) == 1 }
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
    internalformat: i32,
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
                    (utils::gl::bytes_per_pixel(type_ as u32, format as u32) as i32
                        * width) as usize,
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
                (utils::gl::bytes_per_pixel(type_ as u32, format as u32) as i32
                    * width) as usize,
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
            (utils::gl::bytes_per_pixel(type_ as u32, format as u32) as i32 * width)
                as usize,
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
    state.make_current();

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
