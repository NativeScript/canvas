use bytes::{Bytes, BytesMut};

pub struct U8BufferMut(BytesMut);

impl Default for U8BufferMut {
    fn default() -> Self {
        Self(BytesMut::default())
    }
}

#[derive(Clone)]
pub struct U8Buffer(Bytes);

impl U8Buffer {
    pub fn get_buffer(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl Default for U8Buffer {
    fn default() -> Self {
        Self(Bytes::default())
    }
}

impl From<Vec<u8>> for U8Buffer {
    fn from(value: Vec<u8>) -> Self {
        U8Buffer(bytes::Bytes::from(value))
    }
}

pub struct U16Buffer(Vec<u16>);

impl U16Buffer {
    pub fn get_buffer(&self) -> &[u16] {
        self.0.as_slice()
    }
}

impl Default for U16Buffer {
    fn default() -> Self {
        Self(Vec::new())
    }
}

impl From<Vec<u16>> for U16Buffer {
    fn from(value: Vec<u16>) -> Self {
        Self(value)
    }
}

pub struct F32Buffer(Vec<f32>);

impl F32Buffer {
    pub fn get_buffer(&self) -> &[f32] {
        self.0.as_slice()
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
    pub fn get_buffer(&self) -> &[i32] {
        self.0.as_slice()
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
    pub fn get_buffer(&self) -> &[u32] {
        self.0.as_slice()
    }
}

impl Default for U32Buffer {
    fn default() -> Self {
        Self(Vec::new())
    }
}

impl From<Vec<u32>> for U32Buffer {
    fn from(value: Vec<u32>) -> Self {
        Self(value)
    }
}

pub struct StringBuffer(Vec<String>);

impl From<Vec<String>> for StringBuffer {
    fn from(value: Vec<String>) -> Self {
        Self(value)
    }
}
