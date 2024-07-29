#![allow(non_snake_case)]

use std::ffi::CStr;
use std::os::raw::c_char;

pub use canvas_2d::context::text_styles::text_align::TextAlign;
#[cfg(target_os = "android")]
use std::sync::OnceLock;

pub mod canvas2d;

pub mod webgpu;

/* Utils */

#[cfg(target_os = "android")]
pub static API_LEVEL: OnceLock<i32> = OnceLock::new();

#[cfg(target_os = "android")]
pub mod choreographer;

pub mod buffers;
pub mod helper;
pub mod image_asset;
#[cfg(any(target_os = "android", target_os = "ios"))]
mod raf;
pub mod text_decoder;
pub mod text_encoder;
pub mod webgl;

// pub mod impl_test;
/* Raf */
#[cfg(any(target_os = "android", target_os = "ios"))]
#[derive(Clone)]
pub struct Raf(raf::Raf);
/* Raf */

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum InvalidateState {
    None,
    Pending,
    Invalidating,
}

/* Helpers */

#[no_mangle]
pub extern "C" fn canvas_native_font_add_family(
    alias: *const c_char,
    filenames: *const *const c_char,
    length: usize,
) {
    let names = unsafe { std::slice::from_raw_parts(filenames, length) };
    let names = names
        .iter()
        .map(|value| unsafe { CStr::from_ptr(*value).to_string_lossy().to_string() })
        .collect::<Vec<_>>();
    let tmp = names.iter().map(String::as_ref).collect::<Vec<&str>>();
    if alias.is_null() {
        let _ = canvas_2d::context::drawing_text::global_fonts::FontLibrary::add_family(
            None,
            tmp.as_slice(),
        );
    } else {
        let alias = unsafe { CStr::from_ptr(alias) };
        let alias = alias.to_string_lossy();
        let _ = canvas_2d::context::drawing_text::global_fonts::FontLibrary::add_family(
            Some(alias.as_ref()),
            tmp.as_slice(),
        );
    }
}
