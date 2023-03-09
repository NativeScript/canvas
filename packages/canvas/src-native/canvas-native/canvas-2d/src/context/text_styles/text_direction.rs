#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TextDirection {
    LTR = 0,
    RTL = 1,
}

impl From<u32> for TextDirection {
    fn from(value: u32) -> Self {
        match value {
            1 => Self::RTL,
            0 => Self::LTR,
            _ => Self::LTR,
        }
    }
}

impl Into<u32> for TextDirection {
    fn into(self) -> u32 {
        match self {
            Self::RTL => 1,
            Self::LTR => 0,
        }
    }
}

impl TryFrom<&str> for TextDirection {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "ltr" => Ok(TextDirection::LTR),
            "rtl" => Ok(TextDirection::RTL),
            _ => Err("Invalid TextDirection"),
        }
    }
}
