use crate::buffers::U8Buffer;

#[derive(Debug, Clone)]
pub struct ImageData(pub(crate) canvas_2d::context::pixel_manipulation::image_data::ImageData);

impl ImageData {
    pub(crate) fn new(data: canvas_2d::context::pixel_manipulation::image_data::ImageData) -> Self {
        ImageData(data)
    }

    pub fn inner(&self) -> &canvas_2d::context::pixel_manipulation::image_data::ImageData {
        &self.0
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_image_data_create(width: i32, height: i32) -> *mut ImageData {
    let data = canvas_2d::context::pixel_manipulation::ImageData::new(width, height);
    Box::into_raw(Box::new(ImageData::new(data)))
}

#[no_mangle]
pub extern "C" fn canvas_native_image_data_get_width(image_data: &ImageData) -> i32 {
    image_data.0.width()
}

#[no_mangle]
pub extern "C" fn canvas_native_image_data_get_height(image_data: &ImageData) -> i32 {
    image_data.0.height()
}

#[no_mangle]
pub extern "C" fn canvas_native_image_data_get_length(image_data: &ImageData) -> usize {
    image_data.0.data_len()
}


#[no_mangle]
pub extern "C" fn canvas_native_image_data_get_data(image_data: &ImageData) -> *mut U8Buffer {
    Box::into_raw(Box::new(U8Buffer::from(image_data.clone())))
}

#[no_mangle]
pub extern "C" fn canvas_native_image_data_release(value: *mut ImageData) {
    if value.is_null() {
        return;
    }
    unsafe { drop(Box::from_raw(value)) };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::buffers::{
        canvas_native_u8_buffer_get_bytes_mut, canvas_native_u8_buffer_get_length,
        canvas_native_u8_buffer_release,
    };

    // `ImageData` is a manually refcounted handle: cloning it copies the raw
    // pixel pointer and bumps an Arc, and the storage is freed only when the
    // last handle drops. `canvas_native_image_data_get_data` therefore hands
    // back a *second handle to the same pixels*, not a copy of them -- which is
    // what makes `imageData.data` a live view on the JS side, and what the
    // Apple bridge's ImageDataBuffer relies on.

    #[test]
    fn get_data_aliases_the_same_pixels() {
        let image = canvas_native_image_data_create(2, 2);
        let buffer = canvas_native_image_data_get_data(unsafe { &*image });

        unsafe { *canvas_native_u8_buffer_get_bytes_mut(buffer) = 0xAB };

        // The write through the buffer is visible on the ImageData, so the two
        // are the same storage. A deep copy here would silently break putImageData.
        assert_eq!(unsafe { (*image).0.data()[0] }, 0xAB);

        canvas_native_u8_buffer_release(buffer);
        canvas_native_image_data_release(image);
    }

    #[test]
    fn releasing_the_buffer_leaves_the_image_data_usable() {
        let image = canvas_native_image_data_create(2, 2);
        let buffer = canvas_native_image_data_get_data(unsafe { &*image });
        unsafe { *canvas_native_u8_buffer_get_bytes_mut(buffer) = 0xCD };

        canvas_native_u8_buffer_release(buffer);

        // get_data borrows its argument, so releasing the buffer must not have
        // released the ImageData with it. The Apple ImageDataBuffer destructor
        // used to call canvas_native_image_data_release here as well, which
        // double-freed the Box that ImageDataImpl still owns.
        assert_eq!(canvas_native_image_data_get_length(unsafe { &*image }), 16);
        assert_eq!(unsafe { (*image).0.data()[0] }, 0xCD);

        canvas_native_image_data_release(image);
    }

    #[test]
    fn buffer_outlives_the_image_data_handle() {
        let image = canvas_native_image_data_create(2, 2);
        let buffer = canvas_native_image_data_get_data(unsafe { &*image });
        unsafe { *canvas_native_u8_buffer_get_bytes_mut(buffer) = 0x11 };

        // Drop the wrapper first: the pixels stay alive because the buffer still
        // holds a handle, so a retained JS pixel view survives collection of the
        // ImageData wrapper it came from.
        canvas_native_image_data_release(image);

        assert_eq!(canvas_native_u8_buffer_get_length(buffer), 16);
        assert_eq!(unsafe { *canvas_native_u8_buffer_get_bytes_mut(buffer) }, 0x11);
        unsafe { *canvas_native_u8_buffer_get_bytes_mut(buffer) = 0x22 };

        canvas_native_u8_buffer_release(buffer);
    }

    #[test]
    fn release_is_null_safe() {
        canvas_native_image_data_release(std::ptr::null_mut());
    }
}
