//! WebGL 2 (`webgl2/` in the V8 bindings): `WebGL2RenderingContext` = the WebGL 1 surface
//! (`crate::webgl::base_methods`) plus the WebGL 2 additions, and the WebGL 2-only objects.

use napi::sys;

pub unsafe fn init(_env: sys::napi_env, _exports: sys::napi_value) {}
