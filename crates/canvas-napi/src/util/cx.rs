//! Argument access for raw Node-API callbacks.
//!
//! Conversions follow the V8 bindings (and WebIDL): numbers go through `ToNumber`, booleans
//! through `ToBoolean`, so `ctx.fillRect("10", …)` behaves as it does in a browser. The fast path
//! is one Node-API call per argument; coercion only happens when the value is not already the
//! expected primitive.

#![allow(non_upper_case_globals)]

use std::ffi::{c_char, c_void};
use std::ptr;

use napi::sys;

use super::native::{self, Native, NativeType};

/// The receiver and the first `N` arguments of a call. Missing arguments read as `undefined`.
pub struct Cx<const N: usize> {
    pub env: sys::napi_env,
    pub this: sys::napi_value,
    /// The number of arguments actually passed (may exceed `N`).
    pub argc: usize,
    pub argv: [sys::napi_value; N],
}

impl<const N: usize> Cx<N> {
    #[inline]
    pub unsafe fn new(env: sys::napi_env, info: sys::napi_callback_info) -> Self {
        let mut argc = N;
        let mut argv = [ptr::null_mut(); N];
        let mut this = ptr::null_mut();
        sys::napi_get_cb_info(env, info, &mut argc, argv.as_mut_ptr(), &mut this, ptr::null_mut());
        Cx { env, this, argc, argv }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.argc
    }

    #[inline]
    pub fn arg(&self, index: usize) -> sys::napi_value {
        self.argv[index]
    }

    /// The wrapped receiver, if `this` is a `T`.
    #[inline]
    pub fn this<'a, T: Native>(&self) -> Option<&'a mut T> {
        unsafe { native::unwrap::<T>(self.env, self.this) }
    }

    #[inline]
    pub fn native<'a, T: Native>(&self, index: usize) -> Option<&'a mut T> {
        if index >= self.argc.min(N) {
            return None;
        }
        unsafe { native::unwrap::<T>(self.env, self.argv[index]) }
    }

    #[inline]
    pub fn kind(&self, index: usize) -> Option<(NativeType, *mut c_void)> {
        if index >= self.argc.min(N) {
            return None;
        }
        unsafe { native::kind_of(self.env, self.argv[index]) }
    }

    #[inline]
    pub fn value_type(&self, index: usize) -> sys::napi_valuetype {
        value_type(self.env, self.argv[index])
    }

    #[inline]
    pub fn is_nullish(&self, index: usize) -> bool {
        index >= self.argc || matches!(self.value_type(index), sys::ValueType::napi_undefined | sys::ValueType::napi_null)
    }

    #[inline]
    pub fn f64(&self, index: usize) -> f64 {
        to_f64(self.env, self.argv[index])
    }

    #[inline]
    pub fn f32(&self, index: usize) -> f32 {
        self.f64(index) as f32
    }

    /// `ToInt32`.
    #[inline]
    pub fn i32(&self, index: usize) -> i32 {
        to_i32(self.env, self.argv[index])
    }

    /// `ToUint32`.
    #[inline]
    pub fn u32(&self, index: usize) -> u32 {
        to_i32(self.env, self.argv[index]) as u32
    }

    /// `ToBoolean`.
    #[inline]
    pub fn bool(&self, index: usize) -> bool {
        to_bool(self.env, self.argv[index])
    }

    /// A JS array or typed array of numbers as `f32`s.
    pub fn f32_list(&self, index: usize) -> Option<Vec<f32>> {
        list_f32(self.env, self.argv[index])
    }

    /// The argument as a string, if it is one (no `ToString` coercion).
    #[inline]
    pub fn str(&self, index: usize) -> Option<StrBuf> {
        StrBuf::read(self.env, self.argv[index])
    }
}

#[inline]
pub fn value_type(env: sys::napi_env, value: sys::napi_value) -> sys::napi_valuetype {
    let mut kind = sys::ValueType::napi_undefined;
    if value.is_null() {
        return kind;
    }
    unsafe { sys::napi_typeof(env, value, &mut kind) };
    kind
}

#[inline]
pub fn to_f64(env: sys::napi_env, value: sys::napi_value) -> f64 {
    let mut out = f64::NAN;
    if value.is_null() {
        return out;
    }
    unsafe {
        if sys::napi_get_value_double(env, value, &mut out) == sys::Status::napi_ok {
            return out;
        }
        let mut number = ptr::null_mut();
        if sys::napi_coerce_to_number(env, value, &mut number) == sys::Status::napi_ok {
            sys::napi_get_value_double(env, number, &mut out);
        } else {
            // A throwing valueOf leaves an exception pending; the call reports it on return.
            out = f64::NAN;
        }
    }
    out
}

#[inline]
pub fn to_i32(env: sys::napi_env, value: sys::napi_value) -> i32 {
    let mut out = 0i32;
    if value.is_null() {
        return 0;
    }
    unsafe {
        if sys::napi_get_value_int32(env, value, &mut out) == sys::Status::napi_ok {
            return out;
        }
        let mut number = ptr::null_mut();
        if sys::napi_coerce_to_number(env, value, &mut number) == sys::Status::napi_ok {
            sys::napi_get_value_int32(env, number, &mut out);
        }
    }
    out
}

