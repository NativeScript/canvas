//! Lenient readers for the option bags `packages/canvas` hands the WebGPU objects. As in the V8
//! bindings (`GPUUtils.h` and friends), a field of the wrong type keeps its default instead of
//! throwing; only values canvas-c cannot do without (a shader module, a texture to copy into)
//! are reported, as TypeErrors, where the V8 bindings would dereference null.

use std::ffi::{c_char, CStr, CString};
use std::ptr;

use canvas_c::webgpu::enums::{
  CanvasCompareFunction, CanvasGPUTextureFormat, CanvasOptionalGPUTextureFormat,
  CanvasStencilOperation, CanvasTextureAspect, CanvasTextureViewDimension,
};
use canvas_c::webgpu::gpu_device::CanvasConstants;
use canvas_c::webgpu::structs::{
  CanvasBlendFactor, CanvasBlendOperation, CanvasColor, CanvasExtent3d, CanvasOptionalColor,
  CanvasOrigin2d, CanvasOrigin3d,
};
use napi::bindgen_prelude::{ClassInstance, FromNapiValue, Unknown, ValidateNapiValue};
use napi::{sys, Error, JsValue, Result, Status, ValueType};

pub(crate) use crate::module::{as_bool, as_number, as_string, downcast, property, type_of};

/// `object[name]`, unless it is `undefined` or `null`.
pub(crate) fn field<'a>(object: &Unknown<'a>, name: &CStr) -> Option<Unknown<'a>> {
  property(object, name).filter(|value| !is_nullish(value))
}

pub(crate) fn is_nullish(value: &Unknown) -> bool {
  matches!(type_of(value), ValueType::Undefined | ValueType::Null)
}

pub(crate) fn is_object(value: &Unknown) -> bool {
  type_of(value) == ValueType::Object
}

pub(crate) fn string(object: &Unknown, name: &CStr) -> Option<String> {
  field(object, name).and_then(|value| as_string(&value))
}

pub(crate) fn number(object: &Unknown, name: &CStr) -> Option<f64> {
  field(object, name).and_then(|value| as_number(&value))
}

pub(crate) fn boolean(object: &Unknown, name: &CStr) -> Option<bool> {
  field(object, name).and_then(|value| as_bool(&value))
}

/// V8's `IsUint32`: an integral number in `0..=u32::MAX`.
pub(crate) fn uint32_value(value: &Unknown) -> Option<u32> {
  as_number(value)
    .filter(|n| n.fract() == 0. && *n >= 0. && *n <= u32::MAX as f64)
    .map(|n| n as u32)
}

/// V8's `IsInt32`.
pub(crate) fn int32_value(value: &Unknown) -> Option<i32> {
  as_number(value)
    .filter(|n| n.fract() == 0. && *n >= i32::MIN as f64 && *n <= i32::MAX as f64)
    .map(|n| n as i32)
}

pub(crate) fn uint32(object: &Unknown, name: &CStr) -> Option<u32> {
  field(object, name).and_then(|value| uint32_value(&value))
}

pub(crate) fn int32(object: &Unknown, name: &CStr) -> Option<i32> {
  field(object, name).and_then(|value| int32_value(&value))
}

/// `object[name]` as a `#[napi]` class instance, if it is one.
pub(crate) fn class<'a, T: 'a>(object: &Unknown<'a>, name: &CStr) -> Option<ClassInstance<'a, T>>
where
  ClassInstance<'a, T>: FromNapiValue + ValidateNapiValue,
{
  field(object, name).and_then(|value| downcast::<T>(&value))
}

/// The items of a JS array (holes read as `undefined`), `None` for anything else.
pub(crate) fn array<'a>(value: &Unknown<'a>) -> Option<Vec<Unknown<'a>>> {
  let raw = value.value();
  let mut is_array = false;
  let status = unsafe { sys::napi_is_array(raw.env, raw.value, &mut is_array) };
  if status != sys::Status::napi_ok || !is_array {
    return None;
  }
  let mut length = 0;
  unsafe { sys::napi_get_array_length(raw.env, raw.value, &mut length) };
  let mut items = Vec::with_capacity(length as usize);
  for i in 0..length {
    let mut item = ptr::null_mut();
    if unsafe { sys::napi_get_element(raw.env, raw.value, i, &mut item) } != sys::Status::napi_ok {
      return None;
    }
    items.push(unsafe { Unknown::from_raw_unchecked(raw.env, item) });
  }
  Some(items)
}

