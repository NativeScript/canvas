use jni::objects::{JClass, JFloatArray, JIntArray, JObject, JString};
use jni::sys::{jboolean, jfloat, jint, jlong, JNI_FALSE, JNI_TRUE};
use jni::JNIEnv;
use skia_safe::Color;

use canvas_2d::context::compositing::composite_operation_type::CompositeOperationType;
use canvas_2d::context::fill_and_stroke_styles::pattern::Repetition;
use canvas_2d::utils::image::from_image_slice;
use canvas_c::PaintStyle;
use canvas_c::{CanvasRenderingContext2D, ImageAsset};

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


fn draw_image_dx_dy(
    context: jlong,
    image_data: &[u8],
    width: jfloat,
    height: jfloat,
    dx: jfloat,
    dy: jfloat,
) -> jboolean {
    if context == 0 {
        return JNI_FALSE;
    }

    let context = context as *mut CanvasRenderingContext2D;

    let context = unsafe { &mut *context };

    if let Some(image) = from_image_slice(image_data, width as i32, height as i32) {
        context.make_current();
        let mut context = context.get_context_mut();
        context.draw_image_dx_dy(
            &image, dx, dy
        );
        return JNI_TRUE;
    }
    JNI_FALSE
}



fn draw_image_dx_dy_dw_dh(
    context: jlong,
    image_data: &[u8],
    width: jfloat,
    height: jfloat,
    dx: jfloat,
    dy: jfloat,
    d_width: jfloat,
    d_height: jfloat,
) -> jboolean {
    if context == 0 {
        return JNI_FALSE;
    }

    let context = context as *mut CanvasRenderingContext2D;

    let context = unsafe { &mut *context };

    if let Some(image) = from_image_slice(image_data, width as i32, height as i32) {
        context.make_current();
        let mut context = context.get_context_mut();
        context.draw_image_dx_dy_dw_dh(
            &image, dx, dy, d_width, d_height
        );
        return JNI_TRUE;
    }
    JNI_FALSE
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
        return draw_image_dx_dy(
            context,
            bytes.as_slice(),
            width,
            height,
            dx,
            dy
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
        return draw_image_dx_dy_dw_dh(
            context,
            bytes.as_slice(),
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
    env: JNIEnv,
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

#[no_mangle]
pub extern "system" fn nativeDrawAtlasWithBitmap(
    env: JNIEnv,
    _: JClass,
    context: jlong,
    bitmap: JObject,
    xform: JFloatArray,
    tex: JFloatArray,
    colors: JIntArray,
    blend_mode: jint,
) {
    let bytes = crate::utils::image::get_bytes_from_bitmap(&env, bitmap);

    if let Some((bytes, info)) = bytes {
        let xform_len = env.get_array_length(&xform).unwrap_or_default();
        let tex_len = env.get_array_length(&tex).unwrap_or_default();
        let colors_len = env.get_array_length(&colors).unwrap_or_default();

        if xform_len == 0 || tex_len == 0 {
            return;
        }

        let mut xform_buf: Vec<f32> = Vec::with_capacity(xform_len as usize);
        let mut tex_buf: Vec<f32> = Vec::with_capacity(tex_len as usize);

        if let (Ok(_), Ok(_)) = (
            env.get_float_array_region(xform, 0, xform_buf.as_mut_slice()),
            env.get_float_array_region(tex, 0, tex_buf.as_mut_slice()),
        ) {
            let context = context as *mut CanvasRenderingContext2D;

            let context = unsafe { &mut *context };

            if let Some(image) =
                from_image_slice(bytes.as_slice(), info.width() as i32, info.height() as i32)
            {
                context.make_current();
                let mut context = context.get_context_mut();

                let colors_value: Option<Vec<Color>> = if colors_len > 0 {
                    let mut colors_buf: Vec<i32> = Vec::with_capacity(colors_len as usize);
                    let _ = env.get_int_array_region(colors, 0, colors_buf.as_mut_slice());

                    Some(
                        colors_buf
                            .into_iter()
                            .map(|color| Color::new(color as u32))
                            .collect(),
                    )
                } else {
                    None
                };
                context.draw_atlas(
                    &image,
                    xform_buf.as_slice(),
                    tex_buf.as_slice(),
                    colors_value.as_deref(),
                    CompositeOperationType::from(blend_mode),
                )
            }
        }
    }
}

fn draw_image_with_asset(
    context: jlong,
    image: jlong,
    sx: jfloat,
    sy: jfloat,
    s_width: jfloat,
    s_height: jfloat,
    dx: jfloat,
    dy: jfloat,
    d_width: jfloat,
    d_height: jfloat,
) -> jboolean {
    if context == 0 || image == 0 {
        return JNI_FALSE;
    }

    let context = context as *mut CanvasRenderingContext2D;
    let image = image as *mut ImageAsset;

    canvas_c::canvas_native_context_draw_image_asset(
        context, image, sx, sy, s_width, s_height, dx, dy, d_width, d_height,
    );

    JNI_TRUE
}

#[no_mangle]
pub extern "system" fn nativeDrawImageDxDyWithAsset(
    _env: JNIEnv,
    _: JClass,
    context: jlong,
    image: jlong,
    dx: jfloat,
    dy: jfloat,
) -> jboolean {
    if context == 0 || image == 0 {
        return JNI_FALSE;
    }

    let context = context as *mut CanvasRenderingContext2D;
    let image = image as *mut ImageAsset;

    canvas_c::canvas_native_context_draw_image_dx_dy_asset(context, image, dx, dy);

    JNI_TRUE
}

#[no_mangle]
pub extern "system" fn nativeDrawImageDxDyDwDhWithAsset(
    _env: JNIEnv,
    _: JClass,
    context: jlong,
    image: jlong,
    dx: jfloat,
    dy: jfloat,
    d_width: jfloat,
    d_height: jfloat,
) -> jboolean {
    if context == 0 || image == 0 {
        return JNI_FALSE;
    }

    let context = context as *mut CanvasRenderingContext2D;
    let image = image as *mut ImageAsset;

    canvas_c::canvas_native_context_draw_image_dx_dy_dw_dh_asset(
        context, image, dx, dy, d_width, d_height,
    );

    JNI_TRUE
}

#[no_mangle]
pub extern "system" fn nativeDrawImageWithAsset(
    _env: JNIEnv,
    _: JClass,
    context: jlong,
    image: jlong,
    sx: jfloat,
    sy: jfloat,
    s_width: jfloat,
    s_height: jfloat,
    dx: jfloat,
    dy: jfloat,
    d_width: jfloat,
    d_height: jfloat,
) -> jboolean {
    if context == 0 || image == 0 {
        return JNI_FALSE;
    }

    let context = context as *mut CanvasRenderingContext2D;
    let image = image as *mut ImageAsset;

    canvas_c::canvas_native_context_draw_image_asset(
        context, image, sx, sy, s_width, s_height, dx, dy, d_width, d_height,
    );

    JNI_TRUE
}
