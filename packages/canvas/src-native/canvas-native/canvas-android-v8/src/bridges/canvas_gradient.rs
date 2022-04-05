#[derive(Clone)]
pub struct CanvasGradient(canvas_core::context::fill_and_stroke_styles::gradient::Gradient);

#[cxx::bridge]
mod ffi {
    extern "Rust" {
        type CanvasGradient;
    }
}