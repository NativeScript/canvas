//! WebGL extension objects, with the members the V8 bindings' templates give them: `ext_name`
//! (packages/canvas's `getExtension` switches on it), the extension's constants and its methods,
//! under the web names (`drawArraysInstancedANGLE`, not napi-rs's `drawArraysInstancedAngle`).

use crate::js::ToJs;
use napi::bindgen_prelude::{Either, Null, ObjectFinalize, Uint32Array, Unknown};
use napi::*;
use napi_derive::napi;

/// An extension that only adds constants: class `$js`, with `ext_name` and one getter per
/// constant.
macro_rules! constant_extension {
  ($rust:ident, $js:literal $(, $name:literal $getter:ident = $value:expr)* $(,)?) => {
    #[napi(js_name = $js)]
    pub struct $rust;

    #[napi]
    impl $rust {
      #[napi(getter, js_name = "ext_name")]
      pub fn ext_name(&self) -> &'static str {
        $js
      }
      $(
        #[napi(getter, js_name = $name)]
        pub fn $getter(&self) -> u32 {
          $value
        }
      )*
    }
  };
}

constant_extension!(OES_fbo_render_mipmap, "OES_fbo_render_mipmap");

constant_extension!(
  EXT_blend_minmax,
  "EXT_blend_minmax",
  "MIN_EXT" min_ext = 0x8007,
  "MAX_EXT" max_ext = 0x8008,
);

constant_extension!(
  EXT_color_buffer_half_float,
  "EXT_color_buffer_half_float",
  "RGBA16F_EXT" rgba16f_ext = 0x881A,
  "RGB16F_EXT" rgb16f_ext = 0x881B,
  "FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE_EXT" framebuffer_attachment_component_type_ext = 0x8211,
  "UNSIGNED_NORMALIZED_EXT" unsigned_normalized_ext = 0x8C17,
);

// WebGL 2's float render targets (the formats are core; the extension makes them renderable).
constant_extension!(
  EXT_color_buffer_float,
  "EXT_color_buffer_float",
  "R16F" r16f = 0x822D,
  "RG16F" rg16f = 0x822F,
  "RGB16F" rgb16f = 0x881B,
  "R32F" r32f = 0x822E,
  "RG32F" rg32f = 0x8230,
  "RGBA32F" rgba32f = 0x8814,
  "R11F_G11F_B10F" r11f_g11f_b10f = 0x8C3A,
);

constant_extension!(
  EXT_sRGB,
  "EXT_sRGB",
  "SRGB_EXT" srgb_ext = 0x8C40,
  "SRGB_ALPHA_EXT" srgb_alpha_ext = 0x8C42,
  "SRGB8_ALPHA8_EXT" srgb8_alpha8_ext = 0x8C43,
  "FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING_EXT" framebuffer_attachment_color_encoding_ext = 0x8210,
);

constant_extension!(EXT_shader_texture_lod, "EXT_shader_texture_lod");

constant_extension!(
  EXT_texture_filter_anisotropic,
  "EXT_texture_filter_anisotropic",
  "TEXTURE_MAX_ANISOTROPY_EXT" texture_max_anisotropy_ext = 0x84FE,
  "MAX_TEXTURE_MAX_ANISOTROPY_EXT" max_texture_max_anisotropy_ext = 0x84FF,
);

constant_extension!(
  OES_element_index_uint,
  "OES_element_index_uint",
  "UNSIGNED_INT" unsigned_int = 0x1405,
);

constant_extension!(OES_standard_derivatives, "OES_standard_derivatives",
  "FRAGMENT_SHADER_DERIVATIVE_HINT_OES" fragment_shader_derivative_hint_oes = 0x8B8B,
);

constant_extension!(OES_texture_float, "OES_texture_float");

constant_extension!(OES_texture_float_linear, "OES_texture_float_linear");

constant_extension!(
  OES_texture_half_float,
  "OES_texture_half_float",
  "HALF_FLOAT_OES" half_float_oes = 0x8D61,
);

constant_extension!(OES_texture_half_float_linear, "OES_texture_half_float_linear");

constant_extension!(
  WEBGL_color_buffer_float,
  "WEBGL_color_buffer_float",
  "RGBA32F_EXT" rgba32f_ext = 0x8814,
  "RGB32F_EXT" rgb32f_ext = 0x8815,
  "FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE_EXT" framebuffer_attachment_component_type_ext = 0x8211,
  "UNSIGNED_NORMALIZED_EXT" unsigned_normalized_ext = 0x8C17,
);

