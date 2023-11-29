use std::ops::Range;

pub mod color;
pub(crate) mod device;
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
