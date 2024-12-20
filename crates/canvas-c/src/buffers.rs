use std::ffi::{c_char, CString};
use std::sync::Arc;

use crate::c2d::ImageData;
use bytes::BytesMut;
use parking_lot::Mutex;

#[derive(Clone)]
enum U8BufferInner {
    BytesMut(BytesMut),
    Vec(Arc<Mutex<Vec<u8>>>),
    ImageData(ImageData),
}

#[derive(Clone)]
pub struct U8Buffer(U8BufferInner);

impl U8Buffer {
    pub fn new_with_vec(value: Vec<u8>) -> Self {
        Self(U8BufferInner::Vec(Arc::new(Mutex::new(value))))
    }

    pub fn get_buffer(&self) -> &[u8] {
        match &self.0 {
            U8BufferInner::BytesMut(value) => value,
            U8BufferInner::Vec(value) => {
                let lock = value.lock();
                unsafe { std::slice::from_raw_parts(lock.as_ptr(), lock.len()) }
            }
            U8BufferInner::ImageData(value) => {
                value.0.data()
            }
        }
    }

    pub fn get_buffer_mut(&mut self) -> &mut [u8] {
        match &mut self.0 {
            U8BufferInner::BytesMut(value) => value,
            U8BufferInner::Vec(value) => {
                let mut lock = value.lock();
                unsafe { std::slice::from_raw_parts_mut(lock.as_mut_ptr(), lock.len()) }
            }
            U8BufferInner::ImageData(value) => {
                value.0.data_mut()
            }
        }
    }

    pub fn length(&self) -> usize {
        match &self.0 {
            U8BufferInner::BytesMut(value) => value.len(),
            U8BufferInner::Vec(value) => {
                let lock = value.lock();
                lock.len()
            }
            U8BufferInner::ImageData(value) => value.0.data_len()
        }
    }
}

impl Default for U8Buffer {
    fn default() -> Self {
        Self(U8BufferInner::BytesMut(BytesMut::default()))
    }
}

impl From<Vec<u8>> for U8Buffer {
    fn from(value: Vec<u8>) -> Self {
        U8Buffer(U8BufferInner::Vec(Arc::new(Mutex::new(value))))
    }
}

impl From<BytesMut> for U8Buffer {
    fn from(value: BytesMut) -> Self {
        U8Buffer(U8BufferInner::BytesMut(value))
    }
}


impl From<ImageData> for U8Buffer {
    fn from(value: ImageData) -> Self {
        U8Buffer(U8BufferInner::ImageData(value.clone()))
    }
}

pub struct U16Buffer(Vec<u16>);

impl U16Buffer {
    pub fn new_with_vec(value: Vec<u16>) -> Self {
        Self(value)
    }

    pub fn get_buffer(&self) -> &[u16] {
        self.0.as_slice()
    }

    pub fn get_buffer_mut(&mut self) -> &mut [u16] {
        self.0.as_mut_slice()
    }

    pub fn length(&self) -> usize {
        self.0.len()
    }
}

impl Default for U16Buffer {
    fn default() -> Self {
        Self(Vec::new())
    }
}

impl From<Vec<u16>> for U16Buffer {
    fn from(value: Vec<u16>) -> Self {
        Self::new_with_vec(value)
    }
}

pub struct F32Buffer(Vec<f32>);

impl F32Buffer {
    pub fn new_with_vec(value: Vec<f32>) -> Self {
        Self(value)
    }

    pub fn get_buffer(&self) -> &[f32] {
        self.0.as_slice()
    }

    pub fn get_buffer_mut(&mut self) -> &mut [f32] {
        self.0.as_mut_slice()
    }

    pub fn length(&self) -> usize {
        self.0.len()
    }
}

impl Default for F32Buffer {
    fn default() -> Self {
        Self(Vec::new())
    }
}

impl From<Vec<f32>> for F32Buffer {
    fn from(value: Vec<f32>) -> Self {
        Self(value)
    }
}

pub struct I32Buffer(Vec<i32>);

impl I32Buffer {
    pub fn new_with_vec(value: Vec<i32>) -> Self {
        Self(value)
    }
    pub fn get_buffer(&self) -> &[i32] {
        self.0.as_slice()
    }

