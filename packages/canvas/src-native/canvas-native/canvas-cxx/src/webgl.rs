#![allow(non_snake_case)]

extern crate core;


use std::any::Any;
use std::borrow::Cow;
use std::collections::HashMap;
use std::error::Error;
use std::ffi::{CStr, CString};
use std::io::Read;
use std::os::raw::c_void;
use std::os::unix::io::FromRawFd;
use std::os::unix::prelude::IntoRawFd;

use canvas_2d::{
    context::{Context, ContextWrapper},
    context::compositing::composite_operation_type::CompositeOperationType,
    context::drawing_paths::fill_rule::FillRule,
    context::fill_and_stroke_styles::paint::paint_style_set_color_with_string,
    context::fill_and_stroke_styles::pattern::Repetition,
    context::image_asset::OutputFormat,
    context::ImageSmoothingQuality,
    context::line_styles::line_cap::LineCap,
    context::line_styles::line_join::LineJoin,
    context::text_styles::text_align::TextAlign,
    context::text_styles::TextDirection,
    utils::color::{parse_color, to_parsed_color},
    utils::image::{
        from_bitmap_slice, from_image_slice, from_image_slice_encoded,
        from_image_slice_encoded_no_copy, from_image_slice_no_copy, to_image, to_image_encoded,
        to_image_encoded_from_data,
    },
};
use canvas_core::gl::GLContext;
use canvas_core::image_asset::OutputFormat;
use canvas_webgl::prelude::{WebGLExtensionType, WebGLVersion};
use once_cell::sync::OnceCell;

use crate::canvas2d::ImageAsset;
use crate::webgl;
use crate::webgl::ffi::WebGLResultType;

#[cxx::bridge]
pub(crate) mod ffi {
    #[allow(non_camel_case_types)]
    #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
    pub enum WebGLExtensionType {
        EXT_blend_minmax,
        EXT_color_buffer_half_float,
        EXT_disjoint_timer_query,
        EXT_sRGB,
        EXT_shader_texture_lod,
        EXT_texture_filter_anisotropic,
        OES_element_index_uint,
        OES_standard_derivatives,
        OES_texture_float,
        OES_texture_float_linear,
        OES_texture_half_float,
        OES_texture_half_float_linear,
        OES_vertex_array_object,
        WEBGL_color_buffer_float,
        WEBGL_compressed_texture_atc,
        WEBGL_compressed_texture_etc1,
        WEBGL_compressed_texture_s3tc,
        WEBGL_compressed_texture_s3tc_srgb,
        WEBGL_compressed_texture_etc,
        WEBGL_compressed_texture_pvrtc,
        WEBGL_lose_context,
        ANGLE_instanced_arrays,
        WEBGL_depth_texture,
        WEBGL_draw_buffers,
        None,
    }

    #[allow(non_camel_case_types)]
    #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
    pub enum WebGLResultType {
        Boolean,
        I32Array,
        U32Array,
        F32Array,
        BooleanArray,
        U32,
        I32,
        F32,
        String,
        None,
    }


