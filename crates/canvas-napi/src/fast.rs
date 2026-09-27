//! Raw Node-API callbacks for the hottest 2D and WebGL calls, installed over the napi-rs members on
//! the class prototypes.
//!
//! A napi-rs method or accessor creates a reference to `this` and registers a native borrow on
//! every call: about 300 ns, four to five times the rest of a call like `fillRect`. These unwrap
//! `this` directly (checking napi-rs's per-class type tag, so a foreign receiver still throws) and
//! never call back into JS, which is what makes skipping the borrow bookkeeping sound. Each mirrors
//! the napi-rs member it replaces.

use std::ffi::{c_char, c_void, CStr};
use std::ptr;

use napi::bindgen_prelude::TypeTag;
use napi::sys;

use crate::c2d::CanvasRenderingContext2D;
use crate::gl::webgl_buffer::WebGLBuffer;
use crate::gl::webgl_program::WebGLProgram;
use crate::gl::webgl_texture::WebGLTexture;
use crate::gl::webgl_uniform_location::WebGLUniformLocation;
use crate::gl::web_g_l_rendering_context;
use crate::gl2::web_g_l_2_rendering_context;

const OK: sys::napi_status = sys::Status::napi_ok;

unsafe fn tagged(env: sys::napi_env, value: sys::napi_value, tag: &sys::napi_type_tag) -> bool {
  let mut matches = false;
  unsafe { sys::napi_check_object_type_tag(env, value, tag, &mut matches) == OK && matches }
}

/// The native object behind `value` if it is a `T` wrapper.
unsafe fn unwrap<T: TypeTag>(env: sys::napi_env, value: sys::napi_value) -> Option<*mut T> {
  let mut kind = 0;
  if unsafe { sys::napi_typeof(env, value, &mut kind) } != OK || kind != sys::ValueType::napi_object {
    return None;
  }
  if !unsafe { tagged(env, value, &T::type_tag()) } {
    return None;
  }
  let mut data: *mut c_void = ptr::null_mut();
  (unsafe { sys::napi_unwrap(env, value, &mut data) } == OK && !data.is_null()).then_some(data.cast())
}

/// `this` and up to `N` arguments (missing ones are `undefined`).
unsafe fn call_info<const N: usize>(env: sys::napi_env, info: sys::napi_callback_info) -> (sys::napi_value, [sys::napi_value; N]) {
  let mut argc = N;
  let mut argv = [ptr::null_mut(); N];
  let mut this = ptr::null_mut();
  unsafe { sys::napi_get_cb_info(env, info, &mut argc, argv.as_mut_ptr(), &mut this, ptr::null_mut()) };
  if argc < N {
    let mut undefined = ptr::null_mut();
    unsafe { sys::napi_get_undefined(env, &mut undefined) };
    argv[argc..].iter_mut().for_each(|arg| *arg = undefined);
  }
  (this, argv)
}

unsafe fn illegal_invocation(env: sys::napi_env) -> sys::napi_value {
  unsafe { sys::napi_throw_type_error(env, ptr::null(), c"Illegal invocation".as_ptr()) };
  ptr::null_mut()
}

/// A number argument (WebIDL `ToNumber`: anything else is coerced).
unsafe fn num(env: sys::napi_env, value: sys::napi_value) -> f64 {
  let mut out = 0.;
  if unsafe { sys::napi_get_value_double(env, value, &mut out) } == OK {
    return out;
  }
  let mut number = ptr::null_mut();
  if unsafe { sys::napi_coerce_to_number(env, value, &mut number) } == OK
    && unsafe { sys::napi_get_value_double(env, number, &mut out) } == OK
  {
    return out;
  }
  f64::NAN
}

/// WebIDL `long` / `unsigned long` (modulo 2^32, NaN and infinities 0).
unsafe fn int(env: sys::napi_env, value: sys::napi_value) -> i32 {
  let v = unsafe { num(env, value) };
  if v.is_finite() {
    (v.trunc() as i64) as i32
  } else {
    0
  }
}

unsafe fn uint(env: sys::napi_env, value: sys::napi_value) -> u32 {
  unsafe { int(env, value) as u32 }
}

