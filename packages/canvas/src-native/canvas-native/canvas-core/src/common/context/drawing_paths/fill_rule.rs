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

impl FillRule {
    pub fn to_fill_type(&self) -> FillType {
        match self {
            FillRule::EvenOdd => FillType::EvenOdd,
            FillRule::NonZero => FillType::Winding,
        }
    }
}
