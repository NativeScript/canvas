use skia_safe::path::FillType;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum FillRule {
    NonZero = 0,
    EvenOdd = 1,
}

impl Default for FillRule {
    fn default() -> Self {
        Self::NonZero
    }
}

impl Into<i32> for FillRule {
    fn into(self) -> i32 {
        match self {
            FillRule::NonZero => 0,
            FillRule::EvenOdd => 1,
        }
    }
}

impl From<&str> for FillRule {
    fn from(value: &str) -> FillRule {
        match value {
            "nonzero" => FillRule::NonZero,
            "evenodd" => FillRule::EvenOdd,
            _ => FillRule::NonZero,
        }
    }
}

impl From<&String> for FillRule {
    fn from(value: &String) -> FillRule {
        Self::from(value.as_str())
    }
}

impl From<i32> for FillRule {
    fn from(value: i32) -> FillRule {
        match value {
            0 => FillRule::NonZero,
            1 => FillRule::EvenOdd,
            _ => FillRule::NonZero,
        }
    }
}

impl FillRule {
    pub fn from_i32(value: i32) -> Option<Self> {
        match value {
            0 => Some(Self::NonZero),
            1 => Some(Self::EvenOdd),
            _ => None,
        }
    }
    pub fn to_fill_type(&self) -> FillType {
        match self {
            FillRule::EvenOdd => FillType::EvenOdd,
            FillRule::NonZero => FillType::Winding,
        }
    }
}