unsafe fn boolean(env: sys::napi_env, value: sys::napi_value) -> bool {
  let mut out = false;
  let mut coerced = ptr::null_mut();
  unsafe {
    sys::napi_coerce_to_bool(env, value, &mut coerced) == OK && sys::napi_get_value_bool(env, coerced, &mut out) == OK && out
  }
}

unsafe fn number(env: sys::napi_env, value: f64) -> sys::napi_value {
  let mut out = ptr::null_mut();
  unsafe { sys::napi_create_double(env, value, &mut out) };
  out
}

// -------------------------------------------------------------------------------------------- 2D

/// A 2D method on `this` with `N` number arguments.
macro_rules! context_2d {
  ($name:ident, $n:literal, |$ctx:ident, $a:ident| $body:expr) => {
    unsafe extern "C" fn $name(env: sys::napi_env, info: sys::napi_callback_info) -> sys::napi_value {
      let (this, argv) = unsafe { call_info::<$n>(env, info) };
      let Some(wrapper) = (unsafe { unwrap::<CanvasRenderingContext2D>(env, this) }) else {
        return unsafe { illegal_invocation(env) };
      };
      let wrapper = unsafe { &*wrapper };
      let $ctx = wrapper.context;
      #[allow(unused_variables)]
      let $a: [f32; $n] = std::array::from_fn(|i| unsafe { num(env, argv[i]) } as f32);
      #[allow(unused_unsafe)]
      let dirty: bool = unsafe { $body };
      if dirty {
        crate::frame::mark_dirty(&wrapper.frame);
      }
      ptr::null_mut()
    }
  };
}

context_2d!(save, 0, |c, a| { canvas_c::canvas_native_context_save(c); false });
context_2d!(restore, 0, |c, a| { canvas_c::canvas_native_context_restore(c); false });
context_2d!(reset_transform, 0, |c, a| { canvas_c::canvas_native_context_reset_transform(c); false });
context_2d!(begin_path, 0, |c, a| { canvas_c::canvas_native_context_begin_path(c); false });
context_2d!(close_path, 0, |c, a| { canvas_c::canvas_native_context_close_path(c); false });
context_2d!(translate, 2, |c, a| { canvas_c::canvas_native_context_translate(c, a[0], a[1]); false });
context_2d!(rotate, 1, |c, a| { canvas_c::canvas_native_context_rotate(c, a[0]); false });
context_2d!(scale, 2, |c, a| { canvas_c::canvas_native_context_scale(c, a[0], a[1]); false });
context_2d!(move_to, 2, |c, a| { canvas_c::canvas_native_context_move_to(c, a[0], a[1]); false });
context_2d!(line_to, 2, |c, a| { canvas_c::canvas_native_context_line_to(c, a[0], a[1]); false });
context_2d!(bezier_curve_to, 6, |c, a| {
  canvas_c::canvas_native_context_bezier_curve_to(c, a[0], a[1], a[2], a[3], a[4], a[5]);
  false
});
context_2d!(quadratic_curve_to, 4, |c, a| {
  canvas_c::canvas_native_context_quadratic_curve_to(c, a[0], a[1], a[2], a[3]);
  false
});
context_2d!(arc_to, 5, |c, a| { canvas_c::canvas_native_context_arc_to(c, a[0], a[1], a[2], a[3], a[4]); false });
context_2d!(rect, 4, |c, a| { canvas_c::canvas_native_context_rect(c, a[0], a[1], a[2], a[3]); false });
context_2d!(fill_rect, 4, |c, a| { canvas_c::canvas_native_context_fill_rect(c, a[0], a[1], a[2], a[3]); true });
context_2d!(stroke_rect, 4, |c, a| { canvas_c::canvas_native_context_stroke_rect(c, a[0], a[1], a[2], a[3]); true });
context_2d!(clear_rect, 4, |c, a| { canvas_c::canvas_native_context_clear_rect(c, a[0], a[1], a[2], a[3]); true });

/// `arc(x, y, radius, startAngle, endAngle, anticlockwise?)`.
unsafe extern "C" fn arc(env: sys::napi_env, info: sys::napi_callback_info) -> sys::napi_value {
  let (this, argv) = unsafe { call_info::<6>(env, info) };
  let Some(wrapper) = (unsafe { unwrap::<CanvasRenderingContext2D>(env, this) }) else {
    return unsafe { illegal_invocation(env) };
  };
  let a: [f32; 5] = std::array::from_fn(|i| unsafe { num(env, argv[i]) } as f32);
  let anticlockwise = unsafe { boolean(env, argv[5]) };
  canvas_c::canvas_native_context_arc(unsafe { (*wrapper).context }, a[0], a[1], a[2], a[3], a[4], anticlockwise);
  ptr::null_mut()
}

