use skia_safe::paint::Cap;

#[derive(Copy, Clone, Debug)]
pub enum LineCap {
    CapButt = 0,
    CapRound = 1,
    CapSquare = 2,
}

impl Into<Cap> for LineCap {
    fn into(self) -> Cap {
        match self {
            LineCap::CapButt => Cap::Butt,
            LineCap::CapRound => Cap::Round,
            LineCap::CapSquare => Cap::Square,
        }
    }
}

impl Into<i32> for LineCap {
    fn into(self) -> i32 {
        match self {
            LineCap::CapButt => 0,
            LineCap::CapRound => 1,
            LineCap::CapSquare => 2,
        }
    }
}

impl TryFrom<&str> for LineCap {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "butt" => Ok(LineCap::CapButt),
            "round" => Ok(LineCap::CapRound),
            "square" => Ok(LineCap::CapSquare),
            _ => Err("Invalid LineCap"),
        }
    }
}

impl From<i32> for LineCap {
    fn from(value: i32) -> LineCap {
        match value {
            0 => LineCap::CapButt,
            1 => LineCap::CapRound,
            2 => LineCap::CapSquare,
            _ => LineCap::CapButt,
        }
    }
}

impl From<Cap> for LineCap {
    fn from(cap: Cap) -> Self {
        match cap {
            Cap::Butt => LineCap::CapButt,
            Cap::Round => LineCap::CapRound,
            Cap::Square => LineCap::CapSquare,
        }
    }
}

impl Default for LineCap {
    fn default() -> Self {
        Self::CapButt
    }
}
