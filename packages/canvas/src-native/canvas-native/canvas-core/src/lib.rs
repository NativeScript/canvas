#[cfg(any(target_os = "ios", target_os = "macos"))]
extern crate objc;
extern crate core;

#[allow(deprecated, dead_code)]
pub mod common;

#[cfg(target_os = "android")]
#[allow(non_snake_case)]
pub mod android;

#[cfg(any(target_os = "ios", target_os = "macos"))]
pub mod ios;
