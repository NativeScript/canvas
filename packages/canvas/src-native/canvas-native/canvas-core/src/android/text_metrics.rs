use jni::objects::JClass;
use jni::sys::{jfloat, jlong};
use jni::JNIEnv;

use crate::common::context::drawing_text::text_metrics::TextMetrics;

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSTextMetrics_nativeGetWidth(
    _: JNIEnv,
    _: JClass,
    metrics: jlong,
) -> jfloat {
    if metrics == 0 {
        return 0.0;
    }
    unsafe {
        let metrics: *const TextMetrics = metrics as _;
        let metrics = &*metrics;
        metrics.width
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSTextMetrics_nativeGetActualBoundingBoxLeft(
    _: JNIEnv,
    _: JClass,
    metrics: jlong,
) -> jfloat {
    if metrics == 0 {
        return 0.0;
    }
    unsafe {
        let metrics: *const TextMetrics = metrics as _;
        let metrics = &*metrics;
        metrics.actual_bounding_box_left
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSTextMetrics_nativeGetActualBoundingBoxRight(
    _: JNIEnv,
    _: JClass,
    metrics: jlong,
) -> jfloat {
    if metrics == 0 {
        return 0.0;
    }
    unsafe {
        let metrics: *const TextMetrics = metrics as _;
        let metrics = &*metrics;
        metrics.actual_bounding_box_right
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSTextMetrics_nativeGetActualBoundingBoxAscent(
    _: JNIEnv,
    _: JClass,
    metrics: jlong,
) -> jfloat {
    if metrics == 0 {
        return 0.0;
    }
    unsafe {
        let metrics: *const TextMetrics = metrics as _;
        let metrics = &*metrics;
        metrics.actual_bounding_box_ascent
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSTextMetrics_nativeGetActualBoundingBoxDescent(
    _: JNIEnv,
    _: JClass,
    metrics: jlong,
) -> jfloat {
    if metrics == 0 {
        return 0.0;
    }
    unsafe {
        let metrics: *const TextMetrics = metrics as _;
        let metrics = &*metrics;
        metrics.actual_bounding_box_descent
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSTextMetrics_nativeGetFontBoundingBoxAscent(
    _: JNIEnv,
    _: JClass,
    metrics: jlong,
) -> jfloat {
    if metrics == 0 {
        return 0.0;
    }
    unsafe {
        let metrics: *const TextMetrics = metrics as _;
        let metrics = &*metrics;
        metrics.font_bounding_box_ascent
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSTextMetrics_nativeGetFontBoundingBoxDescent(
    _: JNIEnv,
    _: JClass,
    metrics: jlong,
) -> jfloat {
    if metrics == 0 {
        return 0.0;
    }
    unsafe {
        let metrics: *const TextMetrics = metrics as _;
        let metrics = &*metrics;
        metrics.font_bounding_box_descent
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSTextMetrics_nativeGetEmHeightAscent(
    _: JNIEnv,
    _: JClass,
    metrics: jlong,
) -> jfloat {
    if metrics == 0 {
        return 0.0;
    }
    unsafe {
        let metrics: *const TextMetrics = metrics as _;
        let metrics = &*metrics;
        metrics.em_height_ascent
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSTextMetrics_nativeGetEmHeightDescent(
    _: JNIEnv,
    _: JClass,
    metrics: jlong,
) -> jfloat {
    if metrics == 0 {
        return 0.0;
    }
    unsafe {
        let metrics: *const TextMetrics = metrics as _;
        let metrics = &*metrics;
        metrics.em_height_descent
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSTextMetrics_nativeGetHangingBaseline(
    _: JNIEnv,
    _: JClass,
    metrics: jlong,
) -> jfloat {
    if metrics == 0 {
        return 0.0;
    }
    unsafe {
        let metrics: *const TextMetrics = metrics as _;
        let metrics = &*metrics;
        metrics.hanging_baseline
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSTextMetrics_nativeGetAlphabeticBaseline(
    _: JNIEnv,
    _: JClass,
    metrics: jlong,
) -> jfloat {
    if metrics == 0 {
        return 0.0;
    }
    unsafe {
        let metrics: *const TextMetrics = metrics as _;
        let metrics = &*metrics;
        metrics.alphabetic_baseline
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSTextMetrics_nativeGetIdeographicBaseline(
    _: JNIEnv,
    _: JClass,
    metrics: jlong,
) -> jfloat {
    if metrics == 0 {
        return 0.0;
    }
    unsafe {
        let metrics: *const TextMetrics = metrics as _;
        let metrics = &*metrics;
        metrics.ideographic_baseline
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSTextMetrics_nativeDestroy(
    _: JNIEnv,
    _: JClass,
    metrics: jlong,
) {
    if metrics == 0 {
        return;
    }
    unsafe {
        let metrics: *mut TextMetrics = metrics as _;
        let _ = Box::from_raw(metrics);
    }
}
