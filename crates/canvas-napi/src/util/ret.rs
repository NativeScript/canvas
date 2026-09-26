//! Creating return values and throwing. A raw callback returning a null `napi_value` returns
//! `undefined`, so void methods just return [`undefined`].

use std::ffi::{c_char, CStr};
use std::ptr;

use napi::sys;

#[inline]
pub fn undefined() -> sys::napi_value {
    ptr::null_mut()
}

/// An actual `undefined` value, for APIs that need one (promise settlement, arguments).
#[inline]
pub fn undefined_value(env: sys::napi_env) -> sys::napi_value {
    let mut out = ptr::null_mut();
    unsafe { sys::napi_get_undefined(env, &mut out) };
    out
}

#[inline]
pub fn null(env: sys::napi_env) -> sys::napi_value {
    let mut out = ptr::null_mut();
    unsafe { sys::napi_get_null(env, &mut out) };
    out
}

#[inline]
pub fn f64(env: sys::napi_env, value: f64) -> sys::napi_value {
    let mut out = ptr::null_mut();
    unsafe { sys::napi_create_double(env, value, &mut out) };
    out
}

#[inline]
pub fn i32(env: sys::napi_env, value: i32) -> sys::napi_value {
    let mut out = ptr::null_mut();
    unsafe { sys::napi_create_int32(env, value, &mut out) };
    out
}

#[inline]
pub fn u32(env: sys::napi_env, value: u32) -> sys::napi_value {
    let mut out = ptr::null_mut();
    unsafe { sys::napi_create_uint32(env, value, &mut out) };
    out
}

#[inline]
pub fn bool(env: sys::napi_env, value: bool) -> sys::napi_value {
    let mut out = ptr::null_mut();
    unsafe { sys::napi_get_boolean(env, value, &mut out) };
    out
}

#[inline]
pub fn string(env: sys::napi_env, value: &str) -> sys::napi_value {
    let mut out = ptr::null_mut();
    unsafe { sys::napi_create_string_utf8(env, value.as_ptr() as *const c_char, value.len() as isize, &mut out) };
    out
}

/// Creates a JS string from a NUL-terminated string owned by canvas-c and frees it.
pub fn c_string(env: sys::napi_env, value: *const c_char) -> sys::napi_value {
    if value.is_null() {
        return string(env, "");
    }
    let bytes = unsafe { CStr::from_ptr(value) }.to_bytes();
    let mut out = ptr::null_mut();
    unsafe {
        sys::napi_create_string_utf8(env, bytes.as_ptr() as *const c_char, bytes.len() as isize, &mut out);
    }
    canvas_c::canvas_native_string_destroy(value as *mut c_char);
    out
}

pub fn throw_type_error(env: sys::napi_env, message: &CStr) -> sys::napi_value {
    unsafe { sys::napi_throw_type_error(env, ptr::null(), message.as_ptr()) };
    ptr::null_mut()
}

pub fn throw_error(env: sys::napi_env, message: &str) -> sys::napi_value {
    let mut msg = ptr::null_mut();
    let mut error = ptr::null_mut();
    unsafe {
        sys::napi_create_string_utf8(env, message.as_ptr() as *const c_char, message.len() as isize, &mut msg);
        sys::napi_create_error(env, ptr::null_mut(), msg, &mut error);
        sys::napi_throw(env, error);
    }
    ptr::null_mut()
}
