use std::os::raw::{c_float, c_longlong};

use jni::objects::{JClass, JString};
use jni::JNIEnv;

use crate::common::context::fill_and_stroke_styles::paint::PaintStyle;

#[no_mangle]
pub extern "system" fn Java_org_nativescript_canvas_TNSCanvasGradient_nativeAddColorStop(
    env: JNIEnv,
    _: JClass,
    style: c_longlong,
    stop: c_float,
    color: JString,
) {
    if style == 0 {
        return;
    }
    unsafe {
        let style: *mut PaintStyle = style as _;
        let style = &mut *style;
        match style {
            PaintStyle::Gradient(gradient) => {
                if let Ok(color) = env.get_string(color) {
                    let color = color.to_string_lossy();
                    if let Ok(color) = color.as_ref().parse::<css_color_parser::Color>() {
                        gradient.add_color_stop(
                            stop,
                            skia_safe::Color::from_argb(
                                (color.a * 255.0) as u8,
                                color.r,
                                color.g,
                                color.b,
                            ),
                        )
                    }
                }
            }
            _ => {}
        }
    }
}
