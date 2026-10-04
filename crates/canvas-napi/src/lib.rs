#![deny(clippy::all)]
#![allow(clippy::many_single_char_names)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::new_without_default)]

#[macro_use]
extern crate napi_derive;
// #[macro_use]
// extern crate serde_derive;

pub mod gpu;
pub mod c2d;
pub mod image_asset;
pub mod dom_matrix;
mod text_encoder;
mod text_decoder;

pub mod gl;
pub mod gl2;
mod js;
mod host;
mod frame;
mod logger;
mod fast;
mod image_bitmap;
mod offscreen;
mod module;
/// Like the V8 bindings' `install()`: `globalThis.CanvasModule = exports` unless one is already
/// installed.
#[napi_derive::napi(module_exports)]
pub fn install_global(exports: napi::bindgen_prelude::Object, env: napi::Env) -> napi::Result<()> {
  use napi::bindgen_prelude::JsObjectValue;
  logger::install(env.raw())?;
  frame::install_microtask_scheduler(env.raw())?;
  fast::install(env.raw(), napi::JsValue::raw(&exports))?;
  install_canvases_behind(env.raw(), napi::JsValue::raw(&exports));
  let mut global = env.get_global()?;
  if !global.has_named_property("CanvasModule")? {
    global.set_named_property("CanvasModule", exports)?;
  }
  Ok(())
}

/// `CanvasModule.__canvasesBehind`: the count of threaded canvases behind
/// (`canvas_native_canvases_behind`) as memory, which packages/canvas reads every frame. Left out
/// where the host allows no external buffers: requestAnimationFrame is then left alone.
fn install_canvases_behind(env: napi::sys::napi_env, exports: napi::sys::napi_value) {
  use napi::sys;
  let address = canvas_c::canvas_native_canvases_behind_address() as *mut std::ffi::c_void;
  let mut buffer = std::ptr::null_mut();
  unsafe {
    if sys::napi_create_external_arraybuffer(env, address, std::mem::size_of::<u32>(), None, std::ptr::null_mut(), &mut buffer)
      == sys::Status::napi_ok
    {
      sys::napi_set_named_property(env, exports, c"__canvasesBehind".as_ptr(), buffer);
    }
  }
}
