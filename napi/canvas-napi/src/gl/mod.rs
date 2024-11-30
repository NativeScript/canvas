mod webgl_program;
mod webgl_shader;
mod webgl_buffer;
mod webgl_framebuffer;
mod webgl_renderbuffer;
mod webgl_texture;
mod webgl_active_info;
mod context_attributes;
mod extensions;
mod webgl_shader_precision_format;
mod webgl_uniform_location;

const UNPACK_FLIP_Y_WEBGL: u32 = 37440;
const UNPACK_PREMULTIPLY_ALPHA_WEBGL: u32 = 37441;
const UNPACK_COLOR_SPACE_CONVERSION_WEBGL: u32 = 37443;

use crate::gl::extensions::{ANGLE_instanced_arrays, EXT_blend_minmax, EXT_color_buffer_half_float, EXT_disjoint_timer_query, EXT_sRGB, EXT_shader_texture_lod, EXT_texture_filter_anisotropic, OES_element_index_uint, OES_fbo_render_mipmap, OES_standard_derivatives, OES_texture_float, OES_texture_float_linear, OES_texture_half_float, OES_texture_half_float_linear, OES_vertex_array_object, WEBGL_color_buffer_float, WEBGL_compressed_texture_atc, WEBGL_compressed_texture_etc, WEBGL_compressed_texture_etc1, WEBGL_compressed_texture_pvrtc, WEBGL_compressed_texture_s3tc, WEBGL_depth_texture, WEBGL_draw_buffers, WEBGL_lose_context};
use crate::gl::webgl_active_info::WebGLActiveInfo;
use crate::gl::webgl_buffer::WebGLBuffer;
use crate::gl::webgl_framebuffer::WebGLFramebuffer;
use crate::gl::webgl_program::WebGLProgram;
use crate::gl::webgl_renderbuffer::WebGLRenderbuffer;
use crate::gl::webgl_shader::WebGLShader;
use crate::gl::webgl_shader_precision_format::WebGLShaderPrecisionFormat;
use crate::gl::webgl_texture::WebGLTexture;
use crate::gl::webgl_uniform_location::WebGLUniformLocation;
use canvas_c::{WebGLExtension, WebGLResultType};
use napi::bindgen_prelude::{Array, Buffer, ClassInstance, Either3, Either4, Float32Array, Int32Array, ObjectFinalize, Uint32Array, Unknown};
use napi::*;
use napi_derive::napi;
use std::ffi::{c_void, CString, IntoStringError};
use std::slice;
use std::sync::Arc;

#[napi(custom_finalize)]
pub struct web_g_l_rendering_context {
    state: *mut canvas_c::WebGLState,
}

impl ObjectFinalize for web_g_l_rendering_context {
    fn finalize(self, _: Env) -> Result<()> {
        canvas_c::canvas_native_webgl_state_destroy(self.state);
        Ok(())
    }
}


#[napi]
impl web_g_l_rendering_context {
    #[napi]
    pub fn render(&self) {
        canvas_c::canvas_native_webgl_make_current_and_swap_buffers(self.state);
    }

    #[napi(getter)]
    pub fn get_drawing_buffer_width(&self) -> i32 {
        canvas_c::canvas_native_webgl_state_get_drawing_buffer_width(self.state)
    }

    #[napi(getter)]
    pub fn get_drawing_buffer_height(&self) -> i32 {
        canvas_c::canvas_native_webgl_state_get_drawing_buffer_height(self.state)
    }

    #[napi(factory)]
    pub fn with_view(
        view: i64,
        version: i32,
        alpha: bool,
        antialias: bool,
        depth: bool,
        fail_if_major_performance_caveat: bool,
        power_preference: i32,
        premultiplied_alpha: bool,
        preserve_drawing_buffer: bool,
        stencil: bool,
        desynchronized: bool,
        xr_compatible: bool,
    ) -> Result<web_g_l_rendering_context> {
        let ret = canvas_c::canvas_native_webgl_create(
            view as _,
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
        );

        if ret.is_null() {
            return Err(napi::Error::from_reason("Invalid parameter"));
        }

        Ok(web_g_l_rendering_context {
            state: ret
        })
    }

    #[napi(factory)]
    pub fn offscreen(
        width: i32,
        height: i32,
        version: i32,
        alpha: bool,
        antialias: bool,
        depth: bool,
        fail_if_major_performance_caveat: bool,
        power_preference: i32,
        premultiplied_alpha: bool,
        preserve_drawing_buffer: bool,
        stencil: bool,
        desynchronized: bool,
        xr_compatible: bool,
        is_canvas: bool,
    ) -> Result<web_g_l_rendering_context> {
        let ret = canvas_c::canvas_native_webgl_create_no_window(
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
        );

        if ret.is_null() {
            return Err(napi::Error::from_reason("Invalid parameter"));
        }

        Ok(web_g_l_rendering_context {
            state: ret
        })
    }

    #[napi]
    pub fn active_texture(&self, texture: u32) {
        canvas_c::canvas_native_webgl_active_texture(texture, self.state);
    }

    #[napi]
    pub fn attach_shader(&self, program: ClassInstance<WebGLProgram>, shader: ClassInstance<WebGLShader>) {
        canvas_c::canvas_native_webgl_attach_shader(program.0, shader.0, self.state);
    }


    #[napi]
    pub fn bind_attrib_location(&self, program: ClassInstance<WebGLProgram>, index: u32, name: JsString) -> Result<()> {
        let name = name.into_utf8()?;
        let name = name.as_str()?;
        let state = unsafe { &mut *self.state };
        unsafe {
            canvas_webgl::webgl::canvas_native_webgl_bind_attrib_location(
                program.0, index, name, state.get_inner_mut(),
            )
        }
        Ok(())
    }

    #[napi]
    pub fn bind_buffer(&self, target: u32, buffer: ClassInstance<WebGLBuffer>) {
        canvas_c::canvas_native_webgl_bind_buffer(target, buffer.0, self.state);
    }

    #[napi]
    pub fn bind_framebuffer(&self, target: u32, framebuffer: ClassInstance<WebGLFramebuffer>) {
        canvas_c::canvas_native_webgl_bind_frame_buffer(target, framebuffer.0, self.state);
    }

    #[napi]
    pub fn bind_renderbuffer(&self, target: u32, renderbuffer: ClassInstance<WebGLRenderbuffer>) {
        canvas_c::canvas_native_webgl_bind_render_buffer(target, renderbuffer.0, self.state);
    }

    #[napi]
    pub fn bind_texture(&self, target: u32, texture: ClassInstance<WebGLTexture>) {
        canvas_c::canvas_native_webgl_bind_texture(target, texture.0, self.state);
    }

    #[napi]
    pub fn blend_color(&self, red: f64, green: f64, blue: f64, alpha: f64) {
        canvas_c::canvas_native_webgl_blend_color(red as f32, green as f32, blue as f32, alpha as f32, self.state);
    }

    #[napi]
    pub fn blend_equation_separate(&self, mode_r_g_b: u32, mode_alpha: u32) {
        canvas_c::canvas_native_webgl_blend_equation_separate(mode_r_g_b, mode_alpha, self.state)
    }

    #[napi]
    pub fn blend_equation(&self, mode: u32) {
        canvas_c::canvas_native_webgl_blend_equation(mode, self.state)
    }

    #[napi]
    pub fn blend_func_separate(&self, src_r_g_b: Option<u32>, dst_r_g_b: Option<u32>, src_alpha: Option<u32>, dst_alpha: Option<u32>) {
        let src_r_g_b = src_r_g_b.unwrap_or(1);
        let dst_r_g_b = dst_r_g_b.unwrap_or(0);
        let src_alpha = src_alpha.unwrap_or(1);
        let dst_alpha = dst_alpha.unwrap_or(0);
        canvas_c::canvas_native_webgl_blend_func_separate(src_r_g_b, dst_r_g_b, src_alpha, dst_alpha, self.state);
    }

    #[napi]
    pub fn blend_func(&self, sfactor: Option<u32>, dfactor: Option<u32>) {
        let sfactor = sfactor.unwrap_or(1);
        let dfactor = dfactor.unwrap_or(0);
        canvas_c::canvas_native_webgl_blend_func(sfactor, dfactor, self.state);
    }


    #[napi]
    pub fn buffer_data(&self, target: u32, size_or_src_data: Option<Either<i64, Buffer>>, usage: Option<u32>) {
        match size_or_src_data {
            Some(size_or_src_data) => {
                match size_or_src_data {
                    Either::A(size) => {
                        match usage {
                            Some(usage) => {
                                canvas_c::canvas_native_webgl_buffer_data_none(target, size as isize, usage, self.state);
                            }
                            None => {
                                canvas_c::canvas_native_webgl_buffer_data_none(target, 0, size as u32, self.state);
                            }
                        }
                    }
                    Either::B(src_data) => {
                        if let Some(usage) = usage {
                            canvas_c::canvas_native_webgl_buffer_data(target, src_data.as_ptr(), src_data.len(), usage, self.state);
                        }
                    }
                }
            }
            _ => {
                if let Some(usage) = usage {
                    canvas_c::canvas_native_webgl_buffer_data_none(target, 0, usage, self.state);
                }
            }
        }
    }

    #[napi]
    pub fn buffer_sub_data(&self, target: u32, offset: i64, src_data: &[u8]) {
        canvas_c::canvas_native_webgl_buffer_sub_data(
            target, offset as isize, src_data.as_ptr(), src_data.len(), self.state,
        )
    }


    #[napi]
    pub fn check_framebuffer_status(&self, target: u32) -> u32 {
        canvas_c::canvas_native_webgl_check_frame_buffer_status(target, self.state)
    }

    #[napi]
    pub fn clear_color(&self,
                       red: f64,
                       green: f64,
                       blue: f64,
                       alpha: f64) {
        canvas_c::canvas_native_webgl_clear_color(red as f32, green as f32, blue as f32, alpha as f32, self.state);
    }

    #[napi]
    pub fn clear_depth(&self, depth: f64) {
        canvas_c::canvas_native_webgl_clear_depth(depth as f32, self.state);
    }

    #[napi]
    pub fn clear_stencil(&self, stencil: i32) {
        canvas_c::canvas_native_webgl_clear_stencil(stencil, self.state);
    }

    #[napi]
    pub fn clear(&self, mask: u32) {
        canvas_c::canvas_native_webgl_clear(mask, self.state);
    }

    #[napi]
    pub fn color_mask(&self, red: bool, green: bool, blue: bool, alpha: bool) {
        canvas_c::canvas_native_webgl_color_mask(red, green, blue, alpha, self.state);
    }


    #[napi]
    pub fn commit(&self) {
        canvas_c::canvas_native_webgl_commit(self.state);
    }

    #[napi]
    pub fn compile_shader(&self, shader: ClassInstance<WebGLShader>) {
        canvas_c::canvas_native_webgl_compile_shader(shader.0, self.state);
    }

    #[napi]
    pub fn compressed_tex_image_2_d(&self, target: u32, level: i32, internalformat: u32, width: i64, height: i64, border: i32, pixels: &[u8]) {
        canvas_c::canvas_native_webgl_compressed_tex_image2d(
            target, level, internalformat, width as i32, height as i32, border, pixels.as_ptr(), pixels.len(), self.state,
        )
    }

    #[napi]
    pub fn compressed_tex_sub_image_2_d(&self, target: u32, level: i32, xoffset: i64, yoffset: i64, width: f64, height: f64, format: u32, pixels: &[u8]) {
        canvas_c::canvas_native_webgl_compressed_tex_sub_image2d(
            target, level, xoffset as i32, yoffset as i32, width as i32, height as i32, format, pixels.as_ptr(), pixels.len(), self.state,
        )
    }

    #[napi]
    pub fn copy_tex_image_2_d(&self, target: u32, level: i32, internalformat: u32, x: i64, y: i64, width: i64, height: i64, border: i32) {
        canvas_c::canvas_native_webgl_copy_tex_image2d(
            target, level, internalformat, x as i32, y as i32, width as i32, height as i32, border, self.state,
        )
    }

    #[napi]
    pub fn copy_tex_sub_image_2_d(&self, target: u32, level: i32, xoffset: i64, yoffset: i64, x: i64, y: i64, width: i64, height: i64) {
        canvas_c::canvas_native_webgl_copy_tex_sub_image2d(
            target, level, xoffset as i32, yoffset as i32, x as i32, y as i32, width as i32, height as i32, self.state,
        )
    }

    #[napi]
    pub fn create_buffer(&self, env: Env) -> Result<ClassInstance<WebGLBuffer>> {
        WebGLBuffer(
            canvas_c::canvas_native_webgl_create_buffer(self.state)
        ).into_instance(env)
    }

    #[napi]
    pub fn create_framebuffer(&self, env: Env) -> Result<ClassInstance<WebGLFramebuffer>> {
        WebGLFramebuffer(canvas_c::canvas_native_webgl_create_framebuffer(self.state))
            .into_instance(env)
    }

    #[napi]
    pub fn create_program(&self, env: Env) -> Result<ClassInstance<WebGLProgram>> {
        WebGLProgram(
            canvas_c::canvas_native_webgl_create_program(self.state)
        ).into_instance(env)
    }
    #[napi]

    pub fn create_renderbuffer(&self, env: Env) -> Result<ClassInstance<WebGLRenderbuffer>> {
        WebGLRenderbuffer(
            canvas_c::canvas_native_webgl_create_renderbuffer(self.state)
        ).into_instance(env)
    }

    #[napi(ts_args_type = "type: number")]
    pub fn create_shader(&self, env: Env, type_: u32) -> Result<ClassInstance<WebGLShader>> {
        WebGLShader(
            canvas_c::canvas_native_webgl_create_shader(type_, self.state),
        ).into_instance(env)
    }

    #[napi]
    pub fn create_texture(&self, env: Env) -> Result<ClassInstance<WebGLTexture>> {
        WebGLTexture(
            canvas_c::canvas_native_webgl_create_texture(self.state)
        ).into_instance(env)
    }

    #[napi]
    pub fn cull_face(&self, mode: u32) {
        canvas_c::canvas_native_webgl_cull_face(mode, self.state)
    }


    #[napi]
    pub fn delete_buffer(&self, buffer: ClassInstance<WebGLBuffer>) {
        canvas_c::canvas_native_webgl_delete_buffer(buffer.0, self.state)
    }

    #[napi]
    pub fn delete_framebuffer(&self, frame_buffer: ClassInstance<WebGLFramebuffer>) {
        canvas_c::canvas_native_webgl_delete_framebuffer(frame_buffer.0, self.state)
    }

    #[napi]
    pub fn delete_program(&self, program: ClassInstance<WebGLProgram>) {
        canvas_c::canvas_native_webgl_delete_program(program.0, self.state)
    }

    #[napi]
    pub fn delete_renderbuffer(&self, render_buffer: ClassInstance<WebGLRenderbuffer>) {
        canvas_c::canvas_native_webgl_delete_renderbuffer(render_buffer.0, self.state)
    }

