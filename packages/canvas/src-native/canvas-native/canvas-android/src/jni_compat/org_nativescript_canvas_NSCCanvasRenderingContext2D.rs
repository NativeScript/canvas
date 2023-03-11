use jni::objects::{JClass, JObject, JString};
use jni::sys::{jboolean, jlong, JNI_FALSE, JNI_TRUE};
use jni::JNIEnv;

use canvas_2d::context::fill_and_stroke_styles::pattern::Repetition;
use canvas_2d::utils::image::from_image_slice;
use canvas_core::image_asset::ImageAsset;
use canvas_cxx::CanvasRenderingContext2D;
use canvas_cxx::PaintStyle;

#[no_mangle]
pub extern "system" fn Java_org_nativescript_canvas_NSCImageAsset_nativeCreatePattern(
    mut env: JNIEnv,
    _: JClass,
    context: jlong,
    bitmap: JObject,
    repetition: JString
) -> jlong {
    if context == 0 {
        return 0;
    }

    let context = context as *mut CanvasRenderingContext2D;

    let context = unsafe { &mut *context };

    let rep = env.get_string(&repetition).map(|value| value.to_string_lossy().to_string()).unwrap_or("no-repeat".into());

    let mut bm = crate::utils::image::BitmapBytes::new(&env, bitmap);

    let info = bm.info();
    if let Some(bytes) = bm.data() {
        let info = info.unwrap();
        if let Some(image) = from_image_slice(bytes, info.width() as i32, info.height() as i32) {
            return Box::into_raw(Box::new(PaintStyle::new(Some(
                canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Pattern(
                    context
                        .get_context()
                        .create_pattern(image, Repetition::try_from(rep.as_ref()).unwrap_or(Repetition::NoRepeat)),
                ),
            )))) as jlong;
        }
    }

    0
}
