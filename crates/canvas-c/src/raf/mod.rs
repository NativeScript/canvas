#[cfg(target_os = "android")]
mod android;

#[cfg(target_os = "android")]
pub use android::*;

#[cfg(target_os = "ios")]
mod ios;
#[cfg(target_os = "ios")]
pub use ios::*;

// visionOS has no display-link crate support; use a CADisplayLink-backed RAF instead.
#[cfg(target_os = "visionos")]
mod visionos;
#[cfg(target_os = "visionos")]
pub use visionos::*;