    #[napi]
    pub fn delete_shader(&self, shader: ClassInstance<WebGLRenderbuffer>) {
        canvas_c::canvas_native_webgl_delete_shader(shader.0, self.state)
    }

    #[napi]
    pub fn delete_texture(&self, texture: ClassInstance<WebGLTexture>) {
        canvas_c::canvas_native_webgl_delete_texture(texture.0, self.state)
    }

    #[napi]
    pub fn depth_func(&self, func: u32) {
        canvas_c::canvas_native_webgl_depth_func(func, self.state);
    }

    #[napi]
    pub fn depth_mask(&self, flag: bool) {
        canvas_c::canvas_native_webgl_depth_mask(flag, self.state);
    }

    #[napi]
    pub fn depth_range(&self, z_near: f64, z_far: f64) {
        canvas_c::canvas_native_webgl_depth_range(z_near as f32, z_far as f32, self.state)
    }

    #[napi]
    pub fn detach_shader(&self, program: ClassInstance<WebGLProgram>, shader: ClassInstance<WebGLShader>) {
        canvas_c::canvas_native_webgl_detach_shader(program.0, shader.0, self.state)
    }

    #[napi]
    pub fn disable_vertex_attrib_array(&self, index: u32) {
        canvas_c::canvas_native_webgl_disable_vertex_attrib_array(index, self.state);
    }

    #[napi]
    pub fn disable(&self, cap: u32) {
        canvas_c::canvas_native_webgl_disable(cap, self.state);
    }

    #[napi]
    pub fn draw_arrays(&self, mode: u32, first: i32, count: i32) {
        canvas_c::canvas_native_webgl_draw_arrays(mode, first, count, self.state);
    }

    #[napi(ts_args_type = "mode: number, count: number, type: number, offset: number")]
    pub fn draw_elements(&self, mode: u32, count: i32, type_: u32, offset: i64) {
        canvas_c::canvas_native_webgl_draw_elements(mode, count, type_, offset as isize, self.state);
    }

    #[napi]
    pub fn enable_vertex_attrib_array(&self, index: u32) {
        canvas_c::canvas_native_webgl_enable_vertex_attrib_array(index, self.state);
    }

    #[napi]
    pub fn enable(&self, cap: u32) {
        canvas_c::canvas_native_webgl_enable(cap, self.state);
    }

    #[napi]
    pub fn finish(&self) {
        canvas_c::canvas_native_webgl_finish(self.state);
    }

    #[napi]
    pub fn flush(&self) {
        canvas_c::canvas_native_webgl_flush(self.state);
    }

    #[napi]
    pub fn framebuffer_renderbuffer(&self, target: u32, attachment: u32, renderbuffertarget: u32, renderbuffer: ClassInstance<WebGLRenderbuffer>) {
        canvas_c::canvas_native_webgl_framebuffer_renderbuffer(target, attachment, renderbuffertarget, renderbuffer.0, self.state)
    }

    #[napi]
    pub fn framebuffer_texture_2_d(&self, target: u32, attachment: u32, textarget: u32, texture: ClassInstance<WebGLTexture>, level: i32) {
        canvas_c::canvas_native_webgl_framebuffer_texture2d(
            target, attachment, textarget, texture.0, level, self.state,
        )
    }

    #[napi]
    pub fn front_face(&self, mode: u32) {
        canvas_c::canvas_native_webgl_front_face(mode, self.state)
    }

    #[napi]
    pub fn generate_mipmap(&self, target: u32) {
        canvas_c::canvas_native_webgl_generate_mipmap(target, self.state);
    }


    #[napi]
    pub fn get_active_attrib(&self, env: Env, program: ClassInstance<WebGLProgram>, index: u32) -> Result<ClassInstance<WebGLActiveInfo>> {
        WebGLActiveInfo(
            canvas_c::canvas_native_webgl_get_active_attrib(
                program.0, index, self.state,
            )
        ).into_instance(env)
    }

    #[napi]
    pub fn get_active_uniform(&self, env: Env, program: ClassInstance<WebGLProgram>, index: u32) -> Result<ClassInstance<WebGLActiveInfo>> {
        WebGLActiveInfo(
            canvas_c::canvas_native_webgl_get_active_uniform(
                program.0, index, self.state,
            )
        ).into_instance(env)
    }

    #[napi]
    pub fn get_attached_shaders(&self, env: Env, program: ClassInstance<WebGLProgram>) -> Vec<ClassInstance<WebGLShader>> {
        let state = unsafe { &mut *self.state };
        let shaders = canvas_webgl::webgl::canvas_native_webgl_get_attached_shaders(program.0, state.get_inner_mut());
        shaders.into_iter()
            .map(|shader| {
                WebGLShader(shader).into_instance(env).unwrap()
            }).collect::<Vec<_>>()
    }

    #[napi]
    pub fn get_attrib_location(&self, program: ClassInstance<WebGLProgram>, name: JsString) -> Result<i32> {
        let name = name.into_utf8()?;
        let name = name.as_str()?;
        let state = unsafe { &mut *self.state };
        Ok(canvas_webgl::webgl::canvas_native_webgl_get_attrib_location(program.0, name, state.get_inner_mut()))
    }

    #[napi]
    pub fn get_buffer_parameter(&self, target: u32, pname: u32) -> i32 {
        canvas_c::canvas_native_webgl_get_buffer_parameter(target, pname, self.state)
    }

    #[napi]
    pub fn get_context_attributes(&self) -> context_attributes::ContextAttributes {
        let attributes = canvas_c::canvas_native_webgl_get_context_attributes(self.state);
        let ret = context_attributes::ContextAttributes::from_c(attributes);
        canvas_c::canvas_native_context_attributes_destroy(attributes);
        ret
    }

    #[napi]
    pub fn get_error(&self) -> u32 {
        canvas_c::canvas_native_webgl_get_error(self.state)
    }

    #[napi]
    pub fn get_extension(&self, env: Env, name: JsString) -> Result<JsUnknown> {
        let name = name.into_utf8()?;
        let name = name.as_str()?;

        if name == "EXT_disjoint_timer_query_webgl2" {
            return env.get_null().map(|null| null.into_unknown());
        }
        let state = unsafe { &mut *self.state };

        let ext = canvas_webgl::webgl::canvas_native_webgl_get_extension(
            name.as_ref(),
            state.get_inner_mut(),
        );


        match ext {
            None => {
                env.get_null().map(|null| null.into_unknown())
            }
            Some(ext) => {
                let ext = Box::into_raw(Box::new(WebGLExtension::new(Some(ext))));

                match name {
                    "ANGLE_instanced_arrays" => {
                        let ret = canvas_c::canvas_native_webgl_context_extension_to_angle_instanced_arrays(ext);
                        ANGLE_instanced_arrays(ret)
                            .into_instance(env).map(|ext| ext.as_object(env).into_unknown())
                    }
                    "EXT_blend_minmax" => {
                        if ext.is_null() {
                            return env.get_null().map(|null| null.into_unknown());
                        }
                        canvas_c::canvas_native_webgl_extension_destroy(ext);
                        EXT_blend_minmax
                            .into_instance(env).map(|ext| ext.as_object(env).into_unknown())
                    }
                    "EXT_color_buffer_half_float" => {
                        canvas_c::canvas_native_webgl_extension_destroy(ext);
                        EXT_color_buffer_half_float
                            .into_instance(env).map(|ext| ext.as_object(env).into_unknown())
                    }
                    "EXT_disjoint_timer_query" => {
                        let ext = canvas_c::canvas_native_webgl_context_extension_to_ext_disjoint_timer_query(ext);
                        EXT_disjoint_timer_query(ext)
                            .into_instance(env).map(|ext| ext.as_object(env).into_unknown())
                    }
                    "EXT_sRGB" => {
                        canvas_c::canvas_native_webgl_extension_destroy(ext);
                        EXT_sRGB
                            .into_instance(env).map(|ext| ext.as_object(env).into_unknown())
                    }
                    "EXT_shader_texture_lod" => {
                        canvas_c::canvas_native_webgl_extension_destroy(ext);
                        EXT_shader_texture_lod
                            .into_instance(env).map(|ext| ext.as_object(env).into_unknown())
                    }
                    "EXT_texture_filter_anisotropic" => {
                        EXT_texture_filter_anisotropic
                            .into_instance(env).map(|ext| ext.as_object(env).into_unknown())
                    }
                    "OES_element_index_uint" => {
                        canvas_c::canvas_native_webgl_extension_destroy(ext);
                        OES_element_index_uint
                            .into_instance(env).map(|ext| ext.as_object(env).into_unknown())
                    }
                    "OES_standard_derivatives" => {
                        canvas_c::canvas_native_webgl_extension_destroy(ext);
                        OES_standard_derivatives
                            .into_instance(env).map(|ext| ext.as_object(env).into_unknown())
                    }
                    "OES_texture_float" => {
                        canvas_c::canvas_native_webgl_extension_destroy(ext);
                        OES_texture_float
                            .into_instance(env).map(|ext| ext.as_object(env).into_unknown())
                    }
                    "OES_texture_float_linear" => {
                        canvas_c::canvas_native_webgl_extension_destroy(ext);
                        OES_texture_float_linear
                            .into_instance(env).map(|ext| ext.as_object(env).into_unknown())
                    }
                    "OES_texture_half_floatr" => {
                        canvas_c::canvas_native_webgl_extension_destroy(ext);
                        OES_texture_half_float
                            .into_instance(env).map(|ext| ext.as_object(env).into_unknown())
                    }
                    "OES_texture_half_float_linear" => {
                        canvas_c::canvas_native_webgl_extension_destroy(ext);
                        OES_texture_half_float_linear
                            .into_instance(env).map(|ext| ext.as_object(env).into_unknown())
                    }
                    "OES_vertex_array_object" => {
                        let ext = canvas_c::canvas_native_webgl_context_extension_to_oes_vertex_array_object(ext);
                        OES_vertex_array_object(ext)
                            .into_instance(env).map(|ext| ext.as_object(env).into_unknown())
                    }
                    "WEBGL_color_buffer_float" => {
                        canvas_c::canvas_native_webgl_extension_destroy(ext);
                        WEBGL_color_buffer_float
                            .into_instance(env).map(|ext| ext.as_object(env).into_unknown())
                    }
                    "WEBGL_compressed_texture_atc" => {
                        canvas_c::canvas_native_webgl_extension_destroy(ext);
                        WEBGL_compressed_texture_atc
                            .into_instance(env).map(|ext| ext.as_object(env).into_unknown())
                    }
                    "WEBGL_compressed_texture_etc" => {
                        canvas_c::canvas_native_webgl_extension_destroy(ext);
                        WEBGL_compressed_texture_etc
                            .into_instance(env).map(|ext| ext.as_object(env).into_unknown())
                    }
                    "WEBGL_compressed_texture_etc1" => {
                        canvas_c::canvas_native_webgl_extension_destroy(ext);
                        WEBGL_compressed_texture_etc1
                            .into_instance(env).map(|ext| ext.as_object(env).into_unknown())
                    }
                    "WEBGL_compressed_texture_pvrtc" => {
                        canvas_c::canvas_native_webgl_extension_destroy(ext);
                        WEBGL_compressed_texture_pvrtc
                            .into_instance(env).map(|ext| ext.as_object(env).into_unknown())
                    }
                    "WEBGL_compressed_texture_s3tc" => {
                        canvas_c::canvas_native_webgl_extension_destroy(ext);
                        WEBGL_compressed_texture_s3tc
                            .into_instance(env).map(|ext| ext.as_object(env).into_unknown())
                    }
                    "WEBGL_lose_context" => {
                        let ext = canvas_c::canvas_native_webgl_context_extension_to_lose_context(ext);
                        WEBGL_lose_context(ext)
                            .into_instance(env).map(|ext| ext.as_object(env).into_unknown())
                    }
                    "WEBGL_depth_texture" => {
                        canvas_c::canvas_native_webgl_extension_destroy(ext);
                        WEBGL_depth_texture
                            .into_instance(env).map(|ext| ext.as_object(env).into_unknown())
                    }
                    "WEBGL_draw_buffers" => {
                        let ext = canvas_c::canvas_native_webgl_context_extension_to_draw_buffers(ext);
                        WEBGL_draw_buffers(ext)
                            .into_instance(env).map(|ext| ext.as_object(env).into_unknown())
                    }
                    "OES_fbo_render_mipmap" => {
                        canvas_c::canvas_native_webgl_extension_destroy(ext);

                        OES_fbo_render_mipmap.into_instance(env).map(|ext| ext.as_object(env).into_unknown())
                    }
                    _ => env.get_null().map(|null| null.into_unknown()),
                }
            }
        }
    }

    #[napi]
    pub fn get_framebuffer_attachment_parameter(&self, env: Env, target: u32, attachment: u32, pname: u32) -> Result<Unknown> {
        let mut state = unsafe { &mut *self.state };
        let ret = canvas_webgl::webgl::canvas_native_webgl_get_framebuffer_attachment_parameter(
            target,
            attachment,
            pname,
            state.get_inner_mut(),
        );


        if ret.get_is_renderbuffer() {
            return WebGLRenderbuffer(ret.get_value() as _)
                .into_instance(env).map(|v| v.as_object(env).into_unknown());
        } else if ret.get_is_texture() {
            return WebGLTexture(ret.get_value() as _)
                .into_instance(env).map(|v| v.as_object(env).into_unknown())
        }

        env.create_int32(ret.get_value() as _).map(|v| v.into_unknown())
    }


