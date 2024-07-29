use std::sync::Arc;

#[derive(Clone, Copy, Debug)]
pub struct TextMetrics(pub(crate) canvas_2d::context::drawing_text::text_metrics::TextMetrics);

#[no_mangle]
pub extern "C" fn canvas_native_text_metrics_reference(value: *const TextMetrics) {
    if value.is_null() {
        return;
    }

    unsafe { Arc::increment_strong_count(value) }
}

#[no_mangle]
pub extern "C" fn canvas_native_text_metrics_release(value: *const TextMetrics) {
    if value.is_null() {
        return;
    }

    unsafe { Arc::decrement_strong_count(value) }
}

#[no_mangle]
pub extern "C" fn canvas_native_text_metrics_get_width(metrics: *const TextMetrics) -> f32 {
    if metrics.is_null() {
        return 0.;
    }
    let metrics = unsafe { &*metrics };
    metrics.0.width()
}

#[no_mangle]
pub extern "C" fn canvas_native_text_metrics_get_actual_bounding_box_left(
    metrics: *const TextMetrics,
) -> f32 {
    if metrics.is_null() {
        return 0.;
    }
    let metrics = unsafe { &*metrics };
    metrics.0.actual_bounding_box_left()
}

#[no_mangle]
pub extern "C" fn canvas_native_text_metrics_get_actual_bounding_box_right(
    metrics: *const TextMetrics,
) -> f32 {
    if metrics.is_null() {
        return 0.;
    }
    let metrics = unsafe { &*metrics };
    metrics.0.actual_bounding_box_right()
}

#[no_mangle]
pub extern "C" fn canvas_native_text_metrics_get_actual_bounding_box_ascent(
    metrics: *const TextMetrics,
) -> f32 {
    if metrics.is_null() {
        return 0.;
    }
    let metrics = unsafe { &*metrics };
    metrics.0.actual_bounding_box_ascent()
}

#[no_mangle]
pub extern "C" fn canvas_native_text_metrics_get_actual_bounding_box_descent(
    metrics: *const TextMetrics,
) -> f32 {
    if metrics.is_null() {
        return 0.;
    }
    let metrics = unsafe { &*metrics };
    metrics.0.actual_bounding_box_descent()
}

#[no_mangle]
pub extern "C" fn canvas_native_text_metrics_get_font_bounding_box_ascent(
    metrics: *const TextMetrics,
) -> f32 {
    if metrics.is_null() {
        return 0.;
    }
    let metrics = unsafe { &*metrics };
    metrics.0.font_bounding_box_ascent()
}

#[no_mangle]
pub extern "C" fn canvas_native_text_metrics_get_font_bounding_box_descent(
    metrics: *const TextMetrics,
) -> f32 {
    if metrics.is_null() {
        return 0.;
    }
    let metrics = unsafe { &*metrics };
    metrics.0.font_bounding_box_descent()
}

#[no_mangle]
pub extern "C" fn canvas_native_text_metrics_get_em_height_ascent(
    metrics: *const TextMetrics,
) -> f32 {
    if metrics.is_null() {
        return 0.;
    }
    let metrics = unsafe { &*metrics };
    metrics.0.em_height_ascent()
}

#[no_mangle]
pub extern "C" fn canvas_native_text_metrics_get_em_height_descent(
    metrics: *const TextMetrics,
) -> f32 {
    if metrics.is_null() {
        return 0.;
    }
    let metrics = unsafe { &*metrics };
    metrics.0.em_height_descent()
}

#[no_mangle]
pub extern "C" fn canvas_native_text_metrics_get_hanging_baseline(
    metrics: *const TextMetrics,
) -> f32 {
    if metrics.is_null() {
        return 0.;
    }
    let metrics = unsafe { &*metrics };
    metrics.0.hanging_baseline()
}

#[no_mangle]
pub extern "C" fn canvas_native_text_metrics_get_alphabetic_baseline(
    metrics: *const TextMetrics,
) -> f32 {
    if metrics.is_null() {
        return 0.;
    }
    let metrics = unsafe { &*metrics };
    metrics.0.alphabetic_baseline()
}

#[no_mangle]
pub extern "C" fn canvas_native_text_metrics_get_ideographic_baseline(
    metrics: *const TextMetrics,
) -> f32 {
    if metrics.is_null() {
        return 0.;
    }
    let metrics = unsafe { &*metrics };
    metrics.0.ideographic_baseline()
}
