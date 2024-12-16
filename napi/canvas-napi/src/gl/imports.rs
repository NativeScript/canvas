#[macro_export]
macro_rules! webgl_context_imports {
  () => {
    use crate::gl::webgl_buffer::WebGLBuffer;
    use crate::gl::webgl_framebuffer::web_g_l_framebuffer;
    use crate::gl::webgl_program::WebGLProgram;
    use crate::gl::webgl_renderbuffer::WebGLRenderbuffer;
    use crate::gl::webgl_shader::WebGLShader;
    use crate::gl::webgl_texture::WebGLTexture;
    use napi::*;

    use crate::gl::webgl_active_info::web_g_l_active_info;
    use crate::gl::webgl_shader_precision_format::web_g_l_shader_precision_format;
    use crate::gl::webgl_uniform_location::WebGLUniformLocation;
    use canvas_c::{WebGLExtension, WebGLResultType, WebGLState};
    use std::ffi::{c_void, CString, IntoStringError};

    use canvas_c::InvalidateState;

    use crate::gl::extensions::{
      ANGLE_instanced_arrays, EXT_blend_minmax, EXT_color_buffer_half_float,
      EXT_disjoint_timer_query, EXT_sRGB, EXT_shader_texture_lod, EXT_texture_filter_anisotropic,
      OES_element_index_uint, OES_fbo_render_mipmap, OES_standard_derivatives, OES_texture_float,
      OES_texture_float_linear, OES_texture_half_float, OES_texture_half_float_linear,
      OES_vertex_array_object, WEBGL_color_buffer_float, WEBGL_compressed_texture_atc,
      WEBGL_compressed_texture_etc, WEBGL_compressed_texture_etc1, WEBGL_compressed_texture_pvrtc,
      WEBGL_compressed_texture_s3tc, WEBGL_depth_texture, WEBGL_draw_buffers, WEBGL_lose_context,
    };
    use napi::bindgen_prelude::{
      Array, Buffer, ClassInstance, Either3, Either4, Either5,Either7, Float32Array, Int32Array,
      ObjectFinalize, Uint32Array, Unknown,
    };
    use std::sync::Arc;
  };
}