    #[napi]
    pub fn get_parameter(&self, env: Env, pname: u32) -> Result<Unknown> {
        let mut consumed = false;
        let result = canvas_c::canvas_native_webgl_get_parameter(pname, self.state);

        let parameter = match pname {
            gl_bindings::ACTIVE_TEXTURE |
            gl_bindings::ALPHA_BITS |
            gl_bindings::ARRAY_BUFFER_BINDING |
            gl_bindings::BLEND_DST_ALPHA |
            gl_bindings::BLEND_DST_RGB |
            gl_bindings::BLEND_EQUATION |
            gl_bindings::BLEND_EQUATION_ALPHA |
            gl_bindings::BLEND_SRC_ALPHA |
            gl_bindings::BLEND_SRC_RGB |
            gl_bindings::BLUE_BITS |
            gl_bindings::CULL_FACE_MODE |
            gl_bindings::CURRENT_PROGRAM |
            gl_bindings::DEPTH_BITS |
            gl_bindings::DEPTH_FUNC |
            gl_bindings::ELEMENT_ARRAY_BUFFER_BINDING |
            gl_bindings::FRAMEBUFFER_BINDING |
            gl_bindings::FRONT_FACE |
            gl_bindings::GENERATE_MIPMAP_HINT |
            gl_bindings::GREEN_BITS |
            gl_bindings::IMPLEMENTATION_COLOR_READ_FORMAT |
            gl_bindings::IMPLEMENTATION_COLOR_READ_TYPE |
            gl_bindings::MAX_COMBINED_TEXTURE_IMAGE_UNITS |
            gl_bindings::MAX_CUBE_MAP_TEXTURE_SIZE |
            gl_bindings::MAX_FRAGMENT_UNIFORM_VECTORS |
            gl_bindings::MAX_RENDERBUFFER_SIZE |
            gl_bindings::MAX_TEXTURE_IMAGE_UNITS |
            gl_bindings::MAX_TEXTURE_SIZE |
            gl_bindings::MAX_VARYING_VECTORS |
            gl_bindings::MAX_VERTEX_ATTRIBS |
            gl_bindings::MAX_VERTEX_TEXTURE_IMAGE_UNITS |
            gl_bindings::MAX_VERTEX_UNIFORM_VECTORS |
            gl_bindings::PACK_ALIGNMENT |
            gl_bindings::RED_BITS |
            gl_bindings::RENDERBUFFER_BINDING |
            gl_bindings::SAMPLE_BUFFERS |
            gl_bindings::SAMPLES |
            gl_bindings::STENCIL_BACK_FAIL |
            gl_bindings::STENCIL_BACK_FUNC |
            gl_bindings::STENCIL_BACK_PASS_DEPTH_FAIL |
            gl_bindings::STENCIL_BACK_PASS_DEPTH_PASS |
            gl_bindings::STENCIL_BACK_REF |
            gl_bindings::STENCIL_BACK_VALUE_MASK |
            gl_bindings::STENCIL_BACK_WRITEMASK |
            gl_bindings::STENCIL_BITS |
            gl_bindings::STENCIL_CLEAR_VALUE |
            gl_bindings::STENCIL_FAIL |
            gl_bindings::STENCIL_FUNC |
            gl_bindings::STENCIL_PASS_DEPTH_FAIL |
            gl_bindings::STENCIL_PASS_DEPTH_PASS |
            gl_bindings::STENCIL_REF |
            gl_bindings::STENCIL_VALUE_MASK |
            gl_bindings::STENCIL_WRITEMASK |
            gl_bindings::SUBPIXEL_BITS |
            gl_bindings::TEXTURE_BINDING_2D |
            gl_bindings::TEXTURE_BINDING_CUBE_MAP |
            gl_bindings::UNPACK_ALIGNMENT => {
                let value = canvas_c::canvas_native_webgl_result_get_i32(result);
                if (pname == gl_bindings::CURRENT_PROGRAM || pname == gl_bindings::ARRAY_BUFFER_BINDING ||
                    pname == gl_bindings::ELEMENT_ARRAY_BUFFER_BINDING ||
                    pname == gl_bindings::TEXTURE_BINDING_2D ||
                    pname == gl_bindings::TEXTURE_BINDING_CUBE_MAP ||
                    pname == gl_bindings::RENDERBUFFER_BINDING ||
                    pname == gl_bindings::FRAMEBUFFER_BINDING) &&
                    value == 0 {
                    return env.get_null().map(|v| v.into_unknown());
                }

                env.create_int32(value).map(|v| v.into_unknown())
            }
            UNPACK_COLOR_SPACE_CONVERSION_WEBGL => {
                let ret = canvas_c::canvas_native_webgl_state_get_unpack_colorspace_conversion_webgl(self.state);
                env.create_int32(ret).map(|v| v.into_unknown())
            }
            gl_bindings::ALIASED_LINE_WIDTH_RANGE |
            gl_bindings::ALIASED_POINT_SIZE_RANGE |
            gl_bindings::BLEND_COLOR |
            gl_bindings::COLOR_CLEAR_VALUE |
            gl_bindings::DEPTH_RANGE => unsafe {
                let ret = canvas_c::canvas_native_webgl_result_into_f32_array(result);

                if ret.is_null() {
                    return env.get_null().map(|v| v.into_unknown());
                }

                let ret = *Box::from_raw(ret);
                let mut ret = ret.into_vec();

                consumed = true;

                let ptr = ret.as_mut_ptr();
                let len = ret.len();


                let buffer = env.create_arraybuffer_with_borrowed_data(ptr as _, len * size_of::<f32>(), ret, |_, _| {})?;
                buffer.value.into_typedarray(
                    TypedArrayType::Float32,
                    len,
                    0,
                ).map(|v| v.into_unknown())
            }
            UNPACK_FLIP_Y_WEBGL => {
                let ret = canvas_c::canvas_native_webgl_state_get_flip_y(self.state);
                env.get_boolean(ret).map(|v| v.into_unknown())
            }
            UNPACK_PREMULTIPLY_ALPHA_WEBGL => {
                let ret = canvas_c::canvas_native_webgl_state_get_premultiplied_alpha(self.state);
                env.get_boolean(ret).map(|v| v.into_unknown())
            }
            gl_bindings::BLEND |
            gl_bindings::CULL_FACE |
            gl_bindings::DEPTH_TEST |
            gl_bindings::DEPTH_WRITEMASK |
            gl_bindings::DITHER |
            gl_bindings::POLYGON_OFFSET_FILL |
            gl_bindings::SAMPLE_COVERAGE_INVERT |
            gl_bindings::SCISSOR_TEST |
            gl_bindings::STENCIL_TEST => {
                let ret = canvas_c::canvas_native_webgl_result_get_bool(result);
                env.get_boolean(ret).map(|v| v.into_unknown())
            }
            gl_bindings::COLOR_WRITEMASK => {
                let ret = canvas_c::canvas_native_webgl_result_get_bool_array(result);
                let len = canvas_c::canvas_native_u8_buffer_get_length(ret);
                let buf = canvas_c::canvas_native_u8_buffer_get_bytes(ret);
                let buf = unsafe { slice::from_raw_parts(buf, len) };
                let mut array = env.create_array(len as u32)?;

                for i in 0..len {
                    array.set(i as u32, buf[i] == 1)?;
                }
                array.coerce_to_object().map(|v| v.into_unknown())
            }
            gl_bindings::COMPRESSED_TEXTURE_FORMATS |
            gl_bindings::MAX_VIEWPORT_DIMS |
            gl_bindings::SCISSOR_BOX |
            gl_bindings::VIEWPORT => {
                let ret = canvas_c::canvas_native_webgl_result_into_i32_array(result);

                if ret.is_null() {
                    return env.get_null().map(|v| v.into_unknown());
                }

                let ret = unsafe { *Box::from_raw(ret) };
                let mut ret = ret.into_vec();

                consumed = true;

                let ptr = ret.as_mut_ptr();
                let len = ret.len();

                let buffer = unsafe { env.create_arraybuffer_with_borrowed_data(ptr as _, len * size_of::<i32>(), ret, |_, _| {})? };
                buffer.value.into_typedarray(
                    TypedArrayType::Int32,
                    len,
                    0,
                ).map(|v| v.into_unknown())
            }
            gl_bindings::DEPTH_CLEAR_VALUE |
            gl_bindings::LINE_WIDTH |
            gl_bindings::POLYGON_OFFSET_FACTOR |
            gl_bindings::POLYGON_OFFSET_UNITS |
            gl_bindings::SAMPLE_COVERAGE_VALUE => {
                let ret = canvas_c::canvas_native_webgl_result_get_f32(
                    result);
                env.create_double(ret as f64).map(|v| v.into_unknown())
            }
            gl_bindings::RENDERER |
            gl_bindings::SHADING_LANGUAGE_VERSION |
            gl_bindings::VENDOR |
            gl_bindings::VERSION => {
                let ret = canvas_c::canvas_native_webgl_result_get_string(result);
                if ret.is_null() {
                    return env.get_null().map(|v| v.into_unknown());
                }
                let ret = unsafe { CString::from_raw(ret as _) };
                let ret = ret.into_string().map_err(|v| Error::from_reason(v.utf8_error().to_string()))?;
                env.create_string_from_std(ret).map(|v| v.into_unknown())
            }

            _ => {
                env.get_null().map(|v| v.into_unknown())
            }
        };

        if !consumed {
            canvas_c::canvas_native_webgl_WebGLResult_destroy(result);
        }

        parameter
    }


    #[napi]
    pub fn get_program_info_log(&self, program: ClassInstance<WebGLProgram>) -> String {
        let state = unsafe { &mut *self.state };
        canvas_webgl::webgl::canvas_native_webgl_get_program_info_log(program.0, state.get_inner_mut())
    }

    #[napi]
    pub fn get_program_parameter(&self, env: Env, program: ClassInstance<WebGLProgram>, pname: u32) -> Result<Unknown> {
        let result = canvas_c::canvas_native_webgl_get_program_parameter(program.0, pname, self.state);
        if canvas_c::canvas_native_webgl_result_get_is_none(result) {
            // todo
            canvas_c::canvas_native_webgl_WebGLResult_destroy(result);
            return env.get_null().map(|v| v.into_unknown());
        }


        match pname {
            gl_bindings::DELETE_STATUS |
            gl_bindings::LINK_STATUS |
            gl_bindings::VALIDATE_STATUS => {
                let ret = canvas_c::canvas_native_webgl_result_get_bool(result);
                canvas_c::canvas_native_webgl_WebGLResult_destroy(result);
                env.get_boolean(ret).map(|v| v.into_unknown())
            }
            _ => {
                let ret = canvas_c::canvas_native_webgl_result_get_i32(result);
                canvas_c::canvas_native_webgl_WebGLResult_destroy(result);
                env.create_int32(ret).map(|v| v.into_unknown())
            }
        }
    }


    #[napi]
    pub fn get_renderbuffer_parameter(&self, target: u32, pname: u32) -> i32 {
        canvas_c::canvas_native_webgl_get_renderbuffer_parameter(target, pname, self.state)
    }

    #[napi]
    pub fn get_shader_info_log(&self, shader: ClassInstance<WebGLShader>) -> String {
        let state = unsafe { &mut *self.state };
        canvas_webgl::webgl::canvas_native_webgl_get_shader_info_log(shader.0, state.get_inner_mut())
    }

    #[napi]
    pub fn get_shader_parameter(&self, env: Env, shader: ClassInstance<WebGLShader>, pname: u32) -> Result<Unknown> {
        let result = canvas_c::canvas_native_webgl_get_shader_parameter(shader.0, pname, self.state);
        match pname {
            gl_bindings::DELETE_STATUS | gl_bindings::COMPILE_STATUS => {
                let ret = canvas_c::canvas_native_webgl_result_get_bool(result);
                canvas_c::canvas_native_webgl_WebGLResult_destroy(result);
                env.get_boolean(ret).map(|v| v.into_unknown())
            }
            _ => {
                let ret = canvas_c::canvas_native_webgl_result_get_i32(result);
                canvas_c::canvas_native_webgl_WebGLResult_destroy(result);
                env.create_int32(ret).map(|v| v.into_unknown())
            }
        }
    }

    #[napi]
    pub fn get_shader_precision_format(&self, env: Env, shader_type: u32, precision_type: u32) -> Result<ClassInstance<WebGLShaderPrecisionFormat>> {
        let precision = canvas_c::canvas_native_webgl_get_shader_precision_format(shader_type, precision_type, self.state);
        WebGLShaderPrecisionFormat(precision)
            .into_instance(env)
    }

    #[napi]
    pub fn get_shader_source(&self, shader: ClassInstance<WebGLShader>) -> String {
        let state = unsafe { &mut *self.state };
        canvas_webgl::webgl::canvas_native_webgl_get_shader_source(shader.0, state.get_inner_mut())
    }

    #[napi]
    pub fn get_supported_extensions(&self) -> Vec<String> {
        let state = unsafe { &mut *self.state };
        canvas_webgl::webgl::canvas_native_webgl_get_supported_extensions(state.get_inner_mut())
    }

    #[napi]
    pub fn get_tex_parameter(&self, target: u32, pname: u32) -> i32 {
        canvas_c::canvas_native_webgl_get_tex_parameter(target, pname, self.state)
    }

    #[napi]
    pub fn get_uniform_location(&self, env: Env, program: ClassInstance<WebGLProgram>, name: String) -> Result<Unknown> {
        let state = unsafe { &mut *self.state };
        let location = canvas_webgl::webgl::canvas_native_webgl_get_uniform_location(program.0, name.as_str(), state.get_inner_mut());

        if location == -1 {
            return env.get_null().map(|v| v.into_unknown());
        }

        webgl_uniform_location::WebGLUniformLocation(location)
            .into_instance(env)
            .map(|v| v.as_object(env).into_unknown())
    }

    #[napi]
    pub fn get_uniform(&self, env: Env, program: ClassInstance<WebGLProgram>, location: ClassInstance<WebGLUniformLocation>) -> Result<Unknown> {
        let result = canvas_c::canvas_native_webgl_get_uniform(program.0, location.0, self.state);
        let type_ = canvas_c::canvas_native_webgl_result_get_type(result);
        let uniform = match type_ {
            WebGLResultType::Boolean => {
                let ret = canvas_c::canvas_native_webgl_result_get_bool(result);
                env.get_boolean(ret).map(|v| v.into_unknown())
            }
            WebGLResultType::I32Array => unsafe {
                let ret = canvas_c::canvas_native_webgl_result_into_i32_array(result);


                if ret.is_null() {
                    return env.get_null().map(|v| v.into_unknown());
                }

                let ret = *Box::from_raw(ret);
                let mut ret = ret.into_vec();

                let ptr = ret.as_mut_ptr();
                let len = ret.len();

                let buffer = env.create_arraybuffer_with_borrowed_data(ptr as _, len * size_of::<i32>(), ret, |_, _| {})?;
                buffer.value.into_typedarray(
                    TypedArrayType::Int32,
                    len,
                    0,
                ).map(|v| v.into_unknown())
            }
            WebGLResultType::U32Array => unsafe {
                let ret = canvas_c::canvas_native_webgl_result_into_u32_array(result);


                if ret.is_null() {
                    return env.get_null().map(|v| v.into_unknown());
                }

                let ret = *Box::from_raw(ret);
                let mut ret = ret.into_vec();

                let ptr = ret.as_mut_ptr();
                let len = ret.len();

                let buffer = env.create_arraybuffer_with_borrowed_data(ptr as _, len * size_of::<u32>(), ret, |_, _| {})?;
                buffer.value.into_typedarray(
                    TypedArrayType::Uint32,
                    len,
                    0,
                ).map(|v| v.into_unknown())
            }
            WebGLResultType::F32Array => unsafe {
                let ret = canvas_c::canvas_native_webgl_result_into_f32_array(result);


                if ret.is_null() {
                    return env.get_null().map(|v| v.into_unknown());
                }

                let ret = *Box::from_raw(ret);
                let mut ret = ret.into_vec();

                let ptr = ret.as_mut_ptr();
                let len = ret.len();

                let buffer = env.create_arraybuffer_with_borrowed_data(ptr as _, len * size_of::<f32>(), ret, |_, _| {})?;
                buffer.value.into_typedarray(
                    TypedArrayType::Float32,
                    len,
                    0,
                ).map(|v| v.into_unknown())
            }
            WebGLResultType::BooleanArray => {
                let ret = canvas_c::canvas_native_webgl_result_get_bool_array(result);
                let len = canvas_c::canvas_native_u8_buffer_get_length(ret);
                let buf = canvas_c::canvas_native_u8_buffer_get_bytes(ret);
                let buf = unsafe { slice::from_raw_parts(buf, len) };
                let mut array = env.create_array(len as u32)?;

                for i in 0..len {
                    array.set(i as u32, buf[i] == 1)?;
                }
                array.coerce_to_object().map(|v| v.into_unknown())
            }
            WebGLResultType::U32 => {
                let ret = canvas_c::canvas_native_webgl_result_get_u32(result);
                env.create_uint32(ret).map(|v| v.into_unknown())
            }
            WebGLResultType::I32 => {
                let ret = canvas_c::canvas_native_webgl_result_get_i32(result);
                env.create_int32(ret).map(|v| v.into_unknown())
            }
            WebGLResultType::F32 => {
                let ret = canvas_c::canvas_native_webgl_result_get_f32(result);
                env.create_double(ret as f64).map(|v| v.into_unknown())
            }
            WebGLResultType::String => unsafe {
                let ret = canvas_c::canvas_native_webgl_result_get_string(result);
                if ret.is_null() {
                    return Err(Error::from_status(Status::GenericFailure));
                }
                let ret = CString::from_raw(ret as _);
                match ret.into_string() {
                    Ok(s) => env.create_string_from_std(s),
                    Err(_) => Err(Error::from_status(Status::GenericFailure)),
                }.map(|v| v.into_unknown())
            }
            WebGLResultType::None => {
                env.get_null().map(|v| v.into_unknown())
            }
        };

        canvas_c::canvas_native_webgl_WebGLResult_destroy(result);

        uniform
    }

