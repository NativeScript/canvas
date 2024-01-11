use jni::objects::{JClass, JObject, JString};
use jni::sys::{jboolean, jfloat, jlong, JNI_FALSE, JNI_TRUE};
use jni::JNIEnv;

use canvas_2d::context::fill_and_stroke_styles::pattern::Repetition;
use canvas_2d::utils::image::from_image_slice;
use canvas_c::CanvasRenderingContext2D;
use canvas_c::PaintStyle;

#[no_mangle]
pub extern "system" fn nativeCreatePattern(
    mut env: JNIEnv,
    _: JClass,
    context: jlong,
    bitmap: JObject,
    repetition: JString,
) -> jlong {
    if context == 0 {
        return 0;
    }

    let context = context as *mut CanvasRenderingContext2D;

    let context = unsafe { &mut *context };

    let rep = env
        .get_string(&repetition)
        .map(|value| value.to_string_lossy().to_string())
        .unwrap_or("no-repeat".into());

    let mut bm = crate::utils::image::BitmapBytes::new(&env, bitmap);

    let info = bm.info();
    if let Some(bytes) = bm.data() {
        let info = info.unwrap();
        if let Some(image) = from_image_slice(bytes, info.width() as i32, info.height() as i32) {
            return Box::into_raw(Box::new(PaintStyle::new(Some(
                canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Pattern(
                    context.get_context().create_pattern(
                        image,
                        Repetition::try_from(rep.as_ref()).unwrap_or(Repetition::NoRepeat),
                    ),
                ),
            )))) as jlong;
        }
    }

    0
}

fn draw_image(
    context: jlong,
    image_data: &[u8],
    width: jfloat,
    height: jfloat,
    sx: jfloat,
    sy: jfloat,
    s_width: jfloat,
    s_height: jfloat,
    dx: jfloat,
    dy: jfloat,
    d_width: jfloat,
    d_height: jfloat,
) -> jboolean {
    unsafe {
        if context == 0 {
            return JNI_FALSE;
        }

        let context = context as *mut CanvasRenderingContext2D;

        let context = unsafe { &mut *context };

        if let Some(image) = from_image_slice(image_data, width as i32, height as i32) {
            context.make_current();
            let mut context = context.get_context_mut();
            context.draw_image_src_xywh_dst_xywh(
                &image, sx, sy, s_width, s_height, dx, dy, d_width, d_height,
            );
            return JNI_TRUE;
        }
        JNI_FALSE
    }
}

#[no_mangle]
pub extern "system" fn nativeDrawImageDxDyWithBitmap(
    env: JNIEnv,
    _: JClass,
    context: jlong,
    bitmap: JObject,
    width: jfloat,
    height: jfloat,
    dx: jfloat,
    dy: jfloat,
) -> jboolean {
    let bytes = crate::utils::image::get_bytes_from_bitmap(&env, bitmap);
    if let Some((bytes, _)) = bytes {
        return draw_image(
            context,
            bytes.as_slice(),
            width,
            height,
            0.0,
            0.0,
            width,
            height,
            dx,
            dy,
            width,
            height,
        );
    }
    JNI_FALSE
}

#[no_mangle]
pub extern "system" fn nativeDrawImageDxDyDwDhWithBitmap(
    env: JNIEnv,
    _: JClass,
    context: jlong,
    bitmap: JObject,
    width: jfloat,
    height: jfloat,
    dx: jfloat,
    dy: jfloat,
    d_width: jfloat,
    d_height: jfloat,
) -> jboolean {
    let bytes = crate::utils::image::get_bytes_from_bitmap(&env, bitmap);

    if let Some((bytes, _)) = bytes {
        return draw_image(
            context,
            bytes.as_slice(),
            width,
            height,
            0.0,
            0.0,
            width,
            height,
            dx,
            dy,
            d_width,
            d_height,
        );
    }
    JNI_FALSE
}

#[no_mangle]
pub extern "system" fn nativeDrawImageWithBitmap(
    env: &JNIEnv,
    _: JClass,
    context: jlong,
    bitmap: JObject,
    width: jfloat,
    height: jfloat,
    sx: jfloat,
    sy: jfloat,
    s_width: jfloat,
    s_height: jfloat,
    dx: jfloat,
    dy: jfloat,
    d_width: jfloat,
    d_height: jfloat,
) -> jboolean {
    let bytes = crate::utils::image::get_bytes_from_bitmap(&env, bitmap);

    if let Some((bytes, _)) = bytes {
        return draw_image(
            context,
            bytes.as_slice(),
            width,
            height,
            sx,
            sy,
            s_width,
            s_height,
            dx,
            dy,
            d_width,
            d_height,
        );
    }

    JNI_FALSE
}
