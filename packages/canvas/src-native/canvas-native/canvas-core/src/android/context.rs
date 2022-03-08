use std::str::FromStr;


use jni::JNIEnv;
use jni::objects::{JClass, JObject, JString, JValue, ReleaseMode};
use jni::sys::{
    jboolean, jbyteArray, jfloat, jfloatArray, jint, jlong, JNI_FALSE, JNI_TRUE, jobject, jstring,
};
use skia_safe::Rect;

use crate::common::context::compositing::composite_operation_type::CompositeOperationType;
use crate::common::context::Context;
use crate::common::context::drawing_paths::fill_rule::FillRule;
use crate::common::context::fill_and_stroke_styles::paint::PaintStyle;
use crate::common::context::fill_and_stroke_styles::pattern::Repetition;
use crate::common::context::image_asset::ImageAsset;
use crate::common::context::image_smoothing::ImageSmoothingQuality;

use crate::common::context::line_styles::line_join::LineJoin;
use crate::common::context::matrix::Matrix;
use crate::common::context::paths::path::Path;
use crate::common::context::pixel_manipulation::image_data::ImageData;
use crate::common::context::text_styles::text_align::TextAlign;
use crate::common::context::text_styles::text_baseline::TextBaseLine;
use crate::common::context::text_styles::text_direction::TextDirection;
use crate::common::ffi::paint_style_value::{PaintStyleValueType};
use crate::common::utils::color::to_parsed_color;
use crate::common::utils::image::{from_image_slice, from_image_slice_encoded};

