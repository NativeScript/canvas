#[derive(Clone)]
pub struct CanvasPattern(canvas_core::context::fill_and_stroke_styles::pattern::Pattern);

#[cxx::bridge]
mod ffi {
    extern "Rust" {
        type CanvasPattern;
    }
}
