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
