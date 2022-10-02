use skia_safe::{FilterMode, MipmapMode, SamplingOptions};

#[derive(Copy, Clone)]
pub enum FilterQuality {
    None,
    Low,
    Medium,
    High,
}

impl Into<SamplingOptions> for FilterQuality {
    fn into(self) -> SamplingOptions {
        match self {
            FilterQuality::None => SamplingOptions::new(FilterMode::Nearest, MipmapMode::None),
            FilterQuality::Low => SamplingOptions::new(FilterMode::Linear, MipmapMode::Nearest),
            FilterQuality::Medium => SamplingOptions::new(FilterMode::Linear, MipmapMode::Linear),
            FilterQuality::High => SamplingOptions::new(FilterMode::Linear, MipmapMode::Linear)
        }
    }
}
