use crate::canvas2d::matrix::Matrix;
use crate::canvas2d::paint::PaintStyle;

#[no_mangle]
pub extern "C" fn canvas_native_pattern_set_transform(
    pattern: *mut PaintStyle,
    matrix: *const Matrix,
) {
    if pattern.is_null() || matrix.is_null() {
        return;
    }
    let pattern = unsafe { &mut *pattern };
    let mut pattern = &mut pattern.0;
    let matrix = unsafe { &*matrix };
    match pattern {
        canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Pattern(pattern) => {
            pattern.set_pattern_transform(&matrix.0)
        }
        _ => {}
    }
}
