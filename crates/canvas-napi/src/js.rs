use napi::bindgen_prelude::{ToNapiValue, Unknown};
use napi::{Env, Result};

/// Converts any value napi-rs knows how to create (numbers, strings, `Null`, `#[napi]` class
/// structs, typed arrays, `Vec`s …) into an `Unknown`, for methods whose return type depends on
/// their arguments (`getParameter`, `getExtension`, …).
pub(crate) trait ToJs: ToNapiValue + Sized {
  fn to_js<'env>(self, env: &'env Env) -> Result<Unknown<'env>> {
    unsafe {
      let raw = Self::to_napi_value(env.raw(), self)?;
      Ok(Unknown::from_raw_unchecked(env.raw(), raw))
    }
  }
}

impl<T: ToNapiValue> ToJs for T {}

/// An `ArrayBuffer` argument that can take part in `Either` (napi-rs 3's `ArrayBuffer` has no
/// `ValidateNapiValue`). Derefs to the buffer's bytes; no copy.
pub struct AnyArrayBuffer<'env>(pub napi::bindgen_prelude::ArrayBuffer<'env>);

impl napi::bindgen_prelude::TypeName for AnyArrayBuffer<'_> {
  fn type_name() -> &'static str {
    "ArrayBuffer"
  }

  fn value_type() -> napi::ValueType {
    napi::ValueType::Object
  }
}

impl napi::bindgen_prelude::FromNapiValue for AnyArrayBuffer<'_> {
  unsafe fn from_napi_value(
    env: napi::sys::napi_env,
    value: napi::sys::napi_value,
  ) -> Result<Self> {
    Ok(AnyArrayBuffer(
      napi::bindgen_prelude::ArrayBuffer::from_napi_value(env, value)?,
    ))
  }
}

impl napi::bindgen_prelude::ValidateNapiValue for AnyArrayBuffer<'_> {
  unsafe fn validate(
    env: napi::sys::napi_env,
    value: napi::sys::napi_value,
  ) -> Result<napi::sys::napi_value> {
    let mut is_arraybuffer = false;
    napi::check_status!(napi::sys::napi_is_arraybuffer(
      env,
      value,
      &mut is_arraybuffer
    ))?;
    if !is_arraybuffer {
      return Err(napi::Error::new(
        napi::Status::InvalidArg,
        "Expected an ArrayBuffer",
      ));
    }
    Ok(std::ptr::null_mut())
  }
}

impl std::ops::Deref for AnyArrayBuffer<'_> {
  type Target = [u8];

  fn deref(&self) -> &[u8] {
    &self.0
  }
}
