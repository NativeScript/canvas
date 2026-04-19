#[derive(Clone, Copy, Debug)]
pub struct TextMetrics(pub(crate) canvas_2d::context::drawing_text::text_metrics::TextMetrics);

pub const TEXT_METRICS_FIELD_COUNT: usize = 12;

///   out[ 0] = width
///   out[ 1] = actual_bounding_box_left
///   out[ 2] = actual_bounding_box_right
///   out[ 3] = actual_bounding_box_ascent   ← C++ order: actual before font
///   out[ 4] = actual_bounding_box_descent
///   out[ 5] = font_bounding_box_ascent
///   out[ 6] = font_bounding_box_descent
///   out[ 7] = em_height_ascent
///   out[ 8] = em_height_descent
///   out[ 9] = hanging_baseline
///   out[10] = alphabetic_baseline
///   out[11] = ideographic_baseline
#[no_mangle]
pub extern "C" fn canvas_native_text_metrics_get_all(
    metrics: *const TextMetrics,
    out: *mut f32,
    len: usize,
) {
    if metrics.is_null() || out.is_null() || len < TEXT_METRICS_FIELD_COUNT {
        return;
    }
    let m = unsafe { &(*metrics).0 };
    unsafe {
        *out.add(0)  = m.width();
        *out.add(1)  = m.actual_bounding_box_left();
        *out.add(2)  = m.actual_bounding_box_right();
        *out.add(3)  = m.actual_bounding_box_ascent();   // C++ order: actual before font
        *out.add(4)  = m.actual_bounding_box_descent();
        *out.add(5)  = m.font_bounding_box_ascent();
        *out.add(6)  = m.font_bounding_box_descent();
        *out.add(7)  = m.em_height_ascent();
        *out.add(8)  = m.em_height_descent();
        *out.add(9)  = m.hanging_baseline();
        *out.add(10) = m.alphabetic_baseline();
        *out.add(11) = m.ideographic_baseline();
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_text_metrics_release(value: *mut TextMetrics) {
    if value.is_null() {
        return;
    }

    let _ = unsafe { Box::from_raw(value) };
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
