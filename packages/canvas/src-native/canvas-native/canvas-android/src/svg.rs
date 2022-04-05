use jni::objects::{JClass, JString};
use jni::sys::jlong;
use jni::JNIEnv;

use canvas_core::context::{Context, ContextWrapper};

#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn Java_org_nativescript_canvas_TNSSVG_nativeDrawSVG(
    env: JNIEnv,
    _: JClass,
    context: jlong,
    svg: JString,
) {
    unsafe {
        let context: *mut ContextWrapper = context as _;
        let context = &mut *context;
        let mut context = context.get_context();
        if let Ok(svg) = env.get_string(svg) {
            let svg = svg.to_string_lossy();
            canvas_core::svg::draw_svg(&mut context, svg.as_ref());
        }
    }
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn Java_org_nativescript_canvas_TNSSVG_nativeDrawSVGFromPath(
    env: JNIEnv,
    _: JClass,
    context: jlong,
    path: JString,
) {
    unsafe {
        let context: *mut ContextWrapper = context as _;
        let context = &mut *context;
        let mut context = context.get_context();
        if let Ok(path) = env.get_string(path) {
            let path = path.to_string_lossy();
            canvas_core::svg::draw_svg_from_path(&mut context, path.as_ref());
        }
    }
}
