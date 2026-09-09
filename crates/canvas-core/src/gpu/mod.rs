// `mtl` is what pulls in objc2-metal, which this module imports, so gate on the
// feature as well as the platform -- an Apple target without `mtl` otherwise
// fails to build rather than simply omitting Metal support.
#[cfg(all(
    feature = "mtl",
    any(target_os = "macos", target_os = "ios", target_os = "visionos", target_os = "tvos")
))]
pub mod metal;
#[cfg(feature = "vulkan")]
pub mod vulkan;

#[cfg(feature = "gl")]
pub mod gl;