pub(crate) fn array_field<'a>(object: &Unknown<'a>, name: &CStr) -> Option<Vec<Unknown<'a>>> {
  field(object, name).and_then(|value| array(&value))
}

/// `GPULabel`: `object.label` when it is a string.
pub(crate) fn label(object: &Unknown) -> Option<CString> {
  string(object, c"label").and_then(|label| CString::new(label).ok())
}

pub(crate) fn c_str(value: &Option<CString>) -> *const c_char {
  value.as_ref().map_or(ptr::null(), |value| value.as_ptr())
}

/// A string canvas-c hands over (`CString::into_raw`), taken back.
pub(crate) unsafe fn take_string(value: *mut c_char) -> Option<String> {
  (!value.is_null()).then(|| {
    unsafe { CString::from_raw(value) }
      .to_string_lossy()
      .into_owned()
  })
}

pub(crate) fn type_error(message: impl Into<String>) -> Error {
  Error::new(Status::InvalidArg, message.into())
}

// ---------------------------------------------------------------------------------------------
// Dictionaries
// ---------------------------------------------------------------------------------------------

/// `ParseExtent3d`: `[w, h?, d?]` or `{width, height?, depthOrArrayLayers?}`; defaults (0, 1, 1).
pub(crate) fn extent3d(value: Option<&Unknown>) -> CanvasExtent3d {
  let mut ret = CanvasExtent3d {
    width: 0,
    height: 1,
    depth_or_array_layers: 1,
  };
  let Some(value) = value else {
    return ret;
  };
  if let Some(items) = array(value) {
    let at = |i: usize| items.get(i).and_then(uint32_value);
    if let Some(v) = at(0) {
      ret.width = v;
    }
    if let Some(v) = at(1) {
      ret.height = v;
    }
    if let Some(v) = at(2) {
      ret.depth_or_array_layers = v;
    }
  } else if is_object(value) {
    if let Some(v) = uint32(value, c"width") {
      ret.width = v;
    }
    if let Some(v) = uint32(value, c"height") {
      ret.height = v;
    }
    if let Some(v) = uint32(value, c"depthOrArrayLayers") {
      ret.depth_or_array_layers = v;
    }
  }
  ret
}

/// `{x, y, z}` (or `[x, y, z]`), missing coordinates 0.
pub(crate) fn origin3d(value: Option<&Unknown>) -> CanvasOrigin3d {
  let mut ret = CanvasOrigin3d { x: 0, y: 0, z: 0 };
  let Some(value) = value else {
    return ret;
  };
  if let Some(items) = array(value) {
    let at = |i: usize| items.get(i).and_then(uint32_value).unwrap_or(0);
    ret = CanvasOrigin3d {
      x: at(0),
      y: at(1),
      z: at(2),
    };
  } else if is_object(value) {
    ret.x = uint32(value, c"x").unwrap_or(0);
    ret.y = uint32(value, c"y").unwrap_or(0);
    ret.z = uint32(value, c"z").unwrap_or(0);
  }
  ret
}

pub(crate) fn origin2d(value: Option<&Unknown>) -> CanvasOrigin2d {
  let origin = origin3d(value);
  CanvasOrigin2d {
    x: origin.x,
    y: origin.y,
  }
}

