pub(crate) mod surface_texture;
pub mod texture_render;
pub mod webgl2_rendering_context;
pub mod webgl_rendering_context;


extern "C" {
    #[doc = " Returns the API level of the device we're actually running on, or -1 on failure."]
    #[doc = " The returned values correspond to the named constants in `<android/api-level.h>`,"]
    #[doc = " and is equivalent to the Java `Build.VERSION.SDK_INT` API."]
    #[doc = ""]
    #[doc = " See also android_get_application_target_sdk_version()."]
    pub fn android_get_device_api_level() -> ::std::os::raw::c_int;
}