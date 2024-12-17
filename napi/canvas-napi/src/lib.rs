#![deny(clippy::all)]
#![allow(clippy::many_single_char_names)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::new_without_default)]

#[macro_use]
extern crate napi_derive;
// #[macro_use]
// extern crate serde_derive;

pub mod gpu;
pub mod c2d;
pub mod image_asset;
pub mod dom_matrix;
mod text_encoder;
mod text_decoder;

pub mod gl;
pub mod gl2;
mod image_bitmap;