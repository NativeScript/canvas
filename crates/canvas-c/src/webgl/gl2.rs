use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_ulong, c_void};
use canvas_webgl::utils;
use canvas_webgl::utils::gl::bytes_per_pixel;
use crate::buffers::U32Buffer;
use crate::c2d::CanvasRenderingContext2D;
use crate::image_asset::ImageAsset;
use crate::ImageData;
use crate::webgl::gl::{WebGLActiveInfo, WebGLResult, WebGLState};

pub struct WebGLSync(canvas_webgl::webgl2::GLSync);

pub struct WebGLIndexedParameter(canvas_webgl::prelude::WebGLIndexedParameter);

/* WebGLIndexedParameter */
#[no_mangle]
pub extern "C" fn canvas_native_webgl2_indexed_parameter_get_value(
    param: &WebGLIndexedParameter,
) -> isize {
    param.0.get_value()
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_indexed_parameter_get_buffer_value(
    param: &WebGLIndexedParameter,
) -> isize {
    param.0.get_buffer_value()
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_indexed_parameter_get_is_buffer(
    param: &WebGLIndexedParameter,
) -> bool {
    param.0.get_is_buffer()
}
/* WebGLIndexedParameter */

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_begin_query(target: u32, id: u32, state: *mut WebGLState) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_begin_query(target, id, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_begin_transform_feedback(
    primitive_mode: u32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_begin_transform_feedback(
        primitive_mode,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_bind_buffer_base(
    target: u32,
    index: u32,
    buffer: u32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_bind_buffer_base(
        target,
        index,
        buffer,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_bind_buffer_range(
    target: u32,
    index: u32,
    buffer: u32,
    offset: isize,
    size: isize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_bind_buffer_range(
        target,
        index,
        buffer,
        offset,
        size,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_bind_sampler(
    unit: u32,
    sampler: u32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_bind_sampler(unit, sampler, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_bind_transform_feedback(
    target: u32,
    transform_feedback: u32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_bind_sampler(
        target,
        transform_feedback,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_bind_vertex_array(
    vertex_array: u32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_bind_vertex_array(
        vertex_array,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_blit_framebuffer(
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
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_blit_framebuffer(
        src_x0,
        src_y0,
        src_x1,
        src_y1,
        dst_x0,
        dst_y0,
        dst_x1,
        dst_y1,
        mask,
        filter,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_clear_bufferfi(
    buffer: u32,
    drawbuffer: i32,
    depth: f32,
    stencil: i32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_clear_bufferfi(
        buffer,
        drawbuffer,
        depth,
        stencil,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_clear_bufferfv(
    buffer: u32,
    drawbuffer: i32,
    values: *const f32,
    size: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let values = unsafe { std::slice::from_raw_parts(values, size) };
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_clear_bufferfv(
        buffer,
        drawbuffer,
        values,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_clear_bufferiv(
    buffer: u32,
    drawbuffer: i32,
    values: *const i32,
    size: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let values = unsafe { std::slice::from_raw_parts(values, size) };
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_clear_bufferiv(
        buffer,
        drawbuffer,
        values,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_clear_bufferuiv(
    buffer: u32,
    drawbuffer: i32,
    values: *const u32,
    size: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let values = unsafe { std::slice::from_raw_parts(values, size) };
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_clear_bufferuiv(
        buffer,
        drawbuffer,
        values,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_client_wait_sync(
    sync: *const WebGLSync,
    flags: u32,
    timeout: isize,
    state: *mut WebGLState,
) -> u32 {
    assert!(!state.is_null());
    assert!(!sync.is_null());
    let state = unsafe { &mut *state };
    let sync = unsafe { &*sync };
    canvas_webgl::webgl2::canvas_native_webgl2_client_wait_sync(
        &sync.0,
        flags,
        timeout as c_ulong,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_compressed_tex_sub_image3d_none(
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
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_compressed_tex_sub_image3d_none(
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
        offset,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_compressed_tex_sub_image3d(
    target: u32,
    level: i32,
    xoffset: i32,
    yoffset: i32,
    zoffset: i32,
    width: i32,
    height: i32,
    depth: i32,
    format: u32,
    src: *const u8,
    size: usize,
    src_offset: usize,
    src_length_override: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let src = unsafe { std::slice::from_raw_parts(src, size) };
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_compressed_tex_sub_image3d(
        target,
        level,
        xoffset,
        yoffset,
        zoffset,
        width,
        height,
        depth,
        format,
        src,
        src_offset,
        src_length_override,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_copy_buffer_sub_data(
    read_target: u32,
    write_target: u32,
    read_offset: isize,
    write_offset: isize,
    size: isize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_copy_buffer_sub_data(
        read_target,
        write_target,
        read_offset,
        write_offset,
        size,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_copy_tex_sub_image3d(
    target: u32,
    level: i32,
    xoffset: i32,
    yoffset: i32,
    zoffset: i32,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_copy_tex_sub_image3d(
        target,
        level,
        xoffset,
        yoffset,
        zoffset,
        x,
        y,
        width,
        height,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_create_query(state: *mut WebGLState) -> u32 {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_create_query(state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_create_sampler(state: *mut WebGLState) -> u32 {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_create_sampler(state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_create_transform_feedback(state: *mut WebGLState) -> u32 {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_create_transform_feedback(state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_create_vertex_array(state: *mut WebGLState) -> u32 {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_create_vertex_array(state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_delete_query_with_query(id: u32, state: *mut WebGLState) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_delete_query_with_query(id, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_delete_sampler_with_sampler(
    sampler: u32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_delete_sampler_with_sampler(
        sampler,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_delete_sync_with_sync(
    sync: *const WebGLSync,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    assert!(!sync.is_null());
    let state = unsafe { &mut *state };
    let sync = unsafe { &*sync };
    canvas_webgl::webgl2::canvas_native_webgl2_delete_sync_with_sync(&sync.0, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_delete_transform_feedback(
    transform_feedback: u32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_delete_transform_feedback(
        transform_feedback,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_delete_vertex_array_with_vertex_array(
    vertex_array: u32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_delete_vertex_array_with_vertex_array(
        vertex_array,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_draw_arrays_instanced(
    mode: u32,
    first: i32,
    count: i32,
    instance_count: i32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_draw_arrays_instanced(
        mode,
        first,
        count,
        instance_count,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_draw_buffers(
    buffers: *const u32,
    size: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let buffers = unsafe { std::slice::from_raw_parts(buffers, size) };
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_draw_buffers(buffers, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_draw_elements_instanced(
    mode: u32,
    count: i32,
    type_: u32,
    offset: isize,
    instance_count: i32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_draw_elements_instanced(
        mode,
        count,
        type_,
        offset,
        instance_count,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_draw_range_elements(
    mode: u32,
    start: u32,
    end: u32,
    count: i32,
    type_: u32,
    offset: isize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_draw_range_elements(
        mode,
        start,
        end,
        count,
        type_,
        offset,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_end_query(target: u32, state: *mut WebGLState) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_end_query(target, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_end_transform_feedback(state: *mut WebGLState) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_end_transform_feedback(state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_fence_sync(
    condition: u32,
    flags: u32,
    state: *mut WebGLState,
) -> *mut WebGLSync {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    Box::into_raw(Box::new(WebGLSync(
        canvas_webgl::webgl2::canvas_native_webgl2_fence_sync(
            condition,
            flags,
            state.get_inner_mut(),
        ),
    )))
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_framebuffer_texture_layer(
    target: u32,
    attachment: u32,
    texture: u32,
    level: i32,
    layer: i32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_framebuffer_texture_layer(
        target,
        attachment,
        texture,
        level,
        layer,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_get_active_uniform_block_name(
    program: u32,
    uniform_block_index: u32,
    state: *mut WebGLState,
) -> *const c_char {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    CString::new(
        canvas_webgl::webgl2::canvas_native_webgl2_get_active_uniform_block_name(
            program,
            uniform_block_index,
            state.get_inner_mut(),
        ),
    )
        .unwrap()
        .into_raw()
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_get_active_uniform_block_parameter(
    program: u32,
    uniform_block_index: u32,
    pname: u32,
    state: *mut WebGLState,
) -> *mut WebGLResult {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    Box::into_raw(Box::new(WebGLResult(
        canvas_webgl::webgl2::canvas_native_webgl2_get_active_uniform_block_parameter(
            program,
            uniform_block_index,
            pname,
            state.get_inner_mut(),
        ),
    )))
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_get_active_uniforms(
    program: u32,
    uniform_indices: *const u32,
    size: usize,
    pname: u32,
    state: *mut WebGLState,
) -> *mut WebGLResult {
    assert!(!state.is_null());
    let uniform_indices = unsafe { std::slice::from_raw_parts(uniform_indices, size) };
    let state = unsafe { &mut *state };
    Box::into_raw(Box::new(WebGLResult(
        canvas_webgl::webgl2::canvas_native_webgl2_get_active_uniforms(
            program,
            uniform_indices,
            pname,
            state.get_inner_mut(),
        ),
    )))
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_get_buffer_sub_data(
    target: u32,
    src_byte_offset: isize,
    dst_data: *mut u8,
    size: usize,
    dst_offset: usize,
    length: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    let dst_data = unsafe { std::slice::from_raw_parts_mut(dst_data, size) };
    canvas_webgl::webgl2::canvas_native_webgl2_get_buffer_sub_data(
        target,
        src_byte_offset,
        dst_data,
        dst_offset,
        length,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_get_frag_data_location(
    program: u32,
    name: *const c_char,
    state: *mut WebGLState,
) -> i32 {
    assert!(!state.is_null());
    let name = unsafe { CStr::from_ptr(name) };
    let name = name.to_string_lossy();
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_get_frag_data_location(
        program,
        name.as_ref(),
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_get_indexed_parameter(
    target: u32,
    index: u32,
    state: *mut WebGLState,
) -> *mut WebGLIndexedParameter {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    Box::into_raw(Box::new(WebGLIndexedParameter(
        canvas_webgl::webgl2::canvas_native_webgl2_get_indexed_parameter(
            target,
            index,
            state.get_inner_mut(),
        ),
    )))
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_get_internalformat_parameter(
    target: u32,
    internalformat: u32,
    pname: u32,
    state: *mut WebGLState,
) -> *mut WebGLResult {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    Box::into_raw(Box::new(WebGLResult(
        canvas_webgl::webgl2::canvas_native_webgl2_get_internalformat_parameter(
            target,
            internalformat,
            pname,
            state.get_inner_mut(),
        ),
    )))
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_get_parameter(
    pname: u32,
    state: *mut WebGLState,
) -> *mut WebGLResult {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    Box::into_raw(Box::new(WebGLResult(
        canvas_webgl::webgl2::canvas_native_webgl2_get_parameter(pname, state.get_inner_mut()),
    )))
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_get_query_parameter(
    query: u32,
    pname: u32,
    state: *mut WebGLState,
) -> *mut WebGLResult {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    Box::into_raw(Box::new(WebGLResult(
        canvas_webgl::webgl2::canvas_native_webgl2_get_query_parameter(
            query,
            pname,
            state.get_inner_mut(),
        ),
    )))
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_get_query(
    target: u32,
    pname: u32,
    state: *mut WebGLState,
) -> *mut WebGLResult {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    Box::into_raw(Box::new(WebGLResult(
        canvas_webgl::webgl2::canvas_native_webgl2_get_query(target, pname, state.get_inner_mut()),
    )))
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_get_sampler_parameter(
    sampler: u32,
    pname: u32,
    state: *mut WebGLState,
) -> *mut WebGLResult {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    Box::into_raw(Box::new(WebGLResult(
        canvas_webgl::webgl2::canvas_native_webgl2_get_sampler_parameter(
            sampler,
            pname,
            state.get_inner_mut(),
        ),
    )))
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_get_sync_parameter(
    sync: *const WebGLSync,
    pname: u32,
    state: *mut WebGLState,
) -> *mut WebGLResult {
    assert!(!sync.is_null());
    assert!(!state.is_null());
    let sync = unsafe { &*sync };
    let state = unsafe { &mut *state };
    Box::into_raw(Box::new(WebGLResult(
        canvas_webgl::webgl2::canvas_native_webgl2_get_sync_parameter(
            &sync.0,
            pname,
            state.get_inner_mut(),
        ),
    )))
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_get_transform_feedback_varying(
    program: u32,
    index: u32,
    state: *mut WebGLState,
) -> *mut WebGLActiveInfo {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    Box::into_raw(Box::new(WebGLActiveInfo(
        canvas_webgl::webgl2::canvas_native_webgl2_get_transform_feedback_varying(
            program,
            index,
            state.get_inner_mut(),
        ),
    )))
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_get_uniform_block_index(
    program: u32,
    uniform_block_name: *const c_char,
    state: *mut WebGLState,
) -> u32 {
    assert!(!state.is_null());
    let uniform_block_name = unsafe { CStr::from_ptr(uniform_block_name) };
    let uniform_block_name = uniform_block_name.to_string_lossy();
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_get_uniform_block_index(
        program,
        uniform_block_name.as_ref(),
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_get_uniform_indices(
    program: u32,
    uniform_names: *const *const c_char,
    size: usize,
    state: *mut WebGLState,
) -> *mut U32Buffer {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    Box::into_raw(Box::new(U32Buffer::from(
        canvas_webgl::webgl2::canvas_native_webgl2_get_uniform_indices_raw(
            program,
            uniform_names,
            size,
            state.get_inner_mut(),
        ),
    )))
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_invalidate_framebuffer(
    target: u32,
    attachments: *const u32,
    size: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let attachments = unsafe { std::slice::from_raw_parts(attachments, size) };
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_invalidate_framebuffer(
        target,
        attachments,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_invalidate_sub_framebuffer(
    target: u32,
    attachments: *const u32,
    size: usize,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let attachments = unsafe { std::slice::from_raw_parts(attachments, size) };
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_invalidate_sub_framebuffer(
        target,
        attachments,
        x,
        y,
        width,
        height,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_is_query(query: u32, state: *mut WebGLState) -> bool {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_is_query(query, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_is_sampler(sampler: u32, state: *mut WebGLState) -> bool {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_is_sampler(sampler, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_is_sync(
    sync: *const WebGLSync,
    state: *mut WebGLState,
) -> bool {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    let sync = unsafe { &*sync };
    canvas_webgl::webgl2::canvas_native_webgl2_is_sync(&sync.0, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_is_transform_feedback(
    transform_feedback: u32,
    state: *mut WebGLState,
) -> bool {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_is_transform_feedback(
        transform_feedback,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_is_vertex_array(
    vertex_array: u32,
    state: *mut WebGLState,
) -> bool {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_is_vertex_array(vertex_array, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_pause_transform_feedback(state: *mut WebGLState) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_pause_transform_feedback(state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_read_buffer(src: u32, state: *mut WebGLState) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_read_buffer(src, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_renderbuffer_storage_multisample(
    target: u32,
    samples: i32,
    internal_format: u32,
    width: i32,
    height: i32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_renderbuffer_storage_multisample(
        target,
        samples,
        internal_format,
        width,
        height,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_resume_transform_feedback(state: *mut WebGLState) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_resume_transform_feedback(state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_sampler_parameterf(
    sampler: u32,
    pname: u32,
    param: f32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_sampler_parameterf(
        sampler,
        pname,
        param,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_sampler_parameteri(
    sampler: u32,
    pname: u32,
    param: i32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_sampler_parameteri(
        sampler,
        pname,
        param,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_tex_image3d_none(
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
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_tex_image3d_none(
        target,
        level,
        internalformat,
        width,
        height,
        depth,
        border,
        format,
        type_,
        offset,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_tex_image3d_asset(
    target: u32,
    level: i32,
    internalformat: i32,
    width: i32,
    height: i32,
    depth: i32,
    border: i32,
    format: u32,
    type_: u32,
    asset: &ImageAsset,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_tex_image3d_asset(
        target,
        level,
        internalformat,
        width,
        height,
        depth,
        border,
        format,
        type_,
        &asset.0,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_tex_image3d_canvas2d(
    target: u32,
    level: i32,
    internalformat: i32,
    _width: i32,
    _height: i32,
    depth: i32,
    border: i32,
    format: u32,
    type_: u32,
    canvas: *mut CanvasRenderingContext2D,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    let canvas = unsafe { &mut *canvas };

    canvas.make_current();
    let (width, height) = (canvas.context.width(), canvas.context.height());
    let snapshot = canvas.context.as_data();

    // todo handle pre-multipied
    // let premultiply = state.get_inner().get_premultiplied_alpha();

    // let buf = source_ctx.read_pixels_with_alpha_premultiply(&snapshot, format as i32, premultiply);

    state.0.make_current();

    canvas_webgl::webgl2::canvas_native_webgl2_tex_image3d(
        target,
        level,
        internalformat,
        width as i32,
        height as i32,
        depth,
        border,
        format,
        type_,
        snapshot.as_slice(),
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_tex_image3d(
    target: u32,
    level: i32,
    internalformat: i32,
    width: i32,
    height: i32,
    depth: i32,
    border: i32,
    format: u32,
    type_: u32,
    buf: *const u8,
    size: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let buf = unsafe { std::slice::from_raw_parts(buf, size) };
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_tex_image3d(
        target,
        level,
        internalformat,
        width,
        height,
        depth,
        border,
        format,
        type_,
        buf,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_tex_image3d_offset(
    target: u32,
    level: i32,
    internalformat: i32,
    width: i32,
    height: i32,
    depth: i32,
    border: i32,
    format: u32,
    type_: u32,
    buf: *const u8,
    size: usize,
    offset: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let buf = unsafe { std::slice::from_raw_parts(buf, size) };
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_tex_image3d_offset(
        target,
        level,
        internalformat,
        width,
        height,
        depth,
        border,
        format,
        type_,
        buf,
        offset,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_tex_storage2d(
    target: u32,
    levels: i32,
    internalformat: u32,
    width: i32,
    height: i32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_tex_storage2d(
        target,
        levels,
        internalformat,
        width,
        height,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_tex_storage3d(
    target: u32,
    levels: i32,
    internalformat: u32,
    width: i32,
    height: i32,
    depth: i32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_tex_storage3d(
        target,
        levels,
        internalformat,
        width,
        height,
        depth,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_tex_sub_image3d_none(
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
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_tex_sub_image3d_none(
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
        offset,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_tex_sub_image3d(
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
    buf: *const u8,
    size: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let buf = unsafe { std::slice::from_raw_parts(buf, size) };
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_tex_sub_image3d(
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
        buf,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_tex_sub_image3d_asset(
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
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_tex_sub_image3d_asset(
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
        &asset.0,
        state.get_inner_mut(),
    );
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_tex_sub_image3d_canvas2d(
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
    canvas: *mut CanvasRenderingContext2D,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    let canvas = unsafe { &mut *canvas };
    canvas.make_current();
    let (width, height) = (canvas.context.width(), canvas.context.height());
    let data = canvas.context.as_data();

    //  let premultiply = state.get_inner().get_premultiplied_alpha();

    // let buf = source_ctx.read_pixels_with_alpha_premultiply(&snapshot, format as i32, premultiply);

    state.0.make_current();

    canvas_webgl::webgl2::canvas_native_webgl2_tex_sub_image3d(
        target,
        level,
        xoffset,
        yoffset,
        zoffset,
        width as i32,
        height as i32,
        depth,
        format,
        type_,
        data.as_slice(),
        state.get_inner_mut(),
    );
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_tex_sub_image3d_offset(
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
    buf: *const u8,
    size: usize,
    offset: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let buf = unsafe { std::slice::from_raw_parts(buf, size) };
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_tex_sub_image3d_offset(
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
        buf,
        offset,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_transform_feedback_varyings(
    program: u32,
    varyings: *const *const c_char,
    size: usize,
    buffer_mode: u32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_transform_feedback_varyings_raw(
        program,
        varyings,
        size,
        buffer_mode,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_uniform1ui(location: i32, v0: u32, state: *mut WebGLState) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_uniform1ui(location, v0, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_uniform1uiv(
    location: i32,
    data: *const u32,
    size: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let data = unsafe { std::slice::from_raw_parts(data, size) };
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_uniform1uiv(location, data, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_uniform2ui(
    location: i32,
    v0: u32,
    v1: u32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_uniform2ui(location, v0, v1, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_uniform2uiv(
    location: i32,
    data: *const u32,
    size: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let data = unsafe { std::slice::from_raw_parts(data, size) };
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_uniform2uiv(location, data, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_uniform3ui(
    location: i32,
    v0: u32,
    v1: u32,
    v2: u32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_uniform3ui(
        location,
        v0,
        v1,
        v2,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_uniform3uiv(
    location: i32,
    data: *const u32,
    size: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let data = unsafe { std::slice::from_raw_parts(data, size) };
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_uniform3uiv(location, data, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_uniform4ui(
    location: i32,
    v0: u32,
    v1: u32,
    v2: u32,
    v3: u32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_uniform4ui(
        location,
        v0,
        v1,
        v2,
        v3,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_uniform4uiv(
    location: i32,
    data: *const u32,
    size: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let data = unsafe { std::slice::from_raw_parts(data, size) };
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_uniform4uiv(location, data, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_uniform_block_binding(
    program: u32,
    uniform_block_index: u32,
    uniform_block_binding: u32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_uniform_block_binding(
        program,
        uniform_block_index,
        uniform_block_binding,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_uniform_matrix2x3fv(
    location: i32,
    transpose: bool,
    data: *const f32,
    size: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let data = unsafe { std::slice::from_raw_parts(data, size) };
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_uniform_matrix2x3fv(
        location,
        transpose,
        data,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_uniform_matrix2x4fv(
    location: i32,
    transpose: bool,
    data: *const f32,
    size: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let data = unsafe { std::slice::from_raw_parts(data, size) };
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_uniform_matrix2x4fv(
        location,
        transpose,
        data,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_uniform_matrix3x2fv(
    location: i32,
    transpose: bool,
    data: *const f32,
    size: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let data = unsafe { std::slice::from_raw_parts(data, size) };
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_uniform_matrix3x2fv(
        location,
        transpose,
        data,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_uniform_matrix3x4fv(
    location: i32,
    transpose: bool,
    data: *const f32,
    size: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let data = unsafe { std::slice::from_raw_parts(data, size) };
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_uniform_matrix3x4fv(
        location,
        transpose,
        data,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_uniform_matrix4x2fv(
    location: i32,
    transpose: bool,
    data: *const f32,
    size: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let data = unsafe { std::slice::from_raw_parts(data, size) };
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_uniform_matrix4x2fv(
        location,
        transpose,
        data,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_uniform_matrix4x3fv(
    location: i32,
    transpose: bool,
    data: *const f32,
    size: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let data = unsafe { std::slice::from_raw_parts(data, size) };
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_uniform_matrix4x3fv(
        location,
        transpose,
        data,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_vertex_attrib_divisor(
    index: u32,
    divisor: u32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_vertex_attrib_divisor(
        index,
        divisor,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_vertex_attrib_i4i(
    index: u32,
    x: i32,
    y: i32,
    z: i32,
    w: i32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_vertex_attrib_i4i(
        index,
        x,
        y,
        z,
        w,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_vertex_attrib_i4iv(
    index: u32,
    value: *const i32,
    size: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let value = unsafe { std::slice::from_raw_parts(value, size) };
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_vertex_attrib_i4iv(
        index,
        value,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_vertex_attrib_i4ui(
    index: u32,
    x: u32,
    y: u32,
    z: u32,
    w: u32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_vertex_attrib_i4ui(
        index,
        x,
        y,
        z,
        w,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_vertex_attrib_i4uiv(
    index: u32,
    value: *const u32,
    size: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let value = unsafe { std::slice::from_raw_parts(value, size) };
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_vertex_attrib_i4uiv(
        index,
        value,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_tex_image2d_image_asset(
    target: i32,
    level: i32,
    internalformat: i32,
    width: u32,
    height: u32,
    border: i32,
    format: i32,
    type_: i32,
    image_asset: *const ImageAsset,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    assert!(!image_asset.is_null());
    let image_asset = unsafe { &*image_asset };
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_tex_image2d_asset(
        target,
        level,
        internalformat,
        width,
        height,
        border,
        format,
        type_,
        &image_asset.0,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_tex_image2d_src_data_offset(
    target: i32,
    level: i32,
    internalformat: i32,
    width: u32,
    height: u32,
    border: i32,
    format: i32,
    type_: i32,
    src_data: *const u8,
    src_data_size: usize,
    offset: u64,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_tex_image2d_src_data_offset(
        target,
        level,
        internalformat,
        width,
        height,
        border,
        format,
        type_,
        src_data,
        src_data_size,
        offset,
        state.get_inner_mut(),
    )
}


#[no_mangle]
pub extern "C" fn canvas_native_webgl2_tex_image2d_offset(
    target: i32,
    level: i32,
    internalformat: i32,
    width: u32,
    height: u32,
    border: i32,
    format: i32,
    type_: i32,
    offset: u64,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_tex_image2d_offset(
        target,
        level,
        internalformat,
        width,
        height,
        border,
        format,
        type_,
        offset,
        state.get_inner_mut(),
    )
}


#[no_mangle]
pub extern "C" fn canvas_native_webgl2_tex_image2d_webgl(
    target: i32,
    level: i32,
    internalformat: i32,
    width: u32,
    height: u32,
    border: i32,
    format: i32,
    type_: i32,
    webgl: *mut WebGLState,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    assert!(!webgl.is_null());

    let state = unsafe { &mut *state };
    let webgl = unsafe { &mut *webgl };
    let mut pixels = canvas_webgl::webgl::canvas_native_webgl_read_webgl_pixels(
        &mut webgl.0,
        &mut state.0,
        internalformat,
        format,
    );


    state.0.make_current();

    canvas_webgl::webgl::canvas_native_webgl_tex_image2d(
        target,
        level,
        internalformat,
        width as i32,
        height as i32,
        border,
        format,
        type_,
        pixels.2.as_mut_slice(),
        state.get_inner_mut(),
    );
}



#[no_mangle]
pub extern "C" fn canvas_native_webgl2_tex_image2d_canvas2d(
    target: i32,
    level: i32,
    internalformat: i32,
    width: u32,
    height: u32,
    border: i32,
    format: i32,
    type_: i32,
    canvas: *mut CanvasRenderingContext2D,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    assert!(!canvas.is_null());

    let canvas = unsafe { &mut *canvas };
    let state = unsafe { &mut *state };

    canvas.make_current();

    let (source_width, source_height) = canvas.context.dimensions();

    let mut bytes = vec![0u8; (source_width * source_height * 4.) as usize];

    canvas.context.get_pixels(bytes.as_mut_slice(), (0, 0), (source_width as i32, source_height as i32));

    state.0.make_current();

    if format as u32 == gl_bindings::RGBA {
        unsafe {
            if state.0.get_flip_y() {
                let mut buffer = bytes.to_vec();
                utils::gl::flip_in_place(
                    buffer.as_mut_ptr(),
                    buffer.len(),
                    (bytes_per_pixel(type_ as u32, format as u32) * source_width as u32) as usize,
                    source_height as usize,
                );

                gl_bindings::TexImage2D(
                    target as u32,
                    level,
                    internalformat,
                    width as i32,
                    height as i32,
                    border,
                    format as u32,
                    type_ as u32,
                    buffer.as_ptr() as *const c_void,
                );

                return;
            }

            gl_bindings::TexImage2D(
                target as u32,
                level,
                internalformat,
                source_width as i32,
                source_height as i32,
                border,
                format as u32,
                type_ as u32,
                bytes.as_ptr() as *const c_void,
            );
        }
        return;
    }

    let integers = match format as u32 {
        gl_bindings::RGBA_INTEGER => Some(canvas_core::image_asset::ImageAsset::rgba_to_rgba_integer(bytes.as_slice(), source_width as usize, source_height as usize)),
        gl_bindings::RGB_INTEGER => Some(canvas_core::image_asset::ImageAsset::rgba_to_rgb_integer(bytes.as_slice(), source_width as usize, source_height as usize)),
        gl_bindings::RED_INTEGER =>Some(canvas_core::image_asset::ImageAsset::rgba_to_red_integer(bytes.as_slice(), source_width as usize, source_height as usize)),
        gl_bindings::RG_INTEGER =>Some(canvas_core::image_asset::ImageAsset::rgba_to_rg_integer(bytes.as_slice(), source_width as usize, source_height as usize)),
        _ => None,
    };

    if let Some(mut integers) = integers {
        unsafe {
            if state.0.get_flip_y() {
                utils::gl::flip_in_place_integer(
                    integers.as_mut_ptr(),
                    integers.len(),
                    (bytes_per_pixel(type_ as u32, format as u32) * source_width as u32) as usize,
                    source_height as usize,
                );

                gl_bindings::TexImage2D(
                    target as u32,
                    level,
                    internalformat,
                    width as i32,
                    height as i32,
                    border,
                    format as u32,
                    type_ as u32,
                    integers.as_ptr() as *const c_void,
                );

                return;
            }

            gl_bindings::TexImage2D(
                target as u32,
                level,
                internalformat,
                width as i32,
                height as i32,
                border,
                format as u32,
                type_ as u32,
                integers.as_ptr() as *const c_void,
            );
            return;
        }
    }

    let bytes = match format as u32 {
        gl_bindings::RGB => Some(canvas_core::image_asset::ImageAsset::rgba_to_rgb(bytes.as_slice(), source_width as usize, source_height as usize)),
        gl_bindings::LUMINANCE => Some(canvas_core::image_asset::ImageAsset::rgba_to_luminance(bytes.as_slice(), source_width as usize, source_height as usize)),
        gl_bindings::LUMINANCE_ALPHA => Some(canvas_core::image_asset::ImageAsset::rgba_to_luminance_alpha(bytes.as_slice(), source_width as usize, source_height as usize)),
        gl_bindings::ALPHA => Some(canvas_core::image_asset::ImageAsset::rgba_to_alpha(bytes.as_slice(), source_width as usize, source_height as usize)),
        gl_bindings::RED => Some(canvas_core::image_asset::ImageAsset::rgba_to_red(bytes.as_slice(), source_width as usize, source_height as usize)),
        gl_bindings::RG => Some(canvas_core::image_asset::ImageAsset::rgba_to_rg(bytes.as_slice(), source_width as usize, source_height as usize)),
        _ => None,
    };

    if let Some(mut bytes) = bytes {
        unsafe {
            if state.0.get_flip_y() {
                let mut buffer = bytes;
                utils::gl::flip_in_place(
                    buffer.as_mut_ptr(),
                    buffer.len(),
                    (bytes_per_pixel(type_ as u32, format as u32) * source_width as u32) as usize,
                    source_height as usize,
                );

                gl_bindings::TexImage2D(
                    target as u32,
                    level,
                    internalformat,
                    width as i32,
                    height as i32,
                    border,
                    format as u32,
                    type_ as u32,
                    buffer.as_ptr() as *const c_void,
                );

                return;
            }

            gl_bindings::TexImage2D(
                target as u32,
                level,
                internalformat,
                width as i32,
                height as i32,
                border,
                format as u32,
                type_ as u32,
                bytes.as_ptr() as *const c_void,
            );
            return;
        }
    }
}


#[no_mangle]
pub extern "C" fn canvas_native_webgl2_tex_image2d_image_data(
    target: i32,
    level: i32,
    internalformat: i32,
    width: u32,
    height: u32,
    border: i32,
    format: i32,
    type_: i32,
    image_data: *const ImageData,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    assert!(!image_data.is_null());

    let image_data = unsafe { &*image_data };
    let state = unsafe { &mut *state };

    let (source_width, source_height) = image_data.0.dimensions();

    state.0.make_current();

    let bytes = image_data.0.data();

    if format as u32 == gl_bindings::RGBA {
        unsafe {
            if state.0.get_flip_y() {
                let mut buffer = bytes.to_vec();
                utils::gl::flip_in_place(
                    buffer.as_mut_ptr(),
                    buffer.len(),
                    (bytes_per_pixel(type_ as u32, format as u32) * source_width as u32) as usize,
                    source_height as usize,
                );

                gl_bindings::TexImage2D(
                    target as u32,
                    level,
                    internalformat,
                    width as i32,
                    height as i32,
                    border,
                    format as u32,
                    type_ as u32,
                    buffer.as_ptr() as *const c_void,
                );

                return;
            }

            gl_bindings::TexImage2D(
                target as u32,
                level,
                internalformat,
                source_width,
                source_height,
                border,
                format as u32,
                type_ as u32,
                bytes.as_ptr() as *const c_void,
            );
        }
        return;
    }

    let integers = match format as u32 {
        gl_bindings::RGBA_INTEGER => Some(canvas_core::image_asset::ImageAsset::rgba_to_rgba_integer(bytes, source_width as usize, source_height as usize)),
        gl_bindings::RGB_INTEGER => Some(canvas_core::image_asset::ImageAsset::rgba_to_rgb_integer(bytes, source_width as usize, source_height as usize)),
        gl_bindings::RED_INTEGER =>Some(canvas_core::image_asset::ImageAsset::rgba_to_red_integer(bytes, source_width as usize, source_height as usize)),
        gl_bindings::RG_INTEGER =>Some(canvas_core::image_asset::ImageAsset::rgba_to_rg_integer(bytes, source_width as usize, source_height as usize)),
        _ => None,
    };

    if let Some(mut integers) = integers {
        unsafe {
            if state.0.get_flip_y() {
                utils::gl::flip_in_place_integer(
                    integers.as_mut_ptr(),
                    integers.len(),
                    (bytes_per_pixel(type_ as u32, format as u32) * source_width as u32) as usize,
                    source_height as usize,
                );

                gl_bindings::TexImage2D(
                    target as u32,
                    level,
                    internalformat,
                    width as i32,
                    height as i32,
                    border,
                    format as u32,
                    type_ as u32,
                    integers.as_ptr() as *const c_void,
                );

                return;
            }

            gl_bindings::TexImage2D(
                target as u32,
                level,
                internalformat,
                width as i32,
                height as i32,
                border,
                format as u32,
                type_ as u32,
                integers.as_ptr() as *const c_void,
            );
            return;
        }
    }

    let bytes = match format as u32 {
        gl_bindings::RGB => Some(canvas_core::image_asset::ImageAsset::rgba_to_rgb(bytes, source_width as usize, source_height as usize)),
        gl_bindings::LUMINANCE => Some(canvas_core::image_asset::ImageAsset::rgba_to_luminance(bytes, source_width as usize, source_height as usize)),
        gl_bindings::LUMINANCE_ALPHA => Some(canvas_core::image_asset::ImageAsset::rgba_to_luminance_alpha(bytes, source_width as usize, source_height as usize)),
        gl_bindings::ALPHA => Some(canvas_core::image_asset::ImageAsset::rgba_to_alpha(bytes, source_width as usize, source_height as usize)),
        gl_bindings::RED => Some(canvas_core::image_asset::ImageAsset::rgba_to_red(bytes, source_width as usize, source_height as usize)),
        gl_bindings::RG => Some(canvas_core::image_asset::ImageAsset::rgba_to_rg(bytes, source_width as usize, source_height as usize)),
        _ => None,
    };

    if let Some(mut bytes) = bytes {
        unsafe {
            if state.0.get_flip_y() {
                let mut buffer = bytes;
                utils::gl::flip_in_place(
                    buffer.as_mut_ptr(),
                    buffer.len(),
                    (bytes_per_pixel(type_ as u32, format as u32) * source_width as u32) as usize,
                    source_height as usize,
                );

                gl_bindings::TexImage2D(
                    target as u32,
                    level,
                    internalformat,
                    width as i32,
                    height as i32,
                    border,
                    format as u32,
                    type_ as u32,
                    buffer.as_ptr() as *const c_void,
                );

                return;
            }

            gl_bindings::TexImage2D(
                target as u32,
                level,
                internalformat,
                width as i32,
                height as i32,
                border,
                format as u32,
                type_ as u32,
                bytes.as_ptr() as *const c_void,
            );
            return;
        }
    }
}
