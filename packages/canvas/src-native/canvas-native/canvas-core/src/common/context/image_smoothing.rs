use skia_safe::FilterQuality;

use crate::common::context::Context;

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

impl From<i32> for ImageSmoothingQuality {
    fn from(value: i32) -> Self {
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

impl Into<i32> for ImageSmoothingQuality {
    fn into(self) -> i32 {
        match self {
            ImageSmoothingQuality::Low => 0,
            ImageSmoothingQuality::Medium => 1,
            ImageSmoothingQuality::High => 2,
        }
    }
}

impl Context {
    pub fn set_image_smoothing_enabled(&mut self, value: bool) {
        self.state.image_smoothing_enabled = value;
    }

    pub fn get_image_smoothing_enabled(&mut self) -> bool {
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
