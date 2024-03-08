use skia_safe::Rect;

pub trait ScaleUtils {
    fn scale(&mut self, x: f32, y: f32);
    fn from_scale(rect: Rect, x: f32, y: f32) -> Self;
}

impl ScaleUtils for Rect {
    fn scale(&mut self, x: f32, y: f32) {
        self.left *= x;
        self.right *= x;
        self.top *= y;
        self.bottom *= y;
    }
    fn from_scale(rect: Rect, x: f32, y: f32) -> Self {
        Rect::from_xywh(rect.left * x, rect.top * y, rect.right * x, rect.bottom * y)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, Default)]
pub struct ByteBufInner {
    needs_to_clean: bool,
}

#[repr(C)]
pub struct ByteBufMut {
    pub data: *mut u8,
    pub len: usize,
    inner: ByteBufInner,
}

impl ByteBufMut {
    pub fn new(data: *mut u8, length: usize) -> Self {
        Self {
            data,
            len: length,
            inner: ByteBufInner {
                needs_to_clean: false,
            },
        }
    }

    pub fn as_slice<'a>(&self) -> &'a [u8] {
        unsafe { std::slice::from_raw_parts(self.data, self.len) }
    }

    pub fn as_mut_slice<'a>(&self) -> &'a mut [u8] {
        unsafe { std::slice::from_raw_parts_mut(self.data, self.len) }
    }
}

impl From<Vec<u8>> for ByteBufMut {
    fn from(vec: Vec<u8>) -> Self {
        let len = vec.len();
        let mut slice = vec.into_boxed_slice();
        let data = slice.as_mut_ptr();
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

impl Drop for ByteBufMut {
    fn drop(&mut self) {
        if !self.inner.needs_to_clean {
            return;
        }
        if !self.data.is_null() && self.len != 0 {
            unsafe {
                let _ =
                    Box::from_raw(std::slice::from_raw_parts_mut(self.data, self.len)).into_vec();
            }
        }
    }
}

unsafe impl Send for ByteBufMut {}