#[inline]
pub fn to_bool(env: sys::napi_env, value: sys::napi_value) -> bool {
    let mut out = false;
    if value.is_null() {
        return false;
    }
    unsafe {
        if sys::napi_get_value_bool(env, value, &mut out) == sys::Status::napi_ok {
            return out;
        }
        let mut boolean = ptr::null_mut();
        if sys::napi_coerce_to_bool(env, value, &mut boolean) == sys::Status::napi_ok {
            sys::napi_get_value_bool(env, boolean, &mut out);
        }
    }
    out
}

/// A JS array (elements through `ToNumber`) or any numeric typed array, as `f32`s.
pub fn list_f32(env: sys::napi_env, value: sys::napi_value) -> Option<Vec<f32>> {
    if value.is_null() {
        return None;
    }
    unsafe {
        let mut is_array = false;
        sys::napi_is_array(env, value, &mut is_array);
        if is_array {
            let mut len = 0u32;
            sys::napi_get_array_length(env, value, &mut len);
            let mut out = Vec::with_capacity(len as usize);
            for i in 0..len {
                let mut element = ptr::null_mut();
                sys::napi_get_element(env, value, i, &mut element);
                out.push(to_f64(env, element) as f32);
            }
            return Some(out);
        }
        let view = TypedArray::read(env, value)?;
        Some(view.to_f32())
    }
}

/// A borrowed view of a typed array's memory. Valid for the duration of the call.
pub struct TypedArray {
    pub kind: sys::napi_typedarray_type,
    pub len: usize,
    pub data: *mut c_void,
}

impl TypedArray {
    pub unsafe fn read(env: sys::napi_env, value: sys::napi_value) -> Option<TypedArray> {
        let mut is_typed = false;
        if value.is_null() || sys::napi_is_typedarray(env, value, &mut is_typed) != sys::Status::napi_ok || !is_typed {
            return None;
        }
        let mut kind = 0;
        let mut len = 0usize;
        let mut data = ptr::null_mut();
        let mut buffer = ptr::null_mut();
        let mut offset = 0usize;
        if sys::napi_get_typedarray_info(env, value, &mut kind, &mut len, &mut data, &mut buffer, &mut offset)
            != sys::Status::napi_ok
        {
            return None;
        }
        Some(TypedArray { kind, len, data })
    }

    pub fn byte_len(&self) -> usize {
        use sys::TypedarrayType::*;
        let size = match self.kind {
            int8_array | uint8_array | uint8_clamped_array => 1,
            int16_array | uint16_array => 2,
            int32_array | uint32_array | float32_array => 4,
            _ => 8,
        };
        self.len * size
    }

    pub fn bytes(&self) -> &[u8] {
        if self.data.is_null() {
            return &[];
        }
        unsafe { std::slice::from_raw_parts(self.data as *const u8, self.byte_len()) }
    }

    pub fn to_f32(&self) -> Vec<f32> {
        use sys::TypedarrayType::*;
        if self.data.is_null() {
            return Vec::new();
        }
        unsafe {
            macro_rules! convert {
                ($t:ty) => {
                    std::slice::from_raw_parts(self.data as *const $t, self.len).iter().map(|v| *v as f32).collect()
                };
            }
            match self.kind {
                int8_array => convert!(i8),
                uint8_array | uint8_clamped_array => convert!(u8),
                int16_array => convert!(i16),
                uint16_array => convert!(u16),
                int32_array => convert!(i32),
                uint32_array => convert!(u32),
                float32_array => convert!(f32),
                float64_array => convert!(f64),
                bigint64_array => convert!(i64),
                _ => convert!(u64),
            }
        }
    }
}

const INLINE: usize = 64;

/// A JS string as NUL-terminated UTF-8. Short strings (the common case for colours, fonts,
/// enum values) stay on the stack.
pub enum StrBuf {
    Inline { bytes: [u8; INLINE], len: usize },
    Heap(Vec<u8>),
}

impl StrBuf {
    pub fn read(env: sys::napi_env, value: sys::napi_value) -> Option<StrBuf> {
        if value.is_null() {
            return None;
        }
        let mut bytes = [0u8; INLINE];
        let mut len = 0usize;
        let status = unsafe {
            sys::napi_get_value_string_utf8(env, value, bytes.as_mut_ptr() as *mut c_char, INLINE, &mut len)
        };
        if status != sys::Status::napi_ok {
            return None;
        }
        if len + 1 < INLINE {
            return Some(StrBuf::Inline { bytes, len });
        }
        // Possibly truncated: ask for the real length and read again.
        let mut full = 0usize;
        unsafe { sys::napi_get_value_string_utf8(env, value, ptr::null_mut(), 0, &mut full) };
        let mut heap = vec![0u8; full + 1];
        unsafe {
            sys::napi_get_value_string_utf8(env, value, heap.as_mut_ptr() as *mut c_char, full + 1, &mut len)
        };
        heap.truncate(len + 1);
        Some(StrBuf::Heap(heap))
    }

    /// The bytes without the terminating NUL.
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        match self {
            StrBuf::Inline { bytes, len } => &bytes[..*len],
            StrBuf::Heap(heap) => &heap[..heap.len() - 1],
        }
    }

    #[inline]
    pub fn as_str(&self) -> &str {
        // Node-API hands out valid UTF-8.
        unsafe { std::str::from_utf8_unchecked(self.as_bytes()) }
    }

    /// NUL-terminated, for canvas-c's `*const c_char` parameters.
    #[inline]
    pub fn as_ptr(&self) -> *const c_char {
        match self {
            StrBuf::Inline { bytes, .. } => bytes.as_ptr() as *const c_char,
            StrBuf::Heap(heap) => heap.as_ptr() as *const c_char,
        }
    }
}
