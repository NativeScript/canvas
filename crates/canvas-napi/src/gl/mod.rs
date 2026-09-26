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
use crate::gpu::context::g_p_u_canvas_context;
use crate::image_asset::ImageAsset;
use crate::image_bitmap::ImageBitmap;
use crate::{impl_webgl_context, impl_webgl_context_constants};
use crate::frame::FrameSlot;
use crate::module::{as_bool, as_number, property, type_of};
use napi::bindgen_prelude::BigInt;
use napi::*;
use napi_derive::napi;
use std::cell::Cell;
use std::rc::Rc;

#[napi(object)]
pub struct HTMLImageSource<'env> {
  #[napi(js_name = "_image")]
  pub image: ClassInstance<'env, ImageAsset>,
}

#[napi(object)]
pub struct HTMLCanvasSource<'env> {
  #[napi(js_name = "__native__context")]
  pub context: Either4<
    ClassInstance<'env, CanvasRenderingContext2D>,
    ClassInstance<'env, web_g_l_rendering_context>,
    ClassInstance<'env, web_g_l_2_rendering_context>,
    ClassInstance<'env, g_p_u_canvas_context>,
  >,
}

#[napi(custom_finalize)]
pub struct web_g_l_rendering_context {
  pub(crate) state: *mut WebGLState,
  pub(crate) invalidate_state: u32,
  /// `createWebGLContext(options, pointer)` wraps a state the host view owns.
  pub(crate) owns_state: bool,
  /// Dirty tracking: drawing marks the context, the host presents it at frame end.
  pub(crate) frame: Rc<FrameSlot>,
  pub(crate) continuous_render: Cell<bool>,
}

impl ObjectFinalize for web_g_l_rendering_context {
  fn finalize(self, _: Env) -> Result<()> {
    if self.owns_state {
      canvas_c::canvas_native_webgl_state_destroy(self.state);
    }
    Ok(())
  }
}

/// A WebGL object argument that may be absent. `null`, `undefined` and `0` (what packages/canvas
/// passes for "no object", e.g. `bindFramebuffer(target, framebuffer ? framebuffer.native : 0)`)
/// are none, as in the V8 bindings; an object must be the class instance.
pub struct GLObject<'a, T: 'a>(pub Option<bindgen_prelude::ClassInstance<'a, T>>);

impl<'a, T: 'a> GLObject<'a, T> {
  /// The object's GL name, 0 when absent.
  pub fn name(&self, name: impl Fn(&T) -> u32) -> u32 {
    self.0.as_ref().map_or(0, |object| name(object))
  }
}

impl<'a, T: 'a> bindgen_prelude::TypeName for GLObject<'a, T> {
  fn type_name() -> &'static str {
    "WebGLObject | null"
  }

  fn value_type() -> ValueType {
    ValueType::Unknown
  }
}

