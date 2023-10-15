use bytes::{Bytes, BytesMut};

#[derive(Clone)]
enum U8BufferMutInner {
    BytesMut(BytesMut),
    Vec(Vec<u8>),
    Reference(*mut u8, usize),
}
pub struct U8BufferMut(U8BufferMutInner);
impl Default for U8BufferMut {
    fn default() -> Self {
        Self(U8BufferMutInner::BytesMut(BytesMut::default()))
    }
}
impl<'a> From<&'a mut [u8]> for U8BufferMut {
    fn from(value: &mut [u8]) -> Self {
        let mut bytes = BytesMut::with_capacity(value.len());
        bytes.extend_from_slice(value);
        U8BufferMut(U8BufferMutInner::BytesMut(bytes))
    }
}

impl From<Vec<u8>> for U8BufferMut {
    fn from(value: Vec<u8>) -> Self {
        U8BufferMut(U8BufferMutInner::Vec(value))
    }
}

impl U8BufferMut {
    pub fn new_with_vec(value: Vec<u8>) -> Self {
        Self(U8BufferMutInner::Vec(value))
    }

    pub fn new_with_reference(value: *mut u8, size: usize) -> Self {
        Self(U8BufferMutInner::Reference(value, size))
    }
    pub fn get_buffer(&mut self) -> &mut [u8] {
        match &mut self.0 {
            U8BufferMutInner::BytesMut(value) => value.as_mut(),
            U8BufferMutInner::Vec(value) => value.as_mut_slice(),
            U8BufferMutInner::Reference(value, size) => unsafe {
                std::slice::from_raw_parts_mut(*value, *size)
            },
        }
    }

    pub fn length(&self) -> usize {
        match &self.0 {
            U8BufferMutInner::BytesMut(value) => value.len(),
            U8BufferMutInner::Vec(value) => value.len(),
            U8BufferMutInner::Reference(_, size) => *size,
        }
    }
}

#[derive(Clone)]
enum U8BufferInner {
    Bytes(Bytes),
    Vec(Vec<u8>),
    Reference(*const u8, usize),
}

#[derive(Clone)]
pub struct U8Buffer(U8BufferInner);

impl U8Buffer {
    pub fn new_with_vec(value: Vec<u8>) -> Self {
        Self(U8BufferInner::Vec(value))
    }

    pub fn new_with_reference(value: *const u8, size: usize) -> Self {
        Self(U8BufferInner::Reference(value, size))
    }

    pub fn get_buffer(&self) -> &[u8] {
        match &self.0 {
            U8BufferInner::Bytes(value) => value,
            U8BufferInner::Vec(value) => value.as_slice(),
            U8BufferInner::Reference(value, size) => unsafe {
                std::slice::from_raw_parts(*value, *size)
            },
        }
    }

    pub fn length(&self) -> usize {
        match &self.0 {
            U8BufferInner::Bytes(value) => value.len(),
            U8BufferInner::Vec(value) => value.len(),
            U8BufferInner::Reference(_, size) => *size,
        }
    }
}

impl Default for U8Buffer {
    fn default() -> Self {
        Self(U8BufferInner::Bytes(Bytes::default()))
    }
}

impl From<Vec<u8>> for U8Buffer {
    fn from(value: Vec<u8>) -> Self {
        U8Buffer(U8BufferInner::Bytes(bytes::Bytes::from(value)))
    }
}

#[derive(Clone)]
enum U16BufferInner {
    Vec(Vec<u16>),
    Reference(*const u16, usize),
}

pub struct U16Buffer(U16BufferInner);

impl U16Buffer {
    pub fn new_with_vec(value: Vec<u16>) -> Self {
        Self(U16BufferInner::Vec(value))
    }

    pub fn new_with_reference(value: *const u16, size: usize) -> Self {
        Self(U16BufferInner::Reference(value, size))
    }

    pub fn get_buffer(&self) -> &[u16] {
        match &self.0 {
            U16BufferInner::Vec(value) => value.as_slice(),
            U16BufferInner::Reference(value, size) => unsafe {
                std::slice::from_raw_parts(*value, *size)
            },
        }
    }

    pub fn length(&self) -> usize {
        match &self.0 {
            U16BufferInner::Vec(value) => value.len(),
            U16BufferInner::Reference(_, size) => *size,
        }
    }
}