    #[napi]
    pub fn get_vertex_attrib_offset(&self, index: u32, pname: u32) -> i64 {
        canvas_c::canvas_native_webgl_get_vertex_attrib_offset(index, pname, self.state) as i64
    }

    #[napi]
    pub fn get_vertex_attrib(&self, env: Env, index: u32, pname: u32) -> Result<Unknown> {
        let result = canvas_c::canvas_native_webgl_get_vertex_attrib(index, pname, self.state);

        match pname {
            gl_bindings::CURRENT_VERTEX_ATTRIB => unsafe {
                let ret = canvas_c::canvas_native_webgl_result_get_f32_array(result);
                if ret.is_null() {
                    return env.get_null().map(|v| v.into_unknown());
                }

                let ret = *Box::from_raw(ret);
                let mut ret = ret.into_vec();

                let ptr = ret.as_mut_ptr();
                let len = ret.len();

                let buffer = env.create_arraybuffer_with_borrowed_data(ptr as _, len * size_of::<f32>(), ret, |_, _| {})?;
                buffer.value.into_typedarray(
                    TypedArrayType::Float32,
                    len,
                    0,
                ).map(|v| v.into_unknown())
            }
            gl_bindings::VERTEX_ATTRIB_ARRAY_ENABLED | gl_bindings::VERTEX_ATTRIB_ARRAY_NORMALIZED => {
                let ret = canvas_c::canvas_native_webgl_result_get_bool(result);
                env.get_boolean(ret).map(|v| v.into_unknown())
            }
            _ => {
                let ret = canvas_c::canvas_native_webgl_result_get_i32(result);
                env.create_int32(ret).map(|v| v.into_unknown())
            }
        }
    }

    #[napi]
    pub fn hint(&self, target: u32, mode: u32) {
        canvas_c::canvas_native_webgl_hint(target, mode, self.state);
    }


    #[napi]
    pub fn is_buffer(&self, buffer: ClassInstance<WebGLBuffer>) -> bool {
        canvas_c::canvas_native_webgl_is_buffer(buffer.0, self.state)
    }

    #[napi]
    pub fn is_context_lost(&self) -> bool {
        canvas_c::canvas_native_webgl_get_is_context_lost(self.state)
    }

    #[napi]
    pub fn is_enabled(&self, cap: u32) -> bool {
        canvas_c::canvas_native_webgl_is_enabled(cap, self.state)
    }

    #[napi]
    pub fn is_framebuffer(&self, framebuffer: Option<ClassInstance<WebGLFramebuffer>>) -> bool {
        let framebuffer = match framebuffer {
            None => 0,
            Some(framebuffer) => framebuffer.0
        };
        canvas_c::canvas_native_webgl_is_framebuffer(framebuffer, self.state)
    }

    #[napi]
    pub fn is_program(&self, program: ClassInstance<WebGLProgram>) -> bool {
        canvas_c::canvas_native_webgl_is_program(program.0, self.state)
    }

    #[napi]
    pub fn is_renderbuffer(&self, renderbuffer: ClassInstance<WebGLRenderbuffer>) -> bool {
        canvas_c::canvas_native_webgl_is_renderbuffer(renderbuffer.0, self.state)
    }

    #[napi]
    pub fn is_shader(&self, shader: ClassInstance<WebGLShader>) -> bool {
        canvas_c::canvas_native_webgl_is_shader(shader.0, self.state)
    }

    #[napi]
    pub fn is_texture(&self, texture: ClassInstance<WebGLTexture>) -> bool {
        canvas_c::canvas_native_webgl_is_texture(texture.0, self.state)
    }

    #[napi]
    pub fn line_width(&self, width: f64) {
        canvas_c::canvas_native_webgl_line_width(width as f32, self.state);
    }

    #[napi]
    pub fn link_program(&self, program: ClassInstance<WebGLProgram>) {
        canvas_c::canvas_native_webgl_link_program(program.0, self.state)
    }

    #[napi]
    pub fn pixel_storei(&self, pname: u32, param: Either<bool, i32>) {
        match param {
            Either::A(param) => {
                let param = if param { 1 } else { 0 };
                match pname {
                    UNPACK_FLIP_Y_WEBGL | UNPACK_PREMULTIPLY_ALPHA_WEBGL => {
                        canvas_c::canvas_native_webgl_pixel_storei(pname, param, self.state)
                    }
                    _ => {}
                }
            }
            Either::B(param) => {
                match pname {
                    UNPACK_COLORSPACE_CONVERSION_WEBGL => {
                        canvas_c::canvas_native_webgl_pixel_storei(pname, 0x9244, self.state)
                    }
                    _ => {
                        canvas_c::canvas_native_webgl_pixel_storei(pname, param, self.state)
                    }
                }
            }
        }
    }


    #[napi]
    pub fn polygon_offset(&self, factor: f64, units: f64) {
        canvas_c::canvas_native_webgl_polygon_offset(factor as f32, units as f32, self.state)
    }

