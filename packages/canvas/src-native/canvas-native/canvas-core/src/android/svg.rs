use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::jlong;

use crate::common::context::Context;

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Java_com_github_triniwiz_canvas_TNSSVG_nativeDrawSVG(env: JNIEnv,
                                                                              _: JClass, context: jlong, svg: JString) {
   unsafe {
       let context: *mut Context = context as _;
       let context = &mut *context;
       if let Ok(svg) = env.get_string(svg) {
           let svg = svg.to_string_lossy();
           crate::common::svg::draw_svg(context, svg.as_ref());
       }
   }
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Java_com_github_triniwiz_canvas_TNSSVG_nativeDrawSVGFromPath(env: JNIEnv,
                                                                                      _: JClass, context: jlong, path: JString) {
    unsafe {
        let context: *mut Context = context as _;
        let context = &mut *context;
        if let Ok(path) = env.get_string(path) {
            let path = path.to_string_lossy();
            crate::common::svg::draw_svg_from_path(context, path.as_ref());
        }
    }
}