unsafe extern "C" fn get_line_width(env: sys::napi_env, info: sys::napi_callback_info) -> sys::napi_value {
  let (this, _) = unsafe { call_info::<0>(env, info) };
  match unsafe { unwrap::<CanvasRenderingContext2D>(env, this) } {
    Some(wrapper) => unsafe { number(env, (*(*wrapper).context).get_context().line_width() as f64) },
    None => unsafe { illegal_invocation(env) },
  }
}

unsafe extern "C" fn set_line_width(env: sys::napi_env, info: sys::napi_callback_info) -> sys::napi_value {
  let (this, argv) = unsafe { call_info::<1>(env, info) };
  match unsafe { unwrap::<CanvasRenderingContext2D>(env, this) } {
    Some(wrapper) => {
      let width = unsafe { num(env, argv[0]) } as f32;
      unsafe { (*(*wrapper).context).get_context_mut().set_line_width(width) };
      ptr::null_mut()
    }
    None => unsafe { illegal_invocation(env) },
  }
}

unsafe extern "C" fn get_global_alpha(env: sys::napi_env, info: sys::napi_callback_info) -> sys::napi_value {
  let (this, _) = unsafe { call_info::<0>(env, info) };
  match unsafe { unwrap::<CanvasRenderingContext2D>(env, this) } {
    Some(wrapper) => unsafe { number(env, (*(*wrapper).context).get_context().global_alpha() as f64) },
    None => unsafe { illegal_invocation(env) },
  }
}

unsafe extern "C" fn set_global_alpha(env: sys::napi_env, info: sys::napi_callback_info) -> sys::napi_value {
  let (this, argv) = unsafe { call_info::<1>(env, info) };
  match unsafe { unwrap::<CanvasRenderingContext2D>(env, this) } {
    Some(wrapper) => {
      canvas_c::canvas_native_context_set_global_alpha(unsafe { (*wrapper).context }, unsafe { num(env, argv[0]) } as f32);
      ptr::null_mut()
    }
    None => unsafe { illegal_invocation(env) },
  }
}

// ------------------------------------------------------------------------ fillStyle / strokeStyle

const FILL: usize = 0;
const STROKE: usize = 1;

thread_local! {
  /// napi-rs's own `fillStyle` / `strokeStyle` getter and setter: the getter (strings, gradients,
  /// patterns) and non-string values (CanvasGradient / CanvasPattern) go to them.
  static STYLE_ACCESSORS: std::cell::RefCell<[(sys::napi_ref, sys::napi_ref); 2]> =
    const { std::cell::RefCell::new([(ptr::null_mut(), ptr::null_mut()); 2]) };
  /// The last colour string parsed per style and its paint: a colour set again skips the CSS
  /// parse (a scene alternating two or three colours, or setting the same one per shape).
  static LAST_COLOR: std::cell::RefCell<[Option<(Vec<u8>, canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle)>; 2]> =
    const { std::cell::RefCell::new([None, None]) };
}

/// Calls napi-rs's accessor function for `which` (the getter, or the setter with `value`).
unsafe fn original_style(env: sys::napi_env, this: sys::napi_value, which: usize, value: Option<sys::napi_value>) -> sys::napi_value {
  let reference = STYLE_ACCESSORS.with(|a| {
    let accessors = a.borrow()[which];
    if value.is_some() { accessors.1 } else { accessors.0 }
  });
  let (mut function, mut result) = (ptr::null_mut(), ptr::null_mut());
  unsafe {
    if reference.is_null() || sys::napi_get_reference_value(env, reference, &mut function) != OK {
      return ptr::null_mut();
    }
    match value {
      Some(value) => sys::napi_call_function(env, this, function, 1, &value, &mut result),
      None => sys::napi_call_function(env, this, function, 0, ptr::null(), &mut result),
    };
  }
  result
}

