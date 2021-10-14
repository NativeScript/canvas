#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TextDirection {
    LTR = 0,
    RTL = 1,
}

impl From<i32> for TextDirection {
    fn from(value: i32) -> Self {
        match value {
            1 => Self::RTL,
            0 => Self::LTR,
            _ => Self::LTR,
        }
    }
}

impl Into<i32> for TextDirection {
    fn into(self) -> i32 {
        match self {
            Self::RTL => 1,
            Self::LTR => 0,
        }
    }
}
