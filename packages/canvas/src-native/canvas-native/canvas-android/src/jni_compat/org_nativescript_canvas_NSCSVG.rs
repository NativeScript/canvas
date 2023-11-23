use canvas_2d::context::Context;
use jni::objects::{JClass, JString};
use jni::sys::jlong;
use jni::JNIEnv;

#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn nativeDrawSVG(mut env: JNIEnv, _: JClass, context: jlong, svg: JString) {
    unsafe {
        let context: *mut Context = context as _;
        let context = &mut *context;
        if let Ok(svg) = env.get_string(&svg) {
            let svg = svg.to_string_lossy();
            canvas_2d::svg::draw_svg(context, svg.as_ref());
        }
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
    unsafe {
        let context: *mut Context = context as _;
        let context = &mut *context;
        if let Ok(path) = env.get_string(&path) {
            let path = path.to_string_lossy();
            canvas_2d::svg::draw_svg_from_path(context, path.as_ref());
        }
    }
}