constant_extension!(
  WEBGL_compressed_texture_atc,
  "WEBGL_compressed_texture_atc",
  "COMPRESSED_RGB_ATC_WEBGL" compressed_rgb_atc_webgl = 0x8C92,
  "COMPRESSED_RGBA_ATC_EXPLICIT_ALPHA_WEBGL" compressed_rgba_atc_explicit_alpha_webgl = 0x8C93,
  "COMPRESSED_RGBA_ATC_INTERPOLATED_ALPHA_WEBGL" compressed_rgba_atc_interpolated_alpha_webgl = 0x87EE,
);

constant_extension!(
  WEBGL_compressed_texture_etc,
  "WEBGL_compressed_texture_etc",
  "COMPRESSED_R11_EAC" compressed_r11_eac = 0x9270,
  "COMPRESSED_SIGNED_R11_EAC" compressed_signed_r11_eac = 0x9271,
  "COMPRESSED_RG11_EAC" compressed_rg11_eac = 0x9272,
  "COMPRESSED_SIGNED_RG11_EAC" compressed_signed_rg11_eac = 0x9273,
  "COMPRESSED_RGB8_ETC2" compressed_rgb8_etc2 = 0x9274,
  "COMPRESSED_SRGB8_ETC2" compressed_srgb8_etc2 = 0x9275,
  "COMPRESSED_RGB8_PUNCHTHROUGH_ALPHA1_ETC2" compressed_rgb8_punchthrough_alpha1_etc2 = 0x9276,
  "COMPRESSED_SRGB8_PUNCHTHROUGH_ALPHA1_ETC2" compressed_srgb8_punchthrough_alpha1_etc2 = 0x9277,
  "COMPRESSED_RGBA8_ETC2_EAC" compressed_rgba8_etc2_eac = 0x9278,
  "COMPRESSED_SRGB8_ALPHA8_ETC2_EAC" compressed_srgb8_alpha8_etc2_eac = 0x9279,
);

constant_extension!(
  WEBGL_compressed_texture_etc1,
  "WEBGL_compressed_texture_etc1",
  "COMPRESSED_RGB_ETC1_WEBGL" compressed_rgb_etc1_webgl = 0x8D64,
);

constant_extension!(
  WEBGL_compressed_texture_pvrtc,
  "WEBGL_compressed_texture_pvrtc",
  "COMPRESSED_RGB_PVRTC_4BPPV1_IMG" compressed_rgb_pvrtc_4bppv1_img = 0x8C00,
  "COMPRESSED_RGB_PVRTC_2BPPV1_IMG" compressed_rgb_pvrtc_2bppv1_img = 0x8C01,
  "COMPRESSED_RGBA_PVRTC_4BPPV1_IMG" compressed_rgba_pvrtc_4bppv1_img = 0x8C02,
  "COMPRESSED_RGBA_PVRTC_2BPPV1_IMG" compressed_rgba_pvrtc_2bppv1_img = 0x8C03,
);

constant_extension!(
  WEBGL_compressed_texture_s3tc,
  "WEBGL_compressed_texture_s3tc",
  "COMPRESSED_RGB_S3TC_DXT1_EXT" compressed_rgb_s3tc_dxt1_ext = 0x83F0,
  "COMPRESSED_RGBA_S3TC_DXT1_EXT" compressed_rgba_s3tc_dxt1_ext = 0x83F1,
  "COMPRESSED_RGBA_S3TC_DXT3_EXT" compressed_rgba_s3tc_dxt3_ext = 0x83F2,
  "COMPRESSED_RGBA_S3TC_DXT5_EXT" compressed_rgba_s3tc_dxt5_ext = 0x83F3,
);

constant_extension!(
  WEBGL_depth_texture,
  "WEBGL_depth_texture",
  "UNSIGNED_INT_24_8_WEBGL" unsigned_int_24_8_webgl = 0x84FA,
);

#[napi(js_name = "ANGLE_instanced_arrays", custom_finalize)]
pub struct ANGLE_instanced_arrays(pub(crate) *const canvas_c::ANGLE_instanced_arrays);

impl ObjectFinalize for ANGLE_instanced_arrays {
  fn finalize(self, _: Env) -> Result<()> {
    canvas_c::canvas_native_webgl_ANGLE_instanced_arrays_destroy(self.0 as _);
    Ok(())
  }
}

