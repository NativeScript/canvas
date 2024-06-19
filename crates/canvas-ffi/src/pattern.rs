use std::os::raw::c_longlong;

use canvas_core::context::fill_and_stroke_styles::paint::PaintStyle;
use canvas_core::context::matrix::Matrix;

#[no_mangle]
pub extern "C" fn canvas_native_pattern_set_transform(
    pattern: *mut PaintStyle,
    matrix: *mut Matrix,
) {
    assert!(!pattern.is_null());
    assert!(!matrix.is_null());
    unsafe {
        let pattern = &mut *pattern;
        let matrix = &mut *matrix;
        match pattern {
            PaintStyle::Pattern(pattern) => pattern.set_pattern_transform(&matrix),
            _ => {}
        }
    }
}