/// A string's UTF-8 bytes: on the stack when short (colours are), else on the heap.
unsafe fn with_utf8<R>(env: sys::napi_env, value: sys::napi_value, f: impl FnOnce(&[u8]) -> R) -> R {
  let mut stack = [0u8; 64];
  let mut len = 0usize;
  unsafe { sys::napi_get_value_string_utf8(env, value, stack.as_mut_ptr() as *mut c_char, stack.len(), &mut len) };
  if len + 1 < stack.len() {
    return f(&stack[..len]);
  }
  let mut full = 0usize;
  unsafe { sys::napi_get_value_string_utf8(env, value, ptr::null_mut(), 0, &mut full) };
  let mut heap = vec![0u8; full + 1];
  unsafe { sys::napi_get_value_string_utf8(env, value, heap.as_mut_ptr() as *mut c_char, heap.len(), &mut len) };
  f(&heap[..len])
}

unsafe fn style_setter(env: sys::napi_env, info: sys::napi_callback_info, which: usize) -> sys::napi_value {
  use canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle;
  let (this, argv) = unsafe { call_info::<1>(env, info) };
  let mut kind = 0;
  unsafe { sys::napi_typeof(env, argv[0], &mut kind) };
  let wrapper = if kind == sys::ValueType::napi_string {
    unsafe { unwrap::<CanvasRenderingContext2D>(env, this) }
  } else {
    None
  };
  let Some(wrapper) = wrapper else {
    return unsafe { original_style(env, this, which, Some(argv[0])) };
  };
  let style = unsafe {
    with_utf8(env, argv[0], |bytes| {
      LAST_COLOR.with(|last| {
        let mut last = last.borrow_mut();
        if let Some((cached, style)) = &last[which] {
          if cached.as_slice() == bytes {
            return Some(style.clone());
          }
        }
        let parsed = std::str::from_utf8(bytes).ok().and_then(PaintStyle::new_color_str);
        if let Some(style) = &parsed {
          last[which] = Some((bytes.to_vec(), style.clone()));
        }
        parsed
      })
    })
  };
  // Not a colour: ignored, as on the web.
  if let Some(style) = style {
    let context = unsafe { (*(*wrapper).context).get_context_mut() };
    if which == FILL {
      context.set_fill_style(style);
    } else {
      context.set_stroke_style(style);
    }
  }
  ptr::null_mut()
}

/// A colour serialized here, as napi-rs's getter would (`#rrggbb` / `rgba(...)`); gradients and
/// patterns come from napi-rs's getter, which hands out their wrappers.
unsafe fn style_getter(env: sys::napi_env, info: sys::napi_callback_info, which: usize) -> sys::napi_value {
  use canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle;
  use canvas_2d::utils::color::{to_parsed_color, to_parsed_color_4f};
  let (this, _) = unsafe { call_info::<0>(env, info) };
  if let Some(wrapper) = unsafe { unwrap::<CanvasRenderingContext2D>(env, this) } {
    let context = unsafe { (*(*wrapper).context).get_context() };
    let style = if which == FILL { context.fill_style() } else { context.stroke_style() };
    let text = match style {
      PaintStyle::Color(color) => Some(to_parsed_color(*color)),
      PaintStyle::Color4f(color) => Some(to_parsed_color_4f(*color)),
      _ => None,
    };
    if let Some(text) = text {
      let mut value = ptr::null_mut();
      unsafe { sys::napi_create_string_utf8(env, text.as_ptr() as *const c_char, text.len() as isize, &mut value) };
      return value;
    }
  }
  unsafe { original_style(env, this, which, None) }
}

unsafe extern "C" fn get_fill_style(env: sys::napi_env, info: sys::napi_callback_info) -> sys::napi_value {
  unsafe { style_getter(env, info, FILL) }
}

unsafe extern "C" fn set_fill_style(env: sys::napi_env, info: sys::napi_callback_info) -> sys::napi_value {
  unsafe { style_setter(env, info, FILL) }
}

unsafe extern "C" fn get_stroke_style(env: sys::napi_env, info: sys::napi_callback_info) -> sys::napi_value {
  unsafe { style_getter(env, info, STROKE) }
}

unsafe extern "C" fn set_stroke_style(env: sys::napi_env, info: sys::napi_callback_info) -> sys::napi_value {
  unsafe { style_setter(env, info, STROKE) }
}

