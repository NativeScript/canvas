//! Node-API build of the canvas native module.
//!
//! Installs the same `CanvasModule` object the V8 bindings (`packages/canvas/platforms/ios/src/cpp`)
//! install, so `packages/canvas` drives it unchanged. The contract lives in `CanvasJSIModule.cpp`
//! (`CanvasJSIModule::install`) and `packages/canvas/WebGPU/NativeImpl.d.ts`.
//!
//! Classes are raw Node-API callbacks (see `util`): they coerce arguments the way the V8
//! bindings and WebIDL do and avoid napi-derive's per-call bookkeeping on hot paths. napi-rs
//! provides module registration, threadsafe functions and async work.

#![allow(clippy::missing_safety_doc)]

mod c2d;
mod image_asset;
mod module;
mod text;
mod util;
mod webgl;
mod webgl2;
mod webgpu;

use std::ptr;

use napi::bindgen_prelude::{Env, Object};
use napi::sys;
use napi::JsValue;
use napi_derive::napi;

#[napi(module_exports)]
pub fn init(exports: Object, env: Env) -> napi::Result<()> {
    unsafe {
        let env = env.raw();
        let exports = exports.raw();
        c2d::init(env, exports);
        image_asset::init(env, exports);
        text::init(env, exports);
        webgl::init(env, exports);
        webgl::objects::init(env, exports);
        webgl2::init(env, exports);
        webgpu::init(env, exports);
        module::init(env, exports);
        install_global(env, exports);
    }
    Ok(())
}

/// `globalThis.CanvasModule = exports` unless something already installed one, matching the V8
/// bindings' `install()`.
unsafe fn install_global(env: sys::napi_env, exports: sys::napi_value) {
    let mut global = ptr::null_mut();
    if sys::napi_get_global(env, &mut global) != sys::Status::napi_ok {
        return;
    }
    let mut has = false;
    sys::napi_has_named_property(env, global, c"CanvasModule".as_ptr(), &mut has);
    if !has {
        sys::napi_set_named_property(env, global, c"CanvasModule".as_ptr(), exports);
    }
}
