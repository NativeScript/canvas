use jni::objects::JClass;
use jni::sys::{jint, jlong, jobject};
use jni::JNIEnv;

use canvas_core::context::pixel_manipulation::image_data::ImageData;

#[no_mangle]
pub extern "system" fn Java_org_nativescript_canvas_TNSImageData_nativeInit(
    _: JNIEnv,
    _: JClass,
    width: jint,
    height: jint,
) -> jlong {
    Box::into_raw(Box::new(ImageData::new(width, height))) as jlong
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_canvas_TNSImageData_nativeWidth(
    _: JNIEnv,
    _: JClass,
    image_data: jlong,
) -> jint {
    if image_data == 0 {
        return 0;
    }
    unsafe {
        let image_data: *mut ImageData = image_data as _;
        let image_data = &mut *image_data;
        image_data.width()
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_canvas_TNSImageData_nativeHeight(
    _: JNIEnv,
    _: JClass,
    image_data: jlong,
) -> jint {
    if image_data == 0 {
        return 0;
    }
    unsafe {
        let image_data: *mut ImageData = image_data as _;
        let image_data = &mut *image_data;
        image_data.height()
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_canvas_TNSImageData_nativeData(
    env: JNIEnv,
    _: JClass,
    image_data: jlong,
) -> jobject {
    if image_data == 0 {
        let mut slice = [0u8; 0];
        return env.new_direct_byte_buffer(&mut slice).unwrap().into_inner();
    }
    unsafe {
        let image_data: *mut ImageData = image_data as _;
        let image_data = &mut *image_data;
        let slice = image_data.data_mut();
        if let Ok(image_data) = env.new_direct_byte_buffer(slice) {
            return image_data.into_inner();
        }
        let mut slice = [0u8; 0];
        env.new_direct_byte_buffer(&mut slice).unwrap().into_inner()
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_canvas_TNSImageData_nativeDataLength(
    _: JNIEnv,
    _: JClass,
    image_data: jlong,
) -> jint {
    if image_data == 0 {
        return 0;
    }
    unsafe {
        let image_data: *mut ImageData = image_data as _;
        let image_data = &mut *image_data;
        image_data.data_len().try_into().unwrap_or_default()
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_canvas_TNSImageData_nativeDestroy(
    _: JNIEnv,
    _: JClass,
    image_data: jlong,
) {
    if image_data == 0 {
        return;
    }
    unsafe {
        let image_data: *mut ImageData = image_data as _;
        let _ = Box::from_raw(image_data);
    }
}
