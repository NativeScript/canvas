use std::str::FromStr;

use css_color_parser::{Color, ColorParseError};
use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::{jlong, jstring};
use log::{debug, info};

use crate::common::context::Context;
use crate::common::context::fill_and_stroke_styles::paint::PaintStyle;
use crate::common::utils::color::to_parsed_color;

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
        let context: *mut Context = context as _;
        let context = &mut *context;
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
pub extern "C" fn Java_org_nativescript_canvas_TNSCanvasRenderingContext2D_nativeSetFillColorWithString(
    env: JNIEnv,
    _: JClass,
    context: jlong,
    color: JString,
) {
    paint_style_set_color_with_string(env, context, true, color);
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSCanvasRenderingContext2D_nativeSetStrokeColorWithString(
    env: JNIEnv,
    _: JClass,
    context: jlong,
    color: JString,
) {
    paint_style_set_color_with_string(env, context, false, color);
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSColorStyle_nativeDestroy(
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
pub extern "C" fn Java_org_nativescript_canvas_TNSColor_nativeGetColorString(
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
