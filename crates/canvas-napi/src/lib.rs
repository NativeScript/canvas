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
mod image_bitmap;
mod module;
/// Like the V8 bindings' `install()`: `globalThis.CanvasModule = exports` unless one is already
/// installed.
#[napi_derive::napi(module_exports)]
pub fn install_global(exports: napi::bindgen_prelude::Object, env: napi::Env) -> napi::Result<()> {
  use napi::bindgen_prelude::JsObjectValue;
  frame::install_microtask_scheduler(env.raw())?;
  let mut global = env.get_global()?;
  if !global.has_named_property("CanvasModule")? {
    global.set_named_property("CanvasModule", exports)?;
  }
  Ok(())
}
