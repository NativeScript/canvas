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
#[cfg(all(feature = "d3d", target_os = "windows"))]
pub mod d3d;
// Composition swapchains: 2D presents from D3D12 (`d3d`), WebGL from ANGLE's D3D11 (`gl`).
#[cfg(all(target_os = "windows", any(feature = "d3d", feature = "gl")))]
pub mod dxgi;

#[cfg(feature = "gl")]
pub mod gl;