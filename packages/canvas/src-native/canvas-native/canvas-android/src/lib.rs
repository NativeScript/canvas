use std::collections::HashMap;
use std::ffi::c_void;
use std::ops::DerefMut;

use android_logger::Config;
use jni::objects::{GlobalRef, JByteBuffer, JClass, JObject, JString, JValue};
use jni::sys::{jboolean, jbyteArray, jfloat, jint, jlong, jobject, jstring, JNI_FALSE, JNI_TRUE};
use jni::{JNIEnv, JavaVM};
use log::Level;
use once_cell::sync::OnceCell;
use rgb::FromSlice;
use skia_safe::gpu::gl::Interface;
use skia_safe::image::CachingHint;
use skia_safe::{
    AlphaType, Color, ColorType, EncodedImageFormat, ISize, ImageInfo, PixelGeometry, Rect, Surface,
};

use canvas_core::context::paths::path::Path;
use canvas_core::context::text_styles::text_direction::TextDirection;
use canvas_core::context::{Context, ContextWrapper, Device, State};
use canvas_core::ffi::u8_array::U8Array;
use canvas_core::to_data_url;
use prelude::*;

pub mod context;
pub mod gl;
pub mod gradient;
pub mod image_asset;
pub mod image_bitmap;
pub mod image_data;
pub mod matrix;
pub mod paint;
pub mod path;
pub mod pattern;
pub mod prelude;
pub mod svg;
pub mod text_decoder;
pub mod text_encoder;
pub mod text_metrics;
pub mod utils;

const GR_GL_RGB565: u32 = 0x8D62;
const GR_GL_RGBA8: u32 = 0x8058;

pub static JVM: OnceCell<JavaVM> = OnceCell::new();

pub(crate) const TNSGCUTILS_CLASS: &str = "org/nativescript/canvas/TNSGcUtils";

pub(crate) const JSON_CLASS: &str = "org/json/JSONObject";

pub static JVM_CLASS_CACHE: OnceCell<parking_lot::RwLock<HashMap<&'static str, GlobalRef>>> =
    OnceCell::new();

