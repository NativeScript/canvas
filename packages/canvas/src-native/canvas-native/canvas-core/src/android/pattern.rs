use jni::JNIEnv;
use jni::objects::JClass;
use jni::sys::jlong;

use crate::common::context::fill_and_stroke_styles::paint::PaintStyle;
use crate::common::context::matrix::Matrix;

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSPattern_nativeSetTransform(_: JNIEnv,
                                                                                _: JClass, pattern: jlong, matrix: jlong) {
    if pattern == 0 || matrix == 0 {
        return;
    }
    unsafe {
        let pattern: *mut PaintStyle = pattern as _;
        let pattern = &mut *pattern;
        let matrix: *mut Matrix = matrix as _;
        let matrix = &mut *matrix;
        match pattern {
            PaintStyle::Pattern(pattern) => {
                pattern.set_pattern_transform(&matrix)
            }
            _ => {}
        }
    }
}