    pub fn get_buffer_mut(&mut self) -> &mut [i32] {
        self.0.as_mut_slice()
    }

    pub fn length(&self) -> usize {
        self.0.len()
    }
}

impl Default for I32Buffer {
    fn default() -> Self {
        Self(Vec::new())
    }
}

impl From<Vec<i32>> for I32Buffer {
    fn from(value: Vec<i32>) -> Self {
        Self(value)
    }
}

pub struct U32Buffer(Vec<u32>);

impl U32Buffer {
    pub fn new_with_vec(value: Vec<u32>) -> Self {
        Self(value)
    }

    pub fn get_buffer(&self) -> &[u32] {
        self.0.as_slice()
    }

    pub fn get_buffer_mut(&mut self) -> &mut [u32] {
        self.0.as_mut_slice()
    }

    pub fn length(&self) -> usize {
        self.0.len()
    }
}

impl Default for U32Buffer {
    fn default() -> Self {
        Self::new_with_vec(Vec::new())
    }
}

impl From<Vec<u32>> for U32Buffer {
    fn from(value: Vec<u32>) -> Self {
        Self::new_with_vec(value)
    }
}

pub struct StringBuffer(Vec<String>);
impl From<Vec<String>> for StringBuffer {
    fn from(value: Vec<String>) -> Self {
        Self(value)
    }
}

// todo use convert StringBuffer to enum
impl From<Vec<&str>> for StringBuffer {
    fn from(value: Vec<&str>) -> Self {
        Self(value.into_iter().map(|v| v.to_string()).collect())
    }
}

impl StringBuffer {
    pub fn get_buffer(&self) -> &[String] {
        self.0.as_slice()
    }

    pub fn length(&self) -> usize {
        self.0.len()
    }
}