/// `ParseColor`: `[r, g, b, a]` or `{r, g, b, a}`; anything else is no colour.
pub(crate) fn color(value: Option<&Unknown>) -> CanvasOptionalColor {
  let Some(value) = value else {
    return CanvasOptionalColor::None;
  };
  let mut color = CanvasColor {
    r: 0.,
    g: 0.,
    b: 0.,
    a: 0.,
  };
  if let Some(items) = array(value) {
    let at = |i: usize| items.get(i).and_then(as_number).unwrap_or(0.);
    color = CanvasColor {
      r: at(0),
      g: at(1),
      b: at(2),
      a: at(3),
    };
  } else if is_object(value) {
    color.r = number(value, c"r").unwrap_or(0.);
    color.g = number(value, c"g").unwrap_or(0.);
    color.b = number(value, c"b").unwrap_or(0.);
    color.a = number(value, c"a").unwrap_or(0.);
  } else {
    return CanvasOptionalColor::None;
  }
  CanvasOptionalColor::Some(color)
}

/// Pipeline-overridable `constants`: a plain object (what the web and packages/canvas pass) or a
/// `Map` (what the V8 bindings read). Non-number values are skipped.
pub(crate) fn constants(value: Option<&Unknown>) -> Option<CanvasConstants> {
  let value = value.filter(|value| is_object(value))?;
  let raw = value.value();
  let env = raw.env;
  let mut pairs: Vec<(String, f64)> = Vec::new();
  unsafe {
    let mut global = ptr::null_mut();
    let mut map_ctor = ptr::null_mut();
    let mut is_map = false;
    if sys::napi_get_global(env, &mut global) == sys::Status::napi_ok
      && sys::napi_get_named_property(env, global, c"Map".as_ptr(), &mut map_ctor)
        == sys::Status::napi_ok
    {
      sys::napi_instanceof(env, raw.value, map_ctor, &mut is_map);
    }
    if is_map {
      // [[key, value], ...] via Array.from(map).
      let mut array_ctor = ptr::null_mut();
      let mut from = ptr::null_mut();
      let mut entries = ptr::null_mut();
      if sys::napi_get_named_property(env, global, c"Array".as_ptr(), &mut array_ctor)
        == sys::Status::napi_ok
        && sys::napi_get_named_property(env, array_ctor, c"from".as_ptr(), &mut from)
          == sys::Status::napi_ok
        && sys::napi_call_function(env, array_ctor, from, 1, &raw.value, &mut entries)
          == sys::Status::napi_ok
      {
        let entries = Unknown::from_raw_unchecked(env, entries);
        for entry in array(&entries).unwrap_or_default() {
          let Some(pair) = array(&entry) else { continue };
          if let (Some(key), Some(value)) = (
            pair.first().and_then(as_string),
            pair.get(1).and_then(as_number),
          ) {
            pairs.push((key, value));
          }
        }
      }
    } else {
      let mut names = ptr::null_mut();
      if sys::napi_get_all_property_names(
        env,
        raw.value,
        sys::KeyCollectionMode::own_only,
        sys::KeyFilter::enumerable | sys::KeyFilter::skip_symbols,
        sys::KeyConversion::numbers_to_strings,
        &mut names,
      ) == sys::Status::napi_ok
      {
        let names = Unknown::from_raw_unchecked(env, names);
        for name in array(&names).unwrap_or_default() {
          let Some(key) = as_string(&name) else {
            continue;
          };
          let Ok(c_key) = CString::new(key.clone()) else {
            continue;
          };
          if let Some(value) = property(value, &c_key).and_then(|v| as_number(&v)) {
            pairs.push((key, value));
          }
        }
      }
    }
  }
  if pairs.is_empty() {
    return None;
  }
  let mut ret = CanvasConstants::default();
  for (key, value) in pairs {
    if let Ok(key) = CString::new(key) {
      unsafe {
        canvas_c::webgpu::gpu_device::canvas_native_webgpu_constants_insert(
          &mut ret,
          key.as_ptr(),
          value,
        )
      };
    }
  }
  Some(ret)
}

// ---------------------------------------------------------------------------------------------
// Enums (the WebGPU strings)
// ---------------------------------------------------------------------------------------------

pub(crate) fn texture_format(value: &str) -> Option<CanvasGPUTextureFormat> {
  let value = CString::new(value).ok()?;
  match unsafe {
    canvas_c::webgpu::enums::canvas_native_webgpu_enum_string_to_gpu_texture(value.as_ptr())
  } {
    CanvasOptionalGPUTextureFormat::Some(format) => Some(format),
    CanvasOptionalGPUTextureFormat::None => None,
  }
}

