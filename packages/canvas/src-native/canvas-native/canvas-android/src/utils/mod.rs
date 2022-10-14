use std::ffi::{CStr, CString};

use jni::JNIEnv;
use jni::objects::{JClass, JObject};
use jni::sys::jlong;
use libc::{c_char, size_t};

use canvas_core::context::{Context, ContextWrapper};

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
                if let Some((image_data,image_info)) = image_data{
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