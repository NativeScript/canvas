use std::os::raw::c_float;

use canvas_core::context::drawing_text::text_metrics::TextMetrics;

#[no_mangle]
pub extern "C" fn canvas_native_text_metrics_get_width(metrics: *mut TextMetrics) -> c_float {
    assert!(!metrics.is_null());
    unsafe {
        let metrics = &*metrics;
        metrics.width()
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_text_metrics_get_actual_bounding_box_left(
    metrics: *mut TextMetrics,
) -> c_float {
    assert!(!metrics.is_null());
    unsafe {
        let metrics = &*metrics;
        metrics.actual_bounding_box_left()
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_text_metrics_get_actual_bounding_box_right(
    metrics: *mut TextMetrics,
) -> c_float {
    assert!(!metrics.is_null());
    unsafe {
        let metrics = &*metrics;
        metrics.actual_bounding_box_right()
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_text_metrics_get_actual_bounding_box_ascent(
    metrics: *mut TextMetrics,
) -> c_float {
    assert!(!metrics.is_null());
    unsafe {
        let metrics = &*metrics;
        metrics.actual_bounding_box_ascent()
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_text_metrics_get_actual_bounding_box_descent(
    metrics: *mut TextMetrics,
) -> c_float {
    assert!(!metrics.is_null());
    unsafe {
        let metrics = &*metrics;
        metrics.actual_bounding_box_descent()
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_text_metrics_get_font_bounding_box_ascent(
    metrics: *mut TextMetrics,
) -> c_float {
    assert!(!metrics.is_null());
    unsafe {
        let metrics = &*metrics;
        metrics.font_bounding_box_ascent()
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_text_metrics_get_font_bounding_box_descent(
    metrics: *mut TextMetrics,
) -> c_float {
    assert!(!metrics.is_null());
    unsafe {
        let metrics = &*metrics;
        metrics.font_bounding_box_descent()
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_text_metrics_get_em_height_ascent(
    metrics: *mut TextMetrics,
) -> c_float {
    assert!(!metrics.is_null());
    unsafe {
        let metrics = &*metrics;
        metrics.em_height_ascent()
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_text_metrics_get_em_height_descent(
    metrics: *mut TextMetrics,
) -> c_float {
    assert!(!metrics.is_null());
    unsafe {
        let metrics = &*metrics;
        metrics.em_height_descent()
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_text_metrics_get_hanging_baseline(
    metrics: *mut TextMetrics,
) -> c_float {
    assert!(!metrics.is_null());
    unsafe {
        let metrics = &*metrics;
        metrics.hanging_baseline()
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_text_metrics_get_alphabetic_baseline(
    metrics: *mut TextMetrics,
) -> c_float {
    assert!(!metrics.is_null());
    unsafe {
        let metrics = &*metrics;
        metrics.alphabetic_baseline()
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_text_metrics_get_ideographic_baseline(
    metrics: *mut TextMetrics,
) -> c_float {
    assert!(!metrics.is_null());
    unsafe {
        let metrics = &*metrics;
        metrics.ideographic_baseline()
    }
}