impl Default for U16Buffer {
    fn default() -> Self {
        Self::new_with_vec(Vec::new())
    }
}

impl From<Vec<u16>> for U16Buffer {
    fn from(value: Vec<u16>) -> Self {
        Self::new_with_vec(value)
    }
}

#[derive(Clone)]
enum U16BufferMutInner {
    Vec(Vec<u16>),
    Reference(*mut u16, usize),
}
pub struct U16BufferMut(U16BufferMutInner);

impl U16BufferMut {
    pub fn new_with_vec(value: Vec<u16>) -> Self {
        Self(U16BufferMutInner::Vec(value))
    }

    pub fn new_with_reference(value: *mut u16, size: usize) -> Self {
        Self(U16BufferMutInner::Reference(value, size))
    }
    pub fn get_buffer(&mut self) -> &mut [u16] {
        match &mut self.0 {
            U16BufferMutInner::Vec(value) => value.as_mut_slice(),
            U16BufferMutInner::Reference(value, size) => unsafe {
                std::slice::from_raw_parts_mut(*value, *size)
            },
        }
    }

    pub fn length(&self) -> usize {
        match &self.0 {
            U16BufferMutInner::Vec(value) => value.len(),
            U16BufferMutInner::Reference(_, size) => *size,
        }
    }
}

impl Default for U16BufferMut {
    fn default() -> Self {
        Self(U16BufferMutInner::Vec(Vec::new()))
    }
}

impl From<Vec<u16>> for U16BufferMut {
    fn from(value: Vec<u16>) -> Self {
        Self(U16BufferMutInner::Vec(value))
    }
}

#[derive(Clone)]
enum F32BufferInner {
    Vec(Vec<f32>),
    Reference(*const f32, usize),
}

pub struct F32Buffer(F32BufferInner);

impl F32Buffer {
    pub fn new_with_vec(value: Vec<f32>) -> Self {
        Self(F32BufferInner::Vec(value))
    }

    pub fn new_with_reference(value: *const f32, size: usize) -> Self {
        Self(F32BufferInner::Reference(value, size))
    }

    pub fn get_buffer(&self) -> &[f32] {
        match &self.0 {
            F32BufferInner::Vec(value) => value.as_slice(),
            F32BufferInner::Reference(value, size) => unsafe {
                std::slice::from_raw_parts(*value, *size)
            },
        }
    }

    pub fn length(&self) -> usize {
        match &self.0 {
            F32BufferInner::Vec(value) => value.len(),
            F32BufferInner::Reference(_, size) => *size,
        }
    }
}

impl Default for F32Buffer {
    fn default() -> Self {
        Self(F32BufferInner::Vec(Vec::new()))
    }
}

impl From<Vec<f32>> for F32Buffer {
    fn from(value: Vec<f32>) -> Self {
        Self(F32BufferInner::Vec(value))
    }
}






#[derive(Clone)]
enum F32BufferMutInner {
    Vec(Vec<f32>),
    Reference(*mut f32, usize),
}
pub struct F32BufferMut(F32BufferMutInner);

impl F32BufferMut {
    pub fn new_with_vec(value: Vec<f32>) -> Self {
        Self(F32BufferMutInner::Vec(value))
    }

    pub fn new_with_reference(value: *mut f32, size: usize) -> Self {
        Self(F32BufferMutInner::Reference(value, size))
    }
    pub fn get_buffer(&mut self) -> &mut [f32] {
        match &mut self.0 {
            F32BufferMutInner::Vec(value) => value.as_mut_slice(),
            F32BufferMutInner::Reference(value, size) => unsafe {
                std::slice::from_raw_parts_mut(*value, *size)
            },
        }
    }

    pub fn length(&self) -> usize {
        match &self.0 {
            F32BufferMutInner::Vec(value) => value.len(),
            F32BufferMutInner::Reference(_, size) => *size,
        }
    }
}

impl Default for F32BufferMut {
    fn default() -> Self {
        Self(F32BufferMutInner::Vec(Vec::new()))
    }
}

impl From<Vec<f32>> for F32BufferMut {
    fn from(value: Vec<f32>) -> Self {
        Self(F32BufferMutInner::Vec(value))
    }
}


