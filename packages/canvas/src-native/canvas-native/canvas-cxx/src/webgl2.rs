#![allow(non_snake_case)]

use std::os::raw::c_ulong;

use crate::canvas2d::ImageAsset;
use crate::webgl::{WebGLActiveInfo, WebGLResult, WebGLState};

#[cxx::bridge]
pub(crate) mod ffi {

    extern "C++" {
        include!("canvas-cxx/src/lib.rs.h");
        type ImageAsset = crate::canvas2d::ImageAsset;
        type WebGLState = crate::webgl::WebGLState;
        type WebGLActiveInfo = crate::webgl::WebGLActiveInfo;
        type WebGLResult = crate::webgl::WebGLResult;
        type ContextAttributes = crate::webgl::ContextAttributes;
    }

    extern "Rust" {
        type WebGLSync;
        type WebGLIndexedParameter;

        /* WebGL2RenderingContext */

        fn canvas_native_webgl2_begin_query(target: u32, id: u32, state: &mut WebGLState);

        fn canvas_native_webgl2_begin_transform_feedback(
            primitive_mode: u32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl2_bind_buffer_base(
            target: u32,
            index: u32,
            buffer: u32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl2_bind_buffer_range(
            target: u32,
            index: u32,
            buffer: u32,
            offset: isize,
            size: isize,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl2_bind_sampler(unit: u32, sampler: u32, state: &mut WebGLState);

        fn canvas_native_webgl2_bind_transform_feedback(
            target: u32,
            transform_feedback: u32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl2_bind_vertex_array(vertex_array: u32, state: &mut WebGLState);

        fn canvas_native_webgl2_blit_framebuffer(
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
        );

        fn canvas_native_webgl2_clear_bufferfi(
            buffer: u32,
            drawbuffer: i32,
            depth: f32,
            stencil: i32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl2_clear_bufferfv(
            buffer: u32,
            drawbuffer: i32,
            values: &[f32],
            state: &mut WebGLState,
        );

        fn canvas_native_webgl2_clear_bufferiv(
            buffer: u32,
            drawbuffer: i32,
            values: &[i32],
            state: &mut WebGLState,
        );

        fn canvas_native_webgl2_clear_bufferuiv(
            buffer: u32,
            drawbuffer: i32,
            values: &[u32],
            state: &mut WebGLState,
        );

        fn canvas_native_webgl2_client_wait_sync(
            sync: &WebGLSync,
            flags: u32,
            timeout: isize,
            state: &mut WebGLState,
        ) -> u32;

        fn canvas_native_webgl2_compressed_tex_sub_image3d_none(
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
        );

        fn canvas_native_webgl2_compressed_tex_sub_image3d(
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
        );

        fn canvas_native_webgl2_copy_buffer_sub_data(
            read_target: u32,
            write_target: u32,
            read_offset: isize,
            write_offset: isize,
            size: isize,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl2_copy_tex_sub_image3d(
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
        );

        fn canvas_native_webgl2_create_query(state: &mut WebGLState) -> u32;

        fn canvas_native_webgl2_create_sampler(state: &mut WebGLState) -> u32;

        fn canvas_native_webgl2_create_transform_feedback(state: &mut WebGLState) -> u32;

        fn canvas_native_webgl2_create_vertex_array(state: &mut WebGLState) -> u32;

        fn canvas_native_webgl2_delete_query_with_query(id: u32, state: &mut WebGLState);

        fn canvas_native_webgl2_delete_sampler_with_sampler(sampler: u32, state: &mut WebGLState);

        fn canvas_native_webgl2_delete_sync_with_sync(sync: &WebGLSync, state: &mut WebGLState);

        fn canvas_native_webgl2_delete_transform_feedback(
            transform_feedback: u32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl2_delete_vertex_array_with_vertex_array(
            vertex_array: u32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl2_draw_arrays_instanced(
            mode: u32,
            first: i32,
            count: i32,
            instance_count: i32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl2_draw_buffers(buffers: &[u32], state: &mut WebGLState);

        fn canvas_native_webgl2_draw_elements_instanced(
            mode: u32,
            count: i32,
            type_: u32,
            offset: isize,
            instance_count: i32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl2_draw_range_elements(
            mode: u32,
            start: u32,
            end: u32,
            count: i32,
            type_: u32,
            offset: isize,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl2_end_query(target: u32, state: &mut WebGLState);

        fn canvas_native_webgl2_end_transform_feedback(state: &mut WebGLState);

        fn canvas_native_webgl2_fence_sync(
            condition: u32,
            flags: u32,
            state: &mut WebGLState,
        ) -> Box<WebGLSync>;

        fn canvas_native_webgl2_framebuffer_texture_layer(
            target: u32,
            attachment: u32,
            texture: u32,
            level: i32,
            layer: i32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl2_get_active_uniform_block_name(
            program: u32,
            uniform_block_index: u32,
            state: &mut WebGLState,
        ) -> String;

        fn canvas_native_webgl2_get_active_uniform_block_parameter(
            program: u32,
            uniform_block_index: u32,
            pname: u32,
            state: &mut WebGLState,
        ) -> Box<WebGLResult>;

        fn canvas_native_webgl2_get_active_uniforms(
            program: u32,
            uniform_indices: &[u32],
            pname: u32,
            state: &mut WebGLState,
        ) -> Box<WebGLResult>;

        fn canvas_native_webgl2_get_buffer_sub_data(
            target: u32,
            src_byte_offset: isize,
            dst_data: &mut [u8],
            dst_offset: usize,
            length: usize,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl2_get_frag_data_location(
            program: u32,
            name: &str,
            state: &mut WebGLState,
        ) -> i32;

        fn canvas_native_webgl2_get_indexed_parameter(
            target: u32,
            index: u32,
            state: &mut WebGLState,
        ) -> Box<WebGLIndexedParameter>;

        fn canvas_native_webgl2_get_internalformat_parameter(
            target: u32,
            internalformat: u32,
            pname: u32,
            state: &mut WebGLState,
        ) -> Box<WebGLResult>;

        fn canvas_native_webgl2_get_parameter(
            pname: u32,
            state: &mut WebGLState,
        ) -> Box<WebGLResult>;

        fn canvas_native_webgl2_get_query_parameter(
            query: u32,
            pname: u32,
            state: &mut WebGLState,
        ) -> Box<WebGLResult>;

        fn canvas_native_webgl2_get_query(
            target: u32,
            pname: u32,
            state: &mut WebGLState,
        ) -> Box<WebGLResult>;

        fn canvas_native_webgl2_get_sampler_parameter(
            sampler: u32,
            pname: u32,
            state: &mut WebGLState,
        ) -> Box<WebGLResult>;

        fn canvas_native_webgl2_get_sync_parameter(
            sync: &WebGLSync,
            pname: u32,
            state: &mut WebGLState,
        ) -> Box<WebGLResult>;

        fn canvas_native_webgl2_get_transform_feedback_varying(
            program: u32,
            index: u32,
            state: &mut WebGLState,
        ) -> Box<WebGLActiveInfo>;

        fn canvas_native_webgl2_get_uniform_block_index(
            program: u32,
            uniform_block_name: &str,
            state: &mut WebGLState,
        ) -> u32;

        fn canvas_native_webgl2_get_uniform_indices(
            program: u32,
            uniform_names: &[&str],
            state: &mut WebGLState,
        ) -> Vec<u32>;

        fn canvas_native_webgl2_invalidate_framebuffer(
            target: u32,
            attachments: &[u32],
            state: &mut WebGLState,
        );

        fn canvas_native_webgl2_invalidate_sub_framebuffer(
            target: u32,
            attachments: &[u32],
            x: i32,
            y: i32,
            width: i32,
            height: i32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl2_is_query(query: u32, state: &mut WebGLState) -> bool;

        fn canvas_native_webgl2_is_sampler(sampler: u32, state: &mut WebGLState) -> bool;

        fn canvas_native_webgl2_is_sync(sync: &WebGLSync, state: &mut WebGLState) -> bool;

        fn canvas_native_webgl2_is_transform_feedback(
            transform_feedback: u32,
            state: &mut WebGLState,
        ) -> bool;

        fn canvas_native_webgl2_is_vertex_array(vertex_array: u32, state: &mut WebGLState) -> bool;

        fn canvas_native_webgl2_pause_transform_feedback(state: &mut WebGLState);

        fn canvas_native_webgl2_read_buffer(src: u32, state: &mut WebGLState);

        fn canvas_native_webgl2_renderbuffer_storage_multisample(
            target: u32,
            samples: i32,
            internal_format: u32,
            width: i32,
            height: i32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl2_resume_transform_feedback(state: &mut WebGLState);

        fn canvas_native_webgl2_sampler_parameterf(
            sampler: u32,
            pname: u32,
            param: f32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl2_sampler_parameteri(
            sampler: u32,
            pname: u32,
            param: i32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl2_tex_image3d_none(
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
        );

        fn canvas_native_webgl2_tex_image3d_asset(
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
            state: &mut WebGLState,
        );

        fn canvas_native_webgl2_tex_image3d(
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
        );

        fn canvas_native_webgl2_tex_image3d_offset(
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
        );

        fn canvas_native_webgl2_tex_storage2d(
            target: u32,
            levels: i32,
            internalformat: u32,
            width: i32,
            height: i32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl2_tex_storage3d(
            target: u32,
            levels: i32,
            internalformat: u32,
            width: i32,
            height: i32,
            depth: i32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl2_tex_sub_image3d_none(
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
        );

        fn canvas_native_webgl2_tex_sub_image3d(
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
        );

        fn canvas_native_webgl2_tex_sub_image3d_asset(
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
        );

        fn canvas_native_webgl2_tex_sub_image3d_offset(
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
        );

        fn canvas_native_webgl2_transform_feedback_varyings(
            program: u32,
            varyings: &[&str],
            buffer_mode: u32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl2_uniform1ui(location: i32, v0: u32, state: &mut WebGLState);

        fn canvas_native_webgl2_uniform1uiv(location: i32, data: &[u32], state: &mut WebGLState);

        fn canvas_native_webgl2_uniform2ui(location: i32, v0: u32, v1: u32, state: &mut WebGLState);

        fn canvas_native_webgl2_uniform2uiv(location: i32, data: &[u32], state: &mut WebGLState);

        fn canvas_native_webgl2_uniform3ui(
            location: i32,
            v0: u32,
            v1: u32,
            v2: u32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl2_uniform3uiv(location: i32, data: &[u32], state: &mut WebGLState);

        fn canvas_native_webgl2_uniform4ui(
            location: i32,
            v0: u32,
            v1: u32,
            v2: u32,
            v3: u32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl2_uniform4uiv(location: i32, data: &[u32], state: &mut WebGLState);

        fn canvas_native_webgl2_uniform_block_binding(
            program: u32,
            uniform_block_index: u32,
            uniform_block_binding: u32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl2_uniform_matrix2x3fv(
            location: i32,
            transpose: bool,
            data: &[f32],
            state: &mut WebGLState,
        );

        fn canvas_native_webgl2_uniform_matrix2x4fv(
            location: i32,
            transpose: bool,
            data: &[f32],
            state: &mut WebGLState,
        );

        fn canvas_native_webgl2_uniform_matrix3x2fv(
            location: i32,
            transpose: bool,
            data: &[f32],
            state: &mut WebGLState,
        );

        fn canvas_native_webgl2_uniform_matrix3x4fv(
            location: i32,
            transpose: bool,
            data: &[f32],
            state: &mut WebGLState,
        );

        fn canvas_native_webgl2_uniform_matrix4x2fv(
            location: i32,
            transpose: bool,
            data: &[f32],
            state: &mut WebGLState,
        );

        fn canvas_native_webgl2_uniform_matrix4x3fv(
            location: i32,
            transpose: bool,
            data: &[f32],
            state: &mut WebGLState,
        );

        fn canvas_native_webgl2_vertex_attrib_divisor(
            index: u32,
            divisor: u32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl2_vertex_attrib_i4i(
            index: u32,
            x: i32,
            y: i32,
            z: i32,
            w: i32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl2_vertex_attrib_i4iv(
            index: u32,
            value: &[i32],
            state: &mut WebGLState,
        );

        fn canvas_native_webgl2_vertex_attrib_i4ui(
            index: u32,
            x: u32,
            y: u32,
            z: u32,
            w: u32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl2_vertex_attrib_i4uiv(
            index: u32,
            value: &[u32],
            state: &mut WebGLState,
        );

        /* WebGL2RenderingContext */
    }
}

/* WebGL2 */

pub struct WebGLSync(canvas_webgl::webgl2::GLSync);

pub struct WebGLIndexedParameter(canvas_webgl::prelude::WebGLIndexedParameter);

/* WebGLIndexedParameter */
pub fn canvas_native_webgl2_indexed_parameter_get_value(param: &WebGLIndexedParameter) -> isize {
    param.0.get_value() as isize
}

pub fn canvas_native_webgl2_indexed_parameter_get_buffer_value(
    param: &WebGLIndexedParameter,
) -> isize {
    param.0.get_buffer_value() as isize
}

pub fn canvas_native_webgl2_indexed_parameter_get_is_buffer(param: &WebGLIndexedParameter) -> bool {
    param.0.get_is_buffer()
}
/* WebGLIndexedParameter */

pub fn canvas_native_webgl2_begin_query(target: u32, id: u32, state: &mut WebGLState) {
    canvas_webgl::webgl2::canvas_native_webgl2_begin_query(target, id, state.get_inner_mut())
}

pub fn canvas_native_webgl2_begin_transform_feedback(primitive_mode: u32, state: &mut WebGLState) {
    canvas_webgl::webgl2::canvas_native_webgl2_begin_transform_feedback(
        primitive_mode,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl2_bind_buffer_base(
    target: u32,
    index: u32,
    buffer: u32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl2::canvas_native_webgl2_bind_buffer_base(
        target,
        index,
        buffer,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl2_bind_buffer_range(
    target: u32,
    index: u32,
    buffer: u32,
    offset: isize,
    size: isize,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl2::canvas_native_webgl2_bind_buffer_range(
        target,
        index,
        buffer,
        offset,
        size,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl2_bind_sampler(unit: u32, sampler: u32, state: &mut WebGLState) {
    canvas_webgl::webgl2::canvas_native_webgl2_bind_sampler(unit, sampler, state.get_inner_mut())
}

pub fn canvas_native_webgl2_bind_transform_feedback(
    target: u32,
    transform_feedback: u32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl2::canvas_native_webgl2_bind_sampler(
        target,
        transform_feedback,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl2_bind_vertex_array(vertex_array: u32, state: &mut WebGLState) {
    canvas_webgl::webgl2::canvas_native_webgl2_bind_vertex_array(
        vertex_array,
        state.get_inner_mut(),
    )
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

pub fn canvas_native_webgl2_clear_bufferfi(
    buffer: u32,
    drawbuffer: i32,
    depth: f32,
    stencil: i32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl2::canvas_native_webgl2_clear_bufferfi(
        buffer,
        drawbuffer,
        depth,
        stencil,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl2_clear_bufferfv(
    buffer: u32,
    drawbuffer: i32,
    values: &[f32],
    state: &mut WebGLState,
) {
    canvas_webgl::webgl2::canvas_native_webgl2_clear_bufferfv(
        buffer,
        drawbuffer,
        values,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl2_clear_bufferiv(
    buffer: u32,
    drawbuffer: i32,
    values: &[i32],
    state: &mut WebGLState,
) {
    canvas_webgl::webgl2::canvas_native_webgl2_clear_bufferiv(
        buffer,
        drawbuffer,
        values,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl2_clear_bufferuiv(
    buffer: u32,
    drawbuffer: i32,
    values: &[u32],
    state: &mut WebGLState,
) {
    canvas_webgl::webgl2::canvas_native_webgl2_clear_bufferuiv(
        buffer,
        drawbuffer,
        values,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl2_client_wait_sync(
    sync: &WebGLSync,
    flags: u32,
    timeout: isize,
    state: &mut WebGLState,
) -> u32 {
    canvas_webgl::webgl2::canvas_native_webgl2_client_wait_sync(
        &sync.0,
        flags,
        timeout as c_ulong,
        state.get_inner_mut(),
    )
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

pub fn canvas_native_webgl2_copy_buffer_sub_data(
    read_target: u32,
    write_target: u32,
    read_offset: isize,
    write_offset: isize,
    size: isize,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl2::canvas_native_webgl2_copy_buffer_sub_data(
        read_target,
        write_target,
        read_offset,
        write_offset,
        size,
        state.get_inner_mut(),
    )
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

pub fn canvas_native_webgl2_create_query(state: &mut WebGLState) -> u32 {
    canvas_webgl::webgl2::canvas_native_webgl2_create_query(state.get_inner_mut())
}

pub fn canvas_native_webgl2_create_sampler(state: &mut WebGLState) -> u32 {
    canvas_webgl::webgl2::canvas_native_webgl2_create_sampler(state.get_inner_mut())
}

pub fn canvas_native_webgl2_create_transform_feedback(state: &mut WebGLState) -> u32 {
    canvas_webgl::webgl2::canvas_native_webgl2_create_transform_feedback(state.get_inner_mut())
}

pub fn canvas_native_webgl2_create_vertex_array(state: &mut WebGLState) -> u32 {
    canvas_webgl::webgl2::canvas_native_webgl2_create_vertex_array(state.get_inner_mut())
}

pub fn canvas_native_webgl2_delete_query_with_query(id: u32, state: &mut WebGLState) {
    canvas_webgl::webgl2::canvas_native_webgl2_delete_query_with_query(id, state.get_inner_mut())
}

pub fn canvas_native_webgl2_delete_sampler_with_sampler(sampler: u32, state: &mut WebGLState) {
    canvas_webgl::webgl2::canvas_native_webgl2_delete_sampler_with_sampler(
        sampler,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl2_delete_sync_with_sync(sync: &WebGLSync, state: &mut WebGLState) {
    canvas_webgl::webgl2::canvas_native_webgl2_delete_sync_with_sync(&sync.0, state.get_inner_mut())
}

pub fn canvas_native_webgl2_delete_transform_feedback(
    transform_feedback: u32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl2::canvas_native_webgl2_delete_transform_feedback(
        transform_feedback,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl2_delete_vertex_array_with_vertex_array(
    vertex_array: u32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl2::canvas_native_webgl2_delete_vertex_array_with_vertex_array(
        vertex_array,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl2_draw_arrays_instanced(
    mode: u32,
    first: i32,
    count: i32,
    instance_count: i32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl2::canvas_native_webgl2_draw_arrays_instanced(
        mode,
        first,
        count,
        instance_count,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl2_draw_buffers(buffers: &[u32], state: &mut WebGLState) {
    canvas_webgl::webgl2::canvas_native_webgl2_draw_buffers(buffers, state.get_inner_mut())
}

pub fn canvas_native_webgl2_draw_elements_instanced(
    mode: u32,
    count: i32,
    type_: u32,
    offset: isize,
    instance_count: i32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl2::canvas_native_webgl2_draw_elements_instanced(
        mode,
        count,
        type_,
        offset,
        instance_count,
        state.get_inner_mut(),
    )
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

pub fn canvas_native_webgl2_end_query(target: u32, state: &mut WebGLState) {
    canvas_webgl::webgl2::canvas_native_webgl2_end_query(target, state.get_inner_mut())
}

pub fn canvas_native_webgl2_end_transform_feedback(state: &mut WebGLState) {
    canvas_webgl::webgl2::canvas_native_webgl2_end_transform_feedback(state.get_inner_mut())
}

pub fn canvas_native_webgl2_fence_sync(
    condition: u32,
    flags: u32,
    state: &mut WebGLState,
) -> Box<WebGLSync> {
    Box::new(WebGLSync(
        canvas_webgl::webgl2::canvas_native_webgl2_fence_sync(
            condition,
            flags,
            state.get_inner_mut(),
        ),
    ))
}

pub fn canvas_native_webgl2_framebuffer_texture_layer(
    target: u32,
    attachment: u32,
    texture: u32,
    level: i32,
    layer: i32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl2::canvas_native_webgl2_framebuffer_texture_layer(
        target,
        attachment,
        texture,
        level,
        layer,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl2_get_active_uniform_block_name(
    program: u32,
    uniform_block_index: u32,
    state: &mut WebGLState,
) -> String {
    canvas_webgl::webgl2::canvas_native_webgl2_get_active_uniform_block_name(
        program,
        uniform_block_index,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl2_get_active_uniform_block_parameter(
    program: u32,
    uniform_block_index: u32,
    pname: u32,
    state: &mut WebGLState,
) -> Box<WebGLResult> {
    Box::new(WebGLResult(
        canvas_webgl::webgl2::canvas_native_webgl2_get_active_uniform_block_parameter(
            program,
            uniform_block_index,
            pname,
            state.get_inner_mut(),
        ),
    ))
}

pub fn canvas_native_webgl2_get_active_uniforms(
    program: u32,
    uniform_indices: &[u32],
    pname: u32,
    state: &mut WebGLState,
) -> Box<WebGLResult> {
    Box::new(WebGLResult(
        canvas_webgl::webgl2::canvas_native_webgl2_get_active_uniforms(
            program,
            uniform_indices,
            pname,
            state.get_inner_mut(),
        ),
    ))
}

pub fn canvas_native_webgl2_get_buffer_sub_data(
    target: u32,
    src_byte_offset: isize,
    dst_data: &mut [u8],
    dst_offset: usize,
    length: usize,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl2::canvas_native_webgl2_get_buffer_sub_data(
        target,
        src_byte_offset,
        dst_data,
        dst_offset,
        length,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl2_get_frag_data_location(
    program: u32,
    name: &str,
    state: &mut WebGLState,
) -> i32 {
    canvas_webgl::webgl2::canvas_native_webgl2_get_frag_data_location(
        program,
        name,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl2_get_indexed_parameter(
    target: u32,
    index: u32,
    state: &mut WebGLState,
) -> Box<WebGLIndexedParameter> {
    Box::new(WebGLIndexedParameter(
        canvas_webgl::webgl2::canvas_native_webgl2_get_indexed_parameter(
            target,
            index,
            state.get_inner_mut(),
        ),
    ))
}

pub fn canvas_native_webgl2_get_internalformat_parameter(
    target: u32,
    internalformat: u32,
    pname: u32,
    state: &mut WebGLState,
) -> Box<WebGLResult> {
    Box::new(WebGLResult(
        canvas_webgl::webgl2::canvas_native_webgl2_get_internalformat_parameter(
            target,
            internalformat,
            pname,
            state.get_inner_mut(),
        ),
    ))
}

pub fn canvas_native_webgl2_get_parameter(pname: u32, state: &mut WebGLState) -> Box<WebGLResult> {
    Box::new(WebGLResult(
        canvas_webgl::webgl2::canvas_native_webgl2_get_parameter(pname, state.get_inner_mut()),
    ))
}

pub fn canvas_native_webgl2_get_query_parameter(
    query: u32,
    pname: u32,
    state: &mut WebGLState,
) -> Box<WebGLResult> {
    Box::new(WebGLResult(
        canvas_webgl::webgl2::canvas_native_webgl2_get_query_parameter(
            query,
            pname,
            state.get_inner_mut(),
        ),
    ))
}

pub fn canvas_native_webgl2_get_query(
    target: u32,
    pname: u32,
    state: &mut WebGLState,
) -> Box<WebGLResult> {
    Box::new(WebGLResult(
        canvas_webgl::webgl2::canvas_native_webgl2_get_query(target, pname, state.get_inner_mut()),
    ))
}

pub fn canvas_native_webgl2_get_sampler_parameter(
    sampler: u32,
    pname: u32,
    state: &mut WebGLState,
) -> Box<WebGLResult> {
    Box::new(WebGLResult(
        canvas_webgl::webgl2::canvas_native_webgl2_get_sampler_parameter(
            sampler,
            pname,
            state.get_inner_mut(),
        ),
    ))
}

pub fn canvas_native_webgl2_get_sync_parameter(
    sync: &WebGLSync,
    pname: u32,
    state: &mut WebGLState,
) -> Box<WebGLResult> {
    Box::new(WebGLResult(
        canvas_webgl::webgl2::canvas_native_webgl2_get_sync_parameter(
            &sync.0,
            pname,
            state.get_inner_mut(),
        ),
    ))
}

pub fn canvas_native_webgl2_get_transform_feedback_varying(
    program: u32,
    index: u32,
    state: &mut WebGLState,
) -> Box<WebGLActiveInfo> {
    Box::new(WebGLActiveInfo(
        canvas_webgl::webgl2::canvas_native_webgl2_get_transform_feedback_varying(
            program,
            index,
            state.get_inner_mut(),
        ),
    ))
}

pub fn canvas_native_webgl2_get_uniform_block_index(
    program: u32,
    uniform_block_name: &str,
    state: &mut WebGLState,
) -> u32 {
    canvas_webgl::webgl2::canvas_native_webgl2_get_uniform_block_index(
        program,
        uniform_block_name,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl2_get_uniform_indices(
    program: u32,
    uniform_names: &[&str],
    state: &mut WebGLState,
) -> Vec<u32> {
    let uniform_names: Vec<String> = uniform_names.iter().map(|i| i.to_string()).collect();
    canvas_webgl::webgl2::canvas_native_webgl2_get_uniform_indices(
        program,
        uniform_names.as_slice(),
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl2_invalidate_framebuffer(
    target: u32,
    attachments: &[u32],
    state: &mut WebGLState,
) {
    canvas_webgl::webgl2::canvas_native_webgl2_invalidate_framebuffer(
        target,
        attachments,
        state.get_inner_mut(),
    )
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

pub fn canvas_native_webgl2_is_query(query: u32, state: &mut WebGLState) -> bool {
    canvas_webgl::webgl2::canvas_native_webgl2_is_query(query, state.get_inner_mut())
}

pub fn canvas_native_webgl2_is_sampler(sampler: u32, state: &mut WebGLState) -> bool {
    canvas_webgl::webgl2::canvas_native_webgl2_is_sampler(sampler, state.get_inner_mut())
}

pub fn canvas_native_webgl2_is_sync(sync: &WebGLSync, state: &mut WebGLState) -> bool {
    canvas_webgl::webgl2::canvas_native_webgl2_is_sync(&sync.0, state.get_inner_mut())
}

pub fn canvas_native_webgl2_is_transform_feedback(
    transform_feedback: u32,
    state: &mut WebGLState,
) -> bool {
    canvas_webgl::webgl2::canvas_native_webgl2_is_transform_feedback(
        transform_feedback,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl2_is_vertex_array(vertex_array: u32, state: &mut WebGLState) -> bool {
    canvas_webgl::webgl2::canvas_native_webgl2_is_vertex_array(vertex_array, state.get_inner_mut())
}

pub fn canvas_native_webgl2_pause_transform_feedback(state: &mut WebGLState) {
    canvas_webgl::webgl2::canvas_native_webgl2_pause_transform_feedback(state.get_inner_mut())
}

pub fn canvas_native_webgl2_read_buffer(src: u32, state: &mut WebGLState) {
    canvas_webgl::webgl2::canvas_native_webgl2_read_buffer(src, state.get_inner_mut())
}

pub fn canvas_native_webgl2_renderbuffer_storage_multisample(
    target: u32,
    samples: i32,
    internal_format: u32,
    width: i32,
    height: i32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl2::canvas_native_webgl2_renderbuffer_storage_multisample(
        target,
        samples,
        internal_format,
        width,
        height,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl2_resume_transform_feedback(state: &mut WebGLState) {
    canvas_webgl::webgl2::canvas_native_webgl2_resume_transform_feedback(state.get_inner_mut())
}

pub fn canvas_native_webgl2_sampler_parameterf(
    sampler: u32,
    pname: u32,
    param: f32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl2::canvas_native_webgl2_sampler_parameterf(
        sampler,
        pname,
        param,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl2_sampler_parameteri(
    sampler: u32,
    pname: u32,
    param: i32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl2::canvas_native_webgl2_sampler_parameteri(
        sampler,
        pname,
        param,
        state.get_inner_mut(),
    )
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

pub fn canvas_native_webgl2_tex_image3d_asset(
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
    state: &mut WebGLState,
) {
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

pub fn canvas_native_webgl2_tex_storage2d(
    target: u32,
    levels: i32,
    internalformat: u32,
    width: i32,
    height: i32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl2::canvas_native_webgl2_tex_storage2d(
        target,
        levels,
        internalformat,
        width,
        height,
        state.get_inner_mut(),
    )
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

pub fn canvas_native_webgl2_transform_feedback_varyings(
    program: u32,
    varyings: &[&str],
    buffer_mode: u32,
    state: &mut WebGLState,
) {
    let varyings: Vec<String> = varyings.iter().map(|i| i.to_string()).collect();
    canvas_webgl::webgl2::canvas_native_webgl2_transform_feedback_varyings(
        program,
        varyings.as_slice(),
        buffer_mode,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl2_uniform1ui(location: i32, v0: u32, state: &mut WebGLState) {
    canvas_webgl::webgl2::canvas_native_webgl2_uniform1ui(location, v0, state.get_inner_mut())
}

pub fn canvas_native_webgl2_uniform1uiv(location: i32, data: &[u32], state: &mut WebGLState) {
    canvas_webgl::webgl2::canvas_native_webgl2_uniform1uiv(location, data, state.get_inner_mut())
}

pub fn canvas_native_webgl2_uniform2ui(location: i32, v0: u32, v1: u32, state: &mut WebGLState) {
    canvas_webgl::webgl2::canvas_native_webgl2_uniform2ui(location, v0, v1, state.get_inner_mut())
}

pub fn canvas_native_webgl2_uniform2uiv(location: i32, data: &[u32], state: &mut WebGLState) {
    canvas_webgl::webgl2::canvas_native_webgl2_uniform2uiv(location, data, state.get_inner_mut())
}

pub fn canvas_native_webgl2_uniform3ui(
    location: i32,
    v0: u32,
    v1: u32,
    v2: u32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl2::canvas_native_webgl2_uniform3ui(
        location,
        v0,
        v1,
        v2,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl2_uniform3uiv(location: i32, data: &[u32], state: &mut WebGLState) {
    canvas_webgl::webgl2::canvas_native_webgl2_uniform3uiv(location, data, state.get_inner_mut())
}

pub fn canvas_native_webgl2_uniform4ui(
    location: i32,
    v0: u32,
    v1: u32,
    v2: u32,
    v3: u32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl2::canvas_native_webgl2_uniform4ui(
        location,
        v0,
        v1,
        v2,
        v3,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl2_uniform4uiv(location: i32, data: &[u32], state: &mut WebGLState) {
    canvas_webgl::webgl2::canvas_native_webgl2_uniform4uiv(location, data, state.get_inner_mut())
}

pub fn canvas_native_webgl2_uniform_block_binding(
    program: u32,
    uniform_block_index: u32,
    uniform_block_binding: u32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl2::canvas_native_webgl2_uniform_block_binding(
        program,
        uniform_block_index,
        uniform_block_binding,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl2_uniform_matrix2x3fv(
    location: i32,
    transpose: bool,
    data: &[f32],
    state: &mut WebGLState,
) {
    canvas_webgl::webgl2::canvas_native_webgl2_uniform_matrix2x3fv(
        location,
        transpose,
        data,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl2_uniform_matrix2x4fv(
    location: i32,
    transpose: bool,
    data: &[f32],
    state: &mut WebGLState,
) {
    canvas_webgl::webgl2::canvas_native_webgl2_uniform_matrix2x4fv(
        location,
        transpose,
        data,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl2_uniform_matrix3x2fv(
    location: i32,
    transpose: bool,
    data: &[f32],
    state: &mut WebGLState,
) {
    canvas_webgl::webgl2::canvas_native_webgl2_uniform_matrix3x2fv(
        location,
        transpose,
        data,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl2_uniform_matrix3x4fv(
    location: i32,
    transpose: bool,
    data: &[f32],
    state: &mut WebGLState,
) {
    canvas_webgl::webgl2::canvas_native_webgl2_uniform_matrix3x4fv(
        location,
        transpose,
        data,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl2_uniform_matrix4x2fv(
    location: i32,
    transpose: bool,
    data: &[f32],
    state: &mut WebGLState,
) {
    canvas_webgl::webgl2::canvas_native_webgl2_uniform_matrix4x2fv(
        location,
        transpose,
        data,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl2_uniform_matrix4x3fv(
    location: i32,
    transpose: bool,
    data: &[f32],
    state: &mut WebGLState,
) {
    canvas_webgl::webgl2::canvas_native_webgl2_uniform_matrix4x3fv(
        location,
        transpose,
        data,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl2_vertex_attrib_divisor(
    index: u32,
    divisor: u32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl2::canvas_native_webgl2_vertex_attrib_divisor(
        index,
        divisor,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl2_vertex_attrib_i4i(
    index: u32,
    x: i32,
    y: i32,
    z: i32,
    w: i32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl2::canvas_native_webgl2_vertex_attrib_i4i(
        index,
        x,
        y,
        z,
        w,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl2_vertex_attrib_i4iv(index: u32, value: &[i32], state: &mut WebGLState) {
    canvas_webgl::webgl2::canvas_native_webgl2_vertex_attrib_i4iv(
        index,
        value,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl2_vertex_attrib_i4ui(
    index: u32,
    x: u32,
    y: u32,
    z: u32,
    w: u32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl2::canvas_native_webgl2_vertex_attrib_i4ui(
        index,
        x,
        y,
        z,
        w,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl2_vertex_attrib_i4uiv(index: u32, value: &[u32], state: &mut WebGLState) {
    canvas_webgl::webgl2::canvas_native_webgl2_vertex_attrib_i4uiv(
        index,
        value,
        state.get_inner_mut(),
    )
}

/* WebGL2 */
