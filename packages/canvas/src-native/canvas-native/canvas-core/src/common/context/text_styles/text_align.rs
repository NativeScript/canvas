#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum TextAlign {
    START = 0,
    LEFT = 1,
    CENTER = 2,
    RIGHT = 3,
    END = 4,
}
impl Default for TextAlign {
    fn default() -> Self {
        Self::START
    }
}
