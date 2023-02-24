use skia_safe::SamplingOptions;

use crate::context::Context;
use crate::context::filter_quality::FilterQuality;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum ImageSmoothingQuality {
    Low = 0,
    Medium = 1,
    High = 2,
}

impl Default for ImageSmoothingQuality {
    fn default() -> Self {
        Self::Low
    }
}

impl From<u32> for ImageSmoothingQuality {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::Low,
            1 => Self::Medium,
            2 => Self::High,
            _ => Self::Low,
        }
    }
}

impl Into<FilterQuality> for ImageSmoothingQuality {
    fn into(self) -> FilterQuality {
        match self {
            ImageSmoothingQuality::Low => FilterQuality::Low,
            ImageSmoothingQuality::Medium => FilterQuality::Medium,
            ImageSmoothingQuality::High => FilterQuality::High,
        }
    }
}

impl Into<SamplingOptions> for ImageSmoothingQuality {
    fn into(self) -> SamplingOptions {
        match self {
            ImageSmoothingQuality::Low => FilterQuality::Low.into(),
            ImageSmoothingQuality::Medium => FilterQuality::Medium.into(),
            ImageSmoothingQuality::High => FilterQuality::High.into(),
        }
    }
}

impl Into<u32> for ImageSmoothingQuality {
    fn into(self) -> u32 {
        match self {
            ImageSmoothingQuality::Low => 0,
            ImageSmoothingQuality::Medium => 1,
            ImageSmoothingQuality::High => 2,
        }
    }
}

impl TryFrom<&str> for ImageSmoothingQuality {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "low" => Ok(ImageSmoothingQuality::Low),
            "medium" => Ok(ImageSmoothingQuality::Medium),
            "high" => Ok(ImageSmoothingQuality::High),
            _ => Err("Invalid ImageSmoothingQuality"),
        }
    }
}

impl Context {
    pub fn set_image_smoothing_enabled(&mut self, value: bool) {
        self.state.image_smoothing_enabled = value;
    }

    pub fn get_image_smoothing_enabled(&self) -> bool {
        self.state.image_smoothing_enabled
    }

    pub fn get_image_smoothing_quality(&self) -> ImageSmoothingQuality {
        self.state.image_smoothing_quality
    }

    pub fn set_image_smoothing_quality(&mut self, value: ImageSmoothingQuality) {
        self.state.image_smoothing_quality = value;
        self.state
            .paint
            .image_smoothing_quality_set(self.state.image_filter_quality());
    }
}
