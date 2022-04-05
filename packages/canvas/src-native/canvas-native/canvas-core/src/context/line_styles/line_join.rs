use skia_safe::paint::Join;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum LineJoin {
    JoinRound = 0,
    JoinBevel = 1,
    JoinMiter = 2,
}

impl Into<Join> for LineJoin {
    fn into(self) -> Join {
        match self {
            LineJoin::JoinRound => Join::Round,
            LineJoin::JoinBevel => Join::Bevel,
            LineJoin::JoinMiter => Join::Miter,
        }
    }
}

impl Into<i32> for LineJoin {
    fn into(self) -> i32 {
        match self {
            LineJoin::JoinRound => 0,
            LineJoin::JoinBevel => 1,
            LineJoin::JoinMiter => 2,
        }
    }
}

impl From<i32> for LineJoin {
    fn from(value: i32) -> LineJoin {
        match value {
            0 => LineJoin::JoinRound,
            1 => LineJoin::JoinBevel,
            2 => LineJoin::JoinMiter,
            _ => LineJoin::JoinMiter,
        }
    }
}

impl TryFrom<&str> for LineJoin {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "round" => Ok(LineJoin::JoinRound),
            "bevel" => Ok(LineJoin::JoinBevel),
            "miter" => Ok(LineJoin::JoinMiter),
            _ => Err("Invalid LineJoin"),
        }
    }
}

impl Default for LineJoin {
    fn default() -> Self {
        Self::JoinMiter
    }
}
