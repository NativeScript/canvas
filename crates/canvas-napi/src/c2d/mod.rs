//! 2D canvas classes (`canvas2d/` in the V8 bindings).

pub mod context;
pub mod gradient;
pub mod image_data;
pub mod matrix;
pub mod path2d;
pub mod pattern;
pub mod text_metrics;

use napi::sys;

pub unsafe fn init(env: sys::napi_env, exports: sys::napi_value) {
    matrix::init(env, exports);
    path2d::init(env, exports);
    image_data::init(env, exports);
    gradient::init(env, exports);
    pattern::init(env, exports);
    text_metrics::init(env, exports);
    context::init(env, exports);
}
