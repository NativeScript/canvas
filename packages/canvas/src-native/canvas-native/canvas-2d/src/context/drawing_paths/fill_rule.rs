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

impl TryFrom<&str> for FillRule {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "nonzero" => Ok(FillRule::NonZero),
            "evenodd" => Ok(FillRule::EvenOdd),
            _ => Err("Invalid FillRule"),
        }
    }
}


impl TryFrom<u32> for FillRule {
    type Error = &'static str;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(FillRule::NonZero),
            1 => Ok(FillRule::EvenOdd),
            _ => Err("Invalid FillRule"),
        }
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
