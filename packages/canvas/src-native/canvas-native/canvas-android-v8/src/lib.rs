extern crate core;

pub mod bridges;
pub mod choregrapher;
pub mod gl;
pub mod gl_context;
pub mod looper;
pub mod raf;
pub mod utils;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("canvas-android-v8/src/Bridge.h");
        pub(crate) type V8FunctionCallbackInfo;
        pub(crate) fn Init(args: &V8FunctionCallbackInfo);
    }
}

#[no_mangle]
extern "C" fn NSMain(args: &ffi::V8FunctionCallbackInfo) {
    unsafe {
        ffi::Init(args);
    }
}
