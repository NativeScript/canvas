#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum TextBaseLine {
    TOP = 0,
    HANGING = 1,
    MIDDLE = 2,
    ALPHABETIC = 3,
    IDEOGRAPHIC = 4,
    BOTTOM = 5,
}

impl Default for TextBaseLine {
    fn default() -> Self {
        TextBaseLine::ALPHABETIC
    }
}