#[no_mangle]
pub extern "system" fn JNI_OnLoad(vm: JavaVM, _reserved: *const c_void) -> jint {
    {
        android_logger::init_once(Config::default().with_min_level(Level::Debug));
        log::info!("Canvas Native library loaded");
    }

    if let Ok(env) = vm.get_env() {
        JVM_CLASS_CACHE.get_or_init(|| {
            let mut map = HashMap::new();
            map.insert(
                TNSGCUTILS_CLASS,
                env.new_global_ref(env.find_class(TNSGCUTILS_CLASS).unwrap())
                    .unwrap(),
            );
            map.insert(
                JSON_CLASS,
                env.new_global_ref(env.find_class(JSON_CLASS).unwrap())
                    .unwrap(),
            );
            parking_lot::RwLock::new(map)
        });
    }

    JVM.get_or_init(|| vm);

    jni::sys::JNI_VERSION_1_6
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_canvas_TNSGcUtils_nativeDestroyU8Array(
    _: JNIEnv,
    _: JClass,
    array: jlong,
) {
    if array == 0 {
        return;
    }
    unsafe {
        let array: *mut U8Array = array as _;
        let _ = Box::from_raw(array);
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_canvas_TNSCanvas_nativeInitContext(
    _: JNIEnv,
    _: JClass,
    width: jfloat,
    height: jfloat,
    density: jfloat,
    buffer_id: jint,
    samples: jint,
    alpha: jboolean,
    font_color: jint,
    ppi: jfloat,
    direction: jint,
) -> jlong {
    ContextWrapper::new(Context::new_gl(
        width,
        height,
        density,
        buffer_id,
        samples,
        alpha == JNI_TRUE,
        font_color,
        ppi,
        TextDirection::from(direction as u32),
    ))
    .into_raw() as jlong
}

pub(crate) fn init_with_custom_surface(
    width: jfloat,
    height: jfloat,
    density: jfloat,
    alpha: jboolean,
    font_color: jint,
    ppi: jfloat,
    direction: jint,
) -> jlong {
    ContextWrapper::new(Context::new(
        width,
        height,
        density,
        alpha == JNI_TRUE,
        font_color,
        ppi,
        TextDirection::from(direction as u32),
    ))
    .into_raw() as jlong
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_canvas_TNSCanvas_nativeInitContextWithCustomSurface(
    _: JNIEnv,
    _: JClass,
    width: jfloat,
    height: jfloat,
    density: jfloat,
    alpha: jboolean,
    font_color: jint,
    ppi: jfloat,
    direction: jint,
) -> jlong {
    init_with_custom_surface(width, height, density, alpha, font_color, ppi, direction)
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_canvas_TNSCanvas_nativeDestroyContext(
    _: JNIEnv,
    _: JClass,
    context: jlong,
) {
    if context == 0 {
        return;
    }
    unsafe {
        let context: *mut ContextWrapper = context as _;
        let _ = Box::from_raw(context);
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_canvas_TNSCanvas_nativeResizeSurface(
    _: JNIEnv,
    _: JClass,
    context: jlong,
    width: jfloat,
    height: jfloat,
    density: jfloat,
    buffer_id: jint,
    samples: jint,
    alpha: jboolean,
    ppi: jfloat,
) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut ContextWrapper = context as _;
        let context = &mut *context;
        let mut context = context.get_context();
        Context::resize_gl(
            &mut context,
            width,
            height,
            density,
            buffer_id,
            samples,
            alpha == JNI_TRUE,
            ppi,
        )
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_canvas_TNSCanvas_nativeResizeCustomSurface(
    _: JNIEnv,
    _: JClass,
    context: jlong,
    width: jfloat,
    height: jfloat,
    density: jfloat,
    alpha: jboolean,
    ppi: jfloat,
) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut ContextWrapper = context as _;
        let context = &mut *context;
        let mut context = context.get_context();
        Context::resize(&mut context, width, height, density, alpha == JNI_TRUE, ppi)
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_canvas_TNSCanvas_nativeDataURL(
    env: JNIEnv,
    _: JClass,
    context: jlong,
    format: JString,
    quality: jfloat,
) -> jstring {
    unsafe {
        if context == 0 {
            return env.new_string("").unwrap().into_inner();
        }
        let context: *mut ContextWrapper = context as _;
        let context = &mut *context;
        if let Ok(format) = env.get_string(format) {
            let format = format.to_string_lossy();
            let data = to_data_url(context, format.as_ref(), (quality * 100 as f32) as i32);
            return env.new_string(data).unwrap().into_inner();
        }
        return env.new_string("").unwrap().into_inner();
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_canvas_TNSCanvas_nativeSnapshotCanvas(
    env: JNIEnv,
    _: JClass,
    context: jlong,
) -> jobject {
    let mut buf;
    if context == 0 {
        buf = Vec::new();
    } else {
        unsafe {
            let context: *mut ContextWrapper = context as _;
            let context = &mut *context;
            let mut context = context.get_context();
            buf = context.read_pixels()
        }
    }

    let db = env.new_direct_byte_buffer(buf.as_mut_slice()).unwrap();
    let u8 = U8Array::from(buf);
    let ptr = Box::into_raw(Box::new(u8));
    let clazz = find_class(TNSGCUTILS_CLASS).unwrap();
    let db: JValue = db.into();
    env.call_static_method(
        clazz,
        "watchItem",
        "(JLjava/nio/ByteBuffer;)V",
        &[(ptr as i64).into(), db],
    )
    .unwrap();

    db.l().unwrap().into_inner()
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_canvas_TNSCanvas_nativeFlush(
    _: JNIEnv,
    _: JClass,
    context: jlong,
) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut ContextWrapper = context as _;
        let context = &mut *context;
        let mut context = context.get_context();
        context.flush()
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_canvas_TNSCanvas_nativeCustomWithBitmapFlush(
    env: JNIEnv,
    _: JClass,
    context: jlong,
    bitmap: JObject,
) {
    unsafe {
        if context == 0 {
            return;
        }
        utils::image::bitmap_handler(
            env,
            bitmap,
            Box::new(move |image_data, image_info| {
                let context: *mut ContextWrapper = context as _;
                let context = &mut *context;
                let mut context = context.get_context();
                Context::flush_buffer(
                    &mut context,
                    image_info.width as i32,
                    image_info.height as i32,
                    image_data,
                )
            }),
        )
    }
}
