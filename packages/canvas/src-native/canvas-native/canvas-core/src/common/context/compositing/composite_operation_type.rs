use skia_safe::BlendMode;

#[repr(C)]
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

impl Into<i32> for CompositeOperationType {
    fn into(self) -> i32 {
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

impl From<i32> for CompositeOperationType {
    fn from(value: i32) -> Self {
        match value {
            0 => CompositeOperationType::SourceOver,
            1 => CompositeOperationType::SourceIn,
            2 => CompositeOperationType::SourceOut,
            3 => CompositeOperationType::SourceAtop,
            4 => CompositeOperationType::DestinationOver,
            5 => CompositeOperationType::DestinationIn,
            6 => CompositeOperationType::DestinationOut,
            7 => CompositeOperationType::DestinationAtop,
            8 => CompositeOperationType::Lighter,
            9 => CompositeOperationType::Copy,
            10 => CompositeOperationType::Xor,
            11 => CompositeOperationType::Multiply,
            12 => CompositeOperationType::Screen,
            13 => CompositeOperationType::Overlay,
            14 => CompositeOperationType::Darken,
            15 => CompositeOperationType::Lighten,
            16 => CompositeOperationType::ColorDodge,
            17 => CompositeOperationType::ColorBurn,
            18 => CompositeOperationType::HardLight,
            19 => CompositeOperationType::SoftLight,
            20 => CompositeOperationType::Difference,
            21 => CompositeOperationType::Exclusion,
            22 => CompositeOperationType::Hue,
            23 => CompositeOperationType::Saturation,
            24 => CompositeOperationType::Color,
            25 => CompositeOperationType::Luminosity,
            _ => CompositeOperationType::SourceOver,
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
}
