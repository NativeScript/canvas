use skia_safe::{CubicResampler, FilterMode, MipmapMode, SamplingOptions};

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

impl TryFrom<&str> for FilterQuality {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "none" => Ok(FilterQuality::None),
            "low" => Ok(FilterQuality::Low),
            "medium" => Ok(FilterQuality::Medium),
            "high" => Ok(FilterQuality::High),
            _ => Err("Invalid FilterQuality"),
        }
    }
}
