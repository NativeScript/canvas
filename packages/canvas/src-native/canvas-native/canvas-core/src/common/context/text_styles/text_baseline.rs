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

impl From<i32> for TextBaseLine {
    fn from(value: i32) -> TextBaseLine {
        match value {
            0 => Self::TOP,
            1 => Self::HANGING,
            2 => Self::MIDDLE,
            3 => Self::ALPHABETIC,
            4 => Self::IDEOGRAPHIC,
            5 => Self::BOTTOM,
            _ => TextBaseLine::ALPHABETIC
        }
    }
}

impl Into<i32> for TextBaseLine {
    fn into(self) -> i32 {
        match self {
            TextBaseLine::TOP => 0,
            TextBaseLine::HANGING => 1,
            TextBaseLine::MIDDLE => 2,
            TextBaseLine::ALPHABETIC => 3,
            TextBaseLine::IDEOGRAPHIC => 4,
            TextBaseLine::BOTTOM => 5
        }
    }
}