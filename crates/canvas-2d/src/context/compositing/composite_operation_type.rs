use skia_safe::BlendMode;

#[derive(Clone, Copy, Debug)]
pub enum CompositeOperationType {
    SourceOver = 0,
    SourceIn = 1,
    SourceOut = 2,
    SourceAtop = 3,
    DestinationOver = 4,
    DestinationIn = 5,
    DestinationOut = 6,
    DestinationAtop = 7,
    Lighter = 8,
    Copy = 9,
    Xor = 10,
    Multiply = 11,
    Screen = 12,
    Overlay = 13,
    Darken = 14,
    Lighten = 15,
    ColorDodge = 16,
    ColorBurn = 17,
    HardLight = 18,
    SoftLight = 19,
    Difference = 20,
    Exclusion = 21,
    Hue = 22,
    Saturation = 23,
    Color = 24,
    Luminosity = 25,
}

impl Default for CompositeOperationType {
    fn default() -> Self {
        Self::SourceOver
    }
}

impl Into<u32> for CompositeOperationType {
    fn into(self) -> u32 {
        match self {
            CompositeOperationType::SourceOver => 0,
            CompositeOperationType::SourceIn => 1,
            CompositeOperationType::SourceOut => 2,
            CompositeOperationType::SourceAtop => 3,
            CompositeOperationType::DestinationOver => 4,
            CompositeOperationType::DestinationIn => 5,
            CompositeOperationType::DestinationOut => 6,
            CompositeOperationType::DestinationAtop => 7,
            CompositeOperationType::Lighter => 8,
            CompositeOperationType::Copy => 9,
            CompositeOperationType::Xor => 10,
            CompositeOperationType::Multiply => 11,
            CompositeOperationType::Screen => 12,
            CompositeOperationType::Overlay => 13,
            CompositeOperationType::Darken => 14,
            CompositeOperationType::Lighten => 15,
            CompositeOperationType::ColorDodge => 16,
            CompositeOperationType::ColorBurn => 17,
            CompositeOperationType::HardLight => 18,
            CompositeOperationType::SoftLight => 19,
            CompositeOperationType::Difference => 20,
            CompositeOperationType::Exclusion => 21,
            CompositeOperationType::Hue => 22,
            CompositeOperationType::Saturation => 23,
            CompositeOperationType::Color => 24,
            CompositeOperationType::Luminosity => 25,
        }
    }
}

impl TryFrom<u32> for CompositeOperationType {
    type Error = &'static str;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(CompositeOperationType::SourceOver),
            1 => Ok(CompositeOperationType::SourceIn),
            2 => Ok(CompositeOperationType::SourceOut),
            3 => Ok(CompositeOperationType::SourceAtop),
            4 => Ok(CompositeOperationType::DestinationOver),
            5 => Ok(CompositeOperationType::DestinationIn),
            6 => Ok(CompositeOperationType::DestinationOut),
            7 => Ok(CompositeOperationType::DestinationAtop),
            8 => Ok(CompositeOperationType::Lighter),
            9 => Ok(CompositeOperationType::Copy),
            10 => Ok(CompositeOperationType::Xor),
            11 => Ok(CompositeOperationType::Multiply),
            12 => Ok(CompositeOperationType::Screen),
            13 => Ok(CompositeOperationType::Overlay),
            14 => Ok(CompositeOperationType::Darken),
            15 => Ok(CompositeOperationType::Lighten),
            16 => Ok(CompositeOperationType::ColorDodge),
            17 => Ok(CompositeOperationType::ColorBurn),
            18 => Ok(CompositeOperationType::HardLight),
            19 => Ok(CompositeOperationType::SoftLight),
            20 => Ok(CompositeOperationType::Difference),
            21 => Ok(CompositeOperationType::Exclusion),
            22 => Ok(CompositeOperationType::Hue),
            23 => Ok(CompositeOperationType::Saturation),
            24 => Ok(CompositeOperationType::Color),
            25 => Ok(CompositeOperationType::Luminosity),
            _ => Err("Invalid CompositeOperationType"),
        }
    }
}

