use std::os::raw::{c_float, c_longlong};

use crate::common::context::drawing_text::text_metrics::TextMetrics;

#[no_mangle]
pub extern "C" fn text_metrics_get_width(metrics: c_longlong) -> c_float {
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
pub extern "C" fn text_metrics_get_actual_bounding_box_left(metrics: c_longlong) -> c_float {
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
pub extern "C" fn text_metrics_get_actual_bounding_box_right(metrics: c_longlong) -> c_float {
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
pub extern "C" fn text_metrics_get_actual_bounding_box_ascent(metrics: c_longlong) -> c_float {
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
pub extern "C" fn text_metrics_get_actual_bounding_box_descent(metrics: c_longlong) -> c_float {
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
pub extern "C" fn text_metrics_get_font_bounding_box_ascent(metrics: c_longlong) -> c_float {
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
pub extern "C" fn text_metrics_get_font_bounding_box_descent(metrics: c_longlong) -> c_float {
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
pub extern "C" fn text_metrics_get_em_height_ascent(metrics: c_longlong) -> c_float {
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
pub extern "C" fn text_metrics_get_em_height_descent(metrics: c_longlong) -> c_float {
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
pub extern "C" fn text_metrics_get_hanging_baseline(metrics: c_longlong) -> c_float {
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
pub extern "C" fn text_metrics_get_alphabetic_baseline(metrics: c_longlong) -> c_float {
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
pub extern "C" fn text_metrics_get_ideographic_baseline(metrics: c_longlong) -> c_float {
    if metrics == 0 {
        return 0.0;
    }
    unsafe {
        let metrics: *const TextMetrics = metrics as _;
        let metrics = &*metrics;
        metrics.ideographic_baseline
    }
}