pub struct StringRefBuffer<'a>(&'a [&'a str]);

impl<'a> StringRefBuffer<'a> {
    pub fn get_buffer(&self) -> &[&str] {
        self.0
    }

    pub fn length(&self) -> usize {
        self.0.len()
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_u8_buffer_clone(buffer: *const U8Buffer) -> *const U8Buffer {
    assert!(!buffer.is_null());
    let buffer = unsafe { &*buffer };
    Box::into_raw(Box::new(U8Buffer::from(buffer.clone())))
}

#[no_mangle]
pub extern "C" fn canvas_native_u8_buffer_get_bytes(buffer: *const U8Buffer) -> *const u8 {
    assert!(!buffer.is_null());
    let buffer = unsafe { &*buffer };
    buffer.get_buffer().as_ptr()
}
#[no_mangle]
pub extern "C" fn canvas_native_u8_buffer_get_bytes_mut(buffer: *mut U8Buffer) -> *mut u8 {
    assert!(!buffer.is_null());
    let buffer = unsafe { &mut *buffer };
    buffer.get_buffer_mut().as_mut_ptr()
}

#[no_mangle]
pub extern "C" fn canvas_native_u8_buffer_release(buffer: *mut U8Buffer) {
    if buffer.is_null() {
        return;
    }
    unsafe {
        let _ = Box::from_raw(buffer);
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_u8_buffer_get_length(buffer: *const U8Buffer) -> usize {
    assert!(!buffer.is_null());
    let buffer = unsafe { &*buffer };
    buffer.length()
}

#[no_mangle]
pub extern "C" fn canvas_native_u16_buffer_get_bytes(buffer: *const U16Buffer) -> *const u16 {
    assert!(!buffer.is_null());
    let buffer = unsafe { &*buffer };
    buffer.get_buffer().as_ptr()
}
#[no_mangle]
pub extern "C" fn canvas_native_u16_buffer_get_bytes_mut(buffer: *mut U16Buffer) -> *mut u16 {
    assert!(!buffer.is_null());
    let buffer = unsafe { &mut *buffer };
    buffer.get_buffer_mut().as_mut_ptr()
}

#[no_mangle]
pub extern "C" fn canvas_native_u16_buffer_release(buffer: *mut U16Buffer) {
    if buffer.is_null() {
        return;
    }
    unsafe {
        let _ = Box::from_raw(buffer);
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_u16_buffer_get_length(buffer: *const U16Buffer) -> usize {
    assert!(!buffer.is_null());
    let buffer = unsafe { &*buffer };
    buffer.length()
}

#[no_mangle]
pub extern "C" fn canvas_native_u32_buffer_get_bytes(buffer: *const U32Buffer) -> *const u32 {
    assert!(!buffer.is_null());
    let buffer = unsafe { &*buffer };
    buffer.get_buffer().as_ptr()
}
#[no_mangle]
pub extern "C" fn canvas_native_u32_buffer_get_bytes_mut(buffer: *mut U32Buffer) -> *mut u32 {
    assert!(!buffer.is_null());
    let buffer = unsafe { &mut *buffer };
    buffer.get_buffer_mut().as_mut_ptr()
}

#[no_mangle]
pub extern "C" fn canvas_native_u32_buffer_release(buffer: *mut U32Buffer) {
    if buffer.is_null() {
        return;
    }
    unsafe {
        let _ = Box::from_raw(buffer);
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_u32_buffer_get_length(buffer: *const U32Buffer) -> usize {
    assert!(!buffer.is_null());
    let buffer = unsafe { &*buffer };
    buffer.length()
}

#[no_mangle]
pub extern "C" fn canvas_native_i32_buffer_get_bytes(buffer: *const I32Buffer) -> *const i32 {
    assert!(!buffer.is_null());
    let buffer = unsafe { &*buffer };
    buffer.get_buffer().as_ptr()
}
#[no_mangle]
pub extern "C" fn canvas_native_i32_buffer_get_bytes_mut(buffer: *mut I32Buffer) -> *mut i32 {
    assert!(!buffer.is_null());
    let buffer = unsafe { &mut *buffer };
    buffer.get_buffer_mut().as_mut_ptr()
}

#[no_mangle]
pub extern "C" fn canvas_native_i32_buffer_release(buffer: *mut I32Buffer) {
    if buffer.is_null() {
        return;
    }
    unsafe {
        let _ = Box::from_raw(buffer);
    }
}


#[no_mangle]
pub extern "C" fn canvas_native_i32_buffer_get_length(buffer: *const I32Buffer) -> usize {
    assert!(!buffer.is_null());
    let buffer = unsafe { &*buffer };
    buffer.length()
}

#[no_mangle]
pub extern "C" fn canvas_native_f32_buffer_get_bytes(buffer: *const F32Buffer) -> *const f32 {
    assert!(!buffer.is_null());
    let buffer = unsafe { &*buffer };
    buffer.get_buffer().as_ptr()
}
#[no_mangle]
pub extern "C" fn canvas_native_f32_buffer_get_bytes_mut(buffer: *mut F32Buffer) -> *mut f32 {
    assert!(!buffer.is_null());
    let buffer = unsafe { &mut *buffer };
    buffer.get_buffer_mut().as_mut_ptr()
}

#[no_mangle]
pub extern "C" fn canvas_native_f32_buffer_release(buffer: *mut F32Buffer) {
    if buffer.is_null() {
        return;
    }
    unsafe {
        let _ = Box::from_raw(buffer);
    }
}


#[no_mangle]
pub extern "C" fn canvas_native_f32_buffer_get_length(buffer: *const F32Buffer) -> usize {
    assert!(!buffer.is_null());
    let buffer = unsafe { &*buffer };
    buffer.length()
}

#[no_mangle]
pub extern "C" fn canvas_native_string_buffer_get_length(buffer: *const StringBuffer) -> usize {
    assert!(!buffer.is_null());
    let buffer = unsafe { &*buffer };
    buffer.length()
}

#[no_mangle]
pub extern "C" fn canvas_native_string_buffer_get_value_at(
    buffer: *const StringBuffer,
    index: usize,
) -> *mut c_char {
    assert!(!buffer.is_null());
    let buffer = unsafe { &*buffer };
    match buffer.0.get(index) {
        None => std::ptr::null_mut(),
        Some(value) => CString::new(value.to_string()).unwrap().into_raw(),
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_string_buffer_release(buffer: *mut StringBuffer) {
    if buffer.is_null() {
        return;
    }
    unsafe {
        let _ = Box::from_raw(buffer);
    }
}