impl CompositeOperationType {
    pub fn to_str(self) -> &'static str {
        match self {
            CompositeOperationType::SourceOver => "source-over",
            CompositeOperationType::SourceIn => "source-in",
            CompositeOperationType::SourceOut => "source-out",
            CompositeOperationType::SourceAtop => "source-atop",
            CompositeOperationType::DestinationOver => "destination-over",
            CompositeOperationType::DestinationIn => "destination-in",
            CompositeOperationType::DestinationOut => "destination-out",
            CompositeOperationType::DestinationAtop => "destination-atop",
            CompositeOperationType::Lighter => "lighter",
            CompositeOperationType::Copy => "copy",
            CompositeOperationType::Xor => "xor",
            CompositeOperationType::Multiply => "multiply",
            CompositeOperationType::Screen => "screen",
            CompositeOperationType::Overlay => "overlay",
            CompositeOperationType::Darken => "darken",
            CompositeOperationType::Lighten => "lighten",
            CompositeOperationType::ColorDodge => "color-dodge",
            CompositeOperationType::ColorBurn => "color-burn",
            CompositeOperationType::HardLight => "hard-light",
            CompositeOperationType::SoftLight => "soft-light",
            CompositeOperationType::Difference => "difference",
            CompositeOperationType::Exclusion => "exclusion",
            CompositeOperationType::Hue => "hue",
            CompositeOperationType::Saturation => "saturation",
            CompositeOperationType::Color => "color",
            CompositeOperationType::Luminosity => "luminosity",
        }
    }

    #[inline]
    pub fn get_blend_mode(&self) -> BlendMode {
        match self {
            CompositeOperationType::SourceIn => BlendMode::SrcIn,
            CompositeOperationType::SourceOut => BlendMode::SrcOut,
            CompositeOperationType::SourceAtop => BlendMode::SrcATop,
            CompositeOperationType::DestinationOver => BlendMode::DstOver,
            CompositeOperationType::DestinationIn => BlendMode::DstIn,
            CompositeOperationType::DestinationOut => BlendMode::DstOut,
            CompositeOperationType::DestinationAtop => BlendMode::DstATop,
            CompositeOperationType::Lighter => BlendMode::Lighten,
            CompositeOperationType::Copy => BlendMode::Src,
            CompositeOperationType::Xor => BlendMode::Xor,
            CompositeOperationType::Multiply => BlendMode::Multiply,
            CompositeOperationType::Screen => BlendMode::Screen,
            CompositeOperationType::Overlay => BlendMode::Overlay,
            CompositeOperationType::Darken => BlendMode::Darken,
            CompositeOperationType::Lighten => BlendMode::Lighten,
            CompositeOperationType::ColorDodge => BlendMode::ColorDodge,
            CompositeOperationType::ColorBurn => BlendMode::ColorBurn,
            CompositeOperationType::HardLight => BlendMode::HardLight,
            CompositeOperationType::SoftLight => BlendMode::SoftLight,
            CompositeOperationType::Difference => BlendMode::Difference,
            CompositeOperationType::Exclusion => BlendMode::Exclusion,
            CompositeOperationType::Hue => BlendMode::Hue,
            CompositeOperationType::Saturation => BlendMode::Saturation,
            CompositeOperationType::Color => BlendMode::Color,
            CompositeOperationType::Luminosity => BlendMode::Luminosity,
            CompositeOperationType::SourceOver => BlendMode::SrcOver,
        }
    }

    pub fn from_str(value: &str) -> Option<CompositeOperationType> {
        match value {
            "source-over" => Some(CompositeOperationType::SourceOver),
            "source-in" => Some(CompositeOperationType::SourceIn),
            "source-out" => Some(CompositeOperationType::SourceOut),
            "source-atop" => Some(CompositeOperationType::SourceAtop),
            "destination-over" => Some(CompositeOperationType::DestinationOver),
            "destination-in" => Some(CompositeOperationType::DestinationIn),
            "destination-out" => Some(CompositeOperationType::DestinationOut),
            "destination-atop" => Some(CompositeOperationType::DestinationAtop),
            "lighter" => Some(CompositeOperationType::Lighter),
            "copy" => Some(CompositeOperationType::Copy),
            "xor" => Some(CompositeOperationType::Xor),
            "multiply" => Some(CompositeOperationType::Multiply),
            "screen" => Some(CompositeOperationType::Screen),
            "overlay" => Some(CompositeOperationType::Overlay),
            "darken" => Some(CompositeOperationType::Darken),
            "lighten" => Some(CompositeOperationType::Lighten),
            "color-dodge" => Some(CompositeOperationType::ColorDodge),
            "color-burn" => Some(CompositeOperationType::ColorBurn),
            "hard-light" => Some(CompositeOperationType::HardLight),
            "soft-light" => Some(CompositeOperationType::SoftLight),
            "difference" => Some(CompositeOperationType::Difference),
            "exclusion" => Some(CompositeOperationType::Exclusion),
            "hue" => Some(CompositeOperationType::Hue),
            "saturation" => Some(CompositeOperationType::Saturation),
            "color" => Some(CompositeOperationType::Color),
            "luminosity" => Some(CompositeOperationType::Luminosity),
            _ => None,
        }
    }
}
