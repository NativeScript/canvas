#[derive(Copy, Clone, Debug)]
pub enum FilterIn {
    SourceGraphic,
    SourceAlpha,
    BackgroundImage,
    BackgroundAlpha,
    FillPaint,
    StrokePaint,
}

impl FilterIn {
    pub fn to_str(&self) -> &str {
        match self {
            FilterIn::SourceGraphic => "SourceGraphic",
            FilterIn::SourceAlpha => "SourceAlpha",
            FilterIn::BackgroundImage => "BackgroundImage",
            FilterIn::BackgroundAlpha => "BackgroundAlpha",
            FilterIn::FillPaint => "FillPaint",
            FilterIn::StrokePaint => "StrokePaint",
        }
    }

    pub fn from_str(val: &str) -> Option<Self> {
        match val {
            "SourceGraphic" => Some(Self::SourceGraphic),
            "SourceAlpha" => Some(Self::SourceAlpha),
            "BackgroundImage" => Some(Self::BackgroundImage),
            "BackgroundAlpha" => Some(Self::BackgroundAlpha),
            "FillPaint" => Some(Self::FillPaint),
            "StrokePaint" => Some(Self::StrokePaint),
            _ => None,
        }
    }
}
