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

impl Into<i32> for TextAlign {
    fn into(self) -> i32 {
        match self {
            TextAlign::START => 0,
            TextAlign::LEFT => 1,
            TextAlign::CENTER => 2,
            TextAlign::RIGHT => 3,
            TextAlign::END => 4
        }
    }
}


impl From<i32> for TextAlign {
    fn from(value: i32) -> TextAlign {
        match value {
            0 => TextAlign::START,
            1 => TextAlign::LEFT,
            2 => TextAlign::CENTER,
            3 => TextAlign::RIGHT,
            4 => TextAlign::END,
            _ => {
                TextAlign::START
            }
        }
    }
}