#[napi]
impl ANGLE_instanced_arrays {
  #[napi(getter, js_name = "ext_name")]
  pub fn ext_name(&self) -> &'static str {
    "ANGLE_instanced_arrays"
  }

  #[napi(getter, js_name = "VERTEX_ATTRIB_ARRAY_DIVISOR_ANGLE")]
  pub fn vertex_attrib_array_divisor_angle(&self) -> u32 {
    0x88FE
  }

  #[napi(js_name = "drawArraysInstancedANGLE")]
  pub fn draw_arrays_instanced_angle(&self, mode: u32, first: i32, count: i32, primcount: i32) {
    canvas_c::canvas_native_webgl_angle_instanced_arrays_draw_arrays_instanced_angle(
      mode, first, count, primcount, self.0,
    )
  }

  #[napi(
    js_name = "drawElementsInstancedANGLE",
    ts_args_type = "mode: number, count: number, type: number, offset: number, primcount: number"
  )]
  pub fn draw_elements_instanced_angle(
    &self,
    mode: u32,
    count: i32,
    type_: u32,
    offset: i32,
    primcount: i32,
  ) {
    canvas_c::canvas_native_webgl_angle_instanced_arrays_draw_elements_instanced_angle(
      mode, count, type_, offset, primcount, self.0,
    )
  }

  #[napi(js_name = "vertexAttribDivisorANGLE")]
  pub fn vertex_attrib_divisor_angle(&self, index: u32, divisor: u32) {
    canvas_c::canvas_native_webgl_angle_instanced_arrays_vertex_attrib_divisor_angle(
      index, divisor, self.0,
    )
  }
}

#[napi(js_name = "EXT_disjoint_timer_query", custom_finalize)]
pub struct EXT_disjoint_timer_query(pub(crate) *const canvas_c::EXT_disjoint_timer_query);

impl ObjectFinalize for EXT_disjoint_timer_query {
  fn finalize(self, _: Env) -> Result<()> {
    canvas_c::canvas_native_webgl_EXT_disjoint_timer_query_destroy(self.0 as _);
    Ok(())
  }
}

#[napi]
impl EXT_disjoint_timer_query {
  #[napi(getter, js_name = "ext_name")]
  pub fn ext_name(&self) -> &'static str {
    "EXT_disjoint_timer_query"
  }

  #[napi]
  pub fn create_query_ext(&self) -> u32 {
    canvas_c::canvas_native_webgl_ext_disjoint_timer_query_create_query_ext(self.0)
  }

  #[napi]
  pub fn delete_query_ext(&self, query: u32) {
    canvas_c::canvas_native_webgl_ext_disjoint_timer_query_delete_query_ext(query, self.0)
  }

  #[napi]
  pub fn is_query_ext(&self, query: u32) -> bool {
    canvas_c::canvas_native_webgl_ext_disjoint_timer_query_is_query_ext(query, self.0)
  }

  #[napi]
  pub fn begin_query_ext(&self, target: u32, query: u32) {
    canvas_c::canvas_native_webgl_ext_disjoint_timer_query_begin_query_ext(target, query, self.0)
  }

  #[napi]
  pub fn end_query_ext(&self, target: u32) {
    canvas_c::canvas_native_webgl_ext_disjoint_timer_query_end_query_ext(target, self.0)
  }

  #[napi]
  pub fn query_counter_ext(&self, query: u32, target: u32) {
    canvas_c::canvas_native_webgl_ext_disjoint_timer_query_query_counter_ext(query, target, self.0)
  }

  #[napi]
  pub fn get_query_ext(&self, target: u32, pname: u32) -> i32 {
    canvas_c::canvas_native_webgl_ext_disjoint_timer_query_get_query_ext(target, pname, self.0)
  }

  #[napi]
  pub fn get_query_object_ext<'env>(
    &self,
    env: &'env Env,
    target: u32,
    pname: u32,
  ) -> Result<Unknown<'env>> {
    let obj = canvas_c::canvas_native_webgl_ext_disjoint_timer_query_get_query_object_ext(
      target, pname, self.0,
    );
    if obj.is_null() {
      return Null.to_js(env);
    }

    // QUERY_RESULT_AVAILABLE
    if pname == 0x8867 {
      let ret = canvas_c::canvas_native_webgl_result_get_bool(obj as _);
      return ret.to_js(env);
    }
    let ret = canvas_c::canvas_native_webgl_result_get_i32(obj as _);
    ret.to_js(env)
  }

  /// The V8 bindings' name for `getQueryObjectExt`.
  #[napi]
  pub fn get_query_parameter_ext<'env>(
    &self,
    env: &'env Env,
    query: u32,
    pname: u32,
  ) -> Result<Unknown<'env>> {
    self.get_query_object_ext(env, query, pname)
  }

  #[napi(js_name = "QUERY_COUNTER_BITS_EXT", getter)]
  pub fn get_query_counter_bits_ext(&self) -> u32 {
    0x8864
  }

  #[napi(js_name = "CURRENT_QUERY_EXT", getter)]
  pub fn get_current_query_ext(&self) -> u32 {
    0x8865
  }

  #[napi(js_name = "QUERY_RESULT_EXT", getter)]
  pub fn get_query_result_ext(&self) -> u32 {
    0x8866
  }

  #[napi(js_name = "QUERY_RESULT_AVAILABLE_EXT", getter)]
  pub fn get_query_result_available_ext(&self) -> u32 {
    0x8867
  }

  #[napi(js_name = "TIME_ELAPSED_EXT", getter)]
  pub fn get_time_elapsed_ext(&self) -> u32 {
    0x88BF
  }

  #[napi(js_name = "TIMESTAMP_EXT", getter)]
  pub fn get_timestamp_ext(&self) -> u32 {
    0x8E28
  }

  #[napi(js_name = "GPU_DISJOINT_EXT", getter)]
  pub fn get_gpu_disjoint_ext(&self) -> u32 {
    0x8FBB
  }
}

