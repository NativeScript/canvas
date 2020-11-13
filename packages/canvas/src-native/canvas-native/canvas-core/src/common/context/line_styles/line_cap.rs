use skia_safe::paint::Cap;

#[repr(C)]
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