/// Keeps napi-rs's `name` accessor functions on `prototype` for the style accessors to call.
unsafe fn capture_style_accessor(env: sys::napi_env, prototype: sys::napi_value, name: &CStr, which: usize) -> napi::Result<()> {
  unsafe {
    let (mut global, mut object, mut get_descriptor, mut key, mut descriptor) =
      (ptr::null_mut(), ptr::null_mut(), ptr::null_mut(), ptr::null_mut(), ptr::null_mut());
    napi::check_status!(sys::napi_get_global(env, &mut global))?;
    napi::check_status!(sys::napi_get_named_property(env, global, c"Object".as_ptr(), &mut object))?;
    napi::check_status!(sys::napi_get_named_property(env, object, c"getOwnPropertyDescriptor".as_ptr(), &mut get_descriptor))?;
    napi::check_status!(sys::napi_create_string_utf8(env, name.as_ptr(), -1, &mut key))?;
    let args = [prototype, key];
    napi::check_status!(sys::napi_call_function(env, object, get_descriptor, 2, args.as_ptr(), &mut descriptor))?;
    let (mut get, mut set) = (ptr::null_mut(), ptr::null_mut());
    napi::check_status!(sys::napi_get_named_property(env, descriptor, c"get".as_ptr(), &mut get))?;
    napi::check_status!(sys::napi_get_named_property(env, descriptor, c"set".as_ptr(), &mut set))?;
    let (mut get_ref, mut set_ref) = (ptr::null_mut(), ptr::null_mut());
    napi::check_status!(sys::napi_create_reference(env, get, 1, &mut get_ref))?;
    napi::check_status!(sys::napi_create_reference(env, set, 1, &mut set_ref))?;
    STYLE_ACCESSORS.with(|a| a.borrow_mut()[which] = (get_ref, set_ref));
  }
  Ok(())
}

unsafe extern "C" fn forget_style_accessors(_: *mut c_void) {
  // The env (and its references) is going away.
  STYLE_ACCESSORS.with(|a| *a.borrow_mut() = [(ptr::null_mut(), ptr::null_mut()); 2]);
  LAST_COLOR.with(|last| *last.borrow_mut() = [None, None]);
}

// ----------------------------------------------------------------------------------------- WebGL

/// The WebGL or WebGL 2 context behind `this`: its state, invalidation flags and frame slot.
struct Gl {
  state: *mut canvas_c::WebGLState,
  invalidate: *mut u32,
  frame: *const std::rc::Rc<crate::frame::FrameSlot>,
}

impl Gl {
  /// What the napi-rs `update_invalidate_state` does: pending, presented at frame end.
  unsafe fn drew(&self) {
    unsafe { *self.invalidate |= canvas_c::InvalidateState::Pending as u32 };
    crate::frame::mark_dirty(unsafe { &*self.frame });
  }
}

unsafe fn gl(env: sys::napi_env, this: sys::napi_value) -> Option<Gl> {
  if let Some(context) = unsafe { unwrap::<web_g_l_rendering_context>(env, this) } {
    let context = unsafe { &mut *context };
    return Some(Gl { state: context.state, invalidate: &mut context.invalidate_state, frame: &context.frame });
  }
  let context = unsafe { &mut *unwrap::<web_g_l_2_rendering_context>(env, this)? };
  Some(Gl { state: context.state, invalidate: &mut context.invalidate_state, frame: &context.frame })
}

/// A GL object's name: 0 for null/undefined (and anything that is not a `T`).
unsafe fn name<T: TypeTag>(env: sys::napi_env, value: sys::napi_value, get: impl Fn(&T) -> u32) -> u32 {
  unsafe { unwrap::<T>(env, value) }.map_or(0, |object| get(unsafe { &*object }))
}

/// A WebGL method on `this` with `N` arguments.
macro_rules! webgl {
  ($name:ident, $n:literal, |$env:ident, $gl:ident, $a:ident| $body:expr) => {
    unsafe extern "C" fn $name($env: sys::napi_env, info: sys::napi_callback_info) -> sys::napi_value {
      let (this, $a) = unsafe { call_info::<$n>($env, info) };
      let Some($gl) = (unsafe { gl($env, this) }) else {
        return unsafe { illegal_invocation($env) };
      };
      #[allow(unused_unsafe)]
      unsafe {
        $body
      }
    }
  };
}

