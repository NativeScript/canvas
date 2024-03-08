use jni::objects::{JClass, JString};
use jni::sys::jlong;
use jni::JNIEnv;

use canvas_2d::context::Context;

#[no_mangle]
pub extern "system" fn nativeDrawSVG(mut env: JNIEnv, _: JClass, context: jlong, svg: JString) {
    let context = context as *mut canvas_c::CanvasRenderingContext2D;
    let context = unsafe { &mut *context };
    if let Ok(svg) = env.get_string(&svg) {
        let svg = svg.to_string_lossy();
        let mut context = context.get_context_mut();
        canvas_2d::svg::draw_svg(&mut context, svg.as_ref());
    }
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn nativeDrawSVGFromPath(
    mut env: JNIEnv,
    _: JClass,
    context: jlong,
    path: JString,
) {
    let context = context as *mut canvas_c::CanvasRenderingContext2D;
    let context = unsafe { &mut *context };
    if let Ok(path) = env.get_string(&path) {
        let mut context = context.get_context_mut();
        let path = path.to_string_lossy();
        canvas_2d::svg::draw_svg_from_path(&mut context, path.as_ref());
    }
}
