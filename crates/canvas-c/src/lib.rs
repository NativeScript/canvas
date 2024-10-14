#![allow(non_snake_case)]

use std::ffi::CStr;
use std::os::raw::c_char;

pub use canvas_2d::context::text_styles::text_align::TextAlign;
#[cfg(target_os = "android")]
use std::sync::OnceLock;

mod c2d;
pub use c2d::*;

pub mod webgpu;

/* Utils */

#[cfg(target_os = "android")]
pub static API_LEVEL: OnceLock<i32> = OnceLock::new();

#[cfg(target_os = "android")]
pub mod choreographer;

mod buffers;
pub use buffers::*;
mod helper;
pub use helper::*;
mod image_asset;
pub use image_asset::*;
#[cfg(any(target_os = "android", target_os = "ios"))]
mod raf;
mod text_decoder;
pub use text_decoder::*;
mod text_encoder;
pub use text_encoder::*;
mod webgl;
pub use webgl::*;
pub mod impl_test;
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


#[no_mangle]
pub extern "C" fn canvas_native_context_2d_test(context: i64) {
    if context == 0 {
        return;
    }

    let context = context as *mut CanvasRenderingContext2D;
    let context = unsafe { &mut *context };

    {
        let ctx = context.get_context_mut();
        ctx.set_fill_style_with_color("red");
        ctx.fill_rect_xywh(0., 0., 300., 300.);
    }
    context.render();
}


#[no_mangle]
pub extern "C" fn canvas_native_context_2d_path_test(context: i64) {
    if context == 0 {
        return;
    }

    let context = context as *mut CanvasRenderingContext2D;
    let context = unsafe { &mut *context };

    {
        let ctx = context.get_context_mut();

        // Create path
        let mut region = canvas_2d::context::paths::path::Path::default();
        region.move_to(30f32, 90f32);
        region.line_to(110f32, 20f32);
        region.line_to(240f32, 130f32);
        region.line_to(60f32, 130f32);
        region.line_to(190f32, 20f32);
        region.line_to(270f32, 90f32);
        region.close_path();

        // Fill path
        ctx.set_fill_style_with_color("green");
        ctx.fill_rule(
            Some(&mut region),
            canvas_2d::context::drawing_paths::fill_rule::FillRule::EvenOdd,
        );
    }
    context.render();
}


#[no_mangle]
pub extern "C" fn canvas_native_context_2d_conic_test(context: i64) {
    if context == 0 {
        return;
    }

    let context = context as *mut CanvasRenderingContext2D;
    let context = unsafe { &mut *context };

    {
        let ctx = context.get_context_mut();
        use canvas_2d::context::fill_and_stroke_styles::paint::Color;

        // Create a conic gradient
        // The start angle is 0
        // The center position is 100, 100
        let mut gradient = ctx.create_conic_gradient(0., 100., 100.);

        // Add five color stops
        gradient.add_color_stop(0., Color::RED);
        gradient.add_color_stop(0.25, Color::from_rgb(255, 165, 0));
        gradient.add_color_stop(0.5, Color::YELLOW);
        gradient.add_color_stop(0.75, Color::GREEN);
        gradient.add_color_stop(1., Color::BLUE);

        // Set the fill style and draw a rectangle
        ctx.set_fill_style(canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Gradient(gradient));
        ctx.fill_rect_xywh(20., 20., 200., 200.);
    }
    context.render();
}