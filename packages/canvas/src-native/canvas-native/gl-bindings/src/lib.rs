#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_attributes)]

#[cfg(target_os = "android")]
#[allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// #[cfg(target_os = "ios")]
// #[macro_use]
// extern crate objc;

#[cfg(any(target_os = "ios", target_os = "macos"))]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

//
// #[cfg(target_os = "android")]
// #[allow(non_snake_case)]
// mod android_bindings;
// pub use android_bindings::*;
//
// #[cfg(target_os = "ios")]
// mod ios_bindings;
// pub use ios_bindings::*;