pub(crate) fn texture_format_field(
  object: &Unknown,
  name: &CStr,
) -> Option<CanvasGPUTextureFormat> {
  string(object, name).and_then(|value| texture_format(&value))
}

/// Formats of a `viewFormats` / `colorFormats` array (unknown names skipped).
pub(crate) fn texture_formats(value: Option<&Unknown>) -> Vec<CanvasGPUTextureFormat> {
  value
    .and_then(array)
    .unwrap_or_default()
    .iter()
    .filter_map(as_string)
    .filter_map(|value| texture_format(&value))
    .collect()
}

pub(crate) fn texture_format_name(format: CanvasGPUTextureFormat) -> String {
  let name = canvas_c::webgpu::enums::canvas_native_webgpu_enum_gpu_texture_to_string(format);
  unsafe { take_string(name) }.unwrap_or_default()
}

pub(crate) fn aspect(value: Option<String>) -> CanvasTextureAspect {
  match value.as_deref() {
    Some("stencil-only") => CanvasTextureAspect::StencilOnly,
    Some("depth-only") => CanvasTextureAspect::DepthOnly,
    _ => CanvasTextureAspect::All,
  }
}

pub(crate) fn view_dimension(value: &str) -> Option<CanvasTextureViewDimension> {
  Some(match value {
    "1d" => CanvasTextureViewDimension::D1,
    "2d" => CanvasTextureViewDimension::D2,
    "2d-array" => CanvasTextureViewDimension::D2Array,
    "cube" => CanvasTextureViewDimension::Cube,
    "cube-array" => CanvasTextureViewDimension::CubeArray,
    "3d" => CanvasTextureViewDimension::D3,
    _ => return None,
  })
}

pub(crate) fn compare_function(value: Option<String>) -> Option<CanvasCompareFunction> {
  Some(match value.as_deref()? {
    "never" => CanvasCompareFunction::Never,
    "less" => CanvasCompareFunction::Less,
    "equal" => CanvasCompareFunction::Equal,
    "less-equal" => CanvasCompareFunction::LessEqual,
    "greater" => CanvasCompareFunction::Greater,
    "not-equal" => CanvasCompareFunction::NotEqual,
    "greater-equal" => CanvasCompareFunction::GreaterEqual,
    "always" => CanvasCompareFunction::Always,
    _ => return None,
  })
}

pub(crate) fn stencil_operation(value: Option<String>) -> Option<CanvasStencilOperation> {
  Some(match value.as_deref()? {
    "keep" => CanvasStencilOperation::Keep,
    "zero" => CanvasStencilOperation::Zero,
    "replace" => CanvasStencilOperation::Replace,
    "invert" => CanvasStencilOperation::Invert,
    "increment-clamp" => CanvasStencilOperation::IncrementClamp,
    "decrement-clamp" => CanvasStencilOperation::DecrementClamp,
    "increment-wrap" => CanvasStencilOperation::IncrementWrap,
    "decrement-wrap" => CanvasStencilOperation::DecrementWrap,
    _ => return None,
  })
}

pub(crate) fn blend_factor(value: Option<String>) -> Option<CanvasBlendFactor> {
  Some(match value.as_deref()? {
    "zero" => CanvasBlendFactor::Zero,
    "one" => CanvasBlendFactor::One,
    "src" => CanvasBlendFactor::Src,
    "one-minus-src" => CanvasBlendFactor::OneMinusSrc,
    "src-alpha" => CanvasBlendFactor::SrcAlpha,
    "one-minus-src-alpha" => CanvasBlendFactor::OneMinusSrcAlpha,
    "dst" => CanvasBlendFactor::Dst,
    "one-minus-dst" => CanvasBlendFactor::OneMinusDst,
    "dst-alpha" => CanvasBlendFactor::DstAlpha,
    "one-minus-dst-alpha" => CanvasBlendFactor::OneMinusDstAlpha,
    "src-alpha-saturated" => CanvasBlendFactor::SrcAlphaSaturated,
    "constant" => CanvasBlendFactor::Constant,
    "one-minus-constant" => CanvasBlendFactor::OneMinusConstant,
    _ => return None,
  })
}

