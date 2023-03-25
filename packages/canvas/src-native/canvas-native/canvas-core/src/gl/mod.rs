//#[cfg(target_os = "ios")]
mod ios;
//#[cfg(target_os = "ios")]
pub use ios::*;

#[cfg(target_os = "macos")]
mod mac;

#[cfg(target_os = "macos")]
pub use mac::*;

#[cfg(target_os = "android")]
mod android;

#[cfg(target_os = "android")]
pub use android::*;