/// A uniform location, or `None` for null (the call is then a no-op, as in WebGL).
unsafe fn location(env: sys::napi_env, value: sys::napi_value) -> Option<i32> {
  unsafe { unwrap::<WebGLUniformLocation>(env, value) }.map(|location| unsafe { (*location).0 })
}

webgl!(uniform1f, 2, |env, gl, a| {
  if let Some(l) = location(env, a[0]) {
    canvas_c::canvas_native_webgl_uniform1f(l, num(env, a[1]) as f32, gl.state);
  }
  ptr::null_mut()
});
webgl!(uniform2f, 3, |env, gl, a| {
  if let Some(l) = location(env, a[0]) {
    canvas_c::canvas_native_webgl_uniform2f(l, num(env, a[1]) as f32, num(env, a[2]) as f32, gl.state);
  }
  ptr::null_mut()
});
webgl!(uniform3f, 4, |env, gl, a| {
  if let Some(l) = location(env, a[0]) {
    canvas_c::canvas_native_webgl_uniform3f(l, num(env, a[1]) as f32, num(env, a[2]) as f32, num(env, a[3]) as f32, gl.state);
  }
  ptr::null_mut()
});
webgl!(uniform4f, 5, |env, gl, a| {
  if let Some(l) = location(env, a[0]) {
    canvas_c::canvas_native_webgl_uniform4f(
      l,
      num(env, a[1]) as f32,
      num(env, a[2]) as f32,
      num(env, a[3]) as f32,
      num(env, a[4]) as f32,
      gl.state,
    );
  }
  ptr::null_mut()
});
webgl!(uniform1i, 2, |env, gl, a| {
  if let Some(l) = location(env, a[0]) {
    // A boolean is 1 or 0 (ToNumber).
    canvas_c::canvas_native_webgl_uniform1i(l, int(env, a[1]), gl.state);
  }
  ptr::null_mut()
});

/// `uniformMatrix{2,3,4}fv(location, transpose, Float32Array | number[])`.
macro_rules! uniform_matrix {
  ($name:ident, $c:path) => {
    webgl!($name, 3, |env, gl, a| {
      if let Some(l) = location(env, a[0]) {
        let transpose = boolean(env, a[1]);
        let (mut is_typed, mut kind, mut length, mut data) = (false, 0, 0usize, ptr::null_mut());
        sys::napi_is_typedarray(env, a[2], &mut is_typed);
        if is_typed
          && sys::napi_get_typedarray_info(env, a[2], &mut kind, &mut length, &mut data, ptr::null_mut(), ptr::null_mut()) == OK
          && kind == sys::TypedarrayType::float32_array
        {
          $c(l, transpose, data as *const f32, length, gl.state);
        } else {
          let mut length = 0u32;
          if sys::napi_get_array_length(env, a[2], &mut length) == OK {
            let values: Vec<f32> = (0..length)
              .map(|i| {
                let mut element = ptr::null_mut();
                sys::napi_get_element(env, a[2], i, &mut element);
                num(env, element) as f32
              })
              .collect();
            $c(l, transpose, values.as_ptr(), values.len(), gl.state);
          }
        }
      }
      ptr::null_mut()
    });
  };
}
uniform_matrix!(uniform_matrix2fv, canvas_c::canvas_native_webgl_uniform_matrix2fv);
uniform_matrix!(uniform_matrix3fv, canvas_c::canvas_native_webgl_uniform_matrix3fv);
uniform_matrix!(uniform_matrix4fv, canvas_c::canvas_native_webgl_uniform_matrix4fv);

