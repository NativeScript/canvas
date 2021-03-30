#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum SpreadMethod {
    Pad,
    Reflect,
    Repeat,
}

impl SpreadMethod {
    pub fn from_str(val: &str) -> Option<Self> {
        match val {
            "pad" => Some(Self::Pad),
            "reflect" => Some(Self::Reflect),
            "repeat" => Some(Self::Repeat),
            _ => None,
        }
    }
}

impl From<SpreadMethod> for skia_safe::TileMode {
    fn from(method: SpreadMethod) -> Self {
        match method {
            SpreadMethod::Pad => skia_safe::TileMode::Clamp,
            SpreadMethod::Reflect => skia_safe::TileMode::Mirror,
            SpreadMethod::Repeat => skia_safe::TileMode::Repeat
        }
    }
}

