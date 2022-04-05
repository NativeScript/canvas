use std::str::FromStr;

use jni::objects::{JClass, JString};
use jni::sys::{jlong, jstring};
use jni::JNIEnv;

use canvas_core::context::fill_and_stroke_styles::paint::PaintStyle;
use canvas_core::context::{Context, ContextWrapper};
use canvas_core::utils::color::to_parsed_color;

pub(crate) fn paint_style_set_color_with_string(
    env: JNIEnv,
    context: jlong,
    is_fill: bool,
    color: JString,
) {
    if context == 0 || color.is_null() {
        return;
    }
    unsafe {
        let context: *mut ContextWrapper = context as _;
        let context = &mut *context;
        let mut context = context.get_context();
        if let Ok(color) = env.get_string(color) {
            let color = color.to_string_lossy();
            if let Ok(color) = css_color_parser::Color::from_str(color.as_ref()) {
                let style = PaintStyle::Color(skia_safe::Color::from_argb(
                    (color.a * 255.0) as u8,
                    color.r,
                    color.g,
                    color.b,
                ));
                if is_fill {
                    context.set_fill_style(style);
                } else {
                    context.set_stroke_style(style);
                }
            }
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_canvas_TNSCanvasRenderingContext2D_nativeSetFillColorWithString(
    env: JNIEnv,
    _: JClass,
    context: jlong,
    color: JString,
) {
    paint_style_set_color_with_string(env, context, true, color);
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_canvas_TNSCanvasRenderingContext2D_nativeSetStrokeColorWithString(
    env: JNIEnv,
    _: JClass,
    context: jlong,
    color: JString,
) {
    paint_style_set_color_with_string(env, context, false, color);
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_canvas_TNSColorStyle_nativeDestroy(
    _: JNIEnv,
    _: JClass,
    style: jlong,
) {
    if style == 0 {
        return;
    }
    unsafe {
        let style: *mut PaintStyle = style as _;
        let _ = Box::from_raw(style);
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_canvas_TNSColor_nativeGetColorString(
    env: JNIEnv,
    _: JClass,
    color: jlong,
) -> jstring {
    if color == 0 {
        return env.new_string("").unwrap().into_inner();
    }
    unsafe {
        let color: *const PaintStyle = color as _;
        let color = &*color;
        match color {
            PaintStyle::Color(color) => {
                let string = to_parsed_color(*color);
                env.new_string(string).unwrap().into_inner()
            }
            _ => env.new_string("").unwrap().into_inner(),
        }
    }
}
