pub mod base;
mod constants;
pub mod context_attributes;
pub mod extensions;
mod imports;
pub mod webgl_active_info;
pub mod webgl_buffer;
pub mod webgl_framebuffer;
pub mod webgl_program;
pub mod webgl_renderbuffer;
pub mod webgl_shader;
pub mod webgl_shader_precision_format;
pub mod webgl_texture;
pub mod webgl_uniform_location;

use crate::c2d::CanvasRenderingContext2D;
use crate::gl2::web_g_l_2_rendering_context;
use crate::image_asset::ImageAsset;
use crate::image_bitmap::ImageBitmap;
use crate::{impl_webgl_context, impl_webgl_context_constants};
use napi::*;
use napi_derive::napi;

#[napi(custom_finalize)]
pub struct web_g_l_rendering_context {
  pub(crate) state: *mut WebGLState,
  pub(crate) invalidate_state: u32,
}

impl ObjectFinalize for web_g_l_rendering_context {
  fn finalize(self, _: Env) -> Result<()> {
    canvas_c::canvas_native_webgl_state_destroy(self.state);
    Ok(())
  }
}

impl_webgl_context!(web_g_l_rendering_context);

pub(crate) fn get_parameter_inner(
  state: *mut canvas_c::WebGLState,
  env: Env,
  pname: u32,
) -> Result<Unknown> {
  let mut consumed = false;
  let result = canvas_c::canvas_native_webgl_get_parameter(pname, state);

  let parameter = match pname {
    gl_bindings::ACTIVE_TEXTURE
    | gl_bindings::ALPHA_BITS
    | gl_bindings::ARRAY_BUFFER_BINDING
    | gl_bindings::BLEND_DST_ALPHA
    | gl_bindings::BLEND_DST_RGB
    | gl_bindings::BLEND_EQUATION
    | gl_bindings::BLEND_EQUATION_ALPHA
    | gl_bindings::BLEND_SRC_ALPHA
    | gl_bindings::BLEND_SRC_RGB
    | gl_bindings::BLUE_BITS
    | gl_bindings::CULL_FACE_MODE
    | gl_bindings::CURRENT_PROGRAM
    | gl_bindings::DEPTH_BITS
    | gl_bindings::DEPTH_FUNC
    | gl_bindings::ELEMENT_ARRAY_BUFFER_BINDING
    | gl_bindings::FRAMEBUFFER_BINDING
    | gl_bindings::FRONT_FACE
    | gl_bindings::GENERATE_MIPMAP_HINT
    | gl_bindings::GREEN_BITS
    | gl_bindings::IMPLEMENTATION_COLOR_READ_FORMAT
    | gl_bindings::IMPLEMENTATION_COLOR_READ_TYPE
    | gl_bindings::MAX_COMBINED_TEXTURE_IMAGE_UNITS
    | gl_bindings::MAX_CUBE_MAP_TEXTURE_SIZE
    | gl_bindings::MAX_FRAGMENT_UNIFORM_VECTORS
    | gl_bindings::MAX_RENDERBUFFER_SIZE
    | gl_bindings::MAX_TEXTURE_IMAGE_UNITS
    | gl_bindings::MAX_TEXTURE_SIZE
    | gl_bindings::MAX_VARYING_VECTORS
    | gl_bindings::MAX_VERTEX_ATTRIBS
    | gl_bindings::MAX_VERTEX_TEXTURE_IMAGE_UNITS
    | gl_bindings::MAX_VERTEX_UNIFORM_VECTORS
    | gl_bindings::PACK_ALIGNMENT
    | gl_bindings::RED_BITS
    | gl_bindings::RENDERBUFFER_BINDING
    | gl_bindings::SAMPLE_BUFFERS
    | gl_bindings::SAMPLES
    | gl_bindings::STENCIL_BACK_FAIL
    | gl_bindings::STENCIL_BACK_FUNC
    | gl_bindings::STENCIL_BACK_PASS_DEPTH_FAIL
    | gl_bindings::STENCIL_BACK_PASS_DEPTH_PASS
    | gl_bindings::STENCIL_BACK_REF
    | gl_bindings::STENCIL_BACK_VALUE_MASK
    | gl_bindings::STENCIL_BACK_WRITEMASK
    | gl_bindings::STENCIL_BITS
    | gl_bindings::STENCIL_CLEAR_VALUE
    | gl_bindings::STENCIL_FAIL
    | gl_bindings::STENCIL_FUNC
    | gl_bindings::STENCIL_PASS_DEPTH_FAIL
    | gl_bindings::STENCIL_PASS_DEPTH_PASS
    | gl_bindings::STENCIL_REF
    | gl_bindings::STENCIL_VALUE_MASK
    | gl_bindings::STENCIL_WRITEMASK
    | gl_bindings::SUBPIXEL_BITS
    | gl_bindings::TEXTURE_BINDING_2D
    | gl_bindings::TEXTURE_BINDING_CUBE_MAP
    | gl_bindings::UNPACK_ALIGNMENT => {
      let value = canvas_c::canvas_native_webgl_result_get_i32(result);
      if (pname == gl_bindings::CURRENT_PROGRAM
        || pname == gl_bindings::ARRAY_BUFFER_BINDING
        || pname == gl_bindings::ELEMENT_ARRAY_BUFFER_BINDING
        || pname == gl_bindings::TEXTURE_BINDING_2D
        || pname == gl_bindings::TEXTURE_BINDING_CUBE_MAP
        || pname == gl_bindings::RENDERBUFFER_BINDING
        || pname == gl_bindings::FRAMEBUFFER_BINDING)
        && value == 0
      {
        return env.get_null().map(|v| v.into_unknown());
      }

      env.create_int32(value).map(|v| v.into_unknown())
    }
    UNPACK_COLOR_SPACE_CONVERSION_WEBGL => {
      let ret = canvas_c::canvas_native_webgl_state_get_unpack_colorspace_conversion_webgl(state);
      env.create_int32(ret).map(|v| v.into_unknown())
    }
    gl_bindings::ALIASED_LINE_WIDTH_RANGE
    | gl_bindings::ALIASED_POINT_SIZE_RANGE
    | gl_bindings::BLEND_COLOR
    | gl_bindings::COLOR_CLEAR_VALUE
    | gl_bindings::DEPTH_RANGE => unsafe {
      let ret = canvas_c::canvas_native_webgl_result_into_f32_array(result);

      if ret.is_null() {
        return env.get_null().map(|v| v.into_unknown());
      }

      let ret = *Box::from_raw(ret);
      let mut ret = ret.into_vec();

      consumed = true;

      let ptr = ret.as_mut_ptr();
      let len = ret.len();

      let buffer = env.create_arraybuffer_with_borrowed_data(
        ptr as _,
        len * size_of::<f32>(),
        ret,
        |_, _| {},
      )?;
      buffer
        .value
        .into_typedarray(TypedArrayType::Float32, len, 0)
        .map(|v| v.into_unknown())
    },
    UNPACK_FLIP_Y_WEBGL => {
      let ret = canvas_c::canvas_native_webgl_state_get_flip_y(state);
      env.get_boolean(ret).map(|v| v.into_unknown())
    }
    UNPACK_PREMULTIPLY_ALPHA_WEBGL => {
      let ret = canvas_c::canvas_native_webgl_state_get_premultiplied_alpha(state);
      env.get_boolean(ret).map(|v| v.into_unknown())
    }
    gl_bindings::BLEND
    | gl_bindings::CULL_FACE
    | gl_bindings::DEPTH_TEST
    | gl_bindings::DEPTH_WRITEMASK
    | gl_bindings::DITHER
    | gl_bindings::POLYGON_OFFSET_FILL
    | gl_bindings::SAMPLE_COVERAGE_INVERT
    | gl_bindings::SCISSOR_TEST
    | gl_bindings::STENCIL_TEST => {
      let ret = canvas_c::canvas_native_webgl_result_get_bool(result);
      env.get_boolean(ret).map(|v| v.into_unknown())
    }
    gl_bindings::COLOR_WRITEMASK => {
      let ret = canvas_c::canvas_native_webgl_result_get_bool_array(result);
      let len = canvas_c::canvas_native_u8_buffer_get_length(ret);
      let buf = canvas_c::canvas_native_u8_buffer_get_bytes(ret);
      let buf = unsafe { std::slice::from_raw_parts(buf, len) };
      let mut array = env.create_array(len as u32)?;

      for i in 0..len {
        array.set(i as u32, buf[i] == 1)?;
      }
      array.coerce_to_object().map(|v| v.into_unknown())
    }
    gl_bindings::COMPRESSED_TEXTURE_FORMATS
    | gl_bindings::MAX_VIEWPORT_DIMS
    | gl_bindings::SCISSOR_BOX
    | gl_bindings::VIEWPORT => {
      let ret = canvas_c::canvas_native_webgl_result_into_i32_array(result);

      if ret.is_null() {
        return env.get_null().map(|v| v.into_unknown());
      }

      let ret = unsafe { *Box::from_raw(ret) };
      let mut ret = ret.into_vec();

      consumed = true;

      let ptr = ret.as_mut_ptr();
      let len = ret.len();

      let buffer = unsafe {
        env.create_arraybuffer_with_borrowed_data(
          ptr as _,
          len * size_of::<i32>(),
          ret,
          |_, _| {},
        )?
      };
      buffer
        .value
        .into_typedarray(TypedArrayType::Int32, len, 0)
        .map(|v| v.into_unknown())
    }
    gl_bindings::DEPTH_CLEAR_VALUE
    | gl_bindings::LINE_WIDTH
    | gl_bindings::POLYGON_OFFSET_FACTOR
    | gl_bindings::POLYGON_OFFSET_UNITS
    | gl_bindings::SAMPLE_COVERAGE_VALUE => {
      let ret = canvas_c::canvas_native_webgl_result_get_f32(result);
      env.create_double(ret as f64).map(|v| v.into_unknown())
    }
    gl_bindings::RENDERER
    | gl_bindings::SHADING_LANGUAGE_VERSION
    | gl_bindings::VENDOR
    | gl_bindings::VERSION => {
      let ret = canvas_c::canvas_native_webgl_result_get_string(result);
      if ret.is_null() {
        return env.get_null().map(|v| v.into_unknown());
      }
      let ret = unsafe { CString::from_raw(ret as _) };
      let ret = ret
        .into_string()
        .map_err(|v| Error::from_reason(v.utf8_error().to_string()))?;
      env.create_string_from_std(ret).map(|v| v.into_unknown())
    }

    _ => env.get_null().map(|v| v.into_unknown()),
  };

  if !consumed {
    canvas_c::canvas_native_webgl_WebGLResult_destroy(result);
  }

  parameter
}

#[napi]
impl web_g_l_rendering_context {
  #[napi(factory)]
  pub fn with_view(
    view: i64,
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
      1,
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

    Ok(web_g_l_rendering_context { state: ret, invalidate_state: 0 })
  }

  #[napi(factory)]
  pub fn offscreen(
    width: i32,
    height: i32,
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
      1,
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

    Ok(web_g_l_rendering_context { state: ret, invalidate_state: 0 })
  }

  #[napi]
  pub fn get_parameter(&self, env: Env, pname: u32) -> Result<Unknown> {
    get_parameter_inner(self.state, env, pname)
  }
}

impl_webgl_context_constants!(web_g_l_rendering_context);