#[napi(js_name = "OES_vertex_array_object", custom_finalize)]
pub struct OES_vertex_array_object(pub(crate) *const canvas_c::OES_vertex_array_object);

impl ObjectFinalize for OES_vertex_array_object {
  fn finalize(self, _: Env) -> Result<()> {
    canvas_c::canvas_native_webgl_OES_vertex_array_object_destroy(self.0 as _);
    Ok(())
  }
}

#[napi]
impl OES_vertex_array_object {
  #[napi(getter, js_name = "ext_name")]
  pub fn ext_name(&self) -> &'static str {
    "OES_vertex_array_object"
  }

  #[napi(js_name = "VERTEX_ARRAY_BINDING_OES", getter)]
  pub fn get_vertex_array_binding_oes(&self) -> u32 {
    0x85B5
  }

  #[napi(js_name = "createVertexArrayOES")]
  pub fn create_vertex_array_oes(&self) -> u32 {
    canvas_c::canvas_native_webgl_oes_vertex_array_object_create_vertex_array_oes(self.0)
  }

  #[napi(js_name = "deleteVertexArrayOES")]
  pub fn delete_vertex_array_oes(&self, array_object: u32) {
    canvas_c::canvas_native_webgl_oes_vertex_array_object_delete_vertex_array_oes(
      array_object,
      self.0,
    )
  }

  #[napi(js_name = "isVertexArrayOES")]
  pub fn is_vertex_array_oes(&self, array_object: u32) -> bool {
    canvas_c::canvas_native_webgl_oes_vertex_array_object_is_vertex_array_oes(array_object, self.0)
  }

  #[napi(js_name = "bindVertexArrayOES")]
  pub fn bind_vertex_array_oes(&self, array_object: u32) {
    canvas_c::canvas_native_webgl_oes_vertex_array_object_bind_vertex_array_oes(
      array_object,
      self.0,
    )
  }
}

#[napi(js_name = "WEBGL_lose_context", custom_finalize)]
pub struct w_e_b_g_l_lose_context(pub(crate) *const canvas_c::WEBGL_lose_context);

impl ObjectFinalize for w_e_b_g_l_lose_context {
  fn finalize(self, _: Env) -> Result<()> {
    canvas_c::canvas_native_webgl_WEBGL_lose_context_destroy(self.0 as _);
    Ok(())
  }
}

#[napi]
impl w_e_b_g_l_lose_context {
  #[napi(getter, js_name = "ext_name")]
  pub fn ext_name(&self) -> &'static str {
    "WEBGL_lose_context"
  }

  #[napi]
  pub fn lose_context(&self) {
    canvas_c::canvas_native_webgl_lose_context_lose_context(self.0)
  }

  #[napi]
  pub fn restore_context(&self) {
    canvas_c::canvas_native_webgl_lose_context_restore_context(self.0)
  }
}

#[napi(js_name = "WEBGL_draw_buffers", custom_finalize)]
pub struct WEBGL_draw_buffers(pub(crate) *const canvas_c::WEBGL_draw_buffers);