#[derive(Clone)]
enum I32BufferInner {
    Vec(Vec<i32>),
    Reference(*const i32, usize),
}
pub struct I32Buffer(I32BufferInner);

impl I32Buffer {
    pub fn new_with_vec(value: Vec<i32>) -> Self {
        Self(I32BufferInner::Vec(value))
    }

    pub fn new_with_reference(value: *const i32, size: usize) -> Self {
        Self(I32BufferInner::Reference(value, size))
    }
    pub fn get_buffer(&self) -> &[i32] {
        match &self.0 {
            I32BufferInner::Vec(value) => value.as_slice(),
            I32BufferInner::Reference(value, size) => unsafe {
                std::slice::from_raw_parts(*value, *size)
            },
        }
    }

    pub fn length(&self) -> usize {
        match &self.0 {
            I32BufferInner::Vec(value) => value.len(),
            I32BufferInner::Reference(_, size) => *size,
        }
    }
}

impl Default for I32Buffer {
    fn default() -> Self {
        Self(I32BufferInner::Vec(Vec::new()))
    }
}

impl From<Vec<i32>> for I32Buffer {
    fn from(value: Vec<i32>) -> Self {
        Self(I32BufferInner::Vec(value))
    }
}

#[derive(Clone)]
enum U32BufferInner {
    Vec(Vec<u32>),
    Reference(*const u32, usize),
}
pub struct U32Buffer(U32BufferInner);

impl U32Buffer {
    pub fn new_with_vec(value: Vec<u32>) -> Self {
        Self(U32BufferInner::Vec(value))
    }

    pub fn new_with_reference(value: *const u32, size: usize) -> Self {
        Self(U32BufferInner::Reference(value, size))
    }
    pub fn get_buffer(&self) -> &[u32] {
        match &self.0 {
            U32BufferInner::Vec(value) => value.as_slice(),
            U32BufferInner::Reference(value, size) => unsafe {
                std::slice::from_raw_parts(*value, *size)
            },
        }
    }

    pub fn length(&self) -> usize {
        match &self.0 {
            U32BufferInner::Vec(value) => value.len(),
            U32BufferInner::Reference(_, size) => *size,
        }
    }
}

impl Default for U32Buffer {
    fn default() -> Self {
        Self::new_with_vec(Vec::new())
    }
}

impl From<Vec<u32>> for U32Buffer {
    fn from(value: Vec<u32>) -> Self {
        Self::new_with_vec(Vec::new())
    }
}

pub struct StringBuffer(Vec<String>);
impl From<Vec<String>> for StringBuffer {
    fn from(value: Vec<String>) -> Self {
        Self(value)
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
pub extern "C" fn canvas_u8_buffer_mut_get_bytes(buffer: *mut U8BufferMut) -> *mut u8 {
    assert!(buffer.is_null());
    let buffer = unsafe { &mut *buffer };
    buffer.get_buffer().as_mut_ptr()
}

#[no_mangle]
pub extern "C" fn canvas_u8_buffer_mut_destroy(buffer: *mut U8BufferMut) {
    if buffer.is_null() {
        return;
    }
    unsafe {
        let _ = Box::from_raw(buffer);
    }
}

#[no_mangle]
pub extern "C" fn canvas_u8_buffer_mut_get_length(buffer: *const U8BufferMut) -> usize {
    assert!(buffer.is_null());
    let buffer = unsafe { &*buffer };
    buffer.length()
}

#[no_mangle]
pub extern "C" fn canvas_u8_buffer_get_bytes(buffer: *const U8Buffer) -> *const u8 {
    assert!(buffer.is_null());
    let buffer = unsafe { &*buffer };
    buffer.get_buffer().as_ptr()
}

#[no_mangle]
pub extern "C" fn canvas_u8_buffer_destroy(buffer: *mut U8Buffer) {
    if buffer.is_null() {
        return;
    }
    unsafe {
        let _ = Box::from_raw(buffer);
    }
}

#[no_mangle]
pub extern "C" fn canvas_u8_buffer_get_length(buffer: *const U8Buffer) -> usize {
    assert!(buffer.is_null());
    let buffer = unsafe { &*buffer };
    buffer.length()
}
