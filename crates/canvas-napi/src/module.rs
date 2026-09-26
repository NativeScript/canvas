//! Module-level functions of `CanvasModule` from `CanvasJSIModule.cpp` that belong to no class
//! (fonts, base64, files, `createImageBitmap`), plus the desktop-only `__flushAll`.

use napi::sys;

use crate::util::class::export_function;
use crate::util::ret;

unsafe extern "C" fn flush_all(_env: sys::napi_env, _info: sys::napi_callback_info) -> sys::napi_value {
    crate::util::frame::flush_all();
    ret::undefined()
}

pub unsafe fn init(env: sys::napi_env, exports: sys::napi_value) {
    export_function(env, exports, c"__flushAll", flush_all);
}
