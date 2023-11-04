use jni::objects::{JClass, JObject};
use jni::sys::{jboolean, jlong, JNI_FALSE, JNI_TRUE};
use jni::JNIEnv;

use canvas_core::image_asset::ImageAsset;

#[no_mangle]
pub extern "system" fn Java_org_nativescript_canvas_NSCImageAsset_nativeLoadFromBitmap(
    env: &JNIEnv,
    _: JClass,
    asset: jlong,
    bitmap: JObject,
) -> jboolean {
    if asset == 0 {
        return JNI_FALSE;
    }

    let asset = asset as *mut ImageAsset;
    let asset = unsafe { &mut *asset };

    let mut bm = crate::utils::image::BitmapBytes::new(env, bitmap);

    if let Some(bytes) = bm.data() {
        if asset.load_from_bytes(bytes) {
            return JNI_TRUE;
        }
    }

    JNI_FALSE
}
