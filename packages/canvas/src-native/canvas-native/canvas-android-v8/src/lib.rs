extern crate core;

pub mod bridges;
pub mod choregrapher;
pub mod gl;
pub mod gl_context;
pub mod jni;
pub mod looper;
pub mod raf;
pub mod utils;

use ::jni::sys::jint;
use ::jni::JavaVM;
use once_cell::sync::OnceCell;
use std::os::raw::c_void;

pub static JVM: OnceCell<JavaVM> = OnceCell::new();

#[no_mangle]
pub extern "system" fn JNI_OnLoad(vm: JavaVM, _reserved: *const c_void) -> jint {
    JVM.get_or_init(|| vm);
    ::jni::sys::JNI_VERSION_1_6
}

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
