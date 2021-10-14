use std::os::raw::c_longlong;

use crate::common::context::fill_and_stroke_styles::paint::PaintStyle;
use crate::common::context::matrix::Matrix;

#[no_mangle]
pub extern "C" fn pattern_set_transform(pattern: c_longlong, matrix: c_longlong) {
    if pattern == 0 || matrix == 0 {
        return;
    }
    unsafe {
        let pattern: *mut PaintStyle = pattern as _;
        let pattern = &mut *pattern;
        let matrix: *mut Matrix = matrix as _;
        let matrix = &mut *matrix;
        match pattern {
            PaintStyle::Pattern(pattern) => pattern.set_pattern_transform(&matrix),
            _ => {}
        }
    }
}
