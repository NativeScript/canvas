use napi_derive::napi;

use canvas_core::context_attributes::PowerPreference;

#[napi(object)]
#[derive(Clone, Debug, Default)]
pub struct ContextAttributes {
    pub alpha: bool,
    pub antialias: bool,
    pub depth: bool,
    pub fail_if_major_performance_caveat: bool,
    pub power_preference: String,
    pub premultiplied_alpha: bool,
    pub preserve_drawing_buffer: bool,
    pub stencil: bool,
    pub desynchronized: bool,
    pub xr_compatible: bool,
}


impl ContextAttributes {
    pub fn from_c(value: *mut canvas_c::ContextAttributes) -> Self {
        let attributes = unsafe { &*value };
        Self {
            alpha: attributes.get_alpha(),
            antialias: attributes.get_antialias(),
            depth: attributes.get_depth(),
            fail_if_major_performance_caveat: attributes.get_fail_if_major_performance_caveat(),
            power_preference: match attributes.get_power_preference() {
                PowerPreference::Default => "default",
                PowerPreference::HighPerformance => "high-performance",
                PowerPreference::LowPower => "low-power"
            }.to_string(),
            premultiplied_alpha: attributes.get_premultiplied_alpha(),
            preserve_drawing_buffer: attributes.get_preserve_drawing_buffer(),
            stencil: attributes.get_stencil(),
            desynchronized: attributes.get_desynchronized(),
            xr_compatible: attributes.get_xr_compatible(),
        }
    }
}