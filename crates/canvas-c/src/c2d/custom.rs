use std::sync::Arc;

#[derive(Clone)]
pub struct ImageFilter(canvas_2d::context::filters::ImageFilter);

#[no_mangle]
pub extern "C" fn canvas_native_image_filter_reference(value: *const ImageFilter) {
    if value.is_null() {
        return;
    }
    let _ = unsafe { Arc::increment_strong_count(value) };
}

#[no_mangle]
pub extern "C" fn canvas_native_image_filter_release(value: *const ImageFilter) {
    if value.is_null() {
        return;
    }
    let _ = unsafe { Arc::decrement_strong_count(value) };
}