impl<'a, T: 'a> bindgen_prelude::ValidateNapiValue for GLObject<'a, T>
where
  bindgen_prelude::ClassInstance<'a, T>: bindgen_prelude::FromNapiValue + bindgen_prelude::ValidateNapiValue,
{
  unsafe fn validate(env: sys::napi_env, value: sys::napi_value) -> Result<sys::napi_value> {
    let mut kind = 0;
    check_status!(unsafe { sys::napi_typeof(env, value, &mut kind) })?;
    if kind == sys::ValueType::napi_object {
      return unsafe { <bindgen_prelude::ClassInstance<'a, T> as bindgen_prelude::ValidateNapiValue>::validate(env, value) };
    }
    Ok(std::ptr::null_mut())
  }
}

impl<'a, T: 'a> bindgen_prelude::FromNapiValue for GLObject<'a, T>
where
  bindgen_prelude::ClassInstance<'a, T>: bindgen_prelude::FromNapiValue + bindgen_prelude::ValidateNapiValue,
{
  unsafe fn from_napi_value(env: sys::napi_env, value: sys::napi_value) -> Result<Self> {
    let mut kind = 0;
    check_status!(unsafe { sys::napi_typeof(env, value, &mut kind) })?;
    if kind != sys::ValueType::napi_object {
      return Ok(GLObject(None));
    }
    unsafe { <bindgen_prelude::ClassInstance<'a, T> as bindgen_prelude::FromNapiValue>::from_napi_value(env, value) }
      .map(|object| GLObject(Some(object)))
  }
}

/// Frame-end flush for WebGL / WebGL2 contexts: presents on screen, flushes offscreen.
pub(crate) unsafe fn present_webgl(state: *mut std::ffi::c_void) {
  canvas_c::canvas_native_webgl_present(state as *mut WebGLState);
}

impl web_g_l_rendering_context {
  pub(crate) fn from_raw(state: *mut WebGLState, owns_state: bool) -> Self {
    Self {
      state,
      invalidate_state: 0,
      owns_state,
      frame: FrameSlot::new(state as *mut std::ffi::c_void, present_webgl),
      continuous_render: Cell::new(false),
    }
  }
}

/// The options object `createWebGLContext` / `createWebGL2Context` take (the V8 bindings'
/// GLOptions): fields of the wrong type keep their defaults.
pub(crate) struct GLOptions {
  pub version: i32,
  pub alpha: bool,
  pub antialias: bool,
  pub depth: bool,
  pub fail_if_major_performance_caveat: bool,
  pub power_preference: i32,
  pub premultiplied_alpha: bool,
  pub preserve_drawing_buffer: bool,
  pub stencil: bool,
  pub desynchronized: bool,
  pub xr_compatible: bool,
}

impl GLOptions {
  pub(crate) fn parse(options: &Unknown) -> Self {
    let flag = |name: &std::ffi::CStr, default: bool| {
      property(options, name).and_then(|v| as_bool(&v)).unwrap_or(default)
    };
    let int = |name: &std::ffi::CStr, default: i32| {
      property(options, name).and_then(|v| as_number(&v)).map_or(default, |v| v as i32)
    };
    Self {
      version: int(c"version", 0),
      alpha: flag(c"alpha", true),
      antialias: flag(c"antialias", true),
      depth: flag(c"depth", true),
      fail_if_major_performance_caveat: flag(c"failIfMajorPerformanceCaveat", false),
      power_preference: int(c"powerPreference", 0),
      premultiplied_alpha: flag(c"premultipliedAlpha", true),
      preserve_drawing_buffer: flag(c"preserveDrawingBuffer", false),
      stencil: flag(c"stencil", false),
      desynchronized: flag(c"desynchronized", false),
      xr_compatible: flag(c"xrCompatible", false),
    }
  }

  /// A new offscreen context of this version.
  pub(crate) fn create_offscreen(&self, width: i32, height: i32) -> *mut WebGLState {
    canvas_c::canvas_native_webgl_create_no_window(
      width,
      height,
      self.version,
      self.alpha,
      self.antialias,
      self.depth,
      self.fail_if_major_performance_caveat,
      self.power_preference,
      self.premultiplied_alpha,
      self.preserve_drawing_buffer,
      self.stencil,
      self.desynchronized,
      self.xr_compatible,
      false,
    )
  }
}

/// `(options, pointer, ...)`: wrap the host's state; `(options, width, height, ...)`: a new
/// offscreen one. None when `options.version` is not `version`.
pub(crate) fn resolve_webgl_state(
  version: i32,
  options: &Unknown,
  target: &Unknown,
  height: Option<f64>,
) -> Option<(*mut WebGLState, bool)> {
  let options = GLOptions::parse(options);
  if options.version != version {
    return None;
  }
  if type_of(target) == ValueType::BigInt {
    let (pointer, _) = unsafe { target.cast::<BigInt>() }.ok()?.get_i64();
    return (pointer != 0).then_some((pointer as *mut WebGLState, false));
  }
  let width = as_number(target).unwrap_or(300.) as i32;
  let state = options.create_offscreen(width, height.unwrap_or(150.) as i32);
  (!state.is_null()).then_some((state, true))
}

/// `CanvasModule.createWebGLContext(options, pointer, scale, color, ppi, direction)` wraps the
/// host view's context; `(options, width, height, ...)` creates an offscreen one.
#[napi(js_name = "createWebGLContext")]
pub fn create_web_g_l_context(
  options: Unknown,
  target: Unknown,
  height: Option<f64>,
) -> Option<web_g_l_rendering_context> {
  let (state, owns) = resolve_webgl_state(1, &options, &target, height)?;
  Some(web_g_l_rendering_context::from_raw(state, owns))
}

impl_webgl_context!(web_g_l_rendering_context);

pub(crate) fn get_parameter_inner<'env>(
  state: *mut canvas_c::WebGLState,
  env: &'env Env,
  pname: u32,
) -> Result<Unknown<'env>> {
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
        return Null.to_js(env);
      }

      (value).to_js(env)
    }
    UNPACK_COLOR_SPACE_CONVERSION_WEBGL => {
      let ret = canvas_c::canvas_native_webgl_state_get_unpack_colorspace_conversion_webgl(state);
      (ret).to_js(env)
    }
    gl_bindings::ALIASED_LINE_WIDTH_RANGE
    | gl_bindings::ALIASED_POINT_SIZE_RANGE
    | gl_bindings::BLEND_COLOR
    | gl_bindings::COLOR_CLEAR_VALUE
    | gl_bindings::DEPTH_RANGE => unsafe {
      let ret = canvas_c::canvas_native_webgl_result_into_f32_array(result);

      if ret.is_null() {
        return Null.to_js(env);
      }

      let ret = *Box::from_raw(ret);
      let mut ret = ret.into_vec();

      consumed = true;

      Float32Array::new(ret).to_js(env)
    },
    UNPACK_FLIP_Y_WEBGL => {
      let ret = canvas_c::canvas_native_webgl_state_get_flip_y(state);
      (ret).to_js(env)
    }
    UNPACK_PREMULTIPLY_ALPHA_WEBGL => {
      let ret = canvas_c::canvas_native_webgl_state_get_premultiplied_alpha(state);
      (ret).to_js(env)
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
      (ret).to_js(env)
    }
    gl_bindings::COLOR_WRITEMASK => {
      let ret = canvas_c::canvas_native_webgl_result_get_bool_array(result);
      let len = canvas_c::canvas_native_u8_buffer_get_length(ret);
      let buf = canvas_c::canvas_native_u8_buffer_get_bytes(ret);
      let buf = unsafe { std::slice::from_raw_parts(buf, len) };
      buf.iter().map(|v| *v == 1).collect::<Vec<bool>>().to_js(env)
    }
    gl_bindings::COMPRESSED_TEXTURE_FORMATS
    | gl_bindings::MAX_VIEWPORT_DIMS
    | gl_bindings::SCISSOR_BOX
    | gl_bindings::VIEWPORT => {
      let ret = canvas_c::canvas_native_webgl_result_into_i32_array(result);

      if ret.is_null() {
        return Null.to_js(env);
      }

      let ret = unsafe { *Box::from_raw(ret) };
      let mut ret = ret.into_vec();

      consumed = true;

      Int32Array::new(ret).to_js(env)
    }
    gl_bindings::DEPTH_CLEAR_VALUE
    | gl_bindings::LINE_WIDTH
    | gl_bindings::POLYGON_OFFSET_FACTOR
    | gl_bindings::POLYGON_OFFSET_UNITS
    | gl_bindings::SAMPLE_COVERAGE_VALUE => {
      let ret = canvas_c::canvas_native_webgl_result_get_f32(result);
      (ret as f64).to_js(env)
    }
    gl_bindings::RENDERER
    | gl_bindings::SHADING_LANGUAGE_VERSION
    | gl_bindings::VENDOR
    | gl_bindings::VERSION => {
      let ret = canvas_c::canvas_native_webgl_result_get_string(result);
      if ret.is_null() {
        return Null.to_js(env);
      }
      let ret = unsafe { CString::from_raw(ret as _) };
      let ret = ret
        .into_string()
        .map_err(|v| Error::from_reason(v.utf8_error().to_string()))?;
      (ret).to_js(env)
    }

    _ => Null.to_js(env),
  };

  if !consumed {
    canvas_c::canvas_native_webgl_WebGLResult_destroy(result);
  }

  parameter
}

#[napi]
impl web_g_l_rendering_context {
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

    Ok(web_g_l_rendering_context::from_raw(ret, true))
  }

  #[napi]
  pub fn get_parameter<'env>(&self, env: &'env Env, pname: u32) -> Result<Unknown<'env>> {
    get_parameter_inner(self.state, env, pname)
  }
}

impl_webgl_context_constants!(web_g_l_rendering_context);