const JSON_CLASS: &str = "org/json/JSONObject";
const SIG_OBJECT_CTOR: &str = "()V";

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSCanvasRenderingContext2D_nativeSetDirection(
    _: JNIEnv,
    _: JClass,
    context: jlong,
    direction: jint,
) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        context.set_direction(TextDirection::from(direction));
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSCanvasRenderingContext2D_nativeGetDirection(
    _: JNIEnv,
    _: JClass,
    context: jlong,
) -> jint {
    unsafe {
        let context: *const Context = context as _;
        let context = &*context;
        context.direction().into()
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSCanvasRenderingContext2D_nativeSetFillStyle(
    _: JNIEnv,
    _: JClass,
    context: jlong,
    style: jlong,
) {
    unsafe {
        if context == 0 || style == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        let style: *const PaintStyle = style as _;
        let style = &*style;
        context.set_fill_style(style.clone())
    }
}

fn get_style(env: JNIEnv, context: jlong, is_fill: bool) -> jobject {
    let json_class = env.find_class(JSON_CLASS).unwrap();
    let json = env.new_object(json_class, SIG_OBJECT_CTOR, &[]).unwrap();
    if context == 0 {
        return json.into_inner();
    }
    unsafe {
        let context: *mut Context = context as _;
        let context = &mut *context;
        let style;
        if is_fill {
            style = context.fill_style().clone();
        } else {
            style = context.stroke_style().clone();
        }
        let mut value_args = vec![env.new_string("value").unwrap().into()];
        let mut value_type_args = vec![env.new_string("value_type").unwrap().into()];
        match style {
            PaintStyle::Color(_) => {
                value_type_args.push(JValue::Int(
                    PaintStyleValueType::PaintStyleValueTypeColor.into(),
                ));
            }
            PaintStyle::Gradient(_) => {
                value_type_args.push(JValue::Int(
                    PaintStyleValueType::PaintStyleValueTypeGradient.into(),
                ));
            }
            PaintStyle::Pattern(_) => {
                value_type_args.push(JValue::Int(
                    PaintStyleValueType::PaintStyleValueTypePattern.into(),
                ));
            }
        };
        let style = Box::into_raw(Box::new(style)) as jlong;
        value_args.push(JValue::Long(style));
        env.call_method(
            json,
            "put",
            "(Ljava/lang/String;J)Lorg/json/JSONObject;",
            value_args.as_slice(),
        )
            .unwrap();
        env.call_method(
            json,
            "put",
            "(Ljava/lang/String;I)Lorg/json/JSONObject;",
            value_type_args.as_slice(),
        )
            .unwrap();
        json.into_inner()
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSCanvasRenderingContext2D_nativeGetFillStyle(
    env: JNIEnv,
    _: JClass,
    context: jlong,
) -> jobject {
    get_style(env, context, true)
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSCanvasRenderingContext2D_nativeSetFilter(
    env: JNIEnv,
    _: JClass,
    context: jlong,
    filter: JString,
) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        if let Ok(filter) = env.get_string(filter) {
            let filter = filter.to_string_lossy();
            context.set_filter(filter.as_ref())
        }
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSCanvasRenderingContext2D_nativeGetFilter(
    env: JNIEnv,
    _: JClass,
    context: jlong,
) -> jstring {
    unsafe {
        let context: *const Context = context as _;
        let context = &*context;
        let filter = context.get_filter();
        env.new_string(filter).unwrap().into_inner()
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSCanvasRenderingContext2D_nativeSetFont(
    env: JNIEnv,
    _: JClass,
    context: jlong,
    filter: JString,
) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        if let Ok(font) = env.get_string(filter) {
            let font = font.to_string_lossy();
            context.set_font(font.as_ref());
        }
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSCanvasRenderingContext2D_nativeGetFont(
    env: JNIEnv,
    _: JClass,
    context: jlong,
) -> jstring {
    unsafe {
        let context: *const Context = context as _;
        let context = &*context;
        let font = context.font();
        env.new_string(font).unwrap().into_inner()
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSCanvasRenderingContext2D_nativeSetGlobalAlpha(
    _: JNIEnv,
    _: JClass,
    context: jlong,
    alpha: jfloat,
) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        context.set_global_alpha(alpha)
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSCanvasRenderingContext2D_nativeGetGlobalAlpha(
    _: JNIEnv,
    _: JClass,
    context: jlong,
) -> jfloat {
    unsafe {
        let context: *const Context = context as _;
        let context = &*context;
        context.global_alpha()
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSCanvasRenderingContext2D_nativeSetGlobalCompositeOperation(
    _: JNIEnv,
    _: JClass,
    context: jlong,
    operation: jint,
) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        context.set_global_composite_operation(CompositeOperationType::from(operation));
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSCanvasRenderingContext2D_nativeGetGlobalCompositeOperation(
    _: JNIEnv,
    _: JClass,
    context: jlong,
) -> jint {
    unsafe {
        let context: *const Context = context as _;
        let context = &*context;
        context.global_composite_operation().into()
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSCanvasRenderingContext2D_nativeSetImageSmoothingEnabled(
    _: JNIEnv,
    _: JClass,
    context: jlong,
    enabled: jboolean,
) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        context.set_image_smoothing_enabled(enabled == JNI_TRUE);
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSCanvasRenderingContext2D_nativeGetImageSmoothingEnabled(
    _: JNIEnv,
    _: JClass,
    context: jlong,
) -> jboolean {
    unsafe {
        let context: *mut Context = context as _;
        let context = &mut *context;
        return if context.get_image_smoothing_enabled() {
            JNI_TRUE
        } else {
            JNI_FALSE
        };
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSCanvasRenderingContext2D_nativeSetImageSmoothingQuality(
    _: JNIEnv,
    _: JClass,
    context: jlong,
    quality: jint,
) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        context.set_image_smoothing_quality(ImageSmoothingQuality::from(quality));
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSCanvasRenderingContext2D_nativeGetImageSmoothingQuality(
    _: JNIEnv,
    _: JClass,
    context: jlong,
) -> jint {
    unsafe {
        let context: *const Context = context as _;
        let context = &*context;
        context.get_image_smoothing_quality().into()
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSCanvasRenderingContext2D_nativeSetLineCap(
    _: JNIEnv,
    _: JClass,
    context: jlong,
    cap: jint,
) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        context.set_line_cap(cap.into())
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSCanvasRenderingContext2D_nativeGetLineCap(
    _: JNIEnv,
    _: JClass,
    context: jlong,
) -> jint {
    unsafe {
        let context: *const Context = context as _;
        let context = &*context;
        context.line_cap().into()
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSCanvasRenderingContext2D_nativeSetLineDashOffset(
    _: JNIEnv,
    _: JClass,
    context: jlong,
    offset: jfloat,
) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        context.set_line_dash_offset(offset)
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSCanvasRenderingContext2D_nativeGetLineDashOffset(
    _: JNIEnv,
    _: JClass,
    context: jlong,
) -> jfloat {
    unsafe {
        let context: *const Context = context as _;
        let context = &*context;
        context.line_dash_offset()
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSCanvasRenderingContext2D_nativeSetLineJoin(
    _: JNIEnv,
    _: JClass,
    context: jlong,
    join: jint,
) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        context.set_line_join(LineJoin::from(join))
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSCanvasRenderingContext2D_nativeGetLineJoin(
    _: JNIEnv,
    _: JClass,
    context: jlong,
) -> jint {
    unsafe {
        let context: *const Context = context as _;
        let context = &*context;
        context.line_join().into()
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSCanvasRenderingContext2D_nativeSetLineWidth(
    _: JNIEnv,
    _: JClass,
    context: jlong,
    width: jfloat,
) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        context.set_line_width(width)
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSCanvasRenderingContext2D_nativeGetLineWidth(
    _: JNIEnv,
    _: JClass,
    context: jlong,
) -> jfloat {
    unsafe {
        let context: *const Context = context as _;
        let context = &*context;
        context.line_width()
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSCanvasRenderingContext2D_nativeSetMiterLimit(
    _: JNIEnv,
    _: JClass,
    context: jlong,
    limit: jfloat,
) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        context.set_miter_limit(limit)
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSCanvasRenderingContext2D_nativeGetMiterLimit(
    _: JNIEnv,
    _: JClass,
    context: jlong,
) -> jfloat {
    unsafe {
        let context: *const Context = context as _;
        let context = &*context;
        context.miter_limit()
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSCanvasRenderingContext2D_nativeSetShadowBlur(
    _: JNIEnv,
    _: JClass,
    context: jlong,
    blur: jfloat,
) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        context.set_shadow_blur(blur)
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSCanvasRenderingContext2D_nativeGetShadowBlur(
    _: JNIEnv,
    _: JClass,
    context: jlong,
) -> jfloat {
    unsafe {
        let context: *const Context = context as _;
        let context = &*context;
        context.shadow_blur()
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSCanvasRenderingContext2D_nativeSetShadowColor(
    _: JNIEnv,
    _: JClass,
    context: jlong,
    r: u8,
    g: u8,
    b: u8,
    a: u8,
) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        context.set_shadow_color(skia_safe::Color::from_argb(a, r, g, b))
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSCanvasRenderingContext2D_nativeSetShadowColorString(
    env: JNIEnv,
    _: JClass,
    context: jlong,
    color: JString,
) {
    unsafe {
        if context == 0 || color.is_null() {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        if let Ok(color) = env.get_string(color) {
            let color = color.to_string_lossy();
            if let Ok(color) = css_color_parser::Color::from_str(color.as_ref()) {
                context.set_shadow_color(skia_safe::Color::from_argb(
                    (color.a * 255.0) as u8,
                    color.r,
                    color.g,
                    color.b,
                ))
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSCanvasRenderingContext2D_nativeGetShadowColor(
    env: JNIEnv,
    _: JClass,
    context: jlong,
) -> jstring {
    unsafe {
        let context: *const Context = context as _;
        let context = &*context;
        env.new_string(to_parsed_color(context.shadow_color()))
            .unwrap()
            .into_inner()
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSCanvasRenderingContext2D_nativeSetShadowOffsetX(
    _: JNIEnv,
    _: JClass,
    context: jlong,
    x: jfloat,
) {
    unsafe {
        let context: *mut Context = context as _;
        let context = &mut *context;
        context.set_shadow_offset_x(x)
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSCanvasRenderingContext2D_nativeGetShadowOffsetX(
    _: JNIEnv,
    _: JClass,
    context: jlong,
) -> jfloat {
    unsafe {
        let context: *const Context = context as _;
        let context = &*context;
        context.shadow_offset_x()
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSCanvasRenderingContext2D_nativeSetShadowOffsetY(
    _: JNIEnv,
    _: JClass,
    context: jlong,
    y: jfloat,
) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        context.set_shadow_offset_y(y)
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSCanvasRenderingContext2D_nativeGetShadowOffsetY(
    _: JNIEnv,
    _: JClass,
    context: jlong,
) -> jfloat {
    unsafe {
        let context: *const Context = context as _;
        let context = &*context;
        context.shadow_offset_y()
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSCanvasRenderingContext2D_nativeSetStrokeStyle(
    _: JNIEnv,
    _: JClass,
    context: jlong,
    style: jlong,
) {
    unsafe {
        if context == 0 || style == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        let style: *const PaintStyle = style as _;
        let style = &*style;
        context.set_stroke_style(style.clone())
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSCanvasRenderingContext2D_nativeGetStrokeStyle(
    env: JNIEnv,
    _: JClass,
    context: jlong,
) -> jobject {
    get_style(env, context, false)
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSCanvasRenderingContext2D_nativeSetTextAlign(
    _: JNIEnv,
    _: JClass,
    context: jlong,
    align: jint,
) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        context.set_text_align(TextAlign::from(align))
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSCanvasRenderingContext2D_nativeGetTextAlign(
    _: JNIEnv,
    _: JClass,
    context: jlong,
) -> jint {
    unsafe {
        let context: *const Context = context as _;
        let context = &*context;
        context.text_align().into()
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSCanvasRenderingContext2D_nativeSetTextBaseline(
    _: JNIEnv,
    _: JClass,
    context: jlong,
    baseline: jint,
) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        context.set_text_baseline(TextBaseLine::from(baseline));
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSCanvasRenderingContext2D_nativeGetTextBaseline(
    _: JNIEnv,
    _: JClass,
    context: jlong,
) -> jint {
    unsafe {
        let context: *const Context = context as _;
        let context = &*context;
        context.text_baseline().into()
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSCanvasRenderingContext2D_nativeArc(
    _: JNIEnv,
    _: JClass,
    context: jlong,
    x: jfloat,
    y: jfloat,
    radius: jfloat,
    start_angle: jfloat,
    end_angle: jfloat,
    anti_clockwise: jboolean,
) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        context.arc(
            x,
            y,
            radius,
            start_angle,
            end_angle,
            anti_clockwise == JNI_TRUE,
        )
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSCanvasRenderingContext2D_nativeArcTo(
    _: JNIEnv,
    _: JClass,
    context: jlong,
    x1: jfloat,
    y1: jfloat,
    x2: jfloat,
    y2: jfloat,
    radius: jfloat,
) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        context.arc_to(x1, y1, x2, y2, radius)
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSCanvasRenderingContext2D_nativeBeginPath(
    _: JNIEnv,
    _: JClass,
    context: jlong,
) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        context.begin_path()
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSCanvasRenderingContext2D_nativeBezierCurveTo(
    _: JNIEnv,
    _: JClass,
    context: jlong,
    cp1x: jfloat,
    cp1y: jfloat,
    cp2x: jfloat,
    cp2y: jfloat,
    x: jfloat,
    y: jfloat,
) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        context.bezier_curve_to(cp1x, cp1y, cp2x, cp2y, x, y)
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSCanvasRenderingContext2D_nativeClearRect(
    _: JNIEnv,
    _: JClass,
    context: jlong,
    x: jfloat,
    y: jfloat,
    width: jfloat,
    height: jfloat,
) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        context.clear_rect(x, y, width, height)
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSCanvasRenderingContext2D_nativeClip(
    _: JNIEnv,
    _: JClass,
    context: jlong,
    path: jlong,
    rule: jint,
) {
    unsafe {
        if context == 0 || path == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        let path: *mut Path = path as _;
        context.clip(Some(&mut *path), Some(FillRule::from(rule)))
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSCanvasRenderingContext2D_nativeClipRule(
    _: JNIEnv,
    _: JClass,
    context: jlong,
    rule: jint,
) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        context.clip(None, Some(FillRule::from(rule)))
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSCanvasRenderingContext2D_nativeClosePath(
    _: JNIEnv,
    _: JClass,
    context: jlong,
) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        context.close_path()
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSCanvasRenderingContext2D_nativeCreateImageData(
    _: JNIEnv,
    _: JClass,
    width: jint,
    height: jint,
) -> jlong {
    Box::into_raw(Box::new(Context::create_image_data(width, height))) as jlong
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSCanvasRenderingContext2D_nativeCreateLinearGradient(
    _: JNIEnv,
    _: JClass,
    context: jlong,
    x0: jfloat,
    y0: jfloat,
    x1: jfloat,
    y1: jfloat,
) -> jlong {
    unsafe {
        if context == 0 {
            return 0;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        Box::into_raw(Box::new(PaintStyle::Gradient(
            context.create_linear_gradient(x0, y0, x1, y1),
        ))) as jlong
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSCanvasRenderingContext2D_nativeCreatePatternEncoded(
    env: JNIEnv,
    _: JClass,
    context: jlong,
    image_data: jbyteArray,
    repetition: jint,
) -> jlong {
    
    unsafe {
        if context == 0 {
            return 0;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        if let Ok(val) = env.get_byte_array_elements(image_data, ReleaseMode::NoCopyBack) {
            let length = val.size().unwrap_or(0) as usize;
            let buf = std::slice::from_raw_parts(
                std::mem::transmute::<*mut i8, *mut u8>(val.as_ptr()),
                length,
            );
            if let Some(image) = from_image_slice_encoded(buf) {
                return Box::into_raw(Box::new(PaintStyle::Pattern(
                    context.create_pattern(image, Repetition::from(repetition)),
                ))) as jlong;
            }
        }
        0
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSCanvasRenderingContext2D_nativeCreatePattern(
    env: JNIEnv,
    _: JClass,
    context: jlong,
    image_data: jbyteArray,
    width: jint,
    height: jint,
    repetition: jint,
) -> jlong {
    
    unsafe {
        if context == 0 {
            return 0;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        if let Ok(val) = env.get_byte_array_elements(image_data, ReleaseMode::NoCopyBack) {
            let length = val.size().unwrap_or(0) as usize;
            let buf = std::slice::from_raw_parts(
                std::mem::transmute::<*mut i8, *mut u8>(val.as_ptr()),
                length,
            );

            if let Some(image) = from_image_slice(buf, width, height) {
                return Box::into_raw(Box::new(PaintStyle::Pattern(
                    context.create_pattern(image, Repetition::from(repetition)),
                ))) as jlong;
            }
        }
        0
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSCanvasRenderingContext2D_nativeCreatePatternWithAsset(
    _: JNIEnv,
    _: JClass,
    context: jlong,
    asset: jlong,
    repetition: jint,
) -> jlong {
    unsafe {
        if context == 0 {
            return 0;
        }
        if asset == 0 {
            return 0;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        let asset: *mut ImageAsset = asset as _;
        let asset = &mut *asset;
        let bytes = asset.rgba_internal_bytes();
        if let Some(image) = from_image_slice(
            bytes.as_slice(),
            asset.width() as i32,
            asset.height() as i32,
        ) {
            return Box::into_raw(Box::new(PaintStyle::Pattern(
                context.create_pattern(image, Repetition::from(repetition)),
            ))) as jlong;
        }
        0
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSCanvasRenderingContext2D_nativeCreateRadialGradient(
    _: JNIEnv,
    _: JClass,
    context: jlong,
    x0: jfloat,
    y0: jfloat,
    r0: jfloat,
    x1: jfloat,
    y1: jfloat,
    r1: jfloat,
) -> jlong {
    unsafe {
        if context == 0 {
            return 0;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        Box::into_raw(Box::new(PaintStyle::Gradient(
            context.create_radial_gradient(x0, y0, r0, x1, y1, r1),
        ))) as jlong
    }
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
) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        if let Some(image) = from_image_slice(image_data, width as i32, height as i32) {
            context.draw_image(
                &image,
                Rect::from_xywh(sx, sy, s_width, s_height),
                Rect::from_xywh(dx, dy, d_width, d_height),
            )
        }
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSCanvasRenderingContext2D_nativeDrawImageDxDyWithBitmap(
    env: JNIEnv,
    _: JClass,
    context: jlong,
    bitmap: JObject,
    width: jfloat,
    height: jfloat,
    dx: jfloat,
    dy: jfloat,
) {
    let bytes = crate::android::utils::image::get_bytes_from_bitmap(env, bitmap);
    draw_image(
        context,
        bytes.0.as_slice(),
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

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSCanvasRenderingContext2D_nativeDrawImageDxDy(
    env: JNIEnv,
    _: JClass,
    context: jlong,
    image_data: jbyteArray,
    width: jfloat,
    height: jfloat,
    dx: jfloat,
    dy: jfloat,
) {
    unsafe {
        if let Ok(val) = env.get_byte_array_elements(image_data, ReleaseMode::NoCopyBack) {
            let length = val.size().unwrap_or(0) as usize;
            let buf = std::slice::from_raw_parts(
                std::mem::transmute::<*mut i8, *mut u8>(val.as_ptr()),
                length,
            );
            draw_image(
                context, buf, width, height, 0.0, 0.0, width, height, dx, dy, width, height,
            );
        }
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSCanvasRenderingContext2D_nativeDrawImageDxDyWithAsset(
    _: JNIEnv,
    _: JClass,
    context: jlong,
    asset: jlong,
    dx: jfloat,
    dy: jfloat,
) {
    if asset == 0 {
        return;
    }
    unsafe {
        let asset: *mut ImageAsset = asset as _;
        let asset = &mut *asset;
        let bytes = asset.rgba_internal_bytes();
        let width = asset.width() as f32;
        let height = asset.height() as f32;
        draw_image(
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
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSCanvasRenderingContext2D_nativeDrawImageDxDyDwDhWithBitmap(
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
) {
    let bytes = crate::android::utils::image::get_bytes_from_bitmap(env, bitmap);
    draw_image(
        context,
        bytes.0.as_slice(),
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
    )
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSCanvasRenderingContext2D_nativeDrawImageDxDyDwDh(
    env: JNIEnv,
    _: JClass,
    context: jlong,
    image_data: jbyteArray,
    width: jfloat,
    height: jfloat,
    dx: jfloat,
    dy: jfloat,
    d_width: jfloat,
    d_height: jfloat,
) {
    unsafe {
        if let Ok(val) = env.get_byte_array_elements(image_data, ReleaseMode::NoCopyBack) {
            let length = val.size().unwrap_or(0) as usize;
            let buf = std::slice::from_raw_parts(
                std::mem::transmute::<*mut i8, *mut u8>(val.as_ptr()),
                length,
            );
            draw_image(
                context, buf, width, height, 0.0, 0.0, width, height, dx, dy, d_width, d_height,
            )
        }
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSCanvasRenderingContext2D_nativeDrawImageDxDyDwDhWithAsset(
    _: JNIEnv,
    _: JClass,
    context: jlong,
    asset: jlong,
    dx: jfloat,
    dy: jfloat,
    d_width: jfloat,
    d_height: jfloat,
) {
    if asset == 0 {
        return;
    }
    unsafe {
        let asset: *mut ImageAsset = asset as _;
        let asset = &mut *asset;
        let bytes = asset.rgba_internal_bytes();
        let width = asset.width() as f32;
        let height = asset.height() as f32;
        draw_image(
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
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSCanvasRenderingContext2D_nativeDrawImageWithBitmap(
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
) {
    let bytes = crate::android::utils::image::get_bytes_from_bitmap(env, bitmap);
    draw_image(
        context,
        bytes.0.as_slice(),
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
    )
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSCanvasRenderingContext2D_nativeDrawImageWithAsset(
    _: JNIEnv,
    _: JClass,
    context: jlong,
    asset: jlong,
    sx: jfloat,
    sy: jfloat,
    s_width: jfloat,
    s_height: jfloat,
    dx: jfloat,
    dy: jfloat,
    d_width: jfloat,
    d_height: jfloat,
) {
    if asset == 0 {
        return;
    }
    unsafe {
        let asset: *mut ImageAsset = asset as _;
        let asset = &mut *asset;
        let bytes = asset.rgba_internal_bytes();
        let width = asset.width() as f32;
        let height = asset.height() as f32;
        draw_image(
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
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSCanvasRenderingContext2D_nativeDrawImage(
    env: JNIEnv,
    _: JClass,
    context: jlong,
    image_data: jbyteArray,
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
) {
    unsafe {
        if let Ok(val) = env.get_byte_array_elements(image_data, ReleaseMode::NoCopyBack) {
            let length = val.size().unwrap_or(0) as usize;
            let buf = std::slice::from_raw_parts(
                std::mem::transmute::<*mut i8, *mut u8>(val.as_ptr()),
                length,
            );
            draw_image(
                context, buf, width, height, sx, sy, s_width, s_height, dx, dy, d_width, d_height,
            )
        }
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSCanvasRenderingContext2D_nativeEllipse(
    _: JNIEnv,
    _: JClass,
    context: jlong,
    x: jfloat,
    y: jfloat,
    radius_x: jfloat,
    radius_y: jfloat,
    rotation: jfloat,
    start_angle: jfloat,
    end_angle: jfloat,
    anticlockwise: jboolean,
) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        context.ellipse(
            x,
            y,
            radius_x,
            radius_y,
            rotation,
            start_angle,
            end_angle,
            anticlockwise == JNI_TRUE,
        )
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSCanvasRenderingContext2D_nativeFill(
    _: JNIEnv,
    _: JClass,
    context: jlong,
    path: jlong,
    rule: jint,
) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        if path == 0 {
            context.fill(None, FillRule::from(rule))
        } else {
            let path: *mut Path = path as _;
            context.fill(Some(&mut *path), FillRule::from(rule))
        }
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSCanvasRenderingContext2D_nativeFillRect(
    _: JNIEnv,
    _: JClass,
    context: jlong,
    x: jfloat,
    y: jfloat,
    width: jfloat,
    height: jfloat,
) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        let rect = Rect::from_xywh(x, y, width, height);
        context.fill_rect(&rect);
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSCanvasRenderingContext2D_nativeFillText(
    env: JNIEnv,
    _: JClass,
    context: jlong,
    text: JString,
    x: jfloat,
    y: jfloat,
    width: jfloat,
) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        if let Ok(txt) = env.get_string(text) {
            let txt = txt.to_string_lossy();
            context.fill_text(txt.as_ref(), x, y, width)
        }
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSCanvasRenderingContext2D_nativeGetImageData(
    _: JNIEnv,
    _: JClass,
    context: jlong,
    sx: jfloat,
    sy: jfloat,
    sw: jfloat,
    sh: jfloat,
) -> jlong {
    unsafe {
        if context == 0 {
            return 0;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        Box::into_raw(Box::new(context.get_image_data(sx, sy, sw, sh))) as jlong
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSCanvasRenderingContext2D_nativeGetLineDash(
    env: JNIEnv,
    _: JClass,
    context: jlong,
) -> jfloatArray {
    unsafe {
        let context: *const Context = context as _;
        let context = &*context;
        let line_dash = context.line_dash();
        let array = env.new_float_array(line_dash.len() as i32).unwrap();
        env.set_float_array_region(array, 0, line_dash)
            .unwrap_or(());
        array
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSCanvasRenderingContext2D_nativeGetTansform(
    _: JNIEnv,
    _: JClass,
    context: jlong,
) -> jlong {
    unsafe {
        if context == 0 {
            return 0;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        let matrix = Matrix {
            matrix: skia_safe::M44::from(context.get_transform().clone()),
        };
        Box::into_raw(Box::new(matrix)) as jlong
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSCanvasRenderingContext2D_nativeIsPointInPath(
    _: JNIEnv,
    _: JClass,
    context: jlong,
    path: jlong,
    x: jfloat,
    y: jfloat,
    rule: jint,
) -> bool {
    unsafe {
        if context == 0 {
            return false;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        return if path == 0 {
            context.is_point_in_path(None, x, y, FillRule::from(rule))
        } else {
            let path: *mut Path = path as _;
            context.is_point_in_path(Some(&*path), x, y, FillRule::from(rule))
        };
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSCanvasRenderingContext2D_nativeIsPointInStroke(
    _: JNIEnv,
    _: JClass,
    context: jlong,
    path: jlong,
    x: jfloat,
    y: jfloat,
) -> bool {
    unsafe {
        if context == 0 {
            return false;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        if path == 0 {
            context.is_point_in_stroke(None, x, y)
        } else {
            let path: *mut Path = path as _;
            context.is_point_in_stroke(Some(&*path), x, y)
        }
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSCanvasRenderingContext2D_nativeLineTo(
    _: JNIEnv,
    _: JClass,
    context: jlong,
    x: jfloat,
    y: jfloat,
) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        context.line_to(x, y)
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSCanvasRenderingContext2D_nativeMeasureText(
    env: JNIEnv,
    _: JClass,
    context: jlong,
    text: JString,
) -> jlong {
    unsafe {
        let context: *const Context = context as _;
        let context = &*context;
        let text = env.get_string(text).unwrap();
        let text = text.to_string_lossy();
        Box::into_raw(Box::new(context.measure_text(text.as_ref()))) as jlong
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSCanvasRenderingContext2D_nativeMoveTo(
    _: JNIEnv,
    _: JClass,
    context: jlong,
    x: jfloat,
    y: jfloat,
) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        context.move_to(x, y)
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSCanvasRenderingContext2D_nativePutImageData(
    _: JNIEnv,
    _: JClass,
    context: jlong,
    image_data: jlong,
    dx: jfloat,
    dy: jfloat,
    dirty_x: jfloat,
    dirty_y: jfloat,
    dirty_width: jfloat,
    dirty_height: jfloat,
) {
    unsafe {
        if context == 0 || image_data == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        let image_data: *const ImageData = image_data as _;
        context.put_image_data(
            &*image_data,
            dx,
            dy,
            dirty_x,
            dirty_y,
            dirty_width,
            dirty_height,
        )
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSCanvasRenderingContext2D_nativeQuadraticCurveTo(
    _: JNIEnv,
    _: JClass,
    context: jlong,
    cpx: jfloat,
    cpy: jfloat,
    x: jfloat,
    y: jfloat,
) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        context.quadratic_curve_to(cpx, cpy, x, y)
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSCanvasRenderingContext2D_nativeRect(
    _: JNIEnv,
    _: JClass,
    context: jlong,
    x: jfloat,
    y: jfloat,
    width: jfloat,
    height: jfloat,
) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        context.rect(x, y, width, height)
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSCanvasRenderingContext2D_nativeResetTransform(
    _: JNIEnv,
    _: JClass,
    context: jlong,
) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        context.reset_transform()
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSCanvasRenderingContext2D_nativeRestore(
    _: JNIEnv,
    _: JClass,
    context: jlong,
) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        context.restore()
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSCanvasRenderingContext2D_nativeRotate(
    _: JNIEnv,
    _: JClass,
    context: jlong,
    angle: jfloat,
) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        context.rotate(angle)
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSCanvasRenderingContext2D_nativeSave(
    _: JNIEnv,
    _: JClass,
    context: jlong,
) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        context.save()
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSCanvasRenderingContext2D_nativeScale(
    _: JNIEnv,
    _: JClass,
    context: jlong,
    x: jfloat,
    y: jfloat,
) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        context.scale(x, y)
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSCanvasRenderingContext2D_nativeSetLineDash(
    env: JNIEnv,
    _: JClass,
    context: jlong,
    dash: jfloatArray,
) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        if let Ok(length) = env.get_array_length(dash) {
            let mut buf = vec![0f32; length as usize];
            if let Ok(_) = env.get_float_array_region(dash, 0, buf.as_mut_slice()) {
                context.set_line_dash(buf.as_slice())
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSCanvasRenderingContext2D_nativeSetTransform(
    _: JNIEnv,
    _: JClass,
    context: jlong,
    a: jfloat,
    b: jfloat,
    c: jfloat,
    d: jfloat,
    e: jfloat,
    f: jfloat,
) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        context.set_transform(a, b, c, d, e, f)
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSCanvasRenderingContext2D_nativeSetTransformMatrix(
    _: JNIEnv,
    _: JClass,
    context: jlong,
    matrix: jlong,
) {
    unsafe {
        if context == 0 || matrix == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        let matrix: *const Matrix = matrix as _;
        let matrix = &*matrix;
        context.set_transform_matrix(&matrix.matrix.to_m33())
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSCanvasRenderingContext2D_nativeStroke(
    _: JNIEnv,
    _: JClass,
    context: jlong,
    path: jlong,
) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        if path == 0 {
            context.stroke(None)
        } else {
            let path: *mut Path = path as _;
            context.stroke(Some(&mut *path))
        }
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSCanvasRenderingContext2D_nativeStrokeRect(
    _: JNIEnv,
    _: JClass,
    context: jlong,
    x: jfloat,
    y: jfloat,
    width: jfloat,
    height: jfloat,
) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        let rect = Rect::from_xywh(x, y, width, height);
        context.stroke_rect(&rect);
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSCanvasRenderingContext2D_nativeStrokeText(
    env: JNIEnv,
    _: JClass,
    context: jlong,
    text: JString,
    x: jfloat,
    y: jfloat,
    width: jfloat,
) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        let txt = env.get_string(text).unwrap();
        let txt = txt.to_string_lossy();
        context.stroke_text(txt.as_ref(), x, y, width)
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSCanvasRenderingContext2D_nativeTransform(
    _: JNIEnv,
    _: JClass,
    context: jlong,
    a: jfloat,
    b: jfloat,
    c: jfloat,
    d: jfloat,
    e: jfloat,
    f: jfloat,
) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        context.transform(a, b, c, d, e, f)
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSCanvasRenderingContext2D_nativeTranslate(
    _: JNIEnv,
    _: JClass,
    context: jlong,
    x: jfloat,
    y: jfloat,
) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        context.translate(x, y)
    }
}