    extern "Rust" {
        type CanvasRenderingContext2D = crate::canvas2d::CanvasRenderingContext2D;
        type ImageAsset = crate::canvas2d::ImageAsset;
        type WebGLState;
        type WebGLActiveInfo;
        type WebGLResult;
        type ContextAttributes;
        type WebGLExtension;
        type WebGLFramebufferAttachmentParameter;
        type WebGLShaderPrecisionFormat;
        type EXT_blend_minmax;
        type EXT_color_buffer_half_float;
        type EXT_disjoint_timer_query;
        type EXT_sRGB;
        type EXT_shader_texture_lod;
        type EXT_texture_filter_anisotropic;
        type OES_element_index_uint;
        type OES_standard_derivatives;
        type OES_texture_float;
        type OES_texture_float_linear;
        type OES_texture_half_float;
        type OES_texture_half_float_linear;
        type OES_vertex_array_object;
        type WEBGL_color_buffer_float;
        type WEBGL_compressed_texture_atc;
        type WEBGL_compressed_texture_etc1;
        type WEBGL_compressed_texture_s3tc;
        type WEBGL_compressed_texture_s3tc_srgb;
        type WEBGL_compressed_texture_etc;
        type WEBGL_compressed_texture_pvrtc;
        type WEBGL_lose_context;
        type ANGLE_instanced_arrays;
        type WEBGL_depth_texture;
        type WEBGL_draw_buffers;

        /* GL */
        fn canvas_native_webgl_make_current(state: &WebGLState) -> bool;
        fn canvas_native_webgl_swap_buffers(state: &WebGLState) -> bool;

        fn canvas_native_webgl_to_data_url(state: &WebGLState, format: &str, quality: i32) -> String;

        fn canvas_native_webgl_resized(state: &mut WebGLState);
        /* GL */

        /* WebGLActiveInfo */

        fn canvas_native_webgl_active_info_get_name(info: &WebGLActiveInfo) -> &str;
        fn canvas_native_webgl_active_info_get_size(info: &WebGLActiveInfo) -> i32;
        fn canvas_native_webgl_active_info_get_type(info: &WebGLActiveInfo) -> u32;
        fn canvas_native_webgl_active_info_get_is_empty(info: &WebGLActiveInfo) -> bool;

        /* WebGLActiveInfo */

        /* WebGLShaderPrecisionFormat */
        fn canvas_native_webgl_shader_precision_format_get_range_min(
            shader: &WebGLShaderPrecisionFormat,
        ) -> i32;
        fn canvas_native_webgl_shader_precision_format_get_range_max(
            shader: &WebGLShaderPrecisionFormat,
        ) -> i32;
        fn canvas_native_webgl_shader_precision_format_get_precision(
            shader: &WebGLShaderPrecisionFormat,
        ) -> i32;
        /* WebGLShaderPrecisionFormat */

        /* ContextAttributes */
        fn canvas_native_webgl_context_attribute_get_get_alpha(attr: &ContextAttributes) -> bool;
        fn canvas_native_webgl_context_attribute_get_get_antialias(
            attr: &ContextAttributes,
        ) -> bool;
        fn canvas_native_webgl_context_attribute_get_get_depth(attr: &ContextAttributes) -> bool;
        fn canvas_native_webgl_context_attribute_get_get_fail_if_major_performance_caveat(
            attr: &ContextAttributes,
        ) -> bool;
        fn canvas_native_webgl_context_attribute_get_get_power_preference(
            attr: &ContextAttributes,
        ) -> String;
        fn canvas_native_webgl_context_attribute_get_get_premultiplied_alpha(
            attr: &ContextAttributes,
        ) -> bool;
        fn canvas_native_webgl_context_attribute_get_get_preserve_drawing_buffer(
            attr: &ContextAttributes,
        ) -> bool;
        fn canvas_native_webgl_context_attribute_get_get_stencil(attr: &ContextAttributes) -> bool;
        fn canvas_native_webgl_context_attribute_get_get_desynchronized(
            attr: &ContextAttributes,
        ) -> bool;
        fn canvas_native_webgl_context_attribute_get_get_xr_compatible(
            attr: &ContextAttributes,
        ) -> bool;

        /* ContextAttributes */

        /* WebGLExtension */
        fn canvas_native_webgl_context_extension_is_none(extension: &WebGLExtension) -> bool;
        fn canvas_native_webgl_context_extension_get_type(
            extension: &WebGLExtension,
        ) -> WebGLExtensionType;
        fn canvas_native_webgl_context_extension_to_ext_disjoint_timer_query(
            extension: Box<WebGLExtension>,
        ) -> Box<EXT_disjoint_timer_query>;
        pub fn canvas_native_webgl_context_extension_to_angle_instanced_arrays(
            extension: Box<WebGLExtension>,
        ) -> Box<ANGLE_instanced_arrays>;

        pub fn canvas_native_webgl_context_extension_to_lose_context(
            extension: Box<WebGLExtension>,
        ) -> Box<WEBGL_lose_context>;

        pub fn canvas_native_webgl_context_extension_to_draw_buffers(
            extension: Box<WebGLExtension>,
        ) -> Box<WEBGL_draw_buffers>;

        pub fn canvas_native_webgl_context_extension_to_oes_vertex_array_object(
            extension: Box<WebGLExtension>,
        ) -> Box<OES_vertex_array_object>;

        /* WebGLExtension */

        /* WEBGL_lose_context */

        fn canvas_native_webgl_lose_context_lose_context(context: &WEBGL_lose_context);

        fn canvas_native_webgl_lose_context_restore_context(context: &WEBGL_lose_context);

        /* WEBGL_lose_context */

        /* WEBGL_draw_buffers */

        fn canvas_native_webgl_draw_buffers_draw_buffers_webgl(
            buffers: &[u32],
            context: &WEBGL_draw_buffers,
        );

        /* WEBGL_draw_buffers */

        /* OES_vertex_array_object */

        fn canvas_native_webgl_oes_vertex_array_object_create_vertex_array_oes(
            object: &OES_vertex_array_object,
        ) -> u32;

        fn canvas_native_webgl_oes_vertex_array_object_delete_vertex_array_oes(
            array_object: u32,
            object: &OES_vertex_array_object,
        );

        fn canvas_native_webgl_oes_vertex_array_object_is_vertex_array_oes(
            array_object: u32,
            object: &OES_vertex_array_object,
        ) -> bool;

        fn canvas_native_webgl_oes_vertex_array_object_bind_vertex_array_oes(
            array_object: u32,
            object: &OES_vertex_array_object,
        );

        /* OES_vertex_array_object */

        /* WebGLFramebufferAttachmentParameter */
        fn canvas_native_webgl_framebuffer_attachment_parameter_get_is_texture(
            param: &WebGLFramebufferAttachmentParameter,
        ) -> bool;
        fn canvas_native_webgl_framebuffer_attachment_parameter_get_is_renderbuffer(
            param: &WebGLFramebufferAttachmentParameter,
        ) -> bool;
        fn canvas_native_webgl_framebuffer_attachment_parameter_get_value(
            param: &WebGLFramebufferAttachmentParameter,
        ) -> i32;
        /* WebGLFramebufferAttachmentParameter */

        /* WebGLResult */

        fn canvas_native_webgl_result_get_type(result: &WebGLResult) -> WebGLResultType;
        fn canvas_native_webgl_result_get_bool(result: &WebGLResult) -> bool;
        fn canvas_native_webgl_result_get_i32_array(result: &WebGLResult) -> Vec<i32>;
        fn canvas_native_webgl_result_get_u32_array(result: &WebGLResult) -> Vec<u32>;
        fn canvas_native_webgl_result_get_f32_array(result: &WebGLResult) -> Vec<f32>;
        fn canvas_native_webgl_result_get_bool_array(result: &WebGLResult) -> Vec<u8>;
        fn canvas_native_webgl_result_get_u32(result: &WebGLResult) -> u32;
        fn canvas_native_webgl_result_get_i32(result: &WebGLResult) -> i32;
        fn canvas_native_webgl_result_get_f32(result: &WebGLResult) -> f32;
        fn canvas_native_webgl_result_get_string(result: &WebGLResult) -> String;
        fn canvas_native_webgl_result_get_is_none(result: &WebGLResult) -> bool;

        /* WebGLResult */

        /* WebGLState */
        fn canvas_native_webgl_state_get_unpack_colorspace_conversion_webgl(
            state: &WebGLState,
        ) -> i32;
        fn canvas_native_webgl_state_get_flip_y(state: &WebGLState) -> bool;
        fn canvas_native_webgl_state_get_premultiplied_alpha(state: &WebGLState) -> bool;
        fn canvas_native_webgl_state_get_drawing_buffer_width(state: &WebGLState) -> i32;
        fn canvas_native_webgl_state_get_drawing_buffer_height(state: &WebGLState) -> i32;
        /* WebGLState */

        /* EXT_disjoint_timer_query */
        fn canvas_native_webgl_ext_disjoint_timer_query_create_query_ext(
            query: &EXT_disjoint_timer_query,
        ) -> u32;

        fn canvas_native_webgl_ext_disjoint_timer_query_delete_query_ext(
            value: u32,
            query: &EXT_disjoint_timer_query,
        );

        fn canvas_native_webgl_ext_disjoint_timer_query_is_query_ext(
            value: u32,
            query: &EXT_disjoint_timer_query,
        ) -> bool;

        fn canvas_native_webgl_ext_disjoint_timer_query_begin_query_ext(
            target: u32,
            value: u32,
            query: &EXT_disjoint_timer_query,
        );

        fn canvas_native_webgl_ext_disjoint_timer_query_end_query_ext(
            target: u32,
            query: &EXT_disjoint_timer_query,
        );

        fn canvas_native_webgl_ext_disjoint_timer_query_query_counter_ext(
            value: u32,
            target: u32,
            query: &EXT_disjoint_timer_query,
        );

        fn canvas_native_webgl_ext_disjoint_timer_query_get_query_ext(
            target: u32,
            pname: u32,
            query: &EXT_disjoint_timer_query,
        ) -> i32;

        fn canvas_native_webgl_ext_disjoint_timer_query_get_query_object_ext(
            target: u32,
            pname: u32,
            query: &EXT_disjoint_timer_query,
        ) -> Box<WebGLResult>;

        /* EXT_disjoint_timer_query */

        /* ANGLE_instanced_arrays */
        fn canvas_native_webgl_angle_instanced_arrays_draw_arrays_instanced_angle(
            mode: u32,
            first: i32,
            count: i32,
            primcount: i32,
            arrays: &ANGLE_instanced_arrays,
        );
        fn canvas_native_webgl_angle_instanced_arrays_draw_elements_instanced_angle(
            mode: u32,
            count: i32,
            type_: u32,
            offset: i32,
            primcount: i32,
            arrays: &ANGLE_instanced_arrays,
        );
        fn canvas_native_webgl_angle_instanced_arrays_vertex_attrib_divisor_angle(
            index: u32,
            divisor: u32,
            arrays: &ANGLE_instanced_arrays,
        );
        /* ANGLE_instanced_arrays */

        /* WebGLRenderingContext */

        fn canvas_native_webgl_create(
            version: &str,
            alpha: bool,
            antialias: bool,
            depth: bool,
            fail_if_major_performance_caveat: bool,
            power_preference: &str,
            premultiplied_alpha: bool,
            preserve_drawing_buffer: bool,
            stencil: bool,
            desynchronized: bool,
            xr_compatible: bool,
        ) -> Box<WebGLState>;

        fn canvas_native_webgl_create_no_window(
            width: i32,
            height: i32,
            version: &str,
            alpha: bool,
            antialias: bool,
            depth: bool,
            fail_if_major_performance_caveat: bool,
            power_preference: &str,
            premultiplied_alpha: bool,
            preserve_drawing_buffer: bool,
            stencil: bool,
            desynchronized: bool,
            xr_compatible: bool,
            is_bool: bool,
        ) -> Box<WebGLState>;

        fn canvas_native_webgl_active_texture(texture: u32, state: &mut WebGLState);

        fn canvas_native_webgl_attach_shader(program: u32, shader: u32, state: &mut WebGLState);

        fn canvas_native_webgl_bind_attrib_location(
            program: u32,
            index: u32,
            name: &str,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_bind_buffer(target: u32, buffer: u32, state: &mut WebGLState);

        fn canvas_native_webgl_bind_frame_buffer(
            target: u32,
            framebuffer: u32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_bind_render_buffer(
            target: u32,
            renderbuffer: u32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_bind_texture(target: u32, texture: u32, state: &mut WebGLState);

        fn canvas_native_webgl_blend_color(
            red: f32,
            green: f32,
            blue: f32,
            alpha: f32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_blend_equation_separate(
            mode_rgb: u32,
            mode_alpha: u32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_blend_equation(mode: u32, state: &mut WebGLState);

        fn canvas_native_webgl_blend_func_separate(
            src_rgb: u32,
            dst_rgb: u32,
            src_alpha: u32,
            dst_alpha: u32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_blend_func(sfactor: u32, dfactor: u32, state: &mut WebGLState);

        fn canvas_native_webgl_buffer_data(
            target: u32,
            src_data: &[u8],
            usage: u32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_buffer_data_u16(
            target: u32,
            src_data: &[u16],
            usage: u32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_buffer_data_f32(
            target: u32,
            src_data: &[f32],
            usage: u32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_buffer_data_none(
            target: u32,
            size: isize,
            usage: u32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_buffer_sub_data(
            target: u32,
            offset: isize,
            src_data: &[u8],
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_buffer_sub_data_none(
            target: u32,
            offset: isize,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_check_frame_buffer_status(
            target: u32,
            state: &mut WebGLState,
        ) -> u32;

        fn canvas_native_webgl_clear(mask: u32, state: &mut WebGLState);

        fn canvas_native_webgl_clear_color(
            red: f32,
            green: f32,
            blue: f32,
            alpha: f32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_clear_depth(depth: f32, state: &mut WebGLState);

        fn canvas_native_webgl_clear_stencil(stencil: i32, state: &mut WebGLState);

        fn canvas_native_webgl_color_mask(
            red: bool,
            green: bool,
            blue: bool,
            alpha: bool,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_commit(state: &mut WebGLState);

        fn canvas_native_webgl_compile_shader(shader: u32, state: &mut WebGLState);

        fn canvas_native_webgl_compressed_tex_image2d(
            target: u32,
            level: i32,
            internalformat: u32,
            width: i32,
            height: i32,
            border: i32,
            pixels: &[u8],
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_compressed_tex_image2d_none(
            target: u32,
            level: i32,
            internalformat: u32,
            width: i32,
            height: i32,
            border: i32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_compressed_tex_sub_image2d(
            target: u32,
            level: i32,
            xoffset: i32,
            yoffset: i32,
            width: i32,
            height: i32,
            format: u32,
            pixels: &[u8],
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_copy_tex_image2d(
            target: u32,
            level: i32,
            internalformat: u32,
            x: i32,
            y: i32,
            width: i32,
            height: i32,
            border: i32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_copy_tex_sub_image2d(
            target: u32,
            level: i32,
            xoffset: i32,
            yoffset: i32,
            x: i32,
            y: i32,
            width: i32,
            height: i32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_create_buffer(state: &mut WebGLState) -> u32;

        fn canvas_native_webgl_create_framebuffer(state: &mut WebGLState) -> u32;

        fn canvas_native_webgl_create_program(state: &mut WebGLState) -> u32;

        fn canvas_native_webgl_create_renderbuffer(state: &mut WebGLState) -> u32;

        fn canvas_native_webgl_create_shader(shader_type: u32, state: &mut WebGLState) -> u32;

        fn canvas_native_webgl_create_texture(state: &mut WebGLState) -> u32;

        fn canvas_native_webgl_cull_face(mode: u32, state: &mut WebGLState);

        fn canvas_native_webgl_delete_buffer(buffer: u32, state: &mut WebGLState);

        fn canvas_native_webgl_delete_framebuffer(frame_buffer: u32, state: &mut WebGLState);

        fn canvas_native_webgl_delete_program(program: u32, state: &mut WebGLState);

        fn canvas_native_webgl_delete_renderbuffer(render_buffer: u32, state: &mut WebGLState);

        fn canvas_native_webgl_delete_shader(shader: u32, state: &mut WebGLState);

        fn canvas_native_webgl_delete_texture(texture: u32, state: &mut WebGLState);

        fn canvas_native_webgl_depth_func(func: u32, state: &mut WebGLState);

        fn canvas_native_webgl_depth_mask(flag: bool, state: &mut WebGLState);

        fn canvas_native_webgl_depth_range(z_near: f32, z_far: f32, state: &mut WebGLState);

        fn canvas_native_webgl_detach_shader(program: u32, shader: u32, state: &mut WebGLState);

        fn canvas_native_webgl_disable(cap: u32, state: &mut WebGLState);

        fn canvas_native_webgl_disable_vertex_attrib_array(index: u32, state: &mut WebGLState);

        fn canvas_native_webgl_draw_arrays(
            mode: u32,
            first: i32,
            count: i32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_draw_elements(
            mode: u32,
            count: i32,
            element_type: u32,
            offset: isize,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_enable(cap: u32, state: &mut WebGLState);

        fn canvas_native_webgl_enable_vertex_attrib_array(index: u32, state: &mut WebGLState);

        fn canvas_native_webgl_finish(state: &mut WebGLState);

        fn canvas_native_webgl_flush(state: &mut WebGLState);

        fn canvas_native_webgl_framebuffer_renderbuffer(
            target: u32,
            attachment: u32,
            renderbuffertarget: u32,
            renderbuffer: u32,
            state: &mut WebGLState,
        );

        pub fn canvas_native_webgl_framebuffer_texture2d(
            target: u32,
            attachment: u32,
            textarget: u32,
            texture: u32,
            level: i32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_front_face(mode: u32, state: &mut WebGLState);

        fn canvas_native_webgl_generate_mipmap(target: u32, state: &mut WebGLState);

        fn canvas_native_webgl_get_active_attrib(
            program: u32,
            index: u32,
            state: &mut WebGLState,
        ) -> Box<WebGLActiveInfo>;

        fn canvas_native_webgl_get_active_uniform(
            program: u32,
            index: u32,
            state: &mut WebGLState,
        ) -> Box<WebGLActiveInfo>;

        fn canvas_native_webgl_get_attached_shaders(
            program: u32,
            state: &mut WebGLState,
        ) -> Vec<u32>;

        fn canvas_native_webgl_get_attrib_location(
            program: u32,
            name: &str,
            state: &mut WebGLState,
        ) -> i32;

        fn canvas_native_webgl_get_buffer_parameter(
            target: u32,
            pname: u32,
            state: &mut WebGLState,
        ) -> i32;

        fn canvas_native_webgl_get_context_attributes(state: &WebGLState)
                                                      -> Box<ContextAttributes>;

        fn canvas_native_webgl_get_error(state: &mut WebGLState) -> u32;

        fn canvas_native_webgl_get_extension(
            name: &str,
            state: &mut WebGLState,
        ) -> Box<WebGLExtension>;

        fn canvas_native_webgl_get_framebuffer_attachment_parameter(
            target: u32,
            attachment: u32,
            pname: u32,
            state: &mut WebGLState,
        ) -> Box<WebGLFramebufferAttachmentParameter>;

        fn canvas_native_webgl_get_parameter(
            pname: u32,
            state: &mut WebGLState,
        ) -> Box<WebGLResult>;

        fn canvas_native_webgl_get_program_info_log(program: u32, state: &mut WebGLState)
                                                    -> String;

        fn canvas_native_webgl_get_program_parameter(
            program: u32,
            pname: u32,
            state: &mut WebGLState,
        ) -> Box<WebGLResult>;

        fn canvas_native_webgl_get_renderbuffer_parameter(
            target: u32,
            pname: u32,
            state: &mut WebGLState,
        ) -> i32;

        fn canvas_native_webgl_get_shader_info_log(shader: u32, state: &mut WebGLState) -> String;

        fn canvas_native_webgl_get_shader_parameter(
            shader: u32,
            pname: u32,
            state: &mut WebGLState,
        ) -> Box<WebGLResult>;

        fn canvas_native_webgl_get_shader_precision_format(
            shader_type: u32,
            precision_type: u32,
            state: &mut WebGLState,
        ) -> Box<WebGLShaderPrecisionFormat>;

        fn canvas_native_webgl_get_shader_source(shader: u32, state: &mut WebGLState) -> String;

        fn canvas_native_webgl_get_supported_extensions(state: &mut WebGLState) -> Vec<String>;

        fn canvas_native_webgl_get_supported_extensions_to_string(state: &mut WebGLState)
                                                                  -> String;

        fn canvas_native_webgl_get_tex_parameter(
            target: u32,
            pname: u32,
            state: &mut WebGLState,
        ) -> i32;

        fn canvas_native_webgl_get_uniform_location(
            program: u32,
            name: &str,
            state: &mut WebGLState,
        ) -> i32;

        fn canvas_native_webgl_get_uniform(
            program: u32,
            location: i32,
            state: &mut WebGLState,
        ) -> Box<WebGLResult>;

        fn canvas_native_webgl_get_vertex_attrib_offset(
            index: u32,
            pname: u32,
            state: &mut WebGLState,
        ) -> usize;

        fn canvas_native_webgl_get_vertex_attrib(
            index: u32,
            pname: u32,
            state: &mut WebGLState,
        ) -> Box<WebGLResult>;

        fn canvas_native_webgl_get_is_context_lost(state: &mut WebGLState) -> bool;

        fn canvas_native_webgl_hint(target: u32, mode: u32, state: &mut WebGLState);

        fn canvas_native_webgl_is_buffer(buffer: u32, state: &mut WebGLState) -> bool;

        fn canvas_native_webgl_is_enabled(cap: u32, state: &mut WebGLState) -> bool;

        fn canvas_native_webgl_is_framebuffer(framebuffer: u32, state: &mut WebGLState) -> bool;

        fn canvas_native_webgl_is_program(program: u32, state: &mut WebGLState) -> bool;

        fn canvas_native_webgl_is_renderbuffer(renderbuffer: u32, state: &mut WebGLState) -> bool;

        fn canvas_native_webgl_is_shader(shader: u32, state: &mut WebGLState) -> bool;

        fn canvas_native_webgl_is_texture(texture: u32, state: &mut WebGLState) -> bool;

        fn canvas_native_webgl_line_width(width: f32, state: &mut WebGLState);

        fn canvas_native_webgl_link_program(program: u32, state: &mut WebGLState);

        fn canvas_native_webgl_pixel_storei(pname: u32, param: i32, state: &mut WebGLState);

        fn canvas_native_webgl_polygon_offset(factor: f32, units: f32, state: &mut WebGLState);

        fn canvas_native_webgl_read_pixels_u8(
            x: i32,
            y: i32,
            width: i32,
            height: i32,
            format: u32,
            pixel_type: u32,
            pixels: &mut [u8],
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_read_pixels_u16(
            x: i32,
            y: i32,
            width: i32,
            height: i32,
            format: u32,
            pixel_type: u32,
            pixels: &mut [u16],
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_read_pixels_f32(
            x: i32,
            y: i32,
            width: i32,
            height: i32,
            format: u32,
            pixel_type: u32,
            pixels: &mut [f32],
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_renderbuffer_storage(
            target: u32,
            internal_format: u32,
            width: i32,
            height: i32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_sample_coverage(value: f32, invert: bool, state: &mut WebGLState);

        fn canvas_native_webgl_scissor(
            x: i32,
            y: i32,
            width: i32,
            height: i32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_shader_source(shader: u32, source: &str, state: &mut WebGLState);

        fn canvas_native_webgl_stencil_func(
            func: u32,
            reference: i32,
            mask: u32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_stencil_func_separate(
            face: u32,
            func: u32,
            reference: i32,
            mask: u32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_stencil_mask(mask: u32, state: &mut WebGLState);

        fn canvas_native_webgl_stencil_mask_separate(face: u32, mask: u32, state: &mut WebGLState);

        fn canvas_native_webgl_stencil_op(
            fail: u32,
            zfail: u32,
            zpass: u32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_stencil_op_separate(
            face: u32,
            fail: u32,
            zfail: u32,
            zpass: u32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_tex_image2d_image_none(
            target: i32,
            level: i32,
            internalformat: i32,
            format: i32,
            image_type: i32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_tex_image2d_asset(
            target: i32,
            level: i32,
            internalformat: i32,
            format: i32,
            image_type: i32,
            asset: &ImageAsset,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_tex_image2d_canvas2d(
            target: i32,
            level: i32,
            internalformat: i32,
            format: i32,
            image_type: i32,
            canvas: &mut CanvasRenderingContext2D,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_tex_image2d_webgl(
            target: i32,
            level: i32,
            internalformat: i32,
            format: i32,
            image_type: i32,
            webgl: &mut WebGLState,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_tex_image2d(
            target: i32,
            level: i32,
            internalformat: i32,
            width: i32,
            height: i32,
            border: i32,
            format: i32,
            image_type: i32,
            buf: &mut [u8],
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_tex_image2d_none(
            target: i32,
            level: i32,
            internalformat: i32,
            width: i32,
            height: i32,
            border: i32,
            format: i32,
            image_type: i32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_tex_image2d_image_asset(
            target: i32,
            level: i32,
            internalformat: i32,
            format: i32,
            image_type: i32,
            image_asset: &ImageAsset,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_tex_parameterf(
            target: u32,
            pname: u32,
            param: f32,
            state: &WebGLState,
        );

        fn canvas_native_webgl_tex_parameteri(
            target: u32,
            pname: u32,
            param: i32,
            state: &WebGLState,
        );

        fn canvas_native_webgl_tex_sub_image2d_asset(
            target: u32,
            level: i32,
            xoffset: i32,
            yoffset: i32,
            format: u32,
            image_type: i32,
            asset: &ImageAsset,
            state: &WebGLState,
        );

        fn canvas_native_webgl_tex_sub_image2d(
            target: u32,
            level: i32,
            xoffset: i32,
            yoffset: i32,
            width: i32,
            height: i32,
            format: u32,
            image_type: i32,
            buf: &[u8],
            state: &WebGLState,
        );

        fn canvas_native_webgl_uniform1f(location: i32, v0: f32, state: &mut WebGLState);

        fn canvas_native_webgl_uniform1fv(location: i32, value: &[f32], state: &mut WebGLState);

        fn canvas_native_webgl_uniform1i(location: i32, v0: i32, state: &mut WebGLState);

        fn canvas_native_webgl_uniform1iv(location: i32, value: &[i32], state: &mut WebGLState);

        fn canvas_native_webgl_uniform2f(location: i32, v0: f32, v1: f32, state: &mut WebGLState);

        fn canvas_native_webgl_uniform2fv(location: i32, value: &[f32], state: &mut WebGLState);

        fn canvas_native_webgl_uniform2i(location: i32, v0: i32, v1: i32, state: &mut WebGLState);

        fn canvas_native_webgl_uniform2iv(location: i32, value: &[i32], state: &mut WebGLState);

        fn canvas_native_webgl_uniform3f(
            location: i32,
            v0: f32,
            v1: f32,
            v2: f32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_uniform3fv(location: i32, value: &[f32], state: &mut WebGLState);

        fn canvas_native_webgl_uniform3i(
            location: i32,
            v0: i32,
            v1: i32,
            v2: i32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_uniform3iv(location: i32, value: &[i32], state: &mut WebGLState);

        fn canvas_native_webgl_uniform4f(
            location: i32,
            v0: f32,
            v1: f32,
            v2: f32,
            v3: f32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_uniform4fv(location: i32, value: &[f32], state: &mut WebGLState);

        fn canvas_native_webgl_uniform4i(
            location: i32,
            v0: i32,
            v1: i32,
            v2: i32,
            v3: i32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_uniform4iv(location: i32, value: &[i32], state: &mut WebGLState);

        fn canvas_native_webgl_uniform_matrix2fv(
            location: i32,
            transpose: bool,
            value: &[f32],
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_uniform_matrix3fv(
            location: i32,
            transpose: bool,
            value: &[f32],
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_uniform_matrix4fv(
            location: i32,
            transpose: bool,
            value: &[f32],
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_use_program(program: u32, state: &mut WebGLState);

        fn canvas_native_webgl_validate_program(program: u32, state: &mut WebGLState);

        fn canvas_native_webgl_vertex_attrib1f(index: u32, v0: f32, state: &mut WebGLState);

        fn canvas_native_webgl_vertex_attrib1fv(index: u32, value: &[f32], state: &mut WebGLState);

        fn canvas_native_webgl_vertex_attrib2f(
            index: u32,
            v0: f32,
            v1: f32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_vertex_attrib2fv(index: u32, value: &[f32], state: &mut WebGLState);

        fn canvas_native_webgl_vertex_attrib3f(
            index: u32,
            v0: f32,
            v1: f32,
            v2: f32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_vertex_attrib3fv(index: u32, value: &[f32], state: &mut WebGLState);

        fn canvas_native_webgl_vertex_attrib4f(
            index: u32,
            v0: f32,
            v1: f32,
            v2: f32,
            v3: f32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_vertex_attrib4fv(index: u32, value: &[f32], state: &mut WebGLState);

        fn canvas_native_webgl_vertex_attrib_pointer(
            index: u32,
            size: i32,
            d_type: u32,
            normalized: bool,
            stride: i32,
            offset: isize,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_viewport(
            x: i32,
            y: i32,
            width: i32,
            height: i32,
            state: &mut WebGLState,
        );

        /* WebGLRenderingContext */
    }
}


/* GL */

pub fn canvas_native_webgl_make_current(state: &WebGLState) -> bool {
    state.get_inner().make_current()
}

pub fn canvas_native_webgl_swap_buffers(state: &WebGLState) -> bool {
    state.get_inner().swap_buffers()
}
/* GL */

/* WebGL */

fn canvas_native_webgl_resized(state: &mut WebGLState) {
    // state.get_inner_mut().resized();
}

fn canvas_native_webgl_to_data_url(state: &WebGLState, format: &str, quality: i32) -> String {
    let info = state.get_inner();
    let width = info.drawing_buffer_width();
    let height = info.drawing_buffer_height();
    info.make_current();
    // gl_bindings::glPixelStorei(gl_bindings::GL_UNPACK_ALIGNMENT, 1);
    let mut buffer = vec![0u8; (width * height * 4) as usize];
    unsafe {
        gl_bindings::glReadPixels(
            0,
            0,
            width as i32,
            height as i32,
            gl_bindings::GL_RGBA,
            gl_bindings::GL_UNSIGNED_BYTE,
            buffer.as_mut_ptr() as *mut c_void,
        );
    }

    canvas_core::bytes_to_data_url(width, height, buffer.as_slice(), format, quality)
}

#[derive(Debug)]
pub struct WebGLState(canvas_webgl::prelude::WebGLState);

impl WebGLState {
    pub fn new_with_context(
        context: GLContext,
        version: WebGLVersion,
        alpha: bool,
        antialias: bool,
        depth: bool,
        fail_if_major_performance_caveat: bool,
        power_preference: &str,
        premultiplied_alpha: bool,
        preserve_drawing_buffer: bool,
        stencil: bool,
        desynchronized: bool,
        xr_compatible: bool,
        is_canvas: bool,
    ) -> Self {
        Self(canvas_webgl::prelude::WebGLState::new_with_context_attributes(
            context,
            version,
            alpha,
            antialias,
            depth,
            fail_if_major_performance_caveat,
            power_preference,
            premultiplied_alpha,
            preserve_drawing_buffer,
            stencil,
            desynchronized,
            xr_compatible,
            is_canvas,
        ))
    }
    pub fn get_inner(&self) -> &canvas_webgl::prelude::WebGLState {
        &self.0
    }

    pub fn get_inner_mut(&mut self) -> &mut canvas_webgl::prelude::WebGLState {
        &mut self.0
    }
}

pub struct WebGLActiveInfo(canvas_webgl::prelude::WebGLActiveInfo);

impl WebGLActiveInfo {
    pub fn get_name(&self) -> &str {
        self.0.get_name()
    }

    pub fn get_size(&self) -> i32 {
        self.0.get_size()
    }

    pub fn get_type(&self) -> u32 {
        self.0.get_type()
    }

    pub fn get_is_empty(&self) -> bool {
        self.0.get_is_empty()
    }
}

pub struct ContextAttributes(canvas_webgl::prelude::ContextAttributes);

impl ContextAttributes {
    pub fn get_alpha(&self) -> bool {
        self.0.get_alpha()
    }
    pub fn get_antialias(&self) -> bool {
        self.0.get_antialias()
    }
    pub fn get_depth(&self) -> bool {
        self.0.get_depth()
    }
    pub fn get_fail_if_major_performance_caveat(&self) -> bool {
        self.0.get_fail_if_major_performance_caveat()
    }
    pub fn get_power_preference(&self) -> String {
        self.0.get_power_preference()
    }
    pub fn get_premultiplied_alpha(&self) -> bool {
        self.0.get_premultiplied_alpha()
    }
    pub fn get_preserve_drawing_buffer(&self) -> bool {
        self.0.get_preserve_drawing_buffer()
    }
    pub fn get_stencil(&self) -> bool {
        self.0.get_stencil()
    }
    pub fn get_desynchronized(&self) -> bool {
        self.0.get_desynchronized()
    }
    pub fn get_xr_compatible(&self) -> bool {
        self.0.get_xr_compatible()
    }
}

pub struct WebGLFramebufferAttachmentParameter(canvas_webgl::prelude::WebGLFramebufferAttachmentParameter);

impl WebGLFramebufferAttachmentParameter {
    pub fn get_is_texture(&self) -> bool {
        self.0.get_is_texture()
    }

    pub fn get_is_renderbuffer(&self) -> bool {
        self.0.get_is_renderbuffer()
    }

    pub fn get_value(&self) -> i32 {
        self.0.get_value()
    }
}

pub struct WebGLShaderPrecisionFormat(canvas_webgl::prelude::WebGLShaderPrecisionFormat);

impl WebGLShaderPrecisionFormat {
    pub fn get_precision(&self) -> i32 {
        self.0.get_precision()
    }

    pub fn get_range_min(&self) -> i32 {
        self.0.get_range_min()
    }

    pub fn get_range_max(&self) -> i32 {
        self.0.get_range_max()
    }
}

pub struct WebGLExtension(Option<Box<dyn canvas_webgl::prelude::WebGLExtension>>);

impl WebGLExtension {
    pub fn is_none(&self) -> bool {
        self.0.is_none()
    }

    pub fn extension_type(&self) -> WebGLExtensionType {
        if self.is_none() {
            return WebGLExtensionType::None;
        }
        self.0.as_ref().unwrap().extension_type()
    }

    pub fn into_ext_disjoint_timer_query(self) -> Box<EXT_disjoint_timer_query> {
        let ext = Box::into_raw(self.0.unwrap());
        let ext = unsafe { Box::from_raw(ext as *mut canvas_webgl::prelude::EXT_disjoint_timer_query) };
        Box::new(EXT_disjoint_timer_query(*ext))
    }

    pub fn into_angle_instanced_arrays(self) -> Box<ANGLE_instanced_arrays> {
        let ext = Box::into_raw(self.0.unwrap());
        let ext = unsafe { Box::from_raw(ext as *mut canvas_webgl::prelude::ANGLE_instanced_arrays) };
        Box::new(ANGLE_instanced_arrays(*ext))
    }

    pub fn into_lose_context(self) -> Box<WEBGL_lose_context> {
        let ext = Box::into_raw(self.0.unwrap());
        let ext = unsafe { Box::from_raw(ext as *mut canvas_webgl::prelude::WEBGL_lose_context) };
        Box::new(WEBGL_lose_context(*ext))
    }

    pub fn into_draw_buffers(self) -> Box<WEBGL_draw_buffers> {
        let ext = Box::into_raw(self.0.unwrap());
        let ext = unsafe { Box::from_raw(ext as *mut canvas_webgl::prelude::WEBGL_draw_buffers) };
        Box::new(WEBGL_draw_buffers(*ext))
    }

    pub fn into_oes_vertex_array_object(self) -> Box<OES_vertex_array_object> {
        let ext = Box::into_raw(self.0.unwrap());
        let ext = unsafe { Box::from_raw(ext as *mut canvas_webgl::prelude::OES_vertex_array_object) };
        Box::new(OES_vertex_array_object(*ext))
    }
}

#[allow(non_camel_case_types)]
pub struct EXT_blend_minmax(canvas_webgl::prelude::EXT_blend_minmax);

#[allow(non_camel_case_types)]
pub struct EXT_color_buffer_half_float(canvas_webgl::prelude::EXT_color_buffer_half_float);

#[allow(non_camel_case_types)]
pub struct EXT_disjoint_timer_query(canvas_webgl::prelude::EXT_disjoint_timer_query);

#[allow(non_camel_case_types)]
pub struct EXT_sRGB(canvas_webgl::prelude::EXT_sRGB);

#[allow(non_camel_case_types)]
pub struct EXT_shader_texture_lod(canvas_webgl::prelude::EXT_shader_texture_lod);

#[allow(non_camel_case_types)]
pub struct EXT_texture_filter_anisotropic(canvas_webgl::prelude::EXT_texture_filter_anisotropic);

#[allow(non_camel_case_types)]
pub struct OES_element_index_uint(canvas_webgl::prelude::OES_element_index_uint);

#[allow(non_camel_case_types)]
pub struct OES_standard_derivatives(canvas_webgl::prelude::OES_standard_derivatives);

#[allow(non_camel_case_types)]
pub struct OES_texture_float(canvas_webgl::prelude::OES_texture_float);

#[allow(non_camel_case_types)]
pub struct OES_texture_float_linear(canvas_webgl::prelude::OES_texture_float_linear);

#[allow(non_camel_case_types)]
pub struct OES_texture_half_float(canvas_webgl::prelude::OES_texture_half_float);

#[allow(non_camel_case_types)]
pub struct OES_texture_half_float_linear(canvas_webgl::prelude::OES_texture_half_float_linear);

#[allow(non_camel_case_types)]
pub struct OES_vertex_array_object(canvas_webgl::prelude::OES_vertex_array_object);

#[allow(non_camel_case_types)]
pub struct WEBGL_color_buffer_float(canvas_webgl::prelude::WEBGL_color_buffer_float);

#[allow(non_camel_case_types)]
pub struct WEBGL_compressed_texture_atc(canvas_webgl::prelude::WEBGL_compressed_texture_atc);

#[allow(non_camel_case_types)]
pub struct WEBGL_compressed_texture_etc1(canvas_webgl::prelude::WEBGL_compressed_texture_etc1);

#[allow(non_camel_case_types)]
pub struct WEBGL_compressed_texture_s3tc(canvas_webgl::prelude::WEBGL_compressed_texture_s3tc);

#[allow(non_camel_case_types)]
pub struct WEBGL_compressed_texture_s3tc_srgb(canvas_webgl::prelude::WEBGL_compressed_texture_s3tc_srgb);

#[allow(non_camel_case_types)]
pub struct WEBGL_compressed_texture_etc(canvas_webgl::prelude::WEBGL_compressed_texture_etc);

#[allow(non_camel_case_types)]
pub struct WEBGL_compressed_texture_pvrtc(canvas_webgl::prelude::WEBGL_compressed_texture_pvrtc);

#[allow(non_camel_case_types)]
pub struct WEBGL_lose_context(canvas_webgl::prelude::WEBGL_lose_context);

#[allow(non_camel_case_types)]
pub struct ANGLE_instanced_arrays(canvas_webgl::prelude::ANGLE_instanced_arrays);

#[allow(non_camel_case_types)]
pub struct WEBGL_depth_texture(canvas_webgl::prelude::WEBGL_depth_texture);

#[allow(non_camel_case_types)]
pub struct WEBGL_draw_buffers(canvas_webgl::prelude::WEBGL_draw_buffers);

pub struct WebGLResult(canvas_webgl::prelude::WebGLResult);

/* WebGLActiveInfo */

pub fn canvas_native_webgl_active_info_get_name(info: &WebGLActiveInfo) -> &str {
    info.get_name()
}

pub fn canvas_native_webgl_active_info_get_size(info: &WebGLActiveInfo) -> i32 {
    info.get_size()
}

pub fn canvas_native_webgl_active_info_get_type(info: &WebGLActiveInfo) -> u32 {
    info.get_type()
}

pub fn canvas_native_webgl_active_info_get_is_empty(info: &WebGLActiveInfo) -> bool {
    info.get_is_empty()
}

/* WebGLActiveInfo */

/* WebGLShaderPrecisionFormat */

fn canvas_native_webgl_shader_precision_format_get_range_min(
    shader: &WebGLShaderPrecisionFormat,
) -> i32 {
    shader.get_range_min()
}

fn canvas_native_webgl_shader_precision_format_get_range_max(
    shader: &WebGLShaderPrecisionFormat,
) -> i32 {
    shader.get_range_max()
}

fn canvas_native_webgl_shader_precision_format_get_precision(
    shader: &WebGLShaderPrecisionFormat,
) -> i32 {
    shader.get_precision()
}

/* WebGLShaderPrecisionFormat */

/* ContextAttributes */
pub fn canvas_native_webgl_context_attribute_get_get_alpha(attr: &ContextAttributes) -> bool {
    attr.get_alpha()
}

pub fn canvas_native_webgl_context_attribute_get_get_antialias(attr: &ContextAttributes) -> bool {
    attr.get_antialias()
}

pub fn canvas_native_webgl_context_attribute_get_get_depth(attr: &ContextAttributes) -> bool {
    attr.get_depth()
}

pub fn canvas_native_webgl_context_attribute_get_get_fail_if_major_performance_caveat(
    attr: &ContextAttributes,
) -> bool {
    attr.get_fail_if_major_performance_caveat()
}

pub fn canvas_native_webgl_context_attribute_get_get_power_preference(
    attr: &ContextAttributes,
) -> String {
    attr.get_power_preference()
}

pub fn canvas_native_webgl_context_attribute_get_get_premultiplied_alpha(
    attr: &ContextAttributes,
) -> bool {
    attr.get_premultiplied_alpha()
}

pub fn canvas_native_webgl_context_attribute_get_get_preserve_drawing_buffer(
    attr: &ContextAttributes,
) -> bool {
    attr.get_preserve_drawing_buffer()
}

pub fn canvas_native_webgl_context_attribute_get_get_stencil(attr: &ContextAttributes) -> bool {
    attr.get_stencil()
}

pub fn canvas_native_webgl_context_attribute_get_get_desynchronized(
    attr: &ContextAttributes,
) -> bool {
    attr.get_desynchronized()
}

pub fn canvas_native_webgl_context_attribute_get_get_xr_compatible(
    attr: &ContextAttributes,
) -> bool {
    attr.get_xr_compatible()
}

/* ContextAttributes */

/* WebGLExtension */
pub fn canvas_native_webgl_context_extension_is_none(extension: &WebGLExtension) -> bool {
    extension.is_none()
}

pub fn canvas_native_webgl_context_extension_get_type(
    extension: &WebGLExtension,
) -> WebGLExtensionType {
    extension.extension_type()
}

pub fn canvas_native_webgl_context_extension_to_ext_disjoint_timer_query(
    extension: Box<WebGLExtension>,
) -> Box<EXT_disjoint_timer_query> {
    extension.into_ext_disjoint_timer_query()
}

pub fn canvas_native_webgl_context_extension_to_angle_instanced_arrays(
    extension: Box<WebGLExtension>,
) -> Box<ANGLE_instanced_arrays> {
    extension.into_angle_instanced_arrays()
}

pub fn canvas_native_webgl_context_extension_to_lose_context(
    extension: Box<WebGLExtension>,
) -> Box<WEBGL_lose_context> {
    extension.into_lose_context()
}

pub fn canvas_native_webgl_context_extension_to_draw_buffers(
    extension: Box<WebGLExtension>,
) -> Box<WEBGL_draw_buffers> {
    extension.into_draw_buffers()
}

pub fn canvas_native_webgl_context_extension_to_oes_vertex_array_object(
    extension: Box<WebGLExtension>,
) -> Box<OES_vertex_array_object> {
    extension.into_oes_vertex_array_object()
}

/* WebGLExtension */

/* WebGLResult */
fn canvas_native_webgl_result_get_type(result: &WebGLResult) -> WebGLResultType {
    match result.0 {
        canvas_webgl::prelude::WebGLResult::Boolean(_) => WebGLResultType::Boolean,
        canvas_webgl::prelude::WebGLResult::I32Array(_) => WebGLResultType::I32Array,
        canvas_webgl::prelude::WebGLResult::U32Array(_) => WebGLResultType::U32Array,
        canvas_webgl::prelude::WebGLResult::F32Array(_) => WebGLResultType::F32Array,
        canvas_webgl::prelude::WebGLResult::BooleanArray(_) => WebGLResultType::BooleanArray,
        canvas_webgl::prelude::WebGLResult::U32(_) => WebGLResultType::U32,
        canvas_webgl::prelude::WebGLResult::I32(_) => WebGLResultType::I32,
        canvas_webgl::prelude::WebGLResult::F32(_) => WebGLResultType::F32,
        canvas_webgl::prelude::WebGLResult::String(_) => WebGLResultType::String,
        canvas_webgl::prelude::WebGLResult::None => WebGLResultType::None,
    }
}

fn canvas_native_webgl_result_get_bool(result: &WebGLResult) -> bool {
    match result.0 {
        canvas_webgl::prelude::WebGLResult::Boolean(value) => value,
        _ => false,
    }
}

fn canvas_native_webgl_result_get_i32_array(result: &WebGLResult) -> Vec<i32> {
    match &result.0 {
        canvas_webgl::prelude::WebGLResult::I32Array(value) => value.clone(),
        _ => Vec::new(),
    }
}

fn canvas_native_webgl_result_get_u32_array(result: &WebGLResult) -> Vec<u32> {
    match &result.0 {
        canvas_webgl::prelude::WebGLResult::U32Array(value) => value.clone(),
        _ => Vec::new(),
    }
}

fn canvas_native_webgl_result_get_f32_array(result: &WebGLResult) -> Vec<f32> {
    match &result.0 {
        canvas_webgl::prelude::WebGLResult::F32Array(value) => value.clone(),
        _ => Vec::new(),
    }
}

fn canvas_native_webgl_result_get_bool_array(result: &WebGLResult) -> Vec<u8> {
    match &result.0 {
        canvas_webgl::prelude::WebGLResult::BooleanArray(value) => unsafe {
            std::slice::from_raw_parts(value.as_ptr() as *const u8, value.len()).to_vec()
        },
        _ => Vec::new(),
    }
}

fn canvas_native_webgl_result_get_u32(result: &WebGLResult) -> u32 {
    match result.0 {
        canvas_webgl::prelude::WebGLResult::U32(value) => value,
        _ => 0,
    }
}

fn canvas_native_webgl_result_get_i32(result: &WebGLResult) -> i32 {
    match result.0 {
        canvas_webgl::prelude::WebGLResult::I32(value) => value,
        _ => 0,
    }
}

fn canvas_native_webgl_result_get_f32(result: &WebGLResult) -> f32 {
    match result.0 {
        canvas_webgl::prelude::WebGLResult::F32(value) => value,
        _ => 0.,
    }
}

fn canvas_native_webgl_result_get_string(result: &WebGLResult) -> String {
    let mut ret;
    match result.0 {
        canvas_webgl::prelude::WebGLResult::String(ref result) => {
            let val = result.to_string_lossy();

            if val.contains("OpenGL ES 3.0") {
                return "WebGL 2.0 (OpenGL ES 3.0 NativeScript)".to_string();
            }

            ret = "WebGL 1.0 (OpenGL ES 2.0 NativeScript)".to_string();
        }
        _ => {
            ret = String::default();
        }
    };
    ret
}

fn canvas_native_webgl_result_get_is_none(result: &WebGLResult) -> bool {
    match result.0 {
        canvas_webgl::prelude::WebGLResult::None => true,
        _ => false,
    }
}

/* WebGLResult */

/* WebGLState */
pub fn canvas_native_webgl_state_get_unpack_colorspace_conversion_webgl(state: &WebGLState) -> i32 {
    state.get_inner().get_unpack_colorspace_conversion_webgl()
}

pub fn canvas_native_webgl_state_get_flip_y(state: &WebGLState) -> bool {
    dbg!("state {:?}", state);
    state.get_inner().get_flip_y()
}

pub fn canvas_native_webgl_state_get_premultiplied_alpha(state: &WebGLState) -> bool {
    state.get_inner().get_premultiplied_alpha()
}

pub fn canvas_native_webgl_state_get_drawing_buffer_width(state: &WebGLState) -> i32 {
    state.get_inner().get_drawing_buffer_width()
}

pub fn canvas_native_webgl_state_get_drawing_buffer_height(state: &WebGLState) -> i32 {
    state.get_inner().get_drawing_buffer_height()
}

/* WebGLState */

/* EXT_disjoint_timer_query */
pub fn canvas_native_webgl_ext_disjoint_timer_query_create_query_ext(
    query: &EXT_disjoint_timer_query,
) -> u32 {
    query.0.create_query_ext()
}

pub fn canvas_native_webgl_ext_disjoint_timer_query_delete_query_ext(
    value: u32,
    query: &EXT_disjoint_timer_query,
) {
    query.0.delete_query_ext(value)
}

pub fn canvas_native_webgl_ext_disjoint_timer_query_is_query_ext(
    value: u32,
    query: &EXT_disjoint_timer_query,
) -> bool {
    query.0.is_query_ext(value)
}

pub fn canvas_native_webgl_ext_disjoint_timer_query_begin_query_ext(
    target: u32,
    value: u32,
    query: &EXT_disjoint_timer_query,
) {
    query.0.begin_query_ext(target, value)
}

pub fn canvas_native_webgl_ext_disjoint_timer_query_end_query_ext(
    target: u32,
    query: &EXT_disjoint_timer_query,
) {
    query.0.end_query_ext(target)
}

pub fn canvas_native_webgl_ext_disjoint_timer_query_query_counter_ext(
    value: u32,
    target: u32,
    query: &EXT_disjoint_timer_query,
) {
    query.0.query_counter_ext(value, target)
}

pub fn canvas_native_webgl_ext_disjoint_timer_query_get_query_ext(
    target: u32,
    pname: u32,
    query: &EXT_disjoint_timer_query,
) -> i32 {
    query.0.get_query_ext(target, pname)
}

pub fn canvas_native_webgl_ext_disjoint_timer_query_get_query_object_ext(
    target: u32,
    pname: u32,
    query: &EXT_disjoint_timer_query,
) -> Box<WebGLResult> {
    Box::new(WebGLResult(query.0.get_query_object_ext(target, pname)))
}

/* EXT_disjoint_timer_query */

/* ANGLE_instanced_arrays */
fn canvas_native_webgl_angle_instanced_arrays_draw_arrays_instanced_angle(
    mode: u32,
    first: i32,
    count: i32,
    primcount: i32,
    arrays: &ANGLE_instanced_arrays,
) {
    arrays
        .0
        .draw_arrays_instanced_angle(mode, first, count, primcount)
}

fn canvas_native_webgl_angle_instanced_arrays_draw_elements_instanced_angle(
    mode: u32,
    count: i32,
    type_: u32,
    offset: i32,
    primcount: i32,
    arrays: &ANGLE_instanced_arrays,
) {
    arrays
        .0
        .draw_elements_instanced_angle(mode, count, type_, offset, primcount)
}

fn canvas_native_webgl_angle_instanced_arrays_vertex_attrib_divisor_angle(
    index: u32,
    divisor: u32,
    arrays: &ANGLE_instanced_arrays,
) {
    arrays.0.vertex_attrib_divisor_angle(index, divisor)
}
/* ANGLE_instanced_arrays */

/* WEBGL_lose_context */
fn canvas_native_webgl_lose_context_lose_context(context: &WEBGL_lose_context) {
    context.0.lose_context()
}

fn canvas_native_webgl_lose_context_restore_context(context: &WEBGL_lose_context) {
    context.0.restore_context()
}
/* WEBGL_lose_context */

/* WEBGL_draw_buffers */

fn canvas_native_webgl_draw_buffers_draw_buffers_webgl(
    buffers: &[u32],
    context: &WEBGL_draw_buffers,
) {
    context.0.draw_buffers_webgl(buffers);
}

/* WEBGL_draw_buffers */

/* OES_vertex_array_object */

fn canvas_native_webgl_oes_vertex_array_object_create_vertex_array_oes(
    object: &OES_vertex_array_object,
) -> u32 {
    object.0.create_vertex_array_oes()
}

fn canvas_native_webgl_oes_vertex_array_object_delete_vertex_array_oes(
    array_object: u32,
    object: &OES_vertex_array_object,
) {
    object.0.delete_vertex_array_oes(array_object)
}

fn canvas_native_webgl_oes_vertex_array_object_is_vertex_array_oes(
    array_object: u32,
    object: &OES_vertex_array_object,
) -> bool {
    object.0.is_vertex_array_oes(array_object)
}

fn canvas_native_webgl_oes_vertex_array_object_bind_vertex_array_oes(
    array_object: u32,
    object: &OES_vertex_array_object,
) {
    object.0.bind_vertex_array_oes(array_object)
}

/* OES_vertex_array_object */

pub fn canvas_native_webgl_create(
    version: &str,
    alpha: bool,
    antialias: bool,
    depth: bool,
    fail_if_major_performance_caveat: bool,
    power_preference: &str,
    premultiplied_alpha: bool,
    preserve_drawing_buffer: bool,
    stencil: bool,
    desynchronized: bool,
    xr_compatible: bool,
) -> Box<WebGLState> {
    let ctx = GLContext::get_current();
    let version = if version.eq("v1") {
        WebGLVersion::V1
    } else if version.eq("v2") {
        WebGLVersion::V2
    } else {
        WebGLVersion::NONE
    };

    let inner = WebGLState::new_with_context(
        ctx,
        version,
        alpha,
        antialias,
        depth,
        fail_if_major_performance_caveat,
        power_preference,
        premultiplied_alpha,
        preserve_drawing_buffer,
        stencil,
        desynchronized,
        xr_compatible,
        false,
    );
    Box::new(inner)
}

pub fn canvas_native_webgl_create_no_window(
    width: i32,
    height: i32,
    version: &str,
    alpha: bool,
    antialias: bool,
    depth: bool,
    fail_if_major_performance_caveat: bool,
    power_preference: &str,
    premultiplied_alpha: bool,
    preserve_drawing_buffer: bool,
    stencil: bool,
    desynchronized: bool,
    xr_compatible: bool,
    is_canvas: bool,
) -> Box<WebGLState> {
    Box::new(canvas_native_webgl_create_no_window_internal(
        width,
        height,
        version,
        alpha,
        antialias,
        depth,
        fail_if_major_performance_caveat,
        power_preference,
        premultiplied_alpha,
        preserve_drawing_buffer,
        stencil,
        desynchronized,
        xr_compatible,
        is_canvas,
    ))
}

pub fn canvas_native_webgl_create_no_window_internal(
    width: i32,
    height: i32,
    version: &str,
    alpha: bool,
    antialias: bool,
    depth: bool,
    fail_if_major_performance_caveat: bool,
    power_preference: &str,
    premultiplied_alpha: bool,
    preserve_drawing_buffer: bool,
    stencil: bool,
    desynchronized: bool,
    xr_compatible: bool,
    is_canvas: bool,
) -> WebGLState {
    let mut surface = 0;
    let version = if version.eq("v1") || version.eq("canvas") {
        surface = 1;
        WebGLVersion::V1
    } else if version.eq("v2") {
        surface = 2;
        WebGLVersion::V2
    } else {
        WebGLVersion::NONE
    };

    let mut attrs = canvas_core::context_attributes::ContextAttributes::new(
        alpha,
        antialias,
        depth,
        fail_if_major_performance_caveat,
        power_preference,
        premultiplied_alpha,
        preserve_drawing_buffer,
        stencil,
        desynchronized,
        xr_compatible,
        is_canvas,
    );

    let ctx = GLContext::new_with_no_window(width, height, surface, &mut attrs);

    WebGLState::new_with_context(
        ctx,
        version,
        attrs.get_alpha(),
        attrs.get_antialias(),
        attrs.get_depth(),
        attrs.get_fail_if_major_performance_caveat(),
        attrs.get_power_preference().as_str(),
        attrs.get_premultiplied_alpha(),
        attrs.get_preserve_drawing_buffer(),
        attrs.get_stencil(),
        attrs.get_desynchronized(),
        attrs.get_xr_compatible(),
        attrs.get_is_canvas(),
    )
}

pub fn canvas_native_webgl_active_texture(texture: u32, state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_active_texture(texture, state.get_inner_mut())
}

pub fn canvas_native_webgl_attach_shader(program: u32, shader: u32, state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_attach_shader(program, shader, state.get_inner_mut())
}

pub fn canvas_native_webgl_bind_attrib_location(
    program: u32,
    index: u32,
    name: &str,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_bind_attrib_location(program, index, name, state.get_inner_mut())
}

pub fn canvas_native_webgl_bind_buffer(target: u32, buffer: u32, state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_bind_buffer(target, buffer, state.get_inner_mut())
}

pub fn canvas_native_webgl_bind_frame_buffer(
    target: u32,
    framebuffer: u32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_bind_frame_buffer(target, framebuffer, state.get_inner_mut())
}

pub fn canvas_native_webgl_bind_render_buffer(
    target: u32,
    renderbuffer: u32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_bind_render_buffer(target, renderbuffer, state.get_inner_mut())
}

pub fn canvas_native_webgl_bind_texture(target: u32, texture: u32, state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_bind_texture(target, texture, state.get_inner_mut())
}

pub fn canvas_native_webgl_blend_color(
    red: f32,
    green: f32,
    blue: f32,
    alpha: f32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_blend_color(red, green, blue, alpha, state.get_inner_mut())
}

pub fn canvas_native_webgl_blend_equation_separate(
    mode_rgb: u32,
    mode_alpha: u32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_blend_equation_separate(
        mode_rgb,
        mode_alpha,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl_blend_equation(mode: u32, state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_blend_equation(mode, state.get_inner_mut())
}

pub fn canvas_native_webgl_blend_func_separate(
    src_rgb: u32,
    dst_rgb: u32,
    src_alpha: u32,
    dst_alpha: u32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_blend_func_separate(
        src_rgb,
        dst_rgb,
        src_alpha,
        dst_alpha,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl_blend_func(sfactor: u32, dfactor: u32, state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_blend_func(sfactor, dfactor, state.get_inner_mut())
}

pub fn canvas_native_webgl_buffer_data(
    target: u32,
    src_data: &[u8],
    usage: u32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_buffer_data(target, src_data, usage, state.get_inner_mut())
}

pub fn canvas_native_webgl_buffer_data_u16(
    target: u32,
    src_data: &[u16],
    usage: u32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_buffer_data_u16(target, src_data, usage, state.get_inner_mut())
}

pub fn canvas_native_webgl_buffer_data_f32(
    target: u32,
    src_data: &[f32],
    usage: u32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_buffer_data_f32(target, src_data, usage, state.get_inner_mut())
}

pub fn canvas_native_webgl_buffer_data_none(
    target: u32,
    size: isize,
    usage: u32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_buffer_data_none(target, size, usage, state.get_inner_mut())
}

pub fn canvas_native_webgl_buffer_sub_data(
    target: u32,
    offset: isize,
    src_data: &[u8],
    state: &mut WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_buffer_sub_data(target, offset, src_data, state.get_inner_mut())
}

pub fn canvas_native_webgl_buffer_sub_data_none(
    target: u32,
    offset: isize,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_buffer_sub_data_none(target, offset, state.get_inner_mut())
}

pub fn canvas_native_webgl_check_frame_buffer_status(target: u32, state: &mut WebGLState) -> u32 {
    canvas_webgl::webgl::canvas_native_webgl_check_frame_buffer_status(target, state.get_inner_mut())
}

pub fn canvas_native_webgl_clear(mask: u32, state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_clear(mask, state.get_inner_mut())
}

pub fn canvas_native_webgl_clear_color(
    red: f32,
    green: f32,
    blue: f32,
    alpha: f32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_clear_color(red, green, blue, alpha, state.get_inner_mut())
}

pub fn canvas_native_webgl_clear_depth(depth: f32, state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_clear_depth(depth, state.get_inner_mut())
}

pub fn canvas_native_webgl_clear_stencil(stencil: i32, state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_clear_stencil(stencil, state.get_inner_mut())
}

pub fn canvas_native_webgl_color_mask(
    red: bool,
    green: bool,
    blue: bool,
    alpha: bool,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_color_mask(red, green, blue, alpha, state.get_inner_mut())
}

pub fn canvas_native_webgl_commit(_: &mut WebGLState) {
    // noop
}

pub fn canvas_native_webgl_compile_shader(shader: u32, state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_compile_shader(shader, state.get_inner_mut())
}

pub fn canvas_native_webgl_compressed_tex_image2d(
    target: u32,
    level: i32,
    internalformat: u32,
    width: i32,
    height: i32,
    border: i32,
    pixels: &[u8],
    state: &mut WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_compressed_tex_image2d(
        target,
        level,
        internalformat,
        width,
        height,
        border,
        pixels,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl_compressed_tex_image2d_none(
    target: u32,
    level: i32,
    internalformat: u32,
    width: i32,
    height: i32,
    border: i32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_compressed_tex_image2d_none(
        target,
        level,
        internalformat,
        width,
        height,
        border,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl_compressed_tex_sub_image2d(
    target: u32,
    level: i32,
    xoffset: i32,
    yoffset: i32,
    width: i32,
    height: i32,
    format: u32,
    pixels: &[u8],
    state: &mut WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_compressed_tex_sub_image2d(
        target,
        level,
        xoffset,
        yoffset,
        width,
        height,
        format,
        pixels,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl_copy_tex_image2d(
    target: u32,
    level: i32,
    internalformat: u32,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    border: i32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_copy_tex_image2d(
        target,
        level,
        internalformat,
        x,
        y,
        width,
        height,
        border,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl_copy_tex_sub_image2d(
    target: u32,
    level: i32,
    xoffset: i32,
    yoffset: i32,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_copy_tex_sub_image2d(
        target,
        level,
        xoffset,
        yoffset,
        x,
        y,
        width,
        height,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl_create_buffer(state: &mut WebGLState) -> u32 {
    canvas_webgl::webgl::canvas_native_webgl_create_buffer(state.get_inner_mut())
}

pub fn canvas_native_webgl_create_framebuffer(state: &mut WebGLState) -> u32 {
    canvas_webgl::webgl::canvas_native_webgl_create_framebuffer(state.get_inner_mut())
}

pub fn canvas_native_webgl_create_program(state: &mut WebGLState) -> u32 {
    canvas_webgl::webgl::canvas_native_webgl_create_program(state.get_inner_mut())
}

pub fn canvas_native_webgl_create_renderbuffer(state: &mut WebGLState) -> u32 {
    canvas_webgl::webgl::canvas_native_webgl_create_renderbuffer(state.get_inner_mut())
}

pub fn canvas_native_webgl_create_shader(shader_type: u32, state: &mut WebGLState) -> u32 {
    canvas_webgl::webgl::canvas_native_webgl_create_shader(shader_type, state.get_inner_mut())
}

pub fn canvas_native_webgl_create_texture(state: &mut WebGLState) -> u32 {
    canvas_webgl::webgl::canvas_native_webgl_create_texture(state.get_inner_mut())
}

pub fn canvas_native_webgl_cull_face(mode: u32, state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_cull_face(mode, state.get_inner_mut())
}

pub fn canvas_native_webgl_delete_buffer(buffer: u32, state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_delete_buffer(buffer, state.get_inner_mut())
}

pub fn canvas_native_webgl_delete_framebuffer(frame_buffer: u32, state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_delete_framebuffer(frame_buffer, state.get_inner_mut())
}

pub fn canvas_native_webgl_delete_program(program: u32, state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_delete_program(program, state.get_inner_mut())
}

pub fn canvas_native_webgl_delete_renderbuffer(render_buffer: u32, state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_delete_renderbuffer(render_buffer, state.get_inner_mut())
}

pub fn canvas_native_webgl_delete_shader(shader: u32, state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_delete_shader(shader, state.get_inner_mut())
}

pub fn canvas_native_webgl_delete_texture(texture: u32, state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_delete_texture(texture, state.get_inner_mut())
}

pub fn canvas_native_webgl_depth_func(func: u32, state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_depth_func(func, state.get_inner_mut())
}

pub fn canvas_native_webgl_depth_mask(flag: bool, state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_depth_mask(flag, state.get_inner_mut())
}

pub fn canvas_native_webgl_depth_range(z_near: f32, z_far: f32, state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_depth_range(z_near, z_far, state.get_inner_mut())
}

pub fn canvas_native_webgl_detach_shader(program: u32, shader: u32, state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_detach_shader(program, shader, state.get_inner_mut())
}

pub fn canvas_native_webgl_disable(cap: u32, state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_disable(cap, state.get_inner_mut())
}

pub fn canvas_native_webgl_disable_vertex_attrib_array(index: u32, state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_disable_vertex_attrib_array(index, state.get_inner_mut())
}

pub fn canvas_native_webgl_draw_arrays(mode: u32, first: i32, count: i32, state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_draw_arrays(mode, first, count, state.get_inner_mut())
    // Flush Context
}

pub fn canvas_native_webgl_draw_elements(
    mode: u32,
    count: i32,
    element_type: u32,
    offset: isize,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_draw_elements(
        mode,
        count,
        element_type,
        offset,
        state.get_inner_mut(),
    )
    // Flush Context
}

pub fn canvas_native_webgl_enable(cap: u32, state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_enable(cap, state.get_inner_mut())
}

pub fn canvas_native_webgl_enable_vertex_attrib_array(index: u32, state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_enable_vertex_attrib_array(index, state.get_inner_mut())
}

pub fn canvas_native_webgl_finish(state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_finish(state.get_inner_mut())
}

pub fn canvas_native_webgl_flush(state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_flush(state.get_inner_mut())
}

pub fn canvas_native_webgl_framebuffer_renderbuffer(
    target: u32,
    attachment: u32,
    renderbuffertarget: u32,
    renderbuffer: u32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_framebuffer_renderbuffer(
        target,
        attachment,
        renderbuffertarget,
        renderbuffer,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl_framebuffer_texture2d(
    target: u32,
    attachment: u32,
    textarget: u32,
    texture: u32,
    level: i32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_framebuffer_texture2d(
        target,
        attachment,
        textarget,
        texture,
        level,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl_front_face(mode: u32, state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_front_face(mode, state.get_inner_mut())
}

pub fn canvas_native_webgl_generate_mipmap(target: u32, state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_generate_mipmap(target, state.get_inner_mut())
}

pub fn canvas_native_webgl_get_active_attrib(
    program: u32,
    index: u32,
    state: &mut WebGLState,
) -> Box<WebGLActiveInfo> {
    let info =
        canvas_webgl::webgl::canvas_native_webgl_get_active_attrib(program, index, state.get_inner_mut());

    Box::new(WebGLActiveInfo(info))
}

pub fn canvas_native_webgl_get_active_uniform(
    program: u32,
    index: u32,
    state: &mut WebGLState,
) -> Box<WebGLActiveInfo> {
    let info =
        canvas_webgl::webgl::canvas_native_webgl_get_active_uniform(program, index, state.get_inner_mut());

    Box::new(WebGLActiveInfo(info))
}

pub fn canvas_native_webgl_get_attached_shaders(program: u32, state: &mut WebGLState) -> Vec<u32> {
    canvas_webgl::webgl::canvas_native_webgl_get_attached_shaders(program, state.get_inner_mut())
}

pub fn canvas_native_webgl_get_attrib_location(
    program: u32,
    name: &str,
    state: &mut WebGLState,
) -> i32 {
    canvas_webgl::webgl::canvas_native_webgl_get_attrib_location(program, name, state.get_inner_mut())
}

pub fn canvas_native_webgl_get_buffer_parameter(
    target: u32,
    pname: u32,
    state: &mut WebGLState,
) -> i32 {
    canvas_webgl::webgl::canvas_native_webgl_get_buffer_parameter(target, pname, state.get_inner_mut())
}

pub fn canvas_native_webgl_get_context_attributes(state: &WebGLState) -> Box<ContextAttributes> {
    Box::new(ContextAttributes(
        canvas_webgl::webgl::canvas_native_webgl_get_context_attributes(state.get_inner()),
    ))
}

pub fn canvas_native_webgl_get_error(state: &mut WebGLState) -> u32 {
    canvas_webgl::webgl::canvas_native_webgl_get_error(state.get_inner_mut())
}

pub fn canvas_native_webgl_get_extension(
    name: &str,
    state: &mut WebGLState,
) -> Box<WebGLExtension> {
    Box::new(WebGLExtension(
        canvas_webgl::webgl::canvas_native_webgl_get_extension(name, state.get_inner_mut()),
    ))
}

pub fn canvas_native_webgl_get_framebuffer_attachment_parameter(
    target: u32,
    attachment: u32,
    pname: u32,
    state: &mut WebGLState,
) -> Box<WebGLFramebufferAttachmentParameter> {
    Box::new(WebGLFramebufferAttachmentParameter(
        canvas_webgl::webgl::canvas_native_webgl_get_framebuffer_attachment_parameter(
            target,
            attachment,
            pname,
            state.get_inner_mut(),
        ),
    ))
}

pub fn canvas_native_webgl_framebuffer_attachment_parameter_get_is_texture(
    param: &WebGLFramebufferAttachmentParameter,
) -> bool {
    param.get_is_texture()
}

pub fn canvas_native_webgl_framebuffer_attachment_parameter_get_is_renderbuffer(
    param: &WebGLFramebufferAttachmentParameter,
) -> bool {
    param.get_is_renderbuffer()
}

pub fn canvas_native_webgl_framebuffer_attachment_parameter_get_value(
    param: &WebGLFramebufferAttachmentParameter,
) -> i32 {
    param.get_value()
}

pub fn canvas_native_webgl_get_parameter(pname: u32, state: &mut WebGLState) -> Box<WebGLResult> {
    Box::new(WebGLResult(canvas_webgl::webgl::canvas_native_webgl_get_parameter(
        pname,
        state.get_inner_mut(),
    )))
}

pub fn canvas_native_webgl_get_program_info_log(program: u32, state: &mut WebGLState) -> String {
    canvas_webgl::webgl::canvas_native_webgl_get_program_info_log(program, state.get_inner_mut())
}

pub fn canvas_native_webgl_get_program_parameter(
    program: u32,
    pname: u32,
    state: &mut WebGLState,
) -> Box<WebGLResult> {
    Box::new(WebGLResult(
        canvas_webgl::webgl::canvas_native_webgl_get_program_parameter(program, pname, state.get_inner_mut()),
    ))
}

pub fn canvas_native_webgl_get_renderbuffer_parameter(
    target: u32,
    pname: u32,
    state: &mut WebGLState,
) -> i32 {
    canvas_webgl::webgl::canvas_native_webgl_get_renderbuffer_parameter(target, pname, state.get_inner_mut())
}

pub fn canvas_native_webgl_get_shader_info_log(shader: u32, state: &mut WebGLState) -> String {
    canvas_webgl::webgl::canvas_native_webgl_get_shader_info_log(shader, state.get_inner_mut())
}

pub fn canvas_native_webgl_get_shader_parameter(
    shader: u32,
    pname: u32,
    state: &mut WebGLState,
) -> Box<WebGLResult> {
    Box::new(WebGLResult(
        canvas_webgl::webgl::canvas_native_webgl_get_shader_parameter(shader, pname, state.get_inner_mut()),
    ))
}

pub fn canvas_native_webgl_get_shader_precision_format(
    shader_type: u32,
    precision_type: u32,
    state: &mut WebGLState,
) -> Box<WebGLShaderPrecisionFormat> {
    Box::new(WebGLShaderPrecisionFormat(
        canvas_webgl::webgl::canvas_native_webgl_get_shader_precision_format(
            shader_type,
            precision_type,
            state.get_inner_mut(),
        ),
    ))
}

pub fn canvas_native_webgl_get_shader_source(shader: u32, state: &mut WebGLState) -> String {
    canvas_webgl::webgl::canvas_native_webgl_get_shader_source(shader, state.get_inner_mut())
}

pub fn canvas_native_webgl_get_supported_extensions(state: &mut WebGLState) -> Vec<String> {
    canvas_webgl::webgl::canvas_native_webgl_get_supported_extensions(state.get_inner_mut())
}

pub fn canvas_native_webgl_get_supported_extensions_to_string(state: &mut WebGLState) -> String {
    let mut ret = canvas_webgl::webgl::canvas_native_webgl_get_supported_extensions(state.get_inner_mut());
    ret.join(",").to_string()
}

pub fn canvas_native_webgl_get_tex_parameter(
    target: u32,
    pname: u32,
    state: &mut WebGLState,
) -> i32 {
    canvas_webgl::webgl::canvas_native_webgl_get_tex_parameter(target, pname, state.get_inner_mut())
}

pub fn canvas_native_webgl_get_uniform_location(
    program: u32,
    name: &str,
    state: &mut WebGLState,
) -> i32 {
    canvas_webgl::webgl::canvas_native_webgl_get_uniform_location(program, name, state.get_inner_mut())
}

pub fn canvas_native_webgl_get_uniform(
    program: u32,
    location: i32,
    state: &mut WebGLState,
) -> Box<WebGLResult> {
    Box::new(WebGLResult(canvas_webgl::webgl::canvas_native_webgl_get_uniform(
        program,
        location,
        state.get_inner_mut(),
    )))
}

pub fn canvas_native_webgl_get_vertex_attrib_offset(
    index: u32,
    pname: u32,
    state: &mut WebGLState,
) -> usize {
    canvas_webgl::webgl::canvas_native_webgl_get_vertex_attrib_offset(index, pname, state.get_inner_mut())
}

pub fn canvas_native_webgl_get_vertex_attrib(
    index: u32,
    pname: u32,
    state: &mut WebGLState,
) -> Box<WebGLResult> {
    Box::new(WebGLResult(
        canvas_webgl::webgl::canvas_native_webgl_get_vertex_attrib(index, pname, state.get_inner_mut()),
    ))
}

pub fn canvas_native_webgl_get_is_context_lost(_: &mut WebGLState) -> bool {
    // TODO improve
    false
}

pub fn canvas_native_webgl_hint(target: u32, mode: u32, state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_hint(target, mode, state.get_inner_mut())
}

pub fn canvas_native_webgl_is_buffer(buffer: u32, state: &mut WebGLState) -> bool {
    canvas_webgl::webgl::canvas_native_webgl_is_buffer(buffer, state.get_inner_mut())
}

pub fn canvas_native_webgl_is_enabled(cap: u32, state: &mut WebGLState) -> bool {
    canvas_webgl::webgl::canvas_native_webgl_is_enabled(cap, state.get_inner_mut())
}

pub fn canvas_native_webgl_is_framebuffer(framebuffer: u32, state: &mut WebGLState) -> bool {
    canvas_webgl::webgl::canvas_native_webgl_is_framebuffer(framebuffer, state.get_inner_mut())
}

pub fn canvas_native_webgl_is_program(program: u32, state: &mut WebGLState) -> bool {
    canvas_webgl::webgl::canvas_native_webgl_is_program(program, state.get_inner_mut())
}

pub fn canvas_native_webgl_is_renderbuffer(renderbuffer: u32, state: &mut WebGLState) -> bool {
    canvas_webgl::webgl::canvas_native_webgl_is_renderbuffer(renderbuffer, state.get_inner_mut())
}

pub fn canvas_native_webgl_is_shader(shader: u32, state: &mut WebGLState) -> bool {
    canvas_webgl::webgl::canvas_native_webgl_is_shader(shader, state.get_inner_mut())
}

pub fn canvas_native_webgl_is_texture(texture: u32, state: &mut WebGLState) -> bool {
    canvas_webgl::webgl::canvas_native_webgl_is_texture(texture, state.get_inner_mut())
}

pub fn canvas_native_webgl_line_width(width: f32, state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_line_width(width, state.get_inner_mut())
}

pub fn canvas_native_webgl_link_program(program: u32, state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_link_program(program, state.get_inner_mut())
}

pub fn canvas_native_webgl_pixel_storei(pname: u32, param: i32, state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_pixel_storei(pname, param, state.get_inner_mut())
}

pub fn canvas_native_webgl_polygon_offset(factor: f32, units: f32, state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_polygon_offset(factor, units, state.get_inner_mut())
}

pub fn canvas_native_webgl_read_pixels_u8(
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    format: u32,
    pixel_type: u32,
    pixels: &mut [u8],
    state: &mut WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_read_pixels_u8(
        x,
        y,
        width,
        height,
        format,
        pixel_type,
        pixels,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl_read_pixels_u16(
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    format: u32,
    pixel_type: u32,
    pixels: &mut [u16],
    state: &mut WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_read_pixels_u16(
        x,
        y,
        width,
        height,
        format,
        pixel_type,
        pixels,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl_read_pixels_f32(
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    format: u32,
    pixel_type: u32,
    pixels: &mut [f32],
    state: &mut WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_read_pixels_f32(
        x,
        y,
        width,
        height,
        format,
        pixel_type,
        pixels,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl_renderbuffer_storage(
    target: u32,
    internal_format: u32,
    width: i32,
    height: i32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_renderbuffer_storage(
        target,
        internal_format,
        width,
        height,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl_sample_coverage(value: f32, invert: bool, state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_sample_coverage(value, invert, state.get_inner_mut())
}

pub fn canvas_native_webgl_scissor(
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_scissor(x, y, width, height, state.get_inner_mut())
}

pub fn canvas_native_webgl_shader_source(shader: u32, source: &str, state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_shader_source(shader, source, state.get_inner_mut())
}

pub fn canvas_native_webgl_stencil_func(
    func: u32,
    reference: i32,
    mask: u32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_stencil_func(func, reference, mask, state.get_inner_mut())
}

pub fn canvas_native_webgl_stencil_func_separate(
    face: u32,
    func: u32,
    reference: i32,
    mask: u32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_stencil_func_separate(
        face,
        func,
        reference,
        mask,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl_stencil_mask(mask: u32, state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_stencil_mask(mask, state.get_inner_mut())
}

pub fn canvas_native_webgl_stencil_mask_separate(face: u32, mask: u32, state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_stencil_mask_separate(face, mask, state.get_inner_mut())
}

pub fn canvas_native_webgl_stencil_op(fail: u32, zfail: u32, zpass: u32, state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_stencil_op(fail, zfail, zpass, state.get_inner())
}

pub fn canvas_native_webgl_stencil_op_separate(
    face: u32,
    fail: u32,
    zfail: u32,
    zpass: u32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_stencil_op_separate(
        face,
        fail,
        zfail,
        zpass,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl_tex_image2d_image_none(
    target: i32,
    level: i32,
    internalformat: i32,
    format: i32,
    image_type: i32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_tex_image2d_image_none(
        target,
        level,
        internalformat,
        format,
        image_type,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl_tex_image2d_asset(
    target: i32,
    level: i32,
    internalformat: i32,
    format: i32,
    image_type: i32,
    asset: &ImageAsset,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_tex_image2d_asset(
        target,
        level,
        internalformat,
        format,
        image_type,
        asset.0,
        state.get_inner_mut(),
    )
}

fn canvas_native_webgl_tex_image2d_canvas2d(
    target: i32,
    level: i32,
    internalformat: i32,
    format: i32,
    image_type: i32,
    canvas: &mut crate::canvas2d::CanvasRenderingContext2D,
    state: &mut WebGLState,
) {
    let mut pixels = canvas_webgl::webgl::canvas_native_webgl_read_canvas2d_pixels(canvas, &mut state.0);
    canvas_webgl::webgl::canvas_native_webgl_tex_image2d(
        target,
        level,
        internalformat,
        pixels.0,
        pixels.1,
        0,
        format,
        image_type,
        pixels.2.as_mut_slice(),
        state.get_inner_mut(),
    );
}

fn canvas_native_webgl_tex_image2d_webgl(
    target: i32,
    level: i32,
    internalformat: i32,
    format: i32,
    image_type: i32,
    webgl: &mut WebGLState,
    state: &mut WebGLState,
) {
    let mut pixels = canvas_webgl::webgl::canvas_native_webgl_read_webgl_pixels(&mut webgl.0, &mut state.0);
    canvas_webgl::webgl::canvas_native_webgl_tex_image2d(
        target,
        level,
        internalformat,
        pixels.0,
        pixels.1,
        0,
        format,
        image_type,
        pixels.2.as_mut_slice(),
        state.get_inner_mut(),
    );
}

pub fn canvas_native_webgl_tex_image2d(
    target: i32,
    level: i32,
    internalformat: i32,
    width: i32,
    height: i32,
    border: i32,
    format: i32,
    image_type: i32,
    buf: &mut [u8],
    state: &mut WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_tex_image2d(
        target,
        level,
        internalformat,
        width,
        height,
        border,
        format,
        image_type,
        buf,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl_tex_image2d_none(
    target: i32,
    level: i32,
    internalformat: i32,
    width: i32,
    height: i32,
    border: i32,
    format: i32,
    image_type: i32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_tex_image2d_none(
        target,
        level,
        internalformat,
        width,
        height,
        border,
        format,
        image_type,
        state.get_inner_mut(),
    )
}

fn canvas_native_webgl_tex_image2d_image_asset(
    target: i32,
    level: i32,
    internalformat: i32,
    format: i32,
    image_type: i32,
    image_asset: &ImageAsset,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_tex_image2d_asset(
        target,
        level,
        internalformat,
        format,
        image_type,
        &image_asset.0,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl_tex_parameterf(target: u32, pname: u32, param: f32, state: &WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_tex_parameterf(target, pname, param, state.get_inner())
}

pub fn canvas_native_webgl_tex_parameteri(target: u32, pname: u32, param: i32, state: &WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_tex_parameteri(target, pname, param, state.get_inner())
}

pub fn canvas_native_webgl_tex_sub_image2d_asset(
    target: u32,
    level: i32,
    xoffset: i32,
    yoffset: i32,
    format: u32,
    image_type: i32,
    asset: &ImageAsset,
    state: &WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_tex_sub_image2d_asset(
        target,
        level,
        xoffset,
        yoffset,
        format,
        image_type,
        &asset.0,
        state.get_inner(),
    )
}

pub fn canvas_native_webgl_tex_sub_image2d(
    target: u32,
    level: i32,
    xoffset: i32,
    yoffset: i32,
    width: i32,
    height: i32,
    format: u32,
    image_type: i32,
    buf: &[u8],
    state: &WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_tex_sub_image2d(
        target,
        level,
        xoffset,
        yoffset,
        width,
        height,
        format,
        image_type,
        buf,
        state.get_inner(),
    )
}

pub fn canvas_native_webgl_uniform1f(location: i32, v0: f32, state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_uniform1f(location, v0, state.get_inner())
}

pub fn canvas_native_webgl_uniform1fv(location: i32, value: &[f32], state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_uniform1fv(location, value, state.get_inner())
}

pub fn canvas_native_webgl_uniform1i(location: i32, v0: i32, state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_uniform1i(location, v0, state.get_inner())
}

pub fn canvas_native_webgl_uniform1iv(location: i32, value: &[i32], state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_uniform1iv(location, value, state.get_inner())
}

pub fn canvas_native_webgl_uniform2f(location: i32, v0: f32, v1: f32, state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_uniform2f(location, v0, v1, state.get_inner())
}

pub fn canvas_native_webgl_uniform2fv(location: i32, value: &[f32], state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_uniform2fv(location, value, state.get_inner())
}

pub fn canvas_native_webgl_uniform2i(location: i32, v0: i32, v1: i32, state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_uniform2i(location, v0, v1, state.get_inner())
}

pub fn canvas_native_webgl_uniform2iv(location: i32, value: &[i32], state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_uniform2iv(location, value, state.get_inner())
}

pub fn canvas_native_webgl_uniform3f(
    location: i32,
    v0: f32,
    v1: f32,
    v2: f32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_uniform3f(location, v0, v1, v2, state.get_inner())
}

pub fn canvas_native_webgl_uniform3fv(location: i32, value: &[f32], state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_uniform3fv(location, value, state.get_inner())
}

pub fn canvas_native_webgl_uniform3i(
    location: i32,
    v0: i32,
    v1: i32,
    v2: i32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_uniform3i(location, v0, v1, v2, state.get_inner())
}

pub fn canvas_native_webgl_uniform3iv(location: i32, value: &[i32], state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_uniform3iv(location, value, state.get_inner())
}

pub fn canvas_native_webgl_uniform4f(
    location: i32,
    v0: f32,
    v1: f32,
    v2: f32,
    v3: f32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_uniform4f(location, v0, v1, v2, v3, state.get_inner())
}

pub fn canvas_native_webgl_uniform4fv(location: i32, value: &[f32], state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_uniform4fv(location, value, state.get_inner_mut())
}

pub fn canvas_native_webgl_uniform4i(
    location: i32,
    v0: i32,
    v1: i32,
    v2: i32,
    v3: i32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_uniform4i(location, v0, v1, v2, v3, state.get_inner_mut())
}

pub fn canvas_native_webgl_uniform4iv(location: i32, value: &[i32], state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_uniform4iv(location, value, state.get_inner_mut())
}

pub fn canvas_native_webgl_uniform_matrix2fv(
    location: i32,
    transpose: bool,
    value: &[f32],
    state: &mut WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_uniform_matrix2fv(
        location,
        transpose,
        value,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl_uniform_matrix3fv(
    location: i32,
    transpose: bool,
    value: &[f32],
    state: &mut WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_uniform_matrix3fv(
        location,
        transpose,
        value,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl_uniform_matrix4fv(
    location: i32,
    transpose: bool,
    value: &[f32],
    state: &mut WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_uniform_matrix4fv(
        location,
        transpose,
        value,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl_use_program(program: u32, state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_use_program(program, state.get_inner_mut())
}

pub fn canvas_native_webgl_validate_program(program: u32, state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_validate_program(program, state.get_inner_mut())
}

pub fn canvas_native_webgl_vertex_attrib1f(index: u32, v0: f32, state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_vertex_attrib1f(index, v0, state.get_inner_mut())
}

pub fn canvas_native_webgl_vertex_attrib1fv(index: u32, value: &[f32], state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_vertex_attrib1fv(index, value, state.get_inner_mut())
}

pub fn canvas_native_webgl_vertex_attrib2f(index: u32, v0: f32, v1: f32, state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_vertex_attrib2f(index, v0, v1, state.get_inner_mut())
}

pub fn canvas_native_webgl_vertex_attrib2fv(index: u32, value: &[f32], state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_vertex_attrib2fv(index, value, state.get_inner_mut())
}

pub fn canvas_native_webgl_vertex_attrib3f(
    index: u32,
    v0: f32,
    v1: f32,
    v2: f32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_vertex_attrib3f(index, v0, v1, v2, state.get_inner_mut())
}

pub fn canvas_native_webgl_vertex_attrib3fv(index: u32, value: &[f32], state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_vertex_attrib3fv(index, value, state.get_inner_mut())
}

pub fn canvas_native_webgl_vertex_attrib4f(
    index: u32,
    v0: f32,
    v1: f32,
    v2: f32,
    v3: f32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_vertex_attrib4f(index, v0, v1, v2, v3, state.get_inner_mut())
}

pub fn canvas_native_webgl_vertex_attrib4fv(index: u32, value: &[f32], state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_vertex_attrib4fv(index, value, state.get_inner_mut())
}

pub fn canvas_native_webgl_vertex_attrib_pointer(
    index: u32,
    size: i32,
    d_type: u32,
    normalized: bool,
    stride: i32,
    offset: isize,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_vertex_attrib_pointer(
        index,
        size,
        d_type,
        normalized,
        stride,
        offset,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl_viewport(
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_viewport(x, y, width, height, state.get_inner_mut())
}

/* WebGL */
