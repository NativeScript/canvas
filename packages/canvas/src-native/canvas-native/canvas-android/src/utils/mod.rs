use std::ffi::{CStr, CString};

use jni::objects::{JClass, JObject, JString};
use jni::sys::{jboolean, jfloat, jint, jlong, jstring};
use jni::JNIEnv;
use libc::{c_char, size_t};

use canvas_core::context::paths::path::Path;
use canvas_core::context::{Context, ContextWrapper, State};

use crate::console_log;

pub mod gl;
pub mod image;

pub fn get_sdk_version() -> i32 {
    let mut ret = -1;
    let cmd = CString::new("getprop ro.build.version.sdk").unwrap();
    let mode = CString::new("r").unwrap();
    let file = unsafe { libc::popen(cmd.as_ptr(), mode.as_ptr()) };
    if !file.is_null() {
        let mut output = [0 as c_char; 100];
        if !unsafe { libc::fgets(output.as_mut_ptr(), 100, file).is_null() } {
            let string = unsafe { CStr::from_ptr(output.as_ptr()) };
            let lossy = string.to_string_lossy();
            if let Ok(value) = lossy.trim().parse::<i32>() {
                ret = value;
            }
        }

        unsafe {
            libc::pclose(file);
        }
    }
    ret
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
    Box::into_raw(Box::new(Context::new(
        width,
        height,
        density,
        alpha == jni::sys::JNI_TRUE,
        font_color,
        ppi,
        canvas_core::context::text_styles::text_direction::TextDirection::from(direction as u32),
    ))) as jlong
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
pub extern "system" fn Java_org_nativescript_canvas_TNSCanvas_nativeResizeCustomSurface(
    _: JNIEnv,
    _: JClass,
    context: jlong,
    width: jfloat,
    height: jfloat,
    _density: jfloat,
    _alpha: jboolean,
    _ppi: jfloat,
) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut ContextWrapper = context as _;
        let context = &mut *context;
        context.resize(width, height);
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
        image::bitmap_handler(
            env,
            bitmap,
            Box::new(move |image_data| {
                if let Some((image_data, image_info)) = image_data {
                    let context: *mut ContextWrapper = context as _;
                    let context = &mut *context;
                    let mut context = context.get_context_mut();
                    Context::flush_buffer(
                        &mut context,
                        image_info.width() as i32,
                        image_info.height() as i32,
                        image_data,
                    )
                }
            }),
        )
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_canvas_TNSCanvas_nativeDestroyContext(
    _: JNIEnv,
    _: JClass,
    context: jlong,
) {
    unsafe {
        if context == 0 {
            return;
        }

        let context: *mut ContextWrapper = context as _;
        let _ = Box::from_raw(context);
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

            return env
                .new_string(canvas_core::to_data_url(
                    context,
                    format.as_ref(),
                    (quality * 100 as f32) as i32,
                ))
                .unwrap()
                .into_inner();
        }
        return env.new_string("").unwrap().into_inner();
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, Default)]
pub struct ByteBufInner {
    needs_to_clean: bool,
}

#[repr(C)]
pub struct ByteBuf {
    pub data: *const u8,
    pub len: size_t,
    inner: ByteBufInner,
}

impl ByteBuf {
    pub fn new(data: *const u8, length: size_t) -> Self {
        Self {
            data,
            len: length,
            inner: Default::default(),
        }
    }

    pub fn as_slice<'a>(&self) -> &'a [u8] {
        unsafe { std::slice::from_raw_parts(self.data, self.len) }
    }
}

impl From<Vec<u8>> for ByteBuf {
    fn from(vec: Vec<u8>) -> Self {
        let len = vec.len();
        let mut slice = vec.into_boxed_slice();
        let data = slice.as_mut_ptr() as *const u8;
        let _ = Box::into_raw(slice);
        Self {
            data,
            len,
            inner: ByteBufInner {
                needs_to_clean: true,
            },
        }
    }
}

impl Drop for ByteBuf {
    fn drop(&mut self) {
        if !self.inner.needs_to_clean {
            return;
        }
        if !self.data.is_null() && self.len != 0 {
            unsafe {
                let _ = Box::from_raw(std::slice::from_raw_parts_mut(
                    self.data as *mut u8,
                    self.len,
                ));
            }
        }
    }
}

unsafe impl Send for ByteBuf {}