impl ObjectFinalize for WEBGL_draw_buffers {
  fn finalize(self, _: Env) -> Result<()> {
    canvas_c::canvas_native_webgl_WEBGL_draw_buffers_destroy(self.0 as _);
    Ok(())
  }
}

/// `COLOR_ATTACHMENT<n>_EXT` / `DRAW_BUFFER<n>_EXT` getters.
macro_rules! draw_buffer_constants {
  ($($n:literal $attachment:literal $attachment_getter:ident $buffer:literal $buffer_getter:ident),* $(,)?) => {
    #[napi]
    impl WEBGL_draw_buffers {
      $(
        #[napi(getter, js_name = $attachment)]
        pub fn $attachment_getter(&self) -> u32 {
          0x8CE0 + $n
        }

        #[napi(getter, js_name = $buffer)]
        pub fn $buffer_getter(&self) -> u32 {
          0x8825 + $n
        }
      )*
    }
  };
}

draw_buffer_constants!(
  0 "COLOR_ATTACHMENT0_EXT" color_attachment0_ext "DRAW_BUFFER0_EXT" draw_buffer0_ext,
  1 "COLOR_ATTACHMENT1_EXT" color_attachment1_ext "DRAW_BUFFER1_EXT" draw_buffer1_ext,
  2 "COLOR_ATTACHMENT2_EXT" color_attachment2_ext "DRAW_BUFFER2_EXT" draw_buffer2_ext,
  3 "COLOR_ATTACHMENT3_EXT" color_attachment3_ext "DRAW_BUFFER3_EXT" draw_buffer3_ext,
  4 "COLOR_ATTACHMENT4_EXT" color_attachment4_ext "DRAW_BUFFER4_EXT" draw_buffer4_ext,
  5 "COLOR_ATTACHMENT5_EXT" color_attachment5_ext "DRAW_BUFFER5_EXT" draw_buffer5_ext,
  6 "COLOR_ATTACHMENT6_EXT" color_attachment6_ext "DRAW_BUFFER6_EXT" draw_buffer6_ext,
  7 "COLOR_ATTACHMENT7_EXT" color_attachment7_ext "DRAW_BUFFER7_EXT" draw_buffer7_ext,
  8 "COLOR_ATTACHMENT8_EXT" color_attachment8_ext "DRAW_BUFFER8_EXT" draw_buffer8_ext,
  9 "COLOR_ATTACHMENT9_EXT" color_attachment9_ext "DRAW_BUFFER9_EXT" draw_buffer9_ext,
  10 "COLOR_ATTACHMENT10_EXT" color_attachment10_ext "DRAW_BUFFER10_EXT" draw_buffer10_ext,
  11 "COLOR_ATTACHMENT11_EXT" color_attachment11_ext "DRAW_BUFFER11_EXT" draw_buffer11_ext,
  12 "COLOR_ATTACHMENT12_EXT" color_attachment12_ext "DRAW_BUFFER12_EXT" draw_buffer12_ext,
  13 "COLOR_ATTACHMENT13_EXT" color_attachment13_ext "DRAW_BUFFER13_EXT" draw_buffer13_ext,
  14 "COLOR_ATTACHMENT14_EXT" color_attachment14_ext "DRAW_BUFFER14_EXT" draw_buffer14_ext,
  15 "COLOR_ATTACHMENT15_EXT" color_attachment15_ext "DRAW_BUFFER15_EXT" draw_buffer15_ext,
);

#[napi]
impl WEBGL_draw_buffers {
  #[napi(getter, js_name = "ext_name")]
  pub fn ext_name(&self) -> &'static str {
    "WEBGL_draw_buffers"
  }

  #[napi(js_name = "drawBuffersWEBGL")]
  pub fn draw_buffers_webgl(&self, buffers: Either<Vec<u32>, Uint32Array>) {
    let buffers: &[u32] = match &buffers {
      Either::A(buffers) => buffers,
      Either::B(buffers) => buffers,
    };
    canvas_c::canvas_native_webgl_draw_buffers_draw_buffers_webgl(
      buffers.as_ptr(),
      buffers.len(),
      self.0,
    )
  }

  #[napi(getter, js_name = "MAX_COLOR_ATTACHMENTS_EXT")]
  pub fn max_color_attachments_ext(&self) -> u32 {
    0x8CDF
  }

  #[napi(getter, js_name = "MAX_DRAW_BUFFERS_EXT")]
  pub fn max_draw_buffers_ext(&self) -> u32 {
    0x8824
  }
}
