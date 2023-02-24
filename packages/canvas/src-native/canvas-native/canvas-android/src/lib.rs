#![allow(non_snake_case)]

extern crate core;

use std::any::Any;
use std::borrow::Cow;
use std::collections::HashMap;
use std::error::Error;
use std::ffi::{CStr, CString};
use std::io::Read;
use std::os::raw::{c_char, c_int, c_longlong, c_uint, c_ulong, c_void};
use std::os::unix::io::FromRawFd;
use std::os::unix::prelude::IntoRawFd;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, AtomicIsize, Ordering};
use std::sync::Arc;

use ::jni::sys::jint;
use ::jni::JavaVM;
use cxx::{let_cxx_string, type_id, CxxString, CxxVector, ExternType, SharedPtr, UniquePtr};
use once_cell::sync::OnceCell;
use parking_lot::lock_api::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use parking_lot::{Mutex, MutexGuard, RawMutex, RawRwLock};

use canvas_2d::context::compositing::composite_operation_type::CompositeOperationType;
use canvas_2d::context::drawing_paths::fill_rule::FillRule;
use canvas_2d::context::fill_and_stroke_styles::paint::paint_style_set_color_with_string;
use canvas_2d::context::fill_and_stroke_styles::pattern::Repetition;
use canvas_2d::context::image_asset::OutputFormat;
use canvas_2d::context::image_smoothing::ImageSmoothingQuality;
use canvas_2d::context::line_styles::line_cap::LineCap;
use canvas_2d::context::line_styles::line_join::LineJoin;
use canvas_2d::context::text_styles::text_align::TextAlign;
use canvas_2d::context::text_styles::text_direction::TextDirection;
use canvas_2d::context::{Context, ContextWrapper};
use canvas_2d::utils::color::{parse_color, to_parsed_color};
use canvas_2d::utils::image::{
    from_bitmap_slice, from_image_slice, from_image_slice_encoded,
    from_image_slice_encoded_no_copy, from_image_slice_no_copy, to_image, to_image_encoded,
    to_image_encoded_from_data,
};
use gl::prelude::WebGLVersion;
use gl_context::GLContext;

use crate::ffi::{ImageBitmapPremultiplyAlpha, WebGLExtensionType, WebGLResultType};

pub mod choregrapher;
mod jni;
pub mod looper;
pub mod raf;
pub mod utils;

pub static JVM: OnceCell<JavaVM> = OnceCell::new();

#[no_mangle]
pub extern "system" fn JNI_OnLoad(vm: JavaVM, _reserved: *const c_void) -> jint {
    JVM.get_or_init(|| vm);
    ::jni::sys::JNI_VERSION_1_6
}

static THREAD_HANDLE_NEXT_ID: AtomicIsize = AtomicIsize::new(0);

pub static THREAD_HANDLE_MAP: OnceCell<
    parking_lot::RwLock<HashMap<isize, std::thread::JoinHandle<()>>>,
> = OnceCell::new();

/* Utils */

fn get_thread_handle_map<'a>(
) -> &'a parking_lot::RwLock<HashMap<isize, std::thread::JoinHandle<()>>> {
    THREAD_HANDLE_MAP.get_or_init(|| RwLock::new(HashMap::new()))
}

#[derive(Clone, Copy)]
#[repr(isize)]
pub enum LogPriority {
    UNKNOWN = 0,
    DEFAULT = 1,
    VERBOSE = 2,
    DEBUG = 3,
    INFO = 4,
    WARN = 5,
    ERROR = 6,
    FATAL = 7,
    SILENT = 8,
}

impl TryFrom<isize> for LogPriority {
    type Error = &'static str;

    fn try_from(value: isize) -> Result<Self, Self::Error> {
        if value < 0 || value > 8 {
            Err("Invalid LogPriority")
        } else {
            match value {
                0 => Ok(LogPriority::UNKNOWN),
                1 => Ok(LogPriority::DEFAULT),
                2 => Ok(LogPriority::VERBOSE),
                3 => Ok(LogPriority::DEBUG),
                4 => Ok(LogPriority::INFO),
                5 => Ok(LogPriority::WARN),
                6 => Ok(LogPriority::ERROR),
                7 => Ok(LogPriority::FATAL),
                8 => Ok(LogPriority::SILENT),
                _ => Err("Invalid LogPriority"),
            }
        }
    }
}

extern "C" {
    pub fn __android_log_write(prio: c_int, tag: *const c_char, text: *const c_char) -> c_int;
}

pub fn _log(priority: isize, tag: &str, text: &str) {
    __log(priority.try_into().unwrap(), tag, text);
}

pub fn __log(priority: LogPriority, tag: &str, text: &str) {
    let tag = CString::new(tag).unwrap();
    let text = CString::new(text).unwrap();
    unsafe {
        __android_log_write(priority as c_int, tag.as_ptr(), text.as_ptr());
    }
}

pub fn console_log(text: &CxxString) {
    let text = text.to_string_lossy();
    __log(LogPriority::INFO, "JS", text.as_ref());
}

pub fn console_log_rust(text: &str) {
    __log(LogPriority::INFO, "JS", text);
}

pub fn to_rust_string(value: &[c_char]) -> String {
    if value.is_empty() {
        return String::new();
    }
    unsafe { CStr::from_ptr(value.as_ptr()).to_string_lossy().to_string() }
}

pub fn write_to_string(value: &[c_char], mut buf: Pin<&mut CxxString>) {
    if value.is_empty() {
        buf.as_mut().push_str("");
        return;
    }
    let string = unsafe { CStr::from_ptr(value.as_ptr()).to_string_lossy() };
    buf.push_str(string.as_ref());
}

pub fn str_to_buf(value: &str) -> Vec<u8> {
    value.as_bytes().to_vec()
}

pub fn string_to_buf(value: String) -> Vec<u8> {
    value.into_bytes()
}