webgl!(viewport, 4, |env, gl, a| {
  canvas_c::canvas_native_webgl_viewport(int(env, a[0]), int(env, a[1]), int(env, a[2]), int(env, a[3]), gl.state);
  ptr::null_mut()
});
webgl!(clear, 1, |env, gl, a| {
  canvas_c::canvas_native_webgl_clear(uint(env, a[0]), gl.state);
  gl.drew();
  ptr::null_mut()
});
webgl!(clear_color, 4, |env, gl, a| {
  canvas_c::canvas_native_webgl_clear_color(
    num(env, a[0]) as f32,
    num(env, a[1]) as f32,
    num(env, a[2]) as f32,
    num(env, a[3]) as f32,
    gl.state,
  );
  ptr::null_mut()
});
webgl!(enable, 1, |env, gl, a| {
  canvas_c::canvas_native_webgl_enable(uint(env, a[0]), gl.state);
  ptr::null_mut()
});
webgl!(disable, 1, |env, gl, a| {
  canvas_c::canvas_native_webgl_disable(uint(env, a[0]), gl.state);
  ptr::null_mut()
});
webgl!(active_texture, 1, |env, gl, a| {
  canvas_c::canvas_native_webgl_active_texture(uint(env, a[0]), gl.state);
  ptr::null_mut()
});
webgl!(enable_vertex_attrib_array, 1, |env, gl, a| {
  canvas_c::canvas_native_webgl_enable_vertex_attrib_array(uint(env, a[0]), gl.state);
  ptr::null_mut()
});
webgl!(vertex_attrib_pointer, 6, |env, gl, a| {
  canvas_c::canvas_native_webgl_vertex_attrib_pointer(
    uint(env, a[0]),
    int(env, a[1]),
    uint(env, a[2]),
    boolean(env, a[3]),
    int(env, a[4]),
    num(env, a[5]) as isize,
    gl.state,
  );
  ptr::null_mut()
});
webgl!(bind_buffer, 2, |env, gl, a| {
  canvas_c::canvas_native_webgl_bind_buffer(uint(env, a[0]), name::<WebGLBuffer>(env, a[1], |b| b.0), gl.state);
  ptr::null_mut()
});
webgl!(bind_texture, 2, |env, gl, a| {
  canvas_c::canvas_native_webgl_bind_texture(uint(env, a[0]), name::<WebGLTexture>(env, a[1], |t| t.0), gl.state);
  ptr::null_mut()
});
webgl!(use_program, 1, |env, gl, a| {
  canvas_c::canvas_native_webgl_use_program(name::<WebGLProgram>(env, a[0], |p| p.0), gl.state);
  ptr::null_mut()
});
webgl!(draw_arrays, 3, |env, gl, a| {
  canvas_c::canvas_native_webgl_draw_arrays(uint(env, a[0]), int(env, a[1]), int(env, a[2]), gl.state);
  gl.drew();
  ptr::null_mut()
});
webgl!(draw_elements, 4, |env, gl, a| {
  canvas_c::canvas_native_webgl_draw_elements(uint(env, a[0]), int(env, a[1]), uint(env, a[2]), num(env, a[3]) as isize, gl.state);
  gl.drew();
  ptr::null_mut()
});

// --------------------------------------------------------------------------------------- install

type Callback = unsafe extern "C" fn(sys::napi_env, sys::napi_callback_info) -> sys::napi_value;