    #[napi(
        ts_args_type = "x: number, y: number, width: number, height: number, format: number, type: number, pixels: ArrayBuffer | ArrayBufferView"
    )]
    pub fn read_pixels(&self, x: i32, y: i32, width: i32, height: i32, format: u32, type_: u32, mut pixels: Buffer) {
        // todo
        canvas_c::canvas_native_webgl_read_pixels_u8(
            x, y, width, height, format, type_, pixels.as_mut_ptr(), pixels.len(), self.state,
        )
    }

    #[napi]
    pub fn renderbuffer_storage(&self, target: u32, internal_format: u32, width: i32, height: i32) {
        let mut internal_format = internal_format;
        if (internal_format == gl_bindings::DEPTH_STENCIL) {
            // DEPTH24_STENCIL8 = 35056
            // DEPTH24_STENCIL8_OES = 0x88F0
            internal_format = 0x88f0;
        }
        canvas_c::canvas_native_webgl_renderbuffer_storage(target, internal_format, width, height, self.state)
    }

    #[napi]
    pub fn sample_coverage(&self, value: f64, invert: bool) {
        canvas_c::canvas_native_webgl_sample_coverage(value as f32, invert, self.state);
    }

    #[napi]
    pub fn scissor(&self, x: i32, y: i32, width: i32, height: i32) {
        canvas_c::canvas_native_webgl_scissor(x, y, width, height, self.state)
    }

    #[napi]
    pub fn shader_source(&self, shader: ClassInstance<WebGLShader>, source: String) {
        let state = unsafe { &mut *self.state };
        canvas_webgl::webgl::canvas_native_webgl_shader_source(shader.0, source.as_str(), state.get_inner_mut());
    }

    #[napi(ts_args_type = "face:number, func: number, ref: number, mask: number")]
    pub fn stencil_func_separate(&self, face: u32, func: u32, ref_: i32, mask: u32) {
        canvas_c::canvas_native_webgl_stencil_func_separate(face, func, ref_, mask, self.state)
    }

    #[napi(ts_args_type = "func: number, ref: number, mask: number")]
    pub fn stencil_func(&self, func: u32, ref_: i32, mask: u32) {
        canvas_c::canvas_native_webgl_stencil_func(func, ref_, mask, self.state)
    }

    #[napi]
    pub fn stencil_mask_separate(&self, face: u32, mask: u32) {
        canvas_c::canvas_native_webgl_stencil_mask_separate(face, mask, self.state);
    }

    #[napi]
    pub fn stencil_mask(&self, mask: u32) {
        canvas_c::canvas_native_webgl_stencil_mask(mask, self.state);
    }

    #[napi]
    pub fn stencil_op_separate(&self, face: u32, fail: u32, zfail: u32, zpass: u32) {
        canvas_c::canvas_native_webgl_stencil_op_separate(face, fail, zfail, zpass, self.state);
    }

    #[napi]
    pub fn stencil_op(&self, fail: u32, zfail: u32, zpass: u32) {
        canvas_c::canvas_native_webgl_stencil_op(fail, zfail, zpass, self.state);
    }


    // texImage2D(target: number, level: number, internalformat: number, format: number, type: number, pixels: any): void;
    //
    // texImage2D(target: number, level: number, internalformat: number, width: number, height: number, border: number, format: number, type: number, pixels: ArrayBufferView): void;

    #[napi]
    pub fn tex_image_2D(&self, target: i32, level: i32, internalformat: i32, width_or_format: i32, height_or_type: i32, border_or_pixels: Either4<i32, ClassInstance<crate::c2d::CanvasRenderingContext2D>, ClassInstance<web_g_l_rendering_context>, ClassInstance<crate::image_asset::ImageAsset>>, format: Option<i32>, type_: Option<i32>, pixels: Option<Either<Buffer, i64>>, offset: Option<i64>) -> Result<()> {
        match border_or_pixels {
            Either4::A(border) => {
                match (format, type_, pixels) {
                    (Some(format), Some(type_), Some(pixels)) => {
                        match pixels {
                            Either::A(buffer) => {
                                canvas_c::canvas_native_webgl_tex_image2d(
                                    target, level, internalformat, width_or_format, height_or_type, border, format, type_, buffer.as_ptr(), buffer.len(), self.state,
                                )
                            }
                            Either::B(offset) => {
                                canvas_c::canvas_native_webgl2_tex_image2d_offset(
                                    target, level, internalformat, width_or_format as u32, height_or_type as u32, border, format, type_, offset as u64, self.state,
                                )
                            }
                        }
                    }
                    (Some(format), Some(type_), None) => {
                        canvas_c::canvas_native_webgl_tex_image2d_none(
                            target, level, internalformat, width_or_format, height_or_type, border, format, type_, self.state,
                        )
                    }
                    _ => {}
                }
            }
            Either4::B(c2d) => {
                canvas_c::canvas_native_webgl_tex_image2d_canvas2d(
                    target, level, internalformat, width_or_format, height_or_type, c2d.context, self.state,
                )
            }
            Either4::C(gl) => {
                canvas_c::canvas_native_webgl_tex_image2d_webgl(
                    target, level, internalformat, width_or_format, height_or_type, gl.state, self.state,
                )
            }
            Either4::D(image) => {
                canvas_c::canvas_native_webgl_tex_image2d_image_asset(
                    target, level, internalformat, width_or_format, height_or_type, Arc::as_ptr(&image.asset), self.state,
                )
            }
        }
        Ok(())
    }


    #[napi]
    pub fn tex_parameterf(&self, target: u32, pname: u32, param: f64) {
        canvas_c::canvas_native_webgl_tex_parameterf(target, pname, param as f32, self.state);
    }

    #[napi]
    pub fn tex_parameteri(&self, target: u32, pname: u32, param: i32) {
        canvas_c::canvas_native_webgl_tex_parameteri(target, pname, param, self.state);
    }


    #[napi(js_name = "uniform1f")]
    pub fn uniform1f(&self, location: ClassInstance<WebGLUniformLocation>, v0: f64) {
        canvas_c::canvas_native_webgl_uniform1f(location.0, v0 as f32, self.state);
    }


    #[napi(js_name = "uniform1iv")]
    pub fn uniform1iv(&self, location: ClassInstance<WebGLUniformLocation>, value: Either<Vec<i32>, Int32Array>) {
        match value {
            Either::A(array) => {
                canvas_c::canvas_native_webgl_uniform1iv(location.0, array.as_ptr(), array.len(), self.state);
            }
            Either::B(buffer) => {
                canvas_c::canvas_native_webgl_uniform1iv(location.0, buffer.as_ptr(), buffer.len(), self.state);
            }
        }
    }

    #[napi(js_name = "uniform1fv")]
    pub fn uniform1fv(&self, location: ClassInstance<WebGLUniformLocation>, value: Either<Vec<f64>, Float32Array>) {
        match value {
            Either::A(array) => {
                let array = array.into_iter().map(|v| v as f32).collect::<Vec<f32>>();
                canvas_c::canvas_native_webgl_uniform1fv(location.0, array.as_ptr(), array.len(), self.state);
            }
            Either::B(buffer) => {
                canvas_c::canvas_native_webgl_uniform1fv(location.0, buffer.as_ptr(), buffer.len(), self.state);
            }
        }
    }

    #[napi(js_name = "uniform1i")]
    pub fn uniform1i(&self, location: ClassInstance<WebGLUniformLocation>, v0: i32) {
        canvas_c::canvas_native_webgl_uniform1i(location.0, v0, self.state);
    }

    #[napi(js_name = "uniform2f")]
    pub fn uniform2f(&self, location: ClassInstance<WebGLUniformLocation>, v0: f64, v1: f64) {
        canvas_c::canvas_native_webgl_uniform2f(location.0, v0 as f32, v1 as f32, self.state);
    }

    #[napi(js_name = "uniform2iv")]
    pub fn uniform2iv(&self, location: ClassInstance<WebGLUniformLocation>, value: Either<Vec<i32>, Int32Array>) {
        match value {
            Either::A(array) => {
                canvas_c::canvas_native_webgl_uniform2iv(location.0, array.as_ptr(), array.len(), self.state);
            }
            Either::B(buffer) => {
                canvas_c::canvas_native_webgl_uniform2iv(location.0, buffer.as_ptr(), buffer.len(), self.state);
            }
        }
    }

    #[napi(js_name = "uniform2fv")]
    pub fn uniform2fv(&self, location: ClassInstance<WebGLUniformLocation>, value: Either<Vec<f64>, Float32Array>) {
        match value {
            Either::A(array) => {
                let array = array.into_iter().map(|v| v as f32).collect::<Vec<f32>>();
                canvas_c::canvas_native_webgl_uniform2fv(location.0, array.as_ptr(), array.len(), self.state);
            }
            Either::B(buffer) => {
                canvas_c::canvas_native_webgl_uniform2fv(location.0, buffer.as_ptr(), buffer.len(), self.state);
            }
        }
    }

    #[napi(js_name = "uniform2i")]
    pub fn uniform2i(&self, location: ClassInstance<WebGLUniformLocation>, v0: i32, v1: i32) {
        canvas_c::canvas_native_webgl_uniform2i(location.0, v0, v1, self.state);
    }

    #[napi(js_name = "uniform3f")]
    pub fn uniform3f(&self, location: ClassInstance<WebGLUniformLocation>, v0: f64, v1: f64, v2: f64) {
        canvas_c::canvas_native_webgl_uniform3f(location.0, v0 as f32, v1 as f32, v2 as f32, self.state);
    }

    #[napi(js_name = "uniform3iv")]
    pub fn uniform3iv(&self, location: ClassInstance<WebGLUniformLocation>, value: Either<Vec<i32>, Int32Array>) {
        match value {
            Either::A(array) => {
                canvas_c::canvas_native_webgl_uniform3iv(location.0, array.as_ptr(), array.len(), self.state);
            }
            Either::B(buffer) => {
                canvas_c::canvas_native_webgl_uniform3iv(location.0, buffer.as_ptr(), buffer.len(), self.state);
            }
        }
    }

    #[napi(js_name = "uniform3fv")]
    pub fn uniform3fv(&self, location: ClassInstance<WebGLUniformLocation>, value: Either<Vec<f64>, Float32Array>) {
        match value {
            Either::A(array) => {
                let array = array.into_iter().map(|v| v as f32).collect::<Vec<f32>>();
                canvas_c::canvas_native_webgl_uniform3fv(location.0, array.as_ptr(), array.len(), self.state);
            }
            Either::B(buffer) => {
                canvas_c::canvas_native_webgl_uniform3fv(location.0, buffer.as_ptr(), buffer.len(), self.state);
            }
        }
    }

    #[napi(js_name = "uniform3i")]
    pub fn uniform3i(&self, location: ClassInstance<WebGLUniformLocation>, v0: i32, v1: i32, v2: i32) {
        canvas_c::canvas_native_webgl_uniform3i(location.0, v0, v1, v2, self.state);
    }

    #[napi(js_name = "uniform4f")]
    pub fn uniform4f(&self, location: ClassInstance<WebGLUniformLocation>, v0: f64, v1: f64, v2: f64, v3: f64) {
        canvas_c::canvas_native_webgl_uniform4f(location.0, v0 as f32, v1 as f32, v2 as f32, v3 as f32, self.state);
    }

    #[napi(js_name = "uniform4iv")]
    pub fn uniform4iv(&self, location: ClassInstance<WebGLUniformLocation>, value: Either<Vec<i32>, Int32Array>) {
        match value {
            Either::A(array) => {
                canvas_c::canvas_native_webgl_uniform4iv(location.0, array.as_ptr(), array.len(), self.state);
            }
            Either::B(buffer) => {
                canvas_c::canvas_native_webgl_uniform4iv(location.0, buffer.as_ptr(), buffer.len(), self.state);
            }
        }
    }

    #[napi(js_name = "uniform4fv")]
    pub fn uniform4fv(&self, location: ClassInstance<WebGLUniformLocation>, value: Either<Vec<f64>, Float32Array>) {
        match value {
            Either::A(array) => {
                let array = array.into_iter().map(|v| v as f32).collect::<Vec<f32>>();
                canvas_c::canvas_native_webgl_uniform4fv(location.0, array.as_ptr(), array.len(), self.state);
            }
            Either::B(buffer) => {
                canvas_c::canvas_native_webgl_uniform4fv(location.0, buffer.as_ptr(), buffer.len(), self.state);
            }
        }
    }

    #[napi(js_name = "uniform4i")]
    pub fn uniform4i(&self, location: ClassInstance<WebGLUniformLocation>, v0: i32, v1: i32, v2: i32, v3: i32) {
        canvas_c::canvas_native_webgl_uniform4i(location.0, v0, v1, v2, v3, self.state);
    }

    #[napi(js_name = "uniformMatrix2fv")]
    pub fn uniform_matrix2fv(&self, location: ClassInstance<WebGLUniformLocation>, transpose: bool, value: Either<Vec<f64>, Float32Array>) {
        match value {
            Either::A(array) => {
                let array = array.into_iter().map(|v| v as f32).collect::<Vec<f32>>();
                canvas_c::canvas_native_webgl_uniform_matrix2fv(location.0, transpose, array.as_ptr(), array.len(), self.state);
            }
            Either::B(buffer) => {
                canvas_c::canvas_native_webgl_uniform_matrix2fv(location.0, transpose, buffer.as_ptr(), buffer.len(), self.state);
            }
        }
    }

    #[napi(js_name = "uniformMatrix3fv")]
    pub fn uniform_matrix3fv(&self, location: ClassInstance<WebGLUniformLocation>, transpose: bool, value: Either<Vec<f64>, Float32Array>) {
        match value {
            Either::A(array) => {
                let array = array.into_iter().map(|v| v as f32).collect::<Vec<f32>>();
                canvas_c::canvas_native_webgl_uniform_matrix3fv(location.0, transpose, array.as_ptr(), array.len(), self.state);
            }
            Either::B(buffer) => {
                canvas_c::canvas_native_webgl_uniform_matrix3fv(location.0, transpose, buffer.as_ptr(), buffer.len(), self.state);
            }
        }
    }

    #[napi(js_name = "uniformMatrix4fv")]
    pub fn uniform_matrix4fv(&self, location: ClassInstance<WebGLUniformLocation>, transpose: bool, value: Either<Vec<f64>, Float32Array>) {
        match value {
            Either::A(array) => {
                let array = array.into_iter().map(|v| v as f32).collect::<Vec<f32>>();
                canvas_c::canvas_native_webgl_uniform_matrix4fv(location.0, transpose, array.as_ptr(), array.len(), self.state);
            }
            Either::B(buffer) => {
                canvas_c::canvas_native_webgl_uniform_matrix4fv(location.0, transpose, buffer.as_ptr(), buffer.len(), self.state);
            }
        }
    }

    #[napi]
    pub fn use_program(&self, program: Option<ClassInstance<WebGLProgram>>) {
        let program = program.map(|p| p.0).unwrap_or(0);
        canvas_c::canvas_native_webgl_use_program(program, self.state)
    }

    #[napi]
    pub fn validate_program(&self, program: ClassInstance<WebGLProgram>) {
        canvas_c::canvas_native_webgl_validate_program(program.0, self.state)
    }

    #[napi(js_name = "vertexAttrib1f")]
    pub fn vertex_attrib1f(&self, index: u32, v0: f64) {
        canvas_c::canvas_native_webgl_vertex_attrib1f(index, v0 as f32, self.state);
    }

    #[napi(js_name = "vertexAttrib1fv")]
    pub fn vertex_attrib1fv(&self, index: u32, value: Either<Vec<f64>, Float32Array>) {
        match value {
            Either::A(array) => {
                let array = array.into_iter().map(|v| v as f32).collect::<Vec<f32>>();
                canvas_c::canvas_native_webgl_vertex_attrib1fv(index, array.as_ptr(), array.len(), self.state);
            }
            Either::B(buffer) => {
                canvas_c::canvas_native_webgl_vertex_attrib1fv(index, buffer.as_ptr(), buffer.len(), self.state);
            }
        }
    }


    #[napi(js_name = "vertexAttrib2f")]
    pub fn vertex_attrib2f(&self, index: u32, v0: f64, v1: f64) {
        canvas_c::canvas_native_webgl_vertex_attrib2f(index, v0 as f32, v1 as f32, self.state);
    }

    #[napi(js_name = "vertexAttrib2fv")]
    pub fn vertex_attrib2fv(&self, index: u32, value: Either<Vec<f64>, Float32Array>) {
        match value {
            Either::A(array) => {
                let array = array.into_iter().map(|v| v as f32).collect::<Vec<f32>>();
                canvas_c::canvas_native_webgl_vertex_attrib2fv(index, array.as_ptr(), array.len(), self.state);
            }
            Either::B(buffer) => {
                canvas_c::canvas_native_webgl_vertex_attrib2fv(index, buffer.as_ptr(), buffer.len(), self.state);
            }
        }
    }

    #[napi(js_name = "vertexAttrib3f")]
    pub fn vertex_attrib3f(&self, index: u32, v0: f64, v1: f64, v2: f64) {
        canvas_c::canvas_native_webgl_vertex_attrib3f(index, v0 as f32, v1 as f32, v2 as f32, self.state);
    }

    #[napi(js_name = "vertexAttrib3fv")]
    pub fn vertex_attrib3fv(&self, index: u32, value: Either<Vec<f64>, Float32Array>) {
        match value {
            Either::A(array) => {
                let array = array.into_iter().map(|v| v as f32).collect::<Vec<f32>>();
                canvas_c::canvas_native_webgl_vertex_attrib3fv(index, array.as_ptr(), array.len(), self.state);
            }
            Either::B(buffer) => {
                canvas_c::canvas_native_webgl_vertex_attrib3fv(index, buffer.as_ptr(), buffer.len(), self.state);
            }
        }
    }

    #[napi(js_name = "vertexAttrib4f")]
    pub fn vertex_attrib4f(&self, index: u32, v0: f64, v1: f64, v2: f64, v3: f64) {
        canvas_c::canvas_native_webgl_vertex_attrib4f(index, v0 as f32, v1 as f32, v2 as f32, v3 as f32, self.state);
    }

    #[napi(js_name = "vertexAttrib4fv")]
    pub fn vertex_attrib4fv(&self, index: u32, value: Either<Vec<f64>, Float32Array>) {
        match value {
            Either::A(array) => {
                let array = array.into_iter().map(|v| v as f32).collect::<Vec<f32>>();
                canvas_c::canvas_native_webgl_vertex_attrib4fv(index, array.as_ptr(), array.len(), self.state);
            }
            Either::B(buffer) => {
                canvas_c::canvas_native_webgl_vertex_attrib4fv(index, buffer.as_ptr(), buffer.len(), self.state);
            }
        }
    }

    #[napi(
        ts_args_type = "index: number, size: number, type: number, normalized: boolean, stride: number, offset: number"
    )]
    pub fn vertex_attrib_pointer(&self, index: u32, size: i32, type_: u32, normalized: bool, stride: i32, offset: i64) {
        canvas_c::canvas_native_webgl_vertex_attrib_pointer(
            index, size, type_, normalized, stride, offset as isize, self.state,
        )
    }

    #[napi]
    pub fn viewport(&self, x: i32, y: i32, width: i32, height: i32) {
        canvas_c::canvas_native_webgl_viewport(x, y, width, height, self.state);
    }


    #[napi(js_name = "toDataURL")]
    pub fn to_data_url(&self, format: Option<String>, encoderOptions: Option<f64>) -> String {
        let c_str = CString::new(format.unwrap_or("image/png".to_string())).unwrap();
        let quality = encoderOptions
            .map(|v| v as f32)
            .unwrap_or(0.92)
            .try_into()
            .unwrap_or(0.92);
        let quality: u32 = (quality * 100.) as u32;
        let ret = canvas_c::canvas_native_webgl_to_data_url(self.state, c_str.as_ptr(), quality);
        unsafe { CString::from_raw(ret as _).to_string_lossy().to_string() }
    }

    #[napi(getter, js_name = "DEPTH_BUFFER_BIT")]
    pub fn DEPTH_BUFFER_BIT(&self) -> u32 { 0x00000100 }

    #[napi(getter, js_name = "STENCIL_BUFFER_BIT")]
    pub fn STENCIL_BUFFER_BIT(&self) -> u32 { 0x00000400 }

    #[napi(getter, js_name = "COLOR_BUFFER_BIT")]
    pub fn COLOR_BUFFER_BIT(&self) -> u32 { 0x00004000 }

    #[napi(getter, js_name = "POINTS")]
    pub fn POINTS(&self) -> u32 { 0x0000 }

    #[napi(getter, js_name = "LINES")]
    pub fn LINES(&self) -> u32 { 0x0001 }

    #[napi(getter, js_name = "LINE_LOOP")]
    pub fn LINE_LOOP(&self) -> u32 { 0x0002 }

    #[napi(getter, js_name = "LINE_STRIP")]
    pub fn LINE_STRIP(&self) -> u32 { 0x0003 }

    #[napi(getter, js_name = "TRIANGLES")]
    pub fn TRIANGLES(&self) -> u32 { 0x0004 }

    #[napi(getter, js_name = "TRIANGLE_STRIP")]
    pub fn TRIANGLE_STRIP(&self) -> u32 { 0x0005 }

    #[napi(getter, js_name = "TRIANGLE_FAN")]
    pub fn TRIANGLE_FAN(&self) -> u32 { 0x0006 }

    #[napi(getter, js_name = "ZERO")]
    pub fn ZERO(&self) -> u32 { 0 }

    #[napi(getter, js_name = "ONE")]
    pub fn ONE(&self) -> u32 { 1 }

    #[napi(getter, js_name = "SRC_COLOR")]
    pub fn SRC_COLOR(&self) -> u32 { 0x0300 }

    #[napi(getter, js_name = "ONE_MINUS_SRC_COLOR")]
    pub fn ONE_MINUS_SRC_COLOR(&self) -> u32 { 0x0301 }

    #[napi(getter, js_name = "SRC_ALPHA")]
    pub fn SRC_ALPHA(&self) -> u32 { 0x0302 }

    #[napi(getter, js_name = "ONE_MINUS_SRC_ALPHA")]
    pub fn ONE_MINUS_SRC_ALPHA(&self) -> u32 { 0x0303 }

    #[napi(getter, js_name = "DST_ALPHA")]
    pub fn DST_ALPHA(&self) -> u32 { 0x0304 }

    #[napi(getter, js_name = "ONE_MINUS_DST_ALPHA")]
    pub fn ONE_MINUS_DST_ALPHA(&self) -> u32 { 0x0305 }

    #[napi(getter, js_name = "DST_COLOR")]
    pub fn DST_COLOR(&self) -> u32 { 0x0306 }

    #[napi(getter, js_name = "ONE_MINUS_DST_COLOR")]
    pub fn ONE_MINUS_DST_COLOR(&self) -> u32 { 0x0307 }

    #[napi(getter, js_name = "SRC_ALPHA_SATURATE")]
    pub fn SRC_ALPHA_SATURATE(&self) -> u32 { 0x0308 }

    #[napi(getter, js_name = "CONSTANT_COLOR")]
    pub fn CONSTANT_COLOR(&self) -> u32 { 0x8001 }

    #[napi(getter, js_name = "ONE_MINUS_CONSTANT_COLOR")]
    pub fn ONE_MINUS_CONSTANT_COLOR(&self) -> u32 { 0x8002 }

    #[napi(getter, js_name = "CONSTANT_ALPHA")]
    pub fn CONSTANT_ALPHA(&self) -> u32 { 0x8003 }

    #[napi(getter, js_name = "ONE_MINUS_CONSTANT_ALPHA")]
    pub fn ONE_MINUS_CONSTANT_ALPHA(&self) -> u32 { 0x8004 }

    /* Blending equations */
    #[napi(getter, js_name = "FUNC_ADD")]
    pub fn FUNC_ADD(&self) -> u32 { 0x8006 }

    #[napi(getter, js_name = "FUNC_SUBTRACT")]
    pub fn FUNC_SUBTRACT(&self) -> u32 { 0x800A }

    #[napi(getter, js_name = "FUNC_REVERSE_SUBTRACT")]
    pub fn FUNC_REVERSE_SUBTRACT(&self) -> u32 { 0x800B }

    #[napi(getter, js_name = "BLEND_EQUATION")]
    pub fn BLEND_EQUATION(&self) -> u32 { 0x8009 }

    #[napi(getter, js_name = "BLEND_EQUATION_RGB")]
    pub fn BLEND_EQUATION_RGB(&self) -> u32 { 0x8009 }

    #[napi(getter, js_name = "BLEND_EQUATION_ALPHA")]
    pub fn BLEND_EQUATION_ALPHA(&self) -> u32 { 0x883D }

    #[napi(getter, js_name = "BLEND_DST_RGB")]
    pub fn BLEND_DST_RGB(&self) -> u32 { 0x80C8 }

    #[napi(getter, js_name = "BLEND_SRC_RGB")]
    pub fn BLEND_SRC_RGB(&self) -> u32 { 0x80C9 }

    #[napi(getter, js_name = "BLEND_DST_ALPHA")]
    pub fn BLEND_DST_ALPHA(&self) -> u32 { 0x80CA }

    #[napi(getter, js_name = "BLEND_SRC_ALPHA")]
    pub fn BLEND_SRC_ALPHA(&self) -> u32 { 0x80CB }

    #[napi(getter, js_name = "BLEND_COLOR")]
    pub fn BLEND_COLOR(&self) -> u32 { 0x8005 }

    #[napi(getter, js_name = "ARRAY_BUFFER_BINDING")]
    pub fn ARRAY_BUFFER_BINDING(&self) -> u32 { 0x8894 }

    #[napi(getter, js_name = "ELEMENT_ARRAY_BUFFER_BINDING")]
    pub fn ELEMENT_ARRAY_BUFFER_BINDING(&self) -> u32 { 0x8895 }

    #[napi(getter, js_name = "LINE_WIDTH")]
    pub fn LINE_WIDTH(&self) -> u32 { 0x0B21 }

    #[napi(getter, js_name = "ALIASED_POINT_SIZE_RANGE")]
    pub fn ALIASED_POINT_SIZE_RANGE(&self) -> u32 { 0x846D }

    #[napi(getter, js_name = "ALIASED_LINE_WIDTH_RANGE")]
    pub fn ALIASED_LINE_WIDTH_RANGE(&self) -> u32 { 0x846E }

    #[napi(getter, js_name = "CULL_FACE_MODE")]
    pub fn CULL_FACE_MODE(&self) -> u32 { 0x0B45 }

    #[napi(getter, js_name = "FRONT_FACE")]
    pub fn FRONT_FACE(&self) -> u32 { 0x0B46 }

    #[napi(getter, js_name = "DEPTH_RANGE")]
    pub fn DEPTH_RANGE(&self) -> u32 { 0x0B70 }

    #[napi(getter, js_name = "DEPTH_WRITEMASK")]
    pub fn DEPTH_WRITEMASK(&self) -> u32 { 0x0B72 }

    #[napi(getter, js_name = "DEPTH_CLEAR_VALUE")]
    pub fn DEPTH_CLEAR_VALUE(&self) -> u32 { 0x0B73 }

    #[napi(getter, js_name = "DEPTH_FUNC")]
    pub fn DEPTH_FUNC(&self) -> u32 { 0x0B74 }

    #[napi(getter, js_name = "STENCIL_CLEAR_VALUE")]
    pub fn STENCIL_CLEAR_VALUE(&self) -> u32 { 0x0B91 }

    #[napi(getter, js_name = "STENCIL_FUNC")]
    pub fn STENCIL_FUNC(&self) -> u32 { 0x0B92 }

    #[napi(getter, js_name = "STENCIL_FAIL")]
    pub fn STENCIL_FAIL(&self) -> u32 { 0x0B94 }

    #[napi(getter, js_name = "STENCIL_PASS_DEPTH_FAIL")]
    pub fn STENCIL_PASS_DEPTH_FAIL(&self) -> u32 { 0x0B95 }

    #[napi(getter, js_name = "STENCIL_PASS_DEPTH_PASS")]
    pub fn STENCIL_PASS_DEPTH_PASS(&self) -> u32 { 0x0B96 }

    #[napi(getter, js_name = "STENCIL_REF")]
    pub fn STENCIL_REF(&self) -> u32 { 0x0B97 }

    #[napi(getter, js_name = "STENCIL_VALUE_MASK")]
    pub fn STENCIL_VALUE_MASK(&self) -> u32 { 0x0B93 }

    #[napi(getter, js_name = "STENCIL_WRITEMASK")]
    pub fn STENCIL_WRITEMASK(&self) -> u32 { 0x0B98 }

    #[napi(getter, js_name = "STENCIL_BACK_FUNC")]
    pub fn STENCIL_BACK_FUNC(&self) -> u32 { 0x8800 }

    #[napi(getter, js_name = "STENCIL_BACK_FAIL")]
    pub fn STENCIL_BACK_FAIL(&self) -> u32 { 0x8801 }

    #[napi(getter, js_name = "STENCIL_BACK_PASS_DEPTH_FAIL")]
    pub fn STENCIL_BACK_PASS_DEPTH_FAIL(&self) -> u32 { 0x8802 }

    #[napi(getter, js_name = "STENCIL_BACK_PASS_DEPTH_PASS")]
    pub fn STENCIL_BACK_PASS_DEPTH_PASS(&self) -> u32 { 0x8803 }

    #[napi(getter, js_name = "STENCIL_BACK_REF")]
    pub fn STENCIL_BACK_REF(&self) -> u32 { 0x8CA3 }

    #[napi(getter, js_name = "STENCIL_BACK_VALUE_MASK")]
    pub fn STENCIL_BACK_VALUE_MASK(&self) -> u32 { 0x8CA4 }

    #[napi(getter, js_name = "STENCIL_BACK_WRITEMASK")]
    pub fn STENCIL_BACK_WRITEMASK(&self) -> u32 { 0x8CA5 }

    #[napi(getter, js_name = "VIEWPORT")]
    pub fn VIEWPORT(&self) -> u32 { 0x0BA2 }

    #[napi(getter, js_name = "SCISSOR_BOX")]
    pub fn SCISSOR_BOX(&self) -> u32 { 0x0C10 }

    #[napi(getter, js_name = "COLOR_CLEAR_VALUE")]
    pub fn COLOR_CLEAR_VALUE(&self) -> u32 { 0x0C22 }

    #[napi(getter, js_name = "COLOR_WRITEMASK")]
    pub fn COLOR_WRITEMASK(&self) -> u32 { 0x0C23 }

    #[napi(getter, js_name = "UNPACK_ALIGNMENT")]
    pub fn UNPACK_ALIGNMENT(&self) -> u32 { 0x0CF5 }

    #[napi(getter, js_name = "PACK_ALIGNMENT")]
    pub fn PACK_ALIGNMENT(&self) -> u32 { 0x0D05 }

    #[napi(getter, js_name = "MAX_TEXTURE_SIZE")]
    pub fn MAX_TEXTURE_SIZE(&self) -> u32 { 0x0D33 }

    #[napi(getter, js_name = "MAX_VIEWPORT_DIMS")]
    pub fn MAX_VIEWPORT_DIMS(&self) -> u32 { 0x0D3A }

    #[napi(getter, js_name = "SUBPIXEL_BITS")]
    pub fn SUBPIXEL_BITS(&self) -> u32 { 0x0D50 }

    #[napi(getter, js_name = "RED_BITS")]
    pub fn RED_BITS(&self) -> u32 { 0x0D52 }

    #[napi(getter, js_name = "GREEN_BITS")]
    pub fn GREEN_BITS(&self) -> u32 { 0x0D53 }

    #[napi(getter, js_name = "BLUE_BITS")]
    pub fn BLUE_BITS(&self) -> u32 { 0x0D54 }

    #[napi(getter, js_name = "ALPHA_BITS")]
    pub fn ALPHA_BITS(&self) -> u32 { 0x0D55 }

    #[napi(getter, js_name = "DEPTH_BITS")]
    pub fn DEPTH_BITS(&self) -> u32 { 0x0D56 }

    #[napi(getter, js_name = "STENCIL_BITS")]
    pub fn STENCIL_BITS(&self) -> u32 { 0x0D57 }

    #[napi(getter, js_name = "POLYGON_OFFSET_UNITS")]
    pub fn POLYGON_OFFSET_UNITS(&self) -> u32 { 0x2A00 }

    #[napi(getter, js_name = "POLYGON_OFFSET_FACTOR")]
    pub fn POLYGON_OFFSET_FACTOR(&self) -> u32 { 0x8038 }

    #[napi(getter, js_name = "TEXTURE_BINDING_2D")]
    pub fn TEXTURE_BINDING_2D(&self) -> u32 { 0x8069 }

    #[napi(getter, js_name = "SAMPLE_BUFFERS")]
    pub fn SAMPLE_BUFFERS(&self) -> u32 { 0x80A8 }

    #[napi(getter, js_name = "SAMPLES")]
    pub fn SAMPLES(&self) -> u32 { 0x80A9 }

    #[napi(getter, js_name = "SAMPLE_COVERAGE_VALUE")]
    pub fn SAMPLE_COVERAGE_VALUE(&self) -> u32 { 0x80AA }

    #[napi(getter, js_name = "SAMPLE_COVERAGE_INVERT")]
    pub fn SAMPLE_COVERAGE_INVERT(&self) -> u32 { 0x80AB }

    #[napi(getter, js_name = "COMPRESSED_TEXTURE_FORMATS")]
    pub fn COMPRESSED_TEXTURE_FORMATS(&self) -> u32 { 0x86A3 }

    #[napi(getter, js_name = "VENDOR")]
    pub fn VENDOR(&self) -> u32 { 0x1F00 }

    #[napi(getter, js_name = "RENDERER")]
    pub fn RENDERER(&self) -> u32 { 0x1F01 }

    #[napi(getter, js_name = "VERSION")]
    pub fn VERSION(&self) -> u32 { 0x1F02 }

    #[napi(getter, js_name = "IMPLEMENTATION_COLOR_READ_TYPE")]
    pub fn IMPLEMENTATION_COLOR_READ_TYPE(&self) -> u32 { 0x8B9A }

    #[napi(getter, js_name = "IMPLEMENTATION_COLOR_READ_FORMAT")]
    pub fn IMPLEMENTATION_COLOR_READ_FORMAT(&self) -> u32 { 0x8B9B }

    #[napi(getter, js_name = "BROWSER_DEFAULT_WEBGL")]
    pub fn BROWSER_DEFAULT_WEBGL(&self) -> u32 { 0x9244 }

    #[napi(getter, js_name = "STATIC_DRAW")]
    pub fn STATIC_DRAW(&self) -> u32 { 0x88E4 }

    #[napi(getter, js_name = "STREAM_DRAW")]
    pub fn STREAM_DRAW(&self) -> u32 { 0x88E0 }

    #[napi(getter, js_name = "DYNAMIC_DRAW")]
    pub fn DYNAMIC_DRAW(&self) -> u32 { 0x88E8 }

    #[napi(getter, js_name = "ARRAY_BUFFER")]
    pub fn ARRAY_BUFFER(&self) -> u32 { 0x8892 }

    #[napi(getter, js_name = "ELEMENT_ARRAY_BUFFER")]
    pub fn ELEMENT_ARRAY_BUFFER(&self) -> u32 { 0x8893 }

    #[napi(getter, js_name = "BUFFER_SIZE")]
    pub fn BUFFER_SIZE(&self) -> u32 { 0x8764 }

    #[napi(getter, js_name = "BUFFER_USAGE")]
    pub fn BUFFER_USAGE(&self) -> u32 { 0x8765 }

    #[napi(getter, js_name = "CURRENT_VERTEX_ATTRIB")]
    pub fn CURRENT_VERTEX_ATTRIB(&self) -> u32 { 0x8626 }

    #[napi(getter, js_name = "VERTEX_ATTRIB_ARRAY_ENABLED")]
    pub fn VERTEX_ATTRIB_ARRAY_ENABLED(&self) -> u32 { 0x8622 }

    #[napi(getter, js_name = "VERTEX_ATTRIB_ARRAY_SIZE")]
    pub fn VERTEX_ATTRIB_ARRAY_SIZE(&self) -> u32 { 0x8623 }

    #[napi(getter, js_name = "VERTEX_ATTRIB_ARRAY_STRIDE")]
    pub fn VERTEX_ATTRIB_ARRAY_STRIDE(&self) -> u32 { 0x8624 }

    #[napi(getter, js_name = "VERTEX_ATTRIB_ARRAY_TYPE")]
    pub fn VERTEX_ATTRIB_ARRAY_TYPE(&self) -> u32 { 0x8625 }

    #[napi(getter, js_name = "VERTEX_ATTRIB_ARRAY_NORMALIZED")]
    pub fn VERTEX_ATTRIB_ARRAY_NORMALIZED(&self) -> u32 { 0x886A }

    #[napi(getter, js_name = "VERTEX_ATTRIB_ARRAY_POINTER")]
    pub fn VERTEX_ATTRIB_ARRAY_POINTER(&self) -> u32 { 0x8645 }

    #[napi(getter, js_name = "VERTEX_ATTRIB_ARRAY_BUFFER_BINDING")]
    pub fn VERTEX_ATTRIB_ARRAY_BUFFER_BINDING(&self) -> u32 { 0x889F }

    #[napi(getter, js_name = "CULL_FACE")]
    pub fn CULL_FACE(&self) -> u32 { 0x0B44 }

    #[napi(getter, js_name = "FRONT")]
    pub fn FRONT(&self) -> u32 { 0x0404 }

    #[napi(getter, js_name = "BACK")]
    pub fn BACK(&self) -> u32 { 0x0405 }

    #[napi(getter, js_name = "FRONT_AND_BACK")]
    pub fn FRONT_AND_BACK(&self) -> u32 { 0x0408 }

    #[napi(getter, js_name = "BLEND")]
    pub fn BLEND(&self) -> u32 { 0x0BE2 }

    #[napi(getter, js_name = "DEPTH_TEST")]
    pub fn DEPTH_TEST(&self) -> u32 { 0x0B71 }

    #[napi(getter, js_name = "DITHER")]
    pub fn DITHER(&self) -> u32 { 0x0BD0 }

    #[napi(getter, js_name = "POLYGON_OFFSET_FILL")]
    pub fn POLYGON_OFFSET_FILL(&self) -> u32 { 0x8037 }

    #[napi(getter, js_name = "SAMPLE_ALPHA_TO_COVERAGE")]
    pub fn SAMPLE_ALPHA_TO_COVERAGE(&self) -> u32 { 0x809E }

    #[napi(getter, js_name = "SAMPLE_COVERAGE")]
    pub fn SAMPLE_COVERAGE(&self) -> u32 { 0x80A0 }

    #[napi(getter, js_name = "SCISSOR_TEST")]
    pub fn SCISSOR_TEST(&self) -> u32 { 0x0C11 }

    #[napi(getter, js_name = "STENCIL_TEST")]
    pub fn STENCIL_TEST(&self) -> u32 { 0x0B90 }

    /* Errors */
    #[napi(getter, js_name = "NO_ERROR")]
    pub fn NO_ERROR(&self) -> u32 { 0 }

    #[napi(getter, js_name = "INVALID_ENUM")]
    pub fn INVALID_ENUM(&self) -> u32 { 0x0500 }

    #[napi(getter, js_name = "INVALID_VALUE")]
    pub fn INVALID_VALUE(&self) -> u32 { 0x0501 }

    #[napi(getter, js_name = "INVALID_OPERATION")]
    pub fn INVALID_OPERATION(&self) -> u32 { 0x0502 }

    #[napi(getter, js_name = "OUT_OF_MEMORY")]
    pub fn OUT_OF_MEMORY(&self) -> u32 { 0x0505 }

    #[napi(getter, js_name = "CONTEXT_LOST_WEBGL")]
    pub fn CONTEXT_LOST_WEBGL(&self) -> u32 { 0x9242 }

    #[napi(getter, js_name = "CW")]
    pub fn CW(&self) -> u32 { 0x0900 }

    #[napi(getter, js_name = "CCW")]
    pub fn CCW(&self) -> u32 { 0x0901 }

    #[napi(getter, js_name = "DONT_CARE")]
    pub fn DONT_CARE(&self) -> u32 { 0x1100 }

    #[napi(getter, js_name = "FASTEST")]
    pub fn FASTEST(&self) -> u32 { 0x1101 }

    #[napi(getter, js_name = "NICEST")]
    pub fn NICEST(&self) -> u32 { 0x1102 }

    #[napi(getter, js_name = "GENERATE_MIPMAP_HINT")]
    pub fn GENERATE_MIPMAP_HINT(&self) -> u32 { 0x8192 }

    #[napi(getter, js_name = "BYTE")]
    pub fn BYTE(&self) -> u32 { 0x1400 }

    #[napi(getter, js_name = "UNSIGNED_BYTE")]
    pub fn UNSIGNED_BYTE(&self) -> u32 { 0x1401 }

    #[napi(getter, js_name = "SHORT")]
    pub fn SHORT(&self) -> u32 { 0x1402 }

    #[napi(getter, js_name = "UNSIGNED_SHORT")]
    pub fn UNSIGNED_SHORT(&self) -> u32 { 0x1403 }

    #[napi(getter, js_name = "INT")]
    pub fn INT(&self) -> u32 { 0x1404 }

    #[napi(getter, js_name = "UNSIGNED_INT")]
    pub fn UNSIGNED_INT(&self) -> u32 { 0x1405 }

    #[napi(getter, js_name = "FLOAT")]
    pub fn FLOAT(&self) -> u32 { 0x1406 }

    #[napi(getter, js_name = "DEPTH_COMPONENT")]
    pub fn DEPTH_COMPONENT(&self) -> u32 { 0x1902 }

    #[napi(getter, js_name = "ALPHA")]
    pub fn ALPHA(&self) -> u32 { 0x1906 }

    #[napi(getter, js_name = "RGB")]
    pub fn RGB(&self) -> u32 { 0x1907 }

    /* Clearing buffers */

    #[napi(getter, js_name = "RGBA")]
    pub fn RGBA(&self) -> u32 { 0x1908 }

    #[napi(getter, js_name = "LUMINANCE")]
    pub fn LUMINANCE(&self) -> u32 { 0x1909 }

    #[napi(getter, js_name = "LUMINANCE_ALPHA")]
    pub fn LUMINANCE_ALPHA(&self) -> u32 { 0x190A }

    /* Clearing buffers */

    /* Rendering primitives */

    #[napi(getter, js_name = "UNSIGNED_SHORT_4_4_4_4")]
    pub fn UNSIGNED_SHORT_4_4_4_4(&self) -> u32 { 0x8033 }

    #[napi(getter, js_name = "UNSIGNED_SHORT_5_5_5_1")]
    pub fn UNSIGNED_SHORT_5_5_5_1(&self) -> u32 { 0x8034 }

    #[napi(getter, js_name = "UNSIGNED_SHORT_5_6_5")]
    pub fn UNSIGNED_SHORT_5_6_5(&self) -> u32 { 0x8363 }

    #[napi(getter, js_name = "FRAGMENT_SHADER")]
    pub fn FRAGMENT_SHADER(&self) -> u32 { 0x8B30 }

    #[napi(getter, js_name = "VERTEX_SHADER")]
    pub fn VERTEX_SHADER(&self) -> u32 { 0x8B31 }

    #[napi(getter, js_name = "COMPILE_STATUS")]
    pub fn COMPILE_STATUS(&self) -> u32 { 0x8B81 }

    #[napi(getter, js_name = "DELETE_STATUS")]
    pub fn DELETE_STATUS(&self) -> u32 { 0x8B80 }

    /* Rendering primitives */

    /* Blending modes */

    #[napi(getter, js_name = "LINK_STATUS")]
    pub fn LINK_STATUS(&self) -> u32 { 0x8B82 }

    #[napi(getter, js_name = "VALIDATE_STATUS")]
    pub fn VALIDATE_STATUS(&self) -> u32 { 0x8B83 }

    #[napi(getter, js_name = "ATTACHED_SHADERS")]
    pub fn ATTACHED_SHADERS(&self) -> u32 { 0x8B85 }

    #[napi(getter, js_name = "ACTIVE_ATTRIBUTES")]
    pub fn ACTIVE_ATTRIBUTES(&self) -> u32 { 0x8B89 }

    #[napi(getter, js_name = "ACTIVE_UNIFORMS")]
    pub fn ACTIVE_UNIFORMS(&self) -> u32 { 0x8B86 }

    #[napi(getter, js_name = "MAX_VERTEX_ATTRIBS")]
    pub fn MAX_VERTEX_ATTRIBS(&self) -> u32 { 0x8869 }

    #[napi(getter, js_name = "MAX_VERTEX_UNIFORM_VECTORS")]
    pub fn MAX_VERTEX_UNIFORM_VECTORS(&self) -> u32 { 0x8DFB }

    #[napi(getter, js_name = "MAX_VARYING_VECTORS")]
    pub fn MAX_VARYING_VECTORS(&self) -> u32 { 0x8DFC }

    #[napi(getter, js_name = "MAX_COMBINED_TEXTURE_IMAGE_UNITS")]
    pub fn MAX_COMBINED_TEXTURE_IMAGE_UNITS(&self) -> u32 { 0x8B4D }

    #[napi(getter, js_name = "MAX_VERTEX_TEXTURE_IMAGE_UNITS")]
    pub fn MAX_VERTEX_TEXTURE_IMAGE_UNITS(&self) -> u32 { 0x8B4C }

    #[napi(getter, js_name = "MAX_TEXTURE_IMAGE_UNITS")]
    pub fn MAX_TEXTURE_IMAGE_UNITS(&self) -> u32 { 0x8872 }

    #[napi(getter, js_name = "MAX_FRAGMENT_UNIFORM_VECTORS")]
    pub fn MAX_FRAGMENT_UNIFORM_VECTORS(&self) -> u32 { 0x8DFD }

    #[napi(getter, js_name = "SHADER_TYPE")]
    pub fn SHADER_TYPE(&self) -> u32 { 0x8B4F }

    #[napi(getter, js_name = "SHADING_LANGUAGE_VERSION")]
    pub fn SHADING_LANGUAGE_VERSION(&self) -> u32 { 0x8B8C }

    #[napi(getter, js_name = "CURRENT_PROGRAM")]
    pub fn CURRENT_PROGRAM(&self) -> u32 { 0x8B8D }

    /* Blending modes */

    #[napi(getter, js_name = "NEVER")]
    pub fn NEVER(&self) -> u32 { 0x0200 }

    #[napi(getter, js_name = "LESS")]
    pub fn LESS(&self) -> u32 { 0x0201 }

    #[napi(getter, js_name = "EQUAL")]
    pub fn EQUAL(&self) -> u32 { 0x0202 }

    /* Blending equations */

    /* Getting GL parameter information */

    #[napi(getter, js_name = "LEQUAL")]
    pub fn LEQUAL(&self) -> u32 { 0x0203 }

    #[napi(getter, js_name = "GREATER")]
    pub fn GREATER(&self) -> u32 { 0x0204 }

    #[napi(getter, js_name = "NOTEQUAL")]
    pub fn NOTEQUAL(&self) -> u32 { 0x0205 }

    #[napi(getter, js_name = "GEQUAL")]
    pub fn GEQUAL(&self) -> u32 { 0x0206 }

    #[napi(getter, js_name = "ALWAYS")]
    pub fn ALWAYS(&self) -> u32 { 0x0207 }

    #[napi(getter, js_name = "KEEP")]
    pub fn KEEP(&self) -> u32 { 0x1E00 }

    #[napi(getter, js_name = "REPLACE")]
    pub fn REPLACE(&self) -> u32 { 0x1E01 }

    #[napi(getter, js_name = "INCR")]
    pub fn INCR(&self) -> u32 { 0x1E02 }

    #[napi(getter, js_name = "DECR")]
    pub fn DECR(&self) -> u32 { 0x1E03 }

    #[napi(getter, js_name = "INVERT")]
    pub fn INVERT(&self) -> u32 { 0x150A }

    #[napi(getter, js_name = "INCR_WRAP")]
    pub fn INCR_WRAP(&self) -> u32 { 0x8507 }

    #[napi(getter, js_name = "DECR_WRAP")]
    pub fn DECR_WRAP(&self) -> u32 { 0x8508 }

    #[napi(getter, js_name = "NEAREST")]
    pub fn NEAREST(&self) -> u32 { 0x2600 }

    #[napi(getter, js_name = "LINEAR")]
    pub fn LINEAR(&self) -> u32 { 0x2601 }

    #[napi(getter, js_name = "NEAREST_MIPMAP_NEAREST")]
    pub fn NEAREST_MIPMAP_NEAREST(&self) -> u32 { 0x2700 }

    #[napi(getter, js_name = "LINEAR_MIPMAP_NEAREST")]
    pub fn LINEAR_MIPMAP_NEAREST(&self) -> u32 { 0x2701 }

    #[napi(getter, js_name = "NEAREST_MIPMAP_LINEAR")]
    pub fn NEAREST_MIPMAP_LINEAR(&self) -> u32 { 0x2702 }

    #[napi(getter, js_name = "LINEAR_MIPMAP_LINEAR")]
    pub fn LINEAR_MIPMAP_LINEAR(&self) -> u32 { 0x2703 }

    #[napi(getter, js_name = "TEXTURE_MAG_FILTER")]
    pub fn TEXTURE_MAG_FILTER(&self) -> u32 { 0x2800 }

    #[napi(getter, js_name = "TEXTURE_MIN_FILTER")]
    pub fn TEXTURE_MIN_FILTER(&self) -> u32 { 0x2801 }

    #[napi(getter, js_name = "TEXTURE_WRAP_S")]
    pub fn TEXTURE_WRAP_S(&self) -> u32 { 0x2802 }

    #[napi(getter, js_name = "TEXTURE_WRAP_T")]
    pub fn TEXTURE_WRAP_T(&self) -> u32 { 0x2803 }

    #[napi(getter, js_name = "TEXTURE_2D")]
    pub fn TEXTURE_2D(&self) -> u32 { 0x0DE1 }

    #[napi(getter, js_name = "TEXTURE")]
    pub fn TEXTURE(&self) -> u32 { 0x1702 }

    #[napi(getter, js_name = "TEXTURE_CUBE_MAP")]
    pub fn TEXTURE_CUBE_MAP(&self) -> u32 { 0x8513 }

    #[napi(getter, js_name = "TEXTURE_BINDING_CUBE_MAP")]
    pub fn TEXTURE_BINDING_CUBE_MAP(&self) -> u32 { 0x8514 }

    #[napi(getter, js_name = "TEXTURE_CUBE_MAP_POSITIVE_X")]
    pub fn TEXTURE_CUBE_MAP_POSITIVE_X(&self) -> u32 { 0x8515 }

    #[napi(getter, js_name = "TEXTURE_CUBE_MAP_NEGATIVE_X")]
    pub fn TEXTURE_CUBE_MAP_NEGATIVE_X(&self) -> u32 { 0x8516 }

    #[napi(getter, js_name = "TEXTURE_CUBE_MAP_POSITIVE_Y")]
    pub fn TEXTURE_CUBE_MAP_POSITIVE_Y(&self) -> u32 { 0x8517 }

    #[napi(getter, js_name = "TEXTURE_CUBE_MAP_NEGATIVE_Y")]
    pub fn TEXTURE_CUBE_MAP_NEGATIVE_Y(&self) -> u32 { 0x8518 }

    #[napi(getter, js_name = "TEXTURE_CUBE_MAP_POSITIVE_Z")]
    pub fn TEXTURE_CUBE_MAP_POSITIVE_Z(&self) -> u32 { 0x8519 }

    #[napi(getter, js_name = "TEXTURE_CUBE_MAP_NEGATIVE_Z")]
    pub fn TEXTURE_CUBE_MAP_NEGATIVE_Z(&self) -> u32 { 0x851A }

    #[napi(getter, js_name = "MAX_CUBE_MAP_TEXTURE_SIZE")]
    pub fn MAX_CUBE_MAP_TEXTURE_SIZE(&self) -> u32 { 0x851C }

    #[napi(getter, js_name = "TEXTURE0")]
    pub fn TEXTURE0(&self) -> u32 { 0x84C0 }

    #[napi(getter, js_name = "TEXTURE1")]
    pub fn TEXTURE1(&self) -> u32 { 0x84C1 }

    #[napi(getter, js_name = "TEXTURE2")]
    pub fn TEXTURE2(&self) -> u32 { 0x84C2 }

    #[napi(getter, js_name = "TEXTURE3")]
    pub fn TEXTURE3(&self) -> u32 { 0x84C3 }

    #[napi(getter, js_name = "TEXTURE4")]
    pub fn TEXTURE4(&self) -> u32 { 0x84C4 }

    #[napi(getter, js_name = "TEXTURE5")]
    pub fn TEXTURE5(&self) -> u32 { 0x84C5 }

    #[napi(getter, js_name = "TEXTURE6")]
    pub fn TEXTURE6(&self) -> u32 { 0x84C6 }

    #[napi(getter, js_name = "TEXTURE7")]
    pub fn TEXTURE7(&self) -> u32 { 0x84C7 }

    #[napi(getter, js_name = "TEXTURE8")]
    pub fn TEXTURE8(&self) -> u32 { 0x84C8 }

    #[napi(getter, js_name = "TEXTURE9")]
    pub fn TEXTURE9(&self) -> u32 { 0x84C9 }

    #[napi(getter, js_name = "TEXTURE10")]
    pub fn TEXTURE10(&self) -> u32 { 0x84CA }

    #[napi(getter, js_name = "TEXTURE11")]
    pub fn TEXTURE11(&self) -> u32 { 0x84CB }

    #[napi(getter, js_name = "TEXTURE12")]
    pub fn TEXTURE12(&self) -> u32 { 0x84CC }

    #[napi(getter, js_name = "TEXTURE13")]
    pub fn TEXTURE13(&self) -> u32 { 0x84CD }

    #[napi(getter, js_name = "TEXTURE14")]
    pub fn TEXTURE14(&self) -> u32 { 0x84CE }

    #[napi(getter, js_name = "TEXTURE15")]
    pub fn TEXTURE15(&self) -> u32 { 0x84CF }

    #[napi(getter, js_name = "TEXTURE16")]
    pub fn TEXTURE16(&self) -> u32 { 0x84D0 }

    #[napi(getter, js_name = "TEXTURE17")]
    pub fn TEXTURE17(&self) -> u32 { 0x84D1 }

    #[napi(getter, js_name = "TEXTURE18")]
    pub fn TEXTURE18(&self) -> u32 { 0x84D2 }

    #[napi(getter, js_name = "TEXTURE19")]
    pub fn TEXTURE19(&self) -> u32 { 0x84D3 }

    #[napi(getter, js_name = "TEXTURE20")]
    pub fn TEXTURE20(&self) -> u32 { 0x84D4 }

    #[napi(getter, js_name = "TEXTURE21")]
    pub fn TEXTURE21(&self) -> u32 { 0x84D5 }

    #[napi(getter, js_name = "TEXTURE22")]
    pub fn TEXTURE22(&self) -> u32 { 0x84D6 }

    #[napi(getter, js_name = "TEXTURE23")]
    pub fn TEXTURE23(&self) -> u32 { 0x84D7 }

    #[napi(getter, js_name = "TEXTURE24")]
    pub fn TEXTURE24(&self) -> u32 { 0x84D8 }

    #[napi(getter, js_name = "TEXTURE25")]
    pub fn TEXTURE25(&self) -> u32 { 0x84D9 }

    #[napi(getter, js_name = "TEXTURE26")]
    pub fn TEXTURE26(&self) -> u32 { 0x84DA }

    #[napi(getter, js_name = "TEXTURE27")]
    pub fn TEXTURE27(&self) -> u32 { 0x84DB }

    #[napi(getter, js_name = "TEXTURE28")]
    pub fn TEXTURE28(&self) -> u32 { 0x84DC }

    #[napi(getter, js_name = "TEXTURE29")]
    pub fn TEXTURE29(&self) -> u32 { 0x84DD }

    /* Getting GL parameter information */

    /* Buffers */

    #[napi(getter, js_name = "TEXTURE30")]
    pub fn TEXTURE30(&self) -> u32 { 0x84DE }

    #[napi(getter, js_name = "TEXTURE31")]
    pub fn TEXTURE31(&self) -> u32 { 0x84DF }

    #[napi(getter, js_name = "ACTIVE_TEXTURE")]
    pub fn ACTIVE_TEXTURE(&self) -> u32 { 0x84E0 }

    #[napi(getter, js_name = "REPEAT")]
    pub fn REPEAT(&self) -> u32 { 0x2901 }

    #[napi(getter, js_name = "CLAMP_TO_EDGE")]
    pub fn CLAMP_TO_EDGE(&self) -> u32 { 0x812F }

    #[napi(getter, js_name = "MIRRORED_REPEAT")]
    pub fn MIRRORED_REPEAT(&self) -> u32 { 0x8370 }

    #[napi(getter, js_name = "FLOAT_VEC2")]
    pub fn FLOAT_VEC2(&self) -> u32 { 0x8B50 }

    /* Buffers */

    /* Vertex attributes */

    #[napi(getter, js_name = "FLOAT_VEC3")]
    pub fn FLOAT_VEC3(&self) -> u32 { 0x8B51 }

    #[napi(getter, js_name = "FLOAT_VEC4")]
    pub fn FLOAT_VEC4(&self) -> u32 { 0x8B52 }

    #[napi(getter, js_name = "INT_VEC2")]
    pub fn INT_VEC2(&self) -> u32 { 0x8B53 }

    #[napi(getter, js_name = "INT_VEC3")]
    pub fn INT_VEC3(&self) -> u32 { 0x8B54 }

    #[napi(getter, js_name = "INT_VEC4")]
    pub fn INT_VEC4(&self) -> u32 { 0x8B55 }

    #[napi(getter, js_name = "BOOL")]
    pub fn BOOL(&self) -> u32 { 0x8B56 }

    #[napi(getter, js_name = "BOOL_VEC2")]
    pub fn BOOL_VEC2(&self) -> u32 { 0x8B57 }

    #[napi(getter, js_name = "BOOL_VEC3")]
    pub fn BOOL_VEC3(&self) -> u32 { 0x8B58 }

    /* Vertex attributes */

    /* Culling */

    #[napi(getter, js_name = "BOOL_VEC4")]
    pub fn BOOL_VEC4(&self) -> u32 { 0x8B59 }

    #[napi(getter, js_name = "FLOAT_MAT2")]
    pub fn FLOAT_MAT2(&self) -> u32 { 0x8B5A }

    #[napi(getter, js_name = "FLOAT_MAT3")]
    pub fn FLOAT_MAT3(&self) -> u32 { 0x8B5B }

    #[napi(getter, js_name = "FLOAT_MAT4")]
    pub fn FLOAT_MAT4(&self) -> u32 { 0x8B5C }

    /* Culling */

    /* Enabling and disabling */

    #[napi(getter, js_name = "SAMPLER_2D")]
    pub fn SAMPLER_2D(&self) -> u32 { 0x8B5E }

    #[napi(getter, js_name = "SAMPLER_CUBE")]
    pub fn SAMPLER_CUBE(&self) -> u32 { 0x8B60 }

    #[napi(getter, js_name = "LOW_FLOAT")]
    pub fn LOW_FLOAT(&self) -> u32 { 0x8DF0 }

    #[napi(getter, js_name = "MEDIUM_FLOAT")]
    pub fn MEDIUM_FLOAT(&self) -> u32 { 0x8DF1 }

    #[napi(getter, js_name = "HIGH_FLOAT")]
    pub fn HIGH_FLOAT(&self) -> u32 { 0x8DF2 }

    #[napi(getter, js_name = "LOW_INT")]
    pub fn LOW_INT(&self) -> u32 { 0x8DF3 }

    #[napi(getter, js_name = "MEDIUM_INT")]
    pub fn MEDIUM_INT(&self) -> u32 { 0x8DF4 }

    #[napi(getter, js_name = "HIGH_INT")]
    pub fn HIGH_INT(&self) -> u32 { 0x8DF5 }

    /* Enabling and disabling */

    #[napi(getter, js_name = "FRAMEBUFFER")]
    pub fn FRAMEBUFFER(&self) -> u32 { 0x8D40 }

    #[napi(getter, js_name = "RENDERBUFFER")]
    pub fn RENDERBUFFER(&self) -> u32 { 0x8D41 }

    #[napi(getter, js_name = "RGBA4")]
    pub fn RGBA4(&self) -> u32 { 0x8056 }

    #[napi(getter, js_name = "RGB5_A1")]
    pub fn RGB5_A1(&self) -> u32 { 0x8057 }

    #[napi(getter, js_name = "RGB565")]
    pub fn RGB565(&self) -> u32 { 0x8D62 }

    #[napi(getter, js_name = "DEPTH_COMPONENT16")]
    pub fn DEPTH_COMPONENT16(&self) -> u32 { 0x81A5 }

    #[napi(getter, js_name = "STENCIL_INDEX8")]
    pub fn STENCIL_INDEX8(&self) -> u32 { 0x8D48 }

    /* Errors */

    /* Front face directions */

    #[napi(getter, js_name = "DEPTH_STENCIL")]
    pub fn DEPTH_STENCIL(&self) -> u32 { 0x84F9 }

    #[napi(getter, js_name = "RENDERBUFFER_WIDTH")]
    pub fn RENDERBUFFER_WIDTH(&self) -> u32 { 0x8D42 }

    /* Front face directions */

    /* Hints */

    #[napi(getter, js_name = "RENDERBUFFER_HEIGHT")]
    pub fn RENDERBUFFER_HEIGHT(&self) -> u32 { 0x8D43 }

    #[napi(getter, js_name = "RENDERBUFFER_INTERNAL_FORMAT")]
    pub fn RENDERBUFFER_INTERNAL_FORMAT(&self) -> u32 { 0x8D44 }

    #[napi(getter, js_name = "RENDERBUFFER_RED_SIZE")]
    pub fn RENDERBUFFER_RED_SIZE(&self) -> u32 { 0x8D50 }

    #[napi(getter, js_name = "RENDERBUFFER_GREEN_SIZE")]
    pub fn RENDERBUFFER_GREEN_SIZE(&self) -> u32 { 0x8D51 }

    /* Hints */

    /* Data types */

    #[napi(getter, js_name = "RENDERBUFFER_BLUE_SIZE")]
    pub fn RENDERBUFFER_BLUE_SIZE(&self) -> u32 { 0x8D52 }

    #[napi(getter, js_name = "RENDERBUFFER_ALPHA_SIZE")]
    pub fn RENDERBUFFER_ALPHA_SIZE(&self) -> u32 { 0x8D53 }

    #[napi(getter, js_name = "RENDERBUFFER_DEPTH_SIZE")]
    pub fn RENDERBUFFER_DEPTH_SIZE(&self) -> u32 { 0x8D54 }

    #[napi(getter, js_name = "RENDERBUFFER_STENCIL_SIZE")]
    pub fn RENDERBUFFER_STENCIL_SIZE(&self) -> u32 { 0x8D55 }

    #[napi(getter, js_name = "FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE")]
    pub fn FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE(&self) -> u32 { 0x8CD0 }

    #[napi(getter, js_name = "FRAMEBUFFER_ATTACHMENT_OBJECT_NAME")]
    pub fn FRAMEBUFFER_ATTACHMENT_OBJECT_NAME(&self) -> u32 { 0x8CD1 }

    #[napi(getter, js_name = "FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL")]
    pub fn FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL(&self) -> u32 { 0x8CD2 }

    /* Data types */

    /* Pixel formats */

    #[napi(getter, js_name = "FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE")]
    pub fn FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE(&self) -> u32 { 0x8CD3 }

    #[napi(getter, js_name = "COLOR_ATTACHMENT0")]
    pub fn COLOR_ATTACHMENT0(&self) -> u32 { 0x8CE0 }

    #[napi(getter, js_name = "DEPTH_ATTACHMENT")]
    pub fn DEPTH_ATTACHMENT(&self) -> u32 { 0x8D00 }

    #[napi(getter, js_name = "STENCIL_ATTACHMENT")]
    pub fn STENCIL_ATTACHMENT(&self) -> u32 { 0x8D20 }

    #[napi(getter, js_name = "DEPTH_STENCIL_ATTACHMENT")]
    pub fn DEPTH_STENCIL_ATTACHMENT(&self) -> u32 { 0x821A }

    #[napi(getter, js_name = "NONE")]
    pub fn NONE(&self) -> u32 { 0 }

    /* Pixel formats */

    /* Pixel types */

    // #[napi(getter, js_name = "UNSIGNED_BYTE")]
    // pub fn UNSIGNED_BYTE(&self) -> u32 { return UNSIGNED_BYTE}

    #[napi(getter, js_name = "FRAMEBUFFER_COMPLETE")]
    pub fn FRAMEBUFFER_COMPLETE(&self) -> u32 { 0x8CD5 }

    #[napi(getter, js_name = "FRAMEBUFFER_INCOMPLETE_ATTACHMENT")]
    pub fn FRAMEBUFFER_INCOMPLETE_ATTACHMENT(&self) -> u32 { 0x8CD6 }

    #[napi(getter, js_name = "FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT")]
    pub fn FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT(&self) -> u32 { 0x8CD7 }

    /* Pixel types */

    /* Shaders */

    #[napi(getter, js_name = "FRAMEBUFFER_INCOMPLETE_DIMENSIONS")]
    pub fn FRAMEBUFFER_INCOMPLETE_DIMENSIONS(&self) -> u32 { 0x8CD9 }

    #[napi(getter, js_name = "FRAMEBUFFER_UNSUPPORTED")]
    pub fn FRAMEBUFFER_UNSUPPORTED(&self) -> u32 { 0x8CDD }

    #[napi(getter, js_name = "FRAMEBUFFER_BINDING")]
    pub fn FRAMEBUFFER_BINDING(&self) -> u32 { 0x8CA6 }

    #[napi(getter, js_name = "RENDERBUFFER_BINDING")]
    pub fn RENDERBUFFER_BINDING(&self) -> u32 { 0x8CA7 }

    #[napi(getter, js_name = "MAX_RENDERBUFFER_SIZE")]
    pub fn MAX_RENDERBUFFER_SIZE(&self) -> u32 { 0x84E8 }

    #[napi(getter, js_name = "INVALID_FRAMEBUFFER_OPERATION")]
    pub fn INVALID_FRAMEBUFFER_OPERATION(&self) -> u32 { 0x0506 }

    #[napi(getter, js_name = "UNPACK_FLIP_Y_WEBGL")]
    pub fn UNPACK_FLIP_Y_WEBGL(&self) -> u32 { 0x9240 }

    #[napi(getter, js_name = "UNPACK_PREMULTIPLY_ALPHA_WEBGL")]
    pub fn UNPACK_PREMULTIPLY_ALPHA_WEBGL(&self) -> u32 { 0x9241 }

    #[napi(getter, js_name = "UNPACK_COLORSPACE_CONVERSION_WEBGL")]
    pub fn UNPACK_COLORSPACE_CONVERSION_WEBGL(&self) -> u32 { 0x9243 }
}