pub(crate) fn blend_operation(value: Option<String>) -> Option<CanvasBlendOperation> {
  Some(match value.as_deref()? {
    "add" => CanvasBlendOperation::Add,
    "subtract" => CanvasBlendOperation::Subtract,
    "reverse-subtract" => CanvasBlendOperation::ReverseSubtract,
    "min" => CanvasBlendOperation::Min,
    "max" => CanvasBlendOperation::Max,
    _ => return None,
  })
}

/// `setBindGroup`'s dynamic offsets: a `Uint32Array` (with `start` / `length`, defaulting to the
/// whole array) or an array of numbers, checked against the data so canvas-c's bounds asserts
/// never fire.
pub(crate) fn dynamic_offsets(
  data: Option<&Unknown>,
  start: Option<f64>,
  length: Option<f64>,
) -> Result<Vec<u32>> {
  let Some(data) = data.filter(|data| !is_nullish(data)) else {
    return Ok(Vec::new());
  };
  let values: Vec<u32> = if let Some(items) = array(data) {
    items
      .iter()
      .map(|item| as_number(item).map_or(0, |n| n.max(0.) as u32))
      .collect()
  } else {
    let raw = data.value();
    let mut is_typedarray = false;
    unsafe { sys::napi_is_typedarray(raw.env, raw.value, &mut is_typedarray) };
    if !is_typedarray {
      return Ok(Vec::new());
    }
    let (mut kind, mut len, mut ptr_) = (0, 0, ptr::null_mut());
    let (mut arraybuffer, mut offset) = (ptr::null_mut(), 0);
    unsafe {
      sys::napi_get_typedarray_info(
        raw.env,
        raw.value,
        &mut kind,
        &mut len,
        &mut ptr_,
        &mut arraybuffer,
        &mut offset,
      )
    };
    if kind != sys::TypedarrayType::uint32_array {
      return Err(type_error("dynamicOffsetsData is not a Uint32Array"));
    }
    if ptr_.is_null() || len == 0 {
      Vec::new()
    } else {
      unsafe { std::slice::from_raw_parts(ptr_ as *const u32, len) }.to_vec()
    }
  };
  let start = start.map_or(0, |n| n.max(0.) as usize);
  if start > values.len() {
    return Err(Error::new(
      Status::InvalidArg,
      "dynamicOffsetsDataStart is out of range",
    ));
  }
  let length = length.map_or(values.len() - start, |n| n.max(0.) as usize);
  if length > values.len() - start {
    return Err(Error::new(
      Status::InvalidArg,
      "dynamicOffsetsDataLength is out of range",
    ));
  }
  Ok(values[start..start + length].to_vec())
}

/// An index format: packages/canvas's int (0 uint16, 1 uint32) or the string.
pub(crate) fn index_format(value: &Unknown) -> canvas_c::webgpu::enums::CanvasIndexFormat {
  use canvas_c::webgpu::enums::CanvasIndexFormat;
  match (uint32_value(value), as_string(value).as_deref()) {
    (Some(0), _) | (_, Some("uint16")) => CanvasIndexFormat::Uint16,
    _ => CanvasIndexFormat::Uint32,
  }
}

/// A buffer range argument: absent, negative or not a number is -1 (canvas-c's "default").
pub(crate) fn range_arg(value: Option<f64>) -> i64 {
  value.map_or(-1, |n| if n.is_nan() || n < 0. { -1 } else { n as i64 })
}

pub(crate) fn color_value(value: &Unknown) -> CanvasColor {
  match color(Some(value)) {
    CanvasOptionalColor::Some(color) => color,
    CanvasOptionalColor::None => CanvasColor {
      r: 0.,
      g: 0.,
      b: 0.,
      a: 0.,
    },
  }
}