enum Member {
  Method(&'static CStr, Callback),
  Accessor(&'static CStr, Callback, Callback),
}

const CONTEXT_2D: &[Member] = &[
  Member::Method(c"save", save),
  Member::Method(c"restore", restore),
  Member::Method(c"resetTransform", reset_transform),
  Member::Method(c"beginPath", begin_path),
  Member::Method(c"closePath", close_path),
  Member::Method(c"translate", translate),
  Member::Method(c"rotate", rotate),
  Member::Method(c"scale", scale),
  Member::Method(c"moveTo", move_to),
  Member::Method(c"lineTo", line_to),
  Member::Method(c"bezierCurveTo", bezier_curve_to),
  Member::Method(c"quadraticCurveTo", quadratic_curve_to),
  Member::Method(c"arcTo", arc_to),
  Member::Method(c"arc", arc),
  Member::Method(c"rect", rect),
  Member::Method(c"fillRect", fill_rect),
  Member::Method(c"strokeRect", stroke_rect),
  Member::Method(c"clearRect", clear_rect),
  Member::Accessor(c"lineWidth", get_line_width, set_line_width),
  Member::Accessor(c"globalAlpha", get_global_alpha, set_global_alpha),
  Member::Accessor(c"fillStyle", get_fill_style, set_fill_style),
  Member::Accessor(c"strokeStyle", get_stroke_style, set_stroke_style),
];

const WEBGL: &[Member] = &[
  Member::Method(c"uniform1f", uniform1f),
  Member::Method(c"uniform2f", uniform2f),
  Member::Method(c"uniform3f", uniform3f),
  Member::Method(c"uniform4f", uniform4f),
  Member::Method(c"uniform1i", uniform1i),
  Member::Method(c"uniformMatrix2fv", uniform_matrix2fv),
  Member::Method(c"uniformMatrix3fv", uniform_matrix3fv),
  Member::Method(c"uniformMatrix4fv", uniform_matrix4fv),
  Member::Method(c"viewport", viewport),
  Member::Method(c"clear", clear),
  Member::Method(c"clearColor", clear_color),
  Member::Method(c"enable", enable),
  Member::Method(c"disable", disable),
  Member::Method(c"activeTexture", active_texture),
  Member::Method(c"enableVertexAttribArray", enable_vertex_attrib_array),
  Member::Method(c"vertexAttribPointer", vertex_attrib_pointer),
  Member::Method(c"bindBuffer", bind_buffer),
  Member::Method(c"bindTexture", bind_texture),
  Member::Method(c"useProgram", use_program),
  Member::Method(c"drawArrays", draw_arrays),
  Member::Method(c"drawElements", draw_elements),
];

unsafe fn prototype_of(env: sys::napi_env, exports: sys::napi_value, class: &CStr) -> napi::Result<sys::napi_value> {
  let (mut constructor, mut prototype) = (ptr::null_mut(), ptr::null_mut());
  unsafe {
    napi::check_status!(sys::napi_get_named_property(env, exports, class.as_ptr(), &mut constructor))?;
    napi::check_status!(sys::napi_get_named_property(env, constructor, c"prototype".as_ptr(), &mut prototype))?;
  }
  Ok(prototype)
}

unsafe fn install_on(env: sys::napi_env, exports: sys::napi_value, class: &CStr, members: &[Member]) -> napi::Result<()> {
  let prototype = unsafe { prototype_of(env, exports, class) }?;
  // As napi-rs defines class members: configurable, writable (methods), enumerable.
  let attributes = sys::PropertyAttributes::configurable | sys::PropertyAttributes::enumerable;
  let descriptors: Vec<sys::napi_property_descriptor> = members
    .iter()
    .map(|member| match member {
      Member::Method(name, method) => sys::napi_property_descriptor {
        utf8name: name.as_ptr() as *const c_char,
        name: ptr::null_mut(),
        method: Some(*method),
        getter: None,
        setter: None,
        value: ptr::null_mut(),
        attributes: attributes | sys::PropertyAttributes::writable,
        data: ptr::null_mut(),
      },
      Member::Accessor(name, getter, setter) => sys::napi_property_descriptor {
        utf8name: name.as_ptr() as *const c_char,
        name: ptr::null_mut(),
        method: None,
        getter: Some(*getter),
        setter: Some(*setter),
        value: ptr::null_mut(),
        attributes,
        data: ptr::null_mut(),
      },
    })
    .collect();
  unsafe { napi::check_status!(sys::napi_define_properties(env, prototype, descriptors.len(), descriptors.as_ptr())) }
}

/// Replaces the napi-rs members above on the classes in `exports`. Opt out with
/// `CANVAS_NAPI_FAST=0` (to compare, or rule these out).
pub fn install(env: sys::napi_env, exports: sys::napi_value) -> napi::Result<()> {
  if std::env::var("CANVAS_NAPI_FAST").is_ok_and(|v| v == "0") {
    return Ok(());
  }
  unsafe {
    let prototype = prototype_of(env, exports, c"CanvasRenderingContext2D")?;
    capture_style_accessor(env, prototype, c"fillStyle", FILL)?;
    capture_style_accessor(env, prototype, c"strokeStyle", STROKE)?;
    napi::check_status!(sys::napi_add_env_cleanup_hook(env, Some(forget_style_accessors), ptr::null_mut()))?;
    install_on(env, exports, c"CanvasRenderingContext2D", CONTEXT_2D)?;
    install_on(env, exports, c"WebGLRenderingContext", WEBGL)?;
    install_on(env, exports, c"WebGL2RenderingContext", WEBGL)?;
  }
  Ok(())
}
