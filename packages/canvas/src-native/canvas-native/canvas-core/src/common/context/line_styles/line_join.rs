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

impl Default for LineJoin {
    fn default() -> Self {
        Self::JoinMiter
    }
}
