use std::ops::Range;

use skia_safe::Rect;

pub mod color;
pub(crate) mod dimensions;
pub(crate) mod geometry;
pub mod gl;
pub mod image;

//  https://github.com/samizdatco/skia-canvas/blob/35ac526e8b428579f84b7d9557f032cbf5e04883/src/utils.rs#L146
/// Convert from byte-indices to char-indices for a given UTF-8 string
pub fn string_idx_range(text: &str, start_idx: usize, end_idx: usize) -> Range<usize> {
    let mut indices = text.char_indices();
    let obtain_index = |(index, _char)| index;
    let str_len = text.len();

    Range {
        start: indices.nth(start_idx).map_or(str_len, &obtain_index),
        end: indices
            .nth((end_idx - start_idx).max(1) - 1)
            .map_or(str_len, &obtain_index),
    }
}

pub fn fit_bounds(width: f32, height: f32, src: Rect, dst: Rect) -> (Rect, Rect) {
    let mut src = src;
    let mut dst = dst;
    let scale_x = dst.width() / src.width();
    let scale_y = dst.height() / src.height();

    if src.left < 0.0 {
        dst.left += -src.left * scale_x;
        src.left = 0.0;
    }

    if src.top < 0.0 {
        dst.top += -src.top * scale_y;
        src.top = 0.0;
    }

    if src.right > width {
        dst.right -= (src.right - width) * scale_x;
        src.right = width;
    }

    if src.bottom > height {
        dst.bottom -= (src.bottom - height) * scale_y;
        src.bottom = height;
    }

    (src, dst)
}
