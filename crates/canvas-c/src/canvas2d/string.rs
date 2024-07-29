use std::borrow::Cow;
use std::ffi::CString;
use std::os::raw::c_char;
use std::sync::Arc;

#[no_mangle]
pub extern "C" fn canvas_native_string_destroy(value: *mut c_char) {
    if value.is_null() {
        return;
    }
    let _ = unsafe { CString::from_raw(value) };
}

#[derive(Clone)]
pub struct CCow(pub(crate) Cow<'static, str>);

#[no_mangle]
pub extern "C" fn canvas_native_ccow_reference(cow: *const CCow) {
    if cow.is_null() {
        return;
    }
    let _ = unsafe { Arc::increment_strong_count(cow) };
}

#[no_mangle]
pub extern "C" fn canvas_native_ccow_release(cow: *const CCow) {
    if cow.is_null() {
        return;
    }
    let _ = unsafe { Arc::decrement_strong_count(cow) };
}

#[no_mangle]
pub extern "C" fn canvas_native_ccow_get_bytes(cow: *const CCow) -> *const u8 {
    assert!(!cow.is_null());
    let cow = unsafe { &*cow };
    cow.0.as_ptr()
}

#[no_mangle]
pub extern "C" fn canvas_native_ccow_get_length(cow: *const CCow) -> usize {
    assert!(!cow.is_null());
    let cow = unsafe { &*cow };
    cow.0.len()
}
