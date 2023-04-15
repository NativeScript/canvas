#![allow(non_snake_case)]

extern crate android_logger;
extern crate core;
#[macro_use]
extern crate log;

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_void};
use std::pin::Pin;

use ::jni::signature::JavaType;
use ::jni::sys::jint;
use ::jni::JavaVM;
use android_logger::Config;
use log::LevelFilter;
use once_cell::sync::OnceCell;

use crate::utils::gl::st::{SurfaceTexture, SURFACE_TEXTURE};

mod jni_compat;
pub mod utils;

pub static JVM: OnceCell<JavaVM> = OnceCell::new();

pub static API_LEVEL: OnceCell<i32> = OnceCell::new();

pub(crate) const BUILD_VERSION_CLASS: &str = "android/os/Build$VERSION";

#[no_mangle]
pub extern "system" fn JNI_OnLoad(vm: JavaVM, _reserved: *const c_void) -> jint {
    if let Ok(mut env) = vm.get_env() {
        android_logger::init_once(Config::default());

        API_LEVEL.get_or_init(|| {
            let clazz = env.find_class(BUILD_VERSION_CLASS).unwrap();

            let sdk_int_id = env.get_static_field_id(&clazz, "SDK_INT", "I").unwrap();

            let sdk_int = env.get_static_field_unchecked(
                clazz,
                sdk_int_id,
                JavaType::Primitive(jni::signature::Primitive::Int),
            );

            let ret = sdk_int.unwrap().i().unwrap();

            canvas_cxx::API_LEVEL.get_or_init(|| ret);

            ret
        });

        SURFACE_TEXTURE.get_or_init(|| SurfaceTexture::new());

        log::info!("Canvas library loaded");
    }

    JVM.get_or_init(|| vm);
    jni::sys::JNI_VERSION_1_6
}
