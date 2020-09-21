#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]


#[cfg(target_os = "android")]
#[allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/android_bindings.rs"));

// #[cfg(target_os = "ios")]
// #[macro_use]
// extern crate objc;

#[cfg(target_os = "ios")]
include!(concat!(env!("OUT_DIR"), "/ios_bindings.rs"));

//
// #[cfg(target_os = "android")]
// #[allow(non_snake_case)]
// mod android_bindings;
// pub use android_bindings::*;
//
// #[cfg(target_os = "ios")]
// mod ios_bindings;
// pub use ios_bindings::*;


